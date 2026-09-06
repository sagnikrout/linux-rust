//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/sophgo-cv1800b-adc.c
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
// Sophgo CV1800B SARADC Driver
//
// Copyright (C) Bootlin 2024
// Author: Thomas Bonnefille <thomas.bonnefille@bootlin.com>
//

pub const CV1800B_ADC_CTRL_REG: c_uint = 0x04;

pub const CV1800B_ADC_STATUS_REG: c_uint = 0x08;

pub const CV1800B_ADC_CYC_SET_REG: c_uint = 0x0C;

pub const CV1800B_ADC_INTR_EN_REG: c_uint = 0x20;
pub const CV1800B_ADC_INTR_CLR_REG: c_uint = 0x24;

pub const CV1800B_ADC_INTR_STA_REG: c_uint = 0x28;

pub const CV1800B_READ_TIMEOUT_MS: c_int = 1000;

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = index,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),\
    .scan_index = index,					\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800b_adc {
    pub completion: completion,
    pub regs: *mut void __iomem,
    pub /: *mut *mut mutex lock; / ADC Control and Result register,
    pub clk: *mut clk,
    pub irq: c_int,
}

    static const struct iio_chan_spec sophgo_channels[] = {
    CV1800B_ADC_CHANNEL(0),
    CV1800B_ADC_CHANNEL(1),
    CV1800B_ADC_CHANNEL(2),
    };
    static void cv1800b_adc_start_measurement(struct cv1800b_adc *saradc,
    int channel)
    {
    writel(0, saradc.regs + CV1800B_ADC_CTRL_REG);
    writel(CV1800B_ADC_SEL(channel) | CV1800B_ADC_EN,
    saradc.regs + CV1800B_ADC_CTRL_REG);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_adc_wait(saradc: *mut cv1800b_adc) -> c_int {
    static int cv1800b_adc_wait(struct cv1800b_adc *saradc)
    {
    if (saradc.irq < 0) {
    u32 reg;
    return readl_poll_timeout(saradc.regs + CV1800B_ADC_STATUS_REG,
    reg, !(reg & CV1800B_ADC_BUSY),
    500, CV1800B_READ_TIMEOUT_US);
    }
    return wait_for_completion_timeout(&saradc.completion,
    msecs_to_jiffies(CV1800B_READ_TIMEOUT_MS)) > 0 ?
    0 : -ETIMEDOUT;
    }
    static int cv1800b_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct cv1800b_adc *saradc = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    u32 sample;
    scoped_guard(mutex, &saradc.lock) {
    int ret;
    cv1800b_adc_start_measurement(saradc, chan.scan_index);
    ret = cv1800b_adc_wait(saradc);
    if (ret < 0)
    return ret;
    sample = readl(saradc.regs + CV1800B_ADC_CH_RESULT_REG(chan.scan_index));
    }
    if (!(sample & CV1800B_ADC_CH_VALID))
    return -ENODATA;
// val = sample & CV1800B_ADC_CH_RESULT;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
// val = 3300;
// val2 = 12;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ: {
    let mut status_reg: u32 = readl(saradc.regs + CV1800B_ADC_CYC_SET_REG);
    let mut clk_div: c_uint = (1 + FIELD_GET(CV1800B_MASK_CLKDIV, status_reg));
    let mut freq: c_uint = clk_get_rate(saradc.clk) / clk_div;
    let mut nb_startup_cycle: c_uint = 1 + FIELD_GET(CV1800B_MASK_STARTUP_CYCLE, status_reg);
    let mut nb_sample_cycle: c_uint = 1 + FIELD_GET(CV1800B_MASK_SAMPLE_WINDOW, status_reg);
    let mut nb_compare_cycle: c_uint = 1 + FIELD_GET(CV1800B_MASK_COMPARE_CYCLE, status_reg);
// val = freq / (nb_startup_cycle + nb_sample_cycle + nb_compare_cycle);
    return IIO_VAL_INT;
    }
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_adc_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t cv1800b_adc_interrupt_handler(int irq, void *private)
    {
    struct cv1800b_adc *saradc = private;
    let mut reg: u32 = readl(saradc.regs + CV1800B_ADC_INTR_STA_REG);
    if (!(FIELD_GET(CV1800B_ADC_INTR_STA_BIT, reg)))
    return IRQ_NONE;
    writel(CV1800B_ADC_INTR_CLR_BIT, saradc.regs + CV1800B_ADC_INTR_CLR_REG);
    complete(&saradc.completion);
    return IRQ_HANDLED;
    }
    static const struct iio_info cv1800b_adc_info = {
    .read_raw = &cv1800b_adc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn cv1800b_adc_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800b_adc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cv1800b_adc *saradc;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*saradc));
    if (!indio_dev)
    return -ENOMEM;
    saradc = iio_priv(indio_dev);
    indio_dev.name = "sophgo-cv1800b-adc";
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &cv1800b_adc_info;
    indio_dev.num_channels = ARRAY_SIZE(sophgo_channels);
    indio_dev.channels = sophgo_channels;
    saradc.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(saradc.clk))
    return PTR_ERR(saradc.clk);
    saradc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(saradc.regs))
    return PTR_ERR(saradc.regs);
    saradc.irq = platform_get_irq_optional(pdev, 0);
    if (saradc.irq > 0) {
    init_completion(&saradc.completion);
    ret = devm_request_irq(dev, saradc.irq,
    cv1800b_adc_interrupt_handler, 0,
    dev_name(dev), saradc);
    if (ret)
    return ret;
    writel(1, saradc.regs + CV1800B_ADC_INTR_EN_REG);
    }
    ret = devm_mutex_init(dev, &saradc.lock);
    if (ret)
    return ret;
    writel(FIELD_PREP(CV1800B_MASK_STARTUP_CYCLE, 15) |
    FIELD_PREP(CV1800B_MASK_SAMPLE_WINDOW, 15) |
    FIELD_PREP(CV1800B_MASK_CLKDIV, 1) |
    FIELD_PREP(CV1800B_MASK_COMPARE_CYCLE, 15),
    saradc.regs + CV1800B_ADC_CYC_SET_REG);
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id cv1800b_adc_match[] = {
    { .compatible = "sophgo,cv1800b-saradc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, cv1800b_adc_match);
    static struct platform_driver cv1800b_adc_driver = {
    .driver	= {
    .name		= "sophgo-cv1800b-saradc",
    .of_match_table	= cv1800b_adc_match,
    },
    .probe = cv1800b_adc_probe,
    };
    module_platform_driver(cv1800b_adc_driver);
    MODULE_AUTHOR("Thomas Bonnefille <thomas.bonnefille@bootlin.com>");
    MODULE_DESCRIPTION("Sophgo CV1800B SARADC driver");
    MODULE_LICENSE("GPL");
