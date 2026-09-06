//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/sps30_serial.c
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
// Sensirion SPS30 particulate matter sensor serial driver
//
// Copyright (c) 2021 Tomasz Duszynski <tduszyns@gmail.com>
//

pub const SPS30_SERIAL_SOF_EOF: c_uint = 0x7e;

pub const SPS30_SERIAL_MAX_BUF_SIZE: c_int = 263;
pub const SPS30_SERIAL_ESCAPE_CHAR: c_uint = 0x7d;
pub const SPS30_SERIAL_FRAME_MIN_SIZE: c_int = 7;
pub const SPS30_SERIAL_FRAME_ADR_OFFSET: c_int = 1;
pub const SPS30_SERIAL_FRAME_CMD_OFFSET: c_int = 2;
pub const SPS30_SERIAL_FRAME_MOSI_LEN_OFFSET: c_int = 3;
pub const SPS30_SERIAL_FRAME_MISO_STATE_OFFSET: c_int = 3;
pub const SPS30_SERIAL_FRAME_MISO_LEN_OFFSET: c_int = 4;
pub const SPS30_SERIAL_FRAME_MISO_DATA_OFFSET: c_int = 5;
pub const SPS30_SERIAL_START_MEAS: c_uint = 0x00;
pub const SPS30_SERIAL_STOP_MEAS: c_uint = 0x01;
pub const SPS30_SERIAL_READ_MEAS: c_uint = 0x03;
pub const SPS30_SERIAL_RESET: c_uint = 0xd3;
pub const SPS30_SERIAL_CLEAN_FAN: c_uint = 0x56;
pub const SPS30_SERIAL_PERIOD: c_uint = 0x80;
pub const SPS30_SERIAL_DEV_INFO: c_uint = 0xd0;
pub const SPS30_SERIAL_READ_VERSION: c_uint = 0xd1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sps30_serial_priv {
    pub new_frame: completion,
    pub buf: [c_uchar; SPS30_SERIAL_MAX_BUF_SIZE],
    pub num: usize,
    pub escaped: bool,
    pub done: bool,
}

#[no_mangle]
unsafe extern "C" fn sps30_serial_xfer(state: *mut sps30_state, buf: *const c_uchar, size: usize) -> c_int {
    static int sps30_serial_xfer(struct sps30_state *state, const unsigned char *buf, size_t size)
    {
    struct serdev_device *serdev = to_serdev_device(state.dev);
    struct sps30_serial_priv *priv = state.priv;
    int ret;
    priv.num = 0;
    priv.escaped = false;
    priv.done = false;
    ret = serdev_device_write(serdev, buf, size, SPS30_SERIAL_TIMEOUT);
    if (ret < 0)
    return ret;
    if (ret != size)
    return -EIO;
    ret = wait_for_completion_interruptible_timeout(&priv.new_frame, SPS30_SERIAL_TIMEOUT);
    if (ret < 0)
    return ret;
    if (!ret)
    return -ETIMEDOUT;
    return 0;
    }
    static const struct {
    u8 byte;
    u8 byte2;
    } sps30_serial_bytes[] = {
    { 0x11, 0x31 },
    { 0x13, 0x33 },
    { 0x7e, 0x5e },
    { 0x7d, 0x5d },
    };
#[no_mangle]
unsafe extern "C" fn sps30_serial_put_byte(buf: *mut u8, byte: u8) -> c_int {
    static int sps30_serial_put_byte(u8 *buf, u8 byte)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(sps30_serial_bytes); i++) {
    if (sps30_serial_bytes[i].byte != byte)
    continue;
    buf[0] = SPS30_SERIAL_ESCAPE_CHAR;
    buf[1] = sps30_serial_bytes[i].byte2;
    return 2;
    }
    buf[0] = byte;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_get_byte(escaped: bool, byte2: u8) -> u8 {
    static u8 sps30_serial_get_byte(bool escaped, u8 byte2)
    {
    int i;
    if (!escaped)
    return byte2;
    for (i = 0; i < ARRAY_SIZE(sps30_serial_bytes); i++) {
    if (sps30_serial_bytes[i].byte2 != byte2)
    continue;
    return sps30_serial_bytes[i].byte;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_calc_chksum(buf: *const c_uchar, num: usize) -> c_uchar {
    static unsigned char sps30_serial_calc_chksum(const unsigned char *buf, size_t num)
    {
    let mut chksum: c_uint = 0;
    size_t i;
    for (i = 0; i < num; i++)
    chksum += buf[i];
    return ~chksum;
    }
    static int sps30_serial_prep_frame(u8 *buf, u8 cmd, const u8 *arg,
    size_t arg_size)
    {
    unsigned char chksum;
    let mut num: c_int = 0;
    size_t i;
    buf[num++] = SPS30_SERIAL_SOF_EOF;
    buf[num++] = 0;
    num += sps30_serial_put_byte(buf + num, cmd);
    num += sps30_serial_put_byte(buf + num, arg_size);
    for (i = 0; i < arg_size; i++)
    num += sps30_serial_put_byte(buf + num, arg[i]);
// SOF isn't checksummed
    chksum = sps30_serial_calc_chksum(buf + 1, num - 1);
    num += sps30_serial_put_byte(buf + num, chksum);
    buf[num++] = SPS30_SERIAL_SOF_EOF;
    return num;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_frame_valid(state: *mut sps30_state, buf: *const c_uchar) -> bool {
    static bool sps30_serial_frame_valid(struct sps30_state *state, const unsigned char *buf)
    {
    struct sps30_serial_priv *priv = state.priv;
    unsigned char chksum;
    if ((priv.num < SPS30_SERIAL_FRAME_MIN_SIZE) ||
    (priv.num != SPS30_SERIAL_FRAME_MIN_SIZE +
    priv.buf[SPS30_SERIAL_FRAME_MISO_LEN_OFFSET])) {
    dev_err(state.dev, "frame has invalid number of bytes\n");
    return false;
    }
    if ((priv.buf[SPS30_SERIAL_FRAME_ADR_OFFSET] != buf[SPS30_SERIAL_FRAME_ADR_OFFSET]) ||
    (priv.buf[SPS30_SERIAL_FRAME_CMD_OFFSET] != buf[SPS30_SERIAL_FRAME_CMD_OFFSET])) {
    dev_err(state.dev, "frame has wrong ADR and CMD bytes\n");
    return false;
    }
    if (priv.buf[SPS30_SERIAL_FRAME_MISO_STATE_OFFSET]) {
    dev_err(state.dev, "frame with non-zero state received (0x%02x)\n",
    priv.buf[SPS30_SERIAL_FRAME_MISO_STATE_OFFSET]);
    return false;
    }
// SOF, checksum and EOF are not checksummed
    chksum = sps30_serial_calc_chksum(priv.buf + 1, priv.num - 3);
    if (priv.buf[priv.num - 2] != chksum) {
    dev_err(state.dev, "frame integrity check failed\n");
    return false;
    }
    return true;
    }
    static int sps30_serial_command(struct sps30_state *state, unsigned char cmd,
    const void *arg, size_t arg_size, void *rsp, size_t rsp_size)
    {
    struct sps30_serial_priv *priv = state.priv;
    unsigned char buf[SPS30_SERIAL_MAX_BUF_SIZE];
    int ret, size;
    size = sps30_serial_prep_frame(buf, cmd, arg, arg_size);
    ret = sps30_serial_xfer(state, buf, size);
    if (ret)
    return ret;
    if (!sps30_serial_frame_valid(state, buf))
    return -EIO;
    if (rsp) {
    rsp_size = min_t(size_t, priv.buf[SPS30_SERIAL_FRAME_MISO_LEN_OFFSET], rsp_size);
    memcpy(rsp, &priv.buf[SPS30_SERIAL_FRAME_MISO_DATA_OFFSET], rsp_size);
    }
    return rsp_size;
    }
    static size_t sps30_serial_receive_buf(struct serdev_device *serdev,
    const u8 *buf, size_t size)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(&serdev.dev);
    struct sps30_serial_priv *priv;
    struct sps30_state *state;
    size_t i;
    u8 byte;
    if (!indio_dev)
    return 0;
    state = iio_priv(indio_dev);
    priv = state.priv;
// just in case device put some unexpected data on the bus
    if (priv.done)
    return size;
// wait for the start of frame
    if (!priv.num && size && buf[0] != SPS30_SERIAL_SOF_EOF)
    return 1;
    if (priv.num + size >= ARRAY_SIZE(priv.buf))
    size = ARRAY_SIZE(priv.buf) - priv.num;
    for (i = 0; i < size; i++) {
    byte = buf[i];
// remove stuffed bytes on-the-fly
    if (byte == SPS30_SERIAL_ESCAPE_CHAR) {
    priv.escaped = true;
    continue;
    }
    byte = sps30_serial_get_byte(priv.escaped, byte);
    if (priv.escaped && !byte)
    dev_warn(state.dev, "unrecognized escaped char (0x%02x)\n", byte);
    priv.buf[priv.num++] = byte;
// EOF received
    if (!priv.escaped && byte == SPS30_SERIAL_SOF_EOF) {
    if (priv.num < SPS30_SERIAL_FRAME_MIN_SIZE)
    continue;
    priv.done = true;
    complete(&priv.new_frame);
    i++;
    break;
    }
    priv.escaped = false;
    }
    return i;
    }
    static const struct serdev_device_ops sps30_serial_device_ops = {
    .receive_buf = sps30_serial_receive_buf,
    .write_wakeup = serdev_device_write_wakeup,
    };
#[no_mangle]
unsafe extern "C" fn sps30_serial_start_meas(state: *mut sps30_state) -> c_int {
    static int sps30_serial_start_meas(struct sps30_state *state)
    {
// request BE IEEE754 formatted data
    unsigned char buf[] = { 0x01, 0x03 };
    return sps30_serial_command(state, SPS30_SERIAL_START_MEAS, buf, sizeof(buf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_stop_meas(state: *mut sps30_state) -> c_int {
    static int sps30_serial_stop_meas(struct sps30_state *state)
    {
    return sps30_serial_command(state, SPS30_SERIAL_STOP_MEAS, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_reset(state: *mut sps30_state) -> c_int {
    static int sps30_serial_reset(struct sps30_state *state)
    {
    int ret;
    ret = sps30_serial_command(state, SPS30_SERIAL_RESET, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    msleep(500);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_read_meas(state: *mut sps30_state, meas: *mut __be32, num: usize) -> c_int {
    static int sps30_serial_read_meas(struct sps30_state *state, __be32 *meas, size_t num)
    {
    int ret;
// measurements are ready within a second
    if (msleep_interruptible(1000))
    return -EINTR;
    ret = sps30_serial_command(state, SPS30_SERIAL_READ_MEAS, core::ptr::null_mut(), 0, meas, num * sizeof(*meas));
    if (ret < 0)
    return ret;
// if measurements aren't ready sensor returns empty frame
    if (ret == SPS30_SERIAL_FRAME_MIN_SIZE)
    return -ETIMEDOUT;
    if (ret != num * sizeof(*meas))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_clean_fan(state: *mut sps30_state) -> c_int {
    static int sps30_serial_clean_fan(struct sps30_state *state)
    {
    return sps30_serial_command(state, SPS30_SERIAL_CLEAN_FAN, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_read_cleaning_period(state: *mut sps30_state, period: *mut __be32) -> c_int {
    static int sps30_serial_read_cleaning_period(struct sps30_state *state, __be32 *period)
    {
    unsigned char buf[] = { 0x00 };
    int ret;
    ret = sps30_serial_command(state, SPS30_SERIAL_PERIOD, buf, sizeof(buf),
    period, sizeof(*period));
    if (ret < 0)
    return ret;
    if (ret != sizeof(*period))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_write_cleaning_period(state: *mut sps30_state, period: __be32) -> c_int {
    static int sps30_serial_write_cleaning_period(struct sps30_state *state, __be32 period)
    {
    unsigned char buf[5] = { 0x00 };
    memcpy(buf + 1, &period, sizeof(period));
    return sps30_serial_command(state, SPS30_SERIAL_PERIOD, buf, sizeof(buf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sps30_serial_show_info(state: *mut sps30_state) -> c_int {
    static int sps30_serial_show_info(struct sps30_state *state)
    {
//
// tell device do return serial number and add extra nul byte just in case
// serial number isn't a valid string
//
    unsigned char buf[32 + 1] = { 0x03 };
    struct device *dev = state.dev;
    int ret;
    ret = sps30_serial_command(state, SPS30_SERIAL_DEV_INFO, buf, 1, buf, sizeof(buf) - 1);
    if (ret < 0)
    return ret;
    if (ret != sizeof(buf) - 1)
    return -EIO;
    dev_info(dev, "serial number: %s\n", buf);
    ret = sps30_serial_command(state, SPS30_SERIAL_READ_VERSION, core::ptr::null_mut(), 0, buf, sizeof(buf) - 1);
    if (ret < 0)
    return ret;
    if (ret < 2)
    return -EIO;
    dev_info(dev, "fw version: %u.%u\n", buf[0], buf[1]);
    return 0;
    }
    static const struct sps30_ops sps30_serial_ops = {
    .start_meas = sps30_serial_start_meas,
    .stop_meas = sps30_serial_stop_meas,
    .read_meas = sps30_serial_read_meas,
    .reset = sps30_serial_reset,
    .clean_fan = sps30_serial_clean_fan,
    .read_cleaning_period = sps30_serial_read_cleaning_period,
    .write_cleaning_period = sps30_serial_write_cleaning_period,
    .show_info = sps30_serial_show_info,
    };
#[no_mangle]
unsafe extern "C" fn sps30_serial_probe(serdev: *mut serdev_device) -> c_int {
    static int sps30_serial_probe(struct serdev_device *serdev)
    {
    struct device *dev = &serdev.dev;
    struct sps30_serial_priv *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    init_completion(&priv.new_frame);
    serdev_device_set_client_ops(serdev, &sps30_serial_device_ops);
    ret = devm_serdev_device_open(dev, serdev);
    if (ret)
    return ret;
    serdev_device_set_baudrate(serdev, 115200);
    serdev_device_set_flow_control(serdev, false);
    ret = serdev_device_set_parity(serdev, SERDEV_PARITY_NONE);
    if (ret)
    return ret;
    return sps30_probe(dev, SPS30_SERIAL_DEV_NAME, priv, &sps30_serial_ops);
    }
    static const struct of_device_id sps30_serial_of_match[] = {
    { .compatible = "sensirion,sps30" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sps30_serial_of_match);
    static struct serdev_device_driver sps30_serial_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = sps30_serial_of_match,
    },
    .probe = sps30_serial_probe,
    };
    module_serdev_device_driver(sps30_serial_driver);
    MODULE_AUTHOR("Tomasz Duszynski <tduszyns@gmail.com>");
    MODULE_DESCRIPTION("Sensirion SPS30 particulate matter sensor serial driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_SPS30");
