//! Automatically rewritten from C to Rust
//! Source: drivers/iio/buffer/industrialio-triggered-buffer.c
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
// Copyright (c) 2012 Analog Devices, Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

//
// iio_triggered_buffer_setup_ext() - Setup triggered buffer and pollfunc
// @indio_dev:		IIO device structure
// @h:			Function which will be used as pollfunc top half
// @thread:		Function which will be used as pollfunc bottom half
// @direction:		Direction of the data stream (in/out).
// @setup_ops:		Buffer setup functions to use for this device.
// If NULL the default setup functions for triggered
// buffers will be used.
// @buffer_attrs:	Extra sysfs buffer attributes for this IIO buffer
//
// This function combines some common tasks which will normally be performed
// when setting up a triggered buffer. It will allocate the buffer and the
// pollfunc.
//
// Before calling this function the indio_dev structure should already be
// completely initialized, but not yet registered. In practice this means that
// this function should be called right before iio_device_register().
//
// To free the resources allocated by this function call
// iio_triggered_buffer_cleanup().
//
    int iio_triggered_buffer_setup_ext(struct iio_dev *indio_dev,
    irqreturn_t (*h)(int irq, void *p),
    irqreturn_t (*thread)(int irq, void *p),
    enum iio_buffer_direction direction,
    const struct iio_buffer_setup_ops *setup_ops,
    const struct iio_dev_attr **buffer_attrs)
    {
    struct iio_buffer *buffer;
    int ret;
//
// iio_triggered_buffer_cleanup() assumes that the buffer allocated here
// is assigned to indio_dev->buffer but this is only the case if this
// function is the first caller to iio_device_attach_buffer(). If
// indio_dev->buffer is already set then we can't proceed otherwise the
// cleanup function will try to free a buffer that was not allocated here.
//
    if (indio_dev.buffer)
    return -EADDRINUSE;
    buffer = iio_kfifo_allocate();
    if (!buffer) {
    ret = -ENOMEM;
    goto error_ret;
    }
    indio_dev.pollfunc = iio_alloc_pollfunc(h,
    thread,
    IRQF_ONESHOT,
    indio_dev,
    "%s_consumer%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (indio_dev.pollfunc == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto error_kfifo_free;
    }
// Ring buffer functions - here trigger setup related
    indio_dev.setup_ops = setup_ops;
// Flag that polled ring buffering is possible
    indio_dev.modes |= INDIO_BUFFER_TRIGGERED;
    buffer.direction = direction;
    buffer.attrs = buffer_attrs;
    ret = iio_device_attach_buffer(indio_dev, buffer);
    if (ret < 0)
    goto error_dealloc_pollfunc;
    return 0;
    error_dealloc_pollfunc:
    iio_dealloc_pollfunc(indio_dev.pollfunc);
    error_kfifo_free:
    iio_kfifo_free(buffer);
    error_ret:
    return ret;
    }
    EXPORT_SYMBOL(iio_triggered_buffer_setup_ext);
//
// iio_triggered_buffer_cleanup() - Free resources allocated by iio_triggered_buffer_setup_ext()
// @indio_dev: IIO device structure
//
#[no_mangle]
pub unsafe extern "C" fn iio_triggered_buffer_cleanup(indio_dev: *mut iio_dev) {
    void iio_triggered_buffer_cleanup(struct iio_dev *indio_dev)
    {
    iio_dealloc_pollfunc(indio_dev.pollfunc);
    iio_kfifo_free(indio_dev.buffer);
    }
    EXPORT_SYMBOL(iio_triggered_buffer_cleanup);
#[no_mangle]
unsafe extern "C" fn devm_iio_triggered_buffer_clean(indio_dev: *mut c_void) {
    static void devm_iio_triggered_buffer_clean(void *indio_dev)
    {
    iio_triggered_buffer_cleanup(indio_dev);
    }
    int devm_iio_triggered_buffer_setup_ext(struct device *dev,
    struct iio_dev *indio_dev,
    irqreturn_t (*h)(int irq, void *p),
    irqreturn_t (*thread)(int irq, void *p),
    enum iio_buffer_direction direction,
    const struct iio_buffer_setup_ops *ops,
    const struct iio_dev_attr **buffer_attrs)
    {
    int ret;
    ret = iio_triggered_buffer_setup_ext(indio_dev, h, thread, direction,
    ops, buffer_attrs);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, devm_iio_triggered_buffer_clean,
    indio_dev);
    }
    EXPORT_SYMBOL_GPL(devm_iio_triggered_buffer_setup_ext);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("IIO helper functions for setting up triggered buffers");
    MODULE_LICENSE("GPL");
