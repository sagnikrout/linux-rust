//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_psr_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

pub const _TRANS_EXITLINE_A: c_uint = 0x60018;

pub const EXITLINE_SHIFT: c_int = 0;
//
// HSW+ eDP PSR registers
//
// HSW PSR registers are relative to DDIA(_DDI_BUF_CTL_A + 0x800) with just one
// instance of it
//

pub const _SRD_CTL_A: c_uint = 0x60800;
pub const _SRD_CTL_EDP: c_uint = 0x6f800;

//
// Until TGL, IMR/IIR are fixed at 0x648xx. On TGL+ those registers are relative
// to transcoder and bits defined for each one as if using no shift (i.e. as if
// it was for TRANSCODER_EDP)
//

pub const _PSR_IMR_A: c_uint = 0x60814;
pub const _PSR_IIR_A: c_uint = 0x60818;

pub const _SRD_AUX_CTL_A: c_uint = 0x60810;
pub const _SRD_AUX_CTL_EDP: c_uint = 0x6f810;

pub const _SRD_AUX_DATA_A: c_uint = 0x60814;
pub const _SRD_AUX_DATA_EDP: c_uint = 0x6f814;

pub const _SRD_STATUS_A: c_uint = 0x60840;
pub const _SRD_STATUS_EDP: c_uint = 0x6f840;

pub const _SRD_PERF_CNT_A: c_uint = 0x60844;
pub const _SRD_PERF_CNT_EDP: c_uint = 0x6f844;

// PSR_MASK on SKL+

pub const _SRD_DEBUG_A: c_uint = 0x60860;
pub const _SRD_DEBUG_EDP: c_uint = 0x6f860;

pub const _PSR2_CTL_A: c_uint = 0x60900;
pub const _PSR2_CTL_EDP: c_uint = 0x6f900;

pub const EDP_PSR2_IO_BUFFER_WAKE_MAX_LINES: c_int = 8;

pub const TGL_EDP_PSR2_IO_BUFFER_WAKE_MIN_LINES: c_int = 5;

pub const LNL_EDP_PSR2_IO_BUFFER_WAKE_MIN_LINES: c_int = 5;

pub const EDP_PSR2_FAST_WAKE_MAX_LINES: c_int = 8;

pub const TGL_EDP_PSR2_FAST_WAKE_MIN_LINES: c_int = 5;

pub const _PSR_EVENT_TRANS_A: c_uint = 0x60848;
pub const _PSR_EVENT_TRANS_B: c_uint = 0x61848;
pub const _PSR_EVENT_TRANS_C: c_uint = 0x62848;
pub const _PSR_EVENT_TRANS_D: c_uint = 0x63848;
pub const _PSR_EVENT_TRANS_EDP: c_uint = 0x6f848;

pub const _PSR2_STATUS_A: c_uint = 0x60940;
pub const _PSR2_STATUS_EDP: c_uint = 0x6f940;

pub const _PSR2_SU_STATUS_A: c_uint = 0x60914;
pub const _PSR2_SU_STATUS_EDP: c_uint = 0x6f914;

pub const PSR2_SU_STATUS_FRAMES: c_int = 8;
pub const _PSR2_MAN_TRK_CTL_A: c_uint = 0x60910;
pub const _PSR2_MAN_TRK_CTL_EDP: c_uint = 0x6f910;

pub const _LNL_SFF_CTL_A: c_uint = 0x60918;
pub const _LNL_SFF_CTL_B: c_uint = 0x61918;

pub const _LNL_CFF_CTL_A: c_uint = 0x6091c;
pub const _LNL_CFF_CTL_B: c_uint = 0x6191c;

// PSR2 Early transport
pub const _PIPE_SRCSZ_ERLY_TPT_A: c_uint = 0x70074;
pub const _PIPE_SRCSZ_ERLY_TPT_B: c_uint = 0x71074;

pub const _PR_ALPM_CTL_A: c_uint = 0x60948;

pub const _ALPM_CTL_A: c_uint = 0x60950;

pub const ALPM_CTL_EXTENDED_FAST_WAKE_MIN_LINES: c_int = 5;

pub const _ALPM_CTL2_A: c_uint = 0x60954;

pub const _PORT_ALPM_CTL_A: c_uint = 0x16fa2c;
pub const _PORT_ALPM_CTL_B: c_uint = 0x16fc2c;

pub const _PORT_ALPM_LFPS_CTL_A: c_uint = 0x16fa30;
pub const _PORT_ALPM_LFPS_CTL_B: c_uint = 0x16fc30;

pub const PORT_ALPM_LFPS_CTL_LFPS_CYCLE_COUNT_MIN: c_int = 7;

