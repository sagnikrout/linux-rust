//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/32/tlbflush.h
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
// TLB flushing for "classic" hash-MMU 32-bit CPUs, 6xx, 7xx, 7xxx
//
extern "C" {
    pub fn hash__flush_tlb_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn hash__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}
extern "C" {
    pub fn hash__flush_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn hash__flush_gather(tlb: *mut mmu_gather);
}

extern "C" {
    pub fn _tlbie(address: c_ulong);
}

extern "C" {
    pub fn volatile("memory": "tlbie %0; sync" : : "r" (address) :) -> asm;
}

extern "C" {
    pub fn _tlbia();
}
//
// Called at the end of a mmu_gather operation to make sure the
// TLB flush is completely done.
//
// 603 needs to flush the whole TLB here since it doesn't use a hash table.
