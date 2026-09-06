//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/st_pressure_buffer.c
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
// STMicroelectronics pressures driver
//
// Copyright 2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

#[no_mangle]
pub unsafe extern "C" fn st_press_trig_set_state(trig: *mut iio_trigger, state: bool) -> c_int {
    int st_press_trig_set_state(struct iio_trigger *trig, bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    return st_sensors_set_dataready_irq(indio_dev, state);
    }
#[no_mangle]
unsafe extern "C" fn st_press_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int st_press_buffer_postenable(struct iio_dev *indio_dev)
    {
    return st_sensors_set_enable(indio_dev, true);
    }
#[no_mangle]
unsafe extern "C" fn st_press_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int st_press_buffer_predisable(struct iio_dev *indio_dev)
    {
    return st_sensors_set_enable(indio_dev, false);
    }
    static const struct iio_buffer_setup_ops st_press_buffer_setup_ops = {
    .postenable = &st_press_buffer_postenable,
    .predisable = &st_press_buffer_predisable,
    };
#[no_mangle]
pub unsafe extern "C" fn st_press_allocate_ring(indio_dev: *mut iio_dev) -> c_int {
    int st_press_allocate_ring(struct iio_dev *indio_dev)
    {
    return devm_iio_triggered_buffer_setup(indio_dev.dev.parent, indio_dev,
    core::ptr::null_mut(), &st_sensors_trigger_handler, &st_press_buffer_setup_ops);
    }
