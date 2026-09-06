//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/scsi_netlink.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// SCSI Transport Netlink Interface
// Used for the posting of outbound SCSI transport events
//
// Copyright (C) 2006   James Smart, Emulex Corporation
//

//
// This file intended to be included by both kernel and user space
//
// Single Netlink Message type to send all SCSI Transport messages

// SCSI Transport Broadcast Groups
// leaving groups 0 and 1 unassigned

pub const SCSI_NL_GRP_CNT: c_int = 3;
// SCSI_TRANSPORT_MSG event message header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_nl_hdr {
    pub version: __u8,
    pub transport: __u8,
    pub magic: __u16,
    pub msgtype: __u16,
    pub msglen: __u16,
    pub __attribute__((aligned(sizeof(__u64)))): },
// scsi_nl_hdr->version value
pub const SCSI_NL_VERSION: c_int = 1;
// scsi_nl_hdr->magic value
pub const SCSI_NL_MAGIC: c_uint = 0xA1B2;
// scsi_nl_hdr->transport value
pub const SCSI_NL_TRANSPORT: c_int = 0;
pub const SCSI_NL_TRANSPORT_FC: c_int = 1;
pub const SCSI_NL_MAX_TRANSPORTS: c_int = 2;
// Transport-based scsi_nl_hdr->msgtype values are defined in each transport
//
// GENERIC SCSI scsi_nl_hdr->msgtype Values
//
// kernel -> user
pub const SCSI_NL_SHOST_VENDOR: c_uint = 0x0001;
// user -> kernel
// SCSI_NL_SHOST_VENDOR msgtype is kernel->user and user->kernel
//
// Message Structures :
//
// macro to round up message lengths to 8byte boundary

//
// SCSI HOST Vendor Unique messages :
// SCSI_NL_SHOST_VENDOR
//
// Note: The Vendor Unique message payload will begin directly after
// this structure, with the length of the payload per vmsg_datalen.
//
// Note: When specifying vendor_id, be sure to read the Vendor Type and ID
// formatting requirements specified below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_nl_host_vendor_msg {
    pub /: *mut *mut scsi_nl_hdr snlh; / must be 1st element !,
    pub vendor_id: __u64,
    pub host_no: __u16,
    pub vmsg_datalen: __u16,
    pub __attribute__((aligned(sizeof(__u64)))): },
//
// Vendor ID:
// If transports post vendor-unique events, they must pass a well-known
// 32-bit vendor identifier. This identifier consists of 8 bits indicating
// the "type" of identifier contained, and 24 bits of id data.
//
// Identifiers for each type:
// PCI :  ID data is the 16 bit PCI Registered Vendor ID
//
pub const SCSI_NL_VID_TYPE_SHIFT: c_int = 56;

    pub \: (hdr)->version = SCSI_NL_VERSION;,
    pub \: (hdr)->transport = t;,
    pub \: (hdr)->magic = SCSI_NL_MAGIC;,
    pub \: (hdr)->msgtype = mtype;,
    pub \: (hdr)->msglen = mlen;,
