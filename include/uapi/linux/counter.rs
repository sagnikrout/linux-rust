//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/counter.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Userspace ABI for Counter character devices
// Copyright (C) 2020 William Breathitt Gray
//

// Component type definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_component_type {
    COUNTER_COMPONENT_NONE,
    COUNTER_COMPONENT_SIGNAL,
    COUNTER_COMPONENT_COUNT,
    COUNTER_COMPONENT_FUNCTION,
    COUNTER_COMPONENT_SYNAPSE_ACTION,
    COUNTER_COMPONENT_EXTENSION,
}

// Component scope definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_scope {
    COUNTER_SCOPE_DEVICE,
    COUNTER_SCOPE_SIGNAL,
    COUNTER_SCOPE_COUNT,
}

//
// struct counter_component - Counter component identification
// @type: component type (one of enum counter_component_type)
// @scope: component scope (one of enum counter_scope)
// @parent: parent ID (matching the ID suffix of the respective parent sysfs
// path as described by the ABI documentation file
// Documentation/ABI/testing/sysfs-bus-counter)
// @id: component ID (matching the ID provided by the respective *_component_id
// sysfs attribute of the desired component)
//
// For example, if the Count 2 ceiling extension of Counter device 4 is desired,
// set type equal to COUNTER_COMPONENT_EXTENSION, scope equal to
// COUNTER_SCOPE_COUNT, parent equal to 2, and id equal to the value provided by
// the respective /sys/bus/counter/devices/counter4/count2/ceiling_component_id
// sysfs attribute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_component {
    pub type: __u8,
    pub scope: __u8,
    pub parent: __u8,
    pub id: __u8,
}

// Event type definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_event_type {
// Count value increased past ceiling
    COUNTER_EVENT_OVERFLOW,
// Count value decreased past floor
    COUNTER_EVENT_UNDERFLOW,
// Count value increased past ceiling, or decreased past floor
    COUNTER_EVENT_OVERFLOW_UNDERFLOW,
// Count value reached threshold
    COUNTER_EVENT_THRESHOLD,
// Index signal detected
    COUNTER_EVENT_INDEX,
// State of counter is changed
    COUNTER_EVENT_CHANGE_OF_STATE,
// Count value captured
    COUNTER_EVENT_CAPTURE,
// Direction change detected
    COUNTER_EVENT_DIRECTION_CHANGE,
}

//
// struct counter_watch - Counter component watch configuration
// @component: component to watch when event triggers
// @event: event that triggers (one of enum counter_event_type)
// @channel: event channel (typically 0 unless the device supports concurrent
// events of the same type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_watch {
    pub component: counter_component,
    pub event: __u8,
    pub channel: __u8,
}

//
// Queues a Counter watch for the specified event.
//
// The queued watches will not be applied until COUNTER_ENABLE_EVENTS_IOCTL is
// called.
//

//
// Enables monitoring the events specified by the Counter watches that were
// queued by COUNTER_ADD_WATCH_IOCTL.
//
// If events are already enabled, the new set of watches replaces the old one.
// Calling this ioctl also has the effect of clearing the queue of watches added
// by COUNTER_ADD_WATCH_IOCTL.
//

//
// Stops monitoring the previously enabled events.
//

//
// struct counter_event - Counter event data
// @timestamp: best estimate of time of event occurrence, in nanoseconds
// @value: component value
// @watch: component watch configuration
// @status: return status (system error number)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_event {
    pub timestamp: __aligned_u64,
    pub value: __aligned_u64,
    pub watch: counter_watch,
    pub status: __u8,
}

// Count direction values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_count_direction {
    COUNTER_COUNT_DIRECTION_FORWARD,
    COUNTER_COUNT_DIRECTION_BACKWARD,
}

// Count mode values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_count_mode {
    COUNTER_COUNT_MODE_NORMAL,
    COUNTER_COUNT_MODE_RANGE_LIMIT,
    COUNTER_COUNT_MODE_NON_RECYCLE,
    COUNTER_COUNT_MODE_MODULO_N,
    COUNTER_COUNT_MODE_INTERRUPT_ON_TERMINAL_COUNT,
    COUNTER_COUNT_MODE_HARDWARE_RETRIGGERABLE_ONESHOT,
    COUNTER_COUNT_MODE_RATE_GENERATOR,
    COUNTER_COUNT_MODE_SQUARE_WAVE_MODE,
    COUNTER_COUNT_MODE_SOFTWARE_TRIGGERED_STROBE,
    COUNTER_COUNT_MODE_HARDWARE_TRIGGERED_STROBE,
}

// Count function values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_function {
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_DECREASE,
    COUNTER_FUNCTION_PULSE_DIRECTION,
    COUNTER_FUNCTION_QUADRATURE_X1_A,
    COUNTER_FUNCTION_QUADRATURE_X1_B,
    COUNTER_FUNCTION_QUADRATURE_X2_A,
    COUNTER_FUNCTION_QUADRATURE_X2_B,
    COUNTER_FUNCTION_QUADRATURE_X4,
}

// Signal values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_signal_level {
    COUNTER_SIGNAL_LEVEL_LOW,
    COUNTER_SIGNAL_LEVEL_HIGH,
}

// Action mode values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_synapse_action {
    COUNTER_SYNAPSE_ACTION_NONE,
    COUNTER_SYNAPSE_ACTION_RISING_EDGE,
    COUNTER_SYNAPSE_ACTION_FALLING_EDGE,
    COUNTER_SYNAPSE_ACTION_BOTH_EDGES,
}

// Signal polarity values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_signal_polarity {
    COUNTER_SIGNAL_POLARITY_POSITIVE,
    COUNTER_SIGNAL_POLARITY_NEGATIVE,
}
