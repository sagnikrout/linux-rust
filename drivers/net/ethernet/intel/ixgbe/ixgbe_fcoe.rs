//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_fcoe.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

// shift bits within STAT fo FCSTAT
pub const IXGBE_RXDADV_FCSTAT_SHIFT: c_int = 4;
// ddp user buffer

pub const IXGBE_FCPTR_ALIGN: c_int = 16;

pub const IXGBE_FCBUFF_4KB: c_uint = 0x0;
pub const IXGBE_FCBUFF_8KB: c_uint = 0x1;
pub const IXGBE_FCBUFF_16KB: c_uint = 0x2;
pub const IXGBE_FCBUFF_64KB: c_uint = 0x3;

// Default traffic class to use for FCoE
pub const IXGBE_FCOE_DEFTC: c_int = 3;
// fcerr
pub const IXGBE_FCERR_BADCRC: c_uint = 0x00100000;
// FCoE DDP for target mode
pub const __IXGBE_FCOE_TARGET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fcoe_ddp {
    pub len: c_int,
    pub err: u32,
    pub sgc: c_uint,
    pub sgl: *mut scatterlist,
    pub udp: dma_addr_t,
    pub udl: *mut u64,
    pub pool: *mut dma_pool,
}

// per cpu variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fcoe_ddp_pool {
    pub pool: *mut dma_pool,
    pub noddp: u64,
    pub noddp_ext_buff: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fcoe {
    pub ddp_pool: *mut ixgbe_fcoe_ddp_pool __percpu,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub ddp: [ixgbe_fcoe_ddp; IXGBE_FCOE_DDP_MAX_X550],
    pub extra_ddp_buffer: *mut c_void,
    pub extra_ddp_buffer_dma: dma_addr_t,
    pub mode: c_ulong,
    pub up: u8,
}
