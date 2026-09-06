//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/dccg.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phyd32clk_clock_source {
    PHYD32CLKA,
    PHYD32CLKB,
    PHYD32CLKC,
    PHYD32CLKD,
    PHYD32CLKE,
    PHYD32CLKF,
    PHYD32CLKG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum physymclk_clock_source {
    PHYSYMCLK_FORCE_SRC_SYMCLK,    // Select symclk as source of clock which is output to PHY through DCIO.
    PHYSYMCLK_FORCE_SRC_PHYD18CLK, // Select phyd18clk as the source of clock which is output to PHY through DCIO.
    PHYSYMCLK_FORCE_SRC_PHYD32CLK, // Select phyd32clk as the source of clock which is output to PHY through DCIO.
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum streamclk_source {
    REFCLK,                   // Selects REFCLK as source for hdmistreamclk.
    DTBCLK0,                  // Selects DTBCLK0 as source for hdmistreamclk.
    DPREFCLK,                 // Selects DPREFCLK as source for hdmistreamclk
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dentist_dispclk_change_mode {
    DISPCLK_CHANGE_MODE_IMMEDIATE,
    DISPCLK_CHANGE_MODE_RAMPING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_dto_params {
    pub otg_inst: c_int,
    pub signal: signal_type,
    pub clk_src: streamclk_source,
    pub pixclk_hz: u64,
    pub refclk_hz: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pixel_rate_div {
    PIXEL_RATE_DIV_BY_1 = 0,
    PIXEL_RATE_DIV_BY_2 = 1,
    PIXEL_RATE_DIV_BY_4 = 3,
    PIXEL_RATE_DIV_NA = 0xF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dccg_reg_state {
    pub dc_mem_global_pwr_req_cntl: u32,
    pub dccg_audio_dtbclk_dto_modulo: u32,
    pub dccg_audio_dtbclk_dto_phase: u32,
    pub dccg_audio_dto_source: u32,
    pub dccg_audio_dto0_module: u32,
    pub dccg_audio_dto0_phase: u32,
    pub dccg_audio_dto1_module: u32,
    pub dccg_audio_dto1_phase: u32,
    pub dccg_cac_status: u32,
    pub dccg_cac_status2: u32,
    pub dccg_disp_cntl_reg: u32,
    pub dccg_ds_cntl: u32,
    pub dccg_ds_dto_incr: u32,
    pub dccg_ds_dto_modulo: u32,
    pub dccg_ds_hw_cal_interval: u32,
    pub dccg_gate_disable_cntl: u32,
    pub dccg_gate_disable_cntl2: u32,
    pub dccg_gate_disable_cntl3: u32,
    pub dccg_gate_disable_cntl4: u32,
    pub dccg_gate_disable_cntl5: u32,
    pub dccg_gate_disable_cntl6: u32,
    pub dccg_global_fgcg_rep_cntl: u32,
    pub dccg_gtc_cntl: u32,
    pub dccg_gtc_current: u32,
    pub dccg_gtc_dto_incr: u32,
    pub dccg_gtc_dto_modulo: u32,
    pub dccg_perfmon_cntl: u32,
    pub dccg_perfmon_cntl2: u32,
    pub dccg_soft_reset: u32,
    pub dccg_test_clk_sel: u32,
    pub dccg_vsync_cnt_ctrl: u32,
    pub dccg_vsync_cnt_int_ctrl: u32,
    pub dccg_vsync_otg0_latch_value: u32,
    pub dccg_vsync_otg1_latch_value: u32,
    pub dccg_vsync_otg2_latch_value: u32,
    pub dccg_vsync_otg3_latch_value: u32,
    pub dccg_vsync_otg4_latch_value: u32,
    pub dccg_vsync_otg5_latch_value: u32,
    pub dispclk_cgtt_blk_ctrl_reg: u32,
    pub dispclk_freq_change_cntl: u32,
    pub dp_dto_dbuf_en: u32,
    pub dp_dto0_modulo: u32,
    pub dp_dto0_phase: u32,
    pub dp_dto1_modulo: u32,
    pub dp_dto1_phase: u32,
    pub dp_dto2_modulo: u32,
    pub dp_dto2_phase: u32,
    pub dp_dto3_modulo: u32,
    pub dp_dto3_phase: u32,
    pub dpiaclk_540m_dto_modulo: u32,
    pub dpiaclk_540m_dto_phase: u32,
    pub dpiaclk_810m_dto_modulo: u32,
    pub dpiaclk_810m_dto_phase: u32,
    pub dpiaclk_dto_cntl: u32,
    pub dpiasymclk_cntl: u32,
    pub dppclk_cgtt_blk_ctrl_reg: u32,
    pub dppclk_ctrl: u32,
    pub dppclk_dto_ctrl: u32,
    pub dppclk0_dto_param: u32,
    pub dppclk1_dto_param: u32,
    pub dppclk2_dto_param: u32,
    pub dppclk3_dto_param: u32,
    pub dprefclk_cgtt_blk_ctrl_reg: u32,
    pub dprefclk_cntl: u32,
    pub dpstreamclk_cntl: u32,
    pub dscclk_dto_ctrl: u32,
    pub dscclk0_dto_param: u32,
    pub dscclk1_dto_param: u32,
    pub dscclk2_dto_param: u32,
    pub dscclk3_dto_param: u32,
    pub dtbclk_dto_dbuf_en: u32,
    pub dtbclk_dto0_modulo: u32,
    pub dtbclk_dto0_phase: u32,
    pub dtbclk_dto1_modulo: u32,
    pub dtbclk_dto1_phase: u32,
    pub dtbclk_dto2_modulo: u32,
    pub dtbclk_dto2_phase: u32,
    pub dtbclk_dto3_modulo: u32,
    pub dtbclk_dto3_phase: u32,
    pub dtbclk_p_cntl: u32,
    pub force_symclk_disable: u32,
    pub hdmicharclk0_clock_cntl: u32,
    pub hdmistreamclk_cntl: u32,
    pub hdmistreamclk0_dto_param: u32,
    pub microsecond_time_base_div: u32,
    pub millisecond_time_base_div: u32,
    pub otg_pixel_rate_div: u32,
    pub otg0_phypll_pixel_rate_cntl: u32,
    pub otg0_pixel_rate_cntl: u32,
    pub otg1_phypll_pixel_rate_cntl: u32,
    pub otg1_pixel_rate_cntl: u32,
    pub otg2_phypll_pixel_rate_cntl: u32,
    pub otg2_pixel_rate_cntl: u32,
    pub otg3_phypll_pixel_rate_cntl: u32,
    pub otg3_pixel_rate_cntl: u32,
    pub phyasymclk_clock_cntl: u32,
    pub phybsymclk_clock_cntl: u32,
    pub phycsymclk_clock_cntl: u32,
    pub phydsymclk_clock_cntl: u32,
    pub phyesymclk_clock_cntl: u32,
    pub phyplla_pixclk_resync_cntl: u32,
    pub phypllb_pixclk_resync_cntl: u32,
    pub phypllc_pixclk_resync_cntl: u32,
    pub phyplld_pixclk_resync_cntl: u32,
    pub phyplle_pixclk_resync_cntl: u32,
    pub refclk_cgtt_blk_ctrl_reg: u32,
    pub socclk_cgtt_blk_ctrl_reg: u32,
    pub symclk_cgtt_blk_ctrl_reg: u32,
    pub symclk_psp_cntl: u32,
    pub symclk32_le_cntl: u32,
    pub symclk32_se_cntl: u32,
    pub symclka_clock_enable: u32,
    pub symclkb_clock_enable: u32,
    pub symclkc_clock_enable: u32,
    pub symclkd_clock_enable: u32,
    pub symclke_clock_enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg {
    pub ctx: *mut dc_context,
    pub funcs: *const dccg_funcs,
    pub pipe_dppclk_khz: [c_int; MAX_PIPES],
    pub ref_dppclk: c_int,
    pub dpp_clock_gated: [bool; MAX_PIPES],
// int dtbclk_khz[MAX_PIPES];/* TODO needs to be removed
// int audio_dtbclk_khz;/* TODO needs to be removed
// int ref_dtbclk_khz;/* TODO needs to be removed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtbclk_dto_params {
    pub timing: *const dc_crtc_timing,
    pub otg_inst: c_int,
    pub pixclk_khz: c_int,
    pub req_audio_dtbclk_khz: c_int,
    pub num_odm_segments: c_int,
    pub ref_dtbclk_khz: c_int,
    pub is_hdmi: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_funcs {
    pub req_dppclk): c_int,
    pub dccg_ref_freq_inKhz): *mut c_uint,
    pub en): bool,
    pub otg_inst): u32,
    pub otg_inst): u32,
    pub dccg): *mut *mut void (dccg_init)(struct dccg,
    pub /: *mut *mut *mut *mut void (refclk_setup)(struct dccg dccg); / Deprecated - for backward compatibility only,
    pub allow): *mut *mut *mut void (allow_clock_gating)(struct dccg dccg, bool,
    pub enable): *mut *mut *mut void (enable_memory_low_power)(struct dccg dccg, bool,
    pub dccg): *mut *mut bool (is_s0i3_golden_init_wa_done)(struct dccg,
    pub phypll_inst): *mut *mut *mut void (enable_hdmicharclk)(struct dccg dccg, int hpo_inst, int,
    pub hpo_inst): *mut *mut *mut void (disable_hdmicharclk)(struct dccg dccg, int,
    pub otg_inst): u32,
    pub enable): bool,
    pub enable): bool,
    pub dp_hpo_inst): c_int,
    pub phyd32clk): phyd32clk_clock_source,
    pub hpo_se_inst): c_int,
    pub phyd32clk): phyd32clk_clock_source,
    pub hpo_le_inst): c_int,
    pub enable): bool,
    pub force_enable): bool,
    pub enable): bool,
    pub params): *const dtbclk_dto_params,
    pub params): *const dtbclk_dto_params,
    pub change_mode): dentist_dispclk_change_mode,
    pub inst): c_int,
    pub inst): c_int,
    pub k2): pixel_rate_div,
    pub div_factor2): *mut u32,
    pub pixclk_khz): c_int,
    pub dccg): *mut dccg,
    pub clock_on): bool,
    pub link_enc_inst): u32,
    pub link_enc_inst): u32,
    pub params): *const dp_dto_params,
    pub otg_inst): u32,
    pub num_slices_h): *mut *mut *mut void (set_dto_dscclk)(struct dccg dccg, uint32_t dsc_inst, uint32_t,
    pub dsc_inst): *mut *mut *mut void (set_ref_dscclk)(struct dccg dccg, uint32_t,
    pub disable_clock_gating): *mut *mut *mut void (dccg_root_gate_disable_control)(struct dccg dccg, uint32_t pipe_idx, uint32_t,
    pub dccg_reg_state): *mut *mut *mut void (dccg_read_reg_state)(struct dccg dccg, struct dcn_dccg_reg_state,
    pub enable): *mut *mut *mut void (dccg_enable_global_fgcg)(struct dccg dccg, bool,
    pub dccg): *mut *mut bool (dccg_get_global_fgcg_status)(struct dccg,
}
