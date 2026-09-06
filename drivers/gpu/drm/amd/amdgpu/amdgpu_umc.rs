//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_umc.h
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
// Copyright (C) 2019  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

//
// (addr / 256) * 4096, the higher 26 bits in ErrorAddr
// is the index of 4KB block
//

//
// (addr / 256) * 8192, the higher 26 bits in ErrorAddr
// is the index of 8KB block
//

//
// (addr / 256) * 32768, the higher 26 bits in ErrorAddr
// is the index of 8KB block
//

// channel index is the index of 256B block

// offset in 256B block

// Page retirement tag
pub const UMC_ECC_NEW_DETECTED_TAG: c_uint = 0x1;
//
// a flag to indicate v2 of channel index stored in eeprom
//
// v1 (legacy way): store channel index within a umc instance in eeprom
// range in UMC v12: 0 ~ 7
// v2: store global channel index in eeprom
// range in UMC v12: 0 ~ 127
//
// NOTE: it's better to store it in eeprom_table_record.mem_channel,
// but there is only 8 bits in mem_channel, and the channel number may
// increase in the future, we decide to save it in
// eeprom_table_record.retired_page. retired_page is useless in v2,
// we depend on eeprom_table_record.address instead of retired_page in v2.
// Only 48 bits are saved on eeprom, use bit 47 here.
//

//
// save nps value to eeprom_table_record.retired_page[47:40],
// the channel index flag above will be retired.
//
pub const UMC_NPS_SHIFT: c_int = 40;
pub const UMC_NPS_MASK: c_uint = 0xffULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_umc_ras {
    pub ras_block: amdgpu_ras_block_object,
    pub adev): *mut *mut void (err_cnt_init)(struct amdgpu_device,
    pub adev): *mut *mut bool (query_ras_poison_mode)(struct amdgpu_device,
    pub ras_error_status): *mut c_void,
    pub ras_error_status): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_umc_funcs {
    pub adev): *mut *mut void (init_registers)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_umc {
// max error count in one ras query call
    pub max_ras_err_cnt_per_query: u32,
// number of umc channel instance with memory map register access
    pub channel_inst_num: u32,
// number of umc instance with memory map register access
    pub umc_inst_num: u32,
// Total number of umc node instance including harvest one
    pub node_inst_num: u32,
// UMC regiser per channel offset
    pub channel_offs: u32,
// how many pages are retired in one UE
    pub retire_unit: u32,
// channel index table of interleaved memory
    pub channel_idx_tbl: *const u32,
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_umc_funcs,
    pub ras: *mut amdgpu_umc_ras,
// active mask for umc node instance
    pub active_mask: c_ulong,
    pub err_addr_cnt: c_ulong,
}

extern "C" {
    pub fn amdgpu_umc_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_umc_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> c_int;
}
