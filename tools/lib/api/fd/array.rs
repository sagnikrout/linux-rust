//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/api/fd/array.h
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
// struct fdarray: Array of file descriptors
//
// @priv: Per array entry priv area, users should access just its contents,
// not set it to anything, as it is kept in synch with @entries, being
// realloc'ed, * for instance, in fdarray__{grow,filter}.
//
// I.e. using 'fda->priv[N].idx = * value' where N < fda->nr is ok,
// but doing 'fda->priv = malloc(M)' is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdarray {
    pub nr: c_int,
    pub nr_alloc: c_int,
    pub nr_autogrow: c_int,
    pub entries: *mut pollfd,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv {
    pub idx: c_int,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdarray_flags {
    fdarray_flag__default		= 0x00000000,
    fdarray_flag__nonfilterable	= 0x00000001,
    fdarray_flag__non_perf_event	= 0x00000002,
}

extern "C" {
    pub fn fdarray__init(fda: *mut fdarray, nr_autogrow: c_int);
}
extern "C" {
    pub fn fdarray__exit(fda: *mut fdarray);
}
extern "C" {
    pub fn fdarray__delete(fda: *mut fdarray);
}
extern "C" {
    pub fn fdarray__add(fda: *mut fdarray, fd: c_int, revents: c_short, flags: fdarray_flags) -> c_int;
}
extern "C" {
    pub fn fdarray__dup_entry_from(fda: *mut fdarray, pos: c_int, from: *mut fdarray) -> c_int;
}
extern "C" {
    pub fn fdarray__poll(fda: *mut fdarray, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn fdarray__grow(fda: *mut fdarray, extra: c_int) -> c_int;
}
extern "C" {
    pub fn fdarray__fprintf(fda: *mut fdarray, fp: *mut FILE) -> c_int;
}
