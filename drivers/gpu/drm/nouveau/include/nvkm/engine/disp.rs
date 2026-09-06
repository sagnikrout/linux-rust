//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/engine/disp.h
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
pub struct nvkm_disp {
    pub func: *const nvkm_disp_func,
    pub engine: nvkm_engine,
    pub client: nvkm_gsp_client,
    pub device: nvkm_gsp_device,
    pub objcom: nvkm_gsp_object,
    pub object: nvkm_gsp_object,

    pub event: nvkm_event,
    pub hpd: nvkm_gsp_event,
    pub irq: nvkm_gsp_event,
    pub assigned_sors: u32,
    pub rm: },
    pub heads: list_head,
    pub iors: list_head,
    pub outps: list_head,
    pub conns: list_head,
    pub hpd: nvkm_event,

    pub vblank: nvkm_event,
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub pending: u32,
    pub mutex: mutex,
    pub super: },

    pub uevent: nvkm_event,
    pub mask: c_ulong,
    pub nr: c_int,
    pub sor: } wndw, head, dac,,
    pub mask: c_ulong,
    pub nr: c_int,
    pub type: [u8; 3],
    pub pior: },
    pub inst: *mut nvkm_gpuobj,
    pub ramht: *mut nvkm_ramht,
    pub chan: [*mut nvkm_disp_chan; 81],
    pub lock: spinlock_t,
    pub object: nvkm_object,
    pub client: },
}

extern "C" {
    pub fn nv04_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn nv50_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn g84_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gt200_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn g94_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn mcp77_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gt215_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn mcp89_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gf119_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gk104_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gk110_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gm107_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gm200_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gp100_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gp102_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gv100_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn tu102_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn ga102_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gb202_disp_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_disp) -> c_int;
}
