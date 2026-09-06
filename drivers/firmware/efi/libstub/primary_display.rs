//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/primary_display.c
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
// There are two ways of populating the core kernel's sysfb_primary_display
// via the stub:
//
// - using a configuration table, which relies on the EFI init code to
// locate the table and copy the contents; or
//
// - by linking directly to the core kernel's copy of the global symbol.
//
// The latter is preferred because it makes the EFIFB earlycon available very
// early, but it only works if the EFI stub is part of the core kernel image
// itself. The zboot decompressor can only use the configuration table
// approach.
//
    let mut primary_display_guid: static efi_guid_t = LINUX_EFI_PRIMARY_DISPLAY_TABLE_GUID;
    struct sysfb_display_info *__alloc_primary_display(void)
    {
    struct sysfb_display_info *dpy;
    efi_status_t status;
    status = efi_bs_call(allocate_pool, EFI_ACPI_RECLAIM_MEMORY,
    sizeof(*dpy), (void **)&dpy);
    if (status != EFI_SUCCESS)
    return core::ptr::null_mut();
    memset(dpy, 0, sizeof(*dpy));
    status = efi_bs_call(install_configuration_table,
    &primary_display_guid, dpy);
    if (status == EFI_SUCCESS)
    return dpy;
    efi_bs_call(free_pool, dpy);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn free_primary_display(dpy: *mut sysfb_display_info) {
    void free_primary_display(struct sysfb_display_info *dpy)
    {
    if (!dpy)
    return;
    efi_bs_call(install_configuration_table, &primary_display_guid, core::ptr::null_mut());
    efi_bs_call(free_pool, dpy);
    }
