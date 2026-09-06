//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-ssp.c
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
// Copyright 2012 DENX Software Engineering, GmbH
//
// Pulled from code:
// Portions copyright (C) 2003 Russell King, PXA MMCI Driver
// Portions copyright (C) 2004-2005 Pierre Ossman, W83L51xD SD/MMC driver
//
// Copyright 2008 Embedded Alley Solutions, Inc.
// Copyright 2009-2011 Freescale Semiconductor, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn mxs_ssp_set_clk_rate(ssp: *mut mxs_ssp, rate: c_uint) {
    void mxs_ssp_set_clk_rate(struct mxs_ssp *ssp, unsigned int rate)
    {
    unsigned int ssp_clk, ssp_sck;
    u32 clock_divide, clock_rate;
    u32 val;
    ssp_clk = clk_get_rate(ssp.clk);
    for (clock_divide = 2; clock_divide <= 254; clock_divide += 2) {
    clock_rate = DIV_ROUND_UP(ssp_clk, rate * clock_divide);
    clock_rate = (clock_rate > 0) ? clock_rate - 1 : 0;
    if (clock_rate <= 255)
    break;
    }
    if (clock_divide > 254) {
    dev_err(ssp.dev,
    "%s: cannot set clock to %d\n", __func__, rate);
    return;
    }
    ssp_sck = ssp_clk / clock_divide / (1 + clock_rate);
    val = readl(ssp.base + HW_SSP_TIMING(ssp));
    val &= ~(BM_SSP_TIMING_CLOCK_DIVIDE | BM_SSP_TIMING_CLOCK_RATE);
    val |= BF_SSP(clock_divide, TIMING_CLOCK_DIVIDE);
    val |= BF_SSP(clock_rate, TIMING_CLOCK_RATE);
    writel(val, ssp.base + HW_SSP_TIMING(ssp));
    ssp.clk_rate = ssp_sck;
    dev_dbg(ssp.dev,
    "%s: clock_divide %d, clock_rate %d, ssp_clk %d, rate_actual %d, rate_requested %d\n",
    __func__, clock_divide, clock_rate, ssp_clk, ssp_sck, rate);
    }
    EXPORT_SYMBOL_GPL(mxs_ssp_set_clk_rate);
