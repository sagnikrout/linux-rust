//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/e820.c
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
// Copyright (c) 2015, Christoph Hellwig.
// Copyright (c) 2015, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn e820_pmem_remove(pdev: *mut platform_device) {
    static void e820_pmem_remove(struct platform_device *pdev)
    {
    struct nvdimm_bus *nvdimm_bus = platform_get_drvdata(pdev);
    nvdimm_bus_unregister(nvdimm_bus);
    }
#[no_mangle]
unsafe extern "C" fn e820_register_one(res: *mut resource, data: *mut c_void) -> c_int {
    static int e820_register_one(struct resource *res, void *data)
    {
    struct nd_region_desc ndr_desc;
    struct nvdimm_bus *nvdimm_bus = data;
    let mut nid: c_int = phys_to_target_node(res.start);
    memset(&ndr_desc, 0, sizeof(ndr_desc));
    ndr_desc.res = res;
    ndr_desc.numa_node = numa_map_to_online_node(nid);
    ndr_desc.target_node = nid;
    set_bit(ND_REGION_PAGEMAP, &ndr_desc.flags);
    if (!nvdimm_pmem_region_create(nvdimm_bus, &ndr_desc))
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn e820_pmem_probe(pdev: *mut platform_device) -> c_int {
    static int e820_pmem_probe(struct platform_device *pdev)
    {
    static struct nvdimm_bus_descriptor nd_desc;
    struct device *dev = &pdev.dev;
    struct nvdimm_bus *nvdimm_bus;
    let mut rc: c_int = -ENXIO;
    nd_desc.provider_name = "e820";
    nd_desc.module = THIS_MODULE;
    nvdimm_bus = nvdimm_bus_register(dev, &nd_desc);
    if (!nvdimm_bus)
    goto err;
    platform_set_drvdata(pdev, nvdimm_bus);
    rc = walk_iomem_res_desc(IORES_DESC_PERSISTENT_MEMORY_LEGACY,
    IORESOURCE_MEM, 0, -1, nvdimm_bus, e820_register_one);
    if (rc)
    goto err;
    return 0;
    err:
    nvdimm_bus_unregister(nvdimm_bus);
    dev_err(dev, "failed to register legacy persistent memory ranges\n");
    return rc;
    }
    static struct platform_driver e820_pmem_driver = {
    .probe = e820_pmem_probe,
    .remove = e820_pmem_remove,
    .driver = {
    .name = "e820_pmem",
    },
    };
    module_platform_driver(e820_pmem_driver);
    MODULE_ALIAS("platform:e820_pmem*");
    MODULE_DESCRIPTION("NVDIMM support for e820 type-12 memory");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Intel Corporation");
