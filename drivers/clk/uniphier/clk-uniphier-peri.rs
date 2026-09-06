//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-peri.c
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
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

    UNIPHIER_CLK_GATE("uart" #ch, (idx), "uart", 0x24, 19 + (ch))

    UNIPHIER_CLK_GATE("i2c-common", -1, "i2c", 0x20, 1)

    UNIPHIER_CLK_GATE("i2c" #ch, (idx), "i2c-common", 0x24, 5 + (ch))

    UNIPHIER_CLK_GATE("i2c" #ch, (idx), "i2c", 0x24, 24 + (ch))

    UNIPHIER_CLK_GATE("scssi" #ch, (idx), "spi", 0x20, 17 + (ch))

    UNIPHIER_CLK_GATE("mcssi", (idx), "spi", 0x24, 14)
    const struct uniphier_clk_data uniphier_ld4_peri_clk_data[] = {
    UNIPHIER_PERI_CLK_UART(0, 0),
    UNIPHIER_PERI_CLK_UART(1, 1),
    UNIPHIER_PERI_CLK_UART(2, 2),
    UNIPHIER_PERI_CLK_UART(3, 3),
    UNIPHIER_PERI_CLK_I2C_COMMON,
    UNIPHIER_PERI_CLK_I2C(4, 0),
    UNIPHIER_PERI_CLK_I2C(5, 1),
    UNIPHIER_PERI_CLK_I2C(6, 2),
    UNIPHIER_PERI_CLK_I2C(7, 3),
    UNIPHIER_PERI_CLK_I2C(8, 4),
    UNIPHIER_PERI_CLK_SCSSI(11, 0),
    { /* sentinel */ }
    };
    const struct uniphier_clk_data uniphier_pro4_peri_clk_data[] = {
    UNIPHIER_PERI_CLK_UART(0, 0),
    UNIPHIER_PERI_CLK_UART(1, 1),
    UNIPHIER_PERI_CLK_UART(2, 2),
    UNIPHIER_PERI_CLK_UART(3, 3),
    UNIPHIER_PERI_CLK_FI2C(4, 0),
    UNIPHIER_PERI_CLK_FI2C(5, 1),
    UNIPHIER_PERI_CLK_FI2C(6, 2),
    UNIPHIER_PERI_CLK_FI2C(7, 3),
    UNIPHIER_PERI_CLK_FI2C(8, 4),
    UNIPHIER_PERI_CLK_FI2C(9, 5),
    UNIPHIER_PERI_CLK_FI2C(10, 6),
    UNIPHIER_PERI_CLK_SCSSI(11, 0),
    UNIPHIER_PERI_CLK_SCSSI(12, 1),
    UNIPHIER_PERI_CLK_SCSSI(13, 2),
    UNIPHIER_PERI_CLK_SCSSI(14, 3),
    UNIPHIER_PERI_CLK_MCSSI(15),
    { /* sentinel */ }
    };
