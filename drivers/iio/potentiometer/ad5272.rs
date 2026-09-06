//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiometer/ad5272.c
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
// Analog Devices AD5272 digital potentiometer driver
// Copyright (C) 2018 Phil Reid <preid@electromag.com.au>
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/AD5272_5274.pdf
//
// DEVID	#Wipers	#Positions	Resistor Opts (kOhm)	i2c address
// ad5272	1	1024		20, 50, 100		01011xx
// ad5274	1	256		20, 100			01011xx
//

pub const AD5272_RDAC_WR: c_int = 1;
pub const AD5272_RDAC_RD: c_int = 2;
pub const AD5272_RESET: c_int = 4;
pub const AD5272_CTL: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5272_cfg {
    pub max_pos: c_int,
    pub kohms: c_int,
    pub shift: c_int,
}

    enum ad5272_type {
    AD5272_020,
    AD5272_050,
    AD5272_100,
    AD5274_020,
    AD5274_100,
    };
    static const struct ad5272_cfg ad5272_cfg[] = {
    [AD5272_020] = { .max_pos = 1024, .kohms = 20 },
    [AD5272_050] = { .max_pos = 1024, .kohms = 50 },
    [AD5272_100] = { .max_pos = 1024, .kohms = 100 },
    [AD5274_020] = { .max_pos = 256,  .kohms = 20,  .shift = 2 },
    [AD5274_100] = { .max_pos = 256,  .kohms = 100, .shift = 2 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5272_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub cfg: *const ad5272_cfg,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[2],
}

    static const struct iio_chan_spec ad5272_channel = {
    .type = IIO_RESISTANCE,
    .output = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),
    };
#[no_mangle]
unsafe extern "C" fn ad5272_write(data: *mut ad5272_data, reg: c_int, val: c_int) -> c_int {
    static int ad5272_write(struct ad5272_data *data, int reg, int val)
    {
    int ret;
    data.buf[0] = (reg << 2) | ((val >> 8) & 0x3);
    data.buf[1] = (u8)val;
    mutex_lock(&data.lock);
    ret = i2c_master_send(data.client, data.buf, sizeof(data.buf));
    mutex_unlock(&data.lock);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5272_read(data: *mut ad5272_data, reg: c_int, val: *mut c_int) -> c_int {
    static int ad5272_read(struct ad5272_data *data, int reg, int *val)
    {
    int ret;
    data.buf[0] = reg << 2;
    data.buf[1] = 0;
    mutex_lock(&data.lock);
    ret = i2c_master_send(data.client, data.buf, sizeof(data.buf));
    if (ret < 0)
    goto error;
    ret = i2c_master_recv(data.client, data.buf, sizeof(data.buf));
    if (ret < 0)
    goto error;
// val = ((data->buf[0] & 0x3) << 8) | data->buf[1];
    ret = 0;
    error:
    mutex_unlock(&data.lock);
    return ret;
    }
    static int ad5272_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ad5272_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    ret = ad5272_read(data, AD5272_RDAC_RD, val);
// val = *val >> data->cfg->shift;
    return ret ? ret : IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
// val = 1000 * data->cfg->kohms;
// val2 = data->cfg->max_pos;
    return IIO_VAL_FRACTIONAL;
    }
    return -EINVAL;
    }
    static int ad5272_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct ad5272_data *data = iio_priv(indio_dev);
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    if (val >= data.cfg.max_pos || val < 0 || val2)
    return -EINVAL;
    return ad5272_write(data, AD5272_RDAC_WR, val << data.cfg.shift);
    }
    static const struct iio_info ad5272_info = {
    .read_raw = ad5272_read_raw,
    .write_raw = ad5272_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn ad5272_reset(data: *mut ad5272_data) -> c_int {
    static int ad5272_reset(struct ad5272_data *data)
    {
    struct gpio_desc *reset_gpio;
    reset_gpio = devm_gpiod_get_optional(&data.client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(reset_gpio))
    return PTR_ERR(reset_gpio);
    if (reset_gpio) {
    udelay(1);
    gpiod_set_value(reset_gpio, 0);
    } else {
    ad5272_write(data, AD5272_RESET, 0);
    }
    usleep_range(1000, 2000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5272_probe(client: *mut i2c_client) -> c_int {
    static int ad5272_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct ad5272_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    i2c_set_clientdata(client, indio_dev);
    data = iio_priv(indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    data.cfg = &ad5272_cfg[id.driver_data];
    ret = ad5272_reset(data);
    if (ret)
    return ret;
    ret = ad5272_write(data, AD5272_CTL, AD5272_RDAC_WR_EN);
    if (ret < 0)
    return -ENODEV;
    indio_dev.info = &ad5272_info;
    indio_dev.channels = &ad5272_channel;
    indio_dev.num_channels = 1;
    indio_dev.name = client.name;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id ad5272_dt_ids[] = {
    { .compatible = "adi,ad5272-020", .data = (void *)AD5272_020 },
    { .compatible = "adi,ad5272-050", .data = (void *)AD5272_050 },
    { .compatible = "adi,ad5272-100", .data = (void *)AD5272_100 },
    { .compatible = "adi,ad5274-020", .data = (void *)AD5274_020 },
    { .compatible = "adi,ad5274-100", .data = (void *)AD5274_100 },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5272_dt_ids);
    static const struct i2c_device_id ad5272_id[] = {
    { .name = "ad5272-020", .driver_data = AD5272_020 },
    { .name = "ad5272-050", .driver_data = AD5272_050 },
    { .name = "ad5272-100", .driver_data = AD5272_100 },
    { .name = "ad5274-020", .driver_data = AD5274_020 },
    { .name = "ad5274-100", .driver_data = AD5274_100 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5272_id);
    static struct i2c_driver ad5272_driver = {
    .driver = {
    .name	= "ad5272",
    .of_match_table = ad5272_dt_ids,
    },
    .probe		= ad5272_probe,
    .id_table	= ad5272_id,
    };
    module_i2c_driver(ad5272_driver);
    MODULE_AUTHOR("Phil Reid <preid@eletromag.com.au>");
    MODULE_DESCRIPTION("AD5272 digital potentiometer");
    MODULE_LICENSE("GPL v2");
