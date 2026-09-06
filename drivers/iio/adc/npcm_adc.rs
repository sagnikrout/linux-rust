//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/npcm_adc.c
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
// Copyright (c) 2019 Nuvoton Technology corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_adc_info {
    pub data_mask: u32,
    pub internal_vref: u32,
    pub res_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_adc {
    pub int_status: bool,
    pub adc_sample_hz: u32,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub adc_clk: *mut clk,
    pub wq: wait_queue_head_t,
    pub vref: *mut regulator,
    pub reset: *mut reset_control,
//
// Lock to protect the device state during a potential concurrent
// read access from userspace. Reading a raw value requires a sequence
// of register writes, then a wait for a event and finally a register
// read, during which userspace could issue another read request.
// This lock protects a read access from occurring before another one
// has finished.
//
    pub lock: mutex,
    pub data: *const npcm_adc_info,
}

// ADC registers
pub const NPCM_ADCCON: c_uint = 0x00;
pub const NPCM_ADCDATA: c_uint = 0x04;
// ADCCON Register Bits

pub const NPCM_ADCCON_DIV_SHIFT: c_int = 1;

// ADC General Definition
    static const struct npcm_adc_info npxm7xx_adc_info = {
    .data_mask = GENMASK(9, 0),
    .internal_vref = 2048,
    .res_bits = 10,
    };
    static const struct npcm_adc_info npxm8xx_adc_info = {
    .data_mask = GENMASK(11, 0),
    .internal_vref = 1229,
    .res_bits = 12,
    };

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = ch,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),	\
    }
    static const struct iio_chan_spec npcm_adc_iio_channels[] = {
    NPCM_ADC_CHAN(0),
    NPCM_ADC_CHAN(1),
    NPCM_ADC_CHAN(2),
    NPCM_ADC_CHAN(3),
    NPCM_ADC_CHAN(4),
    NPCM_ADC_CHAN(5),
    NPCM_ADC_CHAN(6),
    NPCM_ADC_CHAN(7),
    };
#[no_mangle]
unsafe extern "C" fn npcm_adc_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t npcm_adc_isr(int irq, void *data)
    {
    u32 regtemp;
    struct iio_dev *indio_dev = data;
    struct npcm_adc *info = iio_priv(indio_dev);
    regtemp = ioread32(info.regs + NPCM_ADCCON);
    if (regtemp & NPCM_ADCCON_ADC_INT_ST) {
    iowrite32(regtemp, info.regs + NPCM_ADCCON);
    wake_up_interruptible(&info.wq);
    info.int_status = true;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn npcm_adc_read(info: *mut npcm_adc, val: *mut c_int, channel: u8) -> c_int {
    static int npcm_adc_read(struct npcm_adc *info, int *val, u8 channel)
    {
    int ret;
    u32 regtemp;
// Select ADC channel
    regtemp = ioread32(info.regs + NPCM_ADCCON);
    regtemp &= ~NPCM_ADCCON_CH_MASK;
    info.int_status = false;
    iowrite32(regtemp | NPCM_ADCCON_CH(channel) |
    NPCM_ADCCON_ADC_CONV, info.regs + NPCM_ADCCON);
    ret = wait_event_interruptible_timeout(info.wq, info.int_status,
    msecs_to_jiffies(10));
    if (ret == 0) {
    regtemp = ioread32(info.regs + NPCM_ADCCON);
    if (regtemp & NPCM_ADCCON_ADC_CONV) {
// if conversion failed - reset ADC module
    reset_control_assert(info.reset);
    msleep(100);
    reset_control_deassert(info.reset);
    msleep(100);
// Enable ADC and start conversion module
    iowrite32(NPCM_ADC_ENABLE | NPCM_ADCCON_ADC_CONV,
    info.regs + NPCM_ADCCON);
    dev_err(info.dev, "RESET ADC Complete\n");
    }
    return -ETIMEDOUT;
    }
    if (ret < 0)
    return ret;
// val = ioread32(info->regs + NPCM_ADCDATA);
// val &= info->data->data_mask;
    return 0;
    }
    static int npcm_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    int ret;
    int vref_uv;
    struct npcm_adc *info = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&info.lock);
    ret = npcm_adc_read(info, val, chan.channel);
    mutex_unlock(&info.lock);
    if (ret) {
    dev_err(info.dev, "NPCM ADC read failed\n");
    return ret;
    }
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    if (!IS_ERR(info.vref)) {
    vref_uv = regulator_get_voltage(info.vref);
// val = vref_uv / 1000;
    } else {
// val = info->data->internal_vref;
    }
// val2 = info->data->res_bits;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = info->adc_sample_hz;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct iio_info npcm_adc_iio_info = {
    .read_raw = &npcm_adc_read_raw,
    };
    static const struct of_device_id npcm_adc_match[] = {
    { .compatible = "nuvoton,npcm750-adc", .data = &npxm7xx_adc_info},
    { .compatible = "nuvoton,npcm845-adc", .data = &npxm8xx_adc_info},
    { }
    };
    MODULE_DEVICE_TABLE(of, npcm_adc_match);
#[no_mangle]
unsafe extern "C" fn npcm_adc_probe(pdev: *mut platform_device) -> c_int {
    static int npcm_adc_probe(struct platform_device *pdev)
    {
    int ret;
    int irq;
    u32 div;
    u32 reg_con;
    struct npcm_adc *info;
    struct iio_dev *indio_dev;
    struct device *dev = &pdev.dev;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*info));
    if (!indio_dev)
    return -ENOMEM;
    info = iio_priv(indio_dev);
    info.data = device_get_match_data(dev);
    if (!info.data)
    return -EINVAL;
    mutex_init(&info.lock);
    info.dev = &pdev.dev;
    info.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(info.regs))
    return PTR_ERR(info.regs);
    info.reset = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(info.reset))
    return PTR_ERR(info.reset);
    info.adc_clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(info.adc_clk)) {
    dev_warn(&pdev.dev, "ADC clock failed: can't read clk\n");
    return PTR_ERR(info.adc_clk);
    }
// calculate ADC clock sample rate
    reg_con = ioread32(info.regs + NPCM_ADCCON);
    div = reg_con & NPCM_ADCCON_DIV_MASK;
    div = div >> NPCM_ADCCON_DIV_SHIFT;
    info.adc_sample_hz = clk_get_rate(info.adc_clk) / ((div + 1) * 2);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, npcm_adc_isr, 0,
    "NPCM_ADC", indio_dev);
    if (ret < 0)
    return ret;
    reg_con = ioread32(info.regs + NPCM_ADCCON);
    info.vref = devm_regulator_get_optional(&pdev.dev, "vref");
    if (!IS_ERR(info.vref)) {
    ret = regulator_enable(info.vref);
    if (ret) {
    dev_err(&pdev.dev, "Can't enable ADC reference voltage\n");
    return ret;
    }
    iowrite32(reg_con & ~NPCM_ADCCON_REFSEL,
    info.regs + NPCM_ADCCON);
    } else {
//
// Any error which is not ENODEV indicates the regulator
// has been specified and so is a failure case.
//
    if (PTR_ERR(info.vref) != -ENODEV)
    return PTR_ERR(info.vref);
// Use internal reference
    iowrite32(reg_con | NPCM_ADCCON_REFSEL,
    info.regs + NPCM_ADCCON);
    }
    init_waitqueue_head(&info.wq);
    reg_con = ioread32(info.regs + NPCM_ADCCON);
    reg_con |= NPCM_ADC_ENABLE;
// Enable the ADC Module
    iowrite32(reg_con, info.regs + NPCM_ADCCON);
// Start ADC conversion
    iowrite32(reg_con | NPCM_ADCCON_ADC_CONV, info.regs + NPCM_ADCCON);
    platform_set_drvdata(pdev, indio_dev);
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.info = &npcm_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = npcm_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(npcm_adc_iio_channels);
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&pdev.dev, "Couldn't register the device.\n");
    goto err_iio_register;
    }
    pr_info("NPCM ADC driver probed\n");
    return 0;
    err_iio_register:
    iowrite32(reg_con & ~NPCM_ADCCON_ADC_EN, info.regs + NPCM_ADCCON);
    if (!IS_ERR(info.vref))
    regulator_disable(info.vref);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn npcm_adc_remove(pdev: *mut platform_device) {
    static void npcm_adc_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct npcm_adc *info = iio_priv(indio_dev);
    u32 regtemp;
    iio_device_unregister(indio_dev);
    regtemp = ioread32(info.regs + NPCM_ADCCON);
    iowrite32(regtemp & ~NPCM_ADCCON_ADC_EN, info.regs + NPCM_ADCCON);
    if (!IS_ERR(info.vref))
    regulator_disable(info.vref);
    }
    static struct platform_driver npcm_adc_driver = {
    .probe		= npcm_adc_probe,
    .remove		= npcm_adc_remove,
    .driver		= {
    .name	= "npcm_adc",
    .of_match_table = npcm_adc_match,
    },
    };
    module_platform_driver(npcm_adc_driver);
    MODULE_DESCRIPTION("Nuvoton NPCM ADC Driver");
    MODULE_AUTHOR("Tomer Maimon <tomer.maimon@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
