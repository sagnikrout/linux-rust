//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/sc27xx-efuse.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.

// PMIC global registers definition
pub const SC27XX_MODULE_EN: c_uint = 0xc08;
pub const SC2730_MODULE_EN: c_uint = 0x1808;

// Efuse controller registers definition
pub const SC27XX_EFUSE_GLB_CTRL: c_uint = 0x0;
pub const SC27XX_EFUSE_DATA_RD: c_uint = 0x4;
pub const SC27XX_EFUSE_DATA_WR: c_uint = 0x8;
pub const SC27XX_EFUSE_BLOCK_INDEX: c_uint = 0xc;
pub const SC27XX_EFUSE_MODE_CTRL: c_uint = 0x10;
pub const SC27XX_EFUSE_STATUS: c_uint = 0x14;
pub const SC27XX_EFUSE_WR_TIMING_CTRL: c_uint = 0x20;
pub const SC27XX_EFUSE_RD_TIMING_CTRL: c_uint = 0x24;
pub const SC27XX_EFUSE_EFUSE_DEB_CTRL: c_uint = 0x28;
// Mask definition for SC27XX_EFUSE_BLOCK_INDEX register

// Bits definitions for SC27XX_EFUSE_MODE_CTRL register

// Bits definitions for SC27XX_EFUSE_STATUS register

// Block number and block width (bytes) definitions
pub const SC27XX_EFUSE_BLOCK_MAX: c_int = 32;
pub const SC27XX_EFUSE_BLOCK_WIDTH: c_int = 2;
// Timeout (ms) for the trylock of hardware spinlocks
pub const SC27XX_EFUSE_HWLOCK_TIMEOUT: c_int = 5000;
// Timeout (us) of polling the status
pub const SC27XX_EFUSE_POLL_TIMEOUT: c_int = 3000000;
pub const SC27XX_EFUSE_POLL_DELAY_US: c_int = 10000;
//
// Since different PMICs of SC27xx series can have different
// address , we should save address in the device data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc27xx_efuse_variant_data {
    pub module_en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc27xx_efuse {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub hwlock: *mut hwspinlock,
    pub mutex: mutex,
    pub base: u32,
    pub var_data: *const sc27xx_efuse_variant_data,
}

    static const struct sc27xx_efuse_variant_data sc2731_edata = {
    .module_en = SC27XX_MODULE_EN,
    };
    static const struct sc27xx_efuse_variant_data sc2730_edata = {
    .module_en = SC2730_MODULE_EN,
    };
//
// On Spreadtrum platform, we have multi-subsystems will access the unique
// efuse controller, so we need one hardware spinlock to synchronize between
// the multiple subsystems.
//
#[no_mangle]
unsafe extern "C" fn sc27xx_efuse_lock(efuse: *mut sc27xx_efuse) -> c_int {
    static int sc27xx_efuse_lock(struct sc27xx_efuse *efuse)
    {
    int ret;
    mutex_lock(&efuse.mutex);
    ret = hwspin_lock_timeout_raw(efuse.hwlock,
    SC27XX_EFUSE_HWLOCK_TIMEOUT);
    if (ret) {
    dev_err(efuse.dev, "timeout to get the hwspinlock\n");
    mutex_unlock(&efuse.mutex);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_efuse_unlock(efuse: *mut sc27xx_efuse) {
    static void sc27xx_efuse_unlock(struct sc27xx_efuse *efuse)
    {
    hwspin_unlock_raw(efuse.hwlock);
    mutex_unlock(&efuse.mutex);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_efuse_poll_status(efuse: *mut sc27xx_efuse, bits: u32) -> c_int {
    static int sc27xx_efuse_poll_status(struct sc27xx_efuse *efuse, u32 bits)
    {
    int ret;
    u32 val;
    ret = regmap_read_poll_timeout(efuse.regmap,
    efuse.base + SC27XX_EFUSE_STATUS,
    val, (val & bits),
    SC27XX_EFUSE_POLL_DELAY_US,
    SC27XX_EFUSE_POLL_TIMEOUT);
    if (ret) {
    dev_err(efuse.dev, "timeout to update the efuse status\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_efuse_read(context: *mut c_void, offset: u32, val: *mut c_void, bytes: usize) -> c_int {
    static int sc27xx_efuse_read(void *context, u32 offset, void *val, size_t bytes)
    {
    struct sc27xx_efuse *efuse = context;
    u32 buf, blk_index = offset / SC27XX_EFUSE_BLOCK_WIDTH;
    let mut blk_offset: u32 = (offset % SC27XX_EFUSE_BLOCK_WIDTH) * BITS_PER_BYTE;
    int ret;
    if (blk_index > SC27XX_EFUSE_BLOCK_MAX ||
    bytes > SC27XX_EFUSE_BLOCK_WIDTH)
    return -EINVAL;
    ret = sc27xx_efuse_lock(efuse);
    if (ret)
    return ret;
// Enable the efuse controller.
    ret = regmap_update_bits(efuse.regmap, efuse.var_data.module_en,
    SC27XX_EFUSE_EN, SC27XX_EFUSE_EN);
    if (ret)
    goto unlock_efuse;
//
// Before reading, we should ensure the efuse controller is in
// standby state.
//
    ret = sc27xx_efuse_poll_status(efuse, SC27XX_EFUSE_STANDBY);
    if (ret)
    goto disable_efuse;
// Set the block address to be read.
    ret = regmap_write(efuse.regmap,
    efuse.base + SC27XX_EFUSE_BLOCK_INDEX,
    blk_index & SC27XX_EFUSE_BLOCK_MASK);
    if (ret)
    goto disable_efuse;
// Start reading process from efuse memory.
    ret = regmap_update_bits(efuse.regmap,
    efuse.base + SC27XX_EFUSE_MODE_CTRL,
    SC27XX_EFUSE_RD_START,
    SC27XX_EFUSE_RD_START);
    if (ret)
    goto disable_efuse;
//
// Polling the read done status to make sure the reading process
// is completed, that means the data can be read out now.
//
    ret = sc27xx_efuse_poll_status(efuse, SC27XX_EFUSE_RD_DONE);
    if (ret)
    goto disable_efuse;
// Read data from efuse memory.
    ret = regmap_read(efuse.regmap, efuse.base + SC27XX_EFUSE_DATA_RD,
    &buf);
    if (ret)
    goto disable_efuse;
// Clear the read done flag.
    ret = regmap_update_bits(efuse.regmap,
    efuse.base + SC27XX_EFUSE_MODE_CTRL,
    SC27XX_EFUSE_CLR_RDDONE,
    SC27XX_EFUSE_CLR_RDDONE);
    disable_efuse:
// Disable the efuse controller after reading.
    regmap_update_bits(efuse.regmap, efuse.var_data.module_en, SC27XX_EFUSE_EN, 0);
    unlock_efuse:
    sc27xx_efuse_unlock(efuse);
    if (!ret) {
    buf >>= blk_offset;
    memcpy(val, &buf, bytes);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int sc27xx_efuse_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut econfig: nvmem_config = { };
    struct nvmem_device *nvmem;
    struct sc27xx_efuse *efuse;
    int ret;
    efuse = devm_kzalloc(&pdev.dev, sizeof(*efuse), GFP_KERNEL);
    if (!efuse)
    return -ENOMEM;
    efuse.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!efuse.regmap) {
    dev_err(&pdev.dev, "failed to get efuse regmap\n");
    return -ENODEV;
    }
    ret = of_property_read_u32(np, "reg", &efuse.base);
    if (ret) {
    dev_err(&pdev.dev, "failed to get efuse base address\n");
    return ret;
    }
    ret = of_hwspin_lock_get_id(np, 0);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to get hwspinlock id\n");
    return ret;
    }
    efuse.hwlock = devm_hwspin_lock_request_specific(&pdev.dev, ret);
    if (!efuse.hwlock) {
    dev_err(&pdev.dev, "failed to request hwspinlock\n");
    return -ENXIO;
    }
    mutex_init(&efuse.mutex);
    efuse.dev = &pdev.dev;
    efuse.var_data = of_device_get_match_data(&pdev.dev);
    econfig.stride = 1;
    econfig.word_size = 1;
    econfig.read_only = true;
    econfig.name = "sc27xx-efuse";
    econfig.size = SC27XX_EFUSE_BLOCK_MAX * SC27XX_EFUSE_BLOCK_WIDTH;
    econfig.reg_read = sc27xx_efuse_read;
    econfig.priv = efuse;
    econfig.dev = &pdev.dev;
    econfig.add_legacy_fixed_of_cells = true;
    nvmem = devm_nvmem_register(&pdev.dev, &econfig);
    if (IS_ERR(nvmem)) {
    dev_err(&pdev.dev, "failed to register nvmem config\n");
    return PTR_ERR(nvmem);
    }
    return 0;
    }
    static const struct of_device_id sc27xx_efuse_of_match[] = {
    { .compatible = "sprd,sc2731-efuse", .data = &sc2731_edata},
    { .compatible = "sprd,sc2730-efuse", .data = &sc2730_edata},
    { }
    };
    MODULE_DEVICE_TABLE(of, sc27xx_efuse_of_match);
    static struct platform_driver sc27xx_efuse_driver = {
    .probe = sc27xx_efuse_probe,
    .driver = {
    .name = "sc27xx-efuse",
    .of_match_table = sc27xx_efuse_of_match,
    },
    };
    module_platform_driver(sc27xx_efuse_driver);
    MODULE_AUTHOR("Freeman Liu <freeman.liu@spreadtrum.com>");
    MODULE_DESCRIPTION("Spreadtrum SC27xx efuse driver");
    MODULE_LICENSE("GPL v2");
