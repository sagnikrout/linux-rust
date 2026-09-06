//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/stmpe-adc.c
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
// STMicroelectronics STMPE811 IIO ADC Driver
//
// 4 channel, 10/12-bit ADC
//
// Copyright (C) 2013-2018 Toradex AG <stefan.agner@toradex.com>
//

pub const STMPE_REG_INT_STA: c_uint = 0x0B;
pub const STMPE_REG_ADC_INT_EN: c_uint = 0x0E;
pub const STMPE_REG_ADC_INT_STA: c_uint = 0x0F;
pub const STMPE_REG_ADC_CTRL1: c_uint = 0x20;
pub const STMPE_REG_ADC_CTRL2: c_uint = 0x21;
pub const STMPE_REG_ADC_CAPT: c_uint = 0x22;

pub const STMPE_REG_TEMP_CTRL: c_uint = 0x60;

    STMPE_TEMP_CTRL_ACQ | \
    STMPE_TEMP_CTRL_THRES_EN)
pub const STMPE_REG_TEMP_DATA: c_uint = 0x61;
pub const STMPE_REG_TEMP_TH: c_uint = 0x63;
pub const STMPE_ADC_LAST_NR: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_adc {
    pub stmpe: *mut stmpe,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub lock: mutex,
// We are allocating plus one for the temperature channel
    pub 2]: iio_chan_spec stmpe_adc_iio_channels[STMPE_ADC_LAST_NR +,
    pub completion: completion,
    pub channel: u8,
    pub value: u32,
}

    static int stmpe_read_voltage(struct stmpe_adc *info,
    struct iio_chan_spec const *chan, int *val)
    {
    unsigned long ret;
    mutex_lock(&info.lock);
    reinit_completion(&info.completion);
    info.channel = (u8)chan.channel;
    if (info.channel > STMPE_ADC_LAST_NR) {
    mutex_unlock(&info.lock);
    return -EINVAL;
    }
    stmpe_reg_write(info.stmpe, STMPE_REG_ADC_CAPT,
    STMPE_ADC_CH(info.channel));
    ret = wait_for_completion_timeout(&info.completion, STMPE_ADC_TIMEOUT);
    if (ret == 0) {
    stmpe_reg_write(info.stmpe, STMPE_REG_ADC_INT_STA,
    STMPE_ADC_CH(info.channel));
    mutex_unlock(&info.lock);
    return -ETIMEDOUT;
    }
// val = info->value;
    mutex_unlock(&info.lock);
    return 0;
    }
    static int stmpe_read_temp(struct stmpe_adc *info,
    struct iio_chan_spec const *chan, int *val)
    {
    unsigned long ret;
    mutex_lock(&info.lock);
    reinit_completion(&info.completion);
    info.channel = (u8)chan.channel;
    if (info.channel != STMPE_TEMP_CHANNEL) {
    mutex_unlock(&info.lock);
    return -EINVAL;
    }
    stmpe_reg_write(info.stmpe, STMPE_REG_TEMP_CTRL,
    STMPE_START_ONE_TEMP_CONV);
    ret = wait_for_completion_timeout(&info.completion, STMPE_ADC_TIMEOUT);
    if (ret == 0) {
    mutex_unlock(&info.lock);
    return -ETIMEDOUT;
    }
//
// absolute temp = +V3.3 * value /7.51 [K]
// scale to [milli °C]
//
// val = ((449960l * info->value) / 1024l) - 273150;
    mutex_unlock(&info.lock);
    return 0;
    }
    static int stmpe_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct stmpe_adc *info = iio_priv(indio_dev);
    long ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    case IIO_CHAN_INFO_PROCESSED:
    switch (chan.type) {
    case IIO_VOLTAGE:
    ret = stmpe_read_voltage(info, chan, val);
    break;
    case IIO_TEMP:
    ret = stmpe_read_temp(info, chan, val);
    break;
    default:
    return -EINVAL;
    }
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 3300;
// val2 = info->stmpe->mod_12b ? 12 : 10;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_adc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmpe_adc_isr(int irq, void *dev_id)
    {
    struct stmpe_adc *info = (struct stmpe_adc *)dev_id;
    __be16 data;
    if (info.channel <= STMPE_ADC_LAST_NR) {
    int int_sta;
    int_sta = stmpe_reg_read(info.stmpe, STMPE_REG_ADC_INT_STA);
// Is the interrupt relevant
    if (!(int_sta & STMPE_ADC_CH(info.channel)))
    return IRQ_NONE;
// Read value
    stmpe_block_read(info.stmpe,
    STMPE_REG_ADC_DATA_CH(info.channel), 2, (u8 *) &data);
    stmpe_reg_write(info.stmpe, STMPE_REG_ADC_INT_STA, int_sta);
    } else if (info.channel == STMPE_TEMP_CHANNEL) {
// Read value
    stmpe_block_read(info.stmpe, STMPE_REG_TEMP_DATA, 2,
    (u8 *) &data);
    } else {
    return IRQ_NONE;
    }
    info.value = (u32) be16_to_cpu(data);
    complete(&info.completion);
    return IRQ_HANDLED;
    }
    static const struct iio_info stmpe_adc_iio_info = {
    .read_raw = &stmpe_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn stmpe_adc_voltage_chan(ics: *mut iio_chan_spec, chan: c_int) {
    static void stmpe_adc_voltage_chan(struct iio_chan_spec *ics, int chan)
    {
    ics.type = IIO_VOLTAGE;
    ics.info_mask_separate = BIT(IIO_CHAN_INFO_RAW);
    ics.info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE);
    ics.indexed = 1;
    ics.channel = chan;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_adc_temp_chan(ics: *mut iio_chan_spec, chan: c_int) {
    static void stmpe_adc_temp_chan(struct iio_chan_spec *ics, int chan)
    {
    ics.type = IIO_TEMP;
    ics.info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED);
    ics.indexed = 1;
    ics.channel = chan;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_adc_init_hw(adc: *mut stmpe_adc) -> c_int {
    static int stmpe_adc_init_hw(struct stmpe_adc *adc)
    {
    int ret;
    struct stmpe *stmpe = adc.stmpe;
    ret = stmpe_enable(stmpe, STMPE_BLOCK_ADC);
    if (ret) {
    dev_err(stmpe.dev, "Could not enable clock for ADC\n");
    return ret;
    }
    ret = stmpe811_adc_common_init(stmpe);
    if (ret) {
    stmpe_disable(stmpe, STMPE_BLOCK_ADC);
    return ret;
    }
// use temp irq for each conversion completion
    stmpe_reg_write(stmpe, STMPE_REG_TEMP_TH, 0);
    stmpe_reg_write(stmpe, STMPE_REG_TEMP_TH + 1, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_adc_probe(pdev: *mut platform_device) -> c_int {
    static int stmpe_adc_probe(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev;
    struct stmpe_adc *info;
    struct device_node *np;
    let mut norequest_mask: u32 = 0;
    unsigned long bits;
    int irq_temp, irq_adc;
    let mut num_chan: c_int = 0;
    let mut i: c_int = 0;
    int ret;
    irq_adc = platform_get_irq_byname(pdev, "STMPE_ADC");
    if (irq_adc < 0)
    return irq_adc;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(struct stmpe_adc));
    if (!indio_dev)
    return -ENOMEM;
    info = iio_priv(indio_dev);
    mutex_init(&info.lock);
    init_completion(&info.completion);
    ret = devm_request_threaded_irq(&pdev.dev, irq_adc, core::ptr::null_mut(),
    stmpe_adc_isr, IRQF_ONESHOT,
    "stmpe-adc", info);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed requesting irq, irq = %d\n",
    irq_adc);
    return ret;
    }
    irq_temp = platform_get_irq_byname(pdev, "STMPE_TEMP_SENS");
    if (irq_temp >= 0) {
    ret = devm_request_threaded_irq(&pdev.dev, irq_temp, core::ptr::null_mut(),
    stmpe_adc_isr, IRQF_ONESHOT,
    "stmpe-adc", info);
    if (ret < 0)
    dev_warn(&pdev.dev, "failed requesting irq for"
    " temp sensor, irq = %d\n", irq_temp);
    }
    platform_set_drvdata(pdev, indio_dev);
    indio_dev.name		= dev_name(&pdev.dev);
    indio_dev.info		= &stmpe_adc_iio_info;
    indio_dev.modes	= INDIO_DIRECT_MODE;
    info.stmpe = dev_get_drvdata(pdev.dev.parent);
    np = pdev.dev.of_node;
    if (!np)
    dev_err(&pdev.dev, "no device tree node found\n");
    of_property_read_u32(np, "st,norequest-mask", &norequest_mask);
    bits = norequest_mask;
    for_each_clear_bit(i, &bits, (STMPE_ADC_LAST_NR + 1)) {
    stmpe_adc_voltage_chan(&info.stmpe_adc_iio_channels[num_chan], i);
    num_chan++;
    }
    stmpe_adc_temp_chan(&info.stmpe_adc_iio_channels[num_chan], i);
    num_chan++;
    indio_dev.channels = info.stmpe_adc_iio_channels;
    indio_dev.num_channels = num_chan;
    ret = stmpe_adc_init_hw(info);
    if (ret)
    return ret;
    stmpe_reg_write(info.stmpe, STMPE_REG_ADC_INT_EN,
    ~(norequest_mask & 0xFF));
    stmpe_reg_write(info.stmpe, STMPE_REG_ADC_INT_STA,
    ~(norequest_mask & 0xFF));
    return devm_iio_device_register(&pdev.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_adc_resume(dev: *mut device) -> c_int {
    static int stmpe_adc_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct stmpe_adc *info = iio_priv(indio_dev);
    stmpe_adc_init_hw(info);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(stmpe_adc_pm_ops, core::ptr::null_mut(), stmpe_adc_resume);
    static const struct of_device_id stmpe_adc_ids[] = {
    { .compatible = "st,stmpe-adc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, stmpe_adc_ids);
    static struct platform_driver stmpe_adc_driver = {
    .probe		= stmpe_adc_probe,
    .driver		= {
    .name	= "stmpe-adc",
    .pm	= pm_sleep_ptr(&stmpe_adc_pm_ops),
    .of_match_table = stmpe_adc_ids,
    },
    };
    module_platform_driver(stmpe_adc_driver);
    MODULE_AUTHOR("Stefan Agner <stefan.agner@toradex.com>");
    MODULE_DESCRIPTION("STMPEXXX ADC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:stmpe-adc");
