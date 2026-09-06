//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/acpi.c
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


// SPDX-License-Identifier: ISC
//
// Copyright The Asahi Linux Contributors
//

    void brcmf_acpi_probe(struct device *dev, enum brcmf_bus_type bus_type,
    struct brcmf_mp_device *settings)
    {
    acpi_status status;
    const union acpi_object *o;
    let mut buf: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    struct acpi_device *adev = ACPI_COMPANION(dev);
    if (!adev)
    return;
    if (!ACPI_FAILURE(acpi_dev_get_property(adev, "module-instance",
    ACPI_TYPE_STRING, &o))) {
    brcmf_dbg(INFO, "ACPI module-instance=%s\n", o.string.pointer);
    settings.board_type = devm_kasprintf(dev, GFP_KERNEL,
    "apple,%s",
    o.string.pointer);
    } else {
    brcmf_dbg(INFO, "No ACPI module-instance\n");
    return;
    }
    status = acpi_evaluate_object(adev.handle, "RWCV", core::ptr::null_mut(), &buf);
    o = buf.pointer;
    if (!ACPI_FAILURE(status) && o && o.type == ACPI_TYPE_BUFFER &&
    o.buffer.length >= 2) {
    char *antenna_sku = devm_kzalloc(dev, 3, GFP_KERNEL);
    if (antenna_sku) {
    memcpy(antenna_sku, o.buffer.pointer, 2);
    brcmf_dbg(INFO, "ACPI RWCV data=%*phN antenna-sku=%s\n",
    (int)o.buffer.length, o.buffer.pointer,
    antenna_sku);
    settings.antenna_sku = antenna_sku;
    }
    kfree(buf.pointer);
    } else {
    brcmf_dbg(INFO, "No ACPI antenna-sku\n");
    }
    }
