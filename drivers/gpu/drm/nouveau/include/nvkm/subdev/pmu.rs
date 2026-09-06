//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/pmu.h
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
pub struct nvkm_pmu {
    pub func: *const nvkm_pmu_func,
    pub subdev: nvkm_subdev,
    pub falcon: nvkm_falcon,
    pub qmgr: *mut nvkm_falcon_qmgr,
    pub hpq: *mut nvkm_falcon_cmdq,
    pub lpq: *mut nvkm_falcon_cmdq,
    pub msgq: *mut nvkm_falcon_msgq,
    pub initmsg_received: bool,
    pub wpr_ready: completion,
    pub mutex: mutex,
    pub base: u32,
    pub size: u32,
    pub send: },
    pub base: u32,
    pub size: u32,
    pub work: work_struct,
    pub wait: wait_queue_head_t,
    pub process: u32,
    pub message: u32,
    pub data: [u32; 2],
    pub recv: },
}

extern "C" {
    pub fn nvkm_pmu_pgob(: *mut nvkm_pmu, enable: bool);
}
extern "C" {
    pub fn nvkm_pmu_fan_controlled(: *mut nvkm_device) -> bool;
}
extern "C" {
    pub fn gt215_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gf100_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gf119_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gk104_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gk110_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gk208_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gk20a_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gm107_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gm200_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gm20b_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gp102_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gp10b_pmu_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pmu) -> c_int;
}
// interface to MEMX process running on PMU
extern "C" {
    pub fn nvkm_memx_init(: *mut nvkm_pmu, : *mut nvkm_memx) -> c_int;
}
extern "C" {
    pub fn nvkm_memx_fini(: *mut nvkm_memx, exec: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_memx_wr32(: *mut nvkm_memx, addr: u32, data: u32);
}
extern "C" {
    pub fn nvkm_memx_wait(: *mut nvkm_memx, addr: u32, mask: u32, data: u32, nsec: u32);
}
extern "C" {
    pub fn nvkm_memx_nsec(: *mut nvkm_memx, nsec: u32);
}
extern "C" {
    pub fn nvkm_memx_wait_vblank(: *mut nvkm_memx);
}
extern "C" {
    pub fn nvkm_memx_train(: *mut nvkm_memx);
}
extern "C" {
    pub fn nvkm_memx_train_result(: *mut nvkm_pmu, : *mut u32, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_memx_block(: *mut nvkm_memx);
}
extern "C" {
    pub fn nvkm_memx_unblock(: *mut nvkm_memx);
}
