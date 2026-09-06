//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/qcom-spmi-sdam.c
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
// Copyright (c) 2017, 2020-2021, The Linux Foundation. All rights reserved.
//

pub const SDAM_MEM_START: c_uint = 0x40;
pub const REGISTER_MAP_ID: c_uint = 0x40;
pub const REGISTER_MAP_VERSION: c_uint = 0x41;
pub const SDAM_SIZE: c_uint = 0x44;
pub const SDAM_PBS_TRIG_SET: c_uint = 0xE5;
pub const SDAM_PBS_TRIG_CLR: c_uint = 0xE6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdam_chip {
    pub regmap: *mut regmap,
    pub sdam_config: nvmem_config,
    pub base: c_uint,
    pub size: c_uint,
}

// read only register offsets
    static const u8 sdam_ro_map[] = {
    REGISTER_MAP_ID,
    REGISTER_MAP_VERSION,
    SDAM_SIZE
    };
    static bool sdam_is_valid(struct sdam_chip *sdam, unsigned int offset,
    size_t len)
    {
    let mut sdam_mem_end: c_uint = SDAM_MEM_START + sdam.size - 1;
    if (!len)
    return false;
    if (offset >= SDAM_MEM_START && offset <= sdam_mem_end
    && (offset + len - 1) <= sdam_mem_end)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(SDAM_PBS_TRIG_CLR: (offset == SDAM_PBS_TRIG_SET || offset ==) -> else {
    else if ((offset == SDAM_PBS_TRIG_SET || offset == SDAM_PBS_TRIG_CLR)
    && (len == 1))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sdam_is_ro(offset: c_uint, len: usize) -> bool {
    static bool sdam_is_ro(unsigned int offset, size_t len)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(sdam_ro_map); i++)
    if (offset <= sdam_ro_map[i] && (offset + len) > sdam_ro_map[i])
    return true;
    return false;
    }
    static int sdam_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct sdam_chip *sdam = priv;
    struct device *dev = sdam.sdam_config.dev;
    int rc;
    if (!sdam_is_valid(sdam, offset, bytes)) {
    dev_err(dev, "Invalid SDAM offset %#x len=%zd\n",
    offset, bytes);
    return -EINVAL;
    }
    rc = regmap_bulk_read(sdam.regmap, sdam.base + offset, val, bytes);
    if (rc < 0)
    dev_err(dev, "Failed to read SDAM offset %#x len=%zd, rc=%d\n",
    offset, bytes, rc);
    return rc;
    }
    static int sdam_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct sdam_chip *sdam = priv;
    struct device *dev = sdam.sdam_config.dev;
    int rc;
    if (!sdam_is_valid(sdam, offset, bytes)) {
    dev_err(dev, "Invalid SDAM offset %#x len=%zd\n",
    offset, bytes);
    return -EINVAL;
    }
    if (sdam_is_ro(offset, bytes)) {
    dev_err(dev, "Invalid write offset %#x len=%zd\n",
    offset, bytes);
    return -EINVAL;
    }
    rc = regmap_bulk_write(sdam.regmap, sdam.base + offset, val, bytes);
    if (rc < 0)
    dev_err(dev, "Failed to write SDAM offset %#x len=%zd, rc=%d\n",
    offset, bytes, rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sdam_probe(pdev: *mut platform_device) -> c_int {
    static int sdam_probe(struct platform_device *pdev)
    {
    struct sdam_chip *sdam;
    struct nvmem_device *nvmem;
    unsigned int val;
    int rc;
    sdam = devm_kzalloc(&pdev.dev, sizeof(*sdam), GFP_KERNEL);
    if (!sdam)
    return -ENOMEM;
    sdam.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!sdam.regmap) {
    dev_err(&pdev.dev, "Failed to get regmap handle\n");
    return -ENXIO;
    }
    rc = of_property_read_u32(pdev.dev.of_node, "reg", &sdam.base);
    if (rc < 0) {
    dev_err(&pdev.dev, "Failed to get SDAM base, rc=%d\n", rc);
    return -EINVAL;
    }
    rc = regmap_read(sdam.regmap, sdam.base + SDAM_SIZE, &val);
    if (rc < 0) {
    dev_err(&pdev.dev, "Failed to read SDAM_SIZE rc=%d\n", rc);
    return -EINVAL;
    }
    sdam.size = val * 32;
    sdam.sdam_config.dev = &pdev.dev;
    sdam.sdam_config.name = "spmi_sdam";
    sdam.sdam_config.id = NVMEM_DEVID_AUTO;
    sdam.sdam_config.owner = THIS_MODULE;
    sdam.sdam_config.add_legacy_fixed_of_cells = true;
    sdam.sdam_config.stride = 1;
    sdam.sdam_config.size = sdam.size;
    sdam.sdam_config.word_size = 1;
    sdam.sdam_config.reg_read = sdam_read;
    sdam.sdam_config.reg_write = sdam_write;
    sdam.sdam_config.priv = sdam;
    nvmem = devm_nvmem_register(&pdev.dev, &sdam.sdam_config);
    if (IS_ERR(nvmem)) {
    dev_err(&pdev.dev,
    "Failed to register SDAM nvmem device rc=%ld\n",
    PTR_ERR(nvmem));
    return -ENXIO;
    }
    dev_dbg(&pdev.dev,
    "SDAM base=%#x size=%u registered successfully\n",
    sdam.base, sdam.size);
    return 0;
    }
    static const struct of_device_id sdam_match_table[] = {
    { .compatible = "qcom,spmi-sdam" },
    {},
    };
    MODULE_DEVICE_TABLE(of, sdam_match_table);
    static struct platform_driver sdam_driver = {
    .driver = {
    .name = "qcom,spmi-sdam",
    .of_match_table = sdam_match_table,
    },
    .probe		= sdam_probe,
    };
    module_platform_driver(sdam_driver);
    MODULE_DESCRIPTION("QCOM SPMI SDAM driver");
    MODULE_LICENSE("GPL v2");
