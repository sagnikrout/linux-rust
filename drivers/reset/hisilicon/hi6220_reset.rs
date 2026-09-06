//! Automatically rewritten from C to Rust
//! Source: drivers/reset/hisilicon/hi6220_reset.c
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
// Hisilicon Hi6220 reset controller driver
//
// Copyright (c) 2016 Linaro Limited.
// Copyright (c) 2015-2016 HiSilicon Limited.
//
// Author: Feng Chen <puck.chen@hisilicon.com>
//

pub const PERIPH_ASSERT_OFFSET: c_uint = 0x300;
pub const PERIPH_DEASSERT_OFFSET: c_uint = 0x304;
pub const PERIPH_MAX_INDEX: c_uint = 0x509;
pub const SC_MEDIA_RSTEN: c_uint = 0x052C;
pub const SC_MEDIA_RSTDIS: c_uint = 0x0530;
pub const MEDIA_MAX_INDEX: c_int = 8;

    enum hi6220_reset_ctrl_type {
    PERIPHERAL,
    MEDIA,
    AO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_reset_data {
    pub rc_dev: reset_controller_dev,
    pub regmap: *mut regmap,
}

    static int hi6220_peripheral_assert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    let mut bank: u32 = idx >> 8;
    let mut offset: u32 = idx & 0xff;
    let mut reg: u32 = PERIPH_ASSERT_OFFSET + bank * 0x10;
    return regmap_write(regmap, reg, BIT(offset));
    }
    static int hi6220_peripheral_deassert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    let mut bank: u32 = idx >> 8;
    let mut offset: u32 = idx & 0xff;
    let mut reg: u32 = PERIPH_DEASSERT_OFFSET + bank * 0x10;
    return regmap_write(regmap, reg, BIT(offset));
    }
    static const struct reset_control_ops hi6220_peripheral_reset_ops = {
    .assert = hi6220_peripheral_assert,
    .deassert = hi6220_peripheral_deassert,
    };
    static int hi6220_media_assert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    return regmap_write(regmap, SC_MEDIA_RSTEN, BIT(idx));
    }
    static int hi6220_media_deassert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    return regmap_write(regmap, SC_MEDIA_RSTDIS, BIT(idx));
    }
    static const struct reset_control_ops hi6220_media_reset_ops = {
    .assert = hi6220_media_assert,
    .deassert = hi6220_media_deassert,
    };
pub const AO_SCTRL_SC_PW_CLKEN0: c_uint = 0x800;
pub const AO_SCTRL_SC_PW_CLKDIS0: c_uint = 0x804;
pub const AO_SCTRL_SC_PW_RSTEN0: c_uint = 0x810;
pub const AO_SCTRL_SC_PW_RSTDIS0: c_uint = 0x814;
pub const AO_SCTRL_SC_PW_ISOEN0: c_uint = 0x820;
pub const AO_SCTRL_SC_PW_ISODIS0: c_uint = 0x824;
pub const AO_MAX_INDEX: c_int = 12;
    static int hi6220_ao_assert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    int ret;
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_RSTEN0, BIT(idx));
    if (ret)
    return ret;
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_ISOEN0, BIT(idx));
    if (ret)
    return ret;
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_CLKDIS0, BIT(idx));
    return ret;
    }
    static int hi6220_ao_deassert(struct reset_controller_dev *rc_dev,
    unsigned long idx)
    {
    struct hi6220_reset_data *data = to_reset_data(rc_dev);
    struct regmap *regmap = data.regmap;
    int ret;
//
// It was suggested to disable isolation before enabling
// the clocks and deasserting reset, to avoid glitches.
// But this order is preserved to keep it matching the
// vendor code.
//
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_RSTDIS0, BIT(idx));
    if (ret)
    return ret;
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_ISODIS0, BIT(idx));
    if (ret)
    return ret;
    ret = regmap_write(regmap, AO_SCTRL_SC_PW_CLKEN0, BIT(idx));
    return ret;
    }
    static const struct reset_control_ops hi6220_ao_reset_ops = {
    .assert = hi6220_ao_assert,
    .deassert = hi6220_ao_deassert,
    };
#[no_mangle]
unsafe extern "C" fn hi6220_reset_probe(pdev: *mut platform_device) -> c_int {
    static int hi6220_reset_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    enum hi6220_reset_ctrl_type type;
    struct hi6220_reset_data *data;
    struct regmap *regmap;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    type = (uintptr_t)of_device_get_match_data(dev);
    regmap = syscon_node_to_regmap(np);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to get reset controller regmap\n");
    return PTR_ERR(regmap);
    }
    data.regmap = regmap;
    data.rc_dev.of_node = np;
    if (type == MEDIA) {
    data.rc_dev.ops = &hi6220_media_reset_ops;
    data.rc_dev.nr_resets = MEDIA_MAX_INDEX;
    } else if (type == PERIPHERAL) {
    data.rc_dev.ops = &hi6220_peripheral_reset_ops;
    data.rc_dev.nr_resets = PERIPH_MAX_INDEX;
    } else {
    data.rc_dev.ops = &hi6220_ao_reset_ops;
    data.rc_dev.nr_resets = AO_MAX_INDEX;
    }
    return reset_controller_register(&data.rc_dev);
    }
    static const struct of_device_id hi6220_reset_match[] = {
    {
    .compatible = "hisilicon,hi6220-sysctrl",
    .data = (void *)PERIPHERAL,
    },
    {
    .compatible = "hisilicon,hi6220-mediactrl",
    .data = (void *)MEDIA,
    },
    {
    .compatible = "hisilicon,hi6220-aoctrl",
    .data = (void *)AO,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, hi6220_reset_match);
    static struct platform_driver hi6220_reset_driver = {
    .probe = hi6220_reset_probe,
    .driver = {
    .name = "reset-hi6220",
    .of_match_table = hi6220_reset_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn hi6220_reset_init() -> int __init {
    static int __init hi6220_reset_init(void)
    {
    return platform_driver_register(&hi6220_reset_driver);
    }
    postcore_initcall(hi6220_reset_init);
    MODULE_DESCRIPTION("Hisilicon Hi6220 reset controller driver");
    MODULE_LICENSE("GPL v2");
