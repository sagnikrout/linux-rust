//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/sunxi/pinctrl-sun55i-a523.c
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
// Allwinner A523 SoC pinctrl driver.
//
// Copyright (C) 2023 Arm Ltd.
//

    static const u8 a523_nr_bank_pins[SUNXI_PINCTRL_MAX_BANKS] =
// PA  PB  PC  PD  PE  PF  PG  PH  PI  PJ  PK
    {  0, 15, 17, 24, 16,  7, 15, 20, 17, 28, 24 };
    static const unsigned int a523_irq_bank_map[] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
    static const u8 a523_irq_bank_muxes[SUNXI_PINCTRL_MAX_BANKS] =
// PA  PB  PC  PD  PE  PF  PG  PH  PI  PJ  PK
    {  0, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14};
    static struct sunxi_pinctrl_desc a523_pinctrl_data = {
    .irq_banks = ARRAY_SIZE(a523_irq_bank_map),
    .irq_bank_map = a523_irq_bank_map,
    .io_bias_cfg_variant = BIAS_VOLTAGE_PIO_POW_MODE_SEL,
    };
#[no_mangle]
unsafe extern "C" fn a523_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int a523_pinctrl_probe(struct platform_device *pdev)
    {
    return sunxi_pinctrl_dt_table_init(pdev, a523_nr_bank_pins,
    a523_irq_bank_muxes,
    &a523_pinctrl_data,
    SUNXI_PINCTRL_NEW_REG_LAYOUT |
    SUNXI_PINCTRL_ELEVEN_BANKS);
    }
    static const struct of_device_id a523_pinctrl_match[] = {
    { .compatible = "allwinner,sun55i-a523-pinctrl", },
    {}
    };
    static struct platform_driver a523_pinctrl_driver = {
    .probe	= a523_pinctrl_probe,
    .driver	= {
    .name		= "sun55i-a523-pinctrl",
    .of_match_table	= a523_pinctrl_match,
    },
    };
    builtin_platform_driver(a523_pinctrl_driver);
