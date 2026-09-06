//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/max9611.c
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
// iio/adc/max9611.c
//
// Maxim max9611/max9612 high side current sense amplifier with
// 12-bit ADC interface.
//
// Copyright (C) 2017 Jacopo Mondi
//
// This driver supports input common-mode voltage, current-sense
// amplifier with programmable gains and die temperature reading from
// Maxim max9611/max9612.
//
// Op-amp, analog comparator, and watchdog functionalities are not
// supported by this driver.
//

// max9611 register addresses
pub const MAX9611_REG_CSA_DATA: c_uint = 0x00;
pub const MAX9611_REG_RS_DATA: c_uint = 0x02;
pub const MAX9611_REG_TEMP_DATA: c_uint = 0x08;
pub const MAX9611_REG_CTRL1: c_uint = 0x0a;
pub const MAX9611_REG_CTRL2: c_uint = 0x0b;
// max9611 REG1 mux configuration options

pub const MAX9611_MUX_SENSE_1x: c_uint = 0x00;
pub const MAX9611_MUX_SENSE_4x: c_uint = 0x01;
pub const MAX9611_MUX_SENSE_8x: c_uint = 0x02;
pub const MAX9611_INPUT_VOLT: c_uint = 0x03;
pub const MAX9611_MUX_TEMP: c_uint = 0x06;
// max9611 voltage (both csa and input) helper macros
pub const MAX9611_VOLTAGE_SHIFT: c_uint = 0x04;

//
// max9611 current sense amplifier voltage output:
// LSB and offset values depends on selected gain (1x, 4x, 8x)
//
// GAIN		LSB (nV)	OFFSET (LSB steps)
// 1x		107500		1
// 4x		26880		1
// 8x		13440		3
//
// The complete formula to calculate current sense voltage is:
// (((adc_read >> 4) - offset) / ((1 / LSB) * 10^-3)
//
pub const MAX9611_CSA_1X_LSB_nV: c_int = 107500;
pub const MAX9611_CSA_4X_LSB_nV: c_int = 26880;
pub const MAX9611_CSA_8X_LSB_nV: c_int = 13440;
pub const MAX9611_CSA_1X_OFFS_RAW: c_int = 1;
pub const MAX9611_CSA_4X_OFFS_RAW: c_int = 1;
pub const MAX9611_CSA_8X_OFFS_RAW: c_int = 3;
//
// max9611 common input mode (CIM): LSB is 14mV, with 14mV offset at 25 C
//
// The complete formula to calculate input common voltage is:
// (((adc_read >> 4) * 1000) - offset) / (1 / 14 * 1000)
//
pub const MAX9611_CIM_LSB_mV: c_int = 14;
pub const MAX9611_CIM_OFFSET_RAW: c_int = 1;
//
// max9611 temperature reading: LSB is 480 milli degrees Celsius
//
// The complete formula to calculate temperature is:
// ((adc_read >> 7) * 1000) / (1 / 480 * 1000)
//
pub const MAX9611_TEMP_MAX_POS: c_uint = 0x7f80;
pub const MAX9611_TEMP_MAX_NEG: c_uint = 0xff80;
pub const MAX9611_TEMP_MIN_NEG: c_uint = 0xd980;

pub const MAX9611_TEMP_SHIFT: c_uint = 0x07;

pub const MAX9611_TEMP_SCALE_NUM: c_int = 1000000;
pub const MAX9611_TEMP_SCALE_DIV: c_int = 2083;
//
// Conversion time is 2 ms (typically) at Ta=25 degreeC
// No maximum value is known, so play it safe.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max9611_dev {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub lock: mutex,
    pub shunt_resistor_uohm: c_uint,
}

    enum max9611_conf_ids {
    CONF_SENSE_1x,
    CONF_SENSE_4x,
    CONF_SENSE_8x,
    CONF_IN_VOLT,
    CONF_TEMP,
    };
//
// max9611_mux_conf - associate ADC mux configuration with register address
// where data shall be read from
//
    static const unsigned int max9611_mux_conf[][2] = {
    [CONF_SENSE_1x]	= { MAX9611_MUX_SENSE_1x, MAX9611_REG_CSA_DATA },
    [CONF_SENSE_4x]	= { MAX9611_MUX_SENSE_4x, MAX9611_REG_CSA_DATA },
    [CONF_SENSE_8x]	= { MAX9611_MUX_SENSE_8x, MAX9611_REG_CSA_DATA },
    [CONF_IN_VOLT]	= { MAX9611_INPUT_VOLT, MAX9611_REG_RS_DATA },
    [CONF_TEMP]	= { MAX9611_MUX_TEMP, MAX9611_REG_TEMP_DATA },
    };
    enum max9611_csa_gain {
    CSA_GAIN_1x = CONF_SENSE_1x,
    CSA_GAIN_4x = CONF_SENSE_4x,
    CSA_GAIN_8x = CONF_SENSE_8x,
    };
    enum max9611_csa_gain_params {
    CSA_GAIN_LSB_nV,
    CSA_GAIN_OFFS_RAW,
    };
//
// max9611_csa_gain_conf - associate gain multiplier with LSB and
// offset values.
//
// Group together parameters associated with configurable gain
// on current sense amplifier path to ADC interface.
// Current sense read routine adjusts gain until it gets a meaningful
// value; use this structure to retrieve the correct LSB and offset values.
//
    static const unsigned int max9611_gain_conf[][2] = {
    [CSA_GAIN_1x] = { MAX9611_CSA_1X_LSB_nV, MAX9611_CSA_1X_OFFS_RAW, },
    [CSA_GAIN_4x] = { MAX9611_CSA_4X_LSB_nV, MAX9611_CSA_4X_OFFS_RAW, },
    [CSA_GAIN_8x] = { MAX9611_CSA_8X_LSB_nV, MAX9611_CSA_8X_OFFS_RAW, },
    };
    enum max9611_chan_addrs {
    MAX9611_CHAN_VOLTAGE_INPUT,
    MAX9611_CHAN_VOLTAGE_SENSE,
    MAX9611_CHAN_TEMPERATURE,
    MAX9611_CHAN_CURRENT_LOAD,
    MAX9611_CHAN_POWER_LOAD,
    };
    static const struct iio_chan_spec max9611_channels[] = {
    {
    .type			= IIO_TEMP,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .address		= MAX9611_CHAN_TEMPERATURE,
    },
    {
    .type			= IIO_VOLTAGE,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_PROCESSED),
    .address		= MAX9611_CHAN_VOLTAGE_SENSE,
    .indexed		= 1,
    .channel		= 0,
    },
    {
    .type			= IIO_VOLTAGE,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_RAW)   |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .address		= MAX9611_CHAN_VOLTAGE_INPUT,
    .indexed		= 1,
    .channel		= 1,
    },
    {
    .type			= IIO_CURRENT,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_PROCESSED),
    .address		= MAX9611_CHAN_CURRENT_LOAD,
    },
    {
    .type			= IIO_POWER,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_PROCESSED),
    .address		= MAX9611_CHAN_POWER_LOAD
    },
    };
//
// max9611_read_single() - read a single value from ADC interface
//
// Data registers are 16 bit long, spread between two 8 bit registers
// with consecutive addresses.
// Configure ADC mux first, then read register at address "reg_addr".
// The smbus_read_word routine asks for 16 bits and the ADC is kind enough
// to return values from "reg_addr" and "reg_addr + 1" consecutively.
// Data are transmitted with big-endian ordering: MSB arrives first.
//
// @max9611: max9611 device
// @selector: index for mux and register configuration
// @raw_val: the value returned from ADC
//
    static int max9611_read_single(struct max9611_dev *max9611,
    enum max9611_conf_ids selector,
    u16 *raw_val)
    {
    int ret;
    let mut mux_conf: u8 = max9611_mux_conf[selector][0] & MAX9611_MUX_MASK;
    let mut reg_addr: u8 = max9611_mux_conf[selector][1];
//
// Keep mutex lock held during read-write to avoid mux register
// (CTRL1) re-configuration.
//
    mutex_lock(&max9611.lock);
    ret = i2c_smbus_write_byte_data(max9611.i2c_client,
    MAX9611_REG_CTRL1, mux_conf);
    if (ret) {
    dev_err(max9611.dev, "i2c write byte failed: 0x%2x - 0x%2x\n",
    MAX9611_REG_CTRL1, mux_conf);
    mutex_unlock(&max9611.lock);
    return ret;
    }
// need a delay here to make register configuration stabilize.
    usleep_range(MAX9611_CONV_TIME_US_RANGE);
    ret = i2c_smbus_read_word_swapped(max9611.i2c_client, reg_addr);
    if (ret < 0) {
    dev_err(max9611.dev, "i2c read word from 0x%2x failed\n",
    reg_addr);
    mutex_unlock(&max9611.lock);
    return ret;
    }
// raw_val = ret;
    mutex_unlock(&max9611.lock);
    return 0;
    }
//
// max9611_read_csa_voltage() - read current sense amplifier output voltage
//
// Current sense amplifier output voltage is read through a configurable
// 1x, 4x or 8x gain.
// Start with plain 1x gain, and adjust gain control properly until a
// meaningful value is read from ADC output.
//
// @max9611: max9611 device
// @adc_raw: raw value read from ADC output
// @csa_gain: gain configuration option selector
//
    static int max9611_read_csa_voltage(struct max9611_dev *max9611,
    u16 *adc_raw,
    enum max9611_csa_gain *csa_gain)
    {
    enum max9611_conf_ids gain_selectors[] = {
    CONF_SENSE_1x,
    CONF_SENSE_4x,
    CONF_SENSE_8x
    };
    unsigned int i;
    int ret;
    for (i = 0; i < ARRAY_SIZE(gain_selectors); ++i) {
    ret = max9611_read_single(max9611, gain_selectors[i], adc_raw);
    if (ret)
    return ret;
    if (*adc_raw > 0) {
// csa_gain = (enum max9611_csa_gain)gain_selectors[i];
    return 0;
    }
    }
    return -EIO;
    }
    static int max9611_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max9611_dev *dev = iio_priv(indio_dev);
    enum max9611_csa_gain gain_selector;
    const unsigned int *csa_gain;
    u16 adc_data;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.address) {
    case MAX9611_CHAN_TEMPERATURE:
    ret = max9611_read_single(dev, CONF_TEMP,
    &adc_data);
    if (ret)
    return -EINVAL;
// val = MAX9611_TEMP_RAW(adc_data);
    return IIO_VAL_INT;
    case MAX9611_CHAN_VOLTAGE_INPUT:
    ret = max9611_read_single(dev, CONF_IN_VOLT,
    &adc_data);
    if (ret)
    return -EINVAL;
// val = MAX9611_VOLTAGE_RAW(adc_data);
    return IIO_VAL_INT;
    }
    break;
    case IIO_CHAN_INFO_OFFSET:
// MAX9611_CHAN_VOLTAGE_INPUT
// val = MAX9611_CIM_OFFSET_RAW;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.address) {
    case MAX9611_CHAN_TEMPERATURE:
// val = MAX9611_TEMP_SCALE_NUM;
// val2 = MAX9611_TEMP_SCALE_DIV;
    return IIO_VAL_FRACTIONAL;
    case MAX9611_CHAN_VOLTAGE_INPUT:
// val = MAX9611_CIM_LSB_mV;
    return IIO_VAL_INT;
    }
    break;
    case IIO_CHAN_INFO_PROCESSED:
    switch (chan.address) {
    case MAX9611_CHAN_VOLTAGE_SENSE:
//
// processed (mV): (raw - offset) * LSB (nV) / 10^6
//
// Even if max9611 can output raw csa voltage readings,
// use a produced value as scale depends on gain.
//
    ret = max9611_read_csa_voltage(dev, &adc_data,
    &gain_selector);
    if (ret)
    return -EINVAL;
    csa_gain = max9611_gain_conf[gain_selector];
    adc_data -= csa_gain[CSA_GAIN_OFFS_RAW];
// val = MAX9611_VOLTAGE_RAW(adc_data)
    csa_gain[CSA_GAIN_LSB_nV];
// val2 = 1000000;
    return IIO_VAL_FRACTIONAL;
    case MAX9611_CHAN_CURRENT_LOAD:
// processed (mA): Vcsa (nV) / Rshunt (uOhm)
    ret = max9611_read_csa_voltage(dev, &adc_data,
    &gain_selector);
    if (ret)
    return -EINVAL;
    csa_gain = max9611_gain_conf[gain_selector];
    adc_data -= csa_gain[CSA_GAIN_OFFS_RAW];
// val = MAX9611_VOLTAGE_RAW(adc_data)
    csa_gain[CSA_GAIN_LSB_nV];
// val2 = dev->shunt_resistor_uohm;
    return IIO_VAL_FRACTIONAL;
    case MAX9611_CHAN_POWER_LOAD:
//
// processed (mW): Vin (mV) * Vcsa (uV)
// Rshunt (uOhm)
//
    ret = max9611_read_single(dev, CONF_IN_VOLT,
    &adc_data);
    if (ret)
    return -EINVAL;
    adc_data -= MAX9611_CIM_OFFSET_RAW;
// val = MAX9611_VOLTAGE_RAW(adc_data)
    MAX9611_CIM_LSB_mV;
    ret = max9611_read_csa_voltage(dev, &adc_data,
    &gain_selector);
    if (ret)
    return -EINVAL;
    csa_gain = max9611_gain_conf[gain_selector];
// divide by 10^3 here to avoid 32bit overflow
    adc_data -= csa_gain[CSA_GAIN_OFFS_RAW];
// val *= MAX9611_VOLTAGE_RAW(adc_data)
    csa_gain[CSA_GAIN_LSB_nV] / 1000;
// val2 = dev->shunt_resistor_uohm;
    return IIO_VAL_FRACTIONAL;
    }
    break;
    }
    return -EINVAL;
    }
    static ssize_t max9611_shunt_resistor_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct max9611_dev *max9611 = iio_priv(dev_to_iio_dev(dev));
    unsigned int i, r;
    i = max9611.shunt_resistor_uohm / 1000000;
    r = max9611.shunt_resistor_uohm % 1000000;
    return sysfs_emit(buf, "%u.%06u\n", i, r);
    }
    static IIO_DEVICE_ATTR(in_power_shunt_resistor, 0444,
    max9611_shunt_resistor_show, core::ptr::null_mut(), 0);
    static IIO_DEVICE_ATTR(in_current_shunt_resistor, 0444,
    max9611_shunt_resistor_show, core::ptr::null_mut(), 0);
    static struct attribute *max9611_attributes[] = {
    &iio_dev_attr_in_power_shunt_resistor.dev_attr.attr,
    &iio_dev_attr_in_current_shunt_resistor.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group max9611_attribute_group = {
    .attrs = max9611_attributes,
    };
    static const struct iio_info indio_info = {
    .read_raw	= max9611_read_raw,
    .attrs		= &max9611_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn max9611_init(max9611: *mut max9611_dev) -> c_int {
    static int max9611_init(struct max9611_dev *max9611)
    {
    struct i2c_client *client = max9611.i2c_client;
    u16 regval;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WRITE_BYTE	|
    I2C_FUNC_SMBUS_READ_WORD_DATA)) {
    dev_err(max9611.dev,
    "I2c adapter does not support smbus write_byte or read_word functionalities: aborting probe.\n");
    return -EINVAL;
    }
// Make sure die temperature is in range to test communications.
    ret = max9611_read_single(max9611, CONF_TEMP, &regval);
    if (ret)
    return ret;
    regval &= MAX9611_TEMP_MASK;
    if ((regval > MAX9611_TEMP_MAX_POS &&
    regval < MAX9611_TEMP_MIN_NEG) ||
    regval > MAX9611_TEMP_MAX_NEG) {
    dev_err(max9611.dev,
    "Invalid value received from ADC 0x%4x: aborting\n",
    regval);
    return -EIO;
    }
// Mux shall be zeroed back before applying other configurations
    ret = i2c_smbus_write_byte_data(max9611.i2c_client,
    MAX9611_REG_CTRL1, 0);
    if (ret) {
    dev_err(max9611.dev, "i2c write byte failed: 0x%2x - 0x%2x\n",
    MAX9611_REG_CTRL1, 0);
    return ret;
    }
    ret = i2c_smbus_write_byte_data(max9611.i2c_client,
    MAX9611_REG_CTRL2, 0);
    if (ret) {
    dev_err(max9611.dev, "i2c write byte failed: 0x%2x - 0x%2x\n",
    MAX9611_REG_CTRL2, 0);
    return ret;
    }
    usleep_range(MAX9611_CONV_TIME_US_RANGE);
    return 0;
    }
    static const struct of_device_id max9611_of_table[] = {
    { .compatible = "maxim,max9611", .data = "max9611" },
    { .compatible = "maxim,max9612", .data = "max9612" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max9611_of_table);
#[no_mangle]
unsafe extern "C" fn max9611_probe(client: *mut i2c_client) -> c_int {
    static int max9611_probe(struct i2c_client *client)
    {
    let mut shunt_res_prop: *const char  const = "shunt-resistor-micro-ohms";
    struct max9611_dev *max9611;
    struct iio_dev *indio_dev;
    struct device *dev = &client.dev;
    unsigned int of_shunt;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*max9611));
    if (!indio_dev)
    return -ENOMEM;
    i2c_set_clientdata(client, indio_dev);
    max9611			= iio_priv(indio_dev);
    max9611.dev		= dev;
    max9611.i2c_client	= client;
    mutex_init(&max9611.lock);
    ret = device_property_read_u32(dev, shunt_res_prop, &of_shunt);
    if (ret) {
    dev_err(dev, "Missing %s property for %pfw node\n",
    shunt_res_prop, dev_fwnode(dev));
    return ret;
    }
    max9611.shunt_resistor_uohm = of_shunt;
    ret = max9611_init(max9611);
    if (ret)
    return ret;
    indio_dev.name		= device_get_match_data(dev);
    indio_dev.modes	= INDIO_DIRECT_MODE;
    indio_dev.info		= &indio_info;
    indio_dev.channels	= max9611_channels;
    indio_dev.num_channels	= ARRAY_SIZE(max9611_channels);
    return devm_iio_device_register(dev, indio_dev);
    }
    static struct i2c_driver max9611_driver = {
    .driver = {
    .name = "max9611",
    .of_match_table = max9611_of_table,
    },
    .probe = max9611_probe,
    };
    module_i2c_driver(max9611_driver);
    MODULE_AUTHOR("Jacopo Mondi <jacopo+renesas@jmondi.org>");
    MODULE_DESCRIPTION("Maxim max9611/12 current sense amplifier with 12bit ADC");
    MODULE_LICENSE("GPL v2");
