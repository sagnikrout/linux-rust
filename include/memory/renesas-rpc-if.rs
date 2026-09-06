//! Automatically rewritten from C Header to Rust Module
//! Source: include/memory/renesas-rpc-if.h
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
// Renesas RPC-IF core driver
//
// Copyright (C) 2018~2019 Renesas Solutions Corp.
// Copyright (C) 2019 Macronix International Co., Ltd.
// Copyright (C) 2019-2020 Cogent Embedded, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpcif_data_dir {
    RPCIF_NO_DATA,
    RPCIF_DATA_IN,
    RPCIF_DATA_OUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_op {
    pub buswidth: u8,
    pub opcode: u8,
    pub ddr: bool,
    pub ocmd: } cmd,,
    pub nbytes: u8,
    pub buswidth: u8,
    pub ddr: bool,
    pub val: u64,
    pub addr: },
    pub ncycles: u8,
    pub buswidth: u8,
    pub dummy: },
    pub nbytes: u8,
    pub buswidth: u8,
    pub ddr: bool,
    pub val: u32,
    pub option: },
    pub buswidth: u8,
    pub nbytes: c_uint,
    pub dir: rpcif_data_dir,
    pub ddr: bool,
    pub in: *mut c_void,
    pub out: *const c_void,
    pub buf: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpcif_type {
    RPCIF_RCAR_GEN3,
    RPCIF_RCAR_GEN4,
    RPCIF_RZ_G2L,
    XSPI_RZ_G3E,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif {
    pub dev: *mut device,
    pub dirmap: *mut void __iomem,
    pub size: usize,
    pub xspi: bool,
}

extern "C" {
    pub fn rpcif_sw_init(rpc: *mut rpcif, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rpcif_hw_init(dev: *mut device, hyperflash: bool) -> c_int;
}
extern "C" {
    pub fn rpcif_manual_xfer(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rpcif_dirmap_read(dev: *mut device, offs: u64, len: usize, buf: *mut c_void) -> isize;
}
