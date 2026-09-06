//! Automatically rewritten from C to Rust
//! Source: drivers/reset/hisilicon/reset-hi3660.c
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
// Copyright (c) 2016-2017 Linaro Ltd.
// Copyright (c) 2016-2017 HiSilicon Technologies Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_reset_controller {
    pub rst: reset_controller_dev,
    pub map: *mut regmap,
}

    container_of(_rst, struct hi3660_reset_controller, rst)
    static int hi3660_reset_program_hw(struct reset_controller_dev *rcdev,
    unsigned long idx, bool assert)
    {
    struct hi3660_reset_controller *rc = to_hi3660_reset_controller(rcdev);
    let mut offset: c_uint = idx >> 8;
    let mut mask: c_uint = BIT(idx & 0x1f);
    if (assert)
    return regmap_write(rc.map, offset, mask);
    else
    return regmap_write(rc.map, offset + 4, mask);
    }
    static int hi3660_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    return hi3660_reset_program_hw(rcdev, idx, true);
    }
    static int hi3660_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    return hi3660_reset_program_hw(rcdev, idx, false);
    }
    static int hi3660_reset_dev(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    int err;
    err = hi3660_reset_assert(rcdev, idx);
    if (err)
    return err;
    return hi3660_reset_deassert(rcdev, idx);
    }
    static const struct reset_control_ops hi3660_reset_ops = {
    .reset    = hi3660_reset_dev,
    .assert   = hi3660_reset_assert,
    .deassert = hi3660_reset_deassert,
    };
    static int hi3660_reset_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
    unsigned int offset, bit;
    offset = reset_spec.args[0];
    bit = reset_spec.args[1];
    return (offset << 8) | bit;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_reset_probe(pdev: *mut platform_device) -> c_int {
    static int hi3660_reset_probe(struct platform_device *pdev)
    {
    struct hi3660_reset_controller *rc;
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    rc = devm_kzalloc(dev, sizeof(*rc), GFP_KERNEL);
    if (!rc)
    return -ENOMEM;
    rc.map = syscon_regmap_lookup_by_phandle(np, "hisilicon,rst-syscon");
    if (rc.map == ERR_PTR(-ENODEV)) {
// fall back to the deprecated compatible
    rc.map = syscon_regmap_lookup_by_phandle(np,
    "hisi,rst-syscon");
    }
    if (IS_ERR(rc.map)) {
    return dev_err_probe(dev, PTR_ERR(rc.map),
    "failed to get hisilicon,rst-syscon\n");
    }
    rc.rst.ops = &hi3660_reset_ops,
    rc.rst.of_node = np;
    rc.rst.of_reset_n_cells = 2;
    rc.rst.of_xlate = hi3660_reset_xlate;
    return reset_controller_register(&rc.rst);
    }
    static const struct of_device_id hi3660_reset_match[] = {
    { .compatible = "hisilicon,hi3660-reset", },
    {},
    };
    MODULE_DEVICE_TABLE(of, hi3660_reset_match);
    static struct platform_driver hi3660_reset_driver = {
    .probe = hi3660_reset_probe,
    .driver = {
    .name = "hi3660-reset",
    .of_match_table = hi3660_reset_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn hi3660_reset_init() -> int __init {
    static int __init hi3660_reset_init(void)
    {
    return platform_driver_register(&hi3660_reset_driver);
    }
    arch_initcall(hi3660_reset_init);
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:hi3660-reset");
    MODULE_DESCRIPTION("HiSilicon Hi3660 Reset Driver");
