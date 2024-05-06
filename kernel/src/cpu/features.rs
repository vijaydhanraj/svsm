// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

use super::cpuid::cpuid_table;
use crate::utils::immut_after_init::ImmutAfterInitCell;
use crate::{BIT, OFFSET_BITPOS};

// CPUID feature words
const FEAT_1_EDX: u32 = 0; // CPUID[1].EDX
const FEAT_8000_0001_EDX: u32 = 1; // CPUID[8000_0001].EDX
const FEAT_1_ECX: u32 = 2; // CPUID[1].ECX
const FEATURE_WORDS: usize = 3;

// CPUID level 0x00000001 (EDX), word 0
pub const X86_FEATURE_PGE: u32 = (FEAT_1_EDX << 5) + 13; // Page Global Enable

// CPUID level 0x80000001 (EDX), word 1
pub const X86_FEATURE_NX: u32 = (FEAT_8000_0001_EDX << 5) + 20; // Execute Disable

// CPUID level 0x00000001 (ECX), word 2
pub const X86_FEATURE_X2APIC: u32 = (FEAT_1_ECX << 5) + 21; /* X2APIC */

#[derive(Clone, Copy)]
struct X86Features {
    word: [u32; FEATURE_WORDS],
}

impl X86Features {
    fn new() -> Self {
        let mut word = [0; FEATURE_WORDS];
        word[FEAT_1_EDX as usize] = cpuid_table(0x00000001)
            .expect("cpuid leaf 0x1 edx not found!")
            .edx;
        word[FEAT_8000_0001_EDX as usize] = cpuid_table(0x80000001)
            .expect("cpuid leaf 0x80000001 edx not found!")
            .edx;
        word[FEAT_1_ECX as usize] = cpuid_table(0x00000001)
            .expect("cpuid leaf 0x80000001 edx not found!")
            .ecx;
        X86Features { word }
    }
}

static X86_FEATURES: ImmutAfterInitCell<X86Features> = ImmutAfterInitCell::uninit();

pub fn init_cpuid_features() {
    let feat = X86Features::new();

    X86_FEATURES
        .init(&feat)
        .expect("X86 cpuid features already initialized");
}

pub fn cpu_has_feature(feat: u32) -> bool {
    let (offset, bitpos) = OFFSET_BITPOS!(feat, 32);
    X86_FEATURES.word[offset as usize] & BIT!(bitpos) != 0
}
