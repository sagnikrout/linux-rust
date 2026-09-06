//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/page.h
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

// __pa_symbol should be used for C visible symbols.
//
// We need __phys_reloc_hide() here because gcc may assume that there is no
// overflow during __pa() calculation and can optimize it unexpectedly.
// Newer versions of gcc provide -fno-strict-overflow switch to handle this
// case properly. Once all supported versions of gcc understand it, we can
// remove this Voodoo magic stuff. (i.e. once gcc3.x is deprecated)
//

//
// virt_to_page(kaddr) returns a valid pointer if and only if
// virt_addr_valid(kaddr) returns true.
//

extern "C" {
    pub fn __virt_addr_valid(kaddr: c_ulong) -> bool;
}

extern "C" {
    pub fn __va(PAGE_SHIFT: pfn <<) -> return;
}

// Macro flag: #define HAVE_ARCH_HUGETLB_UNMAPPED_AREA

