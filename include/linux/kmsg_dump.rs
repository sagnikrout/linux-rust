//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kmsg_dump.h
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
// linux/include/kmsg_dump.h
//
// Copyright (C) 2009 Net Insight AB
//
// Author: Simon Kagstrom <simon.kagstrom@netinsight.net>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Keep this list arranged in rough order of priority. Anything listed after
// KMSG_DUMP_OOPS will not be logged by default unless printk.always_kmsg_dump
// is passed to the kernel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kmsg_dump_reason {
    KMSG_DUMP_UNDEF,
    KMSG_DUMP_PANIC,
    KMSG_DUMP_OOPS,
    KMSG_DUMP_EMERG,
    KMSG_DUMP_SHUTDOWN,
    KMSG_DUMP_MAX
}

//
// struct kmsg_dump_iter - iterator for retrieving kernel messages
// @cur_seq:	Points to the oldest message to dump
// @next_seq:	Points after the newest message to dump
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmsg_dump_iter {
    pub cur_seq: u64,
    pub next_seq: u64,
}

//
// struct kmsg_dump_detail - kernel crash detail
// @reason: reason for the crash, see kmsg_dump_reason.
// @description: optional short string, to provide additional information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmsg_dump_detail {
    pub reason: kmsg_dump_reason,
    pub description: *const c_char,
}

//
// struct kmsg_dumper - kernel crash message dumper structure
// @list:	Entry in the dumper list (private)
// @dump:	Call into dumping code which will retrieve the data with
// through the record iterator
// @max_reason:	filter for highest reason number that should be dumped
// @registered:	Flag that specifies if this is already registered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmsg_dumper {
    pub list: list_head,
    pub detail): *mut *mut *mut void (dump)(struct kmsg_dumper dumper, struct kmsg_dump_detail,
    pub max_reason: kmsg_dump_reason,
    pub registered: bool,
}

extern "C" {
    pub fn kmsg_dump_desc(reason: kmsg_dump_reason, desc: *const c_char);
}
extern "C" {
    pub fn kmsg_dump_rewind(iter: *mut kmsg_dump_iter);
}
extern "C" {
    pub fn kmsg_dump_register(dumper: *mut kmsg_dumper) -> c_int;
}
extern "C" {
    pub fn kmsg_dump_unregister(dumper: *mut kmsg_dumper) -> c_int;
}

