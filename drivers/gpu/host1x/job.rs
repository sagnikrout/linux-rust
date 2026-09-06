//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/job.h
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
// Tegra host1x Job
//
// Copyright (c) 2011-2013, NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_job_gather {
    pub words: c_uint,
    pub base: dma_addr_t,
    pub bo: *mut host1x_bo,
    pub offset: c_uint,
    pub handled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_job_wait {
    pub id: u32,
    pub threshold: u32,
    pub next_class: u32,
    pub relative: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_job_cmd {
    pub is_wait: bool,
    pub gather: host1x_job_gather,
    pub wait: host1x_job_wait,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_job_unpin_data {
    pub map: *mut host1x_bo_mapping,
}

//
// Dump contents of job to debug output.
//
extern "C" {
    pub fn host1x_job_dump(dev: *mut device, job: *mut host1x_job);
}
