//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/bcm-vk/bcm_vk_sg.h
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
// Copyright 2018-2020 Broadcom.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_dma {
// for userland buffer
    pub pages: *mut page,
    pub nr_pages: c_int,
// common
    pub handle: dma_addr_t,
//
// sglist is of the following LE format
// [U32] num_sg  = number of sg addresses (N)
// [U32] totalsize = totalsize of data being transferred in sglist
// [U32] size[0] = size of data in address0
// [U32] addr_l[0] = lower 32-bits of address0
// [U32] addr_h[0] = higher 32-bits of address0
// ..
// [U32] size[N-1] = size of data in addressN-1
// [U32] addr_l[N-1] = lower 32-bits of addressN-1
// [U32] addr_h[N-1] = higher 32-bits of addressN-1
//
    pub sglist: *mut u32,
pub const SGLIST_NUM_SG: c_int = 0;
pub const SGLIST_TOTALSIZE: c_int = 1;
pub const SGLIST_VKDATA_START: c_int = 2;
    pub /: *mut *mut int sglen; / Length (bytes) of sglist,
    pub direction: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vk_data {
    pub /: *mut *mut u32 size; / data size in bytes,
    pub /: *mut *mut u64 address; / Pointer to data,
    pub __packed: },
//
// Scatter-gather DMA buffer API.
//
// These functions provide a simple way to create a page list and a
// scatter-gather list from userspace address and map the memory
// for DMA operation.
//
    pub num): c_int,
    pub proc_cnt): *mut c_int,
