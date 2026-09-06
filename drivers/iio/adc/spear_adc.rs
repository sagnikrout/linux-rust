//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/spear_adc.c
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
// ST SPEAr ADC driver
//
// Copyright 2012 Stefan Roese <sr@denx.de>
//

// SPEAR registers definitions

// Bit definitions for SPEAR_ADC_STATUS

pub const SPEAR_ADC_DATA_MASK: c_uint = 0x03ff;
pub const SPEAR_ADC_DATA_BITS: c_int = 10;

pub const SPEAR_ADC_CHANNEL_NUM: c_int = 8;
pub const SPEAR_ADC_CLK_MIN: c_int = 2500000;
pub const SPEAR_ADC_CLK_MAX: c_int = 20000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_regs_spear3xx {
    pub status: u32,
    pub average: u32,
    pub scan_rate: u32,
    pub /: *mut *mut u32 clk; / Not avail for 1340 & 1310,
    pub ch_ctrl: [u32; SPEAR_ADC_CHANNEL_NUM],
    pub ch_data: [u32; SPEAR_ADC_CHANNEL_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_data {
    pub lsb: u32,
    pub msb: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_regs_spear6xx {
    pub status: u32,
    pub pad: [u32; 2],
    pub clk: u32,
    pub ch_ctrl: [u32; SPEAR_ADC_CHANNEL_NUM],
    pub ch_data: [chan_data; SPEAR_ADC_CHANNEL_NUM],
    pub scan_rate_lo: u32,
    pub scan_rate_hi: u32,
    pub average: chan_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_adc_state {
    pub dev: *mut device,
    pub adc_base_spear3xx: *mut adc_regs_spear3xx __iomem,
    pub adc_base_spear6xx: *mut adc_regs_spear6xx __iomem,
    pub clk: *mut clk,
    pub completion: completion,
//
// Lock to protect the device state during a potential concurrent
// read access from userspace. Reading a raw value requires a sequence
// of register writes, then a wait for a completion callback,
// and finally a register read, during which userspace could issue
// another read request. This lock protects a read access from
// occurring before another one has finished.
//
    pub lock: mutex,
    pub current_clk: u32,
    pub sampling_freq: u32,
    pub avg_samples: u32,
    pub vref_external: u32,
    pub value: u32,
}

//
// Functions to access some SPEAr ADC register. Abstracted into
// static inline functions, because of different register offsets
// on different SoC variants (SPEAr300 vs SPEAr600 etc).
//
#[no_mangle]
unsafe extern "C" fn spear_adc_set_status(st: *mut spear_adc_state, val: u32) {
    static void spear_adc_set_status(struct spear_adc_state *st, u32 val)
    {
    __raw_writel(val, &st.adc_base_spear6xx.status);
    }
#[no_mangle]
unsafe extern "C" fn spear_adc_set_clk(st: *mut spear_adc_state, val: u32) {
    static void spear_adc_set_clk(struct spear_adc_state *st, u32 val)
    {
    u32 clk_high, clk_low, count;
    let mut apb_clk: u32 = clk_get_rate(st.clk);
    count = DIV_ROUND_UP(apb_clk, val);
    clk_low = count / 2;
    clk_high = count - clk_low;
    st.current_clk = apb_clk / count;
    __raw_writel(SPEAR_ADC_CLK_LOW(clk_low) | SPEAR_ADC_CLK_HIGH(clk_high),
    &st.adc_base_spear6xx.clk);
    }
    static void spear_adc_set_ctrl(struct spear_adc_state *st, int n,
    u32 val)
    {
    __raw_writel(val, &st.adc_base_spear6xx.ch_ctrl[n]);
    }
#[no_mangle]
unsafe extern "C" fn spear_adc_get_average(st: *mut spear_adc_state) -> u32 {
    static u32 spear_adc_get_average(struct spear_adc_state *st)
    {
    if (device_is_compatible(st.dev, "st,spear600-adc")) {
    return __raw_readl(&st.adc_base_spear6xx.average.msb) &
    SPEAR_ADC_DATA_MASK;
    } else {
    return __raw_readl(&st.adc_base_spear3xx.average) &
    SPEAR_ADC_DATA_MASK;
    }
    }
#[no_mangle]
unsafe extern "C" fn spear_adc_set_scanrate(st: *mut spear_adc_state, rate: u32) {
    static void spear_adc_set_scanrate(struct spear_adc_state *st, u32 rate)
    {
    if (device_is_compatible(st.dev, "st,spear600-adc")) {
    __raw_writel(SPEAR600_ADC_SCAN_RATE_LO(rate),
    &st.adc_base_spear6xx.scan_rate_lo);
    __raw_writel(SPEAR600_ADC_SCAN_RATE_HI(rate),
    &st.adc_base_spear6xx.scan_rate_hi);
    } else {
    __raw_writel(rate, &st.adc_base_spear3xx.scan_rate);
    }
    }
    static int spear_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct spear_adc_state *st = iio_priv(indio_dev);
    u32 status;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&st.lock);
    status = FIELD_PREP(SPEAR_ADC_STATUS_CHANNEL_NUM_MASK, chan.channel) |
    FIELD_PREP(SPEAR_ADC_STATUS_AVG_SAMPLE_MASK, st.avg_samples) |
    SPEAR_ADC_STATUS_START_CONVERSION |
    SPEAR_ADC_STATUS_ADC_ENABLE;
    if (st.vref_external == 0)
    status |= SPEAR_ADC_STATUS_VREF_INTERNAL;
    spear_adc_set_status(st, status);
    wait_for_completion(&st.completion); /* set by ISR */
// val = st->value;
    mutex_unlock(&st.lock);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = st->vref_external;
// val2 = SPEAR_ADC_DATA_BITS;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = st->current_clk;
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    static int spear_adc_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct spear_adc_state *st = iio_priv(indio_dev);
    let mut ret: c_int = 0;
    if (mask != IIO_CHAN_INFO_SAMP_FREQ)
    return -EINVAL;
    mutex_lock(&st.lock);
    if ((val < SPEAR_ADC_CLK_MIN) ||
    (val > SPEAR_ADC_CLK_MAX) ||
    (val2 != 0)) {
    ret = -EINVAL;
    goto out;
    }
    spear_adc_set_clk(st, val);
    out:
    mutex_unlock(&st.lock);
    return ret;
    }

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),\
    .channel = idx,					\
    }
    static const struct iio_chan_spec spear_adc_iio_channels[] = {
    SPEAR_ADC_CHAN(0),
    SPEAR_ADC_CHAN(1),
    SPEAR_ADC_CHAN(2),
    SPEAR_ADC_CHAN(3),
    SPEAR_ADC_CHAN(4),
    SPEAR_ADC_CHAN(5),
    SPEAR_ADC_CHAN(6),
    SPEAR_ADC_CHAN(7),
    };
#[no_mangle]
unsafe extern "C" fn spear_adc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spear_adc_isr(int irq, void *dev_id)
    {
    struct spear_adc_state *st = dev_id;
// Read value to clear IRQ
    st.value = spear_adc_get_average(st);
    complete(&st.completion);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spear_adc_configure(st: *mut spear_adc_state) -> c_int {
    static int spear_adc_configure(struct spear_adc_state *st)
    {
    int i;
// Reset ADC core
    spear_adc_set_status(st, 0);
    __raw_writel(0, &st.adc_base_spear6xx.clk);
    for (i = 0; i < 8; i++)
    spear_adc_set_ctrl(st, i, 0);
    spear_adc_set_scanrate(st, 0);
    spear_adc_set_clk(st, st.sampling_freq);
    return 0;
    }
    static const struct iio_info spear_adc_info = {
    .read_raw = &spear_adc_read_raw,
    .write_raw = &spear_adc_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn spear_adc_probe(pdev: *mut platform_device) -> c_int {
    static int spear_adc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spear_adc_state *st;
    struct iio_dev *indio_dev = core::ptr::null_mut();
    let mut ret: c_int = -ENODEV;
    int irq;
    indio_dev = devm_iio_device_alloc(dev, sizeof(struct spear_adc_state));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.dev = dev;
    init_completion(&st.completion);
    mutex_init(&st.lock);
//
// SPEAr600 has a different register layout than other SPEAr SoC's
// (e.g. SPEAr3xx). Let's provide two register base addresses
// to support multi-arch kernels.
//
    st.adc_base_spear6xx = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(st.adc_base_spear6xx))
    return PTR_ERR(st.adc_base_spear6xx);
    st.adc_base_spear3xx =
    (struct adc_regs_spear3xx __iomem *)st.adc_base_spear6xx;
    st.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(st.clk))
    return dev_err_probe(dev, PTR_ERR(st.clk),
    "failed enabling clock\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, spear_adc_isr, 0, SPEAR_ADC_MOD_NAME,
    st);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed requesting interrupt\n");
    if (device_property_read_u32(dev, "sampling-frequency", &st.sampling_freq))
    return dev_err_probe(dev, -EINVAL,
    "sampling-frequency missing in DT\n");
//
// Optional avg_samples defaults to 0, resulting in single data
// conversion
//
    device_property_read_u32(dev, "average-samples", &st.avg_samples);
//
// Optional vref_external defaults to 0, resulting in internal vref
// selection
//
    device_property_read_u32(dev, "vref-external", &st.vref_external);
    spear_adc_configure(st);
    indio_dev.name = SPEAR_ADC_MOD_NAME;
    indio_dev.info = &spear_adc_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = spear_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(spear_adc_iio_channels);
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret)
    return ret;
    dev_info(dev, "SPEAR ADC driver loaded, IRQ %d\n", irq);
    return 0;
    }
    static const struct of_device_id spear_adc_dt_ids[] = {
    { .compatible = "st,spear600-adc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, spear_adc_dt_ids);
    static struct platform_driver spear_adc_driver = {
    .probe		= spear_adc_probe,
    .driver		= {
    .name	= SPEAR_ADC_MOD_NAME,
    .of_match_table = spear_adc_dt_ids,
    },
    };
    module_platform_driver(spear_adc_driver);
    MODULE_AUTHOR("Stefan Roese <sr@denx.de>");
    MODULE_DESCRIPTION("SPEAr ADC driver");
    MODULE_LICENSE("GPL");
