//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-bcm2835-aux.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2015 Broadcom
//

pub const BCM2835_AUXIRQ: c_uint = 0x00;
pub const BCM2835_AUXENB: c_uint = 0x04;
#[no_mangle]
unsafe extern "C" fn bcm2835_aux_clk_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_aux_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct clk_hw_onecell_data *onecell;
    const char *parent;
    struct clk *parent_clk;
    void __iomem *reg, *gate;
    parent_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(parent_clk))
    return PTR_ERR(parent_clk);
    parent = __clk_get_name(parent_clk);
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    onecell = devm_kmalloc(dev,
    struct_size(onecell, hws,
    BCM2835_AUX_CLOCK_COUNT),
    GFP_KERNEL);
    if (!onecell)
    return -ENOMEM;
    onecell.num = BCM2835_AUX_CLOCK_COUNT;
    gate = reg + BCM2835_AUXENB;
    onecell.hws[BCM2835_AUX_CLOCK_UART] =
    clk_hw_register_gate(dev, "aux_uart", parent, 0, gate, 0, 0, core::ptr::null_mut());
    onecell.hws[BCM2835_AUX_CLOCK_SPI1] =
    clk_hw_register_gate(dev, "aux_spi1", parent, 0, gate, 1, 0, core::ptr::null_mut());
    onecell.hws[BCM2835_AUX_CLOCK_SPI2] =
    clk_hw_register_gate(dev, "aux_spi2", parent, 0, gate, 2, 0, core::ptr::null_mut());
    return of_clk_add_hw_provider(pdev.dev.of_node, of_clk_hw_onecell_get,
    onecell);
    }
    static const struct of_device_id bcm2835_aux_clk_of_match[] = {
    { .compatible = "brcm,bcm2835-aux", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_aux_clk_of_match);
    static struct platform_driver bcm2835_aux_clk_driver = {
    .driver = {
    .name = "bcm2835-aux-clk",
    .of_match_table = bcm2835_aux_clk_of_match,
    },
    .probe          = bcm2835_aux_clk_probe,
    };
    builtin_platform_driver(bcm2835_aux_clk_driver);
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("BCM2835 auxiliary peripheral clock driver");
