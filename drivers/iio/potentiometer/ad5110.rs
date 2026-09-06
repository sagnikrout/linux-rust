//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiometer/ad5110.c
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
// Analog Devices AD5110 digital potentiometer driver
//
// Copyright (C) 2021 Mugilraj Dhavachelvan <dmugil2000@gmail.com>
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/AD5110_5112_5114.pdf
//

// AD5110 commands
pub const AD5110_EEPROM_WR: c_int = 1;
pub const AD5110_RDAC_WR: c_int = 2;
pub const AD5110_SHUTDOWN: c_int = 3;
pub const AD5110_RESET: c_int = 4;
pub const AD5110_RDAC_RD: c_int = 5;
pub const AD5110_EEPROM_RD: c_int = 6;
// AD5110_EEPROM_RD data
pub const AD5110_WIPER_POS: c_int = 0;
pub const AD5110_RESISTOR_TOL: c_int = 1;
pub const AD5110_WIPER_RESISTANCE: c_int = 70;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5110_cfg {
    pub max_pos: c_int,
    pub kohms: c_int,
    pub shift: c_int,
}

    enum ad5110_type {
    AD5110_10,
    AD5110_80,
    AD5112_05,
    AD5112_10,
    AD5112_80,
    AD5114_10,
    AD5114_80,
    };
    static const struct ad5110_cfg ad5110_cfg[] = {
    [AD5110_10] = { .max_pos = 128, .kohms = 10 },
    [AD5110_80] = { .max_pos = 128, .kohms = 80 },
    [AD5112_05] = { .max_pos = 64, .kohms = 5, .shift = 1 },
    [AD5112_10] = { .max_pos = 64, .kohms = 10, .shift = 1 },
    [AD5112_80] = { .max_pos = 64, .kohms = 80, .shift = 1 },
    [AD5114_10] = { .max_pos = 32, .kohms = 10, .shift = 2 },
    [AD5114_80] = { .max_pos = 32, .kohms = 80, .shift = 2 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5110_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut s16 tol; / resistor tolerance,
    pub enable: bool,
    pub lock: mutex,
    pub cfg: *const ad5110_cfg,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[2],
}

    static const struct iio_chan_spec ad5110_channels[] = {
    {
    .type = IIO_RESISTANCE,
    .output = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_OFFSET) |
    BIT(IIO_CHAN_INFO_SCALE) | BIT(IIO_CHAN_INFO_ENABLE),
    },
    };
#[no_mangle]
unsafe extern "C" fn ad5110_read(data: *mut ad5110_data, cmd: u8, val: *mut c_int) -> c_int {
    static int ad5110_read(struct ad5110_data *data, u8 cmd, int *val)
    {
    int ret;
    mutex_lock(&data.lock);
    data.buf[0] = cmd;
    data.buf[1] = *val;
    ret = i2c_master_send_dmasafe(data.client, data.buf, sizeof(data.buf));
    if (ret < 0) {
    goto error;
    } else if (ret != sizeof(data.buf)) {
    ret = -EIO;
    goto error;
    }
    ret = i2c_master_recv_dmasafe(data.client, data.buf, 1);
    if (ret < 0) {
    goto error;
    } else if (ret != 1) {
    ret = -EIO;
    goto error;
    }
// val = data->buf[0];
    ret = 0;
    error:
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5110_write(data: *mut ad5110_data, cmd: u8, val: u8) -> c_int {
    static int ad5110_write(struct ad5110_data *data, u8 cmd, u8 val)
    {
    int ret;
    mutex_lock(&data.lock);
    data.buf[0] = cmd;
    data.buf[1] = val;
    ret = i2c_master_send_dmasafe(data.client, data.buf, sizeof(data.buf));
    if (ret < 0) {
    goto error;
    } else if (ret != sizeof(data.buf)) {
    ret = -EIO;
    goto error;
    }
    ret = 0;
    error:
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5110_resistor_tol(data: *mut ad5110_data, cmd: u8, val: c_int) -> c_int {
    static int ad5110_resistor_tol(struct ad5110_data *data, u8 cmd, int val)
    {
    int ret;
    ret = ad5110_read(data, cmd, &val);
    if (ret)
    return ret;
    data.tol = data.cfg.kohms * (val & GENMASK(6, 0)) * 10 / 8;
    if (!(val & BIT(7)))
    data.tol *= -1;
    return 0;
    }
    static ssize_t store_eeprom_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct ad5110_data *data = iio_priv(indio_dev);
    let mut val: c_int = AD5110_WIPER_POS;
    int ret;
    ret = ad5110_read(data, AD5110_EEPROM_RD, &val);
    if (ret)
    return ret;
    val = val >> data.cfg.shift;
    return iio_format_value(buf, IIO_VAL_INT, 1, &val);
    }
    static ssize_t store_eeprom_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct ad5110_data *data = iio_priv(indio_dev);
    int ret;
    ret = ad5110_write(data, AD5110_EEPROM_WR, 0);
    if (ret) {
    dev_err(&data.client.dev, "RDAC to EEPROM write failed\n");
    return ret;
    }
// The storing of EEPROM data takes approximately 18 ms.
    msleep(20);
    return len;
    }
    static IIO_DEVICE_ATTR_RW(store_eeprom, 0);
    static struct attribute *ad5110_attributes[] = {
    &iio_dev_attr_store_eeprom.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ad5110_attribute_group = {
    .attrs = ad5110_attributes,
    };
    static int ad5110_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ad5110_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = ad5110_read(data, AD5110_RDAC_RD, val);
    if (ret)
    return ret;
// val = *val >> data->cfg->shift;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OFFSET:
// val = AD5110_WIPER_RESISTANCE * data->cfg->max_pos;
// val2 = 1000 * data->cfg->kohms + data->tol;
    return IIO_VAL_FRACTIONAL;
    case IIO_CHAN_INFO_SCALE:
// val = 1000 * data->cfg->kohms + data->tol;
// val2 = data->cfg->max_pos;
    return IIO_VAL_FRACTIONAL;
    case IIO_CHAN_INFO_ENABLE:
// val = data->enable;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int ad5110_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct ad5110_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (val > data.cfg.max_pos || val < 0)
    return -EINVAL;
    return ad5110_write(data, AD5110_RDAC_WR, val << data.cfg.shift);
    case IIO_CHAN_INFO_ENABLE:
    if (val < 0 || val > 1)
    return -EINVAL;
    if (data.enable == val)
    return 0;
    ret = ad5110_write(data, AD5110_SHUTDOWN, val ? 0 : 1);
    if (ret)
    return ret;
    data.enable = val;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info ad5110_info = {
    .read_raw = ad5110_read_raw,
    .write_raw = ad5110_write_raw,
    .attrs = &ad5110_attribute_group,
    };

    .compatible = of_compatible,	\
    .data = &ad5110_cfg[cfg],	\
    }
    static const struct of_device_id ad5110_of_match[] = {
    AD5110_COMPATIBLE("adi,ad5110-10", AD5110_10),
    AD5110_COMPATIBLE("adi,ad5110-80", AD5110_80),
    AD5110_COMPATIBLE("adi,ad5112-05", AD5112_05),
    AD5110_COMPATIBLE("adi,ad5112-10", AD5112_10),
    AD5110_COMPATIBLE("adi,ad5112-80", AD5112_80),
    AD5110_COMPATIBLE("adi,ad5114-10", AD5114_10),
    AD5110_COMPATIBLE("adi,ad5114-80", AD5114_80),
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5110_of_match);

    .name = _name,						\
    .driver_data = (kernel_ulong_t)&ad5110_cfg[cfg],	\
    }
    static const struct i2c_device_id ad5110_id[] = {
    AD5110_ID_TABLE("ad5110-10", AD5110_10),
    AD5110_ID_TABLE("ad5110-80", AD5110_80),
    AD5110_ID_TABLE("ad5112-05", AD5112_05),
    AD5110_ID_TABLE("ad5112-10", AD5112_10),
    AD5110_ID_TABLE("ad5112-80", AD5112_80),
    AD5110_ID_TABLE("ad5114-10", AD5114_10),
    AD5110_ID_TABLE("ad5114-80", AD5114_80),
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5110_id);
#[no_mangle]
unsafe extern "C" fn ad5110_probe(client: *mut i2c_client) -> c_int {
    static int ad5110_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct ad5110_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    data.enable = 1;
    data.cfg = i2c_get_match_data(client);
// refresh RDAC register with EEPROM
    ret = ad5110_write(data, AD5110_RESET, 0);
    if (ret) {
    dev_err(dev, "Refresh RDAC with EEPROM failed\n");
    return ret;
    }
    ret = ad5110_resistor_tol(data, AD5110_EEPROM_RD, AD5110_RESISTOR_TOL);
    if (ret) {
    dev_err(dev, "Read resistor tolerance failed\n");
    return ret;
    }
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &ad5110_info;
    indio_dev.channels = ad5110_channels;
    indio_dev.num_channels = ARRAY_SIZE(ad5110_channels);
    indio_dev.name = client.name;
    return devm_iio_device_register(dev, indio_dev);
    }
    static struct i2c_driver ad5110_driver = {
    .driver = {
    .name	= "ad5110",
    .of_match_table = ad5110_of_match,
    },
    .probe		= ad5110_probe,
    .id_table	= ad5110_id,
    };
    module_i2c_driver(ad5110_driver);
    MODULE_AUTHOR("Mugilraj Dhavachelvan <dmugil2000@gmail.com>");
    MODULE_DESCRIPTION("AD5110 digital potentiometer");
    MODULE_LICENSE("GPL v2");
