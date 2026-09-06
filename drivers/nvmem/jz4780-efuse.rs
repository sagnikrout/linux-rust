//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/jz4780-efuse.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// JZ4780 EFUSE Memory Support driver
//
// Copyright (c) 2017 PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>
// Copyright (c) 2020 H. Nikolaus Schaller <hns@goldelico.com>
//
// Currently supports JZ4780 efuse which has 8K programmable bit.
// Efuse is separated into seven segments as below:
//
// -----------------------------------------------------------------------
// | 64 bit | 128 bit | 128 bit | 3520 bit | 8 bit | 2296 bit | 2048 bit |
// -----------------------------------------------------------------------
//
// The rom itself is accessed using a 9 bit address line and an 8 word wide bus
// which reads/writes based on strobes. The strobe is configured in the config
// register and is based on number of cycles of the bus clock.
//
// Driver supports read only as the writes are done in the Factory.
//

// We read 32 byte chunks to avoid complexity in the driver.
pub const JZ_EFU_READ_SIZE: c_int = 32;
pub const EFUCTRL_ADDR_MASK: c_uint = 0x3FF;
pub const EFUCTRL_ADDR_SHIFT: c_int = 21;
pub const EFUCTRL_LEN_MASK: c_uint = 0x1F;
pub const EFUCTRL_LEN_SHIFT: c_int = 16;

pub const EFUCFG_RD_ADJ_MASK: c_uint = 0xF;
pub const EFUCFG_RD_ADJ_SHIFT: c_int = 20;
pub const EFUCFG_RD_STR_MASK: c_uint = 0xF;
pub const EFUCFG_RD_STR_SHIFT: c_int = 16;
pub const EFUCFG_WR_ADJ_MASK: c_uint = 0xF;
pub const EFUCFG_WR_ADJ_SHIFT: c_int = 12;
pub const EFUCFG_WR_STR_MASK: c_uint = 0xFFF;
pub const EFUCFG_WR_STR_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz4780_efuse {
    pub dev: *mut device,
    pub map: *mut regmap,
    pub clk: *mut clk,
}

// main entry point
    static int jz4780_efuse_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct jz4780_efuse *efuse = context;
    while (bytes > 0) {
    let mut start: usize = offset & ~(JZ_EFU_READ_SIZE - 1);
    size_t chunk = min(bytes, (start + JZ_EFU_READ_SIZE)
    - offset);
    char buf[JZ_EFU_READ_SIZE];
    unsigned int tmp;
    u32 ctrl;
    int ret;
    ctrl = (start << EFUCTRL_ADDR_SHIFT)
    | ((JZ_EFU_READ_SIZE - 1) << EFUCTRL_LEN_SHIFT)
    | EFUCTRL_RD_EN;
    regmap_update_bits(efuse.map, JZ_EFUCTRL,
    (EFUCTRL_ADDR_MASK << EFUCTRL_ADDR_SHIFT) |
    (EFUCTRL_LEN_MASK << EFUCTRL_LEN_SHIFT) |
    EFUCTRL_PG_EN | EFUCTRL_WR_EN |
    EFUCTRL_RD_EN,
    ctrl);
    ret = regmap_read_poll_timeout(efuse.map, JZ_EFUSTATE,
    tmp, tmp & EFUSTATE_RD_DONE,
    1 * MSEC_PER_SEC,
    50 * MSEC_PER_SEC);
    if (ret < 0) {
    dev_err(efuse.dev, "Time out while reading efuse data");
    return ret;
    }
    ret = regmap_bulk_read(efuse.map, JZ_EFUDATA(0),
    buf, JZ_EFU_READ_SIZE / sizeof(u32));
    if (ret < 0)
    return ret;
    memcpy(val, &buf[offset - start], chunk);
    val += chunk;
    offset += chunk;
    bytes -= chunk;
    }
    return 0;
    }
    static struct nvmem_config jz4780_efuse_nvmem_config = {
    .name = "jz4780-efuse",
    .size = 1024,
    .word_size = 1,
    .stride = 1,
    .owner = THIS_MODULE,
    .reg_read = jz4780_efuse_read,
    };
    static const struct regmap_config jz4780_efuse_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = JZ_EFUDATA(7),
    };
#[no_mangle]
unsafe extern "C" fn clk_disable_unprepare_helper(clock: *mut c_void) {
    static void clk_disable_unprepare_helper(void *clock)
    {
    clk_disable_unprepare(clock);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int jz4780_efuse_probe(struct platform_device *pdev)
    {
    struct nvmem_device *nvmem;
    struct jz4780_efuse *efuse;
    struct nvmem_config cfg;
    unsigned long clk_rate;
    unsigned long rd_adj;
    unsigned long rd_strobe;
    struct device *dev = &pdev.dev;
    void __iomem *regs;
    int ret;
    efuse = devm_kzalloc(dev, sizeof(*efuse), GFP_KERNEL);
    if (!efuse)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    efuse.map = devm_regmap_init_mmio(dev, regs,
    &jz4780_efuse_regmap_config);
    if (IS_ERR(efuse.map))
    return PTR_ERR(efuse.map);
    efuse.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(efuse.clk))
    return PTR_ERR(efuse.clk);
    ret = clk_prepare_enable(efuse.clk);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&pdev.dev,
    clk_disable_unprepare_helper,
    efuse.clk);
    if (ret < 0)
    return ret;
    clk_rate = clk_get_rate(efuse.clk);
    efuse.dev = dev;
//
// rd_adj and rd_strobe are 4 bit values
// conditions:
// bus clk_period * (rd_adj + 1) > 6.5ns
// bus clk_period * (rd_adj + 5 + rd_strobe) > 35ns
// i.e. rd_adj >= 6.5ns / clk_period
// i.e. rd_strobe >= 35 ns / clk_period - 5 - rd_adj + 1
// constants:
// 1 / 6.5ns == 153846154 Hz
// 1 / 35ns == 28571429 Hz
//
    rd_adj = clk_rate / 153846154;
    rd_strobe = clk_rate / 28571429 - 5 - rd_adj + 1;
    if (rd_adj > EFUCFG_RD_ADJ_MASK ||
    rd_strobe > EFUCFG_RD_STR_MASK) {
    dev_err(&pdev.dev, "Cannot set clock configuration\n");
    return -EINVAL;
    }
    regmap_update_bits(efuse.map, JZ_EFUCFG,
    (EFUCFG_RD_ADJ_MASK << EFUCFG_RD_ADJ_SHIFT) |
    (EFUCFG_RD_STR_MASK << EFUCFG_RD_STR_SHIFT),
    (rd_adj << EFUCFG_RD_ADJ_SHIFT) |
    (rd_strobe << EFUCFG_RD_STR_SHIFT));
    cfg = jz4780_efuse_nvmem_config;
    cfg.dev = &pdev.dev;
    cfg.priv = efuse;
    nvmem = devm_nvmem_register(dev, &cfg);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id jz4780_efuse_match[] = {
    { .compatible = "ingenic,jz4780-efuse" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, jz4780_efuse_match);
    static struct platform_driver jz4780_efuse_driver = {
    .probe  = jz4780_efuse_probe,
    .driver = {
    .name = "jz4780-efuse",
    .of_match_table = jz4780_efuse_match,
    },
    };
    module_platform_driver(jz4780_efuse_driver);
    MODULE_AUTHOR("PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>");
    MODULE_AUTHOR("H. Nikolaus Schaller <hns@goldelico.com>");
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_DESCRIPTION("Ingenic JZ4780 efuse driver");
    MODULE_LICENSE("GPL v2");
