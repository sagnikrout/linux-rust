//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rs690d.h
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
// Registers
pub const R_00001E_K8_FB_LOCATION: c_uint = 0x00001E;
pub const R_00005F_MC_MISC_UMA_CNTL: c_uint = 0x00005F;

pub const R_000078_MC_INDEX: c_uint = 0x000078;

pub const C_000078_MC_IND_ADDR: c_uint = 0xFFFFFE00;

pub const C_000078_MC_IND_WR_EN: c_uint = 0xFFFFFDFF;
pub const R_00007C_MC_DATA: c_uint = 0x00007C;

pub const C_00007C_MC_DATA: c_uint = 0x00000000;
pub const R_0000F8_CONFIG_MEMSIZE: c_uint = 0x0000F8;

pub const C_0000F8_CONFIG_MEMSIZE: c_uint = 0x00000000;
pub const R_000134_HDP_FB_LOCATION: c_uint = 0x000134;

pub const C_000134_HDP_FB_START: c_uint = 0xFFFF0000;
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
pub const R_006520_DC_LB_MEMORY_SPLIT: c_uint = 0x006520;

pub const C_006520_DC_LB_MEMORY_SPLIT: c_uint = 0xFFFFFFFC;

pub const C_006520_DC_LB_MEMORY_SPLIT_MODE: c_uint = 0xFFFFFFFB;
pub const V_006520_DC_LB_MEMORY_SPLIT_D1HALF_D2HALF: c_int = 0;
pub const V_006520_DC_LB_MEMORY_SPLIT_D1_3Q_D2_1Q: c_int = 1;
pub const V_006520_DC_LB_MEMORY_SPLIT_D1_ONLY: c_int = 2;
pub const V_006520_DC_LB_MEMORY_SPLIT_D1_1Q_D2_3Q: c_int = 3;

pub const C_006520_DC_LB_DISP1_END_ADR: c_uint = 0xFFFF800F;
pub const R_006548_D1MODE_PRIORITY_A_CNT: c_uint = 0x006548;

pub const C_006548_D1MODE_PRIORITY_MARK_A: c_uint = 0xFFFF8000;

pub const C_006548_D1MODE_PRIORITY_A_OFF: c_uint = 0xFFFEFFFF;

pub const C_006548_D1MODE_PRIORITY_A_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006548_D1MODE_PRIORITY_A_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_00654C_D1MODE_PRIORITY_B_CNT: c_uint = 0x00654C;

pub const C_00654C_D1MODE_PRIORITY_MARK_B: c_uint = 0xFFFF8000;

pub const C_00654C_D1MODE_PRIORITY_B_OFF: c_uint = 0xFFFEFFFF;

pub const C_00654C_D1MODE_PRIORITY_B_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_00654C_D1MODE_PRIORITY_B_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_006C9C_DCP_CONTROL: c_uint = 0x006C9C;
pub const R_006D48_D2MODE_PRIORITY_A_CNT: c_uint = 0x006D48;

pub const C_006D48_D2MODE_PRIORITY_MARK_A: c_uint = 0xFFFF8000;

pub const C_006D48_D2MODE_PRIORITY_A_OFF: c_uint = 0xFFFEFFFF;

pub const C_006D48_D2MODE_PRIORITY_A_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006D48_D2MODE_PRIORITY_A_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_006D4C_D2MODE_PRIORITY_B_CNT: c_uint = 0x006D4C;

pub const C_006D4C_D2MODE_PRIORITY_MARK_B: c_uint = 0xFFFF8000;

pub const C_006D4C_D2MODE_PRIORITY_B_OFF: c_uint = 0xFFFEFFFF;

pub const C_006D4C_D2MODE_PRIORITY_B_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006D4C_D2MODE_PRIORITY_B_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_006D58_LB_MAX_REQ_OUTSTANDING: c_uint = 0x006D58;

pub const C_006D58_LB_D1_MAX_REQ_OUTSTANDING: c_uint = 0xFFFFFFF0;

pub const C_006D58_LB_D2_MAX_REQ_OUTSTANDING: c_uint = 0xFFF0FFFF;
pub const R_000090_MC_SYSTEM_STATUS: c_uint = 0x000090;

pub const C_000090_MC_SYSTEM_IDLE: c_uint = 0xFFFFFFFE;

pub const C_000090_MC_SEQUENCER_IDLE: c_uint = 0xFFFFFFFD;

pub const C_000090_MC_ARBITER_IDLE: c_uint = 0xFFFFFFFB;

pub const C_000090_MC_SELECT_PM: c_uint = 0xFFFFFFF7;

pub const C_000090_RESERVED4: c_uint = 0xFFFFFF0F;

pub const C_000090_RESERVED8: c_uint = 0xFFFFF0FF;

pub const C_000090_RESERVED12: c_uint = 0xFFFF0FFF;

pub const C_000090_MCA_INIT_EXECUTED: c_uint = 0xFFFEFFFF;

pub const C_000090_MCA_IDLE: c_uint = 0xFFFDFFFF;

pub const C_000090_MCA_SEQ_IDLE: c_uint = 0xFFFBFFFF;

pub const C_000090_MCA_ARB_IDLE: c_uint = 0xFFF7FFFF;

pub const C_000090_RESERVED20: c_uint = 0x000FFFFF;
pub const R_000100_MCCFG_FB_LOCATION: c_uint = 0x000100;

pub const C_000100_MC_FB_START: c_uint = 0xFFFF0000;

pub const C_000100_MC_FB_TOP: c_uint = 0x0000FFFF;
pub const R_000104_MC_INIT_MISC_LAT_TIMER: c_uint = 0x000104;

pub const C_000104_MC_CPR_INIT_LAT: c_uint = 0xFFFFFFF0;

pub const C_000104_MC_VF_INIT_LAT: c_uint = 0xFFFFFF0F;

pub const C_000104_MC_DISP0R_INIT_LAT: c_uint = 0xFFFFF0FF;

pub const C_000104_MC_DISP1R_INIT_LAT: c_uint = 0xFFFF0FFF;

pub const C_000104_MC_FIXED_INIT_LAT: c_uint = 0xFFF0FFFF;

pub const C_000104_MC_E2R_INIT_LAT: c_uint = 0xFF0FFFFF;

pub const C_000104_SAME_PAGE_PRIO: c_uint = 0xF0FFFFFF;

pub const C_000104_MC_GLOBW_INIT_LAT: c_uint = 0x0FFFFFFF;
