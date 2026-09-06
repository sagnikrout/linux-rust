//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/ramdax.c
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
// Copyright (c) 2025, Mike Rapoport, Microsoft
//
// Based on e820 pmem driver:
// Copyright (c) 2015, Christoph Hellwig.
// Copyright (c) 2015, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ramdax_dimm {
    pub nvdimm: *mut nvdimm,
    pub label_area: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn ramdax_remove(pdev: *mut platform_device) {
    static void ramdax_remove(struct platform_device *pdev)
    {
    struct nvdimm_bus *nvdimm_bus = platform_get_drvdata(pdev);
    nvdimm_bus_unregister(nvdimm_bus);
    }
    static int ramdax_register_region(struct resource *res,
    struct nvdimm *nvdimm,
    struct nvdimm_bus *nvdimm_bus)
    {
    struct nd_mapping_desc mapping;
    struct nd_region_desc ndr_desc;
    struct nd_interleave_set *nd_set;
    let mut nid: c_int = phys_to_target_node(res.start);
    nd_set = kzalloc_obj(*nd_set);
    if (!nd_set)
    return -ENOMEM;
    nd_set.cookie1 = 0xcafebeefcafebeef;
    nd_set.cookie2 = nd_set.cookie1;
    nd_set.altcookie = nd_set.cookie1;
    memset(&mapping, 0, sizeof(mapping));
    mapping.nvdimm = nvdimm;
    mapping.start = 0;
    mapping.size = resource_size(res) - LABEL_AREA_SIZE;
    memset(&ndr_desc, 0, sizeof(ndr_desc));
    ndr_desc.res = res;
    ndr_desc.numa_node = numa_map_to_online_node(nid);
    ndr_desc.target_node = nid;
    ndr_desc.num_mappings = 1;
    ndr_desc.mapping = &mapping;
    ndr_desc.nd_set = nd_set;
    if (!nvdimm_pmem_region_create(nvdimm_bus, &ndr_desc))
    goto err_free_nd_set;
    return 0;
    err_free_nd_set:
    kfree(nd_set);
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn ramdax_register_dimm(res: *mut resource, data: *mut c_void) -> c_int {
    static int ramdax_register_dimm(struct resource *res, void *data)
    {
    let mut start: resource_size_t = res.start;
    let mut size: resource_size_t = resource_size(res);
    let mut flags: c_ulong = 0, cmd_mask = 0;
    struct nvdimm_bus *nvdimm_bus = data;
    struct ramdax_dimm *dimm;
    int err;
    dimm = kzalloc_obj(*dimm);
    if (!dimm)
    return -ENOMEM;
    dimm.label_area = memremap(start + size - LABEL_AREA_SIZE,
    LABEL_AREA_SIZE, MEMREMAP_WB);
    if (!dimm.label_area) {
    err = -ENOMEM;
    goto err_free_dimm;
    }
    set_bit(NDD_LABELING, &flags);
    set_bit(NDD_REGISTER_SYNC, &flags);
    set_bit(ND_CMD_GET_CONFIG_SIZE, &cmd_mask);
    set_bit(ND_CMD_GET_CONFIG_DATA, &cmd_mask);
    set_bit(ND_CMD_SET_CONFIG_DATA, &cmd_mask);
    dimm.nvdimm = nvdimm_create(nvdimm_bus, dimm,
// dimm_attribute_groups */ NULL,
    flags, cmd_mask, 0, core::ptr::null_mut());
    if (!dimm.nvdimm) {
    err = -ENOMEM;
    goto err_unmap_label;
    }
    err = ramdax_register_region(res, dimm.nvdimm, nvdimm_bus);
    if (err)
    goto err_remove_nvdimm;
    return 0;
    err_remove_nvdimm:
    nvdimm_delete(dimm.nvdimm);
    err_unmap_label:
    memunmap(dimm.label_area);
    err_free_dimm:
    kfree(dimm);
    return err;
    }
    static int ramdax_get_config_size(struct nvdimm *nvdimm, int buf_len,
    struct nd_cmd_get_config_size *cmd)
    {
    if (sizeof(*cmd) > buf_len)
    return -EINVAL;
// cmd = (struct nd_cmd_get_config_size){
    .status = 0,
    .config_size = LABEL_AREA_SIZE,
    .max_xfer = 8,
    };
    return 0;
    }
    static int ramdax_get_config_data(struct nvdimm *nvdimm, int buf_len,
    struct nd_cmd_get_config_data_hdr *cmd)
    {
    struct ramdax_dimm *dimm = nvdimm_provider_data(nvdimm);
    if (sizeof(*cmd) > buf_len)
    return -EINVAL;
    if (struct_size(cmd, out_buf, cmd.in_length) > buf_len)
    return -EINVAL;
    if (size_add(cmd.in_offset, cmd.in_length) > LABEL_AREA_SIZE)
    return -EINVAL;
    memcpy(cmd.out_buf, dimm.label_area + cmd.in_offset, cmd.in_length);
    return 0;
    }
    static int ramdax_set_config_data(struct nvdimm *nvdimm, int buf_len,
    struct nd_cmd_set_config_hdr *cmd)
    {
    struct ramdax_dimm *dimm = nvdimm_provider_data(nvdimm);
    if (sizeof(*cmd) > buf_len)
    return -EINVAL;
    if (struct_size(cmd, in_buf, cmd.in_length) > buf_len)
    return -EINVAL;
    if (size_add(cmd.in_offset, cmd.in_length) > LABEL_AREA_SIZE)
    return -EINVAL;
    memcpy(dimm.label_area + cmd.in_offset, cmd.in_buf, cmd.in_length);
    return 0;
    }
    static int ramdax_nvdimm_ctl(struct nvdimm *nvdimm, unsigned int cmd,
    void *buf, unsigned int buf_len)
    {
    let mut cmd_mask: c_ulong = nvdimm_cmd_mask(nvdimm);
    if (!test_bit(cmd, &cmd_mask))
    return -ENOTTY;
    switch (cmd) {
    case ND_CMD_GET_CONFIG_SIZE:
    return ramdax_get_config_size(nvdimm, buf_len, buf);
    case ND_CMD_GET_CONFIG_DATA:
    return ramdax_get_config_data(nvdimm, buf_len, buf);
    case ND_CMD_SET_CONFIG_DATA:
    return ramdax_set_config_data(nvdimm, buf_len, buf);
    default:
    return -ENOTTY;
    }
    }
    static int ramdax_ctl(struct nvdimm_bus_descriptor *nd_desc,
    struct nvdimm *nvdimm, unsigned int cmd, void *buf,
    unsigned int buf_len, int *cmd_rc)
    {
//
// No firmware response to translate, let the transport error
// code take precedence.
//
// cmd_rc = 0;
    if (!nvdimm)
    return -ENOTTY;
    return ramdax_nvdimm_ctl(nvdimm, cmd, buf, buf_len);
    }

    static const struct of_device_id ramdax_of_matches[] = {
    { .compatible = "pmem-region", },
    { },
    };

    static int ramdax_probe_of(struct platform_device *pdev,
    struct nvdimm_bus *bus, struct device_node *np)
    {
    int err;
    if (!of_match_node(ramdax_of_matches, np))
    return -ENODEV;
    for (int i = 0; i < pdev.num_resources; i++) {
    err = ramdax_register_dimm(&pdev.resource[i], bus);
    if (err)
    goto err_unregister;
    }
    return 0;
    err_unregister:
//
// FIXME: should we unregister the dimms that were registered
// successfully
//
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ramdax_probe(pdev: *mut platform_device) -> c_int {
    static int ramdax_probe(struct platform_device *pdev)
    {
    static struct nvdimm_bus_descriptor nd_desc;
    struct device *dev = &pdev.dev;
    struct nvdimm_bus *nvdimm_bus;
    struct device_node *np;
    let mut rc: c_int = -ENXIO;
    nd_desc.provider_name = "ramdax";
    nd_desc.module = THIS_MODULE;
    nd_desc.ndctl = ramdax_ctl;
    nvdimm_bus = nvdimm_bus_register(dev, &nd_desc);
    if (!nvdimm_bus)
    goto err;
    np = dev_of_node(&pdev.dev);
    if (np)
    rc = ramdax_probe_of(pdev, nvdimm_bus, np);
    else
    rc = walk_iomem_res_desc(IORES_DESC_PERSISTENT_MEMORY_LEGACY,
    IORESOURCE_MEM, 0, -1, nvdimm_bus,
    ramdax_register_dimm);
    if (rc)
    goto err;
    platform_set_drvdata(pdev, nvdimm_bus);
    return 0;
    err:
    nvdimm_bus_unregister(nvdimm_bus);
    return rc;
    }
    static struct platform_driver ramdax_driver = {
    .probe = ramdax_probe,
    .remove = ramdax_remove,
    .driver = {
    .name = "ramdax",
    },
    };
    module_platform_driver(ramdax_driver);
    MODULE_DESCRIPTION("NVDIMM support for e820 type-12 memory and OF pmem-region");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Microsoft Corporation");
