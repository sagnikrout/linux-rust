//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/data.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_data_mode {
    PERF_DATA_MODE_WRITE,
    PERF_DATA_MODE_READ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_dir_version {
    PERF_DIR_SINGLE_FILE	= 0,
    PERF_DIR_VERSION	= 1,
}

//
// struct perf_data_file: A wrapper around a file used for perf.data reading or writing. Generally
// part of struct perf_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_data_file {
//
// @path: Path of file. Generally a copy of perf_data.path but for a
// directory it is the file within the directory.
//
    pub path: *mut c_char,
// @fd: File descriptor for read/writes. Valid if use_stdio is false.
    pub fd: c_int,
//
// @fptr: Stdio FILE. Valid if use_stdio is true, currently just
// pipes in perf inject.
//
    pub fptr: *mut FILE,
}

// @size: Size of file when opened.
// @use_stdio: Use buffered stdio operations.
//
// struct perf_data: A wrapper around a file used for perf.data reading or writing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_data {
// @path: Path to open and of the file. NULL implies 'perf.data' will be used.
    pub path: *const c_char,
// @file: Underlying file to be used.
    pub file: perf_data_file,
// @open: Has the file or directory been opened.
    pub open: bool,
// @is_pipe: Underlying file is a pipe.
    pub is_pipe: bool,
// @is_dir: Underlying file is a directory.
    pub is_dir: bool,
// @force: Ignore opening a file creating created by a different user.
    pub force: bool,
// @in_place_update: A file opened for reading but will be written to.
    pub in_place_update: bool,
// @mode: Read or write mode.
    pub mode:8: perf_data_mode,
// @version: perf_dir_version.
    pub version: u64,
// @files: perf data files for the directory.
    pub files: *mut perf_data_file,
// @nr: Number of perf data files for the directory.
    pub nr: c_int,
    pub dir: },
}

extern "C" {
    pub fn perf_data_file__seek(file: *mut perf_data_file, offset: off_t, whence: c_int) -> off_t;
}
extern "C" {
    pub fn perf_data_file__fd(_arg: &data->file) -> return;
}
extern "C" {
    pub fn perf_data__open(data: *mut perf_data) -> c_int;
}
extern "C" {
    pub fn perf_data__close(data: *mut perf_data);
}
extern "C" {
    pub fn perf_data__read(data: *mut perf_data, buf: *mut c_void, size: usize) -> isize;
}
extern "C" {
    pub fn perf_data__seek(data: *mut perf_data, offset: off_t, whence: c_int) -> off_t;
}
//
// If at_exit is set, only rename current perf.data to
// perf.data.<postfix>, continue write on original data.
// Set at_exit when flushing the last output.
//
// Return value is fd of new output.
//
extern "C" {
    pub fn perf_data__create_dir(data: *mut perf_data, nr: c_int) -> c_int;
}
extern "C" {
    pub fn perf_data__open_dir(data: *mut perf_data) -> c_int;
}
extern "C" {
    pub fn perf_data__close_dir(data: *mut perf_data);
}
extern "C" {
    pub fn perf_data__size(data: *mut perf_data) -> c_ulong;
}
extern "C" {
    pub fn perf_data__make_kcore_dir(data: *mut perf_data, buf: *mut c_char, buf_sz: usize) -> c_int;
}
extern "C" {
    pub fn has_kcore_dir(path: *const c_char) -> bool;
}
extern "C" {
    pub fn is_perf_data(path: *const c_char) -> bool;
}
