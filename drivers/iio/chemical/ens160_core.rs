//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/ens160_core.c
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
// ScioSense ENS160 multi-gas sensor driver
//
// Copyright (c) 2024 Gustavo Silva <gustavograzs@gmail.com>
//
// Datasheet:
// https://www.sciosense.com/wp-content/uploads/2023/12/ENS160-Datasheet.pdf
//

pub const ENS160_PART_ID: c_uint = 0x160;

pub const ENS160_REG_PART_ID: c_uint = 0x00;
pub const ENS160_REG_OPMODE: c_uint = 0x10;
pub const ENS160_REG_CONFIG: c_uint = 0x11;

pub const ENS160_REG_MODE_DEEP_SLEEP: c_uint = 0x00;
pub const ENS160_REG_MODE_IDLE: c_uint = 0x01;
pub const ENS160_REG_MODE_STANDARD: c_uint = 0x02;
pub const ENS160_REG_MODE_RESET: c_uint = 0xF0;
pub const ENS160_REG_COMMAND: c_uint = 0x12;
pub const ENS160_REG_COMMAND_GET_APPVER: c_uint = 0x0E;
pub const ENS160_REG_COMMAND_CLRGPR: c_uint = 0xCC;
pub const ENS160_REG_TEMP_IN: c_uint = 0x13;
pub const ENS160_REG_RH_IN: c_uint = 0x15;
pub const ENS160_REG_DEVICE_STATUS: c_uint = 0x20;
pub const ENS160_REG_DATA_AQI: c_uint = 0x21;
pub const ENS160_REG_DATA_TVOC: c_uint = 0x22;
pub const ENS160_REG_DATA_ECO2: c_uint = 0x24;
pub const ENS160_REG_DATA_T: c_uint = 0x30;
pub const ENS160_REG_DATA_RH: c_uint = 0x32;
pub const ENS160_REG_GPR_READ4: c_uint = 0x4C;

pub const ENS160_STATUS_NORMAL: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ens160_data {
    pub regmap: *mut regmap,
// Protect reads from the sensor
    pub mutex: mutex,
    struct {
    pub chans: [__le16; 2],
    pub timestamp: aligned_s64,
    pub __aligned(IIO_DMA_MINALIGN): } scan,
    pub fw_version: [u8; 3],
    pub buf: __le16,
}

    static const struct iio_chan_spec ens160_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .address = ENS160_REG_DATA_TVOC,
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_LE,
    },
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .address = ENS160_REG_DATA_ECO2,
    .scan_index = 1,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_LE,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(2),
    };
    static int __ens160_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val)
    {
    struct ens160_data *data = iio_priv(indio_dev);
    int ret;
    guard(mutex)(&data.mutex);
    ret = regmap_bulk_read(data.regmap, chan.address,
    &data.buf, sizeof(data.buf));
    if (ret)
    return ret;
// val = le16_to_cpu(data->buf);
    return IIO_VAL_INT;
    }
    static int ens160_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __ens160_read_raw(indio_dev, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.channel2) {
    case IIO_MOD_CO2:
// The sensor reads CO2 data as ppm
// val = 0;
// val2 = 100;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_MOD_VOC:
// The sensor reads VOC data as ppb
// val = 0;
// val2 = 100;
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ens160_set_mode(data: *mut ens160_data, mode: u8) -> c_int {
    static int ens160_set_mode(struct ens160_data *data, u8 mode)
    {
    int ret;
    ret = regmap_write(data.regmap, ENS160_REG_OPMODE, mode);
    if (ret)
    return ret;
    msleep(ENS160_BOOTING_TIME_MS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ens160_set_idle(data: *mut c_void) {
    static void ens160_set_idle(void *data)
    {
    ens160_set_mode(data, ENS160_REG_MODE_IDLE);
    }
#[no_mangle]
unsafe extern "C" fn ens160_chip_init(data: *mut ens160_data) -> c_int {
    static int ens160_chip_init(struct ens160_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    unsigned int status;
    int ret;
    ret = ens160_set_mode(data, ENS160_REG_MODE_RESET);
    if (ret)
    return ret;
    ret = regmap_bulk_read(data.regmap, ENS160_REG_PART_ID, &data.buf,
    sizeof(data.buf));
    if (ret)
    return ret;
    if (le16_to_cpu(data.buf) != ENS160_PART_ID)
    return -ENODEV;
    ret = ens160_set_mode(data, ENS160_REG_MODE_IDLE);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, ENS160_REG_COMMAND,
    ENS160_REG_COMMAND_CLRGPR);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, ENS160_REG_COMMAND,
    ENS160_REG_COMMAND_GET_APPVER);
    if (ret)
    return ret;
    ret = regmap_bulk_read(data.regmap, ENS160_REG_GPR_READ4,
    data.fw_version, sizeof(data.fw_version));
    if (ret)
    return ret;
    dev_info(dev, "firmware version: %u.%u.%u\n", data.fw_version[2],
    data.fw_version[1], data.fw_version[0]);
    ret = ens160_set_mode(data, ENS160_REG_MODE_STANDARD);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, ens160_set_idle, data);
    if (ret)
    return ret;
    ret = regmap_read(data.regmap, ENS160_REG_DEVICE_STATUS, &status);
    if (ret)
    return ret;
    if (FIELD_GET(ENS160_STATUS_VALIDITY_FLAG, status)
    != ENS160_STATUS_NORMAL)
    return -EINVAL;
    return 0;
    }
    static const struct iio_info ens160_info = {
    .read_raw = ens160_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ens160_suspend(dev: *mut device) -> c_int {
    static int ens160_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct ens160_data *data = iio_priv(indio_dev);
    return ens160_set_mode(data, ENS160_REG_MODE_DEEP_SLEEP);
    }
#[no_mangle]
unsafe extern "C" fn ens160_resume(dev: *mut device) -> c_int {
    static int ens160_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct ens160_data *data = iio_priv(indio_dev);
    int ret;
    ret = ens160_set_mode(data, ENS160_REG_MODE_IDLE);
    if (ret)
    return ret;
    return ens160_set_mode(data, ENS160_REG_MODE_STANDARD);
    }
    EXPORT_NS_SIMPLE_DEV_PM_OPS(ens160_pm_ops, ens160_suspend, ens160_resume,
    IIO_ENS160);
#[no_mangle]
unsafe extern "C" fn ens160_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ens160_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ens160_data *data = iio_priv(indio_dev);
    int ret;
    guard(mutex)(&data.mutex);
    ret = regmap_bulk_read(data.regmap, ENS160_REG_DATA_TVOC,
    data.scan.chans, sizeof(data.scan.chans));
    if (ret)
    goto err;
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    pf.timestamp);
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ens160_set_trigger_state(trig: *mut iio_trigger, state: bool) -> c_int {
    static int ens160_set_trigger_state(struct iio_trigger *trig, bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct ens160_data *data = iio_priv(indio_dev);
    unsigned int int_bits = ENS160_REG_CONFIG_INTEN |
    ENS160_REG_CONFIG_INTDAT |
    ENS160_REG_CONFIG_INT_CFG;
    if (state)
    return regmap_set_bits(data.regmap, ENS160_REG_CONFIG,
    int_bits);
    else
    return regmap_clear_bits(data.regmap, ENS160_REG_CONFIG,
    int_bits);
    }
    static const struct iio_trigger_ops ens160_trigger_ops = {
    .set_trigger_state = ens160_set_trigger_state,
    .validate_device = iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn ens160_setup_trigger(indio_dev: *mut iio_dev, irq: c_int) -> c_int {
    static int ens160_setup_trigger(struct iio_dev *indio_dev, int irq)
    {
    struct device *dev = indio_dev.dev.parent;
    struct iio_trigger *trig;
    int ret;
    trig = devm_iio_trigger_alloc(dev, "%s-dev%d", indio_dev.name,
    iio_device_id(indio_dev));
    if (!trig)
    return -ENOMEM;
    trig.ops = &ens160_trigger_ops;
    iio_trigger_set_drvdata(trig, indio_dev);
    ret = devm_iio_trigger_register(dev, trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(trig);
    ret = devm_request_irq(dev, irq, iio_trigger_generic_data_rdy_poll,
    IRQF_NO_THREAD, indio_dev.name,
    indio_dev.trig);
    if (ret)
    return ret;
    return 0;
    }
    int devm_ens160_core_probe(struct device *dev, struct regmap *regmap, int irq,
    const char *name)
    {
    struct ens160_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.regmap = regmap;
    indio_dev.name = name;
    indio_dev.info = &ens160_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ens160_channels;
    indio_dev.num_channels = ARRAY_SIZE(ens160_channels);
    if (irq > 0) {
    ret = ens160_setup_trigger(indio_dev, irq);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to setup trigger\n");
    }
    ret = ens160_chip_init(data);
    if (ret)
    return dev_err_probe(dev, ret, "chip initialization failed\n");
    mutex_init(&data.mutex);
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    iio_pollfunc_store_time,
    ens160_trigger_handler, core::ptr::null_mut());
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
    EXPORT_SYMBOL_NS(devm_ens160_core_probe, "IIO_ENS160");
    MODULE_AUTHOR("Gustavo Silva <gustavograzs@gmail.com>");
    MODULE_DESCRIPTION("ScioSense ENS160 driver");
    MODULE_LICENSE("GPL v2");
