//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/tlb.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TLB shootdown specifics for powerpc
//
// Copyright (C) 2002 Anton Blanchard, IBM Corp.
// Copyright (C) 2002 Paul Mackerras, IBM Corp.
//

extern "C" {
    pub fn tlb_flush(tlb: *mut mmu_gather);
}
//
// book3s:
// Hash does not use the linux page-tables, so we can avoid
// the TLB invalidate for page-table freeing, Radix otoh does use the
// page-tables and needs the TLBI.
//
// nohash:
// We still do TLB invalidate in the __pte_free_tlb routine before we
// add the page table pages to mmu gather table batch.
//

// Get the generic bits...

extern "C" {
    pub fn cpumask_test_cpu(_arg: smp_processor_id(), _arg: mm_cpumask(mm)) -> return;
}

extern "C" {
    pub fn radix_enabled() -> return;
}

