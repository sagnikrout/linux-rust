//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ti-adc161s626.c
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
// ti-adc161s626.c - Texas Instruments ADC161S626 1-channel differential ADC
//
// ADC Devices Supported:
// adc141s626 - 14-bit ADC
// adc161s626 - 16-bit ADC
//
// Copyright (C) 2016-2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

    enum {
    TI_ADC141S626,
    TI_ADC161S626,
    };
    static const struct iio_chan_spec ti_adc141s626_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 14,
    .storagebits = 16,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static const struct iio_chan_spec ti_adc161s626_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adc_data {
    pub indio_dev: *mut iio_dev,
    pub spi: *mut spi_device,
    pub ref: *mut regulator,
    pub read_size: u8,
    pub shift: u8,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[3],
}

    static int ti_adc_read_measurement(struct ti_adc_data *data,
    struct iio_chan_spec const *chan, int *val)
    {
    int ret;
    switch (data.read_size) {
    case 2:
    ret = spi_read(data.spi, data.buf, 2);
    if (ret)
    return ret;
// val = get_unaligned_be16(data->buf);
    break;
    case 3:
    ret = spi_read(data.spi, data.buf, 3);
    if (ret)
    return ret;
// val = get_unaligned_be24(data->buf);
    break;
    default:
    return -EINVAL;
    }
// val = sign_extend32(*val >> data->shift, chan->scan_type.realbits - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_adc_trigger_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t ti_adc_trigger_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ti_adc_data *data = iio_priv(indio_dev);
    struct {
    s16 data;
    aligned_s64 timestamp;
    } scan = { };
    int ret, val;
    ret = ti_adc_read_measurement(data, &indio_dev.channels[0], &val);
    if (ret)
    goto exit_notify_done;
    scan.data = val;
    iio_push_to_buffers_with_timestamp(indio_dev, &scan, iio_get_time_ns(indio_dev));
    exit_notify_done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int ti_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ti_adc_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ti_adc_read_measurement(data, chan, val);
    iio_device_release_direct(indio_dev);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = regulator_get_voltage(data.ref);
    if (ret < 0)
    return ret;
// val = ret / 1000;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_OFFSET:
// val = 1 << (chan->scan_type.realbits - 1);
    return IIO_VAL_INT;
    }
    return 0;
    }
    static const struct iio_info ti_adc_info = {
    .read_raw = ti_adc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ti_adc_reg_disable(reg: *mut c_void) {
    static void ti_adc_reg_disable(void *reg)
    {
    regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn ti_adc_probe(spi: *mut spi_device) -> c_int {
    static int ti_adc_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ti_adc_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    indio_dev.info = &ti_adc_info;
    indio_dev.name = TI_ADC_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    data = iio_priv(indio_dev);
    data.spi = spi;
    switch (spi_get_device_id(spi).driver_data) {
    case TI_ADC141S626:
    indio_dev.channels = ti_adc141s626_channels;
    indio_dev.num_channels = ARRAY_SIZE(ti_adc141s626_channels);
    data.shift = 0;
    data.read_size = 2;
    break;
    case TI_ADC161S626:
    indio_dev.channels = ti_adc161s626_channels;
    indio_dev.num_channels = ARRAY_SIZE(ti_adc161s626_channels);
    data.shift = 6;
    data.read_size = 3;
    break;
    }
    data.ref = devm_regulator_get(&spi.dev, "vdda");
    if (IS_ERR(data.ref))
    return PTR_ERR(data.ref);
    ret = regulator_enable(data.ref);
    if (ret < 0)
    return ret;
    ret = devm_add_action_or_reset(&spi.dev, ti_adc_reg_disable,
    data.ref);
    if (ret)
    return ret;
    ret = devm_iio_triggered_buffer_setup(&spi.dev, indio_dev, core::ptr::null_mut(),
    ti_adc_trigger_handler, core::ptr::null_mut());
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct of_device_id ti_adc_dt_ids[] = {
    { .compatible = "ti,adc141s626", },
    { .compatible = "ti,adc161s626", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ti_adc_dt_ids);
    static const struct spi_device_id ti_adc_id[] = {
    { .name = "adc141s626", .driver_data = TI_ADC141S626 },
    { .name = "adc161s626", .driver_data = TI_ADC161S626 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ti_adc_id);
    static struct spi_driver ti_adc_driver = {
    .driver = {
    .name	= TI_ADC_DRV_NAME,
    .of_match_table = ti_adc_dt_ids,
    },
    .probe		= ti_adc_probe,
    .id_table	= ti_adc_id,
    };
    module_spi_driver(ti_adc_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("Texas Instruments ADC1x1S 1-channel differential ADC");
    MODULE_LICENSE("GPL");
