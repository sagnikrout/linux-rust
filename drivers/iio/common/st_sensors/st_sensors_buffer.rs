//! Automatically rewritten from C to Rust
//! Source: drivers/iio/common/st_sensors/st_sensors_buffer.c
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
// STMicroelectronics sensors buffer library driver
//
// Copyright 2012-2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

#[no_mangle]
unsafe extern "C" fn st_sensors_get_buffer_element(indio_dev: *mut iio_dev, buf: *mut u8) -> c_int {
    static int st_sensors_get_buffer_element(struct iio_dev *indio_dev, u8 *buf)
    {
    struct st_sensor_data *sdata = iio_priv(indio_dev);
    let mut num_data_channels: c_uint = sdata.num_data_channels;
    int i;
    for_each_set_bit(i, indio_dev.active_scan_mask, num_data_channels) {
    const struct iio_chan_spec *channel = &indio_dev.channels[i];
    unsigned int bytes_to_read =
    DIV_ROUND_UP(channel.scan_type.realbits +
    channel.scan_type.shift, 8);
    unsigned int storage_bytes =
    channel.scan_type.storagebits >> 3;
    buf = PTR_ALIGN(buf, storage_bytes);
    if (regmap_bulk_read(sdata.regmap, channel.address,
    buf, bytes_to_read) < 0)
    return -EIO;
// Advance the buffer pointer
    buf += storage_bytes;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn st_sensors_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    irqreturn_t st_sensors_trigger_handler(int irq, void *p)
    {
    int len;
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct st_sensor_data *sdata = iio_priv(indio_dev);
    s64 timestamp;
//
// If we do timestamping here, do it before reading the values, because
// once we've read the values, new interrupts can occur (when using
// the hardware trigger) and the hw_timestamp may get updated.
// By storing it in a local variable first, we are safe.
//
    if (iio_trigger_using_own(indio_dev))
    timestamp = sdata.hw_timestamp;
    else
    timestamp = iio_get_time_ns(indio_dev);
    len = st_sensors_get_buffer_element(indio_dev, sdata.buffer_data);
    if (len < 0)
    goto st_sensors_get_buffer_element_error;
    iio_push_to_buffers_with_timestamp(indio_dev, sdata.buffer_data,
    timestamp);
    st_sensors_get_buffer_element_error:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    EXPORT_SYMBOL_NS(st_sensors_trigger_handler, "IIO_ST_SENSORS");
