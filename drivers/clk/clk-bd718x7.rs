//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-bd718x7.c
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
// Copyright (C) 2018 ROHM Semiconductors

// clk control registers
// BD71815
pub const BD71815_REG_OUT32K: c_uint = 0x1d;
// BD71828
pub const BD71828_REG_OUT32K: c_uint = 0x4B;
// BD71837 and BD71847
pub const BD718XX_REG_OUT32K: c_uint = 0x2E;
// BD72720
pub const BD72720_REG_OUT32K: c_uint = 0x9a;
//
// BD71837, BD71847, and BD71828 all use bit [0] to clk output control
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd718xx_clk {
    pub hw: clk_hw,
    pub reg: u8,
    pub mask: u8,
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn bd71837_clk_set(c: *mut bd718xx_clk, status: c_uint) -> c_int {
    static int bd71837_clk_set(struct bd718xx_clk *c, unsigned int status)
    {
    return regmap_update_bits(c.regmap, c.reg, c.mask, status);
    }
#[no_mangle]
unsafe extern "C" fn bd71837_clk_disable(hw: *mut clk_hw) {
    static void bd71837_clk_disable(struct clk_hw *hw)
    {
    int rv;
    struct bd718xx_clk *c = container_of(hw, struct bd718xx_clk, hw);
    rv = bd71837_clk_set(c, 0);
    if (rv)
    dev_dbg(&c.pdev.dev, "Failed to disable 32K clk (%d)\n", rv);
    }
#[no_mangle]
unsafe extern "C" fn bd71837_clk_enable(hw: *mut clk_hw) -> c_int {
    static int bd71837_clk_enable(struct clk_hw *hw)
    {
    struct bd718xx_clk *c = container_of(hw, struct bd718xx_clk, hw);
    return bd71837_clk_set(c, 0xffffffff);
    }
#[no_mangle]
unsafe extern "C" fn bd71837_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int bd71837_clk_is_enabled(struct clk_hw *hw)
    {
    int enabled;
    int rval;
    struct bd718xx_clk *c = container_of(hw, struct bd718xx_clk, hw);
    rval = regmap_read(c.regmap, c.reg, &enabled);
    if (rval)
    return rval;
    return enabled & c.mask;
    }
    static const struct clk_ops bd71837_clk_ops = {
    .prepare = &bd71837_clk_enable,
    .unprepare = &bd71837_clk_disable,
    .is_prepared = &bd71837_clk_is_enabled,
    };
#[no_mangle]
unsafe extern "C" fn bd71837_clk_probe(pdev: *mut platform_device) -> c_int {
    static int bd71837_clk_probe(struct platform_device *pdev)
    {
    struct bd718xx_clk *c;
    let mut rval: c_int = -ENOMEM;
    const char *parent_clk;
    struct device *parent = pdev.dev.parent;
    struct clk_init_data init = {
    .name = "bd718xx-32k-out",
    .ops = &bd71837_clk_ops,
    };
    let mut chip: enum rohm_chip_type = platform_get_device_id(pdev).driver_data;
    c = devm_kzalloc(&pdev.dev, sizeof(*c), GFP_KERNEL);
    if (!c)
    return -ENOMEM;
    c.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!c.regmap)
    return -ENODEV;
    init.num_parents = 1;
    parent_clk = of_clk_get_parent_name(parent.of_node, 0);
    init.parent_names = &parent_clk;
    if (!parent_clk) {
    dev_err(&pdev.dev, "No parent clk found\n");
    return -EINVAL;
    }
    switch (chip) {
    case ROHM_CHIP_TYPE_BD71837:
    case ROHM_CHIP_TYPE_BD71847:
    c.reg = BD718XX_REG_OUT32K;
    c.mask = CLK_OUT_EN_MASK;
    break;
    case ROHM_CHIP_TYPE_BD71828:
    c.reg = BD71828_REG_OUT32K;
    c.mask = CLK_OUT_EN_MASK;
    break;
    case ROHM_CHIP_TYPE_BD71815:
    c.reg = BD71815_REG_OUT32K;
    c.mask = CLK_OUT_EN_MASK;
    break;
    case ROHM_CHIP_TYPE_BD72720:
    c.reg = BD72720_REG_OUT32K;
    c.mask = CLK_OUT_EN_MASK;
    break;
    default:
    dev_err(&pdev.dev, "Unknown clk chip\n");
    return -EINVAL;
    }
    c.pdev = pdev;
    c.hw.init = &init;
    of_property_read_string_index(parent.of_node,
    "clock-output-names", 0, &init.name);
    rval = devm_clk_hw_register(&pdev.dev, &c.hw);
    if (rval) {
    dev_err(&pdev.dev, "failed to register 32K clk");
    return rval;
    }
    rval = devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_simple_get,
    &c.hw);
    if (rval)
    dev_err(&pdev.dev, "adding clk provider failed\n");
    return rval;
    }
    static const struct platform_device_id bd718x7_clk_id[] = {
    { .name = "bd71837-clk", .driver_data = ROHM_CHIP_TYPE_BD71837 },
    { .name = "bd71847-clk", .driver_data = ROHM_CHIP_TYPE_BD71847 },
    { .name = "bd71828-clk", .driver_data = ROHM_CHIP_TYPE_BD71828 },
    { .name = "bd71815-clk", .driver_data = ROHM_CHIP_TYPE_BD71815 },
    { .name = "bd72720-clk", .driver_data = ROHM_CHIP_TYPE_BD72720 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, bd718x7_clk_id);
    static struct platform_driver bd71837_clk = {
    .driver = {
    .name = "bd718xx-clk",
    },
    .probe = bd71837_clk_probe,
    .id_table = bd718x7_clk_id,
    };
    module_platform_driver(bd71837_clk);
    MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
    MODULE_DESCRIPTION("BD718(15/18/28/37/47/50) and BD72720 chip clk driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:bd718xx-clk");
