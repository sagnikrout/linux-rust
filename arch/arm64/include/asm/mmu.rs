//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mmu.h
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
// Copyright (C) 2012 ARM Ltd.
//

pub const MMCF_AARCH32: c_uint = 0x1	/* mm context flag for AArch32 executables */;
pub const USER_ASID_BIT: c_int = 48;

//
// We use atomic64_read() here because the ASID for an 'mm_struct' can
// be reallocated when scheduling one of its threads following a
// rollover event (see new_context() and flush_context()). In this case,
// a concurrent TLBI (e.g. via try_to_unmap_one() and ptep_clear_flush())
// may use a stale ASID. This is fine in principle as the new ASID is
// guaranteed to be clean in the TLB, but the TLBI routines have to take
// care to handle the following race:
//
// CPU 0                    CPU 1                          CPU 2
//
// // ptep_clear_flush(mm)
// xchg_relaxed(pte, 0)
// DSB ISHST
// old = ASID(mm)
// |                                                  <rollover>
// |                   new = new_context(mm)
// \-----------------> atomic_set(mm->context.id, new)
// cpu_switch_mm(mm)
// // Hardware walk of pte using new ASID
// TLBI(old)
//
// In this scenario, the barrier on CPU 0 and the dependency on CPU 1
// ensure that the page-table walker on CPU 1 *must* see the invalid PTE
// written by CPU 0.
//

extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_UNMAP_KERNEL_AT_EL0) -> return;
}
extern "C" {
    pub fn arm64_memblock_init();
}
extern "C" {
    pub fn paging_init();
}
extern "C" {
    pub fn bootmem_init();
}
extern "C" {
    pub fn mark_linear_text_alias_ro();
}
extern "C" {
    pub fn split_kernel_leaf_mapping(start: c_ulong, end: c_ulong) -> c_int;
}
extern "C" {
    pub fn linear_map_maybe_split_to_ptes();
}
//
// This check is triggered during the early boot before the cpufeature
// is initialised. Checking the status on the local CPU allows the boot
// CPU to detect the need for non-global mappings and thus avoiding a
// pagetable re-write after all the CPUs are booted. This check will be
// anyway run on individual CPUs, allowing us to get the consistent
// state once the SMP CPUs are up and thus make the switch to non-global
// mappings if required.
//
// E0PD does a similar job to KPTI so can be used instead
// where available.
//

extern "C" {
    pub fn kpti_install_ng_mappings();
}

