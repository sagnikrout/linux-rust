//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/veml6070.c
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
// veml6070.c - Support for Vishay VEML6070 UV A light sensor
//
// Copyright 2016 Peter Meerwald-Stadler <pmeerw@pmeerw.net>
//
// IIO driver for VEML6070 (7-bit I2C slave addresses 0x38 and 0x39)
//
// TODO: ACK signal
//

pub const VEML6070_ADDR_CONFIG_DATA_MSB: c_uint = 0x38 /* read: MSB data, write: config */;
pub const VEML6070_ADDR_DATA_LSB: c_uint = 0x39 /* LSB data */;

pub const VEML6070_IT_05: c_uint = 0x00;
pub const VEML6070_IT_10: c_uint = 0x01;
pub const VEML6070_IT_20: c_uint = 0x02;
pub const VEML6070_IT_40: c_uint = 0x03;
pub const VEML6070_MIN_RSET_KOHM: c_int = 75;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct veml6070_data {
    pub client1: *mut i2c_client,
    pub client2: *mut i2c_client,
    pub config: u8,
    pub lock: mutex,
    pub rset: u32,
    pub it: [c_int; 4][2],
}

#[no_mangle]
unsafe extern "C" fn veml6070_calc_it(dev: *mut device, data: *mut veml6070_data) -> c_int {
    static int veml6070_calc_it(struct device *dev, struct veml6070_data *data)
    {
    int i, tmp_it;
    data.rset = 270000;
    device_property_read_u32(dev, "vishay,rset-ohms", &data.rset);
    if (data.rset < 75000 || data.rset > 1200000)
    return dev_err_probe(dev, -EINVAL, "Rset out of range\n");
//
// convert to kohm to avoid overflows and work with the same units as
// in the datasheet and simplify UVI operations.
//
    data.rset /= KILO;
    tmp_it = VEML6070_MIN_IT_US * data.rset / VEML6070_MIN_RSET_KOHM;
    for (i = 0; i < ARRAY_SIZE(data.it); i++) {
    data.it[i][0] = (tmp_it << i) / MICRO;
    data.it[i][1] = (tmp_it << i) % MICRO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn veml6070_get_it(data: *mut veml6070_data, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int veml6070_get_it(struct veml6070_data *data, int *val, int *val2)
    {
    let mut it_idx: c_int = FIELD_GET(VEML6070_COMMAND_IT, data.config);
// val = data->it[it_idx][0];
// val2 = data->it[it_idx][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
#[no_mangle]
unsafe extern "C" fn veml6070_set_it(data: *mut veml6070_data, val: c_int, val2: c_int) -> c_int {
    static int veml6070_set_it(struct veml6070_data *data, int val, int val2)
    {
    int it_idx;
    for (it_idx = 0; it_idx < ARRAY_SIZE(data.it); it_idx++) {
    if (data.it[it_idx][0] == val && data.it[it_idx][1] == val2)
    break;
    }
    if (it_idx >= ARRAY_SIZE(data.it))
    return -EINVAL;
    data.config = (data.config & ~VEML6070_COMMAND_IT) |
    FIELD_PREP(VEML6070_COMMAND_IT, it_idx);
    return i2c_smbus_write_byte(data.client1, data.config);
    }
#[no_mangle]
unsafe extern "C" fn veml6070_read(data: *mut veml6070_data) -> c_int {
    static int veml6070_read(struct veml6070_data *data)
    {
    int ret, it_ms, val, val2;
    u8 msb, lsb;
    guard(mutex)(&data.lock);
// disable shutdown
    ret = i2c_smbus_write_byte(data.client1,
    data.config & ~VEML6070_COMMAND_SD);
    if (ret < 0)
    return ret;
    veml6070_get_it(data, &val, &val2);
    it_ms = val * MILLI + val2 / (MICRO / MILLI);
    msleep(it_ms + 10);
    ret = i2c_smbus_read_byte(data.client2); /* read MSB, address 0x39 */
    if (ret < 0)
    return ret;
    msb = ret;
    ret = i2c_smbus_read_byte(data.client1); /* read LSB, address 0x38 */
    if (ret < 0)
    return ret;
    lsb = ret;
// shutdown again
    ret = i2c_smbus_write_byte(data.client1, data.config);
    if (ret < 0)
    return ret;
    return (msb << 8) | lsb;
    }
    static const struct iio_chan_spec veml6070_channels[] = {
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_UV,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    {
    .type = IIO_UVINDEX,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_INT_TIME),
    }
    };
#[no_mangle]
unsafe extern "C" fn veml6070_to_uv_index(data: *mut veml6070_data, val: c_uint) -> c_int {
    static int veml6070_to_uv_index(struct veml6070_data *data, unsigned int val)
    {
//
// conversion of raw UV intensity values to UV index depends on
// integration time (IT) and value of the resistor connected to
// the RSET pin.
//
    unsigned int uvi[11] = {
    187, 373, 560, /* low */
    746, 933, 1120, /* moderate */
    1308, 1494, /* high */
    1681, 1868, 2054}; /* very high */
    int i, it_idx;
    it_idx = FIELD_GET(VEML6070_COMMAND_IT, data.config);
    if (!it_idx)
    val = (val * 270  / data.rset) << 1;
    else
    val = (val * 270 / data.rset) >> (it_idx - 1);
    for (i = 0; i < ARRAY_SIZE(uvi); i++)
    if (val <= uvi[i])
    return i;
    return 11; /* extreme */
    }
    static int veml6070_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct veml6070_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    case IIO_CHAN_INFO_PROCESSED:
    ret = veml6070_read(data);
    if (ret < 0)
    return ret;
    if (mask == IIO_CHAN_INFO_PROCESSED)
// val = veml6070_to_uv_index(data, ret);
    else
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_INT_TIME:
    return veml6070_get_it(data, val, val2);
    default:
    return -EINVAL;
    }
    }
    static int veml6070_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct veml6070_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
// vals = (int *)data->it;
// length = 2 * ARRAY_SIZE(data->it);
// type = IIO_VAL_INT_PLUS_MICRO;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int veml6070_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct veml6070_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
    return veml6070_set_it(data, val, val2);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info veml6070_info = {
    .read_raw = veml6070_read_raw,
    .read_avail  = veml6070_read_avail,
    .write_raw = veml6070_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn veml6070_probe(client: *mut i2c_client) -> c_int {
    static int veml6070_probe(struct i2c_client *client)
    {
    struct veml6070_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client1 = client;
    mutex_init(&data.lock);
    indio_dev.info = &veml6070_info;
    indio_dev.channels = veml6070_channels;
    indio_dev.num_channels = ARRAY_SIZE(veml6070_channels);
    indio_dev.name = VEML6070_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = veml6070_calc_it(&client.dev, data);
    if (ret < 0)
    return ret;
    ret = devm_regulator_get_enable(&client.dev, "vdd");
    if (ret < 0)
    return ret;
    data.client2 = devm_i2c_new_dummy_device(&client.dev, client.adapter,
    VEML6070_ADDR_DATA_LSB);
    if (IS_ERR(data.client2))
    return dev_err_probe(&client.dev, PTR_ERR(data.client2),
    "i2c device for second chip address failed\n");
    data.config = FIELD_PREP(VEML6070_COMMAND_IT, VEML6070_IT_10) |
    VEML6070_COMMAND_RSRVD | VEML6070_COMMAND_SD;
    ret = i2c_smbus_write_byte(data.client1, data.config);
    if (ret < 0)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id veml6070_id[] = {
    { .name = "veml6070" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, veml6070_id);
    static const struct of_device_id veml6070_of_match[] = {
    { .compatible = "vishay,veml6070" },
    { }
    };
    MODULE_DEVICE_TABLE(of, veml6070_of_match);
    static struct i2c_driver veml6070_driver = {
    .driver = {
    .name   = VEML6070_DRV_NAME,
    .of_match_table = veml6070_of_match,
    },
    .probe = veml6070_probe,
    .id_table = veml6070_id,
    };
    module_i2c_driver(veml6070_driver);
    MODULE_AUTHOR("Peter Meerwald-Stadler <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("Vishay VEML6070 UV A light sensor driver");
    MODULE_LICENSE("GPL");
