//! Automatically rewritten from C to Rust
//! Source: drivers/iio/health/max30100.c
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
// max30100.c - Support for MAX30100 heart rate and pulse oximeter sensor
//
// Copyright (C) 2015, 2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const MAX30100_REG_INT_STATUS: c_uint = 0x00;

pub const MAX30100_REG_INT_ENABLE: c_uint = 0x01;

pub const MAX30100_REG_INT_ENABLE_MASK: c_uint = 0xf0;
pub const MAX30100_REG_INT_ENABLE_MASK_SHIFT: c_int = 4;
pub const MAX30100_REG_FIFO_WR_PTR: c_uint = 0x02;
pub const MAX30100_REG_FIFO_OVR_CTR: c_uint = 0x03;
pub const MAX30100_REG_FIFO_RD_PTR: c_uint = 0x04;
pub const MAX30100_REG_FIFO_DATA: c_uint = 0x05;
pub const MAX30100_REG_FIFO_DATA_ENTRY_COUNT: c_int = 16;
pub const MAX30100_REG_FIFO_DATA_ENTRY_LEN: c_int = 4;
pub const MAX30100_REG_MODE_CONFIG: c_uint = 0x06;

pub const MAX30100_REG_MODE_CONFIG_MODE_MASK: c_uint = 0x03;

pub const MAX30100_REG_SPO2_CONFIG: c_uint = 0x07;

pub const MAX30100_REG_SPO2_CONFIG_200US: c_uint = 0x0;
pub const MAX30100_REG_SPO2_CONFIG_400US: c_uint = 0x1;
pub const MAX30100_REG_SPO2_CONFIG_800US: c_uint = 0x2;
pub const MAX30100_REG_SPO2_CONFIG_1600US: c_uint = 0x3;

pub const MAX30100_REG_LED_CONFIG: c_uint = 0x09;
pub const MAX30100_REG_LED_CONFIG_LED_MASK: c_uint = 0x0f;
pub const MAX30100_REG_LED_CONFIG_RED_LED_SHIFT: c_int = 4;
pub const MAX30100_REG_LED_CONFIG_24MA: c_uint = 0x07;
pub const MAX30100_REG_LED_CONFIG_50MA: c_uint = 0x0f;
pub const MAX30100_REG_TEMP_INTEGER: c_uint = 0x16;
pub const MAX30100_REG_TEMP_FRACTION: c_uint = 0x17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max30100_data {
    pub client: *mut i2c_client,
    pub indio_dev: *mut iio_dev,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub /: *mut *mut __be16 buffer[2]; / 2 16-bit channels,
}

#[no_mangle]
unsafe extern "C" fn max30100_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max30100_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX30100_REG_INT_STATUS:
    case MAX30100_REG_MODE_CONFIG:
    case MAX30100_REG_FIFO_WR_PTR:
    case MAX30100_REG_FIFO_OVR_CTR:
    case MAX30100_REG_FIFO_RD_PTR:
    case MAX30100_REG_FIFO_DATA:
    case MAX30100_REG_TEMP_INTEGER:
    case MAX30100_REG_TEMP_FRACTION:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config max30100_regmap_config = {
    .name = "max30100_regmap",
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX30100_REG_TEMP_FRACTION,
    .cache_type = REGCACHE_FLAT,
    .volatile_reg = max30100_is_volatile_reg,
    };
    static const unsigned int max30100_led_current_mapping[] = {
    4400, 7600, 11000, 14200, 17400,
    20800, 24000, 27100, 30600, 33800,
    37000, 40200, 43600, 46800, 50000
    };
    static const unsigned long max30100_scan_masks[] = {0x3, 0};
    static const struct iio_chan_spec max30100_channels[] = {
    {
    .type = IIO_INTENSITY,
    .channel2 = IIO_MOD_LIGHT_IR,
    .modified = 1,
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    },
    {
    .type = IIO_INTENSITY,
    .channel2 = IIO_MOD_LIGHT_RED,
    .modified = 1,
    .scan_index = 1,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    },
    {
    .type = IIO_TEMP,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = -1,
    },
    };
#[no_mangle]
unsafe extern "C" fn max30100_set_powermode(data: *mut max30100_data, state: bool) -> c_int {
    static int max30100_set_powermode(struct max30100_data *data, bool state)
    {
    return regmap_update_bits(data.regmap, MAX30100_REG_MODE_CONFIG,
    MAX30100_REG_MODE_CONFIG_PWR,
    state ? 0 : MAX30100_REG_MODE_CONFIG_PWR);
    }
#[no_mangle]
unsafe extern "C" fn max30100_clear_fifo(data: *mut max30100_data) -> c_int {
    static int max30100_clear_fifo(struct max30100_data *data)
    {
    int ret;
    ret = regmap_write(data.regmap, MAX30100_REG_FIFO_WR_PTR, 0);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, MAX30100_REG_FIFO_OVR_CTR, 0);
    if (ret)
    return ret;
    return regmap_write(data.regmap, MAX30100_REG_FIFO_RD_PTR, 0);
    }
#[no_mangle]
unsafe extern "C" fn max30100_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int max30100_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct max30100_data *data = iio_priv(indio_dev);
    int ret;
    ret = max30100_set_powermode(data, true);
    if (ret)
    return ret;
    return max30100_clear_fifo(data);
    }
#[no_mangle]
unsafe extern "C" fn max30100_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int max30100_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct max30100_data *data = iio_priv(indio_dev);
    return max30100_set_powermode(data, false);
    }
    static const struct iio_buffer_setup_ops max30100_buffer_setup_ops = {
    .postenable = max30100_buffer_postenable,
    .predisable = max30100_buffer_predisable,
    };
#[no_mangle]
pub unsafe extern "C" fn max30100_fifo_count(data: *mut max30100_data) -> c_int {
    static inline int max30100_fifo_count(struct max30100_data *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(data.regmap, MAX30100_REG_INT_STATUS, &val);
    if (ret)
    return ret;
// FIFO is almost full
    if (val & MAX30100_REG_INT_STATUS_FIFO_RDY)
    return MAX30100_REG_FIFO_DATA_ENTRY_COUNT - 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max30100_read_measurement(data: *mut max30100_data) -> c_int {
    static int max30100_read_measurement(struct max30100_data *data)
    {
    int ret;
    ret = i2c_smbus_read_i2c_block_data(data.client,
    MAX30100_REG_FIFO_DATA,
    MAX30100_REG_FIFO_DATA_ENTRY_LEN,
    (u8 *) &data.buffer);
    return (ret == MAX30100_REG_FIFO_DATA_ENTRY_LEN) ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn max30100_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t max30100_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct max30100_data *data = iio_priv(indio_dev);
    int ret, cnt = 0;
    mutex_lock(&data.lock);
    while (cnt || (cnt = max30100_fifo_count(data)) > 0) {
    ret = max30100_read_measurement(data);
    if (ret)
    break;
    iio_push_to_buffers(data.indio_dev, data.buffer);
    cnt--;
    }
    mutex_unlock(&data.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max30100_get_current_idx(val: c_uint, reg: *mut c_int) -> c_int {
    static int max30100_get_current_idx(unsigned int val, int *reg)
    {
    int idx;
// LED turned off
    if (val == 0) {
// reg = 0;
    return 0;
    }
    for (idx = 0; idx < ARRAY_SIZE(max30100_led_current_mapping); idx++) {
    if (max30100_led_current_mapping[idx] == val) {
// reg = idx + 1;
    return 0;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn max30100_led_init(data: *mut max30100_data) -> c_int {
    static int max30100_led_init(struct max30100_data *data)
    {
    struct device *dev = &data.client.dev;
    unsigned int val[2];
    int reg, ret;
    ret = device_property_read_u32_array(dev, "maxim,led-current-microamp",
    (unsigned int *) &val, 2);
    if (ret) {
// Default to 24 mA RED LED, 50 mA IR LED
    reg = (MAX30100_REG_LED_CONFIG_24MA <<
    MAX30100_REG_LED_CONFIG_RED_LED_SHIFT) |
    MAX30100_REG_LED_CONFIG_50MA;
    dev_warn(dev, "no led-current-microamp set");
    return regmap_write(data.regmap, MAX30100_REG_LED_CONFIG, reg);
    }
// RED LED current
    ret = max30100_get_current_idx(val[0], &reg);
    if (ret) {
    dev_err(dev, "invalid RED current setting %d", val[0]);
    return ret;
    }
    ret = regmap_update_bits(data.regmap, MAX30100_REG_LED_CONFIG,
    MAX30100_REG_LED_CONFIG_LED_MASK <<
    MAX30100_REG_LED_CONFIG_RED_LED_SHIFT,
    reg << MAX30100_REG_LED_CONFIG_RED_LED_SHIFT);
    if (ret)
    return ret;
// IR LED current
    ret = max30100_get_current_idx(val[1], &reg);
    if (ret) {
    dev_err(dev, "invalid IR current setting %d", val[1]);
    return ret;
    }
    return regmap_update_bits(data.regmap, MAX30100_REG_LED_CONFIG,
    MAX30100_REG_LED_CONFIG_LED_MASK, reg);
    }
#[no_mangle]
unsafe extern "C" fn max30100_get_pulse_width(pwidth_us: c_uint) -> c_int {
    static int max30100_get_pulse_width(unsigned int pwidth_us)
    {
    switch (pwidth_us) {
    case 200:
    return MAX30100_REG_SPO2_CONFIG_200US;
    case 400:
    return MAX30100_REG_SPO2_CONFIG_400US;
    case 800:
    return MAX30100_REG_SPO2_CONFIG_800US;
    case 1600:
    return MAX30100_REG_SPO2_CONFIG_1600US;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn max30100_chip_init(data: *mut max30100_data) -> c_int {
    static int max30100_chip_init(struct max30100_data *data)
    {
    int ret;
    int pulse_width;
// set default LED pulse-width to 1600 us
    let mut pulse_us: c_uint = 1600;
    struct device *dev = &data.client.dev;
// setup LED current settings
    ret = max30100_led_init(data);
    if (ret)
    return ret;
// Read LED pulse-width-us from DT
    device_property_read_u32(dev, "maxim,pulse-width-us", &pulse_us);
    pulse_width = max30100_get_pulse_width(pulse_us);
    if (pulse_width < 0)
    return dev_err_probe(dev, pulse_width, "invalid LED pulse-width %uus\n", pulse_us);
// enable hi-res SPO2 readings at 100Hz
    ret = regmap_write(data.regmap, MAX30100_REG_SPO2_CONFIG,
    MAX30100_REG_SPO2_CONFIG_HI_RES_EN |
    MAX30100_REG_SPO2_CONFIG_100HZ |
    FIELD_PREP(MAX30100_REG_SPO2_CONFIG_PW_MASK, pulse_width));
    if (ret)
    return ret;
// enable SPO2 mode
    ret = regmap_update_bits(data.regmap, MAX30100_REG_MODE_CONFIG,
    MAX30100_REG_MODE_CONFIG_MODE_MASK,
    MAX30100_REG_MODE_CONFIG_MODE_HR_EN |
    MAX30100_REG_MODE_CONFIG_MODE_SPO2_EN);
    if (ret)
    return ret;
// enable FIFO interrupt
    return regmap_update_bits(data.regmap, MAX30100_REG_INT_ENABLE,
    MAX30100_REG_INT_ENABLE_MASK,
    MAX30100_REG_INT_ENABLE_FIFO_EN
    << MAX30100_REG_INT_ENABLE_MASK_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn max30100_read_temp(data: *mut max30100_data, val: *mut c_int) -> c_int {
    static int max30100_read_temp(struct max30100_data *data, int *val)
    {
    int ret;
    unsigned int reg;
    ret = regmap_read(data.regmap, MAX30100_REG_TEMP_INTEGER, &reg);
    if (ret < 0)
    return ret;
// val = reg << 4;
    ret = regmap_read(data.regmap, MAX30100_REG_TEMP_FRACTION, &reg);
    if (ret < 0)
    return ret;
// val |= reg & 0xf;
// val = sign_extend32(*val, 11);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max30100_get_temp(data: *mut max30100_data, val: *mut c_int) -> c_int {
    static int max30100_get_temp(struct max30100_data *data, int *val)
    {
    int ret;
// start acquisition
    ret = regmap_set_bits(data.regmap, MAX30100_REG_MODE_CONFIG,
    MAX30100_REG_MODE_CONFIG_TEMP_EN);
    if (ret)
    return ret;
    msleep(35);
    return max30100_read_temp(data, val);
    }
    static int max30100_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max30100_data *data = iio_priv(indio_dev);
    let mut ret: c_int = -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
//
// Temperature reading can only be acquired while engine
// is running
//
    if (!iio_device_try_claim_buffer_mode(indio_dev)) {
    ret = -EAGAIN;
    } else {
    ret = max30100_get_temp(data, val);
    if (!ret)
    ret = IIO_VAL_INT;
    iio_device_release_buffer_mode(indio_dev);
    }
    break;
    case IIO_CHAN_INFO_SCALE:
// val = 1;  /* 0.0625
// val2 = 16;
    ret = IIO_VAL_FRACTIONAL;
    break;
    }
    return ret;
    }
    static const struct iio_info max30100_info = {
    .read_raw = max30100_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn max30100_probe(client: *mut i2c_client) -> c_int {
    static int max30100_probe(struct i2c_client *client)
    {
    struct max30100_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    indio_dev.name = MAX30100_DRV_NAME;
    indio_dev.channels = max30100_channels;
    indio_dev.info = &max30100_info;
    indio_dev.num_channels = ARRAY_SIZE(max30100_channels);
    indio_dev.available_scan_masks = max30100_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = devm_iio_kfifo_buffer_setup(&client.dev, indio_dev,
    &max30100_buffer_setup_ops);
    if (ret)
    return ret;
    data = iio_priv(indio_dev);
    data.indio_dev = indio_dev;
    data.client = client;
    mutex_init(&data.lock);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &max30100_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "regmap initialization failed.\n");
    return PTR_ERR(data.regmap);
    }
    max30100_set_powermode(data, false);
    ret = max30100_chip_init(data);
    if (ret)
    return ret;
    if (client.irq <= 0) {
    dev_err(&client.dev, "no valid irq defined\n");
    return -EINVAL;
    }
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), max30100_interrupt_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "max30100_irq", indio_dev);
    if (ret)
    return ret;
    return iio_device_register(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn max30100_remove(client: *mut i2c_client) {
    static void max30100_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct max30100_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    max30100_set_powermode(data, false);
    }
    static const struct i2c_device_id max30100_id[] = {
    { .name = "max30100" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max30100_id);
    static const struct of_device_id max30100_dt_ids[] = {
    { .compatible = "maxim,max30100" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max30100_dt_ids);
    static struct i2c_driver max30100_driver = {
    .driver = {
    .name	= MAX30100_DRV_NAME,
    .of_match_table	= max30100_dt_ids,
    },
    .probe		= max30100_probe,
    .remove		= max30100_remove,
    .id_table	= max30100_id,
    };
    module_i2c_driver(max30100_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("MAX30100 heart rate and pulse oximeter sensor");
    MODULE_LICENSE("GPL");
