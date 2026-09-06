//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/compressed/efi.h
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

extern "C" {
    pub fn __aligned(_arg: __alignof__(u32)) -> typedef guid_t efi_guid_t;
}

//
// Generic EFI table header
//
pub const EFI_CONVENTIONAL_MEMORY: c_int = 7;
pub const EFI_UNACCEPTED_MEMORY: c_int = 15;

pub const EFI_PAGE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_unaccepted_memory {
    pub version: u32,
    pub unit_size: u32,
    pub phys_base: u64,
    pub size: u64,
    pub bitmap: [c_ulong; ],
}

extern "C" {
    pub fn memcmp(_arg: &left, _arg: &right, (efi_guid_t): sizeof) -> return;
}

extern "C" {
    pub fn __efi_soft_reserve_enabled() -> bool __pure;
}

