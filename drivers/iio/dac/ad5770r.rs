//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5770r.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// AD5770R Digital to analog converters driver
//
// Copyright 2018 Analog Devices Inc.
//

pub const ADI_SPI_IF_CONFIG_A: c_uint = 0x00;
pub const ADI_SPI_IF_CONFIG_B: c_uint = 0x01;
pub const ADI_SPI_IF_DEVICE_CONFIG: c_uint = 0x02;
pub const ADI_SPI_IF_CHIP_TYPE: c_uint = 0x03;
pub const ADI_SPI_IF_PRODUCT_ID_L: c_uint = 0x04;
pub const ADI_SPI_IF_PRODUCT_ID_H: c_uint = 0x05;
pub const ADI_SPI_IF_CHIP_GRADE: c_uint = 0x06;
pub const ADI_SPI_IF_SCRACTH_PAD: c_uint = 0x0A;
pub const ADI_SPI_IF_SPI_REVISION: c_uint = 0x0B;
pub const ADI_SPI_IF_SPI_VENDOR_L: c_uint = 0x0C;
pub const ADI_SPI_IF_SPI_VENDOR_H: c_uint = 0x0D;
pub const ADI_SPI_IF_SPI_STREAM_MODE: c_uint = 0x0E;
pub const ADI_SPI_IF_CONFIG_C: c_uint = 0x10;
pub const ADI_SPI_IF_STATUS_A: c_uint = 0x11;
// ADI_SPI_IF_CONFIG_A

// ADI_SPI_IF_CONFIG_B

// ADI_SPI_IF_CONFIG_C

// AD5770R configuration registers
pub const AD5770R_CHANNEL_CONFIG: c_uint = 0x14;

pub const AD5770R_REFERENCE: c_uint = 0x1B;

pub const AD5770R_CH_SELECT: c_uint = 0x34;
pub const AD5770R_CH_ENABLE: c_uint = 0x44;
// AD5770R_CHANNEL_CONFIG

// AD5770R_OUTPUT_RANGE

// AD5770R_REFERENCE

// AD5770R_CH_ENABLE

pub const AD5770R_MAX_CHANNELS: c_int = 6;
pub const AD5770R_MAX_CH_MODES: c_int = 14;
pub const AD5770R_LOW_VREF_mV: c_int = 1250;
pub const AD5770R_HIGH_VREF_mV: c_int = 2500;
    enum ad5770r_ch0_modes {
    AD5770R_CH0_0_300 = 0,
    AD5770R_CH0_NEG_60_0,
    AD5770R_CH0_NEG_60_300
    };
    enum ad5770r_ch1_modes {
    AD5770R_CH1_0_140_LOW_HEAD = 1,
    AD5770R_CH1_0_140_LOW_NOISE,
    AD5770R_CH1_0_250
    };
    enum ad5770r_ch2_5_modes {
    AD5770R_CH_LOW_RANGE = 0,
    AD5770R_CH_HIGH_RANGE
    };
    enum ad5770r_ref_v {
    AD5770R_EXT_2_5_V = 0,
    AD5770R_INT_1_25_V_OUT_ON,
    AD5770R_EXT_1_25_V,
    AD5770R_INT_1_25_V_OUT_OFF
    };
    enum ad5770r_output_filter_resistor {
    AD5770R_FILTER_60_OHM = 0x0,
    AD5770R_FILTER_5_6_KOHM = 0x5,
    AD5770R_FILTER_11_2_KOHM,
    AD5770R_FILTER_22_2_KOHM,
    AD5770R_FILTER_44_4_KOHM,
    AD5770R_FILTER_104_KOHM,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5770r_out_range {
    pub out_scale: u8,
    pub out_range_mode: u8,
}

//
// struct ad5770r_state - driver instance specific data
// @spi:		spi_device
// @regmap:		regmap
// @gpio_reset:		gpio descriptor
// @output_mode:	array contains channels output ranges
// @vref:		reference value
// @ch_pwr_down:	powerdown flags
// @internal_ref:	internal reference flag
// @external_res:	external 2.5k resistor flag
// @transf_buf:		cache aligned buffer for spi read/write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5770r_state {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub gpio_reset: *mut gpio_desc,
    pub output_mode: [ad5770r_out_range; AD5770R_MAX_CHANNELS],
    pub vref: c_int,
    pub ch_pwr_down: [bool; AD5770R_MAX_CHANNELS],
    pub internal_ref: bool,
    pub external_res: bool,
    pub __aligned(IIO_DMA_MINALIGN): u8 transf_buf[2],
}

    static const struct regmap_config ad5770r_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5770r_output_modes {
    pub ch: c_uint,
    pub mode: u8,
    pub min: c_int,
    pub max: c_int,
}

    static const struct ad5770r_output_modes ad5770r_rng_tbl[] = {
    { 0, AD5770R_CH0_0_300, 0, 300 },
    { 0, AD5770R_CH0_NEG_60_0, -60, 0 },
    { 0, AD5770R_CH0_NEG_60_300, -60, 300 },
    { 1, AD5770R_CH1_0_140_LOW_HEAD, 0, 140 },
    { 1, AD5770R_CH1_0_140_LOW_NOISE, 0, 140 },
    { 1, AD5770R_CH1_0_250, 0, 250 },
    { 2, AD5770R_CH_LOW_RANGE, 0, 55 },
    { 2, AD5770R_CH_HIGH_RANGE, 0, 150 },
    { 3, AD5770R_CH_LOW_RANGE, 0, 45 },
    { 3, AD5770R_CH_HIGH_RANGE, 0, 100 },
    { 4, AD5770R_CH_LOW_RANGE, 0, 45 },
    { 4, AD5770R_CH_HIGH_RANGE, 0, 100 },
    { 5, AD5770R_CH_LOW_RANGE, 0, 45 },
    { 5, AD5770R_CH_HIGH_RANGE, 0, 100 },
    };
    static const unsigned int ad5770r_filter_freqs[] = {
    153, 357, 715, 1400, 2800, 262000,
    };
    static const unsigned int ad5770r_filter_reg_vals[] = {
    AD5770R_FILTER_104_KOHM,
    AD5770R_FILTER_44_4_KOHM,
    AD5770R_FILTER_22_2_KOHM,
    AD5770R_FILTER_11_2_KOHM,
    AD5770R_FILTER_5_6_KOHM,
    AD5770R_FILTER_60_OHM
    };
    static int ad5770r_set_output_mode(struct ad5770r_state *st,
    const struct ad5770r_out_range *out_mode,
    int channel)
    {
    unsigned int regval;
    regval = AD5770R_RANGE_OUTPUT_SCALING(out_mode.out_scale) |
    AD5770R_RANGE_MODE(out_mode.out_range_mode);
    return regmap_write(st.regmap,
    AD5770R_OUTPUT_RANGE(channel), regval);
    }
#[no_mangle]
unsafe extern "C" fn ad5770r_set_reference(st: *mut ad5770r_state) -> c_int {
    static int ad5770r_set_reference(struct ad5770r_state *st)
    {
    unsigned int regval;
    regval = AD5770R_REF_RESISTOR_SEL(st.external_res);
    if (st.internal_ref) {
    regval |= AD5770R_REF_SEL(AD5770R_INT_1_25_V_OUT_OFF);
    } else {
    switch (st.vref) {
    case AD5770R_LOW_VREF_mV:
    regval |= AD5770R_REF_SEL(AD5770R_EXT_1_25_V);
    break;
    case AD5770R_HIGH_VREF_mV:
    regval |= AD5770R_REF_SEL(AD5770R_EXT_2_5_V);
    break;
    default:
    regval = AD5770R_REF_SEL(AD5770R_INT_1_25_V_OUT_OFF);
    break;
    }
    }
    return regmap_write(st.regmap, AD5770R_REFERENCE, regval);
    }
#[no_mangle]
unsafe extern "C" fn ad5770r_soft_reset(st: *mut ad5770r_state) -> c_int {
    static int ad5770r_soft_reset(struct ad5770r_state *st)
    {
    return regmap_write(st.regmap, ADI_SPI_IF_CONFIG_A,
    ADI_SPI_IF_SW_RESET_SEL(1));
    }
#[no_mangle]
unsafe extern "C" fn ad5770r_reset(st: *mut ad5770r_state) -> c_int {
    static int ad5770r_reset(struct ad5770r_state *st)
    {
// Perform software reset if no GPIO provided
    if (!st.gpio_reset)
    return ad5770r_soft_reset(st);
    gpiod_set_value_cansleep(st.gpio_reset, 0);
    usleep_range(10, 20);
    gpiod_set_value_cansleep(st.gpio_reset, 1);
// data must not be written during reset timeframe
    usleep_range(100, 200);
    return 0;
    }
    static int ad5770r_get_range(struct ad5770r_state *st,
    int ch, int *min, int *max)
    {
    int i;
    u8 tbl_ch, tbl_mode, out_range;
    out_range = st.output_mode[ch].out_range_mode;
    for (i = 0; i < AD5770R_MAX_CH_MODES; i++) {
    tbl_ch = ad5770r_rng_tbl[i].ch;
    tbl_mode = ad5770r_rng_tbl[i].mode;
    if (tbl_ch == ch && tbl_mode == out_range) {
// min = ad5770r_rng_tbl[i].min;
// max = ad5770r_rng_tbl[i].max;
    return 0;
    }
    }
    return -EINVAL;
    }
    static int ad5770r_get_filter_freq(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int *freq)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    int ret;
    unsigned int regval, i;
    ret = regmap_read(st.regmap,
    AD5770R_FILTER_RESISTOR(chan.channel), &regval);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(ad5770r_filter_reg_vals); i++)
    if (regval == ad5770r_filter_reg_vals[i])
    break;
    if (i == ARRAY_SIZE(ad5770r_filter_reg_vals))
    return -EINVAL;
// freq = ad5770r_filter_freqs[i];
    return IIO_VAL_INT;
    }
    static int ad5770r_set_filter_freq(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    unsigned int freq)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    unsigned int regval, i;
    for (i = 0; i < ARRAY_SIZE(ad5770r_filter_freqs); i++)
    if (ad5770r_filter_freqs[i] >= freq)
    break;
    if (i == ARRAY_SIZE(ad5770r_filter_freqs))
    return -EINVAL;
    regval = ad5770r_filter_reg_vals[i];
    return regmap_write(st.regmap, AD5770R_FILTER_RESISTOR(chan.channel),
    regval);
    }
    static int ad5770r_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    int max, min, ret;
    u16 buf16;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    ret = regmap_bulk_read(st.regmap,
    chan.address,
    st.transf_buf, 2);
    if (ret)
    return ret;
    buf16 = get_unaligned_le16(st.transf_buf);
// val = buf16 >> 2;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = ad5770r_get_range(st, chan.channel, &min, &max);
    if (ret < 0)
    return ret;
// val = max - min;
// There is no sign bit. (negative current is mapped from 0)
// (sourced/sinked) current = raw * scale + offset
// where offset in case of CH0 can be negative.
//
// val2 = 14;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
    return ad5770r_get_filter_freq(indio_dev, chan, val);
    case IIO_CHAN_INFO_OFFSET:
    ret = ad5770r_get_range(st, chan.channel, &min, &max);
    if (ret < 0)
    return ret;
// val = min;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int ad5770r_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long info)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    st.transf_buf[0] = ((u16)val >> 6);
    st.transf_buf[1] = (val & GENMASK(5, 0)) << 2;
    return regmap_bulk_write(st.regmap, chan.address,
    st.transf_buf, 2);
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
    return ad5770r_set_filter_freq(indio_dev, chan, val);
    default:
    return -EINVAL;
    }
    }
    static int ad5770r_read_freq_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
// type = IIO_VAL_INT;
// vals = ad5770r_filter_freqs;
// length = ARRAY_SIZE(ad5770r_filter_freqs);
    return IIO_AVAIL_LIST;
    }
    return -EINVAL;
    }
    static int ad5770r_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int writeval,
    unsigned int *readval)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    else
    return regmap_write(st.regmap, reg, writeval);
    }
    static const struct iio_info ad5770r_info = {
    .read_raw = ad5770r_read_raw,
    .write_raw = ad5770r_write_raw,
    .read_avail = ad5770r_read_freq_avail,
    .debugfs_reg_access = &ad5770r_reg_access,
    };
    static int ad5770r_store_output_range(struct ad5770r_state *st,
    int min, int max, int index)
    {
    int i;
    for (i = 0; i < AD5770R_MAX_CH_MODES; i++) {
    if (ad5770r_rng_tbl[i].ch != index)
    continue;
    if (ad5770r_rng_tbl[i].min != min ||
    ad5770r_rng_tbl[i].max != max)
    continue;
    st.output_mode[index].out_range_mode = ad5770r_rng_tbl[i].mode;
    return 0;
    }
    return -EINVAL;
    }
    static ssize_t ad5770r_read_dac_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    return sysfs_emit(buf, "%d\n", st.ch_pwr_down[chan.channel]);
    }
    static ssize_t ad5770r_write_dac_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct ad5770r_state *st = iio_priv(indio_dev);
    unsigned int regval;
    unsigned int mask;
    bool readin;
    int ret;
    ret = kstrtobool(buf, &readin);
    if (ret)
    return ret;
    readin = !readin;
    regval = AD5770R_CFG_SHUTDOWN_B(readin, chan.channel);
    if (chan.channel == 0 &&
    st.output_mode[0].out_range_mode > AD5770R_CH0_0_300) {
    regval |= AD5770R_CFG_CH0_SINK_EN(readin);
    mask = BIT(chan.channel) + BIT(7);
    } else {
    mask = BIT(chan.channel);
    }
    ret = regmap_update_bits(st.regmap, AD5770R_CHANNEL_CONFIG, mask,
    regval);
    if (ret)
    return ret;
    regval = AD5770R_CH_SET(readin, chan.channel);
    ret = regmap_update_bits(st.regmap, AD5770R_CH_ENABLE,
    BIT(chan.channel), regval);
    if (ret)
    return ret;
    st.ch_pwr_down[chan.channel] = !readin;
    return len;
    }
    static const struct iio_chan_spec_ext_info ad5770r_ext_info[] = {
    {
    .name = "powerdown",
    .read = ad5770r_read_dac_powerdown,
    .write = ad5770r_write_dac_powerdown,
    .shared = IIO_SEPARATE,
    },
    { }
    };

    .type = IIO_CURRENT,						\
    .address = reg,							\
    .indexed = 1,							\
    .channel = index,						\
    .output = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |				\
    BIT(IIO_CHAN_INFO_OFFSET) |				\
    BIT(IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY),	\
    .info_mask_shared_by_type_available =				\
    BIT(IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY),	\
    .ext_info = ad5770r_ext_info,					\
    }
    static const struct iio_chan_spec ad5770r_channels[] = {
    AD5770R_IDAC_CHANNEL(0, AD5770R_DAC_MSB(0)),
    AD5770R_IDAC_CHANNEL(1, AD5770R_DAC_MSB(1)),
    AD5770R_IDAC_CHANNEL(2, AD5770R_DAC_MSB(2)),
    AD5770R_IDAC_CHANNEL(3, AD5770R_DAC_MSB(3)),
    AD5770R_IDAC_CHANNEL(4, AD5770R_DAC_MSB(4)),
    AD5770R_IDAC_CHANNEL(5, AD5770R_DAC_MSB(5)),
    };
#[no_mangle]
unsafe extern "C" fn ad5770r_channel_config(st: *mut ad5770r_state) -> c_int {
    static int ad5770r_channel_config(struct ad5770r_state *st)
    {
    int ret, tmp[2], min, max;
    unsigned int num;
    num = device_get_child_node_count(&st.spi.dev);
    if (num != AD5770R_MAX_CHANNELS)
    return -EINVAL;
    device_for_each_child_node_scoped(&st.spi.dev, child) {
    ret = fwnode_property_read_u32(child, "reg", &num);
    if (ret)
    return ret;
    if (num >= AD5770R_MAX_CHANNELS)
    return -EINVAL;
    ret = fwnode_property_read_u32_array(child,
    "adi,range-microamp",
    tmp, 2);
    if (ret)
    return ret;
    min = tmp[0] / 1000;
    max = tmp[1] / 1000;
    ret = ad5770r_store_output_range(st, min, max, num);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5770r_init(st: *mut ad5770r_state) -> c_int {
    static int ad5770r_init(struct ad5770r_state *st)
    {
    int ret, i;
    st.gpio_reset = devm_gpiod_get_optional(&st.spi.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(st.gpio_reset))
    return PTR_ERR(st.gpio_reset);
// Perform a reset
    ret = ad5770r_reset(st);
    if (ret)
    return ret;
// Set output range
    ret = ad5770r_channel_config(st);
    if (ret)
    return ret;
    for (i = 0; i < AD5770R_MAX_CHANNELS; i++) {
    ret = ad5770r_set_output_mode(st,  &st.output_mode[i], i);
    if (ret)
    return ret;
    }
    st.external_res = fwnode_property_read_bool(st.spi.dev.fwnode,
    "adi,external-resistor");
    ret = ad5770r_set_reference(st);
    if (ret)
    return ret;
// Set outputs off
    ret = regmap_write(st.regmap, AD5770R_CHANNEL_CONFIG, 0x00);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, AD5770R_CH_ENABLE, 0x00);
    if (ret)
    return ret;
    for (i = 0; i < AD5770R_MAX_CHANNELS; i++)
    st.ch_pwr_down[i] = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5770r_probe(spi: *mut spi_device) -> c_int {
    static int ad5770r_probe(struct spi_device *spi)
    {
    struct ad5770r_state *st;
    struct iio_dev *indio_dev;
    struct regmap *regmap;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    spi_set_drvdata(spi, indio_dev);
    st.spi = spi;
    regmap = devm_regmap_init_spi(spi, &ad5770r_spi_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Error initializing spi regmap: %ld\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    st.regmap = regmap;
    ret = devm_regulator_get_enable_read_voltage(&spi.dev, "vref");
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(&spi.dev, ret, "Failed to get vref voltage\n");
    st.internal_ref = ret == -ENODEV;
    st.vref = st.internal_ref ? AD5770R_LOW_VREF_mV : ret / 1000;
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.info = &ad5770r_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ad5770r_channels;
    indio_dev.num_channels = ARRAY_SIZE(ad5770r_channels);
    ret = ad5770r_init(st);
    if (ret < 0) {
    dev_err(&spi.dev, "AD5770R init failed\n");
    return ret;
    }
    return devm_iio_device_register(&st.spi.dev, indio_dev);
    }
    static const struct of_device_id ad5770r_of_id[] = {
    { .compatible = "adi,ad5770r", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5770r_of_id);
    static const struct spi_device_id ad5770r_id[] = {
    { .name = "ad5770r" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad5770r_id);
    static struct spi_driver ad5770r_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = ad5770r_of_id,
    },
    .probe = ad5770r_probe,
    .id_table = ad5770r_id,
    };
    module_spi_driver(ad5770r_driver);
    MODULE_AUTHOR("Mircea Caprioru <mircea.caprioru@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD5770R IDAC");
    MODULE_LICENSE("GPL v2");
