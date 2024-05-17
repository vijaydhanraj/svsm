// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

use crate::error::SvsmError;
use core::arch::asm;

pub const EFER: u32 = 0xC000_0080;
pub const SEV_STATUS: u32 = 0xC001_0131;
pub const SEV_GHCB: u32 = 0xC001_0130;
pub const MSR_GS_BASE: u32 = 0xC000_0101;

pub fn read_msr(msr: u32) -> Result<u64, SvsmError> {
    let eax: u32;
    let edx: u32;
    let rcx: u64;

    unsafe {
        asm!("1: rdmsr",
             "   xorq %rcx, %rcx",
             "2:",
             ".pushsection \"__exception_table\",\"a\"",
             ".balign 16",
             ".quad (1b)",
             ".quad (2b)",
             ".popsection",
                in("ecx") msr,
                out("eax") eax,
                out("edx") edx,
                lateout("rcx") rcx,
                options(att_syntax, nostack));
    }

    if rcx == 0 {
        let ret = (eax as u64) | (edx as u64) << 32;
        Ok(ret)
    } else {
        Err(SvsmError::Msr)
    }
}

pub fn write_msr(msr: u32, val: u64) -> Result<(), SvsmError> {
    let eax = val as u32;
    let edx = (val >> 32) as u32;
    let rcx: u64;

    unsafe {
        asm!("1: wrmsr",
             "   xorq %rcx, %rcx",
             "2:",
             ".pushsection \"__exception_table\",\"a\"",
             ".balign 16",
             ".quad (1b)",
             ".quad (2b)",
             ".popsection",
                in("ecx") msr,
                in("eax") eax,
                in("edx") edx,
                lateout("rcx") rcx,
                options(att_syntax, nostack));
    }

    if rcx == 0 {
        Ok(())
    } else {
        Err(SvsmError::Msr)
    }
}

pub fn rdtsc() -> u64 {
    let eax: u32;
    let edx: u32;

    unsafe {
        asm!("rdtsc",
             out("eax") eax,
             out("edx") edx,
             options(att_syntax, nomem, nostack));
    }
    (eax as u64) | (edx as u64) << 32
}

#[derive(Debug, Clone, Copy)]
pub struct RdtscpOut {
    pub timestamp: u64,
    pub pid: u32,
}

pub fn rdtscp() -> RdtscpOut {
    let eax: u32;
    let edx: u32;
    let ecx: u32;

    unsafe {
        asm!("rdtscp",
             out("eax") eax,
             out("ecx") ecx,
             out("edx") edx,
             options(att_syntax, nomem, nostack));
    }
    RdtscpOut {
        timestamp: (eax as u64) | (edx as u64) << 32,
        pid: ecx,
    }
}

pub fn read_flags() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            r#"
                pushfq
                pop     %rax
            "#,
             out("rax") rax,
             options(att_syntax));
    }
    rax
}
