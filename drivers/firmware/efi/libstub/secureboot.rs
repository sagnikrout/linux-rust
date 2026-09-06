//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/secureboot.c
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
// Secure boot handling.
//
// Copyright (C) 2013,2014 Linaro Limited
// Roy Franz <roy.franz@linaro.org
// Copyright (C) 2013 Red Hat, Inc.
// Mark Salter <msalter@redhat.com>
//

// SHIM variables
    let mut shim_guid: static efi_guid_t = EFI_SHIM_LOCK_GUID;
    static const efi_char16_t shim_MokSBState_name[] = L"MokSBStateRT";
    static efi_status_t get_var(efi_char16_t *name, efi_guid_t *vendor, u32 *attr,
    unsigned long *data_size, void *data)
    {
    return get_efi_var(name, vendor, attr, data_size, data);
    }
//
// Determine whether we're in secure boot mode.
//
#[no_mangle]
pub unsafe extern "C" fn efi_get_secureboot() -> enum efi_secureboot_mode {
    enum efi_secureboot_mode efi_get_secureboot(void)
    {
    u32 attr;
    unsigned long size;
    enum efi_secureboot_mode mode;
    efi_status_t status;
    u8 moksbstate;
    mode = efi_get_secureboot_mode(get_var);
    if (mode == efi_secureboot_mode_unknown) {
    efi_err("Could not determine UEFI Secure Boot status.\n");
    return efi_secureboot_mode_unknown;
    }
    if (mode != efi_secureboot_mode_enabled)
    return mode;
//
// See if a user has put the shim into insecure mode. If so, and if the
// variable doesn't have the non-volatile attribute set, we might as
// well honor that.
//
    size = sizeof(moksbstate);
    status = get_efi_var(shim_MokSBState_name, &shim_guid,
    &attr, &size, &moksbstate);
// If it fails, we don't care why. Default to secure
    if (status != EFI_SUCCESS)
    goto secure_boot_enabled;
    if (!(attr & EFI_VARIABLE_NON_VOLATILE) && moksbstate == 1)
    return efi_secureboot_mode_disabled;
    secure_boot_enabled:
    efi_info("UEFI Secure Boot is enabled.\n");
    return efi_secureboot_mode_enabled;
    }
