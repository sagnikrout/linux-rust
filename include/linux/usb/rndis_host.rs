//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/rndis_host.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Host Side support for RNDIS Networking Links
// Copyright (C) 2005 by David Brownell
//

//
// CONTROL uses CDC "encapsulated commands" with funky notifications.
// - control-out:  SEND_ENCAPSULATED
// - interrupt-in:  RESPONSE_AVAILABLE
// - control-in:  GET_ENCAPSULATED
//
// We'll try to ignore the RESPONSE_AVAILABLE notifications.
//
// REVISIT some RNDIS implementations seem to have curious issues still
// to be resolved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_msg_hdr {
    pub /: *mut *mut *mut __le32 msg_type; / RNDIS_MSG_,
    pub msg_len: __le32,
// followed by data that varies between messages
    pub request_id: __le32,
    pub status: __le32,
// ... and more
// C attribute field omitted
// MS-Windows uses this strange size, but RNDIS spec says 1024 minimum
pub const CONTROL_BUFFER_SIZE: c_int = 1025;
// RNDIS defines an (absurdly huge) 10 second control timeout,
// but ActiveSync seems to use a more usual 5 second timeout
// (which matches the USB 2.0 spec).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_data_hdr {
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_PACKET,
    pub /: *mut *mut __le32 msg_len; / rndis_data_hdr + data_len + pad,
    pub /: *mut *mut __le32 data_offset; / 36 -- right after header,
    pub /: *mut *mut __le32 data_len; / ... real packet size,
    pub /: *mut *mut __le32 oob_data_offset; / zero,
    pub /: *mut *mut __le32 oob_data_len; / zero,
    pub /: *mut *mut __le32 num_oob; / zero,
    pub /: *mut *mut __le32 packet_data_offset; / zero,
    pub /: *mut *mut __le32 packet_data_len; / zero,
    pub /: *mut *mut __le32 vc_handle; / zero,
    pub /: *mut *mut __le32 reserved; / zero,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_init {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_INIT,
    pub /: *mut *mut __le32 msg_len; / 24,
    pub request_id: __le32,
    pub /: *mut *mut __le32 major_version; / of rndis (1.0),
    pub minor_version: __le32,
    pub max_transfer_size: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_init_c {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_INIT_C,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub status: __le32,
    pub /: *mut *mut __le32 major_version; / of rndis (1.0),
    pub minor_version: __le32,
    pub device_flags: __le32,
    pub /: *mut *mut __le32 medium; / zero == 802.3,
    pub max_packets_per_message: __le32,
    pub max_transfer_size: __le32,
    pub /: *mut *mut __le32 packet_alignment; / max 7; (1<<n) bytes,
    pub /: *mut *mut __le32 af_list_offset; / zero,
    pub /: *mut *mut __le32 af_list_size; / zero,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_halt {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_HALT,
    pub msg_len: __le32,
    pub request_id: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_query {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_QUERY,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub oid: __le32,
    pub len: __le32,
    pub offset: __le32,
// ?*/	__le32	handle;				/* zero
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_query_c {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_QUERY_C,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub status: __le32,
    pub len: __le32,
    pub offset: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_set {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_SET,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub oid: __le32,
    pub len: __le32,
    pub offset: __le32,
// ?*/	__le32	handle;				/* zero
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_set_c {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_SET_C,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub status: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_reset {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_RESET,
    pub msg_len: __le32,
    pub reserved: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_reset_c {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_RESET_C,
    pub msg_len: __le32,
    pub status: __le32,
    pub addressing_lost: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_indicate {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_INDICATE,
    pub msg_len: __le32,
    pub status: __le32,
    pub length: __le32,
    pub offset: __le32,
// __le32	diag_status;
    pub error_offset: __le32,
// __le32	message;
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_keepalive {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_KEEPALIVE,
    pub msg_len: __le32,
    pub request_id: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_keepalive_c {
// header and:
    pub /: *mut *mut __le32 msg_type; / RNDIS_MSG_KEEPALIVE_C,
    pub msg_len: __le32,
    pub request_id: __le32,
    pub status: __le32,
// C attribute field omitted
// default filter used with RNDIS devices

// Flags to require specific physical medium type for generic_rndis_bind()
pub const FLAG_RNDIS_PHYM_NOT_WIRELESS: c_uint = 0x0001;
pub const FLAG_RNDIS_PHYM_WIRELESS: c_uint = 0x0002;
// Flags for driver_info::data

    pub urb): *mut *mut extern void rndis_status(struct usbnet dev, struct urb,
    pub buflen): *mut *mut *mut rndis_command(struct usbnet dev, struct rndis_msg_hdr buf, int,
    pub flags): *mut *mut *mut generic_rndis_bind(struct usbnet dev, struct usb_interface intf, int,
    pub intf): *mut *mut extern void rndis_unbind(struct usbnet dev, struct usb_interface,
    pub skb): *mut *mut extern int rndis_rx_fixup(struct usbnet dev, struct sk_buff,
    pub flags): *mut *mut *mut rndis_tx_fixup(struct usbnet dev, struct sk_buff skb, gfp_t,
