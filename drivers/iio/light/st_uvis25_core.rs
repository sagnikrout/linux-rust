//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/st_uvis25_core.c
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
// STMicroelectronics uvis25 sensor driver
//
// Copyright 2017 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

pub const ST_UVIS25_REG_WHOAMI_ADDR: c_uint = 0x0f;
pub const ST_UVIS25_REG_WHOAMI_VAL: c_uint = 0xca;
pub const ST_UVIS25_REG_CTRL1_ADDR: c_uint = 0x20;

pub const ST_UVIS25_REG_CTRL2_ADDR: c_uint = 0x21;

pub const ST_UVIS25_REG_CTRL3_ADDR: c_uint = 0x22;

pub const ST_UVIS25_REG_STATUS_ADDR: c_uint = 0x27;

pub const ST_UVIS25_REG_OUT_ADDR: c_uint = 0x28;
    static const struct iio_chan_spec st_uvis25_channels[] = {
    {
    .type = IIO_UVINDEX,
    .address = ST_UVIS25_REG_OUT_ADDR,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 8,
    .storagebits = 8,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
#[no_mangle]
unsafe extern "C" fn st_uvis25_check_whoami(hw: *mut st_uvis25_hw) -> c_int {
    static int st_uvis25_check_whoami(struct st_uvis25_hw *hw)
    {
    int err, data;
    err = regmap_read(hw.regmap, ST_UVIS25_REG_WHOAMI_ADDR, &data);
    if (err < 0) {
    dev_err(regmap_get_device(hw.regmap),
    "failed to read whoami register\n");
    return err;
    }
    if (data != ST_UVIS25_REG_WHOAMI_VAL) {
    dev_err(regmap_get_device(hw.regmap),
    "wrong whoami {%02x vs %02x}\n",
    data, ST_UVIS25_REG_WHOAMI_VAL);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_set_enable(hw: *mut st_uvis25_hw, enable: bool) -> c_int {
    static int st_uvis25_set_enable(struct st_uvis25_hw *hw, bool enable)
    {
    int err;
    err = regmap_update_bits(hw.regmap, ST_UVIS25_REG_CTRL1_ADDR,
    ST_UVIS25_REG_ODR_MASK, enable);
    if (err < 0)
    return err;
    hw.enabled = enable;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_read_oneshot(hw: *mut st_uvis25_hw, addr: u8, val: *mut c_int) -> c_int {
    static int st_uvis25_read_oneshot(struct st_uvis25_hw *hw, u8 addr, int *val)
    {
    int err;
    err = st_uvis25_set_enable(hw, true);
    if (err < 0)
    return err;
    msleep(1500);
//
// in order to avoid possible race conditions with interrupt
// generation, disable the sensor first and then poll output
// register. That sequence guarantees the interrupt will be reset
// when irq line is unmasked
//
    err = st_uvis25_set_enable(hw, false);
    if (err < 0)
    return err;
    err = regmap_read(hw.regmap, addr, val);
    return err < 0 ? err : IIO_VAL_INT;
    }
    static int st_uvis25_read_raw(struct iio_dev *iio_dev,
    struct iio_chan_spec const *ch,
    int *val, int *val2, long mask)
    {
    int ret;
    if (!iio_device_claim_direct(iio_dev))
    return -EBUSY;
    switch (mask) {
    case IIO_CHAN_INFO_PROCESSED: {
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
//
// mask irq line during oneshot read since the sensor
// does not export the capability to disable data-ready line
// in the register map and it is enabled by default.
// If the line is unmasked during read_raw() it will be set
// active and never reset since the trigger is disabled
//
    if (hw.irq > 0)
    disable_irq(hw.irq);
    ret = st_uvis25_read_oneshot(hw, ch.address, val);
    if (hw.irq > 0)
    enable_irq(hw.irq);
    break;
    }
    default:
    ret = -EINVAL;
    break;
    }
    iio_device_release_direct(iio_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_trigger_handler_thread(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t st_uvis25_trigger_handler_thread(int irq, void *private)
    {
    struct st_uvis25_hw *hw = private;
    int err, status;
    err = regmap_read(hw.regmap, ST_UVIS25_REG_STATUS_ADDR, &status);
    if (err < 0)
    return IRQ_HANDLED;
    if (!(status & ST_UVIS25_REG_UV_DA_MASK))
    return IRQ_NONE;
    iio_trigger_poll_nested(hw.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_allocate_trigger(iio_dev: *mut iio_dev) -> c_int {
    static int st_uvis25_allocate_trigger(struct iio_dev *iio_dev)
    {
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
    struct device *dev = regmap_get_device(hw.regmap);
    let mut irq_active_low: bool = false;
    unsigned long irq_type;
    int err;
    irq_type = irq_get_trigger_type(hw.irq);
    switch (irq_type) {
    case IRQF_TRIGGER_HIGH:
    case IRQF_TRIGGER_RISING:
    break;
    case IRQF_TRIGGER_LOW:
    case IRQF_TRIGGER_FALLING:
    irq_active_low = true;
    break;
    default:
    dev_info(dev, "mode %lx unsupported\n", irq_type);
    return -EINVAL;
    }
    err = regmap_update_bits(hw.regmap, ST_UVIS25_REG_CTRL3_ADDR,
    ST_UVIS25_REG_HL_MASK, irq_active_low);
    if (err < 0)
    return err;
    err = devm_request_threaded_irq(dev, hw.irq, core::ptr::null_mut(),
    st_uvis25_trigger_handler_thread,
    irq_type | IRQF_ONESHOT,
    iio_dev.name, hw);
    if (err)
    return err;
    hw.trig = devm_iio_trigger_alloc(dev, "%s-trigger",
    iio_dev.name);
    if (!hw.trig)
    return -ENOMEM;
    iio_trigger_set_drvdata(hw.trig, iio_dev);
    return devm_iio_trigger_register(dev, hw.trig);
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_buffer_preenable(iio_dev: *mut iio_dev) -> c_int {
    static int st_uvis25_buffer_preenable(struct iio_dev *iio_dev)
    {
    return st_uvis25_set_enable(iio_priv(iio_dev), true);
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_buffer_postdisable(iio_dev: *mut iio_dev) -> c_int {
    static int st_uvis25_buffer_postdisable(struct iio_dev *iio_dev)
    {
    return st_uvis25_set_enable(iio_priv(iio_dev), false);
    }
    static const struct iio_buffer_setup_ops st_uvis25_buffer_ops = {
    .preenable = st_uvis25_buffer_preenable,
    .postdisable = st_uvis25_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn st_uvis25_buffer_handler_thread(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t st_uvis25_buffer_handler_thread(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *iio_dev = pf.indio_dev;
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
    unsigned int val;
    int err;
// Ensure timestamp is naturally aligned
    struct {
    u8 chan;
    aligned_s64 ts;
    } scan = { };
    err = regmap_read(hw.regmap, ST_UVIS25_REG_OUT_ADDR, &val);
    if (err < 0)
    goto out;
    scan.chan = val;
    iio_push_to_buffers_with_ts(iio_dev, &scan, sizeof(scan),
    iio_get_time_ns(iio_dev));
    out:
    iio_trigger_notify_done(hw.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_allocate_buffer(iio_dev: *mut iio_dev) -> c_int {
    static int st_uvis25_allocate_buffer(struct iio_dev *iio_dev)
    {
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
    return devm_iio_triggered_buffer_setup(regmap_get_device(hw.regmap),
    iio_dev, core::ptr::null_mut(),
    st_uvis25_buffer_handler_thread,
    &st_uvis25_buffer_ops);
    }
    static const struct iio_info st_uvis25_info = {
    .read_raw = st_uvis25_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn st_uvis25_init_sensor(hw: *mut st_uvis25_hw) -> c_int {
    static int st_uvis25_init_sensor(struct st_uvis25_hw *hw)
    {
    int err;
    err = regmap_update_bits(hw.regmap, ST_UVIS25_REG_CTRL2_ADDR,
    ST_UVIS25_REG_BOOT_MASK, 1);
    if (err < 0)
    return err;
    msleep(2000);
    return regmap_update_bits(hw.regmap, ST_UVIS25_REG_CTRL1_ADDR,
    ST_UVIS25_REG_BDU_MASK, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn st_uvis25_probe(dev: *mut device, irq: c_int, regmap: *mut regmap) -> c_int {
    int st_uvis25_probe(struct device *dev, int irq, struct regmap *regmap)
    {
    struct st_uvis25_hw *hw;
    struct iio_dev *iio_dev;
    int err;
    iio_dev = devm_iio_device_alloc(dev, sizeof(*hw));
    if (!iio_dev)
    return -ENOMEM;
    dev_set_drvdata(dev, iio_dev);
    hw = iio_priv(iio_dev);
    hw.irq = irq;
    hw.regmap = regmap;
    err = st_uvis25_check_whoami(hw);
    if (err < 0)
    return err;
    iio_dev.modes = INDIO_DIRECT_MODE;
    iio_dev.channels = st_uvis25_channels;
    iio_dev.num_channels = ARRAY_SIZE(st_uvis25_channels);
    iio_dev.name = ST_UVIS25_DEV_NAME;
    iio_dev.info = &st_uvis25_info;
    err = st_uvis25_init_sensor(hw);
    if (err < 0)
    return err;
    if (hw.irq > 0) {
    err = st_uvis25_allocate_buffer(iio_dev);
    if (err < 0)
    return err;
    err = st_uvis25_allocate_trigger(iio_dev);
    if (err)
    return err;
    }
    return devm_iio_device_register(dev, iio_dev);
    }
    EXPORT_SYMBOL_NS(st_uvis25_probe, "IIO_UVIS25");
#[no_mangle]
unsafe extern "C" fn st_uvis25_suspend(dev: *mut device) -> c_int {
    static int st_uvis25_suspend(struct device *dev)
    {
    struct iio_dev *iio_dev = dev_get_drvdata(dev);
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
    return regmap_clear_bits(hw.regmap, ST_UVIS25_REG_CTRL1_ADDR,
    ST_UVIS25_REG_ODR_MASK);
    }
#[no_mangle]
unsafe extern "C" fn st_uvis25_resume(dev: *mut device) -> c_int {
    static int st_uvis25_resume(struct device *dev)
    {
    struct iio_dev *iio_dev = dev_get_drvdata(dev);
    struct st_uvis25_hw *hw = iio_priv(iio_dev);
    if (hw.enabled)
    return regmap_update_bits(hw.regmap, ST_UVIS25_REG_CTRL1_ADDR,
    ST_UVIS25_REG_ODR_MASK, 1);
    return 0;
    }
    EXPORT_NS_SIMPLE_DEV_PM_OPS(st_uvis25_pm_ops, st_uvis25_suspend, st_uvis25_resume, IIO_UVIS25);
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>");
    MODULE_DESCRIPTION("STMicroelectronics uvis25 sensor driver");
    MODULE_LICENSE("GPL v2");
