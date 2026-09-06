//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/xilinx-core.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// struct xilinx_fpga_core - interface between the driver and the core manager
// of Xilinx 7 Series FPGA manager
// @dev:       device node
// @write:     write callback of the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_fpga_core {
// public:
    pub dev: *mut device,
    pub count): usize,
// private: handled by xilinx-core
    pub prog_b: *mut gpio_desc,
    pub init_b: *mut gpio_desc,
    pub done: *mut gpio_desc,
}

extern "C" {
    pub fn xilinx_core_probe(core: *mut xilinx_fpga_core) -> c_int;
}
