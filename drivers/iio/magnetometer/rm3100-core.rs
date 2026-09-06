//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/rm3100-core.c
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
// PNI RM3100 3-axis geomagnetic sensor driver core.
//
// Copyright (C) 2018 Song Qiang <songqiang1304521@gmail.com>
//
// User Manual available at
// <https://www.pnicorp.com/download/rm3100-user-manual/>
//
// TODO: event generation, pm.
//

// Cycle Count Registers.
pub const RM3100_REG_CC_X: c_uint = 0x05;
pub const RM3100_REG_CC_Y: c_uint = 0x07;
pub const RM3100_REG_CC_Z: c_uint = 0x09;
// Poll Measurement Mode register.
pub const RM3100_REG_POLL: c_uint = 0x00;

// Continuous Measurement Mode register.
pub const RM3100_REG_CMM: c_uint = 0x01;

// TiMe Rate Configuration register.
pub const RM3100_REG_TMRC: c_uint = 0x0B;
pub const RM3100_TMRC_OFFSET: c_uint = 0x92;
// Result Status register.
pub const RM3100_REG_STATUS: c_uint = 0x34;

// Measurement result registers.
pub const RM3100_REG_MX2: c_uint = 0x24;
pub const RM3100_REG_MY2: c_uint = 0x27;
pub const RM3100_REG_MZ2: c_uint = 0x2a;

//
// This is computed by hand, is the sum of channel storage bits and padding
// bits, which is 4+4+4+12=24 in here.
//
pub const RM3100_SCAN_BYTES: c_int = 24;
pub const RM3100_CMM_AXIS_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm3100_data {
    pub regmap: *mut regmap,
    pub measuring_done: completion,
    pub use_interrupt: bool,
    pub conversion_time: c_int,
    pub scale: c_int,
// Ensure naturally aligned timestamp
    pub __aligned(8): u8 buffer[RM3100_SCAN_BYTES],
    pub drdy_trig: *mut iio_trigger,
//
// This lock is for protecting the consistency of series of i2c
// operations, that is, to make sure a measurement process will
// not be interrupted by a set frequency operation, which should
// be taken where a series of i2c operation starts, released where
// the operation ends.
//
    pub lock: mutex,
}

    static const struct regmap_range rm3100_readable_ranges[] = {
    regmap_reg_range(RM3100_R_REG_START, RM3100_R_REG_END),
    };
    const struct regmap_access_table rm3100_readable_table = {
    .yes_ranges = rm3100_readable_ranges,
    .n_yes_ranges = ARRAY_SIZE(rm3100_readable_ranges),
    };
    EXPORT_SYMBOL_NS_GPL(rm3100_readable_table, "IIO_RM3100");
    static const struct regmap_range rm3100_writable_ranges[] = {
    regmap_reg_range(RM3100_W_REG_START, RM3100_W_REG_END),
    };
    const struct regmap_access_table rm3100_writable_table = {
    .yes_ranges = rm3100_writable_ranges,
    .n_yes_ranges = ARRAY_SIZE(rm3100_writable_ranges),
    };
    EXPORT_SYMBOL_NS_GPL(rm3100_writable_table, "IIO_RM3100");
    static const struct regmap_range rm3100_volatile_ranges[] = {
    regmap_reg_range(RM3100_V_REG_START, RM3100_V_REG_END),
    };
    const struct regmap_access_table rm3100_volatile_table = {
    .yes_ranges = rm3100_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(rm3100_volatile_ranges),
    };
    EXPORT_SYMBOL_NS_GPL(rm3100_volatile_table, "IIO_RM3100");
#[no_mangle]
unsafe extern "C" fn rm3100_thread_fn(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t rm3100_thread_fn(int irq, void *d)
    {
    struct iio_dev *indio_dev = d;
    struct rm3100_data *data = iio_priv(indio_dev);
//
// Write operation to any register or read operation
// to first byte of results will clear the interrupt.
//
    regmap_write(data.regmap, RM3100_REG_POLL, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rm3100_irq_handler(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t rm3100_irq_handler(int irq, void *d)
    {
    struct iio_dev *indio_dev = d;
    struct rm3100_data *data = iio_priv(indio_dev);
    if (!iio_buffer_enabled(indio_dev))
    complete(&data.measuring_done);
    else
    iio_trigger_poll(data.drdy_trig);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn rm3100_wait_measurement(data: *mut rm3100_data) -> c_int {
    static int rm3100_wait_measurement(struct rm3100_data *data)
    {
    struct regmap *regmap = data.regmap;
    unsigned int val;
    let mut tries: c_int = 20;
    int ret;
//
// A read cycle of 400kbits i2c bus is about 20us, plus the time
// used for scheduling, a read cycle of fast mode of this device
// can reach 1.7ms, it may be possible for data to arrive just
// after we check the RM3100_REG_STATUS. In this case, irq_handler is
// called before measuring_done is reinitialized, it will wait
// forever for data that has already been ready.
// Reinitialize measuring_done before looking up makes sure we
// will always capture interrupt no matter when it happens.
//
    if (data.use_interrupt)
    reinit_completion(&data.measuring_done);
    ret = regmap_read(regmap, RM3100_REG_STATUS, &val);
    if (ret < 0)
    return ret;
    if ((val & RM3100_STATUS_DRDY) != RM3100_STATUS_DRDY) {
    if (data.use_interrupt) {
    ret = wait_for_completion_timeout(&data.measuring_done,
    msecs_to_jiffies(data.conversion_time));
    if (!ret)
    return -ETIMEDOUT;
    } else {
    do {
    usleep_range(1000, 5000);
    ret = regmap_read(regmap, RM3100_REG_STATUS,
    &val);
    if (ret < 0)
    return ret;
    if (val & RM3100_STATUS_DRDY)
    break;
    } while (--tries);
    if (!tries)
    return -ETIMEDOUT;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rm3100_read_mag(data: *mut rm3100_data, idx: c_int, val: *mut c_int) -> c_int {
    static int rm3100_read_mag(struct rm3100_data *data, int idx, int *val)
    {
    struct regmap *regmap = data.regmap;
    u8 buffer[3];
    int ret;
    guard(mutex)(&data.lock);
    ret = regmap_write(regmap, RM3100_REG_POLL, BIT(4 + idx));
    if (ret < 0)
    return ret;
    ret = rm3100_wait_measurement(data);
    if (ret < 0)
    return ret;
    ret = regmap_bulk_read(regmap, RM3100_REG_MX2 + 3 * idx, buffer, 3);
    if (ret < 0)
    return ret;
// val = sign_extend32(get_unaligned_be24(&buffer[0]), 23);
    return IIO_VAL_INT;
    }

    {								\
    .type = IIO_MAGN,					\
    .modified = 1,						\
    .channel2 = IIO_MOD_##axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),			\
    .scan_index = idx,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = 24,					\
    .storagebits = 32,				\
    .shift = 8,					\
    .endianness = IIO_BE,				\
    },							\
    }
    static const struct iio_chan_spec rm3100_channels[] = {
    RM3100_CHANNEL(X, 0),
    RM3100_CHANNEL(Y, 1),
    RM3100_CHANNEL(Z, 2),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static IIO_CONST_ATTR_SAMP_FREQ_AVAIL(
    "600 300 150 75 37 18 9 4.5 2.3 1.2 0.6 0.3 0.015 0.075"
    );
    static struct attribute *rm3100_attributes[] = {
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group rm3100_attribute_group = {
    .attrs = rm3100_attributes,
    };
pub const RM3100_SAMP_NUM: c_int = 14;
//
// Frequency : rm3100_samp_rates[][0].rm3100_samp_rates[][1]Hz.
// Time between reading: rm3100_sam_rates[][2]ms.
// The first one is actually 1.7ms.
//
    static const int rm3100_samp_rates[RM3100_SAMP_NUM][3] = {
    {600, 0, 2}, {300, 0, 3}, {150, 0, 7}, {75, 0, 13}, {37, 0, 27},
    {18, 0, 55}, {9, 0, 110}, {4, 500000, 220}, {2, 300000, 440},
    {1, 200000, 800}, {0, 600000, 1600}, {0, 300000, 3300},
    {0, 15000, 6700},  {0, 75000, 13000}
    };
#[no_mangle]
unsafe extern "C" fn rm3100_get_samp_freq(data: *mut rm3100_data, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int rm3100_get_samp_freq(struct rm3100_data *data, int *val, int *val2)
    {
    unsigned int tmp;
    int ret;
    guard(mutex)(&data.lock);
    ret = regmap_read(data.regmap, RM3100_REG_TMRC, &tmp);
    if (ret < 0)
    return ret;
// val = rm3100_samp_rates[tmp - RM3100_TMRC_OFFSET][0];
// val2 = rm3100_samp_rates[tmp - RM3100_TMRC_OFFSET][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
#[no_mangle]
unsafe extern "C" fn rm3100_set_cycle_count(data: *mut rm3100_data, val: c_int) -> c_int {
    static int rm3100_set_cycle_count(struct rm3100_data *data, int val)
    {
    int ret;
    u8 i;
    for (i = 0; i < 3; i++) {
    ret = regmap_write(data.regmap, RM3100_REG_CC_X + 2 * i, val);
    if (ret < 0)
    return ret;
    }
//
// The scale of this sensor depends on the cycle count value, these
// three values are corresponding to the cycle count value 50, 100,
// 200. scale = output / gain * 10^4.
//
    switch (val) {
    case 50:
    data.scale = 500;
    break;
    case 100:
    data.scale = 263;
    break;
//
// case 200:
// This function will never be called by users' code, so here we
// assume that it will never get a wrong parameter.
//
    default:
    data.scale = 133;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rm3100_set_samp_freq(indio_dev: *mut iio_dev, val: c_int, val2: c_int) -> c_int {
    static int rm3100_set_samp_freq(struct iio_dev *indio_dev, int val, int val2)
    {
    struct rm3100_data *data = iio_priv(indio_dev);
    struct regmap *regmap = data.regmap;
    unsigned int cycle_count;
    int ret;
    int i;
    guard(mutex)(&data.lock);
// All cycle count registers use the same value.
    ret = regmap_read(regmap, RM3100_REG_CC_X, &cycle_count);
    if (ret < 0)
    return ret;
    for (i = 0; i < RM3100_SAMP_NUM; i++) {
    if (val == rm3100_samp_rates[i][0] &&
    val2 == rm3100_samp_rates[i][1])
    break;
    }
    if (i == RM3100_SAMP_NUM)
    return -EINVAL;
    ret = regmap_write(regmap, RM3100_REG_TMRC, i + RM3100_TMRC_OFFSET);
    if (ret < 0)
    return ret;
// Checking if cycle count registers need changing.
    if (val == 600 && cycle_count == 200) {
    ret = rm3100_set_cycle_count(data, 100);
    if (ret < 0)
    return ret;
    } else if (val != 600 && cycle_count == 100) {
    ret = rm3100_set_cycle_count(data, 200);
    if (ret < 0)
    return ret;
    }
    if (iio_buffer_enabled(indio_dev)) {
// Writing TMRC registers requires CMM reset.
    ret = regmap_write(regmap, RM3100_REG_CMM, 0);
    if (ret < 0)
    return ret;
    ret = regmap_write(data.regmap, RM3100_REG_CMM,
    (*indio_dev.active_scan_mask & 0x7) <<
    RM3100_CMM_AXIS_SHIFT | RM3100_CMM_START);
    if (ret < 0)
    return ret;
    }
    data.conversion_time = rm3100_samp_rates[i][2] * 2;
    return 0;
    }
    static int rm3100_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct rm3100_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = rm3100_read_mag(data, chan.scan_index, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = data->scale;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    return rm3100_get_samp_freq(data, val, val2);
    default:
    return -EINVAL;
    }
    }
    static int rm3100_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    return rm3100_set_samp_freq(indio_dev, val, val2);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info rm3100_info = {
    .attrs = &rm3100_attribute_group,
    .read_raw = rm3100_read_raw,
    .write_raw = rm3100_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn rm3100_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int rm3100_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct rm3100_data *data = iio_priv(indio_dev);
// Starting channels enabled.
    return regmap_write(data.regmap, RM3100_REG_CMM,
    (*indio_dev.active_scan_mask & 0x7) << RM3100_CMM_AXIS_SHIFT |
    RM3100_CMM_START);
    }
#[no_mangle]
unsafe extern "C" fn rm3100_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int rm3100_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct rm3100_data *data = iio_priv(indio_dev);
    return regmap_write(data.regmap, RM3100_REG_CMM, 0);
    }
    static const struct iio_buffer_setup_ops rm3100_buffer_ops = {
    .preenable = rm3100_buffer_preenable,
    .postdisable = rm3100_buffer_postdisable,
    };
//
// rm3100_regmap_bulk_read_locked() - Wrapper around regmap_bulk_read() with a mutex
//
// @data: Data structure containing regmap and mutex
// @reg: First register to be read from, passed to regmap_bulk_read()
// @val: Pointer to store read value, in native register size for device,
// passed to regmap_bulk_read()
// @val_count: Number of registers to read, passed to regmap_bulk_read()
//
// Intended for use only in rm3100_trigger_handler().
//
// Return:
// A value of zero on success, a negative errno in error cases.
//
    static int rm3100_regmap_bulk_read_locked(struct rm3100_data *data, unsigned int reg,
    void *val, size_t val_count)
    {
    guard(mutex)(&data.lock);
    return regmap_bulk_read(data.regmap, reg, val, val_count);
    }
#[no_mangle]
unsafe extern "C" fn rm3100_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t rm3100_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    let mut scan_mask: c_ulong = *indio_dev.active_scan_mask;
    let mut mask_len: c_uint = iio_get_masklength(indio_dev);
    struct rm3100_data *data = iio_priv(indio_dev);
    int ret, i, bit;
    switch (scan_mask) {
    case BIT(0) | BIT(1) | BIT(2):
    ret = rm3100_regmap_bulk_read_locked(data, RM3100_REG_MX2,
    data.buffer, 9);
    if (ret < 0)
    goto done;
// Convert XXXYYYZZZxxx to XXXxYYYxZZZx. x for paddings.
    for (i = 2; i > 0; i--)
    memmove(data.buffer + i * 4, data.buffer + i * 3, 3);
    break;
    case BIT(0) | BIT(1):
    ret = rm3100_regmap_bulk_read_locked(data, RM3100_REG_MX2,
    data.buffer, 6);
    if (ret < 0)
    goto done;
    memmove(data.buffer + 4, data.buffer + 3, 3);
    break;
    case BIT(1) | BIT(2):
    ret = rm3100_regmap_bulk_read_locked(data, RM3100_REG_MY2,
    data.buffer, 6);
    if (ret < 0)
    goto done;
    memmove(data.buffer + 4, data.buffer + 3, 3);
    break;
    case BIT(0) | BIT(2):
    ret = rm3100_regmap_bulk_read_locked(data, RM3100_REG_MX2,
    data.buffer, 9);
    if (ret < 0)
    goto done;
    memmove(data.buffer + 4, data.buffer + 6, 3);
    break;
    default:
    for_each_set_bit(bit, &scan_mask, mask_len) {
    ret = rm3100_regmap_bulk_read_locked(data,
    RM3100_REG_MX2 + 3 * bit,
    data.buffer, 3);
    if (ret < 0)
    goto done;
    }
    }
//
// Always using the same buffer so that we wouldn't need to set the
// paddings to 0 in case of leaking any data.
//
    iio_push_to_buffers_with_ts(indio_dev, data.buffer, sizeof(data.buffer),
    pf.timestamp);
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn rm3100_common_probe(dev: *mut device, regmap: *mut regmap, irq: c_int) -> c_int {
    int rm3100_common_probe(struct device *dev, struct regmap *regmap, int irq)
    {
    struct iio_dev *indio_dev;
    struct rm3100_data *data;
    unsigned int tmp;
    int ret;
    int samp_rate_index;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.regmap = regmap;
    mutex_init(&data.lock);
    indio_dev.name = "rm3100";
    indio_dev.info = &rm3100_info;
    indio_dev.channels = rm3100_channels;
    indio_dev.num_channels = ARRAY_SIZE(rm3100_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    if (!irq)
    data.use_interrupt = false;
    else {
    data.use_interrupt = true;
    init_completion(&data.measuring_done);
    ret = devm_request_threaded_irq(dev,
    irq,
    rm3100_irq_handler,
    rm3100_thread_fn,
    IRQF_TRIGGER_HIGH |
    IRQF_ONESHOT,
    indio_dev.name,
    indio_dev);
    if (ret)
    return ret;
    data.drdy_trig = devm_iio_trigger_alloc(dev, "%s-drdy%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.drdy_trig)
    return -ENOMEM;
    ret = devm_iio_trigger_register(dev, data.drdy_trig);
    if (ret < 0)
    return ret;
    }
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    &iio_pollfunc_store_time,
    rm3100_trigger_handler,
    &rm3100_buffer_ops);
    if (ret < 0)
    return ret;
    ret = regmap_read(regmap, RM3100_REG_TMRC, &tmp);
    if (ret < 0)
    return ret;
    samp_rate_index = tmp - RM3100_TMRC_OFFSET;
    if (samp_rate_index < 0 || samp_rate_index >=  RM3100_SAMP_NUM) {
    dev_err(dev, "The value read from RM3100_REG_TMRC is invalid!\n");
    return -EINVAL;
    }
// Initializing max wait time, which is double conversion time.
    data.conversion_time = rm3100_samp_rates[samp_rate_index][2] * 2;
// Cycle count values may not be what we want.
    if ((tmp - RM3100_TMRC_OFFSET) == 0)
    rm3100_set_cycle_count(data, 100);
    else
    rm3100_set_cycle_count(data, 200);
    return devm_iio_device_register(dev, indio_dev);
    }
    EXPORT_SYMBOL_NS_GPL(rm3100_common_probe, "IIO_RM3100");
    MODULE_AUTHOR("Song Qiang <songqiang1304521@gmail.com>");
    MODULE_DESCRIPTION("PNI RM3100 3-axis magnetometer i2c driver");
    MODULE_LICENSE("GPL v2");
