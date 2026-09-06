//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_link_encoder.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Macro flag: #define TO_DCE110_LINK_ENC(link_encoder)\
// Not found regs in dce120 spec
// BIOS_SCRATCH_2
// DP_DPHY_INTERNAL_CTRL
//
// Macro flag: #define AUX_REG_LIST(id)\
// Macro flag: #define HPD_REG_LIST(id)\

// Macro flag: #define LE_COMMON_REG_LIST(id)\

// Macro flag: #define LE_DCE60_REG_LIST(id)\

// Macro flag: #define LE_DCE80_REG_LIST(id)\
// Macro flag: #define LE_DCE100_REG_LIST(id)\
// Macro flag: #define LE_DCE110_REG_LIST(id)\
// Macro flag: #define LE_DCE120_REG_LIST(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_link_enc_aux_registers {
    pub AUX_CONTROL: u32,
    pub AUX_DPHY_RX_CONTROL0: u32,
    pub AUX_DPHY_RX_CONTROL1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_link_enc_hpd_registers {
    pub DC_HPD_CONTROL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_link_enc_registers {
// DMCU registers
    pub MASTER_COMM_DATA_REG1: u32,
    pub MASTER_COMM_DATA_REG2: u32,
    pub MASTER_COMM_DATA_REG3: u32,
    pub MASTER_COMM_CMD_REG: u32,
    pub MASTER_COMM_CNTL_REG: u32,
    pub DMCU_RAM_ACCESS_CTRL: u32,
    pub DCI_MEM_PWR_STATUS: u32,
    pub DMU_MEM_PWR_CNTL: u32,
    pub DMCU_IRAM_RD_CTRL: u32,
    pub DMCU_IRAM_RD_DATA: u32,
    pub DMCU_INTERRUPT_TO_UC_EN_MASK: u32,
// Common DP registers
    pub DIG_BE_CNTL: u32,
    pub DIG_BE_EN_CNTL: u32,
    pub DP_CONFIG: u32,
    pub DP_DPHY_CNTL: u32,
    pub DP_DPHY_INTERNAL_CTRL: u32,
    pub DP_DPHY_PRBS_CNTL: u32,
    pub DP_DPHY_SCRAM_CNTL: u32,
    pub DP_DPHY_SYM0: u32,
    pub DP_DPHY_SYM1: u32,
    pub DP_DPHY_SYM2: u32,
    pub DP_DPHY_TRAINING_PATTERN_SEL: u32,
    pub DP_LINK_CNTL: u32,
    pub DP_LINK_FRAMING_CNTL: u32,
    pub DP_MSE_SAT0: u32,
    pub DP_MSE_SAT1: u32,
    pub DP_MSE_SAT2: u32,
    pub DP_MSE_SAT_UPDATE: u32,
    pub DP_SEC_CNTL: u32,
    pub DP_VID_STREAM_CNTL: u32,
    pub DP_DPHY_FAST_TRAINING: u32,
    pub DP_DPHY_BS_SR_SWAP_CNTL: u32,
    pub DP_DPHY_HBR2_PATTERN_CONTROL: u32,
    pub DP_SEC_CNTL1: u32,
// DAC registers
    pub DAC_ENABLE: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_link_encoder {
    pub base: link_encoder,
    pub link_regs: *const dce110_link_enc_registers,
    pub aux_regs: *const dce110_link_enc_aux_registers,
    pub hpd_regs: *const dce110_link_enc_hpd_registers,
}

// HW programming
// initialize HW */  /* why do we initialze aux in here?
extern "C" {
    pub fn dce110_link_encoder_hw_init(enc: *mut link_encoder);
}
extern "C" {
    pub fn dce110_link_encoder_destroy(enc: *mut link_encoder);
}
// program DIG_MODE in DIG_BE
// TODO can this be combined with enable_output?
// enables TMDS PHY output
// TODO: still need depth or just pass in adjusted pixel clock?
// enables DP PHY output
// enables DP PHY output in MST mode
// enables LVDS PHY output
// enables analog output from the DAC
// disable PHY output
// set DP lane settings
// programs DP MST VC payload allocation
extern "C" {
    pub fn dce110_get_dig_frontend(enc: *mut link_encoder) -> c_uint;
}
extern "C" {
    pub fn dce110_link_encoder_enable_hpd(enc: *mut link_encoder);
}
extern "C" {
    pub fn dce110_link_encoder_disable_hpd(enc: *mut link_encoder);
}
extern "C" {
    pub fn dce110_is_dig_enabled(enc: *mut link_encoder) -> bool;
}
extern "C" {
    pub fn dce110_get_hpd_state(enc: *mut link_encoder) -> bool;
}
extern "C" {
    pub fn dce110_program_hpd_filter(enc: *mut link_encoder, delay_on_connect_in_ms: c_int, delay_on_disconnect_in_ms: c_int) -> bool;
}
