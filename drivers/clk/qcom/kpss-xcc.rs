//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/kpss-xcc.c
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

    static const struct clk_parent_data aux_parents[] = {
    { .fw_name = "pll8_vote", .name = "pll8_vote" },
    { .fw_name = "pxo", .name = "pxo_board" },
    };
    static const u32 aux_parent_map[] = {
    3,
    0,
    };
    static const struct of_device_id kpss_xcc_match_table[] = {
    { .compatible = "qcom,kpss-acc-v1", .data = (void *)1UL },
    { .compatible = "qcom,kpss-gcc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, kpss_xcc_match_table);
#[no_mangle]
unsafe extern "C" fn kpss_xcc_driver_probe(pdev: *mut platform_device) -> c_int {
    static int kpss_xcc_driver_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *base;
    struct clk_hw *hw;
    const char *name;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    if (device_get_match_data(&pdev.dev)) {
    if (of_property_read_string_index(dev.of_node,
    "clock-output-names",
    0, &name))
    return -ENODEV;
    base += 0x14;
    } else {
    name = "acpu_l2_aux";
    base += 0x28;
    }
    hw = devm_clk_hw_register_mux_parent_data_table(dev, name, aux_parents,
    ARRAY_SIZE(aux_parents), 0,
    base, 0, 0x3,
    0, aux_parent_map, core::ptr::null_mut());
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    return of_clk_add_hw_provider(dev.of_node, of_clk_hw_simple_get, hw);
    }
    static struct platform_driver kpss_xcc_driver = {
    .probe = kpss_xcc_driver_probe,
    .driver = {
    .name = "kpss-xcc",
    .of_match_table = kpss_xcc_match_table,
    },
    };
    module_platform_driver(kpss_xcc_driver);
    MODULE_DESCRIPTION("Krait Processor Sub System (KPSS) Clock Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:kpss-xcc");
