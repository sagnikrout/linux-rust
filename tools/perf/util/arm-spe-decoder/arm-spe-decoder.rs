//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/arm-spe-decoder/arm-spe-decoder.h
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
// arm_spe_decoder.h: Arm Statistical Profiling Extensions support
// Copyright (c) 2019-2020, Arm Ltd.
//

// Macro flag: #define INCLUDE__ARM_SPE_DECODER_H__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_op_type {
// First level operation type
    ARM_SPE_OP_OTHER	= 1 << 0,
    ARM_SPE_OP_LDST		= 1 << 1,
    ARM_SPE_OP_BRANCH_ERET	= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_2nd_op_ldst {
    ARM_SPE_OP_GP_REG		= 1 << 8,
    ARM_SPE_OP_UNSPEC_REG		= 1 << 9,
    ARM_SPE_OP_NV_SYSREG		= 1 << 10,
    ARM_SPE_OP_SIMD_FP		= 1 << 11,
    ARM_SPE_OP_SVE			= 1 << 12,
    ARM_SPE_OP_MTE_TAG		= 1 << 13,
    ARM_SPE_OP_MEMCPY		= 1 << 14,
    ARM_SPE_OP_MEMSET		= 1 << 15,
    ARM_SPE_OP_GCS			= 1 << 16,
    ARM_SPE_OP_SME			= 1 << 17,
    ARM_SPE_OP_ASE			= 1 << 18,

// Assisted information for memory / SIMD
    ARM_SPE_OP_LD			= 1 << 20,
    ARM_SPE_OP_ST			= 1 << 21,
    ARM_SPE_OP_ATOMIC		= 1 << 22,
    ARM_SPE_OP_EXCL			= 1 << 23,
    ARM_SPE_OP_AR			= 1 << 24,
    ARM_SPE_OP_DP			= 1 << 25,	/* Data processing */
    ARM_SPE_OP_PRED			= 1 << 26,	/* Predicated */
    ARM_SPE_OP_SG			= 1 << 27,	/* Gather/Scatter */
    ARM_SPE_OP_COMM			= 1 << 28,	/* Common */
    ARM_SPE_OP_FP			= 1 << 29,	/* Floating-point */
    ARM_SPE_OP_COND			= 1 << 30,	/* Conditional */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_2nd_op_branch {
    ARM_SPE_OP_BR_COND		= 1 << 8,
    ARM_SPE_OP_BR_INDIRECT		= 1 << 9,
    ARM_SPE_OP_BR_GCS		= 1 << 10,
    ARM_SPE_OP_BR_CR_BL		= 1 << 11,
    ARM_SPE_OP_BR_CR_RET		= 1 << 12,
    ARM_SPE_OP_BR_CR_NON_BL_RET	= 1 << 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_common_data_source {
    ARM_SPE_COMMON_DS_L1D		= 0x0,
    ARM_SPE_COMMON_DS_L2		= 0x8,
    ARM_SPE_COMMON_DS_PEER_CORE	= 0x9,
    ARM_SPE_COMMON_DS_LOCAL_CLUSTER = 0xa,
    ARM_SPE_COMMON_DS_SYS_CACHE	= 0xb,
    ARM_SPE_COMMON_DS_PEER_CLUSTER	= 0xc,
    ARM_SPE_COMMON_DS_REMOTE	= 0xd,
    ARM_SPE_COMMON_DS_DRAM		= 0xe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_ampereone_data_source {
    ARM_SPE_AMPEREONE_LOCAL_CHIP_CACHE_OR_DEVICE    = 0x0,
    ARM_SPE_AMPEREONE_SLC                           = 0x3,
    ARM_SPE_AMPEREONE_REMOTE_CHIP_CACHE             = 0x5,
    ARM_SPE_AMPEREONE_DDR                           = 0x7,
    ARM_SPE_AMPEREONE_L1D                           = 0x8,
    ARM_SPE_AMPEREONE_L2D                           = 0x9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_hisi_hip_data_source {
    ARM_SPE_HISI_HIP_PEER_CPU		= 0,
    ARM_SPE_HISI_HIP_PEER_CPU_HITM		= 1,
    ARM_SPE_HISI_HIP_L3			= 2,
    ARM_SPE_HISI_HIP_L3_HITM		= 3,
    ARM_SPE_HISI_HIP_PEER_CLUSTER		= 4,
    ARM_SPE_HISI_HIP_PEER_CLUSTER_HITM	= 5,
    ARM_SPE_HISI_HIP_REMOTE_SOCKET		= 6,
    ARM_SPE_HISI_HIP_REMOTE_SOCKET_HITM	= 7,
    ARM_SPE_HISI_HIP_LOCAL_MEM		= 8,
    ARM_SPE_HISI_HIP_REMOTE_MEM		= 9,
    ARM_SPE_HISI_HIP_NC_DEV			= 13,
    ARM_SPE_HISI_HIP_L2			= 16,
    ARM_SPE_HISI_HIP_L2_HITM		= 17,
    ARM_SPE_HISI_HIP_L1			= 18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_spe_record {
    pub type: u64,
    pub err: c_int,
    pub op: u32,
    pub latency: u32,
    pub from_ip: u64,
    pub to_ip: u64,
    pub prev_br_tgt: u64,
    pub timestamp: u64,
    pub virt_addr: u64,
    pub phys_addr: u64,
    pub context_id: u64,
    pub source: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_spe_buffer {
    pub buf: *const c_uchar,
    pub len: usize,
    pub offset: u64,
    pub trace_nr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_spe_params {
    pub data): *mut *mut *mut int (get_trace)(struct arm_spe_buffer buffer, void,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_spe_decoder {
    pub data): *mut *mut *mut int (get_trace)(struct arm_spe_buffer buffer, void,
    pub data: *mut c_void,
    pub record: arm_spe_record,
    pub buf: *const c_uchar,
    pub len: usize,
    pub midr: u64,
}

extern "C" {
    pub fn arm_spe_decoder_free(decoder: *mut arm_spe_decoder);
}
extern "C" {
    pub fn arm_spe_decode(decoder: *mut arm_spe_decoder) -> c_int;
}
