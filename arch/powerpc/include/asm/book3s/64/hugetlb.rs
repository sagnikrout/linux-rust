//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/hugetlb.h
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
// For radix we want generic code to handle hugetlb. But then if we want
// both hash and radix to be enabled together we need to workaround the
// limitations.
//
extern "C" {
    pub fn radix__flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}
extern "C" {
    pub fn radix__local_flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}
//
// We used gigantic page reservation with hypervisor assist in some case.
// We cannot use runtime allocation of gigantic pages in those platforms
// This is hash translation mode LPARs.
//

extern "C" {
    pub fn radix__flush_hugetlb_page(_arg: vma, _arg: vmaddr) -> return;
}
extern "C" {
    pub fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}
//
// We need to make sure that for different page sizes reported by
// firmware we only add hugetlb support for page sizes that can be
// supported by linux page table layout.
// For now we have
// Radix: 2M and 1G
// Hash: 16M and 16G
//

