//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/intel_mrfld_adc.c
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
// ADC driver for Basin Cove PMIC
//
// Copyright (C) 2012 Intel Corporation
// Author: Bin Yang <bin.yang@intel.com>
//
// Rewritten for upstream by:
// Vincent Pelletier <plr.vincent@gmail.com>
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

pub const BCOVE_GPADCREQ: c_uint = 0xDC;

    BCOVE_ADCIRQ_BATTEMP |		\
    BCOVE_ADCIRQ_SYSTEMP |		\
    BCOVE_ADCIRQ_BATTID |		\
    BCOVE_ADCIRQ_VIBATT |		\
    BCOVE_ADCIRQ_CCTICK)

    static const u8 mrfld_adc_requests[] = {
    BCOVE_ADCIRQ_VIBATT,
    BCOVE_ADCIRQ_BATTID,
    BCOVE_ADCIRQ_VIBATT,
    BCOVE_ADCIRQ_SYSTEMP,
    BCOVE_ADCIRQ_BATTEMP,
    BCOVE_ADCIRQ_BATTEMP,
    BCOVE_ADCIRQ_SYSTEMP,
    BCOVE_ADCIRQ_SYSTEMP,
    BCOVE_ADCIRQ_SYSTEMP,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrfld_adc {
    pub regmap: *mut regmap,
    pub completion: completion,
// Lock to protect the IPC transfers
    pub lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn mrfld_adc_thread_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mrfld_adc_thread_isr(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct mrfld_adc *adc = iio_priv(indio_dev);
    complete(&adc.completion);
    return IRQ_HANDLED;
    }
    static int mrfld_adc_single_conv(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *result)
    {
    struct mrfld_adc *adc = iio_priv(indio_dev);
    struct regmap *regmap = adc.regmap;
    unsigned int req;
    long time_left;
    __be16 value;
    int ret;
    reinit_completion(&adc.completion);
    regmap_clear_bits(regmap, BCOVE_MADCIRQ, BCOVE_ADCIRQ_ALL);
    regmap_clear_bits(regmap, BCOVE_MIRQLVL1, BCOVE_LVL1_ADC);
    ret = regmap_read_poll_timeout(regmap, BCOVE_GPADCREQ, req,
    !(req & BCOVE_GPADCREQ_BUSY),
    2000, 1000000);
    if (ret)
    goto done;
    req = mrfld_adc_requests[chan.channel];
    ret = regmap_write(regmap, BCOVE_GPADCREQ, BCOVE_GPADCREQ_IRQEN | req);
    if (ret)
    goto done;
    time_left = wait_for_completion_interruptible_timeout(&adc.completion,
    BCOVE_ADC_TIMEOUT);
    if (time_left < 0) {
    ret = time_left;
    goto done;
    }
    if (time_left == 0) {
    ret = -ETIMEDOUT;
    goto done;
    }
    ret = regmap_bulk_read(regmap, chan.address, &value, sizeof(value));
    if (ret)
    goto done;
// result = be16_to_cpu(value);
    ret = IIO_VAL_INT;
    done:
    regmap_update_bits(regmap, BCOVE_MIRQLVL1, BCOVE_LVL1_ADC, 0xff);
    regmap_update_bits(regmap, BCOVE_MADCIRQ, BCOVE_ADCIRQ_ALL, 0xff);
    return ret;
    }
    static int mrfld_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct mrfld_adc *adc = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&adc.lock);
    ret = mrfld_adc_single_conv(indio_dev, chan, val);
    mutex_unlock(&adc.lock);
    return ret;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info mrfld_adc_iio_info = {
    .read_raw = &mrfld_adc_read_raw,
    };

    {								\
    .indexed = 1,						\
    .type = _type,						\
    .channel = _channel,					\
    .address = _address,					\
    .datasheet_name = _datasheet_name,			\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    }
    static const struct iio_chan_spec mrfld_adc_channels[] = {
    BCOVE_ADC_CHANNEL(IIO_VOLTAGE,    0, "CH0", 0xE9),
    BCOVE_ADC_CHANNEL(IIO_RESISTANCE, 1, "CH1", 0xEB),
    BCOVE_ADC_CHANNEL(IIO_CURRENT,    2, "CH2", 0xED),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       3, "CH3", 0xCC),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       4, "CH4", 0xC8),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       5, "CH5", 0xCA),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       6, "CH6", 0xC2),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       7, "CH7", 0xC4),
    BCOVE_ADC_CHANNEL(IIO_TEMP,       8, "CH8", 0xC6),
    };
    static const struct iio_map iio_maps[] = {
    IIO_MAP("CH0", "bcove-battery", "VBATRSLT"),
    IIO_MAP("CH1", "bcove-battery", "BATTID"),
    IIO_MAP("CH2", "bcove-battery", "IBATRSLT"),
    IIO_MAP("CH3", "bcove-temp",    "PMICTEMP"),
    IIO_MAP("CH4", "bcove-temp",    "BATTEMP0"),
    IIO_MAP("CH5", "bcove-temp",    "BATTEMP1"),
    IIO_MAP("CH6", "bcove-temp",    "SYSTEMP0"),
    IIO_MAP("CH7", "bcove-temp",    "SYSTEMP1"),
    IIO_MAP("CH8", "bcove-temp",    "SYSTEMP2"),
    { }
    };
#[no_mangle]
unsafe extern "C" fn mrfld_adc_probe(pdev: *mut platform_device) -> c_int {
    static int mrfld_adc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev.parent);
    struct iio_dev *indio_dev;
    struct mrfld_adc *adc;
    int irq;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(struct mrfld_adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    mutex_init(&adc.lock);
    init_completion(&adc.completion);
    adc.regmap = pmic.regmap;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), mrfld_adc_thread_isr,
    IRQF_ONESHOT | IRQF_SHARED, pdev.name,
    indio_dev);
    if (ret)
    return ret;
    indio_dev.name = pdev.name;
    indio_dev.channels = mrfld_adc_channels;
    indio_dev.num_channels = ARRAY_SIZE(mrfld_adc_channels);
    indio_dev.info = &mrfld_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = devm_iio_map_array_register(dev, indio_dev, iio_maps);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct platform_device_id mrfld_adc_id_table[] = {
    { .name = "mrfld_bcove_adc" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, mrfld_adc_id_table);
    static struct platform_driver mrfld_adc_driver = {
    .driver = {
    .name = "mrfld_bcove_adc",
    },
    .probe = mrfld_adc_probe,
    .id_table = mrfld_adc_id_table,
    };
    module_platform_driver(mrfld_adc_driver);
    MODULE_AUTHOR("Bin Yang <bin.yang@intel.com>");
    MODULE_AUTHOR("Vincent Pelletier <plr.vincent@gmail.com>");
    MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
    MODULE_DESCRIPTION("ADC driver for Basin Cove PMIC");
    MODULE_LICENSE("GPL v2");
