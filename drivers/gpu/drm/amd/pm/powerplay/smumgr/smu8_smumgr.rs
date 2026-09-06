//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/smumgr/smu8_smumgr.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
pub const MAX_NUM_FIRMWARE: c_int = 8;
pub const MAX_NUM_SCRATCH: c_int = 11;
pub const SMU8_SCRATCH_SIZE_NONGFX_CLOCKGATING: c_int = 1024;
pub const SMU8_SCRATCH_SIZE_NONGFX_GOLDENSETTING: c_int = 2048;
pub const SMU8_SCRATCH_SIZE_SDMA_METADATA: c_int = 1024;

pub const SMU_EnabledFeatureScoreboard_SclkDpmOn: c_uint = 0x00200000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu8_scratch_entry {
    SMU8_SCRATCH_ENTRY_UCODE_ID_SDMA0 = 0,
    SMU8_SCRATCH_ENTRY_UCODE_ID_SDMA1,
    SMU8_SCRATCH_ENTRY_UCODE_ID_CP_CE,
    SMU8_SCRATCH_ENTRY_UCODE_ID_CP_PFP,
    SMU8_SCRATCH_ENTRY_UCODE_ID_CP_ME,
    SMU8_SCRATCH_ENTRY_UCODE_ID_CP_MEC_JT1,
    SMU8_SCRATCH_ENTRY_UCODE_ID_CP_MEC_JT2,
    SMU8_SCRATCH_ENTRY_UCODE_ID_GMCON_RENG,
    SMU8_SCRATCH_ENTRY_UCODE_ID_RLC_G,
    SMU8_SCRATCH_ENTRY_UCODE_ID_RLC_SCRATCH,
    SMU8_SCRATCH_ENTRY_UCODE_ID_RLC_SRM_ARAM,
    SMU8_SCRATCH_ENTRY_UCODE_ID_RLC_SRM_DRAM,
    SMU8_SCRATCH_ENTRY_UCODE_ID_DMCU_ERAM,
    SMU8_SCRATCH_ENTRY_UCODE_ID_DMCU_IRAM,
    SMU8_SCRATCH_ENTRY_UCODE_ID_POWER_PROFILING,
    SMU8_SCRATCH_ENTRY_DATA_ID_SDMA_HALT,
    SMU8_SCRATCH_ENTRY_DATA_ID_SYS_CLOCKGATING,
    SMU8_SCRATCH_ENTRY_DATA_ID_SDMA_RING_REGS,
    SMU8_SCRATCH_ENTRY_DATA_ID_NONGFX_REINIT,
    SMU8_SCRATCH_ENTRY_DATA_ID_SDMA_START,
    SMU8_SCRATCH_ENTRY_DATA_ID_IH_REGISTERS,
    SMU8_SCRATCH_ENTRY_SMU8_FUSION_CLKTABLE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_buffer_entry {
    pub data_size: u32,
    pub mc_addr: u64,
    pub kaddr: *mut c_void,
    pub firmware_ID: smu8_scratch_entry,
    pub /: *mut *mut *mut amdgpu_bo handle; / as bo handle used when release bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_register_index_data_pair {
    pub offset: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_ih_meta_data {
    pub command: u32,
    pub register_index_value_pair: [smu8_register_index_data_pair; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_smumgr {
    pub driver_buffer_length: u8,
    pub scratch_buffer_length: u8,
    pub toc_entry_used_count: u16,
    pub toc_entry_initialize_index: u16,
    pub toc_entry_power_profiling_index: u16,
    pub toc_entry_aram: u16,
    pub toc_entry_ih_register_restore_task_index: u16,
    pub toc_entry_clock_table: u16,
    pub ih_register_restore_task_size: u16,
    pub smu_buffer_used_bytes: u16,
    pub toc_buffer: smu8_buffer_entry,
    pub smu_buffer: smu8_buffer_entry,
    pub firmware_buffer: smu8_buffer_entry,
    pub driver_buffer: [smu8_buffer_entry; MAX_NUM_FIRMWARE],
    pub meta_data_buffer: [smu8_buffer_entry; MAX_NUM_FIRMWARE],
    pub scratch_buffer: [smu8_buffer_entry; MAX_NUM_SCRATCH],
}
