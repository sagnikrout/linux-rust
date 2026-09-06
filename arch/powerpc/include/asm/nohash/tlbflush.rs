//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/tlbflush.h
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
// TLB flushing:
//
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - local_flush_tlb_mm(mm, full) flushes the specified mm context on
// the local processor
// - local_flush_tlb_page(vma, vmaddr) flushes one page on the local processor
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
//
// TLB flushing for software loaded TLB chips
//
// TODO: (CONFIG_PPC_85xx) determine if flush_tlb_range &
// flush_tlb_kernel_range are best implemented as tlbia vs
// specific tlbie's
//

extern "C" {
    pub fn volatile("memory": "sync; tlbia; isync" : : :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "tlbie %0; sync" : : "r" (vmaddr) :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "tlbie %0; sync" : : "r" (vmaddr) :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "tlbie %0; sync" : : "r" (start) :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "sync; tlbia; isync" : : :) -> asm;
}

extern "C" {
    pub fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}
extern "C" {
    pub fn local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: c_ulong, psize: c_int);
}

extern "C" {
    pub fn flush_tlb_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}

