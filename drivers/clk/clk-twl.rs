//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-twl.c
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
//
// Clock driver for twl device.
//
// inspired by the driver for the Palmas device
//

pub const VREG_STATE: c_int = 2;
pub const VREG_GRP: c_int = 0;
pub const TWL6030_CFG_STATE_OFF: c_uint = 0x00;
pub const TWL6030_CFG_STATE_ON: c_uint = 0x01;
pub const TWL6030_CFG_STATE_MASK: c_uint = 0x03;
pub const TWL6030_CFG_STATE_GRP_SHIFT: c_int = 5;
pub const TWL6030_CFG_STATE_APP_SHIFT: c_int = 2;

    TWL6030_CFG_STATE_APP_SHIFT)

    enum twl_type {
    TWL_TYPE_6030,
    TWL_TYPE_6032,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl_clock_info {
    pub dev: *mut device,
    pub type: enum twl_type,
    pub base: u8,
    pub hw: clk_hw,
}

    static inline int
    twlclk_read(struct twl_clock_info *info, unsigned int slave_subgp,
    unsigned int offset)
    {
    u8 value;
    int status;
    status = twl_i2c_read_u8(slave_subgp, &value,
    info.base + offset);
    return (status < 0) ? status : value;
    }
    static inline int
    twlclk_write(struct twl_clock_info *info, unsigned int slave_subgp,
    unsigned int offset, u8 value)
    {
    return twl_i2c_write_u8(slave_subgp, value,
    info.base + offset);
    }
    static inline struct twl_clock_info *to_twl_clks_info(struct clk_hw *hw)
    {
    return container_of(hw, struct twl_clock_info, hw);
    }
    static unsigned long twl_clks_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return 32768;
    }
#[no_mangle]
unsafe extern "C" fn twl6032_clks_prepare(hw: *mut clk_hw) -> c_int {
    static int twl6032_clks_prepare(struct clk_hw *hw)
    {
    struct twl_clock_info *cinfo = to_twl_clks_info(hw);
    if (cinfo.type == TWL_TYPE_6030) {
    int grp;
    grp = twlclk_read(cinfo, TWL_MODULE_PM_RECEIVER, VREG_GRP);
    if (grp < 0)
    return grp;
    return twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE,
    grp << TWL6030_CFG_STATE_GRP_SHIFT |
    TWL6030_CFG_STATE_ON);
    }
    return twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE,
    TWL6030_CFG_STATE_ON);
    }
#[no_mangle]
unsafe extern "C" fn twl6032_clks_unprepare(hw: *mut clk_hw) {
    static void twl6032_clks_unprepare(struct clk_hw *hw)
    {
    struct twl_clock_info *cinfo = to_twl_clks_info(hw);
    int ret;
    if (cinfo.type == TWL_TYPE_6030)
    ret = twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE,
    ALL_GRP << TWL6030_CFG_STATE_GRP_SHIFT |
    TWL6030_CFG_STATE_OFF);
    else
    ret = twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE,
    TWL6030_CFG_STATE_OFF);
    if (ret < 0)
    dev_err(cinfo.dev, "clk unprepare failed\n");
    }
    static const struct clk_ops twl6032_clks_ops = {
    .prepare	= twl6032_clks_prepare,
    .unprepare	= twl6032_clks_unprepare,
    .recalc_rate	= twl_clks_recalc_rate,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl_clks_data {
    pub init: clk_init_data,
    pub base: u8,
}

    static const struct twl_clks_data twl6032_clks[] = {
    {
    .init = {
    .name = "clk32kg",
    .ops = &twl6032_clks_ops,
    .flags = CLK_IGNORE_UNUSED,
    },
    .base = 0x8C,
    },
    {
    .init = {
    .name = "clk32kaudio",
    .ops = &twl6032_clks_ops,
    .flags = CLK_IGNORE_UNUSED,
    },
    .base = 0x8F,
    },
    {
// sentinel
    }
    };
#[no_mangle]
unsafe extern "C" fn twl_clks_probe(pdev: *mut platform_device) -> c_int {
    static int twl_clks_probe(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    const struct twl_clks_data *hw_data;
    struct twl_clock_info *cinfo;
    int ret;
    int i;
    int count;
    hw_data = twl6032_clks;
    for (count = 0; hw_data[count].init.name; count++)
    ;
    clk_data = devm_kzalloc(&pdev.dev,
    struct_size(clk_data, hws, count),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = count;
    cinfo = devm_kcalloc(&pdev.dev, count, sizeof(*cinfo), GFP_KERNEL);
    if (!cinfo)
    return -ENOMEM;
    for (i = 0; i < count; i++) {
    cinfo[i].base = hw_data[i].base;
    cinfo[i].dev = &pdev.dev;
    cinfo[i].type = platform_get_device_id(pdev).driver_data;
    cinfo[i].hw.init = &hw_data[i].init;
    ret = devm_clk_hw_register(&pdev.dev, &cinfo[i].hw);
    if (ret) {
    return dev_err_probe(&pdev.dev, ret,
    "Fail to register clock %s\n",
    hw_data[i].init.name);
    }
    clk_data.hws[i] = &cinfo[i].hw;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev,
    of_clk_hw_onecell_get, clk_data);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Fail to add clock driver\n");
    return 0;
    }
    static const struct platform_device_id twl_clks_id[] = {
    {
    .name = "twl6030-clk",
    .driver_data = TWL_TYPE_6030,
    }, {
    .name = "twl6032-clk",
    .driver_data = TWL_TYPE_6032,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(platform, twl_clks_id);
    static struct platform_driver twl_clks_driver = {
    .driver = {
    .name = "twl-clk",
    },
    .probe = twl_clks_probe,
    .id_table = twl_clks_id,
    };
    module_platform_driver(twl_clks_driver);
    MODULE_DESCRIPTION("Clock driver for TWL Series Devices");
    MODULE_LICENSE("GPL");
