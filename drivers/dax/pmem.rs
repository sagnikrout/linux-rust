//! Automatically rewritten from C to Rust
//! Source: drivers/dax/pmem.c
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
// Copyright(c) 2016 - 2018 Intel Corporation. All rights reserved.

    static struct dev_dax *__dax_pmem_probe(struct device *dev)
    {
    struct range range;
    int rc, id, region_id;
    resource_size_t offset;
    struct nd_pfn_sb *pfn_sb;
    struct dev_dax_data data;
    struct nd_namespace_io *nsio;
    struct dax_region *dax_region;
    let mut pgmap: dev_pagemap = { };
    struct nd_namespace_common *ndns;
    struct nd_dax *nd_dax = to_nd_dax(dev);
    struct nd_pfn *nd_pfn = &nd_dax.nd_pfn;
    struct nd_region *nd_region = to_nd_region(dev.parent);
    ndns = nvdimm_namespace_common_probe(dev);
    if (IS_ERR(ndns))
    return ERR_CAST(ndns);
// parse the 'pfn' info block via ->rw_bytes
    rc = devm_namespace_enable(dev, ndns, nd_info_block_reserve());
    if (rc)
    return ERR_PTR(rc);
    rc = nvdimm_setup_pfn(nd_pfn, &pgmap);
    if (rc)
    return ERR_PTR(rc);
    devm_namespace_disable(dev, ndns);
// reserve the metadata area, device-dax will reserve the data
    pfn_sb = nd_pfn.pfn_sb;
    offset = le64_to_cpu(pfn_sb.dataoff);
    nsio = to_nd_namespace_io(&ndns.dev);
    if (!devm_request_mem_region(dev, nsio.res.start, offset,
    dev_name(&ndns.dev))) {
    dev_warn(dev, "could not reserve metadata\n");
    return ERR_PTR(-EBUSY);
    }
    rc = sscanf(dev_name(&ndns.dev), "namespace%d.%d", &region_id, &id);
    if (rc != 2)
    return ERR_PTR(-EINVAL);
// adjust the dax_region range to the start of data
    range = pgmap.range;
    range.start += offset;
    dax_region = alloc_dax_region(dev, region_id, &range,
    nd_region.target_node, le32_to_cpu(pfn_sb.align),
    IORESOURCE_DAX_STATIC);
    if (!dax_region)
    return ERR_PTR(-ENOMEM);
    data = (struct dev_dax_data) {
    .dax_region = dax_region,
    .id = id,
    .pgmap = &pgmap,
    .size = range_len(&range),
    .memmap_on_memory = false,
    };
    return devm_create_dev_dax(&data);
    }
#[no_mangle]
unsafe extern "C" fn dax_pmem_probe(dev: *mut device) -> c_int {
    static int dax_pmem_probe(struct device *dev)
    {
    return PTR_ERR_OR_ZERO(__dax_pmem_probe(dev));
    }
    static struct nd_device_driver dax_pmem_driver = {
    .probe = dax_pmem_probe,
    .drv = {
    .name = "dax_pmem",
    },
    .type = ND_DRIVER_DAX_PMEM,
    };
#[no_mangle]
unsafe extern "C" fn dax_pmem_init() -> int __init {
    static int __init dax_pmem_init(void)
    {
    return nd_driver_register(&dax_pmem_driver);
    }
    module_init(dax_pmem_init);
#[no_mangle]
unsafe extern "C" fn dax_pmem_exit() -> void __exit {
    static void __exit dax_pmem_exit(void)
    {
    driver_unregister(&dax_pmem_driver.drv);
    }
    module_exit(dax_pmem_exit);
    MODULE_DESCRIPTION("PMEM DAX: direct access to persistent memory");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_ALIAS_ND_DEVICE(ND_DEVICE_DAX_PMEM);
