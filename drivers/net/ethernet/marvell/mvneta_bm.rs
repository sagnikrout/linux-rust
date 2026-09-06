//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/mvneta_bm.h
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


//
// Driver for Marvell NETA network controller Buffer Manager.
//
// Copyright (C) 2015 Marvell
//
// Marcin Wojtas <mw@semihalf.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
// BM Configuration Register
pub const MVNETA_BM_CONFIG_REG: c_uint = 0x0;
pub const MVNETA_BM_STATUS_MASK: c_uint = 0x30;

pub const MVNETA_BM_MAX_IN_BURST_SIZE_MASK: c_uint = 0x60000;

// BM Activation Register
pub const MVNETA_BM_COMMAND_REG: c_uint = 0x4;

// BM Xbar interface Register
pub const MVNETA_BM_XBAR_01_REG: c_uint = 0x8;
pub const MVNETA_BM_XBAR_23_REG: c_uint = 0xc;

// Address of External Buffer Pointers Pool Register

// External Buffer Pointers Pool RD pointer Register

pub const MVNETA_BM_POOL_SET_READ_PTR_MASK: c_uint = 0xfffc;
pub const MVNETA_BM_POOL_GET_READ_PTR_OFFS: c_int = 16;
pub const MVNETA_BM_POOL_GET_READ_PTR_MASK: c_uint = 0xfffc0000;
// External Buffer Pointers Pool WR pointer

pub const MVNETA_BM_POOL_SET_WRITE_PTR_OFFS: c_int = 0;
pub const MVNETA_BM_POOL_SET_WRITE_PTR_MASK: c_uint = 0xfffc;
pub const MVNETA_BM_POOL_GET_WRITE_PTR_OFFS: c_int = 16;
pub const MVNETA_BM_POOL_GET_WRITE_PTR_MASK: c_uint = 0xfffc0000;
// External Buffer Pointers Pool Size Register

pub const MVNETA_BM_POOL_SIZE_MASK: c_uint = 0x3fff;
// BM Interrupt Cause Register

// BM interrupt Mask Register

// Other definitions
pub const MVNETA_BM_SHORT_PKT_SIZE: c_int = 256;
pub const MVNETA_BM_POOLS_NUM: c_int = 4;
pub const MVNETA_BM_POOL_CAP_MIN: c_int = 128;
pub const MVNETA_BM_POOL_CAP_DEF: c_int = 2048;

pub const MVNETA_BM_POOL_CAP_ALIGN: c_int = 32;
pub const MVNETA_BM_POOL_PTR_ALIGN: c_int = 32;
pub const MVNETA_BM_POOL_ACCESS_OFFS: c_int = 8;
pub const MVNETA_BM_BPPI_SIZE: c_uint = 0x100000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvneta_bm_type {
    MVNETA_BM_FREE,
    MVNETA_BM_LONG,
    MVNETA_BM_SHORT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvneta_bm {
    pub reg_base: *mut void __iomem,
    pub clk: *mut clk,
    pub pdev: *mut platform_device,
    pub bppi_pool: *mut gen_pool,
// BPPI virtual base address
    pub bppi_virt_addr: *mut void __iomem,
// BPPI physical base address
    pub bppi_phys_addr: dma_addr_t,
// BM pools
    pub bm_pools: *mut mvneta_bm_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvneta_bm_pool {
    pub hwbm_pool: hwbm_pool,
// Pool number in the range 0-3
    pub id: u8,
    pub type: mvneta_bm_type,
// Packet size
    pub pkt_size: c_int,
// Size of the buffer access through DMA
    pub buf_size: u32,
// BPPE virtual base address
    pub virt_addr: *mut u32,
// BPPE physical base address
    pub phys_addr: dma_addr_t,
// Ports using BM pool
    pub port_map: u8,
    pub priv: *mut mvneta_bm,
}

// Declarations and definitions

extern "C" {
    pub fn mvneta_bm_put(priv: *mut mvneta_bm);
}
extern "C" {
    pub fn mvneta_bm_construct(hwbm_pool: *mut hwbm_pool, buf: *mut c_void) -> c_int;
}

