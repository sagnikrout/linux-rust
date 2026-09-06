//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dio/dcn20/dcn20_link_encoder.h
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

// Macro flag: #define DCN2_AUX_REG_LIST(id)\
// Macro flag: #define UNIPHY_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DPCS_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DPCS_DCN2_MASK_SH_LIST(mask_sh)\
// Macro flag: #define LINK_ENCODER_MASK_SH_LIST_DCN20(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpll_cfg {
    pub mpllb_ana_v2i: u32,
    pub mpllb_ana_freq_vco: u32,
    pub mpllb_ana_cp_int: u32,
    pub mpllb_ana_cp_prop: u32,
    pub mpllb_multiplier: u32,
    pub ref_clk_mpllb_div: u32,
    pub mpllb_word_div2_en: bool,
    pub mpllb_ssc_en: bool,
    pub mpllb_div5_clk_en: bool,
    pub mpllb_div_clk_en: bool,
    pub mpllb_fracn_en: bool,
    pub mpllb_pmix_en: bool,
    pub mpllb_div_multiplier: u32,
    pub mpllb_tx_clk_div: u32,
    pub mpllb_fracn_quot: u32,
    pub mpllb_fracn_den: u32,
    pub mpllb_ssc_peak: u32,
    pub mpllb_ssc_stepsize: u32,
    pub mpllb_ssc_up_spread: u32,
    pub mpllb_fracn_rem: u32,
    pub mpllb_hdmi_div: u32,
// TODO: May not mpll params, need to figure out.
    pub tx_vboost_lvl: u32,
    pub hdmi_pixel_clk_div: u32,
    pub ref_range: u32,
    pub ref_clk: u32,
    pub hdmimode_enable: bool,
    pub sup_pre_hp: bool,
    pub dp_tx0_vergdrv_byp: bool,
    pub dp_tx1_vergdrv_byp: bool,
    pub dp_tx2_vergdrv_byp: bool,
    pub dp_tx3_vergdrv_byp: bool,
    pub tx_peaking_lvl: u32,
    pub ctr_reqs_pll: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcssys_phy_seq_cfg {
    pub program_fuse: bool,
    pub bypass_sram: bool,
    pub lane_en: [bool; 4],
    pub use_calibration_setting: bool,
    pub mpll_cfg: mpll_cfg,
    pub load_sram_fw: bool,
    pub tx_hdmi_frl_mode: bool,

    pub hdmimode_enable: bool,
    pub silver2: bool,
    pub ext_refclk_en: bool,
    pub dp_tx0_term_ctrl: u32,
    pub dp_tx1_term_ctrl: u32,
    pub dp_tx2_term_ctrl: u32,
    pub dp_tx3_term_ctrl: u32,
    pub fw_data: [u32; 0x1000],
    pub dp_tx0_width: u32,
    pub dp_tx1_width: u32,
    pub dp_tx2_width: u32,
    pub dp_tx3_width: u32,
    pub dp_tx0_rate: u32,
    pub dp_tx1_rate: u32,
    pub dp_tx2_rate: u32,
    pub dp_tx3_rate: u32,
    pub dp_tx0_eq_main: u32,
    pub dp_tx0_eq_pre: u32,
    pub dp_tx0_eq_post: u32,
    pub dp_tx1_eq_main: u32,
    pub dp_tx1_eq_pre: u32,
    pub dp_tx1_eq_post: u32,
    pub dp_tx2_eq_main: u32,
    pub dp_tx2_eq_pre: u32,
    pub dp_tx2_eq_post: u32,
    pub dp_tx3_eq_main: u32,
    pub dp_tx3_eq_pre: u32,
    pub dp_tx3_eq_post: u32,
    pub data_swap_en: bool,
    pub data_order_invert_en: bool,
    pub ldpcs_fifo_start_delay: u32,
    pub rdpcs_fifo_start_delay: u32,
    pub rdpcs_reg_fifo_error_mask: bool,
    pub rdpcs_tx_fifo_error_mask: bool,
    pub rdpcs_dpalt_disable_mask: bool,
    pub rdpcs_dpalt_4lane_mask: bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_link_encoder {
    pub enc10: dcn10_link_encoder,
    pub phy_seq_cfg: dpcssys_phy_seq_cfg,
}

extern "C" {
    pub fn enc2_fec_set_enable(enc: *mut link_encoder, enable: bool);
}
extern "C" {
    pub fn enc2_fec_set_ready(enc: *mut link_encoder, ready: bool);
}
extern "C" {
    pub fn enc2_fec_is_active(enc: *mut link_encoder) -> bool;
}
extern "C" {
    pub fn enc2_hw_init(enc: *mut link_encoder);
}
extern "C" {
    pub fn link_enc2_read_state(enc: *mut link_encoder, s: *mut link_enc_state);
}
extern "C" {
    pub fn dcn20_link_encoder_is_in_alt_mode(enc: *mut link_encoder) -> bool;
}
