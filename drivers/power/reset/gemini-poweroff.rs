//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/gemini-poweroff.c
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
// Gemini power management controller
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//
// Inspired by code from the SL3516 board support by Jason Lee
// Inspired by code from Janos Laube <janos.dev@gmail.com>
//

pub const GEMINI_PWC_ID: c_uint = 0x00010500;
pub const GEMINI_PWC_IDREG: c_uint = 0x00;
pub const GEMINI_PWC_CTRLREG: c_uint = 0x04;
pub const GEMINI_PWC_STATREG: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gemini_powercon {
    pub dev: *mut device,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn gemini_powerbutton_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t gemini_powerbutton_interrupt(int irq, void *data)
    {
    struct gemini_powercon *gpw = data;
    u32 val;
// ACK the IRQ
    val = readl(gpw.base + GEMINI_PWC_CTRLREG);
    val |= GEMINI_CTRL_IRQ_CLR;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
    val = readl(gpw.base + GEMINI_PWC_STATREG);
    val &= 0x70U;
    switch (val) {
    case GEMINI_STAT_CIR:
//
// We do not yet have a driver for the infrared
// controller so it can cause spurious poweroff
// events. Ignore those for now.
//
    dev_info(gpw.dev, "infrared poweroff - ignored\n");
    break;
    case GEMINI_STAT_RTC:
    dev_info(gpw.dev, "RTC poweroff\n");
    orderly_poweroff(true);
    break;
    case GEMINI_STAT_POWERBUTTON:
    dev_info(gpw.dev, "poweroff button pressed\n");
    orderly_poweroff(true);
    break;
    default:
    dev_info(gpw.dev, "other power management IRQ\n");
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gemini_poweroff(data: *mut sys_off_data) -> c_int {
    static int gemini_poweroff(struct sys_off_data *data)
    {
    struct gemini_powercon *gpw = data.cb_data;
    u32 val;
    dev_crit(gpw.dev, "Gemini power off\n");
    val = readl(gpw.base + GEMINI_PWC_CTRLREG);
    val |= GEMINI_CTRL_ENABLE | GEMINI_CTRL_IRQ_CLR;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
    val &= ~GEMINI_CTRL_ENABLE;
    val |= GEMINI_CTRL_SHUTDOWN;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn gemini_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int gemini_poweroff_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gemini_powercon *gpw;
    u32 val;
    int irq;
    int ret;
    gpw = devm_kzalloc(dev, sizeof(*gpw), GFP_KERNEL);
    if (!gpw)
    return -ENOMEM;
    gpw.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gpw.base))
    return PTR_ERR(gpw.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    gpw.dev = dev;
    val = readl(gpw.base + GEMINI_PWC_IDREG);
    val &= 0xFFFFFF00U;
    if (val != GEMINI_PWC_ID) {
    dev_err(dev, "wrong power controller ID: %08x\n",
    val);
    return -ENODEV;
    }
//
// Enable the power controller. This is crucial on Gemini
// systems: if this is not done, pressing the power button
// will result in unconditional poweroff without any warning.
// This makes the kernel handle the poweroff.
//
    val = readl(gpw.base + GEMINI_PWC_CTRLREG);
    val |= GEMINI_CTRL_ENABLE;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
// Clear the IRQ
    val = readl(gpw.base + GEMINI_PWC_CTRLREG);
    val |= GEMINI_CTRL_IRQ_CLR;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
// Wait for this to clear
    val = readl(gpw.base + GEMINI_PWC_STATREG);
    while (val & 0x70U)
    val = readl(gpw.base + GEMINI_PWC_STATREG);
// Clear the IRQ again
    val = readl(gpw.base + GEMINI_PWC_CTRLREG);
    val |= GEMINI_CTRL_IRQ_CLR;
    writel(val, gpw.base + GEMINI_PWC_CTRLREG);
    ret = devm_request_irq(dev, irq, gemini_powerbutton_interrupt, 0,
    "poweroff", gpw);
    if (ret)
    return ret;
    ret = devm_register_sys_off_handler(dev, SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    gemini_poweroff, gpw);
    if (ret)
    return ret;
    dev_info(dev, "Gemini poweroff driver registered\n");
    return 0;
    }
    static const struct of_device_id gemini_poweroff_of_match[] = {
    {
    .compatible = "cortina,gemini-power-controller",
    },
    {}
    };
    static struct platform_driver gemini_poweroff_driver = {
    .probe = gemini_poweroff_probe,
    .driver = {
    .name = "gemini-poweroff",
    .of_match_table = gemini_poweroff_of_match,
    },
    };
    builtin_platform_driver(gemini_poweroff_driver);
