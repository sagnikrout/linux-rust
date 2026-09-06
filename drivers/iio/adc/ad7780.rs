//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad7780.c
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
// AD7170/AD7171 and AD7780/AD7781 SPI ADC driver
//
// Copyright 2011 Analog Devices Inc.
// Copyright 2019 Renato Lui Geh
//

pub const AD7170_ID: c_int = 0;
pub const AD7171_ID: c_int = 1;
pub const AD7780_ID: c_int = 1;
pub const AD7781_ID: c_int = 0;

pub const AD7780_PATTERN_GOOD: c_int = 1;

pub const AD7170_PATTERN_GOOD: c_int = 5;

pub const AD7780_GAIN_MIDPOINT: c_int = 64;
pub const AD7780_FILTER_MIDPOINT: c_int = 13350;
    static const unsigned int ad778x_gain[2]      = { 1, 128 };
    static const unsigned int ad778x_odr_avail[2] = { 10000, 16700 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7780_chip_info {
    pub channel: iio_chan_spec,
    pub pattern_mask: c_uint,
    pub pattern: c_uint,
    pub is_ad778x: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7780_state {
    pub chip_info: *const ad7780_chip_info,
    pub reg: *mut regulator,
    pub powerdown_gpio: *mut gpio_desc,
    pub gain_gpio: *mut gpio_desc,
    pub filter_gpio: *mut gpio_desc,
    pub gain: c_uint,
    pub odr: c_uint,
    pub int_vref_mv: c_uint,
    pub sd: ad_sigma_delta,
}

    enum ad7780_supported_device_ids {
    ID_AD7170,
    ID_AD7171,
    ID_AD7780,
    ID_AD7781,
    };
    static struct ad7780_state *ad_sigma_delta_to_ad7780(struct ad_sigma_delta *sd)
    {
    return container_of(sd, struct ad7780_state, sd);
    }
    static int ad7780_set_mode(struct ad_sigma_delta *sigma_delta,
    enum ad_sigma_delta_mode mode)
    {
    struct ad7780_state *st = ad_sigma_delta_to_ad7780(sigma_delta);
    unsigned int val;
    switch (mode) {
    case AD_SD_MODE_SINGLE:
    case AD_SD_MODE_CONTINUOUS:
    val = 1;
    break;
    default:
    val = 0;
    break;
    }
    gpiod_set_value(st.powerdown_gpio, val);
    return 0;
    }
    static int ad7780_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long m)
    {
    struct ad7780_state *st = iio_priv(indio_dev);
    int voltage_uv;
    switch (m) {
    case IIO_CHAN_INFO_RAW:
    return ad_sigma_delta_single_conversion(indio_dev, chan, val);
    case IIO_CHAN_INFO_SCALE:
    voltage_uv = regulator_get_voltage(st.reg);
    if (voltage_uv < 0)
    return voltage_uv;
    voltage_uv /= 1000;
// val = voltage_uv * st->gain;
// val2 = chan->scan_type.realbits - 1;
    st.int_vref_mv = voltage_uv;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_OFFSET:
// val = -(1 << (chan->scan_type.realbits - 1));
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = st->odr;
    return IIO_VAL_INT;
    default:
    break;
    }
    return -EINVAL;
    }
    static int ad7780_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long m)
    {
    struct ad7780_state *st = iio_priv(indio_dev);
    const struct ad7780_chip_info *chip_info = st.chip_info;
    unsigned long long vref;
    unsigned int full_scale, gain;
    if (!chip_info.is_ad778x)
    return -EINVAL;
    switch (m) {
    case IIO_CHAN_INFO_SCALE:
    if (val != 0 || val2 == 0)
    return -EINVAL;
    vref = st.int_vref_mv * 1000000LL;
    full_scale = 1 << (chip_info.channel.scan_type.realbits - 1);
    gain = DIV_ROUND_CLOSEST_ULL(vref, full_scale);
    gain = DIV_ROUND_CLOSEST(gain, val2);
    st.gain = gain;
    if (gain < AD7780_GAIN_MIDPOINT)
    gain = 0;
    else
    gain = 1;
    gpiod_set_value(st.gain_gpio, gain);
    break;
    case IIO_CHAN_INFO_SAMP_FREQ:
    if (1000*val + val2/1000 < AD7780_FILTER_MIDPOINT)
    val = 0;
    else
    val = 1;
    st.odr = ad778x_odr_avail[val];
    gpiod_set_value(st.filter_gpio, val);
    break;
    default:
    break;
    }
    return 0;
    }
    static int ad7780_postprocess_sample(struct ad_sigma_delta *sigma_delta,
    unsigned int raw_sample)
    {
    struct ad7780_state *st = ad_sigma_delta_to_ad7780(sigma_delta);
    const struct ad7780_chip_info *chip_info = st.chip_info;
    if ((raw_sample & AD7780_ERR) ||
    ((raw_sample & chip_info.pattern_mask) != chip_info.pattern))
    return -EIO;
    if (chip_info.is_ad778x) {
    st.gain = ad778x_gain[raw_sample & AD7780_GAIN];
    st.odr = ad778x_odr_avail[raw_sample & AD7780_FILTER];
    }
    return 0;
    }
    static const struct ad_sigma_delta_info ad7780_sigma_delta_info = {
    .set_mode = ad7780_set_mode,
    .postprocess_sample = ad7780_postprocess_sample,
    .has_registers = false,
    .irq_flags = IRQF_TRIGGER_FALLING,
    };

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = 0,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_OFFSET),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = _mask_all,			\
    .scan_index = 1,					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = (_bits),				\
    .storagebits = 32,				\
    .shift = (_wordsize) - (_bits),			\
    .endianness = IIO_BE,				\
    },							\
    }

    _AD7780_CHANNEL(_bits, _wordsize, BIT(IIO_CHAN_INFO_SAMP_FREQ))

    _AD7780_CHANNEL(_bits, _wordsize, 0)
    static const struct ad7780_chip_info ad7780_chip_info_tbl[] = {
    [ID_AD7170] = {
    .channel = AD7170_CHANNEL(12, 24),
    .pattern = AD7170_PATTERN_GOOD,
    .pattern_mask = AD7170_PATTERN_MASK,
    .is_ad778x = false,
    },
    [ID_AD7171] = {
    .channel = AD7170_CHANNEL(16, 24),
    .pattern = AD7170_PATTERN_GOOD,
    .pattern_mask = AD7170_PATTERN_MASK,
    .is_ad778x = false,
    },
    [ID_AD7780] = {
    .channel = AD7780_CHANNEL(24, 32),
    .pattern = AD7780_PATTERN_GOOD,
    .pattern_mask = AD7780_PATTERN_MASK,
    .is_ad778x = true,
    },
    [ID_AD7781] = {
    .channel = AD7780_CHANNEL(20, 32),
    .pattern = AD7780_PATTERN_GOOD,
    .pattern_mask = AD7780_PATTERN_MASK,
    .is_ad778x = true,
    },
    };
    static const struct iio_info ad7780_info = {
    .read_raw = ad7780_read_raw,
    .write_raw = ad7780_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn ad7780_init_gpios(dev: *mut device, st: *mut ad7780_state) -> c_int {
    static int ad7780_init_gpios(struct device *dev, struct ad7780_state *st)
    {
    st.powerdown_gpio = devm_gpiod_get_optional(dev,
    "powerdown",
    GPIOD_OUT_LOW);
    if (IS_ERR(st.powerdown_gpio))
    return dev_err_probe(dev, PTR_ERR(st.powerdown_gpio),
    "Failed to request powerdown GPIO\n");
    if (!st.chip_info.is_ad778x)
    return 0;
    st.gain_gpio = devm_gpiod_get_optional(dev,
    "adi,gain",
    GPIOD_OUT_HIGH);
    if (IS_ERR(st.gain_gpio))
    return dev_err_probe(dev, PTR_ERR(st.gain_gpio),
    "Failed to request gain GPIO\n");
    st.filter_gpio = devm_gpiod_get_optional(dev,
    "adi,filter",
    GPIOD_OUT_HIGH);
    if (IS_ERR(st.filter_gpio))
    return dev_err_probe(dev, PTR_ERR(st.filter_gpio),
    "Failed to request filter GPIO\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7780_reg_disable(reg: *mut c_void) {
    static void ad7780_reg_disable(void *reg)
    {
    regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn ad7780_probe(spi: *mut spi_device) -> c_int {
    static int ad7780_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct ad7780_state *st;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.gain = 1;
    ad_sd_init(&st.sd, indio_dev, spi, &ad7780_sigma_delta_info);
    st.chip_info =
    &ad7780_chip_info_tbl[spi_get_device_id(spi).driver_data];
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = &st.chip_info.channel;
    indio_dev.num_channels = 1;
    indio_dev.info = &ad7780_info;
    ret = ad7780_init_gpios(dev, st);
    if (ret)
    return ret;
    st.reg = devm_regulator_get(dev, "avdd");
    if (IS_ERR(st.reg))
    return PTR_ERR(st.reg);
    ret = regulator_enable(st.reg);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to enable specified AVdd supply\n");
    ret = devm_add_action_or_reset(dev, ad7780_reg_disable, st.reg);
    if (ret)
    return ret;
    ret = devm_ad_sd_setup_buffer_and_trigger(dev, indio_dev);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id ad7780_id[] = {
    { .name = "ad7170", .driver_data = ID_AD7170 },
    { .name = "ad7171", .driver_data = ID_AD7171 },
    { .name = "ad7780", .driver_data = ID_AD7780 },
    { .name = "ad7781", .driver_data = ID_AD7781 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7780_id);
    static struct spi_driver ad7780_driver = {
    .driver = {
    .name	= "ad7780",
    },
    .probe		= ad7780_probe,
    .id_table	= ad7780_id,
    };
    module_spi_driver(ad7780_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD7780 and similar ADCs");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_AD_SIGMA_DELTA");
