//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-udma.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// User DMA functions
extern "C" {
    pub fn ivtv_udma_get_page_info(dma_page: *mut ivtv_dma_page_info, first: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn ivtv_udma_fill_sg_list(dma: *mut ivtv_user_dma, dma_page: *mut ivtv_dma_page_info, map_offset: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_udma_fill_sg_array(dma: *mut ivtv_user_dma, buffer_offset: u32, buffer_offset_2: u32, split: u32);
}
extern "C" {
    pub fn ivtv_udma_unmap(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_udma_free(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_udma_alloc(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_udma_prepare(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_udma_start(itv: *mut ivtv);
}
