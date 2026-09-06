//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/db-export.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// db-export.h: Support for exporting data suitable for import to a database
// Copyright (c) 2014, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct export_sample {
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub al: *mut addr_location,
    pub db_id: u64,
    pub comm_db_id: u64,
    pub dso_db_id: u64,
    pub sym_db_id: u64,
    pub /: *mut *mut u64 offset; / ip offset from symbol start,
    pub addr_dso_db_id: u64,
    pub addr_sym_db_id: u64,
    pub /: *mut *mut u64 addr_offset; / addr offset from symbol start,
    pub call_path_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_export {
    pub evsel): *mut *mut *mut int (export_evsel)(struct db_export dbe, struct evsel,
    pub machine): *mut *mut *mut int (export_machine)(struct db_export dbe, struct machine,
    pub machine): *mut u64 main_thread_db_id, struct machine,
    pub thread): *mut thread,
    pub thread): *mut *mut comm comm, thread,
    pub machine): *mut machine,
    pub dso): *mut dso,
    pub name): *const c_char,
    pub es): *mut *mut *mut int (export_sample)(struct db_export dbe, struct export_sample,
    pub cp): *mut *mut *mut int (export_call_path)(struct db_export dbe, struct call_path,
    pub cr): *mut call_return,
    pub flags): u64 th_in_id, u64 comm_in_id, int,
    pub crp: *mut call_return_processor,
    pub cpr: *mut call_path_root,
    pub evsel_last_db_id: u64,
    pub machine_last_db_id: u64,
    pub thread_last_db_id: u64,
    pub comm_last_db_id: u64,
    pub comm_thread_last_db_id: u64,
    pub dso_last_db_id: u64,
    pub symbol_last_db_id: u64,
    pub sample_last_db_id: u64,
    pub call_path_last_db_id: u64,
    pub call_return_last_db_id: u64,
    pub context_switch_last_db_id: u64,
}

extern "C" {
    pub fn db_export__init(dbe: *mut db_export) -> c_int;
}
extern "C" {
    pub fn db_export__exit(dbe: *mut db_export);
}
extern "C" {
    pub fn db_export__evsel(dbe: *mut db_export, evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn db_export__machine(dbe: *mut db_export, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn db_export__branch_types(dbe: *mut db_export) -> c_int;
}
extern "C" {
    pub fn db_export__call_path(dbe: *mut db_export, cp: *mut call_path) -> c_int;
}
