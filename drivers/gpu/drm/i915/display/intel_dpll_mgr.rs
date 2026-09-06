//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dpll_mgr.h
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
// Copyright © 2012-2016 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// enum intel_dpll_id - possible DPLL ids
//
// Enumeration of possible IDs for a DPLL. Real shared dpll ids must be >= 0.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_dpll_id {
//
// @DPLL_ID_PRIVATE: non-shared dpll in use
//
    DPLL_ID_PRIVATE = -1,

//
// @DPLL_ID_PCH_PLL_A: DPLL A in ILK, SNB and IVB
//
    DPLL_ID_PCH_PLL_A = 0,
//
// @DPLL_ID_PCH_PLL_B: DPLL B in ILK, SNB and IVB
//
    DPLL_ID_PCH_PLL_B = 1,


//
// @DPLL_ID_WRPLL1: HSW and BDW WRPLL1
//
    DPLL_ID_WRPLL1 = 0,
//
// @DPLL_ID_WRPLL2: HSW and BDW WRPLL2
//
    DPLL_ID_WRPLL2 = 1,
//
// @DPLL_ID_SPLL: HSW and BDW SPLL
//
    DPLL_ID_SPLL = 2,
//
// @DPLL_ID_LCPLL_810: HSW and BDW 0.81 GHz LCPLL
//
    DPLL_ID_LCPLL_810 = 3,
//
// @DPLL_ID_LCPLL_1350: HSW and BDW 1.35 GHz LCPLL
//
    DPLL_ID_LCPLL_1350 = 4,
//
// @DPLL_ID_LCPLL_2700: HSW and BDW 2.7 GHz LCPLL
//
    DPLL_ID_LCPLL_2700 = 5,


//
// @DPLL_ID_SKL_DPLL0: SKL and later DPLL0
//
    DPLL_ID_SKL_DPLL0 = 0,
//
// @DPLL_ID_SKL_DPLL1: SKL and later DPLL1
//
    DPLL_ID_SKL_DPLL1 = 1,
//
// @DPLL_ID_SKL_DPLL2: SKL and later DPLL2
//
    DPLL_ID_SKL_DPLL2 = 2,
//
// @DPLL_ID_SKL_DPLL3: SKL and later DPLL3
//
    DPLL_ID_SKL_DPLL3 = 3,


//
// @DPLL_ID_ICL_DPLL0: ICL/TGL combo PHY DPLL0
//
    DPLL_ID_ICL_DPLL0 = 0,
//
// @DPLL_ID_ICL_DPLL1: ICL/TGL combo PHY DPLL1
//
    DPLL_ID_ICL_DPLL1 = 1,
//
// @DPLL_ID_EHL_DPLL4: EHL combo PHY DPLL4
//
    DPLL_ID_EHL_DPLL4 = 2,
//
// @DPLL_ID_ICL_TBTPLL: ICL/TGL TBT PLL
//
    DPLL_ID_ICL_TBTPLL = 2,
//
// @DPLL_ID_ICL_MGPLL1: ICL MG PLL 1 port 1 (C),
// TGL TC PLL 1 port 1 (TC1)
//
    DPLL_ID_ICL_MGPLL1 = 3,
//
// @DPLL_ID_ICL_MGPLL2: ICL MG PLL 1 port 2 (D)
// TGL TC PLL 1 port 2 (TC2)
//
    DPLL_ID_ICL_MGPLL2 = 4,
//
// @DPLL_ID_ICL_MGPLL3: ICL MG PLL 1 port 3 (E)
// TGL TC PLL 1 port 3 (TC3)
//
    DPLL_ID_ICL_MGPLL3 = 5,
//
// @DPLL_ID_ICL_MGPLL4: ICL MG PLL 1 port 4 (F)
// TGL TC PLL 1 port 4 (TC4)
//
    DPLL_ID_ICL_MGPLL4 = 6,
//
// @DPLL_ID_TGL_MGPLL5: TGL TC PLL port 5 (TC5)
//
    DPLL_ID_TGL_MGPLL5 = 7,
//
// @DPLL_ID_TGL_MGPLL6: TGL TC PLL port 6 (TC6)
//
    DPLL_ID_TGL_MGPLL6 = 8,

//
// @DPLL_ID_DG1_DPLL0: DG1 combo PHY DPLL0
//
    DPLL_ID_DG1_DPLL0 = 0,
//
// @DPLL_ID_DG1_DPLL1: DG1 combo PHY DPLL1
//
    DPLL_ID_DG1_DPLL1 = 1,
//
// @DPLL_ID_DG1_DPLL2: DG1 combo PHY DPLL2
//
    DPLL_ID_DG1_DPLL2 = 2,
//
// @DPLL_ID_DG1_DPLL3: DG1 combo PHY DPLL3
//
    DPLL_ID_DG1_DPLL3 = 3,
}

pub const I915_NUM_PLLS: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icl_port_dpll_id {
    ICL_PORT_DPLL_DEFAULT,
    ICL_PORT_DPLL_MG_PHY,

    ICL_PORT_DPLL_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i9xx_dpll_hw_state {
    pub dpll: u32,
    pub dpll_md: u32,
    pub fp0: u32,
    pub fp1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsw_dpll_hw_state {
    pub wrpll: u32,
    pub spll: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skl_dpll_hw_state {
//
// DPLL_CTRL1 has 6 bits for each each this DPLL. We store those in
// lower part of ctrl1 and they get shifted into position when writing
// the register.  This allows us to easily compare the state to share
// the DPLL.
//
    pub ctrl1: u32,
// HDMI only, 0 when used for DP
    pub cfgcr2: u32 cfgcr1,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bxt_dpll_hw_state {
    pub pcsdw12: u32 ebb0, ebb4, pll0, pll1, pll2, pll3, pll6, pll8, pll9, pll10,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icl_dpll_hw_state {
    pub cfgcr1: u32 cfgcr0,,
// tgl
    pub div0: u32,
    pub mg_refclkin_ctl: u32,
    pub mg_clktop2_coreclkctl1: u32,
    pub mg_clktop2_hsclkctl: u32,
    pub mg_pll_div0: u32,
    pub mg_pll_div1: u32,
    pub mg_pll_lf: u32,
    pub mg_pll_frac_lock: u32,
    pub mg_pll_ssc: u32,
    pub mg_pll_bias: u32,
    pub mg_pll_tdc_coldst_bias: u32,
    pub mg_pll_bias_mask: u32,
    pub mg_pll_tdc_coldst_bias_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_mpllb_state {
    pub /: *mut *mut u32 clock; / in KHz,
    pub ref_control: u32,
    pub mpllb_cp: u32,
    pub mpllb_div: u32,
    pub mpllb_div2: u32,
    pub mpllb_fracn1: u32,
    pub mpllb_fracn2: u32,
    pub mpllb_sscen: u32,
    pub mpllb_sscstep: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_c10pll_state {
    pub tx: u8,
    pub cmn: u8,
    pub pll: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_c20pll_state {
    pub tx: [u16; 3],
    pub cmn: [u16; 4],
    pub mplla: [u16; 10],
    pub mpllb: [u16; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_c20pll_vdr_state {
    pub custom_width: u8,
    pub serdes_rate: u8,
    pub hdmi_rate: u8,
    pub vdr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_cx0pll_state {
    pub c10: intel_c10pll_state,
    pub c20: intel_c20pll_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_lt_phy_pll_state {
    pub addr_msb: [u8; 13],
    pub addr_lsb: [u8; 13],
    pub data: [u8; 13][4],
    pub config: [u8; 3],
    pub ssc_enabled: bool,
    pub tbt_mode: bool,
    pub lane_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dpll_hw_state {
    pub i9xx: i9xx_dpll_hw_state,
    pub hsw: hsw_dpll_hw_state,
    pub skl: skl_dpll_hw_state,
    pub bxt: bxt_dpll_hw_state,
    pub icl: icl_dpll_hw_state,
    pub mpllb: intel_mpllb_state,
    pub cx0pll: intel_cx0pll_state,
    pub ltpll: intel_lt_phy_pll_state,
}

//
// struct intel_dpll_state - hold the DPLL atomic state
//
// This structure holds an atomic state for the DPLL, that can represent
// either its current state (in struct &intel_shared_dpll) or a desired
// future state which would be applied by an atomic mode set (stored in
// a struct &intel_atomic_state).
//
// See also intel_reserve_shared_dplls() and intel_release_shared_dplls().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dpll_state {
//
// @pipe_mask: mask of pipes using this DPLL, active or not
//
    pub pipe_mask: u8,
//
// @hw_state: hardware configuration for the DPLL stored in
// struct &intel_dpll_hw_state.
//
    pub hw_state: intel_dpll_hw_state,
}

//
// struct dpll_info - display PLL platform specific info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_info {
//
// @name: DPLL name; used for logging
//
    pub name: *const c_char,
//
// @funcs: platform specific hooks
//
    pub funcs: *const intel_dpll_funcs,
//
// @id: unique identifier for this DPLL
//
    pub id: intel_dpll_id,
//
// @power_domain: extra power domain required by the DPLL
//
    pub power_domain: intel_display_power_domain,
//
// @always_on:
//
// Inform the state checker that the DPLL is kept enabled even if
// not in use by any CRTC.
//
    pub always_on: bool,
//
// @is_alt_port_dpll:
//
// Inform the state checker that the DPLL can be used as a fallback
// (for TC->TBT fallback).
//
    pub is_alt_port_dpll: bool,
}

//
// struct intel_dpll - display PLL with tracked state and users
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dpll {
//
// @state:
//
// Store the state for the pll, including its hw state
// and CRTCs using it.
//
    pub state: intel_dpll_state,
//
// @index: index for atomic state
//
    pub index: u8,
//
// @active_mask: mask of active pipes (i.e. DPMS on) using this DPLL
//
    pub active_mask: u8,
//
// @on: is the PLL actually active? Disabled during modeset
//
    pub on: bool,
//
// @info: platform specific info
//
    pub info: *const dpll_info,
//
// @wakeref: In some platforms a device-level runtime pm reference may
// need to be grabbed to disable DC states while this DPLL is enabled
//
    pub wakeref: *mut ref_tracker,
}

pub const SKL_DPLL0: c_int = 0;
pub const SKL_DPLL1: c_int = 1;
pub const SKL_DPLL2: c_int = 2;
pub const SKL_DPLL3: c_int = 3;
// dpll functions

extern "C" {
    pub fn intel_dpll_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dpll_disable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dpll_swap_state(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dpll_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dpll_update_ref_clks(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dpll_readout_hw_state(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dpll_sanitize_state(display: *mut intel_display);
}
extern "C" {
    pub fn icl_tc_port_to_pll_id(tc_port: tc_port) -> intel_dpll_id;
}
extern "C" {
    pub fn mtl_port_to_pll_id(display: *mut intel_display, port: port) -> intel_dpll_id;
}
extern "C" {
    pub fn intel_dpll_is_combophy(id: intel_dpll_id) -> bool;
}
extern "C" {
    pub fn intel_dpll_verify_disabled(state: *mut intel_atomic_state);
}
