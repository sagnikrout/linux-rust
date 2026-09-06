//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-palmas.c
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
// Clock driver for Palmas device.
//
// Copyright (c) 2013, NVIDIA Corporation.
// Copyright (c) 2013-2014 Texas Instruments, Inc.
//
// Author:	Laxman Dewangan <ldewangan@nvidia.com>
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//

pub const PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE1: c_int = 1;
pub const PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE2: c_int = 2;
pub const PALMAS_CLOCK_DT_EXT_CONTROL_NSLEEP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_clk32k_desc {
    pub clk_name: *const c_char,
    pub control_reg: c_uint,
    pub enable_mask: c_uint,
    pub sleep_mask: c_uint,
    pub sleep_reqstr_id: c_uint,
    pub delay: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_clock_info {
    pub dev: *mut device,
    pub hw: clk_hw,
    pub palmas: *mut palmas,
    pub clk_desc: *const palmas_clk32k_desc,
    pub ext_control_pin: c_int,
}

    static inline struct palmas_clock_info *to_palmas_clks_info(struct clk_hw *hw)
    {
    return container_of(hw, struct palmas_clock_info, hw);
    }
    static unsigned long palmas_clks_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return 32768;
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_prepare(hw: *mut clk_hw) -> c_int {
    static int palmas_clks_prepare(struct clk_hw *hw)
    {
    struct palmas_clock_info *cinfo = to_palmas_clks_info(hw);
    int ret;
    ret = palmas_update_bits(cinfo.palmas, PALMAS_RESOURCE_BASE,
    cinfo.clk_desc.control_reg,
    cinfo.clk_desc.enable_mask,
    cinfo.clk_desc.enable_mask);
    if (ret < 0)
    dev_err(cinfo.dev, "Reg 0x%02x update failed, %d\n",
    cinfo.clk_desc.control_reg, ret);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cinfo->clk_desc->delay) -> else {
    else if (cinfo.clk_desc.delay)
    udelay(cinfo.clk_desc.delay);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_unprepare(hw: *mut clk_hw) {
    static void palmas_clks_unprepare(struct clk_hw *hw)
    {
    struct palmas_clock_info *cinfo = to_palmas_clks_info(hw);
    int ret;
//
// Clock can be disabled through external pin if it is externally
// controlled.
//
    if (cinfo.ext_control_pin)
    return;
    ret = palmas_update_bits(cinfo.palmas, PALMAS_RESOURCE_BASE,
    cinfo.clk_desc.control_reg,
    cinfo.clk_desc.enable_mask, 0);
    if (ret < 0)
    dev_err(cinfo.dev, "Reg 0x%02x update failed, %d\n",
    cinfo.clk_desc.control_reg, ret);
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_is_prepared(hw: *mut clk_hw) -> c_int {
    static int palmas_clks_is_prepared(struct clk_hw *hw)
    {
    struct palmas_clock_info *cinfo = to_palmas_clks_info(hw);
    int ret;
    u32 val;
    if (cinfo.ext_control_pin)
    return 1;
    ret = palmas_read(cinfo.palmas, PALMAS_RESOURCE_BASE,
    cinfo.clk_desc.control_reg, &val);
    if (ret < 0) {
    dev_err(cinfo.dev, "Reg 0x%02x read failed, %d\n",
    cinfo.clk_desc.control_reg, ret);
    return ret;
    }
    return !!(val & cinfo.clk_desc.enable_mask);
    }
    static const struct clk_ops palmas_clks_ops = {
    .prepare	= palmas_clks_prepare,
    .unprepare	= palmas_clks_unprepare,
    .is_prepared	= palmas_clks_is_prepared,
    .recalc_rate	= palmas_clks_recalc_rate,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_clks_of_match_data {
    pub init: clk_init_data,
    pub desc: palmas_clk32k_desc,
}

    static const struct palmas_clks_of_match_data palmas_of_clk32kg = {
    .init = {
    .name = "clk32kg",
    .ops = &palmas_clks_ops,
    .flags = CLK_IGNORE_UNUSED,
    },
    .desc = {
    .clk_name = "clk32kg",
    .control_reg = PALMAS_CLK32KG_CTRL,
    .enable_mask = PALMAS_CLK32KG_CTRL_MODE_ACTIVE,
    .sleep_mask = PALMAS_CLK32KG_CTRL_MODE_SLEEP,
    .sleep_reqstr_id = PALMAS_EXTERNAL_REQSTR_ID_CLK32KG,
    .delay = 200,
    },
    };
    static const struct palmas_clks_of_match_data palmas_of_clk32kgaudio = {
    .init = {
    .name = "clk32kgaudio",
    .ops = &palmas_clks_ops,
    .flags = CLK_IGNORE_UNUSED,
    },
    .desc = {
    .clk_name = "clk32kgaudio",
    .control_reg = PALMAS_CLK32KGAUDIO_CTRL,
    .enable_mask = PALMAS_CLK32KG_CTRL_MODE_ACTIVE,
    .sleep_mask = PALMAS_CLK32KG_CTRL_MODE_SLEEP,
    .sleep_reqstr_id = PALMAS_EXTERNAL_REQSTR_ID_CLK32KGAUDIO,
    .delay = 200,
    },
    };
    static const struct of_device_id palmas_clks_of_match[] = {
    {
    .compatible = "ti,palmas-clk32kg",
    .data = &palmas_of_clk32kg,
    },
    {
    .compatible = "ti,palmas-clk32kgaudio",
    .data = &palmas_of_clk32kgaudio,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, palmas_clks_of_match);
    static void palmas_clks_get_clk_data(struct platform_device *pdev,
    struct palmas_clock_info *cinfo)
    {
    struct device_node *node = pdev.dev.of_node;
    unsigned int prop;
    int ret;
    ret = of_property_read_u32(node, "ti,external-sleep-control",
    &prop);
    if (ret)
    return;
    switch (prop) {
    case PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE1:
    prop = PALMAS_EXT_CONTROL_ENABLE1;
    break;
    case PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE2:
    prop = PALMAS_EXT_CONTROL_ENABLE2;
    break;
    case PALMAS_CLOCK_DT_EXT_CONTROL_NSLEEP:
    prop = PALMAS_EXT_CONTROL_NSLEEP;
    break;
    default:
    dev_warn(&pdev.dev, "%pOFn: Invalid ext control option: %u\n",
    node, prop);
    prop = 0;
    break;
    }
    cinfo.ext_control_pin = prop;
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_unprepare_ext_control(data: *mut c_void) {
    static void palmas_clks_unprepare_ext_control(void *data)
    {
    struct palmas_clock_info *cinfo = data;
    clk_unprepare(cinfo.hw.clk);
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_init_configure(cinfo: *mut palmas_clock_info) -> c_int {
    static int palmas_clks_init_configure(struct palmas_clock_info *cinfo)
    {
    int ret;
    ret = palmas_update_bits(cinfo.palmas, PALMAS_RESOURCE_BASE,
    cinfo.clk_desc.control_reg,
    cinfo.clk_desc.sleep_mask, 0);
    if (ret < 0) {
    dev_err(cinfo.dev, "Reg 0x%02x update failed, %d\n",
    cinfo.clk_desc.control_reg, ret);
    return ret;
    }
    if (cinfo.ext_control_pin) {
    ret = clk_prepare(cinfo.hw.clk);
    if (ret < 0) {
    dev_err(cinfo.dev, "Clock prep failed, %d\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(cinfo.dev,
    palmas_clks_unprepare_ext_control,
    cinfo);
    if (ret)
    return ret;
    ret = palmas_ext_control_req_config(cinfo.palmas,
    cinfo.clk_desc.sleep_reqstr_id,
    cinfo.ext_control_pin, true);
    if (ret < 0) {
    dev_err(cinfo.dev, "Ext config for %s failed, %d\n",
    cinfo.clk_desc.clk_name, ret);
    return ret;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn palmas_clks_probe(pdev: *mut platform_device) -> c_int {
    static int palmas_clks_probe(struct platform_device *pdev)
    {
    struct palmas *palmas = dev_get_drvdata(pdev.dev.parent);
    const struct palmas_clks_of_match_data *match_data;
    struct palmas_clock_info *cinfo;
    int ret;
    match_data = of_device_get_match_data(&pdev.dev);
    if (!match_data)
    return 1;
    cinfo = devm_kzalloc(&pdev.dev, sizeof(*cinfo), GFP_KERNEL);
    if (!cinfo)
    return -ENOMEM;
    palmas_clks_get_clk_data(pdev, cinfo);
    platform_set_drvdata(pdev, cinfo);
    cinfo.dev = &pdev.dev;
    cinfo.palmas = palmas;
    cinfo.clk_desc = &match_data.desc;
    cinfo.hw.init = &match_data.init;
    ret = devm_clk_hw_register(&pdev.dev, &cinfo.hw);
    if (ret) {
    dev_err(&pdev.dev, "Fail to register clock %s, %d\n",
    match_data.desc.clk_name, ret);
    return ret;
    }
    ret = palmas_clks_init_configure(cinfo);
    if (ret < 0) {
    dev_err(&pdev.dev, "Clock config failed, %d\n", ret);
    return ret;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_simple_get,
    &cinfo.hw);
    if (ret < 0)
    dev_err(&pdev.dev, "Fail to add clock driver, %d\n", ret);
    return ret;
    }
    static struct platform_driver palmas_clks_driver = {
    .driver = {
    .name = "palmas-clk",
    .of_match_table = palmas_clks_of_match,
    },
    .probe = palmas_clks_probe,
    };
    module_platform_driver(palmas_clks_driver);
    MODULE_DESCRIPTION("Clock driver for Palmas Series Devices");
    MODULE_ALIAS("platform:palmas-clk");
    MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
    MODULE_LICENSE("GPL v2");
