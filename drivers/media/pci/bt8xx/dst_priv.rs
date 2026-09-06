//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/dst_priv.h
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
// dst-bt878.h: part of the DST driver for the TwinHan DST Frontend
//
// Copyright (C) 2003 Jamie Honan
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_gpio_enable {
    pub mask: u32,
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_gpio_output {
    pub mask: u32,
    pub highvals: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_gpio_read {
    pub value: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dst_gpio_packet {
    pub enb: dst_gpio_enable,
    pub outp: dst_gpio_output,
    pub rd: dst_gpio_read,
    pub psize: c_int,
}

pub const DST_IG_ENABLE: c_int = 0;
pub const DST_IG_WRITE: c_int = 1;
pub const DST_IG_READ: c_int = 2;
pub const DST_IG_TS: c_int = 3;
extern "C" {
    pub fn bt878_device_control(bt: *mut bt878, cmd: c_uint, mp: *mut dst_gpio_packet) -> c_int;
}
