//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/tps65217_charger.c
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
// Battery charger driver for TI's tps65217
//
// Copyright (C) 2015 Collabora Ltd.
// Author: Enric Balletbo i Serra <enric.balletbo@collabora.com>
//
// Battery charger driver for TI's tps65217
//

pub const NUM_CHARGER_IRQS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65217_charger {
    pub tps: *mut tps65217,
    pub dev: *mut device,
    pub psy: *mut power_supply,
    pub online: c_int,
    pub prev_online: c_int,
    pub poll_task: *mut task_struct,
}

    static enum power_supply_property tps65217_charger_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
#[no_mangle]
unsafe extern "C" fn tps65217_config_charger(charger: *mut tps65217_charger) -> c_int {
    static int tps65217_config_charger(struct tps65217_charger *charger)
    {
    int ret;
//
// tps65217 rev. G, p. 31 (see p. 32 for NTC schematic)
//
// The device can be configured to support a 100k NTC (B = 3960) by
// setting the NTC_TYPE bit in register CHGCONFIG1 to 1. However it
// is not recommended to do so. In sleep mode, the charger continues
// charging the battery, but all register values are reset to default
// values. Therefore, the charger would get the wrong temperature
// information. If 100k NTC setting is required, please contact the
// factory.
//
// ATTENTION, conflicting information, from p. 46
//
// NTC TYPE (for battery temperature measurement)
// 0 – 100k (curve 1, B = 3960)
// 1 – 10k  (curve 2, B = 3480) (default on reset)
//
    ret = tps65217_clear_bits(charger.tps, TPS65217_REG_CHGCONFIG1,
    TPS65217_CHGCONFIG1_NTC_TYPE,
    TPS65217_PROTECT_NONE);
    if (ret) {
    dev_err(charger.dev,
    "failed to set 100k NTC setting: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_enable_charging(charger: *mut tps65217_charger) -> c_int {
    static int tps65217_enable_charging(struct tps65217_charger *charger)
    {
    int ret;
// charger already enabled
    if (charger.online)
    return 0;
    dev_dbg(charger.dev, "%s: enable charging\n", __func__);
    ret = tps65217_set_bits(charger.tps, TPS65217_REG_CHGCONFIG1,
    TPS65217_CHGCONFIG1_CHG_EN,
    TPS65217_CHGCONFIG1_CHG_EN,
    TPS65217_PROTECT_NONE);
    if (ret) {
    dev_err(charger.dev,
    "%s: Error in writing CHG_EN in reg 0x%x: %d\n",
    __func__, TPS65217_REG_CHGCONFIG1, ret);
    return ret;
    }
    charger.online = 1;
    return 0;
    }
    static int tps65217_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct tps65217_charger *charger = power_supply_get_drvdata(psy);
    if (psp == POWER_SUPPLY_PROP_ONLINE) {
    val.intval = charger.online;
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_charger_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps65217_charger_irq(int irq, void *dev)
    {
    int ret, val;
    struct tps65217_charger *charger = dev;
    charger.prev_online = charger.online;
    ret = tps65217_reg_read(charger.tps, TPS65217_REG_STATUS, &val);
    if (ret < 0) {
    dev_err(charger.dev, "%s: Error in reading reg 0x%x\n",
    __func__, TPS65217_REG_STATUS);
    return IRQ_HANDLED;
    }
    dev_dbg(charger.dev, "%s: 0x%x\n", __func__, val);
// check for charger status bit
    if (val & CHARGER_STATUS_PRESENT) {
    ret = tps65217_enable_charging(charger);
    if (ret) {
    dev_err(charger.dev,
    "failed to enable charger: %d\n", ret);
    return IRQ_HANDLED;
    }
    } else {
    charger.online = 0;
    }
    if (charger.prev_online != charger.online)
    power_supply_changed(charger.psy);
    ret = tps65217_reg_read(charger.tps, TPS65217_REG_CHGCONFIG0, &val);
    if (ret < 0) {
    dev_err(charger.dev, "%s: Error in reading reg 0x%x\n",
    __func__, TPS65217_REG_CHGCONFIG0);
    return IRQ_HANDLED;
    }
    if (val & TPS65217_CHGCONFIG0_ACTIVE)
    dev_dbg(charger.dev, "%s: charger is charging\n", __func__);
    else
    dev_dbg(charger.dev,
    "%s: charger is NOT charging\n", __func__);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_charger_poll_task(data: *mut c_void) -> c_int {
    static int tps65217_charger_poll_task(void *data)
    {
    set_freezable();
    while (!kthread_should_stop()) {
    schedule_timeout_interruptible(POLL_INTERVAL);
    try_to_freeze();
    tps65217_charger_irq(-1, data);
    }
    return 0;
    }
    static const struct power_supply_desc tps65217_charger_desc = {
    .name			= "tps65217-charger",
    .type			= POWER_SUPPLY_TYPE_MAINS,
    .get_property		= tps65217_charger_get_property,
    .properties		= tps65217_charger_props,
    .num_properties		= ARRAY_SIZE(tps65217_charger_props),
    };
#[no_mangle]
unsafe extern "C" fn tps65217_charger_probe(pdev: *mut platform_device) -> c_int {
    static int tps65217_charger_probe(struct platform_device *pdev)
    {
    struct tps65217 *tps = dev_get_drvdata(pdev.dev.parent);
    struct tps65217_charger *charger;
    let mut cfg: power_supply_config = {};
    struct task_struct *poll_task;
    int irq[NUM_CHARGER_IRQS];
    int ret;
    int i;
    charger = devm_kzalloc(&pdev.dev, sizeof(*charger), GFP_KERNEL);
    if (!charger)
    return -ENOMEM;
    platform_set_drvdata(pdev, charger);
    charger.tps = tps;
    charger.dev = &pdev.dev;
    cfg.fwnode = dev_fwnode(&pdev.dev);
    cfg.drv_data = charger;
    charger.psy = devm_power_supply_register(&pdev.dev,
    &tps65217_charger_desc,
    &cfg);
    if (IS_ERR(charger.psy)) {
    dev_err(&pdev.dev, "failed: power supply register\n");
    return PTR_ERR(charger.psy);
    }
    irq[0] = platform_get_irq_byname(pdev, "USB");
    irq[1] = platform_get_irq_byname(pdev, "AC");
    ret = tps65217_config_charger(charger);
    if (ret < 0) {
    dev_err(charger.dev, "charger config failed, err %d\n", ret);
    return ret;
    }
// Create a polling thread if an interrupt is invalid
    if (irq[0] < 0 || irq[1] < 0) {
    poll_task = kthread_run(tps65217_charger_poll_task,
    charger, "ktps65217charger");
    if (IS_ERR(poll_task)) {
    ret = PTR_ERR(poll_task);
    dev_err(charger.dev,
    "Unable to run kthread err %d\n", ret);
    return ret;
    }
    charger.poll_task = poll_task;
    return 0;
    }
// Create IRQ threads for charger interrupts
    for (i = 0; i < NUM_CHARGER_IRQS; i++) {
    ret = devm_request_threaded_irq(&pdev.dev, irq[i], core::ptr::null_mut(),
    tps65217_charger_irq,
    IRQF_SHARED, "tps65217-charger",
    charger);
    if (ret)
    return ret;
// Check current state
    tps65217_charger_irq(-1, charger);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_charger_remove(pdev: *mut platform_device) {
    static void tps65217_charger_remove(struct platform_device *pdev)
    {
    struct tps65217_charger *charger = platform_get_drvdata(pdev);
    if (charger.poll_task)
    kthread_stop(charger.poll_task);
    }
    static const struct of_device_id tps65217_charger_match_table[] = {
    { .compatible = "ti,tps65217-charger", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tps65217_charger_match_table);
    static struct platform_driver tps65217_charger_driver = {
    .probe	= tps65217_charger_probe,
    .remove	= tps65217_charger_remove,
    .driver	= {
    .name	= "tps65217-charger",
    .of_match_table = of_match_ptr(tps65217_charger_match_table),
    },
    };
    module_platform_driver(tps65217_charger_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Enric Balletbo Serra <enric.balletbo@collabora.com>");
    MODULE_DESCRIPTION("TPS65217 battery charger driver");
