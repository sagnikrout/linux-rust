//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/engine/fifo.h
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

pub const NVKM_FIFO_ENGN_NR: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_chan {
    pub func: *const nvkm_chan_func,
    pub name: [c_char; 64],
    pub cgrp: *mut nvkm_cgrp,
    pub runq: c_int,
    pub inst: *mut nvkm_gpuobj,
    pub vmm: *mut nvkm_vmm,
    pub push: *mut nvkm_gpuobj,
    pub id: c_int,
    pub mem: *mut nvkm_memory,
    pub base: u32,
    pub userd: },
    pub ramfc_offset: u32,
    pub ramfc: *mut nvkm_gpuobj,
    pub cache: *mut nvkm_gpuobj,
    pub eng: *mut nvkm_gpuobj,
    pub pgd: *mut nvkm_gpuobj,
    pub ramht: *mut nvkm_ramht,
    pub lock: spinlock_t,
    pub blocked: core::sync::atomic::AtomicI32,
    pub errored: core::sync::atomic::AtomicI32,
    pub object: nvkm_gsp_object,
    pub addr: dma_addr_t,
    pub ptr: *mut c_void,
    pub mthdbuf: },
    pub grctx: *mut nvkm_vctx,
    pub rm: },
    pub cctxs: list_head,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_chan_put(: *mut nvkm_chan, irqflags: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fifo {
    pub func: *const nvkm_fifo_func,
    pub engine: nvkm_engine,
    pub chid: *mut nvkm_chid,
    pub cgid: *mut nvkm_chid,
    pub runqs: list_head,
    pub runls: list_head,

    pub event: nvkm_event,
    pub intr: nvkm_inth,
    pub nonstall: },
    pub chan_msec: u32,
    pub timeout: },
    pub mem: *mut nvkm_memory,
    pub bar1: *mut nvkm_vma,
    pub userd: },
    pub mthdbuf_size: u32,
    pub rm: },
    pub lock: spinlock_t,
    pub mutex: mutex,
}

extern "C" {
    pub fn nvkm_fifo_fault(: *mut nvkm_fifo, : *mut nvkm_fault_data);
}
extern "C" {
    pub fn nvkm_fifo_pause(: *mut nvkm_fifo, : *mut c_ulong);
}
extern "C" {
    pub fn nvkm_fifo_start(: *mut nvkm_fifo, : *mut c_ulong);
}
extern "C" {
    pub fn nvkm_fifo_ctxsw_in_progress(: *mut nvkm_engine) -> bool;
}
extern "C" {
    pub fn nv04_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv10_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv17_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv40_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv50_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn g84_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn g98_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gf100_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk104_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk110_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk208_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk20a_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gm107_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gm200_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gp100_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gv100_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn tu102_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn ga100_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn ga102_fifo_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fifo) -> c_int;
}
