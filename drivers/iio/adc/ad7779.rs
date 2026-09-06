//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad7779.c
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
// AD7770, AD7771, AD7779 ADC
//
// Copyright 2023-2024 Analog Devices Inc.
//

pub const AD7779_REG_CH_DISABLE: c_uint = 0x08;

pub const AD7779_REG_GENERAL_USER_CONFIG_1: c_uint = 0x11;
pub const AD7779_REG_GENERAL_USER_CONFIG_2: c_uint = 0x12;
pub const AD7779_REG_GENERAL_USER_CONFIG_3: c_uint = 0x13;
pub const AD7779_REG_DOUT_FORMAT: c_uint = 0x14;
pub const AD7779_REG_ADC_MUX_CONFIG: c_uint = 0x15;
pub const AD7779_REG_GPIO_CONFIG: c_uint = 0x17;
pub const AD7779_REG_BUFFER_CONFIG_1: c_uint = 0x19;
pub const AD7779_REG_GLOBAL_MUX_CONFIG: c_uint = 0x16;
pub const AD7779_REG_BUFFER_CONFIG_2: c_uint = 0x1A;
pub const AD7779_REG_GPIO_DATA: c_uint = 0x18;

pub const AD7779_REG_CH0_1_SAT_ERR: c_uint = 0x54;

pub const AD7779_REG_CH2_3_SAT_ERR: c_uint = 0x55;
pub const AD7779_REG_CH4_5_SAT_ERR: c_uint = 0x56;
pub const AD7779_REG_CH6_7_SAT_ERR: c_uint = 0x57;
pub const AD7779_REG_CHX_ERR_REG_EN: c_uint = 0x58;
pub const AD7779_REG_GEN_ERR_REG_1: c_uint = 0x59;
pub const AD7779_REG_GEN_ERR_REG_1_EN: c_uint = 0x5A;
pub const AD7779_REG_GEN_ERR_REG_2: c_uint = 0x5B;
pub const AD7779_REG_GEN_ERR_REG_2_EN: c_uint = 0x5C;
pub const AD7779_REG_STATUS_REG_1: c_uint = 0x5D;
pub const AD7779_REG_STATUS_REG_2: c_uint = 0x5E;
pub const AD7779_REG_STATUS_REG_3: c_uint = 0x5F;
pub const AD7779_REG_SRC_N_MSB: c_uint = 0x60;
pub const AD7779_REG_SRC_N_LSB: c_uint = 0x61;
pub const AD7779_REG_SRC_IF_MSB: c_uint = 0x62;
pub const AD7779_REG_SRC_IF_LSB: c_uint = 0x63;
pub const AD7779_REG_SRC_UPDATE: c_uint = 0x64;

// AD7779_REG_DOUT_FORMAT

pub const AD7779_NUM_CHANNELS: c_int = 8;
pub const AD7779_RESET_BUF_SIZE: c_int = 8;
pub const AD7779_CHAN_DATA_SIZE: c_int = 4;
pub const AD7779_LOWPOWER_DIV: c_int = 512;
pub const AD7779_HIGHPOWER_DIV: c_int = 2048;

pub const GAIN_REL: c_uint = 0x555555;

pub const AD7779_CRC8_POLY: c_uint = 0x07;
    DECLARE_CRC8_TABLE(ad7779_crc8_table);
    enum ad7779_filter {
    AD7779_SINC3,
    AD7779_SINC5,
    };
    enum ad7779_variant {
    ad7770,
    ad7771,
    ad7779,
    };
    enum ad7779_power_mode {
    AD7779_LOW_POWER,
    AD7779_HIGH_POWER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7779_chip_info {
    pub name: *const c_char,
    pub channels: *const iio_chan_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7779_state {
    pub spi: *mut spi_device,
    pub chip_info: *const ad7779_chip_info,
    pub mclk: *mut clk,
    pub trig: *mut iio_trigger,
    pub sampling_freq: c_uint,
    pub filter_enabled: enum ad7779_filter,
    pub back: *mut iio_backend,
//
// DMA (thus cache coherency maintenance) requires the
// transfer buffers to live in their own cache lines.
//
    struct {
    pub chans: [u32; 8],
    pub timestamp: aligned_s64,
    pub __aligned(IIO_DMA_MINALIGN): } data,
    pub spidata_tx: [u32; 8],
    pub reg_rx_buf: [u8; 3],
    pub reg_tx_buf: [u8; 3],
    pub reset_buf: [u8; 8],
}

    static const char * const ad7779_filter_type[] = {
    [AD7779_SINC3] = "sinc3",
    [AD7779_SINC5] = "sinc5",
    };
    static const char * const ad7779_power_supplies[] = {
    "avdd1", "avdd2", "avdd4",
    };
#[no_mangle]
unsafe extern "C" fn ad7779_spi_read(st: *mut ad7779_state, reg: u8, rbuf: *mut u8) -> c_int {
    static int ad7779_spi_read(struct ad7779_state *st, u8 reg, u8 *rbuf)
    {
    int ret;
    u8 crc_buf[2];
    u8 exp_crc;
    struct spi_transfer t = {
    .tx_buf = st.reg_tx_buf,
    .rx_buf = st.reg_rx_buf,
    };
    st.reg_tx_buf[0] = AD7779_SPI_READ_CMD | FIELD_GET(AD7779_REG_MSK, reg);
    st.reg_tx_buf[1] = 0;
    if (reg == AD7779_REG_GEN_ERR_REG_1_EN) {
    t.len = 2;
    } else {
    t.len = 3;
    st.reg_tx_buf[2] = crc8(ad7779_crc8_table, st.reg_tx_buf,
    t.len - 1, 0);
    }
    ret = spi_sync_transfer(st.spi, &t, 1);
    if (ret)
    return ret;
    crc_buf[0] = AD7779_SPI_READ_CMD | FIELD_GET(AD7779_REG_MSK, reg);
    crc_buf[1] = st.reg_rx_buf[1];
    exp_crc = crc8(ad7779_crc8_table, crc_buf, ARRAY_SIZE(crc_buf), 0);
    if (reg != AD7779_REG_GEN_ERR_REG_1_EN && exp_crc != st.reg_rx_buf[2]) {
    dev_err(&st.spi.dev, "Bad CRC %x, expected %x",
    st.reg_rx_buf[2], exp_crc);
    return -EINVAL;
    }
// rbuf = st->reg_rx_buf[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_spi_write(st: *mut ad7779_state, reg: u8, val: u8) -> c_int {
    static int ad7779_spi_write(struct ad7779_state *st, u8 reg, u8 val)
    {
    let mut length: u8 = 3;
    st.reg_tx_buf[0] = FIELD_GET(AD7779_REG_MSK, reg);
    st.reg_tx_buf[1] = val;
    if (reg == AD7779_REG_GEN_ERR_REG_1_EN)
    length = 2;
    else
    st.reg_tx_buf[2] = crc8(ad7779_crc8_table, st.reg_tx_buf,
    length - 1, 0);
    return spi_write(st.spi, st.reg_tx_buf, length);
    }
    static int ad7779_spi_write_mask(struct ad7779_state *st, u8 reg, u8 mask,
    u8 val)
    {
    int ret;
    u8 regval, data;
    ret = ad7779_spi_read(st, reg, &data);
    if (ret)
    return ret;
    regval = (data & ~mask) | (val & mask);
    if (regval == data)
    return 0;
    return ad7779_spi_write(st, reg, regval);
    }
    static int ad7779_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int writeval,
    unsigned int *readval)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    u8 rval;
    int ret;
    if (readval) {
    ret = ad7779_spi_read(st, reg, &rval);
// readval = rval;
    return ret;
    }
    return ad7779_spi_write(st, reg, writeval);
    }
    static int ad7779_set_sampling_frequency(struct ad7779_state *st,
    unsigned int sampling_freq)
    {
    int ret;
    unsigned int dec;
    unsigned int frac;
    unsigned int div;
    unsigned int decimal;
    unsigned int freq_khz;
    if (st.filter_enabled == AD7779_SINC3 &&
    sampling_freq > AD7779_SINC3_MAXFREQ)
    return -EINVAL;
    if (st.filter_enabled == AD7779_SINC5 &&
    sampling_freq > AD7779_SINC5_MAXFREQ)
    return -EINVAL;
    if (sampling_freq > AD7779_SPIMODE_MAX_SAMP_FREQ)
    return -EINVAL;
    div = AD7779_HIGHPOWER_DIV;
    freq_khz = sampling_freq / HZ_PER_KHZ;
    dec = div / freq_khz;
    frac = div % freq_khz;
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_MSB,
    FIELD_GET(AD7779_FREQ_MSB_MSK, dec));
    if (ret)
    return ret;
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_LSB,
    FIELD_GET(AD7779_FREQ_LSB_MSK, dec));
    if (ret)
    return ret;
    if (frac) {
//
// In order to obtain the first three decimals of the decimation
// the initial number is multiplied with 10^3 prior to the
// division, then the original division result is subtracted and
// the number is divided by 10^3.
//
    decimal = ((mult_frac(div, KILO, freq_khz) - dec * KILO) << 16)
    / KILO;
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_MSB,
    FIELD_GET(AD7779_FREQ_MSB_MSK, decimal));
    if (ret)
    return ret;
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_LSB,
    FIELD_GET(AD7779_FREQ_LSB_MSK, decimal));
    if (ret)
    return ret;
    } else {
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_MSB,
    FIELD_GET(AD7779_FREQ_MSB_MSK, 0x0));
    if (ret)
    return ret;
    ret = ad7779_spi_write(st, AD7779_REG_SRC_N_LSB,
    FIELD_GET(AD7779_FREQ_LSB_MSK, 0x0));
    if (ret)
    return ret;
    }
    ret = ad7779_spi_write(st, AD7779_REG_SRC_UPDATE, BIT(0));
    if (ret)
    return ret;
// SRC update settling time
    fsleep(15);
    ret = ad7779_spi_write(st, AD7779_REG_SRC_UPDATE, 0x0);
    if (ret)
    return ret;
// SRC update settling time
    fsleep(15);
    st.sampling_freq = sampling_freq;
    return 0;
    }
    static int ad7779_get_filter(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    u8 temp;
    int ret;
    ret = ad7779_spi_read(st, AD7779_REG_GENERAL_USER_CONFIG_2, &temp);
    if (ret)
    return ret;
    return FIELD_GET(AD7779_FILTER_MSK, temp);
    }
    static int ad7779_set_filter(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    unsigned int mode)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    int ret;
    ret = ad7779_spi_write_mask(st,
    AD7779_REG_GENERAL_USER_CONFIG_2,
    AD7779_FILTER_MSK,
    FIELD_PREP(AD7779_FILTER_MSK, mode));
    if (ret)
    return ret;
    ret = ad7779_set_sampling_frequency(st, st.sampling_freq);
    if (ret)
    return ret;
    st.filter_enabled = mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_get_calibscale(st: *mut ad7779_state, channel: c_int) -> c_int {
    static int ad7779_get_calibscale(struct ad7779_state *st, int channel)
    {
    int ret;
    u8 calibscale[3];
    ret = ad7779_spi_read(st, AD7779_REG_CH_GAIN_LOWER_BYTE(channel),
    &calibscale[0]);
    if (ret)
    return ret;
    ret = ad7779_spi_read(st, AD7779_REG_CH_GAIN_MID_BYTE(channel),
    &calibscale[1]);
    if (ret)
    return ret;
    ret = ad7779_spi_read(st, AD7779_REG_CH_GAIN_UPPER_BYTE(channel),
    &calibscale[2]);
    if (ret)
    return ret;
    return get_unaligned_be24(calibscale);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_set_calibscale(st: *mut ad7779_state, channel: c_int, val: c_int) -> c_int {
    static int ad7779_set_calibscale(struct ad7779_state *st, int channel, int val)
    {
    int ret;
    unsigned int gain;
    u8 gain_bytes[3];
//
// The gain value is relative to 0x555555, which represents a gain of 1
//
    gain = DIV_ROUND_CLOSEST_ULL((u64)val * 5592405LL, MEGA);
    put_unaligned_be24(gain, gain_bytes);
    ret = ad7779_spi_write(st, AD7779_REG_CH_GAIN_UPPER_BYTE(channel),
    gain_bytes[0]);
    if (ret)
    return ret;
    ret = ad7779_spi_write(st, AD7779_REG_CH_GAIN_MID_BYTE(channel),
    gain_bytes[1]);
    if (ret)
    return ret;
    return ad7779_spi_write(st, AD7779_REG_CH_GAIN_LOWER_BYTE(channel),
    gain_bytes[2]);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_get_calibbias(st: *mut ad7779_state, channel: c_int) -> c_int {
    static int ad7779_get_calibbias(struct ad7779_state *st, int channel)
    {
    int ret;
    u8 calibbias[3];
    ret = ad7779_spi_read(st, AD7779_REG_CH_OFFSET_LOWER_BYTE(channel),
    &calibbias[0]);
    if (ret)
    return ret;
    ret = ad7779_spi_read(st, AD7779_REG_CH_OFFSET_MID_BYTE(channel),
    &calibbias[1]);
    if (ret)
    return ret;
    ret = ad7779_spi_read(st, AD7779_REG_CH_OFFSET_UPPER_BYTE(channel),
    &calibbias[2]);
    if (ret)
    return ret;
    return get_unaligned_be24(calibbias);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_set_calibbias(st: *mut ad7779_state, channel: c_int, val: c_int) -> c_int {
    static int ad7779_set_calibbias(struct ad7779_state *st, int channel, int val)
    {
    int ret;
    u8 calibbias[3];
    put_unaligned_be24(val, calibbias);
    ret = ad7779_spi_write(st, AD7779_REG_CH_OFFSET_UPPER_BYTE(channel),
    calibbias[0]);
    if (ret)
    return ret;
    ret = ad7779_spi_write(st, AD7779_REG_CH_OFFSET_MID_BYTE(channel),
    calibbias[1]);
    if (ret)
    return ret;
    return ad7779_spi_write(st, AD7779_REG_CH_OFFSET_LOWER_BYTE(channel),
    calibbias[2]);
    }
    static int __ad7779_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBSCALE:
    ret = ad7779_get_calibscale(st, chan.channel);
    if (ret < 0)
    return ret;
// val = ret;
// val2 = GAIN_REL;
    return IIO_VAL_FRACTIONAL;
    case IIO_CHAN_INFO_CALIBBIAS:
    ret = ad7779_get_calibbias(st, chan.channel);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = st->sampling_freq;
    if (*val < 0)
    return -EINVAL;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int ad7779_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __ad7779_read_raw(indio_dev, chan, val, val2, mask);
    iio_device_release_direct(indio_dev);
    return ret;
    }
    static int __ad7779_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2,
    long mask)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_CALIBSCALE:
    return ad7779_set_calibscale(st, chan.channel, val2);
    case IIO_CHAN_INFO_CALIBBIAS:
    return ad7779_set_calibbias(st, chan.channel, val);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return ad7779_set_sampling_frequency(st, val);
    default:
    return -EINVAL;
    }
    }
    static int ad7779_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val, int val2,
    long mask)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __ad7779_write_raw(indio_dev, chan, val, val2, mask);
    iio_device_release_direct(indio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7779_buffer_preenable(struct iio_dev *indio_dev)
    {
    int ret;
    struct ad7779_state *st = iio_priv(indio_dev);
    ret = ad7779_spi_write_mask(st,
    AD7779_REG_GENERAL_USER_CONFIG_3,
    AD7779_MOD_SPI_EN_MSK,
    FIELD_PREP(AD7779_MOD_SPI_EN_MSK, 1));
    if (ret)
    return ret;
//
// DRDY output cannot be disabled at device level therefore we mask
// the irq at host end.
//
    enable_irq(st.spi.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7779_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    disable_irq(st.spi.irq);
    return ad7779_spi_write(st, AD7779_REG_GENERAL_USER_CONFIG_3,
    AD7779_DISABLE_SD);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad7779_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ad7779_state *st = iio_priv(indio_dev);
    int ret;
    struct spi_transfer t = {
    .rx_buf = st.data.chans,
    .tx_buf = st.spidata_tx,
    .len = AD7779_NUM_CHANNELS * AD7779_CHAN_DATA_SIZE,
    };
    st.spidata_tx[0] = AD7779_SPI_READ_CMD;
    ret = spi_sync_transfer(st.spi, &t, 1);
    if (ret) {
    dev_err(&st.spi.dev, "SPI transfer error in IRQ handler");
    goto exit_handler;
    }
    iio_push_to_buffers_with_ts(indio_dev, &st.data, sizeof(st.data),
    pf.timestamp);
    exit_handler:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_reset(indio_dev: *mut iio_dev, reset_gpio: *mut gpio_desc) -> c_int {
    static int ad7779_reset(struct iio_dev *indio_dev, struct gpio_desc *reset_gpio)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    int ret;
    struct spi_transfer t = {
    .tx_buf = st.reset_buf,
    .len = 8,
    };
    if (reset_gpio) {
    gpiod_set_value(reset_gpio, 1);
// Delay for reset to occur is 225 microseconds
    fsleep(230);
    ret = 0;
    } else {
    memset(st.reset_buf, 0xff, sizeof(st.reset_buf));
    ret = spi_sync_transfer(st.spi, &t, 1);
    if (ret)
    return ret;
    }
// Delay for reset to occur is 225 microseconds
    fsleep(230);
    return ret;
    }
    static int ad7779_update_scan_mode(struct iio_dev *indio_dev,
    const unsigned long *scan_mask)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    unsigned int c;
    int ret;
    for (c = 0; c < AD7779_NUM_CHANNELS; c++) {
    if (test_bit(c, scan_mask))
    ret = iio_backend_chan_enable(st.back, c);
    else
    ret = iio_backend_chan_disable(st.back, c);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct iio_info ad7779_info = {
    .read_raw = ad7779_read_raw,
    .write_raw = ad7779_write_raw,
    .debugfs_reg_access = &ad7779_reg_access,
    };
    static const struct iio_info ad7779_info_data = {
    .read_raw = ad7779_read_raw,
    .write_raw = ad7779_write_raw,
    .debugfs_reg_access = &ad7779_reg_access,
    .update_scan_mode = &ad7779_update_scan_mode,
    };
    static const struct iio_enum ad7779_filter_enum = {
    .items = ad7779_filter_type,
    .num_items = ARRAY_SIZE(ad7779_filter_type),
    .get = ad7779_get_filter,
    .set = ad7779_set_filter,
    };
    static const struct iio_chan_spec_ext_info ad7779_ext_filter[] = {
    IIO_ENUM("filter_type", IIO_SHARED_BY_ALL, &ad7779_filter_enum),
    IIO_ENUM_AVAILABLE("filter_type", IIO_SHARED_BY_ALL,
    &ad7779_filter_enum),
    { }
    };

    {								\
    .type = IIO_VOLTAGE,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_CALIBSCALE)  |	\
    BIT(IIO_CHAN_INFO_CALIBBIAS),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),\
    .address = (index),					\
    .indexed = 1,						\
    .channel = (index),					\
    .scan_index = (index),					\
    .ext_info = (_ext_info),				\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = 24,					\
    .storagebits = 32,				\
    .endianness = IIO_BE,				\
    },							\
    }

    AD777x_CHAN_S(index, core::ptr::null_mut())

    AD777x_CHAN_S(index, ad7779_ext_filter)
    static const struct iio_chan_spec ad7779_channels[] = {
    AD777x_CHAN_NO_FILTER_S(0),
    AD777x_CHAN_NO_FILTER_S(1),
    AD777x_CHAN_NO_FILTER_S(2),
    AD777x_CHAN_NO_FILTER_S(3),
    AD777x_CHAN_NO_FILTER_S(4),
    AD777x_CHAN_NO_FILTER_S(5),
    AD777x_CHAN_NO_FILTER_S(6),
    AD777x_CHAN_NO_FILTER_S(7),
    IIO_CHAN_SOFT_TIMESTAMP(8),
    };
    static const struct iio_chan_spec ad7779_channels_filter[] = {
    AD777x_CHAN_FILTER_S(0),
    AD777x_CHAN_FILTER_S(1),
    AD777x_CHAN_FILTER_S(2),
    AD777x_CHAN_FILTER_S(3),
    AD777x_CHAN_FILTER_S(4),
    AD777x_CHAN_FILTER_S(5),
    AD777x_CHAN_FILTER_S(6),
    AD777x_CHAN_FILTER_S(7),
    IIO_CHAN_SOFT_TIMESTAMP(8),
    };
    static const struct iio_buffer_setup_ops ad7779_buffer_setup_ops = {
    .preenable = ad7779_buffer_preenable,
    .postdisable = ad7779_buffer_postdisable,
    };
    static const struct iio_trigger_ops ad7779_trigger_ops = {
    .validate_device = iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn ad7779_conf(st: *mut ad7779_state, start_gpio: *mut gpio_desc) -> c_int {
    static int ad7779_conf(struct ad7779_state *st, struct gpio_desc *start_gpio)
    {
    int ret;
    ret = ad7779_spi_write_mask(st, AD7779_REG_GEN_ERR_REG_1_EN,
    AD7779_SPI_CRC_EN_MSK,
    FIELD_PREP(AD7779_SPI_CRC_EN_MSK, 1));
    if (ret)
    return ret;
    ret = ad7779_spi_write_mask(st, AD7779_REG_GENERAL_USER_CONFIG_1,
    AD7779_USRMOD_INIT_MSK,
    FIELD_PREP(AD7779_USRMOD_INIT_MSK, 5));
    if (ret)
    return ret;
    ret = ad7779_spi_write_mask(st, AD7779_REG_DOUT_FORMAT,
    AD7779_DCLK_CLK_DIV_MSK,
    FIELD_PREP(AD7779_DCLK_CLK_DIV_MSK, 1));
    if (ret)
    return ret;
    ret = ad7779_spi_write_mask(st, AD7779_REG_ADC_MUX_CONFIG,
    AD7779_REFMUX_CTRL_MSK,
    FIELD_PREP(AD7779_REFMUX_CTRL_MSK, 1));
    if (ret)
    return ret;
    ret = ad7779_set_sampling_frequency(st, AD7779_DEFAULT_SAMPLING_FREQ);
    if (ret)
    return ret;
    gpiod_set_value(start_gpio, 0);
// Start setup time
    fsleep(15);
    gpiod_set_value(start_gpio, 1);
// Start setup time
    fsleep(15);
    gpiod_set_value(start_gpio, 0);
// Start setup time
    fsleep(15);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_set_data_lines(indio_dev: *mut iio_dev, num_lanes: u32) -> c_int {
    static int ad7779_set_data_lines(struct iio_dev *indio_dev, u32 num_lanes)
    {
    struct ad7779_state *st = iio_priv(indio_dev);
    int ret;
    if (num_lanes != 1 && num_lanes != 2 && num_lanes != 4)
    return -EINVAL;
    ret = ad7779_set_sampling_frequency(st, num_lanes * AD7779_DEFAULT_SAMPLING_1LINE);
    if (ret)
    return ret;
    ret = iio_backend_num_lanes_set(st.back, num_lanes);
    if (ret)
    return ret;
    return ad7779_spi_write_mask(st, AD7779_REG_DOUT_FORMAT,
    AD7779_DOUT_FORMAT_MSK,
    FIELD_PREP(AD7779_DOUT_FORMAT_MSK, 2 - ilog2(num_lanes)));
    }
#[no_mangle]
unsafe extern "C" fn ad7779_setup_channels(indio_dev: *mut iio_dev, st: *const ad7779_state) -> c_int {
    static int ad7779_setup_channels(struct iio_dev *indio_dev, const struct ad7779_state *st)
    {
    struct iio_chan_spec *channels;
    struct device *dev = &st.spi.dev;
    channels = devm_kmemdup_array(dev, st.chip_info.channels,
    ARRAY_SIZE(ad7779_channels),
    sizeof(*channels), GFP_KERNEL);
    if (!channels)
    return -ENOMEM;
    for (unsigned int i = 0; i < ARRAY_SIZE(ad7779_channels); i++)
    channels[i].scan_type.endianness = IIO_CPU;
    indio_dev.channels = channels;
    indio_dev.num_channels = ARRAY_SIZE(ad7779_channels);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7779_setup_without_backend(st: *mut ad7779_state, indio_dev: *mut iio_dev) -> c_int {
    static int ad7779_setup_without_backend(struct ad7779_state *st, struct iio_dev *indio_dev)
    {
    int ret;
    struct device *dev = &st.spi.dev;
    indio_dev.info = &ad7779_info;
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = ARRAY_SIZE(ad7779_channels);
    st.trig = devm_iio_trigger_alloc(dev, "%s-dev%d", indio_dev.name,
    iio_device_id(indio_dev));
    if (!st.trig)
    return -ENOMEM;
    st.trig.ops = &ad7779_trigger_ops;
    iio_trigger_set_drvdata(st.trig, st);
    ret = devm_request_irq(dev, st.spi.irq, iio_trigger_generic_data_rdy_poll,
    IRQF_NO_THREAD | IRQF_NO_AUTOEN, indio_dev.name,
    st.trig);
    if (ret)
    return ret;
    ret = devm_iio_trigger_register(dev, st.trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(st.trig);
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    &iio_pollfunc_store_time,
    &ad7779_trigger_handler,
    &ad7779_buffer_setup_ops);
    if (ret)
    return ret;
    return ad7779_spi_write_mask(st, AD7779_REG_DOUT_FORMAT,
    AD7779_DCLK_CLK_DIV_MSK,
    FIELD_PREP(AD7779_DCLK_CLK_DIV_MSK, 7));
    }
#[no_mangle]
unsafe extern "C" fn ad7779_setup_backend(st: *mut ad7779_state, indio_dev: *mut iio_dev) -> c_int {
    static int ad7779_setup_backend(struct ad7779_state *st, struct iio_dev *indio_dev)
    {
    struct device *dev = &st.spi.dev;
    int ret;
    u32 num_lanes;
    indio_dev.info = &ad7779_info_data;
    ret = ad7779_setup_channels(indio_dev, st);
    if (ret)
    return ret;
    st.back = devm_iio_backend_get(dev, core::ptr::null_mut());
    if (IS_ERR(st.back))
    return dev_err_probe(dev, PTR_ERR(st.back),
    "failed to get iio backend");
    ret = devm_iio_backend_request_buffer(dev, st.back, indio_dev);
    if (ret)
    return ret;
    ret = devm_iio_backend_enable(dev, st.back);
    if (ret)
    return ret;
    num_lanes = 4;
    ret = device_property_read_u32(dev, "adi,num-lanes", &num_lanes);
    if (ret && ret != -EINVAL)
    return ret;
    return ad7779_set_data_lines(indio_dev, num_lanes);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_probe(spi: *mut spi_device) -> c_int {
    static int ad7779_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ad7779_state *st;
    struct gpio_desc *reset_gpio, *start_gpio;
    struct device *dev = &spi.dev;
    let mut ret: c_int = -EINVAL;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    ret = devm_regulator_bulk_get_enable(dev,
    ARRAY_SIZE(ad7779_power_supplies),
    ad7779_power_supplies);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get and enable supplies\n");
    st.mclk = devm_clk_get_enabled(dev, "mclk");
    if (IS_ERR(st.mclk))
    return PTR_ERR(st.mclk);
    reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset_gpio))
    return PTR_ERR(reset_gpio);
    start_gpio = devm_gpiod_get(dev, "start", GPIOD_OUT_HIGH);
    if (IS_ERR(start_gpio))
    return PTR_ERR(start_gpio);
    crc8_populate_msb(ad7779_crc8_table, AD7779_CRC8_POLY);
    st.spi = spi;
    st.chip_info = spi_get_device_match_data(spi);
    if (!st.chip_info)
    return -ENODEV;
    ret = ad7779_reset(indio_dev, reset_gpio);
    if (ret)
    return ret;
    ret = ad7779_conf(st, start_gpio);
    if (ret)
    return ret;
    indio_dev.name = st.chip_info.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    if (device_property_present(dev, "io-backends"))
    ret = ad7779_setup_backend(st, indio_dev);
    else
    ret = ad7779_setup_without_backend(st, indio_dev);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn ad7779_suspend(dev: *mut device) -> c_int {
    static int ad7779_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct ad7779_state *st = iio_priv(indio_dev);
    return ad7779_spi_write_mask(st, AD7779_REG_GENERAL_USER_CONFIG_1,
    AD7779_MOD_POWERMODE_MSK,
    FIELD_PREP(AD7779_MOD_POWERMODE_MSK,
    AD7779_LOW_POWER));
    }
#[no_mangle]
unsafe extern "C" fn ad7779_resume(dev: *mut device) -> c_int {
    static int ad7779_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct ad7779_state *st = iio_priv(indio_dev);
    return ad7779_spi_write_mask(st, AD7779_REG_GENERAL_USER_CONFIG_1,
    AD7779_MOD_POWERMODE_MSK,
    FIELD_PREP(AD7779_MOD_POWERMODE_MSK,
    AD7779_HIGH_POWER));
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ad7779_pm_ops, ad7779_suspend, ad7779_resume);
    static const struct ad7779_chip_info ad7770_chip_info = {
    .name = "ad7770",
    .channels = ad7779_channels,
    };
    static const struct ad7779_chip_info ad7771_chip_info = {
    .name = "ad7771",
    .channels = ad7779_channels_filter,
    };
    static const struct ad7779_chip_info ad7779_chip_info = {
    .name = "ad7779",
    .channels = ad7779_channels,
    };
    static const struct spi_device_id ad7779_id[] = {
    {
    .name = "ad7770",
    .driver_data = (kernel_ulong_t)&ad7770_chip_info,
    },
    {
    .name = "ad7771",
    .driver_data = (kernel_ulong_t)&ad7771_chip_info,
    },
    {
    .name = "ad7779",
    .driver_data = (kernel_ulong_t)&ad7779_chip_info,
    },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7779_id);
    static const struct of_device_id ad7779_of_table[] = {
    {
    .compatible = "adi,ad7770",
    .data = &ad7770_chip_info,
    },
    {
    .compatible = "adi,ad7771",
    .data = &ad7771_chip_info,
    },
    {
    .compatible = "adi,ad7779",
    .data = &ad7779_chip_info,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad7779_of_table);
    static struct spi_driver ad7779_driver = {
    .driver = {
    .name = "ad7779",
    .pm = pm_sleep_ptr(&ad7779_pm_ops),
    .of_match_table = ad7779_of_table,
    },
    .probe = ad7779_probe,
    .id_table = ad7779_id,
    };
    module_spi_driver(ad7779_driver);
    MODULE_AUTHOR("Ramona Alexandra Nechita <ramona.nechita@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD7779 ADC");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BACKEND");
