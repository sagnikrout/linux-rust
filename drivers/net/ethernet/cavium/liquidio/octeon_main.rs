//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_main.h
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
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file octeon_main.h
// \brief Host Driver: This file is included by all host driver source files
// to include common definitions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_device_priv {
// Tasklet structures for this device.
    pub droq_tasklet: tasklet_struct,
    pub napi_mask: c_ulong,
    pub dev: *mut octeon_device,
}

// This structure is used by NIC driver to store information required
// to free the sk_buff when the packet has been fetched by Octeon.
// Bytes offset below assume worst-case of a 64-bit system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octnet_buf_free_info {
// Bytes 1-8.  Pointer to network device private structure.
    pub lio: *mut lio,
// Bytes 9-16.  Pointer to sk_buff.
    pub skb: *mut sk_buff,
// Bytes 17-24.  Pointer to gather list.
    pub g: *mut octnic_gather,
// Bytes 25-32. Physical address of skb->data or gather list.
    pub dptr: u64,
// Bytes 33-47. Piggybacked soft command, if any
    pub sc: *mut octeon_soft_command,
}

// BQL-related functions
extern "C" {
    pub fn octeon_report_sent_bytes_to_bql(buf: *mut c_void, reqtype: c_int) -> c_int;
}
extern "C" {
    pub fn octeon_pf_changed_vf_macaddr(oct: *mut octeon_device, mac: *mut u8);
}
// Swap 8B blocks
//
// \brief unmaps a PCI BAR
// @param oct Pointer to Octeon device
// @param baridx bar index
//
// \brief maps a PCI BAR
// @param oct Pointer to Octeon device
// @param baridx bar index
// @param max_map_len maximum length of mapped memory
//
// input parameter:
// sc: pointer to a soft request
// timeout: milli sec which an application wants to wait for the
// 0: the request will wait until its response gets back
// from the firmware within LIO_SC_MAX_TMO_MS milli sec.
// If the response does not return within
// LIO_SC_MAX_TMO_MS milli sec, lio_process_ordered_list()
// will move the request to zombie response list.
//
// return value:
// 0: got the response from firmware for the sc request.
// errno -EINTR: user abort the command.
// errno -ETIME: user spefified timeout value has been expired.
// errno -EBUSY: the response of the request does not return in
// resonable time (LIO_SC_MAX_TMO_MS).
// the sc wll be move to zombie response list by
// lio_process_ordered_list()
//
// A request with non-zero return value, the sc->caller_is_done
// will be marked 1.
// When getting a request with zero return value, the requestor
// should mark sc->caller_is_done with 1 after examing the
// response of sc.
// lio_process_ordered_list() will free the soft command on behalf
// of the soft command requestor.
// This is to fix the possible race condition of both timeout process
// and lio_process_ordered_list()/callback function to free a
// sc strucutre.
//

