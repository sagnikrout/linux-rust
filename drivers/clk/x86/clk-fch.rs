//! Automatically rewritten from C to Rust
//! Source: drivers/clk/x86/clk-fch.c
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


// SPDX-License-Identifier: MIT
//
// clock framework for AMD FCH controller block
//
// Copyright 2018 Advanced Micro Devices, Inc.
//

// Clock Driving Strength 2 register
pub const CLKDRVSTR2: c_uint = 0x28;
// Clock Control 1 register
pub const MISCCLKCNTL1: c_uint = 0x40;
// Auxiliary clock1 enable bit
pub const OSCCLKENB: c_int = 2;
// 25Mhz auxiliary output clock freq bit
pub const OSCOUT1CLK25MHZ: c_int = 16;
pub const ST_CLK_48M: c_int = 0;
pub const ST_CLK_25M: c_int = 1;
pub const ST_CLK_MUX: c_int = 2;
pub const ST_CLK_GATE: c_int = 3;
pub const ST_MAX_CLKS: c_int = 4;
pub const CLK_48M_FIXED: c_int = 0;
pub const CLK_GATE_FIXED: c_int = 1;
pub const CLK_MAX_FIXED: c_int = 2;
// List of supported CPU ids for clk mux with 25Mhz clk support
pub const AMD_CPU_ID_ST: c_uint = 0x1576;
    static const char * const clk_oscout1_parents[] = { "clk48MHz", "clk25MHz" };
    static struct clk_hw *hws[ST_MAX_CLKS];
    static const struct pci_device_id fch_pci_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, AMD_CPU_ID_ST) },
    { }
    };
#[no_mangle]
unsafe extern "C" fn fch_clk_probe(pdev: *mut platform_device) -> c_int {
    static int fch_clk_probe(struct platform_device *pdev)
    {
    struct fch_clk_data *fch_data;
    struct pci_dev *rdev;
    fch_data = dev_get_platdata(&pdev.dev);
    if (!fch_data || !fch_data.base)
    return -EINVAL;
    rdev = pci_get_domain_bus_and_slot(0, 0, PCI_DEVFN(0, 0));
    if (!rdev) {
    dev_err(&pdev.dev, "FCH device not found\n");
    return -ENODEV;
    }
    if (pci_match_id(fch_pci_ids, rdev)) {
    hws[ST_CLK_48M] = clk_hw_register_fixed_rate(core::ptr::null_mut(), "clk48MHz",
    core::ptr::null_mut(), 0, 48000000);
    hws[ST_CLK_25M] = clk_hw_register_fixed_rate(core::ptr::null_mut(), "clk25MHz",
    core::ptr::null_mut(), 0, 25000000);
    hws[ST_CLK_MUX] = clk_hw_register_mux(core::ptr::null_mut(), "oscout1_mux",
    clk_oscout1_parents, ARRAY_SIZE(clk_oscout1_parents),
    0, fch_data.base + CLKDRVSTR2, OSCOUT1CLK25MHZ, 3, 0,
    core::ptr::null_mut());
    clk_set_parent(hws[ST_CLK_MUX].clk, hws[ST_CLK_48M].clk);
    hws[ST_CLK_GATE] = clk_hw_register_gate(core::ptr::null_mut(), "oscout1",
    "oscout1_mux", 0, fch_data.base + MISCCLKCNTL1,
    OSCCLKENB, CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
    devm_clk_hw_register_clkdev(&pdev.dev, hws[ST_CLK_GATE],
    fch_data.name, core::ptr::null_mut());
    } else {
    hws[CLK_48M_FIXED] = clk_hw_register_fixed_rate(core::ptr::null_mut(), "clk48MHz",
    core::ptr::null_mut(), 0, 48000000);
    hws[CLK_GATE_FIXED] = clk_hw_register_gate(core::ptr::null_mut(), "oscout1",
    "clk48MHz", 0, fch_data.base + MISCCLKCNTL1,
    OSCCLKENB, 0, core::ptr::null_mut());
    devm_clk_hw_register_clkdev(&pdev.dev, hws[CLK_GATE_FIXED],
    fch_data.name, core::ptr::null_mut());
    }
    pci_dev_put(rdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fch_clk_remove(pdev: *mut platform_device) {
    static void fch_clk_remove(struct platform_device *pdev)
    {
    int i, clks;
    struct pci_dev *rdev;
    rdev = pci_get_domain_bus_and_slot(0, 0, PCI_DEVFN(0, 0));
    if (!rdev)
    return;
    clks = pci_match_id(fch_pci_ids, rdev) ? CLK_MAX_FIXED : ST_MAX_CLKS;
    for (i = 0; i < clks; i++)
    clk_hw_unregister(hws[i]);
    pci_dev_put(rdev);
    }
    static struct platform_driver fch_clk_driver = {
    .driver = {
    .name = "clk-fch",
    .suppress_bind_attrs = true,
    },
    .probe = fch_clk_probe,
    .remove = fch_clk_remove,
    };
    builtin_platform_driver(fch_clk_driver);
