//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/cli_p.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_cb_data {
    pub params: *mut osnoise_params,
    pub trace_output: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timerlat_cb_data {
    pub params: *mut timerlat_params,
    pub trace_output: *mut c_char,
}

//
// Non-zero default values for parameters
//
// Range checking for long long and int option callbacks.
//
// Pass a pointer to a const struct as opt->data to enable range checking.
// If opt->data is NULL, no range check is performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llong_range {
    pub min: c_longlong,
    pub max: c_longlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int_range {
    pub min: c_int,
    pub max: c_int,
}

//
// OPT_CALLBACK variant that populates .data (for range checking).
//

//
// Shorthand macros for integer/long long command line options using
// opt_int_callback/opt_llong_callback, with variants that set defval
// and/or data (for range checking).
//
// Note: defval's type is intptr_t. opt_int_callback interprets it directly as
// an int, opt_llong_callback interprets it as a pointer to a long long, as
// long long does not fit into intptr_t on 32-bit architectures.
//

//
// Macros for command line options common to all tools
//
// Note: Some of the options are common to both timerlat and osnoise, but
// have a slightly different meaning. Such options take additional arguments
// that have to be provided by the *_parse_args() function of the corresponding
// tool.
//
// All macros defined here assume the presence of a params variable of
// the corresponding tool type (i.e struct timerlat_params or struct osnoise_params)
// and a cb_data variable of the matching type.
//

//
// Helper functions for parsing numeric option arguments.
//
// value = tmp;
//
// Common callback functions for command line options
//
// value = opt->defval ? *(long long *)opt->defval : 0;
// value = (int)opt->defval;
// Allow -C=<cgroup_name> next to -C[ ]<cgroup_name>
// events = tevent;
//
// Macros for command line options specific to osnoise
//

//
// Callback functions for command line options for osnoise tools
//
// trace_output = NULL;
// trace_output = "osnoise_trace.txt";
// trace_output = (char *)arg;
// Allow -t=<trace_output> next to -t[ ]<trace_output>
//
// Macros for command line options specific to timerlat
//

//
// Callback functions for command line options for timerlat tools
//
// trace_output = NULL;
// trace_output = "timerlat_trace.txt";
// trace_output = (char *)arg;
// Allow -t=<trace_output> next to -t[ ]<trace_output>
// format = default_stack_format;
// format = parse_stack_format((char *)arg);
//
// Macros for command line options specific to histogram-based tools
//

