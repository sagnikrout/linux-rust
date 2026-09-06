//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dummy/iio_simple_dummy_buffer.c
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
// Copyright (c) 2011 Jonathan Cameron
//
// Buffer handling elements of industrial I/O reference driver.
// Uses the kfifo buffer.
//
// To test without hardware use the sysfs trigger.
//

// Some fake data
    static const s16 fakedata[] = {
    [DUMMY_INDEX_VOLTAGE_0] = 7,
    [DUMMY_INDEX_DIFFVOLTAGE_1M2] = -33,
    [DUMMY_INDEX_DIFFVOLTAGE_3M4] = -2,
    [DUMMY_INDEX_ACCELX] = 344,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_scan {
    pub data: [i16; ARRAY_SIZE(fakedata)],
    pub timestamp: aligned_s64,
}

//
// iio_simple_dummy_trigger_h() - the trigger handler function
// @irq: the interrupt number
// @p: private data - always a pointer to the poll func.
//
// This is the guts of buffered capture. On a trigger event occurring,
// if the pollfunc is attached then this handler is called as a threaded
// interrupt (and hence may sleep). It is responsible for grabbing data
// from the device and pushing it into the associated buffer.
//
#[no_mangle]
unsafe extern "C" fn iio_simple_dummy_trigger_h(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t iio_simple_dummy_trigger_h(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct dummy_scan *scan;
    let mut i: c_int = 0, j;
//
// Note that some buses such as SPI require DMA safe buffers which
// cannot be on the stack. Two easy ways to do this:
// - Local kzalloc (as done here)
// - A buffer at the end of the structure accessed via iio_priv()
// that is marked __aligned(IIO_DMA_MINALIGN).
//
    scan = kzalloc_obj(*scan);
    if (!scan)
    goto done;
//
// Three common options here:
// hardware scans:
// certain combinations of channels make up a fast read. The capture
// will consist of all of them. Hence we just call the grab data
// function and fill the buffer without processing.
// software scans:
// can be considered to be random access so efficient reading is just
// a case of minimal bus transactions.
// software culled hardware scans:
// occasionally a driver may process the nearest hardware scan to avoid
// storing elements that are not desired. This is the fiddliest option
// by far.
// Here let's pretend we have random access. And the values are in the
// constant table fakedata.
//
    iio_for_each_active_channel(indio_dev, j)
    scan.data[i++] = fakedata[j];
    iio_push_to_buffers_with_ts(indio_dev, scan, sizeof(*scan),
    iio_get_time_ns(indio_dev));
    kfree(scan);
    done:
//
// Tell the core we are done with this trigger and ready for the
// next one.
//
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static const struct iio_buffer_setup_ops iio_simple_dummy_buffer_setup_ops = {
    };
#[no_mangle]
pub unsafe extern "C" fn iio_simple_dummy_configure_buffer(indio_dev: *mut iio_dev) -> c_int {
    int iio_simple_dummy_configure_buffer(struct iio_dev *indio_dev)
    {
    return iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    iio_simple_dummy_trigger_h,
    &iio_simple_dummy_buffer_setup_ops);
    }
//
// iio_simple_dummy_unconfigure_buffer() - release buffer resources
// @indio_dev: device instance state
//
#[no_mangle]
pub unsafe extern "C" fn iio_simple_dummy_unconfigure_buffer(indio_dev: *mut iio_dev) {
    void iio_simple_dummy_unconfigure_buffer(struct iio_dev *indio_dev)
    {
    iio_triggered_buffer_cleanup(indio_dev);
    }
