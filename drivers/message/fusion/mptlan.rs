//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptlan.h
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
// linux/drivers/message/fusion/mptlan.h
// IP Over Fibre Channel device driver.
// For use with LSI Fibre Channel PCI chip/adapters
// running LSI Fusion MPT (Message Passing Technology) firmware.
//
// Copyright (c) 2000-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
// mptlan.h

// Macro flag: #define LINUX_MPTLAN_H_INCLUDED
//

// #include <linux/etherdevice.h>

// #include <linux/fddidevice.h>

// Override mptbase.h by pre-defining these!

//

//
pub const MPT_LAN_MAX_BUCKETS_OUT: c_int = 256;

pub const MPT_LAN_BUCKETS_REMAIN_MISMATCH_THRESH: c_int = 10;
pub const MPT_LAN_RX_COPYBREAK: c_int = 200;

pub const MPT_TX_MAX_OUT_LIM: c_int = 127;

pub const MPT_LAN_NAA_RFC2625: c_uint = 0x1;
pub const MPT_LAN_NAA_QLOGIC: c_uint = 0x2;
// MPT LAN Reset and Suspend Resource Flags Defines
pub const MPT_LAN_RESOURCE_FLAG_RETURN_POSTED_BUCKETS: c_uint = 0x01;
pub const MPT_LAN_RESOURCE_FLAG_RETURN_PEND_TRANSMITS: c_uint = 0x02;
//

//
