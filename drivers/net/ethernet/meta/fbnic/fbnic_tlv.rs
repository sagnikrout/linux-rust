//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_tlv.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// TLV Header Format
// 3			  2		      1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |		Length		   |M|I|RSV|	   Type / ID	   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// The TLV header format described above will be used for transferring
// messages between the host and the firmware. To ensure byte ordering
// we have defined all fields as being little endian.
// Type/ID: Identifier for message and/or attribute
// RSV: Reserved field for future use, likely as additional flags
// I: cannot_ignore flag, identifies if unrecognized attribute can be ignored
// M: is_msg, indicates that this is the start of a new message
// Length: Total length of message in dwords including header
// or
// Total length of attribute in bytes including header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tlv_hdr {

    pub /: *mut *mut u16 type : 12; / 0 .. 11 Type / ID,
    pub /: *mut *mut u16 rsvd : 2; / 12 .. 13 Reserved for future use,
    pub /: *mut *mut u16 cannot_ignore : 1; / 14 Attribute can be ignored,
    pub /: *mut *mut u16 is_msg : 1; / 15 Header belongs to message,

    pub /: *mut *mut u16 is_msg : 1; / 15 Header belongs to message,
    pub /: *mut *mut u16 cannot_ignore : 1; / 14 Attribute can be ignored,
    pub /: *mut *mut u16 rsvd : 2; / 13 .. 12 Reserved for future use,
    pub /: *mut *mut u16 type : 12; / 11 .. 0 Type / ID,

    pub /: *mut *mut __le16 len; / 16 .. 32 length including TLV header,
}

pub const FBNIC_TLV_RESULTS_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tlv_msg {
    pub hdr: fbnic_tlv_hdr,
    pub value: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_tlv_type {
    FBNIC_TLV_STRING,
    FBNIC_TLV_FLAG,
    FBNIC_TLV_UNSIGNED,
    FBNIC_TLV_SIGNED,
    FBNIC_TLV_BINARY,
    FBNIC_TLV_NESTED,
    FBNIC_TLV_ARRAY,
    __FBNIC_TLV_MAX_TYPE
}

// TLV Index
// Defines the relationship between the attribute IDs and their types.
// For each entry in the index there will be a size and type associated
// with it so that we can use this to parse the data and verify it matches
// the expected layout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tlv_index {
    pub id: u16,
    pub len: u16,
    pub type: fbnic_tlv_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tlv_parser {
    pub id: u16,
    pub attr: *const fbnic_tlv_index,
    pub results): *mut fbnic_tlv_msg,
}

extern "C" {
    pub fn fbnic_tlv_attr_get_unsigned(attr: *mut fbnic_tlv_msg, def: u64) -> u64;
}
extern "C" {
    pub fn fbnic_tlv_attr_get_signed(attr: *mut fbnic_tlv_msg, def: i64) -> i64;
}
extern "C" {
    pub fn fbnic_tlv_attr_put_flag(msg: *mut fbnic_tlv_msg, attr_id: u16) -> c_int;
}

extern "C" {
    pub fn fbnic_tlv_attr_nest_stop(msg: *mut fbnic_tlv_msg);
}
extern "C" {
    pub fn fbnic_tlv_attr_addr_copy(dest: *mut u8, src: *mut fbnic_tlv_msg);
}
extern "C" {
    pub fn fbnic_tlv_parser_error(opaque: *mut c_void, results: *mut fbnic_tlv_msg) -> c_int;
}

pub const FBNIC_TLV_MSG_ID_TEST: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_tlv_test_attr_id {
    FBNIC_TLV_TEST_MSG_U64,
    FBNIC_TLV_TEST_MSG_S64,
    FBNIC_TLV_TEST_MSG_U32,
    FBNIC_TLV_TEST_MSG_S32,
    FBNIC_TLV_TEST_MSG_U16,
    FBNIC_TLV_TEST_MSG_S16,
    FBNIC_TLV_TEST_MSG_MAC_ADDR,
    FBNIC_TLV_TEST_MSG_FLAG_TRUE,
    FBNIC_TLV_TEST_MSG_FLAG_FALSE,
    FBNIC_TLV_TEST_MSG_STRING,
    FBNIC_TLV_TEST_MSG_NESTED,
    FBNIC_TLV_TEST_MSG_ARRAY,
    FBNIC_TLV_TEST_MSG_MAX
}

extern "C" {
    pub fn fbnic_tlv_parser_test(opaque: *mut c_void, results: *mut fbnic_tlv_msg) -> c_int;
}

