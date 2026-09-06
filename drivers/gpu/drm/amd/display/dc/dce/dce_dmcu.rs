//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_dmcu.h
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
// Copyright 2012-16 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

// Macro flag: #define DMCU_DCN10_REG_LIST()\
// Macro flag: #define DMCU_DCN20_REG_LIST()\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_dmcu_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_dmcu_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_dmcu_registers {
    pub DMCU_CTRL: u32,
    pub DMCU_STATUS: u32,
    pub DMCU_RAM_ACCESS_CTRL: u32,
    pub DCI_MEM_PWR_STATUS: u32,
    pub DMU_MEM_PWR_CNTL: u32,
    pub DMCU_IRAM_WR_CTRL: u32,
    pub DMCU_IRAM_WR_DATA: u32,
    pub MASTER_COMM_DATA_REG1: u32,
    pub MASTER_COMM_DATA_REG2: u32,
    pub MASTER_COMM_DATA_REG3: u32,
    pub MASTER_COMM_CMD_REG: u32,
    pub MASTER_COMM_CNTL_REG: u32,
    pub SLAVE_COMM_DATA_REG1: u32,
    pub SLAVE_COMM_DATA_REG2: u32,
    pub SLAVE_COMM_DATA_REG3: u32,
    pub SLAVE_COMM_CMD_REG: u32,
    pub SLAVE_COMM_CNTL_REG: u32,
    pub DMCU_IRAM_RD_CTRL: u32,
    pub DMCU_IRAM_RD_DATA: u32,
    pub DMCU_INTERRUPT_TO_UC_EN_MASK: u32,
    pub SMU_INTERRUPT_CONTROL: u32,
    pub DC_DMCU_SCRATCH: u32,
    pub DMCUB_SCRATCH15: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_dmcu {
    pub base: dmcu,
    pub regs: *const dce_dmcu_registers,
    pub dmcu_shift: *const dce_dmcu_shift,
    pub dmcu_mask: *const dce_dmcu_mask,
}

//
// MASTER_COMM_DATA_REG1   Bit position    Data
// 7:0	            hyst_frames[7:0]
// 14:8	        hyst_lines[6:0]
// 15	            RFB_UPDATE_AUTO_EN
// 18:16	        phy_num[2:0]
// 21:19	        dcp_sel[2:0]
// 22	            phy_type
// 23	            frame_cap_ind
// 26:24	        aux_chan[2:0]
// 30:27	        aux_repeat[3:0]
// 31:31	        reserved[31:31]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dce_dmcu_psr_config_data_reg1 {
    pub /*[7:0]*/: *mut unsigned int timehyst_frames:8;,
    pub /*[14:8]*/: *mut unsigned int hyst_lines:7;,
    pub /*[15:15]*/: *mut unsigned int rfb_update_auto_en:1;,
    pub /*[18:16]*/: *mut unsigned int dp_port_num:3;,
    pub /*[21:19]*/: *mut unsigned int dcp_sel:3;,
    pub /*[22:22]*/: *mut unsigned int phy_type:1;,
    pub /*[23:23]*/: *mut unsigned int frame_cap_ind:1;,
    pub /*[26:24]*/: *mut unsigned int aux_chan:3;,
    pub /*[30:27]*/: *mut unsigned int aux_repeat:4;,
    pub /*[31:31]*/: *mut unsigned int allow_smu_optimizations:1;,
    pub bits: },
    pub u32All: c_uint,
}

//
// MASTER_COMM_DATA_REG2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dce_dmcu_psr_config_data_reg2 {
    pub /*[2:0]*/: *mut unsigned int dig_fe:3;,
    pub /*[5:3]*/: *mut unsigned int dig_be:3;,
    pub /*[6:6]*/: *mut unsigned int skip_wait_for_pll_lock:1;,
    pub /*[15:7]*/: *mut unsigned int reserved:9;,
    pub /*[23:16]*/: *mut unsigned int frame_delay:8;,
    pub /*[27:24]*/: *mut unsigned int smu_phy_id:4;,
    pub /*[31:28]*/: *mut unsigned int num_of_controllers:4;,
    pub bits: },
    pub u32All: c_uint,
}

//
// MASTER_COMM_DATA_REG3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dce_dmcu_psr_config_data_reg3 {
    pub /*[15:0]*/: *mut unsigned int psr_level:16;,
    pub /*[19:16]*/: *mut unsigned int link_rate:4;,
    pub /*[31:20]*/: *mut unsigned int reserved:12;,
    pub bits: },
    pub u32All: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dce_dmcu_psr_config_data_wait_loop_reg1 {
    pub /: *mut *mut unsigned int wait_loop:16; / [15:0],
    pub /: *mut *mut unsigned int reserved:16; / [31:16],
    pub bits: },
    pub u32: c_uint,
}

extern "C" {
    pub fn dce_dmcu_destroy(dmcu: *mut dmcu);
}
