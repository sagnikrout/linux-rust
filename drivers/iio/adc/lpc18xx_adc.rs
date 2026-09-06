//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/lpc18xx_adc.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// IIO ADC driver for NXP LPC18xx ADC
//
// Copyright (C) 2016 Joachim Eastwood <manabian@gmail.com>
//
// UNSUPPORTED hardware features:
// - Hardware triggers
// - Burst mode
// - Interrupts
// - DMA
//

// LPC18XX ADC registers and bits
pub const LPC18XX_ADC_CR: c_uint = 0x000;
pub const LPC18XX_ADC_CR_CLKDIV_SHIFT: c_int = 8;

pub const LPC18XX_ADC_GDR: c_uint = 0x004;
// Data register bits
pub const LPC18XX_ADC_SAMPLE_SHIFT: c_int = 6;
pub const LPC18XX_ADC_SAMPLE_MASK: c_uint = 0x3ff;

// Clock should be 4.5 MHz or less
pub const LPC18XX_ADC_CLK_TARGET: c_int = 4500000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_adc {
    pub vref: *mut regulator,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub lock: mutex,
    pub clk: *mut clk,
    pub cr_reg: u32,
}

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = _idx,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    }
    static const struct iio_chan_spec lpc18xx_adc_iio_channels[] = {
    LPC18XX_ADC_CHAN(0),
    LPC18XX_ADC_CHAN(1),
    LPC18XX_ADC_CHAN(2),
    LPC18XX_ADC_CHAN(3),
    LPC18XX_ADC_CHAN(4),
    LPC18XX_ADC_CHAN(5),
    LPC18XX_ADC_CHAN(6),
    LPC18XX_ADC_CHAN(7),
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_adc_read_chan(adc: *mut lpc18xx_adc, ch: c_uint) -> c_int {
    static int lpc18xx_adc_read_chan(struct lpc18xx_adc *adc, unsigned int ch)
    {
    int ret;
    u32 reg;
    reg = adc.cr_reg | BIT(ch) | LPC18XX_ADC_CR_START_NOW;
    writel(reg, adc.base + LPC18XX_ADC_CR);
    ret = readl_poll_timeout(adc.base + LPC18XX_ADC_GDR, reg,
    reg & LPC18XX_ADC_CONV_DONE, 3, 9);
    if (ret) {
    dev_warn(adc.dev, "adc read timed out\n");
    return ret;
    }
    return (reg >> LPC18XX_ADC_SAMPLE_SHIFT) & LPC18XX_ADC_SAMPLE_MASK;
    }
    static int lpc18xx_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct lpc18xx_adc *adc = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&adc.lock);
// val = lpc18xx_adc_read_chan(adc, chan->channel);
    mutex_unlock(&adc.lock);
    if (*val < 0)
    return *val;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = regulator_get_voltage(adc->vref) / 1000;
// val2 = 10;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
    return -EINVAL;
    }
    static const struct iio_info lpc18xx_adc_info = {
    .read_raw = lpc18xx_adc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_clear_cr_reg(data: *mut c_void) {
    static void lpc18xx_clear_cr_reg(void *data)
    {
    struct lpc18xx_adc *adc = data;
    writel(0, adc.base + LPC18XX_ADC_CR);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_regulator_disable(vref: *mut c_void) {
    static void lpc18xx_regulator_disable(void *vref)
    {
    regulator_disable(vref);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_adc_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_adc_probe(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev;
    struct lpc18xx_adc *adc;
    unsigned int clkdiv;
    unsigned long rate;
    int ret;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    adc.dev = &pdev.dev;
    mutex_init(&adc.lock);
    adc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(adc.base))
    return PTR_ERR(adc.base);
    adc.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(adc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(adc.clk),
    "error getting clock\n");
    adc.vref = devm_regulator_get(&pdev.dev, "vref");
    if (IS_ERR(adc.vref))
    return dev_err_probe(&pdev.dev, PTR_ERR(adc.vref),
    "error getting regulator\n");
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.info = &lpc18xx_adc_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = lpc18xx_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(lpc18xx_adc_iio_channels);
    ret = regulator_enable(adc.vref);
    if (ret) {
    dev_err(&pdev.dev, "unable to enable regulator\n");
    return ret;
    }
    ret = devm_add_action_or_reset(&pdev.dev, lpc18xx_regulator_disable, adc.vref);
    if (ret)
    return ret;
    rate = clk_get_rate(adc.clk);
    clkdiv = DIV_ROUND_UP(rate, LPC18XX_ADC_CLK_TARGET);
    adc.cr_reg = (clkdiv << LPC18XX_ADC_CR_CLKDIV_SHIFT) |
    LPC18XX_ADC_CR_PDN;
    writel(adc.cr_reg, adc.base + LPC18XX_ADC_CR);
    ret = devm_add_action_or_reset(&pdev.dev, lpc18xx_clear_cr_reg, adc);
    if (ret)
    return ret;
    return devm_iio_device_register(&pdev.dev, indio_dev);
    }
    static const struct of_device_id lpc18xx_adc_match[] = {
    { .compatible = "nxp,lpc1850-adc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_adc_match);
    static struct platform_driver lpc18xx_adc_driver = {
    .probe	= lpc18xx_adc_probe,
    .driver	= {
    .name = "lpc18xx-adc",
    .of_match_table = lpc18xx_adc_match,
    },
    };
    module_platform_driver(lpc18xx_adc_driver);
    MODULE_DESCRIPTION("LPC18xx ADC driver");
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_LICENSE("GPL v2");
