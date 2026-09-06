//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/altera-fpga2sdram.c
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
// FPGA to SDRAM Bridge Driver for Altera SoCFPGA Devices
//
// Copyright (C) 2013-2016 Altera Corporation, All Rights Reserved.
//
// This driver manages a bridge between an FPGA and the SDRAM used by the ARM
// host processor system (HPS).
//
// The bridge contains 4 read ports, 4 write ports, and 6 command ports.
// Reconfiguring these ports requires that no SDRAM transactions occur during
// reconfiguration.  The code reconfiguring the ports cannot run out of SDRAM
// nor can the FPGA access the SDRAM during reconfiguration.  This driver does
// not support reconfiguring the ports.  The ports are configured by code
// running out of on chip ram before Linux is started and the configuration
// is passed in a handoff register in the system manager.
//
// This driver supports enabling and disabling of the configured ports, which
// allows for safe reprogramming of the FPGA, assuming that the new FPGA image
// uses the same port configuration.  Bridges must be disabled before
// reprogramming the FPGA and re-enabled after the FPGA has been programmed.
//

pub const ALT_SDR_CTL_FPGAPORTRST_OFST: c_uint = 0x80;
pub const ALT_SDR_CTL_FPGAPORTRST_PORTRSTN_MSK: c_uint = 0x00003fff;
pub const ALT_SDR_CTL_FPGAPORTRST_RD_SHIFT: c_int = 0;
pub const ALT_SDR_CTL_FPGAPORTRST_WR_SHIFT: c_int = 4;
pub const ALT_SDR_CTL_FPGAPORTRST_CTRL_SHIFT: c_int = 8;
//
// From the Cyclone V HPS Memory Map document:
// These registers are used to store handoff information between the
// preloader and the OS. These 8 registers can be used to store any
// information. The contents of these registers have no impact on
// the state of the HPS hardware.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_fpga2sdram_data {
    pub dev: *mut device,
    pub sdrctl: *mut regmap,
    pub mask: c_int,
}

#[no_mangle]
unsafe extern "C" fn alt_fpga2sdram_enable_show(bridge: *mut fpga_bridge) -> c_int {
    static int alt_fpga2sdram_enable_show(struct fpga_bridge *bridge)
    {
    struct alt_fpga2sdram_data *priv = bridge.priv;
    int value;
    regmap_read(priv.sdrctl, ALT_SDR_CTL_FPGAPORTRST_OFST, &value);
    return (value & priv.mask) == priv.mask;
    }
    static inline int _alt_fpga2sdram_enable_set(struct alt_fpga2sdram_data *priv,
    bool enable)
    {
    return regmap_update_bits(priv.sdrctl, ALT_SDR_CTL_FPGAPORTRST_OFST,
    priv.mask, enable ? priv.mask : 0);
    }
#[no_mangle]
unsafe extern "C" fn alt_fpga2sdram_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    static int alt_fpga2sdram_enable_set(struct fpga_bridge *bridge, bool enable)
    {
    return _alt_fpga2sdram_enable_set(bridge.priv, enable);
    }
    static const struct fpga_bridge_ops altera_fpga2sdram_br_ops = {
    .enable_set = alt_fpga2sdram_enable_set,
    .enable_show = alt_fpga2sdram_enable_show,
    };
    static const struct of_device_id altera_fpga_of_match[] = {
    { .compatible = "altr,socfpga-fpga2sdram-bridge" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn alt_fpga_bridge_probe(pdev: *mut platform_device) -> c_int {
    static int alt_fpga_bridge_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct alt_fpga2sdram_data *priv;
    struct fpga_bridge *br;
    u32 enable;
    struct regmap *sysmgr;
    let mut ret: c_int = 0;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.sdrctl = syscon_regmap_lookup_by_compatible("altr,sdr-ctl");
    if (IS_ERR(priv.sdrctl)) {
    dev_err(dev, "regmap for altr,sdr-ctl lookup failed.\n");
    return PTR_ERR(priv.sdrctl);
    }
    sysmgr = syscon_regmap_lookup_by_compatible("altr,sys-mgr");
    if (IS_ERR(sysmgr)) {
    dev_err(dev, "regmap for altr,sys-mgr lookup failed.\n");
    return PTR_ERR(sysmgr);
    }
// Get f2s bridge configuration saved in handoff register
    regmap_read(sysmgr, SYSMGR_ISWGRP_HANDOFF3, &priv.mask);
    br = fpga_bridge_register(dev, F2S_BRIDGE_NAME,
    &altera_fpga2sdram_br_ops, priv);
    if (IS_ERR(br))
    return PTR_ERR(br);
    platform_set_drvdata(pdev, br);
    dev_info(dev, "driver initialized with handoff %08x\n", priv.mask);
    if (!of_property_read_u32(dev.of_node, "bridge-enable", &enable)) {
    if (enable > 1) {
    dev_warn(dev, "invalid bridge-enable %u > 1\n", enable);
    } else {
    dev_info(dev, "%s bridge\n",
    (enable ? "enabling" : "disabling"));
    ret = _alt_fpga2sdram_enable_set(priv, enable);
    if (ret) {
    fpga_bridge_unregister(br);
    return ret;
    }
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alt_fpga_bridge_remove(pdev: *mut platform_device) {
    static void alt_fpga_bridge_remove(struct platform_device *pdev)
    {
    struct fpga_bridge *br = platform_get_drvdata(pdev);
    fpga_bridge_unregister(br);
    }
    MODULE_DEVICE_TABLE(of, altera_fpga_of_match);
    static struct platform_driver altera_fpga_driver = {
    .probe = alt_fpga_bridge_probe,
    .remove = alt_fpga_bridge_remove,
    .driver = {
    .name	= "altera_fpga2sdram_bridge",
    .of_match_table = of_match_ptr(altera_fpga_of_match),
    },
    };
    module_platform_driver(altera_fpga_driver);
    MODULE_DESCRIPTION("Altera SoCFPGA FPGA to SDRAM Bridge");
    MODULE_AUTHOR("Alan Tull <atull@opensource.altera.com>");
    MODULE_LICENSE("GPL v2");
