//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/sx9310.c
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
// Copyright 2018 Google LLC.
//
// Driver for Semtech's SX9310/SX9311 capacitive proximity/button solution.
// Based on SX9500 driver and Semtech driver using the input framework
// <https://my.syncplicity.com/share/teouwsim8niiaud
// linux-driver-SX9310_NoSmartHSensing>.
// Reworked in April 2019 by Evan Green <evgreen@chromium.org>
// and in January 2020 by Daniel Campello <campello@chromium.org>.
//

// Register definitions.

pub const SX9310_REG_STAT0: c_uint = 0x01;
pub const SX9310_REG_STAT1: c_uint = 0x02;

pub const SX9310_REG_IRQ_MSK: c_uint = 0x03;

pub const SX9310_REG_IRQ_FUNC: c_uint = 0x04;
pub const SX9310_REG_PROX_CTRL0: c_uint = 0x10;

pub const SX9310_REG_PROX_CTRL0_SCANPERIOD_15MS: c_uint = 0x01;
pub const SX9310_REG_PROX_CTRL1: c_uint = 0x11;
pub const SX9310_REG_PROX_CTRL2: c_uint = 0x12;

pub const SX9310_REG_PROX_CTRL3: c_uint = 0x13;

pub const SX9310_REG_PROX_CTRL3_GAIN12_X4: c_uint = 0x02;
pub const SX9310_REG_PROX_CTRL4: c_uint = 0x14;

pub const SX9310_REG_PROX_CTRL4_RESOLUTION_FINEST: c_uint = 0x07;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_VERY_FINE: c_uint = 0x06;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_FINE: c_uint = 0x05;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_MEDIUM: c_uint = 0x04;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_MEDIUM_COARSE: c_uint = 0x03;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_COARSE: c_uint = 0x02;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_VERY_COARSE: c_uint = 0x01;
pub const SX9310_REG_PROX_CTRL4_RESOLUTION_COARSEST: c_uint = 0x00;
pub const SX9310_REG_PROX_CTRL5: c_uint = 0x15;

pub const SX9310_REG_PROX_CTRL5_RAWFILT_SHIFT: c_int = 0;
pub const SX9310_REG_PROX_CTRL5_RAWFILT_1P25: c_uint = 0x02;
pub const SX9310_REG_PROX_CTRL6: c_uint = 0x16;
pub const SX9310_REG_PROX_CTRL6_AVGTHRESH_DEFAULT: c_uint = 0x20;
pub const SX9310_REG_PROX_CTRL7: c_uint = 0x17;

pub const SX9310_REG_PROX_CTRL7_AVGPOSFILT_SHIFT: c_int = 0;
pub const SX9310_REG_PROX_CTRL7_AVGPOSFILT_512: c_uint = 0x05;
pub const SX9310_REG_PROX_CTRL8: c_uint = 0x18;

pub const SX9310_REG_PROX_CTRL9: c_uint = 0x19;

pub const SX9310_REG_PROX_CTRL8_9_BODYTHRESH_900: c_uint = 0x03;
pub const SX9310_REG_PROX_CTRL8_9_BODYTHRESH_1500: c_uint = 0x05;
pub const SX9310_REG_PROX_CTRL10: c_uint = 0x1a;

pub const SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_2: c_uint = 0x01;
pub const SX9310_REG_PROX_CTRL11: c_uint = 0x1b;
pub const SX9310_REG_PROX_CTRL12: c_uint = 0x1c;
pub const SX9310_REG_PROX_CTRL13: c_uint = 0x1d;
pub const SX9310_REG_PROX_CTRL14: c_uint = 0x1e;
pub const SX9310_REG_PROX_CTRL15: c_uint = 0x1f;
pub const SX9310_REG_PROX_CTRL16: c_uint = 0x20;
pub const SX9310_REG_PROX_CTRL17: c_uint = 0x21;
pub const SX9310_REG_PROX_CTRL18: c_uint = 0x22;
pub const SX9310_REG_PROX_CTRL19: c_uint = 0x23;
pub const SX9310_REG_SAR_CTRL0: c_uint = 0x2a;

pub const SX9310_REG_SAR_CTRL1: c_uint = 0x2b;
// Each increment of the slope register is 0.0078125.

pub const SX9310_REG_SAR_CTRL2: c_uint = 0x2c;
pub const SX9310_REG_SAR_CTRL2_SAROFFSET_DEFAULT: c_uint = 0x3c;
pub const SX9310_REG_SENSOR_SEL: c_uint = 0x30;
pub const SX9310_REG_USE_MSB: c_uint = 0x31;
pub const SX9310_REG_USE_LSB: c_uint = 0x32;
pub const SX9310_REG_AVG_MSB: c_uint = 0x33;
pub const SX9310_REG_AVG_LSB: c_uint = 0x34;
pub const SX9310_REG_DIFF_MSB: c_uint = 0x35;
pub const SX9310_REG_DIFF_LSB: c_uint = 0x36;
pub const SX9310_REG_OFFSET_MSB: c_uint = 0x37;
pub const SX9310_REG_OFFSET_LSB: c_uint = 0x38;
pub const SX9310_REG_SAR_MSB: c_uint = 0x39;
pub const SX9310_REG_SAR_LSB: c_uint = 0x3a;
pub const SX9310_REG_I2C_ADDR: c_uint = 0x40;
pub const SX9310_REG_PAUSE: c_uint = 0x41;
pub const SX9310_REG_WHOAMI: c_uint = 0x42;
pub const SX9310_WHOAMI_VALUE: c_uint = 0x01;
pub const SX9311_WHOAMI_VALUE: c_uint = 0x02;
pub const SX9310_REG_RESET: c_uint = 0x7f;
// 4 hardware channels, as defined in STAT0: COMB, CS2, CS1 and CS0.
pub const SX9310_NUM_CHANNELS: c_int = 4;
    static_assert(SX9310_NUM_CHANNELS <= SX_COMMON_MAX_NUM_CHANNELS);

    {								 \
    .type = IIO_PROXIMITY,					 \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		 \
    BIT(IIO_CHAN_INFO_HARDWAREGAIN),   \
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ), \
    .info_mask_separate_available =				 \
    BIT(IIO_CHAN_INFO_HARDWAREGAIN),		 \
    .info_mask_shared_by_all_available =			 \
    BIT(IIO_CHAN_INFO_SAMP_FREQ),			 \
    .indexed = 1,						 \
    .channel = idx,						 \
    .extend_name = name,					 \
    .address = SX9310_REG_DIFF_MSB,				 \
    .event_spec = sx_common_events,				 \
    .num_event_specs = ARRAY_SIZE(sx_common_events),	 \
    .scan_index = idx,					 \
    .scan_type = {						 \
    .sign = 's',					 \
    .realbits = 12,					 \
    .storagebits = 16,				 \
    .endianness = IIO_BE,				 \
    },							 \
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx931x_info {
    pub name: *const c_char,
    pub whoami: c_uint,
}

    static const struct iio_chan_spec sx9310_channels[] = {
    SX9310_CHANNEL(0),			/* CS0 */
    SX9310_CHANNEL(1),			/* CS1 */
    SX9310_CHANNEL(2),			/* CS2 */
    SX9310_NAMED_CHANNEL(3, "comb"),	/* COMB */
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
//
// Each entry contains the integer part (val) and the fractional part, in micro
// seconds. It conforms to the IIO output IIO_VAL_INT_PLUS_MICRO.
//
    static const struct {
    int val;
    int val2;
    } sx9310_samp_freq_table[] = {
    { 500, 0 }, /* 0000: Min (no idle time) */
    { 66, 666666 }, /* 0001: 15 ms */
    { 33, 333333 }, /* 0010: 30 ms (Typ.) */
    { 22, 222222 }, /* 0011: 45 ms */
    { 16, 666666 }, /* 0100: 60 ms */
    { 11, 111111 }, /* 0101: 90 ms */
    { 8, 333333 }, /* 0110: 120 ms */
    { 5, 0 }, /* 0111: 200 ms */
    { 2, 500000 }, /* 1000: 400 ms */
    { 1, 666666 }, /* 1001: 600 ms */
    { 1, 250000 }, /* 1010: 800 ms */
    { 1, 0 }, /* 1011: 1 s */
    { 0, 500000 }, /* 1100: 2 s */
    { 0, 333333 }, /* 1101: 3 s */
    { 0, 250000 }, /* 1110: 4 s */
    { 0, 200000 }, /* 1111: 5 s */
    };
    static const unsigned int sx9310_scan_period_table[] = {
    2,   15,  30,  45,   60,   90,	 120,  200,
    400, 600, 800, 1000, 2000, 3000, 4000, 5000,
    };
    static const struct regmap_range sx9310_writable_reg_ranges[] = {
    regmap_reg_range(SX9310_REG_IRQ_MSK, SX9310_REG_IRQ_FUNC),
    regmap_reg_range(SX9310_REG_PROX_CTRL0, SX9310_REG_PROX_CTRL19),
    regmap_reg_range(SX9310_REG_SAR_CTRL0, SX9310_REG_SAR_CTRL2),
    regmap_reg_range(SX9310_REG_SENSOR_SEL, SX9310_REG_SENSOR_SEL),
    regmap_reg_range(SX9310_REG_OFFSET_MSB, SX9310_REG_OFFSET_LSB),
    regmap_reg_range(SX9310_REG_PAUSE, SX9310_REG_PAUSE),
    regmap_reg_range(SX9310_REG_RESET, SX9310_REG_RESET),
    };
    static const struct regmap_access_table sx9310_writeable_regs = {
    .yes_ranges = sx9310_writable_reg_ranges,
    .n_yes_ranges = ARRAY_SIZE(sx9310_writable_reg_ranges),
    };
    static const struct regmap_range sx9310_readable_reg_ranges[] = {
    regmap_reg_range(SX9310_REG_IRQ_SRC, SX9310_REG_IRQ_FUNC),
    regmap_reg_range(SX9310_REG_PROX_CTRL0, SX9310_REG_PROX_CTRL19),
    regmap_reg_range(SX9310_REG_SAR_CTRL0, SX9310_REG_SAR_CTRL2),
    regmap_reg_range(SX9310_REG_SENSOR_SEL, SX9310_REG_SAR_LSB),
    regmap_reg_range(SX9310_REG_I2C_ADDR, SX9310_REG_WHOAMI),
    regmap_reg_range(SX9310_REG_RESET, SX9310_REG_RESET),
    };
    static const struct regmap_access_table sx9310_readable_regs = {
    .yes_ranges = sx9310_readable_reg_ranges,
    .n_yes_ranges = ARRAY_SIZE(sx9310_readable_reg_ranges),
    };
    static const struct regmap_range sx9310_volatile_reg_ranges[] = {
    regmap_reg_range(SX9310_REG_IRQ_SRC, SX9310_REG_STAT1),
    regmap_reg_range(SX9310_REG_USE_MSB, SX9310_REG_DIFF_LSB),
    regmap_reg_range(SX9310_REG_SAR_MSB, SX9310_REG_SAR_LSB),
    regmap_reg_range(SX9310_REG_RESET, SX9310_REG_RESET),
    };
    static const struct regmap_access_table sx9310_volatile_regs = {
    .yes_ranges = sx9310_volatile_reg_ranges,
    .n_yes_ranges = ARRAY_SIZE(sx9310_volatile_reg_ranges),
    };
    static const struct regmap_config sx9310_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = SX9310_REG_RESET,
    .cache_type = REGCACHE_RBTREE,
    .wr_table = &sx9310_writeable_regs,
    .rd_table = &sx9310_readable_regs,
    .volatile_table = &sx9310_volatile_regs,
    };
    static int sx9310_read_prox_data(struct sx_common_data *data,
    const struct iio_chan_spec *chan, __be16 *val)
    {
    int ret;
    ret = regmap_write(data.regmap, SX9310_REG_SENSOR_SEL, chan.channel);
    if (ret)
    return ret;
    return regmap_bulk_read(data.regmap, chan.address, val, sizeof(*val));
    }
//
// If we have no interrupt support, we have to wait for a scan period
// after enabling a channel to get a result.
//
#[no_mangle]
unsafe extern "C" fn sx9310_wait_for_sample(data: *mut sx_common_data) -> c_int {
    static int sx9310_wait_for_sample(struct sx_common_data *data)
    {
    int ret;
    unsigned int val;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL0, &val);
    if (ret)
    return ret;
    val = FIELD_GET(SX9310_REG_PROX_CTRL0_SCANPERIOD_MASK, val);
    msleep(sx9310_scan_period_table[val]);
    return 0;
    }
    static int sx9310_read_gain(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int *val)
    {
    unsigned int regval, gain;
    int ret;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL3, &regval);
    if (ret)
    return ret;
    switch (chan.channel) {
    case 0:
    case 3:
    gain = FIELD_GET(SX9310_REG_PROX_CTRL3_GAIN0_MASK, regval);
    break;
    case 1:
    case 2:
    gain = FIELD_GET(SX9310_REG_PROX_CTRL3_GAIN12_MASK, regval);
    break;
    default:
    return -EINVAL;
    }
// val = 1 << gain;
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn sx9310_read_samp_freq(data: *mut sx_common_data, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int sx9310_read_samp_freq(struct sx_common_data *data, int *val, int *val2)
    {
    unsigned int regval;
    int ret;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL0, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(SX9310_REG_PROX_CTRL0_SCANPERIOD_MASK, regval);
// val = sx9310_samp_freq_table[regval].val;
// val2 = sx9310_samp_freq_table[regval].val2;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    static int sx9310_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int *val,
    int *val2, long mask)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    int ret;
    if (chan.type != IIO_PROXIMITY)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = sx_common_read_proximity(data, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_HARDWAREGAIN:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = sx9310_read_gain(data, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SAMP_FREQ:
    return sx9310_read_samp_freq(data, val, val2);
    default:
    return -EINVAL;
    }
    }
    static const int sx9310_gain_vals[] = { 1, 2, 4, 8 };
    static int sx9310_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    if (chan.type != IIO_PROXIMITY)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_HARDWAREGAIN:
// type = IIO_VAL_INT;
// length = ARRAY_SIZE(sx9310_gain_vals);
// vals = sx9310_gain_vals;
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_SAMP_FREQ:
// type = IIO_VAL_INT_PLUS_MICRO;
// length = ARRAY_SIZE(sx9310_samp_freq_table) * 2;
// vals = (int *)sx9310_samp_freq_table;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static const unsigned int sx9310_pthresh_codes[] = {
    2, 4, 6, 8, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 72, 80, 88, 96, 112,
    128, 144, 160, 192, 224, 256, 320, 384, 512, 640, 768, 1024, 1536
    };
#[no_mangle]
unsafe extern "C" fn sx9310_get_thresh_reg(channel: c_uint) -> c_int {
    static int sx9310_get_thresh_reg(unsigned int channel)
    {
    switch (channel) {
    case 0:
    case 3:
    return SX9310_REG_PROX_CTRL8;
    case 1:
    case 2:
    return SX9310_REG_PROX_CTRL9;
    default:
    return -EINVAL;
    }
    }
    static int sx9310_read_thresh(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int *val)
    {
    unsigned int reg;
    unsigned int regval;
    int ret;
    reg = ret = sx9310_get_thresh_reg(chan.channel);
    if (ret < 0)
    return ret;
    ret = regmap_read(data.regmap, reg, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(SX9310_REG_PROX_CTRL8_9_PTHRESH_MASK, regval);
    if (regval >= ARRAY_SIZE(sx9310_pthresh_codes))
    return -EINVAL;
// val = sx9310_pthresh_codes[regval];
    return IIO_VAL_INT;
    }
    static int sx9310_read_hysteresis(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int *val)
    {
    unsigned int regval, pthresh;
    int ret;
    ret = sx9310_read_thresh(data, chan, &pthresh);
    if (ret < 0)
    return ret;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL10, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(SX9310_REG_PROX_CTRL10_HYST_MASK, regval);
    if (!regval)
    regval = 5;
// regval is at most 5
// val = pthresh >> (5 - regval);
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn sx9310_read_far_debounce(data: *mut sx_common_data, val: *mut c_int) -> c_int {
    static int sx9310_read_far_debounce(struct sx_common_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL10, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_MASK, regval);
    if (regval)
// val = 1 << regval;
    else
// val = 0;
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn sx9310_read_close_debounce(data: *mut sx_common_data, val: *mut c_int) -> c_int {
    static int sx9310_read_close_debounce(struct sx_common_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL10, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(SX9310_REG_PROX_CTRL10_CLOSE_DEBOUNCE_MASK, regval);
    if (regval)
// val = 1 << regval;
    else
// val = 0;
    return IIO_VAL_INT;
    }
    static int sx9310_read_event_val(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int *val, int *val2)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    if (chan.type != IIO_PROXIMITY)
    return -EINVAL;
    switch (info) {
    case IIO_EV_INFO_VALUE:
    return sx9310_read_thresh(data, chan, val);
    case IIO_EV_INFO_PERIOD:
    switch (dir) {
    case IIO_EV_DIR_RISING:
    return sx9310_read_far_debounce(data, val);
    case IIO_EV_DIR_FALLING:
    return sx9310_read_close_debounce(data, val);
    default:
    return -EINVAL;
    }
    case IIO_EV_INFO_HYSTERESIS:
    return sx9310_read_hysteresis(data, chan, val);
    default:
    return -EINVAL;
    }
    }
    static int sx9310_write_thresh(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int val)
    {
    unsigned int reg;
    unsigned int regval;
    int ret, i;
    reg = ret = sx9310_get_thresh_reg(chan.channel);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(sx9310_pthresh_codes); i++) {
    if (sx9310_pthresh_codes[i] == val) {
    regval = i;
    break;
    }
    }
    if (i == ARRAY_SIZE(sx9310_pthresh_codes))
    return -EINVAL;
    regval = FIELD_PREP(SX9310_REG_PROX_CTRL8_9_PTHRESH_MASK, regval);
    guard(mutex)(&data.mutex);
    return regmap_update_bits(data.regmap, reg,
    SX9310_REG_PROX_CTRL8_9_PTHRESH_MASK, regval);
    }
    static int sx9310_write_hysteresis(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int _val)
    {
    unsigned int hyst, val = _val;
    int ret, pthresh;
    ret = sx9310_read_thresh(data, chan, &pthresh);
    if (ret < 0)
    return ret;
    if (val == 0)
    hyst = 0;
#[no_mangle]
pub unsafe extern "C" fn if(2: val == pthresh >>) -> else {
    else if (val == pthresh >> 2)
    hyst = 3;
#[no_mangle]
pub unsafe extern "C" fn if(3: val == pthresh >>) -> else {
    else if (val == pthresh >> 3)
    hyst = 2;
#[no_mangle]
pub unsafe extern "C" fn if(4: val == pthresh >>) -> else {
    else if (val == pthresh >> 4)
    hyst = 1;
    else
    return -EINVAL;
    hyst = FIELD_PREP(SX9310_REG_PROX_CTRL10_HYST_MASK, hyst);
    guard(mutex)(&data.mutex);
    return regmap_update_bits(data.regmap, SX9310_REG_PROX_CTRL10,
    SX9310_REG_PROX_CTRL10_HYST_MASK, hyst);
    }
#[no_mangle]
unsafe extern "C" fn sx9310_write_far_debounce(data: *mut sx_common_data, val: c_int) -> c_int {
    static int sx9310_write_far_debounce(struct sx_common_data *data, int val)
    {
    unsigned int regval;
    if (val > 0)
    val = ilog2(val);
    if (!FIELD_FIT(SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_MASK, val))
    return -EINVAL;
    regval = FIELD_PREP(SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_MASK, val);
    guard(mutex)(&data.mutex);
    return regmap_update_bits(data.regmap, SX9310_REG_PROX_CTRL10,
    SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_MASK,
    regval);
    }
#[no_mangle]
unsafe extern "C" fn sx9310_write_close_debounce(data: *mut sx_common_data, val: c_int) -> c_int {
    static int sx9310_write_close_debounce(struct sx_common_data *data, int val)
    {
    unsigned int regval;
    if (val > 0)
    val = ilog2(val);
    if (!FIELD_FIT(SX9310_REG_PROX_CTRL10_CLOSE_DEBOUNCE_MASK, val))
    return -EINVAL;
    regval = FIELD_PREP(SX9310_REG_PROX_CTRL10_CLOSE_DEBOUNCE_MASK, val);
    guard(mutex)(&data.mutex);
    return regmap_update_bits(data.regmap, SX9310_REG_PROX_CTRL10,
    SX9310_REG_PROX_CTRL10_CLOSE_DEBOUNCE_MASK,
    regval);
    }
    static int sx9310_write_event_val(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int val, int val2)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    if (chan.type != IIO_PROXIMITY)
    return -EINVAL;
    switch (info) {
    case IIO_EV_INFO_VALUE:
    return sx9310_write_thresh(data, chan, val);
    case IIO_EV_INFO_PERIOD:
    switch (dir) {
    case IIO_EV_DIR_RISING:
    return sx9310_write_far_debounce(data, val);
    case IIO_EV_DIR_FALLING:
    return sx9310_write_close_debounce(data, val);
    default:
    return -EINVAL;
    }
    case IIO_EV_INFO_HYSTERESIS:
    return sx9310_write_hysteresis(data, chan, val);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn sx9310_set_samp_freq(data: *mut sx_common_data, val: c_int, val2: c_int) -> c_int {
    static int sx9310_set_samp_freq(struct sx_common_data *data, int val, int val2)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(sx9310_samp_freq_table); i++)
    if (val == sx9310_samp_freq_table[i].val &&
    val2 == sx9310_samp_freq_table[i].val2)
    break;
    if (i == ARRAY_SIZE(sx9310_samp_freq_table))
    return -EINVAL;
    guard(mutex)(&data.mutex);
    return regmap_update_bits(
    data.regmap, SX9310_REG_PROX_CTRL0,
    SX9310_REG_PROX_CTRL0_SCANPERIOD_MASK,
    FIELD_PREP(SX9310_REG_PROX_CTRL0_SCANPERIOD_MASK, i));
    }
    static int sx9310_write_gain(struct sx_common_data *data,
    const struct iio_chan_spec *chan, int val)
    {
    unsigned int gain, mask;
    gain = ilog2(val);
    switch (chan.channel) {
    case 0:
    case 3:
    mask = SX9310_REG_PROX_CTRL3_GAIN0_MASK;
    gain = FIELD_PREP(SX9310_REG_PROX_CTRL3_GAIN0_MASK, gain);
    break;
    case 1:
    case 2:
    mask = SX9310_REG_PROX_CTRL3_GAIN12_MASK;
    gain = FIELD_PREP(SX9310_REG_PROX_CTRL3_GAIN12_MASK, gain);
    break;
    default:
    return -EINVAL;
    }
    guard(mutex)(&data.mutex);
    return regmap_update_bits(data.regmap, SX9310_REG_PROX_CTRL3, mask,
    gain);
    }
    static int sx9310_write_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int val, int val2,
    long mask)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    if (chan.type != IIO_PROXIMITY)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    return sx9310_set_samp_freq(data, val, val2);
    case IIO_CHAN_INFO_HARDWAREGAIN:
    return sx9310_write_gain(data, chan, val);
    default:
    return -EINVAL;
    }
    }
    static const struct sx_common_reg_default sx9310_default_regs[] = {
    { SX9310_REG_IRQ_MSK, 0x00 },
    { SX9310_REG_IRQ_FUNC, 0x00 },
//
// The lower 4 bits should not be set as it enable sensors measurements.
// Turning the detection on before the configuration values are set to
// good values can cause the device to return erroneous readings.
//
    { SX9310_REG_PROX_CTRL0, SX9310_REG_PROX_CTRL0_SCANPERIOD_15MS },
    { SX9310_REG_PROX_CTRL1, 0x00 },
    { SX9310_REG_PROX_CTRL2, SX9310_REG_PROX_CTRL2_COMBMODE_CS1_CS2 |
    SX9310_REG_PROX_CTRL2_SHIELDEN_DYNAMIC },
    { SX9310_REG_PROX_CTRL3, SX9310_REG_PROX_CTRL3_GAIN0_X8 |
    SX9310_REG_PROX_CTRL3_GAIN12_X4 },
    { SX9310_REG_PROX_CTRL4, SX9310_REG_PROX_CTRL4_RESOLUTION_FINEST },
    { SX9310_REG_PROX_CTRL5, SX9310_REG_PROX_CTRL5_RANGE_SMALL |
    SX9310_REG_PROX_CTRL5_STARTUPSENS_CS1 |
    SX9310_REG_PROX_CTRL5_RAWFILT_1P25 },
    { SX9310_REG_PROX_CTRL6, SX9310_REG_PROX_CTRL6_AVGTHRESH_DEFAULT },
    { SX9310_REG_PROX_CTRL7, SX9310_REG_PROX_CTRL7_AVGNEGFILT_2 |
    SX9310_REG_PROX_CTRL7_AVGPOSFILT_512 },
    { SX9310_REG_PROX_CTRL8, SX9310_REG_PROX_CTRL8_9_PTHRESH_96 |
    SX9310_REG_PROX_CTRL8_9_BODYTHRESH_1500 },
    { SX9310_REG_PROX_CTRL9, SX9310_REG_PROX_CTRL8_9_PTHRESH_28 |
    SX9310_REG_PROX_CTRL8_9_BODYTHRESH_900 },
    { SX9310_REG_PROX_CTRL10, SX9310_REG_PROX_CTRL10_HYST_6PCT |
    SX9310_REG_PROX_CTRL10_FAR_DEBOUNCE_2 },
    { SX9310_REG_PROX_CTRL11, 0x00 },
    { SX9310_REG_PROX_CTRL12, 0x00 },
    { SX9310_REG_PROX_CTRL13, 0x00 },
    { SX9310_REG_PROX_CTRL14, 0x00 },
    { SX9310_REG_PROX_CTRL15, 0x00 },
    { SX9310_REG_PROX_CTRL16, 0x00 },
    { SX9310_REG_PROX_CTRL17, 0x00 },
    { SX9310_REG_PROX_CTRL18, 0x00 },
    { SX9310_REG_PROX_CTRL19, 0x00 },
    { SX9310_REG_SAR_CTRL0, SX9310_REG_SAR_CTRL0_SARDEB_4_SAMPLES |
    SX9310_REG_SAR_CTRL0_SARHYST_8 },
    { SX9310_REG_SAR_CTRL1, SX9310_REG_SAR_CTRL1_SLOPE(10781250) },
    { SX9310_REG_SAR_CTRL2, SX9310_REG_SAR_CTRL2_SAROFFSET_DEFAULT },
    };
// Activate all channels and perform an initial compensation.
#[no_mangle]
unsafe extern "C" fn sx9310_init_compensation(indio_dev: *mut iio_dev) -> c_int {
    static int sx9310_init_compensation(struct iio_dev *indio_dev)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    int ret;
    unsigned int val;
    unsigned int ctrl0;
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL0, &ctrl0);
    if (ret)
    return ret;
// run the compensation phase on all channels
    ret = regmap_write(data.regmap, SX9310_REG_PROX_CTRL0,
    ctrl0 | SX9310_REG_PROX_CTRL0_SENSOREN_MASK);
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(data.regmap, SX9310_REG_STAT1, val,
    !(val & SX9310_REG_STAT1_COMPSTAT_MASK),
    20000, 2000000);
    if (ret)
    return ret;
    regmap_write(data.regmap, SX9310_REG_PROX_CTRL0, ctrl0);
    return ret;
    }
    static const struct sx_common_reg_default *
    sx9310_get_default_reg(struct device *dev, int idx,
    struct sx_common_reg_default *reg_def)
    {
    u32 combined[SX9310_NUM_CHANNELS];
    let mut start: u32 = 0, raw = 0, pos = 0;
    let mut comb_mask: c_ulong = 0;
    int ret, i, count;
    const char *res;
    memcpy(reg_def, &sx9310_default_regs[idx], sizeof(*reg_def));
    switch (reg_def.reg) {
    case SX9310_REG_PROX_CTRL2:
    if (device_property_read_bool(dev, "semtech,cs0-ground")) {
    reg_def.def &= ~SX9310_REG_PROX_CTRL2_SHIELDEN_MASK;
    reg_def.def |= SX9310_REG_PROX_CTRL2_SHIELDEN_GROUND;
    }
    count = device_property_count_u32(dev, "semtech,combined-sensors");
    if (count < 0 || count > ARRAY_SIZE(combined))
    break;
    ret = device_property_read_u32_array(dev, "semtech,combined-sensors",
    combined, count);
    if (ret)
    break;
    for (i = 0; i < count; i++)
    comb_mask |= BIT(combined[i]);
    reg_def.def &= ~SX9310_REG_PROX_CTRL2_COMBMODE_MASK;
    if (comb_mask == (BIT(3) | BIT(2) | BIT(1) | BIT(0)))
    reg_def.def |= SX9310_REG_PROX_CTRL2_COMBMODE_CS0_CS1_CS2_CS3;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(2)): comb_mask == (BIT(1) |) -> else {
    else if (comb_mask == (BIT(1) | BIT(2)))
    reg_def.def |= SX9310_REG_PROX_CTRL2_COMBMODE_CS1_CS2;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(1)): comb_mask == (BIT(0) |) -> else {
    else if (comb_mask == (BIT(0) | BIT(1)))
    reg_def.def |= SX9310_REG_PROX_CTRL2_COMBMODE_CS0_CS1;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(3): comb_mask ==) -> else {
    else if (comb_mask == BIT(3))
    reg_def.def |= SX9310_REG_PROX_CTRL2_COMBMODE_CS3;
    break;
    case SX9310_REG_PROX_CTRL4:
    ret = device_property_read_string(dev, "semtech,resolution", &res);
    if (ret)
    break;
    reg_def.def &= ~SX9310_REG_PROX_CTRL4_RESOLUTION_MASK;
    if (!strcmp(res, "coarsest"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_COARSEST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "very-coarse")) -> else {
    else if (!strcmp(res, "very-coarse"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_VERY_COARSE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "coarse")) -> else {
    else if (!strcmp(res, "coarse"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_COARSE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "medium-coarse")) -> else {
    else if (!strcmp(res, "medium-coarse"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_MEDIUM_COARSE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "medium")) -> else {
    else if (!strcmp(res, "medium"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_MEDIUM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "fine")) -> else {
    else if (!strcmp(res, "fine"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_FINE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "very-fine")) -> else {
    else if (!strcmp(res, "very-fine"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_VERY_FINE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(res, _arg: "finest")) -> else {
    else if (!strcmp(res, "finest"))
    reg_def.def |= SX9310_REG_PROX_CTRL4_RESOLUTION_FINEST;
    break;
    case SX9310_REG_PROX_CTRL5:
    ret = device_property_read_u32(dev, "semtech,startup-sensor", &start);
    if (ret) {
    start = FIELD_GET(SX9310_REG_PROX_CTRL5_STARTUPSENS_MASK,
    reg_def.def);
    }
    reg_def.def &= ~SX9310_REG_PROX_CTRL5_STARTUPSENS_MASK;
    reg_def.def |= FIELD_PREP(SX9310_REG_PROX_CTRL5_STARTUPSENS_MASK,
    start);
    ret = device_property_read_u32(dev, "semtech,proxraw-strength", &raw);
    if (ret) {
    raw = FIELD_GET(SX9310_REG_PROX_CTRL5_RAWFILT_MASK,
    reg_def.def);
    } else {
    raw = ilog2(raw);
    }
    reg_def.def &= ~SX9310_REG_PROX_CTRL5_RAWFILT_MASK;
    reg_def.def |= FIELD_PREP(SX9310_REG_PROX_CTRL5_RAWFILT_MASK,
    raw);
    break;
    case SX9310_REG_PROX_CTRL7:
    ret = device_property_read_u32(dev, "semtech,avg-pos-strength", &pos);
    if (ret)
    break;
// Powers of 2, except for a gap between 16 and 64
    pos = clamp(ilog2(pos), 3, 11) - (pos >= 32 ? 4 : 3);
    reg_def.def &= ~SX9310_REG_PROX_CTRL7_AVGPOSFILT_MASK;
    reg_def.def |= FIELD_PREP(SX9310_REG_PROX_CTRL7_AVGPOSFILT_MASK,
    pos);
    break;
    }
    return reg_def;
    }
    static int sx9310_check_whoami(struct device *dev,
    struct iio_dev *indio_dev)
    {
    struct sx_common_data *data = iio_priv(indio_dev);
    const struct sx931x_info *ddata;
    unsigned int whoami;
    int ret;
    ret = regmap_read(data.regmap, SX9310_REG_WHOAMI, &whoami);
    if (ret)
    return ret;
    ddata = device_get_match_data(dev);
    if (ddata.whoami != whoami)
    return -ENODEV;
    indio_dev.name = ddata.name;
    return 0;
    }
    static const struct sx_common_chip_info sx9310_chip_info = {
    .reg_stat = SX9310_REG_STAT0,
    .reg_irq_msk = SX9310_REG_IRQ_MSK,
    .reg_enable_chan = SX9310_REG_PROX_CTRL0,
    .reg_reset = SX9310_REG_RESET,
    .mask_enable_chan = SX9310_REG_STAT1_COMPSTAT_MASK,
    .irq_msk_offset = 3,
    .num_channels = SX9310_NUM_CHANNELS,
    .num_default_regs = ARRAY_SIZE(sx9310_default_regs),
    .ops = {
    .read_prox_data = sx9310_read_prox_data,
    .check_whoami = sx9310_check_whoami,
    .init_compensation = sx9310_init_compensation,
    .wait_for_sample = sx9310_wait_for_sample,
    .get_default_reg = sx9310_get_default_reg,
    },
    .iio_channels = sx9310_channels,
    .num_iio_channels = ARRAY_SIZE(sx9310_channels),
    .iio_info =  {
    .read_raw = sx9310_read_raw,
    .read_avail = sx9310_read_avail,
    .read_event_value = sx9310_read_event_val,
    .write_event_value = sx9310_write_event_val,
    .write_raw = sx9310_write_raw,
    .read_event_config = sx_common_read_event_config,
    .write_event_config = sx_common_write_event_config,
    },
    };
#[no_mangle]
unsafe extern "C" fn sx9310_probe(client: *mut i2c_client) -> c_int {
    static int sx9310_probe(struct i2c_client *client)
    {
    return sx_common_probe(client, &sx9310_chip_info, &sx9310_regmap_config);
    }
#[no_mangle]
unsafe extern "C" fn sx9310_suspend(dev: *mut device) -> c_int {
    static int sx9310_suspend(struct device *dev)
    {
    struct sx_common_data *data = iio_priv(dev_get_drvdata(dev));
    u8 ctrl0;
    int ret;
    disable_irq_nosync(data.client.irq);
    guard(mutex)(&data.mutex);
    ret = regmap_read(data.regmap, SX9310_REG_PROX_CTRL0,
    &data.suspend_ctrl);
    if (ret)
    return ret;
    ctrl0 = data.suspend_ctrl & ~SX9310_REG_PROX_CTRL0_SENSOREN_MASK;
    ret = regmap_write(data.regmap, SX9310_REG_PROX_CTRL0, ctrl0);
    if (ret)
    return ret;
    return regmap_write(data.regmap, SX9310_REG_PAUSE, 0);
    }
#[no_mangle]
unsafe extern "C" fn sx9310_resume(dev: *mut device) -> c_int {
    static int sx9310_resume(struct device *dev)
    {
    struct sx_common_data *data = iio_priv(dev_get_drvdata(dev));
    int ret;
    scoped_guard(mutex, &data.mutex) {
    ret = regmap_write(data.regmap, SX9310_REG_PAUSE, 1);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, SX9310_REG_PROX_CTRL0,
    data.suspend_ctrl);
    if (ret)
    return ret;
    }
    enable_irq(data.client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sx9310_pm_ops, sx9310_suspend, sx9310_resume);
    static const struct sx931x_info sx9310_info = {
    .name = "sx9310",
    .whoami = SX9310_WHOAMI_VALUE,
    };
    static const struct sx931x_info sx9311_info = {
    .name = "sx9311",
    .whoami = SX9311_WHOAMI_VALUE,
    };
    static const struct acpi_device_id sx9310_acpi_match[] = {
    { "STH9310", (kernel_ulong_t)&sx9310_info },
    { "STH9311", (kernel_ulong_t)&sx9311_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, sx9310_acpi_match);
    static const struct of_device_id sx9310_of_match[] = {
    { .compatible = "semtech,sx9310", &sx9310_info },
    { .compatible = "semtech,sx9311", &sx9311_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, sx9310_of_match);
    static const struct i2c_device_id sx9310_id[] = {
    { .name = "sx9310", .driver_data = (kernel_ulong_t)&sx9310_info },
    { .name = "sx9311", .driver_data = (kernel_ulong_t)&sx9311_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sx9310_id);
    static struct i2c_driver sx9310_driver = {
    .driver = {
    .name	= "sx9310",
    .acpi_match_table = sx9310_acpi_match,
    .of_match_table = sx9310_of_match,
    .pm = pm_sleep_ptr(&sx9310_pm_ops),
//
// Lots of i2c transfers in probe + over 200 ms waiting in
// sx9310_init_compensation() mean a slow probe; prefer async
// so we don't delay boot if we're builtin to the kernel.
//
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= sx9310_probe,
    .id_table	= sx9310_id,
    };
    module_i2c_driver(sx9310_driver);
    MODULE_AUTHOR("Gwendal Grignou <gwendal@chromium.org>");
    MODULE_AUTHOR("Daniel Campello <campello@chromium.org>");
    MODULE_DESCRIPTION("Driver for Semtech SX9310/SX9311 proximity sensor");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("SEMTECH_PROX");
