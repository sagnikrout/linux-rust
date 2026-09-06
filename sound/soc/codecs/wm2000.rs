//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm2000.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// wm2000.h  --  WM2000 Soc Audio driver
//
pub const WM2000_REG_SYS_START: c_uint = 0x8000;
pub const WM2000_REG_ANC_GAIN_CTRL: c_uint = 0x8fa2;
pub const WM2000_REG_MSE_TH2: c_uint = 0x8fdf;
pub const WM2000_REG_MSE_TH1: c_uint = 0x8fe0;
pub const WM2000_REG_SPEECH_CLARITY: c_uint = 0x8fef;
pub const WM2000_REG_SYS_WATCHDOG: c_uint = 0x8ff6;
pub const WM2000_REG_ANA_VMID_PD_TIME: c_uint = 0x8ff7;
pub const WM2000_REG_ANA_VMID_PU_TIME: c_uint = 0x8ff8;
pub const WM2000_REG_CAT_FLTR_INDX: c_uint = 0x8ff9;
pub const WM2000_REG_CAT_GAIN_0: c_uint = 0x8ffa;
pub const WM2000_REG_SYS_STATUS: c_uint = 0x8ffc;
pub const WM2000_REG_SYS_MODE_CNTRL: c_uint = 0x8ffd;
pub const WM2000_REG_SYS_START0: c_uint = 0x8ffe;
pub const WM2000_REG_SYS_START1: c_uint = 0x8fff;
pub const WM2000_REG_ID1: c_uint = 0xf000;
pub const WM2000_REG_ID2: c_uint = 0xf001;
pub const WM2000_REG_REVISON: c_uint = 0xf002;
pub const WM2000_REG_SYS_CTL1: c_uint = 0xf003;
pub const WM2000_REG_SYS_CTL2: c_uint = 0xf004;
pub const WM2000_REG_ANC_STAT: c_uint = 0xf005;
pub const WM2000_REG_IF_CTL: c_uint = 0xf006;
pub const WM2000_REG_ANA_MIC_CTL: c_uint = 0xf028;
pub const WM2000_REG_SPK_CTL: c_uint = 0xf034;
// SPEECH_CLARITY
pub const WM2000_SPEECH_CLARITY: c_uint = 0x01;
// SYS_STATUS
pub const WM2000_STATUS_MOUSE_ACTIVE: c_uint = 0x40;
pub const WM2000_STATUS_CAT_FREQ_COMPLETE: c_uint = 0x20;
pub const WM2000_STATUS_CAT_GAIN_COMPLETE: c_uint = 0x10;
pub const WM2000_STATUS_THERMAL_SHUTDOWN_COMPLETE: c_uint = 0x08;
pub const WM2000_STATUS_ANC_DISABLED: c_uint = 0x04;
pub const WM2000_STATUS_POWER_DOWN_COMPLETE: c_uint = 0x02;
pub const WM2000_STATUS_BOOT_COMPLETE: c_uint = 0x01;
// SYS_MODE_CNTRL
pub const WM2000_MODE_ANA_SEQ_INCLUDE: c_uint = 0x80;
pub const WM2000_MODE_MOUSE_ENABLE: c_uint = 0x40;
pub const WM2000_MODE_CAT_FREQ_ENABLE: c_uint = 0x20;
pub const WM2000_MODE_CAT_GAIN_ENABLE: c_uint = 0x10;
pub const WM2000_MODE_BYPASS_ENTRY: c_uint = 0x08;
pub const WM2000_MODE_STANDBY_ENTRY: c_uint = 0x04;
pub const WM2000_MODE_THERMAL_ENABLE: c_uint = 0x02;
pub const WM2000_MODE_POWER_DOWN: c_uint = 0x01;
// SYS_CTL1
pub const WM2000_SYS_STBY: c_uint = 0x01;
// SYS_CTL2
pub const WM2000_MCLK_DIV2_ENA_CLR: c_uint = 0x80;
pub const WM2000_MCLK_DIV2_ENA_SET: c_uint = 0x40;
pub const WM2000_ANC_ENG_CLR: c_uint = 0x20;
pub const WM2000_ANC_ENG_SET: c_uint = 0x10;
pub const WM2000_ANC_INT_N_CLR: c_uint = 0x08;
pub const WM2000_ANC_INT_N_SET: c_uint = 0x04;
pub const WM2000_RAM_CLR: c_uint = 0x02;
pub const WM2000_RAM_SET: c_uint = 0x01;
// ANC_STAT
pub const WM2000_ANC_ENG_IDLE: c_uint = 0x01;
