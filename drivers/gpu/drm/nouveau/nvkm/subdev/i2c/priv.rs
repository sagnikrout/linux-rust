//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/priv.h
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
pub struct nvkm_i2c_func {
    pub ): *mut *mut *mut int (pad_x_new)(struct nvkm_i2c , int id, struct nvkm_i2c_pad,
    pub ): *mut *mut *mut int (pad_s_new)(struct nvkm_i2c , int id, struct nvkm_i2c_pad,
// number of native dp aux channels present
    pub aux: c_int,
// read and ack pending interrupts, returning only data
// for ports that have not been masked off, while still
// performing the ack for anything that was pending.
//
    pub ): *mut *mut *mut *mut *mut *mut void (aux_stat)(struct nvkm_i2c , u32 , u32 , u32 , u32,
// mask on/off interrupt types for a given set of auxch
//
    pub u32): *mut *mut *mut void (aux_mask)(struct nvkm_i2c , u32, u32,,
// enable/disable HW-initiated DPCD reads
//
    pub enable): *mut *mut *mut void (aux_autodpcd)(struct nvkm_i2c , int aux, bool,
}

extern "C" {
    pub fn g94_aux_stat(: *mut nvkm_i2c, : *mut u32, : *mut u32, : *mut u32, : *mut u32);
}
extern "C" {
    pub fn g94_aux_mask(: *mut nvkm_i2c, _arg: u32, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn gk104_aux_stat(: *mut nvkm_i2c, : *mut u32, : *mut u32, : *mut u32, : *mut u32);
}
extern "C" {
    pub fn gk104_aux_mask(: *mut nvkm_i2c, _arg: u32, _arg: u32, _arg: u32);
}
