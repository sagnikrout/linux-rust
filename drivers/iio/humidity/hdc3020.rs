//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/hdc3020.c
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
// hdc3020.c - Support for the TI HDC3020,HDC3021 and HDC3022
// temperature + relative humidity sensors
//
// Copyright (C) 2023
//
// Copyright (C) 2024 Liebherr-Electronics and Drives GmbH
//
// Datasheet: https://www.ti.com/lit/ds/symlink/hdc3020.pdf
//

pub const HDC3020_S_AUTO_10HZ_MOD0: c_uint = 0x2737;
pub const HDC3020_S_STATUS: c_uint = 0x3041;
pub const HDC3020_HEATER_DISABLE: c_uint = 0x3066;
pub const HDC3020_HEATER_ENABLE: c_uint = 0x306D;
pub const HDC3020_HEATER_CONFIG: c_uint = 0x306E;
pub const HDC3020_EXIT_AUTO: c_uint = 0x3093;
pub const HDC3020_S_T_RH_THRESH_LOW: c_uint = 0x6100;
pub const HDC3020_S_T_RH_THRESH_LOW_CLR: c_uint = 0x610B;
pub const HDC3020_S_T_RH_THRESH_HIGH_CLR: c_uint = 0x6116;
pub const HDC3020_S_T_RH_THRESH_HIGH: c_uint = 0x611D;
pub const HDC3020_R_T_RH_AUTO: c_uint = 0xE000;
pub const HDC3020_R_T_LOW_AUTO: c_uint = 0xE002;
pub const HDC3020_R_T_HIGH_AUTO: c_uint = 0xE003;
pub const HDC3020_R_RH_LOW_AUTO: c_uint = 0xE004;
pub const HDC3020_R_RH_HIGH_AUTO: c_uint = 0xE005;
pub const HDC3020_R_T_RH_THRESH_LOW: c_uint = 0xE102;
pub const HDC3020_R_T_RH_THRESH_LOW_CLR: c_uint = 0xE109;
pub const HDC3020_R_T_RH_THRESH_HIGH_CLR: c_uint = 0xE114;
pub const HDC3020_R_T_RH_THRESH_HIGH: c_uint = 0xE11F;
pub const HDC3020_R_STATUS: c_uint = 0xF32D;

pub const HDC3020_THRESH_TEMP_TRUNC_SHIFT: c_int = 7;

pub const HDC3020_THRESH_HUM_TRUNC_SHIFT: c_int = 9;

pub const HDC3020_READ_RETRY_TIMES: c_int = 10;
pub const HDC3020_BUSY_DELAY_MS: c_int = 10;
pub const HDC3020_CRC8_POLYNOMIAL: c_uint = 0x31;

pub const HDC3020_MAX_TEMP_MICRO: c_int = 124875639;
pub const HDC3020_MAX_TEMP_HYST_MICRO: c_int = 164748607;
pub const HDC3020_MAX_HUM_MICRO: c_int = 99220264;
// Divide 65535 from the datasheet by 5 to avoid overflows

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdc3020_data {
    pub client: *mut i2c_client,
    pub reset_gpio: *mut gpio_desc,
    pub vdd_supply: *mut regulator,
//
// Ensure that the sensor configuration (currently only heater is
// supported) will not be changed during the process of reading
// sensor data (this driver will try HDC3020_READ_RETRY_TIMES times
// if the device does not respond).
//
    pub lock: mutex,
}

    static const int hdc3020_heater_vals[] = {0, 1, 0x3FFF};
    static const struct iio_event_spec hdc3020_t_rh_event[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_HYSTERESIS),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_HYSTERESIS),
    },
    };
    static const struct iio_chan_spec hdc3020_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) | BIT(IIO_CHAN_INFO_PEAK) |
    BIT(IIO_CHAN_INFO_TROUGH) | BIT(IIO_CHAN_INFO_OFFSET),
    .event_spec = hdc3020_t_rh_event,
    .num_event_specs = ARRAY_SIZE(hdc3020_t_rh_event),
    },
    {
    .type = IIO_HUMIDITYRELATIVE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) | BIT(IIO_CHAN_INFO_PEAK) |
    BIT(IIO_CHAN_INFO_TROUGH),
    .event_spec = hdc3020_t_rh_event,
    .num_event_specs = ARRAY_SIZE(hdc3020_t_rh_event),
    },
    {
//
// For setting the internal heater, which can be switched on to
// prevent or remove any condensation that may develop when the
// ambient environment approaches its dew point temperature.
//
    .type = IIO_CURRENT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_separate_available = BIT(IIO_CHAN_INFO_RAW),
    .output = 1,
    },
    };
    DECLARE_CRC8_TABLE(hdc3020_crc8_table);
#[no_mangle]
unsafe extern "C" fn hdc3020_write_bytes(data: *mut hdc3020_data, buf: *mut u8, len: u8) -> c_int {
    static int hdc3020_write_bytes(struct hdc3020_data *data, u8 *buf, u8 len)
    {
    struct i2c_client *client = data.client;
    struct i2c_msg msg;
    int ret, cnt;
    msg.addr = client.addr;
    msg.flags = 0;
    msg.buf = buf;
    msg.len = len;
//
// During the measurement process, HDC3020 will not return data.
// So wait for a while and try again
//
    for (cnt = 0; cnt < HDC3020_READ_RETRY_TIMES; cnt++) {
    ret = i2c_transfer(client.adapter, &msg, 1);
    if (ret == 1)
    return 0;
    mdelay(HDC3020_BUSY_DELAY_MS);
    }
    dev_err(&client.dev, "Could not write sensor command\n");
    return -ETIMEDOUT;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn hdc3020_read_bytes(data: *mut hdc3020_data, reg: u16, buf: *mut u8, len: c_int) -> c_int {
    int hdc3020_read_bytes(struct hdc3020_data *data, u16 reg, u8 *buf, int len)
    {
    u8 reg_buf[2];
    int ret, cnt;
    struct i2c_client *client = data.client;
    struct i2c_msg msg[2] = {
    [0] = {
    .addr = client.addr,
    .flags = 0,
    .buf = reg_buf,
    .len = 2,
    },
    [1] = {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .buf = buf,
    .len = len,
    },
    };
    put_unaligned_be16(reg, reg_buf);
//
// During the measurement process, HDC3020 will not return data.
// So wait for a while and try again
//
    for (cnt = 0; cnt < HDC3020_READ_RETRY_TIMES; cnt++) {
    ret = i2c_transfer(client.adapter, msg, 2);
    if (ret == 2)
    return 0;
    mdelay(HDC3020_BUSY_DELAY_MS);
    }
    dev_err(&client.dev, "Could not read sensor data\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_read_be16(data: *mut hdc3020_data, reg: u16) -> c_int {
    static int hdc3020_read_be16(struct hdc3020_data *data, u16 reg)
    {
    u8 crc, buf[3];
    int ret;
    ret = hdc3020_read_bytes(data, reg, buf, 3);
    if (ret < 0)
    return ret;
    crc = crc8(hdc3020_crc8_table, buf, 2, CRC8_INIT_VALUE);
    if (crc != buf[2])
    return -EINVAL;
    return get_unaligned_be16(buf);
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_exec_cmd(data: *mut hdc3020_data, reg: u16) -> c_int {
    static int hdc3020_exec_cmd(struct hdc3020_data *data, u16 reg)
    {
    u8 reg_buf[2];
    put_unaligned_be16(reg, reg_buf);
    return hdc3020_write_bytes(data, reg_buf, 2);
    }
    static int hdc3020_read_measurement(struct hdc3020_data *data,
    enum iio_chan_type type, int *val)
    {
    u8 crc, buf[6];
    int ret;
    ret = hdc3020_read_bytes(data, HDC3020_R_T_RH_AUTO, buf, 6);
    if (ret < 0)
    return ret;
// CRC check of the temperature measurement
    crc = crc8(hdc3020_crc8_table, buf, 2, CRC8_INIT_VALUE);
    if (crc != buf[2])
    return -EINVAL;
// CRC check of the relative humidity measurement
    crc = crc8(hdc3020_crc8_table, buf + 3, 2, CRC8_INIT_VALUE);
    if (crc != buf[5])
    return -EINVAL;
    if (type == IIO_TEMP)
// val = get_unaligned_be16(buf);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_HUMIDITYRELATIVE: type ==) -> else {
    else if (type == IIO_HUMIDITYRELATIVE)
// val = get_unaligned_be16(&buf[3]);
    else
    return -EINVAL;
    return 0;
    }
    static int hdc3020_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct hdc3020_data *data = iio_priv(indio_dev);
    int ret;
    if (chan.type != IIO_TEMP && chan.type != IIO_HUMIDITYRELATIVE)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    guard(mutex)(&data.lock);
    ret = hdc3020_read_measurement(data, chan.type, val);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_PEAK: {
    guard(mutex)(&data.lock);
    if (chan.type == IIO_TEMP)
    ret = hdc3020_read_be16(data, HDC3020_R_T_HIGH_AUTO);
    else
    ret = hdc3020_read_be16(data, HDC3020_R_RH_HIGH_AUTO);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_TROUGH: {
    guard(mutex)(&data.lock);
    if (chan.type == IIO_TEMP)
    ret = hdc3020_read_be16(data, HDC3020_R_T_LOW_AUTO);
    else
    ret = hdc3020_read_be16(data, HDC3020_R_RH_LOW_AUTO);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
// val2 = 65536;
    if (chan.type == IIO_TEMP)
// val = 175 * MILLI;
    else
// val = 100 * MILLI;
    return IIO_VAL_FRACTIONAL;
    case IIO_CHAN_INFO_OFFSET:
    if (chan.type != IIO_TEMP)
    return -EINVAL;
// val = -16852;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int hdc3020_read_available(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals,
    int *type, int *length, long mask)
    {
    if (mask != IIO_CHAN_INFO_RAW || chan.type != IIO_CURRENT)
    return -EINVAL;
// vals = hdc3020_heater_vals;
// type = IIO_VAL_INT;
    return IIO_AVAIL_RANGE;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_update_heater(data: *mut hdc3020_data, val: c_int) -> c_int {
    static int hdc3020_update_heater(struct hdc3020_data *data, int val)
    {
    u8 buf[5];
    int ret;
    if (val < hdc3020_heater_vals[0] || val > hdc3020_heater_vals[2])
    return -EINVAL;
    if (!val)
    hdc3020_exec_cmd(data, HDC3020_HEATER_DISABLE);
    put_unaligned_be16(HDC3020_HEATER_CONFIG, buf);
    put_unaligned_be16(val & GENMASK(13, 0), &buf[2]);
    buf[4] = crc8(hdc3020_crc8_table, buf + 2, 2, CRC8_INIT_VALUE);
    ret = hdc3020_write_bytes(data, buf, 5);
    if (ret < 0)
    return ret;
    return hdc3020_exec_cmd(data, HDC3020_HEATER_ENABLE);
    }
    static int hdc3020_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct hdc3020_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (chan.type != IIO_CURRENT)
    return -EINVAL;
    guard(mutex)(&data.lock);
    return hdc3020_update_heater(data, val);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_thresh_get_temp(thresh: u16) -> c_int {
    static int hdc3020_thresh_get_temp(u16 thresh)
    {
    int temp;
//
// Get the temperature threshold from 9 LSBs, shift them to get the
// truncated temperature threshold representation and calculate the
// threshold according to the explicit formula in the datasheet:
// T(C) = -45 + (175 * temp) / 65535.
// Additionally scale by HDC3020_THRESH_FRACTION to avoid precision loss
// when calculating threshold and hysteresis values. Result is degree
// celsius scaled by HDC3020_THRESH_FRACTION.
//
    temp = FIELD_GET(HDC3020_THRESH_TEMP_MASK, thresh) <<
    HDC3020_THRESH_TEMP_TRUNC_SHIFT;
    return -2949075 / 5 + (175 / 5 * temp);
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_thresh_get_hum(thresh: u16) -> c_int {
    static int hdc3020_thresh_get_hum(u16 thresh)
    {
    int hum;
//
// Get the humidity threshold from 7 MSBs, shift them to get the
// truncated humidity threshold representation and calculate the
// threshold according to the explicit formula in the datasheet:
// RH(%) = 100 * hum / 65535.
// Additionally scale by HDC3020_THRESH_FRACTION to avoid precision loss
// when calculating threshold and hysteresis values. Result is percent
// scaled by HDC3020_THRESH_FRACTION.
//
    hum = FIELD_GET(HDC3020_THRESH_HUM_MASK, thresh) <<
    HDC3020_THRESH_HUM_TRUNC_SHIFT;
    return hum * 100 / 5;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_thresh_set_temp(s_temp: c_int, curr_thresh: u16) -> u16 {
    static u16 hdc3020_thresh_set_temp(int s_temp, u16 curr_thresh)
    {
    u64 temp;
    u16 thresh;
//
// Calculate temperature threshold, shift it down to get the
// truncated threshold representation in the 9LSBs while keeping
// the current humidity threshold in the 7 MSBs.
//
    temp = (u64)(s_temp + 45000000) * 65535ULL;
    temp = div_u64(temp, 1000000 * 175) >> HDC3020_THRESH_TEMP_TRUNC_SHIFT;
    thresh = FIELD_PREP(HDC3020_THRESH_TEMP_MASK, temp);
    thresh |= (FIELD_GET(HDC3020_THRESH_HUM_MASK, curr_thresh) <<
    HDC3020_THRESH_HUM_TRUNC_SHIFT);
    return thresh;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_thresh_set_hum(s_hum: c_int, curr_thresh: u16) -> u16 {
    static u16 hdc3020_thresh_set_hum(int s_hum, u16 curr_thresh)
    {
    u64 hum;
    u16 thresh;
//
// Calculate humidity threshold, shift it down and up to get the
// truncated threshold representation in the 7MSBs while keeping
// the current temperature threshold in the 9 LSBs.
//
    hum = (u64)(s_hum) * 65535ULL;
    hum = div_u64(hum, 1000000 * 100) >> HDC3020_THRESH_HUM_TRUNC_SHIFT;
    thresh = FIELD_PREP(HDC3020_THRESH_HUM_MASK, hum);
    thresh |= FIELD_GET(HDC3020_THRESH_TEMP_MASK, curr_thresh);
    return thresh;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn hdc3020_thresh_clr(s_thresh: i64, s_hyst: i64, dir: enum iio_event_direction) -> c_int {
    int hdc3020_thresh_clr(s64 s_thresh, s64 s_hyst, enum iio_event_direction dir)
    {
    s64 s_clr;
//
// Include directions when calculation the clear value,
// since hysteresis is unsigned by definition and the
// clear value is an absolute value which is signed.
//
    if (dir == IIO_EV_DIR_RISING)
    s_clr = s_thresh - s_hyst;
    else
    s_clr = s_thresh + s_hyst;
// Divide by HDC3020_THRESH_FRACTION to get units of micro
    return div_s64(s_clr, HDC3020_THRESH_FRACTION);
    }
#[no_mangle]
unsafe extern "C" fn _hdc3020_write_thresh(data: *mut hdc3020_data, reg: u16, val: u16) -> c_int {
    static int _hdc3020_write_thresh(struct hdc3020_data *data, u16 reg, u16 val)
    {
    u8 buf[5];
    put_unaligned_be16(reg, buf);
    put_unaligned_be16(val, buf + 2);
    buf[4] = crc8(hdc3020_crc8_table, buf + 2, 2, CRC8_INIT_VALUE);
    return hdc3020_write_bytes(data, buf, 5);
    }
    static int hdc3020_write_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct hdc3020_data *data = iio_priv(indio_dev);
    u16 reg, reg_val, reg_thresh_rd, reg_clr_rd, reg_thresh_wr, reg_clr_wr;
    s64 s_thresh, s_hyst, s_clr;
    int s_val, thresh, clr, ret;
// Select threshold registers
    if (dir == IIO_EV_DIR_RISING) {
    reg_thresh_rd = HDC3020_R_T_RH_THRESH_HIGH;
    reg_thresh_wr = HDC3020_S_T_RH_THRESH_HIGH;
    reg_clr_rd = HDC3020_R_T_RH_THRESH_HIGH_CLR;
    reg_clr_wr = HDC3020_S_T_RH_THRESH_HIGH_CLR;
    } else {
    reg_thresh_rd = HDC3020_R_T_RH_THRESH_LOW;
    reg_thresh_wr = HDC3020_S_T_RH_THRESH_LOW;
    reg_clr_rd = HDC3020_R_T_RH_THRESH_LOW_CLR;
    reg_clr_wr = HDC3020_S_T_RH_THRESH_LOW_CLR;
    }
    guard(mutex)(&data.lock);
    ret = hdc3020_read_be16(data, reg_thresh_rd);
    if (ret < 0)
    return ret;
    thresh = ret;
    ret = hdc3020_read_be16(data, reg_clr_rd);
    if (ret < 0)
    return ret;
    clr = ret;
// Scale value to include decimal part into calculations
    s_val = (val < 0) ? (val * 1000 - val2) : (val * 1000 + val2);
    switch (chan.type) {
    case IIO_TEMP:
    switch (info) {
    case IIO_EV_INFO_VALUE:
    s_val = max(s_val, HDC3020_MIN_TEMP_MICRO);
    s_val = min(s_val, HDC3020_MAX_TEMP_MICRO);
    reg = reg_thresh_wr;
    reg_val = hdc3020_thresh_set_temp(s_val, thresh);
    ret = _hdc3020_write_thresh(data, reg, reg_val);
    if (ret < 0)
    return ret;
// Calculate old hysteresis
    s_thresh = (s64)hdc3020_thresh_get_temp(thresh) * 1000000;
    s_clr = (s64)hdc3020_thresh_get_temp(clr) * 1000000;
    s_hyst = div_s64(abs(s_thresh - s_clr),
    HDC3020_THRESH_FRACTION);
// Set new threshold
    thresh = reg_val;
// Set old hysteresis
    s_val = s_hyst;
    fallthrough;
    case IIO_EV_INFO_HYSTERESIS:
//
// Function hdc3020_thresh_get_temp returns temperature
// in degree celsius scaled by HDC3020_THRESH_FRACTION.
// Scale by 1000000 to be able to subtract scaled
// hysteresis value.
//
    s_thresh = (s64)hdc3020_thresh_get_temp(thresh) * 1000000;
//
// Units of s_val are in micro degree celsius, scale by
// HDC3020_THRESH_FRACTION to get same units as s_thresh.
//
    s_val = min(abs(s_val), HDC3020_MAX_TEMP_HYST_MICRO);
    s_hyst = (s64)s_val * HDC3020_THRESH_FRACTION;
    s_clr = hdc3020_thresh_clr(s_thresh, s_hyst, dir);
    s_clr = max(s_clr, HDC3020_MIN_TEMP_MICRO);
    s_clr = min(s_clr, HDC3020_MAX_TEMP_MICRO);
    reg = reg_clr_wr;
    reg_val = hdc3020_thresh_set_temp(s_clr, clr);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case IIO_HUMIDITYRELATIVE:
    s_val = (s_val < 0) ? 0 : min(s_val, HDC3020_MAX_HUM_MICRO);
    switch (info) {
    case IIO_EV_INFO_VALUE:
    reg = reg_thresh_wr;
    reg_val = hdc3020_thresh_set_hum(s_val, thresh);
    ret = _hdc3020_write_thresh(data, reg, reg_val);
    if (ret < 0)
    return ret;
// Calculate old hysteresis
    s_thresh = (s64)hdc3020_thresh_get_hum(thresh) * 1000000;
    s_clr = (s64)hdc3020_thresh_get_hum(clr) * 1000000;
    s_hyst = div_s64(abs(s_thresh - s_clr),
    HDC3020_THRESH_FRACTION);
// Set new threshold
    thresh = reg_val;
// Try to set old hysteresis
    s_val = min(abs(s_hyst), HDC3020_MAX_HUM_MICRO);
    fallthrough;
    case IIO_EV_INFO_HYSTERESIS:
//
// Function hdc3020_thresh_get_hum returns relative
// humidity in percent scaled by HDC3020_THRESH_FRACTION.
// Scale by 1000000 to be able to subtract scaled
// hysteresis value.
//
    s_thresh = (s64)hdc3020_thresh_get_hum(thresh) * 1000000;
//
// Units of s_val are in micro percent, scale by
// HDC3020_THRESH_FRACTION to get same units as s_thresh.
//
    s_hyst = (s64)s_val * HDC3020_THRESH_FRACTION;
    s_clr = hdc3020_thresh_clr(s_thresh, s_hyst, dir);
    s_clr = max(s_clr, 0);
    s_clr = min(s_clr, HDC3020_MAX_HUM_MICRO);
    reg = reg_clr_wr;
    reg_val = hdc3020_thresh_set_hum(s_clr, clr);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return _hdc3020_write_thresh(data, reg, reg_val);
    }
    static int hdc3020_read_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    struct hdc3020_data *data = iio_priv(indio_dev);
    u16 reg_thresh, reg_clr;
    int thresh, clr, ret;
// Select threshold registers
    if (dir == IIO_EV_DIR_RISING) {
    reg_thresh = HDC3020_R_T_RH_THRESH_HIGH;
    reg_clr = HDC3020_R_T_RH_THRESH_HIGH_CLR;
    } else {
    reg_thresh = HDC3020_R_T_RH_THRESH_LOW;
    reg_clr = HDC3020_R_T_RH_THRESH_LOW_CLR;
    }
    guard(mutex)(&data.lock);
    ret = hdc3020_read_be16(data, reg_thresh);
    if (ret < 0)
    return ret;
    switch (chan.type) {
    case IIO_TEMP:
    thresh = hdc3020_thresh_get_temp(ret);
    switch (info) {
    case IIO_EV_INFO_VALUE:
// val = thresh * MILLI;
    break;
    case IIO_EV_INFO_HYSTERESIS:
    ret = hdc3020_read_be16(data, reg_clr);
    if (ret < 0)
    return ret;
    clr = hdc3020_thresh_get_temp(ret);
// val = abs(thresh - clr) * MILLI;
    break;
    default:
    return -EOPNOTSUPP;
    }
// val2 = HDC3020_THRESH_FRACTION;
    return IIO_VAL_FRACTIONAL;
    case IIO_HUMIDITYRELATIVE:
    thresh = hdc3020_thresh_get_hum(ret);
    switch (info) {
    case IIO_EV_INFO_VALUE:
// val = thresh * MILLI;
    break;
    case IIO_EV_INFO_HYSTERESIS:
    ret = hdc3020_read_be16(data, reg_clr);
    if (ret < 0)
    return ret;
    clr = hdc3020_thresh_get_hum(ret);
// val = abs(thresh - clr) * MILLI;
    break;
    default:
    return -EOPNOTSUPP;
    }
// val2 = HDC3020_THRESH_FRACTION;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t hdc3020_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct hdc3020_data *data;
    s64 time;
    int ret;
    data = iio_priv(indio_dev);
    ret = hdc3020_read_be16(data, HDC3020_R_STATUS);
    if (ret < 0)
    return IRQ_HANDLED;
    if (!(ret & (HDC3020_STATUS_T_HIGH_ALERT | HDC3020_STATUS_T_LOW_ALERT |
    HDC3020_STATUS_RH_HIGH_ALERT | HDC3020_STATUS_RH_LOW_ALERT)))
    return IRQ_NONE;
    time = iio_get_time_ns(indio_dev);
    if (ret & HDC3020_STATUS_T_HIGH_ALERT)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_NO_MOD,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    time);
    if (ret & HDC3020_STATUS_T_LOW_ALERT)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_TEMP, 0,
    IIO_NO_MOD,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    time);
    if (ret & HDC3020_STATUS_RH_HIGH_ALERT)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_HUMIDITYRELATIVE, 0,
    IIO_NO_MOD,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    time);
    if (ret & HDC3020_STATUS_RH_LOW_ALERT)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_HUMIDITYRELATIVE, 0,
    IIO_NO_MOD,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    time);
    return IRQ_HANDLED;
    }
    static const struct iio_info hdc3020_info = {
    .read_raw = hdc3020_read_raw,
    .write_raw = hdc3020_write_raw,
    .read_avail = hdc3020_read_available,
    .read_event_value = hdc3020_read_thresh,
    .write_event_value = hdc3020_write_thresh,
    };
#[no_mangle]
unsafe extern "C" fn hdc3020_power_off(data: *mut hdc3020_data) -> c_int {
    static int hdc3020_power_off(struct hdc3020_data *data)
    {
    hdc3020_exec_cmd(data, HDC3020_EXIT_AUTO);
    if (data.reset_gpio)
    gpiod_set_value_cansleep(data.reset_gpio, 1);
    return regulator_disable(data.vdd_supply);
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_power_on(data: *mut hdc3020_data) -> c_int {
    static int hdc3020_power_on(struct hdc3020_data *data)
    {
    int ret;
    ret = regulator_enable(data.vdd_supply);
    if (ret)
    return ret;
    fsleep(5000);
    if (data.reset_gpio) {
    gpiod_set_value_cansleep(data.reset_gpio, 0);
    fsleep(3000);
    }
    if (data.client.irq) {
//
// The alert output is activated by default upon power up,
// hardware reset, and soft reset. Clear the status register.
//
    ret = hdc3020_exec_cmd(data, HDC3020_S_STATUS);
    if (ret) {
    hdc3020_power_off(data);
    return ret;
    }
    }
    ret = hdc3020_exec_cmd(data, HDC3020_S_AUTO_10HZ_MOD0);
    if (ret)
    hdc3020_power_off(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_exit(data: *mut c_void) {
    static void hdc3020_exit(void *data)
    {
    hdc3020_power_off(data);
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_probe(client: *mut i2c_client) -> c_int {
    static int hdc3020_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct hdc3020_data *data;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    dev_set_drvdata(&client.dev, indio_dev);
    data = iio_priv(indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    crc8_populate_msb(hdc3020_crc8_table, HDC3020_CRC8_POLYNOMIAL);
    indio_dev.name = "hdc3020";
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &hdc3020_info;
    indio_dev.channels = hdc3020_channels;
    indio_dev.num_channels = ARRAY_SIZE(hdc3020_channels);
    data.vdd_supply = devm_regulator_get(&client.dev, "vdd");
    if (IS_ERR(data.vdd_supply))
    return dev_err_probe(&client.dev, PTR_ERR(data.vdd_supply),
    "Unable to get VDD regulator\n");
    data.reset_gpio = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(data.reset_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(data.reset_gpio),
    "Cannot get reset GPIO\n");
    ret = hdc3020_power_on(data);
    if (ret)
    return dev_err_probe(&client.dev, ret, "Power on failed\n");
    ret = devm_add_action_or_reset(&data.client.dev, hdc3020_exit, data);
    if (ret)
    return ret;
    if (client.irq) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), hdc3020_interrupt_handler,
    IRQF_ONESHOT, "hdc3020",
    indio_dev);
    if (ret)
    return ret;
    }
    ret = devm_iio_device_register(&data.client.dev, indio_dev);
    if (ret)
    return dev_err_probe(&client.dev, ret, "Failed to add device");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_suspend(dev: *mut device) -> c_int {
    static int hdc3020_suspend(struct device *dev)
    {
    struct iio_dev *iio_dev = dev_get_drvdata(dev);
    struct hdc3020_data *data = iio_priv(iio_dev);
    return hdc3020_power_off(data);
    }
#[no_mangle]
unsafe extern "C" fn hdc3020_resume(dev: *mut device) -> c_int {
    static int hdc3020_resume(struct device *dev)
    {
    struct iio_dev *iio_dev = dev_get_drvdata(dev);
    struct hdc3020_data *data = iio_priv(iio_dev);
    return hdc3020_power_on(data);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(hdc3020_pm_ops, hdc3020_suspend, hdc3020_resume);
    static const struct i2c_device_id hdc3020_id[] = {
    { .name = "hdc3020" },
    { .name = "hdc3021" },
    { .name = "hdc3022" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hdc3020_id);
    static const struct of_device_id hdc3020_dt_ids[] = {
    { .compatible = "ti,hdc3020" },
    { .compatible = "ti,hdc3021" },
    { .compatible = "ti,hdc3022" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hdc3020_dt_ids);
    static struct i2c_driver hdc3020_driver = {
    .driver = {
    .name = "hdc3020",
    .pm = pm_sleep_ptr(&hdc3020_pm_ops),
    .of_match_table = hdc3020_dt_ids,
    },
    .probe = hdc3020_probe,
    .id_table = hdc3020_id,
    };
    module_i2c_driver(hdc3020_driver);
    MODULE_AUTHOR("Javier Carrasco <javier.carrasco.cruz@gmail.com>");
    MODULE_AUTHOR("Li peiyu <579lpy@gmail.com>");
    MODULE_DESCRIPTION("TI HDC3020 humidity and temperature sensor driver");
    MODULE_LICENSE("GPL");
