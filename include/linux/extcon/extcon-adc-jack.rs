//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/extcon/extcon-adc-jack.h
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
// include/linux/extcon/extcon-adc-jack.h
//
// Analog Jack extcon driver with ADC-based detection capability.
//
// Copyright (C) 2012 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

//
// struct adc_jack_cond - condition to use an extcon state
// denotes the last adc_jack_cond element among the array)
// @id:			the unique id of each external connector
// @min_adc:		min adc value for this condition
// @max_adc:		max adc value for this condition
//
// For example, if { .state = 0x3, .min_adc = 100, .max_adc = 200}, it means
// that if ADC value is between (inclusive) 100 and 200, than the cable 0 and
// 1 are attached (1<<0 | 1<<1 == 0x3)
//
// Note that you don't need to describe condition for "no cable attached"
// because when no adc_jack_cond is met, state = 0 is automatically chosen.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_jack_cond {
    pub id: c_uint,
    pub min_adc: u32,
    pub max_adc: u32,
}

//
// struct adc_jack_pdata - platform data for adc jack device.
// @name:		name of the extcon device. If null, "adc-jack" is used.
// @consumer_channel:	Unique name to identify the channel on the consumer
// side. This typically describes the channels used within
// the consumer. E.g. 'battery_voltage'
// @cable_names:	array of extcon id for supported cables.
// @adc_contitions:	array of struct adc_jack_cond conditions ending
// with .state = 0 entry. This describes how to decode
// adc values into extcon state.
// @irq_flags:		irq flags used for the @irq
// @handling_delay_ms:	in some devices, we need to read ADC value some
// milli-seconds after the interrupt occurs. You may
// describe such delays with @handling_delay_ms, which
// is rounded-off by jiffies.
// @wakeup_source:	flag to wake up the system for extcon events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_jack_pdata {
    pub name: *const c_char,
    pub consumer_channel: *const c_char,
    pub cable_names: *const c_uint,
// The last entry's state should be 0
    pub adc_conditions: *mut adc_jack_cond,
    pub irq_flags: c_ulong,
    pub /: *mut *mut unsigned long handling_delay_ms; / in ms,
    pub wakeup_source: bool,
}
