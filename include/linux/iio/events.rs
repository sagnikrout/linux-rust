//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/events.h
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
// The industrial I/O - event passing to userspace
//
// Copyright (c) 2008-2011 Jonathan Cameron
//

//
// _IIO_EVENT_CODE() - create event identifier
// @chan_type:	Type of the channel. Should be one of enum iio_chan_type.
// @diff:	Whether the event is for an differential channel or not.
// @modifier:	Modifier for the channel. Should be one of enum iio_modifier.
// @direction:	Direction of the event. One of enum iio_event_direction.
// @type:	Type of the event. Should be one of enum iio_event_type.
// @chan:	Channel number for non-differential channels.
// @chan1:	First channel number for differential channels.
// @chan2:	Second channel number for differential channels.
//
// Drivers should use the specialized macros below instead of using this one
// directly.
//

//
// IIO_MOD_EVENT_CODE() - create event identifier for modified (non
// differential) channels
// @chan_type:	Type of the channel. Should be one of enum iio_chan_type.
// @number:	Channel number.
// @modifier:	Modifier for the channel. Should be one of enum iio_modifier.
// @type:	Type of the event. Should be one of enum iio_event_type.
// @direction:	Direction of the event. One of enum iio_event_direction.
//

//
// IIO_UNMOD_EVENT_CODE() - create event identifier for unmodified (non
// differential) channels
// @chan_type:	Type of the channel. Should be one of enum iio_chan_type.
// @number:	Channel number.
// @type:	Type of the event. Should be one of enum iio_event_type.
// @direction:	Direction of the event. One of enum iio_event_direction.
//

//
// IIO_DIFF_EVENT_CODE() - create event identifier for differential channels
// @chan_type:	Type of the channel. Should be one of enum iio_chan_type.
// @chan1:	First channel number for differential channels.
// @chan2:	Second channel number for differential channels.
// @type:	Type of the event. Should be one of enum iio_event_type.
// @direction:	Direction of the event. One of enum iio_event_direction.
//

