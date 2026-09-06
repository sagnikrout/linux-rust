//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qcom/qcom-spmi-adc-tm5.c
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
// Copyright (c) 2020 Linaro Limited
//
// Based on original driver:
// Copyright (c) 2012-2020, The Linux Foundation. All rights reserved.
//
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// Thermal monitoring block consists of 8 (ADC_TM5_NUM_CHANNELS) channels. Each
// channel is programmed to use one of ADC channels for voltage comparison.
// Voltages are programmed using ADC codes, so we have to convert temp to
// voltage and then to ADC code value.
//
// Configuration of TM channels must match configuration of corresponding ADC
// channels.
//
pub const ADC5_MAX_CHANNEL: c_uint = 0xc0;
pub const ADC_TM5_NUM_CHANNELS: c_int = 8;
pub const ADC_TM5_STATUS_LOW: c_uint = 0x0a;
pub const ADC_TM5_STATUS_HIGH: c_uint = 0x0b;
pub const ADC_TM5_NUM_BTM: c_uint = 0x0f;
pub const ADC_TM5_ADC_DIG_PARAM: c_uint = 0x42;

pub const ADC_TM5_MEAS_INTERVAL_CTL2_MASK: c_uint = 0xf0;

pub const ADC_TM5_MEAS_INTERVAL_CTL3_MASK: c_uint = 0xf;

pub const ADC_TM_EN_CTL1: c_uint = 0x46;

pub const ADC_TM_CONV_REQ: c_uint = 0x47;

pub const ADC_TM5_M_CHAN_BASE: c_uint = 0x60;

pub const ADC_TM5_M_CTL_HW_SETTLE_DELAY_MASK: c_uint = 0xf;
pub const ADC_TM5_M_CTL_CAL_SEL_MASK: c_uint = 0x30;
pub const ADC_TM5_M_CTL_CAL_VAL: c_uint = 0x40;

pub const ADC_TM_GEN2_STATUS1: c_uint = 0x08;
pub const ADC_TM_GEN2_STATUS_LOW_SET: c_uint = 0x09;
pub const ADC_TM_GEN2_STATUS_LOW_CLR: c_uint = 0x0a;
pub const ADC_TM_GEN2_STATUS_HIGH_SET: c_uint = 0x0b;
pub const ADC_TM_GEN2_STATUS_HIGH_CLR: c_uint = 0x0c;
pub const ADC_TM_GEN2_CFG_HS_SET: c_uint = 0x0d;

pub const ADC_TM_GEN2_CFG_HS_CLR: c_uint = 0x0e;
pub const ADC_TM_GEN2_SID: c_uint = 0x40;
pub const ADC_TM_GEN2_CH_CTL: c_uint = 0x41;

pub const ADC_TM_GEN2_ADC_DIG_PARAM: c_uint = 0x42;

pub const ADC_TM_GEN2_FAST_AVG_CTL: c_uint = 0x43;

pub const ADC_TM_GEN2_ADC_CH_SEL_CTL: c_uint = 0x44;
pub const ADC_TM_GEN2_DELAY_CTL: c_uint = 0x45;

pub const ADC_TM_GEN2_EN_CTL1: c_uint = 0x46;

pub const ADC_TM_GEN2_CONV_REQ: c_uint = 0x47;

pub const ADC_TM_GEN2_LOW_THR0: c_uint = 0x49;
pub const ADC_TM_GEN2_LOW_THR1: c_uint = 0x4a;
pub const ADC_TM_GEN2_HIGH_THR0: c_uint = 0x4b;
pub const ADC_TM_GEN2_HIGH_THR1: c_uint = 0x4c;

pub const ADC_TM_GEN2_MEAS_IRQ_EN: c_uint = 0x4d;

pub const ADC_TM_GEN2_MEAS_INT_LSB: c_uint = 0x50;
pub const ADC_TM_GEN2_MEAS_INT_MSB: c_uint = 0x51;
pub const ADC_TM_GEN2_MEAS_INT_MODE: c_uint = 0x52;

pub const ADC_TM_GEN2_DATA_SHIFT: c_int = 8;
    enum adc5_timer_select {
    ADC5_TIMER_SEL_1 = 0,
    ADC5_TIMER_SEL_2,
    ADC5_TIMER_SEL_3,
    ADC5_TIMER_SEL_NONE,
    };
    enum adc5_gen {
    ADC_TM5,
    ADC_TM_HC,
    ADC_TM5_GEN2,
    ADC_TM5_MAX
    };
    enum adc_tm5_cal_method {
    ADC_TM5_NO_CAL = 0,
    ADC_TM5_RATIOMETRIC_CAL,
    ADC_TM5_ABSOLUTE_CAL
    };
    enum adc_tm_gen2_time_select {
    MEAS_INT_50MS = 0,
    MEAS_INT_100MS,
    MEAS_INT_1S,
    MEAS_INT_SET,
    MEAS_INT_NONE,
    };
    struct adc_tm5_chip;
    struct adc_tm5_channel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_tm5_data {
    pub full_scale_code_volt: u32,
    pub decimation: *mut c_uint,
    pub hw_settle: *mut c_uint,
    pub channel): *mut *mut int (disable_channel)(struct adc_tm5_channel,
    pub high): *mut *mut *mut int (configure)(struct adc_tm5_channel channel, int low, int,
    pub data): *mut *mut irqreturn_t (isr)(int irq, void,
    pub chip): *mut *mut int (init)(struct adc_tm5_chip,
    pub irq_name: *mut c_char,
    pub gen: c_int,
}

//
// struct adc_tm5_channel - ADC Thermal Monitoring channel data.
// @channel: channel number.
// @adc_channel: corresponding ADC channel number.
// @cal_method: calibration method.
// @prescale: channel scaling performed on the input signal.
// @hw_settle_time: the time between AMUX being configured and the
// start of conversion.
// @decimation: sampling rate supported for the channel.
// @avg_samples: ability to provide single result from the ADC
// that is an average of multiple measurements.
// @high_thr_en: channel upper voltage threshold enable state.
// @low_thr_en: channel lower voltage threshold enable state.
// @meas_en: recurring measurement enable state
// @iio: IIO channel instance used by this channel.
// @chip: ADC TM chip instance.
// @tzd: thermal zone device used by this channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_tm5_channel {
    pub channel: c_uint,
    pub adc_channel: c_uint,
    pub cal_method: enum adc_tm5_cal_method,
    pub prescale: c_uint,
    pub hw_settle_time: c_uint,
    pub /: *mut *mut unsigned int decimation; / For Gen2 ADC_TM,
    pub /: *mut *mut unsigned int avg_samples; / For Gen2 ADC_TM,
    pub /: *mut *mut bool high_thr_en; / For Gen2 ADC_TM,
    pub /: *mut *mut bool low_thr_en; / For Gen2 ADC_TM,
    pub /: *mut *mut bool meas_en; / For Gen2 ADC_TM,
    pub iio: *mut iio_channel,
    pub chip: *mut adc_tm5_chip,
    pub tzd: *mut thermal_zone_device,
}

//
// struct adc_tm5_chip - ADC Thermal Monitoring properties
// @regmap: SPMI ADC5 Thermal Monitoring  peripheral register map field.
// @dev: SPMI ADC5 device.
// @data: software configuration data.
// @channels: array of ADC TM channel data.
// @nchannels: amount of channels defined/allocated
// @decimation: sampling rate supported for the channel.
// Applies to all channels, used only on Gen1 ADC_TM.
// @avg_samples: ability to provide single result from the ADC
// that is an average of multiple measurements. Applies to all
// channels, used only on Gen1 ADC_TM.
// @base: base address of TM registers.
// @adc_mutex_lock: ADC_TM mutex lock, used only on Gen2 ADC_TM.
// It is used to ensure only one ADC channel configuration
// is done at a time using the shared set of configuration
// registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_tm5_chip {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub data: *const adc_tm5_data,
    pub channels: *mut adc_tm5_channel,
    pub nchannels: c_uint,
    pub decimation: c_uint,
    pub avg_samples: c_uint,
    pub base: u16,
    pub adc_mutex_lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn adc_tm5_read(adc_tm: *mut adc_tm5_chip, offset: u16, data: *mut u8, len: c_int) -> c_int {
    static int adc_tm5_read(struct adc_tm5_chip *adc_tm, u16 offset, u8 *data, int len)
    {
    return regmap_bulk_read(adc_tm.regmap, adc_tm.base + offset, data, len);
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_write(adc_tm: *mut adc_tm5_chip, offset: u16, data: *mut u8, len: c_int) -> c_int {
    static int adc_tm5_write(struct adc_tm5_chip *adc_tm, u16 offset, u8 *data, int len)
    {
    return regmap_bulk_write(adc_tm.regmap, adc_tm.base + offset, data, len);
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_reg_update(adc_tm: *mut adc_tm5_chip, offset: u16, mask: u8, val: u8) -> c_int {
    static int adc_tm5_reg_update(struct adc_tm5_chip *adc_tm, u16 offset, u8 mask, u8 val)
    {
    return regmap_write_bits(adc_tm.regmap, adc_tm.base + offset, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t adc_tm5_isr(int irq, void *data)
    {
    struct adc_tm5_chip *chip = data;
    u8 status_low, status_high, ctl;
    int ret, i;
    ret = adc_tm5_read(chip, ADC_TM5_STATUS_LOW, &status_low, sizeof(status_low));
    if (unlikely(ret)) {
    dev_err(chip.dev, "read status low failed: %d\n", ret);
    return IRQ_HANDLED;
    }
    ret = adc_tm5_read(chip, ADC_TM5_STATUS_HIGH, &status_high, sizeof(status_high));
    if (unlikely(ret)) {
    dev_err(chip.dev, "read status high failed: %d\n", ret);
    return IRQ_HANDLED;
    }
    for (i = 0; i < chip.nchannels; i++) {
    let mut upper_set: bool = false, lower_set = false;
    let mut ch: c_uint = chip.channels[i].channel;
// No TZD, we warned at the boot time
    if (!chip.channels[i].tzd)
    continue;
    ret = adc_tm5_read(chip, ADC_TM5_M_EN(ch), &ctl, sizeof(ctl));
    if (unlikely(ret)) {
    dev_err(chip.dev, "ctl read failed: %d, channel %d\n", ret, i);
    continue;
    }
    if (!(ctl & ADC_TM5_M_MEAS_EN))
    continue;
    lower_set = (status_low & BIT(ch)) &&
    (ctl & ADC_TM5_M_LOW_THR_INT_EN);
    upper_set = (status_high & BIT(ch)) &&
    (ctl & ADC_TM5_M_HIGH_THR_INT_EN);
    if (upper_set || lower_set)
    thermal_zone_device_update(chip.channels[i].tzd,
    THERMAL_EVENT_UNSPECIFIED);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_gen2_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t adc_tm5_gen2_isr(int irq, void *data)
    {
    struct adc_tm5_chip *chip = data;
    u8 status_low, status_high;
    int ret, i;
    ret = adc_tm5_read(chip, ADC_TM_GEN2_STATUS_LOW_CLR, &status_low, sizeof(status_low));
    if (ret) {
    dev_err(chip.dev, "read status_low failed: %d\n", ret);
    return IRQ_HANDLED;
    }
    ret = adc_tm5_read(chip, ADC_TM_GEN2_STATUS_HIGH_CLR, &status_high, sizeof(status_high));
    if (ret) {
    dev_err(chip.dev, "read status_high failed: %d\n", ret);
    return IRQ_HANDLED;
    }
    ret = adc_tm5_write(chip, ADC_TM_GEN2_STATUS_LOW_CLR, &status_low, sizeof(status_low));
    if (ret < 0) {
    dev_err(chip.dev, "clear status low failed with %d\n", ret);
    return IRQ_HANDLED;
    }
    ret = adc_tm5_write(chip, ADC_TM_GEN2_STATUS_HIGH_CLR, &status_high, sizeof(status_high));
    if (ret < 0) {
    dev_err(chip.dev, "clear status high failed with %d\n", ret);
    return IRQ_HANDLED;
    }
    for (i = 0; i < chip.nchannels; i++) {
    let mut upper_set: bool = false, lower_set = false;
    let mut ch: c_uint = chip.channels[i].channel;
// No TZD, we warned at the boot time
    if (!chip.channels[i].tzd)
    continue;
    if (!chip.channels[i].meas_en)
    continue;
    lower_set = (status_low & BIT(ch)) &&
    (chip.channels[i].low_thr_en);
    upper_set = (status_high & BIT(ch)) &&
    (chip.channels[i].high_thr_en);
    if (upper_set || lower_set)
    thermal_zone_device_update(chip.channels[i].tzd,
    THERMAL_EVENT_UNSPECIFIED);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int adc_tm5_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct adc_tm5_channel *channel = thermal_zone_device_priv(tz);
    int ret;
    if (!channel || !channel.iio)
    return -EINVAL;
    ret = iio_read_channel_processed(channel.iio, temp);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_disable_channel(channel: *mut adc_tm5_channel) -> c_int {
    static int adc_tm5_disable_channel(struct adc_tm5_channel *channel)
    {
    struct adc_tm5_chip *chip = channel.chip;
    let mut reg: c_uint = ADC_TM5_M_EN(channel.channel);
    return adc_tm5_reg_update(chip, reg,
    ADC_TM5_M_MEAS_EN |
    ADC_TM5_M_HIGH_THR_INT_EN |
    ADC_TM5_M_LOW_THR_INT_EN,
    0);
    }
pub const ADC_TM_GEN2_POLL_DELAY_MIN_US: c_int = 100;
pub const ADC_TM_GEN2_POLL_DELAY_MAX_US: c_int = 110;
pub const ADC_TM_GEN2_POLL_RETRY_COUNT: c_int = 3;
#[no_mangle]
unsafe extern "C" fn adc_tm5_gen2_conv_req(chip: *mut adc_tm5_chip) -> i32 {
    static int32_t adc_tm5_gen2_conv_req(struct adc_tm5_chip *chip)
    {
    int ret;
    u8 data;
    unsigned int count;
    data = ADC_TM_GEN2_EN;
    ret = adc_tm5_write(chip, ADC_TM_GEN2_EN_CTL1, &data, 1);
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm enable failed with %d\n", ret);
    return ret;
    }
    data = ADC_TM_GEN2_CFG_HS_FLAG;
    ret = adc_tm5_write(chip, ADC_TM_GEN2_CFG_HS_SET, &data, 1);
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm handshake failed with %d\n", ret);
    return ret;
    }
    data = ADC_TM_GEN2_CONV_REQ_EN;
    ret = adc_tm5_write(chip, ADC_TM_GEN2_CONV_REQ, &data, 1);
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm request conversion failed with %d\n", ret);
    return ret;
    }
//
// SW sets a handshake bit and waits for PBS to clear it
// before the next conversion request can be queued.
//
    for (count = 0; count < ADC_TM_GEN2_POLL_RETRY_COUNT; count++) {
    ret = adc_tm5_read(chip, ADC_TM_GEN2_CFG_HS_SET, &data, sizeof(data));
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm read failed with %d\n", ret);
    return ret;
    }
    if (!(data & ADC_TM_GEN2_CFG_HS_FLAG))
    return ret;
    usleep_range(ADC_TM_GEN2_POLL_DELAY_MIN_US,
    ADC_TM_GEN2_POLL_DELAY_MAX_US);
    }
    dev_err(chip.dev, "adc-tm conversion request handshake timed out\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_gen2_disable_channel(channel: *mut adc_tm5_channel) -> c_int {
    static int adc_tm5_gen2_disable_channel(struct adc_tm5_channel *channel)
    {
    struct adc_tm5_chip *chip = channel.chip;
    int ret;
    u8 val;
    mutex_lock(&chip.adc_mutex_lock);
    channel.meas_en = false;
    channel.high_thr_en = false;
    channel.low_thr_en = false;
    ret = adc_tm5_read(chip, ADC_TM_GEN2_CH_CTL, &val, sizeof(val));
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm block read failed with %d\n", ret);
    goto disable_fail;
    }
    val &= ~ADC_TM_GEN2_TM_CH_SEL;
    val |= FIELD_PREP(ADC_TM_GEN2_TM_CH_SEL, channel.channel);
    ret = adc_tm5_write(chip, ADC_TM_GEN2_CH_CTL, &val, 1);
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm channel disable failed with %d\n", ret);
    goto disable_fail;
    }
    val = 0;
    ret = adc_tm5_write(chip, ADC_TM_GEN2_MEAS_IRQ_EN, &val, 1);
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm interrupt disable failed with %d\n", ret);
    goto disable_fail;
    }
    ret = adc_tm5_gen2_conv_req(channel.chip);
    if (ret < 0)
    dev_err(chip.dev, "adc-tm channel configure failed with %d\n", ret);
    disable_fail:
    mutex_unlock(&chip.adc_mutex_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_enable(chip: *mut adc_tm5_chip) -> c_int {
    static int adc_tm5_enable(struct adc_tm5_chip *chip)
    {
    int ret;
    u8 data;
    data = ADC_TM_EN;
    ret = adc_tm5_write(chip, ADC_TM_EN_CTL1, &data, sizeof(data));
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm enable failed\n");
    return ret;
    }
    data = ADC_TM_CONV_REQ_EN;
    ret = adc_tm5_write(chip, ADC_TM_CONV_REQ, &data, sizeof(data));
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm request conversion failed\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_configure(channel: *mut adc_tm5_channel, low: c_int, high: c_int) -> c_int {
    static int adc_tm5_configure(struct adc_tm5_channel *channel, int low, int high)
    {
    struct adc_tm5_chip *chip = channel.chip;
    u8 buf[8];
    let mut reg: u16 = ADC_TM5_M_ADC_CH_SEL_CTL(channel.channel);
    int ret;
    ret = adc_tm5_read(chip, reg, buf, sizeof(buf));
    if (ret) {
    dev_err(chip.dev, "channel %d params read failed: %d\n", channel.channel, ret);
    return ret;
    }
    buf[0] = channel.adc_channel;
// High temperature corresponds to low voltage threshold
    if (high != INT_MAX) {
    u16 adc_code = qcom_adc_tm5_temp_volt_scale(channel.prescale,
    chip.data.full_scale_code_volt, high);
    put_unaligned_le16(adc_code, &buf[1]);
    buf[7] |= ADC_TM5_M_LOW_THR_INT_EN;
    } else {
    buf[7] &= ~ADC_TM5_M_LOW_THR_INT_EN;
    }
// Low temperature corresponds to high voltage threshold
    if (low != -INT_MAX) {
    u16 adc_code = qcom_adc_tm5_temp_volt_scale(channel.prescale,
    chip.data.full_scale_code_volt, low);
    put_unaligned_le16(adc_code, &buf[3]);
    buf[7] |= ADC_TM5_M_HIGH_THR_INT_EN;
    } else {
    buf[7] &= ~ADC_TM5_M_HIGH_THR_INT_EN;
    }
    buf[5] = ADC5_TIMER_SEL_2;
// Set calibration select, hw_settle delay
    buf[6] &= ~ADC_TM5_M_CTL_HW_SETTLE_DELAY_MASK;
    buf[6] |= FIELD_PREP(ADC_TM5_M_CTL_HW_SETTLE_DELAY_MASK, channel.hw_settle_time);
    buf[6] &= ~ADC_TM5_M_CTL_CAL_SEL_MASK;
    buf[6] |= FIELD_PREP(ADC_TM5_M_CTL_CAL_SEL_MASK, channel.cal_method);
    buf[7] |= ADC_TM5_M_MEAS_EN;
    ret = adc_tm5_write(chip, reg, buf, sizeof(buf));
    if (ret) {
    dev_err(chip.dev, "channel %d params write failed: %d\n", channel.channel, ret);
    return ret;
    }
    return adc_tm5_enable(chip);
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_gen2_configure(channel: *mut adc_tm5_channel, low: c_int, high: c_int) -> c_int {
    static int adc_tm5_gen2_configure(struct adc_tm5_channel *channel, int low, int high)
    {
    struct adc_tm5_chip *chip = channel.chip;
    int ret;
    u8 buf[14];
    u16 adc_code;
    mutex_lock(&chip.adc_mutex_lock);
    channel.meas_en = true;
    ret = adc_tm5_read(chip, ADC_TM_GEN2_SID, buf, sizeof(buf));
    if (ret < 0) {
    dev_err(chip.dev, "adc-tm block read failed with %d\n", ret);
    goto config_fail;
    }
// Set SID from virtual channel number
    buf[0] = channel.adc_channel >> 8;
// Set TM channel number used and measurement interval
    buf[1] &= ~ADC_TM_GEN2_TM_CH_SEL;
    buf[1] |= FIELD_PREP(ADC_TM_GEN2_TM_CH_SEL, channel.channel);
    buf[1] &= ~ADC_TM_GEN2_MEAS_INT_SEL;
    buf[1] |= FIELD_PREP(ADC_TM_GEN2_MEAS_INT_SEL, MEAS_INT_1S);
    buf[2] &= ~ADC_TM_GEN2_CTL_DEC_RATIO_MASK;
    buf[2] |= FIELD_PREP(ADC_TM_GEN2_CTL_DEC_RATIO_MASK, channel.decimation);
    buf[2] &= ~ADC_TM_GEN2_CTL_CAL_SEL;
    buf[2] |= FIELD_PREP(ADC_TM_GEN2_CTL_CAL_SEL, channel.cal_method);
    buf[3] = channel.avg_samples | ADC_TM_GEN2_FAST_AVG_EN;
    buf[4] = channel.adc_channel & 0xff;
    buf[5] = channel.hw_settle_time & ADC_TM_GEN2_HW_SETTLE_DELAY;
// High temperature corresponds to low voltage threshold
    if (high != INT_MAX) {
    channel.low_thr_en = true;
    adc_code = qcom_adc_tm5_gen2_temp_res_scale(high);
    put_unaligned_le16(adc_code, &buf[9]);
    } else {
    channel.low_thr_en = false;
    }
// Low temperature corresponds to high voltage threshold
    if (low != -INT_MAX) {
    channel.high_thr_en = true;
    adc_code = qcom_adc_tm5_gen2_temp_res_scale(low);
    put_unaligned_le16(adc_code, &buf[11]);
    } else {
    channel.high_thr_en = false;
    }
    buf[13] = ADC_TM_GEN2_MEAS_EN;
    if (channel.high_thr_en)
    buf[13] |= ADC_TM5_GEN2_HIGH_THR_INT_EN;
    if (channel.low_thr_en)
    buf[13] |= ADC_TM5_GEN2_LOW_THR_INT_EN;
    ret = adc_tm5_write(chip, ADC_TM_GEN2_SID, buf, sizeof(buf));
    if (ret) {
    dev_err(chip.dev, "channel %d params write failed: %d\n", channel.channel, ret);
    goto config_fail;
    }
    ret = adc_tm5_gen2_conv_req(channel.chip);
    if (ret < 0)
    dev_err(chip.dev, "adc-tm channel configure failed with %d\n", ret);
    config_fail:
    mutex_unlock(&chip.adc_mutex_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int) -> c_int {
    static int adc_tm5_set_trips(struct thermal_zone_device *tz, int low, int high)
    {
    struct adc_tm5_channel *channel = thermal_zone_device_priv(tz);
    struct adc_tm5_chip *chip;
    int ret;
    if (!channel)
    return -EINVAL;
    chip = channel.chip;
    dev_dbg(chip.dev, "%d:low(mdegC):%d, high(mdegC):%d\n",
    channel.channel, low, high);
    if (high == INT_MAX && low <= -INT_MAX)
    ret = chip.data.disable_channel(channel);
    else
    ret = chip.data.configure(channel, low, high);
    return ret;
    }
    static const struct thermal_zone_device_ops adc_tm5_thermal_ops = {
    .get_temp = adc_tm5_get_temp,
    .set_trips = adc_tm5_set_trips,
    };
#[no_mangle]
unsafe extern "C" fn adc_tm5_register_tzd(adc_tm: *mut adc_tm5_chip) -> c_int {
    static int adc_tm5_register_tzd(struct adc_tm5_chip *adc_tm)
    {
    unsigned int i;
    struct thermal_zone_device *tzd;
    for (i = 0; i < adc_tm.nchannels; i++) {
    adc_tm.channels[i].chip = adc_tm;
    tzd = devm_thermal_of_zone_register(adc_tm.dev,
    adc_tm.channels[i].channel,
    &adc_tm.channels[i],
    &adc_tm5_thermal_ops);
    if (IS_ERR(tzd)) {
    if (PTR_ERR(tzd) == -ENODEV) {
    dev_dbg(adc_tm.dev, "thermal sensor on channel %d is not used\n",
    adc_tm.channels[i].channel);
    continue;
    }
    dev_err(adc_tm.dev, "Error registering TZ zone for channel %d: %ld\n",
    adc_tm.channels[i].channel, PTR_ERR(tzd));
    return PTR_ERR(tzd);
    }
    adc_tm.channels[i].tzd = tzd;
    devm_thermal_add_hwmon_sysfs(adc_tm.dev, tzd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm_hc_init(chip: *mut adc_tm5_chip) -> c_int {
    static int adc_tm_hc_init(struct adc_tm5_chip *chip)
    {
    unsigned int i;
    u8 buf[2];
    int ret;
    for (i = 0; i < chip.nchannels; i++) {
    if (chip.channels[i].channel >= ADC_TM5_NUM_CHANNELS) {
    dev_err(chip.dev, "Invalid channel %d\n", chip.channels[i].channel);
    return -EINVAL;
    }
    }
    buf[0] = chip.decimation;
    buf[1] = chip.avg_samples | ADC_TM5_FAST_AVG_EN;
    ret = adc_tm5_write(chip, ADC_TM5_ADC_DIG_PARAM, buf, sizeof(buf));
    if (ret)
    dev_err(chip.dev, "block write failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_init(chip: *mut adc_tm5_chip) -> c_int {
    static int adc_tm5_init(struct adc_tm5_chip *chip)
    {
    u8 buf[4], channels_available;
    int ret;
    unsigned int i;
    ret = adc_tm5_read(chip, ADC_TM5_NUM_BTM,
    &channels_available, sizeof(channels_available));
    if (ret) {
    dev_err(chip.dev, "read failed for BTM channels\n");
    return ret;
    }
    for (i = 0; i < chip.nchannels; i++) {
    if (chip.channels[i].channel >= channels_available) {
    dev_err(chip.dev, "Invalid channel %d\n", chip.channels[i].channel);
    return -EINVAL;
    }
    }
    buf[0] = chip.decimation;
    buf[1] = chip.avg_samples | ADC_TM5_FAST_AVG_EN;
    buf[2] = ADC_TM5_TIMER1;
    buf[3] = FIELD_PREP(ADC_TM5_MEAS_INTERVAL_CTL2_MASK, ADC_TM5_TIMER2) |
    FIELD_PREP(ADC_TM5_MEAS_INTERVAL_CTL3_MASK, ADC_TM5_TIMER3);
    ret = adc_tm5_write(chip, ADC_TM5_ADC_DIG_PARAM, buf, sizeof(buf));
    if (ret) {
    dev_err(chip.dev, "block write failed: %d\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_gen2_init(chip: *mut adc_tm5_chip) -> c_int {
    static int adc_tm5_gen2_init(struct adc_tm5_chip *chip)
    {
    u8 channels_available;
    int ret;
    unsigned int i;
    ret = adc_tm5_read(chip, ADC_TM5_NUM_BTM,
    &channels_available, sizeof(channels_available));
    if (ret) {
    dev_err(chip.dev, "read failed for BTM channels\n");
    return ret;
    }
    for (i = 0; i < chip.nchannels; i++) {
    if (chip.channels[i].channel >= channels_available) {
    dev_err(chip.dev, "Invalid channel %d\n", chip.channels[i].channel);
    return -EINVAL;
    }
    }
    mutex_init(&chip.adc_mutex_lock);
    return ret;
    }
    static int adc_tm5_get_dt_channel_data(struct adc_tm5_chip *adc_tm,
    struct adc_tm5_channel *channel,
    struct device_node *node)
    {
    const char *name = node.name;
    u32 chan, value, adc_channel, varr[2];
    int ret;
    struct device *dev = adc_tm.dev;
    struct of_phandle_args args;
    ret = of_property_read_u32(node, "reg", &chan);
    if (ret) {
    dev_err(dev, "%s: invalid channel number %d\n", name, ret);
    return ret;
    }
    if (chan >= ADC_TM5_NUM_CHANNELS) {
    dev_err(dev, "%s: channel number too big: %d\n", name, chan);
    return -EINVAL;
    }
    channel.channel = chan;
//
// We are tied to PMIC's ADC controller, which always use single
// argument for channel number.  So don't bother parsing
// #io-channel-cells, just enforce cell_count = 1.
//
    ret = of_parse_phandle_with_fixed_args(node, "io-channels", 1, 0, &args);
    if (ret < 0) {
    dev_err(dev, "%s: error parsing ADC channel number %d: %d\n", name, chan, ret);
    return ret;
    }
    of_node_put(args.np);
    if (args.args_count != 1) {
    dev_err(dev, "%s: invalid args count for ADC channel %d\n", name, chan);
    return -EINVAL;
    }
    adc_channel = args.args[0];
    if (adc_tm.data.gen == ADC_TM5_GEN2)
    adc_channel &= 0xff;
    if (adc_channel >= ADC5_MAX_CHANNEL) {
    dev_err(dev, "%s: invalid ADC channel number %d\n", name, chan);
    return -EINVAL;
    }
    channel.adc_channel = args.args[0];
    channel.iio = devm_fwnode_iio_channel_get_by_name(adc_tm.dev,
    of_fwnode_handle(node), core::ptr::null_mut());
    if (IS_ERR(channel.iio))
    return dev_err_probe(dev, PTR_ERR(channel.iio), "%s: error getting channel\n",
    name);
    ret = of_property_read_u32_array(node, "qcom,pre-scaling", varr, 2);
    if (!ret) {
    ret = qcom_adc5_prescaling_from_dt(varr[0], varr[1]);
    if (ret < 0) {
    dev_err(dev, "%s: invalid pre-scaling <%d %d>\n",
    name, varr[0], varr[1]);
    return ret;
    }
    channel.prescale = ret;
    } else {
// 1:1 prescale is index 0
    channel.prescale = 0;
    }
    ret = of_property_read_u32(node, "qcom,hw-settle-time-us", &value);
    if (!ret) {
    ret = qcom_adc5_hw_settle_time_from_dt(value, adc_tm.data.hw_settle);
    if (ret < 0) {
    dev_err(dev, "%s invalid hw-settle-time-us %d us\n",
    name, value);
    return ret;
    }
    channel.hw_settle_time = ret;
    } else {
    channel.hw_settle_time = VADC_DEF_HW_SETTLE_TIME;
    }
    if (of_property_read_bool(node, "qcom,ratiometric"))
    channel.cal_method = ADC_TM5_RATIOMETRIC_CAL;
    else
    channel.cal_method = ADC_TM5_ABSOLUTE_CAL;
    if (adc_tm.data.gen == ADC_TM5_GEN2) {
    ret = of_property_read_u32(node, "qcom,decimation", &value);
    if (!ret) {
    ret = qcom_adc5_decimation_from_dt(value, adc_tm.data.decimation);
    if (ret < 0) {
    dev_err(dev, "invalid decimation %d\n", value);
    return ret;
    }
    channel.decimation = ret;
    } else {
    channel.decimation = ADC5_DECIMATION_DEFAULT;
    }
    ret = of_property_read_u32(node, "qcom,avg-samples", &value);
    if (!ret) {
    ret = qcom_adc5_avg_samples_from_dt(value);
    if (ret < 0) {
    dev_err(dev, "invalid avg-samples %d\n", value);
    return ret;
    }
    channel.avg_samples = ret;
    } else {
    channel.avg_samples = VADC_DEF_AVG_SAMPLES;
    }
    }
    return 0;
    }
    static const struct adc_tm5_data adc_tm5_data_pmic = {
    .full_scale_code_volt = 0x70e4,
    .decimation = (unsigned int []) { 250, 420, 840 },
    .hw_settle = (unsigned int []) { 15, 100, 200, 300, 400, 500, 600, 700,
    1000, 2000, 4000, 8000, 16000, 32000,
    64000, 128000 },
    .disable_channel = adc_tm5_disable_channel,
    .configure = adc_tm5_configure,
    .isr = adc_tm5_isr,
    .init = adc_tm5_init,
    .irq_name = "pm-adc-tm5",
    .gen = ADC_TM5,
    };
    static const struct adc_tm5_data adc_tm_hc_data_pmic = {
    .full_scale_code_volt = 0x70e4,
    .decimation = (unsigned int []) { 256, 512, 1024 },
    .hw_settle = (unsigned int []) { 0, 100, 200, 300, 400, 500, 600, 700,
    1000, 2000, 4000, 6000, 8000, 10000 },
    .disable_channel = adc_tm5_disable_channel,
    .configure = adc_tm5_configure,
    .isr = adc_tm5_isr,
    .init = adc_tm_hc_init,
    .irq_name = "pm-adc-tm5",
    .gen = ADC_TM_HC,
    };
    static const struct adc_tm5_data adc_tm5_gen2_data_pmic = {
    .full_scale_code_volt = 0x70e4,
    .decimation = (unsigned int []) { 85, 340, 1360 },
    .hw_settle = (unsigned int []) { 15, 100, 200, 300, 400, 500, 600, 700,
    1000, 2000, 4000, 8000, 16000, 32000,
    64000, 128000 },
    .disable_channel = adc_tm5_gen2_disable_channel,
    .configure = adc_tm5_gen2_configure,
    .isr = adc_tm5_gen2_isr,
    .init = adc_tm5_gen2_init,
    .irq_name = "pm-adc-tm5-gen2",
    .gen = ADC_TM5_GEN2,
    };
#[no_mangle]
unsafe extern "C" fn adc_tm5_get_dt_data(adc_tm: *mut adc_tm5_chip, node: *mut device_node) -> c_int {
    static int adc_tm5_get_dt_data(struct adc_tm5_chip *adc_tm, struct device_node *node)
    {
    struct adc_tm5_channel *channels;
    u32 value;
    int ret;
    struct device *dev = adc_tm.dev;
    adc_tm.nchannels = of_get_available_child_count(node);
    if (!adc_tm.nchannels)
    return -EINVAL;
    adc_tm.channels = devm_kcalloc(dev, adc_tm.nchannels,
    sizeof(*adc_tm.channels), GFP_KERNEL);
    if (!adc_tm.channels)
    return -ENOMEM;
    channels = adc_tm.channels;
    adc_tm.data = of_device_get_match_data(dev);
    if (!adc_tm.data)
    adc_tm.data = &adc_tm5_data_pmic;
    ret = of_property_read_u32(node, "qcom,decimation", &value);
    if (!ret) {
    ret = qcom_adc5_decimation_from_dt(value, adc_tm.data.decimation);
    if (ret < 0) {
    dev_err(dev, "invalid decimation %d\n", value);
    return ret;
    }
    adc_tm.decimation = ret;
    } else {
    adc_tm.decimation = ADC5_DECIMATION_DEFAULT;
    }
    ret = of_property_read_u32(node, "qcom,avg-samples", &value);
    if (!ret) {
    ret = qcom_adc5_avg_samples_from_dt(value);
    if (ret < 0) {
    dev_err(dev, "invalid avg-samples %d\n", value);
    return ret;
    }
    adc_tm.avg_samples = ret;
    } else {
    adc_tm.avg_samples = VADC_DEF_AVG_SAMPLES;
    }
    for_each_available_child_of_node_scoped(node, child) {
    ret = adc_tm5_get_dt_channel_data(adc_tm, channels, child);
    if (ret)
    return ret;
    channels++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc_tm5_probe(pdev: *mut platform_device) -> c_int {
    static int adc_tm5_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct adc_tm5_chip *adc_tm;
    struct regmap *regmap;
    int ret, irq;
    u32 reg;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    ret = of_property_read_u32(node, "reg", &reg);
    if (ret)
    return ret;
    adc_tm = devm_kzalloc(&pdev.dev, sizeof(*adc_tm), GFP_KERNEL);
    if (!adc_tm)
    return -ENOMEM;
    adc_tm.regmap = regmap;
    adc_tm.dev = dev;
    adc_tm.base = reg;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = adc_tm5_get_dt_data(adc_tm, node);
    if (ret)
    return dev_err_probe(dev, ret, "get dt data failed\n");
    ret = adc_tm.data.init(adc_tm);
    if (ret) {
    dev_err(dev, "adc-tm init failed\n");
    return ret;
    }
    ret = adc_tm5_register_tzd(adc_tm);
    if (ret) {
    dev_err(dev, "tzd register failed\n");
    return ret;
    }
    return devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), adc_tm.data.isr,
    IRQF_ONESHOT, adc_tm.data.irq_name, adc_tm);
    }
    static const struct of_device_id adc_tm5_match_table[] = {
    {
    .compatible = "qcom,spmi-adc-tm5",
    .data = &adc_tm5_data_pmic,
    },
    {
    .compatible = "qcom,spmi-adc-tm-hc",
    .data = &adc_tm_hc_data_pmic,
    },
    {
    .compatible = "qcom,spmi-adc-tm5-gen2",
    .data = &adc_tm5_gen2_data_pmic,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, adc_tm5_match_table);
    static struct platform_driver adc_tm5_driver = {
    .driver = {
    .name = "qcom-spmi-adc-tm5",
    .of_match_table = adc_tm5_match_table,
    },
    .probe = adc_tm5_probe,
    };
    module_platform_driver(adc_tm5_driver);
    MODULE_DESCRIPTION("SPMI PMIC Thermal Monitor ADC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");
