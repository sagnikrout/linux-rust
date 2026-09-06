//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/pasemi-rng.c
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
// Copyright (C) 2006-2007 PA Semi, Inc
//
// Maintained by: Olof Johansson <olof@lixom.net>
//
// Driver for the PWRficient onchip rng
//

pub const SDCRNG_CTL_REG: c_uint = 0x00;
pub const SDCRNG_CTL_FVLD_M: c_uint = 0x0000f000;
pub const SDCRNG_CTL_FVLD_S: c_int = 12;
pub const SDCRNG_CTL_KSZ: c_uint = 0x00000800;
pub const SDCRNG_CTL_RSRC_CRG: c_uint = 0x00000010;
pub const SDCRNG_CTL_RSRC_RRG: c_uint = 0x00000000;
pub const SDCRNG_CTL_CE: c_uint = 0x00000004;
pub const SDCRNG_CTL_RE: c_uint = 0x00000002;
pub const SDCRNG_CTL_DR: c_uint = 0x00000001;

pub const SDCRNG_VAL_REG: c_uint = 0x20;

#[no_mangle]
unsafe extern "C" fn pasemi_rng_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int pasemi_rng_data_present(struct hwrng *rng, int wait)
    {
    void __iomem *rng_regs = (void __iomem *)rng.priv;
    int data, i;
    for (i = 0; i < 20; i++) {
    data = (in_le32(rng_regs + SDCRNG_CTL_REG)
    & SDCRNG_CTL_FVLD_M) ? 1 : 0;
    if (data || !wait)
    break;
    udelay(10);
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int pasemi_rng_data_read(struct hwrng *rng, u32 *data)
    {
    void __iomem *rng_regs = (void __iomem *)rng.priv;
// data = in_le32(rng_regs + SDCRNG_VAL_REG);
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_rng_init(rng: *mut hwrng) -> c_int {
    static int pasemi_rng_init(struct hwrng *rng)
    {
    void __iomem *rng_regs = (void __iomem *)rng.priv;
    u32 ctl;
    ctl = SDCRNG_CTL_DR | SDCRNG_CTL_SELECT_RRG_RNG | SDCRNG_CTL_KSZ;
    out_le32(rng_regs + SDCRNG_CTL_REG, ctl);
    out_le32(rng_regs + SDCRNG_CTL_REG, ctl & ~SDCRNG_CTL_DR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_rng_cleanup(rng: *mut hwrng) {
    static void pasemi_rng_cleanup(struct hwrng *rng)
    {
    void __iomem *rng_regs = (void __iomem *)rng.priv;
    u32 ctl;
    ctl = SDCRNG_CTL_RE | SDCRNG_CTL_CE;
    out_le32(rng_regs + SDCRNG_CTL_REG,
    in_le32(rng_regs + SDCRNG_CTL_REG) & ~ctl);
    }
    static struct hwrng pasemi_rng = {
    .name		= MODULE_NAME,
    .init		= pasemi_rng_init,
    .cleanup	= pasemi_rng_cleanup,
    .data_present	= pasemi_rng_data_present,
    .data_read	= pasemi_rng_data_read,
    };
#[no_mangle]
unsafe extern "C" fn rng_probe(pdev: *mut platform_device) -> c_int {
    static int rng_probe(struct platform_device *pdev)
    {
    void __iomem *rng_regs;
    rng_regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rng_regs))
    return PTR_ERR(rng_regs);
    pasemi_rng.priv = (unsigned long)rng_regs;
    pr_info("Registering PA Semi RNG\n");
    return devm_hwrng_register(&pdev.dev, &pasemi_rng);
    }
    static const struct of_device_id rng_match[] = {
    { .compatible      = "1682m-rng", },
    { .compatible      = "pasemi,pwrficient-rng", },
    { },
    };
    MODULE_DEVICE_TABLE(of, rng_match);
    static struct platform_driver rng_driver = {
    .driver = {
    .name = "pasemi-rng",
    .of_match_table = rng_match,
    },
    .probe		= rng_probe,
    };
    module_platform_driver(rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Egor Martovetsky <egor@pasemi.com>");
    MODULE_DESCRIPTION("H/W RNG driver for PA Semi processor");
