//! Automatically rewritten from C to Rust
//! Source: tools/testing/cxl/mock_acpi.c
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
// Copyright(c) 2021 Intel Corporation. All rights reserved.

    struct acpi_device *to_cxl_host_bridge(struct device *host, struct device *dev)
    {
    int index;
    struct acpi_device *adev, *found = core::ptr::null_mut();
    struct cxl_mock_ops *ops = get_cxl_mock_ops(&index);
    if (ops && ops.is_mock_bridge(dev)) {
    found = ACPI_COMPANION(dev);
    goto out;
    }
    if (dev_is_platform(dev))
    goto out;
    adev = to_acpi_device(dev);
    if (!acpi_pci_find_root(adev.handle))
    goto out;
    if (strcmp(acpi_device_hid(adev), "ACPI0016") == 0) {
    found = adev;
    dev_dbg(host, "found host bridge %s\n", dev_name(&adev.dev));
    }
    out:
    put_cxl_mock_ops(index);
    return found;
    }
