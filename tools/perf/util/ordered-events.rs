//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/ordered-events.h
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
pub struct ordered_event {
    pub timestamp: u64,
    pub file_offset: u64,
    pub file_path: *const c_char,
    pub event: *mut perf_event,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oe_flush {
    OE_FLUSH__NONE,
    OE_FLUSH__FINAL,
    OE_FLUSH__ROUND,
    OE_FLUSH__HALF,
    OE_FLUSH__TOP,
    OE_FLUSH__TIME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ordered_events_buffer {
    pub list: list_head,
    pub event: [ordered_event; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ordered_events {
    pub last_flush: u64,
    pub next_flush: u64,
    pub max_timestamp: u64,
    pub max_alloc_size: u64,
    pub cur_alloc_size: u64,
    pub events: list_head,
    pub cache: list_head,
    pub to_free: list_head,
    pub buffer: *mut ordered_events_buffer,
    pub last: *mut ordered_event,
    pub deliver: ordered_events__deliver_t,
    pub buffer_idx: c_int,
    pub nr_events: c_uint,
    pub last_flush_type: oe_flush,
    pub nr_unordered_events: u32,
    pub copy_on_queue: bool,
    pub data: *mut c_void,
}

extern "C" {
    pub fn ordered_events__delete(oe: *mut ordered_events, event: *mut ordered_event);
}
extern "C" {
    pub fn ordered_events__flush(oe: *mut ordered_events, how: oe_flush) -> c_int;
}
extern "C" {
    pub fn ordered_events__flush_time(oe: *mut ordered_events, timestamp: u64) -> c_int;
}
extern "C" {
    pub fn ordered_events__free(oe: *mut ordered_events);
}
extern "C" {
    pub fn ordered_events__reinit(oe: *mut ordered_events);
}
extern "C" {
    pub fn ordered_events__first_time(oe: *mut ordered_events) -> u64;
}
