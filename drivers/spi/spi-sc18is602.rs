//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-sc18is602.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// NXP SC18IS602/603 SPI driver
//
// Copyright (C) Guenter Roeck <linux@roeck-us.net>
//

    enum chips { sc18is602, sc18is602b, sc18is603 };
pub const SC18IS602_BUFSIZ: c_int = 200;
pub const SC18IS602_CLOCK: c_int = 7372000;

pub const SC18IS602_MODE_CLOCK_DIV_4: c_uint = 0x0;
pub const SC18IS602_MODE_CLOCK_DIV_16: c_uint = 0x1;
pub const SC18IS602_MODE_CLOCK_DIV_64: c_uint = 0x2;
pub const SC18IS602_MODE_CLOCK_DIV_128: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc18is602 {
    pub host: *mut spi_controller,
    pub dev: *mut device,
    pub ctrl: u8,
    pub freq: u32,
    pub speed: u32,
// I2C data
    pub client: *mut i2c_client,
    pub id: enum chips,
    pub 1]: u8 buffer[SC18IS602_BUFSIZ +,
    pub /: *mut *mut int tlen; / Data queued for tx in buffer,
    pub /: *mut *mut int rindex; / Receive data index in buffer,
    pub reset: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn sc18is602_wait_ready(hw: *mut sc18is602, len: c_int) -> c_int {
    static int sc18is602_wait_ready(struct sc18is602 *hw, int len)
    {
    int i, err;
    let mut usecs: c_int = 1000000 * len / hw.speed + 1;
    u8 dummy[1];
    for (i = 0; i < 10; i++) {
    err = i2c_master_recv(hw.client, dummy, 1);
    if (err >= 0)
    return 0;
    usleep_range(usecs, usecs * 2);
    }
    return -ETIMEDOUT;
    }
    static int sc18is602_txrx(struct sc18is602 *hw, struct spi_message *msg,
    struct spi_transfer *t, bool do_transfer)
    {
    let mut len: c_uint = t.len;
    int ret;
    if (hw.tlen == 0) {
// First byte (I2C command) is chip select
    hw.buffer[0] = 1 << spi_get_chipselect(msg.spi, 0);
    hw.tlen = 1;
    hw.rindex = 0;
    }
//
// We can not immediately send data to the chip, since each I2C message
// resembles a full SPI message (from CS active to CS inactive).
// Enqueue messages up to the first read or until do_transfer is true.
//
    if (t.tx_buf) {
    memcpy(&hw.buffer[hw.tlen], t.tx_buf, len);
    hw.tlen += len;
    if (t.rx_buf)
    do_transfer = true;
    else
    hw.rindex = hw.tlen - 1;
    } else if (t.rx_buf) {
//
// For receive-only transfers we still need to perform a dummy
// write to receive data from the SPI chip.
// Read data starts at the end of transmit data (minus 1 to
// account for CS).
//
    hw.rindex = hw.tlen - 1;
    memset(&hw.buffer[hw.tlen], 0, len);
    hw.tlen += len;
    do_transfer = true;
    }
    if (do_transfer && hw.tlen > 1) {
    ret = sc18is602_wait_ready(hw, SC18IS602_BUFSIZ);
    if (ret < 0)
    return ret;
    ret = i2c_master_send(hw.client, hw.buffer, hw.tlen);
    if (ret < 0)
    return ret;
    if (ret != hw.tlen)
    return -EIO;
    if (t.rx_buf) {
    let mut rlen: c_int = hw.rindex + len;
    ret = sc18is602_wait_ready(hw, hw.tlen);
    if (ret < 0)
    return ret;
    ret = i2c_master_recv(hw.client, hw.buffer, rlen);
    if (ret < 0)
    return ret;
    if (ret != rlen)
    return -EIO;
    memcpy(t.rx_buf, &hw.buffer[hw.rindex], len);
    }
    hw.tlen = 0;
    }
    return len;
    }
#[no_mangle]
unsafe extern "C" fn sc18is602_setup_transfer(hw: *mut sc18is602, hz: u32, mode: u8) -> c_int {
    static int sc18is602_setup_transfer(struct sc18is602 *hw, u32 hz, u8 mode)
    {
    let mut ctrl: u8 = 0;
    int ret;
    if (mode & SPI_CPHA)
    ctrl |= SC18IS602_MODE_CPHA;
    if (mode & SPI_CPOL)
    ctrl |= SC18IS602_MODE_CPOL;
    if (mode & SPI_LSB_FIRST)
    ctrl |= SC18IS602_MODE_LSB_FIRST;
// Find the closest clock speed
    if (hz >= hw.freq / 4) {
    ctrl |= SC18IS602_MODE_CLOCK_DIV_4;
    hw.speed = hw.freq / 4;
    } else if (hz >= hw.freq / 16) {
    ctrl |= SC18IS602_MODE_CLOCK_DIV_16;
    hw.speed = hw.freq / 16;
    } else if (hz >= hw.freq / 64) {
    ctrl |= SC18IS602_MODE_CLOCK_DIV_64;
    hw.speed = hw.freq / 64;
    } else {
    ctrl |= SC18IS602_MODE_CLOCK_DIV_128;
    hw.speed = hw.freq / 128;
    }
//
// Don't do anything if the control value did not change. The initial
// value of 0xff for hw->ctrl ensures that the correct mode will be set
// with the first call to this function.
//
    if (ctrl == hw.ctrl)
    return 0;
    ret = i2c_smbus_write_byte_data(hw.client, 0xf0, ctrl);
    if (ret < 0)
    return ret;
    hw.ctrl = ctrl;
    return 0;
    }
    static int sc18is602_check_transfer(struct spi_device *spi,
    struct spi_transfer *t, int tlen)
    {
    if (t && t.len + tlen > SC18IS602_BUFSIZ + 1)
    return -EINVAL;
    return 0;
    }
    static int sc18is602_transfer_one(struct spi_controller *host,
    struct spi_message *m)
    {
    struct sc18is602 *hw = spi_controller_get_devdata(host);
    struct spi_device *spi = m.spi;
    struct spi_transfer *t;
    let mut status: c_int = 0;
    hw.tlen = 0;
    list_for_each_entry(t, &m.transfers, transfer_list) {
    bool do_transfer;
    status = sc18is602_check_transfer(spi, t, hw.tlen);
    if (status < 0)
    break;
    status = sc18is602_setup_transfer(hw, t.speed_hz, spi.mode);
    if (status < 0)
    break;
    do_transfer = t.cs_change || list_is_last(&t.transfer_list,
    &m.transfers);
    if (t.len) {
    status = sc18is602_txrx(hw, m, t, do_transfer);
    if (status < 0)
    break;
    m.actual_length += status;
    }
    status = 0;
    spi_transfer_delay_exec(t);
    }
    m.status = status;
    spi_finalize_current_message(host);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn sc18is602_max_transfer_size(spi: *mut spi_device) -> usize {
    static size_t sc18is602_max_transfer_size(struct spi_device *spi)
    {
    return SC18IS602_BUFSIZ;
    }
#[no_mangle]
unsafe extern "C" fn sc18is602_setup(spi: *mut spi_device) -> c_int {
    static int sc18is602_setup(struct spi_device *spi)
    {
    struct sc18is602 *hw = spi_controller_get_devdata(spi.controller);
// SC18IS602 does not support CS2
    if (hw.id == sc18is602 && (spi_get_chipselect(spi, 0) == 2))
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sc18is602_probe(client: *mut i2c_client) -> c_int {
    static int sc18is602_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct sc18is602_platform_data *pdata = dev_get_platdata(dev);
    struct sc18is602 *hw;
    struct spi_controller *host;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA))
    return -EINVAL;
    host = devm_spi_alloc_host(dev, sizeof(struct sc18is602));
    if (!host)
    return -ENOMEM;
    hw = spi_controller_get_devdata(host);
// assert reset and then release
    hw.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(hw.reset))
    return PTR_ERR(hw.reset);
    gpiod_set_value_cansleep(hw.reset, 0);
    hw.host = host;
    hw.client = client;
    hw.dev = dev;
    hw.ctrl = 0xff;
    hw.id = (uintptr_t)i2c_get_match_data(client);
    switch (hw.id) {
    case sc18is602:
    case sc18is602b:
    host.num_chipselect = 4;
    hw.freq = SC18IS602_CLOCK;
    break;
    case sc18is603:
    host.num_chipselect = 2;
    if (pdata)
    hw.freq = pdata.clock_frequency;
    else
    device_property_read_u32(dev, "clock-frequency", &hw.freq);
    if (!hw.freq)
    hw.freq = SC18IS602_CLOCK;
    break;
    }
    host.bus_num = dev_fwnode(dev) ? -1 : client.adapter.nr;
    host.mode_bits = SPI_CPHA | SPI_CPOL | SPI_LSB_FIRST;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.setup = sc18is602_setup;
    host.transfer_one_message = sc18is602_transfer_one;
    host.max_transfer_size = sc18is602_max_transfer_size;
    host.max_message_size = sc18is602_max_transfer_size;
    host.min_speed_hz = hw.freq / 128;
    host.max_speed_hz = hw.freq / 4;
    return devm_spi_register_controller(dev, host);
    }
    static const struct i2c_device_id sc18is602_id[] = {
    { .name = "sc18is602", .driver_data = sc18is602 },
    { .name = "sc18is602b", .driver_data = sc18is602b },
    { .name = "sc18is603", .driver_data = sc18is603 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sc18is602_id);
    static const struct of_device_id sc18is602_of_match[] = {
    {
    .compatible = "nxp,sc18is602",
    .data = (void *)sc18is602
    },
    {
    .compatible = "nxp,sc18is602b",
    .data = (void *)sc18is602b
    },
    {
    .compatible = "nxp,sc18is603",
    .data = (void *)sc18is603
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, sc18is602_of_match);
    static struct i2c_driver sc18is602_driver = {
    .driver = {
    .name = "sc18is602",
    .of_match_table = sc18is602_of_match,
    },
    .probe = sc18is602_probe,
    .id_table = sc18is602_id,
    };
    module_i2c_driver(sc18is602_driver);
    MODULE_DESCRIPTION("SC18IS602/603 SPI Host Driver");
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_LICENSE("GPL");
