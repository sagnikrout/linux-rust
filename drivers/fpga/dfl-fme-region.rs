//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-fme-region.c
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
// FPGA Region Driver for FPGA Management Engine (FME)
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

#[no_mangle]
unsafe extern "C" fn fme_region_get_bridges(region: *mut fpga_region) -> c_int {
    static int fme_region_get_bridges(struct fpga_region *region)
    {
    struct dfl_fme_region_pdata *pdata = region.priv;
    struct device *dev = &pdata.br.dev;
    return fpga_bridge_get_to_list(dev, region.info, &region.bridge_list);
    }
#[no_mangle]
unsafe extern "C" fn fme_region_probe(pdev: *mut platform_device) -> c_int {
    static int fme_region_probe(struct platform_device *pdev)
    {
    struct dfl_fme_region_pdata *pdata = dev_get_platdata(&pdev.dev);
    let mut info: fpga_region_info = { 0 };
    struct device *dev = &pdev.dev;
    struct fpga_region *region;
    struct fpga_manager *mgr;
    int ret;
    mgr = fpga_mgr_get(&pdata.mgr.dev);
    if (IS_ERR(mgr))
    return -EPROBE_DEFER;
    info.mgr = mgr;
    info.compat_id = mgr.compat_id;
    info.get_bridges = fme_region_get_bridges;
    info.priv = pdata;
    region = fpga_region_register_full(dev, &info);
    if (IS_ERR(region)) {
    ret = PTR_ERR(region);
    goto eprobe_mgr_put;
    }
    platform_set_drvdata(pdev, region);
    dev_dbg(dev, "DFL FME FPGA Region probed\n");
    return 0;
    eprobe_mgr_put:
    fpga_mgr_put(mgr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fme_region_remove(pdev: *mut platform_device) {
    static void fme_region_remove(struct platform_device *pdev)
    {
    struct fpga_region *region = platform_get_drvdata(pdev);
    struct fpga_manager *mgr = region.mgr;
    fpga_region_unregister(region);
    fpga_mgr_put(mgr);
    }
    static struct platform_driver fme_region_driver = {
    .driver = {
    .name = DFL_FPGA_FME_REGION,
    },
    .probe = fme_region_probe,
    .remove = fme_region_remove,
    };
    module_platform_driver(fme_region_driver);
    MODULE_DESCRIPTION("FPGA Region for DFL FPGA Management Engine");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:dfl-fme-region");
