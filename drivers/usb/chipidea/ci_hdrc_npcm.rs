//! Automatically rewritten from C to Rust
//! Source: drivers/usb/chipidea/ci_hdrc_npcm.c
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
// Copyright (c) 2023 Nuvoton Technology corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_udc_data {
    pub ci: *mut platform_device,
    pub core_clk: *mut clk,
    pub pdata: ci_hdrc_platform_data,
}

#[no_mangle]
unsafe extern "C" fn npcm_udc_notify_event(ci: *mut ci_hdrc, event: c_uint) -> c_int {
    static int npcm_udc_notify_event(struct ci_hdrc *ci, unsigned int event)
    {
    struct device *dev = ci.dev.parent;
    switch (event) {
    case CI_HDRC_CONTROLLER_RESET_EVENT:
// clear all mode bits
    hw_write(ci, OP_USBMODE, 0xffffffff, 0x0);
    break;
    default:
    dev_dbg(dev, "unknown ci_hdrc event (%d)\n", event);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_udc_probe(pdev: *mut platform_device) -> c_int {
    static int npcm_udc_probe(struct platform_device *pdev)
    {
    int ret;
    struct npcm_udc_data *ci;
    struct platform_device *plat_ci;
    struct device *dev = &pdev.dev;
    ci = devm_kzalloc(&pdev.dev, sizeof(*ci), GFP_KERNEL);
    if (!ci)
    return -ENOMEM;
    platform_set_drvdata(pdev, ci);
    ci.core_clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(ci.core_clk))
    return PTR_ERR(ci.core_clk);
    ret = clk_prepare_enable(ci.core_clk);
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable the clock: %d\n", ret);
    ci.pdata.name = dev_name(dev);
    ci.pdata.capoffset = DEF_CAPOFFSET;
    ci.pdata.flags	= CI_HDRC_REQUIRES_ALIGNED_DMA |
    CI_HDRC_FORCE_VBUS_ACTIVE_ALWAYS;
    ci.pdata.phy_mode = USBPHY_INTERFACE_MODE_UTMI;
    ci.pdata.notify_event = npcm_udc_notify_event;
    plat_ci = ci_hdrc_add_device(dev, pdev.resource, pdev.num_resources,
    &ci.pdata);
    if (IS_ERR(plat_ci)) {
    ret = PTR_ERR(plat_ci);
    dev_err(dev, "failed to register HDRC NPCM device: %d\n", ret);
    goto clk_err;
    }
    pm_runtime_no_callbacks(dev);
    pm_runtime_enable(dev);
    return 0;
    clk_err:
    clk_disable_unprepare(ci.core_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn npcm_udc_remove(pdev: *mut platform_device) {
    static void npcm_udc_remove(struct platform_device *pdev)
    {
    struct npcm_udc_data *ci = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    ci_hdrc_remove_device(ci.ci);
    clk_disable_unprepare(ci.core_clk);
    }
    static const struct of_device_id npcm_udc_dt_match[] = {
    { .compatible = "nuvoton,npcm750-udc", },
    { .compatible = "nuvoton,npcm845-udc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, npcm_udc_dt_match);
    static struct platform_driver npcm_udc_driver = {
    .probe = npcm_udc_probe,
    .remove = npcm_udc_remove,
    .driver = {
    .name = "npcm_udc",
    .of_match_table = npcm_udc_dt_match,
    },
    };
    module_platform_driver(npcm_udc_driver);
    MODULE_DESCRIPTION("NPCM USB device controller driver");
    MODULE_AUTHOR("Tomer Maimon <tomer.maimon@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
