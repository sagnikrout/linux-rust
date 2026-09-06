//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/buildid.h
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

pub const BUILD_ID_SIZE_MAX: c_int = 20;
extern "C" {
    pub fn build_id_parse(vma: *mut vm_area_struct, build_id: *mut c_uchar, size: *mut __u32) -> c_int;
}
extern "C" {
    pub fn build_id_parse_file(file: *mut file, build_id: *mut c_uchar, size: *mut __u32) -> c_int;
}
extern "C" {
    pub fn build_id_parse_nofault(vma: *mut vm_area_struct, build_id: *mut c_uchar, size: *mut __u32) -> c_int;
}
extern "C" {
    pub fn build_id_parse_buf(buf: *const c_void, build_id: *mut c_uchar, buf_size: u32) -> c_int;
}

extern "C" {
    pub fn init_vmlinux_build_id();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freader {
    pub buf: *mut c_void,
    pub buf_sz: u32,
    pub err: c_int,
    pub file: *mut file,
    pub folio: *mut folio,
    pub addr: *mut c_void,
    pub folio_off: loff_t,
    pub may_fault: bool,
}

extern "C" {
    pub fn freader_init_from_mem(r: *mut freader, data: *const c_char, data_sz: u64);
}
extern "C" {
    pub fn freader_cleanup(r: *mut freader);
}
