//! Automatically rewritten from C to Rust
//! Source: security/integrity/efi_secureboot.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// Copyright (C) 2018 IBM Corporation
//

#[no_mangle]
unsafe extern "C" fn get_sb_mode() -> enum efi_secureboot_mode {
    static enum efi_secureboot_mode get_sb_mode(void)
    {
    enum efi_secureboot_mode mode;
    if (!efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE)) {
    pr_info("integrity: secureboot mode unknown, no efi\n");
    return efi_secureboot_mode_unknown;
    }
    mode = efi_get_secureboot_mode(efi.get_variable);
    if (mode == efi_secureboot_mode_disabled)
    pr_info("integrity: secureboot mode disabled\n");
#[no_mangle]
pub unsafe extern "C" fn if(efi_secureboot_mode_unknown: mode ==) -> else {
    else if (mode == efi_secureboot_mode_unknown)
    pr_info("integrity: secureboot mode unknown\n");
    else
    pr_info("integrity: secureboot mode enabled\n");
    return mode;
    }
//
// Query secure boot status
//
// Note don't call this function too early e.g. in __setup hook otherwise the
// kernel may hang when calling efi_get_secureboot_mode.
//
#[no_mangle]
pub unsafe extern "C" fn arch_get_secureboot() -> bool {
    bool arch_get_secureboot(void)
    {
    static enum efi_secureboot_mode sb_mode;
    static bool initialized;
    if (!initialized && efi_enabled(EFI_BOOT)) {
    sb_mode = arch_efi_boot_mode;
    if (sb_mode == efi_secureboot_mode_unset)
    sb_mode = get_sb_mode();
    initialized = true;
    }
    if (sb_mode == efi_secureboot_mode_enabled)
    return true;
    else
    return false;
    }
