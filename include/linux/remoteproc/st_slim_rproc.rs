//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/remoteproc/st_slim_rproc.h
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
// SLIM core rproc driver header
//
// Copyright (C) 2016 STMicroelectronics
//
// Author: Peter Griffin <peter.griffin@linaro.org>
//
pub const ST_SLIM_MEM_MAX: c_int = 2;
pub const ST_SLIM_MAX_CLK: c_int = 4;
//
// struct st_slim_mem - slim internal memory structure
// @cpu_addr: MPU virtual address of the memory region
// @bus_addr: Bus address used to access the memory region
// @size: Size of the memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_slim_mem {
    pub cpu_addr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub size: usize,
}

//
// struct st_slim_rproc - SLIM slim core
// @rproc: rproc handle
// @mem: slim memory information
// @slimcore: slim slimcore regs
// @peri: slim peripheral regs
// @clks: slim clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_slim_rproc {
    pub rproc: *mut rproc,
    pub mem: [st_slim_mem; ST_SLIM_MEM_MAX],
    pub slimcore: *mut void __iomem,
    pub peri: *mut void __iomem,
// st_slim_rproc private
    pub clks: [*mut clk; ST_SLIM_MAX_CLK],
}

extern "C" {
    pub fn st_slim_rproc_put(slim_rproc: *mut st_slim_rproc);
}
