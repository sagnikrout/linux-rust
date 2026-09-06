//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/smumgr/smu7_smumgr.h
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

pub const SMC_RAM_END: c_uint = 0x40000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu7_buffer_entry {
    pub data_size: u32,
    pub mc_addr: u64,
    pub kaddr: *mut c_void,
    pub handle: *mut amdgpu_bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu7_smumgr {
    pub smu_buffer: smu7_buffer_entry,
    pub header_buffer: smu7_buffer_entry,
    pub toc: *mut SMU_DRAMData_TOC,
    pub soft_regs_start: u32,
    pub dpm_table_start: u32,
    pub mc_reg_table_start: u32,
    pub fan_table_start: u32,
    pub arb_table_start: u32,
    pub ulv_setting_starts: u32,
    pub security_hard_key: u8,
    pub acpi_optimization: u32,
    pub avfs_btc_param: u32,
}

extern "C" {
    pub fn smu7_program_jump_on_start(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_is_smc_ram_running(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn smu7_send_msg_to_smc(hwmgr: *mut pp_hwmgr, msg: u16) -> c_int;
}
extern "C" {
    pub fn smu7_get_argument(hwmgr: *mut pp_hwmgr) -> u32;
}
extern "C" {
    pub fn smu7_send_msg_to_smc_offset(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_convert_fw_type_to_cgs(fw_type: u32) -> cgs_ucode_id;
}
extern "C" {
    pub fn smu7_request_smu_load_fw(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_check_fw_load_finish(hwmgr: *mut pp_hwmgr, fw_type: u32) -> c_int;
}
extern "C" {
    pub fn smu7_reload_firmware(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_upload_smu_firmware_image(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_smu_fini(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_setup_pwr_virus(hwmgr: *mut pp_hwmgr) -> c_int;
}
