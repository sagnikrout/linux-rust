//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/logic_pio.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2017 HiSilicon Limited, All Rights Reserved.
// Author: Gabriele Paoloni <gabriele.paoloni@huawei.com>
// Author: Zhichang Yuan <yuanzhichang@hisilicon.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_pio_hwaddr {
    pub list: list_head,
    pub fwnode: *const fwnode_handle,
    pub hw_start: resource_size_t,
    pub io_start: resource_size_t,
    pub /: *mut *mut resource_size_t size; / range size populated,
    pub flags: c_ulong,
    pub hostdata: *mut c_void,
    pub ops: *const logic_pio_host_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_pio_host_ops {
    pub dwidth): *mut *mut *mut u32 (in)(void hostdata, unsigned long addr, size_t,
    pub dwidth): usize,
    pub count): size_t dwidth, unsigned int,
    pub count): size_t dwidth, unsigned int,
}

extern "C" {
    pub fn logic_inb(addr: c_ulong) -> u8;
}
extern "C" {
    pub fn logic_inw(addr: c_ulong) -> u16;
}
extern "C" {
    pub fn logic_inl(addr: c_ulong) -> u32;
}
extern "C" {
    pub fn logic_outb(value: u8, addr: c_ulong);
}
extern "C" {
    pub fn logic_outw(value: u16, addr: c_ulong);
}
extern "C" {
    pub fn logic_outl(value: u32, addr: c_ulong);
}
extern "C" {
    pub fn logic_insb(addr: c_ulong, buffer: *mut c_void, count: c_uint);
}
extern "C" {
    pub fn logic_insl(addr: c_ulong, buffer: *mut c_void, count: c_uint);
}
extern "C" {
    pub fn logic_insw(addr: c_ulong, buffer: *mut c_void, count: c_uint);
}
extern "C" {
    pub fn logic_outsb(addr: c_ulong, buffer: *const c_void, count: c_uint);
}
extern "C" {
    pub fn logic_outsw(addr: c_ulong, buffer: *const c_void, count: c_uint);
}
extern "C" {
    pub fn logic_outsl(addr: c_ulong, buffer: *const c_void, count: c_uint);
}

//
// We reserve 0x4000 bytes for Indirect IO as so far this library is only
// used by the HiSilicon LPC Host. If needed, we can reserve a wider IO
// area by redefining the macro below.
//
pub const PIO_INDIRECT_SIZE: c_uint = 0x4000;

pub const PIO_INDIRECT_SIZE: c_int = 0;

extern "C" {
    pub fn logic_pio_register_range(newrange: *mut logic_pio_hwaddr) -> c_int;
}
extern "C" {
    pub fn logic_pio_unregister_range(range: *mut logic_pio_hwaddr);
}
extern "C" {
    pub fn logic_pio_to_hwaddr(pio: c_ulong) -> resource_size_t;
}
extern "C" {
    pub fn logic_pio_trans_cpuaddr(hw_addr: resource_size_t) -> c_ulong;
}
