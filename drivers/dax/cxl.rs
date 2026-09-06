//! Automatically rewritten from C to Rust
//! Source: drivers/dax/cxl.c
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
// Copyright(c) 2023 Intel Corporation. All rights reserved.

#[no_mangle]
unsafe extern "C" fn cxl_dax_region_probe(dev: *mut device) -> c_int {
    static int cxl_dax_region_probe(struct device *dev)
    {
    struct cxl_dax_region *cxlr_dax = to_cxl_dax_region(dev);
    let mut nid: c_int = phys_to_target_node(cxlr_dax.hpa_range.start);
    struct cxl_region *cxlr = cxlr_dax.cxlr;
    struct dax_region *dax_region;
    struct dev_dax_data data;
    if (nid == NUMA_NO_NODE)
    nid = memory_add_physaddr_to_nid(cxlr_dax.hpa_range.start);
    dax_region = alloc_dax_region(dev, cxlr.id, &cxlr_dax.hpa_range, nid,
    PMD_SIZE, IORESOURCE_DAX_KMEM);
    if (!dax_region)
    return -ENOMEM;
    data = (struct dev_dax_data) {
    .dax_region = dax_region,
    .id = -1,
    .size = range_len(&cxlr_dax.hpa_range),
    .memmap_on_memory = true,
    };
    return PTR_ERR_OR_ZERO(devm_create_dev_dax(&data));
    }
    static struct cxl_driver cxl_dax_region_driver = {
    .name = "cxl_dax_region",
    .probe = cxl_dax_region_probe,
    .id = CXL_DEVICE_DAX_REGION,
    .drv = {
    .suppress_bind_attrs = true,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn cxl_dax_region_driver_register(work: *mut work_struct) {
    static void cxl_dax_region_driver_register(struct work_struct *work)
    {
    dax_hmem_flush_work();
    cxl_driver_register(&cxl_dax_region_driver);
    }
    static DECLARE_WORK(cxl_dax_region_driver_work, cxl_dax_region_driver_register);
#[no_mangle]
unsafe extern "C" fn cxl_dax_region_init() -> int __init {
    static int __init cxl_dax_region_init(void)
    {
//
// Need to resolve a race with dax_hmem wanting to drive regions
// instead of CXL
//
    queue_work(system_long_wq, &cxl_dax_region_driver_work);
    return 0;
    }
    module_init(cxl_dax_region_init);
#[no_mangle]
unsafe extern "C" fn cxl_dax_region_exit() -> void __exit {
    static void __exit cxl_dax_region_exit(void)
    {
    flush_work(&cxl_dax_region_driver_work);
    cxl_driver_unregister(&cxl_dax_region_driver);
    }
    module_exit(cxl_dax_region_exit);
    MODULE_ALIAS_CXL(CXL_DEVICE_DAX_REGION);
    MODULE_DESCRIPTION("CXL DAX: direct access to CXL regions");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_IMPORT_NS("CXL");
