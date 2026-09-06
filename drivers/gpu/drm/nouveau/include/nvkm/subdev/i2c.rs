//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/i2c.h
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
pub struct nvkm_i2c_bus_probe {
    pub dev: i2c_board_info,
    pub /: *mut *mut u8 udelay; / set to 0 to use the standard delay,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_i2c_bus {
    pub func: *const nvkm_i2c_bus_func,
    pub pad: *mut nvkm_i2c_pad,

    pub id: c_int,
    pub mutex: mutex,
    pub head: list_head,
    pub i2c: i2c_adapter,
    pub enabled: u8,
}

extern "C" {
    pub fn nvkm_i2c_bus_acquire(: *mut nvkm_i2c_bus) -> c_int;
}
extern "C" {
    pub fn nvkm_i2c_bus_release(: *mut nvkm_i2c_bus);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_i2c_aux {
    pub func: *const nvkm_i2c_aux_func,
    pub pad: *mut nvkm_i2c_pad,

    pub id: c_int,
    pub mutex: mutex,
    pub head: list_head,
    pub i2c: i2c_adapter,
    pub enabled: u8,
    pub intr: u32,
}

extern "C" {
    pub fn nvkm_i2c_aux_monitor(: *mut nvkm_i2c_aux, monitor: bool);
}
extern "C" {
    pub fn nvkm_i2c_aux_acquire(: *mut nvkm_i2c_aux) -> c_int;
}
extern "C" {
    pub fn nvkm_i2c_aux_release(: *mut nvkm_i2c_aux);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_i2c {
    pub func: *const nvkm_i2c_func,
    pub subdev: nvkm_subdev,
    pub pad: list_head,
    pub bus: list_head,
    pub aux: list_head,

    pub event: nvkm_event,
}

extern "C" {
    pub fn nv04_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn nv4e_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn nv50_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn g94_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn gf117_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn gf119_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn gk104_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn gk110_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
extern "C" {
    pub fn gm200_i2c_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_i2c) -> c_int;
}
