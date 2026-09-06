//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/map_benchmark.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2022-2025 HiSilicon Limited.
//

pub const DMA_MAP_MAX_THREADS: c_int = 1024;
pub const DMA_MAP_MAX_SECONDS: c_int = 300;

pub const DMA_MAP_BIDIRECTIONAL: c_int = 0;
pub const DMA_MAP_TO_DEVICE: c_int = 1;
pub const DMA_MAP_FROM_DEVICE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_benchmark {
    pub /: *mut *mut __u64 avg_map_100ns; / average map latency in 100ns,
    pub /: *mut *mut __u64 map_stddev; / standard deviation of map latency,
    pub /: *mut *mut __u64 avg_unmap_100ns; / as above,
    pub unmap_stddev: __u64,
    pub /: *mut *mut __u32 threads; / how many threads will do map/unmap in parallel,
    pub /: *mut *mut __u32 seconds; / how long the test will last,
    pub /: *mut *mut __s32 node; / which numa node this benchmark will run on,
    pub /: *mut *mut __u32 dma_bits; / DMA addressing capability,
    pub /: *mut *mut __u32 dma_dir; / DMA data direction,
    pub /: *mut *mut __u32 dma_trans_ns; / time for DMA transmission in ns,
    pub operation: *mut *mut __u32 granule; / - SINGLE_MODE: number of pages mapped/unmapped per,
// - SG_MODE: number of scatterlist entries (each maps one page)
//
    pub /: *mut *mut __u8 map_mode; / the mode of dma map,
    pub /: *mut *mut __u8 expansion[75]; / For future use,
}
