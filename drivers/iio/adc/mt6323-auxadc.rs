//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/mt6323-auxadc.c
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
// Copyright (c) 2026 Roman Vivchar <rva333@protonmail.com>
//
// Based on drivers/iio/adc/mt6359-auxadc.c
//

pub const AUXADC_DEFAULT_OSR: c_int = 3;

    {                                                               \
    .type = IIO_VOLTAGE,                                    \
    .indexed = 1,                                           \
    .channel = _chan,                                       \
    .address = _addr,                                       \
    .datasheet_name = __stringify(_name),                   \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |          \
    BIT(IIO_CHAN_INFO_SCALE),         \
    }
//
// AUXADC reports everything in mV, including temperature and
// current channels. Channel macros are mapped such that their
// ID matches their respective hardware bit position in CON22.
//
    static const struct iio_chan_spec mt6323_auxadc_channels[] = {
    MTK_PMIC_IIO_CHAN(baton2,    MT6323_AUXADC_BATON2,    MT6323_AUXADC_ADC6),
    MTK_PMIC_IIO_CHAN(ch6,       MT6323_AUXADC_CH6,       MT6323_AUXADC_ADC11),
    MTK_PMIC_IIO_CHAN(bat_temp,  MT6323_AUXADC_BAT_TEMP,  MT6323_AUXADC_ADC5),
    MTK_PMIC_IIO_CHAN(chip_temp, MT6323_AUXADC_CHIP_TEMP, MT6323_AUXADC_ADC4),
    MTK_PMIC_IIO_CHAN(vcdt,      MT6323_AUXADC_VCDT,      MT6323_AUXADC_ADC2),
    MTK_PMIC_IIO_CHAN(baton1,    MT6323_AUXADC_BATON1,    MT6323_AUXADC_ADC3),
    MTK_PMIC_IIO_CHAN(isense,    MT6323_AUXADC_ISENSE,    MT6323_AUXADC_ADC1),
    MTK_PMIC_IIO_CHAN(batsns,    MT6323_AUXADC_BATSNS,    MT6323_AUXADC_ADC0),
    MTK_PMIC_IIO_CHAN(accdet,    MT6323_AUXADC_ACCDET,    MT6323_AUXADC_ADC7),
    };
//
// The MediaTek MT6323 (as well as a lot of other PMICs) has the following hierarchy:
// PMIC AUXADC <- PMIC MFD <- SoC PWRAP (wrapper for PWRAP FSM)
//
// Therefore, PWRAP regmap should be obtained using dev->parent->parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6323_auxadc {
    pub regmap: *mut regmap,
// AUXADC doesn't support reading multiple channels simultaneously.
    pub lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn mt6323_auxadc_prepare_channel(auxadc: *mut mt6323_auxadc) -> c_int {
    static int mt6323_auxadc_prepare_channel(struct mt6323_auxadc *auxadc)
    {
    struct regmap *map = auxadc.regmap;
    u32 val;
    int ret;
    ret = regmap_read(map, MT6323_AUXADC_CON19, &val);
    if (ret)
    return ret;
// The ADC is idle.
    if (!(val & AUXADC_CON19_DECI_GDLY_MASK))
    return 0;
    ret = regmap_read_poll_timeout(map, MT6323_AUXADC_ADC19,
    val, !(val & AUXADC_ADC19_BUSY_MASK),
    10, 500);
    if (ret)
    return ret;
    return regmap_clear_bits(map, MT6323_AUXADC_CON19,
    AUXADC_CON19_DECI_GDLY_MASK);
    }
    static int mt6323_auxadc_request(struct mt6323_auxadc *auxadc,
    unsigned long channel)
    {
    struct regmap *map = auxadc.regmap;
    int ret;
    ret = regmap_set_bits(map, MT6323_AUXADC_CON11, AUXADC_CON11_VBUF_EN);
    if (ret)
    return ret;
    return regmap_set_bits(map, MT6323_AUXADC_CON22, BIT(channel));
    }
    static int mt6323_auxadc_release(struct mt6323_auxadc *auxadc,
    unsigned long channel)
    {
    struct regmap *map = auxadc.regmap;
    int ret;
    ret = regmap_clear_bits(map, MT6323_AUXADC_CON22, BIT(channel));
    if (ret)
    return ret;
    return regmap_clear_bits(map, MT6323_AUXADC_CON11, AUXADC_CON11_VBUF_EN);
    }
    static int mt6323_auxadc_read(struct mt6323_auxadc *auxadc,
    const struct iio_chan_spec *chan, int *out)
    {
    struct regmap *map = auxadc.regmap;
    u32 val;
    int ret;
    ret = regmap_read_poll_timeout(map, chan.address,
    val, (val & AUXADC_READY_MASK),
    1 * USEC_PER_MSEC, 100 * USEC_PER_MSEC);
    if (ret)
    return ret;
// out = FIELD_GET(AUXADC_DATA_MASK, val);
    return 0;
    }
    static int mt6323_auxadc_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct mt6323_auxadc *auxadc = iio_priv(indio_dev);
    int ret, mult;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    if (chan.channel == MT6323_AUXADC_ISENSE ||
    chan.channel == MT6323_AUXADC_BATSNS)
    mult = 4;
    else
    mult = 1;
// 1800mV full range with 15-bit resolution.
// val = mult * 1800;
// val2 = 15;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_RAW: {
    guard(mutex)(&auxadc.lock);
    ret = mt6323_auxadc_prepare_channel(auxadc);
    if (ret)
    return ret;
    ret = mt6323_auxadc_request(auxadc, chan.channel);
    if (ret)
    return ret;
// Hardware limitation: the AUXADC needs a delay to become ready.
    fsleep(300);
    ret = mt6323_auxadc_read(auxadc, chan, val);
    if (mt6323_auxadc_release(auxadc, chan.channel))
    dev_err(&indio_dev.dev,
    "failed to release channel %d\n", chan.channel);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    }
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn mt6323_auxadc_init(auxadc: *mut mt6323_auxadc) -> c_int {
    static int mt6323_auxadc_init(struct mt6323_auxadc *auxadc)
    {
    struct regmap *map = auxadc.regmap;
    int ret;
    ret = regmap_set_bits(map, MT6323_STRUP_CON10,
    AUXADC_STRUP_CON10_RSTB_SW |
    AUXADC_STRUP_CON10_RSTB_SEL);
    if (ret)
    return ret;
    ret = regmap_set_bits(map, MT6323_TOP_CKPDN2, AUXADC_TOP_CKPDN2_CTL_CK);
    if (ret)
    return ret;
    ret = regmap_update_bits(map, MT6323_AUXADC_CON10,
    AUXADC_TRIM_CH2_MASK | AUXADC_TRIM_CH4_MASK |
    AUXADC_TRIM_CH5_MASK | AUXADC_TRIM_CH6_MASK,
    FIELD_PREP(AUXADC_TRIM_CH2_MASK, 1) |
    FIELD_PREP(AUXADC_TRIM_CH4_MASK, 1) |
    FIELD_PREP(AUXADC_TRIM_CH5_MASK, 1) |
    FIELD_PREP(AUXADC_TRIM_CH6_MASK, 1));
    if (ret)
    return ret;
    ret = regmap_set_bits(map, MT6323_AUXADC_CON27,
    AUXADC_CON27_VREF18_ENB_MD |
    AUXADC_CON27_MD_STATUS);
    if (ret)
    return ret;
    ret = regmap_set_bits(map, MT6323_AUXADC_CON19, AUXADC_CON19_GPS_STATUS);
    if (ret)
    return ret;
    ret = regmap_set_bits(map, MT6323_AUXADC_CON26,
    AUXADC_CON26_VREF18_SELB |
    AUXADC_CON26_DECI_GDLY_SEL);
    if (ret)
    return ret;
    return regmap_update_bits(map, MT6323_AUXADC_CON9, AUXADC_CON9_OSR_MASK,
    FIELD_PREP(AUXADC_CON9_OSR_MASK, AUXADC_DEFAULT_OSR));
    }
    static const struct iio_info mt6323_auxadc_iio_info = {
    .read_raw = mt6323_auxadc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn mt6323_auxadc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6323_auxadc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mt6323_auxadc *auxadc;
    struct regmap *regmap;
    struct iio_dev *iio;
    int ret;
    regmap = dev_get_regmap(dev.parent.parent, core::ptr::null_mut());
    if (!regmap)
    return dev_err_probe(dev, -ENODEV, "failed to get regmap\n");
    iio = devm_iio_device_alloc(dev, sizeof(*auxadc));
    if (!iio)
    return -ENOMEM;
    auxadc = iio_priv(iio);
    auxadc.regmap = regmap;
    ret = devm_mutex_init(dev, &auxadc.lock);
    if (ret)
    return ret;
    ret = mt6323_auxadc_init(auxadc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize auxadc\n");
    iio.name = "mt6323-auxadc";
    iio.info = &mt6323_auxadc_iio_info;
    iio.modes = INDIO_DIRECT_MODE;
    iio.channels = mt6323_auxadc_channels;
    iio.num_channels = ARRAY_SIZE(mt6323_auxadc_channels);
    return devm_iio_device_register(dev, iio);
    }
    static const struct of_device_id mt6323_auxadc_of_match[] = {
    { .compatible = "mediatek,mt6323-auxadc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mt6323_auxadc_of_match);
    static struct platform_driver mt6323_auxadc_driver = {
    .driver = {
    .name = "mt6323-auxadc",
    .of_match_table = mt6323_auxadc_of_match,
    },
    .probe	= mt6323_auxadc_probe,
    };
    module_platform_driver(mt6323_auxadc_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MediaTek MT6323 PMIC AUXADC Driver");
