//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/apds9960.c
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
// apds9960.c - Support for Avago APDS9960 gesture/RGB/ALS/proximity sensor
//
// Copyright (C) 2015, 2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//
// TODO: gesture + proximity calib offsets
//

pub const APDS9960_REG_RAM_START: c_uint = 0x00;
pub const APDS9960_REG_RAM_END: c_uint = 0x7f;
pub const APDS9960_REG_ENABLE: c_uint = 0x80;
pub const APDS9960_REG_ATIME: c_uint = 0x81;
pub const APDS9960_REG_WTIME: c_uint = 0x83;
pub const APDS9960_REG_AILTL: c_uint = 0x84;
pub const APDS9960_REG_AILTH: c_uint = 0x85;
pub const APDS9960_REG_AIHTL: c_uint = 0x86;
pub const APDS9960_REG_AIHTH: c_uint = 0x87;
pub const APDS9960_REG_PILT: c_uint = 0x89;
pub const APDS9960_REG_PIHT: c_uint = 0x8b;
pub const APDS9960_REG_PERS: c_uint = 0x8c;
pub const APDS9960_REG_CONFIG_1: c_uint = 0x8d;
pub const APDS9960_REG_PPULSE: c_uint = 0x8e;
pub const APDS9960_REG_CONTROL: c_uint = 0x8f;
pub const APDS9960_REG_CONTROL_AGAIN_MASK: c_uint = 0x03;
pub const APDS9960_REG_CONTROL_PGAIN_MASK: c_uint = 0x0c;
pub const APDS9960_REG_CONTROL_AGAIN_MASK_SHIFT: c_int = 0;
pub const APDS9960_REG_CONTROL_PGAIN_MASK_SHIFT: c_int = 2;
pub const APDS9960_REG_CONFIG_2: c_uint = 0x90;
pub const APDS9960_REG_ID: c_uint = 0x92;
pub const APDS9960_REG_STATUS: c_uint = 0x93;

pub const APDS9960_REG_PDATA: c_uint = 0x9c;
pub const APDS9960_REG_POFFSET_UR: c_uint = 0x9d;
pub const APDS9960_REG_POFFSET_DL: c_uint = 0x9e;
pub const APDS9960_REG_CONFIG_3: c_uint = 0x9f;
pub const APDS9960_REG_GPENTH: c_uint = 0xa0;
pub const APDS9960_REG_GEXTH: c_uint = 0xa1;
pub const APDS9960_REG_GCONF_1: c_uint = 0xa2;
pub const APDS9960_REG_GCONF_1_GFIFO_THRES_MASK: c_uint = 0xc0;
pub const APDS9960_REG_GCONF_1_GFIFO_THRES_MASK_SHIFT: c_int = 6;
pub const APDS9960_REG_GCONF_2: c_uint = 0xa3;
pub const APDS9960_REG_GCONF_2_GGAIN_MASK: c_uint = 0x60;
pub const APDS9960_REG_GCONF_2_GGAIN_MASK_SHIFT: c_int = 5;
pub const APDS9960_REG_GOFFSET_U: c_uint = 0xa4;
pub const APDS9960_REG_GOFFSET_D: c_uint = 0xa5;
pub const APDS9960_REG_GPULSE: c_uint = 0xa6;
pub const APDS9960_REG_GOFFSET_L: c_uint = 0xa7;
pub const APDS9960_REG_GOFFSET_R: c_uint = 0xa9;
pub const APDS9960_REG_GCONF_3: c_uint = 0xaa;
pub const APDS9960_REG_GCONF_4: c_uint = 0xab;
pub const APDS9960_REG_GFLVL: c_uint = 0xae;
pub const APDS9960_REG_GSTATUS: c_uint = 0xaf;
pub const APDS9960_REG_IFORCE: c_uint = 0xe4;
pub const APDS9960_REG_PICLEAR: c_uint = 0xe5;
pub const APDS9960_REG_CICLEAR: c_uint = 0xe6;
pub const APDS9960_REG_AICLEAR: c_uint = 0xe7;
pub const APDS9960_DEFAULT_PERS: c_uint = 0x33;
pub const APDS9960_DEFAULT_GPENTH: c_uint = 0x50;
pub const APDS9960_DEFAULT_GEXTH: c_uint = 0x40;
pub const APDS9960_MAX_PXS_THRES_VAL: c_int = 255;
pub const APDS9960_MAX_ALS_THRES_VAL: c_uint = 0xffff;
pub const APDS9960_MAX_INT_TIME_IN_US: c_int = 1000000;
    enum apds9960_als_channel_idx {
    IDX_ALS_CLEAR, IDX_ALS_RED, IDX_ALS_GREEN, IDX_ALS_BLUE,
    };
pub const APDS9960_REG_ALS_BASE: c_uint = 0x94;

    (APDS9960_REG_ALS_BASE + (IDX_ALS_##_colour * 2))
    enum apds9960_gesture_channel_idx {
    IDX_DIR_UP, IDX_DIR_DOWN, IDX_DIR_LEFT, IDX_DIR_RIGHT,
    };
pub const APDS9960_REG_GFIFO_BASE: c_uint = 0xfc;

    (APDS9960_REG_GFIFO_BASE + IDX_DIR_##_dir)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apds9960_data {
    pub client: *mut i2c_client,
    pub indio_dev: *mut iio_dev,
    pub lock: mutex,
// regmap fields
    pub regmap: *mut regmap,
    pub reg_int_als: *mut regmap_field,
    pub reg_int_ges: *mut regmap_field,
    pub reg_int_pxs: *mut regmap_field,
    pub reg_enable_als: *mut regmap_field,
    pub reg_enable_ges: *mut regmap_field,
    pub reg_enable_pxs: *mut regmap_field,
// state
    pub als_int: bool,
    pub pxs_int: bool,
    pub gesture_mode_running: c_int,
// gain values
    pub als_gain: c_int,
    pub pxs_gain: c_int,
// integration time value in us
    pub als_adc_int_us: c_int,
// gesture buffer
    pub /: *mut *mut u8 buffer[4]; / 4 8-bit channels,
// calibration value buffer
    pub calibbias: [c_int; 5],
}

    enum {
    APDS9960_CHAN_PROXIMITY,
    APDS9960_CHAN_GESTURE_UP,
    APDS9960_CHAN_GESTURE_DOWN,
    APDS9960_CHAN_GESTURE_LEFT,
    APDS9960_CHAN_GESTURE_RIGHT,
    };
    static const unsigned int apds9960_offset_regs[][2] = {
    [APDS9960_CHAN_PROXIMITY] = {APDS9960_REG_POFFSET_UR, APDS9960_REG_POFFSET_DL},
    [APDS9960_CHAN_GESTURE_UP] = {APDS9960_REG_GOFFSET_U, 0},
    [APDS9960_CHAN_GESTURE_DOWN] = {APDS9960_REG_GOFFSET_D, 0},
    [APDS9960_CHAN_GESTURE_LEFT] = {APDS9960_REG_GOFFSET_L, 0},
    [APDS9960_CHAN_GESTURE_RIGHT] = {APDS9960_REG_GOFFSET_R, 0},
    };
    static const struct reg_default apds9960_reg_defaults[] = {
// Default ALS integration time = 2.48ms
    { APDS9960_REG_ATIME, 0xff },
    };
    static const struct regmap_range apds9960_volatile_ranges[] = {
    regmap_reg_range(APDS9960_REG_STATUS,
    APDS9960_REG_PDATA),
    regmap_reg_range(APDS9960_REG_GFLVL,
    APDS9960_REG_GSTATUS),
    regmap_reg_range(APDS9960_REG_GFIFO_DIR(UP),
    APDS9960_REG_GFIFO_DIR(RIGHT)),
    regmap_reg_range(APDS9960_REG_IFORCE,
    APDS9960_REG_AICLEAR),
    };
    static const struct regmap_access_table apds9960_volatile_table = {
    .yes_ranges	= apds9960_volatile_ranges,
    .n_yes_ranges	= ARRAY_SIZE(apds9960_volatile_ranges),
    };
    static const struct regmap_range apds9960_precious_ranges[] = {
    regmap_reg_range(APDS9960_REG_RAM_START, APDS9960_REG_RAM_END),
    };
    static const struct regmap_access_table apds9960_precious_table = {
    .yes_ranges	= apds9960_precious_ranges,
    .n_yes_ranges	= ARRAY_SIZE(apds9960_precious_ranges),
    };
    static const struct regmap_range apds9960_readable_ranges[] = {
    regmap_reg_range(APDS9960_REG_ENABLE,
    APDS9960_REG_GSTATUS),
    regmap_reg_range(APDS9960_REG_GFIFO_DIR(UP),
    APDS9960_REG_GFIFO_DIR(RIGHT)),
    };
    static const struct regmap_access_table apds9960_readable_table = {
    .yes_ranges	= apds9960_readable_ranges,
    .n_yes_ranges	= ARRAY_SIZE(apds9960_readable_ranges),
    };
    static const struct regmap_range apds9960_writeable_ranges[] = {
    regmap_reg_range(APDS9960_REG_ENABLE, APDS9960_REG_CONFIG_2),
    regmap_reg_range(APDS9960_REG_POFFSET_UR, APDS9960_REG_GCONF_4),
    regmap_reg_range(APDS9960_REG_IFORCE, APDS9960_REG_AICLEAR),
    };
    static const struct regmap_access_table apds9960_writeable_table = {
    .yes_ranges	= apds9960_writeable_ranges,
    .n_yes_ranges	= ARRAY_SIZE(apds9960_writeable_ranges),
    };
    static const struct regmap_config apds9960_regmap_config = {
    .name = "apds9960_regmap",
    .reg_bits = 8,
    .val_bits = 8,
    .use_single_read = true,
    .use_single_write = true,
    .volatile_table = &apds9960_volatile_table,
    .precious_table = &apds9960_precious_table,
    .rd_table = &apds9960_readable_table,
    .wr_table = &apds9960_writeable_table,
    .reg_defaults = apds9960_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(apds9960_reg_defaults),
    .max_register = APDS9960_REG_GFIFO_DIR(RIGHT),
    .cache_type = REGCACHE_MAPLE,
    };
    static const struct iio_event_spec apds9960_pxs_event_spec[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_event_spec apds9960_als_event_spec[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };

    .type = IIO_PROXIMITY, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_CALIBBIAS), \
    .channel = _si + 1, \
    .scan_index = _si, \
    .indexed = 1, \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 8, \
    .storagebits = 8, \
    }, \
    }

    .type = IIO_INTENSITY, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_INT_TIME), \
    .channel2 = IIO_MOD_LIGHT_##_colour, \
    .address = APDS9960_REG_ALS_CHANNEL(_colour), \
    .modified = 1, \
    .scan_index = -1, \
    }
    static const unsigned long apds9960_scan_masks[] = {0xf, 0};
    static const struct iio_chan_spec apds9960_channels[] = {
    {
    .type = IIO_PROXIMITY,
    .address = APDS9960_REG_PDATA,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_CALIBBIAS),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),
    .channel = 0,
    .indexed = 0,
    .scan_index = -1,
    .event_spec = apds9960_pxs_event_spec,
    .num_event_specs = ARRAY_SIZE(apds9960_pxs_event_spec),
    },
// Gesture Sensor
    APDS9960_GESTURE_CHANNEL(UP, 0),
    APDS9960_GESTURE_CHANNEL(DOWN, 1),
    APDS9960_GESTURE_CHANNEL(LEFT, 2),
    APDS9960_GESTURE_CHANNEL(RIGHT, 3),
// ALS
    {
    .type = IIO_INTENSITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_INT_TIME),
    .channel2 = IIO_MOD_LIGHT_CLEAR,
    .address = APDS9960_REG_ALS_CHANNEL(CLEAR),
    .modified = 1,
    .scan_index = -1,
    .event_spec = apds9960_als_event_spec,
    .num_event_specs = ARRAY_SIZE(apds9960_als_event_spec),
    },
// RGB Sensor
    APDS9960_INTENSITY_CHANNEL(RED),
    APDS9960_INTENSITY_CHANNEL(GREEN),
    APDS9960_INTENSITY_CHANNEL(BLUE),
    };
    static int apds9960_set_calibbias(struct apds9960_data *data,
    struct iio_chan_spec const *chan, int calibbias)
    {
    int ret, i;
    if (calibbias < S8_MIN || calibbias > S8_MAX)
    return -EINVAL;
    guard(mutex)(&data.lock);
    for (i = 0; i < 2; i++) {
    if (apds9960_offset_regs[chan.channel][i] == 0)
    break;
    ret = regmap_write(data.regmap, apds9960_offset_regs[chan.channel][i], calibbias);
    if (ret < 0)
    return ret;
    }
    data.calibbias[chan.channel] = calibbias;
    return 0;
    }
// integration time in us
    static const int apds9960_int_time[][2] = {
    { 28000, 246},
    {100000, 219},
    {200000, 182},
    {700000,   0}
    };
// gain mapping
    static const int apds9960_pxs_gain_map[] = {1, 2, 4, 8};
    static const int apds9960_als_gain_map[] = {1, 4, 16, 64};
    static IIO_CONST_ATTR(proximity_scale_available, "1 2 4 8");
    static IIO_CONST_ATTR(intensity_scale_available, "1 4 16 64");
    static IIO_CONST_ATTR_INT_TIME_AVAIL("0.028 0.1 0.2 0.7");
    static struct attribute *apds9960_attributes[] = {
    &iio_const_attr_proximity_scale_available.dev_attr.attr,
    &iio_const_attr_intensity_scale_available.dev_attr.attr,
    &iio_const_attr_integration_time_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group apds9960_attribute_group = {
    .attrs = apds9960_attributes,
    };
    static const struct reg_field apds9960_reg_field_int_als =
    REG_FIELD(APDS9960_REG_ENABLE, 4, 4);
    static const struct reg_field apds9960_reg_field_int_ges =
    REG_FIELD(APDS9960_REG_GCONF_4, 1, 1);
    static const struct reg_field apds9960_reg_field_int_pxs =
    REG_FIELD(APDS9960_REG_ENABLE, 5, 5);
    static const struct reg_field apds9960_reg_field_enable_als =
    REG_FIELD(APDS9960_REG_ENABLE, 1, 1);
    static const struct reg_field apds9960_reg_field_enable_ges =
    REG_FIELD(APDS9960_REG_ENABLE, 6, 6);
    static const struct reg_field apds9960_reg_field_enable_pxs =
    REG_FIELD(APDS9960_REG_ENABLE, 2, 2);
#[no_mangle]
unsafe extern "C" fn apds9960_set_it_time(data: *mut apds9960_data, val2: c_int) -> c_int {
    static int apds9960_set_it_time(struct apds9960_data *data, int val2)
    {
    let mut ret: c_int = -EINVAL;
    int idx;
    for (idx = 0; idx < ARRAY_SIZE(apds9960_int_time); idx++) {
    if (apds9960_int_time[idx][0] == val2) {
    mutex_lock(&data.lock);
    ret = regmap_write(data.regmap, APDS9960_REG_ATIME,
    apds9960_int_time[idx][1]);
    if (!ret)
    data.als_adc_int_us = val2;
    mutex_unlock(&data.lock);
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_set_pxs_gain(data: *mut apds9960_data, val: c_int) -> c_int {
    static int apds9960_set_pxs_gain(struct apds9960_data *data, int val)
    {
    let mut ret: c_int = -EINVAL;
    int idx;
    for (idx = 0; idx < ARRAY_SIZE(apds9960_pxs_gain_map); idx++) {
    if (apds9960_pxs_gain_map[idx] == val) {
// pxs + gesture gains are mirrored
    mutex_lock(&data.lock);
    ret = regmap_update_bits(data.regmap,
    APDS9960_REG_CONTROL,
    APDS9960_REG_CONTROL_PGAIN_MASK,
    idx << APDS9960_REG_CONTROL_PGAIN_MASK_SHIFT);
    if (ret) {
    mutex_unlock(&data.lock);
    break;
    }
    ret = regmap_update_bits(data.regmap,
    APDS9960_REG_GCONF_2,
    APDS9960_REG_GCONF_2_GGAIN_MASK,
    idx << APDS9960_REG_GCONF_2_GGAIN_MASK_SHIFT);
    if (!ret)
    data.pxs_gain = idx;
    mutex_unlock(&data.lock);
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_set_als_gain(data: *mut apds9960_data, val: c_int) -> c_int {
    static int apds9960_set_als_gain(struct apds9960_data *data, int val)
    {
    let mut ret: c_int = -EINVAL;
    int idx;
    for (idx = 0; idx < ARRAY_SIZE(apds9960_als_gain_map); idx++) {
    if (apds9960_als_gain_map[idx] == val) {
    mutex_lock(&data.lock);
    ret = regmap_update_bits(data.regmap,
    APDS9960_REG_CONTROL,
    APDS9960_REG_CONTROL_AGAIN_MASK, idx);
    if (!ret)
    data.als_gain = idx;
    mutex_unlock(&data.lock);
    break;
    }
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn apds9960_set_power_state(data: *mut apds9960_data, on: bool) -> c_int {
    static int apds9960_set_power_state(struct apds9960_data *data, bool on)
    {
    struct device *dev = &data.client.dev;
    let mut ret: c_int = 0;
    mutex_lock(&data.lock);
    if (on) {
    int suspended;
    suspended = pm_runtime_suspended(dev);
    ret = pm_runtime_get_sync(dev);
// Allow one integration cycle before allowing a reading
    if (suspended)
    usleep_range(data.als_adc_int_us,
    APDS9960_MAX_INT_TIME_IN_US);
    } else {
    ret = pm_runtime_put_autosuspend(dev);
    }
    mutex_unlock(&data.lock);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn apds9960_set_power_state(data: *mut apds9960_data, on: bool) -> c_int {
    static int apds9960_set_power_state(struct apds9960_data *data, bool on)
    {
    return 0;
    }

    static int apds9960_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    __le16 buf;
    let mut ret: c_int = -EINVAL;
    if (data.gesture_mode_running)
    return -EBUSY;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    apds9960_set_power_state(data, true);
    switch (chan.type) {
    case IIO_PROXIMITY:
    ret = regmap_read(data.regmap, chan.address, val);
    if (!ret)
    ret = IIO_VAL_INT;
    break;
    case IIO_INTENSITY:
    ret = regmap_bulk_read(data.regmap, chan.address,
    &buf, 2);
    if (!ret) {
    ret = IIO_VAL_INT;
// val = le16_to_cpu(buf);
    }
    break;
    default:
    ret = -EINVAL;
    }
    apds9960_set_power_state(data, false);
    break;
    case IIO_CHAN_INFO_INT_TIME:
// RGB + ALS sensors only have integration time
    mutex_lock(&data.lock);
    switch (chan.type) {
    case IIO_INTENSITY:
// val = 0;
// val2 = data->als_adc_int_us;
    ret = IIO_VAL_INT_PLUS_MICRO;
    break;
    default:
    ret = -EINVAL;
    }
    mutex_unlock(&data.lock);
    break;
    case IIO_CHAN_INFO_SCALE:
    mutex_lock(&data.lock);
    switch (chan.type) {
    case IIO_PROXIMITY:
// val = apds9960_pxs_gain_map[data->pxs_gain];
    ret = IIO_VAL_INT;
    break;
    case IIO_INTENSITY:
// val = apds9960_als_gain_map[data->als_gain];
    ret = IIO_VAL_INT;
    break;
    default:
    ret = -EINVAL;
    }
    mutex_unlock(&data.lock);
    break;
    case IIO_CHAN_INFO_CALIBBIAS:
    mutex_lock(&data.lock);
// val = data->calibbias[chan->channel];
    ret = IIO_VAL_INT;
    mutex_unlock(&data.lock);
    break;
    }
    return ret;
    };
    static int apds9960_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
// RGB + ALS sensors only have int time
    switch (chan.type) {
    case IIO_INTENSITY:
    if (val != 0)
    return -EINVAL;
    return apds9960_set_it_time(data, val2);
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    if (val2 != 0)
    return -EINVAL;
    switch (chan.type) {
    case IIO_PROXIMITY:
    return apds9960_set_pxs_gain(data, val);
    case IIO_INTENSITY:
    return apds9960_set_als_gain(data, val);
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_CALIBBIAS:
    if (val2 != 0)
    return -EINVAL;
    return apds9960_set_calibbias(data, chan, val);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static inline int apds9960_get_thres_reg(const struct iio_chan_spec *chan,
    enum iio_event_direction dir,
    u8 *reg)
    {
    switch (dir) {
    case IIO_EV_DIR_RISING:
    switch (chan.type) {
    case IIO_PROXIMITY:
// reg = APDS9960_REG_PIHT;
    break;
    case IIO_INTENSITY:
// reg = APDS9960_REG_AIHTL;
    break;
    default:
    return -EINVAL;
    }
    break;
    case IIO_EV_DIR_FALLING:
    switch (chan.type) {
    case IIO_PROXIMITY:
// reg = APDS9960_REG_PILT;
    break;
    case IIO_INTENSITY:
// reg = APDS9960_REG_AILTL;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int apds9960_read_event(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    u8 reg;
    __le16 buf;
    let mut ret: c_int = 0;
    struct apds9960_data *data = iio_priv(indio_dev);
    if (info != IIO_EV_INFO_VALUE)
    return -EINVAL;
    ret = apds9960_get_thres_reg(chan, dir, &reg);
    if (ret < 0)
    return ret;
    if (chan.type == IIO_PROXIMITY) {
    ret = regmap_read(data.regmap, reg, val);
    if (ret < 0)
    return ret;
    } else if (chan.type == IIO_INTENSITY) {
    ret = regmap_bulk_read(data.regmap, reg, &buf, 2);
    if (ret < 0)
    return ret;
// val = le16_to_cpu(buf);
    } else
    return -EINVAL;
// val2 = 0;
    return IIO_VAL_INT;
    }
    static int apds9960_write_event(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    u8 reg;
    __le16 buf;
    let mut ret: c_int = 0;
    struct apds9960_data *data = iio_priv(indio_dev);
    if (info != IIO_EV_INFO_VALUE)
    return -EINVAL;
    ret = apds9960_get_thres_reg(chan, dir, &reg);
    if (ret < 0)
    return ret;
    if (chan.type == IIO_PROXIMITY) {
    if (val < 0 || val > APDS9960_MAX_PXS_THRES_VAL)
    return -EINVAL;
    ret = regmap_write(data.regmap, reg, val);
    if (ret < 0)
    return ret;
    } else if (chan.type == IIO_INTENSITY) {
    if (val < 0 || val > APDS9960_MAX_ALS_THRES_VAL)
    return -EINVAL;
    buf = cpu_to_le16(val);
    ret = regmap_bulk_write(data.regmap, reg, &buf, 2);
    if (ret < 0)
    return ret;
    } else
    return -EINVAL;
    return 0;
    }
    static int apds9960_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    switch (chan.type) {
    case IIO_PROXIMITY:
    return data.pxs_int;
    case IIO_INTENSITY:
    return data.als_int;
    default:
    return -EINVAL;
    }
    }
    static int apds9960_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    int ret;
    switch (chan.type) {
    case IIO_PROXIMITY:
    if (data.pxs_int == state)
    return -EINVAL;
    ret = regmap_field_write(data.reg_int_pxs, state);
    if (ret)
    return ret;
    data.pxs_int = state;
    apds9960_set_power_state(data, state);
    break;
    case IIO_INTENSITY:
    if (data.als_int == state)
    return -EINVAL;
    ret = regmap_field_write(data.reg_int_als, state);
    if (ret)
    return ret;
    data.als_int = state;
    apds9960_set_power_state(data, state);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct iio_info apds9960_info = {
    .attrs = &apds9960_attribute_group,
    .read_raw = apds9960_read_raw,
    .write_raw = apds9960_write_raw,
    .read_event_value = apds9960_read_event,
    .write_event_value = apds9960_write_event,
    .read_event_config = apds9960_read_event_config,
    .write_event_config = apds9960_write_event_config,
    };
#[no_mangle]
pub unsafe extern "C" fn apds9660_fifo_is_empty(data: *mut apds9960_data) -> c_int {
    static inline int apds9660_fifo_is_empty(struct apds9960_data *data)
    {
    int cnt;
    int ret;
    ret = regmap_read(data.regmap, APDS9960_REG_GFLVL, &cnt);
    if (ret)
    return ret;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_read_gesture_fifo(data: *mut apds9960_data) {
    static void apds9960_read_gesture_fifo(struct apds9960_data *data)
    {
    int ret, cnt = 0;
    mutex_lock(&data.lock);
    data.gesture_mode_running = 1;
    while (cnt || (cnt = apds9660_fifo_is_empty(data) > 0)) {
    ret = regmap_bulk_read(data.regmap, APDS9960_REG_GFIFO_BASE,
    &data.buffer, 4);
    if (ret)
    goto err_read;
    iio_push_to_buffers(data.indio_dev, data.buffer);
    cnt--;
    }
    err_read:
    data.gesture_mode_running = 0;
    mutex_unlock(&data.lock);
    }
#[no_mangle]
unsafe extern "C" fn apds9960_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t apds9960_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct apds9960_data *data = iio_priv(indio_dev);
    int ret, status;
    ret = regmap_read(data.regmap, APDS9960_REG_STATUS, &status);
    if (ret < 0) {
    dev_err(&data.client.dev, "irq status reg read failed\n");
    return IRQ_HANDLED;
    }
    if ((status & APDS9960_REG_STATUS_ALS_INT) && data.als_int) {
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(IIO_INTENSITY, 0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(indio_dev));
    regmap_write(data.regmap, APDS9960_REG_CICLEAR, 1);
    }
    if ((status & APDS9960_REG_STATUS_PS_INT) && data.pxs_int) {
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(IIO_PROXIMITY, 0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(indio_dev));
    regmap_write(data.regmap, APDS9960_REG_PICLEAR, 1);
    }
    if (status & APDS9960_REG_STATUS_GINT)
    apds9960_read_gesture_fifo(data);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_set_powermode(data: *mut apds9960_data, state: bool) -> c_int {
    static int apds9960_set_powermode(struct apds9960_data *data, bool state)
    {
    return regmap_update_bits(data.regmap, APDS9960_REG_ENABLE, 1, state);
    }
#[no_mangle]
unsafe extern "C" fn apds9960_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int apds9960_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    int ret;
    ret = regmap_field_write(data.reg_int_ges, 1);
    if (ret)
    return ret;
    ret = regmap_field_write(data.reg_enable_ges, 1);
    if (ret)
    return ret;
    pm_runtime_get_sync(&data.client.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int apds9960_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct apds9960_data *data = iio_priv(indio_dev);
    int ret;
    ret = regmap_field_write(data.reg_enable_ges, 0);
    if (ret)
    return ret;
    ret = regmap_field_write(data.reg_int_ges, 0);
    if (ret)
    return ret;
    pm_runtime_put_autosuspend(&data.client.dev);
    return 0;
    }
    static const struct iio_buffer_setup_ops apds9960_buffer_setup_ops = {
    .postenable = apds9960_buffer_postenable,
    .predisable = apds9960_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn apds9960_regfield_init(data: *mut apds9960_data) -> c_int {
    static int apds9960_regfield_init(struct apds9960_data *data)
    {
    struct device *dev = &data.client.dev;
    struct regmap *regmap = data.regmap;
    data.reg_int_als = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_int_als);
    if (IS_ERR(data.reg_int_als)) {
    dev_err(dev, "INT ALS reg field init failed\n");
    return PTR_ERR(data.reg_int_als);
    }
    data.reg_int_ges = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_int_ges);
    if (IS_ERR(data.reg_int_ges)) {
    dev_err(dev, "INT gesture reg field init failed\n");
    return PTR_ERR(data.reg_int_ges);
    }
    data.reg_int_pxs = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_int_pxs);
    if (IS_ERR(data.reg_int_pxs)) {
    dev_err(dev, "INT pxs reg field init failed\n");
    return PTR_ERR(data.reg_int_pxs);
    }
    data.reg_enable_als = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_enable_als);
    if (IS_ERR(data.reg_enable_als)) {
    dev_err(dev, "Enable ALS reg field init failed\n");
    return PTR_ERR(data.reg_enable_als);
    }
    data.reg_enable_ges = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_enable_ges);
    if (IS_ERR(data.reg_enable_ges)) {
    dev_err(dev, "Enable gesture reg field init failed\n");
    return PTR_ERR(data.reg_enable_ges);
    }
    data.reg_enable_pxs = devm_regmap_field_alloc(dev, regmap,
    apds9960_reg_field_enable_pxs);
    if (IS_ERR(data.reg_enable_pxs)) {
    dev_err(dev, "Enable PXS reg field init failed\n");
    return PTR_ERR(data.reg_enable_pxs);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_chip_init(data: *mut apds9960_data) -> c_int {
    static int apds9960_chip_init(struct apds9960_data *data)
    {
    int ret;
// Default IT for ALS of 28 ms
    ret = apds9960_set_it_time(data, 28000);
    if (ret)
    return ret;
// Ensure gesture interrupt is OFF
    ret = regmap_field_write(data.reg_int_ges, 0);
    if (ret)
    return ret;
// Disable gesture sensor, since polling is useless from user-space
    ret = regmap_field_write(data.reg_enable_ges, 0);
    if (ret)
    return ret;
// Ensure proximity interrupt is OFF
    ret = regmap_field_write(data.reg_int_pxs, 0);
    if (ret)
    return ret;
// Enable proximity sensor for polling
    ret = regmap_field_write(data.reg_enable_pxs, 1);
    if (ret)
    return ret;
// Ensure ALS interrupt is OFF
    ret = regmap_field_write(data.reg_int_als, 0);
    if (ret)
    return ret;
// Enable ALS sensor for polling
    ret = regmap_field_write(data.reg_enable_als, 1);
    if (ret)
    return ret;
//
// When enabled trigger an interrupt after 3 readings
// outside threshold for ALS + PXS
//
    ret = regmap_write(data.regmap, APDS9960_REG_PERS,
    APDS9960_DEFAULT_PERS);
    if (ret)
    return ret;
//
// Wait for 4 event outside gesture threshold to prevent interrupt
// flooding.
//
    ret = regmap_update_bits(data.regmap, APDS9960_REG_GCONF_1,
    APDS9960_REG_GCONF_1_GFIFO_THRES_MASK,
    BIT(0) << APDS9960_REG_GCONF_1_GFIFO_THRES_MASK_SHIFT);
    if (ret)
    return ret;
// Default ENTER and EXIT thresholds for the GESTURE engine.
    ret = regmap_write(data.regmap, APDS9960_REG_GPENTH,
    APDS9960_DEFAULT_GPENTH);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, APDS9960_REG_GEXTH,
    APDS9960_DEFAULT_GEXTH);
    if (ret)
    return ret;
    return apds9960_set_powermode(data, 1);
    }
#[no_mangle]
unsafe extern "C" fn apds9960_probe(client: *mut i2c_client) -> c_int {
    static int apds9960_probe(struct i2c_client *client)
    {
    struct apds9960_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    indio_dev.info = &apds9960_info;
    indio_dev.name = APDS9960_DRV_NAME;
    indio_dev.channels = apds9960_channels;
    indio_dev.num_channels = ARRAY_SIZE(apds9960_channels);
    indio_dev.available_scan_masks = apds9960_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = devm_iio_kfifo_buffer_setup(&client.dev, indio_dev,
    &apds9960_buffer_setup_ops);
    if (ret)
    return ret;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &apds9960_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "regmap initialization failed.\n");
    return PTR_ERR(data.regmap);
    }
    data.client = client;
    data.indio_dev = indio_dev;
    mutex_init(&data.lock);
    ret = pm_runtime_set_active(&client.dev);
    if (ret)
    goto error_power_down;
    pm_runtime_enable(&client.dev);
    pm_runtime_set_autosuspend_delay(&client.dev, 5000);
    pm_runtime_use_autosuspend(&client.dev);
    apds9960_set_power_state(data, true);
    ret = apds9960_regfield_init(data);
    if (ret)
    goto error_power_down;
    ret = apds9960_chip_init(data);
    if (ret)
    goto error_power_down;
    if (client.irq <= 0) {
    dev_err(&client.dev, "no valid irq defined\n");
    ret = -EINVAL;
    goto error_power_down;
    }
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), apds9960_interrupt_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "apds9960_event",
    indio_dev);
    if (ret)
    goto error_power_down;
    ret = iio_device_register(indio_dev);
    if (ret)
    goto error_power_down;
    apds9960_set_power_state(data, false);
    return 0;
    error_power_down:
    apds9960_set_power_state(data, false);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apds9960_remove(client: *mut i2c_client) {
    static void apds9960_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct apds9960_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    apds9960_set_powermode(data, 0);
    }

#[no_mangle]
unsafe extern "C" fn apds9960_runtime_suspend(dev: *mut device) -> c_int {
    static int apds9960_runtime_suspend(struct device *dev)
    {
    struct apds9960_data *data =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return apds9960_set_powermode(data, 0);
    }
#[no_mangle]
unsafe extern "C" fn apds9960_runtime_resume(dev: *mut device) -> c_int {
    static int apds9960_runtime_resume(struct device *dev)
    {
    struct apds9960_data *data =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    return apds9960_set_powermode(data, 1);
    }

    static const struct dev_pm_ops apds9960_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    SET_RUNTIME_PM_OPS(apds9960_runtime_suspend,
    apds9960_runtime_resume, core::ptr::null_mut())
    };
    static const struct i2c_device_id apds9960_id[] = {
    { .name = "apds9960" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, apds9960_id);
    static const struct acpi_device_id apds9960_acpi_match[] = {
    { "MSHW0184" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, apds9960_acpi_match);
    static const struct of_device_id apds9960_of_match[] = {
    { .compatible = "avago,apds9960" },
    { }
    };
    MODULE_DEVICE_TABLE(of, apds9960_of_match);
    static struct i2c_driver apds9960_driver = {
    .driver = {
    .name	= APDS9960_DRV_NAME,
    .of_match_table = apds9960_of_match,
    .pm	= &apds9960_pm_ops,
    .acpi_match_table = apds9960_acpi_match,
    },
    .probe		= apds9960_probe,
    .remove		= apds9960_remove,
    .id_table	= apds9960_id,
    };
    module_i2c_driver(apds9960_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("APDS9960 Gesture/RGB/ALS/Proximity sensor");
    MODULE_LICENSE("GPL");
