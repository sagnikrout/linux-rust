//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/msgr.h
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
// Data types for message passing layer used by Ceph.
//

//
// tcp connection banner.  include a protocol version. and adjust
// whenever the wire protocol changes.  try to keep this string length
// constant.
//

pub const CEPH_BANNER_LEN: c_int = 9;
pub const CEPH_BANNER_MAX_LEN: c_int = 30;
//
// messenger V2 connection banner prefix.
// The full banner string should have the form: "ceph v2\n<le16>"
// the 2 bytes are the length of the remaining banner.
//

pub const CEPH_BANNER_V2_LEN: c_int = 8;

//
// messenger V2 features
//

//
// Rollover-safe type and comparator for 32-bit sequence numbers.
// Comparator returns -1, 0, or 1.
//
pub type ceph_seq_t = __u32;
//
// entity_name -- logical name for a process participating in the
// network, e.g. 'mds0' or 'osd3'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_entity_name {
    pub /: *mut *mut *mut __u8 type; / CEPH_ENTITY_TYPE_,
    pub num: __le64,
// C attribute field omitted
pub const CEPH_ENTITY_TYPE_MON: c_uint = 0x01;
pub const CEPH_ENTITY_TYPE_MDS: c_uint = 0x02;
pub const CEPH_ENTITY_TYPE_OSD: c_uint = 0x04;
pub const CEPH_ENTITY_TYPE_CLIENT: c_uint = 0x08;
pub const CEPH_ENTITY_TYPE_AUTH: c_uint = 0x20;
pub const CEPH_ENTITY_TYPE_ANY: c_uint = 0xFF;
    pub type): *const *const extern char ceph_entity_type_name(int,
//
// entity_addr -- network address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_entity_addr {
    pub /: *mut *mut *mut __le32 type; / CEPH_ENTITY_ADDR_TYPE_,
    pub /: *mut *mut __le32 nonce; / unique id for process (e.g. pid),
    pub in_addr: sockaddr_storage,
// C attribute field omitted
    pub rhs->nonce: lhs->nonce ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_entity_inst {
    pub name: ceph_entity_name,
    pub addr: ceph_entity_addr,
// C attribute field omitted
// used by message exchange protocol

//
// connection negotiation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_connect {
    pub /: *mut *mut __le64 features; / supported feature bits,
    pub /: *mut *mut *mut __le32 host_type; / CEPH_ENTITY_TYPE_,
    pub /: *mut *mut __le32 global_seq; / count connections initiated by this host,
    pub /: *mut *mut __le32 connect_seq; / count connections initiated in this session,
    pub protocol_version: __le32,
    pub authorizer_protocol: __le32,
    pub authorizer_len: __le32,
    pub /: *mut *mut *mut __u8 flags; / CEPH_MSG_CONNECT_,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_connect_reply {
    pub tag: __u8,
    pub /: *mut *mut __le64 features; / feature bits for this session,
    pub global_seq: __le32,
    pub connect_seq: __le32,
    pub protocol_version: __le32,
    pub authorizer_len: __le32,
    pub flags: __u8,
// C attribute field omitted

//
// message header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_header_old {
    pub /: *mut *mut __le64 seq; / message seq# for this session,
    pub /: *mut *mut __le64 tid; / transaction id,
    pub /: *mut *mut __le16 type; / message type,
    pub /: *mut *mut __le16 priority; / priority. higher value == higher priority,
    pub /: *mut *mut __le16 version; / version of message encoding,
    pub /: *mut *mut __le32 front_len; / bytes in main payload,
    pub /: *mut *mut __le32 middle_len;/ bytes in middle payload,
    pub /: *mut *mut __le32 data_len; / bytes of data payload,
    pub offset: *mut *mut __le16 data_off; / sender: include full,
    pub orig_src: ceph_entity_inst src,,
    pub reserved: __le32,
    pub /: *mut *mut __le32 crc; / header crc32c,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_header {
    pub /: *mut *mut __le64 seq; / message seq# for this session,
    pub /: *mut *mut __le64 tid; / transaction id,
    pub /: *mut *mut __le16 type; / message type,
    pub /: *mut *mut __le16 priority; / priority. higher value == higher priority,
    pub /: *mut *mut __le16 version; / version of message encoding,
    pub /: *mut *mut __le32 front_len; / bytes in main payload,
    pub /: *mut *mut __le32 middle_len;/ bytes in middle payload,
    pub /: *mut *mut __le32 data_len; / bytes of data payload,
    pub offset: *mut *mut __le16 data_off; / sender: include full,
    pub src: ceph_entity_name,
    pub compat_version: __le16,
    pub reserved: __le16,
    pub /: *mut *mut __le32 crc; / header crc32c,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_header2 {
    pub /: *mut *mut __le64 seq; / message seq# for this session,
    pub /: *mut *mut __le64 tid; / transaction id,
    pub /: *mut *mut __le16 type; / message type,
    pub /: *mut *mut __le16 priority; / priority. higher value == higher priority,
    pub /: *mut *mut __le16 version; / version of message encoding,
    pub data_pre_padding_len: __le32,
    pub offset: *mut *mut __le16 data_off; / sender: include full,
    pub ack_seq: __le64,
    pub flags: __u8,
// oldest code we think can decode this.  unknown if zero.
    pub compat_version: __le16,
    pub reserved: __le16,
// C attribute field omitted
pub const CEPH_MSG_PRIO_LOW: c_int = 64;
pub const CEPH_MSG_PRIO_DEFAULT: c_int = 127;
pub const CEPH_MSG_PRIO_HIGH: c_int = 196;
pub const CEPH_MSG_PRIO_HIGHEST: c_int = 255;
//
// follows data payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_footer_old {
    pub data_crc: __le32 front_crc, middle_crc,,
    pub flags: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_footer {
    pub data_crc: __le32 front_crc, middle_crc,,
// sig holds the 64 bits of the digital signature for the message PLR
    pub sig: __le64,
    pub flags: __u8,
// C attribute field omitted

