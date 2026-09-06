//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/region_dax.c
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
// Copyright(c) 2022 Intel Corporation. All rights reserved.
// Copyright(c) 2026 Meta Technologies Inc. All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn cxl_dax_region_release(dev: *mut device) {
    static void cxl_dax_region_release(struct device *dev)
    {
    struct cxl_dax_region *cxlr_dax = to_cxl_dax_region(dev);
    kfree(cxlr_dax);
    }
    static const struct attribute_group *cxl_dax_region_attribute_groups[] = {
    &cxl_base_attribute_group,
    core::ptr::null_mut()
    };
    const struct device_type cxl_dax_region_type = {
    .name = "cxl_dax_region",
    .release = cxl_dax_region_release,
    .groups = cxl_dax_region_attribute_groups,
    };
#[no_mangle]
unsafe extern "C" fn is_cxl_dax_region(dev: *mut device) -> bool {
    static bool is_cxl_dax_region(struct device *dev)
    {
    return dev.type == &cxl_dax_region_type;
    }
    struct cxl_dax_region *to_cxl_dax_region(struct device *dev)
    {
    if (dev_WARN_ONCE(dev, !is_cxl_dax_region(dev),
    "not a cxl_dax_region device\n"))
    return core::ptr::null_mut();
    return container_of(dev, struct cxl_dax_region, dev);
    }
    EXPORT_SYMBOL_NS_GPL(to_cxl_dax_region, "CXL");
    static struct lock_class_key cxl_dax_region_key;
    static struct cxl_dax_region *cxl_dax_region_alloc(struct cxl_region *cxlr)
    {
    struct cxl_region_params *p = &cxlr.params;
    struct cxl_dax_region *cxlr_dax;
    struct device *dev;
    guard(rwsem_read)(&cxl_rwsem.region);
    if (p.state != CXL_CONFIG_COMMIT)
    return ERR_PTR(-ENXIO);
    cxlr_dax = kzalloc_obj(*cxlr_dax);
    if (!cxlr_dax)
    return ERR_PTR(-ENOMEM);
    cxlr_dax.hpa_range.start = p.res.start;
    cxlr_dax.hpa_range.end = p.res.end;
    dev = &cxlr_dax.dev;
    cxlr_dax.cxlr = cxlr;
    device_initialize(dev);
    lockdep_set_class(&dev.mutex, &cxl_dax_region_key);
    device_set_pm_not_required(dev);
    dev.parent = &cxlr.dev;
    dev.bus = &cxl_bus_type;
    dev.type = &cxl_dax_region_type;
    return cxlr_dax;
    }
#[no_mangle]
unsafe extern "C" fn cxlr_dax_unregister(_cxlr_dax: *mut c_void) {
    static void cxlr_dax_unregister(void *_cxlr_dax)
    {
    struct cxl_dax_region *cxlr_dax = _cxlr_dax;
    device_unregister(&cxlr_dax.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn devm_cxl_add_dax_region(cxlr: *mut cxl_region) -> c_int {
    int devm_cxl_add_dax_region(struct cxl_region *cxlr)
    {
    struct device *dev;
    int rc;
    struct cxl_dax_region *cxlr_dax __free(put_cxl_dax_region) =
    cxl_dax_region_alloc(cxlr);
    if (IS_ERR(cxlr_dax))
    return PTR_ERR(cxlr_dax);
    dev = &cxlr_dax.dev;
    rc = dev_set_name(dev, "dax_region%d", cxlr.id);
    if (rc)
    return rc;
    rc = device_add(dev);
    if (rc)
    return rc;
    dev_dbg(&cxlr.dev, "%s: register %s\n", dev_name(dev.parent),
    dev_name(dev));
    return devm_add_action_or_reset(&cxlr.dev, cxlr_dax_unregister,
    no_free_ptr(cxlr_dax));
    }
