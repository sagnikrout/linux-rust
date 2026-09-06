//! Automatically rewritten from C to Rust
//! Source: drivers/reset/amlogic/reset-meson-audio-arb.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_audio_arb_data {
    pub rstc: reset_controller_dev,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub reset_bits: *const c_uint,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_audio_arb_match_data {
    pub reset_bits: *const c_uint,
    pub reset_num: c_uint,
}

pub const ARB_GENERAL_BIT: c_int = 31;
    static const unsigned int axg_audio_arb_reset_bits[] = {
    [AXG_ARB_TODDR_A]	= 0,
    [AXG_ARB_TODDR_B]	= 1,
    [AXG_ARB_TODDR_C]	= 2,
    [AXG_ARB_FRDDR_A]	= 4,
    [AXG_ARB_FRDDR_B]	= 5,
    [AXG_ARB_FRDDR_C]	= 6,
    };
    static const struct meson_audio_arb_match_data axg_audio_arb_match = {
    .reset_bits = axg_audio_arb_reset_bits,
    .reset_num = ARRAY_SIZE(axg_audio_arb_reset_bits),
    };
    static const unsigned int sm1_audio_arb_reset_bits[] = {
    [AXG_ARB_TODDR_A]	= 0,
    [AXG_ARB_TODDR_B]	= 1,
    [AXG_ARB_TODDR_C]	= 2,
    [AXG_ARB_FRDDR_A]	= 4,
    [AXG_ARB_FRDDR_B]	= 5,
    [AXG_ARB_FRDDR_C]	= 6,
    [AXG_ARB_TODDR_D]	= 3,
    [AXG_ARB_FRDDR_D]	= 7,
    };
    static const struct meson_audio_arb_match_data sm1_audio_arb_match = {
    .reset_bits = sm1_audio_arb_reset_bits,
    .reset_num = ARRAY_SIZE(sm1_audio_arb_reset_bits),
    };
    static int meson_audio_arb_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    u32 val;
    struct meson_audio_arb_data *arb =
    container_of(rcdev, struct meson_audio_arb_data, rstc);
    spin_lock(&arb.lock);
    val = readl(arb.regs);
    if (assert)
    val &= ~BIT(arb.reset_bits[id]);
    else
    val |= BIT(arb.reset_bits[id]);
    writel(val, arb.regs);
    spin_unlock(&arb.lock);
    return 0;
    }
    static int meson_audio_arb_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    u32 val;
    struct meson_audio_arb_data *arb =
    container_of(rcdev, struct meson_audio_arb_data, rstc);
    val = readl(arb.regs);
    return !(val & BIT(arb.reset_bits[id]));
    }
    static int meson_audio_arb_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return meson_audio_arb_update(rcdev, id, true);
    }
    static int meson_audio_arb_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return meson_audio_arb_update(rcdev, id, false);
    }
    static const struct reset_control_ops meson_audio_arb_rstc_ops = {
    .assert = meson_audio_arb_assert,
    .deassert = meson_audio_arb_deassert,
    .status = meson_audio_arb_status,
    };
    static const struct of_device_id meson_audio_arb_of_match[] = {
    {
    .compatible = "amlogic,meson-axg-audio-arb",
    .data = &axg_audio_arb_match,
    }, {
    .compatible = "amlogic,meson-sm1-audio-arb",
    .data = &sm1_audio_arb_match,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, meson_audio_arb_of_match);
#[no_mangle]
unsafe extern "C" fn meson_audio_arb_remove(pdev: *mut platform_device) {
    static void meson_audio_arb_remove(struct platform_device *pdev)
    {
    struct meson_audio_arb_data *arb = platform_get_drvdata(pdev);
// Disable all access
    spin_lock(&arb.lock);
    writel(0, arb.regs);
    spin_unlock(&arb.lock);
    }
#[no_mangle]
unsafe extern "C" fn meson_audio_arb_probe(pdev: *mut platform_device) -> c_int {
    static int meson_audio_arb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct meson_audio_arb_match_data *data;
    struct meson_audio_arb_data *arb;
    int ret;
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    arb = devm_kzalloc(dev, sizeof(*arb), GFP_KERNEL);
    if (!arb)
    return -ENOMEM;
    platform_set_drvdata(pdev, arb);
    arb.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(arb.clk))
    return dev_err_probe(dev, PTR_ERR(arb.clk), "failed to get clock\n");
    arb.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(arb.regs))
    return PTR_ERR(arb.regs);
    spin_lock_init(&arb.lock);
    arb.reset_bits = data.reset_bits;
    arb.rstc.nr_resets = data.reset_num;
    arb.rstc.ops = &meson_audio_arb_rstc_ops;
    arb.rstc.of_node = dev.of_node;
    arb.rstc.owner = THIS_MODULE;
//
// Enable general :
// In the initial state, all memory interfaces are disabled
// and the general bit is on
//
    writel(BIT(ARB_GENERAL_BIT), arb.regs);
// Register reset controller
    ret = devm_reset_controller_register(dev, &arb.rstc);
    if (ret) {
    dev_err(dev, "failed to register arb reset controller\n");
    meson_audio_arb_remove(pdev);
    }
    return ret;
    }
    static struct platform_driver meson_audio_arb_pdrv = {
    .probe = meson_audio_arb_probe,
    .remove = meson_audio_arb_remove,
    .driver = {
    .name = "meson-audio-arb-reset",
    .of_match_table = meson_audio_arb_of_match,
    },
    };
    module_platform_driver(meson_audio_arb_pdrv);
    MODULE_DESCRIPTION("Amlogic A113 Audio Memory Arbiter");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL v2");
