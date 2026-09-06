//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/powr1220.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// powr1220.c - Driver for the Lattice POWR1220 programmable power supply
// and monitor. Users can read all ADC inputs along with their labels
// using the sysfs nodes.
//
// Copyright (c) 2014 Echo360 https://www.echo360.com
// Scott Kanowitz <skanowitz@echo360.com> <scott.kanowitz@gmail.com>
//

pub const ADC_STEP_MV: c_int = 2;
pub const ADC_MAX_LOW_MEASUREMENT_MV: c_int = 2000;
    enum powr1xxx_chips { powr1014, powr1220 };
    enum powr1220_regs {
    VMON_STATUS0,
    VMON_STATUS1,
    VMON_STATUS2,
    OUTPUT_STATUS0,
    OUTPUT_STATUS1,
    OUTPUT_STATUS2,
    INPUT_STATUS,
    ADC_VALUE_LOW,
    ADC_VALUE_HIGH,
    ADC_MUX,
    UES_BYTE0,
    UES_BYTE1,
    UES_BYTE2,
    UES_BYTE3,
    GP_OUTPUT1,
    GP_OUTPUT2,
    GP_OUTPUT3,
    INPUT_VALUE,
    RESET,
    TRIM1_TRIM,
    TRIM2_TRIM,
    TRIM3_TRIM,
    TRIM4_TRIM,
    TRIM5_TRIM,
    TRIM6_TRIM,
    TRIM7_TRIM,
    TRIM8_TRIM,
    MAX_POWR1220_REGS
    };
    enum powr1220_adc_values {
    VMON1,
    VMON2,
    VMON3,
    VMON4,
    VMON5,
    VMON6,
    VMON7,
    VMON8,
    VMON9,
    VMON10,
    VMON11,
    VMON12,
    VCCA,
    VCCINP,
    MAX_POWR1220_ADC_VALUES
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powr1220_data {
    pub client: *mut i2c_client,
    pub max_channels: u8,
    pub adc_valid: [bool; MAX_POWR1220_ADC_VALUES],
// the next value is in jiffies
    pub adc_last_updated: [c_ulong; MAX_POWR1220_ADC_VALUES],
// values
    pub adc_maxes: [c_int; MAX_POWR1220_ADC_VALUES],
    pub adc_values: [c_int; MAX_POWR1220_ADC_VALUES],
}

    static const char * const input_names[] = {
    [VMON1]    = "vmon1",
    [VMON2]    = "vmon2",
    [VMON3]    = "vmon3",
    [VMON4]    = "vmon4",
    [VMON5]    = "vmon5",
    [VMON6]    = "vmon6",
    [VMON7]    = "vmon7",
    [VMON8]    = "vmon8",
    [VMON9]    = "vmon9",
    [VMON10]   = "vmon10",
    [VMON11]   = "vmon11",
    [VMON12]   = "vmon12",
    [VCCA]     = "vcca",
    [VCCINP]   = "vccinp",
    };
// Reads the specified ADC channel
#[no_mangle]
unsafe extern "C" fn powr1220_read_adc(dev: *mut device, ch_num: c_int) -> c_int {
    static int powr1220_read_adc(struct device *dev, int ch_num)
    {
    struct powr1220_data *data = dev_get_drvdata(dev);
    int reading;
    int result;
    let mut adc_range: c_int = 0;
    if (time_after(jiffies, data.adc_last_updated[ch_num] + HZ) ||
    !data.adc_valid[ch_num]) {
//
// figure out if we need to use the attenuator for
// high inputs or inputs that we don't yet have a measurement
// for. We dynamically set the attenuator depending on the
// max reading.
//
    if (data.adc_maxes[ch_num] > ADC_MAX_LOW_MEASUREMENT_MV ||
    data.adc_maxes[ch_num] == 0)
    adc_range = 1 << 4;
// set the attenuator and mux
    result = i2c_smbus_write_byte_data(data.client, ADC_MUX,
    adc_range | ch_num);
    if (result < 0)
    return result;
//
// wait at least Tconvert time (200 us) for the
// conversion to complete
//
    udelay(200);
// get the ADC reading
    result = i2c_smbus_read_byte_data(data.client, ADC_VALUE_LOW);
    if (result < 0)
    return result;
    reading = result >> 4;
// get the upper half of the reading
    result = i2c_smbus_read_byte_data(data.client, ADC_VALUE_HIGH);
    if (result < 0)
    return result;
    reading |= result << 4;
// now convert the reading to a voltage
    reading *= ADC_STEP_MV;
    data.adc_values[ch_num] = reading;
    data.adc_valid[ch_num] = true;
    data.adc_last_updated[ch_num] = jiffies;
    result = reading;
    if (reading > data.adc_maxes[ch_num])
    data.adc_maxes[ch_num] = reading;
    } else {
    result = data.adc_values[ch_num];
    }
    return result;
    }
    static umode_t
    powr1220_is_visible(const void *data, enum hwmon_sensor_types type, u32
    attr, int channel)
    {
    struct powr1220_data *chip_data = (struct powr1220_data *)data;
    if (channel >= chip_data.max_channels)
    return 0;
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_highest:
    case hwmon_in_label:
    return 0444;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static int
    powr1220_read_string(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_label:
// str = input_names[channel];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
    static int
    powr1220_read(struct device *dev, enum hwmon_sensor_types type, u32
    attr, int channel, long *val)
    {
    struct powr1220_data *data = dev_get_drvdata(dev);
    int ret;
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    ret = powr1220_read_adc(dev, channel);
    if (ret < 0)
    return ret;
// val = ret;
    break;
    case hwmon_in_highest:
// val = data->adc_maxes[channel];
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const powr1220_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_HIGHEST | HWMON_I_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops powr1220_hwmon_ops = {
    .read = powr1220_read,
    .read_string = powr1220_read_string,
    .is_visible = powr1220_is_visible,
    };
    static const struct hwmon_chip_info powr1220_chip_info = {
    .ops = &powr1220_hwmon_ops,
    .info = powr1220_info,
    };
#[no_mangle]
unsafe extern "C" fn powr1220_probe(client: *mut i2c_client) -> c_int {
    static int powr1220_probe(struct i2c_client *client)
    {
    struct powr1220_data *data;
    struct device *hwmon_dev;
    enum powr1xxx_chips chip;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    chip = (uintptr_t)i2c_get_match_data(client);
    switch (chip) {
    case powr1014:
    data.max_channels = 10;
    break;
    default:
    data.max_channels = 12;
    break;
    }
    data.client = client;
    hwmon_dev = devm_hwmon_device_register_with_info(&client.dev,
    client.name,
    data,
    &powr1220_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id powr1220_ids[] = {
    { .name = "powr1014", .driver_data = powr1014 },
    { .name = "powr1220", .driver_data = powr1220 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, powr1220_ids);
    static struct i2c_driver powr1220_driver = {
    .driver = {
    .name	= "powr1220",
    },
    .probe		= powr1220_probe,
    .id_table	= powr1220_ids,
    };
    module_i2c_driver(powr1220_driver);
    MODULE_AUTHOR("Scott Kanowitz");
    MODULE_DESCRIPTION("POWR1220 driver");
    MODULE_LICENSE("GPL");
