//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/meson_saradc.c
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
// Amlogic Meson Successive Approximation Register (SAR) A/D Converter
//
// Copyright (C) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

pub const MESON_SAR_ADC_REG0: c_uint = 0x00;

pub const MESON_SAR_ADC_CHAN_LIST: c_uint = 0x04;

    (GENMASK(2, 0) << ((_chan) * 3))
pub const MESON_SAR_ADC_AVG_CNTL: c_uint = 0x08;

    (16 + ((_chan) * 2))

    (GENMASK(17, 16) << ((_chan) * 2))

    (0 + ((_chan) * 2))

    (GENMASK(1, 0) << ((_chan) * 2))
pub const MESON_SAR_ADC_REG3: c_uint = 0x0c;

pub const MESON_SAR_ADC_REG3_ADC_CLK_DIV_SHIFT: c_int = 10;
pub const MESON_SAR_ADC_REG3_ADC_CLK_DIV_WIDTH: c_int = 6;

pub const MESON_SAR_ADC_DELAY: c_uint = 0x10;

pub const MESON_SAR_ADC_LAST_RD: c_uint = 0x14;

pub const MESON_SAR_ADC_FIFO_RD: c_uint = 0x18;

pub const MESON_SAR_ADC_AUX_SW: c_uint = 0x1c;

    (8 + (((_chan) - 2) * 3))

pub const MESON_SAR_ADC_CHAN_10_SW: c_uint = 0x20;

pub const MESON_SAR_ADC_DETECT_IDLE_SW: c_uint = 0x24;

pub const MESON_SAR_ADC_DELTA_10: c_uint = 0x28;

//
// NOTE: registers from here are undocumented (the vendor Linux kernel driver
// and u-boot source served as reference). These only seem to be relevant on
// GXBB and newer.
//
pub const MESON_SAR_ADC_REG11: c_uint = 0x2c;

pub const MESON_SAR_ADC_REG12: c_uint = 0x30;

pub const MESON_SAR_ADC_REG13: c_uint = 0x34;

pub const MESON_SAR_ADC_MAX_FIFO_SIZE: c_int = 32;

pub const MESON_SAR_ADC_VOLTAGE_AND_TEMP_CHANNEL: c_int = 6;
pub const MESON_SAR_ADC_VOLTAGE_AND_MUX_CHANNEL: c_int = 7;
pub const MESON_SAR_ADC_TEMP_OFFSET: c_int = 27;
// temperature sensor calibration information in eFuse
pub const MESON_SAR_ADC_EFUSE_BYTES: c_int = 4;

pub const MESON_HHI_DPLL_TOP_0: c_uint = 0x318;

// for use with IIO_VAL_INT_PLUS_MICRO
pub const MILLION: c_int = 1000000;

    .type = IIO_VOLTAGE,						\
    .indexed = 1,							\
    .channel = _chan,						\
    .address = _chan,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_AVERAGE_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),		\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_CALIBBIAS) |	\
    BIT(IIO_CHAN_INFO_CALIBSCALE),		\
    .datasheet_name = "SAR_ADC_CH"#_chan,				\
    }

    .type = IIO_TEMP,						\
    .channel = _chan,						\
    .address = MESON_SAR_ADC_VOLTAGE_AND_TEMP_CHANNEL,		\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_AVERAGE_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_OFFSET) |		\
    BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_CALIBBIAS) |	\
    BIT(IIO_CHAN_INFO_CALIBSCALE),		\
    .datasheet_name = "TEMP_SENSOR",				\
    }

    .type = IIO_VOLTAGE,						\
    .channel = _chan,						\
    .indexed = 1,							\
    .address = MESON_SAR_ADC_VOLTAGE_AND_MUX_CHANNEL,		\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_AVERAGE_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),		\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_CALIBBIAS) |	\
    BIT(IIO_CHAN_INFO_CALIBSCALE),		\
    .datasheet_name = "SAR_ADC_MUX_"#_sel,				\
    }
    enum meson_sar_adc_vref_sel {
    VREF_CALIBATION_VOLTAGE = 0,
    VREF_VDDA = 1,
    };
    enum meson_sar_adc_avg_mode {
    NO_AVERAGING = 0x0,
    MEAN_AVERAGING = 0x1,
    MEDIAN_AVERAGING = 0x2,
    };
    enum meson_sar_adc_num_samples {
    ONE_SAMPLE = 0x0,
    TWO_SAMPLES = 0x1,
    FOUR_SAMPLES = 0x2,
    EIGHT_SAMPLES = 0x3,
    };
    enum meson_sar_adc_chan7_mux_sel {
    CHAN7_MUX_VSS = 0x0,
    CHAN7_MUX_VDD_DIV4 = 0x1,
    CHAN7_MUX_VDD_DIV2 = 0x2,
    CHAN7_MUX_VDD_MUL3_DIV4 = 0x3,
    CHAN7_MUX_VDD = 0x4,
    CHAN7_MUX_CH7_INPUT = 0x7,
    };
    enum meson_sar_adc_channel_index {
    NUM_CHAN_0,
    NUM_CHAN_1,
    NUM_CHAN_2,
    NUM_CHAN_3,
    NUM_CHAN_4,
    NUM_CHAN_5,
    NUM_CHAN_6,
    NUM_CHAN_7,
    NUM_CHAN_TEMP,
    NUM_MUX_0_VSS,
    NUM_MUX_1_VDD_DIV4,
    NUM_MUX_2_VDD_DIV2,
    NUM_MUX_3_VDD_MUL3_DIV4,
    NUM_MUX_4_VDD,
    };
    static enum meson_sar_adc_chan7_mux_sel chan7_mux_values[] = {
    CHAN7_MUX_VSS,
    CHAN7_MUX_VDD_DIV4,
    CHAN7_MUX_VDD_DIV2,
    CHAN7_MUX_VDD_MUL3_DIV4,
    CHAN7_MUX_VDD,
    };
    static const char * const chan7_mux_names[] = {
    [CHAN7_MUX_VSS] = "gnd",
    [CHAN7_MUX_VDD_DIV4] = "0.25vdd",
    [CHAN7_MUX_VDD_DIV2] = "0.5vdd",
    [CHAN7_MUX_VDD_MUL3_DIV4] = "0.75vdd",
    [CHAN7_MUX_VDD] = "vdd",
    };
    static const struct iio_chan_spec meson_sar_adc_iio_channels[] = {
    MESON_SAR_ADC_CHAN(NUM_CHAN_0),
    MESON_SAR_ADC_CHAN(NUM_CHAN_1),
    MESON_SAR_ADC_CHAN(NUM_CHAN_2),
    MESON_SAR_ADC_CHAN(NUM_CHAN_3),
    MESON_SAR_ADC_CHAN(NUM_CHAN_4),
    MESON_SAR_ADC_CHAN(NUM_CHAN_5),
    MESON_SAR_ADC_CHAN(NUM_CHAN_6),
    MESON_SAR_ADC_CHAN(NUM_CHAN_7),
    MESON_SAR_ADC_MUX(NUM_MUX_0_VSS, 0),
    MESON_SAR_ADC_MUX(NUM_MUX_1_VDD_DIV4, 1),
    MESON_SAR_ADC_MUX(NUM_MUX_2_VDD_DIV2, 2),
    MESON_SAR_ADC_MUX(NUM_MUX_3_VDD_MUL3_DIV4, 3),
    MESON_SAR_ADC_MUX(NUM_MUX_4_VDD, 4),
    };
    static const struct iio_chan_spec meson_sar_adc_and_temp_iio_channels[] = {
    MESON_SAR_ADC_CHAN(NUM_CHAN_0),
    MESON_SAR_ADC_CHAN(NUM_CHAN_1),
    MESON_SAR_ADC_CHAN(NUM_CHAN_2),
    MESON_SAR_ADC_CHAN(NUM_CHAN_3),
    MESON_SAR_ADC_CHAN(NUM_CHAN_4),
    MESON_SAR_ADC_CHAN(NUM_CHAN_5),
    MESON_SAR_ADC_CHAN(NUM_CHAN_6),
    MESON_SAR_ADC_CHAN(NUM_CHAN_7),
    MESON_SAR_ADC_TEMP_CHAN(NUM_CHAN_TEMP),
    MESON_SAR_ADC_MUX(NUM_MUX_0_VSS, 0),
    MESON_SAR_ADC_MUX(NUM_MUX_1_VDD_DIV4, 1),
    MESON_SAR_ADC_MUX(NUM_MUX_2_VDD_DIV2, 2),
    MESON_SAR_ADC_MUX(NUM_MUX_3_VDD_MUL3_DIV4, 3),
    MESON_SAR_ADC_MUX(NUM_MUX_4_VDD, 4),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_sar_adc_param {
    pub has_bl30_integration: bool,
    pub clock_rate: c_ulong,
    pub resolution: c_uint,
    pub regmap_config: *const regmap_config,
    pub temperature_trimming_bits: u8,
    pub temperature_multiplier: c_uint,
    pub temperature_divider: c_uint,
    pub disable_ring_counter: u8,
    pub has_vref_select: bool,
    pub vref_select: u8,
    pub cmv_select: u8,
    pub adc_eoc: u8,
    pub vref_voltage: enum meson_sar_adc_vref_sel,
    pub enable_mpll_clock_workaround: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_sar_adc_data {
    pub param: *const meson_sar_adc_param,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_sar_adc_priv {
    pub regmap: *mut regmap,
    pub vref: *mut regulator,
    pub param: *const meson_sar_adc_param,
    pub clkin: *mut clk,
    pub core_clk: *mut clk,
    pub adc_sel_clk: *mut clk,
    pub adc_clk: *mut clk,
    pub clk_gate: clk_gate,
    pub adc_div_clk: *mut clk,
    pub clk_div: clk_divider,
    pub done: completion,
// lock to protect against multiple access to the device
    pub lock: mutex,
    pub calibbias: c_int,
    pub calibscale: c_int,
    pub tsc_regmap: *mut regmap,
    pub temperature_sensor_calibrated: bool,
    pub temperature_sensor_coefficient: u8,
    pub temperature_sensor_adc_val: u16,
    pub chan7_mux_sel: enum meson_sar_adc_chan7_mux_sel,
}

    static const struct regmap_config meson_sar_adc_regmap_config_gxbb = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = MESON_SAR_ADC_REG13,
    };
    static const struct regmap_config meson_sar_adc_regmap_config_meson8 = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = MESON_SAR_ADC_DELTA_10,
    };
    static const struct iio_chan_spec *
    find_channel_by_num(struct iio_dev *indio_dev, int num)
    {
    int i;
    for (i = 0; i < indio_dev.num_channels; i++)
    if (indio_dev.channels[i].channel == num)
    return &indio_dev.channels[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_get_fifo_count(indio_dev: *mut iio_dev) -> c_uint {
    static unsigned int meson_sar_adc_get_fifo_count(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    u32 regval;
    regmap_read(priv.regmap, MESON_SAR_ADC_REG0, &regval);
    return FIELD_GET(MESON_SAR_ADC_REG0_FIFO_COUNT_MASK, regval);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_calib_val(indio_dev: *mut iio_dev, val: c_int) -> c_int {
    static int meson_sar_adc_calib_val(struct iio_dev *indio_dev, int val)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int tmp;
// use val_calib = scale * val_raw + offset calibration function
    tmp = div_s64((s64)val * priv.calibscale, MILLION) + priv.calibbias;
    return clamp(tmp, 0, (1 << priv.param.resolution) - 1);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_wait_busy_clear(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_wait_busy_clear(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int val;
//
// NOTE: we need a small delay before reading the status, otherwise
// the sample engine may not have started internally (which would
// seem to us that sampling is already finished).
//
    udelay(1);
    return regmap_read_poll_timeout_atomic(priv.regmap, MESON_SAR_ADC_REG0, val,
    !FIELD_GET(MESON_SAR_ADC_REG0_BUSY_MASK, val),
    1, 10000);
    }
    static void meson_sar_adc_set_chan7_mux(struct iio_dev *indio_dev,
    enum meson_sar_adc_chan7_mux_sel sel)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    u32 regval;
    regval = FIELD_PREP(MESON_SAR_ADC_REG3_CTRL_CHAN7_MUX_SEL_MASK, sel);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_CTRL_CHAN7_MUX_SEL_MASK, regval);
    usleep_range(10, 20);
    priv.chan7_mux_sel = sel;
    }
    static int meson_sar_adc_read_raw_sample(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    int regval, fifo_chan, fifo_val, count;
    if (!wait_for_completion_timeout(&priv.done,
    msecs_to_jiffies(MESON_SAR_ADC_TIMEOUT)))
    return -ETIMEDOUT;
    count = meson_sar_adc_get_fifo_count(indio_dev);
    if (count != 1) {
    dev_err(dev, "ADC FIFO has %d element(s) instead of one\n", count);
    return -EINVAL;
    }
    regmap_read(priv.regmap, MESON_SAR_ADC_FIFO_RD, &regval);
    fifo_chan = FIELD_GET(MESON_SAR_ADC_FIFO_RD_CHAN_ID_MASK, regval);
    if (fifo_chan != chan.address) {
    dev_err(dev, "ADC FIFO entry belongs to channel %d instead of %lu\n",
    fifo_chan, chan.address);
    return -EINVAL;
    }
    fifo_val = FIELD_GET(MESON_SAR_ADC_FIFO_RD_SAMPLE_VALUE_MASK, regval);
    fifo_val &= GENMASK(priv.param.resolution - 1, 0);
// val = meson_sar_adc_calib_val(indio_dev, fifo_val);
    return 0;
    }
    static void meson_sar_adc_set_averaging(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum meson_sar_adc_avg_mode mode,
    enum meson_sar_adc_num_samples samples)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int val, address = chan.address;
    val = samples << MESON_SAR_ADC_AVG_CNTL_NUM_SAMPLES_SHIFT(address);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_AVG_CNTL,
    MESON_SAR_ADC_AVG_CNTL_NUM_SAMPLES_MASK(address),
    val);
    val = mode << MESON_SAR_ADC_AVG_CNTL_AVG_MODE_SHIFT(address);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_AVG_CNTL,
    MESON_SAR_ADC_AVG_CNTL_AVG_MODE_MASK(address), val);
    }
    static void meson_sar_adc_enable_channel(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    u32 regval;
//
// the SAR ADC engine allows sampling multiple channels at the same
// time. to keep it simple we're only working with one *internal
// channel, which starts counting at index 0 (which means: count = 1).
//
    regval = FIELD_PREP(MESON_SAR_ADC_CHAN_LIST_MAX_INDEX_MASK, 0);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_CHAN_LIST,
    MESON_SAR_ADC_CHAN_LIST_MAX_INDEX_MASK, regval);
// map channel index 0 to the channel which we want to read
    regval = FIELD_PREP(MESON_SAR_ADC_CHAN_LIST_ENTRY_MASK(0),
    chan.address);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_CHAN_LIST,
    MESON_SAR_ADC_CHAN_LIST_ENTRY_MASK(0), regval);
    regval = FIELD_PREP(MESON_SAR_ADC_DETECT_IDLE_SW_DETECT_MUX_MASK,
    chan.address);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DETECT_IDLE_SW,
    MESON_SAR_ADC_DETECT_IDLE_SW_DETECT_MUX_MASK,
    regval);
    regval = FIELD_PREP(MESON_SAR_ADC_DETECT_IDLE_SW_IDLE_MUX_SEL_MASK,
    chan.address);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DETECT_IDLE_SW,
    MESON_SAR_ADC_DETECT_IDLE_SW_IDLE_MUX_SEL_MASK,
    regval);
    if (chan.address == MESON_SAR_ADC_VOLTAGE_AND_TEMP_CHANNEL) {
    if (chan.type == IIO_TEMP)
    regval = MESON_SAR_ADC_DELTA_10_TEMP_SEL;
    else
    regval = 0;
    regmap_update_bits(priv.regmap,
    MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TEMP_SEL, regval);
    } else if (chan.address == MESON_SAR_ADC_VOLTAGE_AND_MUX_CHANNEL) {
    enum meson_sar_adc_chan7_mux_sel sel;
    if (chan.channel == NUM_CHAN_7)
    sel = CHAN7_MUX_CH7_INPUT;
    else
    sel = chan7_mux_values[chan.channel - NUM_MUX_0_VSS];
    if (sel != priv.chan7_mux_sel)
    meson_sar_adc_set_chan7_mux(indio_dev, sel);
    }
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_start_sample_engine(indio_dev: *mut iio_dev) {
    static void meson_sar_adc_start_sample_engine(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    reinit_completion(&priv.done);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_FIFO_IRQ_EN);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_SAMPLE_ENGINE_ENABLE);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_SAMPLING_START);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_stop_sample_engine(indio_dev: *mut iio_dev) {
    static void meson_sar_adc_stop_sample_engine(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_FIFO_IRQ_EN);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_SAMPLING_STOP);
// wait until all modules are stopped
    meson_sar_adc_wait_busy_clear(indio_dev);
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_SAMPLE_ENGINE_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_lock(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_lock(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int val, ret;
    mutex_lock(&priv.lock);
    if (priv.param.has_bl30_integration) {
// prevent BL30 from using the SAR ADC while we are using it
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_KERNEL_BUSY);
    udelay(1);
//
// wait until BL30 releases it's lock (so we can use the SAR
// ADC)
//
    ret = regmap_read_poll_timeout_atomic(priv.regmap, MESON_SAR_ADC_DELAY, val,
    !(val & MESON_SAR_ADC_DELAY_BL30_BUSY),
    1, 10000);
    if (ret) {
    mutex_unlock(&priv.lock);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_unlock(indio_dev: *mut iio_dev) {
    static void meson_sar_adc_unlock(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    if (priv.param.has_bl30_integration)
// allow BL30 to use the SAR ADC again
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_KERNEL_BUSY);
    mutex_unlock(&priv.lock);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_clear_fifo(indio_dev: *mut iio_dev) {
    static void meson_sar_adc_clear_fifo(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    unsigned int count, tmp;
    for (count = 0; count < MESON_SAR_ADC_MAX_FIFO_SIZE; count++) {
    if (!meson_sar_adc_get_fifo_count(indio_dev))
    break;
    regmap_read(priv.regmap, MESON_SAR_ADC_FIFO_RD, &tmp);
    }
    }
    static int meson_sar_adc_get_sample(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum meson_sar_adc_avg_mode avg_mode,
    enum meson_sar_adc_num_samples avg_samples,
    int *val)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    int ret;
    if (chan.type == IIO_TEMP && !priv.temperature_sensor_calibrated)
    return -ENOTSUPP;
    ret = meson_sar_adc_lock(indio_dev);
    if (ret)
    return ret;
// clear the FIFO to make sure we're not reading old values
    meson_sar_adc_clear_fifo(indio_dev);
    meson_sar_adc_set_averaging(indio_dev, chan, avg_mode, avg_samples);
    meson_sar_adc_enable_channel(indio_dev, chan);
    meson_sar_adc_start_sample_engine(indio_dev);
    ret = meson_sar_adc_read_raw_sample(indio_dev, chan, val);
    meson_sar_adc_stop_sample_engine(indio_dev);
    meson_sar_adc_unlock(indio_dev);
    if (ret) {
    dev_warn(dev, "failed to read sample for channel %lu: %d\n",
    chan.address, ret);
    return ret;
    }
    return IIO_VAL_INT;
    }
    static int meson_sar_adc_iio_info_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return meson_sar_adc_get_sample(indio_dev, chan, NO_AVERAGING,
    ONE_SAMPLE, val);
    case IIO_CHAN_INFO_AVERAGE_RAW:
    return meson_sar_adc_get_sample(indio_dev, chan,
    MEAN_AVERAGING, EIGHT_SAMPLES,
    val);
    case IIO_CHAN_INFO_SCALE:
    if (chan.type == IIO_VOLTAGE) {
    ret = regulator_get_voltage(priv.vref);
    if (ret < 0) {
    dev_err(dev, "failed to get vref voltage: %d\n", ret);
    return ret;
    }
// val = ret / 1000;
// val2 = priv->param->resolution;
    return IIO_VAL_FRACTIONAL_LOG2;
    } else if (chan.type == IIO_TEMP) {
// SoC specific multiplier and divider
// val = priv->param->temperature_multiplier;
// val2 = priv->param->temperature_divider;
// celsius to millicelsius
// val *= 1000;
    return IIO_VAL_FRACTIONAL;
    } else {
    return -EINVAL;
    }
    case IIO_CHAN_INFO_CALIBBIAS:
// val = priv->calibbias;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_CALIBSCALE:
// val = priv->calibscale / MILLION;
// val2 = priv->calibscale % MILLION;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_OFFSET:
// val = DIV_ROUND_CLOSEST(MESON_SAR_ADC_TEMP_OFFSET
    priv.param.temperature_divider,
    priv.param.temperature_multiplier);
// val -= priv->temperature_sensor_adc_val;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int meson_sar_adc_clk_init(struct iio_dev *indio_dev,
    void __iomem *base)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    struct clk_init_data init;
    const char *clk_parents[1];
    init.name = devm_kasprintf(dev, GFP_KERNEL, "%s#adc_div", dev_name(dev));
    if (!init.name)
    return -ENOMEM;
    init.flags = 0;
    init.ops = &clk_divider_ops;
    clk_parents[0] = __clk_get_name(priv.clkin);
    init.parent_names = clk_parents;
    init.num_parents = 1;
    priv.clk_div.reg = base + MESON_SAR_ADC_REG3;
    priv.clk_div.shift = MESON_SAR_ADC_REG3_ADC_CLK_DIV_SHIFT;
    priv.clk_div.width = MESON_SAR_ADC_REG3_ADC_CLK_DIV_WIDTH;
    priv.clk_div.hw.init = &init;
    priv.clk_div.flags = 0;
    priv.adc_div_clk = devm_clk_register(dev, &priv.clk_div.hw);
    if (WARN_ON(IS_ERR(priv.adc_div_clk)))
    return PTR_ERR(priv.adc_div_clk);
    init.name = devm_kasprintf(dev, GFP_KERNEL, "%s#adc_en", dev_name(dev));
    if (!init.name)
    return -ENOMEM;
    init.flags = CLK_SET_RATE_PARENT;
    init.ops = &clk_gate_ops;
    clk_parents[0] = __clk_get_name(priv.adc_div_clk);
    init.parent_names = clk_parents;
    init.num_parents = 1;
    priv.clk_gate.reg = base + MESON_SAR_ADC_REG3;
    priv.clk_gate.bit_idx = __ffs(MESON_SAR_ADC_REG3_CLK_EN);
    priv.clk_gate.hw.init = &init;
    priv.adc_clk = devm_clk_register(dev, &priv.clk_gate.hw);
    if (WARN_ON(IS_ERR(priv.adc_clk)))
    return PTR_ERR(priv.adc_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_temp_sensor_init(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_temp_sensor_init(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    u8 *buf, trimming_bits, trimming_mask, upper_adc_val;
    struct device *dev = indio_dev.dev.parent;
    struct nvmem_cell *temperature_calib;
    size_t read_len;
    int ret;
    temperature_calib = nvmem_cell_get(dev, "temperature_calib");
    if (IS_ERR(temperature_calib)) {
    ret = PTR_ERR(temperature_calib);
//
// leave the temperature sensor disabled if no calibration data
// was passed via nvmem-cells.
//
    if (ret == -ENODEV)
    return 0;
    return dev_err_probe(dev, ret, "failed to get temperature_calib cell\n");
    }
    read_len = MESON_SAR_ADC_EFUSE_BYTES;
    buf = nvmem_cell_read(temperature_calib, &read_len);
    nvmem_cell_put(temperature_calib);
    if (IS_ERR(buf))
    return dev_err_probe(dev, PTR_ERR(buf), "failed to read temperature_calib cell\n");
    if (read_len != MESON_SAR_ADC_EFUSE_BYTES) {
    kfree(buf);
    return dev_err_probe(dev, -EINVAL, "invalid read size of temperature_calib cell\n");
    }
    priv.tsc_regmap = syscon_regmap_lookup_by_phandle(dev.of_node, "amlogic,hhi-sysctrl");
    if (IS_ERR(priv.tsc_regmap)) {
    kfree(buf);
    return dev_err_probe(dev, PTR_ERR(priv.tsc_regmap),
    "failed to get amlogic,hhi-sysctrl regmap\n");
    }
    trimming_bits = priv.param.temperature_trimming_bits;
    trimming_mask = BIT(trimming_bits) - 1;
    priv.temperature_sensor_calibrated =
    buf[3] & MESON_SAR_ADC_EFUSE_BYTE3_IS_CALIBRATED;
    priv.temperature_sensor_coefficient = buf[2] & trimming_mask;
    upper_adc_val = FIELD_GET(MESON_SAR_ADC_EFUSE_BYTE3_UPPER_ADC_VAL,
    buf[3]);
    priv.temperature_sensor_adc_val = buf[2];
    priv.temperature_sensor_adc_val |= upper_adc_val << BITS_PER_BYTE;
    priv.temperature_sensor_adc_val >>= trimming_bits;
    kfree(buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_init(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_init(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    int regval, i, ret;
//
// make sure we start at CH7 input since the other muxes are only used
// for internal calibration.
//
    meson_sar_adc_set_chan7_mux(indio_dev, CHAN7_MUX_CH7_INPUT);
    if (priv.param.has_bl30_integration) {
//
// leave sampling delay and the input clocks as configured by
// BL30 to make sure BL30 gets the values it expects when
// reading the temperature sensor.
//
    regmap_read(priv.regmap, MESON_SAR_ADC_REG3, &regval);
    if (regval & MESON_SAR_ADC_REG3_BL30_INITIALIZED)
    return 0;
    }
    meson_sar_adc_stop_sample_engine(indio_dev);
//
// disable this bit as seems to be only relevant for Meson6 (based
// on the vendor driver), which we don't support at the moment.
//
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_ADC_TEMP_SEN_SEL);
// disable all channels by default
    regmap_write(priv.regmap, MESON_SAR_ADC_CHAN_LIST, 0x0);
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_CTRL_SAMPLING_CLOCK_PHASE);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_CNTL_USE_SC_DLY);
// delay between two samples = (10+1) * 1uS
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_INPUT_DLY_CNT_MASK,
    FIELD_PREP(MESON_SAR_ADC_DELAY_SAMPLE_DLY_CNT_MASK,
    10));
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_SAMPLE_DLY_SEL_MASK,
    FIELD_PREP(MESON_SAR_ADC_DELAY_SAMPLE_DLY_SEL_MASK,
    0));
// delay between two samples = (10+1) * 1uS
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_INPUT_DLY_CNT_MASK,
    FIELD_PREP(MESON_SAR_ADC_DELAY_INPUT_DLY_CNT_MASK,
    10));
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELAY,
    MESON_SAR_ADC_DELAY_INPUT_DLY_SEL_MASK,
    FIELD_PREP(MESON_SAR_ADC_DELAY_INPUT_DLY_SEL_MASK,
    1));
//
// set up the input channel muxes in MESON_SAR_ADC_CHAN_10_SW
// (0 = SAR_ADC_CH0, 1 = SAR_ADC_CH1)
//
    regval = FIELD_PREP(MESON_SAR_ADC_CHAN_10_SW_CHAN0_MUX_SEL_MASK, 0);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN0_MUX_SEL_MASK,
    regval);
    regval = FIELD_PREP(MESON_SAR_ADC_CHAN_10_SW_CHAN1_MUX_SEL_MASK, 1);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN1_MUX_SEL_MASK,
    regval);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN0_XP_DRIVE_SW);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN0_YP_DRIVE_SW);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN1_XP_DRIVE_SW);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_CHAN_10_SW,
    MESON_SAR_ADC_CHAN_10_SW_CHAN1_YP_DRIVE_SW);
//
// set up the input channel muxes in MESON_SAR_ADC_AUX_SW
// (2 = SAR_ADC_CH2, 3 = SAR_ADC_CH3, ...) and enable
// MESON_SAR_ADC_AUX_SW_YP_DRIVE_SW and
// MESON_SAR_ADC_AUX_SW_XP_DRIVE_SW like the vendor driver.
//
    regval = 0;
    for (i = 2; i <= 7; i++)
    regval |= i << MESON_SAR_ADC_AUX_SW_MUX_SEL_CHAN_SHIFT(i);
    regval |= MESON_SAR_ADC_AUX_SW_YP_DRIVE_SW;
    regval |= MESON_SAR_ADC_AUX_SW_XP_DRIVE_SW;
    regmap_write(priv.regmap, MESON_SAR_ADC_AUX_SW, regval);
    if (priv.temperature_sensor_calibrated) {
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_REVE1);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_REVE0);
//
// set bits [3:0] of the TSC (temperature sensor coefficient)
// to get the correct values when reading the temperature.
//
    regval = FIELD_PREP(MESON_SAR_ADC_DELTA_10_TS_C_MASK,
    priv.temperature_sensor_coefficient);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_C_MASK, regval);
    if (priv.param.temperature_trimming_bits == 5) {
    if (priv.temperature_sensor_coefficient & BIT(4))
    regval = MESON_HHI_DPLL_TOP_0_TSC_BIT4;
    else
    regval = 0;
//
// bit [4] (the 5th bit when starting to count at 1)
// of the TSC is located in the HHI register area.
//
    regmap_update_bits(priv.tsc_regmap,
    MESON_HHI_DPLL_TOP_0,
    MESON_HHI_DPLL_TOP_0_TSC_BIT4,
    regval);
    }
    } else {
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_REVE1);
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_REVE0);
    }
    regval = FIELD_PREP(MESON_SAR_ADC_REG3_CTRL_CONT_RING_COUNTER_EN,
    priv.param.disable_ring_counter);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_CTRL_CONT_RING_COUNTER_EN,
    regval);
    if (priv.param.regmap_config.max_register >= MESON_SAR_ADC_REG11) {
    regval = FIELD_PREP(MESON_SAR_ADC_REG11_EOC, priv.param.adc_eoc);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG11,
    MESON_SAR_ADC_REG11_EOC, regval);
    if (priv.param.has_vref_select) {
    regval = FIELD_PREP(MESON_SAR_ADC_REG11_VREF_SEL,
    priv.param.vref_select);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG11,
    MESON_SAR_ADC_REG11_VREF_SEL, regval);
    }
    regval = FIELD_PREP(MESON_SAR_ADC_REG11_VREF_VOLTAGE,
    priv.param.vref_voltage);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG11,
    MESON_SAR_ADC_REG11_VREF_VOLTAGE, regval);
    regval = FIELD_PREP(MESON_SAR_ADC_REG11_CMV_SEL,
    priv.param.cmv_select);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG11,
    MESON_SAR_ADC_REG11_CMV_SEL, regval);
    if (priv.param.enable_mpll_clock_workaround) {
    dev_warn(dev,
    "Enabling unknown bits to make the MPLL clocks work. This may change so always update dtbs and kernel together\n");
    regmap_write(priv.regmap, MESON_SAR_ADC_REG12,
    MESON_SAR_ADC_REG12_MPLL0_UNKNOWN |
    MESON_SAR_ADC_REG12_MPLL1_UNKNOWN |
    MESON_SAR_ADC_REG12_MPLL2_UNKNOWN);
    }
    }
    ret = clk_set_parent(priv.adc_sel_clk, priv.clkin);
    if (ret)
    return dev_err_probe(dev, ret, "failed to set adc parent to clkin\n");
    ret = clk_set_rate(priv.adc_clk, priv.param.clock_rate);
    if (ret)
    return dev_err_probe(dev, ret, "failed to set adc clock rate\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_set_bandgap(indio_dev: *mut iio_dev, on_off: bool) {
    static void meson_sar_adc_set_bandgap(struct iio_dev *indio_dev, bool on_off)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    if (priv.param.regmap_config.max_register >= MESON_SAR_ADC_REG11)
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG11,
    MESON_SAR_ADC_REG11_BANDGAP_EN,
    on_off ? MESON_SAR_ADC_REG11_BANDGAP_EN : 0);
    else
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_DELTA_10,
    MESON_SAR_ADC_DELTA_10_TS_VBG_EN,
    on_off ? MESON_SAR_ADC_DELTA_10_TS_VBG_EN : 0);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_hw_enable(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_hw_enable(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    int ret;
    u32 regval;
    ret = meson_sar_adc_lock(indio_dev);
    if (ret) {
    dev_err(dev, "failed to lock adc\n");
    goto err_lock;
    }
    ret = regulator_enable(priv.vref);
    if (ret < 0) {
    dev_err(dev, "failed to enable vref regulator\n");
    goto err_vref;
    }
    regval = FIELD_PREP(MESON_SAR_ADC_REG0_FIFO_CNT_IRQ_MASK, 1);
    regmap_update_bits(priv.regmap, MESON_SAR_ADC_REG0,
    MESON_SAR_ADC_REG0_FIFO_CNT_IRQ_MASK, regval);
    meson_sar_adc_set_bandgap(indio_dev, true);
    regmap_set_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_ADC_EN);
    udelay(5);
    ret = clk_prepare_enable(priv.adc_clk);
    if (ret) {
    dev_err(dev, "failed to enable adc clk\n");
    goto err_adc_clk;
    }
    meson_sar_adc_unlock(indio_dev);
    return 0;
    err_adc_clk:
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_ADC_EN);
    meson_sar_adc_set_bandgap(indio_dev, false);
    regulator_disable(priv.vref);
    err_vref:
    meson_sar_adc_unlock(indio_dev);
    err_lock:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_hw_disable(indio_dev: *mut iio_dev) {
    static void meson_sar_adc_hw_disable(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int ret;
//
// If taking the lock fails we have to assume that BL30 is broken. The
// best we can do then is to release the resources anyhow.
//
    ret = meson_sar_adc_lock(indio_dev);
    if (ret)
    dev_err(indio_dev.dev.parent, "Failed to lock ADC (%pE)\n", ERR_PTR(ret));
    clk_disable_unprepare(priv.adc_clk);
    regmap_clear_bits(priv.regmap, MESON_SAR_ADC_REG3,
    MESON_SAR_ADC_REG3_ADC_EN);
    meson_sar_adc_set_bandgap(indio_dev, false);
    regulator_disable(priv.vref);
    if (!ret)
    meson_sar_adc_unlock(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_sar_adc_irq(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    unsigned int cnt, threshold;
    u32 regval;
    regmap_read(priv.regmap, MESON_SAR_ADC_REG0, &regval);
    cnt = FIELD_GET(MESON_SAR_ADC_REG0_FIFO_COUNT_MASK, regval);
    threshold = FIELD_GET(MESON_SAR_ADC_REG0_FIFO_CNT_IRQ_MASK, regval);
    if (cnt < threshold)
    return IRQ_NONE;
    complete(&priv.done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_calib(indio_dev: *mut iio_dev) -> c_int {
    static int meson_sar_adc_calib(struct iio_dev *indio_dev)
    {
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int ret, nominal0, nominal1, value0, value1;
// use points 25% and 75% for calibration
    nominal0 = (1 << priv.param.resolution) / 4;
    nominal1 = (1 << priv.param.resolution) * 3 / 4;
    meson_sar_adc_set_chan7_mux(indio_dev, CHAN7_MUX_VDD_DIV4);
    usleep_range(10, 20);
    ret = meson_sar_adc_get_sample(indio_dev,
    find_channel_by_num(indio_dev,
    NUM_MUX_1_VDD_DIV4),
    MEAN_AVERAGING, EIGHT_SAMPLES, &value0);
    if (ret < 0)
    goto out;
    meson_sar_adc_set_chan7_mux(indio_dev, CHAN7_MUX_VDD_MUL3_DIV4);
    usleep_range(10, 20);
    ret = meson_sar_adc_get_sample(indio_dev,
    find_channel_by_num(indio_dev,
    NUM_MUX_3_VDD_MUL3_DIV4),
    MEAN_AVERAGING, EIGHT_SAMPLES, &value1);
    if (ret < 0)
    goto out;
    if (value1 <= value0) {
    ret = -EINVAL;
    goto out;
    }
    priv.calibscale = div_s64((nominal1 - nominal0) * (s64)MILLION,
    value1 - value0);
    priv.calibbias = nominal0 - div_s64((s64)value0 * priv.calibscale,
    MILLION);
    ret = 0;
    out:
    meson_sar_adc_set_chan7_mux(indio_dev, CHAN7_MUX_CH7_INPUT);
    return ret;
    }
    static int read_label(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    char *label)
    {
    if (chan.type == IIO_TEMP)
    return sysfs_emit(label, "temp-sensor\n");
    if (chan.type == IIO_VOLTAGE && chan.channel >= NUM_MUX_0_VSS)
    return sysfs_emit(label, "%s\n",
    chan7_mux_names[chan.channel - NUM_MUX_0_VSS]);
    if (chan.type == IIO_VOLTAGE)
    return sysfs_emit(label, "channel-%d\n", chan.channel);
    return 0;
    }
    static const struct iio_info meson_sar_adc_iio_info = {
    .read_raw = meson_sar_adc_iio_info_read_raw,
    .read_label = read_label,
    };
    static const struct meson_sar_adc_param meson_sar_adc_meson8_param = {
    .has_bl30_integration = false,
    .clock_rate = 1150000,
    .regmap_config = &meson_sar_adc_regmap_config_meson8,
    .resolution = 10,
    .temperature_trimming_bits = 4,
    .temperature_multiplier = 18 * 10000,
    .temperature_divider = 1024 * 10 * 85,
    };
    static const struct meson_sar_adc_param meson_sar_adc_meson8b_param = {
    .has_bl30_integration = false,
    .clock_rate = 1150000,
    .regmap_config = &meson_sar_adc_regmap_config_meson8,
    .resolution = 10,
    .temperature_trimming_bits = 5,
    .temperature_multiplier = 10,
    .temperature_divider = 32,
    };
    static const struct meson_sar_adc_param meson_sar_adc_gxbb_param = {
    .has_bl30_integration = true,
    .clock_rate = 1200000,
    .regmap_config = &meson_sar_adc_regmap_config_gxbb,
    .resolution = 10,
    .vref_voltage = 1,
    .cmv_select = 1,
    };
    static const struct meson_sar_adc_param meson_sar_adc_gxl_param = {
    .has_bl30_integration = true,
    .clock_rate = 1200000,
    .regmap_config = &meson_sar_adc_regmap_config_gxbb,
    .resolution = 12,
    .disable_ring_counter = 1,
    .vref_voltage = 1,
    .cmv_select = 1,
    };
    static const struct meson_sar_adc_param meson_sar_adc_gxlx_param = {
    .has_bl30_integration = true,
    .clock_rate = 1200000,
    .regmap_config = &meson_sar_adc_regmap_config_gxbb,
    .resolution = 12,
    .disable_ring_counter = 1,
    .vref_voltage = 1,
    .cmv_select = true,
    .enable_mpll_clock_workaround = true,
    };
    static const struct meson_sar_adc_param meson_sar_adc_axg_param = {
    .has_bl30_integration = true,
    .clock_rate = 1200000,
    .regmap_config = &meson_sar_adc_regmap_config_gxbb,
    .resolution = 12,
    .disable_ring_counter = 1,
    .vref_voltage = 1,
    .has_vref_select = true,
    .vref_select = VREF_VDDA,
    .cmv_select = 1,
    };
    static const struct meson_sar_adc_param meson_sar_adc_g12a_param = {
    .has_bl30_integration = false,
    .clock_rate = 1200000,
    .regmap_config = &meson_sar_adc_regmap_config_gxbb,
    .resolution = 12,
    .disable_ring_counter = 1,
    .adc_eoc = 1,
    .has_vref_select = true,
    .vref_select = VREF_VDDA,
    };
    static const struct meson_sar_adc_data meson_sar_adc_meson8_data = {
    .param = &meson_sar_adc_meson8_param,
    .name = "meson-meson8-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_meson8b_data = {
    .param = &meson_sar_adc_meson8b_param,
    .name = "meson-meson8b-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_meson8m2_data = {
    .param = &meson_sar_adc_meson8b_param,
    .name = "meson-meson8m2-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_gxbb_data = {
    .param = &meson_sar_adc_gxbb_param,
    .name = "meson-gxbb-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_gxl_data = {
    .param = &meson_sar_adc_gxl_param,
    .name = "meson-gxl-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_gxlx_data = {
    .param = &meson_sar_adc_gxlx_param,
    .name = "meson-gxlx-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_gxm_data = {
    .param = &meson_sar_adc_gxl_param,
    .name = "meson-gxm-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_axg_data = {
    .param = &meson_sar_adc_axg_param,
    .name = "meson-axg-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_g12a_data = {
    .param = &meson_sar_adc_g12a_param,
    .name = "meson-g12a-saradc",
    };
    static const struct meson_sar_adc_data meson_sar_adc_s4_data = {
    .param = &meson_sar_adc_g12a_param,
    .name = "meson-s4-saradc",
    };
    static const struct of_device_id meson_sar_adc_of_match[] = {
    {
    .compatible = "amlogic,meson8-saradc",
    .data = &meson_sar_adc_meson8_data,
    }, {
    .compatible = "amlogic,meson8b-saradc",
    .data = &meson_sar_adc_meson8b_data,
    }, {
    .compatible = "amlogic,meson8m2-saradc",
    .data = &meson_sar_adc_meson8m2_data,
    }, {
    .compatible = "amlogic,meson-gxbb-saradc",
    .data = &meson_sar_adc_gxbb_data,
    }, {
    .compatible = "amlogic,meson-gxl-saradc",
    .data = &meson_sar_adc_gxl_data,
    }, {
    .compatible = "amlogic,meson-gxlx-saradc",
    .data = &meson_sar_adc_gxlx_data,
    }, {
    .compatible = "amlogic,meson-gxm-saradc",
    .data = &meson_sar_adc_gxm_data,
    }, {
    .compatible = "amlogic,meson-axg-saradc",
    .data = &meson_sar_adc_axg_data,
    }, {
    .compatible = "amlogic,meson-g12a-saradc",
    .data = &meson_sar_adc_g12a_data,
    }, {
    .compatible = "amlogic,meson-s4-saradc",
    .data = &meson_sar_adc_s4_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, meson_sar_adc_of_match);
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_probe(pdev: *mut platform_device) -> c_int {
    static int meson_sar_adc_probe(struct platform_device *pdev)
    {
    const struct meson_sar_adc_data *match_data;
    struct meson_sar_adc_priv *priv;
    struct device *dev = &pdev.dev;
    struct iio_dev *indio_dev;
    void __iomem *base;
    int irq, ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*priv));
    if (!indio_dev)
    return -ENOMEM;
    priv = iio_priv(indio_dev);
    init_completion(&priv.done);
    match_data = of_device_get_match_data(dev);
    if (!match_data)
    return dev_err_probe(dev, -ENODEV, "failed to get match data\n");
    priv.param = match_data.param;
    indio_dev.name = match_data.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &meson_sar_adc_iio_info;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.regmap = devm_regmap_init_mmio(dev, base, priv.param.regmap_config);
    if (IS_ERR(priv.regmap))
    return dev_err_probe(dev, PTR_ERR(priv.regmap), "failed to init regmap\n");
    irq = irq_of_parse_and_map(dev.of_node, 0);
    if (!irq)
    return dev_err_probe(dev, -EINVAL, "failed to get irq\n");
    ret = devm_request_irq(dev, irq, meson_sar_adc_irq, IRQF_SHARED, dev_name(dev), indio_dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request irq\n");
    priv.clkin = devm_clk_get(dev, "clkin");
    if (IS_ERR(priv.clkin))
    return dev_err_probe(dev, PTR_ERR(priv.clkin), "failed to get clkin\n");
    priv.core_clk = devm_clk_get_enabled(dev, "core");
    if (IS_ERR(priv.core_clk))
    return dev_err_probe(dev, PTR_ERR(priv.core_clk), "failed to get core clk\n");
    priv.adc_clk = devm_clk_get_optional(dev, "adc_clk");
    if (IS_ERR(priv.adc_clk))
    return dev_err_probe(dev, PTR_ERR(priv.adc_clk), "failed to get adc clk\n");
    priv.adc_sel_clk = devm_clk_get_optional(dev, "adc_sel");
    if (IS_ERR(priv.adc_sel_clk))
    return dev_err_probe(dev, PTR_ERR(priv.adc_sel_clk), "failed to get adc_sel clk\n");
// on pre-GXBB SoCs the SAR ADC itself provides the ADC clock:
    if (!priv.adc_clk) {
    ret = meson_sar_adc_clk_init(indio_dev, base);
    if (ret)
    return dev_err_probe(dev, ret, "failed to init internal clk\n");
    }
    priv.vref = devm_regulator_get(dev, "vref");
    if (IS_ERR(priv.vref))
    return dev_err_probe(dev, PTR_ERR(priv.vref), "failed to get vref regulator\n");
    priv.calibscale = MILLION;
    if (priv.param.temperature_trimming_bits) {
    ret = meson_sar_adc_temp_sensor_init(indio_dev);
    if (ret)
    return ret;
    }
    if (priv.temperature_sensor_calibrated) {
    indio_dev.channels = meson_sar_adc_and_temp_iio_channels;
    indio_dev.num_channels =
    ARRAY_SIZE(meson_sar_adc_and_temp_iio_channels);
    } else {
    indio_dev.channels = meson_sar_adc_iio_channels;
    indio_dev.num_channels =
    ARRAY_SIZE(meson_sar_adc_iio_channels);
    }
    ret = meson_sar_adc_init(indio_dev);
    if (ret)
    goto err;
    mutex_init(&priv.lock);
    ret = meson_sar_adc_hw_enable(indio_dev);
    if (ret)
    goto err;
    ret = meson_sar_adc_calib(indio_dev);
    if (ret)
    dev_warn(dev, "calibration failed\n");
    platform_set_drvdata(pdev, indio_dev);
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err_probe(dev, ret, "failed to register iio device\n");
    goto err_hw;
    }
    return 0;
    err_hw:
    meson_sar_adc_hw_disable(indio_dev);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_remove(pdev: *mut platform_device) {
    static void meson_sar_adc_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    iio_device_unregister(indio_dev);
    meson_sar_adc_hw_disable(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_suspend(dev: *mut device) -> c_int {
    static int meson_sar_adc_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    meson_sar_adc_hw_disable(indio_dev);
    clk_disable_unprepare(priv.core_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_sar_adc_resume(dev: *mut device) -> c_int {
    static int meson_sar_adc_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct meson_sar_adc_priv *priv = iio_priv(indio_dev);
    int ret;
    ret = clk_prepare_enable(priv.core_clk);
    if (ret) {
    dev_err(dev, "failed to enable core clk\n");
    return ret;
    }
    return meson_sar_adc_hw_enable(indio_dev);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(meson_sar_adc_pm_ops,
    meson_sar_adc_suspend, meson_sar_adc_resume);
    static struct platform_driver meson_sar_adc_driver = {
    .probe		= meson_sar_adc_probe,
    .remove		= meson_sar_adc_remove,
    .driver		= {
    .name	= "meson-saradc",
    .of_match_table = meson_sar_adc_of_match,
    .pm = pm_sleep_ptr(&meson_sar_adc_pm_ops),
    },
    };
    module_platform_driver(meson_sar_adc_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Amlogic Meson SAR ADC driver");
    MODULE_LICENSE("GPL v2");
