//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/tlbflush.h
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
// Flush all TLB entries on the local CPU.
//
extern "C" {
    pub fn volatile("memory": "ptlb" : : :) -> asm;
}
//
// Flush TLB entries for a specific ASCE on all CPUs
//
// Global TLB flush for the mm
extern "C" {
    pub fn volatile(0: "idte, _arg: %1, (opt): %0" : : "a", "cc": "a" (asce) :) -> asm;
}
//
// Flush all TLB entries on all CPUs.
//
// Flush TLB entries for a specific mm on all CPUs (in case gmap is used
// this implicates multiple ASCEs!).
//
// Reset TLB flush mask
// Global TLB flush
//
// TLB flushing:
// flush_tlb_all() - flushes all processes TLBs
// flush_tlb_mm(mm) - flushes the specified mm context TLB's
// flush_tlb_page(vma, vmaddr) - flushes one page
// flush_tlb_range(vma, start, end) - flushes a range of pages
// flush_tlb_kernel_range(start, end) - flushes a range of kernel pages
//
// flush_tlb_mm goes together with ptep_set_wrprotect for the
// copy_page_range operation and flush_tlb_range is related to
// ptep_get_and_clear for change_protection. ptep_set_wrprotect and
// ptep_get_and_clear do not flush the TLBs directly if the mm has
// only one user. At the end of the update the flush_tlb_mm and
// flush_tlb_range functions need to do the flush.
//

