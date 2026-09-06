//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/xilinx-pr-decoupler.c
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
// Copyright (c) 2017, National Instruments Corp.
// Copyright (c) 2017, Xilinx Inc
//
// FPGA Bridge Driver for the Xilinx LogiCORE Partial Reconfiguration
// Decoupler IP Core.
//

pub const CTRL_CMD_COUPLE: c_int = 0;
pub const CTRL_OFFSET: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlnx_config_data {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlnx_pr_decoupler_data {
    pub ipconfig: *const xlnx_config_data,
    pub io_base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline void xlnx_pr_decoupler_write(struct xlnx_pr_decoupler_data *d,
    u32 offset, u32 val)
    {
    writel(val, d.io_base + offset);
    }
    static inline u32 xlnx_pr_decouple_read(const struct xlnx_pr_decoupler_data *d,
    u32 offset)
    {
    return readl(d.io_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn xlnx_pr_decoupler_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    static int xlnx_pr_decoupler_enable_set(struct fpga_bridge *bridge, bool enable)
    {
    int err;
    struct xlnx_pr_decoupler_data *priv = bridge.priv;
    err = clk_enable(priv.clk);
    if (err)
    return err;
    if (enable)
    xlnx_pr_decoupler_write(priv, CTRL_OFFSET, CTRL_CMD_COUPLE);
    else
    xlnx_pr_decoupler_write(priv, CTRL_OFFSET, CTRL_CMD_DECOUPLE);
    clk_disable(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_pr_decoupler_enable_show(bridge: *mut fpga_bridge) -> c_int {
    static int xlnx_pr_decoupler_enable_show(struct fpga_bridge *bridge)
    {
    const struct xlnx_pr_decoupler_data *priv = bridge.priv;
    u32 status;
    int err;
    err = clk_enable(priv.clk);
    if (err)
    return err;
    status = xlnx_pr_decouple_read(priv, CTRL_OFFSET);
    clk_disable(priv.clk);
    return !status;
    }
    static const struct fpga_bridge_ops xlnx_pr_decoupler_br_ops = {
    .enable_set = xlnx_pr_decoupler_enable_set,
    .enable_show = xlnx_pr_decoupler_enable_show,
    };
    static const struct xlnx_config_data decoupler_config = {
    .name = "Xilinx PR Decoupler",
    };
    static const struct xlnx_config_data shutdown_config = {
    .name = "Xilinx DFX AXI Shutdown Manager",
    };
    static const struct of_device_id xlnx_pr_decoupler_of_match[] = {
    { .compatible = "xlnx,pr-decoupler-1.00", .data = &decoupler_config },
    { .compatible = "xlnx,pr-decoupler", .data = &decoupler_config },
    { .compatible = "xlnx,dfx-axi-shutdown-manager-1.00",
    .data = &shutdown_config },
    { .compatible = "xlnx,dfx-axi-shutdown-manager",
    .data = &shutdown_config },
    {},
    };
    MODULE_DEVICE_TABLE(of, xlnx_pr_decoupler_of_match);
#[no_mangle]
unsafe extern "C" fn xlnx_pr_decoupler_probe(pdev: *mut platform_device) -> c_int {
    static int xlnx_pr_decoupler_probe(struct platform_device *pdev)
    {
    struct xlnx_pr_decoupler_data *priv;
    struct fpga_bridge *br;
    int err;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.ipconfig = device_get_match_data(&pdev.dev);
    priv.io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.io_base))
    return PTR_ERR(priv.io_base);
    priv.clk = devm_clk_get_prepared(&pdev.dev, "aclk");
    if (IS_ERR(priv.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.clk),
    "input clock not found\n");
    br = fpga_bridge_register(&pdev.dev, priv.ipconfig.name,
    &xlnx_pr_decoupler_br_ops, priv);
    if (IS_ERR(br)) {
    err = PTR_ERR(br);
    dev_err(&pdev.dev, "unable to register %s",
    priv.ipconfig.name);
    return err;
    }
    platform_set_drvdata(pdev, br);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_pr_decoupler_remove(pdev: *mut platform_device) {
    static void xlnx_pr_decoupler_remove(struct platform_device *pdev)
    {
    struct fpga_bridge *bridge = platform_get_drvdata(pdev);
    fpga_bridge_unregister(bridge);
    }
    static struct platform_driver xlnx_pr_decoupler_driver = {
    .probe = xlnx_pr_decoupler_probe,
    .remove = xlnx_pr_decoupler_remove,
    .driver = {
    .name = "xlnx_pr_decoupler",
    .of_match_table = xlnx_pr_decoupler_of_match,
    },
    };
    module_platform_driver(xlnx_pr_decoupler_driver);
    MODULE_DESCRIPTION("Xilinx Partial Reconfiguration Decoupler");
    MODULE_AUTHOR("Moritz Fischer <mdf@kernel.org>");
    MODULE_AUTHOR("Michal Simek <michal.simek@amd.com>");
    MODULE_LICENSE("GPL v2");
