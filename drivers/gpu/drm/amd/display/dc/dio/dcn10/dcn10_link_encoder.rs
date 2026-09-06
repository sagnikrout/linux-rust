//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dio/dcn10/dcn10_link_encoder.h
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

// Macro flag: #define TO_DCN10_LINK_ENC(link_encoder)\
// Macro flag: #define AUX_REG_LIST(id)\
// Macro flag: #define HPD_REG_LIST(id)\

// Macro flag: #define LE_DCN10_REG_LIST(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_enc_aux_registers {
    pub AUX_CONTROL: u32,
    pub AUX_DPHY_RX_CONTROL0: u32,
    pub AUX_DPHY_TX_CONTROL: u32,
    pub AUX_DPHY_RX_CONTROL1: u32,
    pub DC_GPIO_DDC: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_enc_hpd_registers {
    pub DC_HPD_CONTROL: u32,
    pub DC_HPD_INT_STATUS: u32,
    pub DC_HPD_TOGGLE_FILT_CNTL: u32,
    pub HPD_CTRL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_enc_registers {
    pub DIG_BE_CNTL: u32,
    pub DIG_BE_EN_CNTL: u32,
    pub DIG_CLOCK_PATTERN: u32,
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
    pub TMDS_CTL_BITS: u32,
// DCCG
    pub CLOCK_ENABLE: u32,
// DIG
    pub DIG_LANE_ENABLE: u32,
// UNIPHY
    pub CHANNEL_XBAR_CNTL: u32,
// DPCS
    pub RDPCSTX_PHY_CNTL3: u32,
    pub RDPCSTX_PHY_CNTL4: u32,
    pub RDPCSTX_PHY_CNTL5: u32,
    pub RDPCSTX_PHY_CNTL6: u32,
    pub RDPCSPIPE_PHY_CNTL6: u32,
    pub RDPCSTX_PHY_CNTL7: u32,
    pub RDPCSTX_PHY_CNTL8: u32,
    pub RDPCSTX_PHY_CNTL9: u32,
    pub RDPCSTX_PHY_CNTL10: u32,
    pub RDPCSTX_PHY_CNTL11: u32,
    pub RDPCSTX_PHY_CNTL12: u32,
    pub RDPCSTX_PHY_CNTL13: u32,
    pub RDPCSTX_PHY_CNTL14: u32,
    pub RDPCSTX_PHY_CNTL15: u32,
    pub RDPCSTX_CNTL: u32,
    pub RDPCSTX_CLOCK_CNTL: u32,
    pub RDPCSTX_PHY_CNTL0: u32,
    pub RDPCSTX_PHY_CNTL2: u32,
    pub RDPCSTX_PLL_UPDATE_DATA: u32,
    pub RDPCS_TX_CR_ADDR: u32,
    pub RDPCS_TX_CR_DATA: u32,
    pub DPCSTX_TX_CLOCK_CNTL: u32,
    pub DPCSTX_TX_CNTL: u32,
    pub RDPCSTX_INTERRUPT_CONTROL: u32,
    pub RDPCSTX_PHY_FUSE0: u32,
    pub RDPCSTX_PHY_FUSE1: u32,
    pub RDPCSTX_PHY_FUSE2: u32,
    pub RDPCSTX_PHY_FUSE3: u32,
    pub RDPCSTX_PHY_RX_LD_VAL: u32,
    pub DPCSTX_DEBUG_CONFIG: u32,
    pub RDPCSTX_DEBUG_CONFIG: u32,
    pub RDPCSTX0_RDPCSTX_SCRATCH: u32,
    pub RDPCSTX_DMCU_DPALT_DIS_BLOCK_REG: u32,
    pub DCIO_SOFT_RESET: u32,
// indirect registers
    pub RAWLANE0_DIG_PCS_XF_RX_OVRD_IN_2: u32,
    pub RAWLANE0_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE1_DIG_PCS_XF_RX_OVRD_IN_2: u32,
    pub RAWLANE1_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE2_DIG_PCS_XF_RX_OVRD_IN_2: u32,
    pub RAWLANE2_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE3_DIG_PCS_XF_RX_OVRD_IN_2: u32,
    pub RAWLANE3_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub TMDS_DCBALANCER_CONTROL: u32,
    pub PHYA_LINK_CNTL2: u32,
    pub PHYB_LINK_CNTL2: u32,
    pub PHYC_LINK_CNTL2: u32,
    pub DIO_LINKA_CNTL: u32,
    pub DIO_LINKB_CNTL: u32,
    pub DIO_LINKC_CNTL: u32,
    pub DIO_LINKD_CNTL: u32,
    pub DIO_LINKE_CNTL: u32,
    pub DIO_LINKF_CNTL: u32,
    pub DIO_CLK_CNTL: u32,
    pub DIG_BE_CLK_CNTL: u32,
    pub HDCP_I2C_CONTROL_0: u32,
    pub HDCP_INT_CONTROL: u32,
}

// Macro flag: #define LINK_ENCODER_MASK_SH_LIST_DCN10(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_enc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_enc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_link_encoder {
    pub base: link_encoder,
    pub link_regs: *const dcn10_link_enc_registers,
    pub aux_regs: *const dcn10_link_enc_aux_registers,
    pub hpd_regs: *const dcn10_link_enc_hpd_registers,
    pub link_shift: *const dcn10_link_enc_shift,
    pub link_mask: *const dcn10_link_enc_mask,
}

// HW programming
// initialize HW */  /* why do we initialze aux in here?
extern "C" {
    pub fn dcn10_link_encoder_hw_init(enc: *mut link_encoder);
}
extern "C" {
    pub fn dcn10_link_encoder_destroy(enc: *mut link_encoder);
}
// program DIG_MODE in DIG_BE
// TODO can this be combined with enable_output?
// enables TMDS PHY output
// TODO: still need depth or just pass in adjusted pixel clock?
// enables DP PHY output
// enables DP PHY output in MST mode
// disable PHY output
// set DP lane settings
// programs DP MST VC payload allocation
extern "C" {
    pub fn dcn10_link_encoder_enable_hpd(enc: *mut link_encoder);
}
extern "C" {
    pub fn dcn10_link_encoder_disable_hpd(enc: *mut link_encoder);
}
extern "C" {
    pub fn dcn10_is_dig_enabled(enc: *mut link_encoder) -> bool;
}
extern "C" {
    pub fn dcn10_get_dig_frontend(enc: *mut link_encoder) -> c_uint;
}
extern "C" {
    pub fn dcn10_aux_initialize(enc10: *mut dcn10_link_encoder);
}
extern "C" {
    pub fn dcn10_get_hpd_state(enc: *mut link_encoder) -> bool;
}
extern "C" {
    pub fn dcn10_program_hpd_filter(enc: *mut link_encoder, delay_on_connect_in_ms: c_int, delay_on_disconnect_in_ms: c_int) -> bool;
}
