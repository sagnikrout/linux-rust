//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/st-rng.c
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
// ST Random Number Generator Driver ST's Platforms
//
// Author: Pankaj Dev: <pankaj.dev@st.com>
// Lee Jones <lee.jones@linaro.org>
//
// Copyright (C) 2015 STMicroelectronics (R&D) Limited
//

// Registers
pub const ST_RNG_STATUS_REG: c_uint = 0x20;
pub const ST_RNG_DATA_REG: c_uint = 0x24;
// Registers fields

pub const ST_RNG_FIFO_DEPTH: c_int = 4;

//
// Samples are documented to be available every 0.667us, so in theory
// the 4 sample deep FIFO should take 2.668us to fill.  However, during
// thorough testing, it became apparent that filling the FIFO actually
// takes closer to 12us.  We then multiply by 2 in order to account for
// the lack of udelay()'s reliability, suggested by Russell King.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_rng_data {
    pub base: *mut void __iomem,
    pub ops: hwrng,
}

#[no_mangle]
unsafe extern "C" fn st_rng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int st_rng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    struct st_rng_data *ddata = (struct st_rng_data *)rng.priv;
    u32 status;
    int i;
// Wait until FIFO is full - max 4uS
    for (i = 0; i < ST_RNG_FILL_FIFO_TIMEOUT; i++) {
    status = readl_relaxed(ddata.base + ST_RNG_STATUS_REG);
    if (status & ST_RNG_STATUS_FIFO_FULL)
    break;
    udelay(1);
    }
    if (i == ST_RNG_FILL_FIFO_TIMEOUT)
    return 0;
    for (i = 0; i < ST_RNG_FIFO_SIZE && i < max; i += 2)
// (u16 *)(data + i) =
    readl_relaxed(ddata.base + ST_RNG_DATA_REG);
    return i;	/* No of bytes read */
    }
#[no_mangle]
unsafe extern "C" fn st_rng_probe(pdev: *mut platform_device) -> c_int {
    static int st_rng_probe(struct platform_device *pdev)
    {
    struct st_rng_data *ddata;
    struct clk *clk;
    void __iomem *base;
    int ret;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    ddata.ops.priv	= (unsigned long)ddata;
    ddata.ops.read	= st_rng_read;
    ddata.ops.name	= pdev.name;
    ddata.base	= base;
    ret = devm_hwrng_register(&pdev.dev, &ddata.ops);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register HW RNG\n");
    return ret;
    }
    dev_info(&pdev.dev, "Successfully registered HW RNG\n");
    return 0;
    }
    static const struct of_device_id st_rng_match[] __maybe_unused = {
    { .compatible = "st,rng" },
    {},
    };
    MODULE_DEVICE_TABLE(of, st_rng_match);
    static struct platform_driver st_rng_driver = {
    .driver = {
    .name = "st-hwrandom",
    .of_match_table = of_match_ptr(st_rng_match),
    },
    .probe = st_rng_probe,
    };
    module_platform_driver(st_rng_driver);
    MODULE_AUTHOR("Pankaj Dev <pankaj.dev@st.com>");
    MODULE_DESCRIPTION("ST Microelectronics HW Random Number Generator");
    MODULE_LICENSE("GPL v2");
