//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/pgtable-bits.h
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
// Page table bits

pub const _PAGE_VALID_SHIFT: c_int = 0;

pub const _PAGE_DIRTY_SHIFT: c_int = 1;

pub const _PAGE_GLOBAL_SHIFT: c_int = 6;

pub const _PAGE_PRESENT_SHIFT: c_int = 7;
pub const _PAGE_PFN_SHIFT: c_int = 8;

pub const _PAGE_SWP_EXCLUSIVE_SHIFT: c_int = 13;
pub const _PAGE_PFN_END_SHIFT: c_int = 28;
pub const _PAGE_WRITE_SHIFT: c_int = 29;
pub const _PAGE_MODIFIED_SHIFT: c_int = 30;
pub const _PAGE_PRESENT_INVALID_SHIFT: c_int = 31;

pub const _PAGE_VALID_SHIFT: c_int = 0;

pub const _PAGE_DIRTY_SHIFT: c_int = 1;

pub const _PAGE_GLOBAL_SHIFT: c_int = 6;

pub const _PAGE_PRESENT_SHIFT: c_int = 7;
pub const _PAGE_WRITE_SHIFT: c_int = 8;
pub const _PAGE_MODIFIED_SHIFT: c_int = 9;
pub const _PAGE_PROTNONE_SHIFT: c_int = 10;
pub const _PAGE_SPECIAL_SHIFT: c_int = 11;
pub const _PAGE_PFN_SHIFT: c_int = 12;

pub const _PAGE_SWP_EXCLUSIVE_SHIFT: c_int = 23;
pub const _PAGE_PFN_END_SHIFT: c_int = 48;
pub const _PAGE_PRESENT_INVALID_SHIFT: c_int = 60;
pub const _PAGE_NO_READ_SHIFT: c_int = 61;
pub const _PAGE_NO_EXEC_SHIFT: c_int = 62;
pub const _PAGE_RPLV_SHIFT: c_int = 63;

// Used by software

pub const _PAGE_PROTNONE: c_int = 0;
pub const _PAGE_SPECIAL: c_int = 0;

// We borrow bit 13/23 to store the exclusive marker in swap PTEs.

// Used by TLB hardware (placed in EntryLo*)

pub const _PAGE_NO_READ: c_int = 0;
pub const _PAGE_NO_EXEC: c_int = 0;
pub const _PAGE_RPLV: c_int = 0;

//
// Cache attributes
//

extern "C" {
    pub fn __pgprot(_PAGE_NO_EXEC: pgprot_val(_prot) |) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: prot) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: prot) -> return;
}

