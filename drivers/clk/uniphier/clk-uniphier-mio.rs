//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-mio.c
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

    UNIPHIER_CLK_FACTOR("sd-44m", -1, "sd-133m", 1, 3),		\
    UNIPHIER_CLK_FACTOR("sd-33m", -1, "sd-200m", 1, 6),		\
    UNIPHIER_CLK_FACTOR("sd-50m", -1, "sd-200m", 1, 4),		\
    UNIPHIER_CLK_FACTOR("sd-67m", -1, "sd-200m", 1, 3),		\
    UNIPHIER_CLK_FACTOR("sd-100m", -1, "sd-200m", 1, 2),		\
    UNIPHIER_CLK_FACTOR("sd-40m", -1, "sd-200m", 1, 5),		\
    UNIPHIER_CLK_FACTOR("sd-25m", -1, "sd-200m", 1, 8),		\
    UNIPHIER_CLK_FACTOR("sd-22m", -1, "sd-133m", 1, 6)

    {								\
    .name = "sd" #ch "-sel",				\
    .type = UNIPHIER_CLK_TYPE_MUX,				\
    .idx = -1,						\
    .data.mux = {						\
    .parent_names = {				\
    "sd-44m",				\
    "sd-33m",				\
    "sd-50m",				\
    "sd-67m",				\
    "sd-100m",				\
    "sd-40m",				\
    "sd-25m",				\
    "sd-22m",				\
    },						\
    .num_parents = 8,				\
    .reg = 0x30 + 0x200 * (ch),			\
    .masks = {					\
    0x00031000,				\
    0x00031000,				\
    0x00031000,				\
    0x00031000,				\
    0x00001300,				\
    0x00001300,				\
    0x00001300,				\
    0x00001300,				\
    },						\
    .vals = {					\
    0x00000000,				\
    0x00010000,				\
    0x00020000,				\
    0x00030000,				\
    0x00001000,				\
    0x00001100,				\
    0x00001200,				\
    0x00001300,				\
    },						\
    },							\
    },								\
    UNIPHIER_CLK_GATE("sd" #ch, (_idx), "sd" #ch "-sel", 0x20 + 0x200 * (ch), 8)

    UNIPHIER_CLK_GATE("usb2" #ch, (idx), "usb2", 0x20 + 0x200 * (ch), 28)

    UNIPHIER_CLK_GATE("usb2" #ch "-phy", (idx), "usb2", 0x20 + 0x200 * (ch), 29)
    const struct uniphier_clk_data uniphier_ld4_mio_clk_data[] = {
    UNIPHIER_MIO_CLK_SD_FIXED,
    UNIPHIER_MIO_CLK_SD(0, 0),
    UNIPHIER_MIO_CLK_SD(1, 1),
    UNIPHIER_MIO_CLK_SD(2, 2),
    UNIPHIER_CLK_GATE("miodmac", 7, core::ptr::null_mut(), 0x20, 25),
    UNIPHIER_MIO_CLK_USB2(8, 0),
    UNIPHIER_MIO_CLK_USB2(9, 1),
    UNIPHIER_MIO_CLK_USB2(10, 2),
    UNIPHIER_MIO_CLK_USB2_PHY(12, 0),
    UNIPHIER_MIO_CLK_USB2_PHY(13, 1),
    UNIPHIER_MIO_CLK_USB2_PHY(14, 2),
    { /* sentinel */ }
    };
    const struct uniphier_clk_data uniphier_pro5_sd_clk_data[] = {
    UNIPHIER_MIO_CLK_SD_FIXED,
    UNIPHIER_MIO_CLK_SD(0, 0),
    UNIPHIER_MIO_CLK_SD(1, 1),
    { /* sentinel */ }
    };
