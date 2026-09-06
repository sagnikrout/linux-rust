//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/mmap.h
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


pub const __PERF_MMAP_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_cpu_mask {
    pub bits: *mut c_ulong,
    pub nbits: usize,
}

//
// struct mmap - perf's ring buffer mmap details
//
// @refcnt - e.g. code using PERF_EVENT_IOC_SET_OUTPUT to share this
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap {
    pub core: perf_mmap,
    pub auxtrace_mmap: auxtrace_mmap,

    pub data: *mut c_void,
    pub cblocks: *mut aiocb,
    pub aiocb: *mut aiocb,
    pub nr_cblocks: c_int,
    pub aio: },

    pub affinity_mask: mmap_cpu_mask,
    pub data: *mut c_void,
    pub file: *mut perf_data_file,
    pub zstd_data: zstd_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_params {
    pub core: perf_mmap_param,
    pub comp_level: int nr_cblocks, affinity, flush,,
    pub auxtrace_mp: auxtrace_mmap_params,
}

extern "C" {
    pub fn mmap__mmap(map: *mut mmap, mp: *mut mmap_params, fd: c_int, cpu: perf_cpu) -> c_int;
}
extern "C" {
    pub fn mmap__munmap(map: *mut mmap);
}
extern "C" {
    pub fn push(map: *mut mmap, to: *mut c_void, buf: *mut c_void, size): usize) -> c_int;
}
extern "C" {
    pub fn mmap__mmap_len(map: *mut mmap) -> usize;
}
extern "C" {
    pub fn mmap_cpu_mask__scnprintf(mask: *mut mmap_cpu_mask, tag: *const c_char);
}
