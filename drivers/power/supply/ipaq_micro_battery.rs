//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/ipaq_micro_battery.c
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
// h3xxx atmel micro companion support, battery subdevice
// based on previous kernel 2.4 version
// Author : Alessandro Gardich <gremlin@gremlin.it>
// Author : Linus Walleij <linus.walleij@linaro.org>
//

pub const MICRO_BATT_CHEM_ALKALINE: c_uint = 0x01;
pub const MICRO_BATT_CHEM_NICD: c_uint = 0x02;
pub const MICRO_BATT_CHEM_NIMH: c_uint = 0x03;
pub const MICRO_BATT_CHEM_LION: c_uint = 0x04;
pub const MICRO_BATT_CHEM_LIPOLY: c_uint = 0x05;
pub const MICRO_BATT_CHEM_NOT_INSTALLED: c_uint = 0x06;
pub const MICRO_BATT_CHEM_UNKNOWN: c_uint = 0xff;
pub const MICRO_BATT_STATUS_HIGH: c_uint = 0x01;
pub const MICRO_BATT_STATUS_LOW: c_uint = 0x02;
pub const MICRO_BATT_STATUS_CRITICAL: c_uint = 0x04;
pub const MICRO_BATT_STATUS_CHARGING: c_uint = 0x08;
pub const MICRO_BATT_STATUS_CHARGEMAIN: c_uint = 0x10;
pub const MICRO_BATT_STATUS_DEAD: c_uint = 0x20 /* Battery will not charge */;
pub const MICRO_BATT_STATUS_NOTINSTALLED: c_uint = 0x20 /* For expansion pack batteries */;
pub const MICRO_BATT_STATUS_FULL: c_uint = 0x40 /* Battery fully charged */;
pub const MICRO_BATT_STATUS_NOBATTERY: c_uint = 0x80;
pub const MICRO_BATT_STATUS_UNKNOWN: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct micro_battery {
    pub micro: *mut ipaq_micro,
    pub wq: *mut workqueue_struct,
    pub update: delayed_work,
    pub ac: u8,
    pub chemistry: u8,
    pub voltage: c_uint,
    pub temperature: u16,
    pub flag: u8,
}

#[no_mangle]
unsafe extern "C" fn micro_battery_work(work: *mut work_struct) {
    static void micro_battery_work(struct work_struct *work)
    {
    struct micro_battery *mb = container_of(work,
    struct micro_battery, update.work);
    struct ipaq_micro_msg msg_battery = {
    .id = MSG_BATTERY,
    };
    struct ipaq_micro_msg msg_sensor = {
    .id = MSG_THERMAL_SENSOR,
    };
// First send battery message
    ipaq_micro_tx_msg_sync(mb.micro, &msg_battery);
    if (msg_battery.rx_len < 4)
    pr_info("ERROR");
//
// Returned message format:
// byte 0:   0x00 = Not plugged in
// 0x01 = AC adapter plugged in
// byte 1:   chemistry
// byte 2:   voltage LSB
// byte 3:   voltage MSB
// byte 4:   flags
// byte 5-9: same for battery 2
//
    mb.ac = msg_battery.rx_data[0];
    mb.chemistry = msg_battery.rx_data[1];
    mb.voltage = ((((unsigned short)msg_battery.rx_data[3] << 8) +
    msg_battery.rx_data[2]) * 5000L) * 1000 / 1024;
    mb.flag = msg_battery.rx_data[4];
    if (msg_battery.rx_len == 9)
    pr_debug("second battery ignored\n");
// Then read the sensor
    ipaq_micro_tx_msg_sync(mb.micro, &msg_sensor);
    mb.temperature = msg_sensor.rx_data[1] << 8 | msg_sensor.rx_data[0];
    queue_delayed_work(mb.wq, &mb.update, msecs_to_jiffies(BATT_PERIOD));
    }
#[no_mangle]
unsafe extern "C" fn get_capacity(b: *mut power_supply) -> c_int {
    static int get_capacity(struct power_supply *b)
    {
    struct micro_battery *mb = dev_get_drvdata(b.dev.parent);
    switch (mb.flag & 0x07) {
    case MICRO_BATT_STATUS_HIGH:
    return 100;
    break;
    case MICRO_BATT_STATUS_LOW:
    return 50;
    break;
    case MICRO_BATT_STATUS_CRITICAL:
    return 5;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_status(b: *mut power_supply) -> c_int {
    static int get_status(struct power_supply *b)
    {
    struct micro_battery *mb = dev_get_drvdata(b.dev.parent);
    if (mb.flag == MICRO_BATT_STATUS_UNKNOWN)
    return POWER_SUPPLY_STATUS_UNKNOWN;
    if (mb.flag & MICRO_BATT_STATUS_FULL)
    return POWER_SUPPLY_STATUS_FULL;
    if ((mb.flag & MICRO_BATT_STATUS_CHARGING) ||
    (mb.flag & MICRO_BATT_STATUS_CHARGEMAIN))
    return POWER_SUPPLY_STATUS_CHARGING;
    return POWER_SUPPLY_STATUS_DISCHARGING;
    }
    static int micro_batt_get_property(struct power_supply *b,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct micro_battery *mb = dev_get_drvdata(b.dev.parent);
    switch (psp) {
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    switch (mb.chemistry) {
    case MICRO_BATT_CHEM_NICD:
    val.intval = POWER_SUPPLY_TECHNOLOGY_NiCd;
    break;
    case MICRO_BATT_CHEM_NIMH:
    val.intval = POWER_SUPPLY_TECHNOLOGY_NiMH;
    break;
    case MICRO_BATT_CHEM_LION:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LION;
    break;
    case MICRO_BATT_CHEM_LIPOLY:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LIPO;
    break;
    default:
    val.intval = POWER_SUPPLY_TECHNOLOGY_UNKNOWN;
    break;
    }
    break;
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = get_status(b);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    val.intval = 4700000;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    val.intval = get_capacity(b);
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval = mb.temperature;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = mb.voltage;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int micro_ac_get_property(struct power_supply *b,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct micro_battery *mb = dev_get_drvdata(b.dev.parent);
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = mb.ac;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static enum power_supply_property micro_batt_power_props[] = {
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    };
    static const struct power_supply_desc micro_batt_power_desc = {
    .name			= "main-battery",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= micro_batt_power_props,
    .num_properties		= ARRAY_SIZE(micro_batt_power_props),
    .get_property		= micro_batt_get_property,
    .use_for_apm		= 1,
    };
    static enum power_supply_property micro_ac_power_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc micro_ac_power_desc = {
    .name			= "ac",
    .type			= POWER_SUPPLY_TYPE_MAINS,
    .properties		= micro_ac_power_props,
    .num_properties		= ARRAY_SIZE(micro_ac_power_props),
    .get_property		= micro_ac_get_property,
    };
    static struct power_supply *micro_batt_power, *micro_ac_power;
#[no_mangle]
unsafe extern "C" fn micro_batt_probe(pdev: *mut platform_device) -> c_int {
    static int micro_batt_probe(struct platform_device *pdev)
    {
    struct micro_battery *mb;
    int ret;
    mb = devm_kzalloc(&pdev.dev, sizeof(*mb), GFP_KERNEL);
    if (!mb)
    return -ENOMEM;
    mb.micro = dev_get_drvdata(pdev.dev.parent);
    mb.wq = devm_alloc_workqueue(&pdev.dev, "ipaq-battery-wq",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!mb.wq)
    return -ENOMEM;
    ret = devm_delayed_work_autocancel(&pdev.dev, &mb.update, micro_battery_work);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, mb);
    queue_delayed_work(mb.wq, &mb.update, 1);
    micro_batt_power = devm_power_supply_register(&pdev.dev,
    &micro_batt_power_desc,
    core::ptr::null_mut());
    if (IS_ERR(micro_batt_power))
    return PTR_ERR(micro_batt_power);
    micro_ac_power = devm_power_supply_register(&pdev.dev,
    &micro_ac_power_desc, core::ptr::null_mut());
    if (IS_ERR(micro_ac_power))
    return PTR_ERR(micro_ac_power);
    dev_info(&pdev.dev, "iPAQ micro battery driver\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_batt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused micro_batt_suspend(struct device *dev)
    {
    struct micro_battery *mb = dev_get_drvdata(dev);
    cancel_delayed_work_sync(&mb.update);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_batt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused micro_batt_resume(struct device *dev)
    {
    struct micro_battery *mb = dev_get_drvdata(dev);
    queue_delayed_work(mb.wq, &mb.update, msecs_to_jiffies(BATT_PERIOD));
    return 0;
    }
    static const struct dev_pm_ops micro_batt_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(micro_batt_suspend, micro_batt_resume)
    };
    static struct platform_driver micro_batt_device_driver = {
    .driver		= {
    .name	= "ipaq-micro-battery",
    .pm	= &micro_batt_dev_pm_ops,
    },
    .probe		= micro_batt_probe,
    };
    module_platform_driver(micro_batt_device_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("driver for iPAQ Atmel micro battery");
    MODULE_ALIAS("platform:ipaq-micro-battery");
