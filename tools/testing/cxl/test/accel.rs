//! Automatically rewritten from C to Rust
//! Source: tools/testing/cxl/test/accel.c
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
// Copyright(c) 2026 Intel Corporation. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mock_cxl_accel {
    pub cxlds: cxl_dev_state,
    pub cxlmd: *mut cxl_memdev,
}

#[no_mangle]
unsafe extern "C" fn cxl_mock_accel_probe(pdev: *mut platform_device) -> c_int {
    static int cxl_mock_accel_probe(struct platform_device *pdev)
    {
    struct mock_cxl_accel *cxl_accel;
    struct device *dev = &pdev.dev;
    struct cxl_dev_state *cxlds;
    struct cxl_memdev *cxlmd;
    struct range mock_range;
    int rc;
    cxl_accel = devm_cxl_dev_state_create(&pdev.dev, CXL_DEVTYPE_DEVMEM,
    pdev.id + 1, 0,
    struct mock_cxl_accel, cxlds,
    false);
    if (!cxl_accel)
    return -ENOMEM;
    cxlds = &cxl_accel.cxlds;
    cxlds.media_ready = true;
    rc = cxl_set_capacity(cxlds, SZ_512M);
    if (rc)
    return rc;
    cxlmd = devm_cxl_probe_mem(cxlds, &mock_range);
    if (IS_ERR(cxlmd))
    return PTR_ERR(cxlmd);
    cxl_accel.cxlmd = cxlmd;
    dev_dbg(dev, "Probed mock accelerator with range %pra\n", &mock_range);
    return 0;
    }
    static const struct platform_device_id cxl_mock_accel_ids[] = {
    { .name = "cxl_type2_accel" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, cxl_mock_accel_ids);
    static struct platform_driver cxl_mock_accel_driver = {
    .probe = cxl_mock_accel_probe,
    .id_table = cxl_mock_accel_ids,
    .driver = {
    .name = KBUILD_MODNAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(cxl_mock_accel_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("cxl_test: accelerator device mock module");
    MODULE_IMPORT_NS("CXL");
