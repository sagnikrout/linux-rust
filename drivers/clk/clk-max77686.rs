//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-max77686.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// clk-max77686.c - Clock driver for Maxim 77686/MAX77802
//
// Copyright (C) 2012 Samsung Electornics
// Jonghwa Lee <jonghwa3.lee@samsung.com>

pub const MAX77802_CLOCK_LOW_JITTER_SHIFT: c_uint = 0x3;
    enum max77686_chip_name {
    CHIP_MAX77686,
    CHIP_MAX77802,
    CHIP_MAX77620,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77686_hw_clk_info {
    pub name: *const c_char,
    pub clk_reg: u32,
    pub clk_enable_mask: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77686_clk_init_data {
    pub regmap: *mut regmap,
    pub hw: clk_hw,
    pub clk_idata: clk_init_data,
    pub clk_info: *const max77686_hw_clk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77686_clk_driver_data {
    pub chip: enum max77686_chip_name,
    pub num_clks: usize,
    pub __counted_by(num_clks): max77686_clk_init_data max_clk_data[],
}

    static const struct
    max77686_hw_clk_info max77686_hw_clks_info[MAX77686_CLKS_NUM] = {
    [MAX77686_CLK_AP] = {
    .name = "32khz_ap",
    .clk_reg = MAX77686_REG_32KHZ,
    .clk_enable_mask = BIT(MAX77686_CLK_AP),
    },
    [MAX77686_CLK_CP] = {
    .name = "32khz_cp",
    .clk_reg = MAX77686_REG_32KHZ,
    .clk_enable_mask = BIT(MAX77686_CLK_CP),
    },
    [MAX77686_CLK_PMIC] = {
    .name = "32khz_pmic",
    .clk_reg = MAX77686_REG_32KHZ,
    .clk_enable_mask = BIT(MAX77686_CLK_PMIC),
    },
    };
    static const struct
    max77686_hw_clk_info max77802_hw_clks_info[MAX77802_CLKS_NUM] = {
    [MAX77802_CLK_32K_AP] = {
    .name = "32khz_ap",
    .clk_reg = MAX77802_REG_32KHZ,
    .clk_enable_mask = BIT(MAX77802_CLK_32K_AP),
    },
    [MAX77802_CLK_32K_CP] = {
    .name = "32khz_cp",
    .clk_reg = MAX77802_REG_32KHZ,
    .clk_enable_mask = BIT(MAX77802_CLK_32K_CP),
    },
    };
    static const struct
    max77686_hw_clk_info max77620_hw_clks_info[MAX77620_CLKS_NUM] = {
    [MAX77620_CLK_32K_OUT0] = {
    .name = "32khz_out0",
    .clk_reg = MAX77620_REG_CNFG1_32K,
    .clk_enable_mask = MAX77620_CNFG1_32K_OUT0_EN,
    },
    };
    static struct max77686_clk_init_data *to_max77686_clk_init_data(
    struct clk_hw *hw)
    {
    return container_of(hw, struct max77686_clk_init_data, hw);
    }
#[no_mangle]
unsafe extern "C" fn max77686_clk_prepare(hw: *mut clk_hw) -> c_int {
    static int max77686_clk_prepare(struct clk_hw *hw)
    {
    struct max77686_clk_init_data *max77686 = to_max77686_clk_init_data(hw);
    return regmap_update_bits(max77686.regmap, max77686.clk_info.clk_reg,
    max77686.clk_info.clk_enable_mask,
    max77686.clk_info.clk_enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn max77686_clk_unprepare(hw: *mut clk_hw) {
    static void max77686_clk_unprepare(struct clk_hw *hw)
    {
    struct max77686_clk_init_data *max77686 = to_max77686_clk_init_data(hw);
    regmap_update_bits(max77686.regmap, max77686.clk_info.clk_reg,
    max77686.clk_info.clk_enable_mask,
    ~max77686.clk_info.clk_enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn max77686_clk_is_prepared(hw: *mut clk_hw) -> c_int {
    static int max77686_clk_is_prepared(struct clk_hw *hw)
    {
    struct max77686_clk_init_data *max77686 = to_max77686_clk_init_data(hw);
    int ret;
    u32 val;
    ret = regmap_read(max77686.regmap, max77686.clk_info.clk_reg, &val);
    if (ret < 0)
    return -EINVAL;
    return val & max77686.clk_info.clk_enable_mask;
    }
    static unsigned long max77686_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return 32768;
    }
    static const struct clk_ops max77686_clk_ops = {
    .prepare	= max77686_clk_prepare,
    .unprepare	= max77686_clk_unprepare,
    .is_prepared	= max77686_clk_is_prepared,
    .recalc_rate	= max77686_recalc_rate,
    };
    static struct clk_hw *
    of_clk_max77686_get(struct of_phandle_args *clkspec, void *data)
    {
    struct max77686_clk_driver_data *drv_data = data;
    let mut idx: c_uint = clkspec.args[0];
    if (idx >= drv_data.num_clks) {
    pr_err("%s: invalid index %u\n", __func__, idx);
    return ERR_PTR(-EINVAL);
    }
    return &drv_data.max_clk_data[idx].hw;
    }
#[no_mangle]
unsafe extern "C" fn max77686_clk_probe(pdev: *mut platform_device) -> c_int {
    static int max77686_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *parent = dev.parent;
    const struct platform_device_id *id = platform_get_device_id(pdev);
    struct max77686_clk_driver_data *drv_data;
    const struct max77686_hw_clk_info *hw_clks;
    struct regmap *regmap;
    int i, ret, num_clks;
    switch (id.driver_data) {
    case CHIP_MAX77686:
    num_clks = MAX77686_CLKS_NUM;
    hw_clks = max77686_hw_clks_info;
    break;
    case CHIP_MAX77802:
    num_clks = MAX77802_CLKS_NUM;
    hw_clks = max77802_hw_clks_info;
    break;
    case CHIP_MAX77620:
    num_clks = MAX77620_CLKS_NUM;
    hw_clks = max77620_hw_clks_info;
    break;
    default:
    dev_err(dev, "Unknown Chip ID\n");
    return -EINVAL;
    }
    drv_data = devm_kzalloc(dev, struct_size(drv_data, max_clk_data, num_clks), GFP_KERNEL);
    if (!drv_data)
    return -ENOMEM;
    drv_data.num_clks = num_clks;
    drv_data.chip = id.driver_data;
    regmap = dev_get_regmap(parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(dev, "Failed to get rtc regmap\n");
    return -ENODEV;
    }
    for (i = 0; i < num_clks; i++) {
    struct max77686_clk_init_data *max_clk_data;
    const char *clk_name;
    max_clk_data = &drv_data.max_clk_data[i];
    max_clk_data.regmap = regmap;
    max_clk_data.clk_info = &hw_clks[i];
    max_clk_data.clk_idata.flags = hw_clks[i].flags;
    max_clk_data.clk_idata.ops = &max77686_clk_ops;
    if (parent.of_node &&
    !of_property_read_string_index(parent.of_node,
    "clock-output-names",
    i, &clk_name))
    max_clk_data.clk_idata.name = clk_name;
    else
    max_clk_data.clk_idata.name = hw_clks[i].name;
    max_clk_data.hw.init = &max_clk_data.clk_idata;
    ret = devm_clk_hw_register(dev, &max_clk_data.hw);
    if (ret) {
    dev_err(dev, "Failed to clock register: %d\n", ret);
    return ret;
    }
    ret = devm_clk_hw_register_clkdev(dev, &max_clk_data.hw,
    max_clk_data.clk_idata.name,
    core::ptr::null_mut());
    if (ret < 0) {
    dev_err(dev, "Failed to clkdev register: %d\n", ret);
    return ret;
    }
    }
    if (parent.of_node) {
    ret = devm_of_clk_add_hw_provider(dev, of_clk_max77686_get,
    drv_data);
    if (ret < 0) {
    dev_err(dev, "Failed to register OF clock provider: %d\n",
    ret);
    return ret;
    }
    }
// MAX77802: Enable low-jitter mode on the 32khz clocks.
    if (drv_data.chip == CHIP_MAX77802) {
    ret = regmap_update_bits(regmap, MAX77802_REG_32KHZ,
    1 << MAX77802_CLOCK_LOW_JITTER_SHIFT,
    1 << MAX77802_CLOCK_LOW_JITTER_SHIFT);
    if (ret < 0) {
    dev_err(dev, "Failed to config low-jitter: %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct platform_device_id max77686_clk_id[] = {
    { .name = "max77686-clk", .driver_data = CHIP_MAX77686 },
    { .name = "max77802-clk", .driver_data = CHIP_MAX77802 },
    { .name = "max77620-clock", .driver_data = CHIP_MAX77620 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max77686_clk_id);
    static struct platform_driver max77686_clk_driver = {
    .driver = {
    .name  = "max77686-clk",
    },
    .probe = max77686_clk_probe,
    .id_table = max77686_clk_id,
    };
    module_platform_driver(max77686_clk_driver);
    MODULE_DESCRIPTION("MAXIM 77686 Clock Driver");
    MODULE_AUTHOR("Jonghwa Lee <jonghwa3.lee@samsung.com>");
    MODULE_LICENSE("GPL");
