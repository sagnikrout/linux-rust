//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/simple_ring_buffer.h
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
// Ideally those struct would stay private but the caller needs to know
// the allocation size for simple_ring_buffer_init().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_buffer_page {
    pub link: list_head,
    pub page: *mut buffer_data_page,
    pub entries: u64,
    pub write: u32,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_rb_per_cpu {
    pub tail_page: *mut simple_buffer_page,
    pub reader_page: *mut simple_buffer_page,
    pub head_page: *mut simple_buffer_page,
    pub bpages: *mut simple_buffer_page,
    pub meta: *mut trace_buffer_meta,
    pub nr_pages: u32,
pub const SIMPLE_RB_UNAVAILABLE: c_int = 0;
pub const SIMPLE_RB_READY: c_int = 1;
pub const SIMPLE_RB_WRITING: c_int = 2;
    pub status: u32,
    pub last_overrun: u64,
    pub write_stamp: u64,
    pub cbs: *mut simple_rb_cbs,
}

extern "C" {
    pub fn simple_ring_buffer_unload(cpu_buffer: *mut simple_rb_per_cpu);
}
extern "C" {
    pub fn simple_ring_buffer_commit(cpu_buffer: *mut simple_rb_per_cpu);
}
extern "C" {
    pub fn simple_ring_buffer_enable_tracing(cpu_buffer: *mut simple_rb_per_cpu, enable: bool) -> c_int;
}
extern "C" {
    pub fn simple_ring_buffer_reset(cpu_buffer: *mut simple_rb_per_cpu) -> c_int;
}
extern "C" {
    pub fn simple_ring_buffer_swap_reader_page(cpu_buffer: *mut simple_rb_per_cpu) -> c_int;
}
