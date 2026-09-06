//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/dax_devs.c
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
// Copyright(c) 2013-2016 Intel Corporation. All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn nd_dax_release(dev: *mut device) {
    static void nd_dax_release(struct device *dev)
    {
    struct nd_region *nd_region = to_nd_region(dev.parent);
    struct nd_dax *nd_dax = to_nd_dax(dev);
    struct nd_pfn *nd_pfn = &nd_dax.nd_pfn;
    dev_dbg(dev, "trace\n");
    nd_detach_ndns(dev, &nd_pfn.ndns);
    ida_free(&nd_region.dax_ida, nd_pfn.id);
    kfree(nd_pfn.uuid);
    kfree(nd_dax);
    }
    struct nd_dax *to_nd_dax(struct device *dev)
    {
    struct nd_dax *nd_dax = container_of(dev, struct nd_dax, nd_pfn.dev);
    WARN_ON(!is_nd_dax(dev));
    return nd_dax;
    }
    EXPORT_SYMBOL(to_nd_dax);
    static const struct device_type nd_dax_device_type = {
    .name = "nd_dax",
    .release = nd_dax_release,
    .groups = nd_pfn_attribute_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn is_nd_dax(dev: *const device) -> bool {
    bool is_nd_dax(const struct device *dev)
    {
    return dev ? dev.type == &nd_dax_device_type : false;
    }
    EXPORT_SYMBOL(is_nd_dax);
    static struct nd_dax *nd_dax_alloc(struct nd_region *nd_region)
    {
    struct nd_pfn *nd_pfn;
    struct nd_dax *nd_dax;
    struct device *dev;
    nd_dax = kzalloc_obj(*nd_dax);
    if (!nd_dax)
    return core::ptr::null_mut();
    nd_pfn = &nd_dax.nd_pfn;
    nd_pfn.id = ida_alloc(&nd_region.dax_ida, GFP_KERNEL);
    if (nd_pfn.id < 0) {
    kfree(nd_dax);
    return core::ptr::null_mut();
    }
    dev = &nd_pfn.dev;
    dev_set_name(dev, "dax%d.%d", nd_region.id, nd_pfn.id);
    dev.type = &nd_dax_device_type;
    dev.parent = &nd_region.dev;
    return nd_dax;
    }
    struct device *nd_dax_create(struct nd_region *nd_region)
    {
    struct device *dev = core::ptr::null_mut();
    struct nd_dax *nd_dax;
    if (!is_memory(&nd_region.dev))
    return core::ptr::null_mut();
    nd_dax = nd_dax_alloc(nd_region);
    if (nd_dax)
    dev = nd_pfn_devinit(&nd_dax.nd_pfn, core::ptr::null_mut());
    nd_device_register(dev);
    return dev;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_dax_probe(dev: *mut device, ndns: *mut nd_namespace_common) -> c_int {
    int nd_dax_probe(struct device *dev, struct nd_namespace_common *ndns)
    {
    int rc;
    struct nd_dax *nd_dax;
    struct device *dax_dev;
    struct nd_pfn *nd_pfn;
    struct nd_pfn_sb *pfn_sb;
    struct nd_region *nd_region = to_nd_region(ndns.dev.parent);
    if (ndns.force_raw)
    return -ENODEV;
    switch (ndns.claim_class) {
    case NVDIMM_CCLASS_NONE:
    case NVDIMM_CCLASS_DAX:
    break;
    default:
    return -ENODEV;
    }
    scoped_guard(nvdimm_bus, &ndns.dev) {
    nd_dax = nd_dax_alloc(nd_region);
    dax_dev = nd_dax_devinit(nd_dax, ndns);
    if (!dax_dev)
    return -ENOMEM;
    }
    pfn_sb = devm_kmalloc(dev, sizeof(*pfn_sb), GFP_KERNEL);
    nd_pfn = &nd_dax.nd_pfn;
    nd_pfn.pfn_sb = pfn_sb;
    rc = nd_pfn_validate(nd_pfn, DAX_SIG);
    dev_dbg(dev, "dax: %s\n", rc == 0 ? dev_name(dax_dev) : "<none>");
    if (rc < 0) {
    nd_detach_ndns(dax_dev, &nd_pfn.ndns);
    put_device(dax_dev);
    } else
    nd_device_register(dax_dev);
    return rc;
    }
    EXPORT_SYMBOL(nd_dax_probe);
