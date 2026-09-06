//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/efi.h
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
//

extern "C" {
    pub fn efi_init();
}

// Macro flag: #define efi_init()

extern "C" {
    pub fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> c_int;
}
extern "C" {
    pub fn efi_set_mapping_permissions(mm: *mut mm_struct, md: *mut efi_memory_desc_t, _arg: bool) -> c_int;
}

// Load initrd anywhere in system RAM
//
// RISC-V requires the kernel image to placed 2 MB aligned base for 64
// bit and 4MB for 32 bit.
//

extern "C" {
    pub fn arch_efi_call_virt_setup();
}
extern "C" {
    pub fn arch_efi_call_virt_teardown();
}
extern "C" {
    pub fn stext_offset() -> c_ulong;
}
extern "C" {
    pub fn efi_icache_sync(start: c_ulong, end: c_ulong);
}
