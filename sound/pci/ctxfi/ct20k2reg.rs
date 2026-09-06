//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ct20k2reg.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// Timer Registers
pub const WC: c_uint = 0x1b7000;
pub const TIMR: c_uint = 0x1b7004;

pub const GIP: c_uint = 0x1b7010;
pub const GIE: c_uint = 0x1b7014;
// I2C Registers
pub const I2C_IF_ADDRESS: c_uint = 0x1B9000;
pub const I2C_IF_WDATA: c_uint = 0x1B9004;
pub const I2C_IF_RDATA: c_uint = 0x1B9008;
pub const I2C_IF_STATUS: c_uint = 0x1B900C;
pub const I2C_IF_WLOCK: c_uint = 0x1B9010;
// Global Control Registers
pub const GLOBAL_CNTL_GCTL: c_uint = 0x1B7090;
// PLL Registers
pub const PLL_CTL: c_uint = 0x1B7080;
pub const PLL_STAT: c_uint = 0x1B7084;
pub const PLL_ENB: c_uint = 0x1B7088;
// SRC Registers
pub const SRC_CTL: c_uint = 0x1A0000 /* 0x1A0000 + (256 * Chn) */;
pub const SRC_CCR: c_uint = 0x1A0004 /* 0x1A0004 + (256 * Chn) */;
pub const SRC_IMAP: c_uint = 0x1A0008 /* 0x1A0008 + (256 * Chn) */;
pub const SRC_CA: c_uint = 0x1A0010 /* 0x1A0010 + (256 * Chn) */;
pub const SRC_CF: c_uint = 0x1A0014 /* 0x1A0014 + (256 * Chn) */;
pub const SRC_SA: c_uint = 0x1A0018 /* 0x1A0018 + (256 * Chn) */;
pub const SRC_LA: c_uint = 0x1A001C /* 0x1A001C + (256 * Chn) */;
pub const SRC_CTLSWR: c_uint = 0x1A0020 /* 0x1A0020 + (256 * Chn) */;
pub const SRC_CD: c_uint = 0x1A0080 /* 0x1A0080 + (256 * Chn) + (4 * Regn) */;
pub const SRC_MCTL: c_uint = 0x1A012C;
pub const SRC_IP: c_uint = 0x1A102C /* 0x1A102C + (256 * Regn) */;
pub const SRC_ENB: c_uint = 0x1A282C /* 0x1A282C + (256 * Regn) */;
pub const SRC_ENBSTAT: c_uint = 0x1A202C;
pub const SRC_ENBSA: c_uint = 0x1A232C;
pub const SRC_DN0Z: c_uint = 0x1A0030;
pub const SRC_DN1Z: c_uint = 0x1A0040;
pub const SRC_UPZ: c_uint = 0x1A0060;
// GPIO Registers
pub const GPIO_DATA: c_uint = 0x1B7020;
pub const GPIO_CTRL: c_uint = 0x1B7024;
pub const GPIO_EXT_DATA: c_uint = 0x1B70A0;
// Virtual memory registers
pub const VMEM_PTPAL: c_uint = 0x1C6300 /* 0x1C6300 + (16 * Chn) */;
pub const VMEM_PTPAH: c_uint = 0x1C6304 /* 0x1C6304 + (16 * Chn) */;
pub const VMEM_CTL: c_uint = 0x1C7000;
// Transport Registers
pub const TRANSPORT_ENB: c_uint = 0x1B6000;
pub const TRANSPORT_CTL: c_uint = 0x1B6004;
pub const TRANSPORT_INT: c_uint = 0x1B6008;
// Audio IO
pub const AUDIO_IO_AIM: c_uint = 0x1B5000 /* 0x1B5000 + (0x04 * Chn) */;
pub const AUDIO_IO_TX_CTL: c_uint = 0x1B5400 /* 0x1B5400 + (0x40 * Chn) */;
pub const AUDIO_IO_TX_CSTAT_L: c_uint = 0x1B5408 /* 0x1B5408 + (0x40 * Chn) */;
pub const AUDIO_IO_TX_CSTAT_H: c_uint = 0x1B540C /* 0x1B540C + (0x40 * Chn) */;
pub const AUDIO_IO_RX_CTL: c_uint = 0x1B5410 /* 0x1B5410 + (0x40 * Chn) */;
pub const AUDIO_IO_RX_SRT_CTL: c_uint = 0x1B5420 /* 0x1B5420 + (0x40 * Chn) */;
pub const AUDIO_IO_MCLK: c_uint = 0x1B5600;
pub const AUDIO_IO_TX_BLRCLK: c_uint = 0x1B5604;
pub const AUDIO_IO_RX_BLRCLK: c_uint = 0x1B5608;
// Mixer
pub const MIXER_AMOPLO: c_uint = 0x130000 /* 0x130000 + (8 * Chn) [4095 : 0] */;
pub const MIXER_AMOPHI: c_uint = 0x130004 /* 0x130004 + (8 * Chn) [4095 : 0] */;
pub const MIXER_PRING_LO_HI: c_uint = 0x188000 /* 0x188000 + (4 * Chn) [4095 : 0] */;
pub const MIXER_PMOPLO: c_uint = 0x138000 /* 0x138000 + (8 * Chn) [4095 : 0] */;
pub const MIXER_PMOPHI: c_uint = 0x138004 /* 0x138004 + (8 * Chn) [4095 : 0] */;
pub const MIXER_AR_ENABLE: c_uint = 0x19000C;
