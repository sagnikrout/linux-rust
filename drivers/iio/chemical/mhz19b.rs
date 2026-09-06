//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/mhz19b.c
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
// mh-z19b CO₂ sensor driver
//
// Copyright (c) 2025 Gyeyoung Baek <gye976@gmail.com>
//
// Datasheet:
// https://www.winsen-sensor.com/d/files/infrared-gas-sensor/mh-z19b-co2-ver1_0.pdf
//

//
// Commands have following format:
//
// +------+------+-----+------+------+------+------+------+-------+
// | 0xFF | 0x01 | cmd | arg0 | arg1 | 0x00 | 0x00 | 0x00 | cksum |
// +------+------+-----+------+------+------+------+------+-------+
//
pub const MHZ19B_CMD_SIZE: c_int = 9;
// ABC logic in MHZ19B means auto calibration.
pub const MHZ19B_ABC_LOGIC_CMD: c_uint = 0x79;
pub const MHZ19B_READ_CO2_CMD: c_uint = 0x86;
pub const MHZ19B_SPAN_POINT_CMD: c_uint = 0x88;
pub const MHZ19B_ZERO_POINT_CMD: c_uint = 0x87;
pub const MHZ19B_SPAN_POINT_PPM_MIN: c_int = 1000;
pub const MHZ19B_SPAN_POINT_PPM_MAX: c_int = 5000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhz19b_state {
    pub serdev: *mut serdev_device,
// Must wait until the 'buf' is filled with 9 bytes.
    pub buf_ready: completion,
    pub buf_idx: u8,
    pub buf_overflow: bool,
//
// Serdev receive buffer.
// When data is received from the MH-Z19B,
// the 'mhz19b_receive_buf' callback function is called and fills this buffer.
//
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[MHZ19B_CMD_SIZE],
}

#[no_mangle]
unsafe extern "C" fn mhz19b_get_checksum(cmd_buf: *mut u8) -> u8 {
    static u8 mhz19b_get_checksum(u8 *cmd_buf)
    {
    u8 i, checksum = 0;
//
// +------+------+-----+------+------+------+------+------+-------+
// | 0xFF | 0x01 | cmd | arg0 | arg1 | 0x00 | 0x00 | 0x00 | cksum |
// +------+------+-----+------+------+------+------+------+-------+
// i:1    2      3      4      5      6      7
//
// Sum all cmd_buf elements from index 1 to 7.
//
    for (i = 1; i < 8; i++)
    checksum += cmd_buf[i];
    return -checksum;
    }
#[no_mangle]
unsafe extern "C" fn mhz19b_serdev_cmd(indio_dev: *mut iio_dev, cmd: c_int, arg: u16) -> c_int {
    static int mhz19b_serdev_cmd(struct iio_dev *indio_dev, int cmd, u16 arg)
    {
    struct mhz19b_state *st = iio_priv(indio_dev);
    struct serdev_device *serdev = st.serdev;
    struct device *dev = &indio_dev.dev;
    int ret;
//
// cmd_buf[3,4] : arg0,1
// cmd_buf[8]	: checksum
//
    u8 cmd_buf[MHZ19B_CMD_SIZE] = {
    0xFF, 0x01, cmd,
    };
    switch (cmd) {
    case MHZ19B_ABC_LOGIC_CMD:
    cmd_buf[3] = arg ? 0xA0 : 0;
    break;
    case MHZ19B_SPAN_POINT_CMD:
    put_unaligned_be16(arg, &cmd_buf[3]);
    break;
    default:
    break;
    }
    cmd_buf[8] = mhz19b_get_checksum(cmd_buf);
// Write buf to uart ctrl synchronously
    st.buf_idx = 0;
    st.buf_overflow = false;
    reinit_completion(&st.buf_ready);
    ret = serdev_device_write(serdev, cmd_buf, MHZ19B_CMD_SIZE, 0);
    if (ret < 0)
    return ret;
    if (ret != MHZ19B_CMD_SIZE)
    return -EIO;
    switch (cmd) {
    case MHZ19B_READ_CO2_CMD:
    ret = wait_for_completion_interruptible_timeout(&st.buf_ready,
    MHZ19B_SERDEV_TIMEOUT);
    if (ret < 0)
    return ret;
    if (!ret)
    return -ETIMEDOUT;
    if (st.buf_overflow)
    return -EMSGSIZE;
    if (st.buf[8] != mhz19b_get_checksum(st.buf)) {
    dev_err(dev, "checksum err");
    return -EINVAL;
    }
    return get_unaligned_be16(&st.buf[2]);
    default:
// No response commands.
    return 0;
    }
    }
    static int mhz19b_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    ret = mhz19b_serdev_cmd(indio_dev, MHZ19B_READ_CO2_CMD, 0);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    }
//
// echo 0 > calibration_auto_enable : ABC logic off
// echo 1 > calibration_auto_enable : ABC logic on
//
    static ssize_t calibration_auto_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    bool enable;
    int ret;
    ret = kstrtobool(buf, &enable);
    if (ret)
    return ret;
    ret = mhz19b_serdev_cmd(indio_dev, MHZ19B_ABC_LOGIC_CMD, enable);
    if (ret < 0)
    return ret;
    return len;
    }
    static IIO_DEVICE_ATTR_WO(calibration_auto_enable, 0);
//
// echo 0 > calibration_forced_value		 : zero point calibration
// (make sure the sensor has been working under 400ppm for over 20 minutes.)
// echo [1000 1 5000] > calibration_forced_value : span point calibration
// (make sure the sensor has been working under a certain level CO₂ for over 20 minutes.)
//
    static ssize_t calibration_forced_value_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    u16 ppm;
    int cmd, ret;
    ret = kstrtou16(buf, 0, &ppm);
    if (ret)
    return ret;
    if (ppm) {
    if (!in_range(ppm, MHZ19B_SPAN_POINT_PPM_MIN,
    MHZ19B_SPAN_POINT_PPM_MAX - MHZ19B_SPAN_POINT_PPM_MIN + 1)) {
    dev_dbg(&indio_dev.dev,
    "span point ppm should be in a range [%d-%d]\n",
    MHZ19B_SPAN_POINT_PPM_MIN, MHZ19B_SPAN_POINT_PPM_MAX);
    return -EINVAL;
    }
    cmd = MHZ19B_SPAN_POINT_CMD;
    } else {
    cmd = MHZ19B_ZERO_POINT_CMD;
    }
    ret = mhz19b_serdev_cmd(indio_dev, cmd, ppm);
    if (ret < 0)
    return ret;
    return len;
    }
    static IIO_DEVICE_ATTR_WO(calibration_forced_value, 0);
    static struct attribute *mhz19b_attrs[] = {
    &iio_dev_attr_calibration_auto_enable.dev_attr.attr,
    &iio_dev_attr_calibration_forced_value.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group mhz19b_attr_group = {
    .attrs = mhz19b_attrs,
    };
    static const struct iio_info mhz19b_info = {
    .attrs = &mhz19b_attr_group,
    .read_raw = mhz19b_read_raw,
    };
    static const struct iio_chan_spec mhz19b_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    },
    };
    static size_t mhz19b_receive_buf(struct serdev_device *serdev,
    const u8 *data, size_t len)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(&serdev.dev);
    struct mhz19b_state *st = iio_priv(indio_dev);
    let mut remaining: usize = MHZ19B_CMD_SIZE - st.buf_idx;
    if (len > remaining) {
    st.buf_idx = 0;
    st.buf_overflow = true;
    complete(&st.buf_ready);
    return len;
    }
    memcpy(st.buf + st.buf_idx, data, len);
    st.buf_idx += len;
    if (st.buf_idx == MHZ19B_CMD_SIZE) {
    st.buf_idx = 0;
    complete(&st.buf_ready);
    }
    return len;
    }
    static const struct serdev_device_ops mhz19b_ops = {
    .receive_buf = mhz19b_receive_buf,
    .write_wakeup = serdev_device_write_wakeup,
    };
#[no_mangle]
unsafe extern "C" fn mhz19b_probe(serdev: *mut serdev_device) -> c_int {
    static int mhz19b_probe(struct serdev_device *serdev)
    {
    int ret;
    struct device *dev = &serdev.dev;
    struct iio_dev *indio_dev;
    struct mhz19b_state *st;
    serdev_device_set_client_ops(serdev, &mhz19b_ops);
    ret = devm_serdev_device_open(dev, serdev);
    if (ret)
    return ret;
    serdev_device_set_baudrate(serdev, 9600);
    serdev_device_set_flow_control(serdev, false);
    ret = serdev_device_set_parity(serdev, SERDEV_PARITY_NONE);
    if (ret)
    return ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    serdev_device_set_drvdata(serdev, indio_dev);
    st = iio_priv(indio_dev);
    st.serdev = serdev;
    init_completion(&st.buf_ready);
    ret = devm_regulator_get_enable(dev, "vin");
    if (ret)
    return ret;
    indio_dev.name = "mh-z19b";
    indio_dev.channels = mhz19b_channels;
    indio_dev.num_channels = ARRAY_SIZE(mhz19b_channels);
    indio_dev.info = &mhz19b_info;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id mhz19b_of_match[] = {
    { .compatible = "winsen,mhz19b", },
    { }
    };
    MODULE_DEVICE_TABLE(of, mhz19b_of_match);
    static struct serdev_device_driver mhz19b_driver = {
    .driver = {
    .name = "mhz19b",
    .of_match_table = mhz19b_of_match,
    },
    .probe = mhz19b_probe,
    };
    module_serdev_device_driver(mhz19b_driver);
    MODULE_AUTHOR("Gyeyoung Baek");
    MODULE_DESCRIPTION("MH-Z19B CO2 sensor driver using serdev interface");
    MODULE_LICENSE("GPL");
