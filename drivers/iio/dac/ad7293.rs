//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad7293.c
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
// AD7293 driver
//
// Copyright 2021 Analog Devices Inc.
//

// AD7293 Register Map Common

// AD7293 Register Map Page 0x0

// AD7293 Register Map Page 0x2

// AD7293 Register Map Page 0x3

// AD7293 Register Map Page 0xE

// AD7293 Miscellaneous Definitions

pub const AD7293_CHIP_ID: c_uint = 0x18;
    enum ad7293_ch_type {
    AD7293_ADC_VINX,
    AD7293_ADC_TSENSE,
    AD7293_ADC_ISENSE,
    AD7293_DAC,
    };
    enum ad7293_max_offset {
    AD7293_TSENSE_MIN_OFFSET_CH = 4,
    AD7293_ISENSE_MIN_OFFSET_CH = 7,
    AD7293_VOUT_MIN_OFFSET_CH = 11,
    AD7293_VOUT_MAX_OFFSET_CH = 18,
    };
    static const int dac_offset_table[] = {0, 1, 2};
    static const int isense_gain_table[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    static const int adc_range_table[] = {0, 1, 2, 3};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7293_state {
    pub spi: *mut spi_device,
// Protect against concurrent accesses to the device, page selection and data content
    pub lock: mutex,
    pub gpio_reset: *mut gpio_desc,
    pub vrefin_en: bool,
    pub page_select: u8,
    pub __aligned(IIO_DMA_MINALIGN): u8 data[3],
}

#[no_mangle]
unsafe extern "C" fn ad7293_page_select(st: *mut ad7293_state, reg: c_uint) -> c_int {
    static int ad7293_page_select(struct ad7293_state *st, unsigned int reg)
    {
    int ret;
    if (st.page_select != FIELD_GET(AD7293_PAGE_ADDR_MSK, reg)) {
    st.data[0] = FIELD_GET(AD7293_REG_ADDR_MSK, AD7293_REG_PAGE_SELECT);
    st.data[1] = FIELD_GET(AD7293_PAGE_ADDR_MSK, reg);
    ret = spi_write(st.spi, &st.data[0], 2);
    if (ret)
    return ret;
    st.page_select = FIELD_GET(AD7293_PAGE_ADDR_MSK, reg);
    }
    return 0;
    }
    static int __ad7293_spi_read(struct ad7293_state *st, unsigned int reg,
    u16 *val)
    {
    int ret;
    unsigned int length;
    let mut t: spi_transfer = {0};
    length = FIELD_GET(AD7293_TRANSF_LEN_MSK, reg);
    ret = ad7293_page_select(st, reg);
    if (ret)
    return ret;
    st.data[0] = AD7293_READ | FIELD_GET(AD7293_REG_ADDR_MSK, reg);
    st.data[1] = 0x0;
    st.data[2] = 0x0;
    t.tx_buf = &st.data[0];
    t.rx_buf = &st.data[0];
    t.len = length + 1;
    ret = spi_sync_transfer(st.spi, &t, 1);
    if (ret)
    return ret;
    if (length == 1)
// val = st->data[1];
    else
// val = get_unaligned_be16(&st->data[1]);
    return 0;
    }
    static int ad7293_spi_read(struct ad7293_state *st, unsigned int reg,
    u16 *val)
    {
    int ret;
    mutex_lock(&st.lock);
    ret = __ad7293_spi_read(st, reg, val);
    mutex_unlock(&st.lock);
    return ret;
    }
    static int __ad7293_spi_write(struct ad7293_state *st, unsigned int reg,
    u16 val)
    {
    int ret;
    unsigned int length;
    length = FIELD_GET(AD7293_TRANSF_LEN_MSK, reg);
    ret = ad7293_page_select(st, reg);
    if (ret)
    return ret;
    st.data[0] = FIELD_GET(AD7293_REG_ADDR_MSK, reg);
    if (length == 1)
    st.data[1] = val;
    else
    put_unaligned_be16(val, &st.data[1]);
    return spi_write(st.spi, &st.data[0], length + 1);
    }
    static int ad7293_spi_write(struct ad7293_state *st, unsigned int reg,
    u16 val)
    {
    int ret;
    mutex_lock(&st.lock);
    ret = __ad7293_spi_write(st, reg, val);
    mutex_unlock(&st.lock);
    return ret;
    }
    static int __ad7293_spi_update_bits(struct ad7293_state *st, unsigned int reg,
    u16 mask, u16 val)
    {
    int ret;
    u16 data, temp;
    ret = __ad7293_spi_read(st, reg, &data);
    if (ret)
    return ret;
    temp = (data & ~mask) | (val & mask);
    return __ad7293_spi_write(st, reg, temp);
    }
    static int ad7293_spi_update_bits(struct ad7293_state *st, unsigned int reg,
    u16 mask, u16 val)
    {
    int ret;
    mutex_lock(&st.lock);
    ret = __ad7293_spi_update_bits(st, reg, mask, val);
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad7293_adc_get_scale(struct ad7293_state *st, unsigned int ch,
    u16 *range)
    {
    int ret;
    u16 data;
    mutex_lock(&st.lock);
    ret = __ad7293_spi_read(st, AD7293_REG_VINX_RANGE1, &data);
    if (ret)
    goto exit;
// range = AD7293_REG_VINX_RANGE_GET_CH_MSK(data, ch);
    ret = __ad7293_spi_read(st, AD7293_REG_VINX_RANGE0, &data);
    if (ret)
    goto exit;
// range |= AD7293_REG_VINX_RANGE_GET_CH_MSK(data, ch) << 1;
    exit:
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad7293_adc_set_scale(struct ad7293_state *st, unsigned int ch,
    u16 range)
    {
    int ret;
    let mut ch_msk: c_uint = BIT(ch);
    mutex_lock(&st.lock);
    ret = __ad7293_spi_update_bits(st, AD7293_REG_VINX_RANGE1, ch_msk,
    AD7293_REG_VINX_RANGE_SET_CH_MSK(range, ch));
    if (ret)
    goto exit;
    ret = __ad7293_spi_update_bits(st, AD7293_REG_VINX_RANGE0, ch_msk,
    AD7293_REG_VINX_RANGE_SET_CH_MSK((range >> 1), ch));
    exit:
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad7293_get_offset(struct ad7293_state *st, unsigned int ch,
    u16 *offset)
    {
    if (ch < AD7293_TSENSE_MIN_OFFSET_CH)
    return ad7293_spi_read(st, AD7293_REG_VIN0_OFFSET + ch, offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_ISENSE_MIN_OFFSET_CH: ch <) -> else {
    else if (ch < AD7293_ISENSE_MIN_OFFSET_CH)
    return ad7293_spi_read(st, AD7293_REG_TSENSE_INT_OFFSET + (ch - 4), offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_VOUT_MIN_OFFSET_CH: ch <) -> else {
    else if (ch < AD7293_VOUT_MIN_OFFSET_CH)
    return ad7293_spi_read(st, AD7293_REG_ISENSE0_OFFSET + (ch - 7), offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_VOUT_MAX_OFFSET_CH: ch <=) -> else {
    else if (ch <= AD7293_VOUT_MAX_OFFSET_CH)
    return ad7293_spi_read(st, AD7293_REG_UNI_VOUT0_OFFSET + (ch - 11), offset);
    return -EINVAL;
    }
    static int ad7293_set_offset(struct ad7293_state *st, unsigned int ch,
    u16 offset)
    {
    if (ch < AD7293_TSENSE_MIN_OFFSET_CH)
    return ad7293_spi_write(st, AD7293_REG_VIN0_OFFSET + ch,
    offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_ISENSE_MIN_OFFSET_CH: ch <) -> else {
    else if (ch < AD7293_ISENSE_MIN_OFFSET_CH)
    return ad7293_spi_write(st,
    AD7293_REG_TSENSE_INT_OFFSET +
    (ch - AD7293_TSENSE_MIN_OFFSET_CH),
    offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_VOUT_MIN_OFFSET_CH: ch <) -> else {
    else if (ch < AD7293_VOUT_MIN_OFFSET_CH)
    return ad7293_spi_write(st,
    AD7293_REG_ISENSE0_OFFSET +
    (ch - AD7293_ISENSE_MIN_OFFSET_CH),
    offset);
#[no_mangle]
pub unsafe extern "C" fn if(AD7293_VOUT_MAX_OFFSET_CH: ch <=) -> else {
    else if (ch <= AD7293_VOUT_MAX_OFFSET_CH)
    return ad7293_spi_update_bits(st,
    AD7293_REG_UNI_VOUT0_OFFSET +
    (ch - AD7293_VOUT_MIN_OFFSET_CH),
    AD7293_REG_VOUT_OFFSET_MSK,
    FIELD_PREP(AD7293_REG_VOUT_OFFSET_MSK, offset));
    return -EINVAL;
    }
    static int ad7293_isense_set_scale(struct ad7293_state *st, unsigned int ch,
    u16 gain)
    {
    let mut ch_msk: c_uint = (0xf << (4 * ch));
    return ad7293_spi_update_bits(st, AD7293_REG_ISENSE_GAIN, ch_msk,
    gain << (4 * ch));
    }
    static int ad7293_isense_get_scale(struct ad7293_state *st, unsigned int ch,
    u16 *gain)
    {
    int ret;
    ret = ad7293_spi_read(st, AD7293_REG_ISENSE_GAIN, gain);
    if (ret)
    return ret;
// gain = (*gain >> (4 * ch)) & 0xf;
    return ret;
    }
    static int ad7293_dac_write_raw(struct ad7293_state *st, unsigned int ch,
    u16 raw)
    {
    int ret;
    mutex_lock(&st.lock);
    ret = __ad7293_spi_update_bits(st, AD7293_REG_DAC_EN, BIT(ch), BIT(ch));
    if (ret)
    goto exit;
    ret =  __ad7293_spi_write(st, AD7293_REG_UNI_VOUT0 + ch,
    FIELD_PREP(AD7293_REG_DATA_RAW_MSK, raw));
    exit:
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad7293_ch_read_raw(struct ad7293_state *st, enum ad7293_ch_type type,
    unsigned int ch, u16 *raw)
    {
    int ret;
    unsigned int reg_wr, reg_rd, data_wr;
    switch (type) {
    case AD7293_ADC_VINX:
    reg_wr = AD7293_REG_VINX_SEQ;
    reg_rd = AD7293_REG_VIN0 + ch;
    data_wr = BIT(ch);
    break;
    case AD7293_ADC_TSENSE:
    reg_wr = AD7293_REG_ISENSEX_TSENSEX_SEQ;
    reg_rd = AD7293_REG_TSENSE_INT + ch;
    data_wr = BIT(ch);
    break;
    case AD7293_ADC_ISENSE:
    reg_wr = AD7293_REG_ISENSEX_TSENSEX_SEQ;
    reg_rd = AD7293_REG_ISENSE_0 + ch;
    data_wr = BIT(ch) << 8;
    break;
    case AD7293_DAC:
    reg_rd = AD7293_REG_UNI_VOUT0 + ch;
    break;
    default:
    return -EINVAL;
    }
    mutex_lock(&st.lock);
    if (type != AD7293_DAC) {
    if (type == AD7293_ADC_TSENSE) {
    ret = __ad7293_spi_write(st, AD7293_REG_TSENSE_BG_EN,
    BIT(ch));
    if (ret)
    goto exit;
    usleep_range(9000, 9900);
    } else if (type == AD7293_ADC_ISENSE) {
    ret = __ad7293_spi_write(st, AD7293_REG_ISENSE_BG_EN,
    BIT(ch));
    if (ret)
    goto exit;
    usleep_range(2000, 7000);
    }
    ret = __ad7293_spi_write(st, reg_wr, data_wr);
    if (ret)
    goto exit;
    ret = __ad7293_spi_write(st, AD7293_REG_CONV_CMD, 0x82);
    if (ret)
    goto exit;
    }
    ret = __ad7293_spi_read(st, reg_rd, raw);
// raw = FIELD_GET(AD7293_REG_DATA_RAW_MSK, *raw);
    exit:
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad7293_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct ad7293_state *st = iio_priv(indio_dev);
    int ret;
    u16 data;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.type) {
    case IIO_VOLTAGE:
    if (chan.output)
    ret =  ad7293_ch_read_raw(st, AD7293_DAC,
    chan.channel, &data);
    else
    ret =  ad7293_ch_read_raw(st, AD7293_ADC_VINX,
    chan.channel, &data);
    break;
    case IIO_CURRENT:
    ret =  ad7293_ch_read_raw(st, AD7293_ADC_ISENSE,
    chan.channel, &data);
    break;
    case IIO_TEMP:
    ret =  ad7293_ch_read_raw(st, AD7293_ADC_TSENSE,
    chan.channel, &data);
    break;
    default:
    return -EINVAL;
    }
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_VOLTAGE:
    if (chan.output) {
    ret = ad7293_get_offset(st,
    chan.channel + AD7293_VOUT_MIN_OFFSET_CH,
    &data);
    data = FIELD_GET(AD7293_REG_VOUT_OFFSET_MSK, data);
    } else {
    ret = ad7293_get_offset(st, chan.channel, &data);
    }
    break;
    case IIO_CURRENT:
    ret = ad7293_get_offset(st,
    chan.channel + AD7293_ISENSE_MIN_OFFSET_CH,
    &data);
    break;
    case IIO_TEMP:
    ret = ad7293_get_offset(st,
    chan.channel + AD7293_TSENSE_MIN_OFFSET_CH,
    &data);
    break;
    default:
    return -EINVAL;
    }
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_VOLTAGE:
    ret = ad7293_adc_get_scale(st, chan.channel, &data);
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    case IIO_CURRENT:
    ret = ad7293_isense_get_scale(st, chan.channel, &data);
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    case IIO_TEMP:
// val = 1;
// val2 = 8;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static int ad7293_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long info)
    {
    struct ad7293_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.type) {
    case IIO_VOLTAGE:
    if (!chan.output)
    return -EINVAL;
    return ad7293_dac_write_raw(st, chan.channel, val);
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_VOLTAGE:
    if (chan.output)
    return ad7293_set_offset(st,
    chan.channel +
    AD7293_VOUT_MIN_OFFSET_CH,
    val);
    else
    return ad7293_set_offset(st, chan.channel, val);
    case IIO_CURRENT:
    return ad7293_set_offset(st,
    chan.channel +
    AD7293_ISENSE_MIN_OFFSET_CH,
    val);
    case IIO_TEMP:
    return ad7293_set_offset(st,
    chan.channel +
    AD7293_TSENSE_MIN_OFFSET_CH,
    val);
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_VOLTAGE:
    return ad7293_adc_set_scale(st, chan.channel, val);
    case IIO_CURRENT:
    return ad7293_isense_set_scale(st, chan.channel, val);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static int ad7293_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int write_val,
    unsigned int *read_val)
    {
    struct ad7293_state *st = iio_priv(indio_dev);
    int ret;
    if (read_val) {
    u16 temp;
    ret = ad7293_spi_read(st, reg, &temp);
// read_val = temp;
    } else {
    ret = ad7293_spi_write(st, reg, (u16)write_val);
    }
    return ret;
    }
    static int ad7293_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long info)
    {
    switch (info) {
    case IIO_CHAN_INFO_OFFSET:
// vals = dac_offset_table;
// type = IIO_VAL_INT;
// length = ARRAY_SIZE(dac_offset_table);
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_SCALE:
// type = IIO_VAL_INT;
    switch (chan.type) {
    case IIO_VOLTAGE:
// vals = adc_range_table;
// length = ARRAY_SIZE(adc_range_table);
    return IIO_AVAIL_LIST;
    case IIO_CURRENT:
// vals = isense_gain_table;
// length = ARRAY_SIZE(isense_gain_table);
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }

    .type = IIO_VOLTAGE,						\
    .output = 0,							\
    .indexed = 1,							\
    .channel = _channel,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_OFFSET),		\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE)	\
    }

    .type = IIO_VOLTAGE,						\
    .output = 1,							\
    .indexed = 1,							\
    .channel = _channel,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_OFFSET),		\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_OFFSET)	\
    }

    .type = IIO_CURRENT,						\
    .output = 0,							\
    .indexed = 1,							\
    .channel = _channel,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_OFFSET) |		\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE)	\
    }

    .type = IIO_TEMP,						\
    .output = 0,							\
    .indexed = 1,							\
    .channel = _channel,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_OFFSET),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE)		\
    }
    static const struct iio_chan_spec ad7293_channels[] = {
    AD7293_CHAN_ADC(0),
    AD7293_CHAN_ADC(1),
    AD7293_CHAN_ADC(2),
    AD7293_CHAN_ADC(3),
    AD7293_CHAN_ISENSE(0),
    AD7293_CHAN_ISENSE(1),
    AD7293_CHAN_ISENSE(2),
    AD7293_CHAN_ISENSE(3),
    AD7293_CHAN_TEMP(0),
    AD7293_CHAN_TEMP(1),
    AD7293_CHAN_TEMP(2),
    AD7293_CHAN_DAC(0),
    AD7293_CHAN_DAC(1),
    AD7293_CHAN_DAC(2),
    AD7293_CHAN_DAC(3),
    AD7293_CHAN_DAC(4),
    AD7293_CHAN_DAC(5),
    AD7293_CHAN_DAC(6),
    AD7293_CHAN_DAC(7)
    };
#[no_mangle]
unsafe extern "C" fn ad7293_soft_reset(st: *mut ad7293_state) -> c_int {
    static int ad7293_soft_reset(struct ad7293_state *st)
    {
    int ret;
    ret = __ad7293_spi_write(st, AD7293_REG_SOFT_RESET, 0x7293);
    if (ret)
    return ret;
    return __ad7293_spi_write(st, AD7293_REG_SOFT_RESET, 0x0000);
    }
#[no_mangle]
unsafe extern "C" fn ad7293_reset(st: *mut ad7293_state) -> c_int {
    static int ad7293_reset(struct ad7293_state *st)
    {
    if (st.gpio_reset) {
    gpiod_set_value(st.gpio_reset, 0);
    usleep_range(100, 1000);
    gpiod_set_value(st.gpio_reset, 1);
    usleep_range(100, 1000);
    return 0;
    }
// Perform a software reset
    return ad7293_soft_reset(st);
    }
#[no_mangle]
unsafe extern "C" fn ad7293_properties_parse(st: *mut ad7293_state) -> c_int {
    static int ad7293_properties_parse(struct ad7293_state *st)
    {
    struct device *dev = &st.spi.dev;
    int ret;
    ret = devm_regulator_get_enable(dev, "avdd");
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable AVDD\n");
    ret = devm_regulator_get_enable(dev, "vdrive");
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable VDRIVE\n");
    ret = devm_regulator_get_enable_optional(dev, "vrefin");
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(dev, ret, "failed to enable VREFIN\n");
    st.vrefin_en = ret != -ENODEV;
    st.gpio_reset = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(st.gpio_reset))
    return dev_err_probe(dev, PTR_ERR(st.gpio_reset),
    "failed to get the reset GPIO\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7293_init(st: *mut ad7293_state) -> c_int {
    static int ad7293_init(struct ad7293_state *st)
    {
    int ret;
    u16 chip_id;
    struct device *dev = &st.spi.dev;
    ret = ad7293_properties_parse(st);
    if (ret)
    return ret;
    ret = ad7293_reset(st);
    if (ret)
    return ret;
// Check Chip ID
    ret = __ad7293_spi_read(st, AD7293_REG_DEVICE_ID, &chip_id);
    if (ret)
    return ret;
    if (chip_id != AD7293_CHIP_ID)
    return dev_err_probe(dev, -EINVAL, "Invalid Chip ID.\n");
    if (!st.vrefin_en)
    return __ad7293_spi_update_bits(st, AD7293_REG_GENERAL,
    AD7293_GENERAL_ADC_REF_MSK,
    AD7293_GENERAL_ADC_REF_MSK);
    return 0;
    }
    static const struct iio_info ad7293_info = {
    .read_raw = ad7293_read_raw,
    .write_raw = ad7293_write_raw,
    .read_avail = &ad7293_read_avail,
    .debugfs_reg_access = &ad7293_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn ad7293_probe(spi: *mut spi_device) -> c_int {
    static int ad7293_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ad7293_state *st;
    struct device *dev = &spi.dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    indio_dev.info = &ad7293_info;
    indio_dev.name = "ad7293";
    indio_dev.channels = ad7293_channels;
    indio_dev.num_channels = ARRAY_SIZE(ad7293_channels);
    st.spi = spi;
    st.page_select = 0;
    mutex_init(&st.lock);
    ret = ad7293_init(st);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id ad7293_id[] = {
    { .name = "ad7293" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7293_id);
    static const struct of_device_id ad7293_of_match[] = {
    { .compatible = "adi,ad7293" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad7293_of_match);
    static struct spi_driver ad7293_driver = {
    .driver = {
    .name = "ad7293",
    .of_match_table = ad7293_of_match,
    },
    .probe = ad7293_probe,
    .id_table = ad7293_id,
    };
    module_spi_driver(ad7293_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com");
    MODULE_DESCRIPTION("Analog Devices AD7293");
    MODULE_LICENSE("GPL v2");
