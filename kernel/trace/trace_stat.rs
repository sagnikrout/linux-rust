//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_stat.h
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
// If you want to provide a stat file (one-shot statistics), fill
// an iterator with stat_start/stat_next and a stat_show callbacks.
// The others callbacks are optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_stat {
// The name of your stat file
    pub name: *const c_char,
// Iteration over statistic entries
    pub trace): *mut *mut *mut void (stat_start)(struct tracer_stat,
    pub idx): *mut *mut *mut *mut void (stat_next)(void prev, int,
// Compare two entries for stats sorting
    pub stat_cmp: cmp_func_t,
// Print a stat entry
    pub p): *mut *mut *mut int (stat_show)(struct seq_file s, void,
// Release an entry
    pub stat): *mut *mut void (stat_release)(void,
// Print the headers of your stat entries
    pub s): *mut *mut int (stat_headers)(struct seq_file,
}

//
// Destroy or create a stat file
//
extern "C" {
    pub fn register_stat_tracer(trace: *mut tracer_stat) -> c_int;
}
extern "C" {
    pub fn unregister_stat_tracer(trace: *mut tracer_stat);
}
