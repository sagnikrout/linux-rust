//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/fcp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Focusrite Control Protocol Driver for ALSA
//
// Copyright (c) 2024-2025 by Geoffrey D. Bennett <g at b4.vu>
//
// DOC: FCP (Focusrite Control Protocol) User-Space API
//
// This header defines the interface between the FCP kernel driver and
// user-space programs to enable the use of the proprietary features
// available in Focusrite USB audio interfaces. This includes Scarlett
// 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, Clarett+, and Vocaster
// series devices.
//
// The interface is provided via ALSA's hwdep interface. Opening the
// hwdep device requires CAP_SYS_RAWIO privileges as this interface
// provides near-direct access.
//
// For details on the FCP protocol, refer to the kernel scarlett2
// driver in sound/usb/mixer_scarlett2.c and the fcp-support project
// at https://github.com/geoffreybennett/fcp-support
//
// For examples of using these IOCTLs, see the fcp-server source in
// the fcp-support project.
//
// IOCTL Interface
// --------------
// FCP_IOCTL_PVERSION:
// Returns the protocol version supported by the driver.
//
// FCP_IOCTL_INIT:
// Initialises the protocol and synchronises sequence numbers
// between the driver and device. Must be called at least once
// before sending commands. Can be safely called again at any time.
//
// FCP_IOCTL_CMD:
// Sends an FCP command to the device and returns the response.
// Requires prior initialisation via FCP_IOCTL_INIT.
//
// FCP_IOCTL_SET_METER_MAP:
// Configures the Level Meter control's mapping between device
// meters and control channels. Requires FCP_IOCTL_INIT to have been
// called first. The map size and number of slots cannot be changed
// after initial configuration, although the map itself can be
// updated. Once configured, the Level Meter remains functional even
// after the hwdep device is closed.
//
// FCP_IOCTL_SET_METER_LABELS:
// Set the labels for the Level Meter control. Requires
// FCP_IOCTL_SET_METER_MAP to have been called first. labels[]
// should contain a sequence of null-terminated labels corresponding
// to the control's channels.
//

pub const FCP_HWDEP_MAJOR: c_int = 2;
pub const FCP_HWDEP_MINOR: c_int = 0;
pub const FCP_HWDEP_SUBMINOR: c_int = 0;

// Get protocol version

// Start the protocol
// Step 0 and step 2 responses are variable length and placed in
// resp[] one after the other.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_init {
    pub step0_resp_size: __u16,
    pub step2_resp_size: __u16,
    pub init1_opcode: __u32,
    pub init2_opcode: __u32,
    pub resp: [__u8; ],
    pub __attribute__((packed)): },

// Perform a command
// The request data is placed in data[] and the response data will
// overwrite it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmd {
    pub opcode: __u32,
    pub req_size: __u16,
    pub resp_size: __u16,
    pub data: [__u8; ],
    pub __attribute__((packed)): },

// Set the meter map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_meter_map {
    pub map_size: __u16,
    pub meter_slots: __u16,
    pub map: [__s16; ],
    pub __attribute__((packed)): },

// Set the meter labels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_meter_labels {
    pub labels_size: __u16,
    pub labels: [c_char; ],
    pub __attribute__((packed)): },

