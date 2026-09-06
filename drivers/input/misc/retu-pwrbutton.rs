//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/retu-pwrbutton.c
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
// Retu power button driver.
//
// Copyright (C) 2004-2010 Nokia Corporation
//
// Original code written by Ari Saastamoinen, Juha Yrjölä and Felipe Balbi.
// Rewritten by Aaro Koskinen.
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//

#[no_mangle]
unsafe extern "C" fn retu_pwrbutton_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t retu_pwrbutton_irq(int irq, void *_pwr)
    {
    struct input_dev *idev = _pwr;
    struct retu_dev *rdev = input_get_drvdata(idev);
    bool state;
    state = !(retu_read(rdev, RETU_REG_STATUS) & RETU_STATUS_PWRONX);
    input_report_key(idev, KEY_POWER, state);
    input_sync(idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn retu_pwrbutton_probe(pdev: *mut platform_device) -> c_int {
    static int retu_pwrbutton_probe(struct platform_device *pdev)
    {
    struct retu_dev *rdev = dev_get_drvdata(pdev.dev.parent);
    struct input_dev *idev;
    int irq;
    int error;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    idev = devm_input_allocate_device(&pdev.dev);
    if (!idev)
    return -ENOMEM;
    idev.name = "retu-pwrbutton";
    idev.dev.parent = &pdev.dev;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    input_set_drvdata(idev, rdev);
    error = devm_request_threaded_irq(&pdev.dev, irq,
    core::ptr::null_mut(), retu_pwrbutton_irq,
    IRQF_ONESHOT,
    "retu-pwrbutton", idev);
    if (error)
    return error;
    error = input_register_device(idev);
    if (error)
    return error;
    return 0;
    }
    static struct platform_driver retu_pwrbutton_driver = {
    .probe		= retu_pwrbutton_probe,
    .driver		= {
    .name	= "retu-pwrbutton",
    },
    };
    module_platform_driver(retu_pwrbutton_driver);
    MODULE_ALIAS("platform:retu-pwrbutton");
    MODULE_DESCRIPTION("Retu Power Button");
    MODULE_AUTHOR("Ari Saastamoinen");
    MODULE_AUTHOR("Felipe Balbi");
    MODULE_AUTHOR("Aaro Koskinen <aaro.koskinen@iki.fi>");
    MODULE_LICENSE("GPL");
