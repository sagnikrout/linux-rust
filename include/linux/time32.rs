//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/time32.h
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


//
// These are all interfaces based on the old time_t definition
// that overflows in 2038 on 32-bit architectures. New code
// should use the replacements based on time64_t and timespec64.
//
// Any interfaces in here that become unused as we migrate
// code to time64_t should get removed.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_itimerspec32 {
    pub it_interval: old_timespec32,
    pub it_value: old_timespec32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_utimbuf32 {
    pub actime: old_time32_t,
    pub modtime: old_time32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_timex32 {
    pub modes: u32,
    pub offset: i32,
    pub freq: i32,
    pub maxerror: i32,
    pub esterror: i32,
    pub status: i32,
    pub constant: i32,
    pub precision: i32,
    pub tolerance: i32,
    pub time: old_timeval32,
    pub tick: i32,
    pub ppsfreq: i32,
    pub jitter: i32,
    pub shift: i32,
    pub stabil: i32,
    pub jitcnt: i32,
    pub calcnt: i32,
    pub errcnt: i32,
    pub stbcnt: i32,
    pub tai: i32,
    pub s32:32: s32:32; s32:32; s32:32;,
    pub s32:32: s32:32; s32:32; s32:32;,
    pub s32:32: s32:32; s32:32;,
}

extern "C" {
    pub fn get_old_timespec32(: *mut timespec64, : *const void __user) -> c_int;
}
extern "C" {
    pub fn put_old_timespec32(: *const timespec64, : *mut void __user) -> c_int;
}
extern "C" {
    pub fn get_old_timex32(: *mut __kernel_timex, : *const old_timex32 __user) -> c_int;
}
extern "C" {
    pub fn put_old_timex32(: *mut old_timex32 __user, : *const __kernel_timex) -> c_int;
}
//
// ns_to_kernel_old_timeval - Convert nanoseconds to timeval
// @nsec:	the nanoseconds value to be converted
//
// Returns the timeval representation of the nsec parameter.
//
extern "C" {
    pub fn ns_to_kernel_old_timeval(nsec: i64) -> __kernel_old_timeval;
}
