//! Automatically rewritten from C to Rust
//! Source: sound/soc/tegra/tegra20_das.c
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
// tegra20_das.c - Tegra20 DAS driver
//
// Author: Stephen Warren <swarren@nvidia.com>
// Copyright (C) 2010 - NVIDIA, Inc.
//

// Register TEGRA20_DAS_DAP_CTRL_SEL
pub const TEGRA20_DAS_DAP_CTRL_SEL: c_uint = 0x00;
pub const TEGRA20_DAS_DAP_CTRL_SEL_COUNT: c_int = 5;
pub const TEGRA20_DAS_DAP_CTRL_SEL_STRIDE: c_int = 4;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_MS_SEL_P: c_int = 31;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_MS_SEL_S: c_int = 1;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA1_TX_RX_P: c_int = 30;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA1_TX_RX_S: c_int = 1;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA2_TX_RX_P: c_int = 29;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA2_TX_RX_S: c_int = 1;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_P: c_int = 0;
pub const TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_S: c_int = 5;
// Values for field TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL
pub const TEGRA20_DAS_DAP_SEL_DAC1: c_int = 0;
pub const TEGRA20_DAS_DAP_SEL_DAC2: c_int = 1;
pub const TEGRA20_DAS_DAP_SEL_DAC3: c_int = 2;
pub const TEGRA20_DAS_DAP_SEL_DAP1: c_int = 16;
pub const TEGRA20_DAS_DAP_SEL_DAP2: c_int = 17;
pub const TEGRA20_DAS_DAP_SEL_DAP3: c_int = 18;
pub const TEGRA20_DAS_DAP_SEL_DAP4: c_int = 19;
pub const TEGRA20_DAS_DAP_SEL_DAP5: c_int = 20;
// Register TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL: c_uint = 0x40;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_COUNT: c_int = 3;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE: c_int = 4;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_P: c_int = 28;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_S: c_int = 4;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_P: c_int = 24;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_S: c_int = 4;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_P: c_int = 0;
pub const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_S: c_int = 4;
//
// Values for:
// TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL
// TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL
// TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL
//
pub const TEGRA20_DAS_DAC_SEL_DAP1: c_int = 0;
pub const TEGRA20_DAS_DAC_SEL_DAP2: c_int = 1;
pub const TEGRA20_DAS_DAC_SEL_DAP3: c_int = 2;
pub const TEGRA20_DAS_DAC_SEL_DAP4: c_int = 3;
pub const TEGRA20_DAS_DAC_SEL_DAP5: c_int = 4;
//
// Names/IDs of the DACs/DAPs.
//
pub const TEGRA20_DAS_DAP_ID_1: c_int = 0;
pub const TEGRA20_DAS_DAP_ID_2: c_int = 1;
pub const TEGRA20_DAS_DAP_ID_3: c_int = 2;
pub const TEGRA20_DAS_DAP_ID_4: c_int = 3;
pub const TEGRA20_DAS_DAP_ID_5: c_int = 4;
pub const TEGRA20_DAS_DAC_ID_1: c_int = 0;
pub const TEGRA20_DAS_DAC_ID_2: c_int = 1;
pub const TEGRA20_DAS_DAC_ID_3: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra20_das {
    pub regmap: *mut regmap,
}

//
// Terminology:
// DAS: Digital audio switch (HW module controlled by this driver)
// DAP: Digital audio port (port/pins on Tegra device)
// DAC: Digital audio controller (e.g. I2S or AC97 controller elsewhere)
//
// The Tegra DAS is a mux/cross-bar which can connect each DAP to a specific
// DAC, or another DAP. When DAPs are connected, one must be the master and
// one the slave. Each DAC allows selection of a specific DAP for input, to
// cater for the case where N DAPs are connected to 1 DAC for broadcast
// output.
//
// This driver is dumb; no attempt is made to ensure that a valid routing
// configuration is programmed.
//
#[no_mangle]
pub unsafe extern "C" fn tegra20_das_write(das: *mut tegra20_das, reg: u32, val: u32) {
    static inline void tegra20_das_write(struct tegra20_das *das, u32 reg, u32 val)
    {
    regmap_write(das.regmap, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_das_connect_dap_to_dac(das: *mut tegra20_das, dap: c_int, dac: c_int) {
    static void tegra20_das_connect_dap_to_dac(struct tegra20_das *das, int dap, int dac)
    {
    u32 addr;
    u32 reg;
    addr = TEGRA20_DAS_DAP_CTRL_SEL +
    (dap * TEGRA20_DAS_DAP_CTRL_SEL_STRIDE);
    reg = dac << TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_P;
    tegra20_das_write(das, addr, reg);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_das_connect_dac_to_dap(das: *mut tegra20_das, dac: c_int, dap: c_int) {
    static void tegra20_das_connect_dac_to_dap(struct tegra20_das *das, int dac, int dap)
    {
    u32 addr;
    u32 reg;
    addr = TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL +
    (dac * TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE);
    reg = dap << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_P |
    dap << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_P |
    dap << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_P;
    tegra20_das_write(das, addr, reg);
    }

    (TEGRA20_DAS_##name + \
    (TEGRA20_DAS_##name##_STRIDE * (TEGRA20_DAS_##name##_COUNT - 1)))
#[no_mangle]
unsafe extern "C" fn tegra20_das_wr_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tegra20_das_wr_rd_reg(struct device *dev, unsigned int reg)
    {
    if (reg <= LAST_REG(DAP_CTRL_SEL))
    return true;
    if ((reg >= TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL) &&
    (reg <= LAST_REG(DAC_INPUT_DATA_CLK_SEL)))
    return true;
    return false;
    }
    static const struct regmap_config tegra20_das_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = LAST_REG(DAC_INPUT_DATA_CLK_SEL),
    .writeable_reg = tegra20_das_wr_rd_reg,
    .readable_reg = tegra20_das_wr_rd_reg,
    .cache_type = REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn tegra20_das_probe(pdev: *mut platform_device) -> c_int {
    static int tegra20_das_probe(struct platform_device *pdev)
    {
    void __iomem *regs;
    struct tegra20_das *das;
    das = devm_kzalloc(&pdev.dev, sizeof(struct tegra20_das), GFP_KERNEL);
    if (!das)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    das.regmap = devm_regmap_init_mmio(&pdev.dev, regs,
    &tegra20_das_regmap_config);
    if (IS_ERR(das.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(das.regmap),
    "regmap init failed\n");
    tegra20_das_connect_dap_to_dac(das, TEGRA20_DAS_DAP_ID_1,
    TEGRA20_DAS_DAP_SEL_DAC1);
    tegra20_das_connect_dac_to_dap(das, TEGRA20_DAS_DAC_ID_1,
    TEGRA20_DAS_DAC_SEL_DAP1);
    tegra20_das_connect_dap_to_dac(das, TEGRA20_DAS_DAP_ID_3,
    TEGRA20_DAS_DAP_SEL_DAC3);
    tegra20_das_connect_dac_to_dap(das, TEGRA20_DAS_DAC_ID_3,
    TEGRA20_DAS_DAC_SEL_DAP3);
    return 0;
    }
    static const struct of_device_id tegra20_das_of_match[] = {
    { .compatible = "nvidia,tegra20-das", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tegra20_das_of_match);
    static struct platform_driver tegra20_das_driver = {
    .probe = tegra20_das_probe,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = tegra20_das_of_match,
    },
    };
    module_platform_driver(tegra20_das_driver);
    MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>");
    MODULE_DESCRIPTION("Tegra20 DAS driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
