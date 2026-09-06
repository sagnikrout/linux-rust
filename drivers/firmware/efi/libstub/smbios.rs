//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/smbios.c
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
// Copyright 2022 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

    typedef union efi_smbios_protocol efi_smbios_protocol_t;
    union efi_smbios_protocol {
    struct {
    efi_status_t (__efiapi *add)(efi_smbios_protocol_t *, efi_handle_t,
    u16 *, struct efi_smbios_record *);
    efi_status_t (__efiapi *update_string)(efi_smbios_protocol_t *, u16 *,
    unsigned long *, u8 *);
    efi_status_t (__efiapi *remove)(efi_smbios_protocol_t *, u16);
    efi_status_t (__efiapi *get_next)(efi_smbios_protocol_t *, u16 *, u8 *,
    struct efi_smbios_record **,
    efi_handle_t *);
    u8 major_version;
    u8 minor_version;
    };
    struct {
    u32 add;
    u32 update_string;
    u32 remove;
    u32 get_next;
    u8 major_version;
    u8 minor_version;
    } mixed_mode;
    };
    const struct efi_smbios_record *efi_get_smbios_record(u8 type)
    {
    struct efi_smbios_record *record;
    efi_smbios_protocol_t *smbios;
    efi_status_t status;
    let mut handle: u16 = 0xfffe;
    status = efi_bs_call(locate_protocol, &EFI_SMBIOS_PROTOCOL_GUID, core::ptr::null_mut(),
    (void **)&smbios) ?:
    efi_call_proto(smbios, get_next, &handle, &type, &record, core::ptr::null_mut());
    if (status != EFI_SUCCESS)
    return core::ptr::null_mut();
    return record;
    }
    const u8 *__efi_get_smbios_string(const struct efi_smbios_record *record,
    const u8 *offset)
    {
    const u8 *strtable;
    if (!record)
    return core::ptr::null_mut();
    strtable = (u8 *)record + record.length;
    for (int i = 1; i < *offset; i++) {
    let mut len: c_int = strlen(strtable);
    if (!len)
    return core::ptr::null_mut();
    strtable += len + 1;
    }
    return strtable;
    }
