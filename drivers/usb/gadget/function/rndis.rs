//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/rndis.h
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
// RNDIS	Definitions for Remote NDIS
//
// Authors:	Benedikt Spranger, Pengutronix
// Robert Schwebel, Pengutronix
//
// This software was originally developed in conformance with
// Microsoft's Remote NDIS Specification License Agreement.
//

pub const RNDIS_MAXIMUM_FRAME_SIZE: c_int = 1518;
pub const RNDIS_MAX_TOTAL_SIZE: c_int = 1558;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_packet_msg_type {
    pub MessageType: __le32,
    pub MessageLength: __le32,
    pub DataOffset: __le32,
    pub DataLength: __le32,
    pub OOBDataOffset: __le32,
    pub OOBDataLength: __le32,
    pub NumOOBDataElements: __le32,
    pub PerPacketInfoOffset: __le32,
    pub PerPacketInfoLength: __le32,
    pub VcHandle: __le32,
    pub Reserved: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_config_parameter {
    pub ParameterNameOffset: __le32,
    pub ParameterNameLength: __le32,
    pub ParameterType: __le32,
    pub ParameterValueOffset: __le32,
    pub ParameterValueLength: __le32,
}

// implementation specific
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rndis_state {
    RNDIS_UNINITIALIZED,
    RNDIS_INITIALIZED,
    RNDIS_DATA_INITIALIZED,
}

// RNDIS Message parser and other useless functions
extern "C" {
    pub fn rndis_msg_parser(params: *mut rndis_params, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn rndis_deregister(params: *mut rndis_params);
}
extern "C" {
    pub fn rndis_add_hdr(skb: *mut sk_buff);
}
extern "C" {
    pub fn rndis_free_response(params: *mut rndis_params, buf: *mut u8);
}
extern "C" {
    pub fn rndis_uninit(params: *mut rndis_params);
}
extern "C" {
    pub fn rndis_signal_connect(params: *mut rndis_params) -> c_int;
}
extern "C" {
    pub fn rndis_signal_disconnect(params: *mut rndis_params) -> c_int;
}
extern "C" {
    pub fn rndis_state(params: *mut rndis_params) -> c_int;
}
extern "C" {
    pub fn rndis_set_host_mac(params: *mut rndis_params, addr: *const u8);
}
