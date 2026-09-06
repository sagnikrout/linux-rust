//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/amd/quirks.c
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
// Quirks for AMD IOMMU
//
// Copyright (C) 2019 Kai-Heng Feng <kai.heng.feng@canonical.com>
//

pub const IVHD_SPECIAL_IOAPIC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivrs_quirk_entry {
    pub id: u8,
    pub devid: u32,
}

    enum {
    DELL_INSPIRON_7375 = 0,
    DELL_LATITUDE_5495,
    LENOVO_IDEAPAD_330S_15ARR,
    };
    static const struct ivrs_quirk_entry ivrs_ioapic_quirks[][3] __initconst = {
// ivrs_ioapic[4]=00:14.0 ivrs_ioapic[5]=00:00.2
    [DELL_INSPIRON_7375] = {
    { .id = 4, .devid = 0xa0 },
    { .id = 5, .devid = 0x2 },
    {}
    },
// ivrs_ioapic[4]=00:14.0
    [DELL_LATITUDE_5495] = {
    { .id = 4, .devid = 0xa0 },
    {}
    },
// ivrs_ioapic[32]=00:14.0
    [LENOVO_IDEAPAD_330S_15ARR] = {
    { .id = 32, .devid = 0xa0 },
    {}
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn ivrs_ioapic_quirk_cb(d: *const dmi_system_id) -> int __init {
    static int __init ivrs_ioapic_quirk_cb(const struct dmi_system_id *d)
    {
    const struct ivrs_quirk_entry *i;
    for (i = d.driver_data; i.id != 0 && i.devid != 0; i++)
    add_special_device(IVHD_SPECIAL_IOAPIC, i.id, (u32 *)&i.devid, 0);
    return 0;
    }
    static const struct dmi_system_id ivrs_quirks[] __initconst = {
    {
    .callback = ivrs_ioapic_quirk_cb,
    .ident = "Dell Inspiron 7375",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Inspiron 7375"),
    },
    .driver_data = (void *)&ivrs_ioapic_quirks[DELL_INSPIRON_7375],
    },
    {
    .callback = ivrs_ioapic_quirk_cb,
    .ident = "Dell Latitude 5495",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Latitude 5495"),
    },
    .driver_data = (void *)&ivrs_ioapic_quirks[DELL_LATITUDE_5495],
    },
    {
//
// Acer Aspire A315-41 requires the very same workaround as
// Dell Latitude 5495
//
    .callback = ivrs_ioapic_quirk_cb,
    .ident = "Acer Aspire A315-41",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Acer"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Aspire A315-41"),
    },
    .driver_data = (void *)&ivrs_ioapic_quirks[DELL_LATITUDE_5495],
    },
    {
    .callback = ivrs_ioapic_quirk_cb,
    .ident = "Lenovo ideapad 330S-15ARR",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "81FB"),
    },
    .driver_data = (void *)&ivrs_ioapic_quirks[LENOVO_IDEAPAD_330S_15ARR],
    },
    {}
    };
#[no_mangle]
pub unsafe extern "C" fn amd_iommu_apply_ivrs_quirks() -> void __init {
    void __init amd_iommu_apply_ivrs_quirks(void)
    {
    dmi_check_system(ivrs_quirks);
    }
