//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/utils.h
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
// '18446744073709551615\0'
//
pub const BUFF_U64_STR_SIZE: c_int = 24;
pub const MAX_PATH: c_int = 1024;
pub const MAX_NICE: c_int = 20;

// Calculate string length at compile time (excluding null terminator)

// Compare string with static string, length determined at compile time

//
// str_has_prefix - Test if a string has a given prefix
// @str: The string to test
// @prefix: The string to see if @str starts with
//
// Returns: true if @str starts with @prefix, false otherwise
//
extern "C" {
    pub fn debug_msg(fmt: *const c_char, ...);
}
extern "C" {
    pub fn err_msg(fmt: *const c_char, ...);
}
extern "C" {
    pub fn fatal(fmt: *const c_char, ...);
}
extern "C" {
    pub fn parse_seconds_duration(val: *mut c_char) -> c_long;
}
extern "C" {
    pub fn get_duration(start_time: time_t, output: *mut c_char, output_size: c_int);
}
extern "C" {
    pub fn get_llong_from_str(start: *mut c_char) -> c_longlong;
}
// a = *b;
// a += *b;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_attr {
    pub size: u32,
    pub sched_policy: u32,
    pub sched_flags: u64,
    pub sched_nice: i32,
    pub sched_priority: u32,
    pub sched_runtime: u64,
    pub sched_deadline: u64,
    pub sched_period: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_format {
    STACK_FORMAT_TRUNCATE,
    STACK_FORMAT_SKIP,
    STACK_FORMAT_FULL
}

extern "C" {
    pub fn parse_prio(arg: *mut c_char, sched_param: *mut sched_attr) -> c_int;
}
extern "C" {
    pub fn parse_cpu_set(cpu_list: *mut c_char, set: *mut cpu_set_t) -> c_int;
}
extern "C" {
    pub fn parse_stack_format(arg: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __set_sched_attr(pid: c_int, attr: *mut sched_attr) -> c_int;
}
extern "C" {
    pub fn set_comm_sched_attr(comm_prefix: *const c_char, attr: *mut sched_attr) -> c_int;
}
extern "C" {
    pub fn set_comm_cgroup(comm_prefix: *const c_char, cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_pid_cgroup(pid: pid_t, cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_cpu_dma_latency(latency: i32) -> c_int;
}

extern "C" {
    pub fn save_cpu_idle_disable_state(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn restore_cpu_idle_disable_state(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn free_cpu_idle_disable_states();
}
extern "C" {
    pub fn set_deepest_cpu_idle_state(cpu: c_uint, state: c_uint) -> c_int;
}

extern "C" {
    pub fn auto_house_keeping(monitored_cpus: *mut cpu_set_t) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum result {
    PASSED	= EXIT_SUCCESS,
    ERROR	= EXIT_FAILURE,
    FAILED, /* test hit the stop tracing condition */
}
