//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/ftm.h
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


// SPDX-License-Identifier: GPL-2.0
pub const FTM_SC: c_uint = 0x0 /* Status And Control */;
pub const FTM_CNT: c_uint = 0x4 /* Counter */;
pub const FTM_MOD: c_uint = 0x8 /* Modulo */;
pub const FTM_CNTIN: c_uint = 0x4C /* Counter Initial Value */;
pub const FTM_STATUS: c_uint = 0x50 /* Capture And Compare Status */;
pub const FTM_MODE: c_uint = 0x54 /* Features Mode Selection */;
pub const FTM_SYNC: c_uint = 0x58 /* Synchronization */;
pub const FTM_OUTINIT: c_uint = 0x5C /* Initial State For Channels Output */;
pub const FTM_OUTMASK: c_uint = 0x60 /* Output Mask */;
pub const FTM_COMBINE: c_uint = 0x64 /* Function For Linked Channels */;
pub const FTM_DEADTIME: c_uint = 0x68 /* Deadtime Insertion Control */;
pub const FTM_EXTTRIG: c_uint = 0x6C /* FTM External Trigger */;
pub const FTM_POL: c_uint = 0x70 /* Channels Polarity */;
pub const FTM_FMS: c_uint = 0x74 /* Fault Mode Status */;
pub const FTM_FILTER: c_uint = 0x78 /* Input Capture Filter Control */;
pub const FTM_FLTCTRL: c_uint = 0x7C /* Fault Control */;
pub const FTM_QDCTRL: c_uint = 0x80 /* Quadrature Decoder Control And Status */;
pub const FTM_CONF: c_uint = 0x84 /* Configuration */;
pub const FTM_FLTPOL: c_uint = 0x88 /* FTM Fault Input Polarity */;
pub const FTM_SYNCONF: c_uint = 0x8C /* Synchronization Configuration */;
pub const FTM_INVCTRL: c_uint = 0x90 /* FTM Inverting Control */;
pub const FTM_SWOCTRL: c_uint = 0x94 /* FTM Software Output Control */;
pub const FTM_PWMLOAD: c_uint = 0x98 /* FTM PWM Load */;
pub const FTM_SC_CLK_MASK_SHIFT: c_int = 3;

pub const FTM_SC_TOF: c_uint = 0x80;
pub const FTM_SC_TOIE: c_uint = 0x40;
pub const FTM_SC_CPWMS: c_uint = 0x20;
pub const FTM_SC_CLKS: c_uint = 0x18;
pub const FTM_SC_PS_1: c_uint = 0x0;
pub const FTM_SC_PS_2: c_uint = 0x1;
pub const FTM_SC_PS_4: c_uint = 0x2;
pub const FTM_SC_PS_8: c_uint = 0x3;
pub const FTM_SC_PS_16: c_uint = 0x4;
pub const FTM_SC_PS_32: c_uint = 0x5;
pub const FTM_SC_PS_64: c_uint = 0x6;
pub const FTM_SC_PS_128: c_uint = 0x7;
pub const FTM_SC_PS_MASK: c_uint = 0x7;
pub const FTM_MODE_FAULTIE: c_uint = 0x80;
pub const FTM_MODE_FAULTM: c_uint = 0x60;
pub const FTM_MODE_CAPTEST: c_uint = 0x10;
pub const FTM_MODE_PWMSYNC: c_uint = 0x8;
pub const FTM_MODE_WPDIS: c_uint = 0x4;
pub const FTM_MODE_INIT: c_uint = 0x2;
pub const FTM_MODE_FTMEN: c_uint = 0x1;
// NXP Errata: The PHAFLTREN and PHBFLTREN bits are tide to zero internally
// and these bits cannot be set. Flextimer cannot use Filter in
// Quadrature Decoder Mode.
// https://community.nxp.com/thread/467648#comment-1010319
//
pub const FTM_QDCTRL_PHAFLTREN: c_uint = 0x80;
pub const FTM_QDCTRL_PHBFLTREN: c_uint = 0x40;
pub const FTM_QDCTRL_PHAPOL: c_uint = 0x20;
pub const FTM_QDCTRL_PHBPOL: c_uint = 0x10;
pub const FTM_QDCTRL_QUADMODE: c_uint = 0x8;
pub const FTM_QDCTRL_QUADDIR: c_uint = 0x4;
pub const FTM_QDCTRL_TOFDIR: c_uint = 0x2;
pub const FTM_QDCTRL_QUADEN: c_uint = 0x1;
pub const FTM_FMS_FAULTF: c_uint = 0x80;
pub const FTM_FMS_WPEN: c_uint = 0x40;
pub const FTM_FMS_FAULTIN: c_uint = 0x10;
pub const FTM_FMS_FAULTF3: c_uint = 0x8;
pub const FTM_FMS_FAULTF2: c_uint = 0x4;
pub const FTM_FMS_FAULTF1: c_uint = 0x2;
pub const FTM_FMS_FAULTF0: c_uint = 0x1;
pub const FTM_CSC_BASE: c_uint = 0xC;
pub const FTM_CSC_MSB: c_uint = 0x20;
pub const FTM_CSC_MSA: c_uint = 0x10;
pub const FTM_CSC_ELSB: c_uint = 0x8;
pub const FTM_CSC_ELSA: c_uint = 0x4;

pub const FTM_CV_BASE: c_uint = 0x10;

pub const FTM_PS_MAX: c_int = 7;
