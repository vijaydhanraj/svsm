// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024 Intel Corporation
//
// Author: Vijay Dhanraj <vijay.dhanraj@intel.com>

use super::features::{cpu_has_feature, X86_FEATURE_X2APIC};
use super::msr::{
    rdmsr, wrmsr, MSR_IA32_APIC_BASE, MSR_IA32_X2APIC_APICID, MSR_IA32_X2APIC_EOI,
    MSR_IA32_X2APIC_ICR, MSR_IA32_X2APIC_ISR0, MSR_IA32_X2APIC_ISR7, MSR_IA32_X2APIC_SELF_IPI,
    MSR_IA32_X2APIC_SIVR,
};
use crate::utils::immut_after_init::ImmutAfterInitCell;

// fields in APIC_BASE MSR
const APIC_X2APIC_ENABLED: u64 = 1u64 << 10;
const APIC_XAPIC_ENABLED: u64 = 1u64 << 11;

// fields in SVR MSR
const APIC_SVR_ENABLE: u64 = 1u64 << 8;
const APIC_SVR_VECTOR: u64 = 0xff;

// fields in ICR MSR
const APIC_ICR_DEST_MODE_PHYSICAL: u64 = 0u64 << 11;

#[derive(Clone, Copy, Debug)]
pub struct LocalApic;

pub static LAPIC: ImmutAfterInitCell<LocalApic> = ImmutAfterInitCell::new(LocalApic);

impl LocalApic {
    pub fn init(&self) {
        assert!(cpu_has_feature(X86_FEATURE_X2APIC));

        let base = rdmsr(MSR_IA32_APIC_BASE).unwrap();
        if base & APIC_X2APIC_ENABLED == 0 {
            wrmsr(
                MSR_IA32_APIC_BASE,
                base | APIC_X2APIC_ENABLED | APIC_XAPIC_ENABLED,
            )
            .unwrap();
        }

        wrmsr(MSR_IA32_X2APIC_SIVR, 0).unwrap();
        // TODO: register spurious-interrupt handler
        wrmsr(MSR_IA32_X2APIC_SIVR, APIC_SVR_ENABLE | APIC_SVR_VECTOR).unwrap();

        // Make sure ISR is not set
        for isr in (MSR_IA32_X2APIC_ISR0..=MSR_IA32_X2APIC_ISR7).rev() {
            if rdmsr(isr).unwrap() != 0 {
                wrmsr(MSR_IA32_X2APIC_EOI, 0).unwrap();
            }
        }
    }

    pub fn id(&self) -> u32 {
        rdmsr(MSR_IA32_X2APIC_APICID).unwrap() as u32
    }

    pub fn eoi(&self) {
        wrmsr(MSR_IA32_X2APIC_EOI, 0).unwrap();
    }

    pub fn send_ipi(&self, apic_id: u32, vec: u8) {
        if self.id() == apic_id {
            wrmsr(MSR_IA32_X2APIC_SELF_IPI, vec as u64).unwrap();
        } else {
            let icr = vec as u64 | APIC_ICR_DEST_MODE_PHYSICAL | ((apic_id as u64) << 32);
            wrmsr(MSR_IA32_X2APIC_ICR, icr).unwrap();
        }
    }
}
