//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/vl53l0x-i2c.c
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
// Support for ST VL53L0X FlightSense ToF Ranging Sensor on a i2c bus.
//
// Copyright (C) 2016 STMicroelectronics Imaging Division.
// Copyright (C) 2018 Song Qiang <songqiang1304521@gmail.com>
// Copyright (C) 2020 Ivan Drobyshevskyi <drobyshevskyi@gmail.com>
//
// Datasheet available at
// <https://www.st.com/resource/en/datasheet/vl53l0x.pdf>
//
// Default 7-bit i2c slave address 0x29.
//
// TODO: FIFO buffer, continuous mode, range selection, sensor ID check.
//

pub const VL_REG_SYSRANGE_START: c_uint = 0x00;

pub const VL_REG_SYSRANGE_MODE_SINGLESHOT: c_uint = 0x00;

pub const VL_REG_SYSTEM_INTERRUPT_CONFIG_GPIO: c_uint = 0x0A;

pub const VL_REG_SYSTEM_INTERRUPT_CLEAR: c_uint = 0x0B;
pub const VL_REG_RESULT_INT_STATUS: c_uint = 0x13;
pub const VL_REG_RESULT_RANGE_STATUS: c_uint = 0x14;
pub const VL_REG_IDENTIFICATION_MODEL_ID: c_uint = 0xC0;

pub const VL53L0X_MODEL_ID_VAL: c_uint = 0xEE;
pub const VL53L0X_CONTINUOUS_MODE: c_uint = 0x02;
pub const VL53L0X_SINGLE_MODE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl53l0x_data {
    pub client: *mut i2c_client,
    pub completion: completion,
    pub vdd_supply: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub trig: *mut iio_trigger,
}

#[no_mangle]
unsafe extern "C" fn vl53l0x_clear_irq(data: *mut vl53l0x_data) -> c_int {
    static int vl53l0x_clear_irq(struct vl53l0x_data *data)
    {	int ret;
    ret = i2c_smbus_write_byte_data(data.client,
    VL_REG_SYSTEM_INTERRUPT_CLEAR, 1);
    if (ret < 0) {
    dev_err(&data.client.dev, "failed to clear irq: %d\n", ret);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_trigger_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t vl53l0x_trigger_handler(int irq, void *priv)
    {
    struct iio_poll_func *pf = priv;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct vl53l0x_data *data = iio_priv(indio_dev);
    u8 buffer[12];
    int ret;
    struct {
    u16 chan;
    aligned_s64 timestamp;
    } scan = { };
    ret = i2c_smbus_read_i2c_block_data(data.client,
    VL_REG_RESULT_RANGE_STATUS,
    sizeof(buffer), buffer);
    if (ret != 12)
    goto done;
    scan.chan = get_unaligned_be16(&buffer[10]);
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    vl53l0x_clear_irq(data);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_threaded_irq(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t vl53l0x_threaded_irq(int irq, void *priv)
    {
    struct iio_dev *indio_dev = priv;
    struct vl53l0x_data *data = iio_priv(indio_dev);
    if (iio_buffer_enabled(indio_dev))
    iio_trigger_poll_nested(indio_dev.trig);
    else
    complete(&data.completion);
    return IRQ_HANDLED;
    }
    static int vl53l0x_configure_irq(struct i2c_client *client,
    struct iio_dev *indio_dev)
    {
    let mut irq_flags: c_int = irq_get_trigger_type(client.irq);
    struct vl53l0x_data *data = iio_priv(indio_dev);
    int ret;
    if (!irq_flags)
    irq_flags = IRQF_TRIGGER_FALLING;
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), vl53l0x_threaded_irq,
    irq_flags | IRQF_ONESHOT, indio_dev.name, indio_dev);
    if (ret)
    return ret;
    ret = i2c_smbus_write_byte_data(data.client,
    VL_REG_SYSTEM_INTERRUPT_CONFIG_GPIO,
    VL_REG_SYSTEM_INTERRUPT_GPIO_NEW_SAMPLE_READY);
    if (ret < 0)
    dev_err(&client.dev, "failed to configure IRQ: %d\n", ret);
    return ret;
    }
    static int vl53l0x_read_proximity(struct vl53l0x_data *data,
    const struct iio_chan_spec *chan,
    int *val)
    {
    struct i2c_client *client = data.client;
    let mut tries: u16 = 20;
    u8 buffer[12];
    int ret;
    unsigned long time_left;
    ret = i2c_smbus_write_byte_data(client, VL_REG_SYSRANGE_START, 1);
    if (ret < 0)
    return ret;
    if (data.client.irq) {
    reinit_completion(&data.completion);
    time_left = wait_for_completion_timeout(&data.completion, HZ/10);
    if (time_left == 0)
    return -ETIMEDOUT;
    ret = vl53l0x_clear_irq(data);
    if (ret < 0)
    return ret;
    } else {
    do {
    ret = i2c_smbus_read_byte_data(client,
    VL_REG_RESULT_RANGE_STATUS);
    if (ret < 0)
    return ret;
    if (ret & VL_REG_RESULT_RANGE_STATUS_COMPLETE)
    break;
    usleep_range(1000, 5000);
    } while (--tries);
    if (!tries)
    return -ETIMEDOUT;
    }
    ret = i2c_smbus_read_i2c_block_data(client, VL_REG_RESULT_RANGE_STATUS,
    12, buffer);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(12: ret !=) -> else {
    else if (ret != 12)
    return -EREMOTEIO;
// Values should be between 30~1200 in millimeters.
// val = get_unaligned_be16(&buffer[10]);
    return 0;
    }
    static const struct iio_chan_spec vl53l0x_channels[] = {
    {
    .type = IIO_DISTANCE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 12,
    .storagebits = 16,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static int vl53l0x_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct vl53l0x_data *data = iio_priv(indio_dev);
    int ret;
    if (chan.type != IIO_DISTANCE)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = vl53l0x_read_proximity(data, chan, val);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_validate_trigger(indio_dev: *mut iio_dev, trig: *mut iio_trigger) -> c_int {
    static int vl53l0x_validate_trigger(struct iio_dev *indio_dev, struct iio_trigger *trig)
    {
    struct vl53l0x_data *data = iio_priv(indio_dev);
    return data.trig == trig ? 0 : -EINVAL;
    }
    static const struct iio_info vl53l0x_info = {
    .read_raw = vl53l0x_read_raw,
    .validate_trigger = vl53l0x_validate_trigger,
    };
#[no_mangle]
unsafe extern "C" fn vl53l0x_power_off(_data: *mut c_void) {
    static void vl53l0x_power_off(void *_data)
    {
    struct vl53l0x_data *data = _data;
    gpiod_set_value_cansleep(data.reset_gpio, 1);
    regulator_disable(data.vdd_supply);
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_power_on(data: *mut vl53l0x_data) -> c_int {
    static int vl53l0x_power_on(struct vl53l0x_data *data)
    {
    int ret;
    ret = regulator_enable(data.vdd_supply);
    if (ret)
    return ret;
    gpiod_set_value_cansleep(data.reset_gpio, 0);
    usleep_range(3200, 5000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int vl53l0x_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct vl53l0x_data *data = iio_priv(indio_dev);
    return i2c_smbus_write_byte_data(data.client, VL_REG_SYSRANGE_START,
    VL53L0X_CONTINUOUS_MODE);
    }
#[no_mangle]
unsafe extern "C" fn vl53l0x_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int vl53l0x_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct vl53l0x_data *data = iio_priv(indio_dev);
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, VL_REG_SYSRANGE_START,
    VL53L0X_SINGLE_MODE);
    if (ret < 0)
    return ret;
// Let the ongoing reading finish
    reinit_completion(&data.completion);
    wait_for_completion_timeout(&data.completion, HZ / 10);
    return vl53l0x_clear_irq(data);
    }
    static const struct iio_buffer_setup_ops iio_triggered_buffer_setup_ops = {
    .postenable = &vl53l0x_buffer_postenable,
    .postdisable = &vl53l0x_buffer_postdisable,
    };
    static const struct iio_trigger_ops vl53l0x_trigger_ops = {
    .validate_device = iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn vl53l0x_probe(client: *mut i2c_client) -> c_int {
    static int vl53l0x_probe(struct i2c_client *client)
    {
    struct vl53l0x_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    i2c_set_clientdata(client, indio_dev);
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_I2C_BLOCK |
    I2C_FUNC_SMBUS_BYTE_DATA))
    return -EOPNOTSUPP;
    ret = i2c_smbus_read_byte_data(data.client, VL_REG_IDENTIFICATION_MODEL_ID);
    if (ret < 0)
    return -EINVAL;
    if (ret != VL53L0X_MODEL_ID_VAL)
    dev_info(&client.dev, "Unknown model id: 0x%x", ret);
    data.vdd_supply = devm_regulator_get(&client.dev, "vdd");
    if (IS_ERR(data.vdd_supply))
    return dev_err_probe(&client.dev, PTR_ERR(data.vdd_supply),
    "Unable to get VDD regulator\n");
    data.reset_gpio = devm_gpiod_get_optional(&client.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(data.reset_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(data.reset_gpio),
    "Cannot get reset GPIO\n");
    ret = vl53l0x_power_on(data);
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "Failed to power on the chip\n");
    ret = devm_add_action_or_reset(&client.dev, vl53l0x_power_off, data);
    if (ret)
    return ret;
    indio_dev.name = "vl53l0x";
    indio_dev.info = &vl53l0x_info;
    indio_dev.channels = vl53l0x_channels;
    indio_dev.num_channels = ARRAY_SIZE(vl53l0x_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
// usage of interrupt is optional
    if (client.irq) {
    init_completion(&data.completion);
    data.trig = devm_iio_trigger_alloc(&client.dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.trig)
    return -ENOMEM;
    data.trig.ops = &vl53l0x_trigger_ops;
    iio_trigger_set_drvdata(data.trig, indio_dev);
    ret = devm_iio_trigger_register(&client.dev, data.trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(data.trig);
    ret = vl53l0x_configure_irq(client, indio_dev);
    if (ret)
    return ret;
    ret = devm_iio_triggered_buffer_setup(&client.dev,
    indio_dev,
    core::ptr::null_mut(),
    &vl53l0x_trigger_handler,
    &iio_triggered_buffer_setup_ops);
    if (ret)
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id vl53l0x_id[] = {
    { .name = "vl53l0x" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, vl53l0x_id);
    static const struct of_device_id st_vl53l0x_dt_match[] = {
    { .compatible = "st,vl53l0x", },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_vl53l0x_dt_match);
    static struct i2c_driver vl53l0x_driver = {
    .driver = {
    .name = "vl53l0x-i2c",
    .of_match_table = st_vl53l0x_dt_match,
    },
    .probe = vl53l0x_probe,
    .id_table = vl53l0x_id,
    };
    module_i2c_driver(vl53l0x_driver);
    MODULE_AUTHOR("Song Qiang <songqiang1304521@gmail.com>");
    MODULE_DESCRIPTION("ST vl53l0x ToF ranging sensor driver");
    MODULE_LICENSE("GPL v2");
