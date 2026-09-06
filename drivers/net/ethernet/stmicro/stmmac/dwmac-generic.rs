//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-generic.c
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


//
// Generic DWMAC platform driver
//
// Copyright (C) 2007-2011  STMicroelectronics Ltd
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[no_mangle]
unsafe extern "C" fn dwmac_generic_probe(pdev: *mut platform_device) -> c_int {
    static int dwmac_generic_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    if (pdev.dev.of_node) {
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat)) {
    dev_err(&pdev.dev, "dt configuration failed\n");
    return PTR_ERR(plat_dat);
    }
    } else {
    plat_dat = dev_get_platdata(&pdev.dev);
    if (!plat_dat) {
    dev_err(&pdev.dev, "no platform data provided\n");
    return  -EINVAL;
    }
// Set default value for multicast hash bins
    plat_dat.multicast_filter_bins = HASH_TABLE_SIZE;
// Set default value for unicast filter entries
    plat_dat.unicast_filter_entries = 1;
    }
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id dwmac_generic_match[] = {
    { .compatible = "st,spear600-gmac"},
    { .compatible = "snps,dwmac-3.40a"},
    { .compatible = "snps,dwmac-3.50a"},
    { .compatible = "snps,dwmac-3.610"},
    { .compatible = "snps,dwmac-3.70a"},
    { .compatible = "snps,dwmac-3.710"},
    { .compatible = "snps,dwmac-3.72a"},
    { .compatible = "snps,dwmac-4.00"},
    { .compatible = "snps,dwmac-4.10a"},
    { .compatible = "snps,dwmac"},
    { .compatible = "snps,dwxgmac-2.10"},
    { .compatible = "snps,dwxgmac"},
    { }
    };
    MODULE_DEVICE_TABLE(of, dwmac_generic_match);
    static struct platform_driver dwmac_generic_driver = {
    .probe  = dwmac_generic_probe,
    .driver = {
    .name           = STMMAC_RESOURCE_NAME,
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = dwmac_generic_match,
    },
    };
    module_platform_driver(dwmac_generic_driver);
    MODULE_DESCRIPTION("Generic dwmac driver");
    MODULE_LICENSE("GPL v2");
