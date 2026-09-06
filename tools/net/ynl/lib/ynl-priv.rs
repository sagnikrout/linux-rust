//! Automatically rewritten from C Header to Rust Module
//! Source: tools/net/ynl/lib/ynl-priv.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
pub const __YNL_C_PRIV_H: c_int = 1;

//
// YNL internals / low level stuff
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ynl_policy_type {
    YNL_PT_REJECT = 1,
    YNL_PT_IGNORE,
    YNL_PT_NEST,
    YNL_PT_FLAG,
    YNL_PT_BINARY,
    YNL_PT_U8,
    YNL_PT_U16,
    YNL_PT_U32,
    YNL_PT_U64,
    YNL_PT_UINT,
    YNL_PT_NUL_STR,
    YNL_PT_BITFIELD32,
    YNL_PT_SUBMSG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ynl_parse_result {
    YNL_PARSE_CB_ERROR = -1,
    YNL_PARSE_CB_STOP = 0,
    YNL_PARSE_CB_OK = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_policy_attr {
    pub type:8: ynl_policy_type,
    pub is_submsg:1: __u8,
    pub is_selector:1: __u8,
    pub selector_type: __u16,
    pub len: c_uint,
    pub name: *const c_char,
    pub nest: *const ynl_policy_nest,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_policy_nest {
    pub max_attr: c_uint,
    pub table: *const ynl_policy_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_parse_arg {
    pub ys: *mut ynl_sock,
    pub rsp_policy: *const ynl_policy_nest,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_dump_list_type {
    pub next: *mut ynl_dump_list_type,
    pub __attribute__((aligned(8))): unsigned char data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_ntf_base_type {
    pub family: __u16,
    pub cmd: __u8,
    pub next: *mut ynl_ntf_base_type,
    pub ntf): *mut *mut void (free)(struct ynl_ntf_base_type,
    pub __attribute__((aligned(8))): unsigned char data[],
}

// YNL specific helpers used by the auto-generated code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_req_state {
    pub yarg: ynl_parse_arg,
    pub cb: ynl_parse_cb_t,
    pub rsp_cmd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_dump_state {
    pub yarg: ynl_parse_arg,
    pub first: *mut c_void,
    pub last: *mut ynl_dump_list_type,
    pub alloc_sz: usize,
    pub cb: ynl_parse_cb_t,
    pub rsp_cmd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ynl_ntf_info {
    pub policy: *const ynl_policy_nest,
    pub cb: ynl_parse_cb_t,
    pub alloc_sz: usize,
    pub ntf): *mut *mut void (free)(struct ynl_ntf_base_type,
}

extern "C" {
    pub fn ynl_error_unknown_notification(ys: *mut ynl_sock, cmd: __u8);
}
extern "C" {
    pub fn ynl_error_parse(yarg: *mut ynl_parse_arg, msg: *const c_char) -> c_int;
}
// Netlink message handling helpers
pub const YNL_MSG_OVERFLOW: c_int = 1;
// Netlink attribute helpers

extern "C" {
    pub fn ynl_attr_if_good(_arg: end, _arg: attr) -> return;
}
extern "C" {
    pub fn ynl_attr_if_good(len: *mut *mut (char )start +, _arg: attr) -> return;
}
// ynl_msg_start() stashed buffer length in nlmsg_pid.
// YNL_MSG_OVERFLOW is < NLMSG_HDRLEN, all subsequent checks
// are guaranteed to fail.
//
extern "C" {
    pub fn ynl_attr_get_u32(_arg: attr) -> return;
}
extern "C" {
    pub fn ynl_attr_get_u64(_arg: attr) -> return;
}
extern "C" {
    pub fn ynl_attr_get_s32(_arg: attr) -> return;
}
extern "C" {
    pub fn ynl_attr_get_s64(_arg: attr) -> return;
}
extern "C" {
    pub fn __ynl_attr_validate(_arg: yarg, _arg: attr, _arg: ynl_attr_type(attr)) -> return;
}
