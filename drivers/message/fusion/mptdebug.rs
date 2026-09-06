//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptdebug.h
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
//
// linux/drivers/message/fusion/mptdebug.h
// For use with LSI PCI chip/adapter(s)
// running LSI Fusion MPT (Message Passing Technology) firmware.
//
// Copyright (c) 1999-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

// Macro flag: #define MPTDEBUG_H_INCLUDED
//
// debug level can be programmed on the fly via SysFS (hex values)
//
// Example:  (programming for MPT_DEBUG_EVENTS on host 5)
//
// echo 8 > /sys/class/scsi_host/host5/debug_level
//
// --------------------------------------------------------
// mpt_debug_level - command line parameter
// this allow enabling debug at driver load time (for all iocs)
//
// Example  (programming for MPT_DEBUG_EVENTS)
//
// insmod mptbase.ko mpt_debug_level=8
//
// --------------------------------------------------------
// CONFIG_FUSION_LOGGING - enables compiling debug into driver
// this can be enabled in the driver Makefile
//
// --------------------------------------------------------
// Please note most debug prints are set to logging priority = debug
// This is the lowest level, and most verbose.  Please refer to manual
// pages for syslogd or syslogd-ng on how to configure this.
//
pub const MPT_DEBUG: c_uint = 0x00000001;
pub const MPT_DEBUG_MSG_FRAME: c_uint = 0x00000002;
pub const MPT_DEBUG_SG: c_uint = 0x00000004;
pub const MPT_DEBUG_EVENTS: c_uint = 0x00000008;
pub const MPT_DEBUG_VERBOSE_EVENTS: c_uint = 0x00000010;
pub const MPT_DEBUG_INIT: c_uint = 0x00000020;
pub const MPT_DEBUG_EXIT: c_uint = 0x00000040;
pub const MPT_DEBUG_FAIL: c_uint = 0x00000080;
pub const MPT_DEBUG_TM: c_uint = 0x00000100;
pub const MPT_DEBUG_DV: c_uint = 0x00000200;
pub const MPT_DEBUG_REPLY: c_uint = 0x00000400;
pub const MPT_DEBUG_HANDSHAKE: c_uint = 0x00000800;
pub const MPT_DEBUG_CONFIG: c_uint = 0x00001000;
pub const MPT_DEBUG_DL: c_uint = 0x00002000;
pub const MPT_DEBUG_RESET: c_uint = 0x00008000;
pub const MPT_DEBUG_SCSI: c_uint = 0x00010000;
pub const MPT_DEBUG_IOCTL: c_uint = 0x00020000;
pub const MPT_DEBUG_FC: c_uint = 0x00080000;
pub const MPT_DEBUG_SAS: c_uint = 0x00100000;
pub const MPT_DEBUG_SAS_WIDE: c_uint = 0x00200000;
pub const MPT_DEBUG_36GB_MEM: c_uint = 0x00400000;
//
// CONFIG_FUSION_LOGGING - enabled in Kconfig
//

//
// debug macros
//

//
// Verbose logging
//

