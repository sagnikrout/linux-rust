//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_device.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2018-2019 Qiang Yu <yuq825@gmail.com>

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lima_gpu_id {
    lima_gpu_mali400 = 0,
    lima_gpu_mali450,
    lima_gpu_num,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lima_ip_id {
    lima_ip_pmu,
    lima_ip_gpmmu,
    lima_ip_ppmmu0,
    lima_ip_ppmmu1,
    lima_ip_ppmmu2,
    lima_ip_ppmmu3,
    lima_ip_ppmmu4,
    lima_ip_ppmmu5,
    lima_ip_ppmmu6,
    lima_ip_ppmmu7,
    lima_ip_gp,
    lima_ip_pp0,
    lima_ip_pp1,
    lima_ip_pp2,
    lima_ip_pp3,
    lima_ip_pp4,
    lima_ip_pp5,
    lima_ip_pp6,
    lima_ip_pp7,
    lima_ip_l2_cache0,
    lima_ip_l2_cache1,
    lima_ip_l2_cache2,
    lima_ip_dlbu,
    lima_ip_bcast,
    lima_ip_pp_bcast,
    lima_ip_ppmmu_bcast,
    lima_ip_num,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_ip {
    pub dev: *mut lima_device,
    pub id: lima_ip_id,
    pub present: bool,
    pub iomem: *mut void __iomem,
    pub irq: c_int,
// gp/pp
    pub async_reset: bool,
// l2 cache
    pub lock: spinlock_t,
// pmu/bcast
    pub mask: u32,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lima_pipe_id {
    lima_pipe_gp,
    lima_pipe_pp,
    lima_pipe_num,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_device {
    pub dev: *mut device,
    pub ddev: *mut drm_device,
    pub id: lima_gpu_id,
    pub gp_version: u32,
    pub pp_version: u32,
    pub num_pp: c_int,
    pub iomem: *mut void __iomem,
    pub clk_bus: *mut clk,
    pub clk_gpu: *mut clk,
    pub reset: *mut reset_control,
    pub regulator: *mut regulator,
    pub ip: [lima_ip; lima_ip_num],
    pub pipe: [lima_sched_pipe; lima_pipe_num],
    pub empty_vm: *mut lima_vm,
    pub va_start: u64,
    pub va_end: u64,
    pub dlbu_cpu: *mut u32,
    pub dlbu_dma: dma_addr_t,
    pub devfreq: lima_devfreq,
// debug info
    pub dump: lima_dump_head,
    pub error_task_list: list_head,
    pub error_task_list_lock: mutex,
}

extern "C" {
    pub fn lima_device_init(ldev: *mut lima_device) -> c_int;
}
extern "C" {
    pub fn lima_device_fini(ldev: *mut lima_device);
}
extern "C" {
    pub fn int(: *mut *mut lima_poll_func_t)(struct lima_ip) -> typedef;
}
extern "C" {
    pub fn lima_device_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn lima_device_resume(dev: *mut device) -> c_int;
}
