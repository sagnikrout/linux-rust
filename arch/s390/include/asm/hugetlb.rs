//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/hugetlb.h
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
// IBM System z Huge TLB Page Support for Kernel.
//
// Copyright IBM Corp. 2008
// Author(s): Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

extern "C" {
    pub fn huge_ptep_get(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t;
}
extern "C" {
    pub fn __huge_ptep_get_and_clear(_arg: mm, _arg: addr, _arg: ptep) -> return;
}
extern "C" {
    pub fn __huge_ptep_get_and_clear(_arg: vma->vm_mm, _arg: address, _arg: ptep) -> return;
}

