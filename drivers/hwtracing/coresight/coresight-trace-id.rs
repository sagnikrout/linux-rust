//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-trace-id.h
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
// Copyright(C) 2022 Linaro Limited. All rights reserved.
// Author: Mike Leach <mike.leach@linaro.org>
//
// Coresight trace ID allocation API
//
// With multi cpu systems, and more additional trace sources a scalable
// trace ID reservation system is required.
//
// The system will allocate Ids on a demand basis, and allow them to be
// released when done.
//
// In order to ensure that a consistent cpu / ID matching is maintained
// throughout a perf cs_etm event session - a session in progress flag will be
// maintained for each sink, and IDs are cleared when all the perf sessions
// complete. This allows the same CPU to be re-allocated its prior ID when
// events are scheduled in and out.
//
// Trace ID maps will be created and initialised to prevent architecturally
// reserved IDs from being allocated.
//
// API permits multiple maps to be maintained - for large systems where
// different sets of cpus trace into different independent sinks.
//

// ID 0 is reserved
pub const CORESIGHT_TRACE_ID_RES_0: c_int = 0;
// ID 0x70 onwards are reserved
pub const CORESIGHT_TRACE_ID_RES_TOP: c_uint = 0x70;
// check an ID is in the valid range

//
// Read and optionally allocate a CoreSight trace ID and associate with a CPU.
//
// Function will read the current trace ID for the associated CPU,
// allocating an new ID if one is not currently allocated.
//
// Numeric ID values allocated use legacy allocation algorithm if possible,
// otherwise any available ID is used.
//
// @cpu: The CPU index to allocate for.
//
// return: CoreSight trace ID or -EINVAL if allocation impossible.
//
extern "C" {
    pub fn coresight_trace_id_get_cpu_id(cpu: c_int) -> c_int;
}
//
// Version of coresight_trace_id_get_cpu_id() that allows the ID map to operate
// on to be provided.
//
extern "C" {
    pub fn coresight_trace_id_get_cpu_id_map(cpu: c_int, id_map: *mut coresight_trace_id_map) -> c_int;
}
//
// Release an allocated trace ID associated with the CPU.
//
// This will release the CoreSight trace ID associated with the CPU.
//
// @cpu: The CPU index to release the associated trace ID.
//
extern "C" {
    pub fn coresight_trace_id_put_cpu_id(cpu: c_int);
}
//
// Version of coresight_trace_id_put_cpu_id() that allows the ID map to operate
// on to be provided.
//
extern "C" {
    pub fn coresight_trace_id_put_cpu_id_map(cpu: c_int, id_map: *mut coresight_trace_id_map);
}
//
// Read the current allocated CoreSight Trace ID value for the CPU.
//
// Fast read of the current value that does not allocate if no ID allocated
// for the CPU.
//
// Used in perf context  where it is known that the value for the CPU will not
// be changing, when perf starts and event on a core and outputs the Trace ID
// for the CPU as a packet in the data file. IDs cannot change during a perf
// session.
//
// This function does not take the lock protecting the ID lists, avoiding
// locking dependency issues with perf locks.
//
// @cpu: The CPU index to read.
//
// return: current value, will be 0 if unallocated.
//
extern "C" {
    pub fn coresight_trace_id_read_cpu_id(cpu: c_int) -> c_int;
}
//
// Version of coresight_trace_id_read_cpu_id() that allows the ID map to operate
// on to be provided.
//
extern "C" {
    pub fn coresight_trace_id_read_cpu_id_map(cpu: c_int, id_map: *mut coresight_trace_id_map) -> c_int;
}
//
// Allocate a CoreSight trace ID for a system component.
//
// Unconditionally allocates a Trace ID, without associating the ID with a CPU.
//
// Used to allocate IDs for system trace sources such as STM.
//
// return: Trace ID or -EINVAL if allocation is impossible.
//
extern "C" {
    pub fn coresight_trace_id_get_system_id() -> c_int;
}
//
// Allocate a CoreSight static trace ID for a system component.
//
// Used to allocate static IDs for system trace sources such as dummy source.
//
// return: Trace ID or -EINVAL if allocation is impossible.
//
extern "C" {
    pub fn coresight_trace_id_get_static_system_id(id: c_int) -> c_int;
}
//
// Release an allocated system trace ID.
//
// Unconditionally release a trace ID allocated to a system component.
//
// @id: value of trace ID allocated.
//
extern "C" {
    pub fn coresight_trace_id_put_system_id(id: c_int);
}
// notifiers for perf session start and stop
//
// Notify the Trace ID allocator that a perf session is starting.
//
// Increase the perf session reference count - called by perf when setting up a
// trace event.
//
// Perf sessions never free trace IDs to ensure that the ID associated with a
// CPU cannot change during their and other's concurrent sessions. Instead,
// this refcount is used so that the last event to finish always frees all IDs.
//
extern "C" {
    pub fn coresight_trace_id_perf_start(id_map: *mut coresight_trace_id_map);
}
//
// Notify the ID allocator that a perf session is stopping.
//
// Decrease the perf session reference count. If this causes the count to go to
// zero, then all Trace IDs will be released.
//
extern "C" {
    pub fn coresight_trace_id_perf_stop(id_map: *mut coresight_trace_id_map);
}
