//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu_ucode_xfer_cz.h
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


//
// Copyright 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// CZ Ucode Loading Definitions
pub const NUM_JOBLIST_ENTRIES: c_int = 32;
pub const TASK_TYPE_NO_ACTION: c_int = 0;
pub const TASK_TYPE_UCODE_LOAD: c_int = 1;
pub const TASK_TYPE_UCODE_SAVE: c_int = 2;
pub const TASK_TYPE_REG_LOAD: c_int = 3;
pub const TASK_TYPE_REG_SAVE: c_int = 4;
pub const TASK_TYPE_INITIALIZE: c_int = 5;
pub const TASK_ARG_REG_SMCIND: c_int = 0;
pub const TASK_ARG_REG_MMIO: c_int = 1;
pub const TASK_ARG_REG_FCH: c_int = 2;
pub const TASK_ARG_REG_UNB: c_int = 3;
pub const TASK_ARG_INIT_MM_PWR_LOG: c_int = 0;
pub const TASK_ARG_INIT_CLK_TABLE: c_int = 1;
pub const JOB_GFX_SAVE: c_int = 0;
pub const JOB_GFX_RESTORE: c_int = 1;
pub const JOB_FCH_SAVE: c_int = 2;
pub const JOB_FCH_RESTORE: c_int = 3;
pub const JOB_UNB_SAVE: c_int = 4;
pub const JOB_UNB_RESTORE: c_int = 5;
pub const JOB_GMC_SAVE: c_int = 6;
pub const JOB_GMC_RESTORE: c_int = 7;
pub const JOB_GNB_SAVE: c_int = 8;
pub const JOB_GNB_RESTORE: c_int = 9;
pub const IGNORE_JOB: c_uint = 0xff;

// Size of DRAM regions (in bytes) requested by SMU:
pub const SMU_DRAM_REQ_MM_PWR_LOG: c_int = 48;
pub const UCODE_ID_SDMA0: c_int = 0;
pub const UCODE_ID_SDMA1: c_int = 1;
pub const UCODE_ID_CP_CE: c_int = 2;
pub const UCODE_ID_CP_PFP: c_int = 3;
pub const UCODE_ID_CP_ME: c_int = 4;
pub const UCODE_ID_CP_MEC_JT1: c_int = 5;
pub const UCODE_ID_CP_MEC_JT2: c_int = 6;
pub const UCODE_ID_GMCON_RENG: c_int = 7;
pub const UCODE_ID_RLC_G: c_int = 8;
pub const UCODE_ID_RLC_SCRATCH: c_int = 9;
pub const UCODE_ID_RLC_SRM_ARAM: c_int = 10;
pub const UCODE_ID_RLC_SRM_DRAM: c_int = 11;
pub const UCODE_ID_DMCU_ERAM: c_int = 12;
pub const UCODE_ID_DMCU_IRAM: c_int = 13;
pub const UCODE_ID_SDMA0_MASK: c_uint = 0x00000001;
pub const UCODE_ID_SDMA1_MASK: c_uint = 0x00000002;
pub const UCODE_ID_CP_CE_MASK: c_uint = 0x00000004;
pub const UCODE_ID_CP_PFP_MASK: c_uint = 0x00000008;
pub const UCODE_ID_CP_ME_MASK: c_uint = 0x00000010;
pub const UCODE_ID_CP_MEC_JT1_MASK: c_uint = 0x00000020;
pub const UCODE_ID_CP_MEC_JT2_MASK: c_uint = 0x00000040;
pub const UCODE_ID_GMCON_RENG_MASK: c_uint = 0x00000080;
pub const UCODE_ID_RLC_G_MASK: c_uint = 0x00000100;
pub const UCODE_ID_RLC_SCRATCH_MASK: c_uint = 0x00000200;
pub const UCODE_ID_RLC_SRM_ARAM_MASK: c_uint = 0x00000400;
pub const UCODE_ID_RLC_SRM_DRAM_MASK: c_uint = 0x00000800;
pub const UCODE_ID_DMCU_ERAM_MASK: c_uint = 0x00001000;
pub const UCODE_ID_DMCU_IRAM_MASK: c_uint = 0x00002000;
pub const UCODE_ID_SDMA0_SIZE_BYTE: c_int = 10368;
pub const UCODE_ID_SDMA1_SIZE_BYTE: c_int = 10368;
pub const UCODE_ID_CP_CE_SIZE_BYTE: c_int = 8576;
pub const UCODE_ID_CP_PFP_SIZE_BYTE: c_int = 16768;
pub const UCODE_ID_CP_ME_SIZE_BYTE: c_int = 16768;
pub const UCODE_ID_CP_MEC_JT1_SIZE_BYTE: c_int = 384;
pub const UCODE_ID_CP_MEC_JT2_SIZE_BYTE: c_int = 384;
pub const UCODE_ID_GMCON_RENG_SIZE_BYTE: c_int = 4096;
pub const UCODE_ID_RLC_G_SIZE_BYTE: c_int = 2048;
pub const UCODE_ID_RLC_SCRATCH_SIZE_BYTE: c_int = 132;
pub const UCODE_ID_RLC_SRM_ARAM_SIZE_BYTE: c_int = 8192;
pub const UCODE_ID_RLC_SRM_DRAM_SIZE_BYTE: c_int = 4096;
pub const UCODE_ID_DMCU_ERAM_SIZE_BYTE: c_int = 24576;
pub const UCODE_ID_DMCU_IRAM_SIZE_BYTE: c_int = 1024;
pub const NUM_UCODES: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_Task {
    pub type: u8,
    pub arg: u8,
    pub next: u16,
    pub addr: data_64_t,
    pub size_bytes: u32,
}

pub type SMU_Task = SMU_Task;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TOC {
    pub JobList: [u8; NUM_JOBLIST_ENTRIES],
    pub tasks: [SMU_Task; ],
}

// META DATA COMMAND Definitions
pub const METADATA_CMD_MODE0: c_uint = 0x00000103;
pub const METADATA_CMD_MODE1: c_uint = 0x00000113;
pub const METADATA_CMD_MODE2: c_uint = 0x00000123;
pub const METADATA_CMD_MODE3: c_uint = 0x00000133;
pub const METADATA_CMD_DELAY: c_uint = 0x00000203;
pub const METADATA_CMD_CHNG_REGSPACE: c_uint = 0x00000303;
pub const METADATA_PERFORM_ON_SAVE: c_uint = 0x00001000;
pub const METADATA_PERFORM_ON_LOAD: c_uint = 0x00002000;
pub const METADATA_CMD_ARG_MASK: c_uint = 0xFFFF0000;
pub const METADATA_CMD_ARG_SHIFT: c_int = 16;
// Simple register addr/data fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_MetaData_Mode0 {
    pub register_address: u32,
    pub register_data: u32,
}

pub type SMU_MetaData_Mode0 = SMU_MetaData_Mode0;
// Register addr/data with mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_MetaData_Mode1 {
    pub register_address: u32,
    pub register_mask: u32,
    pub register_data: u32,
}

pub type SMU_MetaData_Mode1 = SMU_MetaData_Mode1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_MetaData_Mode2 {
    pub register_address: u32,
    pub register_mask: u32,
    pub target_value: u32,
}

pub type SMU_MetaData_Mode2 = SMU_MetaData_Mode2;
// Always write data (even on a save operation)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_MetaData_Mode3 {
    pub register_address: u32,
    pub register_mask: u32,
    pub register_data: u32,
}

pub type SMU_MetaData_Mode3 = SMU_MetaData_Mode3;
