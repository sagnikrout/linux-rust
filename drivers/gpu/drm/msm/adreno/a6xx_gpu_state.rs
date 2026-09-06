//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/a6xx_gpu_state.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const A6XX_NUM_CONTEXTS: c_int = 2;
pub const A6XX_NUM_SHADER_BANKS: c_int = 3;
pub const CLUSTER_FE: c_int = 0;
pub const CLUSTER_SP_VS: c_int = 1;
pub const CLUSTER_PC_VS: c_int = 2;
pub const CLUSTER_GRAS: c_int = 3;
pub const CLUSTER_SP_PS: c_int = 4;
pub const CLUSTER_PS: c_int = 5;
pub const CLUSTER_VPC_PS: c_int = 6;
pub const CLUSTER_NONE: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_registers {
    pub registers: *const u32,
    pub count: usize,
    pub val0: u32,
    pub val1: u32,
}

// RBBM
// CP
// VSC
// UCHE
// GRAS
// VPC
// PC
// VFD
// HLSQ
// UCHE

// RBBM_STATUS - RBBM_STATUS3
// CP_STATUS_1
// GMU GX
// GMU CX
// GMU AO
// GPU CC
// GPU CC ACD
// GPU CC
// GPU RSCC
extern "C" {
    pub fn a6xx_get_cp_roq_size(gpu: *mut msm_gpu) -> static u32;
}
extern "C" {
    pub fn a7xx_get_cp_roq_size(gpu: *mut msm_gpu) -> static u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_indexed_registers {
    pub name: *const c_char,
    pub addr: u32,
    pub data: u32,
    pub count: u32,
    pub gpu): *mut *mut u32 (count_fn)(struct msm_gpu,
}

pub const STATE_NON_CONTEXT: c_int = 0;
pub const STATE_TOGGLE_CTXT: c_int = 1;
pub const STATE_FORCE_CTXT_0: c_int = 2;
pub const STATE_FORCE_CTXT_1: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen7_sel_reg {
    pub host_reg: c_uint,
    pub cd_reg: c_uint,
    pub val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen7_cluster_registers {
// cluster_id: Cluster identifier
    pub cluster_id: c_int,
// pipe_id: Pipe Identifier
    pub pipe_id: c_int,
// context_id: one of STATE_ that identifies the context to dump
    pub context_id: c_int,
// regs: Pointer to an array of register pairs
    pub regs: *const u32,
// sel: Pointer to a selector register to write before reading
    pub sel: *const gen7_sel_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen7_sptp_cluster_registers {
// cluster_id: Cluster identifier
    pub cluster_id: a7xx_cluster,
// statetype: SP block state type for the cluster
    pub statetype: a7xx_statetype_id,
// pipe_id: Pipe identifier
    pub pipe_id: adreno_pipe,
// context_id: Context identifier
    pub context_id: c_int,
// location_id: Location identifier
    pub location_id: a7xx_state_location,
// regs: Pointer to the list of register pairs to read
    pub regs: *const u32,
// regbase: Dword offset of the register block in the GPu register space
    pub regbase: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen7_shader_block {
// statetype: Type identifer for the block
    pub statetype: u32,
// size: Size of the block (in dwords)
    pub size: u32,
// num_sps: The SP id to dump
    pub num_sps: u32,
// num_usptps: The number of USPTPs to dump */;
    pub num_usptps: u32,
// pipe_id: Pipe identifier for the block data
    pub pipeid: u32,
// location: Location identifer for the block data
    pub location: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen7_reg_list {
    pub regs: *const u32,
    pub sel: *const gen7_sel_reg,
}

// adreno_gen7_x_y_snapshot.h defines which debugbus blocks a given family has, but the
// list of debugbus blocks is global on a7xx.
//

