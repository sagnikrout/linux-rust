//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/collie_battery.c
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
// Battery and Power Management code for the Sharp SL-5x00
//
// Copyright (C) 2009 Thomas Kunze
//
// based on tosa_battery.c
//

    static DEFINE_MUTEX(bat_lock); /* protects gpio pins */
    static struct work_struct bat_work;
    static struct ucb1x00 *ucb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct collie_bat {
    pub status: c_int,
    pub psy: *mut power_supply,
    pub full_chrg: c_int,
    pub /: *mut *mut mutex work_lock; / protects data,
    pub bat): *mut *mut bool (is_present)(struct collie_bat,
    pub gpio_full: *mut gpio_desc,
    pub gpio_charge_on: *mut gpio_desc,
    pub technology: c_int,
    pub gpio_bat: *mut gpio_desc,
    pub adc_bat: c_int,
    pub adc_bat_divider: c_int,
    pub bat_max: c_int,
    pub bat_min: c_int,
    pub gpio_temp: *mut gpio_desc,
    pub adc_temp: c_int,
    pub adc_temp_divider: c_int,
}

    static struct collie_bat collie_bat_main;
#[no_mangle]
unsafe extern "C" fn collie_read_bat(bat: *mut collie_bat) -> c_ulong {
    static unsigned long collie_read_bat(struct collie_bat *bat)
    {
    let mut value: c_ulong = 0;
    if (!bat.gpio_bat || bat.adc_bat < 0)
    return 0;
    mutex_lock(&bat_lock);
    gpiod_set_value(bat.gpio_bat, 1);
    msleep(5);
    ucb1x00_adc_enable(ucb);
    value = ucb1x00_adc_read(ucb, bat.adc_bat, UCB_SYNC);
    ucb1x00_adc_disable(ucb);
    gpiod_set_value(bat.gpio_bat, 0);
    mutex_unlock(&bat_lock);
    value = value * 1000000 / bat.adc_bat_divider;
    return value;
    }
#[no_mangle]
unsafe extern "C" fn collie_read_temp(bat: *mut collie_bat) -> c_ulong {
    static unsigned long collie_read_temp(struct collie_bat *bat)
    {
    let mut value: c_ulong = 0;
    if (!bat.gpio_temp || bat.adc_temp < 0)
    return 0;
    mutex_lock(&bat_lock);
    gpiod_set_value(bat.gpio_temp, 1);
    msleep(5);
    ucb1x00_adc_enable(ucb);
    value = ucb1x00_adc_read(ucb, bat.adc_temp, UCB_SYNC);
    ucb1x00_adc_disable(ucb);
    gpiod_set_value(bat.gpio_temp, 0);
    mutex_unlock(&bat_lock);
    value = value * 10000 / bat.adc_temp_divider;
    return value;
    }
    static int collie_bat_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    let mut ret: c_int = 0;
    struct collie_bat *bat = power_supply_get_drvdata(psy);
    if (bat.is_present && !bat.is_present(bat)
    && psp != POWER_SUPPLY_PROP_PRESENT) {
    return -ENODEV;
    }
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = bat.status;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = bat.technology;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = collie_read_bat(bat);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    if (bat.full_chrg == -1)
    val.intval = bat.bat_max;
    else
    val.intval = bat.full_chrg;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    val.intval = bat.bat_max;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN:
    val.intval = bat.bat_min;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval = collie_read_temp(bat);
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = bat.is_present ? bat.is_present(bat) : 1;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_external_power_changed(psy: *mut power_supply) {
    static void collie_bat_external_power_changed(struct power_supply *psy)
    {
    schedule_work(&bat_work);
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_gpio_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t collie_bat_gpio_isr(int irq, void *data)
    {
    pr_info("collie_bat_gpio irq\n");
    schedule_work(&bat_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_update(bat: *mut collie_bat) {
    static void collie_bat_update(struct collie_bat *bat)
    {
    int old;
    struct power_supply *psy = bat.psy;
    mutex_lock(&bat.work_lock);
    old = bat.status;
    if (bat.is_present && !bat.is_present(bat)) {
    printk(KERN_NOTICE "%s not present\n", psy.desc.name);
    bat.status = POWER_SUPPLY_STATUS_UNKNOWN;
    bat.full_chrg = -1;
    } else if (power_supply_am_i_supplied(psy)) {
    if (bat.status == POWER_SUPPLY_STATUS_DISCHARGING) {
    gpiod_set_value(bat.gpio_charge_on, 1);
    mdelay(15);
    }
    if (gpiod_get_value(bat.gpio_full)) {
    if (old == POWER_SUPPLY_STATUS_CHARGING ||
    bat.full_chrg == -1)
    bat.full_chrg = collie_read_bat(bat);
    gpiod_set_value(bat.gpio_charge_on, 0);
    bat.status = POWER_SUPPLY_STATUS_FULL;
    } else {
    gpiod_set_value(bat.gpio_charge_on, 1);
    bat.status = POWER_SUPPLY_STATUS_CHARGING;
    }
    } else {
    gpiod_set_value(bat.gpio_charge_on, 0);
    bat.status = POWER_SUPPLY_STATUS_DISCHARGING;
    }
    if (old != bat.status)
    power_supply_changed(psy);
    mutex_unlock(&bat.work_lock);
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_work(work: *mut work_struct) {
    static void collie_bat_work(struct work_struct *work)
    {
    collie_bat_update(&collie_bat_main);
    }
    static enum power_supply_property collie_bat_main_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_TEMP,
    };
    static enum power_supply_property collie_bat_bu_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_PRESENT,
    };
    static const struct power_supply_desc collie_bat_main_desc = {
    .name		= "main-battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .properties	= collie_bat_main_props,
    .num_properties	= ARRAY_SIZE(collie_bat_main_props),
    .get_property	= collie_bat_get_property,
    .external_power_changed = collie_bat_external_power_changed,
    .use_for_apm	= 1,
    };
    static struct collie_bat collie_bat_main = {
    .status = POWER_SUPPLY_STATUS_DISCHARGING,
    .full_chrg = -1,
    .psy = core::ptr::null_mut(),
    .gpio_full = core::ptr::null_mut(),
    .gpio_charge_on = core::ptr::null_mut(),
    .technology = POWER_SUPPLY_TECHNOLOGY_LIPO,
    .gpio_bat = core::ptr::null_mut(),
    .adc_bat = UCB_ADC_INP_AD1,
    .adc_bat_divider = 155,
    .bat_max = 4310000,
    .bat_min = 1551 * 1000000 / 414,
    .gpio_temp = core::ptr::null_mut(),
    .adc_temp = UCB_ADC_INP_AD0,
    .adc_temp_divider = 10000,
    };
    static const struct power_supply_desc collie_bat_bu_desc = {
    .name		= "backup-battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .properties	= collie_bat_bu_props,
    .num_properties	= ARRAY_SIZE(collie_bat_bu_props),
    .get_property	= collie_bat_get_property,
    .external_power_changed = collie_bat_external_power_changed,
    };
    static struct collie_bat collie_bat_bu = {
    .status = POWER_SUPPLY_STATUS_UNKNOWN,
    .full_chrg = -1,
    .psy = core::ptr::null_mut(),
    .gpio_full = core::ptr::null_mut(),
    .gpio_charge_on = core::ptr::null_mut(),
    .technology = POWER_SUPPLY_TECHNOLOGY_LiMn,
    .gpio_bat = core::ptr::null_mut(),
    .adc_bat = UCB_ADC_INP_AD1,
    .adc_bat_divider = 155,
    .bat_max = 3000000,
    .bat_min = 1900000,
    .gpio_temp = core::ptr::null_mut(),
    .adc_temp = -1,
    .adc_temp_divider = -1,
    };
// Obtained but unused GPIO
    static struct gpio_desc *collie_mbat_low;

    static int wakeup_enabled;
#[no_mangle]
unsafe extern "C" fn collie_bat_suspend(dev: *mut ucb1x00_dev) -> c_int {
    static int collie_bat_suspend(struct ucb1x00_dev *dev)
    {
// flush all pending status updates
    flush_work(&bat_work);
    if (device_may_wakeup(&dev.ucb.dev) &&
    collie_bat_main.status == POWER_SUPPLY_STATUS_CHARGING)
    wakeup_enabled = !enable_irq_wake(gpiod_to_irq(collie_bat_main.gpio_full));
    else
    wakeup_enabled = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_resume(dev: *mut ucb1x00_dev) -> c_int {
    static int collie_bat_resume(struct ucb1x00_dev *dev)
    {
    if (wakeup_enabled)
    disable_irq_wake(gpiod_to_irq(collie_bat_main.gpio_full));
// things may have changed while we were away
    schedule_work(&bat_work);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn collie_bat_probe(dev: *mut ucb1x00_dev) -> c_int {
    static int collie_bat_probe(struct ucb1x00_dev *dev)
    {
    int ret;
    let mut psy_main_cfg: power_supply_config = {}, psy_bu_cfg = {};
    struct gpio_chip *gc = &dev.ucb.gpio;
    if (!machine_is_collie())
    return -ENODEV;
    ucb = dev.ucb;
// Obtain all the main battery GPIOs
    collie_bat_main.gpio_full = gpiod_get(&dev.ucb.dev,
    "main battery full",
    GPIOD_IN);
    if (IS_ERR(collie_bat_main.gpio_full))
    return PTR_ERR(collie_bat_main.gpio_full);
    collie_mbat_low = gpiod_get(&dev.ucb.dev,
    "main battery low",
    GPIOD_IN);
    if (IS_ERR(collie_mbat_low)) {
    ret = PTR_ERR(collie_mbat_low);
    goto err_put_gpio_full;
    }
    collie_bat_main.gpio_charge_on = gpiod_get(&dev.ucb.dev,
    "main charge on",
    GPIOD_OUT_LOW);
    if (IS_ERR(collie_bat_main.gpio_charge_on)) {
    ret = PTR_ERR(collie_bat_main.gpio_charge_on);
    goto err_put_mbat_low;
    }
// COLLIE_GPIO_MBAT_ON = GPIO 7 on the UCB (TC35143)
    collie_bat_main.gpio_bat = gpiochip_request_own_desc(gc,
    7,
    "main battery",
    GPIO_ACTIVE_HIGH,
    GPIOD_OUT_LOW);
    if (IS_ERR(collie_bat_main.gpio_bat)) {
    ret = PTR_ERR(collie_bat_main.gpio_bat);
    goto err_put_gpio_charge_on;
    }
// COLLIE_GPIO_TMP_ON = GPIO 9 on the UCB (TC35143)
    collie_bat_main.gpio_temp = gpiochip_request_own_desc(gc,
    9,
    "main battery temp",
    GPIO_ACTIVE_HIGH,
    GPIOD_OUT_LOW);
    if (IS_ERR(collie_bat_main.gpio_temp)) {
    ret = PTR_ERR(collie_bat_main.gpio_temp);
    goto err_free_gpio_bat;
    }
//
// Obtain the backup battery COLLIE_GPIO_BBAT_ON which is
// GPIO 8 on the UCB (TC35143)
//
    collie_bat_bu.gpio_bat = gpiochip_request_own_desc(gc,
    8,
    "backup battery",
    GPIO_ACTIVE_HIGH,
    GPIOD_OUT_LOW);
    if (IS_ERR(collie_bat_bu.gpio_bat)) {
    ret = PTR_ERR(collie_bat_bu.gpio_bat);
    goto err_free_gpio_temp;
    }
    mutex_init(&collie_bat_main.work_lock);
    INIT_WORK(&bat_work, collie_bat_work);
    psy_main_cfg.drv_data = &collie_bat_main;
    collie_bat_main.psy = power_supply_register(&dev.ucb.dev,
    &collie_bat_main_desc,
    &psy_main_cfg);
    if (IS_ERR(collie_bat_main.psy)) {
    ret = PTR_ERR(collie_bat_main.psy);
    goto err_psy_reg_main;
    }
    psy_bu_cfg.drv_data = &collie_bat_bu;
    collie_bat_bu.psy = power_supply_register(&dev.ucb.dev,
    &collie_bat_bu_desc,
    &psy_bu_cfg);
    if (IS_ERR(collie_bat_bu.psy)) {
    ret = PTR_ERR(collie_bat_bu.psy);
    goto err_psy_reg_bu;
    }
    ret = request_irq(gpiod_to_irq(collie_bat_main.gpio_full),
    collie_bat_gpio_isr,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
    "main full", &collie_bat_main);
    if (ret)
    goto err_irq;
    device_init_wakeup(&ucb.dev, 1);
    schedule_work(&bat_work);
    return 0;
    err_irq:
    power_supply_unregister(collie_bat_bu.psy);
    err_psy_reg_bu:
    power_supply_unregister(collie_bat_main.psy);
    err_psy_reg_main:
// see comment in collie_bat_remove
    cancel_work_sync(&bat_work);
    gpiochip_free_own_desc(collie_bat_bu.gpio_bat);
    err_free_gpio_temp:
    gpiochip_free_own_desc(collie_bat_main.gpio_temp);
    err_free_gpio_bat:
    gpiochip_free_own_desc(collie_bat_main.gpio_bat);
    err_put_gpio_charge_on:
    gpiod_put(collie_bat_main.gpio_charge_on);
    err_put_mbat_low:
    gpiod_put(collie_mbat_low);
    err_put_gpio_full:
    gpiod_put(collie_bat_main.gpio_full);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_remove(dev: *mut ucb1x00_dev) {
    static void collie_bat_remove(struct ucb1x00_dev *dev)
    {
    device_init_wakeup(&ucb.dev, 0);
    free_irq(gpiod_to_irq(collie_bat_main.gpio_full), &collie_bat_main);
    power_supply_unregister(collie_bat_bu.psy);
    power_supply_unregister(collie_bat_main.psy);
// These are obtained from the machine
    gpiod_put(collie_bat_main.gpio_full);
    gpiod_put(collie_mbat_low);
    gpiod_put(collie_bat_main.gpio_charge_on);
// These are directly from the UCB so let's free them
    gpiochip_free_own_desc(collie_bat_main.gpio_bat);
    gpiochip_free_own_desc(collie_bat_main.gpio_temp);
    gpiochip_free_own_desc(collie_bat_bu.gpio_bat);
//
// Now cancel the bat_work.  We won't get any more schedules,
// since all sources (isr and external_power_changed) are
// unregistered now.
//
    cancel_work_sync(&bat_work);
    }
    static struct ucb1x00_driver collie_bat_driver = {
    .add		= collie_bat_probe,
    .remove		= collie_bat_remove,
    .suspend	= collie_bat_suspend,
    .resume		= collie_bat_resume,
    };
#[no_mangle]
unsafe extern "C" fn collie_bat_init() -> int __init {
    static int __init collie_bat_init(void)
    {
    return ucb1x00_register_driver(&collie_bat_driver);
    }
#[no_mangle]
unsafe extern "C" fn collie_bat_exit() -> void __exit {
    static void __exit collie_bat_exit(void)
    {
    ucb1x00_unregister_driver(&collie_bat_driver);
    }
    module_init(collie_bat_init);
    module_exit(collie_bat_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Thomas Kunze");
    MODULE_DESCRIPTION("Collie battery driver");
