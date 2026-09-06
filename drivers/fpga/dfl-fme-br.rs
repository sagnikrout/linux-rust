//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-fme-br.c
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
//
// FPGA Bridge Driver for FPGA Management Engine (FME)
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Wu Hao <hao.wu@intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Henry Mitchel <henry.mitchel@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fme_br_priv {
    pub pdata: *mut dfl_fme_br_pdata,
    pub port_ops: *mut dfl_fpga_port_ops,
    pub port_fdata: *mut dfl_feature_dev_data,
}

#[no_mangle]
unsafe extern "C" fn fme_bridge_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    static int fme_bridge_enable_set(struct fpga_bridge *bridge, bool enable)
    {
    struct fme_br_priv *priv = bridge.priv;
    struct dfl_feature_dev_data *port_fdata;
    struct dfl_fpga_port_ops *ops;
    if (!priv.port_fdata) {
    port_fdata = dfl_fpga_cdev_find_port_data(priv.pdata.cdev,
    &priv.pdata.port_id,
    dfl_fpga_check_port_id);
    if (!port_fdata)
    return -ENODEV;
    priv.port_fdata = port_fdata;
    }
    if (priv.port_fdata && !priv.port_ops) {
    ops = dfl_fpga_port_ops_get(priv.port_fdata);
    if (!ops || !ops.enable_set)
    return -ENOENT;
    priv.port_ops = ops;
    }
    return priv.port_ops.enable_set(priv.port_fdata, enable);
    }
    static const struct fpga_bridge_ops fme_bridge_ops = {
    .enable_set = fme_bridge_enable_set,
    };
#[no_mangle]
unsafe extern "C" fn fme_br_probe(pdev: *mut platform_device) -> c_int {
    static int fme_br_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct fme_br_priv *priv;
    struct fpga_bridge *br;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.pdata = dev_get_platdata(dev);
    br = fpga_bridge_register(dev, "DFL FPGA FME Bridge",
    &fme_bridge_ops, priv);
    if (IS_ERR(br))
    return PTR_ERR(br);
    platform_set_drvdata(pdev, br);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fme_br_remove(pdev: *mut platform_device) {
    static void fme_br_remove(struct platform_device *pdev)
    {
    struct fpga_bridge *br = platform_get_drvdata(pdev);
    struct fme_br_priv *priv = br.priv;
    fpga_bridge_unregister(br);
    if (priv.port_ops)
    dfl_fpga_port_ops_put(priv.port_ops);
    }
    static struct platform_driver fme_br_driver = {
    .driver = {
    .name = DFL_FPGA_FME_BRIDGE,
    },
    .probe = fme_br_probe,
    .remove = fme_br_remove,
    };
    module_platform_driver(fme_br_driver);
    MODULE_DESCRIPTION("FPGA Bridge for DFL FPGA Management Engine");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:dfl-fme-bridge");
