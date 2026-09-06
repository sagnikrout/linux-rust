//! Automatically rewritten from C to Rust
//! Source: drivers/soc/lantiq/fpi-bus.c
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
// Copyright (C) 2011-2015 John Crispin <blogic@phrozen.org>
// Copyright (C) 2015 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
// Copyright (C) 2017 Hauke Mehrtens <hauke@hauke-m.de>
//

pub const XBAR_ALWAYS_LAST: c_uint = 0x430;

pub const RCU_VR9_BE_AHB1S: c_uint = 0x00000008;
#[no_mangle]
unsafe extern "C" fn ltq_fpi_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_fpi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct regmap *rcu_regmap;
    void __iomem *xbar_membase;
    u32 rcu_ahb_endianness_reg_offset;
    int ret;
    xbar_membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xbar_membase))
    return PTR_ERR(xbar_membase);
// RCU configuration is optional
    rcu_regmap = syscon_regmap_lookup_by_phandle(np, "lantiq,rcu");
    if (IS_ERR(rcu_regmap))
    return PTR_ERR(rcu_regmap);
    ret = device_property_read_u32(dev, "lantiq,offset-endianness",
    &rcu_ahb_endianness_reg_offset);
    if (ret) {
    dev_err(&pdev.dev, "Failed to get RCU reg offset\n");
    return ret;
    }
    ret = regmap_update_bits(rcu_regmap, rcu_ahb_endianness_reg_offset,
    RCU_VR9_BE_AHB1S, RCU_VR9_BE_AHB1S);
    if (ret) {
    dev_warn(&pdev.dev,
    "Failed to configure RCU AHB endianness\n");
    return ret;
    }
// disable fpi burst
    ltq_w32_mask(XBAR_FPI_BURST_EN, 0, xbar_membase + XBAR_ALWAYS_LAST);
    return of_platform_populate(dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    }
    static const struct of_device_id ltq_fpi_match[] = {
    { .compatible = "lantiq,xrx200-fpi" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltq_fpi_match);
    static struct platform_driver ltq_fpi_driver = {
    .probe = ltq_fpi_probe,
    .driver = {
    .name = "fpi-xway",
    .of_match_table = ltq_fpi_match,
    },
    };
    module_platform_driver(ltq_fpi_driver);
    MODULE_DESCRIPTION("Lantiq FPI bus driver");
    MODULE_LICENSE("GPL");
