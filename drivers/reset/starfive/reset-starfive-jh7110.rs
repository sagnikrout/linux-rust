//! Automatically rewritten from C to Rust
//! Source: drivers/reset/starfive/reset-starfive-jh7110.c
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
// Reset driver for the StarFive JH7110 SoC
//
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_reset_info {
    pub nr_resets: c_uint,
    pub assert_offset: c_uint,
    pub status_offset: c_uint,
}

    static const struct jh7110_reset_info jh7110_sys_info = {
    .nr_resets = JH7110_SYSRST_END,
    .assert_offset = 0x2F8,
    .status_offset = 0x308,
    };
    static const struct jh7110_reset_info jh7110_aon_info = {
    .nr_resets = JH7110_AONRST_END,
    .assert_offset = 0x38,
    .status_offset = 0x3C,
    };
    static const struct jh7110_reset_info jh7110_stg_info = {
    .nr_resets = JH7110_STGRST_END,
    .assert_offset = 0x74,
    .status_offset = 0x78,
    };
    static const struct jh7110_reset_info jh7110_isp_info = {
    .nr_resets = JH7110_ISPRST_END,
    .assert_offset = 0x38,
    .status_offset = 0x3C,
    };
    static const struct jh7110_reset_info jh7110_vout_info = {
    .nr_resets = JH7110_VOUTRST_END,
    .assert_offset = 0x48,
    .status_offset = 0x4C,
    };
    static int jh7110_reset_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct jh7110_reset_info *info = (struct jh7110_reset_info *)(id.driver_data);
    struct jh71x0_reset_adev *rdev = to_jh71x0_reset_adev(adev);
    void __iomem *base = rdev.base;
    if (!info || !base)
    return -ENODEV;
    return reset_starfive_jh71x0_register(&adev.dev, adev.dev.parent.of_node,
    base + info.assert_offset,
    base + info.status_offset,
    core::ptr::null_mut(),
    info.nr_resets,
    core::ptr::null_mut());
    }
    static const struct auxiliary_device_id jh7110_reset_ids[] = {
    {
    .name = "clk_starfive_jh7110_sys.rst-sys",
    .driver_data = (kernel_ulong_t)&jh7110_sys_info,
    },
    {
    .name = "clk_starfive_jh7110_sys.rst-aon",
    .driver_data = (kernel_ulong_t)&jh7110_aon_info,
    },
    {
    .name = "clk_starfive_jh7110_sys.rst-stg",
    .driver_data = (kernel_ulong_t)&jh7110_stg_info,
    },
    {
    .name = "clk_starfive_jh7110_sys.rst-isp",
    .driver_data = (kernel_ulong_t)&jh7110_isp_info,
    },
    {
    .name = "clk_starfive_jh7110_sys.rst-vo",
    .driver_data = (kernel_ulong_t)&jh7110_vout_info,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(auxiliary, jh7110_reset_ids);
    static struct auxiliary_driver jh7110_reset_driver = {
    .probe		= jh7110_reset_probe,
    .id_table	= jh7110_reset_ids,
    };
    module_auxiliary_driver(jh7110_reset_driver);
    MODULE_AUTHOR("Hal Feng <hal.feng@starfivetech.com>");
    MODULE_DESCRIPTION("StarFive JH7110 reset driver");
    MODULE_LICENSE("GPL");
