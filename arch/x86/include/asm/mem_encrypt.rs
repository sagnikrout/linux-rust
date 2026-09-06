//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mem_encrypt.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AMD Memory Encryption Support
//
// Copyright (C) 2016 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

extern "C" {
    pub fn mem_encrypt_init() -> void __init;
}
extern "C" {
    pub fn mem_encrypt_setup_arch() -> void __init;
}

extern "C" {
    pub fn sme_map_bootdata(real_mode_data: *mut c_char) -> void __init;
}
extern "C" {
    pub fn sme_unmap_bootdata(real_mode_data: *mut c_char) -> void __init;
}
extern "C" {
    pub fn sme_early_init() -> void __init;
}
extern "C" {
    pub fn sme_encrypt_kernel(bp: *mut boot_params);
}
extern "C" {
    pub fn sme_enable(bp: *mut boot_params);
}
extern "C" {
    pub fn early_set_memory_decrypted(vaddr: c_ulong, size: c_ulong) -> int __init;
}
extern "C" {
    pub fn early_set_memory_encrypted(vaddr: c_ulong, size: c_ulong) -> int __init;
}
extern "C" {
    pub fn mem_encrypt_free_decrypted_mem() -> void __init;
}
extern "C" {
    pub fn sev_es_init_vc_handling() -> void __init;
}

// Macro flag: #define __bss_decrypted

extern "C" {
    pub fn add_encrypt_protection_map();
}
//
// The __sme_pa() and __sme_pa_nodebug() macros are meant for use when
// writing to or comparing values from the cr3 register.  Having the
// encryption mask set in cr3 enables the PGD entry to be encrypted and
// avoid special case handling of PGD allocations.
//

