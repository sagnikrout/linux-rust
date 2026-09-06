//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/spu_info.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// SPU info structures
//
// (C) Copyright 2006 IBM Corp.
//
// Author: Dwayne Grant McConnell <decimal@us.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_cq_sr {
    pub mfc_cq_data0_RW: __u64,
    pub mfc_cq_data1_RW: __u64,
    pub mfc_cq_data2_RW: __u64,
    pub mfc_cq_data3_RW: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_dma_info {
    pub dma_info_type: __u64,
    pub dma_info_mask: __u64,
    pub dma_info_status: __u64,
    pub dma_info_stall_and_notify: __u64,
    pub dma_info_atomic_command_status: __u64,
    pub dma_info_command_data: [mfc_cq_sr; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_proxydma_info {
    pub proxydma_info_type: __u64,
    pub proxydma_info_mask: __u64,
    pub proxydma_info_status: __u64,
    pub proxydma_info_command_data: [mfc_cq_sr; 8],
}
