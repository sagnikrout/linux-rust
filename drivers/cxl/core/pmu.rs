//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/pmu.c
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
// Copyright(c) 2023 Huawei. All rights reserved.

#[no_mangle]
unsafe extern "C" fn cxl_pmu_release(dev: *mut device) {
    static void cxl_pmu_release(struct device *dev)
    {
    struct cxl_pmu *pmu = to_cxl_pmu(dev);
    kfree(pmu);
    }
    const struct device_type cxl_pmu_type = {
    .name = "cxl_pmu",
    .release = cxl_pmu_release,
    };
#[no_mangle]
unsafe extern "C" fn remove_dev(dev: *mut c_void) {
    static void remove_dev(void *dev)
    {
    device_unregister(dev);
    }
    int devm_cxl_pmu_add(struct device *parent, struct cxl_pmu_regs *regs,
    int assoc_id, int index, enum cxl_pmu_type type)
    {
    struct cxl_pmu *pmu;
    struct device *dev;
    int rc;
    pmu = kzalloc_obj(*pmu);
    if (!pmu)
    return -ENOMEM;
    pmu.assoc_id = assoc_id;
    pmu.index = index;
    pmu.type = type;
    pmu.base = regs.pmu;
    dev = &pmu.dev;
    device_initialize(dev);
    device_set_pm_not_required(dev);
    dev.parent = parent;
    dev.bus = &cxl_bus_type;
    dev.type = &cxl_pmu_type;
    switch (pmu.type) {
    case CXL_PMU_MEMDEV:
    rc = dev_set_name(dev, "pmu_mem%d.%d", assoc_id, index);
    break;
    }
    if (rc)
    goto err;
    rc = device_add(dev);
    if (rc)
    goto err;
    return devm_add_action_or_reset(parent, remove_dev, dev);
    err:
    put_device(&pmu.dev);
    return rc;
    }
    EXPORT_SYMBOL_NS_GPL(devm_cxl_pmu_add, "CXL");
