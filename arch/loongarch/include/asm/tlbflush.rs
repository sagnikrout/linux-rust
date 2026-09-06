//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/tlbflush.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// TLB flushing:
//
// - flush_tlb_all() flushes all processes TLB entries
// - flush_tlb_mm(mm) flushes the specified mm context TLB entries
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
//
extern "C" {
    pub fn local_flush_tlb_all();
}
extern "C" {
    pub fn local_flush_tlb_user();
}
extern "C" {
    pub fn local_flush_tlb_kernel();
}
extern "C" {
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn local_flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
}
extern "C" {
    pub fn local_flush_tlb_one(vaddr: c_ulong);
}

extern "C" {
    pub fn flush_tlb_all();
}
extern "C" {
    pub fn flush_tlb_mm(: *mut mm_struct);
}
extern "C" {
    pub fn flush_tlb_range(vma: *mut vm_area_struct, long: unsigned, long: unsigned);
}
extern "C" {
    pub fn flush_tlb_kernel_range(long: unsigned, long: unsigned);
}
extern "C" {
    pub fn flush_tlb_page(: *mut vm_area_struct, long: unsigned);
}
extern "C" {
    pub fn flush_tlb_one(vaddr: c_ulong);
}

