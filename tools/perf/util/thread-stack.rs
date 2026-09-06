//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/thread-stack.h
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
// thread-stack.h: Synthesize a thread's stack using call / return events
// Copyright (c) 2014, Intel Corporation.
//

//
// Call/Return flags.
//
// CALL_RETURN_NO_CALL: 'return' but no matching 'call'
// CALL_RETURN_NO_RETURN: 'call' but no matching 'return'
// CALL_RETURN_NON_CALL: a branch but not a 'call' to the start of a different
// symbol
//
// struct call_return - paired call/return information.
// @thread: thread in which call/return occurred
// @comm: comm in which call/return occurred
// @cp: call path
// @call_time: timestamp of call (if known)
// @return_time: timestamp of return (if known)
// @branch_count: number of branches seen between call and return
// @insn_count: approx. number of instructions between call and return
// @cyc_count: approx. number of cycles between call and return
// @call_ref: external reference to 'call' sample (e.g. db_id)
// @return_ref:  external reference to 'return' sample (e.g. db_id)
// @db_id: id used for db-export
// @parent_db_id: id of parent call used for db-export
// @flags: Call/Return flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_return {
    pub thread: *mut thread,
    pub comm: *mut comm,
    pub cp: *mut call_path,
    pub call_time: u64,
    pub return_time: u64,
    pub branch_count: u64,
    pub insn_count: u64,
    pub cyc_count: u64,
    pub call_ref: u64,
    pub return_ref: u64,
    pub db_id: u64,
    pub parent_db_id: u64,
    pub flags: u32,
}

//
// struct call_return_processor - provides a call-back to consume call-return
// information.
// @cpr: call path root
// @process: call-back that accepts call/return information
// @data: anonymous data for call-back
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_return_processor {
    pub cpr: *mut call_path_root,
    pub data): *mut *mut *mut *mut int (process)(struct call_return cr, u64 parent_db_id, void,
    pub data: *mut c_void,
}

extern "C" {
    pub fn thread_stack__set_trace_nr(thread: *mut thread, cpu: c_int, trace_nr: u64);
}
extern "C" {
    pub fn thread_stack__flush(thread: *mut thread) -> c_int;
}
extern "C" {
    pub fn thread_stack__free(thread: *mut thread);
}
extern "C" {
    pub fn thread_stack__depth(thread: *mut thread, cpu: c_int) -> usize;
}
extern "C" {
    pub fn call_return_processor__free(crp: *mut call_return_processor);
}
