//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/region_pmem.c
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
// Copyright(c) 2022 Intel Corporation. All rights reserved.

#[no_mangle]
unsafe extern "C" fn cxl_pmem_region_release(dev: *mut device) {
    static void cxl_pmem_region_release(struct device *dev)
    {
    struct cxl_pmem_region *cxlr_pmem = to_cxl_pmem_region(dev);
    int i;
    for (i = 0; i < cxlr_pmem.nr_mappings; i++) {
    struct cxl_memdev *cxlmd = cxlr_pmem.mapping[i].cxlmd;
    put_device(&cxlmd.dev);
    }
    kfree(cxlr_pmem);
    }
    static const struct attribute_group *cxl_pmem_region_attribute_groups[] = {
    &cxl_base_attribute_group,
    core::ptr::null_mut()
    };
    const struct device_type cxl_pmem_region_type = {
    .name = "cxl_pmem_region",
    .release = cxl_pmem_region_release,
    .groups = cxl_pmem_region_attribute_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn is_cxl_pmem_region(dev: *mut device) -> bool {
    bool is_cxl_pmem_region(struct device *dev)
    {
    return dev.type == &cxl_pmem_region_type;
    }
    EXPORT_SYMBOL_NS_GPL(is_cxl_pmem_region, "CXL");
    struct cxl_pmem_region *to_cxl_pmem_region(struct device *dev)
    {
    if (dev_WARN_ONCE(dev, !is_cxl_pmem_region(dev),
    "not a cxl_pmem_region device\n"))
    return core::ptr::null_mut();
    return container_of(dev, struct cxl_pmem_region, dev);
    }
    EXPORT_SYMBOL_NS_GPL(to_cxl_pmem_region, "CXL");
    static struct lock_class_key cxl_pmem_region_key;
#[no_mangle]
unsafe extern "C" fn cxl_pmem_region_alloc(cxlr: *mut cxl_region) -> c_int {
    static int cxl_pmem_region_alloc(struct cxl_region *cxlr)
    {
    struct cxl_region_params *p = &cxlr.params;
    struct cxl_nvdimm_bridge *cxl_nvb;
    struct device *dev;
    int i;
    guard(rwsem_read)(&cxl_rwsem.region);
    if (p.state != CXL_CONFIG_COMMIT)
    return -ENXIO;
    struct cxl_pmem_region *cxlr_pmem __free(kfree) =
    kzalloc_flex(*cxlr_pmem, mapping, p.nr_targets);
    if (!cxlr_pmem)
    return -ENOMEM;
    cxlr_pmem.hpa_range.start = p.res.start;
    cxlr_pmem.hpa_range.end = p.res.end;
// Snapshot the region configuration underneath the cxl_rwsem.region
    cxlr_pmem.nr_mappings = p.nr_targets;
    for (i = 0; i < p.nr_targets; i++) {
    struct cxl_endpoint_decoder *cxled = p.targets[i];
    struct cxl_memdev *cxlmd = cxled_to_memdev(cxled);
    struct cxl_pmem_region_mapping *m = &cxlr_pmem.mapping[i];
//
// Regions never span CXL root devices, so by definition the
// bridge for one device is the same for all.
//
    if (i == 0) {
    cxl_nvb = cxl_find_nvdimm_bridge(cxlmd.endpoint);
    if (!cxl_nvb)
    return -ENODEV;
    cxlr.cxl_nvb = cxl_nvb;
    }
    m.cxlmd = cxlmd;
    get_device(&cxlmd.dev);
    m.start = cxled.dpa_res.start;
    m.size = resource_size(cxled.dpa_res);
    m.position = i;
    }
    dev = &cxlr_pmem.dev;
    device_initialize(dev);
    lockdep_set_class(&dev.mutex, &cxl_pmem_region_key);
    device_set_pm_not_required(dev);
    dev.parent = &cxlr.dev;
    dev.bus = &cxl_bus_type;
    dev.type = &cxl_pmem_region_type;
    cxlr_pmem.cxlr = cxlr;
    cxlr.cxlr_pmem = no_free_ptr(cxlr_pmem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cxlr_pmem_unregister(_cxlr_pmem: *mut c_void) {
    static void cxlr_pmem_unregister(void *_cxlr_pmem)
    {
    struct cxl_pmem_region *cxlr_pmem = _cxlr_pmem;
    struct cxl_region *cxlr = cxlr_pmem.cxlr;
    struct cxl_nvdimm_bridge *cxl_nvb = cxlr.cxl_nvb;
//
// Either the bridge is in ->remove() context under the device_lock(),
// or cxlr_release_nvdimm() is cancelling the bridge's release action
// for @cxlr_pmem and doing it itself (while manually holding the bridge
// lock).
//
    device_lock_assert(&cxl_nvb.dev);
    cxlr.cxlr_pmem = core::ptr::null_mut();
    cxlr_pmem.cxlr = core::ptr::null_mut();
    device_unregister(&cxlr_pmem.dev);
    }
#[no_mangle]
unsafe extern "C" fn cxlr_release_nvdimm(_cxlr: *mut c_void) {
    static void cxlr_release_nvdimm(void *_cxlr)
    {
    struct cxl_region *cxlr = _cxlr;
    struct cxl_nvdimm_bridge *cxl_nvb = cxlr.cxl_nvb;
    scoped_guard(device, &cxl_nvb.dev) {
    if (cxlr.cxlr_pmem)
    devm_release_action(&cxl_nvb.dev, cxlr_pmem_unregister,
    cxlr.cxlr_pmem);
    }
    cxlr.cxl_nvb = core::ptr::null_mut();
    put_device(&cxl_nvb.dev);
    }
//
// devm_cxl_add_pmem_region() - add a cxl_region-to-nd_region bridge
// @cxlr: parent CXL region for this pmem region bridge device
//
// Return: 0 on success negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_cxl_add_pmem_region(cxlr: *mut cxl_region) -> c_int {
    int devm_cxl_add_pmem_region(struct cxl_region *cxlr)
    {
    struct cxl_pmem_region *cxlr_pmem;
    struct cxl_nvdimm_bridge *cxl_nvb;
    struct device *dev;
    int rc;
    rc = cxl_pmem_region_alloc(cxlr);
    if (rc)
    return rc;
    cxlr_pmem = cxlr.cxlr_pmem;
    cxl_nvb = cxlr.cxl_nvb;
    dev = &cxlr_pmem.dev;
    rc = dev_set_name(dev, "pmem_region%d", cxlr.id);
    if (rc)
    goto err;
    rc = device_add(dev);
    if (rc)
    goto err;
    dev_dbg(&cxlr.dev, "%s: register %s\n", dev_name(dev.parent),
    dev_name(dev));
    scoped_guard(device, &cxl_nvb.dev) {
    if (cxl_nvb.dev.driver)
    rc = devm_add_action_or_reset(&cxl_nvb.dev,
    cxlr_pmem_unregister,
    cxlr_pmem);
    else
    rc = -ENXIO;
    }
    if (rc)
    goto err_bridge;
// @cxlr carries a reference on @cxl_nvb until cxlr_release_nvdimm
    return devm_add_action_or_reset(&cxlr.dev, cxlr_release_nvdimm, cxlr);
    err:
    put_device(dev);
    err_bridge:
    put_device(&cxl_nvb.dev);
    cxlr.cxl_nvb = core::ptr::null_mut();
    return rc;
    }
