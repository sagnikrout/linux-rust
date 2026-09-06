//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/counter.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Counter interface
// Copyright (C) 2018 William Breathitt Gray
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_comp_type {
    COUNTER_COMP_U8,
    COUNTER_COMP_U64,
    COUNTER_COMP_BOOL,
    COUNTER_COMP_SIGNAL_LEVEL,
    COUNTER_COMP_FUNCTION,
    COUNTER_COMP_SYNAPSE_ACTION,
    COUNTER_COMP_ENUM,
    COUNTER_COMP_COUNT_DIRECTION,
    COUNTER_COMP_COUNT_MODE,
    COUNTER_COMP_SIGNAL_POLARITY,
    COUNTER_COMP_ARRAY,
}

//
// struct counter_comp - Counter component node
// @type:		Counter component data type
// @name:		device-specific component name
// @priv:		component-relevant data
// @action_read:	Synapse action mode read callback. The read value of the
// respective Synapse action mode should be passed back via
// the action parameter.
// @device_u8_read:	Device u8 component read callback. The read value of the
// respective Device u8 component should be passed back via
// the val parameter.
// @count_u8_read:	Count u8 component read callback. The read value of the
// respective Count u8 component should be passed back via
// the val parameter.
// @signal_u8_read:	Signal u8 component read callback. The read value of the
// respective Signal u8 component should be passed back via
// the val parameter.
// @device_u32_read:	Device u32 component read callback. The read value of
// the respective Device u32 component should be passed
// back via the val parameter.
// @count_u32_read:	Count u32 component read callback. The read value of the
// respective Count u32 component should be passed back via
// the val parameter.
// @signal_u32_read:	Signal u32 component read callback. The read value of
// the respective Signal u32 component should be passed
// back via the val parameter.
// @device_u64_read:	Device u64 component read callback. The read value of
// the respective Device u64 component should be passed
// back via the val parameter.
// @count_u64_read:	Count u64 component read callback. The read value of the
// respective Count u64 component should be passed back via
// the val parameter.
// @signal_u64_read:	Signal u64 component read callback. The read value of
// the respective Signal u64 component should be passed
// back via the val parameter.
// @signal_array_u32_read:	Signal u32 array component read callback. The
// index of the respective Count u32 array
// component element is passed via the idx
// parameter. The read value of the respective
// Count u32 array component element should be
// passed back via the val parameter.
// @device_array_u64_read:	Device u64 array component read callback. The
// index of the respective Device u64 array
// component element is passed via the idx
// parameter. The read value of the respective
// Device u64 array component element should be
// passed back via the val parameter.
// @count_array_u64_read:	Count u64 array component read callback. The
// index of the respective Count u64 array
// component element is passed via the idx
// parameter. The read value of the respective
// Count u64 array component element should be
// passed back via the val parameter.
// @signal_array_u64_read:	Signal u64 array component read callback. The
// index of the respective Count u64 array
// component element is passed via the idx
// parameter. The read value of the respective
// Count u64 array component element should be
// passed back via the val parameter.
// @action_write:	Synapse action mode write callback. The write value of
// the respective Synapse action mode is passed via the
// action parameter.
// @device_u8_write:	Device u8 component write callback. The write value of
// the respective Device u8 component is passed via the val
// parameter.
// @count_u8_write:	Count u8 component write callback. The write value of
// the respective Count u8 component is passed via the val
// parameter.
// @signal_u8_write:	Signal u8 component write callback. The write value of
// the respective Signal u8 component is passed via the val
// parameter.
// @device_u32_write:	Device u32 component write callback. The write value of
// the respective Device u32 component is passed via the
// val parameter.
// @count_u32_write:	Count u32 component write callback. The write value of
// the respective Count u32 component is passed via the val
// parameter.
// @signal_u32_write:	Signal u32 component write callback. The write value of
// the respective Signal u32 component is passed via the
// val parameter.
// @device_u64_write:	Device u64 component write callback. The write value of
// the respective Device u64 component is passed via the
// val parameter.
// @count_u64_write:	Count u64 component write callback. The write value of
// the respective Count u64 component is passed via the val
// parameter.
// @signal_u64_write:	Signal u64 component write callback. The write value of
// the respective Signal u64 component is passed via the
// val parameter.
// @signal_array_u32_write:	Signal u32 array component write callback. The
// index of the respective Signal u32 array
// component element is passed via the idx
// parameter. The write value of the respective
// Signal u32 array component element is passed via
// the val parameter.
// @device_array_u64_write:	Device u64 array component write callback. The
// index of the respective Device u64 array
// component element is passed via the idx
// parameter. The write value of the respective
// Device u64 array component element is passed via
// the val parameter.
// @count_array_u64_write:	Count u64 array component write callback. The
// index of the respective Count u64 array
// component element is passed via the idx
// parameter. The write value of the respective
// Count u64 array component element is passed via
// the val parameter.
// @signal_array_u64_write:	Signal u64 array component write callback. The
// index of the respective Signal u64 array
// component element is passed via the idx
// parameter. The write value of the respective
// Signal u64 array component element is passed via
// the val parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_comp {
    pub type: counter_comp_type,
    pub name: *const c_char,
    pub priv: *mut c_void,
    pub action): *mut counter_synapse_action,
    pub val): *mut *mut *mut int (device_u8_read)(struct counter_device counter, u8,
    pub val): *mut *mut counter_count count, u8,
    pub val): *mut *mut counter_signal signal, u8,
    pub val): *mut u32,
    pub val): *mut *mut counter_count count, u32,
    pub val): *mut *mut counter_signal signal, u32,
    pub val): *mut u64,
    pub val): *mut *mut counter_count count, u64,
    pub val): *mut *mut counter_signal signal, u64,
    pub val): *mut size_t idx, u32,
    pub val): *mut size_t idx, u64,
    pub val): *mut size_t idx, u64,
    pub val): *mut size_t idx, u64,
}

//
// struct counter_signal - Counter Signal node
// @id:		unique ID used to identify the Signal
// @name:	device-specific Signal name
// @ext:	optional array of Signal extensions
// @num_ext:	number of Signal extensions specified in @ext
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_signal {
    pub id: c_int,
    pub name: *const c_char,
    pub ext: *mut counter_comp,
    pub num_ext: usize,
}

//
// struct counter_synapse - Counter Synapse node
// @actions_list:	array of available action modes
// @num_actions:	number of action modes specified in @actions_list
// @signal:		pointer to the associated Signal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_synapse {
    pub actions_list: *const counter_synapse_action,
    pub num_actions: usize,
    pub signal: *mut counter_signal,
}

//
// struct counter_count - Counter Count node
// @id:			unique ID used to identify the Count
// @name:		device-specific Count name
// @functions_list:	array of available function modes
// @num_functions:	number of function modes specified in @functions_list
// @synapses:		array of Synapses for initialization
// @num_synapses:	number of Synapses specified in @synapses
// @ext:		optional array of Count extensions
// @num_ext:		number of Count extensions specified in @ext
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_count {
    pub id: c_int,
    pub name: *const c_char,
    pub functions_list: *const counter_function,
    pub num_functions: usize,
    pub synapses: *mut counter_synapse,
    pub num_synapses: usize,
    pub ext: *mut counter_comp,
    pub num_ext: usize,
}

//
// struct counter_event_node - Counter Event node
// @l:		list of current watching Counter events
// @event:	event that triggers
// @channel:	event channel
// @comp_list:	list of components to watch when event triggers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_event_node {
    pub l: list_head,
    pub event: u8,
    pub channel: u8,
    pub comp_list: list_head,
}

//
// struct counter_ops - Callbacks from driver
// @signal_read:	optional read callback for Signals. The read level of
// the respective Signal should be passed back via the
// level parameter.
// @count_read:		read callback for Counts. The read value of the
// respective Count should be passed back via the value
// parameter.
// @count_write:	optional write callback for Counts. The write value for
// the respective Count is passed in via the value
// parameter.
// @function_read:	read callback the Count function modes. The read
// function mode of the respective Count should be passed
// back via the function parameter.
// @function_write:	optional write callback for Count function modes. The
// function mode to write for the respective Count is
// passed in via the function parameter.
// @action_read:	optional read callback the Synapse action modes. The
// read action mode of the respective Synapse should be
// passed back via the action parameter.
// @action_write:	optional write callback for Synapse action modes. The
// action mode to write for the respective Synapse is
// passed in via the action parameter.
// @events_configure:	optional write callback to configure events. The list of
// struct counter_event_node may be accessed via the
// events_list member of the counter parameter.
// @watch_validate:	optional callback to validate a watch. The Counter
// component watch configuration is passed in via the watch
// parameter. A return value of 0 indicates a valid Counter
// component watch configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_ops {
    pub level): *mut counter_signal_level,
    pub value): *mut *mut counter_count count, u64,
    pub value): *mut *mut counter_count count, u64,
    pub function): *mut counter_function,
    pub function): counter_function,
    pub action): *mut counter_synapse_action,
    pub action): counter_synapse_action,
    pub counter): *mut *mut int (events_configure)(struct counter_device,
    pub watch): *const counter_watch,
}

//
// struct counter_device - Counter data structure
// @name:		name of the device
// @parent:		optional parent device providing the counters
// @ops:		callbacks from driver
// @signals:		array of Signals
// @num_signals:	number of Signals specified in @signals
// @counts:		array of Counts
// @num_counts:		number of Counts specified in @counts
// @ext:		optional array of Counter device extensions
// @num_ext:		number of Counter device extensions specified in @ext
// @dev:		internal device structure
// @chrdev:		internal character device structure
// @events_list:	list of current watching Counter events
// @events_list_lock:	lock to protect Counter events list operations
// @next_events_list:	list of next watching Counter events
// @n_events_list_lock:	lock to protect Counter next events list operations
// @events:		queue of detected Counter events
// @events_wait:	wait queue to allow blocking reads of Counter events
// @events_in_lock:	lock to protect Counter events queue in operations
// @events_out_lock:	lock to protect Counter events queue out operations
// @ops_exist_lock:	lock to prevent use during removal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_device {
    pub name: *const c_char,
    pub parent: *mut device,
    pub ops: *const counter_ops,
    pub signals: *mut counter_signal,
    pub num_signals: usize,
    pub counts: *mut counter_count,
    pub num_counts: usize,
    pub ext: *mut counter_comp,
    pub num_ext: usize,
    pub dev: device,
    pub chrdev: cdev,
    pub events_list: list_head,
    pub events_list_lock: spinlock_t,
    pub next_events_list: list_head,
    pub n_events_list_lock: mutex,
    pub counter_event): DECLARE_KFIFO_PTR(events, struct,
    pub events_wait: wait_queue_head_t,
    pub events_in_lock: spinlock_t,
    pub events_out_lock: mutex,
    pub ops_exist_lock: mutex,
}

extern "C" {
    pub fn counter_put(counter: *const *const counter_device);
}
extern "C" {
    pub fn counter_add(counter: *const *const counter_device) -> c_int;
}
extern "C" {
    pub fn counter_unregister(counter: *const *const counter_device);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_available {
    pub enums: *const u32,
    pub strs: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_array {
    pub type: counter_comp_type,
    pub avail: *const counter_available,
    pub length: usize,
    pub idx: usize,
}

