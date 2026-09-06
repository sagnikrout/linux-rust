//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/generic-adc-battery.c
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
// Generic battery driver using IIO
// Copyright (C) 2012, Anish Kumar <yesanishhere@gmail.com>
// Copyright (c) 2023, Sebastian Reichel <sre@kernel.org>
//

    enum gab_chan_type {
    GAB_VOLTAGE = 0,
    GAB_CURRENT,
    GAB_POWER,
    GAB_TEMP,
    GAB_MAX_CHAN_TYPE
    };
//
// gab_chan_name suggests the standard channel names for commonly used
// channel types.
//
    static const char *const gab_chan_name[] = {
    [GAB_VOLTAGE]	= "voltage",
    [GAB_CURRENT]	= "current",
    [GAB_POWER]	= "power",
    [GAB_TEMP]	= "temperature",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gab {
    pub psy: *mut power_supply,
    pub psy_desc: power_supply_desc,
    pub channel: [*mut iio_channel; GAB_MAX_CHAN_TYPE],
    pub bat_work: delayed_work,
    pub status: c_int,
    pub charge_finished: *mut gpio_desc,
}

    static struct gab *to_generic_bat(struct power_supply *psy)
    {
    return power_supply_get_drvdata(psy);
    }
#[no_mangle]
unsafe extern "C" fn gab_ext_power_changed(psy: *mut power_supply) {
    static void gab_ext_power_changed(struct power_supply *psy)
    {
    struct gab *adc_bat = to_generic_bat(psy);
    schedule_delayed_work(&adc_bat.bat_work, msecs_to_jiffies(0));
    }
    static const enum power_supply_property gab_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    };
//
// This properties are set based on the received platform data and this
// should correspond one-to-one with enum chan_type.
//
    static const enum power_supply_property gab_dyn_props[] = {
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn gab_charge_finished(adc_bat: *mut gab) -> bool {
    static bool gab_charge_finished(struct gab *adc_bat)
    {
    if (!adc_bat.charge_finished)
    return false;
    return gpiod_get_value(adc_bat.charge_finished);
    }
    static int gab_read_channel(struct gab *adc_bat, enum gab_chan_type channel,
    int *result)
    {
    int ret;
    ret = iio_read_channel_processed(adc_bat.channel[channel], result);
    if (ret < 0)
    dev_err(&adc_bat.psy.dev, "read channel error: %d\n", ret);
    else
// result *= 1000;
    return ret;
    }
    static int gab_get_property(struct power_supply *psy,
    enum power_supply_property psp, union power_supply_propval *val)
    {
    struct gab *adc_bat = to_generic_bat(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = adc_bat.status;
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    return gab_read_channel(adc_bat, GAB_VOLTAGE, &val.intval);
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    return gab_read_channel(adc_bat, GAB_CURRENT, &val.intval);
    case POWER_SUPPLY_PROP_POWER_NOW:
    return gab_read_channel(adc_bat, GAB_POWER, &val.intval);
    case POWER_SUPPLY_PROP_TEMP:
    return gab_read_channel(adc_bat, GAB_TEMP, &val.intval);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn gab_work(work: *mut work_struct) {
    static void gab_work(struct work_struct *work)
    {
    struct gab *adc_bat;
    struct delayed_work *delayed_work;
    int status;
    delayed_work = to_delayed_work(work);
    adc_bat = container_of(delayed_work, struct gab, bat_work);
    status = adc_bat.status;
    if (!power_supply_am_i_supplied(adc_bat.psy))
    adc_bat.status =  POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gab_charge_finished(adc_bat)) -> else {
    else if (gab_charge_finished(adc_bat))
    adc_bat.status = POWER_SUPPLY_STATUS_NOT_CHARGING;
    else
    adc_bat.status = POWER_SUPPLY_STATUS_CHARGING;
    if (status != adc_bat.status)
    power_supply_changed(adc_bat.psy);
    }
#[no_mangle]
unsafe extern "C" fn gab_charged(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t gab_charged(int irq, void *dev_id)
    {
    struct gab *adc_bat = dev_id;
    schedule_delayed_work(&adc_bat.bat_work,
    msecs_to_jiffies(JITTER_DEFAULT));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gab_probe(pdev: *mut platform_device) -> c_int {
    static int gab_probe(struct platform_device *pdev)
    {
    struct gab *adc_bat;
    struct power_supply_desc *psy_desc;
    let mut psy_cfg: power_supply_config = {};
    enum power_supply_property *properties;
    let mut ret: c_int = 0;
    int chan;
    let mut index: c_int = ARRAY_SIZE(gab_props);
    let mut any: bool = false;
    adc_bat = devm_kzalloc(&pdev.dev, sizeof(*adc_bat), GFP_KERNEL);
    if (!adc_bat)
    return -ENOMEM;
    psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    psy_cfg.drv_data = adc_bat;
    psy_desc = &adc_bat.psy_desc;
    psy_desc.name = dev_name(&pdev.dev);
// bootup default values for the battery
    adc_bat.status = POWER_SUPPLY_STATUS_DISCHARGING;
    psy_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    psy_desc.get_property = gab_get_property;
    psy_desc.external_power_changed = gab_ext_power_changed;
//
// copying the static properties and allocating extra memory for holding
// the extra configurable properties received from platform data.
//
    properties = devm_kcalloc(&pdev.dev,
    ARRAY_SIZE(gab_props) +
    ARRAY_SIZE(gab_chan_name),
    sizeof(*properties),
    GFP_KERNEL);
    if (!properties)
    return -ENOMEM;
    memcpy(properties, gab_props, sizeof(gab_props));
//
// getting channel from iio and copying the battery properties
// based on the channel supported by consumer device.
//
    for (chan = 0; chan < ARRAY_SIZE(gab_chan_name); chan++) {
    adc_bat.channel[chan] = devm_iio_channel_get(&pdev.dev, gab_chan_name[chan]);
    if (IS_ERR(adc_bat.channel[chan])) {
    ret = PTR_ERR(adc_bat.channel[chan]);
    if (ret != -ENODEV)
    return dev_err_probe(&pdev.dev, ret, "Failed to get ADC channel %s\n", gab_chan_name[chan]);
    adc_bat.channel[chan] = core::ptr::null_mut();
    } else if (adc_bat.channel[chan]) {
// copying properties for supported channels only
    int index2;
    for (index2 = 0; index2 < index; index2++) {
    if (properties[index2] == gab_dyn_props[chan])
    break;	/* already known */
    }
    if (index2 == index)	/* really new */
    properties[index++] = gab_dyn_props[chan];
    any = true;
    }
    }
// none of the channels are supported so let's bail out
    if (!any)
    return dev_err_probe(&pdev.dev, -ENODEV, "Failed to get any ADC channel\n");
//
// Total number of properties is equal to static properties
// plus the dynamic properties.Some properties may not be set
// as come channels may be not be supported by the device.So
// we need to take care of that.
//
    psy_desc.properties = properties;
    psy_desc.num_properties = index;
    adc_bat.psy = devm_power_supply_register(&pdev.dev, psy_desc, &psy_cfg);
    if (IS_ERR(adc_bat.psy))
    return dev_err_probe(&pdev.dev, PTR_ERR(adc_bat.psy), "Failed to register power-supply device\n");
    ret = devm_delayed_work_autocancel(&pdev.dev, &adc_bat.bat_work, gab_work);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register delayed work\n");
    adc_bat.charge_finished = devm_gpiod_get_optional(&pdev.dev, "charged", GPIOD_IN);
    if (adc_bat.charge_finished) {
    int irq;
    irq = gpiod_to_irq(adc_bat.charge_finished);
    ret = devm_request_any_context_irq(&pdev.dev, irq, gab_charged,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
    "battery charged", adc_bat);
    if (ret < 0)
    return ret;
    }
    platform_set_drvdata(pdev, adc_bat);
// Schedule timer to check current status
    schedule_delayed_work(&adc_bat.bat_work,
    msecs_to_jiffies(0));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gab_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused gab_suspend(struct device *dev)
    {
    struct gab *adc_bat = dev_get_drvdata(dev);
    cancel_delayed_work_sync(&adc_bat.bat_work);
    adc_bat.status = POWER_SUPPLY_STATUS_UNKNOWN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gab_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused gab_resume(struct device *dev)
    {
    struct gab *adc_bat = dev_get_drvdata(dev);
// Schedule timer to check current status
    schedule_delayed_work(&adc_bat.bat_work,
    msecs_to_jiffies(JITTER_DEFAULT));
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(gab_pm_ops, gab_suspend, gab_resume);
    static const struct of_device_id gab_match[] = {
    { .compatible = "adc-battery" },
    { }
    };
    MODULE_DEVICE_TABLE(of, gab_match);
    static struct platform_driver gab_driver = {
    .driver		= {
    .name	= "generic-adc-battery",
    .pm	= &gab_pm_ops,
    .of_match_table = gab_match,
    },
    .probe		= gab_probe,
    };
    module_platform_driver(gab_driver);
    MODULE_AUTHOR("anish kumar <yesanishhere@gmail.com>");
    MODULE_DESCRIPTION("generic battery driver using IIO");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
