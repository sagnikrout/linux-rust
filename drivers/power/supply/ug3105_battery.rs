//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/ug3105_battery.c
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
// Battery monitor driver for the uPI uG3105 battery monitor
//
// Note the uG3105 is not a full-featured autonomous fuel-gauge. Instead it is
// expected to be use in combination with some always on microcontroller reading
// its coulomb-counter before it can wrap (must be read every 400 seconds!).
//
// Since Linux does not monitor coulomb-counter changes while the device
// is off or suspended, the coulomb counter is not used atm.
//
// Possible improvements:
// 1. Add coulumb counter reading, e.g. something like this:
// Read + reset coulomb counter every 10 polls (every 300 seconds)
//
// if ((chip->poll_count % 10) == 0) {
// val = ug3105_read_word(chip->client, UG3105_REG_COULOMB_CNT);
// if (val < 0)
// goto out;
//
// i2c_smbus_write_byte_data(chip->client, UG3105_REG_CTRL1,
// UG3105_CTRL1_RESET_COULOMB_CNT);
//
// chip->total_coulomb_count += (s16)val;
// dev_dbg(&chip->client->dev, "coulomb count %d total %d\n",
// (s16)val, chip->total_coulomb_count);
// }
//
// 2. Reset total_coulomb_count val to 0 when the battery is as good as empty
// and remember that we did this (and clear the flag for this on susp/resume)
// 3. When the battery is full check if the flag that we set total_coulomb_count
// to when the battery was empty is set. If so we now know the capacity,
// not the design, but actual capacity, of the battery
// 4. Add some mechanism (needs userspace help, or maybe use efivar?) to remember
// the actual capacity of the battery over reboots
// 5. When we know the actual capacity at probe time, add energy_now and
// energy_full attributes. Guess boot + resume energy_now value based on ocv
// and then use total_coulomb_count to report energy_now over time, resetting
// things to adjust for drift when empty/full. This should give more accurate
// readings, esp. in the 30-70% range and allow userspace to estimate time
// remaining till empty/full
// 6. Maybe unregister + reregister the psy device when we learn the actual
// capacity during run-time ?
//
// The above will also require some sort of mwh_per_unit calculation. Testing
// has shown that an estimated 7404mWh increase of the battery's energy results
// in a total_coulomb_count increase of 3277 units with a 5 milli-ohm sense R.
//
// Copyright (C) 2021 - 2025 Hans de Goede <hansg@kernel.org>
//

pub const UG3105_REG_MODE: c_uint = 0x00;
pub const UG3105_REG_CTRL1: c_uint = 0x01;
pub const UG3105_REG_COULOMB_CNT: c_uint = 0x02;
pub const UG3105_REG_BAT_VOLT: c_uint = 0x08;
pub const UG3105_REG_BAT_CURR: c_uint = 0x0c;
pub const UG3105_MODE_STANDBY: c_uint = 0x00;
pub const UG3105_MODE_RUN: c_uint = 0x10;
pub const UG3105_CTRL1_RESET_COULOMB_CNT: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ug3105_chip {
// Must be the first member see adc-battery-helper documentation
    pub helper: adc_battery_helper,
    pub client: *mut i2c_client,
    pub psy: *mut power_supply,
    pub uv_per_unit: c_int,
    pub ua_per_unit: c_int,
}

#[no_mangle]
unsafe extern "C" fn ug3105_read_word(client: *mut i2c_client, reg: u8) -> c_int {
    static int ug3105_read_word(struct i2c_client *client, u8 reg)
    {
    int val;
    val = i2c_smbus_read_word_data(client, reg);
    if (val < 0)
    dev_err(&client.dev, "Error reading reg 0x%02x\n", reg);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ug3105_get_voltage_and_current_now(psy: *mut power_supply, volt: *mut c_int, curr: *mut c_int) -> c_int {
    static int ug3105_get_voltage_and_current_now(struct power_supply *psy, int *volt, int *curr)
    {
    struct ug3105_chip *chip = power_supply_get_drvdata(psy);
    int ret;
    ret = ug3105_read_word(chip.client, UG3105_REG_BAT_VOLT);
    if (ret < 0)
    return ret;
// volt = ret * chip->uv_per_unit;
    ret = ug3105_read_word(chip.client, UG3105_REG_BAT_CURR);
    if (ret < 0)
    return ret;
// curr = (s16)ret * chip->ua_per_unit;
    return 0;
    }
    static const struct power_supply_desc ug3105_psy_desc = {
    .name		= "ug3105_battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .get_property	= adc_battery_helper_get_property,
    .external_power_changed	= adc_battery_helper_external_power_changed,
    .properties	= adc_battery_helper_properties,
    .num_properties	= ADC_HELPER_NUM_PROPERTIES,
    };
#[no_mangle]
unsafe extern "C" fn ug3105_start(client: *mut i2c_client) {
    static void ug3105_start(struct i2c_client *client)
    {
    i2c_smbus_write_byte_data(client, UG3105_REG_MODE, UG3105_MODE_RUN);
    i2c_smbus_write_byte_data(client, UG3105_REG_CTRL1, UG3105_CTRL1_RESET_COULOMB_CNT);
    }
#[no_mangle]
unsafe extern "C" fn ug3105_stop(client: *mut i2c_client) {
    static void ug3105_stop(struct i2c_client *client)
    {
    i2c_smbus_write_byte_data(client, UG3105_REG_MODE, UG3105_MODE_STANDBY);
    }
#[no_mangle]
unsafe extern "C" fn ug3105_probe(client: *mut i2c_client) -> c_int {
    static int ug3105_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {};
    struct device *dev = &client.dev;
    let mut curr_sense_res_uohm: u32 = 10000;
    struct ug3105_chip *chip;
    int ret;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.client = client;
    ug3105_start(client);
    device_property_read_u32(dev, "upisemi,rsns-microohm", &curr_sense_res_uohm);
//
// DAC maximum is 4.5V divided by 65536 steps + an unknown factor of 10
// coming from somewhere for some reason (verified with a volt-meter).
//
    chip.uv_per_unit = 45000000 / 65536;
// Datasheet says 8.1 uV per unit for the current ADC
    chip.ua_per_unit = 8100000 / curr_sense_res_uohm;
    psy_cfg.drv_data = chip;
    chip.psy = devm_power_supply_register(dev, &ug3105_psy_desc, &psy_cfg);
    if (IS_ERR(chip.psy)) {
    ret = PTR_ERR(chip.psy);
    goto stop;
    }
    ret = adc_battery_helper_init(&chip.helper, chip.psy,
    ug3105_get_voltage_and_current_now, core::ptr::null_mut());
    if (ret)
    goto stop;
    i2c_set_clientdata(client, chip);
    return 0;
    stop:
    ug3105_stop(client);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ug3105_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ug3105_suspend(struct device *dev)
    {
    struct ug3105_chip *chip = dev_get_drvdata(dev);
    adc_battery_helper_suspend(dev);
    ug3105_stop(chip.client);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ug3105_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ug3105_resume(struct device *dev)
    {
    struct ug3105_chip *chip = dev_get_drvdata(dev);
    ug3105_start(chip.client);
    adc_battery_helper_resume(dev);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(ug3105_pm_ops, ug3105_suspend,
    ug3105_resume);
    static const struct i2c_device_id ug3105_id[] = {
    { .name = "ug3105" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ug3105_id);
    static struct i2c_driver ug3105_i2c_driver = {
    .driver	= {
    .name = "ug3105",
    .pm = &ug3105_pm_ops,
    },
    .probe = ug3105_probe,
    .remove = ug3105_stop,
    .shutdown = ug3105_stop,
    .id_table = ug3105_id,
    };
    module_i2c_driver(ug3105_i2c_driver);
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org");
    MODULE_DESCRIPTION("uPI uG3105 battery monitor driver");
    MODULE_LICENSE("GPL");
