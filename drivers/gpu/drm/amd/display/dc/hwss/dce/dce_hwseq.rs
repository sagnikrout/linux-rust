//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/dce/dce_hwseq.h
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
// Copyright 2016, 2026 Advanced Micro Devices, Inc.
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

// Macro flag: #define HWSEQ_DCN_REG_LIST()\
// Macro flag: #define MMHUB_DCN_REG_LIST()\
// todo:  get these from GVM instead of reading registers ourselves */\
// Macro flag: #define HWSEQ_DCN1_REG_LIST()\
// Macro flag: #define HWSEQ_DCN2_REG_LIST()\
// SR(DOMAIN10_PG_CONFIG), Navi1x HUBP5 not powergate-able*/\
// SR(DOMAIN11_PG_CONFIG), Navi1x DPP5 is not powergate-able */\
// Macro flag: #define HWSEQ_DCN21_REG_LIST()\
// Macro flag: #define HWSEQ_DCN201_REG_LIST()\
// Macro flag: #define HWSEQ_DCN30_REG_LIST()\
// Macro flag: #define HWSEQ_DCN301_REG_LIST()\
// Macro flag: #define HWSEQ_DCN302_REG_LIST()\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_hwseq_registers {
    pub DCFE_CLOCK_CONTROL: [u32; 6],
    pub DCFEV_CLOCK_CONTROL: u32,
    pub DC_MEM_GLOBAL_PWR_REQ_CNTL: u32,
    pub BLND_V_UPDATE_LOCK: [u32; 6],
    pub BLND_CONTROL: [u32; 6],
    pub BLNDV_CONTROL: u32,
    pub CRTC_H_BLANK_START_END: [u32; 6],
    pub PIXEL_RATE_CNTL: [u32; 6],
    pub PHYPLL_PIXEL_RATE_CNTL: [u32; 6],
// DCHUB
    pub DCHUB_FB_LOCATION: u32,
    pub DCHUB_AGP_BASE: u32,
    pub DCHUB_AGP_BOT: u32,
    pub DCHUB_AGP_TOP: u32,
    pub REFCLK_CNTL: u32,
    pub DCHUBBUB_GLOBAL_TIMER_CNTL: u32,
    pub DCHUBBUB_SDPIF_FB_BASE: u32,
    pub DCHUBBUB_SDPIF_FB_OFFSET: u32,
    pub DCHUBBUB_SDPIF_AGP_BASE: u32,
    pub DCHUBBUB_SDPIF_AGP_BOT: u32,
    pub DCHUBBUB_SDPIF_AGP_TOP: u32,
    pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32,
    pub DOMAIN1_PG_CONFIG: u32,
    pub DOMAIN2_PG_CONFIG: u32,
    pub DOMAIN3_PG_CONFIG: u32,
    pub DOMAIN4_PG_CONFIG: u32,
    pub DOMAIN5_PG_CONFIG: u32,
    pub DOMAIN6_PG_CONFIG: u32,
    pub DOMAIN7_PG_CONFIG: u32,
    pub DOMAIN8_PG_CONFIG: u32,
    pub DOMAIN9_PG_CONFIG: u32,
    pub DOMAIN10_PG_CONFIG: u32,
    pub DOMAIN11_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32,
    pub DOMAIN17_PG_CONFIG: u32,
    pub DOMAIN18_PG_CONFIG: u32,
    pub DOMAIN19_PG_CONFIG: u32,
    pub DOMAIN20_PG_CONFIG: u32,
    pub DOMAIN21_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32,
    pub DOMAIN1_PG_STATUS: u32,
    pub DOMAIN2_PG_STATUS: u32,
    pub DOMAIN3_PG_STATUS: u32,
    pub DOMAIN4_PG_STATUS: u32,
    pub DOMAIN5_PG_STATUS: u32,
    pub DOMAIN6_PG_STATUS: u32,
    pub DOMAIN7_PG_STATUS: u32,
    pub DOMAIN8_PG_STATUS: u32,
    pub DOMAIN9_PG_STATUS: u32,
    pub DOMAIN10_PG_STATUS: u32,
    pub DOMAIN11_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32,
    pub DOMAIN17_PG_STATUS: u32,
    pub DOMAIN18_PG_STATUS: u32,
    pub DOMAIN19_PG_STATUS: u32,
    pub DOMAIN20_PG_STATUS: u32,
    pub DOMAIN21_PG_STATUS: u32,
    pub DIO_MEM_PWR_CTRL: u32,
    pub DCCG_GATE_DISABLE_CNTL: u32,
    pub DCCG_GATE_DISABLE_CNTL2: u32,
    pub DCFCLK_CNTL: u32,
    pub MICROSECOND_TIME_BASE_DIV: u32,
    pub MILLISECOND_TIME_BASE_DIV: u32,
    pub DISPCLK_FREQ_CHANGE_CNTL: u32,
    pub RBBMIF_TIMEOUT_DIS: u32,
    pub RBBMIF_TIMEOUT_DIS_2: u32,
    pub DCHUBBUB_CRC_CTRL: u32,
    pub DPP_TOP0_DPP_CRC_CTRL: u32,
    pub DPP_TOP0_DPP_CRC_VAL_R_G: u32,
    pub DPP_TOP0_DPP_CRC_VAL_B_A: u32,
    pub DPP_TOP0_DPP_CRC_VAL_R: u32,
    pub DPP_TOP0_DPP_CRC_VAL_G: u32,
    pub DPP_TOP0_DPP_CRC_VAL_B: u32,
    pub DPP_TOP0_DPP_CRC_VAL_A: u32,
    pub MPC_CRC_CTRL: u32,
    pub MPC_CRC_RESULT_GB: u32,
    pub MPC_CRC_RESULT_C: u32,
    pub MPC_CRC_RESULT_AR: u32,
    pub MPC_CRC_RESULT_R: u32,
    pub MPC_CRC_RESULT_G: u32,
    pub MPC_CRC_RESULT_B: u32,
    pub MPC_CRC_RESULT_A: u32,
    pub D1VGA_CONTROL: u32,
    pub D2VGA_CONTROL: u32,
    pub D3VGA_CONTROL: u32,
    pub D4VGA_CONTROL: u32,
    pub D5VGA_CONTROL: u32,
    pub D6VGA_CONTROL: u32,
    pub VGA_TEST_CONTROL: u32,
// MMHUB registers. read only. temporary hack
    pub VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32: u32,
    pub VM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32: u32,
    pub VM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32: u32,
    pub MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB: u32,
    pub MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB: u32,
    pub MC_VM_SYSTEM_APERTURE_LOW_ADDR: u32,
    pub MC_VM_SYSTEM_APERTURE_HIGH_ADDR: u32,
    pub MC_VM_XGMI_LFB_CNTL: u32,
    pub AZALIA_AUDIO_DTO: u32,
    pub AZALIA_CONTROLLER_CLOCK_GATING: u32,
// MMHUB VM
    pub MC_VM_FB_LOCATION_BASE: u32,
    pub MC_VM_FB_LOCATION_TOP: u32,
    pub MC_VM_FB_OFFSET: u32,
    pub MMHUBBUB_MEM_PWR_CNTL: u32,
    pub HPO_TOP_CLOCK_CONTROL: u32,
    pub ODM_MEM_PWR_CTRL3: u32,
    pub DMU_MEM_PWR_CNTL: u32,
    pub DCHUBBUB_ARB_HOSTVM_CNTL: u32,
    pub HPO_TOP_HW_CONTROL: u32,
    pub DMU_CLK_CNTL: u32,
    pub DCCG_GATE_DISABLE_CNTL4: u32,
    pub DCCG_GATE_DISABLE_CNTL5: u32,
    pub DOMAIN22_PG_CONFIG: u32,
    pub DOMAIN23_PG_CONFIG: u32,
    pub DOMAIN24_PG_CONFIG: u32,
    pub DOMAIN25_PG_CONFIG: u32,
    pub DOMAIN22_PG_STATUS: u32,
    pub DOMAIN23_PG_STATUS: u32,
    pub DOMAIN24_PG_STATUS: u32,
    pub DOMAIN25_PG_STATUS: u32,
    pub DOMAIN26_PG_CONFIG: u32,
    pub DOMAIN26_PG_STATUS: u32,
    pub HDCP_INTERRUPT_DEST: u32,
}

// set field name

// Macro flag: #define HWSEQ_DCE6_MASK_SH_LIST(mask_sh)\

// Macro flag: #define HWSEQ_DCE8_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCE10_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCE11_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCE112_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_GFX9_DCHUB_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCE12_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_VG20_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN1_MASK_SH_LIST(mask_sh)\
// todo:  get these from GVM instead of reading registers ourselves */\
// Macro flag: #define HWSEQ_DCN2_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN21_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN201_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN30_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN301_MASK_SH_LIST(mask_sh)\
// Macro flag: #define HWSEQ_DCN302_MASK_SH_LIST(mask_sh)\

// todo:  get these from GVM instead of reading registers ourselves */\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_hwseq_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_hwseq_mask {
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blnd_mode {
    BLND_MODE_CURRENT_PIPE = 0,/* Data from current pipe only */
    BLND_MODE_OTHER_PIPE, /* Data from other pipe only */
    BLND_MODE_BLENDING,/* Alpha blending - blend 'current' and 'other' */
}

extern "C" {
    pub fn dce_use_lut(format: surface_pixel_format) -> bool;
}
