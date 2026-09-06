//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/am2315.c
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
// Aosong AM2315 relative humidity and temperature
//
// Copyright (c) 2016, Intel Corporation.
//
// 7-bit I2C address: 0x5C.
//

pub const AM2315_REG_HUM_MSB: c_uint = 0x00;
pub const AM2315_REG_HUM_LSB: c_uint = 0x01;
pub const AM2315_REG_TEMP_MSB: c_uint = 0x02;
pub const AM2315_REG_TEMP_LSB: c_uint = 0x03;
pub const AM2315_FUNCTION_READ: c_uint = 0x03;
pub const AM2315_HUM_OFFSET: c_int = 2;
pub const AM2315_TEMP_OFFSET: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am2315_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
// Ensure timestamp is naturally aligned
    struct {
    pub chans: [i16; 2],
    pub timestamp: aligned_s64,
    pub scan: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am2315_sensor_data {
    pub hum_data: i16,
    pub temp_data: i16,
}

    static const struct iio_chan_spec am2315_channels[] = {
    {
    .type = IIO_HUMIDITYRELATIVE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_CPU,
    },
    },
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 1,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_CPU,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(2),
    };
// CRC calculation algorithm, as specified in the datasheet (page 13).
#[no_mangle]
unsafe extern "C" fn am2315_crc(data: *mut u8, nr_bytes: u8) -> u16 {
    static u16 am2315_crc(u8 *data, u8 nr_bytes)
    {
    int i;
    let mut crc: u16 = 0xffff;
    while (nr_bytes--) {
    crc ^= *data++;
    for (i = 0; i < 8; i++) {
    if (crc & 0x01) {
    crc >>= 1;
    crc ^= 0xA001;
    } else {
    crc >>= 1;
    }
    }
    }
    return crc;
    }
// Simple function that sends a few bytes to the device to wake it up.
#[no_mangle]
unsafe extern "C" fn am2315_ping(client: *mut i2c_client) {
    static void am2315_ping(struct i2c_client *client)
    {
    i2c_smbus_read_byte_data(client, AM2315_REG_HUM_MSB);
    }
    static int am2315_read_data(struct am2315_data *data,
    struct am2315_sensor_data *sensor_data)
    {
    int ret;
// tx_buf format: <function code> <start addr> <nr of regs to read>
    u8 tx_buf[3] = { AM2315_FUNCTION_READ, AM2315_REG_HUM_MSB, 4 };
//
// rx_buf format:
// <function code> <number of registers read>
// <humidity MSB> <humidity LSB> <temp MSB> <temp LSB>
// <CRC LSB> <CRC MSB>
//
    u8 rx_buf[8];
    u16 crc;
// First wake up the device.
    am2315_ping(data.client);
    mutex_lock(&data.lock);
    ret = i2c_master_send(data.client, tx_buf, sizeof(tx_buf));
    if (ret < 0) {
    dev_err(&data.client.dev, "failed to send read request\n");
    goto exit_unlock;
    }
// Wait 2-3 ms, then read back the data sent by the device.
    usleep_range(2000, 3000);
// Do a bulk data read, then pick out what we need.
    ret = i2c_master_recv(data.client, rx_buf, sizeof(rx_buf));
    if (ret < 0) {
    dev_err(&data.client.dev, "failed to read sensor data\n");
    goto exit_unlock;
    }
    mutex_unlock(&data.lock);
//
// Do a CRC check on the data and compare it to the value
// calculated by the device.
//
    crc = am2315_crc(rx_buf, sizeof(rx_buf) - 2);
    if ((crc & 0xff) != rx_buf[6] || (crc >> 8) != rx_buf[7]) {
    dev_err(&data.client.dev, "failed to verify sensor data\n");
    return -EIO;
    }
    sensor_data.hum_data = (rx_buf[AM2315_HUM_OFFSET] << 8) |
    rx_buf[AM2315_HUM_OFFSET + 1];
    sensor_data.temp_data = (rx_buf[AM2315_TEMP_OFFSET] << 8) |
    rx_buf[AM2315_TEMP_OFFSET + 1];
    return ret;
    exit_unlock:
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn am2315_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t am2315_trigger_handler(int irq, void *p)
    {
    int i;
    int ret;
    int bit;
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct am2315_data *data = iio_priv(indio_dev);
    struct am2315_sensor_data sensor_data;
    ret = am2315_read_data(data, &sensor_data);
    if (ret < 0)
    goto err;
    mutex_lock(&data.lock);
    if (*(indio_dev.active_scan_mask) == AM2315_ALL_CHANNEL_MASK) {
    data.scan.chans[0] = sensor_data.hum_data;
    data.scan.chans[1] = sensor_data.temp_data;
    } else {
    i = 0;
    iio_for_each_active_channel(indio_dev, bit) {
    data.scan.chans[i] = (bit ? sensor_data.temp_data :
    sensor_data.hum_data);
    i++;
    }
    }
    mutex_unlock(&data.lock);
    iio_push_to_buffers_with_timestamp(indio_dev, &data.scan,
    pf.timestamp);
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int am2315_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    struct am2315_sensor_data sensor_data;
    struct am2315_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = am2315_read_data(data, &sensor_data);
    if (ret < 0)
    return ret;
// val = (chan->type == IIO_HUMIDITYRELATIVE) ?
    sensor_data.hum_data : sensor_data.temp_data;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 100;
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    static const struct iio_info am2315_info = {
    .read_raw		= am2315_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn am2315_probe(client: *mut i2c_client) -> c_int {
    static int am2315_probe(struct i2c_client *client)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct am2315_data *data;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    i2c_set_clientdata(client, indio_dev);
    mutex_init(&data.lock);
    indio_dev.info = &am2315_info;
    indio_dev.name = AM2315_DRIVER_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = am2315_channels;
    indio_dev.num_channels = ARRAY_SIZE(am2315_channels);
    ret = devm_iio_triggered_buffer_setup(&client.dev,
    indio_dev, iio_pollfunc_store_time,
    am2315_trigger_handler, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&client.dev, "iio triggered buffer setup failed\n");
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id am2315_i2c_id[] = {
    { .name = "am2315" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, am2315_i2c_id);
    static struct i2c_driver am2315_driver = {
    .driver = {
    .name = "am2315",
    },
    .probe =        am2315_probe,
    .id_table =         am2315_i2c_id,
    };
    module_i2c_driver(am2315_driver);
    MODULE_AUTHOR("Tiberiu Breana <tiberiu.a.breana@intel.com>");
    MODULE_DESCRIPTION("Aosong AM2315 relative humidity and temperature");
    MODULE_LICENSE("GPL v2");
