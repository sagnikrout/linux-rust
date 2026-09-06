//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-sky1-audss.c
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
// Cix Sky1 Audio Subsystem reset controller driver
//
// Copyright 2026 Cix Technology Group Co., Ltd.
//

pub const SKY1_RESET_SLEEP_US: c_int = 50;
pub const AUDSS_SW_RST: c_uint = 0x78;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_audss_reset_map {
    pub offset: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_audss_reset {
    pub rcdev: reset_controller_dev,
    pub regmap: *mut regmap,
    pub map: *const sky1_audss_reset_map,
}

    static const struct sky1_audss_reset_map sky1_audss_reset_map[] = {
    [AUDSS_I2S0_SW_RST]   = { AUDSS_SW_RST, BIT(0) },
    [AUDSS_I2S1_SW_RST]   = { AUDSS_SW_RST, BIT(1) },
    [AUDSS_I2S2_SW_RST]   = { AUDSS_SW_RST, BIT(2) },
    [AUDSS_I2S3_SW_RST]   = { AUDSS_SW_RST, BIT(3) },
    [AUDSS_I2S4_SW_RST]   = { AUDSS_SW_RST, BIT(4) },
    [AUDSS_I2S5_SW_RST]   = { AUDSS_SW_RST, BIT(5) },
    [AUDSS_I2S6_SW_RST]   = { AUDSS_SW_RST, BIT(6) },
    [AUDSS_I2S7_SW_RST]   = { AUDSS_SW_RST, BIT(7) },
    [AUDSS_I2S8_SW_RST]   = { AUDSS_SW_RST, BIT(8) },
    [AUDSS_I2S9_SW_RST]   = { AUDSS_SW_RST, BIT(9) },
    [AUDSS_WDT_SW_RST]    = { AUDSS_SW_RST, BIT(10) },
    [AUDSS_TIMER_SW_RST]  = { AUDSS_SW_RST, BIT(11) },
    [AUDSS_MB0_SW_RST]    = { AUDSS_SW_RST, BIT(12) },
    [AUDSS_MB1_SW_RST]    = { AUDSS_SW_RST, BIT(13) },
    [AUDSS_HDA_SW_RST]    = { AUDSS_SW_RST, BIT(14) },
    [AUDSS_DMAC_SW_RST]   = { AUDSS_SW_RST, BIT(15) },
    };
    static struct sky1_audss_reset *to_sky1_audss_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct sky1_audss_reset, rcdev);
    }
    static int sky1_audss_reset_set(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct sky1_audss_reset *priv = to_sky1_audss_reset(rcdev);
    const struct sky1_audss_reset_map *signal = &priv.map[id];
    int ret;
    ret = regmap_assign_bits(priv.regmap, signal.offset,
    signal.mask, !assert);
    if (ret)
    return ret;
    fsleep(SKY1_RESET_SLEEP_US);
    return 0;
    }
    static int sky1_audss_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return sky1_audss_reset_set(rcdev, id, true);
    }
    static int sky1_audss_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return sky1_audss_reset_set(rcdev, id, false);
    }
    static const struct reset_control_ops sky1_audss_reset_ops = {
    .assert   = sky1_audss_reset_assert,
    .deassert = sky1_audss_reset_deassert,
    };
    static int sky1_audss_reset_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct sky1_audss_reset *priv;
    struct device *dev = &adev.dev;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.regmap)
    return dev_err_probe(dev, -ENODEV, "failed to get parent regmap\n");
    priv.map = sky1_audss_reset_map;
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.nr_resets = ARRAY_SIZE(sky1_audss_reset_map);
    priv.rcdev.ops = &sky1_audss_reset_ops;
    priv.rcdev.of_node = dev.of_node;
    priv.rcdev.dev = dev;
    return devm_reset_controller_register(dev, &priv.rcdev);
    }
    static const struct auxiliary_device_id sky1_audss_reset_ids[] = {
    { .name = "clk_sky1_audss.reset" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, sky1_audss_reset_ids);
    static struct auxiliary_driver sky1_audss_reset_driver = {
    .probe = sky1_audss_reset_probe,
    .id_table = sky1_audss_reset_ids,
    };
    module_auxiliary_driver(sky1_audss_reset_driver);
    MODULE_AUTHOR("Joakim Zhang <joakim.zhang@cixtech.com>");
    MODULE_DESCRIPTION("Cix Sky1 Audio Subsystem reset driver");
    MODULE_LICENSE("GPL");
