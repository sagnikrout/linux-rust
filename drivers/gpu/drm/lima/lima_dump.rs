//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_dump.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2020 Qiang Yu <yuq825@gmail.com>

//
// dump file format for all the information to start a lima task
//
// top level format
// | magic code "LIMA" | format version | num tasks | data size |
// | reserved | reserved | reserved | reserved |
// | task 1 ID | task 1 size | num chunks | reserved | task 1 data |
// | task 2 ID | task 2 size | num chunks | reserved | task 2 data |
// ...
//
// task data format
// | chunk 1 ID | chunk 1 size | reserved | reserved | chunk 1 data |
// | chunk 2 ID | chunk 2 size | reserved | reserved | chunk 2 data |
// ...
//
pub const LIMA_DUMP_MAJOR: c_int = 1;
pub const LIMA_DUMP_MINOR: c_int = 0;
pub const LIMA_DUMP_MAGIC: c_uint = 0x414d494c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_dump_head {
    pub magic: __u32,
    pub version_major: __u16,
    pub version_minor: __u16,
    pub num_tasks: __u32,
    pub size: __u32,
    pub reserved: [__u32; 4],
}

pub const LIMA_DUMP_TASK_GP: c_int = 0;
pub const LIMA_DUMP_TASK_PP: c_int = 1;
pub const LIMA_DUMP_TASK_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_dump_task {
    pub id: __u32,
    pub size: __u32,
    pub num_chunks: __u32,
    pub reserved: __u32,
}

pub const LIMA_DUMP_CHUNK_FRAME: c_int = 0;
pub const LIMA_DUMP_CHUNK_BUFFER: c_int = 1;
pub const LIMA_DUMP_CHUNK_PROCESS_NAME: c_int = 2;
pub const LIMA_DUMP_CHUNK_PROCESS_ID: c_int = 3;
pub const LIMA_DUMP_CHUNK_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_dump_chunk {
    pub id: __u32,
    pub size: __u32,
    pub reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_dump_chunk_buffer {
    pub id: __u32,
    pub size: __u32,
    pub va: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_dump_chunk_pid {
    pub id: __u32,
    pub size: __u32,
    pub pid: __u32,
    pub reserved: __u32,
}
