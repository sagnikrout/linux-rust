//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-mmp3-hsic.c
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
// Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
//

pub const HSIC_CTRL: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp3_hsic_data {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn mmp3_hsic_phy_init(phy: *mut phy) -> c_int {
    static int mmp3_hsic_phy_init(struct phy *phy)
    {
    struct mmp3_hsic_data *mmp3 = phy_get_drvdata(phy);
    u32 hsic_ctrl;
    hsic_ctrl = readl_relaxed(mmp3.base + HSIC_CTRL);
    hsic_ctrl |= HSIC_ENABLE;
    hsic_ctrl |= PLL_BYPASS;
    writel_relaxed(hsic_ctrl, mmp3.base + HSIC_CTRL);
    return 0;
    }
    static const struct phy_ops mmp3_hsic_phy_ops = {
    .init		= mmp3_hsic_phy_init,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id mmp3_hsic_phy_of_match[] = {
    { .compatible = "marvell,mmp3-hsic-phy", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mmp3_hsic_phy_of_match);
#[no_mangle]
unsafe extern "C" fn mmp3_hsic_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mmp3_hsic_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mmp3_hsic_data *mmp3;
    struct phy_provider *provider;
    struct phy *phy;
    mmp3 = devm_kzalloc(dev, sizeof(*mmp3), GFP_KERNEL);
    if (!mmp3)
    return -ENOMEM;
    mmp3.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(mmp3.base))
    return PTR_ERR(mmp3.base);
    phy = devm_phy_create(dev, core::ptr::null_mut(), &mmp3_hsic_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, mmp3);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "failed to register PHY provider\n");
    return PTR_ERR(provider);
    }
    return 0;
    }
    static struct platform_driver mmp3_hsic_phy_driver = {
    .probe		= mmp3_hsic_phy_probe,
    .driver		= {
    .name	= "mmp3-hsic-phy",
    .of_match_table = mmp3_hsic_phy_of_match,
    },
    };
    module_platform_driver(mmp3_hsic_phy_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("Marvell MMP3 USB HSIC PHY Driver");
    MODULE_LICENSE("GPL");
