//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad4080.c
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
// Analog Devices AD4080 SPI ADC driver
//
// Copyright 2025 Analog Devices Inc.
//

// Register Definition
pub const AD4080_REG_INTERFACE_CONFIG_A: c_uint = 0x00;
pub const AD4080_REG_INTERFACE_CONFIG_B: c_uint = 0x01;
pub const AD4080_REG_DEVICE_CONFIG: c_uint = 0x02;
pub const AD4080_REG_CHIP_TYPE: c_uint = 0x03;
pub const AD4080_REG_PRODUCT_ID_L: c_uint = 0x04;
pub const AD4080_REG_PRODUCT_ID_H: c_uint = 0x05;
pub const AD4080_REG_CHIP_GRADE: c_uint = 0x06;
pub const AD4080_REG_SCRATCH_PAD: c_uint = 0x0A;
pub const AD4080_REG_SPI_REVISION: c_uint = 0x0B;
pub const AD4080_REG_VENDOR_L: c_uint = 0x0C;
pub const AD4080_REG_VENDOR_H: c_uint = 0x0D;
pub const AD4080_REG_STREAM_MODE: c_uint = 0x0E;
pub const AD4080_REG_TRANSFER_CONFIG: c_uint = 0x0F;
pub const AD4080_REG_INTERFACE_CONFIG_C: c_uint = 0x10;
pub const AD4080_REG_INTERFACE_STATUS_A: c_uint = 0x11;
pub const AD4080_REG_DEVICE_STATUS: c_uint = 0x14;
pub const AD4080_REG_ADC_DATA_INTF_CONFIG_A: c_uint = 0x15;
pub const AD4080_REG_ADC_DATA_INTF_CONFIG_B: c_uint = 0x16;
pub const AD4080_REG_ADC_DATA_INTF_CONFIG_C: c_uint = 0x17;
pub const AD4080_REG_PWR_CTRL: c_uint = 0x18;
pub const AD4080_REG_GPIO_CONFIG_A: c_uint = 0x19;
pub const AD4080_REG_GPIO_CONFIG_B: c_uint = 0x1A;
pub const AD4080_REG_GPIO_CONFIG_C: c_uint = 0x1B;
pub const AD4080_REG_GENERAL_CONFIG: c_uint = 0x1C;
pub const AD4080_REG_FIFO_WATERMARK_LSB: c_uint = 0x1D;
pub const AD4080_REG_FIFO_WATERMARK_MSB: c_uint = 0x1E;
pub const AD4080_REG_EVENT_HYSTERESIS_LSB: c_uint = 0x1F;
pub const AD4080_REG_EVENT_HYSTERESIS_MSB: c_uint = 0x20;
pub const AD4080_REG_EVENT_DETECTION_HI_LSB: c_uint = 0x21;
pub const AD4080_REG_EVENT_DETECTION_HI_MSB: c_uint = 0x22;
pub const AD4080_REG_EVENT_DETECTION_LO_LSB: c_uint = 0x23;
pub const AD4080_REG_EVENT_DETECTION_LO_MSB: c_uint = 0x24;
pub const AD4080_REG_OFFSET_LSB: c_uint = 0x25;
pub const AD4080_REG_OFFSET_MSB: c_uint = 0x26;
pub const AD4080_REG_GAIN_LSB: c_uint = 0x27;
pub const AD4080_REG_GAIN_MSB: c_uint = 0x28;
pub const AD4080_REG_FILTER_CONFIG: c_uint = 0x29;
// AD4080_REG_INTERFACE_CONFIG_A Bit Definition

// AD4080_REG_INTERFACE_CONFIG_B Bit Definition

// AD4080_REG_DEVICE_CONFIG Bit Definition

// AD4080_REG_TRANSFER_CONFIG Bit Definition

// AD4080_REG_INTERFACE_CONFIG_C Bit Definition

// AD4080_REG_ADC_DATA_INTF_CONFIG_A Bit Definition

// AD4080_REG_ADC_DATA_INTF_CONFIG_B Bit Definition

// AD4080_REG_ADC_DATA_INTF_CONFIG_C Bit Definition

// AD4080_REG_PWR_CTRL Bit Definition

// AD4080_REG_GPIO_CONFIG_A Bit Definition

// AD4080_REG_GPIO_CONFIG_B Bit Definition

pub const AD4080_GPIO_CONFIG_B_GPIO_SPI_SDO: c_int = 0;
pub const AD4080_GPIO_CONFIG_B_GPIO_FIFO_FULL: c_int = 1;
pub const AD4080_GPIO_CONFIG_B_GPIO_FIFO_READ_DONE: c_int = 2;
pub const AD4080_GPIO_CONFIG_B_GPIO_FILTER_RES_RDY: c_int = 3;
pub const AD4080_GPIO_CONFIG_B_GPIO_H_THRESH: c_int = 4;
pub const AD4080_GPIO_CONFIG_B_GPIO_L_THRESH: c_int = 5;
pub const AD4080_GPIO_CONFIG_B_GPIO_STATUS_ALERT: c_int = 6;
pub const AD4080_GPIO_CONFIG_B_GPIO_GPIO_DATA: c_int = 7;
pub const AD4080_GPIO_CONFIG_B_GPIO_FILTER_SYNC: c_int = 8;
pub const AD4080_GPIO_CONFIG_B_GPIO_EXTERNAL_EVENT: c_int = 9;
// AD4080_REG_FIFO_CONFIG Bit Definition

// AD4080_REG_FILTER_CONFIG Bit Definition

// Miscellaneous Definitions

pub const AD4080_CHIP_ID: c_uint = 0x0050;
pub const AD4081_CHIP_ID: c_uint = 0x0051;
pub const AD4082_CHIP_ID: c_uint = 0x0052;
pub const AD4083_CHIP_ID: c_uint = 0x0053;
pub const AD4084_CHIP_ID: c_uint = 0x0054;
pub const AD4085_CHIP_ID: c_uint = 0x0055;
pub const AD4086_CHIP_ID: c_uint = 0x0056;
pub const AD4087_CHIP_ID: c_uint = 0x0057;
pub const AD4088_CHIP_ID: c_uint = 0x0058;
pub const AD4880_CHIP_ID: c_uint = 0x0059;
pub const AD4883_CHIP_ID: c_uint = 0x005B;
pub const AD4884_CHIP_ID: c_uint = 0x005C;
pub const AD4080_MAX_CHANNELS: c_int = 2;
pub const AD4080_LVDS_CNV_CLK_CNT_MAX: c_int = 7;
pub const AD4080_MAX_SAMP_FREQ: c_int = 40000000;
pub const AD4080_MIN_SAMP_FREQ: c_int = 1250000;
    enum ad4080_filter_type {
    FILTER_NONE,
    SINC_1,
    SINC_5,
    SINC_5_COMP
    };
    static const unsigned int ad4080_scale_table[][2] = {
    { 6000, 0 },
    };
    static const char *const ad4080_filter_type_iio_enum[] = {
    [FILTER_NONE]      = "none",
    [SINC_1]           = "sinc1",
    [SINC_5]           = "sinc5",
    [SINC_5_COMP]      = "sinc5+pf1",
    };
    static const int ad4080_dec_rate_avail[] = {
    2, 4, 8, 16, 32, 64, 128, 256, 512, 1024,
    };
    static const int ad4080_dec_rate_none[] = { 1 };
    static const char * const ad4080_power_supplies[] = {
    "vdd33", "vdd11", "vddldo", "iovdd", "vrefin",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad4080_chip_info {
    pub name: *const c_char,
    pub product_id: c_uint,
    pub num_scales: c_int,
    pub (*scale_table)[2]: *const c_uint,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub lvds_cnv_clk_cnt_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad4080_state {
    pub spi: [*mut spi_device; AD4080_MAX_CHANNELS],
    pub regmap: [*mut regmap; AD4080_MAX_CHANNELS],
    pub back: [*mut iio_backend; AD4080_MAX_CHANNELS],
    pub info: *const ad4080_chip_info,
//
// Synchronize access to members the of driver state, and ensure
// atomicity of consecutive regmap operations.
//
    pub lock: mutex,
    pub num_lanes: c_uint,
    pub clk_rate: c_ulong,
    pub filter_type: [enum ad4080_filter_type; AD4080_MAX_CHANNELS],
    pub lvds_cnv_en: bool,
}

    static const struct regmap_config ad4080_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    .max_register = 0x29,
    };
    static int ad4080_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap[0], reg, readval);
    return regmap_write(st.regmap[0], reg, writeval);
    }
#[no_mangle]
unsafe extern "C" fn ad4080_get_scale(st: *mut ad4080_state, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int ad4080_get_scale(struct ad4080_state *st, int *val, int *val2)
    {
    unsigned int tmp;
    tmp = (st.info.scale_table[0][0] * 1000000ULL) >>
    st.info.channels[0].scan_type.realbits;
// val = tmp / 1000000;
// val2 = tmp % 1000000;
    return IIO_VAL_INT_PLUS_NANO;
    }
    static unsigned int ad4080_get_dec_rate(struct iio_dev *dev,
    const struct iio_chan_spec *chan)
    {
    struct ad4080_state *st = iio_priv(dev);
    int ret;
    unsigned int data;
    let mut ch: c_uint = chan.channel;
    ret = regmap_read(st.regmap[ch], AD4080_REG_FILTER_CONFIG, &data);
    if (ret)
    return ret;
    return 1 << (FIELD_GET(AD4080_FILTER_CONFIG_SINC_DEC_RATE_MSK, data) + 1);
    }
    static int ad4080_set_dec_rate(struct iio_dev *dev,
    const struct iio_chan_spec *chan,
    unsigned int mode)
    {
    struct ad4080_state *st = iio_priv(dev);
    let mut ch: c_uint = chan.channel;
    guard(mutex)(&st.lock);
    if ((st.filter_type[ch] >= SINC_5 && mode >= 512) || mode < 2)
    return -EINVAL;
    return regmap_update_bits(st.regmap[ch], AD4080_REG_FILTER_CONFIG,
    AD4080_FILTER_CONFIG_SINC_DEC_RATE_MSK,
    FIELD_PREP(AD4080_FILTER_CONFIG_SINC_DEC_RATE_MSK,
    (ilog2(mode) - 1)));
    }
    static int ad4080_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long m)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    int dec_rate;
    switch (m) {
    case IIO_CHAN_INFO_SCALE:
    return ad4080_get_scale(st, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    dec_rate = ad4080_get_dec_rate(indio_dev, chan);
    if (dec_rate < 0)
    return dec_rate;
    if (st.filter_type[chan.channel] == SINC_5_COMP)
    dec_rate *= 2;
    if (st.filter_type[chan.channel])
// val = DIV_ROUND_CLOSEST(st->clk_rate, dec_rate);
    else
// val = st->clk_rate;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
    if (st.filter_type[chan.channel] == FILTER_NONE) {
// val = 1;
    } else {
// val = ad4080_get_dec_rate(indio_dev, chan);
    if (*val < 0)
    return *val;
    }
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int ad4080_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
    if (st.filter_type[chan.channel] == FILTER_NONE && val > 1)
    return -EINVAL;
    return ad4080_set_dec_rate(indio_dev, chan, val);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ad4080_lvds_sync_write(st: *mut ad4080_state, ch: c_uint) -> c_int {
    static int ad4080_lvds_sync_write(struct ad4080_state *st, unsigned int ch)
    {
    struct device *dev = regmap_get_device(st.regmap[ch]);
    int ret;
    ret = regmap_set_bits(st.regmap[ch], AD4080_REG_ADC_DATA_INTF_CONFIG_A,
    AD4080_ADC_DATA_INTF_CONFIG_A_INTF_CHK_EN);
    if (ret)
    return ret;
    ret = iio_backend_interface_data_align(st.back[ch], 10000);
    if (ret)
    return dev_err_probe(dev, ret,
    "Data alignment process failed\n");
    dev_dbg(dev, "Success: Pattern correct and Locked!\n");
    return regmap_clear_bits(st.regmap[ch], AD4080_REG_ADC_DATA_INTF_CONFIG_A,
    AD4080_ADC_DATA_INTF_CONFIG_A_INTF_CHK_EN);
    }
    static int ad4080_get_filter_type(struct iio_dev *dev,
    const struct iio_chan_spec *chan)
    {
    struct ad4080_state *st = iio_priv(dev);
    unsigned int data;
    let mut ch: c_uint = chan.channel;
    int ret;
    ret = regmap_read(st.regmap[ch], AD4080_REG_FILTER_CONFIG, &data);
    if (ret)
    return ret;
    return FIELD_GET(AD4080_FILTER_CONFIG_FILTER_SEL_MSK, data);
    }
    static int ad4080_set_filter_type(struct iio_dev *dev,
    const struct iio_chan_spec *chan,
    unsigned int mode)
    {
    struct ad4080_state *st = iio_priv(dev);
    let mut ch: c_uint = chan.channel;
    int dec_rate;
    int ret;
    guard(mutex)(&st.lock);
    dec_rate = ad4080_get_dec_rate(dev, chan);
    if (dec_rate < 0)
    return dec_rate;
    if (mode >= SINC_5 && dec_rate >= 512)
    return -EINVAL;
    ret = iio_backend_filter_type_set(st.back[ch], mode);
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap[ch], AD4080_REG_FILTER_CONFIG,
    AD4080_FILTER_CONFIG_FILTER_SEL_MSK,
    FIELD_PREP(AD4080_FILTER_CONFIG_FILTER_SEL_MSK,
    mode));
    if (ret)
    return ret;
    st.filter_type[ch] = mode;
    return 0;
    }
    static int ad4080_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
    switch (st.filter_type[chan.channel]) {
    case FILTER_NONE:
// vals = ad4080_dec_rate_none;
// length = ARRAY_SIZE(ad4080_dec_rate_none);
    break;
    default:
// vals = ad4080_dec_rate_avail;
// length = st->filter_type[chan->channel] >= SINC_5 ?
    (ARRAY_SIZE(ad4080_dec_rate_avail) - 2) :
    ARRAY_SIZE(ad4080_dec_rate_avail);
    break;
    }
// type = IIO_VAL_INT;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int ad4880_update_scan_mode(struct iio_dev *indio_dev,
    const unsigned long *scan_mask)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    int ret;
    for (unsigned int ch = 0; ch < st.info.num_channels; ch++) {
//
// Each backend has a single channel (channel 0 from the
// backend's perspective), so always use channel index 0.
//
    if (test_bit(ch, scan_mask))
    ret = iio_backend_chan_enable(st.back[ch], 0);
    else
    ret = iio_backend_chan_disable(st.back[ch], 0);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct iio_info ad4080_iio_info = {
    .debugfs_reg_access = ad4080_reg_access,
    .read_raw = ad4080_read_raw,
    .write_raw = ad4080_write_raw,
    .read_avail = ad4080_read_avail,
    };
//
// AD4880 needs update_scan_mode to enable/disable individual backend channels.
// Single-channel devices don't need this as their backends may not implement
// chan_enable/chan_disable operations.
//
    static const struct iio_info ad4880_iio_info = {
    .debugfs_reg_access = ad4080_reg_access,
    .read_raw = ad4080_read_raw,
    .write_raw = ad4080_write_raw,
    .read_avail = ad4080_read_avail,
    .update_scan_mode = ad4880_update_scan_mode,
    };
    static const struct iio_enum ad4080_filter_type_enum = {
    .items = ad4080_filter_type_iio_enum,
    .num_items = ARRAY_SIZE(ad4080_filter_type_iio_enum),
    .set = ad4080_set_filter_type,
    .get = ad4080_get_filter_type,
    };
    static struct iio_chan_spec_ext_info ad4080_ext_info[] = {
    IIO_ENUM("filter_type", IIO_SHARED_BY_ALL, &ad4080_filter_type_enum),
    IIO_ENUM_AVAILABLE("filter_type", IIO_SHARED_BY_ALL,
    &ad4080_filter_type_enum),
    { }
    };
//
// AD4880 needs per-channel filter configuration since each channel has
// its own independent ADC with separate SPI interface.
//
    static struct iio_chan_spec_ext_info ad4880_ext_info[] = {
    IIO_ENUM("filter_type", IIO_SEPARATE, &ad4080_filter_type_enum),
    IIO_ENUM_AVAILABLE("filter_type", IIO_SEPARATE,
    &ad4080_filter_type_enum),
    { }
    };

    .type = IIO_VOLTAGE,						\
    .indexed = 1,							\
    .channel = (idx),						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_SCALE),			\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ) |	\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),		\
    .info_mask_shared_by_all_available =				\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),		\
    .ext_info = ad4080_ext_info,					\
    .scan_index = (idx),						\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = (bits),					\
    .storagebits = (storage),				\
    },								\
    }
//
// AD4880 has per-channel attributes (filter_type, oversampling_ratio,
// sampling_frequency) since each channel has its own independent ADC
// with separate SPI configuration interface.
//

    .type = IIO_VOLTAGE,						\
    .indexed = 1,							\
    .channel = (idx),						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_SAMP_FREQ) |			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),		\
    .info_mask_separate_available =					\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),		\
    .ext_info = ad4880_ext_info,				\
    .scan_index = (idx),						\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = (bits),					\
    .storagebits = (storage),				\
    },								\
    }
    let mut ad4080_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(20, 32, 0);
    let mut ad4081_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(20, 32, 0);
    let mut ad4082_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(20, 32, 0);
    let mut ad4083_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(16, 16, 0);
    let mut ad4084_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(16, 16, 0);
    let mut ad4085_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(16, 16, 0);
    let mut ad4086_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(14, 16, 0);
    let mut ad4087_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(14, 16, 0);
    let mut ad4088_channel: static struct iio_chan_spec = AD4080_CHANNEL_DEFINE(14, 16, 0);
    static const struct iio_chan_spec ad4880_channels[] = {
    AD4880_CHANNEL_DEFINE(20, 32, 0),
    AD4880_CHANNEL_DEFINE(20, 32, 1),
    };
    static const struct iio_chan_spec ad4883_channels[] = {
    AD4880_CHANNEL_DEFINE(16, 16, 0),
    AD4880_CHANNEL_DEFINE(16, 16, 1),
    };
    static const struct iio_chan_spec ad4884_channels[] = {
    AD4880_CHANNEL_DEFINE(16, 16, 0),
    AD4880_CHANNEL_DEFINE(16, 16, 1),
    };
    static const struct ad4080_chip_info ad4080_chip_info = {
    .name = "ad4080",
    .product_id = AD4080_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4080_channel,
    .lvds_cnv_clk_cnt_max = AD4080_LVDS_CNV_CLK_CNT_MAX,
    };
    static const struct ad4080_chip_info ad4081_chip_info = {
    .name = "ad4081",
    .product_id = AD4081_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4081_channel,
    .lvds_cnv_clk_cnt_max = 2,
    };
    static const struct ad4080_chip_info ad4082_chip_info = {
    .name = "ad4082",
    .product_id = AD4082_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4082_channel,
    .lvds_cnv_clk_cnt_max = 8,
    };
    static const struct ad4080_chip_info ad4083_chip_info = {
    .name = "ad4083",
    .product_id = AD4083_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4083_channel,
    .lvds_cnv_clk_cnt_max = 5,
    };
    static const struct ad4080_chip_info ad4084_chip_info = {
    .name = "ad4084",
    .product_id = AD4084_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4084_channel,
    .lvds_cnv_clk_cnt_max = 2,
    };
    static const struct ad4080_chip_info ad4085_chip_info = {
    .name = "ad4085",
    .product_id = AD4085_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4085_channel,
    .lvds_cnv_clk_cnt_max = 8,
    };
    static const struct ad4080_chip_info ad4086_chip_info = {
    .name = "ad4086",
    .product_id = AD4086_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4086_channel,
    .lvds_cnv_clk_cnt_max = 4,
    };
    static const struct ad4080_chip_info ad4087_chip_info = {
    .name = "ad4087",
    .product_id = AD4087_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4087_channel,
    .lvds_cnv_clk_cnt_max = 1,
    };
    static const struct ad4080_chip_info ad4088_chip_info = {
    .name = "ad4088",
    .product_id = AD4088_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 1,
    .channels = &ad4088_channel,
    .lvds_cnv_clk_cnt_max = 8,
    };
    static const struct ad4080_chip_info ad4880_chip_info = {
    .name = "ad4880",
    .product_id = AD4880_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 2,
    .channels = ad4880_channels,
    .lvds_cnv_clk_cnt_max = AD4080_LVDS_CNV_CLK_CNT_MAX,
    };
    static const struct ad4080_chip_info ad4883_chip_info = {
    .name = "ad4883",
    .product_id = AD4883_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 2,
    .channels = ad4883_channels,
    .lvds_cnv_clk_cnt_max = 5,
    };
    static const struct ad4080_chip_info ad4884_chip_info = {
    .name = "ad4884",
    .product_id = AD4884_CHIP_ID,
    .scale_table = ad4080_scale_table,
    .num_scales = ARRAY_SIZE(ad4080_scale_table),
    .num_channels = 2,
    .channels = ad4884_channels,
    .lvds_cnv_clk_cnt_max = 2,
    };
#[no_mangle]
unsafe extern "C" fn ad4080_setup_channel(st: *mut ad4080_state, ch: c_uint) -> c_int {
    static int ad4080_setup_channel(struct ad4080_state *st, unsigned int ch)
    {
    struct device *dev = regmap_get_device(st.regmap[ch]);
    __le16 id_le;
    u16 id;
    int ret;
    ret = regmap_write(st.regmap[ch], AD4080_REG_INTERFACE_CONFIG_A,
    AD4080_INTERFACE_CONFIG_A_SW_RESET);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap[ch], AD4080_REG_INTERFACE_CONFIG_A,
    AD4080_INTERFACE_CONFIG_A_SDO_ENABLE);
    if (ret)
    return ret;
    ret = regmap_bulk_read(st.regmap[ch], AD4080_REG_PRODUCT_ID_L, &id_le,
    sizeof(id_le));
    if (ret)
    return ret;
    id = le16_to_cpu(id_le);
    if (id != st.info.product_id)
    dev_info(dev, "Unrecognized CHIP_ID 0x%X\n", id);
    ret = regmap_set_bits(st.regmap[ch], AD4080_REG_GPIO_CONFIG_A,
    AD4080_GPIO_CONFIG_A_GPO_1_EN);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap[ch], AD4080_REG_GPIO_CONFIG_B,
    FIELD_PREP(AD4080_GPIO_CONFIG_B_GPIO_1_SEL_MSK,
    AD4080_GPIO_CONFIG_B_GPIO_FILTER_RES_RDY));
    if (ret)
    return ret;
    ret = iio_backend_num_lanes_set(st.back[ch], st.num_lanes);
    if (ret)
    return ret;
    ret = iio_backend_data_size_set(st.back[ch],
    st.info.channels[0].scan_type.realbits);
    if (ret)
    return ret;
    if (!st.lvds_cnv_en)
    return 0;
// Set maximum LVDS Data Transfer Latency
    ret = regmap_update_bits(st.regmap[ch],
    AD4080_REG_ADC_DATA_INTF_CONFIG_B,
    AD4080_ADC_DATA_INTF_CONFIG_B_LVDS_CNV_CLK_CNT_MSK,
    FIELD_PREP(AD4080_ADC_DATA_INTF_CONFIG_B_LVDS_CNV_CLK_CNT_MSK,
    st.info.lvds_cnv_clk_cnt_max));
    if (ret)
    return ret;
    if (st.num_lanes > 1) {
    ret = regmap_set_bits(st.regmap[ch], AD4080_REG_ADC_DATA_INTF_CONFIG_A,
    AD4080_ADC_DATA_INTF_CONFIG_A_SPI_LVDS_LANES);
    if (ret)
    return ret;
    }
    ret = regmap_set_bits(st.regmap[ch],
    AD4080_REG_ADC_DATA_INTF_CONFIG_B,
    AD4080_ADC_DATA_INTF_CONFIG_B_LVDS_CNV_EN);
    if (ret)
    return ret;
    return ad4080_lvds_sync_write(st, ch);
    }
#[no_mangle]
unsafe extern "C" fn ad4080_setup(indio_dev: *mut iio_dev) -> c_int {
    static int ad4080_setup(struct iio_dev *indio_dev)
    {
    struct ad4080_state *st = iio_priv(indio_dev);
    int ret;
    for (unsigned int ch = 0; ch < st.info.num_channels; ch++) {
    ret = ad4080_setup_channel(st, ch);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int ad4080_properties_parse(struct ad4080_state *st,
    struct device *dev)
    {
    st.lvds_cnv_en = device_property_read_bool(dev, "adi,lvds-cnv-enable");
    st.num_lanes = 1;
    device_property_read_u32(dev, "adi,num-lanes", &st.num_lanes);
    if (!st.num_lanes || st.num_lanes > 2)
    return dev_err_probe(dev, -EINVAL,
    "Invalid 'adi,num-lanes' value: %u",
    st.num_lanes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4080_probe(spi: *mut spi_device) -> c_int {
    static int ad4080_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct device *dev = &spi.dev;
    struct ad4080_state *st;
    struct clk *clk;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    ret = devm_regulator_bulk_get_enable(dev,
    ARRAY_SIZE(ad4080_power_supplies),
    ad4080_power_supplies);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get and enable supplies\n");
// Setup primary SPI device (channel 0)
    st.spi[0] = spi;
    st.regmap[0] = devm_regmap_init_spi(spi, &ad4080_regmap_config);
    if (IS_ERR(st.regmap[0]))
    return PTR_ERR(st.regmap[0]);
    st.info = spi_get_device_match_data(spi);
    if (!st.info)
    return -ENODEV;
// Setup ancillary SPI devices for additional channels
    for (unsigned int ch = 1; ch < st.info.num_channels; ch++) {
    st.spi[ch] = devm_spi_new_ancillary_device(spi, spi_get_chipselect(spi, ch));
    if (IS_ERR(st.spi[ch]))
    return dev_err_probe(dev, PTR_ERR(st.spi[ch]),
    "failed to register ancillary device\n");
    st.regmap[ch] = devm_regmap_init_spi(st.spi[ch], &ad4080_regmap_config);
    if (IS_ERR(st.regmap[ch]))
    return PTR_ERR(st.regmap[ch]);
    }
    ret = devm_mutex_init(dev, &st.lock);
    if (ret)
    return ret;
    indio_dev.name = st.info.name;
    indio_dev.channels = st.info.channels;
    indio_dev.num_channels = st.info.num_channels;
    indio_dev.info = st.info.num_channels > 1 ?
    &ad4880_iio_info : &ad4080_iio_info;
    ret = ad4080_properties_parse(st, dev);
    if (ret)
    return ret;
    clk = devm_clk_get_enabled(&spi.dev, "cnv");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    st.clk_rate = clk_get_rate(clk);
// Get backends for all channels
    for (unsigned int ch = 0; ch < st.info.num_channels; ch++) {
    st.back[ch] = devm_iio_backend_get_by_index(dev, ch);
    if (IS_ERR(st.back[ch]))
    return PTR_ERR(st.back[ch]);
    ret = devm_iio_backend_enable(dev, st.back[ch]);
    if (ret)
    return ret;
    }
//
// Request buffer from the first backend only. For multi-channel
// devices (e.g., AD4880), the FPGA uses two axi_ad408x IP instances
// (one per ADC channel) whose outputs are combined by a packer block
// that interleaves all channel data into a single DMA stream routed
// through the first backend's clock domain.
//
    ret = devm_iio_backend_request_buffer(dev, st.back[0], indio_dev);
    if (ret)
    return ret;
    ret = ad4080_setup(indio_dev);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id ad4080_id[] = {
    { .name = "ad4080", .driver_data = (kernel_ulong_t)&ad4080_chip_info },
    { .name = "ad4081", .driver_data = (kernel_ulong_t)&ad4081_chip_info },
    { .name = "ad4082", .driver_data = (kernel_ulong_t)&ad4082_chip_info },
    { .name = "ad4083", .driver_data = (kernel_ulong_t)&ad4083_chip_info },
    { .name = "ad4084", .driver_data = (kernel_ulong_t)&ad4084_chip_info },
    { .name = "ad4085", .driver_data = (kernel_ulong_t)&ad4085_chip_info },
    { .name = "ad4086", .driver_data = (kernel_ulong_t)&ad4086_chip_info },
    { .name = "ad4087", .driver_data = (kernel_ulong_t)&ad4087_chip_info },
    { .name = "ad4088", .driver_data = (kernel_ulong_t)&ad4088_chip_info },
    { .name = "ad4880", .driver_data = (kernel_ulong_t)&ad4880_chip_info },
    { .name = "ad4883", .driver_data = (kernel_ulong_t)&ad4883_chip_info },
    { .name = "ad4884", .driver_data = (kernel_ulong_t)&ad4884_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad4080_id);
    static const struct of_device_id ad4080_of_match[] = {
    { .compatible = "adi,ad4080", &ad4080_chip_info },
    { .compatible = "adi,ad4081", &ad4081_chip_info },
    { .compatible = "adi,ad4082", &ad4082_chip_info },
    { .compatible = "adi,ad4083", &ad4083_chip_info },
    { .compatible = "adi,ad4084", &ad4084_chip_info },
    { .compatible = "adi,ad4085", &ad4085_chip_info },
    { .compatible = "adi,ad4086", &ad4086_chip_info },
    { .compatible = "adi,ad4087", &ad4087_chip_info },
    { .compatible = "adi,ad4088", &ad4088_chip_info },
    { .compatible = "adi,ad4880", &ad4880_chip_info },
    { .compatible = "adi,ad4883", &ad4883_chip_info },
    { .compatible = "adi,ad4884", &ad4884_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad4080_of_match);
    static struct spi_driver ad4080_driver = {
    .driver = {
    .name = "ad4080",
    .of_match_table = ad4080_of_match,
    },
    .probe = ad4080_probe,
    .id_table = ad4080_id,
    };
    module_spi_driver(ad4080_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com");
    MODULE_DESCRIPTION("Analog Devices AD4080");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BACKEND");
