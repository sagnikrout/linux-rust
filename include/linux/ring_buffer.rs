//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ring_buffer.h
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
// Don't refer to this struct directly, use functions below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_event {
    pub time_delta:27: u32 type_len:5,,
    pub array: [u32; ],
}

//
// enum ring_buffer_type - internal ring buffer types
//
// @RINGBUF_TYPE_PADDING:	Left over page padding or discarded event
// If time_delta is 0:
// array is ignored
// size is variable depending on how much
// padding is needed
// If time_delta is non zero:
// array[0] holds the actual length
// size = 4 + length (bytes)
//
// @RINGBUF_TYPE_TIME_EXTEND:	Extend the time delta
// array[0] = time delta (28 .. 59)
// size = 8 bytes
//
// @RINGBUF_TYPE_TIME_STAMP:	Absolute timestamp
// Same format as TIME_EXTEND except that the
// value is an absolute timestamp, not a delta
// event.time_delta contains bottom 27 bits
// array[0] = top (28 .. 59) bits
// size = 8 bytes
//
// <= @RINGBUF_TYPE_DATA_TYPE_LEN_MAX:
// Data record
// If type_len is zero:
// array[0] holds the actual length
// array[1..(length+3)/4] holds data
// size = 4 + length (bytes)
// else
// length = type_len << 2
// array[0..(length+3)/4-1] holds data
// size = 4 + length (bytes)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_buffer_type {
    RINGBUF_TYPE_DATA_TYPE_LEN_MAX = 28,
    RINGBUF_TYPE_PADDING,
    RINGBUF_TYPE_TIME_EXTEND,
    RINGBUF_TYPE_TIME_STAMP,
}

extern "C" {
    pub fn ring_buffer_event_length(event: *mut ring_buffer_event) -> unsigned;
}
//
// ring_buffer_discard_commit will remove an event that has not
// been committed yet. If this is used, then ring_buffer_unlock_commit
// must not be called on the discarded event. This function
// will try to remove the event from the ring buffer completely
// if another event has not been written after it.
//
// Example use:
//
// if (some_condition)
// ring_buffer_discard_commit(buffer, event);
// else
// ring_buffer_unlock_commit(buffer, event);
//
// size is in bytes for each per CPU buffer.
//
// Because the ring buffer is generic, if other users of the ring buffer get
// traced by ftrace, it can produce lockdep warnings. We need to keep each
// ring buffer's lock class separate.
//

//
// Because the ring buffer is generic, if other users of the ring buffer get
// traced by ftrace, it can produce lockdep warnings. We need to keep each
// ring buffer's lock class separate.
//

extern "C" {
    pub fn bool(data: *mut *mut ring_buffer_cond_fn)(void) -> typedef;
}
extern "C" {
    pub fn ring_buffer_wake_waiters(buffer: *mut trace_buffer, cpu: c_int);
}

extern "C" {
    pub fn ring_buffer_free(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_resize(buffer: *mut trace_buffer, size: c_ulong, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn ring_buffer_change_overwrite(buffer: *mut trace_buffer, val: c_int);
}
extern "C" {
    pub fn ring_buffer_unlock_commit(buffer: *mut trace_buffer) -> c_int;
}
extern "C" {
    pub fn ring_buffer_nest_start(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_nest_end(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_read_finish(iter: *mut ring_buffer_iter);
}
extern "C" {
    pub fn ring_buffer_iter_advance(iter: *mut ring_buffer_iter);
}
extern "C" {
    pub fn ring_buffer_iter_reset(iter: *mut ring_buffer_iter);
}
extern "C" {
    pub fn ring_buffer_iter_empty(iter: *mut ring_buffer_iter) -> c_int;
}
extern "C" {
    pub fn ring_buffer_iter_dropped(iter: *mut ring_buffer_iter) -> bool;
}
extern "C" {
    pub fn ring_buffer_size(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_max_event_size(buffer: *mut trace_buffer) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_reset_cpu(buffer: *mut trace_buffer, cpu: c_int);
}
extern "C" {
    pub fn ring_buffer_reset_online_cpus(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_reset(buffer: *mut trace_buffer);
}

extern "C" {
    pub fn ring_buffer_empty(buffer: *mut trace_buffer) -> bool;
}
extern "C" {
    pub fn ring_buffer_empty_cpu(buffer: *mut trace_buffer, cpu: c_int) -> bool;
}
extern "C" {
    pub fn ring_buffer_record_disable(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_record_enable(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_record_off(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_record_on(buffer: *mut trace_buffer);
}
extern "C" {
    pub fn ring_buffer_record_is_on(buffer: *mut trace_buffer) -> bool;
}
extern "C" {
    pub fn ring_buffer_record_is_set_on(buffer: *mut trace_buffer) -> bool;
}
extern "C" {
    pub fn ring_buffer_record_is_on_cpu(buffer: *mut trace_buffer, cpu: c_int) -> bool;
}
extern "C" {
    pub fn ring_buffer_record_disable_cpu(buffer: *mut trace_buffer, cpu: c_int);
}
extern "C" {
    pub fn ring_buffer_record_enable_cpu(buffer: *mut trace_buffer, cpu: c_int);
}
extern "C" {
    pub fn ring_buffer_oldest_event_ts(buffer: *mut trace_buffer, cpu: c_int) -> u64;
}
extern "C" {
    pub fn ring_buffer_bytes_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_entries(buffer: *mut trace_buffer) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_overruns(buffer: *mut trace_buffer) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_entries_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_overrun_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_commit_overrun_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_dropped_events_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_read_events_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn ring_buffer_time_stamp(buffer: *mut trace_buffer) -> u64;
}
extern "C" {
    pub fn ring_buffer_set_time_stamp_abs(buffer: *mut trace_buffer, abs: bool);
}
extern "C" {
    pub fn ring_buffer_time_stamp_abs(buffer: *mut trace_buffer) -> bool;
}
extern "C" {
    pub fn ring_buffer_nr_dirty_pages(buffer: *mut trace_buffer, cpu: c_int) -> usize;
}
extern "C" {
    pub fn ring_buffer_print_entry_header(s: *mut trace_seq) -> c_int;
}
extern "C" {
    pub fn ring_buffer_print_page_header(buffer: *mut trace_buffer, s: *mut trace_seq) -> c_int;
}
extern "C" {
    pub fn ring_buffer_subbuf_order_get(buffer: *mut trace_buffer) -> c_int;
}
extern "C" {
    pub fn ring_buffer_subbuf_order_set(buffer: *mut trace_buffer, order: c_int) -> c_int;
}
extern "C" {
    pub fn ring_buffer_subbuf_size_get(buffer: *mut trace_buffer) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_buffer_flags {
    RB_FL_OVERWRITE		= 1 << 0,
    RB_FL_TESTING		= 1 << 1,
}

extern "C" {
    pub fn trace_rb_cpu_prepare(cpu: c_uint, node: *mut hlist_node) -> c_int;
}

extern "C" {
    pub fn ring_buffer_map_dup(buffer: *mut trace_buffer, cpu: c_int);
}
extern "C" {
    pub fn ring_buffer_unmap(buffer: *mut trace_buffer, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn ring_buffer_map_get_reader(buffer: *mut trace_buffer, cpu: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_desc {
    pub cpu: c_int,
    pub /: *mut *mut unsigned int nr_page_va; / excludes the meta page,
    pub meta_va: c_ulong,
    pub __counted_by(nr_page_va): unsigned long page_va[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_buffer_desc {
    pub nr_cpus: c_int,
    pub struct_len: usize,
    pub /: *mut *mut char __data[]; / list of ring_buffer_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_remote {
    pub desc: *mut trace_buffer_desc,
    pub priv): *mut *mut int (swap_reader_page)(unsigned int cpu, void,
    pub priv): *mut *mut int (reset)(unsigned int cpu, void,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn ring_buffer_poll_remote(buffer: *mut trace_buffer, cpu: c_int) -> c_int;
}

