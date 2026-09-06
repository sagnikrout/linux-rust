//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/events/internal.h
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

// Buffer handling
pub const RING_BUFFER_WRITABLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_buffer {
    pub refcount: refcount_t,
    pub rcu_head: rcu_head,

    pub work: work_struct,
    pub /: *mut *mut int page_order; / allocation order,

    pub /: *mut *mut int nr_pages; / nr of data pages,
    pub /: *mut *mut int overwrite; / can overwrite itself,
    pub /: *mut *mut int paused; / can write into ring buffer,
    pub /: *mut *mut atomic_t poll; / POLL_ for wakeups,
    pub /: *mut *mut local_t head; / write position,
    pub /: *mut *mut unsigned int nest; / nested writers,
    pub /: *mut *mut local_t events; / event limit,
    pub /: *mut *mut local_t wakeup; / wakeup stamp,
    pub /: *mut *mut local_t lost; / nr records lost,
    pub /: *mut *mut long watermark; / wakeup watermark,
    pub aux_watermark: c_long,
// poll crap
    pub event_lock: spinlock_t,
    pub event_list: list_head,
    pub mmap_count: refcount_t,
    pub mmap_locked: c_ulong,
    pub mmap_user: *mut user_struct,
// AUX area
    pub aux_mutex: mutex,
    pub aux_head: c_long,
    pub aux_nest: c_uint,
    pub /: *mut *mut long aux_wakeup; / last aux_watermark boundary crossed by aux_head,
    pub aux_pgoff: c_ulong,
    pub aux_nr_pages: c_int,
    pub aux_overwrite: c_int,
    pub aux_mmap_count: refcount_t,
    pub aux_mmap_locked: c_ulong,
    pub ): *mut *mut void (free_aux)(void,
    pub aux_refcount: refcount_t,
    pub aux_in_sampling: c_int,
    pub aux_in_pause_resume: c_int,
    pub aux_pages: *mut c_void,
    pub aux_priv: *mut c_void,
    pub user_page: *mut perf_event_mmap_page,
    pub data_pages: [*mut c_void; ],
}

extern "C" {
    pub fn rb_free(rb: *mut perf_buffer);
}
extern "C" {
    pub fn perf_event_wakeup(event: *mut perf_event);
}
extern "C" {
    pub fn rb_free_aux(rb: *mut perf_buffer);
}
extern "C" {
    pub fn ring_buffer_put(rb: *mut perf_buffer);
}

//
// Back perf_mmap() with vmalloc memory.
//
// Required for architectures that have d-cache aliasing issues.
//

pub const perf_user_stack_pointer(regs): c_int = 0;

