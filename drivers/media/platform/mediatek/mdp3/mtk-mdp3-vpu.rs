//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-vpu.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_ipi_result {
    MDP_IPI_SUCCESS	= 0,
    MDP_IPI_ENOMEM	= 12,
    MDP_IPI_EBUSY	= 16,
    MDP_IPI_EINVAL	= 22,
    MDP_IPI_EMINST	= 24,
    MDP_IPI_ERANGE	= 34,
    MDP_IPI_NR_ERRNO,

    MDP_IPI_EOTHER	= MDP_IPI_NR_ERRNO,
    MDP_IPI_PATH_CANT_MERGE,
    MDP_IPI_OP_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ipi_init_msg {
    pub status: u32,
    pub drv_data: u64,
    pub /: *mut *mut u32 work_addr; / [in] working buffer address,
    pub /: *mut *mut u32 work_size; / [in] working buffer size,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ipi_deinit_msg {
    pub status: u32,
    pub drv_data: u64,
    pub work_addr: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_vpu_dev {
// synchronization protect for accessing vpu working buffer info
    pub lock: *mut mutex,
    pub scp: *mut mtk_scp,
    pub ipi_acked: completion,
    pub param: *mut c_void,
    pub param_addr: dma_addr_t,
    pub param_size: usize,
    pub work: *mut c_void,
    pub work_addr: dma_addr_t,
    pub work_size: usize,
    pub config: *mut c_void,
    pub config_addr: dma_addr_t,
    pub config_size: usize,
    pub status: u32,
}

extern "C" {
    pub fn mdp_vpu_shared_mem_free(vpu: *mut mdp_vpu_dev);
}
extern "C" {
    pub fn mdp_vpu_dev_deinit(vpu: *mut mdp_vpu_dev) -> c_int;
}
extern "C" {
    pub fn mdp_vpu_process(vpu: *mut mdp_vpu_dev, param: *mut img_ipi_frameparam) -> c_int;
}
