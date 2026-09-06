//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptp_classify.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PTP 1588 support
//
// This file implements a BPF that recognizes PTP event messages.
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const PTP_CLASS_NONE: c_uint = 0x00 /* not a PTP event message */;
pub const PTP_CLASS_V1: c_uint = 0x01 /* protocol version 1 */;
pub const PTP_CLASS_V2: c_uint = 0x02 /* protocol version 2 */;
pub const PTP_CLASS_VMASK: c_uint = 0x0f /* max protocol version is 15 */;
pub const PTP_CLASS_IPV4: c_uint = 0x10 /* event in an IPV4 UDP packet */;
pub const PTP_CLASS_IPV6: c_uint = 0x20 /* event in an IPV6 UDP packet */;
pub const PTP_CLASS_L2: c_uint = 0x40 /* event in a L2 packet */;
pub const PTP_CLASS_PMASK: c_uint = 0x70 /* mask for the packet type field */;
pub const PTP_CLASS_VLAN: c_uint = 0x80 /* event in a VLAN tagged packet */;

pub const PTP_MSGTYPE_SYNC: c_uint = 0x0;
pub const PTP_MSGTYPE_DELAY_REQ: c_uint = 0x1;
pub const PTP_MSGTYPE_PDELAY_REQ: c_uint = 0x2;
pub const PTP_MSGTYPE_PDELAY_RESP: c_uint = 0x3;
pub const PTP_EV_PORT: c_int = 319;
pub const PTP_GEN_PORT: c_int = 320;
pub const PTP_GEN_BIT: c_uint = 0x08 /* indicates general message, if set in message type */;

pub const OFF_PTP_SEQUENCE_ID: c_int = 30;
// PTP header flag fields

// Below defines should actually be removed at some point in time.
pub const IP6_HLEN: c_int = 40;
pub const UDP_HLEN: c_int = 8;
pub const OFF_IHL: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_identity {
    pub id: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_identity {
    pub clock_identity: clock_identity,
    pub port_number: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_header {
    pub /: *mut *mut u8 tsmt; / transportSpecific | messageType,
    pub /: *mut *mut u8 ver; / reserved | versionPTP,
    pub message_length: __be16,
    pub domain_number: u8,
    pub reserved1: u8,
    pub flag_field: [u8; 2],
    pub correction: __be64,
    pub reserved2: __be32,
    pub source_port_identity: port_identity,
    pub sequence_id: __be16,
    pub control: u8,
    pub log_message_interval: u8,
    pub __packed: },

//
// ptp_classify_raw - classify a PTP packet
// @skb: buffer
//
// Runs a minimal BPF dissector to classify a network packet to
// determine the PTP class. In case the skb does not contain any
// PTP protocol data, PTP_CLASS_NONE will be returned, otherwise
// PTP_CLASS_V1_IPV{4,6}, PTP_CLASS_V2_IPV{4,6} or
// PTP_CLASS_V2_{L2,VLAN}, depending on the packet content.
//
    pub skb): *const unsigned int ptp_classify_raw(struct sk_buff,
//
// ptp_parse_header - Get pointer to the PTP v2 header
// @skb: packet buffer
// @type: type of the packet (see ptp_classify_raw())
//
// This function takes care of the VLAN, UDP, IPv4 and IPv6 headers. The length
// is checked.
//
// Note, internally skb_mac_header() is used. Make sure that the @skb is
// initialized accordingly.
//
// Return: Pointer to the ptp v2 header or NULL if not found
//
    pub type): *mut *mut *mut ptp_header ptp_parse_header(sk_buff skb, unsigned int,
//
// ptp_get_msgtype - Extract ptp message type from given header
// @hdr: ptp header
// @type: type of the packet (see ptp_classify_raw())
//
// This function returns the message type for a given ptp header. It takes care
// of the different ptp header versions (v1 or v2).
//
// Return: The message type
//
    pub msgtype: u8,
// msg type is located at the control field for ptp v1
    pub hdr->control: msgtype =,
    pub 0x0f: msgtype = hdr->tsmt &,
    pub msgtype: return,
//
// ptp_check_diff8 - Computes new checksum (when altering a 64-bit field)
// @old: old field value
// @new: new field value
// @oldsum: previous checksum
//
// This function can be used to calculate a new checksum when only a single
// field is changed. Similar as ip_vs_check_diff*() in ip_vs.h.
//
// Return: Updated checksum
//
    pub }: __be64 diff[2] = { ~old, new,
    pub oldsum): return csum_partial(diff, sizeof(diff),,
//
// ptp_header_update_correction - Update PTP header's correction field
// @skb: packet buffer
// @type: type of the packet (see ptp_classify_raw())
// @hdr: ptp header
// @correction: new correction value
//
// This updates the correction field of a PTP header and updates the UDP
// checksum (if UDP is used as transport). It is needed for hardware capable of
// one-step P2P that does not already modify the correction field of Pdelay_Req
// event messages on ingress.
//
    pub correction_old: __be64,
    pub uhdr: *mut udphdr,
// previous correction value is required for checksum update.
    pub sizeof(correction_old)): memcpy(&correction_old, &hdr->correction,,
// write new correction value
    pub &hdr->correction): put_unaligned_be64((u64)correction,,
// locate udp header
    pub udphdr)): *mut *mut *mut uhdr = (struct udphdr )((char )hdr - sizeof(struct,
// update checksum
    pub CSUM_MANGLED_0: uhdr->check =,
    pub CHECKSUM_NONE: skb->ip_summed =,
//
// ptp_msg_is_sync - Evaluates whether the given skb is a PTP Sync message
// @skb: packet buffer
// @type: type of the packet (see ptp_classify_raw())
//
// This function evaluates whether the given skb is a PTP Sync message.
//
// Return: true if sync message, false otherwise
//
    pub type): *mut *mut bool ptp_msg_is_sync(struct sk_buff skb, unsigned int,
    pub ptp_classifier_init(void): void __init,

    pub PTP_CLASS_NONE: return,
    pub NULL: return,
// The return is meaningless. The stub function would not be
// executed since no available header from ptp_parse_header.
//
    pub PTP_MSGTYPE_SYNC: return,
    pub false: return,

