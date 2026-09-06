//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fman/fman_port.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2015 Freescale Semiconductor Inc.
//

// FM Port API
// The FM uses a general module called "port" to represent a Tx port (MAC),
// an Rx port (MAC).
// The number of ports in an FM varies between SOCs.
// The SW driver manages these ports as sub-modules of the FM,i.e. after an
// FM is initialized, its ports may be initialized and operated upon.
// The port is initialized aware of its type, but other functions on a port
// may be indifferent to its type. When necessary, the driver verifies
// coherence and returns error if applicable.
// On initialization, user specifies the port type and it's index (relative
// to the port's type) - always starting at 0.
//
// FM Frame error
// Frame Descriptor errors
// Not for Rx-Port! Unsupported Format

// Not for Rx-Port! Length Error

// DMA Data error

// non Frame-Manager error; probably come from SEC that was chained to FM

// IPR error

// IPR non-consistent-sp

// Rx FIFO overflow, FCS error, code error, running disparity
// error (SGMII and TBI modes), FIFO parity error.
// PHY Sequence error, PHY error control character detected.
//

// Frame too long OR Frame size exceeds max_length_frame

// indicates a classifier "drop" operation

// Extract Out of Frame

// No Scheme Selected

// Keysize Overflow

// Frame color is red

// Frame color is yellow

// Parser Time out Exceed

// Invalid Soft Parser instruction

// Header error was identified during parsing

// Frame parsed beyind 256 first bytes

// FPM Frame Processing Timeout Exceeded
pub const FM_PORT_FRM_ERR_PROCESS_TIMEOUT: c_uint = 0x00000001;
// A structure for additional Rx port parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_port_rx_params {
    pub /: *mut *mut u32 err_fqid; / Error Queue Id.,
    pub /: *mut *mut u32 dflt_fqid; / Default Queue Id.,
    pub /: *mut *mut u32 pcd_base_fqid; / PCD base Queue Id.,
    pub /: *mut *mut u32 pcd_fqs_count; / Number of PCD FQs.,
// Which external buffer pools are used
// (up to FMAN_PORT_MAX_EXT_POOLS_NUM), and their sizes.
//
    pub ext_buf_pools: fman_ext_pools,
}

// A structure for additional non-Rx port parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_port_non_rx_params {
// Error Queue Id.
    pub err_fqid: u32,
// For Tx - Default Confirmation queue, 0 means no Tx confirmation
// for processed frames. For OP port - default Rx queue.
//
    pub dflt_fqid: u32,
}

// A union for additional parameters depending on port type
#[repr(C)]
#[derive(Copy, Clone)]
pub union fman_port_specific_params {
// Rx port parameters structure
    pub rx_params: fman_port_rx_params,
// Non-Rx port parameters structure
    pub non_rx_params: fman_port_non_rx_params,
}

// A structure representing FM initialization parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_port_params {
// Virtual Address of memory mapped FM Port registers.
    pub fm: *mut c_void,
    pub specific_params: fman_port_specific_params,
// Additional parameters depending on port type.
}

extern "C" {
    pub fn fman_port_config(port: *mut fman_port, params: *mut fman_port_params) -> c_int;
}
extern "C" {
    pub fn fman_port_use_kg_hash(port: *mut fman_port, enable: bool);
}
extern "C" {
    pub fn fman_port_init(port: *mut fman_port) -> c_int;
}
// buffer_prefix_content);
extern "C" {
    pub fn fman_port_disable(port: *mut fman_port) -> c_int;
}
extern "C" {
    pub fn fman_port_enable(port: *mut fman_port) -> c_int;
}
extern "C" {
    pub fn fman_port_get_qman_channel_id(port: *mut fman_port) -> u32;
}
extern "C" {
    pub fn fman_port_get_hash_result_offset(port: *mut fman_port, offset: *mut u32) -> c_int;
}
extern "C" {
    pub fn fman_port_get_tstamp(port: *mut fman_port, data: *const c_void, tstamp: *mut u64) -> c_int;
}
