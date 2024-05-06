//
// Copyright (c) 2023-2024 Intel Corporation.
//
// Author:
// Chuanxiao Dong <chuanxiao.dong@intel.com>
// Vijay Dhanraj <vijay.dhanraj@intel.com>

use super::features::{cpu_has_feature, X86_FEATURE_X2APIC};
use super::msr::{
    read_msr, write_msr, MSR_IA32_APIC_BASE, MSR_IA32_X2APIC_APICID, MSR_IA32_X2APIC_EOI,
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

        let base = read_msr(MSR_IA32_APIC_BASE);
        if base & APIC_X2APIC_ENABLED == 0 {
            write_msr(
                MSR_IA32_APIC_BASE,
                base | APIC_X2APIC_ENABLED | APIC_XAPIC_ENABLED,
            );
        }

        write_msr(MSR_IA32_X2APIC_SIVR, 0);
        // TODO: register spurious-interrupt handler
        write_msr(MSR_IA32_X2APIC_SIVR, APIC_SVR_ENABLE | APIC_SVR_VECTOR);

        // Make sure ISR is not set
        for isr in (MSR_IA32_X2APIC_ISR0..=MSR_IA32_X2APIC_ISR7).rev() {
            if read_msr(isr) != 0 {
                write_msr(MSR_IA32_X2APIC_EOI, 0);
            }
        }
    }

    pub fn id(&self) -> u32 {
        read_msr(MSR_IA32_X2APIC_APICID) as u32
    }

    pub fn eoi(&self) {
        write_msr(MSR_IA32_X2APIC_EOI, 0);
    }

    pub fn send_ipi(&self, apic_id: u32, vec: u8) {
        if self.id() == apic_id {
            write_msr(MSR_IA32_X2APIC_SELF_IPI, vec as u64);
        } else {
            let icr = vec as u64 | APIC_ICR_DEST_MODE_PHYSICAL | ((apic_id as u64) << 32);
            write_msr(MSR_IA32_X2APIC_ICR, icr);
        }
    }
}
