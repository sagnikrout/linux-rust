//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/cpumap.h
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
// struct perf_cpu - wrapper around a CPU number.
// @cpu: CPU number, -1 for the "any CPU"/dummy value.
//
// int16_t limits this to 32767 CPUs.  Widening to int requires a libperf
// ABI bump — see tools/lib/perf/TODO for the full scope.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cache {
    pub cache_lvl: c_int,
    pub cache: c_int,
}

//
// perf_cpu_map__new_any_cpu - a map with a singular "any CPU"/dummy -1 value.
//
// perf_cpu_map__new_online_cpus - a map read from
// /sys/devices/system/cpu/online if
// available. If reading wasn't possible a map
// is created using the online processors
// assuming the first 'n' processors are all
// online.
//
// perf_cpu_map__new - create a map from the given cpu_list such as "0-7". If no
// cpu_list argument is provided then
// perf_cpu_map__new_online_cpus is returned.
//
// perf_cpu_map__new_int - create a map with the one given cpu.
extern "C" {
    pub fn perf_cpu_map__put(map: *mut perf_cpu_map) -> LIBPERF_API void;
}
//
// perf_cpu_map__cpu - get the CPU value at the given index. Returns -1 if index
// is invalid.
//
extern "C" {
    pub fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_uint) -> LIBPERF_API struct perf_cpu;
}
//
// perf_cpu_map__nr - for an empty map returns 1, as perf_cpu_map__cpu returns a
// cpu of -1 for an invalid index, this makes an empty map
// look like it contains the "any CPU"/dummy value. Otherwise
// the result is the number CPUs in the map plus one if the
// "any CPU"/dummy value is present.
//
extern "C" {
    pub fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> LIBPERF_API unsigned int;
}
//
// perf_cpu_map__has_any_cpu_or_is_empty - is map either empty or has the "any CPU"/dummy value.
//
extern "C" {
    pub fn perf_cpu_map__has_any_cpu_or_is_empty(map: *const perf_cpu_map) -> LIBPERF_API bool;
}
//
// perf_cpu_map__is_any_cpu_or_is_empty - is map either empty or the "any CPU"/dummy value.
//
extern "C" {
    pub fn perf_cpu_map__is_any_cpu_or_is_empty(map: *const perf_cpu_map) -> LIBPERF_API bool;
}
//
// perf_cpu_map__is_empty - does the map contain no values and it doesn't
// contain the special "any CPU"/dummy value.
//
extern "C" {
    pub fn perf_cpu_map__is_empty(map: *const perf_cpu_map) -> LIBPERF_API bool;
}
//
// perf_cpu_map__min - the minimum CPU value or -1 if empty or just the "any CPU"/dummy value.
//
extern "C" {
    pub fn perf_cpu_map__min(map: *const perf_cpu_map) -> LIBPERF_API struct perf_cpu;
}
//
// perf_cpu_map__max - the maximum CPU value or -1 if empty or just the "any CPU"/dummy value.
//
extern "C" {
    pub fn perf_cpu_map__max(map: *const perf_cpu_map) -> LIBPERF_API struct perf_cpu;
}
extern "C" {
    pub fn perf_cpu_map__has(map: *const perf_cpu_map, cpu: perf_cpu) -> LIBPERF_API bool;
}
//
// perf_cpu_map__any_cpu - Does the map contain the "any CPU"/dummy -1 value?
//
extern "C" {
    pub fn perf_cpu_map__has_any_cpu(map: *const perf_cpu_map) -> LIBPERF_API bool;
}

