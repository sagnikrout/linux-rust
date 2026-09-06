//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/sun6i-prcm.c
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
// Copyright (C) 2014 Free Electrons
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//
// Allwinner PRCM (Power/Reset/Clock Management) driver
//

pub const SUN8I_CODEC_ANALOG_BASE: c_uint = 0x1c0;
pub const SUN8I_CODEC_ANALOG_SIZE: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prcm_data {
    pub nsubdevs: c_int,
    pub subdevs: *const mfd_cell,
}

    static const struct resource sun6i_a31_ar100_clk_res[] = {
    DEFINE_RES_MEM(0x0, 4)
    };
    static const struct resource sun6i_a31_apb0_clk_res[] = {
    DEFINE_RES_MEM(0xc, 4)
    };
    static const struct resource sun6i_a31_apb0_gates_clk_res[] = {
    DEFINE_RES_MEM(0x28, 4)
    };
    static const struct resource sun6i_a31_ir_clk_res[] = {
    DEFINE_RES_MEM(0x54, 4)
    };
    static const struct resource sun6i_a31_apb0_rstc_res[] = {
    DEFINE_RES_MEM(0xb0, 4)
    };
    static const struct resource sun8i_codec_analog_res[] = {
    DEFINE_RES_MEM(SUN8I_CODEC_ANALOG_BASE, SUN8I_CODEC_ANALOG_SIZE),
    };
    static const struct mfd_cell sun6i_a31_prcm_subdevs[] = {
    {
    .name = "sun6i-a31-ar100-clk",
    .of_compatible = "allwinner,sun6i-a31-ar100-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_ar100_clk_res),
    .resources = sun6i_a31_ar100_clk_res,
    },
    {
    .name = "sun6i-a31-apb0-clk",
    .of_compatible = "allwinner,sun6i-a31-apb0-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_clk_res),
    .resources = sun6i_a31_apb0_clk_res,
    },
    {
    .name = "sun6i-a31-apb0-gates-clk",
    .of_compatible = "allwinner,sun6i-a31-apb0-gates-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_gates_clk_res),
    .resources = sun6i_a31_apb0_gates_clk_res,
    },
    {
    .name = "sun6i-a31-ir-clk",
    .of_compatible = "allwinner,sun4i-a10-mod0-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_ir_clk_res),
    .resources = sun6i_a31_ir_clk_res,
    },
    {
    .name = "sun6i-a31-apb0-clock-reset",
    .of_compatible = "allwinner,sun6i-a31-clock-reset",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_rstc_res),
    .resources = sun6i_a31_apb0_rstc_res,
    },
    };
    static const struct mfd_cell sun8i_a23_prcm_subdevs[] = {
    {
    .name = "sun8i-a23-apb0-clk",
    .of_compatible = "allwinner,sun8i-a23-apb0-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_clk_res),
    .resources = sun6i_a31_apb0_clk_res,
    },
    {
    .name = "sun6i-a31-apb0-gates-clk",
    .of_compatible = "allwinner,sun8i-a23-apb0-gates-clk",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_gates_clk_res),
    .resources = sun6i_a31_apb0_gates_clk_res,
    },
    {
    .name = "sun6i-a31-apb0-clock-reset",
    .of_compatible = "allwinner,sun6i-a31-clock-reset",
    .num_resources = ARRAY_SIZE(sun6i_a31_apb0_rstc_res),
    .resources = sun6i_a31_apb0_rstc_res,
    },
    {
    .name		= "sun8i-codec-analog",
    .of_compatible	= "allwinner,sun8i-a23-codec-analog",
    .num_resources	= ARRAY_SIZE(sun8i_codec_analog_res),
    .resources	= sun8i_codec_analog_res,
    },
    };
    static const struct prcm_data sun6i_a31_prcm_data = {
    .nsubdevs = ARRAY_SIZE(sun6i_a31_prcm_subdevs),
    .subdevs = sun6i_a31_prcm_subdevs,
    };
    static const struct prcm_data sun8i_a23_prcm_data = {
    .nsubdevs = ARRAY_SIZE(sun8i_a23_prcm_subdevs),
    .subdevs = sun8i_a23_prcm_subdevs,
    };
    static const struct of_device_id sun6i_prcm_dt_ids[] = {
    {
    .compatible = "allwinner,sun6i-a31-prcm",
    .data = &sun6i_a31_prcm_data,
    },
    {
    .compatible = "allwinner,sun8i-a23-prcm",
    .data = &sun8i_a23_prcm_data,
    },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn sun6i_prcm_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_prcm_probe(struct platform_device *pdev)
    {
    const struct of_device_id *match;
    const struct prcm_data *data;
    struct resource *res;
    int ret;
    match = of_match_node(sun6i_prcm_dt_ids, pdev.dev.of_node);
    if (!match)
    return -EINVAL;
    data = match.data;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "no prcm memory region provided\n");
    return -ENOENT;
    }
    ret = mfd_add_devices(&pdev.dev, 0, data.subdevs, data.nsubdevs,
    res, -1, core::ptr::null_mut());
    if (ret) {
    dev_err(&pdev.dev, "failed to add subdevices\n");
    return ret;
    }
    return 0;
    }
    static struct platform_driver sun6i_prcm_driver = {
    .driver = {
    .name = "sun6i-prcm",
    .of_match_table = sun6i_prcm_dt_ids,
    },
    .probe = sun6i_prcm_probe,
    };
    builtin_platform_driver(sun6i_prcm_driver);
