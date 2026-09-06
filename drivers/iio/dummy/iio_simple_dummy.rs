//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dummy/iio_simple_dummy.h
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
// Join together the various functionality of iio_simple_dummy driver
//

//
// struct iio_dummy_state - device instance specific state.
// @dac_val:			cache for dac value
// @single_ended_adc_val:	cache for single ended adc value
// @differential_adc_val:	cache for differential adc value
// @accel_val:			cache for acceleration value
// @accel_calibbias:		cache for acceleration calibbias
// @accel_calibscale:		cache for acceleration calibscale
// @lock:			lock to ensure state is consistent
// @event_irq:			irq number for event line (faked)
// @event_val:			cache for event threshold value
// @event_en:			cache of whether event is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dummy_state {
    pub dac_val: c_int,
    pub single_ended_adc_val: c_int,
    pub differential_adc_val: [c_int; 2],
    pub accel_val: c_int,
    pub accel_calibbias: c_int,
    pub activity_running: c_int,
    pub activity_walking: c_int,
    pub accel_calibscale: *const iio_dummy_accel_calibscale,
    pub lock: mutex,
    pub regs: *mut iio_dummy_regs,
    pub steps_enabled: c_int,
    pub steps: c_int,
    pub height: c_int,

    pub event_irq: c_int,
    pub event_val: c_int,
    pub event_en: bool,
    pub event_timestamp: i64,

}

extern "C" {
    pub fn iio_simple_dummy_events_register(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_simple_dummy_events_unregister(indio_dev: *mut iio_dev);
}

//
// enum iio_simple_dummy_scan_elements - scan index enum
// @DUMMY_INDEX_VOLTAGE_0:         the single ended voltage channel
// @DUMMY_INDEX_DIFFVOLTAGE_1M2:   first differential channel
// @DUMMY_INDEX_DIFFVOLTAGE_3M4:   second differential channel
// @DUMMY_INDEX_ACCELX:            acceleration channel
//
// Enum provides convenient numbering for the scan index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_simple_dummy_scan_elements {
    DUMMY_INDEX_VOLTAGE_0,
    DUMMY_INDEX_DIFFVOLTAGE_1M2,
    DUMMY_INDEX_DIFFVOLTAGE_3M4,
    DUMMY_INDEX_ACCELX,
}

extern "C" {
    pub fn iio_simple_dummy_configure_buffer(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_simple_dummy_unconfigure_buffer(indio_dev: *mut iio_dev);
}

