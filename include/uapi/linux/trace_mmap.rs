//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/trace_mmap.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// struct trace_buffer_meta - Ring-buffer Meta-page description
// @meta_page_size:	Size of this meta-page.
// @meta_struct_len:	Size of this structure.
// @subbuf_size:	Size of each sub-buffer.
// @nr_subbufs:		Number of subbfs in the ring-buffer, including the reader.
// @reader:		The reader composite info structure
// @reader.lost_events:	Number of events lost at the time of the reader swap.
// @reader.id:		subbuf ID of the current reader. ID range [0 : @nr_subbufs - 1]
// @reader.read:	Number of bytes read on the reader subbuf.
// @flags:		Placeholder for now, 0 until new features are supported.
// @entries:		Number of entries in the ring-buffer.
// @overrun:		Number of entries lost in the ring-buffer.
// @read:		Number of entries that have been read.
// @pages_lost:		Number of pages overwritten by the writer.
// @pages_touched:	Number of pages written by the writer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_buffer_meta {
    pub meta_page_size: __u32,
    pub meta_struct_len: __u32,
    pub subbuf_size: __u32,
    pub nr_subbufs: __u32,
    pub lost_events: __u64,
    pub id: __u32,
    pub read: __u32,
    pub reader: },
    pub flags: __u64,
    pub entries: __u64,
    pub overrun: __u64,
    pub read: __u64,
    pub pages_lost: __u64,
    pub pages_touched: __u64,
}

