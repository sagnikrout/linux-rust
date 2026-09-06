//! Automatically rewritten from C to Rust
//! Source: drivers/iio/health/max30102.c
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
// max30102.c - Support for MAX30102 heart rate and pulse oximeter sensor
//
// Copyright (C) 2017 Matt Ranostay <matt.ranostay@konsulko.com>
//
// Support for MAX30105 optical particle sensor
// Copyright (C) 2017 Peter Meerwald-Stadler <pmeerw@pmeerw.net>
//
// 7-bit I2C chip address: 0x57
// TODO: proximity power saving feature
//

pub const MAX30102_PART_NUMBER: c_uint = 0x15;
    enum max30102_chip_id {
    max30102,
    max30105,
    };
    enum max3012_led_idx {
    MAX30102_LED_RED,
    MAX30102_LED_IR,
    MAX30105_LED_GREEN,
    };
pub const MAX30102_REG_INT_STATUS: c_uint = 0x00;

pub const MAX30102_REG_INT_ENABLE: c_uint = 0x02;

pub const MAX30102_REG_INT_ENABLE_MASK: c_uint = 0xf0;
pub const MAX30102_REG_INT_ENABLE_MASK_SHIFT: c_int = 4;
pub const MAX30102_REG_FIFO_WR_PTR: c_uint = 0x04;
pub const MAX30102_REG_FIFO_OVR_CTR: c_uint = 0x05;
pub const MAX30102_REG_FIFO_RD_PTR: c_uint = 0x06;
pub const MAX30102_REG_FIFO_DATA: c_uint = 0x07;
pub const MAX30102_REG_FIFO_DATA_BYTES: c_int = 3;
pub const MAX30102_REG_FIFO_CONFIG: c_uint = 0x08;

pub const MAX30102_REG_FIFO_CONFIG_AVG_SHIFT: c_int = 5;

pub const MAX30102_REG_MODE_CONFIG: c_uint = 0x09;
pub const MAX30102_REG_MODE_CONFIG_MODE_NONE: c_uint = 0x00;
pub const MAX30102_REG_MODE_CONFIG_MODE_HR: c_uint = 0x02 /* red LED */;
pub const MAX30102_REG_MODE_CONFIG_MODE_HR_SPO2: c_uint = 0x03 /* red + IR LED */;
pub const MAX30102_REG_MODE_CONFIG_MODE_MULTI: c_uint = 0x07 /* multi-LED mode */;

pub const MAX30102_REG_MODE_CONTROL_SLOT21: c_uint = 0x11 /* multi-LED control */;
pub const MAX30102_REG_MODE_CONTROL_SLOT43: c_uint = 0x12;

pub const MAX30102_REG_MODE_CONTROL_SLOT_SHIFT: c_int = 4;
pub const MAX30102_REG_SPO2_CONFIG: c_uint = 0x0a;
pub const MAX30102_REG_SPO2_CONFIG_PULSE_411_US: c_uint = 0x03;
pub const MAX30102_REG_SPO2_CONFIG_SR_400HZ: c_uint = 0x03;
pub const MAX30102_REG_SPO2_CONFIG_SR_MASK: c_uint = 0x07;
pub const MAX30102_REG_SPO2_CONFIG_SR_MASK_SHIFT: c_int = 2;

pub const MAX30102_REG_SPO2_CONFIG_ADC_MASK_SHIFT: c_int = 5;
pub const MAX30102_REG_RED_LED_CONFIG: c_uint = 0x0c;
pub const MAX30102_REG_IR_LED_CONFIG: c_uint = 0x0d;
pub const MAX30105_REG_GREEN_LED_CONFIG: c_uint = 0x0e;
pub const MAX30102_REG_TEMP_CONFIG: c_uint = 0x21;

pub const MAX30102_REG_TEMP_INTEGER: c_uint = 0x1f;
pub const MAX30102_REG_TEMP_FRACTION: c_uint = 0x20;
pub const MAX30102_REG_REV_ID: c_uint = 0xfe;
pub const MAX30102_REG_PART_ID: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max30102_data {
    pub client: *mut i2c_client,
    pub indio_dev: *mut iio_dev,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub chip_id: enum max30102_chip_id,
    pub buffer: [u8; 12],
    pub /: *mut *mut __be32 processed_buffer[3]; / 3 x 18-bit (padded to 32-bits),
}

    static const struct regmap_config max30102_regmap_config = {
    .name = "max30102_regmap",
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const unsigned long max30102_scan_masks[] = {
    BIT(MAX30102_LED_RED) | BIT(MAX30102_LED_IR),
    0
    };
    static const unsigned long max30105_scan_masks[] = {
    BIT(MAX30102_LED_RED) | BIT(MAX30102_LED_IR),
    BIT(MAX30102_LED_RED) | BIT(MAX30102_LED_IR) |
    BIT(MAX30105_LED_GREEN),
    0
    };

    .type = IIO_INTENSITY, \
    .channel2 = _mod, \
    .modified = 1, \
    .scan_index = _si, \
    .scan_type = { \
    .sign = 'u', \
    .shift = 8, \
    .realbits = 18, \
    .storagebits = 32, \
    .endianness = IIO_BE, \
    }, \
    }
    static const struct iio_chan_spec max30102_channels[] = {
    MAX30102_INTENSITY_CHANNEL(MAX30102_LED_RED, IIO_MOD_LIGHT_RED),
    MAX30102_INTENSITY_CHANNEL(MAX30102_LED_IR, IIO_MOD_LIGHT_IR),
    {
    .type = IIO_TEMP,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = -1,
    },
    };
    static const struct iio_chan_spec max30105_channels[] = {
    MAX30102_INTENSITY_CHANNEL(MAX30102_LED_RED, IIO_MOD_LIGHT_RED),
    MAX30102_INTENSITY_CHANNEL(MAX30102_LED_IR, IIO_MOD_LIGHT_IR),
    MAX30102_INTENSITY_CHANNEL(MAX30105_LED_GREEN, IIO_MOD_LIGHT_GREEN),
    {
    .type = IIO_TEMP,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = -1,
    },
    };
#[no_mangle]
unsafe extern "C" fn max30102_set_power(data: *mut max30102_data, en: bool) -> c_int {
    static int max30102_set_power(struct max30102_data *data, bool en)
    {
    return regmap_update_bits(data.regmap, MAX30102_REG_MODE_CONFIG,
    MAX30102_REG_MODE_CONFIG_PWR,
    en ? 0 : MAX30102_REG_MODE_CONFIG_PWR);
    }
#[no_mangle]
unsafe extern "C" fn max30102_set_powermode(data: *mut max30102_data, mode: u8, en: bool) -> c_int {
    static int max30102_set_powermode(struct max30102_data *data, u8 mode, bool en)
    {
    let mut reg: u8 = mode;
    if (!en)
    reg |= MAX30102_REG_MODE_CONFIG_PWR;
    return regmap_update_bits(data.regmap, MAX30102_REG_MODE_CONFIG,
    MAX30102_REG_MODE_CONFIG_PWR |
    MAX30102_REG_MODE_CONFIG_MODE_MASK, reg);
    }

    ((slot2 << MAX30102_REG_MODE_CONTROL_SLOT_SHIFT) | slot1)
#[no_mangle]
unsafe extern "C" fn max30102_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int max30102_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct max30102_data *data = iio_priv(indio_dev);
    int ret;
    u8 reg;
    switch (*indio_dev.active_scan_mask) {
    case BIT(MAX30102_LED_RED) | BIT(MAX30102_LED_IR):
    reg = MAX30102_REG_MODE_CONFIG_MODE_HR_SPO2;
    break;
    case BIT(MAX30102_LED_RED) | BIT(MAX30102_LED_IR) |
    BIT(MAX30105_LED_GREEN):
    ret = regmap_update_bits(data.regmap,
    MAX30102_REG_MODE_CONTROL_SLOT21,
    MAX30102_REG_MODE_CONTROL_SLOT_MASK,
    MAX30102_MODE_CONTROL_LED_SLOTS(2, 1));
    if (ret)
    return ret;
    ret = regmap_update_bits(data.regmap,
    MAX30102_REG_MODE_CONTROL_SLOT43,
    MAX30102_REG_MODE_CONTROL_SLOT_MASK,
    MAX30102_MODE_CONTROL_LED_SLOTS(0, 3));
    if (ret)
    return ret;
    reg = MAX30102_REG_MODE_CONFIG_MODE_MULTI;
    break;
    default:
    return -EINVAL;
    }
    return max30102_set_powermode(data, reg, true);
    }
#[no_mangle]
unsafe extern "C" fn max30102_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int max30102_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct max30102_data *data = iio_priv(indio_dev);
    return max30102_set_powermode(data, MAX30102_REG_MODE_CONFIG_MODE_NONE,
    false);
    }
    static const struct iio_buffer_setup_ops max30102_buffer_setup_ops = {
    .postenable = max30102_buffer_postenable,
    .predisable = max30102_buffer_predisable,
    };
#[no_mangle]
pub unsafe extern "C" fn max30102_fifo_count(data: *mut max30102_data) -> c_int {
    static inline int max30102_fifo_count(struct max30102_data *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(data.regmap, MAX30102_REG_INT_STATUS, &val);
    if (ret)
    return ret;
// FIFO has one sample slot left
    if (val & MAX30102_REG_INT_STATUS_FIFO_RDY)
    return 1;
    return 0;
    }

    memcpy(&data.processed_buffer[(i)], \
    &buffer[(i) * MAX30102_REG_FIFO_DATA_BYTES], \
    MAX30102_REG_FIFO_DATA_BYTES)
    static int max30102_read_measurement(struct max30102_data *data,
    unsigned int measurements)
    {
    int ret;
    u8 *buffer = (u8 *) &data.buffer;
    ret = i2c_smbus_read_i2c_block_data(data.client,
    MAX30102_REG_FIFO_DATA,
    measurements *
    MAX30102_REG_FIFO_DATA_BYTES,
    buffer);
    switch (measurements) {
    case 3:
    MAX30102_COPY_DATA(2);
    fallthrough;
    case 2:
    MAX30102_COPY_DATA(1);
    fallthrough;
    case 1:
    MAX30102_COPY_DATA(0);
    break;
    default:
    return -EINVAL;
    }
    return (ret == measurements * MAX30102_REG_FIFO_DATA_BYTES) ?
    0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn max30102_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t max30102_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct max30102_data *data = iio_priv(indio_dev);
    unsigned int measurements = bitmap_weight(indio_dev.active_scan_mask,
    iio_get_masklength(indio_dev));
    int ret, cnt = 0;
    mutex_lock(&data.lock);
    while (cnt || (cnt = max30102_fifo_count(data)) > 0) {
    ret = max30102_read_measurement(data, measurements);
    if (ret)
    break;
    iio_push_to_buffers(data.indio_dev, data.processed_buffer);
    cnt--;
    }
    mutex_unlock(&data.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max30102_get_current_idx(val: c_uint, reg: *mut c_int) -> c_int {
    static int max30102_get_current_idx(unsigned int val, int *reg)
    {
// each step is 0.200 mA
// reg = val / 200;
    return *reg > 0xff ? -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn max30102_led_init(data: *mut max30102_data) -> c_int {
    static int max30102_led_init(struct max30102_data *data)
    {
    struct device *dev = &data.client.dev;
    unsigned int val;
    int reg, ret;
    ret = device_property_read_u32(dev, "maxim,red-led-current-microamp", &val);
    if (ret) {
    dev_info(dev, "no red-led-current-microamp set\n");
// Default to 7 mA RED LED
    val = 7000;
    }
    ret = max30102_get_current_idx(val, &reg);
    if (ret) {
    dev_err(dev, "invalid RED LED current setting %d\n", val);
    return ret;
    }
    ret = regmap_write(data.regmap, MAX30102_REG_RED_LED_CONFIG, reg);
    if (ret)
    return ret;
    if (data.chip_id == max30105) {
    ret = device_property_read_u32(dev,
    "maxim,green-led-current-microamp", &val);
    if (ret) {
    dev_info(dev, "no green-led-current-microamp set\n");
// Default to 7 mA green LED
    val = 7000;
    }
    ret = max30102_get_current_idx(val, &reg);
    if (ret) {
    dev_err(dev, "invalid green LED current setting %d\n",
    val);
    return ret;
    }
    ret = regmap_write(data.regmap, MAX30105_REG_GREEN_LED_CONFIG,
    reg);
    if (ret)
    return ret;
    }
    ret = device_property_read_u32(dev, "maxim,ir-led-current-microamp", &val);
    if (ret) {
    dev_info(dev, "no ir-led-current-microamp set\n");
// Default to 7 mA IR LED
    val = 7000;
    }
    ret = max30102_get_current_idx(val, &reg);
    if (ret) {
    dev_err(dev, "invalid IR LED current setting %d\n", val);
    return ret;
    }
    return regmap_write(data.regmap, MAX30102_REG_IR_LED_CONFIG, reg);
    }
#[no_mangle]
unsafe extern "C" fn max30102_chip_init(data: *mut max30102_data) -> c_int {
    static int max30102_chip_init(struct max30102_data *data)
    {
    int ret;
// setup LED current settings
    ret = max30102_led_init(data);
    if (ret)
    return ret;
// configure 18-bit HR + SpO2 readings at 400Hz
    ret = regmap_write(data.regmap, MAX30102_REG_SPO2_CONFIG,
    (MAX30102_REG_SPO2_CONFIG_ADC_4096_STEPS
    << MAX30102_REG_SPO2_CONFIG_ADC_MASK_SHIFT) |
    (MAX30102_REG_SPO2_CONFIG_SR_400HZ
    << MAX30102_REG_SPO2_CONFIG_SR_MASK_SHIFT) |
    MAX30102_REG_SPO2_CONFIG_PULSE_411_US);
    if (ret)
    return ret;
// average 4 samples + generate FIFO interrupt
    ret = regmap_write(data.regmap, MAX30102_REG_FIFO_CONFIG,
    (MAX30102_REG_FIFO_CONFIG_AVG_4SAMPLES
    << MAX30102_REG_FIFO_CONFIG_AVG_SHIFT) |
    MAX30102_REG_FIFO_CONFIG_AFULL);
    if (ret)
    return ret;
// enable FIFO interrupt
    return regmap_update_bits(data.regmap, MAX30102_REG_INT_ENABLE,
    MAX30102_REG_INT_ENABLE_MASK,
    MAX30102_REG_INT_ENABLE_FIFO_EN);
    }
#[no_mangle]
unsafe extern "C" fn max30102_read_temp(data: *mut max30102_data, val: *mut c_int) -> c_int {
    static int max30102_read_temp(struct max30102_data *data, int *val)
    {
    int ret;
    unsigned int reg;
    ret = regmap_read(data.regmap, MAX30102_REG_TEMP_INTEGER, &reg);
    if (ret < 0)
    return ret;
// val = reg << 4;
    ret = regmap_read(data.regmap, MAX30102_REG_TEMP_FRACTION, &reg);
    if (ret < 0)
    return ret;
// val |= reg & 0xf;
// val = sign_extend32(*val, 11);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max30102_get_temp(data: *mut max30102_data, val: *mut c_int, en: bool) -> c_int {
    static int max30102_get_temp(struct max30102_data *data, int *val, bool en)
    {
    int ret;
    if (en) {
    ret = max30102_set_power(data, true);
    if (ret)
    return ret;
    }
// start acquisition
    ret = regmap_set_bits(data.regmap, MAX30102_REG_TEMP_CONFIG,
    MAX30102_REG_TEMP_CONFIG_TEMP_EN);
    if (ret)
    goto out;
    msleep(35);
    ret = max30102_read_temp(data, val);
    out:
    if (en)
    max30102_set_power(data, false);
    return ret;
    }
    static int max30102_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max30102_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
//
// Temperature reading can only be acquired when not in
// shutdown; leave shutdown briefly when buffer not running
//
    IIO_DEV_GUARD_CURRENT_MODE(indio_dev);
    ret = max30102_get_temp(data, val, !iio_buffer_enabled(indio_dev));
    if (ret)
    return ret;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
// val = 1000;  /* 62.5
// val2 = 16;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info max30102_info = {
    .read_raw = max30102_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn max30102_probe(client: *mut i2c_client) -> c_int {
    static int max30102_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct max30102_data *data;
    struct iio_dev *indio_dev;
    int ret;
    unsigned int reg;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    indio_dev.name = MAX30102_DRV_NAME;
    indio_dev.info = &max30102_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    data = iio_priv(indio_dev);
    data.indio_dev = indio_dev;
    data.client = client;
    data.chip_id = id.driver_data;
    mutex_init(&data.lock);
    i2c_set_clientdata(client, indio_dev);
    switch (data.chip_id) {
    case max30105:
    indio_dev.channels = max30105_channels;
    indio_dev.num_channels = ARRAY_SIZE(max30105_channels);
    indio_dev.available_scan_masks = max30105_scan_masks;
    break;
    case max30102:
    indio_dev.channels = max30102_channels;
    indio_dev.num_channels = ARRAY_SIZE(max30102_channels);
    indio_dev.available_scan_masks = max30102_scan_masks;
    break;
    default:
    return -ENODEV;
    }
    ret = devm_iio_kfifo_buffer_setup(&client.dev, indio_dev,
    &max30102_buffer_setup_ops);
    if (ret)
    return ret;
    data.regmap = devm_regmap_init_i2c(client, &max30102_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "regmap initialization failed\n");
    return PTR_ERR(data.regmap);
    }
// check part ID
    ret = regmap_read(data.regmap, MAX30102_REG_PART_ID, &reg);
    if (ret)
    return ret;
    if (reg != MAX30102_PART_NUMBER)
    return -ENODEV;
// show revision ID
    ret = regmap_read(data.regmap, MAX30102_REG_REV_ID, &reg);
    if (ret)
    return ret;
    dev_dbg(&client.dev, "max3010x revision %02x\n", reg);
// clear mode setting, chip shutdown
    ret = max30102_set_powermode(data, MAX30102_REG_MODE_CONFIG_MODE_NONE,
    false);
    if (ret)
    return ret;
    ret = max30102_chip_init(data);
    if (ret)
    return ret;
    if (client.irq <= 0) {
    dev_err(&client.dev, "no valid irq defined\n");
    return -EINVAL;
    }
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), max30102_interrupt_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "max30102_irq", indio_dev);
    if (ret)
    return ret;
    return iio_device_register(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn max30102_remove(client: *mut i2c_client) {
    static void max30102_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct max30102_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    max30102_set_power(data, false);
    }
    static const struct i2c_device_id max30102_id[] = {
    { .name = "max30101", .driver_data = max30105 },
    { .name = "max30102", .driver_data = max30102 },
    { .name = "max30105", .driver_data = max30105 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max30102_id);
    static const struct of_device_id max30102_dt_ids[] = {
    { .compatible = "maxim,max30101" },
    { .compatible = "maxim,max30102" },
    { .compatible = "maxim,max30105" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max30102_dt_ids);
    static struct i2c_driver max30102_driver = {
    .driver = {
    .name	= MAX30102_DRV_NAME,
    .of_match_table	= max30102_dt_ids,
    },
    .probe		= max30102_probe,
    .remove		= max30102_remove,
    .id_table	= max30102_id,
    };
    module_i2c_driver(max30102_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("MAX30102 heart rate/pulse oximeter and MAX30105 particle sensor driver");
    MODULE_LICENSE("GPL");
