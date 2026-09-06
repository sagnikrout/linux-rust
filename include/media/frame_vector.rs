//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/frame_vector.h
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
// Container for pinned pfns / pages in frame_vector.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_vector {
    pub /: *mut *mut unsigned int nr_allocated; / Number of frames we have space for,
    pub /: *mut *mut unsigned int nr_frames; / Number of frames stored in ptrs array,
    pub /: *mut *mut bool got_ref; / Did we pin pages by getting page ref?,
    pub /: *mut *mut bool is_pfns; / Does array contain pages or pfns?,
    pub Use: *mut *mut *mut void ptrs[]; / Array of pinned pfns / pages.,
// pfns_vector_pages() or pfns_vector_pfns()
// for access
}

extern "C" {
    pub fn frame_vector_destroy(vec: *mut frame_vector);
}
extern "C" {
    pub fn put_vaddr_frames(vec: *mut frame_vector);
}
extern "C" {
    pub fn frame_vector_to_pages(vec: *mut frame_vector) -> c_int;
}
extern "C" {
    pub fn frame_vector_to_pfns(vec: *mut frame_vector);
}
extern "C" {
    pub fn ERR_PTR(_arg: err) -> return;
}
