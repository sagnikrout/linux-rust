//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_tlv.h
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
// Copyright(c) 2013 - 2019 Intel Corporation.
// forward declaration

// Message / Argument header format
// 3			  2		      1			  0
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |	     Length	   | Flags |	      Type / ID		   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// The message header format described here is used for messages that are
// passed between the PF and the VF.  To allow for messages larger then
// mailbox size we will provide a message with the above header and it
// will be segmented and transported to the mailbox to the other side where
// it is reassembled.  It contains the following fields:
// Length: Length of the message in bytes excluding the message header
// Flags: TBD
// Type/ID: These will be the message/argument types we pass
//
// message data header
pub const FM10K_TLV_ID_SHIFT: c_int = 0;
pub const FM10K_TLV_ID_SIZE: c_int = 16;

pub const FM10K_TLV_FLAGS_SHIFT: c_int = 16;
pub const FM10K_TLV_FLAGS_MSG: c_uint = 0x1;
pub const FM10K_TLV_FLAGS_SIZE: c_int = 4;
pub const FM10K_TLV_LEN_SHIFT: c_int = 20;
pub const FM10K_TLV_LEN_SIZE: c_int = 12;

pub const FM10K_TLV_RESULTS_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_tlv_type {
    FM10K_TLV_NULL_STRING,
    FM10K_TLV_MAC_ADDR,
    FM10K_TLV_BOOL,
    FM10K_TLV_UNSIGNED,
    FM10K_TLV_SIGNED,
    FM10K_TLV_LE_STRUCT,
    FM10K_TLV_NESTED,
    FM10K_TLV_MAX_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_tlv_attr {
    pub id: c_uint,
    pub type: fm10k_tlv_type,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_msg_data {
    pub id: c_uint,
    pub attr: *const fm10k_tlv_attr,
    pub ): *mut fm10k_mbx_info,
}

extern "C" {
    pub fn fm10k_tlv_msg_init(: *mut u32, _arg: u16) -> i32;
}
extern "C" {
    pub fn fm10k_tlv_attr_put_mac_vlan(: *mut u32, _arg: u16, : *const u8, _arg: u16) -> i32;
}
extern "C" {
    pub fn fm10k_tlv_attr_get_mac_vlan(: *mut u32, : *mut u8, : *mut u16) -> i32;
}
extern "C" {
    pub fn fm10k_tlv_attr_put_bool(: *mut u32, _arg: u16) -> i32;
}
extern "C" {
    pub fn fm10k_tlv_attr_put_value(: *mut u32, _arg: u16, _arg: i64, _arg: u32) -> i32;
}

extern "C" {
    pub fn fm10k_tlv_attr_get_value(: *mut u32, : *mut c_void, _arg: u32) -> i32;
}

extern "C" {
    pub fn fm10k_tlv_attr_put_le_struct(: *mut u32, _arg: u16, : *const c_void, _arg: u32) -> i32;
}
extern "C" {
    pub fn fm10k_tlv_attr_get_le_struct(: *mut u32, : *mut c_void, _arg: u32) -> i32;
}
pub const FM10K_TLV_MSG_ID_TEST: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_tlv_test_attr_id {
    FM10K_TEST_MSG_UNSET,
    FM10K_TEST_MSG_STRING,
    FM10K_TEST_MSG_MAC_ADDR,
    FM10K_TEST_MSG_U8,
    FM10K_TEST_MSG_U16,
    FM10K_TEST_MSG_U32,
    FM10K_TEST_MSG_U64,
    FM10K_TEST_MSG_S8,
    FM10K_TEST_MSG_S16,
    FM10K_TEST_MSG_S32,
    FM10K_TEST_MSG_S64,
    FM10K_TEST_MSG_LE_STRUCT,
    FM10K_TEST_MSG_NESTED,
    FM10K_TEST_MSG_RESULT,
    FM10K_TEST_MSG_MAX
}

extern "C" {
    pub fn fm10k_tlv_msg_test_create(: *mut u32, _arg: u32);
}
extern "C" {
    pub fn fm10k_tlv_msg_test(: *mut fm10k_hw, : *mut u32, : *mut fm10k_mbx_info) -> i32;
}

