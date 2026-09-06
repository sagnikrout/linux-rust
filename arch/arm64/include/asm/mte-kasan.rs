//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mte-kasan.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020 ARM Ltd.
//

// Whether the MTE asynchronous mode is enabled.
extern "C" {
    pub fn static_branch_unlikely(_arg: &mte_async_or_asymm_mode) -> return;
}

//
// The Tag Check Flag (TCF) mode for MTE is per EL, hence TCF0
// affects EL0 and TCF affects EL1 irrespective of which TTBR is
// used.
// The kernel accesses TTBR0 usually with LDTR/STTR instructions
// when UAO is available, so these would act as EL0 accesses using
// TCF0.
// However futex.h code uses exclusives which would be executed as
// EL1, this can potentially cause a tag check fault even if the
// user disables TCF0.
//
// To address the problem we set the PSTATE.TCO bit in uaccess_enable()
// and reset it in uaccess_disable().
//
// The Tag check override (TCO) bit disables temporarily the tag checking
// preventing the issue.
//
// These functions disable tag checking only if in MTE async mode
// since the sync mode generates exceptions synchronously and the
// nofault or load_unaligned_zeropad can handle them.
//
// These functions are meant to be only used from KASAN runtime through
// the arch_*() interface defined in asm/memory.h.
// These functions don't include system_supports_mte() checks,
// as KASAN only calls them when MTE is supported and enabled.
//
// Note: The format of KASAN tags is 0xF<x>
// Get allocation tag for the address.
extern "C" {
    pub fn mte_get_ptr_tag(_arg: addr) -> return;
}
// Generate a random tag.
extern "C" {
    pub fn mte_get_ptr_tag(_arg: addr) -> return;
}
extern "C" {
    pub fn volatile(gva: __MTE_PREAMBLE "dc, "memory": %0" : : "r"(p) :) -> asm;
}
extern "C" {
    pub fn volatile(gzva: __MTE_PREAMBLE "dc, "memory": %0" : : "r"(p) :) -> asm;
}
//
// Assign allocation tags for a region of memory based on the pointer tag.
// Note: The address must be non-NULL and MTE_GRANULE_SIZE aligned and
// size must be MTE_GRANULE_SIZE aligned.
//
// Read DC G(Z)VA block size from the system register.
// STG/STZG up to the end of the first block.
// DC GVA / GZVA in [end1, end2)
//
// The following code uses STG on the first DC GVA block even if the
// start address is aligned - it appears to be faster than an alignment
// check + conditional branch. Also, if the range size is at least 2 DC
// GVA blocks, the first two loops can use post-condition to save one
// branch each.
//

extern "C" {
    pub fn mte_enable_kernel_sync();
}
extern "C" {
    pub fn mte_enable_kernel_async();
}
extern "C" {
    pub fn mte_enable_kernel_asymm();
}
extern "C" {
    pub fn mte_enable_kernel_store_only() -> c_int;
}

