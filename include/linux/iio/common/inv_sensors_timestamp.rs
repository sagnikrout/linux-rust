//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/common/inv_sensors_timestamp.h
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
// Copyright (C) 2020 Invensense, Inc.
//
// struct inv_sensors_timestamp_chip - chip internal properties
// @clock_period:	internal clock period in ns
// @jitter:		acceptable jitter in per-mille
// @init_period:	chip initial period at reset in ns
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_sensors_timestamp_chip {
    pub clock_period: u32,
    pub jitter: u32,
    pub init_period: u32,
}

//
// struct inv_sensors_timestamp_interval - timestamps interval
// @lo:	interval lower bound
// @up:	interval upper bound
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_sensors_timestamp_interval {
    pub lo: i64,
    pub up: i64,
}

//
// struct inv_sensors_timestamp_acc - accumulator for computing an estimation
// @val:	current estimation of the value, the mean of all values
// @idx:	current index of the next free place in values table
// @values:	table of all measured values, use for computing the mean
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_sensors_timestamp_acc {
    pub val: u32,
    pub idx: usize,
    pub values: [u32; 32],
}

//
// struct inv_sensors_timestamp - timestamp management states
// @chip:		chip internal characteristics
// @min_period:		minimal acceptable clock period
// @max_period:		maximal acceptable clock period
// @it:			interrupts interval timestamps
// @delta:		interval timestamps between several interrupts
// @delta_counter:	number of data samples in the delta interval
// @timestamp:		store last timestamp for computing next data timestamp
// @mult:		current internal period multiplier
// @new_mult:		new set internal period multiplier (not yet effective)
// @period:		measured current period of the sensor
// @chip_period:	accumulator for computing internal chip period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_sensors_timestamp {
    pub chip: inv_sensors_timestamp_chip,
    pub min_period: u32,
    pub max_period: u32,
    pub it: inv_sensors_timestamp_interval,
    pub delta: inv_sensors_timestamp_interval,
    pub delta_counter: u32,
    pub timestamp: i64,
    pub mult: u32,
    pub new_mult: u32,
    pub period: u32,
    pub chip_period: inv_sensors_timestamp_acc,
}
