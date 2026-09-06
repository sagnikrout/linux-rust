//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/phy-omap-otg.c
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
// OMAP OTG controller driver
//
// Based on code from tahvo-usb.c and isp1301_omap.c drivers.
//
// Copyright (C) 2005-2006 Nokia Corporation
// Copyright (C) 2004 Texas Instruments
// Copyright (C) 2004 David Brownell
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otg_device {
    pub base: *mut void __iomem,
    pub id: bool,
    pub vbus: bool,
    pub extcon: *mut extcon_dev,
    pub vbus_nb: notifier_block,
    pub id_nb: notifier_block,
}

pub const OMAP_OTG_CTRL: c_uint = 0x0c;

    (OMAP_OTG_ASESSVLD | OMAP_OTG_BSESSEND | OMAP_OTG_BSESSVLD | \
    OMAP_OTG_VBUSVLD  | OMAP_OTG_ID)
#[no_mangle]
unsafe extern "C" fn omap_otg_ctrl(otg_dev: *mut otg_device, outputs: u32) {
    static void omap_otg_ctrl(struct otg_device *otg_dev, u32 outputs)
    {
    u32 l;
    l = readl(otg_dev.base + OMAP_OTG_CTRL);
    l &= ~OMAP_OTG_XCEIV_OUTPUTS;
    l |= outputs;
    writel(l, otg_dev.base + OMAP_OTG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn omap_otg_set_mode(otg_dev: *mut otg_device) {
    static void omap_otg_set_mode(struct otg_device *otg_dev)
    {
    if (!otg_dev.id && otg_dev.vbus)
// Set B-session valid.
    omap_otg_ctrl(otg_dev, OMAP_OTG_ID | OMAP_OTG_BSESSVLD);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: otg_dev->vbus) -> else {
    else if (otg_dev.vbus)
// Set A-session valid.
    omap_otg_ctrl(otg_dev, OMAP_OTG_ASESSVLD);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !otg_dev->id) -> else {
    else if (!otg_dev.id)
// Set B-session end to indicate no VBUS.
    omap_otg_ctrl(otg_dev, OMAP_OTG_ID | OMAP_OTG_BSESSEND);
    }
    static int omap_otg_id_notifier(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct otg_device *otg_dev = container_of(nb, struct otg_device, id_nb);
    otg_dev.id = event;
    omap_otg_set_mode(otg_dev);
    return NOTIFY_DONE;
    }
    static int omap_otg_vbus_notifier(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct otg_device *otg_dev = container_of(nb, struct otg_device,
    vbus_nb);
    otg_dev.vbus = event;
    omap_otg_set_mode(otg_dev);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn omap_otg_probe(pdev: *mut platform_device) -> c_int {
    static int omap_otg_probe(struct platform_device *pdev)
    {
    const struct omap_usb_config *config = pdev.dev.platform_data;
    struct otg_device *otg_dev;
    struct extcon_dev *extcon;
    int ret;
    u32 rev;
    if (!config || !config.extcon)
    return -ENODEV;
    extcon = extcon_get_extcon_dev(config.extcon);
    if (IS_ERR(extcon))
    return PTR_ERR(extcon);
    otg_dev = devm_kzalloc(&pdev.dev, sizeof(*otg_dev), GFP_KERNEL);
    if (!otg_dev)
    return -ENOMEM;
    otg_dev.base = devm_ioremap_resource(&pdev.dev, &pdev.resource[0]);
    if (IS_ERR(otg_dev.base))
    return PTR_ERR(otg_dev.base);
    otg_dev.extcon = extcon;
    otg_dev.id_nb.notifier_call = omap_otg_id_notifier;
    otg_dev.vbus_nb.notifier_call = omap_otg_vbus_notifier;
    ret = devm_extcon_register_notifier(&pdev.dev, extcon,
    EXTCON_USB_HOST, &otg_dev.id_nb);
    if (ret)
    return ret;
    ret = devm_extcon_register_notifier(&pdev.dev, extcon,
    EXTCON_USB, &otg_dev.vbus_nb);
    if (ret) {
    return ret;
    }
    otg_dev.id = extcon_get_state(extcon, EXTCON_USB_HOST);
    otg_dev.vbus = extcon_get_state(extcon, EXTCON_USB);
    omap_otg_set_mode(otg_dev);
    rev = readl(otg_dev.base);
    dev_info(&pdev.dev,
    "OMAP USB OTG controller rev %d.%d (%s, id=%d, vbus=%d)\n",
    (rev >> 4) & 0xf, rev & 0xf, config.extcon, otg_dev.id,
    otg_dev.vbus);
    platform_set_drvdata(pdev, otg_dev);
    return 0;
    }
    static struct platform_driver omap_otg_driver = {
    .probe		= omap_otg_probe,
    .driver		= {
    .name	= "omap_otg",
    },
    };
    module_platform_driver(omap_otg_driver);
    MODULE_DESCRIPTION("OMAP USB OTG controller driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Aaro Koskinen <aaro.koskinen@iki.fi>");
