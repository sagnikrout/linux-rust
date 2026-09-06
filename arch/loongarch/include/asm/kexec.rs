//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kexec.h
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
// kexec.h for kexec
//
// Copyright (C) 2022 Loongson Technology Corporation Limited
//

// Maximum physical address we can use pages from

// Maximum address we can reach in physical address mode

// Maximum address we can use for the control code buffer

// Reserve a page for the control code buffer

// The native architecture

// Macro flag: #define ARCH_HAS_KIMAGE_ARCH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
    pub efi_boot: c_ulong,
    pub cmdline_ptr: c_ulong,
    pub systable_ptr: c_ulong,
}

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

extern "C" {
    pub fn kexec_reboot();
}

