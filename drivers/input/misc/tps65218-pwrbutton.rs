//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/tps65218-pwrbutton.c
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
// Texas Instruments' TPS65217 and TPS65218 Power Button Input Driver
//
// Copyright (C) 2014 Texas Instruments Incorporated - http://www.ti.com
// Author: Felipe Balbi <balbi@ti.com>
// Author: Marcin Niestroj <m.niestroj@grinn-global.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6521x_data {
    pub reg_status: c_uint,
    pub pb_mask: c_uint,
    pub name: *const c_char,
}

    static const struct tps6521x_data tps65217_data = {
    .reg_status = TPS65217_REG_STATUS,
    .pb_mask = TPS65217_STATUS_PB,
    .name = "tps65217_pwrbutton",
    };
    static const struct tps6521x_data tps65218_data = {
    .reg_status = TPS65218_REG_STATUS,
    .pb_mask = TPS65218_STATUS_PB_STATE,
    .name = "tps65218_pwrbutton",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6521x_pwrbutton {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub idev: *mut input_dev,
    pub data: *const tps6521x_data,
    pub phys: [c_char; 32],
}

    static const struct of_device_id of_tps6521x_pb_match[] = {
    { .compatible = "ti,tps65217-pwrbutton", .data = &tps65217_data },
    { .compatible = "ti,tps65218-pwrbutton", .data = &tps65218_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_tps6521x_pb_match);
#[no_mangle]
unsafe extern "C" fn tps6521x_pb_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6521x_pb_irq(int irq, void *_pwr)
    {
    struct tps6521x_pwrbutton *pwr = _pwr;
    const struct tps6521x_data *tps_data = pwr.data;
    unsigned int reg;
    int error;
    error = regmap_read(pwr.regmap, tps_data.reg_status, &reg);
    if (error) {
    dev_err(pwr.dev, "can't read register: %d\n", error);
    goto out;
    }
    if (reg & tps_data.pb_mask) {
    input_report_key(pwr.idev, KEY_POWER, 1);
    pm_wakeup_event(pwr.dev, 0);
    } else {
    input_report_key(pwr.idev, KEY_POWER, 0);
    }
    input_sync(pwr.idev);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps6521x_pb_probe(pdev: *mut platform_device) -> c_int {
    static int tps6521x_pb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct tps6521x_pwrbutton *pwr;
    struct input_dev *idev;
    const struct of_device_id *match;
    int error;
    int irq;
    match = of_match_node(of_tps6521x_pb_match, dev.of_node);
    if (!match)
    return -ENXIO;
    pwr = devm_kzalloc(dev, sizeof(*pwr), GFP_KERNEL);
    if (!pwr)
    return -ENOMEM;
    pwr.data = match.data;
    idev = devm_input_allocate_device(dev);
    if (!idev)
    return -ENOMEM;
    idev.name = pwr.data.name;
    snprintf(pwr.phys, sizeof(pwr.phys), "%s/input0",
    pwr.data.name);
    idev.phys = pwr.phys;
    idev.dev.parent = dev;
    idev.id.bustype = BUS_I2C;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    pwr.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    pwr.dev = dev;
    pwr.idev = idev;
    device_init_wakeup(dev, true);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    error = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), tps6521x_pb_irq,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    pwr.data.name, pwr);
    if (error) {
    dev_err(dev, "failed to request IRQ #%d: %d\n", irq, error);
    return error;
    }
    error= input_register_device(idev);
    if (error) {
    dev_err(dev, "Can't register power button: %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct platform_device_id tps6521x_pwrbtn_id_table[] = {
    { "tps65218-pwrbutton", },
    { "tps65217-pwrbutton", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps6521x_pwrbtn_id_table);
    static struct platform_driver tps6521x_pb_driver = {
    .probe	= tps6521x_pb_probe,
    .driver	= {
    .name	= "tps6521x_pwrbutton",
    .of_match_table = of_tps6521x_pb_match,
    },
    .id_table = tps6521x_pwrbtn_id_table,
    };
    module_platform_driver(tps6521x_pb_driver);
    MODULE_DESCRIPTION("TPS6521X Power Button");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Felipe Balbi <balbi@ti.com>");
