//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/sunxi/pinctrl-sun50i-h616-r.c
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
// Allwinner H616 R_PIO pin controller driver
//
// Copyright (C) 2020 Arm Ltd.
// Based on former work, which is:
// Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
//

    static const struct sunxi_desc_pin sun50i_h616_r_pins[] = {
    SUNXI_PIN(SUNXI_PINCTRL_PIN(L, 0),
    SUNXI_FUNCTION(0x0, "gpio_in"),
    SUNXI_FUNCTION(0x1, "gpio_out"),
    SUNXI_FUNCTION(0x2, "s_rsb"),		/* SCK */
    SUNXI_FUNCTION(0x3, "s_i2c")),	/* SCK */
    SUNXI_PIN(SUNXI_PINCTRL_PIN(L, 1),
    SUNXI_FUNCTION(0x0, "gpio_in"),
    SUNXI_FUNCTION(0x1, "gpio_out"),
    SUNXI_FUNCTION(0x2, "s_rsb"),		/* SDA */
    SUNXI_FUNCTION(0x3, "s_i2c")),	/* SDA */
    };
    static const struct sunxi_pinctrl_desc sun50i_h616_r_pinctrl_data = {
    .pins = sun50i_h616_r_pins,
    .npins = ARRAY_SIZE(sun50i_h616_r_pins),
    .pin_base = PL_BASE,
    };
#[no_mangle]
unsafe extern "C" fn sun50i_h616_r_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_h616_r_pinctrl_probe(struct platform_device *pdev)
    {
    return sunxi_pinctrl_init(pdev,
    &sun50i_h616_r_pinctrl_data);
    }
    static const struct of_device_id sun50i_h616_r_pinctrl_match[] = {
    { .compatible = "allwinner,sun50i-h616-r-pinctrl", },
    {}
    };
    static struct platform_driver sun50i_h616_r_pinctrl_driver = {
    .probe	= sun50i_h616_r_pinctrl_probe,
    .driver	= {
    .name		= "sun50i-h616-r-pinctrl",
    .of_match_table	= sun50i_h616_r_pinctrl_match,
    },
    };
    builtin_platform_driver(sun50i_h616_r_pinctrl_driver);
