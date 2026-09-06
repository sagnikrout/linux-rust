//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/hyp/fault.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2015 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

//
// Resolve the IPA the hard way using the guest VA.
//
// Stage-1 translation already validated the memory access
// rights. As such, we can use the EL1 translation regime, and
// don't have to distinguish between EL0 and EL1 access.
//
// We do need to save/restore PAR_EL1 though, as we haven't
// saved the guest context yet, and we may return early...
//
// Convert PAR to HPFAR format
// hpfar = PAR_TO_HPFAR(tmp);
//
// Checks for the conditions when HPFAR_EL2 is written, per ARM ARM R_FKLWR.
//
// CPUs affected by ARM erratum #834220 may incorrectly report a
// stage-2 translation fault when a stage-1 permission fault occurs.
//
// Re-walk the page tables to determine if a stage-1 fault actually
// occurred.
//
extern "C" {
    pub fn esr_fsc_is_addr_sz_fault(_arg: esr) -> return;
}
//
// Hijack HPFAR_EL2.NS (RES0 in Non-secure) to indicate a valid
// HPFAR value.
//
