//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/as3935.c
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
// as3935.c - Support for AS3935 Franklin lightning sensor
//
// Copyright (C) 2014, 2017-2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const AS3935_AFE_GAIN: c_uint = 0x00;
pub const AS3935_AFE_MASK: c_uint = 0x3F;
pub const AS3935_AFE_GAIN_MAX: c_uint = 0x1F;

pub const AS3935_NFLWDTH: c_uint = 0x01;
pub const AS3935_NFLWDTH_MASK: c_uint = 0x7f;
pub const AS3935_INT: c_uint = 0x03;
pub const AS3935_INT_MASK: c_uint = 0x0f;

pub const AS3935_DATA: c_uint = 0x07;
pub const AS3935_DATA_MASK: c_uint = 0x3F;
pub const AS3935_TUNE_CAP: c_uint = 0x08;
pub const AS3935_DEFAULTS: c_uint = 0x3C;
pub const AS3935_CALIBRATE: c_uint = 0x3D;

pub const MAX_PF_CAP: c_int = 120;
pub const TUNE_CAP_DIV: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3935_state {
    pub spi: *mut spi_device,
    pub trig: *mut iio_trigger,
    pub lock: mutex,
    pub work: delayed_work,
    pub noise_tripped: c_ulong,
    pub tune_cap: u32,
    pub nflwdth_reg: u32,
// Ensure timestamp is naturally aligned
    struct {
    pub chan: u8,
    pub timestamp: aligned_s64,
    pub scan: },
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[2],
}

    static const struct iio_chan_spec as3935_channels[] = {
    {
    .type           = IIO_PROXIMITY,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_PROCESSED) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index     = 0,
    .scan_type = {
    .sign           = 'u',
    .realbits       = 6,
    .storagebits    = 8,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
#[no_mangle]
unsafe extern "C" fn as3935_read(st: *mut as3935_state, reg: c_uint, val: *mut c_int) -> c_int {
    static int as3935_read(struct as3935_state *st, unsigned int reg, int *val)
    {
    u8 cmd;
    int ret;
    cmd = (AS3935_READ_DATA | AS3935_ADDRESS(reg)) >> 8;
    ret = spi_w8r8(st.spi, cmd);
    if (ret < 0)
    return ret;
// val = ret;
    return 0;
    }
    static int as3935_write(struct as3935_state *st,
    unsigned int reg,
    unsigned int val)
    {
    u8 *buf = st.buf;
    buf[0] = AS3935_ADDRESS(reg) >> 8;
    buf[1] = val;
    return spi_write(st.spi, buf, 2);
    }
    static ssize_t as3935_sensor_sensitivity_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct as3935_state *st = iio_priv(dev_to_iio_dev(dev));
    int val, ret;
    ret = as3935_read(st, AS3935_AFE_GAIN, &val);
    if (ret)
    return ret;
    val = (val & AS3935_AFE_MASK) >> 1;
    return sysfs_emit(buf, "%d\n", val);
    }
    static ssize_t as3935_sensor_sensitivity_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct as3935_state *st = iio_priv(dev_to_iio_dev(dev));
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return -EINVAL;
    if (val > AS3935_AFE_GAIN_MAX)
    return -EINVAL;
    as3935_write(st, AS3935_AFE_GAIN, val << 1);
    return len;
    }
    static ssize_t as3935_noise_level_tripped_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct as3935_state *st = iio_priv(dev_to_iio_dev(dev));
    int ret;
    mutex_lock(&st.lock);
    ret = sysfs_emit(buf, "%d\n", !time_after(jiffies, st.noise_tripped + HZ));
    mutex_unlock(&st.lock);
    return ret;
    }
    static IIO_DEVICE_ATTR(sensor_sensitivity, S_IRUGO | S_IWUSR,
    as3935_sensor_sensitivity_show, as3935_sensor_sensitivity_store, 0);
    static IIO_DEVICE_ATTR(noise_level_tripped, S_IRUGO,
    as3935_noise_level_tripped_show, core::ptr::null_mut(), 0);
    static struct attribute *as3935_attributes[] = {
    &iio_dev_attr_sensor_sensitivity.dev_attr.attr,
    &iio_dev_attr_noise_level_tripped.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group as3935_attribute_group = {
    .attrs = as3935_attributes,
    };
    static int as3935_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long m)
    {
    struct as3935_state *st = iio_priv(indio_dev);
    int ret;
    switch (m) {
    case IIO_CHAN_INFO_PROCESSED:
    case IIO_CHAN_INFO_RAW:
// val2 = 0;
    ret = as3935_read(st, AS3935_DATA, val);
    if (ret)
    return ret;
// storm out of range
    if (*val == AS3935_DATA_MASK)
    return -EINVAL;
    if (m == IIO_CHAN_INFO_RAW)
    return IIO_VAL_INT;
    if (m == IIO_CHAN_INFO_PROCESSED)
// val *= 1000;
    break;
    case IIO_CHAN_INFO_SCALE:
// val = 1000;
    break;
    default:
    return -EINVAL;
    }
    return IIO_VAL_INT;
    }
    static const struct iio_info as3935_info = {
    .attrs = &as3935_attribute_group,
    .read_raw = &as3935_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn as3935_trigger_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t as3935_trigger_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct as3935_state *st = iio_priv(indio_dev);
    int val, ret;
    ret = as3935_read(st, AS3935_DATA, &val);
    if (ret)
    goto err_read;
    st.scan.chan = val & AS3935_DATA_MASK;
    iio_push_to_buffers_with_ts(indio_dev, &st.scan, sizeof(st.scan),
    iio_get_time_ns(indio_dev));
    err_read:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn as3935_event_work(work: *mut work_struct) {
    static void as3935_event_work(struct work_struct *work)
    {
    struct as3935_state *st;
    int val;
    int ret;
    st = container_of(work, struct as3935_state, work.work);
    ret = as3935_read(st, AS3935_INT, &val);
    if (ret) {
    dev_warn(&st.spi.dev, "read error\n");
    return;
    }
    val &= AS3935_INT_MASK;
    switch (val) {
    case AS3935_EVENT_INT:
    iio_trigger_poll_nested(st.trig);
    break;
    case AS3935_DISTURB_INT:
    case AS3935_NOISE_INT:
    mutex_lock(&st.lock);
    st.noise_tripped = jiffies;
    mutex_unlock(&st.lock);
    dev_warn(&st.spi.dev, "noise level is too high\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn as3935_interrupt_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t as3935_interrupt_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct as3935_state *st = iio_priv(indio_dev);
//
// Delay work for >2 milliseconds after an interrupt to allow
// estimated distance to recalculated.
//
    schedule_delayed_work(&st.work, msecs_to_jiffies(3));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn calibrate_as3935(st: *mut as3935_state) {
    static void calibrate_as3935(struct as3935_state *st)
    {
    as3935_write(st, AS3935_DEFAULTS, 0x96);
    as3935_write(st, AS3935_CALIBRATE, 0x96);
    as3935_write(st, AS3935_TUNE_CAP,
    BIT(5) | (st.tune_cap / TUNE_CAP_DIV));
    mdelay(2);
    as3935_write(st, AS3935_TUNE_CAP, (st.tune_cap / TUNE_CAP_DIV));
    as3935_write(st, AS3935_NFLWDTH, st.nflwdth_reg);
    }
#[no_mangle]
unsafe extern "C" fn as3935_suspend(dev: *mut device) -> c_int {
    static int as3935_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct as3935_state *st = iio_priv(indio_dev);
    int val, ret;
    mutex_lock(&st.lock);
    ret = as3935_read(st, AS3935_AFE_GAIN, &val);
    if (ret)
    goto err_suspend;
    val |= AS3935_AFE_PWR_BIT;
    ret = as3935_write(st, AS3935_AFE_GAIN, val);
    err_suspend:
    mutex_unlock(&st.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn as3935_resume(dev: *mut device) -> c_int {
    static int as3935_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct as3935_state *st = iio_priv(indio_dev);
    int val, ret;
    mutex_lock(&st.lock);
    ret = as3935_read(st, AS3935_AFE_GAIN, &val);
    if (ret)
    goto err_resume;
    val &= ~AS3935_AFE_PWR_BIT;
    ret = as3935_write(st, AS3935_AFE_GAIN, val);
    calibrate_as3935(st);
    err_resume:
    mutex_unlock(&st.lock);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(as3935_pm_ops, as3935_suspend, as3935_resume);
#[no_mangle]
unsafe extern "C" fn as3935_probe(spi: *mut spi_device) -> c_int {
    static int as3935_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct iio_trigger *trig;
    struct as3935_state *st;
    int ret;
// Be sure lightning event interrupt is specified
    if (!spi.irq) {
    dev_err(dev, "unable to get event interrupt\n");
    return -EINVAL;
    }
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    spi_set_drvdata(spi, indio_dev);
    mutex_init(&st.lock);
    ret = device_property_read_u32(dev,
    "ams,tuning-capacitor-pf", &st.tune_cap);
    if (ret) {
    st.tune_cap = 0;
    dev_warn(dev, "no tuning-capacitor-pf set, defaulting to %d",
    st.tune_cap);
    }
    if (st.tune_cap > MAX_PF_CAP) {
    dev_err(dev, "wrong tuning-capacitor-pf setting of %d\n",
    st.tune_cap);
    return -EINVAL;
    }
    ret = device_property_read_u32(dev,
    "ams,nflwdth", &st.nflwdth_reg);
    if (!ret && st.nflwdth_reg > AS3935_NFLWDTH_MASK) {
    dev_err(dev, "invalid nflwdth setting of %d\n",
    st.nflwdth_reg);
    return -EINVAL;
    }
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.channels = as3935_channels;
    indio_dev.num_channels = ARRAY_SIZE(as3935_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &as3935_info;
    trig = devm_iio_trigger_alloc(dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!trig)
    return -ENOMEM;
    st.trig = trig;
    st.noise_tripped = jiffies - HZ;
    iio_trigger_set_drvdata(trig, indio_dev);
    ret = devm_iio_trigger_register(dev, trig);
    if (ret) {
    dev_err(dev, "failed to register trigger\n");
    return ret;
    }
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    iio_pollfunc_store_time,
    as3935_trigger_handler, core::ptr::null_mut());
    if (ret) {
    dev_err(dev, "cannot setup iio trigger\n");
    return ret;
    }
    calibrate_as3935(st);
    ret = devm_delayed_work_autocancel(dev, &st.work, as3935_event_work);
    if (ret)
    return ret;
    ret = devm_request_irq(dev, spi.irq,
    &as3935_interrupt_handler,
    IRQF_TRIGGER_RISING,
    dev_name(dev),
    indio_dev);
    if (ret)
    return ret;
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret < 0) {
    dev_err(dev, "unable to register device\n");
    return ret;
    }
    return 0;
    }
    static const struct of_device_id as3935_of_match[] = {
    { .compatible = "ams,as3935", },
    { }
    };
    MODULE_DEVICE_TABLE(of, as3935_of_match);
    static const struct spi_device_id as3935_id[] = {
    { .name = "as3935" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, as3935_id);
    static struct spi_driver as3935_driver = {
    .driver = {
    .name	= "as3935",
    .of_match_table = as3935_of_match,
    .pm	= pm_sleep_ptr(&as3935_pm_ops),
    },
    .probe		= as3935_probe,
    .id_table	= as3935_id,
    };
    module_spi_driver(as3935_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("AS3935 lightning sensor");
    MODULE_LICENSE("GPL");
