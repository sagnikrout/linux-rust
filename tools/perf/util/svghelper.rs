//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/svghelper.h
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

extern "C" {
    pub fn open_svg(filename: *const c_char, cpus: c_int, rows: c_int, start: u64, end: u64);
}
extern "C" {
    pub fn svg_ubox(Yslot: c_int, start: u64, end: u64, height: double, type: *const c_char, fd: c_int, err: c_int, merges: c_int);
}
extern "C" {
    pub fn svg_lbox(Yslot: c_int, start: u64, end: u64, height: double, type: *const c_char, fd: c_int, err: c_int, merges: c_int);
}
extern "C" {
    pub fn svg_fbox(Yslot: c_int, start: u64, end: u64, height: double, type: *const c_char, fd: c_int, err: c_int, merges: c_int);
}
extern "C" {
    pub fn svg_box(Yslot: c_int, start: u64, end: u64, type: *const c_char);
}
extern "C" {
    pub fn svg_blocked(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_running(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_waiting(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_cpu_box(cpu: c_int, max_frequency: u64, turbo_frequency: u64);
}
extern "C" {
    pub fn svg_process(cpu: c_int, start: u64, end: u64, pid: c_int, name: *const c_char, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_cstate(cpu: c_int, start: u64, end: u64, type: c_int);
}
extern "C" {
    pub fn svg_pstate(cpu: c_int, start: u64, end: u64, freq: u64);
}
extern "C" {
    pub fn svg_time_grid(min_thickness: double);
}
extern "C" {
    pub fn svg_io_legenda();
}
extern "C" {
    pub fn svg_legenda();
}
extern "C" {
    pub fn svg_wakeline(start: u64, row1: c_int, row2: c_int, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_partial_wakeline(start: u64, row1: c_int, desc1: *mut c_char, row2: c_int, desc2: *mut c_char, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_interrupt(start: u64, row: c_int, backtrace: *const c_char);
}
extern "C" {
    pub fn svg_text(Yslot: c_int, start: u64, text: *const c_char);
}
extern "C" {
    pub fn svg_close();
}
extern "C" {
    pub fn svg_build_topology_map(env: *mut perf_env) -> c_int;
}
