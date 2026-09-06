//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-tpda.h
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
// Copyright (c) 2023,2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

// Cross trigger global (all ports) flush request bit

// Cross trigger FREQ packets timestamp bit

// Cross trigger FREQ packet request bit

// Cross trigger FLAG packet request interface bit

// Cross trigger synchronization bit

// Bits 6 ~ 12 is for atid value

//
// Channel mode bit of the packetization of CMB/MCB traffic
// 0 - raw channel mapping mode
// 1 - channel pair marking mode
//

// Aggregator port enable bit

// Aggregator port CMB data set element size bit

// Aggregator port DSB data set element size bit

// TPDA_SYNCR count mask

// TPDA_SYNCR mode control bit

pub const TPDA_MAX_INPORTS: c_int = 32;
//
// struct tpda_drvdata - specifics associated to an TPDA component
// @base:       memory mapped base address for this component.
// @dev:        The device entity associated to this component.
// @csdev:      component vitals needed by the framework.
// @spinlock:   lock for the drvdata value.
// @enable:     enable status of the component.
// @dsb_esize   Record the DSB element size.
// @cmb_esize   Record the CMB element size.
// @trig_async:	Enable/disable cross trigger synchronization sequence interface.
// @trig_flag_ts: Enable/disable cross trigger FLAG packet request interface.
// @trig_freq:	Enable/disable cross trigger FREQ packet request interface.
// @freq_ts:	Enable/disable the timestamp for all FREQ packets.
// @cmbchan_mode: Configure the CMB/MCMB channel mode.
// @syncr_mode:	Setting the mode for counting packets.
// @syncr_count: Setting the value of the count.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpda_drvdata {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub csdev: *mut coresight_device,
    pub spinlock: spinlock_t,
    pub atid: u8,
    pub dsb_esize: u32,
    pub cmb_esize: u32,
    pub trig_async: bool,
    pub trig_flag_ts: bool,
    pub trig_freq: bool,
    pub freq_ts: bool,
    pub cmbchan_mode: bool,
    pub syncr_mode: bool,
    pub syncr_count: u32,
}

// Enumerate members of global control register(cr)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpda_cr_mem {
    FREQTS,
    FRIE,
    FLRIE,
    SRIE,
    CMBCHANMODE
}

//
// struct tpda_trig_sysfs_attribute - Record the member variables of cross
// trigger register that need to be operated by sysfs file
// @attr:	The device attribute
// @mem:	The member in the control register data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpda_trig_sysfs_attribute {
    pub attr: device_attribute,
    pub mem: tpda_cr_mem,
}

