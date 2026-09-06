//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_nic.h
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
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more
// details.
//
// !  \file octeon_nic.h
// \brief Host NIC Driver: Routine to send network data &
// control packet to Octeon.
//
// Maximum number of 8-byte words can be sent in a NIC control message.
//
pub const MAX_NCTRL_UDD: c_int = 32;
extern "C" {
    pub fn void(: *mut *mut octnic_ctrl_pkt_cb_fn_t) (void) -> typedef;
}
// Structure of control information passed by the NIC module to the OSI
// layer when sending control commands to Octeon device software.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octnic_ctrl_pkt {
// Command to be passed to the Octeon device software.
    pub ncmd: octnet_cmd,
// Send buffer
    pub data: *mut c_void,
    pub dmadata: u64,
// Response buffer
    pub rdata: *mut c_void,
    pub dmardata: u64,
// Additional data that may be needed by some commands.
    pub udd: [u64; MAX_NCTRL_UDD],
// Input queue to use to send this command.
    pub iq_no: u64,
// The network device that issued the control command.
    pub netpndev: u64,
// Callback function called when the command has been fetched
    pub cb_fn: octnic_ctrl_pkt_cb_fn_t,
    pub sc_status: u32,
}

// Structure of data information passed by the NIC module to the OSI
// layer when forwarding data to Octeon device software.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octnic_data_pkt {
// Pointer to information maintained by NIC module for this packet. The
// OSI layer passes this as-is to the driver.
//
    pub buf: *mut c_void,
// Type of buffer passed in "buf" above.
    pub reqtype: u32,
// Total data bytes to be transferred in this command.
    pub datasize: u32,
// Command to be passed to the Octeon device software.
    pub cmd: octeon_instr_64B,
// Input queue to use to send this command.
    pub q_no: u32,
}

// Structure passed by NIC module to OSI layer to prepare a command to send
// network data to Octeon.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union octnic_cmd_setup {
    pub iq_no:8: u32,
    pub gather:1: u32,
    pub timestamp:1: u32,
    pub ip_csum:1: u32,
    pub transport_csum:1: u32,
    pub tnl_csum:1: u32,
    pub rsvd:19: u32,
    pub datasize: u32,
    pub gatherptrs: u32,
    pub u: },
    pub s: },
    pub u64: u64,
}

// assume that rflag is cleared so therefore front data will only have
// irh and ossp[0], ossp[1] for a total of 32 bytes
//
// assume that rflag is cleared so therefore front data will only have
// irh and ossp[1] and ossp[2] for a total of 24 bytes
//
// PKI IH
// Utility function to prepare a 64B NIC instruction based on a setup command
// @param cmd - pointer to instruction to be filled in.
// @param setup - pointer to the setup structure
// @param q_no - which queue for back pressure
//
// Assumes the cmd instruction is pre-allocated, but no fields are filled in.
//
// Allocate and a soft command with space for a response immediately following
// the commnad.
// @param oct - octeon device pointer
// @param cmd - pointer to the command structure, pre-filled for everything
// except the response.
// @param rdatasize - size in bytes of the response.
//
// @returns pointer to allocated buffer with command copied into it, and
// response space immediately following.
//
// Send a NIC data packet to the device
// @param oct - octeon device pointer
// @param ndata - control structure with queueing, and buffer information
//
// @returns IQ_FAILED if it failed to add to the input queue. IQ_STOP if the
// queue should be stopped, and IQ_SEND_OK if it sent okay.
//
// Send a NIC control packet to the device
// @param oct - octeon device pointer
// @param nctrl - control structure with command, timout, and callback info
// @returns IQ_FAILED if it failed to add to the input queue. IQ_STOP if the
// queue should be stopped, and IQ_SEND_OK if it sent okay.
//
