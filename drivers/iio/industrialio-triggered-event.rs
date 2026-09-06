//! Automatically rewritten from C to Rust
//! Source: drivers/iio/industrialio-triggered-event.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2015 Cogent Embedded, Inc.
//

//
// iio_triggered_event_setup() - Setup pollfunc_event for triggered event
// @indio_dev:	IIO device structure
// @h:		Function which will be used as pollfunc_event top half
// @thread:	Function which will be used as pollfunc_event bottom half
//
// This function combines some common tasks which will normally be performed
// when setting up a triggered event. It will allocate the pollfunc_event and
// set mode to use it for triggered event.
//
// Before calling this function the indio_dev structure should already be
// completely initialized, but not yet registered. In practice this means that
// this function should be called right before iio_device_register().
//
// To free the resources allocated by this function call
// iio_triggered_event_cleanup().
//
    int iio_triggered_event_setup(struct iio_dev *indio_dev,
    irqreturn_t (*h)(int irq, void *p),
    irqreturn_t (*thread)(int irq, void *p))
    {
    indio_dev.pollfunc_event = iio_alloc_pollfunc(h,
    thread,
    IRQF_ONESHOT,
    indio_dev,
    "%s_consumer%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (indio_dev.pollfunc_event == core::ptr::null_mut())
    return -ENOMEM;
// Flag that events polling is possible
    indio_dev.modes |= INDIO_EVENT_TRIGGERED;
    return 0;
    }
    EXPORT_SYMBOL(iio_triggered_event_setup);
//
// iio_triggered_event_cleanup() - Free resources allocated by iio_triggered_event_setup()
// @indio_dev: IIO device structure
//
#[no_mangle]
pub unsafe extern "C" fn iio_triggered_event_cleanup(indio_dev: *mut iio_dev) {
    void iio_triggered_event_cleanup(struct iio_dev *indio_dev)
    {
    indio_dev.modes &= ~INDIO_EVENT_TRIGGERED;
    iio_dealloc_pollfunc(indio_dev.pollfunc_event);
    }
    EXPORT_SYMBOL(iio_triggered_event_cleanup);
    MODULE_AUTHOR("Vladimir Barinov");
    MODULE_DESCRIPTION("IIO helper functions for setting up triggered events");
    MODULE_LICENSE("GPL");
