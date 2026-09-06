//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/aht10.c
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
// aht10.c - Linux hwmon driver for AHT10/AHT20 Temperature and Humidity sensors
// Copyright (C) 2020 Johannes Cornelis Draaijer
//

pub const AHT10_MEAS_SIZE: c_int = 6;
pub const AHT20_MEAS_SIZE: c_int = 7;
pub const AHT20_CRC8_POLY: c_uint = 0x31;
//
// Poll intervals (in milliseconds)
//
pub const AHT10_DEFAULT_MIN_POLL_INTERVAL: c_int = 2000;
pub const AHT10_MIN_POLL_INTERVAL: c_int = 2000;
//
// I2C command delays (in microseconds)
//
pub const AHT10_MEAS_DELAY: c_int = 80000;
pub const AHT10_CMD_DELAY: c_int = 350000;
pub const AHT10_DELAY_EXTRA: c_int = 100000;
//
// Command bytes
//

//
// Flags in the answer byte/command
//

pub const AHT10_MAX_POLL_INTERVAL_LEN: c_int = 30;
    enum aht10_variant { aht10, aht20, dht20};
    static const struct i2c_device_id aht10_id[] = {
    { .name = "aht10", .driver_data = aht10 },
    { .name = "aht20", .driver_data = aht20 },
    { .name = "dht20", .driver_data = dht20 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, aht10_id);
    static const struct of_device_id aht10_of_match[] = {
    { .compatible = "aosong,aht10", .data = (void *)aht10 },
    { .compatible = "aosong,aht20", .data = (void *)aht20 },
    { .compatible = "aosong,dht20", .data = (void *)dht20 },
    {}
    };
    MODULE_DEVICE_TABLE(of, aht10_of_match);
//
// struct aht10_data - All the data required to operate an AHT10/AHT20 chip
// @client: the i2c client associated with the AHT10/AHT20
// @min_poll_interval: the minimum poll interval
// While the poll rate limit is not 100% necessary,
// the datasheet recommends that a measurement
// is not performed too often to prevent
// the chip from warming up due to the heat it generates.
// If it's unwanted, it can be ignored setting it to
// it to 0. Default value is 2000 ms
// @previous_poll_time: the previous time that the AHT10/AHT20
// was polled
// @temperature: the latest temperature value received from
// the AHT10/AHT20
// @humidity: the latest humidity value received from the
// AHT10/AHT20
// @crc8: crc8 support flag
// @meas_size: measurements data size
// @init_cmd: Initialization command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aht10_data {
    pub client: *mut i2c_client,
    pub min_poll_interval: ktime_t,
    pub previous_poll_time: ktime_t,
    pub temperature: c_int,
    pub humidity: c_int,
    pub crc8: bool,
    pub meas_size: c_uint,
    pub init_cmd: u8,
}

//
// aht10_init() - Initialize an AHT10/AHT20 chip
// @data: the data associated with this AHT10/AHT20 chip
// Return: 0 if successful, 1 if not
//
#[no_mangle]
unsafe extern "C" fn aht10_init(data: *mut aht10_data) -> c_int {
    static int aht10_init(struct aht10_data *data)
    {
    const u8 cmd_init[] = {data.init_cmd, AHT10_CAL_ENABLED | AHT10_MODE_CYC,
    0x00};
    int res;
    u8 status;
    struct i2c_client *client = data.client;
    res = i2c_master_send(client, cmd_init, sizeof(cmd_init));
    if (res < 0)
    return res;
    usleep_range(AHT10_CMD_DELAY, AHT10_CMD_DELAY +
    AHT10_DELAY_EXTRA);
    res = i2c_master_recv(client, &status, 1);
    if (res != 1)
    return -ENODATA;
    if (status & AHT10_BUSY)
    return -EBUSY;
    return 0;
    }
//
// aht10_polltime_expired() - check if the minimum poll interval has
// expired
// @data: the data containing the time to compare
// Return: 1 if the minimum poll interval has expired, 0 if not
//
#[no_mangle]
unsafe extern "C" fn aht10_polltime_expired(data: *mut aht10_data) -> c_int {
    static int aht10_polltime_expired(struct aht10_data *data)
    {
    let mut current_time: ktime_t = ktime_get_boottime();
    let mut difference: ktime_t = ktime_sub(current_time, data.previous_poll_time);
    return ktime_after(difference, data.min_poll_interval);
    }
    DECLARE_CRC8_TABLE(crc8_table);
//
// crc8_check() - check crc of the sensor's measurements
// @raw_data: data frame received from sensor(including crc as the last byte)
// @count: size of the data frame
// Return: 0 if successful, 1 if not
//
#[no_mangle]
unsafe extern "C" fn crc8_check(raw_data: *mut u8, count: c_int) -> c_int {
    static int crc8_check(u8 *raw_data, int count)
    {
//
// crc calculated on the whole frame(including crc byte) should yield
// zero in case of correctly received bytes
//
    return crc8(crc8_table, raw_data, count, CRC8_INIT_VALUE);
    }
//
// aht10_read_values() - read and parse the raw data from the AHT10/AHT20
// @data: the struct aht10_data to use for the lock
// Return: 0 if successful, 1 if not
//
#[no_mangle]
unsafe extern "C" fn aht10_read_values(data: *mut aht10_data) -> c_int {
    static int aht10_read_values(struct aht10_data *data)
    {
    const u8 cmd_meas[] = {AHT10_CMD_MEAS, 0x33, 0x00};
    u32 temp, hum;
    int res;
    u8 raw_data[AHT20_MEAS_SIZE];
    struct i2c_client *client = data.client;
    if (!aht10_polltime_expired(data))
    return 0;
    res = i2c_master_send(client, cmd_meas, sizeof(cmd_meas));
    if (res < 0)
    return res;
    usleep_range(AHT10_MEAS_DELAY, AHT10_MEAS_DELAY + AHT10_DELAY_EXTRA);
    res = i2c_master_recv(client, raw_data, data.meas_size);
    if (res != data.meas_size) {
    if (res >= 0)
    return -ENODATA;
    return res;
    }
    if (data.crc8 && crc8_check(raw_data, data.meas_size))
    return -EIO;
    hum =   ((u32)raw_data[1] << 12u) |
    ((u32)raw_data[2] << 4u) |
    ((raw_data[3] & 0xF0u) >> 4u);
    temp =  ((u32)(raw_data[3] & 0x0Fu) << 16u) |
    ((u32)raw_data[4] << 8u) |
    raw_data[5];
    temp = ((temp * 625) >> 15u) * 10;
    hum = ((hum * 625) >> 16u) * 10;
    data.temperature = (int)temp - 50000;
    data.humidity = hum;
    data.previous_poll_time = ktime_get_boottime();
    return 0;
    }
//
// aht10_interval_write() - store the given minimum poll interval.
// Return: 0 on success, -EINVAL if a value lower than the
// AHT10_MIN_POLL_INTERVAL is given
//
    static ssize_t aht10_interval_write(struct aht10_data *data,
    long val)
    {
    data.min_poll_interval = ms_to_ktime(clamp_val(val, 2000, LONG_MAX));
    return 0;
    }
//
// aht10_interval_read() - read the minimum poll interval
// in milliseconds
//
    static ssize_t aht10_interval_read(struct aht10_data *data,
    long *val)
    {
// val = ktime_to_ms(data->min_poll_interval);
    return 0;
    }
//
// aht10_temperature1_read() - read the temperature in millidegrees
//
#[no_mangle]
unsafe extern "C" fn aht10_temperature1_read(data: *mut aht10_data, val: *mut c_long) -> c_int {
    static int aht10_temperature1_read(struct aht10_data *data, long *val)
    {
    int res;
    res = aht10_read_values(data);
    if (res < 0)
    return res;
// val = data->temperature;
    return 0;
    }
//
// aht10_humidity1_read() - read the relative humidity in millipercent
//
#[no_mangle]
unsafe extern "C" fn aht10_humidity1_read(data: *mut aht10_data, val: *mut c_long) -> c_int {
    static int aht10_humidity1_read(struct aht10_data *data, long *val)
    {
    int res;
    res = aht10_read_values(data);
    if (res < 0)
    return res;
// val = data->humidity;
    return 0;
    }
    static umode_t aht10_hwmon_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    case hwmon_humidity:
    return 0444;
    case hwmon_chip:
    return 0644;
    default:
    return 0;
    }
    }
    static int aht10_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct aht10_data *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_temp:
    return aht10_temperature1_read(data, val);
    case hwmon_humidity:
    return aht10_humidity1_read(data, val);
    case hwmon_chip:
    return aht10_interval_read(data, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int aht10_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct aht10_data *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_chip:
    return aht10_interval_write(data, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_channel_info * const aht10_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_UPDATE_INTERVAL),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(humidity, HWMON_H_INPUT),
    core::ptr::null_mut(),
    };
    static const struct hwmon_ops aht10_hwmon_ops = {
    .is_visible = aht10_hwmon_visible,
    .read = aht10_hwmon_read,
    .write = aht10_hwmon_write,
    };
    static const struct hwmon_chip_info aht10_chip_info = {
    .ops = &aht10_hwmon_ops,
    .info = aht10_info,
    };
#[no_mangle]
unsafe extern "C" fn aht10_probe(client: *mut i2c_client) -> c_int {
    static int aht10_probe(struct i2c_client *client)
    {
    let mut variant: enum aht10_variant = (uintptr_t)i2c_get_match_data(client);
    struct device *device = &client.dev;
    struct device *hwmon_dev;
    struct aht10_data *data;
    int res;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENOENT;
    data = devm_kzalloc(device, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.min_poll_interval = ms_to_ktime(AHT10_DEFAULT_MIN_POLL_INTERVAL);
    data.client = client;
    switch (variant) {
    case aht20:
    data.meas_size = AHT20_MEAS_SIZE;
    data.crc8 = true;
    crc8_populate_msb(crc8_table, AHT20_CRC8_POLY);
    data.init_cmd = AHT20_CMD_INIT;
    break;
    case dht20:
    data.meas_size = AHT20_MEAS_SIZE;
    data.crc8 = true;
    crc8_populate_msb(crc8_table, AHT20_CRC8_POLY);
    data.init_cmd = DHT20_CMD_INIT;
    break;
    default:
    data.meas_size = AHT10_MEAS_SIZE;
    data.init_cmd = AHT10_CMD_INIT;
    break;
    }
    res = aht10_init(data);
    if (res < 0)
    return res;
    res = aht10_read_values(data);
    if (res < 0)
    return res;
    hwmon_dev = devm_hwmon_device_register_with_info(device,
    client.name,
    data,
    &aht10_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct i2c_driver aht10_driver = {
    .driver = {
    .name = "aht10",
    .of_match_table = aht10_of_match,
    },
    .probe      = aht10_probe,
    .id_table   = aht10_id,
    };
    module_i2c_driver(aht10_driver);
    MODULE_AUTHOR("Johannes Cornelis Draaijer <jcdra1@gmail.com>");
    MODULE_DESCRIPTION("AHT10/AHT20 Temperature and Humidity sensor driver");
    MODULE_VERSION("1.0");
    MODULE_LICENSE("GPL v2");
