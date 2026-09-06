//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/sps30_i2c.c
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
// Sensirion SPS30 particulate matter sensor i2c driver
//
// Copyright (c) 2020 Tomasz Duszynski <tduszyns@gmail.com>
//
// I2C slave address: 0x69
//

pub const SPS30_I2C_CRC8_POLYNOMIAL: c_uint = 0x31;
// max number of bytes needed to store PM measurements or serial string
pub const SPS30_I2C_MAX_BUF_SIZE: c_int = 48;
    DECLARE_CRC8_TABLE(sps30_i2c_crc8_table);
pub const SPS30_I2C_START_MEAS: c_uint = 0x0010;
pub const SPS30_I2C_STOP_MEAS: c_uint = 0x0104;
pub const SPS30_I2C_READ_MEAS: c_uint = 0x0300;
pub const SPS30_I2C_MEAS_READY: c_uint = 0x0202;
pub const SPS30_I2C_RESET: c_uint = 0xd304;
pub const SPS30_I2C_CLEAN_FAN: c_uint = 0x5607;
pub const SPS30_I2C_PERIOD: c_uint = 0x8004;
pub const SPS30_I2C_READ_SERIAL: c_uint = 0xd033;
pub const SPS30_I2C_READ_VERSION: c_uint = 0xd100;
    static int sps30_i2c_xfer(struct sps30_state *state, unsigned char *txbuf, size_t txsize,
    unsigned char *rxbuf, size_t rxsize)
    {
    struct i2c_client *client = to_i2c_client(state.dev);
    int ret;
//
// Sensor does not support repeated start so instead of
// sending two i2c messages in a row we just send one by one.
//
    ret = i2c_master_send(client, txbuf, txsize);
    if (ret < 0)
    return ret;
    if (ret != txsize)
    return -EIO;
    if (!rxsize)
    return 0;
    ret = i2c_master_recv(client, rxbuf, rxsize);
    if (ret < 0)
    return ret;
    if (ret != rxsize)
    return -EIO;
    return 0;
    }
    static int sps30_i2c_command(struct sps30_state *state, u16 cmd, void *arg, size_t arg_size,
    void *rsp, size_t rsp_size)
    {
//
// Internally sensor stores measurements in a following manner:
//
// PM1:   upper two bytes, crc8, lower two bytes, crc8
// PM2P5: upper two bytes, crc8, lower two bytes, crc8
// PM4:   upper two bytes, crc8, lower two bytes, crc8
// PM10:  upper two bytes, crc8, lower two bytes, crc8
//
// What follows next are number concentration measurements and
// typical particle size measurement which we omit.
//
    unsigned char buf[SPS30_I2C_MAX_BUF_SIZE];
    unsigned char *tmp;
    unsigned char crc;
    size_t i;
    int ret;
    put_unaligned_be16(cmd, buf);
    i = 2;
    if (rsp) {
// each two bytes are followed by a crc8
    rsp_size += rsp_size / 2;
    } else {
    tmp = arg;
    while (arg_size) {
    buf[i] = *tmp++;
    buf[i + 1] = *tmp++;
    buf[i + 2] = crc8(sps30_i2c_crc8_table, buf + i, 2, CRC8_INIT_VALUE);
    arg_size -= 2;
    i += 3;
    }
    }
    ret = sps30_i2c_xfer(state, buf, i, buf, rsp_size);
    if (ret)
    return ret;
// validate received data and strip off crc bytes
    tmp = rsp;
    for (i = 0; i < rsp_size; i += 3) {
    crc = crc8(sps30_i2c_crc8_table, buf + i, 2, CRC8_INIT_VALUE);
    if (crc != buf[i + 2]) {
    dev_err(state.dev, "data integrity check failed\n");
    return -EIO;
    }
// tmp++ = buf[i];
// tmp++ = buf[i + 1];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_start_meas(state: *mut sps30_state) -> c_int {
    static int sps30_i2c_start_meas(struct sps30_state *state)
    {
// request BE IEEE754 formatted data
    unsigned char buf[] = { 0x03, 0x00 };
    return sps30_i2c_command(state, SPS30_I2C_START_MEAS, buf, sizeof(buf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_stop_meas(state: *mut sps30_state) -> c_int {
    static int sps30_i2c_stop_meas(struct sps30_state *state)
    {
    return sps30_i2c_command(state, SPS30_I2C_STOP_MEAS, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_reset(state: *mut sps30_state) -> c_int {
    static int sps30_i2c_reset(struct sps30_state *state)
    {
    int ret;
    ret = sps30_i2c_command(state, SPS30_I2C_RESET, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    msleep(500);
//
// Power-on-reset causes sensor to produce some glitch on i2c bus and
// some controllers end up in error state. Recover simply by placing
// some data on the bus, for example STOP_MEAS command, which
// is NOP in this case.
//
    sps30_i2c_stop_meas(state);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_meas_ready(state: *mut sps30_state) -> bool {
    static bool sps30_i2c_meas_ready(struct sps30_state *state)
    {
    unsigned char buf[2];
    int ret;
    ret = sps30_i2c_command(state, SPS30_I2C_MEAS_READY, core::ptr::null_mut(), 0, buf, sizeof(buf));
    if (ret)
    return false;
    return buf[1];
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_read_meas(state: *mut sps30_state, meas: *mut __be32, num: usize) -> c_int {
    static int sps30_i2c_read_meas(struct sps30_state *state, __be32 *meas, size_t num)
    {
// measurements are ready within a second
    if (msleep_interruptible(1000))
    return -EINTR;
    if (!sps30_i2c_meas_ready(state))
    return -ETIMEDOUT;
    return sps30_i2c_command(state, SPS30_I2C_READ_MEAS, core::ptr::null_mut(), 0, meas, sizeof(*meas) * num);
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_clean_fan(state: *mut sps30_state) -> c_int {
    static int sps30_i2c_clean_fan(struct sps30_state *state)
    {
    return sps30_i2c_command(state, SPS30_I2C_CLEAN_FAN, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_read_cleaning_period(state: *mut sps30_state, period: *mut __be32) -> c_int {
    static int sps30_i2c_read_cleaning_period(struct sps30_state *state, __be32 *period)
    {
    return sps30_i2c_command(state, SPS30_I2C_PERIOD, core::ptr::null_mut(), 0, period, sizeof(*period));
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_write_cleaning_period(state: *mut sps30_state, period: __be32) -> c_int {
    static int sps30_i2c_write_cleaning_period(struct sps30_state *state, __be32 period)
    {
    return sps30_i2c_command(state, SPS30_I2C_PERIOD, &period, sizeof(period), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_i2c_show_info(state: *mut sps30_state) -> c_int {
    static int sps30_i2c_show_info(struct sps30_state *state)
    {
// extra nul just in case
    unsigned char buf[32 + 1] = { 0x00 };
    int ret;
    ret = sps30_i2c_command(state, SPS30_I2C_READ_SERIAL, core::ptr::null_mut(), 0, buf, sizeof(buf) - 1);
    if (ret)
    return ret;
    dev_info(state.dev, "serial number: %s\n", buf);
    ret = sps30_i2c_command(state, SPS30_I2C_READ_VERSION, core::ptr::null_mut(), 0, buf, 2);
    if (ret)
    return ret;
    dev_info(state.dev, "fw version: %u.%u\n", buf[0], buf[1]);
    return 0;
    }
    static const struct sps30_ops sps30_i2c_ops = {
    .start_meas = sps30_i2c_start_meas,
    .stop_meas = sps30_i2c_stop_meas,
    .read_meas = sps30_i2c_read_meas,
    .reset = sps30_i2c_reset,
    .clean_fan = sps30_i2c_clean_fan,
    .read_cleaning_period = sps30_i2c_read_cleaning_period,
    .write_cleaning_period = sps30_i2c_write_cleaning_period,
    .show_info = sps30_i2c_show_info,
    };
#[no_mangle]
unsafe extern "C" fn sps30_i2c_probe(client: *mut i2c_client) -> c_int {
    static int sps30_i2c_probe(struct i2c_client *client)
    {
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    crc8_populate_msb(sps30_i2c_crc8_table, SPS30_I2C_CRC8_POLYNOMIAL);
    return sps30_probe(&client.dev, client.name, core::ptr::null_mut(), &sps30_i2c_ops);
    }
    static const struct i2c_device_id sps30_i2c_id[] = {
    { .name = "sps30" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sps30_i2c_id);
    static const struct of_device_id sps30_i2c_of_match[] = {
    { .compatible = "sensirion,sps30" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sps30_i2c_of_match);
    static struct i2c_driver sps30_i2c_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = sps30_i2c_of_match,
    },
    .id_table = sps30_i2c_id,
    .probe = sps30_i2c_probe,
    };
    module_i2c_driver(sps30_i2c_driver);
    MODULE_AUTHOR("Tomasz Duszynski <tduszyns@gmail.com>");
    MODULE_DESCRIPTION("Sensirion SPS30 particulate matter sensor i2c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_SPS30");
