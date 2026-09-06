//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/efi-bgrt.c
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
// Copyright 2012 Intel Corporation
// Author: Josh Triplett <josh@joshtriplett.org>
//
// Based on the bgrt driver:
// Copyright 2012 Red Hat, Inc <mjg@redhat.com>
// Author: Matthew Garrett
//

    struct acpi_table_bgrt bgrt_tab;
    size_t bgrt_image_size;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_header {
    pub id: u16,
    pub size: u32,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn efi_bgrt_init(table: *mut acpi_table_header) -> void __init {
    void __init efi_bgrt_init(struct acpi_table_header *table)
    {
    pub image: *mut c_void,
    pub bmp_header: bmp_header,
    pub &bgrt_tab: *mut *mut acpi_table_bgrt bgrt =,
    pub mem_type: c_int,
    if (acpi_disabled)
    if (!efi_enabled(EFI_MEMMAP) && !efi_enabled(EFI_PARAVIRT))
    if (table.length < sizeof(bgrt_tab)) {
    pr_notice("Ignoring BGRT: invalid length %u (expected %zu)\n",
    pub sizeof(bgrt_tab)): table->length,,
    }
// bgrt = *(struct acpi_table_bgrt *)table;
//
// Only version 1 is defined but some older laptops (seen on Lenovo
// Ivy Bridge models) have a correct version 1 BGRT table with the
// version set to 0, so we accept version 0 and 1.
//
    if (bgrt.version > 1) {
    pr_notice("Ignoring BGRT: invalid version %u (expected 1)\n",
    pub out: goto,
    }
    if (bgrt.image_type != 0) {
    pr_notice("Ignoring BGRT: invalid image type %u (expected 0)\n",
    pub out: goto,
    }
    if (!bgrt.image_address) {
    pub address\n"): pr_notice("Ignoring BGRT: null image,
    pub out: goto,
    }
    pub efi_mem_type(bgrt->image_address): mem_type =,
    if (mem_type != EFI_BOOT_SERVICES_DATA &&
    mem_type != EFI_ACPI_RECLAIM_MEMORY) {
    pub address\n"): pr_notice("Ignoring BGRT: invalid image,
    pub out: goto,
    }
    pub sizeof(bmp_header)): image = early_memremap(bgrt->image_address,,
    if (!image) {
    pub memory\n"): pr_notice("Ignoring BGRT: failed to map image header,
    pub out: goto,
    }
    pub sizeof(bmp_header)): memcpy(&bmp_header, image,,
    pub sizeof(bmp_header)): early_memunmap(image,,
    if (bmp_header.id != 0x4d42) {
    pr_notice("Ignoring BGRT: Incorrect BMP magic number 0x%x (expected 0x4d42)\n",
    pub out: goto,
    }
    pub bmp_header.size: bgrt_image_size =,
    pub bgrt_image_size): efi_mem_reserve(bgrt->image_address,,
    out:
    pub sizeof(bgrt_tab)): memset(bgrt, 0,,
    }
