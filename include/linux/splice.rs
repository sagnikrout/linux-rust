//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/splice.h
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
// Function declerations and data structures related to the splice
// implementation.
//
// Copyright (C) 2007 Jens Axboe <jens.axboe@oracle.com>
//

//
// Flags passed in from splice/tee/vmsplice
//

// we may still block on the fd we splice
// from/to, of course

//
// Passed to the actors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct splice_desc {
    pub /: *mut *mut size_t total_len; / remaining length,
    pub /: *mut *mut unsigned int len; / current length,
    pub /: *mut *mut unsigned int flags; / splice flags,
//
// actor() private data
//
    pub /: *mut *mut *mut void __user userptr; / memory to write to,
    pub /: *mut *mut *mut file file; / file to read/write,
    pub /: *mut *mut *mut void data; / cookie,
    pub u: },
    pub /: *mut *mut *mut *mut void (splice_eof)(struct splice_desc sd); / Unexpected EOF handler,
    pub /: *mut *mut loff_t pos; / file position,
    pub /: *mut *mut *mut loff_t opos; / sendfile: output position,
    pub /: *mut *mut size_t num_spliced; / number of bytes already spliced,
    pub /: *mut *mut bool need_wakeup; / need to wake up writer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct partial_page {
    pub offset: c_uint,
    pub len: c_uint,
    pub private: c_ulong,
}

//
// Passed to splice_to_pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct splice_pipe_desc {
    pub /: *mut *mut *mut *mut page pages; / page map,
    pub /: *mut *mut *mut partial_page partial; / pages[] may not be contig,
    pub /: *mut *mut int nr_pages; / number of populated pages in map,
    pub /: *mut *mut unsigned int nr_pages_max; / pages[] & partial[] arrays size,
    pub /: *const *const *const pipe_buf_operations ops;/ ops associated with output pipe,
    pub int): *mut *mut *mut void (spd_release)(struct splice_pipe_desc , unsigned,
}

extern "C" {
    pub fn add_to_pipe(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> isize;
}
extern "C" {
    pub fn splice_file_range(_arg: in, _arg: &pos_in, _arg: out, _arg: &pos_out, _arg: len) -> return;
}
//
// for dynamic pipe sizing
//
extern "C" {
    pub fn splice_grow_spd(: *const pipe_inode_info, : *mut splice_pipe_desc) -> c_int;
}
extern "C" {
    pub fn splice_shrink_spd(: *mut splice_pipe_desc);
}
