//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad7791.c
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
// AD7787/AD7788/AD7789/AD7790/AD7791 SPI ADC driver
//
// Copyright 2012 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

pub const AD7791_REG_COMM: c_uint = 0x0 /* For writes */;
pub const AD7791_REG_STATUS: c_uint = 0x0 /* For reads */;
pub const AD7791_REG_MODE: c_uint = 0x1;
pub const AD7791_REG_FILTER: c_uint = 0x2;
pub const AD7791_REG_DATA: c_uint = 0x3;
pub const AD7791_MODE_CONTINUOUS: c_uint = 0x00;
pub const AD7791_MODE_SINGLE: c_uint = 0x02;
pub const AD7791_MODE_POWERDOWN: c_uint = 0x03;
pub const AD7791_CH_AIN1P_AIN1N: c_uint = 0x00;
pub const AD7791_CH_AIN2: c_uint = 0x01;
pub const AD7791_CH_AIN1N_AIN1N: c_uint = 0x02;
pub const AD7791_CH_AVDD_MONITOR: c_uint = 0x03;

pub const AD7791_FILTER_RATE_120: c_uint = 0x0;
pub const AD7791_FILTER_RATE_100: c_uint = 0x1;
pub const AD7791_FILTER_RATE_33_3: c_uint = 0x2;
pub const AD7791_FILTER_RATE_20: c_uint = 0x3;
pub const AD7791_FILTER_RATE_16_6: c_uint = 0x4;
pub const AD7791_FILTER_RATE_16_7: c_uint = 0x5;
pub const AD7791_FILTER_RATE_13_3: c_uint = 0x6;
pub const AD7791_FILTER_RATE_9_5: c_uint = 0x7;
pub const AD7791_FILTER_RATE_MASK: c_uint = 0x7;

    _storagebits, _shift, _extend_name, _type, _mask_all) \
    { \
    .type = (_type), \
    .differential = (_channel2 == -1 ? 0 : 1), \
    .indexed = 1, \
    .channel = (_channel1), \
    .channel2 = (_channel2), \
    .address = (_address), \
    .extend_name = (_extend_name), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_OFFSET), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .info_mask_shared_by_all = _mask_all, \
    .scan_index = (_si), \
    .scan_type = { \
    .sign = 'u', \
    .realbits = (_bits), \
    .storagebits = (_storagebits), \
    .shift = (_shift), \
    .endianness = IIO_BE, \
    }, \
    }

    _storagebits, _shift) \
    __AD7991_CHANNEL(_si, _channel, _channel, _address, _bits, \
    _storagebits, _shift, "shorted", IIO_VOLTAGE, \
    BIT(IIO_CHAN_INFO_SAMP_FREQ))

    _storagebits, _shift) \
    __AD7991_CHANNEL(_si, _channel, -1, _address, _bits, \
    _storagebits, _shift, core::ptr::null_mut(), IIO_VOLTAGE, \
    BIT(IIO_CHAN_INFO_SAMP_FREQ))

    _storagebits, _shift) \
    __AD7991_CHANNEL(_si, _channel1, _channel2, _address, _bits, \
    _storagebits, _shift, core::ptr::null_mut(), IIO_VOLTAGE, \
    BIT(IIO_CHAN_INFO_SAMP_FREQ))

    _shift) \
    __AD7991_CHANNEL(_si, _channel, -1, _address, _bits, \
    _storagebits, _shift, "supply", IIO_VOLTAGE, \
    BIT(IIO_CHAN_INFO_SAMP_FREQ))

    const struct iio_chan_spec name[] = { \
    AD7991_DIFF_CHANNEL(0, 0, 0, AD7791_CH_AIN1P_AIN1N, \
    (bits), (storagebits), 0), \
    AD7991_CHANNEL(1, 1, AD7791_CH_AIN2, (bits), (storagebits), 0), \
    AD7991_SHORTED_CHANNEL(2, 0, AD7791_CH_AIN1N_AIN1N, \
    (bits), (storagebits), 0), \
    AD7991_SUPPLY_CHANNEL(3, 2, AD7791_CH_AVDD_MONITOR,  \
    (bits), (storagebits), 0), \
    IIO_CHAN_SOFT_TIMESTAMP(4), \
    }

    const struct iio_chan_spec name[] = { \
    AD7991_DIFF_CHANNEL(0, 0, 0, AD7791_CH_AIN1P_AIN1N, \
    (bits), (storagebits), 0), \
    AD7991_SHORTED_CHANNEL(1, 0, AD7791_CH_AIN1N_AIN1N, \
    (bits), (storagebits), 0), \
    AD7991_SUPPLY_CHANNEL(2, 1, AD7791_CH_AVDD_MONITOR, \
    (bits), (storagebits), 0), \
    IIO_CHAN_SOFT_TIMESTAMP(3), \
    }
    static DECLARE_AD7787_CHANNELS(ad7787_channels, 24, 32);
    static DECLARE_AD7791_CHANNELS(ad7790_channels, 16, 16);
    static DECLARE_AD7791_CHANNELS(ad7791_channels, 24, 32);
    enum {
    AD7787,
    AD7788,
    AD7789,
    AD7790,
    AD7791,
    };
    enum ad7791_chip_info_flags {
    AD7791_FLAG_HAS_FILTER		= (1 << 0),
    AD7791_FLAG_HAS_BUFFER		= (1 << 1),
    AD7791_FLAG_HAS_UNIPOLAR	= (1 << 2),
    AD7791_FLAG_HAS_BURNOUT		= (1 << 3),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7791_chip_info {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub flags: enum ad7791_chip_info_flags,
}

    static const struct ad7791_chip_info ad7791_chip_infos[] = {
    [AD7787] = {
    .channels = ad7787_channels,
    .num_channels = ARRAY_SIZE(ad7787_channels),
    .flags = AD7791_FLAG_HAS_FILTER | AD7791_FLAG_HAS_BUFFER |
    AD7791_FLAG_HAS_UNIPOLAR | AD7791_FLAG_HAS_BURNOUT,
    },
    [AD7788] = {
    .channels = ad7790_channels,
    .num_channels = ARRAY_SIZE(ad7790_channels),
    .flags = AD7791_FLAG_HAS_UNIPOLAR,
    },
    [AD7789] = {
    .channels = ad7791_channels,
    .num_channels = ARRAY_SIZE(ad7791_channels),
    .flags = AD7791_FLAG_HAS_UNIPOLAR,
    },
    [AD7790] = {
    .channels = ad7790_channels,
    .num_channels = ARRAY_SIZE(ad7790_channels),
    .flags = AD7791_FLAG_HAS_FILTER | AD7791_FLAG_HAS_BUFFER |
    AD7791_FLAG_HAS_BURNOUT,
    },
    [AD7791] = {
    .channels = ad7791_channels,
    .num_channels = ARRAY_SIZE(ad7791_channels),
    .flags = AD7791_FLAG_HAS_FILTER | AD7791_FLAG_HAS_BUFFER |
    AD7791_FLAG_HAS_UNIPOLAR | AD7791_FLAG_HAS_BURNOUT,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7791_state {
    pub sd: ad_sigma_delta,
    pub mode: u8,
    pub filter: u8,
    pub reg: *mut regulator,
    pub info: *const ad7791_chip_info,
}

    static const int ad7791_sample_freq_avail[8][2] = {
    [AD7791_FILTER_RATE_120] =  { 120, 0 },
    [AD7791_FILTER_RATE_100] =  { 100, 0 },
    [AD7791_FILTER_RATE_33_3] = { 33,  300000 },
    [AD7791_FILTER_RATE_20] =   { 20,  0 },
    [AD7791_FILTER_RATE_16_6] = { 16,  600000 },
    [AD7791_FILTER_RATE_16_7] = { 16,  700000 },
    [AD7791_FILTER_RATE_13_3] = { 13,  300000 },
    [AD7791_FILTER_RATE_9_5] =  { 9,   500000 },
    };
    static struct ad7791_state *ad_sigma_delta_to_ad7791(struct ad_sigma_delta *sd)
    {
    return container_of(sd, struct ad7791_state, sd);
    }
#[no_mangle]
unsafe extern "C" fn ad7791_set_channel(sd: *mut ad_sigma_delta, channel: c_uint) -> c_int {
    static int ad7791_set_channel(struct ad_sigma_delta *sd, unsigned int channel)
    {
    ad_sd_set_comm(sd, channel);
    return 0;
    }
    static int ad7791_set_mode(struct ad_sigma_delta *sd,
    enum ad_sigma_delta_mode mode)
    {
    struct ad7791_state *st = ad_sigma_delta_to_ad7791(sd);
    switch (mode) {
    case AD_SD_MODE_CONTINUOUS:
    mode = AD7791_MODE_CONTINUOUS;
    break;
    case AD_SD_MODE_SINGLE:
    mode = AD7791_MODE_SINGLE;
    break;
    case AD_SD_MODE_IDLE:
    case AD_SD_MODE_POWERDOWN:
    mode = AD7791_MODE_POWERDOWN;
    break;
    }
    st.mode &= ~AD7791_MODE_SEL_MASK;
    st.mode |= AD7791_MODE_SEL(mode);
    return ad_sd_write_reg(sd, AD7791_REG_MODE, sizeof(st.mode), st.mode);
    }
    static const struct ad_sigma_delta_info ad7791_sigma_delta_info = {
    .set_channel = ad7791_set_channel,
    .set_mode = ad7791_set_mode,
    .has_registers = true,
    .addr_shift = 4,
    .read_mask = BIT(3),
    .irq_flags = IRQF_TRIGGER_FALLING,
    .num_resetclks = 32,
    };
    static int ad7791_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int *val, int *val2, long info)
    {
    struct ad7791_state *st = iio_priv(indio_dev);
    let mut unipolar: bool = !!(st.mode & AD7791_MODE_UNIPOLAR);
    unsigned int rate;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    return ad_sigma_delta_single_conversion(indio_dev, chan, val);
    case IIO_CHAN_INFO_OFFSET:
//
// Unipolar: 0 to VREF
// Bipolar -VREF to VREF
//
    if (unipolar)
// val = 0;
    else
// val = -(1 << (chan->scan_type.realbits - 1));
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// The monitor channel uses an internal reference.
    if (chan.address == AD7791_CH_AVDD_MONITOR) {
//
// The signal is attenuated by a factor of 5 and
// compared against a 1.17V internal reference.
//
// val = 1170 * 5;
    } else {
    int voltage_uv;
    voltage_uv = regulator_get_voltage(st.reg);
    if (voltage_uv < 0)
    return voltage_uv;
// val = voltage_uv / 1000;
    }
    if (unipolar)
// val2 = chan->scan_type.realbits;
    else
// val2 = chan->scan_type.realbits - 1;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
    rate = st.filter & AD7791_FILTER_RATE_MASK;
// val = ad7791_sample_freq_avail[rate][0];
// val2 = ad7791_sample_freq_avail[rate][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int __ad7791_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val, int val2, long mask)
    {
    struct ad7791_state *st = iio_priv(indio_dev);
    int i;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    for (i = 0; i < ARRAY_SIZE(ad7791_sample_freq_avail); i++) {
    if (ad7791_sample_freq_avail[i][0] == val &&
    ad7791_sample_freq_avail[i][1] == val2)
    break;
    }
    if (i == ARRAY_SIZE(ad7791_sample_freq_avail))
    return -EINVAL;
    st.filter &= ~AD7791_FILTER_RATE_MASK;
    st.filter |= i;
    ad_sd_write_reg(&st.sd, AD7791_REG_FILTER,
    sizeof(st.filter),
    st.filter);
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int ad7791_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val, int val2, long mask)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __ad7791_write_raw(indio_dev, chan, val, val2, mask);
    iio_device_release_direct(indio_dev);
    return ret;
    }
    static IIO_CONST_ATTR_SAMP_FREQ_AVAIL("120 100 33.3 20 16.7 16.6 13.3 9.5");
    static struct attribute *ad7791_attributes[] = {
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ad7791_attribute_group = {
    .attrs = ad7791_attributes,
    };
    static const struct iio_info ad7791_info = {
    .read_raw = &ad7791_read_raw,
    .write_raw = &ad7791_write_raw,
    .attrs = &ad7791_attribute_group,
    .validate_trigger = ad_sd_validate_trigger,
    };
    static const struct iio_info ad7791_no_filter_info = {
    .read_raw = &ad7791_read_raw,
    .write_raw = &ad7791_write_raw,
    .validate_trigger = ad_sd_validate_trigger,
    };
    static int ad7791_setup(struct ad7791_state *st,
    const struct ad7791_platform_data *pdata)
    {
// Set to poweron-reset default values
    st.mode = AD7791_MODE_BUFFER;
    st.filter = AD7791_FILTER_RATE_16_6;
    if (!pdata)
    return 0;
    if ((st.info.flags & AD7791_FLAG_HAS_BUFFER) && !pdata.buffered)
    st.mode &= ~AD7791_MODE_BUFFER;
    if ((st.info.flags & AD7791_FLAG_HAS_BURNOUT) &&
    pdata.burnout_current)
    st.mode |= AD7791_MODE_BURNOUT;
    if ((st.info.flags & AD7791_FLAG_HAS_UNIPOLAR) && pdata.unipolar)
    st.mode |= AD7791_MODE_UNIPOLAR;
    return ad_sd_write_reg(&st.sd, AD7791_REG_MODE, sizeof(st.mode),
    st.mode);
    }
#[no_mangle]
unsafe extern "C" fn ad7791_reg_disable(reg: *mut c_void) {
    static void ad7791_reg_disable(void *reg)
    {
    regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn ad7791_probe(spi: *mut spi_device) -> c_int {
    static int ad7791_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    const struct ad7791_platform_data *pdata = dev_get_platdata(dev);
    struct iio_dev *indio_dev;
    struct ad7791_state *st;
    int ret;
    if (!spi.irq)
    return dev_err_probe(dev, -ENXIO, "Missing IRQ.\n");
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.reg = devm_regulator_get(dev, "refin");
    if (IS_ERR(st.reg))
    return PTR_ERR(st.reg);
    ret = regulator_enable(st.reg);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, ad7791_reg_disable, st.reg);
    if (ret)
    return ret;
    st.info = &ad7791_chip_infos[spi_get_device_id(spi).driver_data];
    ad_sd_init(&st.sd, indio_dev, spi, &ad7791_sigma_delta_info);
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = st.info.channels;
    indio_dev.num_channels = st.info.num_channels;
    if (st.info.flags & AD7791_FLAG_HAS_FILTER)
    indio_dev.info = &ad7791_info;
    else
    indio_dev.info = &ad7791_no_filter_info;
    ret = devm_ad_sd_setup_buffer_and_trigger(dev, indio_dev);
    if (ret)
    return ret;
    ret = ad7791_setup(st, pdata);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id ad7791_spi_ids[] = {
    { .name = "ad7787", .driver_data = AD7787 },
    { .name = "ad7788", .driver_data = AD7788 },
    { .name = "ad7789", .driver_data = AD7789 },
    { .name = "ad7790", .driver_data = AD7790 },
    { .name = "ad7791", .driver_data = AD7791 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7791_spi_ids);
    static struct spi_driver ad7791_driver = {
    .driver = {
    .name	= "ad7791",
    },
    .probe		= ad7791_probe,
    .id_table	= ad7791_spi_ids,
    };
    module_spi_driver(ad7791_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("Analog Devices AD7787/AD7788/AD7789/AD7790/AD7791 ADC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_AD_SIGMA_DELTA");
