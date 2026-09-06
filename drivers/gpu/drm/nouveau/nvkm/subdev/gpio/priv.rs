//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gpio/priv.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gpio_func {
    pub lines: c_int,
// read and ack pending interrupts, returning only data
// for lines that have not been masked off, while still
// performing the ack for anything that was pending.
//
    pub ): *mut *mut *mut *mut void (intr_stat)(struct nvkm_gpio , u32 , u32,
// mask on/off interrupts for hi/lo transitions on a
// given set of gpio lines
//
    pub u32): *mut *mut *mut void (intr_mask)(struct nvkm_gpio , u32, u32,,
// configure gpio direction and output value
    pub out): *mut *mut *mut int (drive)(struct nvkm_gpio , int line, int dir, int,
// sense current state of given gpio line
    pub line): *mut *mut *mut int (sense)(struct nvkm_gpio , int,
// XXX
    pub u8): *mut *mut *mut void (reset)(struct nvkm_gpio ,,
}

extern "C" {
    pub fn nv50_gpio_reset(: *mut nvkm_gpio, _arg: u8);
}
extern "C" {
    pub fn nv50_gpio_drive(: *mut nvkm_gpio, _arg: c_int, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_gpio_sense(: *mut nvkm_gpio, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn g94_gpio_intr_stat(: *mut nvkm_gpio, : *mut u32, : *mut u32);
}
extern "C" {
    pub fn g94_gpio_intr_mask(: *mut nvkm_gpio, _arg: u32, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn gf119_gpio_reset(: *mut nvkm_gpio, _arg: u8);
}
extern "C" {
    pub fn gf119_gpio_drive(: *mut nvkm_gpio, _arg: c_int, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gf119_gpio_sense(: *mut nvkm_gpio, _arg: c_int) -> c_int;
}
