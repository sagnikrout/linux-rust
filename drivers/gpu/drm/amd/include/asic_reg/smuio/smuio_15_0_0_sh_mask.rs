//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/smuio/smuio_15_0_0_sh_mask.h
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
// Copyright (C) 2025  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _smuio_15_0_0_SH_MASK_HEADER
// addressBlock: smuio_smuio_misc_SmuSmuioDec
// SMUIO_MCM_CONFIG
pub const SMUIO_MCM_CONFIG__DIE_ID__SHIFT: c_uint = 0x0;
pub const SMUIO_MCM_CONFIG__PKG_TYPE__SHIFT: c_uint = 0x2;
pub const SMUIO_MCM_CONFIG__SOCKET_ID__SHIFT: c_uint = 0x8;
pub const SMUIO_MCM_CONFIG__CONSOLE_K__SHIFT: c_uint = 0x10;
pub const SMUIO_MCM_CONFIG__CONSOLE_A__SHIFT: c_uint = 0x11;
pub const SMUIO_MCM_CONFIG__PKG_SUBTYPE__SHIFT: c_uint = 0x12;
pub const SMUIO_MCM_CONFIG__DIE_ID_MASK: c_uint = 0x00000003L;
pub const SMUIO_MCM_CONFIG__PKG_TYPE_MASK: c_uint = 0x0000003CL;
pub const SMUIO_MCM_CONFIG__SOCKET_ID_MASK: c_uint = 0x00000100L;
pub const SMUIO_MCM_CONFIG__CONSOLE_K_MASK: c_uint = 0x00010000L;
pub const SMUIO_MCM_CONFIG__CONSOLE_A_MASK: c_uint = 0x00020000L;
pub const SMUIO_MCM_CONFIG__PKG_SUBTYPE_MASK: c_uint = 0x00040000L;
// IP_DISCOVERY_VERSION
pub const IP_DISCOVERY_VERSION__IP_DISCOVERY_VERSION__SHIFT: c_uint = 0x0;
pub const IP_DISCOVERY_VERSION__IP_DISCOVERY_VERSION_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER0
pub const SCRATCH_REGISTER0__ScratchPad0__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER0__ScratchPad0_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER1
pub const SCRATCH_REGISTER1__ScratchPad1__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER1__ScratchPad1_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER2
pub const SCRATCH_REGISTER2__ScratchPad2__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER2__ScratchPad2_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER3
pub const SCRATCH_REGISTER3__ScratchPad3__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER3__ScratchPad3_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER4
pub const SCRATCH_REGISTER4__ScratchPad4__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER4__ScratchPad4_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER5
pub const SCRATCH_REGISTER5__ScratchPad5__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER5__ScratchPad5_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER6
pub const SCRATCH_REGISTER6__ScratchPad6__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER6__ScratchPad6_MASK: c_uint = 0xFFFFFFFFL;
// SCRATCH_REGISTER7
pub const SCRATCH_REGISTER7__ScratchPad7__SHIFT: c_uint = 0x0;
pub const SCRATCH_REGISTER7__ScratchPad7_MASK: c_uint = 0xFFFFFFFFL;
// IO_SMUIO_PINSTRAP
pub const IO_SMUIO_PINSTRAP__AUD_PORT_CONN__SHIFT: c_uint = 0x0;
pub const IO_SMUIO_PINSTRAP__AUD__SHIFT: c_uint = 0x3;
pub const IO_SMUIO_PINSTRAP__AUD_PORT_CONN_MASK: c_uint = 0x00000007L;
pub const IO_SMUIO_PINSTRAP__AUD_MASK: c_uint = 0x00000018L;
// addressBlock: smuio_smuio_reset_SmuSmuioDec
// SMUIO_GFX_MISC_CNTL
pub const SMUIO_GFX_MISC_CNTL__SMU_GFX_cold_vs_gfxoff__SHIFT: c_uint = 0x0;
pub const SMUIO_GFX_MISC_CNTL__PWR_GFXOFF_STATUS__SHIFT: c_uint = 0x1;
pub const SMUIO_GFX_MISC_CNTL__SMU_GFX_cold_vs_gfxoff_MASK: c_uint = 0x00000001L;
pub const SMUIO_GFX_MISC_CNTL__PWR_GFXOFF_STATUS_MASK: c_uint = 0x00000006L;
// addressBlock: smuio_smuio_tsc_SmuSmuioDec
// PWROK_REFCLK_GAP_CYCLES
pub const PWROK_REFCLK_GAP_CYCLES__Pwrok_PreAssertion_clkgap_cycles__SHIFT: c_uint = 0x0;
pub const PWROK_REFCLK_GAP_CYCLES__Pwrok_PostAssertion_clkgap_cycles__SHIFT: c_uint = 0x8;
pub const PWROK_REFCLK_GAP_CYCLES__Pwrok_PreAssertion_clkgap_cycles_MASK: c_uint = 0x000000FFL;
pub const PWROK_REFCLK_GAP_CYCLES__Pwrok_PostAssertion_clkgap_cycles_MASK: c_uint = 0x0000FF00L;
// GOLDEN_TSC_INCREMENT_UPPER
pub const GOLDEN_TSC_INCREMENT_UPPER__GoldenTscIncrementUpper__SHIFT: c_uint = 0x0;
pub const GOLDEN_TSC_INCREMENT_UPPER__GoldenTscIncrementUpper_MASK: c_uint = 0x00FFFFFFL;
// GOLDEN_TSC_INCREMENT_LOWER
pub const GOLDEN_TSC_INCREMENT_LOWER__GoldenTscIncrementLower__SHIFT: c_uint = 0x0;
pub const GOLDEN_TSC_INCREMENT_LOWER__GoldenTscIncrementLower_MASK: c_uint = 0xFFFFFFFFL;
// GOLDEN_TSC_COUNT_UPPER
pub const GOLDEN_TSC_COUNT_UPPER__GoldenTscCountUpper__SHIFT: c_uint = 0x0;
pub const GOLDEN_TSC_COUNT_UPPER__GoldenTscCountUpper_MASK: c_uint = 0x00FFFFFFL;
// GOLDEN_TSC_COUNT_LOWER
pub const GOLDEN_TSC_COUNT_LOWER__GoldenTscCountLower__SHIFT: c_uint = 0x0;
pub const GOLDEN_TSC_COUNT_LOWER__GoldenTscCountLower_MASK: c_uint = 0xFFFFFFFFL;
// SOC_GOLDEN_TSC_SHADOW_UPPER
pub const SOC_GOLDEN_TSC_SHADOW_UPPER__SocGoldenTscShadowUpper__SHIFT: c_uint = 0x0;
pub const SOC_GOLDEN_TSC_SHADOW_UPPER__SocGoldenTscShadowUpper_MASK: c_uint = 0x00FFFFFFL;
// SOC_GOLDEN_TSC_SHADOW_LOWER
pub const SOC_GOLDEN_TSC_SHADOW_LOWER__SocGoldenTscShadowLower__SHIFT: c_uint = 0x0;
pub const SOC_GOLDEN_TSC_SHADOW_LOWER__SocGoldenTscShadowLower_MASK: c_uint = 0xFFFFFFFFL;
// SOC_GAP_PWROK
pub const SOC_GAP_PWROK__soc_gap_pwrok__SHIFT: c_uint = 0x0;
pub const SOC_GAP_PWROK__soc_gap_pwrok_MASK: c_uint = 0x00000001L;
// addressBlock: smuio_smuio_swtimer_SmuSmuioDec
// PWR_VIRT_RESET_REQ
pub const PWR_VIRT_RESET_REQ__VF_FLR__SHIFT: c_uint = 0x0;
pub const PWR_VIRT_RESET_REQ__PF_FLR__SHIFT: c_uint = 0x1f;
pub const PWR_VIRT_RESET_REQ__VF_FLR_MASK: c_uint = 0x7FFFFFFFL;
pub const PWR_VIRT_RESET_REQ__PF_FLR_MASK: c_uint = 0x80000000L;
// PWR_DISP_TIMER_CONTROL
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_COUNT__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_ENABLE__SHIFT: c_uint = 0x19;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_DISABLE__SHIFT: c_uint = 0x1a;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_MASK__SHIFT: c_uint = 0x1b;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_STAT_AK__SHIFT: c_uint = 0x1c;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_TYPE__SHIFT: c_uint = 0x1d;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_MODE__SHIFT: c_uint = 0x1e;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_COUNT_MASK: c_uint = 0x01FFFFFFL;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_ENABLE_MASK: c_uint = 0x02000000L;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_DISABLE_MASK: c_uint = 0x04000000L;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_MASK_MASK: c_uint = 0x08000000L;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_STAT_AK_MASK: c_uint = 0x10000000L;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_TYPE_MASK: c_uint = 0x20000000L;
pub const PWR_DISP_TIMER_CONTROL__DISP_TIMER_INT_MODE_MASK: c_uint = 0x40000000L;
// PWR_DISP_TIMER_DEBUG
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT_RUNNING__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT_STAT__SHIFT: c_uint = 0x1;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT__SHIFT: c_uint = 0x2;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_RUN_VAL__SHIFT: c_uint = 0x7;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT_RUNNING_MASK: c_uint = 0x00000001L;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT_STAT_MASK: c_uint = 0x00000002L;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_INT_MASK: c_uint = 0x00000004L;
pub const PWR_DISP_TIMER_DEBUG__DISP_TIMER_RUN_VAL_MASK: c_uint = 0xFFFFFF80L;
// PWR_DISP_TIMER_ELAPSED_CONTROL
pub const PWR_DISP_TIMER_ELAPSED_CONTROL__DISP_TIMER_ELAPSED_TIME_COUNT__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER_ELAPSED_CONTROL__DISP_TIMER_COMP_ENABLE__SHIFT: c_uint = 0x19;
pub const PWR_DISP_TIMER_ELAPSED_CONTROL__DISP_TIMER_ELAPSED_TIME_COUNT_MASK: c_uint = 0x01FFFFFFL;
pub const PWR_DISP_TIMER_ELAPSED_CONTROL__DISP_TIMER_COMP_ENABLE_MASK: c_uint = 0x02000000L;
// PWR_DISP_TIMER2_CONTROL
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_COUNT__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_ENABLE__SHIFT: c_uint = 0x19;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_DISABLE__SHIFT: c_uint = 0x1a;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_MASK__SHIFT: c_uint = 0x1b;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_STAT_AK__SHIFT: c_uint = 0x1c;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_TYPE__SHIFT: c_uint = 0x1d;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_MODE__SHIFT: c_uint = 0x1e;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_COUNT_MASK: c_uint = 0x01FFFFFFL;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_ENABLE_MASK: c_uint = 0x02000000L;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_DISABLE_MASK: c_uint = 0x04000000L;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_MASK_MASK: c_uint = 0x08000000L;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_STAT_AK_MASK: c_uint = 0x10000000L;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_TYPE_MASK: c_uint = 0x20000000L;
pub const PWR_DISP_TIMER2_CONTROL__DISP_TIMER_INT_MODE_MASK: c_uint = 0x40000000L;
// PWR_DISP_TIMER2_DEBUG
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT_RUNNING__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT_STAT__SHIFT: c_uint = 0x1;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT__SHIFT: c_uint = 0x2;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_RUN_VAL__SHIFT: c_uint = 0x7;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT_RUNNING_MASK: c_uint = 0x00000001L;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT_STAT_MASK: c_uint = 0x00000002L;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_INT_MASK: c_uint = 0x00000004L;
pub const PWR_DISP_TIMER2_DEBUG__DISP_TIMER_RUN_VAL_MASK: c_uint = 0xFFFFFF80L;
// PWR_DISP_TIMER2_ELAPSED_CONTROL
pub const PWR_DISP_TIMER2_ELAPSED_CONTROL__DISP_TIMER_ELAPSED_TIME_COUNT__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER2_ELAPSED_CONTROL__DISP_TIMER_COMP_ENABLE__SHIFT: c_uint = 0x19;
pub const PWR_DISP_TIMER2_ELAPSED_CONTROL__DISP_TIMER_ELAPSED_TIME_COUNT_MASK: c_uint = 0x01FFFFFFL;
pub const PWR_DISP_TIMER2_ELAPSED_CONTROL__DISP_TIMER_COMP_ENABLE_MASK: c_uint = 0x02000000L;
// PWR_DISP_TIMER_GLOBAL_CONTROL
pub const PWR_DISP_TIMER_GLOBAL_CONTROL__DISP_TIMER_PULSE_WIDTH__SHIFT: c_uint = 0x0;
pub const PWR_DISP_TIMER_GLOBAL_CONTROL__DISP_TIMER_PULSE_EN__SHIFT: c_uint = 0xa;
pub const PWR_DISP_TIMER_GLOBAL_CONTROL__DISP_TIMER_PULSE_WIDTH_MASK: c_uint = 0x000003FFL;
pub const PWR_DISP_TIMER_GLOBAL_CONTROL__DISP_TIMER_PULSE_EN_MASK: c_uint = 0x00000400L;
// PWR_IH_CONTROL
pub const PWR_IH_CONTROL__MAX_CREDIT__SHIFT: c_uint = 0x0;
pub const PWR_IH_CONTROL__DISP_TIMER_TRIGGER_MASK__SHIFT: c_uint = 0x5;
pub const PWR_IH_CONTROL__DISP_TIMER2_TRIGGER_MASK__SHIFT: c_uint = 0x6;
pub const PWR_IH_CONTROL__PWR_IH_CLK_GATE_EN__SHIFT: c_uint = 0x1f;
pub const PWR_IH_CONTROL__MAX_CREDIT_MASK: c_uint = 0x0000001FL;
pub const PWR_IH_CONTROL__DISP_TIMER_TRIGGER_MASK_MASK: c_uint = 0x00000020L;
pub const PWR_IH_CONTROL__DISP_TIMER2_TRIGGER_MASK_MASK: c_uint = 0x00000040L;
pub const PWR_IH_CONTROL__PWR_IH_CLK_GATE_EN_MASK: c_uint = 0x80000000L;
