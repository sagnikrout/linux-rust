//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/tsl2563.c
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
// drivers/iio/light/tsl2563.c
//
// Copyright (C) 2008 Nokia Corporation
//
// Written by Timo O. Karjalainen <timo.o.karjalainen@nokia.com>
// Contact: Amit Kucheria <amit.kucheria@verdurent.com>
//
// Converted to IIO driver
// Amit Kucheria <amit.kucheria@verdurent.com>
//

// Use this many bits for fraction part.
pub const ADC_FRAC_BITS: c_int = 14;
// Given number of 1/10000's in ADC_FRAC_BITS precision.

// Bits used for fraction in calibration coefficients.
pub const CALIB_FRAC_BITS: c_int = 10;
// Decimal 10^(digits in sysfs presentation)
pub const CALIB_BASE_SYSFS: c_int = 1000;

pub const TSL2563_REG_CTRL: c_uint = 0x00;
pub const TSL2563_REG_TIMING: c_uint = 0x01;
pub const TSL2563_REG_LOW: c_uint = 0x02 /* data0 low threshold, 2 bytes */;
pub const TSL2563_REG_HIGH: c_uint = 0x04 /* data0 high threshold, 2 bytes */;
pub const TSL2563_REG_INT: c_uint = 0x06;
pub const TSL2563_REG_ID: c_uint = 0x0a;
pub const TSL2563_REG_DATA0: c_uint = 0x0c /* broadband sensor value, 2 bytes */;
pub const TSL2563_REG_DATA1: c_uint = 0x0e /* infrared sensor value, 2 bytes */;
pub const TSL2563_CMD_POWER_ON: c_uint = 0x03;
pub const TSL2563_CMD_POWER_OFF: c_uint = 0x00;

pub const TSL2563_TIMING_13MS: c_uint = 0x00;
pub const TSL2563_TIMING_100MS: c_uint = 0x01;
pub const TSL2563_TIMING_400MS: c_uint = 0x02;

pub const TSL2563_TIMING_GAIN16: c_uint = 0x10;
pub const TSL2563_TIMING_GAIN1: c_uint = 0x00;
pub const TSL2563_INT_DISABLED: c_uint = 0x00;
pub const TSL2563_INT_LEVEL: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2563_gainlevel_coeff {
    pub gaintime: u8,
    pub min: u16,
    pub max: u16,
}

    static const struct tsl2563_gainlevel_coeff tsl2563_gainlevel_table[] = {
    {
    .gaintime	= TSL2563_TIMING_400MS | TSL2563_TIMING_GAIN16,
    .min		= 0,
    .max		= 65534,
    }, {
    .gaintime	= TSL2563_TIMING_400MS | TSL2563_TIMING_GAIN1,
    .min		= 2048,
    .max		= 65534,
    }, {
    .gaintime	= TSL2563_TIMING_100MS | TSL2563_TIMING_GAIN1,
    .min		= 4095,
    .max		= 37177,
    }, {
    .gaintime	= TSL2563_TIMING_13MS | TSL2563_TIMING_GAIN1,
    .min		= 3000,
    .max		= 65535,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2563_chip {
    pub lock: mutex,
    pub client: *mut i2c_client,
    pub poweroff_work: delayed_work,
// Remember state for suspend and resume functions
    pub suspended: bool,
    pub gainlevel: *const tsl2563_gainlevel_coeff,
    pub low_thres: u16,
    pub high_thres: u16,
    pub intr: u8,
    pub int_enabled: bool,
// Calibration coefficients
    pub calib0: u32,
    pub calib1: u32,
    pub cover_comp_gain: c_int,
// Cache current values, to be returned while suspended
    pub data0: u32,
    pub data1: u32,
}

#[no_mangle]
unsafe extern "C" fn tsl2563_set_power(chip: *mut tsl2563_chip, on: c_int) -> c_int {
    static int tsl2563_set_power(struct tsl2563_chip *chip, int on)
    {
    struct i2c_client *client = chip.client;
    u8 cmd;
    cmd = on ? TSL2563_CMD_POWER_ON : TSL2563_CMD_POWER_OFF;
    return i2c_smbus_write_byte_data(client,
    TSL2563_CMD | TSL2563_REG_CTRL, cmd);
    }
//
// Return value is 0 for off, 1 for on, or a negative error
// code if reading failed.
//
#[no_mangle]
unsafe extern "C" fn tsl2563_get_power(chip: *mut tsl2563_chip) -> c_int {
    static int tsl2563_get_power(struct tsl2563_chip *chip)
    {
    struct i2c_client *client = chip.client;
    int ret;
    ret = i2c_smbus_read_byte_data(client, TSL2563_CMD | TSL2563_REG_CTRL);
    if (ret < 0)
    return ret;
    return (ret & TSL2563_CTRL_POWER_MASK) == TSL2563_CMD_POWER_ON;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_configure(chip: *mut tsl2563_chip) -> c_int {
    static int tsl2563_configure(struct tsl2563_chip *chip)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(chip.client,
    TSL2563_CMD | TSL2563_REG_TIMING,
    chip.gainlevel.gaintime);
    if (ret)
    goto error_ret;
    ret = i2c_smbus_write_word_data(chip.client,
    TSL2563_CMD | TSL2563_REG_HIGH,
    chip.high_thres);
    if (ret)
    goto error_ret;
    ret = i2c_smbus_write_word_data(chip.client,
    TSL2563_CMD | TSL2563_REG_LOW,
    chip.low_thres);
    if (ret)
    goto error_ret;
//
// Interrupt register is automatically written anyway if it is relevant
// so is not here.
//
    error_ret:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_poweroff_work(work: *mut work_struct) {
    static void tsl2563_poweroff_work(struct work_struct *work)
    {
    struct tsl2563_chip *chip =
    container_of(work, struct tsl2563_chip, poweroff_work.work);
    tsl2563_set_power(chip, 0);
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_detect(chip: *mut tsl2563_chip) -> c_int {
    static int tsl2563_detect(struct tsl2563_chip *chip)
    {
    int ret;
    ret = tsl2563_set_power(chip, 1);
    if (ret)
    return ret;
    ret = tsl2563_get_power(chip);
    if (ret < 0)
    return ret;
    return ret ? 0 : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_read_id(chip: *mut tsl2563_chip, id: *mut u8) -> c_int {
    static int tsl2563_read_id(struct tsl2563_chip *chip, u8 *id)
    {
    struct i2c_client *client = chip.client;
    int ret;
    ret = i2c_smbus_read_byte_data(client, TSL2563_CMD | TSL2563_REG_ID);
    if (ret < 0)
    return ret;
// id = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_configure_irq(chip: *mut tsl2563_chip, enable: bool) -> c_int {
    static int tsl2563_configure_irq(struct tsl2563_chip *chip, bool enable)
    {
    int ret;
    chip.intr &= ~TSL2563_INT_MASK;
    if (enable)
    chip.intr |= TSL2563_INT_LEVEL;
    ret = i2c_smbus_write_byte_data(chip.client,
    TSL2563_CMD | TSL2563_REG_INT,
    chip.intr);
    if (ret < 0)
    return ret;
    chip.int_enabled = enable;
    return 0;
    }
//
// "Normalized" ADC value is one obtained with 400ms of integration time and
// 16x gain. This function returns the number of bits of shift needed to
// convert between normalized values and HW values obtained using given
// timing and gain settings.
//
#[no_mangle]
unsafe extern "C" fn tsl2563_adc_shiftbits(timing: u8) -> c_int {
    static int tsl2563_adc_shiftbits(u8 timing)
    {
    let mut shift: c_int = 0;
    switch (timing & TSL2563_TIMING_MASK) {
    case TSL2563_TIMING_13MS:
    shift += 5;
    break;
    case TSL2563_TIMING_100MS:
    shift += 2;
    break;
    case TSL2563_TIMING_400MS:
// no-op
    break;
    }
    if (!(timing & TSL2563_TIMING_GAIN16))
    shift += 4;
    return shift;
    }
// Convert a HW ADC value to normalized scale.
#[no_mangle]
unsafe extern "C" fn tsl2563_normalize_adc(adc: u16, timing: u8) -> u32 {
    static u32 tsl2563_normalize_adc(u16 adc, u8 timing)
    {
    return adc << tsl2563_adc_shiftbits(timing);
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_wait_adc(chip: *mut tsl2563_chip) {
    static void tsl2563_wait_adc(struct tsl2563_chip *chip)
    {
    unsigned int delay;
    switch (chip.gainlevel.gaintime & TSL2563_TIMING_MASK) {
    case TSL2563_TIMING_13MS:
    delay = 14;
    break;
    case TSL2563_TIMING_100MS:
    delay = 101;
    break;
    default:
    delay = 402;
    }
//
// TODO: Make sure that we wait at least required delay but why we
// have to extend it one tick more?
//
    schedule_timeout_interruptible(msecs_to_jiffies(delay) + 2);
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_adjust_gainlevel(chip: *mut tsl2563_chip, adc: u16) -> c_int {
    static int tsl2563_adjust_gainlevel(struct tsl2563_chip *chip, u16 adc)
    {
    struct i2c_client *client = chip.client;
    if (adc > chip.gainlevel.max || adc < chip.gainlevel.min) {
    (adc > chip.gainlevel.max) ?
    chip.gainlevel++ : chip.gainlevel--;
    i2c_smbus_write_byte_data(client,
    TSL2563_CMD | TSL2563_REG_TIMING,
    chip.gainlevel.gaintime);
    tsl2563_wait_adc(chip);
    tsl2563_wait_adc(chip);
    return 1;
    } else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_get_adc(chip: *mut tsl2563_chip) -> c_int {
    static int tsl2563_get_adc(struct tsl2563_chip *chip)
    {
    struct i2c_client *client = chip.client;
    u16 adc0, adc1;
    let mut retry: c_int = 1;
    let mut ret: c_int = 0;
    if (chip.suspended)
    goto out;
    if (!chip.int_enabled) {
    cancel_delayed_work_sync(&chip.poweroff_work);
    if (!tsl2563_get_power(chip)) {
    ret = tsl2563_set_power(chip, 1);
    if (ret)
    goto out;
    ret = tsl2563_configure(chip);
    if (ret)
    goto out;
    tsl2563_wait_adc(chip);
    }
    }
    while (retry) {
    ret = i2c_smbus_read_word_data(client,
    TSL2563_CMD | TSL2563_REG_DATA0);
    if (ret < 0)
    goto out;
    adc0 = ret;
    ret = i2c_smbus_read_word_data(client,
    TSL2563_CMD | TSL2563_REG_DATA1);
    if (ret < 0)
    goto out;
    adc1 = ret;
    retry = tsl2563_adjust_gainlevel(chip, adc0);
    }
    chip.data0 = tsl2563_normalize_adc(adc0, chip.gainlevel.gaintime);
    chip.data1 = tsl2563_normalize_adc(adc1, chip.gainlevel.gaintime);
    if (!chip.int_enabled)
    schedule_delayed_work(&chip.poweroff_work, 5 * HZ);
    ret = 0;
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tsl2563_calib_to_sysfs(calib: u32) -> c_int {
    static inline int tsl2563_calib_to_sysfs(u32 calib)
    {
    return (int)DIV_ROUND_CLOSEST(calib * CALIB_BASE_SYSFS, BIT(CALIB_FRAC_BITS));
    }
#[no_mangle]
pub unsafe extern "C" fn tsl2563_calib_from_sysfs(value: c_int) -> u32 {
    static inline u32 tsl2563_calib_from_sysfs(int value)
    {
// Make a fraction from a number n that was multiplied with b.
    return (((u32) value) << CALIB_FRAC_BITS) / CALIB_BASE_SYSFS;
    }
//
// Conversions between lux and ADC values.
//
// The basic formula is lux = c0 * adc0 - c1 * adc1, where c0 and c1 are
// appropriate constants. Different constants are needed for different
// kinds of light, determined by the ratio adc1/adc0 (basically the ratio
// of the intensities in infrared and visible wavelengths). lux_table below
// lists the upper threshold of the adc1/adc0 ratio and the corresponding
// constants.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2563_lux_coeff {
    pub ch_ratio: c_ulong,
    pub ch0_coeff: c_ulong,
    pub ch1_coeff: c_ulong,
}

    static const struct tsl2563_lux_coeff lux_table[] = {
    {
    .ch_ratio	= FRAC10K(1300),
    .ch0_coeff	= FRAC10K(315),
    .ch1_coeff	= FRAC10K(262),
    }, {
    .ch_ratio	= FRAC10K(2600),
    .ch0_coeff	= FRAC10K(337),
    .ch1_coeff	= FRAC10K(430),
    }, {
    .ch_ratio	= FRAC10K(3900),
    .ch0_coeff	= FRAC10K(363),
    .ch1_coeff	= FRAC10K(529),
    }, {
    .ch_ratio	= FRAC10K(5200),
    .ch0_coeff	= FRAC10K(392),
    .ch1_coeff	= FRAC10K(605),
    }, {
    .ch_ratio	= FRAC10K(6500),
    .ch0_coeff	= FRAC10K(229),
    .ch1_coeff	= FRAC10K(291),
    }, {
    .ch_ratio	= FRAC10K(8000),
    .ch0_coeff	= FRAC10K(157),
    .ch1_coeff	= FRAC10K(180),
    }, {
    .ch_ratio	= FRAC10K(13000),
    .ch0_coeff	= FRAC10K(34),
    .ch1_coeff	= FRAC10K(26),
    }, {
    .ch_ratio	= ULONG_MAX,
    .ch0_coeff	= 0,
    .ch1_coeff	= 0,
    },
    };
// Convert normalized, scaled ADC values to lux.
#[no_mangle]
unsafe extern "C" fn tsl2563_adc_to_lux(adc0: u32, adc1: u32) -> c_uint {
    static unsigned int tsl2563_adc_to_lux(u32 adc0, u32 adc1)
    {
    const struct tsl2563_lux_coeff *lp = lux_table;
    unsigned long ratio, lux, ch0 = adc0, ch1 = adc1;
    ratio = ch0 ? ((ch1 << ADC_FRAC_BITS) / ch0) : ULONG_MAX;
    while (lp.ch_ratio < ratio)
    lp++;
    lux = ch0 * lp.ch0_coeff - ch1 * lp.ch1_coeff;
    return (unsigned int) (lux >> ADC_FRAC_BITS);
    }
// Apply calibration coefficient to ADC count.
#[no_mangle]
unsafe extern "C" fn tsl2563_calib_adc(adc: u32, calib: u32) -> u32 {
    static u32 tsl2563_calib_adc(u32 adc, u32 calib)
    {
    let mut scaled: c_ulong = adc;
    scaled *= calib;
    scaled >>= CALIB_FRAC_BITS;
    return (u32) scaled;
    }
    static int tsl2563_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    if (mask != IIO_CHAN_INFO_CALIBSCALE)
    return -EINVAL;
    if (chan.channel2 == IIO_MOD_LIGHT_BOTH)
    chip.calib0 = tsl2563_calib_from_sysfs(val);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_MOD_LIGHT_IR: chan->channel2 ==) -> else {
    else if (chan.channel2 == IIO_MOD_LIGHT_IR)
    chip.calib1 = tsl2563_calib_from_sysfs(val);
    else
    return -EINVAL;
    return 0;
    }
    static int tsl2563_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    let mut ret: c_int = -EINVAL;
    u32 calib0, calib1;
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    mutex_lock(&chip.lock);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    case IIO_CHAN_INFO_PROCESSED:
    switch (chan.type) {
    case IIO_LIGHT:
    ret = tsl2563_get_adc(chip);
    if (ret)
    goto error_ret;
    calib0 = tsl2563_calib_adc(chip.data0, chip.calib0) *
    chip.cover_comp_gain;
    calib1 = tsl2563_calib_adc(chip.data1, chip.calib1) *
    chip.cover_comp_gain;
// val = tsl2563_adc_to_lux(calib0, calib1);
    ret = IIO_VAL_INT;
    break;
    case IIO_INTENSITY:
    ret = tsl2563_get_adc(chip);
    if (ret)
    goto error_ret;
    if (chan.channel2 == IIO_MOD_LIGHT_BOTH)
// val = chip->data0;
    else
// val = chip->data1;
    ret = IIO_VAL_INT;
    break;
    default:
    break;
    }
    break;
    case IIO_CHAN_INFO_CALIBSCALE:
    if (chan.channel2 == IIO_MOD_LIGHT_BOTH)
// val = tsl2563_calib_to_sysfs(chip->calib0);
    else
// val = tsl2563_calib_to_sysfs(chip->calib1);
    ret = IIO_VAL_INT;
    break;
    default:
    ret = -EINVAL;
    goto error_ret;
    }
    error_ret:
    mutex_unlock(&chip.lock);
    return ret;
    }
    static const struct iio_event_spec tsl2563_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    }, {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_chan_spec tsl2563_channels[] = {
    {
    .type = IIO_LIGHT,
    .indexed = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .channel = 0,
    }, {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_BOTH,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_CALIBSCALE),
    .event_spec = tsl2563_events,
    .num_event_specs = ARRAY_SIZE(tsl2563_events),
    }, {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_IR,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_CALIBSCALE),
    }
    };
    static int tsl2563_read_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, enum iio_event_info info, int *val,
    int *val2)
    {
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    switch (dir) {
    case IIO_EV_DIR_RISING:
// val = chip->high_thres;
    break;
    case IIO_EV_DIR_FALLING:
// val = chip->low_thres;
    break;
    default:
    return -EINVAL;
    }
    return IIO_VAL_INT;
    }
    static int tsl2563_write_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, enum iio_event_info info, int val,
    int val2)
    {
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    int ret;
    mutex_lock(&chip.lock);
    if (dir == IIO_EV_DIR_RISING)
    ret = i2c_smbus_write_word_data(chip.client,
    TSL2563_CMD | TSL2563_REG_HIGH, val);
    else
    ret = i2c_smbus_write_word_data(chip.client,
    TSL2563_CMD | TSL2563_REG_LOW, val);
    if (ret)
    goto error_ret;
    if (dir == IIO_EV_DIR_RISING)
    chip.high_thres = val;
    else
    chip.low_thres = val;
    error_ret:
    mutex_unlock(&chip.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_event_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t tsl2563_event_handler(int irq, void *private)
    {
    struct iio_dev *dev_info = private;
    struct tsl2563_chip *chip = iio_priv(dev_info);
    iio_push_event(dev_info,
    IIO_UNMOD_EVENT_CODE(IIO_INTENSITY,
    0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(dev_info));
// clear the interrupt and push the event
    i2c_smbus_write_byte(chip.client, TSL2563_CMD | TSL2563_CLEARINT);
    return IRQ_HANDLED;
    }
    static int tsl2563_write_interrupt_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir, bool state)
    {
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    let mut ret: c_int = 0;
    mutex_lock(&chip.lock);
    if (state && !(chip.intr & TSL2563_INT_MASK)) {
// ensure the chip is actually on
    cancel_delayed_work_sync(&chip.poweroff_work);
    if (!tsl2563_get_power(chip)) {
    ret = tsl2563_set_power(chip, 1);
    if (ret)
    goto out;
    ret = tsl2563_configure(chip);
    if (ret)
    goto out;
    }
    ret = tsl2563_configure_irq(chip, true);
    }
    if (!state && (chip.intr & TSL2563_INT_MASK)) {
    ret = tsl2563_configure_irq(chip, false);
// now the interrupt is not enabled, we can go to sleep
    schedule_delayed_work(&chip.poweroff_work, 5 * HZ);
    }
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
    static int tsl2563_read_interrupt_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    int ret;
    mutex_lock(&chip.lock);
    ret = i2c_smbus_read_byte_data(chip.client,
    TSL2563_CMD | TSL2563_REG_INT);
    mutex_unlock(&chip.lock);
    if (ret < 0)
    return ret;
    return !!(ret & TSL2563_INT_MASK);
    }
    static const struct iio_info tsl2563_info_no_irq = {
    .read_raw = &tsl2563_read_raw,
    .write_raw = &tsl2563_write_raw,
    };
    static const struct iio_info tsl2563_info = {
    .read_raw = &tsl2563_read_raw,
    .write_raw = &tsl2563_write_raw,
    .read_event_value = &tsl2563_read_thresh,
    .write_event_value = &tsl2563_write_thresh,
    .read_event_config = &tsl2563_read_interrupt_config,
    .write_event_config = &tsl2563_write_interrupt_config,
    };
#[no_mangle]
unsafe extern "C" fn tsl2563_probe(client: *mut i2c_client) -> c_int {
    static int tsl2563_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct tsl2563_chip *chip;
    unsigned long irq_flags;
    let mut id: u8 = 0;
    int err;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*chip));
    if (!indio_dev)
    return -ENOMEM;
    chip = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    chip.client = client;
    err = tsl2563_detect(chip);
    if (err)
    return dev_err_probe(dev, err, "detect error\n");
    err = tsl2563_read_id(chip, &id);
    if (err)
    return dev_err_probe(dev, err, "read id error\n");
    mutex_init(&chip.lock);
// Default values used until userspace says otherwise
    chip.low_thres = 0x0;
    chip.high_thres = 0xffff;
    chip.gainlevel = tsl2563_gainlevel_table;
    chip.intr = TSL2563_INT_PERSIST(4);
    chip.calib0 = tsl2563_calib_from_sysfs(CALIB_BASE_SYSFS);
    chip.calib1 = tsl2563_calib_from_sysfs(CALIB_BASE_SYSFS);
    chip.cover_comp_gain = 1;
    device_property_read_u32(dev, "amstaos,cover-comp-gain", &chip.cover_comp_gain);
    dev_info(dev, "model %d, rev. %d\n", id >> 4, id & 0x0f);
    indio_dev.name = client.name;
    indio_dev.channels = tsl2563_channels;
    indio_dev.num_channels = ARRAY_SIZE(tsl2563_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    if (client.irq)
    indio_dev.info = &tsl2563_info;
    else
    indio_dev.info = &tsl2563_info_no_irq;
    if (client.irq) {
    irq_flags = irq_get_trigger_type(client.irq);
    if (irq_flags == IRQF_TRIGGER_NONE)
    irq_flags = IRQF_TRIGGER_RISING;
    irq_flags |= IRQF_ONESHOT;
    err = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(),
    &tsl2563_event_handler,
    irq_flags,
    "tsl2563_event",
    indio_dev);
    if (err)
    return err;
    }
    err = tsl2563_configure(chip);
    if (err)
    return dev_err_probe(dev, err, "configure error\n");
    INIT_DELAYED_WORK(&chip.poweroff_work, tsl2563_poweroff_work);
// The interrupt cannot yet be enabled so this is fine without lock
    schedule_delayed_work(&chip.poweroff_work, 5 * HZ);
    err = iio_device_register(indio_dev);
    if (err) {
    dev_err_probe(dev, err, "iio registration error\n");
    goto fail;
    }
    return 0;
    fail:
    cancel_delayed_work_sync(&chip.poweroff_work);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_remove(client: *mut i2c_client) {
    static void tsl2563_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    if (!chip.int_enabled)
    cancel_delayed_work_sync(&chip.poweroff_work);
// Ensure that interrupts are disabled - then flush any bottom halves
    tsl2563_configure_irq(chip, false);
    tsl2563_set_power(chip, 0);
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_suspend(dev: *mut device) -> c_int {
    static int tsl2563_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    int ret;
    mutex_lock(&chip.lock);
    ret = tsl2563_set_power(chip, 0);
    if (ret)
    goto out;
    chip.suspended = true;
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tsl2563_resume(dev: *mut device) -> c_int {
    static int tsl2563_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct tsl2563_chip *chip = iio_priv(indio_dev);
    int ret;
    mutex_lock(&chip.lock);
    ret = tsl2563_set_power(chip, 1);
    if (ret)
    goto out;
    ret = tsl2563_configure(chip);
    if (ret)
    goto out;
    chip.suspended = false;
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tsl2563_pm_ops, tsl2563_suspend,
    tsl2563_resume);
    static const struct i2c_device_id tsl2563_id[] = {
    { .name = "tsl2560" },
    { .name = "tsl2561" },
    { .name = "tsl2562" },
    { .name = "tsl2563" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tsl2563_id);
    static const struct of_device_id tsl2563_of_match[] = {
    { .compatible = "amstaos,tsl2560" },
    { .compatible = "amstaos,tsl2561" },
    { .compatible = "amstaos,tsl2562" },
    { .compatible = "amstaos,tsl2563" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tsl2563_of_match);
    static struct i2c_driver tsl2563_i2c_driver = {
    .driver = {
    .name	 = "tsl2563",
    .of_match_table = tsl2563_of_match,
    .pm	= pm_sleep_ptr(&tsl2563_pm_ops),
    },
    .probe		= tsl2563_probe,
    .remove		= tsl2563_remove,
    .id_table	= tsl2563_id,
    };
    module_i2c_driver(tsl2563_i2c_driver);
    MODULE_AUTHOR("Nokia Corporation");
    MODULE_DESCRIPTION("tsl2563 light sensor driver");
    MODULE_LICENSE("GPL");
