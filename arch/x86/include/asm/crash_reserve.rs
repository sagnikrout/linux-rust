//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/crash_reserve.h
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
// 16M alignment for crash kernel regions

//
// Keep the crash kernel below this limit.
//
// Earlier 32-bits kernels would limit the kernel to the low 512 MB range
// due to mapping restrictions.
//
// 64-bit kdump kernels need to be restricted to be under 64 TB, which is
// the upper limit of system RAM in 4-level paging mode. Since the kdump
// jump could be from 5-level paging to 4-level paging, the jump will fail if
// the kernel is put above 64 TB, and during the 1st kernel bootup there's
// no good way to detect the paging mode of the target kernel which will be
// loaded for dumping.
//
extern "C" {
    pub fn swiotlb_size_or_default() -> c_ulong;
}

extern "C" {
    pub fn max(20): swiotlb_size_or_default() + (8UL <<, 20: 256UL <<) -> return;
}

// Macro flag: #define HAVE_ARCH_ADD_CRASH_RES_TO_IOMEM_EARLY
