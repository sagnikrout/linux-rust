//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/thread.h
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
pub struct lbr_stitch {
    pub lists: list_head,
    pub free_lists: list_head,
    pub prev_sample: perf_sample,
    pub prev_lbr_cursor: *mut callchain_cursor_node,
    pub prev_lbr_cursor_size: c_uint,
}

// @maps: mmaps associated with this thread.
// @tid: thread ID number unique to a machine.
// @ppid: parent process of the process this thread belongs to.
//
// @exited: Has the thread had an exit event. Such threads are usually
// removed from the machine's threads but some events/tools require
// access to dead threads.
//
// @e_flags: The ELF EF_* associated with the thread. Valid if e_machine != EM_NONE.
//
// @e_machine: The ELF EM_* associated with the thread. EM_NONE if not
// computed.
//
// @e_is_big_endian: True if the ELF architecture of the thread is big endian.
// Valid if e_machine != EM_NONE.
//
// LBR call stack stitch
extern "C" {
    pub fn thread__init_maps(thread: *mut thread, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn thread__delete(thread: *mut thread);
}
extern "C" {
    pub fn thread__set_priv_destructor(priv): *mut *mut void (destructor)(void);
}
extern "C" {
    pub fn thread__put(thread: *mut thread);
}
// thread = NULL;

extern "C" {
    pub fn __thread__set_comm(_arg: thread, _arg: comm, _arg: timestamp, _arg: false) -> return;
}
extern "C" {
    pub fn thread__set_comm_from_proc(thread: *mut thread) -> c_int;
}
extern "C" {
    pub fn thread__comm_len(thread: *mut thread) -> c_int;
}
extern "C" {
    pub fn thread__insert_map(thread: *mut thread, map: *mut map) -> c_int;
}
extern "C" {
    pub fn thread__fork(thread: *mut thread, parent: *mut thread, timestamp: u64, do_maps_clone: bool) -> c_int;
}
extern "C" {
    pub fn thread__fprintf(thread: *mut thread, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn list_empty(_arg: &RC_CHK_ACCESS(thread)->namespaces_list) -> return;
}
extern "C" {
    pub fn thread__e_machine_endian(_arg: thread, _arg: machine, _arg: e_flags, _arg: NULL) -> return;
}
extern "C" {
    pub fn thread__free_stitch_list(thread: *mut thread);
}
