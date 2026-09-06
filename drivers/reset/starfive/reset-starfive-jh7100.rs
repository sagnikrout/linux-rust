//! Automatically rewritten from C to Rust
//! Source: drivers/reset/starfive/reset-starfive-jh7100.c
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
// Reset driver for the StarFive JH7100 SoC
//
// Copyright (C) 2021 Emil Renner Berthing <kernel@esmil.dk>
//

// register offsets
pub const JH7100_RESET_ASSERT0: c_uint = 0x00;
pub const JH7100_RESET_ASSERT1: c_uint = 0x04;
pub const JH7100_RESET_ASSERT2: c_uint = 0x08;
pub const JH7100_RESET_ASSERT3: c_uint = 0x0c;
pub const JH7100_RESET_STATUS0: c_uint = 0x10;
pub const JH7100_RESET_STATUS1: c_uint = 0x14;
pub const JH7100_RESET_STATUS2: c_uint = 0x18;
pub const JH7100_RESET_STATUS3: c_uint = 0x1c;
//
// Writing a 1 to the n'th bit of the m'th ASSERT register asserts
// line 32m + n, and writing a 0 deasserts the same line.
// Most reset lines have their status inverted so a 0 bit in the STATUS
// register means the line is asserted and a 1 means it's deasserted. A few
// lines don't though, so store the expected value of the status registers when
// all lines are asserted.
//
    static const u32 jh7100_reset_asserted[4] = {
// STATUS0
    BIT(JH7100_RST_U74 % 32) |
    BIT(JH7100_RST_VP6_DRESET % 32) |
    BIT(JH7100_RST_VP6_BRESET % 32),
// STATUS1
    BIT(JH7100_RST_HIFI4_DRESET % 32) |
    BIT(JH7100_RST_HIFI4_BRESET % 32),
// STATUS2
    BIT(JH7100_RST_E24 % 32),
// STATUS3
    0,
    };
#[no_mangle]
unsafe extern "C" fn jh7100_reset_probe(pdev: *mut platform_device) -> int __init {
    static int __init jh7100_reset_probe(struct platform_device *pdev)
    {
    void __iomem *base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    return reset_starfive_jh71x0_register(&pdev.dev, pdev.dev.of_node,
    base + JH7100_RESET_ASSERT0,
    base + JH7100_RESET_STATUS0,
    jh7100_reset_asserted,
    JH7100_RSTN_END,
    THIS_MODULE);
    }
    static const struct of_device_id jh7100_reset_dt_ids[] = {
    { .compatible = "starfive,jh7100-reset" },
    { /* sentinel */ }
    };
    static struct platform_driver jh7100_reset_driver = {
    .driver = {
    .name = "jh7100-reset",
    .of_match_table = jh7100_reset_dt_ids,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver_probe(jh7100_reset_driver, jh7100_reset_probe);
