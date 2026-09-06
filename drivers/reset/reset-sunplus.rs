//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-sunplus.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// SP7021 reset driver
//
// Copyright (C) Sunplus Technology Co., Ltd.
// All rights reserved.
//

// HIWORD_MASK_REG BITS
pub const BITS_PER_HWM_REG: c_int = 16;
// resets HW info: reg_index_shift
    static const u32 sp_resets[] = {
// SP7021: mo_reset0 ~ mo_reset9
    0x00,
    0x02,
    0x03,
    0x04,
    0x05,
    0x06,
    0x07,
    0x08,
    0x09,
    0x0a,
    0x0b,
    0x0d,
    0x0e,
    0x0f,
    0x10,
    0x12,
    0x14,
    0x15,
    0x16,
    0x17,
    0x18,
    0x19,
    0x1a,
    0x1b,
    0x1c,
    0x1d,
    0x1e,
    0x1f,
    0x20,
    0x21,
    0x22,
    0x23,
    0x24,
    0x25,
    0x26,
    0x2a,
    0x2b,
    0x2d,
    0x2e,
    0x30,
    0x31,
    0x32,
    0x33,
    0x3d,
    0x3e,
    0x3f,
    0x42,
    0x44,
    0x4b,
    0x4c,
    0x4d,
    0x4e,
    0x4f,
    0x50,
    0x55,
    0x60,
    0x61,
    0x6a,
    0x6f,
    0x70,
    0x73,
    0x74,
    0x86,
    0x8a,
    0x8b,
    0x8d,
    0x8e,
    0x8f,
    0x90,
    0x92,
    0x93,
    0x94,
    0x95,
    0x96,
    0x97,
    0x98,
    0x99,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_reset {
    pub rcdev: reset_controller_dev,
    pub base: *mut void __iomem,
}

    static inline struct sp_reset *to_sp_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct sp_reset, rcdev);
    }
    static int sp_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct sp_reset *reset = to_sp_reset(rcdev);
    let mut index: c_int = sp_resets[id] / BITS_PER_HWM_REG;
    let mut shift: c_int = sp_resets[id] % BITS_PER_HWM_REG;
    u32 val;
    val = (1 << (16 + shift)) | (assert << shift);
    writel(val, reset.base + (index * 4));
    return 0;
    }
    static int sp_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return sp_reset_update(rcdev, id, true);
    }
    static int sp_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return sp_reset_update(rcdev, id, false);
    }
    static int sp_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct sp_reset *reset = to_sp_reset(rcdev);
    let mut index: c_int = sp_resets[id] / BITS_PER_HWM_REG;
    let mut shift: c_int = sp_resets[id] % BITS_PER_HWM_REG;
    u32 reg;
    reg = readl(reset.base + (index * 4));
    return !!(reg & BIT(shift));
    }
    static const struct reset_control_ops sp_reset_ops = {
    .assert   = sp_reset_assert,
    .deassert = sp_reset_deassert,
    .status   = sp_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn sp_restart(data: *mut sys_off_data) -> c_int {
    static int sp_restart(struct sys_off_data *data)
    {
    struct sp_reset *reset = data.cb_data;
    sp_reset_assert(&reset.rcdev, 0);
    sp_reset_deassert(&reset.rcdev, 0);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn sp_reset_probe(pdev: *mut platform_device) -> c_int {
    static int sp_reset_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_reset *reset;
    struct resource *res;
    int ret;
    reset = devm_kzalloc(dev, sizeof(*reset), GFP_KERNEL);
    if (!reset)
    return -ENOMEM;
    reset.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(reset.base))
    return PTR_ERR(reset.base);
    reset.rcdev.ops = &sp_reset_ops;
    reset.rcdev.owner = THIS_MODULE;
    reset.rcdev.of_node = dev.of_node;
    reset.rcdev.nr_resets = resource_size(res) / 4 * BITS_PER_HWM_REG;
    ret = devm_reset_controller_register(dev, &reset.rcdev);
    if (ret)
    return ret;
    return devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART,
    192, sp_restart, reset);
    }
    static const struct of_device_id sp_reset_dt_ids[] = {
    {.compatible = "sunplus,sp7021-reset",},
    { /* sentinel */ },
    };
    static struct platform_driver sp_reset_driver = {
    .probe = sp_reset_probe,
    .driver = {
    .name			= "sunplus-reset",
    .of_match_table		= sp_reset_dt_ids,
    .suppress_bind_attrs	= true,
    },
    };
    builtin_platform_driver(sp_reset_driver);
