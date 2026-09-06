//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/vz89x.c
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
// vz89x.c - Support for SGX Sensortech MiCS VZ89X VOC sensors
//
// Copyright (C) 2015-2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const VZ89X_REG_MEASUREMENT: c_uint = 0x09;
pub const VZ89X_REG_MEASUREMENT_RD_SIZE: c_int = 6;
pub const VZ89X_REG_MEASUREMENT_WR_SIZE: c_int = 3;
pub const VZ89X_VOC_CO2_IDX: c_int = 0;
pub const VZ89X_VOC_SHORT_IDX: c_int = 1;
pub const VZ89X_VOC_TVOC_IDX: c_int = 2;
pub const VZ89X_VOC_RESISTANCE_IDX: c_int = 3;
pub const VZ89TE_REG_MEASUREMENT: c_uint = 0x0c;
pub const VZ89TE_REG_MEASUREMENT_RD_SIZE: c_int = 7;
pub const VZ89TE_REG_MEASUREMENT_WR_SIZE: c_int = 6;
pub const VZ89TE_VOC_TVOC_IDX: c_int = 0;
pub const VZ89TE_VOC_CO2_IDX: c_int = 1;
pub const VZ89TE_VOC_RESISTANCE_IDX: c_int = 2;
    enum {
    VZ89X,
    VZ89TE,
    };
    struct vz89x_chip_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vz89x_data {
    pub client: *mut i2c_client,
    pub chip: *const vz89x_chip_data,
    pub lock: mutex,
    pub cmd): *mut *mut *mut int (xfer)(struct vz89x_data data, u8,
    pub is_valid: bool,
    pub last_update: c_ulong,
    pub buffer: [u8; VZ89TE_REG_MEASUREMENT_RD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vz89x_chip_data {
    pub data): *mut *mut bool (valid)(struct vz89x_data,
    pub channels: *const iio_chan_spec,
    pub num_channels: u8,
    pub cmd: u8,
    pub read_size: u8,
    pub write_size: u8,
}

    static const struct iio_chan_spec vz89x_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_OFFSET) | BIT(IIO_CHAN_INFO_RAW),
    .address = VZ89X_VOC_CO2_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .address = VZ89X_VOC_SHORT_IDX,
    .extend_name = "short",
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_OFFSET) | BIT(IIO_CHAN_INFO_RAW),
    .address = VZ89X_VOC_TVOC_IDX,
    },
    {
    .type = IIO_RESISTANCE,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .address = VZ89X_VOC_RESISTANCE_IDX,
    .scan_index = -1,
    .scan_type = {
    .endianness = IIO_LE,
    },
    },
    };
    static const struct iio_chan_spec vz89te_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_OFFSET) | BIT(IIO_CHAN_INFO_RAW),
    .address = VZ89TE_VOC_TVOC_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_OFFSET) | BIT(IIO_CHAN_INFO_RAW),
    .address = VZ89TE_VOC_CO2_IDX,
    },
    {
    .type = IIO_RESISTANCE,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .address = VZ89TE_VOC_RESISTANCE_IDX,
    .scan_index = -1,
    .scan_type = {
    .endianness = IIO_BE,
    },
    },
    };
    static IIO_CONST_ATTR(in_concentration_co2_scale, "0.00000698689");
    static IIO_CONST_ATTR(in_concentration_voc_scale, "0.00000000436681223");
    static struct attribute *vz89x_attributes[] = {
    &iio_const_attr_in_concentration_co2_scale.dev_attr.attr,
    &iio_const_attr_in_concentration_voc_scale.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group vz89x_attrs_group = {
    .attrs = vz89x_attributes,
    };
//
// Chipset sometime updates in the middle of a reading causing it to reset the
// data pointer, and causing invalid reading of previous data.
// We can check for this by reading MSB of the resistance reading that is
// always zero, and by also confirming the VOC_short isn't zero.
//
#[no_mangle]
unsafe extern "C" fn vz89x_measurement_is_valid(data: *mut vz89x_data) -> bool {
    static bool vz89x_measurement_is_valid(struct vz89x_data *data)
    {
    if (data.buffer[VZ89X_VOC_SHORT_IDX] == 0)
    return true;
    return !!(data.buffer[data.chip.read_size - 1] > 0);
    }
// VZ89TE device has a modified CRC-8 two complement check
#[no_mangle]
unsafe extern "C" fn vz89te_measurement_is_valid(data: *mut vz89x_data) -> bool {
    static bool vz89te_measurement_is_valid(struct vz89x_data *data)
    {
    let mut crc: u8 = 0;
    int i, sum = 0;
    for (i = 0; i < (data.chip.read_size - 1); i++) {
    sum = crc + data.buffer[i];
    crc = sum;
    crc += sum / 256;
    }
    return !((0xff - crc) == data.buffer[data.chip.read_size - 1]);
    }
#[no_mangle]
unsafe extern "C" fn vz89x_i2c_xfer(data: *mut vz89x_data, cmd: u8) -> c_int {
    static int vz89x_i2c_xfer(struct vz89x_data *data, u8 cmd)
    {
    const struct vz89x_chip_data *chip = data.chip;
    struct i2c_client *client = data.client;
    struct i2c_msg msg[2];
    int ret;
    u8 buf[6] = { cmd, 0, 0, 0, 0, 0xf3 };
    msg[0].addr = client.addr;
    msg[0].flags = client.flags;
    msg[0].len = chip.write_size;
    msg[0].buf  = (char *) &buf;
    msg[1].addr = client.addr;
    msg[1].flags = client.flags | I2C_M_RD;
    msg[1].len = chip.read_size;
    msg[1].buf = (char *) &data.buffer;
    ret = i2c_transfer(client.adapter, msg, 2);
    return (ret == 2) ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn vz89x_smbus_xfer(data: *mut vz89x_data, cmd: u8) -> c_int {
    static int vz89x_smbus_xfer(struct vz89x_data *data, u8 cmd)
    {
    struct i2c_client *client = data.client;
    int ret;
    int i;
    ret = i2c_smbus_write_word_data(client, cmd, 0);
    if (ret < 0)
    return ret;
    for (i = 0; i < data.chip.read_size; i++) {
    ret = i2c_smbus_read_byte(client);
    if (ret < 0)
    return ret;
    data.buffer[i] = ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vz89x_get_measurement(data: *mut vz89x_data) -> c_int {
    static int vz89x_get_measurement(struct vz89x_data *data)
    {
    const struct vz89x_chip_data *chip = data.chip;
    int ret;
// sensor can only be polled once a second max per datasheet
    if (!time_after(jiffies, data.last_update + HZ))
    return data.is_valid ? 0 : -EAGAIN;
    data.is_valid = false;
    data.last_update = jiffies;
    ret = data.xfer(data, chip.cmd);
    if (ret < 0)
    return ret;
    ret = chip.valid(data);
    if (ret)
    return -EAGAIN;
    data.is_valid = true;
    return 0;
    }
    static int vz89x_get_resistance_reading(struct vz89x_data *data,
    struct iio_chan_spec const *chan,
    int *val)
    {
    u8 *tmp = &data.buffer[chan.address];
    switch (chan.scan_type.endianness) {
    case IIO_LE:
// val = le32_to_cpup((__le32 *) tmp) & GENMASK(23, 0);
    break;
    case IIO_BE:
// val = be32_to_cpup((__be32 *) tmp) >> 8;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int vz89x_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct vz89x_data *data = iio_priv(indio_dev);
    let mut ret: c_int = -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&data.lock);
    ret = vz89x_get_measurement(data);
    mutex_unlock(&data.lock);
    if (ret)
    return ret;
    switch (chan.type) {
    case IIO_CONCENTRATION:
// val = data->buffer[chan->address];
    return IIO_VAL_INT;
    case IIO_RESISTANCE:
    ret = vz89x_get_resistance_reading(data, chan, val);
    if (!ret)
    return IIO_VAL_INT;
    break;
    default:
    return -EINVAL;
    }
    break;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_RESISTANCE:
// val = 10;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    break;
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.channel2) {
    case IIO_MOD_CO2:
// val = 44;
// val2 = 250000;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_MOD_VOC:
// val = -13;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    return ret;
    }
    static const struct iio_info vz89x_info = {
    .attrs		= &vz89x_attrs_group,
    .read_raw	= vz89x_read_raw,
    };
    static const struct vz89x_chip_data vz89x_chips[] = {
    {
    .valid = vz89x_measurement_is_valid,
    .cmd = VZ89X_REG_MEASUREMENT,
    .read_size = VZ89X_REG_MEASUREMENT_RD_SIZE,
    .write_size = VZ89X_REG_MEASUREMENT_WR_SIZE,
    .channels = vz89x_channels,
    .num_channels = ARRAY_SIZE(vz89x_channels),
    },
    {
    .valid = vz89te_measurement_is_valid,
    .cmd = VZ89TE_REG_MEASUREMENT,
    .read_size = VZ89TE_REG_MEASUREMENT_RD_SIZE,
    .write_size = VZ89TE_REG_MEASUREMENT_WR_SIZE,
    .channels = vz89te_channels,
    .num_channels = ARRAY_SIZE(vz89te_channels),
    },
    };
    static const struct of_device_id vz89x_dt_ids[] = {
    { .compatible = "sgx,vz89x", .data = &vz89x_chips[VZ89X] },
    { .compatible = "sgx,vz89te", .data = &vz89x_chips[VZ89TE] },
    { }
    };
    MODULE_DEVICE_TABLE(of, vz89x_dt_ids);
#[no_mangle]
unsafe extern "C" fn vz89x_probe(client: *mut i2c_client) -> c_int {
    static int vz89x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct vz89x_data *data;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    if (i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    data.xfer = vz89x_i2c_xfer;
    else if (i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WORD_DATA | I2C_FUNC_SMBUS_BYTE))
    data.xfer = vz89x_smbus_xfer;
    else
    return -EOPNOTSUPP;
    data.chip = i2c_get_match_data(client);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    data.last_update = jiffies - HZ;
    mutex_init(&data.lock);
    indio_dev.info = &vz89x_info;
    indio_dev.name = dev_name(dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = data.chip.channels;
    indio_dev.num_channels = data.chip.num_channels;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct i2c_device_id vz89x_id[] = {
    { .name = "vz89x", .driver_data = (kernel_ulong_t)&vz89x_chips[VZ89X] },
    { .name = "vz89te", .driver_data = (kernel_ulong_t)&vz89x_chips[VZ89TE] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, vz89x_id);
    static struct i2c_driver vz89x_driver = {
    .driver = {
    .name	= "vz89x",
    .of_match_table = vz89x_dt_ids,
    },
    .probe = vz89x_probe,
    .id_table = vz89x_id,
    };
    module_i2c_driver(vz89x_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("SGX Sensortech MiCS VZ89X VOC sensors");
    MODULE_LICENSE("GPL v2");
