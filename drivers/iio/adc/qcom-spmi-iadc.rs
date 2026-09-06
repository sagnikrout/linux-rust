//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/qcom-spmi-iadc.c
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
// Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
//

// IADC register and bit definition
pub const IADC_REVISION2: c_uint = 0x1;
pub const IADC_REVISION2_SUPPORTED_IADC: c_int = 1;
pub const IADC_PERPH_TYPE: c_uint = 0x4;
pub const IADC_PERPH_TYPE_ADC: c_int = 8;
pub const IADC_PERPH_SUBTYPE: c_uint = 0x5;
pub const IADC_PERPH_SUBTYPE_IADC: c_int = 3;
pub const IADC_STATUS1: c_uint = 0x8;
pub const IADC_STATUS1_OP_MODE: c_int = 4;

pub const IADC_STATUS1_REQ_STS_EOC_MASK: c_uint = 0x3;
pub const IADC_MODE_CTL: c_uint = 0x40;
pub const IADC_OP_MODE_SHIFT: c_int = 3;
pub const IADC_OP_MODE_NORMAL: c_int = 0;

pub const IADC_EN_CTL1: c_uint = 0x46;

pub const IADC_CH_SEL_CTL: c_uint = 0x48;
pub const IADC_DIG_PARAM: c_uint = 0x50;
pub const IADC_DIG_DEC_RATIO_SEL_SHIFT: c_int = 2;
pub const IADC_HW_SETTLE_DELAY: c_uint = 0x51;
pub const IADC_CONV_REQ: c_uint = 0x52;

pub const IADC_FAST_AVG_CTL: c_uint = 0x5a;
pub const IADC_FAST_AVG_EN: c_uint = 0x5b;

pub const IADC_PERH_RESET_CTL3: c_uint = 0xda;

pub const IADC_DATA: c_uint = 0x60	/* 16 bits */;
pub const IADC_SEC_ACCESS: c_uint = 0xd0;
pub const IADC_SEC_ACCESS_DATA: c_uint = 0xa5;
pub const IADC_NOMINAL_RSENSE: c_uint = 0xf4;

pub const IADC_REF_GAIN_MICRO_VOLTS: c_int = 17857;

pub const IADC_CONV_TIME_MIN_US: c_int = 2000;
pub const IADC_CONV_TIME_MAX_US: c_int = 2100;

// IADC channel list
pub const IADC_INT_RSENSE: c_int = 0;
pub const IADC_EXT_RSENSE: c_int = 1;
pub const IADC_GAIN_17P857MV: c_int = 3;
pub const IADC_EXT_OFFSET_CSP_CSN: c_int = 5;
pub const IADC_INT_OFFSET_CSP2_CSN2: c_int = 6;
//
// struct iadc_chip - IADC Current ADC device structure.
// @regmap: regmap for register read/write.
// @dev: This device pointer.
// @base: base offset for the ADC peripheral.
// @rsense: Values of the internal and external sense resister in micro Ohms.
// @poll_eoc: Poll for end of conversion instead of waiting for IRQ.
// @offset: Raw offset values for the internal and external channels.
// @gain: Raw gain of the channels.
// @lock: ADC lock for access to the peripheral.
// @complete: ADC notification after end of conversion interrupt is received.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iadc_chip {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub base: u16,
    pub poll_eoc: bool,
    pub rsense: [u32; 2],
    pub offset: [u16; 2],
    pub gain: u16,
    pub lock: mutex,
    pub complete: completion,
}

#[no_mangle]
unsafe extern "C" fn iadc_read(iadc: *mut iadc_chip, offset: u16, data: *mut u8) -> c_int {
    static int iadc_read(struct iadc_chip *iadc, u16 offset, u8 *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(iadc.regmap, iadc.base + offset, &val);
    if (ret < 0)
    return ret;
// data = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iadc_write(iadc: *mut iadc_chip, offset: u16, data: u8) -> c_int {
    static int iadc_write(struct iadc_chip *iadc, u16 offset, u8 data)
    {
    return regmap_write(iadc.regmap, iadc.base + offset, data);
    }
#[no_mangle]
unsafe extern "C" fn iadc_reset(iadc: *mut iadc_chip) -> c_int {
    static int iadc_reset(struct iadc_chip *iadc)
    {
    u8 data;
    int ret;
    ret = iadc_write(iadc, IADC_SEC_ACCESS, IADC_SEC_ACCESS_DATA);
    if (ret < 0)
    return ret;
    ret = iadc_read(iadc, IADC_PERH_RESET_CTL3, &data);
    if (ret < 0)
    return ret;
    ret = iadc_write(iadc, IADC_SEC_ACCESS, IADC_SEC_ACCESS_DATA);
    if (ret < 0)
    return ret;
    data |= IADC_FOLLOW_WARM_RB;
    return iadc_write(iadc, IADC_PERH_RESET_CTL3, data);
    }
#[no_mangle]
unsafe extern "C" fn iadc_set_state(iadc: *mut iadc_chip, state: bool) -> c_int {
    static int iadc_set_state(struct iadc_chip *iadc, bool state)
    {
    return iadc_write(iadc, IADC_EN_CTL1, state ? IADC_EN_CTL1_SET : 0);
    }
#[no_mangle]
unsafe extern "C" fn iadc_status_show(iadc: *mut iadc_chip) {
    static void iadc_status_show(struct iadc_chip *iadc)
    {
    u8 mode, sta1, chan, dig, en, req;
    int ret;
    ret = iadc_read(iadc, IADC_MODE_CTL, &mode);
    if (ret < 0)
    return;
    ret = iadc_read(iadc, IADC_DIG_PARAM, &dig);
    if (ret < 0)
    return;
    ret = iadc_read(iadc, IADC_CH_SEL_CTL, &chan);
    if (ret < 0)
    return;
    ret = iadc_read(iadc, IADC_CONV_REQ, &req);
    if (ret < 0)
    return;
    ret = iadc_read(iadc, IADC_STATUS1, &sta1);
    if (ret < 0)
    return;
    ret = iadc_read(iadc, IADC_EN_CTL1, &en);
    if (ret < 0)
    return;
    dev_err(iadc.dev,
    "mode:%02x en:%02x chan:%02x dig:%02x req:%02x sta1:%02x\n",
    mode, en, chan, dig, req, sta1);
    }
#[no_mangle]
unsafe extern "C" fn iadc_configure(iadc: *mut iadc_chip, channel: c_int) -> c_int {
    static int iadc_configure(struct iadc_chip *iadc, int channel)
    {
    u8 decim, mode;
    int ret;
// Mode selection
    mode = (IADC_OP_MODE_NORMAL << IADC_OP_MODE_SHIFT) | IADC_TRIM_EN;
    ret = iadc_write(iadc, IADC_MODE_CTL, mode);
    if (ret < 0)
    return ret;
// Channel selection
    ret = iadc_write(iadc, IADC_CH_SEL_CTL, channel);
    if (ret < 0)
    return ret;
// Digital parameter setup
    decim = IADC_DEF_DECIMATION << IADC_DIG_DEC_RATIO_SEL_SHIFT;
    ret = iadc_write(iadc, IADC_DIG_PARAM, decim);
    if (ret < 0)
    return ret;
// HW settle time delay
    ret = iadc_write(iadc, IADC_HW_SETTLE_DELAY, IADC_DEF_HW_SETTLE_TIME);
    if (ret < 0)
    return ret;
    ret = iadc_write(iadc, IADC_FAST_AVG_CTL, IADC_DEF_AVG_SAMPLES);
    if (ret < 0)
    return ret;
    if (IADC_DEF_AVG_SAMPLES)
    ret = iadc_write(iadc, IADC_FAST_AVG_EN, IADC_FAST_AVG_EN_SET);
    else
    ret = iadc_write(iadc, IADC_FAST_AVG_EN, 0);
    if (ret < 0)
    return ret;
    if (!iadc.poll_eoc)
    reinit_completion(&iadc.complete);
    ret = iadc_set_state(iadc, true);
    if (ret < 0)
    return ret;
// Request conversion
    return iadc_write(iadc, IADC_CONV_REQ, IADC_CONV_REQ_SET);
    }
#[no_mangle]
unsafe extern "C" fn iadc_poll_wait_eoc(iadc: *mut iadc_chip, interval_us: c_uint) -> c_int {
    static int iadc_poll_wait_eoc(struct iadc_chip *iadc, unsigned int interval_us)
    {
    unsigned int count, retry;
    int ret;
    u8 sta1;
    retry = interval_us / IADC_CONV_TIME_MIN_US;
    for (count = 0; count < retry; count++) {
    ret = iadc_read(iadc, IADC_STATUS1, &sta1);
    if (ret < 0)
    return ret;
    sta1 &= IADC_STATUS1_REQ_STS_EOC_MASK;
    if (sta1 == IADC_STATUS1_EOC)
    return 0;
    usleep_range(IADC_CONV_TIME_MIN_US, IADC_CONV_TIME_MAX_US);
    }
    iadc_status_show(iadc);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn iadc_read_result(iadc: *mut iadc_chip, data: *mut u16) -> c_int {
    static int iadc_read_result(struct iadc_chip *iadc, u16 *data)
    {
    return regmap_bulk_read(iadc.regmap, iadc.base + IADC_DATA, data, 2);
    }
#[no_mangle]
unsafe extern "C" fn iadc_do_conversion(iadc: *mut iadc_chip, chan: c_int, data: *mut u16) -> c_int {
    static int iadc_do_conversion(struct iadc_chip *iadc, int chan, u16 *data)
    {
    unsigned int wait;
    int ret;
    ret = iadc_configure(iadc, chan);
    if (ret < 0)
    goto exit;
    wait = BIT(IADC_DEF_AVG_SAMPLES) * IADC_CONV_TIME_MIN_US * 2;
    if (iadc.poll_eoc) {
    ret = iadc_poll_wait_eoc(iadc, wait);
    } else {
    ret = wait_for_completion_timeout(&iadc.complete,
    usecs_to_jiffies(wait));
    if (!ret)
    ret = -ETIMEDOUT;
    else
// double check conversion status
    ret = iadc_poll_wait_eoc(iadc, IADC_CONV_TIME_MIN_US);
    }
    if (!ret)
    ret = iadc_read_result(iadc, data);
    exit:
    iadc_set_state(iadc, false);
    if (ret < 0)
    dev_err(iadc.dev, "conversion failed\n");
    return ret;
    }
    static int iadc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct iadc_chip *iadc = iio_priv(indio_dev);
    s32 isense_ua, vsense_uv;
    u16 adc_raw, vsense_raw;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&iadc.lock);
    ret = iadc_do_conversion(iadc, chan.channel, &adc_raw);
    mutex_unlock(&iadc.lock);
    if (ret < 0)
    return ret;
    vsense_raw = adc_raw - iadc.offset[chan.channel];
    vsense_uv = vsense_raw * IADC_REF_GAIN_MICRO_VOLTS;
    vsense_uv /= (s32)iadc.gain - iadc.offset[chan.channel];
    isense_ua = vsense_uv / iadc.rsense[chan.channel];
    dev_dbg(iadc.dev, "off %d gain %d adc %d %duV I %duA\n",
    iadc.offset[chan.channel], iadc.gain,
    adc_raw, vsense_uv, isense_ua);
// val = isense_ua;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static const struct iio_info iadc_info = {
    .read_raw = iadc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn iadc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t iadc_isr(int irq, void *dev_id)
    {
    struct iadc_chip *iadc = dev_id;
    complete(&iadc.complete);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn iadc_update_offset(iadc: *mut iadc_chip) -> c_int {
    static int iadc_update_offset(struct iadc_chip *iadc)
    {
    int ret;
    ret = iadc_do_conversion(iadc, IADC_GAIN_17P857MV, &iadc.gain);
    if (ret < 0)
    return ret;
    ret = iadc_do_conversion(iadc, IADC_INT_OFFSET_CSP2_CSN2,
    &iadc.offset[IADC_INT_RSENSE]);
    if (ret < 0)
    return ret;
    if (iadc.gain == iadc.offset[IADC_INT_RSENSE]) {
    dev_err(iadc.dev, "error: internal offset == gain %d\n",
    iadc.gain);
    return -EINVAL;
    }
    ret = iadc_do_conversion(iadc, IADC_EXT_OFFSET_CSP_CSN,
    &iadc.offset[IADC_EXT_RSENSE]);
    if (ret < 0)
    return ret;
    if (iadc.gain == iadc.offset[IADC_EXT_RSENSE]) {
    dev_err(iadc.dev, "error: external offset == gain %d\n",
    iadc.gain);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iadc_version_check(iadc: *mut iadc_chip) -> c_int {
    static int iadc_version_check(struct iadc_chip *iadc)
    {
    u8 val;
    int ret;
    ret = iadc_read(iadc, IADC_PERPH_TYPE, &val);
    if (ret < 0)
    return ret;
    if (val < IADC_PERPH_TYPE_ADC) {
    dev_err(iadc.dev, "%d is not ADC\n", val);
    return -EINVAL;
    }
    ret = iadc_read(iadc, IADC_PERPH_SUBTYPE, &val);
    if (ret < 0)
    return ret;
    if (val < IADC_PERPH_SUBTYPE_IADC) {
    dev_err(iadc.dev, "%d is not IADC\n", val);
    return -EINVAL;
    }
    ret = iadc_read(iadc, IADC_REVISION2, &val);
    if (ret < 0)
    return ret;
    if (val < IADC_REVISION2_SUPPORTED_IADC) {
    dev_err(iadc.dev, "revision %d not supported\n", val);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iadc_rsense_read(iadc: *mut iadc_chip, node: *mut device_node) -> c_int {
    static int iadc_rsense_read(struct iadc_chip *iadc, struct device_node *node)
    {
    int ret, sign, int_sense;
    u8 deviation;
    ret = of_property_read_u32(node, "qcom,external-resistor-micro-ohms",
    &iadc.rsense[IADC_EXT_RSENSE]);
    if (ret < 0)
    iadc.rsense[IADC_EXT_RSENSE] = IADC_INT_RSENSE_IDEAL_VALUE;
    if (!iadc.rsense[IADC_EXT_RSENSE]) {
    dev_err(iadc.dev, "external resistor can't be zero Ohms");
    return -EINVAL;
    }
    ret = iadc_read(iadc, IADC_NOMINAL_RSENSE, &deviation);
    if (ret < 0)
    return ret;
//
// Deviation value stored is an offset from 10 mili Ohms, bit 7 is
// the sign, the remaining bits have an LSB of 15625 nano Ohms.
//
    sign = (deviation & IADC_NOMINAL_RSENSE_SIGN_MASK) ? -1 : 1;
    deviation &= ~IADC_NOMINAL_RSENSE_SIGN_MASK;
// Scale it to nono Ohms
    int_sense = IADC_INT_RSENSE_IDEAL_VALUE * 1000;
    int_sense += sign * deviation * IADC_INT_RSENSE_DEVIATION;
    int_sense /= 1000; /* micro Ohms */
    iadc.rsense[IADC_INT_RSENSE] = int_sense;
    return 0;
    }
    static const struct iio_chan_spec iadc_channels[] = {
    {
    .type = IIO_CURRENT,
    .datasheet_name	= "INTERNAL_RSENSE",
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .indexed = 1,
    },
    {
    .type = IIO_CURRENT,
    .datasheet_name	= "EXTERNAL_RSENSE",
    .channel = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .indexed = 1,
    },
    };
#[no_mangle]
unsafe extern "C" fn iadc_disable_irq_wake(data: *mut c_void) {
    static void iadc_disable_irq_wake(void *data)
    {
    disable_irq_wake((unsigned long)data);
    }
#[no_mangle]
unsafe extern "C" fn iadc_probe(pdev: *mut platform_device) -> c_int {
    static int iadc_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct iio_dev *indio_dev;
    struct iadc_chip *iadc;
    int ret, irq_eoc;
    u32 res;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*iadc));
    if (!indio_dev)
    return -ENOMEM;
    iadc = iio_priv(indio_dev);
    iadc.dev = dev;
    iadc.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!iadc.regmap)
    return -ENODEV;
    init_completion(&iadc.complete);
    mutex_init(&iadc.lock);
    ret = of_property_read_u32(node, "reg", &res);
    if (ret < 0)
    return -ENODEV;
    iadc.base = res;
    ret = iadc_version_check(iadc);
    if (ret < 0)
    return -ENODEV;
    ret = iadc_rsense_read(iadc, node);
    if (ret < 0)
    return -ENODEV;
    dev_dbg(iadc.dev, "sense resistors %d and %d micro Ohm\n",
    iadc.rsense[IADC_INT_RSENSE],
    iadc.rsense[IADC_EXT_RSENSE]);
    irq_eoc = platform_get_irq(pdev, 0);
    if (irq_eoc == -EPROBE_DEFER)
    return irq_eoc;
    if (irq_eoc < 0)
    iadc.poll_eoc = true;
    ret = iadc_reset(iadc);
    if (ret < 0) {
    dev_err(dev, "reset failed\n");
    return ret;
    }
    if (!iadc.poll_eoc) {
    ret = devm_request_irq(dev, irq_eoc, iadc_isr, 0,
    "spmi-iadc", iadc);
    if (ret)
    return ret;
    ret = enable_irq_wake(irq_eoc);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, iadc_disable_irq_wake,
    (void *)(unsigned long)irq_eoc);
    if (ret)
    return ret;
    } else {
    ret = devm_device_init_wakeup(iadc.dev);
    if (ret)
    return dev_err_probe(iadc.dev, ret, "Failed to init wakeup\n");
    }
    ret = iadc_update_offset(iadc);
    if (ret < 0) {
    dev_err(dev, "failed offset calibration\n");
    return ret;
    }
    indio_dev.name = pdev.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &iadc_info;
    indio_dev.channels = iadc_channels;
    indio_dev.num_channels = ARRAY_SIZE(iadc_channels);
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id iadc_match_table[] = {
    { .compatible = "qcom,spmi-iadc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, iadc_match_table);
    static struct platform_driver iadc_driver = {
    .driver = {
    .name = "qcom-spmi-iadc",
    .of_match_table = iadc_match_table,
    },
    .probe = iadc_probe,
    };
    module_platform_driver(iadc_driver);
    MODULE_ALIAS("platform:qcom-spmi-iadc");
    MODULE_DESCRIPTION("Qualcomm SPMI PMIC current ADC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Ivan T. Ivanov <iivanov@mm-sol.com>");
