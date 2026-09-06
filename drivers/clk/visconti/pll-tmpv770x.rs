//! Automatically rewritten from C to Rust
//! Source: drivers/clk/visconti/pll-tmpv770x.c
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
// Toshiba Visconti PLL controller
//
// Copyright (c) 2021 TOSHIBA CORPORATION
// Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
//
// Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
//

// Must be equal to the last pll ID increased by one

    static DEFINE_SPINLOCK(tmpv770x_pll_lock);
    static const struct visconti_pll_rate_table pipll0_rates[] __initconst = {
    VISCONTI_PLL_RATE(840000000, 0x1, 0x0, 0x1, 0x54, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE(780000000, 0x1, 0x0, 0x1, 0x4e, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE(600000000, 0x1, 0x0, 0x1, 0x3c, 0x000000, 0x2, 0x1),
    { /* sentinel */ },
    };
    static const struct visconti_pll_rate_table piddrcpll_rates[] __initconst = {
    VISCONTI_PLL_RATE(780000000, 0x1, 0x0, 0x1, 0x4e, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE(760000000, 0x1, 0x0, 0x1, 0x4c, 0x000000, 0x2, 0x1),
    { /* sentinel */ },
    };
    static const struct visconti_pll_rate_table pivoifpll_rates[] __initconst = {
    VISCONTI_PLL_RATE(165000000, 0x1, 0x0, 0x1, 0x42, 0x000000, 0x4, 0x2),
    VISCONTI_PLL_RATE(148500000, 0x1, 0x1, 0x1, 0x3b, 0x666666, 0x4, 0x2),
    VISCONTI_PLL_RATE(96000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x2),
    VISCONTI_PLL_RATE(74250000, 0x1, 0x1, 0x1, 0x3b, 0x666666, 0x4, 0x4),
    VISCONTI_PLL_RATE(54000000, 0x1, 0x0, 0x1, 0x36, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE(48000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE(35750000, 0x1, 0x1, 0x1, 0x32, 0x0ccccc, 0x7, 0x4),
    { /* sentinel */ },
    };
    static const struct visconti_pll_rate_table piimgerpll_rates[] __initconst = {
    VISCONTI_PLL_RATE(165000000, 0x1, 0x0, 0x1, 0x42, 0x000000, 0x4, 0x2),
    VISCONTI_PLL_RATE(96000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x2),
    VISCONTI_PLL_RATE(54000000, 0x1, 0x0, 0x1, 0x36, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE(48000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x4),
    { /* sentinel */ },
    };
    static const struct visconti_pll_info pll_info[] __initconst = {
    { TMPV770X_PLL_PIPLL0, "pipll0", "osc2-clk", 0x0, pipll0_rates },
    { TMPV770X_PLL_PIDDRCPLL, "piddrcpll", "osc2-clk", 0x500, piddrcpll_rates },
    { TMPV770X_PLL_PIVOIFPLL, "pivoifpll", "osc2-clk", 0x600, pivoifpll_rates },
    { TMPV770X_PLL_PIIMGERPLL, "piimgerpll", "osc2-clk", 0x700, piimgerpll_rates },
    };
#[no_mangle]
unsafe extern "C" fn tmpv770x_setup_plls(np: *mut device_node) -> void __init {
    static void __init tmpv770x_setup_plls(struct device_node *np)
    {
    struct visconti_pll_provider *ctx;
    void __iomem *reg_base;
    reg_base = of_iomap(np, 0);
    if (!reg_base)
    return;
    ctx = visconti_init_pll(np, reg_base, PLLS_NR);
    if (IS_ERR(ctx)) {
    iounmap(reg_base);
    return;
    }
    ctx.clk_data.hws[TMPV770X_PLL_PIPLL1] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "pipll1", core::ptr::null_mut(), 0, 600000000);
    ctx.clk_data.hws[TMPV770X_PLL_PIDNNPLL] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "pidnnpll", core::ptr::null_mut(), 0, 500000000);
    ctx.clk_data.hws[TMPV770X_PLL_PIETHERPLL] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "pietherpll", core::ptr::null_mut(), 0, 500000000);
    visconti_register_plls(ctx, pll_info, ARRAY_SIZE(pll_info), &tmpv770x_pll_lock);
    }
    CLK_OF_DECLARE(tmpv770x_plls, "toshiba,tmpv7708-pipllct", tmpv770x_setup_plls);
