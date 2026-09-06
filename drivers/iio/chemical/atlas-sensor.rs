//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/atlas-sensor.c
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
// atlas-sensor.c - Support for Atlas Scientific OEM SM sensors
//
// Copyright (C) 2015-2019 Konsulko Group
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const ATLAS_REG_DEV_TYPE: c_uint = 0x00;
pub const ATLAS_REG_DEV_VERSION: c_uint = 0x01;
pub const ATLAS_REG_INT_CONTROL: c_uint = 0x04;

pub const ATLAS_REG_PWR_CONTROL: c_uint = 0x06;
pub const ATLAS_REG_PH_CALIB_STATUS: c_uint = 0x0d;
pub const ATLAS_REG_PH_CALIB_STATUS_MASK: c_uint = 0x07;

pub const ATLAS_REG_EC_CALIB_STATUS: c_uint = 0x0f;
pub const ATLAS_REG_EC_CALIB_STATUS_MASK: c_uint = 0x0f;

pub const ATLAS_REG_DO_CALIB_STATUS: c_uint = 0x09;
pub const ATLAS_REG_DO_CALIB_STATUS_MASK: c_uint = 0x03;

pub const ATLAS_REG_RTD_DATA: c_uint = 0x0e;
pub const ATLAS_REG_PH_TEMP_DATA: c_uint = 0x0e;
pub const ATLAS_REG_PH_DATA: c_uint = 0x16;
pub const ATLAS_REG_EC_PROBE: c_uint = 0x08;
pub const ATLAS_REG_EC_TEMP_DATA: c_uint = 0x10;
pub const ATLAS_REG_EC_DATA: c_uint = 0x18;
pub const ATLAS_REG_TDS_DATA: c_uint = 0x1c;
pub const ATLAS_REG_PSS_DATA: c_uint = 0x20;
pub const ATLAS_REG_ORP_CALIB_STATUS: c_uint = 0x0d;
pub const ATLAS_REG_ORP_DATA: c_uint = 0x0e;
pub const ATLAS_REG_DO_TEMP_DATA: c_uint = 0x12;
pub const ATLAS_REG_DO_DATA: c_uint = 0x22;
pub const ATLAS_PH_INT_TIME_IN_MS: c_int = 450;
pub const ATLAS_EC_INT_TIME_IN_MS: c_int = 650;
pub const ATLAS_ORP_INT_TIME_IN_MS: c_int = 450;
pub const ATLAS_DO_INT_TIME_IN_MS: c_int = 450;
pub const ATLAS_RTD_INT_TIME_IN_MS: c_int = 450;
    enum {
    ATLAS_PH_SM,
    ATLAS_EC_SM,
    ATLAS_ORP_SM,
    ATLAS_DO_SM,
    ATLAS_RTD_SM,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlas_data {
    pub client: *mut i2c_client,
    pub trig: *mut iio_trigger,
    pub chip: *const atlas_device,
    pub regmap: *mut regmap,
    pub interrupt_enabled: c_uint,
// 96-bit data + 32-bit pad + 64-bit timestamp
    pub __aligned(8): __be32 buffer[6],
}

    static const struct regmap_config atlas_regmap_config = {
    .name = "atlas_regmap",
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn atlas_buffer_num_channels(spec: *const iio_chan_spec) -> c_int {
    static int atlas_buffer_num_channels(const struct iio_chan_spec *spec)
    {
    let mut idx: c_int = 0;
    for (; spec.type != IIO_TIMESTAMP; spec++)
    idx++;
    return idx;
    };
    static const struct iio_chan_spec atlas_ph_channels[] = {
    {
    .type = IIO_PH,
    .address = ATLAS_REG_PH_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    {
    .type = IIO_TEMP,
    .address = ATLAS_REG_PH_TEMP_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .output = 1,
    .scan_index = -1
    },
    };

    {\
    .type = IIO_CONCENTRATION, \
    .indexed = 1, \
    .channel = _idx, \
    .address = _addr, \
    .info_mask_separate = \
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE), \
    .scan_index = _idx + 1, \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 32, \
    .storagebits = 32, \
    .endianness = IIO_BE, \
    }, \
    }
    static const struct iio_chan_spec atlas_ec_channels[] = {
    {
    .type = IIO_ELECTRICALCONDUCTIVITY,
    .address = ATLAS_REG_EC_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    },
    ATLAS_CONCENTRATION_CHANNEL(0, ATLAS_REG_TDS_DATA),
    ATLAS_CONCENTRATION_CHANNEL(1, ATLAS_REG_PSS_DATA),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    {
    .type = IIO_TEMP,
    .address = ATLAS_REG_EC_TEMP_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .output = 1,
    .scan_index = -1
    },
    };
    static const struct iio_chan_spec atlas_orp_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .address = ATLAS_REG_ORP_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static const struct iio_chan_spec atlas_do_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .address = ATLAS_REG_DO_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    {
    .type = IIO_TEMP,
    .address = ATLAS_REG_DO_TEMP_DATA,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    .output = 1,
    .scan_index = -1
    },
    };
    static const struct iio_chan_spec atlas_rtd_channels[] = {
    {
    .type = IIO_TEMP,
    .address = ATLAS_REG_RTD_DATA,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 32,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
#[no_mangle]
unsafe extern "C" fn atlas_check_ph_calibration(data: *mut atlas_data) -> c_int {
    static int atlas_check_ph_calibration(struct atlas_data *data)
    {
    struct device *dev = &data.client.dev;
    int ret;
    unsigned int val;
    ret = regmap_read(data.regmap, ATLAS_REG_PH_CALIB_STATUS, &val);
    if (ret)
    return ret;
    if (!(val & ATLAS_REG_PH_CALIB_STATUS_MASK)) {
    dev_warn(dev, "device has not been calibrated\n");
    return 0;
    }
    if (!(val & ATLAS_REG_PH_CALIB_STATUS_LOW))
    dev_warn(dev, "device missing low point calibration\n");
    if (!(val & ATLAS_REG_PH_CALIB_STATUS_MID))
    dev_warn(dev, "device missing mid point calibration\n");
    if (!(val & ATLAS_REG_PH_CALIB_STATUS_HIGH))
    dev_warn(dev, "device missing high point calibration\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atlas_check_ec_calibration(data: *mut atlas_data) -> c_int {
    static int atlas_check_ec_calibration(struct atlas_data *data)
    {
    struct device *dev = &data.client.dev;
    int ret;
    unsigned int val;
    __be16	rval;
    ret = regmap_bulk_read(data.regmap, ATLAS_REG_EC_PROBE, &rval, 2);
    if (ret)
    return ret;
    val = be16_to_cpu(rval);
    dev_info(dev, "probe set to K = %d.%.2d", val / 100, val % 100);
    ret = regmap_read(data.regmap, ATLAS_REG_EC_CALIB_STATUS, &val);
    if (ret)
    return ret;
    if (!(val & ATLAS_REG_EC_CALIB_STATUS_MASK)) {
    dev_warn(dev, "device has not been calibrated\n");
    return 0;
    }
    if (!(val & ATLAS_REG_EC_CALIB_STATUS_DRY))
    dev_warn(dev, "device missing dry point calibration\n");
    if (val & ATLAS_REG_EC_CALIB_STATUS_SINGLE) {
    dev_warn(dev, "device using single point calibration\n");
    } else {
    if (!(val & ATLAS_REG_EC_CALIB_STATUS_LOW))
    dev_warn(dev, "device missing low point calibration\n");
    if (!(val & ATLAS_REG_EC_CALIB_STATUS_HIGH))
    dev_warn(dev, "device missing high point calibration\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atlas_check_orp_calibration(data: *mut atlas_data) -> c_int {
    static int atlas_check_orp_calibration(struct atlas_data *data)
    {
    struct device *dev = &data.client.dev;
    int ret;
    unsigned int val;
    ret = regmap_read(data.regmap, ATLAS_REG_ORP_CALIB_STATUS, &val);
    if (ret)
    return ret;
    if (!val)
    dev_warn(dev, "device has not been calibrated\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atlas_check_do_calibration(data: *mut atlas_data) -> c_int {
    static int atlas_check_do_calibration(struct atlas_data *data)
    {
    struct device *dev = &data.client.dev;
    int ret;
    unsigned int val;
    ret = regmap_read(data.regmap, ATLAS_REG_DO_CALIB_STATUS, &val);
    if (ret)
    return ret;
    if (!(val & ATLAS_REG_DO_CALIB_STATUS_MASK)) {
    dev_warn(dev, "device has not been calibrated\n");
    return 0;
    }
    if (!(val & ATLAS_REG_DO_CALIB_STATUS_PRESSURE))
    dev_warn(dev, "device missing atmospheric pressure calibration\n");
    if (!(val & ATLAS_REG_DO_CALIB_STATUS_DO))
    dev_warn(dev, "device missing dissolved oxygen calibration\n");
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlas_device {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
    pub data_reg: c_int,
    pub data): *mut *mut int (calibration)(struct atlas_data,
    pub delay: c_int,
}

    static const struct atlas_device atlas_devices[] = {
    [ATLAS_PH_SM] = {
    .channels = atlas_ph_channels,
    .num_channels = 3,
    .data_reg = ATLAS_REG_PH_DATA,
    .calibration = &atlas_check_ph_calibration,
    .delay = ATLAS_PH_INT_TIME_IN_MS,
    },
    [ATLAS_EC_SM] = {
    .channels = atlas_ec_channels,
    .num_channels = 5,
    .data_reg = ATLAS_REG_EC_DATA,
    .calibration = &atlas_check_ec_calibration,
    .delay = ATLAS_EC_INT_TIME_IN_MS,
    },
    [ATLAS_ORP_SM] = {
    .channels = atlas_orp_channels,
    .num_channels = 2,
    .data_reg = ATLAS_REG_ORP_DATA,
    .calibration = &atlas_check_orp_calibration,
    .delay = ATLAS_ORP_INT_TIME_IN_MS,
    },
    [ATLAS_DO_SM] = {
    .channels = atlas_do_channels,
    .num_channels = 3,
    .data_reg = ATLAS_REG_DO_DATA,
    .calibration = &atlas_check_do_calibration,
    .delay = ATLAS_DO_INT_TIME_IN_MS,
    },
    [ATLAS_RTD_SM] = {
    .channels = atlas_rtd_channels,
    .num_channels = 2,
    .data_reg = ATLAS_REG_RTD_DATA,
    .delay = ATLAS_RTD_INT_TIME_IN_MS,
    },
    };
#[no_mangle]
unsafe extern "C" fn atlas_set_powermode(data: *mut atlas_data, on: c_int) -> c_int {
    static int atlas_set_powermode(struct atlas_data *data, int on)
    {
    return regmap_write(data.regmap, ATLAS_REG_PWR_CONTROL, on);
    }
#[no_mangle]
unsafe extern "C" fn atlas_set_interrupt(data: *mut atlas_data, state: bool) -> c_int {
    static int atlas_set_interrupt(struct atlas_data *data, bool state)
    {
    if (!data.interrupt_enabled)
    return 0;
    return regmap_update_bits(data.regmap, ATLAS_REG_INT_CONTROL,
    ATLAS_REG_INT_CONTROL_EN,
    state ? ATLAS_REG_INT_CONTROL_EN : 0);
    }
#[no_mangle]
unsafe extern "C" fn atlas_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int atlas_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct atlas_data *data = iio_priv(indio_dev);
    int ret;
    ret = pm_runtime_resume_and_get(&data.client.dev);
    if (ret)
    return ret;
    ret = atlas_set_interrupt(data, true);
    if (ret)
    pm_runtime_put_autosuspend(&data.client.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atlas_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int atlas_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct atlas_data *data = iio_priv(indio_dev);
    int ret;
    ret = atlas_set_interrupt(data, false);
    if (ret)
    return ret;
    ret = pm_runtime_put_autosuspend(&data.client.dev);
    if (ret)
    return ret;
    return 0;
    }
    static const struct iio_buffer_setup_ops atlas_buffer_setup_ops = {
    .postenable = atlas_buffer_postenable,
    .predisable = atlas_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn atlas_trigger_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t atlas_trigger_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct atlas_data *data = iio_priv(indio_dev);
    let mut channels: c_int = atlas_buffer_num_channels(data.chip.channels);
    int ret;
    ret = regmap_bulk_read(data.regmap, data.chip.data_reg,
    &data.buffer, sizeof(__be32) * channels);
    if (!ret)
    iio_push_to_buffers_with_ts(indio_dev, data.buffer,
    sizeof(data.buffer),
    iio_get_time_ns(indio_dev));
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn atlas_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t atlas_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct atlas_data *data = iio_priv(indio_dev);
    iio_trigger_poll_nested(data.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn atlas_read_measurement(data: *mut atlas_data, reg: c_int, val: *mut __be32) -> c_int {
    static int atlas_read_measurement(struct atlas_data *data, int reg, __be32 *val)
    {
    struct device *dev = &data.client.dev;
    let mut suspended: c_int = pm_runtime_suspended(dev);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    if (suspended)
    msleep(data.chip.delay);
    ret = regmap_bulk_read(data.regmap, reg, val, sizeof(*val));
    pm_runtime_put_autosuspend(dev);
    return ret;
    }
    static int atlas_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct atlas_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_PROCESSED:
    case IIO_CHAN_INFO_RAW: {
    int ret;
    __be32 reg;
    switch (chan.type) {
    case IIO_TEMP:
    ret = regmap_bulk_read(data.regmap, chan.address,
    &reg, sizeof(reg));
    break;
    case IIO_PH:
    case IIO_CONCENTRATION:
    case IIO_ELECTRICALCONDUCTIVITY:
    case IIO_VOLTAGE:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = atlas_read_measurement(data, chan.address, &reg);
    iio_device_release_direct(indio_dev);
    break;
    default:
    ret = -EINVAL;
    }
    if (!ret) {
// val = be32_to_cpu(reg);
    ret = IIO_VAL_INT;
    }
    return ret;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_TEMP:
// val = 10;
    return IIO_VAL_INT;
    case IIO_PH:
// val = 1; /* 0.001
// val2 = 1000;
    break;
    case IIO_ELECTRICALCONDUCTIVITY:
// val = 1; /* 0.00001
// val2 = 100000;
    break;
    case IIO_CONCENTRATION:
// val = 0; /* 0.000000001
// val2 = 1000;
    return IIO_VAL_INT_PLUS_NANO;
    case IIO_VOLTAGE:
// val = 1; /* 0.1
// val2 = 10;
    break;
    default:
    return -EINVAL;
    }
    return IIO_VAL_FRACTIONAL;
    }
    return -EINVAL;
    }
    static int atlas_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct atlas_data *data = iio_priv(indio_dev);
    let mut reg: __be32 = cpu_to_be32(val / 10);
    if (val2 != 0 || val < 0 || val > 20000)
    return -EINVAL;
    if (mask != IIO_CHAN_INFO_RAW || chan.type != IIO_TEMP)
    return -EINVAL;
    return regmap_bulk_write(data.regmap, chan.address,
    &reg, sizeof(reg));
    }
    static const struct iio_info atlas_info = {
    .read_raw = atlas_read_raw,
    .write_raw = atlas_write_raw,
    };
    static const struct i2c_device_id atlas_id[] = {
    { .name = "atlas-ph-sm", .driver_data = (kernel_ulong_t)&atlas_devices[ATLAS_PH_SM] },
    { .name = "atlas-ec-sm", .driver_data = (kernel_ulong_t)&atlas_devices[ATLAS_EC_SM] },
    { .name = "atlas-orp-sm", .driver_data = (kernel_ulong_t)&atlas_devices[ATLAS_ORP_SM] },
    { .name = "atlas-do-sm", .driver_data = (kernel_ulong_t)&atlas_devices[ATLAS_DO_SM] },
    { .name = "atlas-rtd-sm", .driver_data = (kernel_ulong_t)&atlas_devices[ATLAS_RTD_SM] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, atlas_id);
    static const struct of_device_id atlas_dt_ids[] = {
    { .compatible = "atlas,ph-sm", .data = &atlas_devices[ATLAS_PH_SM] },
    { .compatible = "atlas,ec-sm", .data = &atlas_devices[ATLAS_EC_SM] },
    { .compatible = "atlas,orp-sm", .data = &atlas_devices[ATLAS_ORP_SM] },
    { .compatible = "atlas,do-sm", .data = &atlas_devices[ATLAS_DO_SM] },
    { .compatible = "atlas,rtd-sm", .data = &atlas_devices[ATLAS_RTD_SM] },
    { }
    };
    MODULE_DEVICE_TABLE(of, atlas_dt_ids);
#[no_mangle]
unsafe extern "C" fn atlas_probe(client: *mut i2c_client) -> c_int {
    static int atlas_probe(struct i2c_client *client)
    {
    struct atlas_data *data;
    const struct atlas_device *chip;
    struct iio_trigger *trig;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    chip = i2c_get_match_data(client);
    indio_dev.info = &atlas_info;
    indio_dev.name = ATLAS_DRV_NAME;
    indio_dev.channels = chip.channels;
    indio_dev.num_channels = chip.num_channels;
    indio_dev.modes = INDIO_BUFFER_SOFTWARE | INDIO_DIRECT_MODE;
    trig = devm_iio_trigger_alloc(&client.dev, "%s-dev%d",
    indio_dev.name, iio_device_id(indio_dev));
    if (!trig)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    data.trig = trig;
    data.chip = chip;
    iio_trigger_set_drvdata(trig, indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &atlas_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "regmap initialization failed\n");
    return PTR_ERR(data.regmap);
    }
    ret = pm_runtime_set_active(&client.dev);
    if (ret)
    return ret;
    ret = chip.calibration(data);
    if (ret)
    return ret;
    ret = iio_trigger_register(trig);
    if (ret) {
    dev_err(&client.dev, "failed to register trigger\n");
    return ret;
    }
    ret = iio_triggered_buffer_setup(indio_dev, &iio_pollfunc_store_time,
    &atlas_trigger_handler, &atlas_buffer_setup_ops);
    if (ret) {
    dev_err(&client.dev, "cannot setup iio trigger\n");
    goto unregister_trigger;
    }
    if (client.irq > 0) {
// interrupt pin toggles on new conversion
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), atlas_interrupt_handler,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "atlas_irq",
    indio_dev);
    if (ret)
    dev_warn(&client.dev,
    "request irq (%d) failed\n", client.irq);
    else
    data.interrupt_enabled = 1;
    }
    ret = atlas_set_powermode(data, 1);
    if (ret) {
    dev_err(&client.dev, "cannot power device on");
    goto unregister_buffer;
    }
    pm_runtime_enable(&client.dev);
    pm_runtime_set_autosuspend_delay(&client.dev, 2500);
    pm_runtime_use_autosuspend(&client.dev);
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&client.dev, "unable to register device\n");
    goto unregister_pm;
    }
    return 0;
    unregister_pm:
    pm_runtime_disable(&client.dev);
    atlas_set_powermode(data, 0);
    unregister_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    unregister_trigger:
    iio_trigger_unregister(data.trig);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atlas_remove(client: *mut i2c_client) {
    static void atlas_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct atlas_data *data = iio_priv(indio_dev);
    int ret;
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    iio_trigger_unregister(data.trig);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    ret = atlas_set_powermode(data, 0);
    if (ret)
    dev_err(&client.dev, "Failed to power down device (%pe)\n",
    ERR_PTR(ret));
    }
#[no_mangle]
unsafe extern "C" fn atlas_runtime_suspend(dev: *mut device) -> c_int {
    static int atlas_runtime_suspend(struct device *dev)
    {
    struct atlas_data *data =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return atlas_set_powermode(data, 0);
    }
#[no_mangle]
unsafe extern "C" fn atlas_runtime_resume(dev: *mut device) -> c_int {
    static int atlas_runtime_resume(struct device *dev)
    {
    struct atlas_data *data =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return atlas_set_powermode(data, 1);
    }
    static const struct dev_pm_ops atlas_pm_ops = {
    RUNTIME_PM_OPS(atlas_runtime_suspend, atlas_runtime_resume, core::ptr::null_mut())
    };
    static struct i2c_driver atlas_driver = {
    .driver = {
    .name	= ATLAS_DRV_NAME,
    .of_match_table	= atlas_dt_ids,
    .pm	= pm_ptr(&atlas_pm_ops),
    },
    .probe		= atlas_probe,
    .remove		= atlas_remove,
    .id_table	= atlas_id,
    };
    module_i2c_driver(atlas_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("Atlas Scientific SM sensors");
    MODULE_LICENSE("GPL");
