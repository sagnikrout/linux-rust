//! Automatically rewritten from C to Rust
//! Source: drivers/iio/temperature/tmp007.c
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
// tmp007.c - Support for TI TMP007 IR thermopile sensor with integrated math engine
//
// Copyright (c) 2017 Manivannan Sadhasivam <manivannanece23@gmail.com>
//
// Driver for the Texas Instruments I2C 16-bit IR thermopile sensor
//
// (7-bit I2C slave address (0x40 - 0x47), changeable via ADR pins)
//
// Note:
// 1. This driver assumes that the sensor has been calibrated beforehand
// 2. Limit threshold events are enabled at the start
// 3. Operating mode: INT
//

pub const TMP007_TDIE: c_uint = 0x01;
pub const TMP007_CONFIG: c_uint = 0x02;
pub const TMP007_TOBJECT: c_uint = 0x03;
pub const TMP007_STATUS: c_uint = 0x04;
pub const TMP007_STATUS_MASK: c_uint = 0x05;
pub const TMP007_TOBJ_HIGH_LIMIT: c_uint = 0x06;
pub const TMP007_TOBJ_LOW_LIMIT: c_uint = 0x07;
pub const TMP007_TDIE_HIGH_LIMIT: c_uint = 0x08;
pub const TMP007_TDIE_LOW_LIMIT: c_uint = 0x09;
pub const TMP007_MANUFACTURER_ID: c_uint = 0x1e;
pub const TMP007_DEVICE_ID: c_uint = 0x1f;

pub const TMP007_CONFIG_CR_SHIFT: c_int = 9;
// Status register flags

pub const TMP007_MANUFACTURER_MAGIC: c_uint = 0x5449;
pub const TMP007_DEVICE_MAGIC: c_uint = 0x0078;
pub const TMP007_TEMP_SHIFT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp007_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub config: u16,
    pub status_mask: u16,
}

    static const int tmp007_avgs[5][2] = { {4, 0}, {2, 0}, {1, 0},
    {0, 500000}, {0, 250000} };
#[no_mangle]
unsafe extern "C" fn tmp007_read_temperature(data: *mut tmp007_data, reg: u8) -> c_int {
    static int tmp007_read_temperature(struct tmp007_data *data, u8 reg)
    {
    s32 ret;
    let mut tries: c_int = 50;
    while (tries-- > 0) {
    ret = i2c_smbus_read_word_swapped(data.client,
    TMP007_STATUS);
    if (ret < 0)
    return ret;
    if ((ret & TMP007_STATUS_CONV_READY) &&
    !(ret & TMP007_STATUS_DATA_VALID))
    break;
    msleep(100);
    }
    if (tries < 0)
    return -EIO;
    return i2c_smbus_read_word_swapped(data.client, reg);
    }
#[no_mangle]
unsafe extern "C" fn tmp007_powerdown(data: *mut tmp007_data) -> c_int {
    static int tmp007_powerdown(struct tmp007_data *data)
    {
    return i2c_smbus_write_word_swapped(data.client, TMP007_CONFIG,
    data.config & ~TMP007_CONFIG_CONV_EN);
    }
    static int tmp007_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int *val,
    int *val2, long mask)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    s32 ret;
    int conv_rate;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    switch (channel.channel2) {
    case IIO_MOD_TEMP_AMBIENT: /* LSB: 0.03125 degree Celsius */
    ret = i2c_smbus_read_word_swapped(data.client, TMP007_TDIE);
    if (ret < 0)
    return ret;
    break;
    case IIO_MOD_TEMP_OBJECT:
    ret = tmp007_read_temperature(data, TMP007_TOBJECT);
    if (ret < 0)
    return ret;
    break;
    default:
    return -EINVAL;
    }
// val = sign_extend32(ret, 15) >> TMP007_TEMP_SHIFT;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 31;
// val2 = 250000;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    conv_rate = (data.config & TMP007_CONFIG_CR_MASK)
    >> TMP007_CONFIG_CR_SHIFT;
// val = tmp007_avgs[conv_rate][0];
// val2 = tmp007_avgs[conv_rate][1];
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int tmp007_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int val,
    int val2, long mask)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    int i;
    u16 tmp;
    if (mask == IIO_CHAN_INFO_SAMP_FREQ) {
    for (i = 0; i < ARRAY_SIZE(tmp007_avgs); i++) {
    if ((val == tmp007_avgs[i][0]) &&
    (val2 == tmp007_avgs[i][1])) {
    tmp = data.config & ~TMP007_CONFIG_CR_MASK;
    tmp |= (i << TMP007_CONFIG_CR_SHIFT);
    return i2c_smbus_write_word_swapped(data.client,
    TMP007_CONFIG,
    data.config = tmp);
    }
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tmp007_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t tmp007_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct tmp007_data *data = iio_priv(indio_dev);
    int ret;
    ret = i2c_smbus_read_word_swapped(data.client, TMP007_STATUS);
    if ((ret < 0) || !(ret & (TMP007_STATUS_OHF | TMP007_STATUS_OLF |
    TMP007_STATUS_LHF | TMP007_STATUS_LLF)))
    return IRQ_NONE;
    if (ret & TMP007_STATUS_OHF)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_MOD_TEMP_OBJECT,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    iio_get_time_ns(indio_dev));
    if (ret & TMP007_STATUS_OLF)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_MOD_TEMP_OBJECT,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    iio_get_time_ns(indio_dev));
    if (ret & TMP007_STATUS_LHF)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_MOD_TEMP_AMBIENT,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    iio_get_time_ns(indio_dev));
    if (ret & TMP007_STATUS_LLF)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_MOD_TEMP_AMBIENT,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    iio_get_time_ns(indio_dev));
    return IRQ_HANDLED;
    }
    static int tmp007_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, bool state)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    unsigned int status_mask;
    int ret;
    switch (chan.channel2) {
    case IIO_MOD_TEMP_AMBIENT:
    if (dir == IIO_EV_DIR_RISING)
    status_mask = TMP007_STATUS_LHF;
    else
    status_mask = TMP007_STATUS_LLF;
    break;
    case IIO_MOD_TEMP_OBJECT:
    if (dir == IIO_EV_DIR_RISING)
    status_mask = TMP007_STATUS_OHF;
    else
    status_mask = TMP007_STATUS_OLF;
    break;
    default:
    return -EINVAL;
    }
    mutex_lock(&data.lock);
    ret = i2c_smbus_read_word_swapped(data.client, TMP007_STATUS_MASK);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
    if (state)
    ret |= status_mask;
    else
    ret &= ~status_mask;
    return i2c_smbus_write_word_swapped(data.client, TMP007_STATUS_MASK,
    data.status_mask = ret);
    }
    static int tmp007_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    unsigned int mask;
    switch (chan.channel2) {
    case IIO_MOD_TEMP_AMBIENT:
    if (dir == IIO_EV_DIR_RISING)
    mask = TMP007_STATUS_LHF;
    else
    mask = TMP007_STATUS_LLF;
    break;
    case IIO_MOD_TEMP_OBJECT:
    if (dir == IIO_EV_DIR_RISING)
    mask = TMP007_STATUS_OHF;
    else
    mask = TMP007_STATUS_OLF;
    break;
    default:
    return -EINVAL;
    }
    return !!(data.status_mask & mask);
    }
    static int tmp007_read_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, enum iio_event_info info,
    int *val, int *val2)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    int ret;
    u8 reg;
    switch (chan.channel2) {
    case IIO_MOD_TEMP_AMBIENT: /* LSB: 0.5 degree Celsius */
    if (dir == IIO_EV_DIR_RISING)
    reg = TMP007_TDIE_HIGH_LIMIT;
    else
    reg = TMP007_TDIE_LOW_LIMIT;
    break;
    case IIO_MOD_TEMP_OBJECT:
    if (dir == IIO_EV_DIR_RISING)
    reg = TMP007_TOBJ_HIGH_LIMIT;
    else
    reg = TMP007_TOBJ_LOW_LIMIT;
    break;
    default:
    return -EINVAL;
    }
    ret = i2c_smbus_read_word_swapped(data.client, reg);
    if (ret < 0)
    return ret;
// Shift length 7 bits = 6(15:6) + 1(0.5 LSB)
// val = sign_extend32(ret, 15) >> 7;
    return IIO_VAL_INT;
    }
    static int tmp007_write_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, enum iio_event_info info,
    int val, int val2)
    {
    struct tmp007_data *data = iio_priv(indio_dev);
    u8 reg;
    switch (chan.channel2) {
    case IIO_MOD_TEMP_AMBIENT:
    if (dir == IIO_EV_DIR_RISING)
    reg = TMP007_TDIE_HIGH_LIMIT;
    else
    reg = TMP007_TDIE_LOW_LIMIT;
    break;
    case IIO_MOD_TEMP_OBJECT:
    if (dir == IIO_EV_DIR_RISING)
    reg = TMP007_TOBJ_HIGH_LIMIT;
    else
    reg = TMP007_TOBJ_LOW_LIMIT;
    break;
    default:
    return -EINVAL;
    }
// Full scale threshold value is +/- 256 degree Celsius
    if (val < -256 || val > 255)
    return -EINVAL;
// Shift length 7 bits = 6(15:6) + 1(0.5 LSB)
    return i2c_smbus_write_word_swapped(data.client, reg, (val << 7));
    }
    static IIO_CONST_ATTR(sampling_frequency_available, "4 2 1 0.5 0.25");
    static struct attribute *tmp007_attributes[] = {
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tmp007_attribute_group = {
    .attrs = tmp007_attributes,
    };
    static const struct iio_event_spec tmp007_obj_event[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_event_spec tmp007_die_event[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_chan_spec tmp007_channels[] = {
    {
    .type = IIO_TEMP,
    .modified = 1,
    .channel2 = IIO_MOD_TEMP_AMBIENT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .event_spec = tmp007_die_event,
    .num_event_specs = ARRAY_SIZE(tmp007_die_event),
    },
    {
    .type = IIO_TEMP,
    .modified = 1,
    .channel2 = IIO_MOD_TEMP_OBJECT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .event_spec = tmp007_obj_event,
    .num_event_specs = ARRAY_SIZE(tmp007_obj_event),
    }
    };
    static const struct iio_info tmp007_info = {
    .read_raw = tmp007_read_raw,
    .write_raw = tmp007_write_raw,
    .read_event_config = tmp007_read_event_config,
    .write_event_config = tmp007_write_event_config,
    .read_event_value = tmp007_read_thresh,
    .write_event_value = tmp007_write_thresh,
    .attrs = &tmp007_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn tmp007_identify(client: *mut i2c_client) -> bool {
    static bool tmp007_identify(struct i2c_client *client)
    {
    int manf_id, dev_id;
    manf_id = i2c_smbus_read_word_swapped(client, TMP007_MANUFACTURER_ID);
    if (manf_id < 0)
    return false;
    dev_id = i2c_smbus_read_word_swapped(client, TMP007_DEVICE_ID);
    if (dev_id < 0)
    return false;
    return (manf_id == TMP007_MANUFACTURER_MAGIC && dev_id == TMP007_DEVICE_MAGIC);
    }
#[no_mangle]
unsafe extern "C" fn tmp007_powerdown_action_cb(priv: *mut c_void) {
    static void tmp007_powerdown_action_cb(void *priv)
    {
    struct tmp007_data *data = priv;
    tmp007_powerdown(data);
    }
#[no_mangle]
unsafe extern "C" fn tmp007_probe(client: *mut i2c_client) -> c_int {
    static int tmp007_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *tmp007_id = i2c_client_get_device_id(client);
    struct tmp007_data *data;
    struct iio_dev *indio_dev;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EOPNOTSUPP;
    if (!tmp007_identify(client)) {
    dev_err(&client.dev, "TMP007 not found\n");
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    indio_dev.name = "tmp007";
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &tmp007_info;
    indio_dev.channels = tmp007_channels;
    indio_dev.num_channels = ARRAY_SIZE(tmp007_channels);
//
// Set Configuration register:
// 1. Conversion ON
// 2. ALERT enable
// 3. Transient correction enable
//
    ret = i2c_smbus_read_word_swapped(data.client, TMP007_CONFIG);
    if (ret < 0)
    return ret;
    data.config = ret;
    data.config |= (TMP007_CONFIG_CONV_EN | TMP007_CONFIG_ALERT_EN | TMP007_CONFIG_TC_EN);
    ret = i2c_smbus_write_word_swapped(data.client, TMP007_CONFIG,
    data.config);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&client.dev, tmp007_powerdown_action_cb, data);
    if (ret)
    return ret;
//
// Only the following flags can activate ALERT pin. Data conversion/validity flags
// flags can still be polled for getting temperature data
//
// Set Status Mask register:
// 1. Object temperature high limit enable
// 2. Object temperature low limit enable
// 3. TDIE temperature high limit enable
// 4. TDIE temperature low limit enable
//
    ret = i2c_smbus_read_word_swapped(data.client, TMP007_STATUS_MASK);
    if (ret < 0)
    return ret;
    data.status_mask = ret;
    data.status_mask |= (TMP007_STATUS_OHF | TMP007_STATUS_OLF
    | TMP007_STATUS_LHF | TMP007_STATUS_LLF);
    ret = i2c_smbus_write_word_swapped(data.client, TMP007_STATUS_MASK, data.status_mask);
    if (ret < 0)
    return ret;
    if (client.irq) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), tmp007_interrupt_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    tmp007_id.name, indio_dev);
    if (ret)
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn tmp007_suspend(dev: *mut device) -> c_int {
    static int tmp007_suspend(struct device *dev)
    {
    struct tmp007_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return tmp007_powerdown(data);
    }
#[no_mangle]
unsafe extern "C" fn tmp007_resume(dev: *mut device) -> c_int {
    static int tmp007_resume(struct device *dev)
    {
    struct tmp007_data *data = iio_priv(i2c_get_clientdata(
    to_i2c_client(dev)));
    return i2c_smbus_write_word_swapped(data.client, TMP007_CONFIG,
    data.config | TMP007_CONFIG_CONV_EN);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tmp007_pm_ops, tmp007_suspend, tmp007_resume);
    static const struct of_device_id tmp007_of_match[] = {
    { .compatible = "ti,tmp007", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tmp007_of_match);
    static const struct i2c_device_id tmp007_id[] = {
    { .name = "tmp007" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tmp007_id);
    static struct i2c_driver tmp007_driver = {
    .driver = {
    .name	= "tmp007",
    .of_match_table = tmp007_of_match,
    .pm	= pm_sleep_ptr(&tmp007_pm_ops),
    },
    .probe		= tmp007_probe,
    .id_table	= tmp007_id,
    };
    module_i2c_driver(tmp007_driver);
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannanece23@gmail.com>");
    MODULE_DESCRIPTION("TI TMP007 IR thermopile sensor driver");
    MODULE_LICENSE("GPL");
