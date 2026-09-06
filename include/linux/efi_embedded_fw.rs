//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/efi_embedded_fw.h
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

pub const EFI_EMBEDDED_FW_PREFIX_LEN: c_int = 8;
//
// This struct is private to the efi-embedded fw implementation.
// They are in this header for use by lib/test_firmware.c only!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_embedded_fw {
    pub list: list_head,
    pub name: *const c_char,
    pub data: *const u8,
    pub length: usize,
}

//
// struct efi_embedded_fw_desc - This struct is used by the EFI embedded-fw
// code to search for embedded firmwares.
//
// @name:   Name to register the firmware with if found
// @prefix: First 8 bytes of the firmware
// @length: Length of the firmware in bytes including prefix
// @sha256: SHA256 of the firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_embedded_fw_desc {
    pub name: *const c_char,
    pub prefix: [u8; EFI_EMBEDDED_FW_PREFIX_LEN],
    pub length: u32,
    pub sha256: [u8; 32],
}

extern "C" {
    pub fn efi_get_embedded_fw(name: *const c_char, dat: *const u8, sz: *mut usize) -> c_int;
}
