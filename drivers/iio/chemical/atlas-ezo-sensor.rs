//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/atlas-ezo-sensor.c
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
// atlas-ezo-sensor.c - Support for Atlas Scientific EZO sensors
//
// Copyright (C) 2020 Konsulko Group
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const ATLAS_INT_TIME_IN_MS: c_int = 950;
pub const ATLAS_INT_HUM_TIME_IN_MS: c_int = 350;
    enum {
    ATLAS_CO2_EZO,
    ATLAS_O2_EZO,
    ATLAS_HUM_EZO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlas_ezo_device {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
    pub delay: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlas_ezo_data {
    pub client: *mut i2c_client,
    pub chip: *const atlas_ezo_device,
// lock to avoid multiple concurrent read calls
    pub lock: mutex,
    pub buffer: [u8; 8],
}

    { \
    .type = IIO_CONCENTRATION, \
    .modified = 1,\
    .channel2 = _modifier, \
    .info_mask_separate = \
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE), \
    .scan_index = 0, \
    .scan_type =  { \
    .sign = 'u', \
    .realbits = 32, \
    .storagebits = 32, \
    .endianness = IIO_CPU, \
    }, \
    }
    static const struct iio_chan_spec atlas_co2_ezo_channels[] = {
    ATLAS_CONCENTRATION_CHANNEL(IIO_MOD_CO2),
    };
    static const struct iio_chan_spec atlas_o2_ezo_channels[] = {
    ATLAS_CONCENTRATION_CHANNEL(IIO_MOD_O2),
    };
    static const struct iio_chan_spec atlas_hum_ezo_channels[] = {
    {
    .type = IIO_HUMIDITYRELATIVE,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type =  {
    .sign = 'u',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_CPU,
    },
    },
    };
    static const struct atlas_ezo_device atlas_ezo_devices[] = {
    [ATLAS_CO2_EZO] = {
    .channels = atlas_co2_ezo_channels,
    .num_channels = 1,
    .delay = ATLAS_INT_TIME_IN_MS,
    },
    [ATLAS_O2_EZO] = {
    .channels = atlas_o2_ezo_channels,
    .num_channels = 1,
    .delay = ATLAS_INT_TIME_IN_MS,
    },
    [ATLAS_HUM_EZO] = {
    .channels = atlas_hum_ezo_channels,
    .num_channels = 1,
    .delay = ATLAS_INT_HUM_TIME_IN_MS,
    },
    };
#[no_mangle]
unsafe extern "C" fn atlas_ezo_sanitize(buf: *mut c_char) {
    static void atlas_ezo_sanitize(char *buf)
    {
    char *ptr = strchr(buf, '.');
    if (!ptr)
    return;
    memmove(ptr, ptr + 1, strlen(ptr));
    }
    static int atlas_ezo_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct atlas_ezo_data *data = iio_priv(indio_dev);
    struct i2c_client *client = data.client;
    if (chan.type != IIO_CONCENTRATION)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    int ret;
    long tmp;
    mutex_lock(&data.lock);
    tmp = i2c_smbus_write_byte(client, 'R');
    if (tmp < 0) {
    mutex_unlock(&data.lock);
    return tmp;
    }
    msleep(data.chip.delay);
    tmp = i2c_master_recv(client, data.buffer, sizeof(data.buffer));
    if (tmp < 0 || data.buffer[0] != 1) {
    mutex_unlock(&data.lock);
    return -EBUSY;
    }
// removing floating point for fixed number representation
    atlas_ezo_sanitize(data.buffer + 2);
    ret = kstrtol(data.buffer + 1, 10, &tmp);
// val = tmp;
    mutex_unlock(&data.lock);
    return ret ? ret : IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_HUMIDITYRELATIVE:
// val = 10;
    return IIO_VAL_INT;
    case IIO_CONCENTRATION:
    break;
    default:
    return -EINVAL;
    }
// IIO_CONCENTRATION modifiers
    switch (chan.channel2) {
    case IIO_MOD_CO2:
// val = 0;
// val2 = 100; /* 0.0001
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_MOD_O2:
// val = 100;
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    return 0;
    }
    static const struct iio_info atlas_info = {
    .read_raw = atlas_ezo_read_raw,
    };
    static const struct i2c_device_id atlas_ezo_id[] = {
    { .name = "atlas-co2-ezo", .driver_data = (kernel_ulong_t)&atlas_ezo_devices[ATLAS_CO2_EZO] },
    { .name = "atlas-o2-ezo", .driver_data = (kernel_ulong_t)&atlas_ezo_devices[ATLAS_O2_EZO] },
    { .name = "atlas-hum-ezo", .driver_data = (kernel_ulong_t)&atlas_ezo_devices[ATLAS_HUM_EZO] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, atlas_ezo_id);
    static const struct of_device_id atlas_ezo_dt_ids[] = {
    { .compatible = "atlas,co2-ezo", .data = &atlas_ezo_devices[ATLAS_CO2_EZO], },
    { .compatible = "atlas,o2-ezo", .data = &atlas_ezo_devices[ATLAS_O2_EZO], },
    { .compatible = "atlas,hum-ezo", .data = &atlas_ezo_devices[ATLAS_HUM_EZO], },
    { }
    };
    MODULE_DEVICE_TABLE(of, atlas_ezo_dt_ids);
#[no_mangle]
unsafe extern "C" fn atlas_ezo_probe(client: *mut i2c_client) -> c_int {
    static int atlas_ezo_probe(struct i2c_client *client)
    {
    const struct atlas_ezo_device *chip;
    struct atlas_ezo_data *data;
    struct iio_dev *indio_dev;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    chip = i2c_get_match_data(client);
    if (!chip)
    return -EINVAL;
    indio_dev.info = &atlas_info;
    indio_dev.name = ATLAS_EZO_DRV_NAME;
    indio_dev.channels = chip.channels;
    indio_dev.num_channels = chip.num_channels;
    indio_dev.modes = INDIO_DIRECT_MODE;
    data = iio_priv(indio_dev);
    data.client = client;
    data.chip = chip;
    mutex_init(&data.lock);
    return devm_iio_device_register(&client.dev, indio_dev);
    };
    static struct i2c_driver atlas_ezo_driver = {
    .driver = {
    .name	= ATLAS_EZO_DRV_NAME,
    .of_match_table	= atlas_ezo_dt_ids,
    },
    .probe		= atlas_ezo_probe,
    .id_table	= atlas_ezo_id,
    };
    module_i2c_driver(atlas_ezo_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("Atlas Scientific EZO sensors");
    MODULE_LICENSE("GPL");
