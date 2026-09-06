//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad4062.c
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
// Analog Devices AD4062 I3C ADC driver
//
// Copyright 2025 Analog Devices Inc.
//

pub const AD4062_REG_INTERFACE_CONFIG_A: c_uint = 0x00;
pub const AD4062_REG_DEVICE_CONFIG: c_uint = 0x02;

pub const AD4062_REG_DEVICE_CONFIG_LOW_POWER_MODE: c_int = 3;
pub const AD4062_REG_PROD_ID_1: c_uint = 0x05;
pub const AD4062_REG_DEVICE_GRADE: c_uint = 0x06;
pub const AD4062_REG_SCRATCH_PAD: c_uint = 0x0A;
pub const AD4062_REG_VENDOR_H: c_uint = 0x0D;
pub const AD4062_REG_STREAM_MODE: c_uint = 0x0E;
pub const AD4062_REG_INTERFACE_STATUS: c_uint = 0x11;
pub const AD4062_REG_MODE_SET: c_uint = 0x20;

pub const AD4062_REG_ADC_MODES: c_uint = 0x21;

pub const AD4062_REG_ADC_CONFIG: c_uint = 0x22;

pub const AD4062_REG_AVG_CONFIG: c_uint = 0x23;
pub const AD4062_REG_GP_CONF: c_uint = 0x24;

pub const AD4062_REG_INTR_CONF: c_uint = 0x25;

pub const AD4062_REG_TIMER_CONFIG: c_uint = 0x27;

pub const AD4062_REG_MAX_LIMIT: c_uint = 0x29;
pub const AD4062_REG_MIN_LIMIT: c_uint = 0x2B;
pub const AD4062_REG_MAX_HYST: c_uint = 0x2C;
pub const AD4062_REG_MIN_HYST: c_uint = 0x2D;
pub const AD4062_REG_MON_VAL: c_uint = 0x2F;
pub const AD4062_REG_ADC_IBI_EN: c_uint = 0x31;

pub const AD4062_REG_FUSE_CRC: c_uint = 0x40;
pub const AD4062_REG_DEVICE_STATUS: c_uint = 0x41;

pub const AD4062_REG_IBI_STATUS: c_uint = 0x48;
pub const AD4062_REG_CONV_READ_LSB: c_uint = 0x50;
pub const AD4062_REG_CONV_READ_16BITS: c_uint = 0x51;
pub const AD4062_REG_CONV_READ_32BITS: c_uint = 0x53;
pub const AD4062_REG_CONV_TRIGGER_16BITS: c_uint = 0x57;
pub const AD4062_REG_CONV_TRIGGER_32BITS: c_uint = 0x59;
pub const AD4062_REG_CONV_AUTO: c_uint = 0x61;

pub const AD4062_MON_VAL_MIDDLE_POINT: c_uint = 0x8000;
pub const AD4062_I3C_VENDOR: c_uint = 0x0177;
pub const AD4062_SOFT_RESET: c_uint = 0x81;
pub const AD4060_PROD_ID: c_uint = 0x7A;
pub const AD4062_PROD_ID: c_uint = 0x7C;
pub const AD4062_GP_DISABLED: c_uint = 0x0;
pub const AD4062_GP_INTR: c_uint = 0x1;
pub const AD4062_GP_DRDY: c_uint = 0x2;
pub const AD4062_GP_STATIC_LOW: c_uint = 0x5;
pub const AD4062_GP_STATIC_HIGH: c_uint = 0x6;
pub const AD4062_LIMIT_BITS: c_int = 12;
pub const AD4062_INTR_EN_NEITHER: c_uint = 0x0;
pub const AD4062_INTR_EN_EITHER: c_uint = 0x3;
pub const AD4062_TCONV_NS: c_int = 270;
    enum ad4062_operation_mode {
    AD4062_SAMPLE_MODE = 0x0,
    AD4062_BURST_AVERAGING_MODE = 0x1,
    AD4062_MONITOR_MODE = 0x3,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad4062_chip_info {
    pub channels: [iio_chan_spec; 1],
    pub name: *const c_char,
    pub prod_id: u16,
    pub avg_max: u16,
}

    enum {
    AD4062_SCAN_TYPE_SAMPLE,
    AD4062_SCAN_TYPE_BURST_AVG,
    };
    static const struct iio_scan_type ad4062_scan_type_12_s[] = {
    [AD4062_SCAN_TYPE_SAMPLE] = {
    .sign = 's',
    .realbits = 12,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    [AD4062_SCAN_TYPE_BURST_AVG] = {
    .sign = 's',
    .realbits = 14,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    };
    static const struct iio_scan_type ad4062_scan_type_16_s[] = {
    [AD4062_SCAN_TYPE_SAMPLE] = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    [AD4062_SCAN_TYPE_BURST_AVG] = {
    .sign = 's',
    .realbits = 20,
    .storagebits = 32,
    .endianness = IIO_BE,
    },
    };
    static const unsigned int ad4062_conversion_freqs[] = {
    2000000, 1000000, 300000, 100000,	/*  0 -  3 */
    33300, 10000, 3000, 500,		/*  4 -  7 */
    333, 250, 200, 166,			/*  8 - 11 */
    140, 124, 111,				/* 12 - 15 */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad4062_state {
    pub chip: *const ad4062_chip_info,
    pub ops: *const ad4062_bus_ops,
    pub mode: enum ad4062_operation_mode,
    pub trig_conv: work_struct,
    pub completion: completion,
    pub trigger: *mut iio_trigger,
    pub indio_dev: *mut iio_dev,
    pub i3cdev: *mut i3c_device,
    pub regmap: *mut regmap,
    pub wait_event: bool,
    pub vref_uV: c_int,
    pub samp_freqs: [c_uint; ARRAY_SIZE(ad4062_conversion_freqs)],
    pub gpo_irq: [bool; 2],
    pub sampling_frequency: u16,
    pub events_frequency: u16,
    pub oversamp_ratio: u8,
    pub conv_sizeof: u8,
    pub conv_addr: u8,
    union {
    pub be32: __be32,
    pub be16: __be16,
    pub __aligned(IIO_DMA_MINALIGN): } buf,
}

    static const struct regmap_range ad4062_regmap_rd_ranges[] = {
    regmap_reg_range(AD4062_REG_INTERFACE_CONFIG_A, AD4062_REG_DEVICE_GRADE),
    regmap_reg_range(AD4062_REG_SCRATCH_PAD, AD4062_REG_INTERFACE_STATUS),
    regmap_reg_range(AD4062_REG_MODE_SET, AD4062_REG_ADC_IBI_EN),
    regmap_reg_range(AD4062_REG_FUSE_CRC, AD4062_REG_IBI_STATUS),
    regmap_reg_range(AD4062_REG_CONV_READ_LSB, AD4062_REG_CONV_AUTO),
    };
    static const struct regmap_access_table ad4062_regmap_rd_table = {
    .yes_ranges = ad4062_regmap_rd_ranges,
    .n_yes_ranges = ARRAY_SIZE(ad4062_regmap_rd_ranges),
    };
    static const struct regmap_range ad4062_regmap_wr_ranges[] = {
    regmap_reg_range(AD4062_REG_INTERFACE_CONFIG_A, AD4062_REG_DEVICE_CONFIG),
    regmap_reg_range(AD4062_REG_SCRATCH_PAD, AD4062_REG_SCRATCH_PAD),
    regmap_reg_range(AD4062_REG_STREAM_MODE, AD4062_REG_INTERFACE_STATUS),
    regmap_reg_range(AD4062_REG_MODE_SET, AD4062_REG_ADC_IBI_EN),
    regmap_reg_range(AD4062_REG_FUSE_CRC, AD4062_REG_DEVICE_STATUS),
    };
    static const struct regmap_access_table ad4062_regmap_wr_table = {
    .yes_ranges = ad4062_regmap_wr_ranges,
    .n_yes_ranges = ARRAY_SIZE(ad4062_regmap_wr_ranges),
    };
    static const struct iio_event_spec ad4062_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_shared_by_all = BIT(IIO_EV_INFO_ENABLE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_shared_by_all = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_HYSTERESIS),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_shared_by_all = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_HYSTERESIS),
    },
    };

    .type = IIO_VOLTAGE,								\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_RAW) |				\
    BIT(IIO_CHAN_INFO_SCALE) |				\
    BIT(IIO_CHAN_INFO_CALIBSCALE) |			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),		\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),			\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_SAMP_FREQ),		\
    .indexed = 1,									\
    .channel = 0,									\
    .event_spec = ad4062_events,							\
    .num_event_specs = ARRAY_SIZE(ad4062_events),					\
    .has_ext_scan_type = 1,								\
    .ext_scan_type = ad4062_scan_type_##bits##_s,					\
    .num_ext_scan_type = ARRAY_SIZE(ad4062_scan_type_##bits##_s),			\
    }
    static const struct ad4062_chip_info ad4060_chip_info = {
    .name = "ad4060",
    .channels = { AD4062_CHAN(12) },
    .prod_id = AD4060_PROD_ID,
    .avg_max = 256,
    };
    static const struct ad4062_chip_info ad4062_chip_info = {
    .name = "ad4062",
    .channels = { AD4062_CHAN(16) },
    .prod_id = AD4062_PROD_ID,
    .avg_max = 4096,
    };
    static ssize_t sampling_frequency_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ad4062_state *st = iio_priv(dev_to_iio_dev(dev));
    return sysfs_emit(buf, "%d\n", ad4062_conversion_freqs[st.events_frequency]);
    }
    static int sampling_frequency_store_dispatch(struct iio_dev *indio_dev,
    const char *buf)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int val, ret;
    if (st.wait_event)
    return -EBUSY;
    ret = kstrtoint(buf, 10, &val);
    if (ret)
    return ret;
    st.events_frequency = find_closest_descending(val, ad4062_conversion_freqs,
    ARRAY_SIZE(ad4062_conversion_freqs));
    return 0;
    }
    static ssize_t sampling_frequency_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = sampling_frequency_store_dispatch(indio_dev, buf);
    iio_device_release_direct(indio_dev);
    return ret ?: len;
    }
    static IIO_DEVICE_ATTR_RW(sampling_frequency, 0);
    static ssize_t sampling_frequency_available_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut ret: c_int = 0;
    for (u8 i = 0; i < ARRAY_SIZE(ad4062_conversion_freqs); i++)
    ret += sysfs_emit_at(buf, ret, "%d%s", ad4062_conversion_freqs[i],
    i != (ARRAY_SIZE(ad4062_conversion_freqs) - 1) ? " " : "\n");
    return ret;
    }
    static IIO_DEVICE_ATTR_RO(sampling_frequency_available, 0);
    static struct attribute *ad4062_event_attributes[] = {
    &iio_dev_attr_sampling_frequency.dev_attr.attr,
    &iio_dev_attr_sampling_frequency_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ad4062_event_attribute_group = {
    .attrs = ad4062_event_attributes,
    };
#[no_mangle]
unsafe extern "C" fn ad4062_set_oversampling_ratio(st: *mut ad4062_state, val: c_int, val2: c_int) -> c_int {
    static int ad4062_set_oversampling_ratio(struct ad4062_state *st, int val, int val2)
    {
    let mut _max: u32 = st.chip.avg_max;
    let mut _min: u32 = 1;
    int ret;
    if (!in_range(val, _min, _max) || val2 != 0)
    return -EINVAL;
// 1 disables oversampling
    val = ilog2(val);
    if (val == 0) {
    st.mode = AD4062_SAMPLE_MODE;
    } else {
    st.mode = AD4062_BURST_AVERAGING_MODE;
    ret = regmap_write(st.regmap, AD4062_REG_AVG_CONFIG, val - 1);
    if (ret)
    return ret;
    }
    st.oversamp_ratio = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_get_oversampling_ratio(st: *mut ad4062_state, val: *mut c_int) -> c_int {
    static int ad4062_get_oversampling_ratio(struct ad4062_state *st, int *val)
    {
    int ret, buf;
    if (st.mode == AD4062_SAMPLE_MODE) {
// val = 1;
    return 0;
    }
    ret = regmap_read(st.regmap, AD4062_REG_AVG_CONFIG, &buf);
    if (ret)
    return ret;
// val = BIT(buf + 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_calc_sampling_frequency(fosc: c_uint, oversamp_ratio: c_uint) -> c_int {
    static int ad4062_calc_sampling_frequency(unsigned int fosc, unsigned int oversamp_ratio)
    {
// From datasheet p.31: (n_avg - 1)/fosc + tconv
    let mut n_avg: u32 = BIT(oversamp_ratio) - 1;
    let mut period_ns: u32 = NSEC_PER_SEC / fosc;
// Result is less than 1 Hz
    if (n_avg >= fosc)
    return 1;
    return NSEC_PER_SEC / (n_avg * period_ns + AD4062_TCONV_NS);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_populate_sampling_frequency(st: *mut ad4062_state) -> c_int {
    static int ad4062_populate_sampling_frequency(struct ad4062_state *st)
    {
    for (u8 i = 0; i < ARRAY_SIZE(ad4062_conversion_freqs); i++)
    st.samp_freqs[i] =
    ad4062_calc_sampling_frequency(ad4062_conversion_freqs[i],
    st.oversamp_ratio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_get_sampling_frequency(st: *mut ad4062_state, val: *mut c_int) -> c_int {
    static int ad4062_get_sampling_frequency(struct ad4062_state *st, int *val)
    {
    let mut freq: c_int = ad4062_conversion_freqs[st.sampling_frequency];
// val = ad4062_calc_sampling_frequency(freq, st->oversamp_ratio);
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_set_sampling_frequency(st: *mut ad4062_state, val: c_int, val2: c_int) -> c_int {
    static int ad4062_set_sampling_frequency(struct ad4062_state *st, int val, int val2)
    {
    int ret;
    if (val2 != 0)
    return -EINVAL;
    ret = ad4062_populate_sampling_frequency(st);
    if (ret)
    return ret;
    st.sampling_frequency =
    find_closest_descending(val, st.samp_freqs,
    ARRAY_SIZE(ad4062_conversion_freqs));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_check_ids(st: *mut ad4062_state) -> c_int {
    static int ad4062_check_ids(struct ad4062_state *st)
    {
    struct device *dev = &st.i3cdev.dev;
    int ret;
    u16 val;
    ret = regmap_bulk_read(st.regmap, AD4062_REG_PROD_ID_1,
    &st.buf.be16, sizeof(st.buf.be16));
    if (ret)
    return ret;
    val = be16_to_cpu(st.buf.be16);
    if (val != st.chip.prod_id)
    dev_warn(dev, "Production ID x%x does not match known values", val);
    ret = regmap_bulk_read(st.regmap, AD4062_REG_VENDOR_H,
    &st.buf.be16, sizeof(st.buf.be16));
    if (ret)
    return ret;
    val = be16_to_cpu(st.buf.be16);
    if (val != AD4062_I3C_VENDOR)
    return dev_err_probe(dev, -ENODEV,
    "Vendor ID x%x does not match expected value\n", val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_conversion_frequency_set(st: *mut ad4062_state, val: u8) -> c_int {
    static int ad4062_conversion_frequency_set(struct ad4062_state *st, u8 val)
    {
    return regmap_write(st.regmap, AD4062_REG_TIMER_CONFIG,
    FIELD_PREP(AD4062_REG_TIMER_CONFIG_FS_MASK, val));
    }
    static int ad4062_set_operation_mode(struct ad4062_state *st,
    enum ad4062_operation_mode mode)
    {
    const unsigned int samp_freq = mode == AD4062_MONITOR_MODE ?
    st.events_frequency : st.sampling_frequency;
    int ret;
    ret = ad4062_conversion_frequency_set(st, samp_freq);
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, AD4062_REG_ADC_MODES,
    AD4062_REG_ADC_MODES_MODE_MSK, mode);
    if (ret)
    return ret;
    if (mode == AD4062_MONITOR_MODE) {
// Change address pointer to enter monitor mode
    struct i3c_xfer xfer_trigger = {
    .data.out = &st.conv_addr,
    .len = sizeof(st.conv_addr),
    .rnw = false,
    };
    st.conv_addr = AD4062_REG_CONV_TRIGGER_32BITS;
    return i3c_device_do_xfers(st.i3cdev, &xfer_trigger, 1, I3C_SDR);
    }
    return regmap_write(st.regmap, AD4062_REG_MODE_SET,
    AD4062_REG_MODE_SET_ENTER_ADC);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_soft_reset(st: *mut ad4062_state) -> c_int {
    static int ad4062_soft_reset(struct ad4062_state *st)
    {
    let mut val: u8 = AD4062_SOFT_RESET;
    int ret;
    ret = regmap_write(st.regmap, AD4062_REG_INTERFACE_CONFIG_A, val);
    if (ret)
    return ret;
// Wait AD4062 treset time, datasheet p8
    ndelay(60);
    return 0;
    }
    static int ad4062_setup(struct iio_dev *indio_dev, struct iio_chan_spec const *chan,
    const bool *ref_sel)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    const struct iio_scan_type *scan_type;
    int ret;
    scan_type = iio_get_current_scan_type(indio_dev, chan);
    if (IS_ERR(scan_type))
    return PTR_ERR(scan_type);
    ret = regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    AD4062_REG_GP_CONF_MODE_MSK_0,
    FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_0,
    AD4062_GP_INTR));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    AD4062_REG_GP_CONF_MODE_MSK_1,
    FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_1,
    AD4062_GP_DRDY));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, AD4062_REG_ADC_CONFIG,
    AD4062_REG_ADC_CONFIG_REF_EN_MSK,
    FIELD_PREP(AD4062_REG_ADC_CONFIG_REF_EN_MSK,
// ref_sel));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, AD4062_REG_DEVICE_STATUS,
    AD4062_REG_DEVICE_STATUS_DEVICE_RESET);
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, AD4062_REG_INTR_CONF,
    AD4062_REG_INTR_CONF_EN_MSK_0,
    FIELD_PREP(AD4062_REG_INTR_CONF_EN_MSK_0,
    AD4062_INTR_EN_EITHER));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, AD4062_REG_INTR_CONF,
    AD4062_REG_INTR_CONF_EN_MSK_1,
    FIELD_PREP(AD4062_REG_INTR_CONF_EN_MSK_1,
    AD4062_INTR_EN_NEITHER));
    if (ret)
    return ret;
    st.buf.be16 = cpu_to_be16(AD4062_MON_VAL_MIDDLE_POINT);
    return regmap_bulk_write(st.regmap, AD4062_REG_MON_VAL,
    &st.buf.be16, sizeof(st.buf.be16));
    }
#[no_mangle]
unsafe extern "C" fn ad4062_irq_handler_thresh(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad4062_irq_handler_thresh(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(IIO_VOLTAGE, 0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(indio_dev));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_irq_handler_drdy(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad4062_irq_handler_drdy(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct ad4062_state *st = iio_priv(indio_dev);
    if (iio_buffer_enabled(indio_dev) && iio_trigger_using_own(indio_dev))
    iio_trigger_poll(st.trigger);
    else
    complete(&st.completion);
    return IRQ_HANDLED;
    }
    static void ad4062_ibi_handler(struct i3c_device *i3cdev,
    const struct i3c_ibi_payload *payload)
    {
    struct ad4062_state *st = i3cdev_get_drvdata(i3cdev);
    if (st.wait_event) {
    iio_push_event(st.indio_dev,
    IIO_UNMOD_EVENT_CODE(IIO_VOLTAGE, 0,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(st.indio_dev));
    return;
    }
    if (iio_buffer_enabled(st.indio_dev))
    iio_trigger_poll_nested(st.trigger);
    else
    complete(&st.completion);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_trigger_work(work: *mut work_struct) {
    static void ad4062_trigger_work(struct work_struct *work)
    {
    struct ad4062_state *st =
    container_of(work, struct ad4062_state, trig_conv);
    int ret;
//
// Read current conversion, if at reg CONV_READ, stop bit triggers
// next sample and does not need writing the address.
//
    struct i3c_xfer xfer_sample = {
    .data.in = &st.buf.be32,
    .len = st.conv_sizeof,
    .rnw = true,
    };
    struct i3c_xfer xfer_trigger = {
    .data.out = &st.conv_addr,
    .len = sizeof(st.conv_addr),
    .rnw = false,
    };
    ret = i3c_device_do_xfers(st.i3cdev, &xfer_sample, 1, I3C_SDR);
    if (ret)
    return;
    iio_push_to_buffers_with_ts(st.indio_dev, &st.buf.be32, st.conv_sizeof,
    iio_get_time_ns(st.indio_dev));
    if (st.gpo_irq[1])
    return;
    i3c_device_do_xfers(st.i3cdev, &xfer_trigger, 1, I3C_SDR);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_poll_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad4062_poll_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ad4062_state *st = iio_priv(indio_dev);
    iio_trigger_notify_done(indio_dev.trig);
    schedule_work(&st.trig_conv);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_disable_ibi(data: *mut c_void) {
    static void ad4062_disable_ibi(void *data)
    {
    struct i3c_device *i3cdev = data;
    i3c_device_disable_ibi(i3cdev);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_free_ibi(data: *mut c_void) {
    static void ad4062_free_ibi(void *data)
    {
    struct i3c_device *i3cdev = data;
    i3c_device_free_ibi(i3cdev);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_request_ibi(i3cdev: *mut i3c_device) -> c_int {
    static int ad4062_request_ibi(struct i3c_device *i3cdev)
    {
    const struct i3c_ibi_setup ibireq = {
    .max_payload_len = 1,
    .num_slots = 1,
    .handler = ad4062_ibi_handler,
    };
    int ret;
    ret = i3c_device_request_ibi(i3cdev, &ibireq);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&i3cdev.dev, ad4062_free_ibi, i3cdev);
    if (ret)
    return ret;
    ret = i3c_device_enable_ibi(i3cdev);
    if (ret)
    return ret;
    return devm_add_action_or_reset(&i3cdev.dev, ad4062_disable_ibi, i3cdev);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_request_irq(indio_dev: *mut iio_dev) -> c_int {
    static int ad4062_request_irq(struct iio_dev *indio_dev)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    struct device *dev = &st.i3cdev.dev;
    int ret;
    ret = fwnode_irq_get_byname(dev_fwnode(&st.i3cdev.dev), "gp0");
    if (ret == -EPROBE_DEFER)
    return ret;
    if (ret < 0) {
    st.gpo_irq[0] = false;
    ret = regmap_update_bits(st.regmap, AD4062_REG_ADC_IBI_EN,
    AD4062_REG_ADC_IBI_EN_MAX | AD4062_REG_ADC_IBI_EN_MIN,
    AD4062_REG_ADC_IBI_EN_MAX | AD4062_REG_ADC_IBI_EN_MIN);
    if (ret)
    return ret;
    } else {
    st.gpo_irq[0] = true;
    ret = devm_request_threaded_irq(dev, ret, core::ptr::null_mut(),
    ad4062_irq_handler_thresh,
    IRQF_ONESHOT, indio_dev.name,
    indio_dev);
    if (ret)
    return ret;
    }
    ret = fwnode_irq_get_byname(dev_fwnode(&st.i3cdev.dev), "gp1");
    if (ret == -EPROBE_DEFER)
    return ret;
    if (ret < 0) {
    st.gpo_irq[1] = false;
    return regmap_update_bits(st.regmap, AD4062_REG_ADC_IBI_EN,
    AD4062_REG_ADC_IBI_EN_CONV_TRIGGER,
    AD4062_REG_ADC_IBI_EN_CONV_TRIGGER);
    }
    st.gpo_irq[1] = true;
    return devm_request_irq(dev, ret, ad4062_irq_handler_drdy,
    IRQF_NO_THREAD, indio_dev.name, indio_dev);
    }
    static const struct iio_trigger_ops ad4062_trigger_ops = {
    .validate_device = &iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn ad4062_request_trigger(indio_dev: *mut iio_dev) -> c_int {
    static int ad4062_request_trigger(struct iio_dev *indio_dev)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    struct device *dev = &st.i3cdev.dev;
    int ret;
    st.trigger = devm_iio_trigger_alloc(dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!st.trigger)
    return -ENOMEM;
    st.trigger.ops = &ad4062_trigger_ops;
    iio_trigger_set_drvdata(st.trigger, indio_dev);
    ret = devm_iio_trigger_register(dev, st.trigger);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(st.trigger);
    return 0;
    }
    static const int ad4062_oversampling_avail[] = {
    1, 2, 4, 8, 16, 32, 64, 128,		/*  0 -  7 */
    256, 512, 1024, 2048, 4096,		/*  8 - 12 */
    };
    static int ad4062_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, const int **vals,
    int *type, int *len, long mask)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
// vals = ad4062_oversampling_avail;
// len = ARRAY_SIZE(ad4062_oversampling_avail);
// len -= st->chip->avg_max == 256 ? 4 : 0;
// type = IIO_VAL_INT;
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = ad4062_populate_sampling_frequency(st);
    if (ret)
    return ret;
// vals = st->samp_freqs;
// len = st->oversamp_ratio ? ARRAY_SIZE(ad4062_conversion_freqs) : 1;
// type = IIO_VAL_INT;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ad4062_get_chan_scale(indio_dev: *mut iio_dev, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int ad4062_get_chan_scale(struct iio_dev *indio_dev, int *val, int *val2)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    const struct iio_scan_type *scan_type;
//
// In burst averaging mode the averaging filter accumulates resulting
// in a sample with increased precision.
//
    scan_type = iio_get_current_scan_type(indio_dev, st.chip.channels);
    if (IS_ERR(scan_type))
    return PTR_ERR(scan_type);
// val = (st->vref_uV * 2) / (MICRO / MILLI); /* signed
// val2 = scan_type->realbits - 1;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_get_chan_calibscale(st: *mut ad4062_state, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int ad4062_get_chan_calibscale(struct ad4062_state *st, int *val, int *val2)
    {
    int ret;
    ret = regmap_bulk_read(st.regmap, AD4062_REG_MON_VAL,
    &st.buf.be16, sizeof(st.buf.be16));
    if (ret)
    return ret;
// From datasheet: code out = code in × mon_val/0x8000
// val = be16_to_cpu(st->buf.be16) * 2;
// val2 = 16;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
    static int ad4062_set_chan_calibscale(struct ad4062_state *st, int gain_int,
    int gain_frac)
    {
// Divide numerator and denumerator by known great common divider
    let mut mon_val: u32 = AD4062_MON_VAL_MIDDLE_POINT / 64;
    let mut micro: u32 = MICRO / 64;
    let mut gain_fp: u32 = gain_int * MICRO + gain_frac;
    let mut reg_val: u32 = DIV_ROUND_CLOSEST(gain_fp * mon_val, micro);
    int ret;
// Checks if the gain is in range and the value fits the field
    if (gain_int < 0 || gain_int > 1 || reg_val > BIT(16) - 1)
    return -EINVAL;
    st.buf.be16 = cpu_to_be16(reg_val);
    ret = regmap_bulk_write(st.regmap, AD4062_REG_MON_VAL,
    &st.buf.be16, sizeof(st.buf.be16));
    if (ret)
    return ret;
// Enable scale if gain is not equal to one
    return regmap_update_bits(st.regmap, AD4062_REG_ADC_CONFIG,
    AD4062_REG_ADC_CONFIG_SCALE_EN_MSK,
    FIELD_PREP(AD4062_REG_ADC_CONFIG_SCALE_EN_MSK,
    !(gain_int == 1 && gain_frac == 0)));
    }
#[no_mangle]
unsafe extern "C" fn ad4062_read_chan_raw(st: *mut ad4062_state, val: *mut c_int) -> c_int {
    static int ad4062_read_chan_raw(struct ad4062_state *st, int *val)
    {
    struct i3c_device *i3cdev = st.i3cdev;
    struct i3c_xfer xfer_trigger = {
    .data.out = &st.conv_addr,
    .len = sizeof(st.conv_addr),
    .rnw = false,
    };
    struct i3c_xfer xfer_sample = {
    .data.in = &st.buf.be32,
    .len = sizeof(st.buf.be32),
    .rnw = true,
    };
    int ret;
    PM_RUNTIME_ACQUIRE(&st.i3cdev.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    ret = ad4062_set_operation_mode(st, st.mode);
    if (ret)
    return ret;
    reinit_completion(&st.completion);
// Change address pointer to trigger conversion
    st.conv_addr = AD4062_REG_CONV_TRIGGER_32BITS;
    ret = i3c_device_do_xfers(i3cdev, &xfer_trigger, 1, I3C_SDR);
    if (ret)
    return ret;
//
// Single sample read should be used only for oversampling and
// sampling frequency pairs that take less than 1 sec.
//
    ret = wait_for_completion_timeout(&st.completion,
    msecs_to_jiffies(1000));
    if (!ret)
    return -ETIMEDOUT;
    ret = i3c_device_do_xfers(i3cdev, &xfer_sample, 1, I3C_SDR);
    if (ret)
    return ret;
// val = be32_to_cpu(st->buf.be32);
    return 0;
    }
    static int ad4062_read_raw_dispatch(struct ad4062_state *st,
    int *val, int *val2, long info)
    {
    if (st.wait_event)
    return -EBUSY;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    return ad4062_read_chan_raw(st, val);
    case IIO_CHAN_INFO_CALIBSCALE:
    return ad4062_get_chan_calibscale(st, val, val2);
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
    return ad4062_get_oversampling_ratio(st, val);
    default:
    return -EINVAL;
    }
    }
    static int ad4062_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int ret;
    switch (info) {
    case IIO_CHAN_INFO_SCALE:
    return ad4062_get_chan_scale(indio_dev, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return ad4062_get_sampling_frequency(st, val);
    }
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad4062_read_raw_dispatch(st, val, val2, info);
    iio_device_release_direct(indio_dev);
    return ret ?: IIO_VAL_INT;
    }
    static int ad4062_write_raw_dispatch(struct ad4062_state *st, int val, int val2,
    long info)
    {
    if (st.wait_event)
    return -EBUSY;
    switch (info) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
    return ad4062_set_oversampling_ratio(st, val, val2);
    case IIO_CHAN_INFO_CALIBSCALE:
    return ad4062_set_chan_calibscale(st, val, val2);
    default:
    return -EINVAL;
    }
    }
    static int ad4062_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long info)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int ret;
    switch (info) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    return ad4062_set_sampling_frequency(st, val, val2);
    }
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad4062_write_raw_dispatch(st, val, val2, info);
    iio_device_release_direct(indio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm_ad4062_monitor_mode_enable(st: *mut ad4062_state) -> c_int {
    static int pm_ad4062_monitor_mode_enable(struct ad4062_state *st)
    {
    int ret;
    PM_RUNTIME_ACQUIRE(&st.i3cdev.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    return ad4062_set_operation_mode(st, AD4062_MONITOR_MODE);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_monitor_mode_enable(st: *mut ad4062_state) -> c_int {
    static int ad4062_monitor_mode_enable(struct ad4062_state *st)
    {
    int ret;
    ret = pm_ad4062_monitor_mode_enable(st);
    if (ret)
    return ret;
    pm_runtime_get_noresume(&st.i3cdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_monitor_mode_disable(st: *mut ad4062_state) -> c_int {
    static int ad4062_monitor_mode_disable(struct ad4062_state *st)
    {
    pm_runtime_put_autosuspend(&st.i3cdev.dev);
    return 0;
    }
    static int ad4062_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    return st.wait_event;
    }
    static int ad4062_write_event_config_dispatch(struct iio_dev *indio_dev,
    bool state)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int ret;
    if (st.wait_event == state)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: state) -> else {
    else if (state)
    ret = ad4062_monitor_mode_enable(st);
    else
    ret = ad4062_monitor_mode_disable(st);
    if (ret)
    return ret;
    st.wait_event = state;
    return 0;
    }
    static int ad4062_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad4062_write_event_config_dispatch(indio_dev, state);
    iio_device_release_direct(indio_dev);
    return ret;
    }
    static int __ad4062_read_event_info_value(struct ad4062_state *st,
    enum iio_event_direction dir, int *val)
    {
    int ret;
    u8 reg;
    if (dir == IIO_EV_DIR_RISING)
    reg = AD4062_REG_MAX_LIMIT;
    else
    reg = AD4062_REG_MIN_LIMIT;
    ret = regmap_bulk_read(st.regmap, reg, &st.buf.be16,
    sizeof(st.buf.be16));
    if (ret)
    return ret;
// val = sign_extend32(be16_to_cpu(st->buf.be16), AD4062_LIMIT_BITS - 1);
    return 0;
    }
    static int __ad4062_read_event_info_hysteresis(struct ad4062_state *st,
    enum iio_event_direction dir, int *val)
    {
    u8 reg;
    if (dir == IIO_EV_DIR_RISING)
    reg = AD4062_REG_MAX_HYST;
    else
    reg = AD4062_REG_MIN_HYST;
    return regmap_read(st.regmap, reg, val);
    }
    static int ad4062_read_event_config_dispatch(struct iio_dev *indio_dev,
    enum iio_event_direction dir,
    enum iio_event_info info, int *val)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    if (st.wait_event)
    return -EBUSY;
    switch (info) {
    case IIO_EV_INFO_VALUE:
    return __ad4062_read_event_info_value(st, dir, val);
    case IIO_EV_INFO_HYSTERESIS:
    return __ad4062_read_event_info_hysteresis(st, dir, val);
    default:
    return -EINVAL;
    }
    }
    static int ad4062_read_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int *val,
    int *val2)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad4062_read_event_config_dispatch(indio_dev, dir, info, val);
    iio_device_release_direct(indio_dev);
    return ret ?: IIO_VAL_INT;
    }
    static int __ad4062_write_event_info_value(struct ad4062_state *st,
    enum iio_event_direction dir, int val)
    {
    u8 reg;
    if (val != sign_extend32(val, AD4062_LIMIT_BITS - 1))
    return -EINVAL;
    if (dir == IIO_EV_DIR_RISING)
    reg = AD4062_REG_MAX_LIMIT;
    else
    reg = AD4062_REG_MIN_LIMIT;
    st.buf.be16 = cpu_to_be16(val);
    return regmap_bulk_write(st.regmap, reg, &st.buf.be16,
    sizeof(st.buf.be16));
    }
    static int __ad4062_write_event_info_hysteresis(struct ad4062_state *st,
    enum iio_event_direction dir, int val)
    {
    u8 reg;
    if (val > BIT(7) - 1)
    return -EINVAL;
    if (dir == IIO_EV_DIR_RISING)
    reg = AD4062_REG_MAX_HYST;
    else
    reg = AD4062_REG_MIN_HYST;
    return regmap_write(st.regmap, reg, val);
    }
    static int ad4062_write_event_value_dispatch(struct iio_dev *indio_dev,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int val)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    if (st.wait_event)
    return -EBUSY;
    switch (type) {
    case IIO_EV_TYPE_THRESH:
    switch (info) {
    case IIO_EV_INFO_VALUE:
    return __ad4062_write_event_info_value(st, dir, val);
    case IIO_EV_INFO_HYSTERESIS:
    return __ad4062_write_event_info_hysteresis(st, dir, val);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static int ad4062_write_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int val,
    int val2)
    {
    int ret;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad4062_write_event_value_dispatch(indio_dev, type, dir, info, val);
    iio_device_release_direct(indio_dev);
    return ret;
    }
//
// The AD4062 in burst averaging mode increases realbits from 16-bits to
// 20-bits, increasing the storagebits from 16-bits to 32-bits.
//
#[no_mangle]
pub unsafe extern "C" fn ad4062_sizeof_storagebits(st: *mut ad4062_state) -> c_int {
    static inline int ad4062_sizeof_storagebits(struct ad4062_state *st)
    {
    const struct iio_scan_type *scan_type =
    iio_get_current_scan_type(st.indio_dev, st.chip.channels);
    if (IS_ERR(scan_type))
    return PTR_ERR(scan_type);
    return BITS_TO_BYTES(scan_type.storagebits);
    }
// Read registers only with realbits (no sign extension bytes)
#[no_mangle]
pub unsafe extern "C" fn ad4062_get_conv_addr(st: *mut ad4062_state, _sizeof: usize) -> usize {
    static inline size_t ad4062_get_conv_addr(struct ad4062_state *st, size_t _sizeof)
    {
    if (st.gpo_irq[1])
    return _sizeof == sizeof(u32) ? AD4062_REG_CONV_READ_32BITS :
    AD4062_REG_CONV_READ_16BITS;
    return _sizeof == sizeof(u32) ? AD4062_REG_CONV_TRIGGER_32BITS :
    AD4062_REG_CONV_TRIGGER_16BITS;
    }
#[no_mangle]
unsafe extern "C" fn pm_ad4062_triggered_buffer_postenable(st: *mut ad4062_state) -> c_int {
    static int pm_ad4062_triggered_buffer_postenable(struct ad4062_state *st)
    {
    int ret;
    PM_RUNTIME_ACQUIRE(&st.i3cdev.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    if (st.wait_event)
    return -EBUSY;
    ret = ad4062_set_operation_mode(st, st.mode);
    if (ret)
    return ret;
    ret = ad4062_sizeof_storagebits(st);
    if (ret < 0)
    return ret;
    st.conv_sizeof = ret;
    st.conv_addr = ad4062_get_conv_addr(st, st.conv_sizeof);
// CONV_READ requires read to trigger first sample.
    struct i3c_xfer xfer_sample[2] = {
    {
    .data.out = &st.conv_addr,
    .len = sizeof(st.conv_addr),
    .rnw = false,
    },
    {
    .data.in = &st.buf.be32,
    .len = sizeof(st.buf.be32),
    .rnw = true,
    }
    };
    return i3c_device_do_xfers(st.i3cdev, xfer_sample,
    st.gpo_irq[1] ? 2 : 1, I3C_SDR);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_triggered_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int ad4062_triggered_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    int ret;
    ret = pm_ad4062_triggered_buffer_postenable(st);
    if (ret)
    return ret;
    pm_runtime_get_noresume(&st.i3cdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_triggered_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int ad4062_triggered_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    pm_runtime_put_autosuspend(&st.i3cdev.dev);
    return 0;
    }
    static const struct iio_buffer_setup_ops ad4062_triggered_buffer_setup_ops = {
    .postenable = &ad4062_triggered_buffer_postenable,
    .predisable = &ad4062_triggered_buffer_predisable,
    };
    static int ad4062_debugfs_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    else
    return regmap_write(st.regmap, reg, writeval);
    }
    static int ad4062_get_current_scan_type(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct ad4062_state *st = iio_priv(indio_dev);
    return st.mode == AD4062_BURST_AVERAGING_MODE ?
    AD4062_SCAN_TYPE_BURST_AVG :
    AD4062_SCAN_TYPE_SAMPLE;
    }
    static const struct iio_info ad4062_info = {
    .read_raw = ad4062_read_raw,
    .write_raw = ad4062_write_raw,
    .read_avail = ad4062_read_avail,
    .read_event_config = ad4062_read_event_config,
    .write_event_config = ad4062_write_event_config,
    .read_event_value = ad4062_read_event_value,
    .write_event_value = ad4062_write_event_value,
    .event_attrs = &ad4062_event_attribute_group,
    .get_current_scan_type = ad4062_get_current_scan_type,
    .debugfs_reg_access = ad4062_debugfs_reg_access,
    };
    static const struct regmap_config ad4062_regmap_config = {
    .name = "ad4062",
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AD4062_MAX_REG,
    .rd_table = &ad4062_regmap_rd_table,
    .wr_table = &ad4062_regmap_wr_table,
    .can_sleep = true,
    };
#[no_mangle]
unsafe extern "C" fn ad4062_regulators_get(st: *mut ad4062_state, ref_sel: *mut bool) -> c_int {
    static int ad4062_regulators_get(struct ad4062_state *st, bool *ref_sel)
    {
    struct device *dev = &st.i3cdev.dev;
    int ret;
    ret = devm_regulator_get_enable(dev, "vio");
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable vio voltage\n");
    st.vref_uV = devm_regulator_get_enable_read_voltage(dev, "ref");
// ref_sel = st->vref_uV == -ENODEV;
    if (st.vref_uV < 0 && !*ref_sel)
    return dev_err_probe(dev, st.vref_uV,
    "Failed to enable and read ref voltage\n");
    if (*ref_sel) {
    st.vref_uV = devm_regulator_get_enable_read_voltage(dev, "vdd");
    if (st.vref_uV < 0)
    return dev_err_probe(dev, st.vref_uV,
    "Failed to enable and read vdd voltage\n");
    } else {
    ret = devm_regulator_get_enable(dev, "vdd");
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to enable vdd regulator\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int ad4062_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int ad4062_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct ad4062_state *st = gpiochip_get_data(gc);
    let mut reg_val: c_uint = value ? AD4062_GP_STATIC_HIGH : AD4062_GP_STATIC_LOW;
    if (offset)
    return regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    AD4062_REG_GP_CONF_MODE_MSK_1,
    FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_1, reg_val));
    else
    return regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    AD4062_REG_GP_CONF_MODE_MSK_0,
    FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_0, reg_val));
    }
#[no_mangle]
unsafe extern "C" fn ad4062_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int ad4062_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct ad4062_state *st = gpiochip_get_data(gc);
    unsigned int reg_val;
    int ret;
    ret = regmap_read(st.regmap, AD4062_REG_GP_CONF, &reg_val);
    if (ret)
    return ret;
    if (offset)
    reg_val = FIELD_GET(AD4062_REG_GP_CONF_MODE_MSK_1, reg_val);
    else
    reg_val = FIELD_GET(AD4062_REG_GP_CONF_MODE_MSK_0, reg_val);
    let mut reg_val: return = = AD4062_GP_STATIC_HIGH;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_gpio_disable(data: *mut c_void) {
    static void ad4062_gpio_disable(void *data)
    {
    struct ad4062_state *st = data;
    u8 val = FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_0, AD4062_GP_DISABLED) |
    FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_1, AD4062_GP_DISABLED);
    regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    AD4062_REG_GP_CONF_MODE_MSK_1 | AD4062_REG_GP_CONF_MODE_MSK_0,
    val);
    }
    static int ad4062_gpio_init_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask,
    unsigned int ngpios)
    {
    struct ad4062_state *st = gpiochip_get_data(gc);
    bitmap_zero(valid_mask, ngpios);
    for (unsigned int i = 0; i < ARRAY_SIZE(st.gpo_irq); i++)
    __assign_bit(i, valid_mask, !st.gpo_irq[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad4062_gpio_init(st: *mut ad4062_state) -> c_int {
    static int ad4062_gpio_init(struct ad4062_state *st)
    {
    struct device *dev = &st.i3cdev.dev;
    struct gpio_chip *gc;
    u8 val, mask;
    int ret;
    if (!device_property_read_bool(dev, "gpio-controller"))
    return 0;
    gc = devm_kzalloc(dev, sizeof(*gc), GFP_KERNEL);
    if (!gc)
    return -ENOMEM;
    val = 0;
    mask = 0;
    if (!st.gpo_irq[0]) {
    mask |= AD4062_REG_GP_CONF_MODE_MSK_0;
    val |= FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_0, AD4062_GP_STATIC_LOW);
    }
    if (!st.gpo_irq[1]) {
    mask |= AD4062_REG_GP_CONF_MODE_MSK_1;
    val |= FIELD_PREP(AD4062_REG_GP_CONF_MODE_MSK_1, AD4062_GP_STATIC_LOW);
    }
    ret = regmap_update_bits(st.regmap, AD4062_REG_GP_CONF,
    mask, val);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, ad4062_gpio_disable, st);
    if (ret)
    return ret;
    gc.parent = dev;
    gc.label = st.chip.name;
    gc.owner = THIS_MODULE;
    gc.base = -1;
    gc.ngpio = 2;
    gc.init_valid_mask = ad4062_gpio_init_valid_mask;
    gc.get_direction = ad4062_gpio_get_direction;
    gc.set = ad4062_gpio_set;
    gc.get = ad4062_gpio_get;
    gc.can_sleep = true;
    ret = devm_gpiochip_add_data(dev, gc, st);
    if (ret)
    return dev_err_probe(dev, ret, "Unable to register GPIO chip\n");
    return 0;
    }
    static const struct i3c_device_id ad4062_id_table[] = {
    I3C_DEVICE(AD4062_I3C_VENDOR, AD4060_PROD_ID, &ad4060_chip_info),
    I3C_DEVICE(AD4062_I3C_VENDOR, AD4062_PROD_ID, &ad4062_chip_info),
    { }
    };
    MODULE_DEVICE_TABLE(i3c, ad4062_id_table);
#[no_mangle]
unsafe extern "C" fn ad4062_probe(i3cdev: *mut i3c_device) -> c_int {
    static int ad4062_probe(struct i3c_device *i3cdev)
    {
    const struct i3c_device_id *id = i3c_device_match_id(i3cdev, ad4062_id_table);
    const struct ad4062_chip_info *chip = id.data;
    struct device *dev = &i3cdev.dev;
    struct iio_dev *indio_dev;
    struct ad4062_state *st;
    bool ref_sel;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.i3cdev = i3cdev;
    i3cdev_set_drvdata(i3cdev, st);
    init_completion(&st.completion);
    ret = ad4062_regulators_get(st, &ref_sel);
    if (ret)
    return ret;
    st.regmap = devm_regmap_init_i3c(i3cdev, &ad4062_regmap_config);
    if (IS_ERR(st.regmap))
    return dev_err_probe(dev, PTR_ERR(st.regmap),
    "Failed to initialize regmap\n");
    st.mode = AD4062_SAMPLE_MODE;
    st.wait_event = false;
    st.chip = chip;
    st.sampling_frequency = 0;
    st.events_frequency = 0;
    st.oversamp_ratio = 0;
    st.indio_dev = indio_dev;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.num_channels = 1;
    indio_dev.info = &ad4062_info;
    indio_dev.name = chip.name;
    indio_dev.channels = chip.channels;
    ret = ad4062_soft_reset(st);
    if (ret)
    return dev_err_probe(dev, ret, "AD4062 failed to soft reset\n");
    ret = ad4062_check_ids(st);
    if (ret)
    return ret;
    ret = ad4062_setup(indio_dev, indio_dev.channels, &ref_sel);
    if (ret)
    return ret;
    ret = ad4062_request_irq(indio_dev);
    if (ret)
    return ret;
    ret = ad4062_request_trigger(indio_dev);
    if (ret)
    return ret;
    ret = devm_iio_triggered_buffer_setup(&i3cdev.dev, indio_dev,
    iio_pollfunc_store_time,
    ad4062_poll_handler,
    &ad4062_triggered_buffer_setup_ops);
    if (ret)
    return ret;
    pm_runtime_set_active(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable pm_runtime\n");
    pm_runtime_set_autosuspend_delay(dev, 1000);
    pm_runtime_use_autosuspend(dev);
    ret = ad4062_request_ibi(i3cdev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request i3c ibi\n");
    ret = ad4062_gpio_init(st);
    if (ret)
    return ret;
    ret = devm_work_autocancel(dev, &st.trig_conv, ad4062_trigger_work);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn ad4062_runtime_suspend(dev: *mut device) -> c_int {
    static int ad4062_runtime_suspend(struct device *dev)
    {
    struct ad4062_state *st = dev_get_drvdata(dev);
    return regmap_write(st.regmap, AD4062_REG_DEVICE_CONFIG,
    FIELD_PREP(AD4062_REG_DEVICE_CONFIG_POWER_MODE_MSK,
    AD4062_REG_DEVICE_CONFIG_LOW_POWER_MODE));
    }
#[no_mangle]
unsafe extern "C" fn ad4062_runtime_resume(dev: *mut device) -> c_int {
    static int ad4062_runtime_resume(struct device *dev)
    {
    struct ad4062_state *st = dev_get_drvdata(dev);
    int ret;
    ret = regmap_clear_bits(st.regmap, AD4062_REG_DEVICE_CONFIG,
    AD4062_REG_DEVICE_CONFIG_POWER_MODE_MSK);
    if (ret)
    return ret;
// Wait device functional blocks to power up
    fsleep(3 * USEC_PER_MSEC);
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(ad4062_pm_ops,
    ad4062_runtime_suspend, ad4062_runtime_resume, core::ptr::null_mut());
    static struct i3c_driver ad4062_driver = {
    .driver = {
    .name = "ad4062",
    .pm = pm_ptr(&ad4062_pm_ops),
    },
    .probe = ad4062_probe,
    .id_table = ad4062_id_table,
    };
    module_i3c_driver(ad4062_driver);
    MODULE_AUTHOR("Jorge Marques <jorge.marques@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD4062");
    MODULE_LICENSE("GPL");
