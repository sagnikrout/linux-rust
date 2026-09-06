//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r420d.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const R_0001F8_MC_IND_INDEX: c_uint = 0x0001F8;

pub const C_0001F8_MC_IND_ADDR: c_uint = 0xFFFFFF80;

pub const C_0001F8_MC_IND_WR_EN: c_uint = 0xFFFFFEFF;
pub const R_0001FC_MC_IND_DATA: c_uint = 0x0001FC;

pub const C_0001FC_MC_IND_DATA: c_uint = 0x00000000;
pub const R_0007C0_CP_STAT: c_uint = 0x0007C0;

pub const C_0007C0_MRU_BUSY: c_uint = 0xFFFFFFFE;

pub const C_0007C0_MWU_BUSY: c_uint = 0xFFFFFFFD;

pub const C_0007C0_RSIU_BUSY: c_uint = 0xFFFFFFFB;

pub const C_0007C0_RCIU_BUSY: c_uint = 0xFFFFFFF7;

pub const C_0007C0_CSF_PRIMARY_BUSY: c_uint = 0xFFFFFDFF;

pub const C_0007C0_CSF_INDIRECT_BUSY: c_uint = 0xFFFFFBFF;

pub const C_0007C0_CSQ_PRIMARY_BUSY: c_uint = 0xFFFFF7FF;

pub const C_0007C0_CSQ_INDIRECT_BUSY: c_uint = 0xFFFFEFFF;

pub const C_0007C0_CSI_BUSY: c_uint = 0xFFFFDFFF;

pub const C_0007C0_CSF_INDIRECT2_BUSY: c_uint = 0xFFFFBFFF;

pub const C_0007C0_CSQ_INDIRECT2_BUSY: c_uint = 0xFFFF7FFF;

pub const C_0007C0_GUIDMA_BUSY: c_uint = 0xEFFFFFFF;

pub const C_0007C0_VIDDMA_BUSY: c_uint = 0xDFFFFFFF;

pub const C_0007C0_CMDSTRM_BUSY: c_uint = 0xBFFFFFFF;

pub const C_0007C0_CP_BUSY: c_uint = 0x7FFFFFFF;
pub const R_000E40_RBBM_STATUS: c_uint = 0x000E40;

pub const C_000E40_CMDFIFO_AVAIL: c_uint = 0xFFFFFF80;

pub const C_000E40_HIRQ_ON_RBB: c_uint = 0xFFFFFEFF;

pub const C_000E40_CPRQ_ON_RBB: c_uint = 0xFFFFFDFF;

pub const C_000E40_CFRQ_ON_RBB: c_uint = 0xFFFFFBFF;

pub const C_000E40_HIRQ_IN_RTBUF: c_uint = 0xFFFFF7FF;

pub const C_000E40_CPRQ_IN_RTBUF: c_uint = 0xFFFFEFFF;

pub const C_000E40_CFRQ_IN_RTBUF: c_uint = 0xFFFFDFFF;

pub const C_000E40_CF_PIPE_BUSY: c_uint = 0xFFFFBFFF;

pub const C_000E40_ENG_EV_BUSY: c_uint = 0xFFFF7FFF;

pub const C_000E40_CP_CMDSTRM_BUSY: c_uint = 0xFFFEFFFF;

pub const C_000E40_E2_BUSY: c_uint = 0xFFFDFFFF;

pub const C_000E40_RB2D_BUSY: c_uint = 0xFFFBFFFF;

pub const C_000E40_RB3D_BUSY: c_uint = 0xFFF7FFFF;

pub const C_000E40_VAP_BUSY: c_uint = 0xFFEFFFFF;

pub const C_000E40_RE_BUSY: c_uint = 0xFFDFFFFF;

pub const C_000E40_TAM_BUSY: c_uint = 0xFFBFFFFF;

pub const C_000E40_TDM_BUSY: c_uint = 0xFF7FFFFF;

pub const C_000E40_PB_BUSY: c_uint = 0xFEFFFFFF;

pub const C_000E40_TIM_BUSY: c_uint = 0xFDFFFFFF;

pub const C_000E40_GA_BUSY: c_uint = 0xFBFFFFFF;

pub const C_000E40_CBA2D_BUSY: c_uint = 0xF7FFFFFF;

pub const C_000E40_GUI_ACTIVE: c_uint = 0x7FFFFFFF;
// CLK registers
pub const R_00000D_SCLK_CNTL: c_uint = 0x00000D;

pub const C_00000D_SCLK_SRC_SEL: c_uint = 0xFFFFFFF8;

pub const C_00000D_CP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFF7;

pub const C_00000D_HDP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFEF;

pub const C_00000D_TV_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFDF;

pub const C_00000D_E2_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFBF;

pub const C_00000D_SE_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFF7F;

pub const C_00000D_IDCT_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFEFF;

pub const C_00000D_VIP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFDFF;

pub const C_00000D_RE_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFBFF;

pub const C_00000D_PB_MAX_DYN_STOP_LAT: c_uint = 0xFFFFF7FF;

pub const C_00000D_TAM_MAX_DYN_STOP_LAT: c_uint = 0xFFFFEFFF;

pub const C_00000D_TDM_MAX_DYN_STOP_LAT: c_uint = 0xFFFFDFFF;

pub const C_00000D_RB_MAX_DYN_STOP_LAT: c_uint = 0xFFFFBFFF;

pub const C_00000D_FORCE_DISP2: c_uint = 0xFFFF7FFF;

pub const C_00000D_FORCE_CP: c_uint = 0xFFFEFFFF;

pub const C_00000D_FORCE_HDP: c_uint = 0xFFFDFFFF;

pub const C_00000D_FORCE_DISP1: c_uint = 0xFFFBFFFF;

pub const C_00000D_FORCE_TOP: c_uint = 0xFFF7FFFF;

pub const C_00000D_FORCE_E2: c_uint = 0xFFEFFFFF;

pub const C_00000D_FORCE_VAP: c_uint = 0xFFDFFFFF;

pub const C_00000D_FORCE_IDCT: c_uint = 0xFFBFFFFF;

pub const C_00000D_FORCE_VIP: c_uint = 0xFF7FFFFF;

pub const C_00000D_FORCE_RE: c_uint = 0xFEFFFFFF;

pub const C_00000D_FORCE_SR: c_uint = 0xFDFFFFFF;

pub const C_00000D_FORCE_PX: c_uint = 0xFBFFFFFF;

pub const C_00000D_FORCE_TX: c_uint = 0xF7FFFFFF;

pub const C_00000D_FORCE_US: c_uint = 0xEFFFFFFF;

pub const C_00000D_FORCE_TV_SCLK: c_uint = 0xDFFFFFFF;

pub const C_00000D_FORCE_SU: c_uint = 0xBFFFFFFF;

pub const C_00000D_FORCE_OV0: c_uint = 0x7FFFFFFF;
