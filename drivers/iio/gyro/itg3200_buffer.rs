//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/itg3200_buffer.c
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
// itg3200_buffer.c -- support InvenSense ITG3200
// Digital 3-Axis Gyroscope driver
//
// Copyright (c) 2011 Christian Strobel <christian.strobel@iis.fraunhofer.de>
// Copyright (c) 2011 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
// Copyright (c) 2012 Thorsten Nowak <thorsten.nowak@iis.fraunhofer.de>
//

#[no_mangle]
unsafe extern "C" fn itg3200_read_all_channels(i2c: *mut i2c_client, buf: *mut __be16) -> c_int {
    static int itg3200_read_all_channels(struct i2c_client *i2c, __be16 *buf)
    {
    let mut tx: u8 = 0x80 | ITG3200_REG_TEMP_OUT_H;
    struct i2c_msg msg[2] = {
    {
    .addr = i2c.addr,
    .flags = i2c.flags,
    .len = 1,
    .buf = &tx,
    },
    {
    .addr = i2c.addr,
    .flags = i2c.flags | I2C_M_RD,
    .len = ITG3200_SCAN_ELEMENTS * sizeof(s16),
    .buf = (char *)buf,
    },
    };
    return i2c_transfer(i2c.adapter, msg, 2);
    }
#[no_mangle]
unsafe extern "C" fn itg3200_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t itg3200_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct itg3200 *st = iio_priv(indio_dev);
//
// Ensure correct alignment and padding including for the
// timestamp that may be inserted.
//
    struct {
    __be16 buf[ITG3200_SCAN_ELEMENTS];
    aligned_s64 ts;
    } scan;
    let mut ret: c_int = itg3200_read_all_channels(st.i2c, scan.buf);
    if (ret < 0)
    goto error_ret;
    iio_push_to_buffers_with_timestamp(indio_dev, &scan, pf.timestamp);
    error_ret:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn itg3200_buffer_configure(indio_dev: *mut iio_dev) -> c_int {
    int itg3200_buffer_configure(struct iio_dev *indio_dev)
    {
    return iio_triggered_buffer_setup(indio_dev, &iio_pollfunc_store_time,
    itg3200_trigger_handler, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn itg3200_buffer_unconfigure(indio_dev: *mut iio_dev) {
    void itg3200_buffer_unconfigure(struct iio_dev *indio_dev)
    {
    iio_triggered_buffer_cleanup(indio_dev);
    }
    static int itg3200_data_rdy_trigger_set_state(struct iio_trigger *trig,
    bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    int ret;
    u8 msc;
    ret = itg3200_read_reg_8(indio_dev, ITG3200_REG_IRQ_CONFIG, &msc);
    if (ret)
    goto error_ret;
    if (state)
    msc |= ITG3200_IRQ_DATA_RDY_ENABLE;
    else
    msc &= ~ITG3200_IRQ_DATA_RDY_ENABLE;
    ret = itg3200_write_reg_8(indio_dev, ITG3200_REG_IRQ_CONFIG, msc);
    if (ret)
    goto error_ret;
    error_ret:
    return ret;
    }
    static const struct iio_trigger_ops itg3200_trigger_ops = {
    .set_trigger_state = &itg3200_data_rdy_trigger_set_state,
    };
#[no_mangle]
pub unsafe extern "C" fn itg3200_probe_trigger(indio_dev: *mut iio_dev) -> c_int {
    int itg3200_probe_trigger(struct iio_dev *indio_dev)
    {
    int ret;
    struct itg3200 *st = iio_priv(indio_dev);
    st.trig = iio_trigger_alloc(&st.i2c.dev, "%s-dev%d", indio_dev.name,
    iio_device_id(indio_dev));
    if (!st.trig)
    return -ENOMEM;
    ret = request_irq(st.i2c.irq, &iio_trigger_generic_data_rdy_poll,
    IRQF_TRIGGER_RISING | IRQF_NO_THREAD,
    "itg3200_data_rdy", st.trig);
    if (ret)
    goto error_free_trig;
    st.trig.ops = &itg3200_trigger_ops;
    iio_trigger_set_drvdata(st.trig, indio_dev);
    ret = iio_trigger_register(st.trig);
    if (ret)
    goto error_free_irq;
// select default trigger
    indio_dev.trig = iio_trigger_get(st.trig);
    return 0;
    error_free_irq:
    free_irq(st.i2c.irq, st.trig);
    error_free_trig:
    iio_trigger_free(st.trig);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn itg3200_remove_trigger(indio_dev: *mut iio_dev) {
    void itg3200_remove_trigger(struct iio_dev *indio_dev)
    {
    struct itg3200 *st = iio_priv(indio_dev);
    iio_trigger_unregister(st.trig);
    free_irq(st.i2c.irq, st.trig);
    iio_trigger_free(st.trig);
    }
