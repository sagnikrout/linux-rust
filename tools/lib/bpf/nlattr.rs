//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/nlattr.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// NETLINK      Netlink attributes
//
// Copyright (c) 2003-2013 Thomas Graf <tgraf@suug.ch>
//

// avoid multiple definition of netlink features
//
// Standard attribute types to specify validation policy
//

//
// @ingroup attr
// Attribute validation policy.
//
// See section @core_doc{core_attr_parse,Attribute Parsing} for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libbpf_nla_policy {
// Type of attribute or LIBBPF_NLA_UNSPEC
    pub type: u16,
// Minimal length of payload required
    pub minlen: u16,
// Maximal length of payload allowed
    pub maxlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libbpf_nla_req {
    pub nh: nlmsghdr,
    pub ifinfo: ifinfomsg,
    pub tc: tcmsg,
    pub gnl: genlmsghdr,
}

//
// @ingroup attr
// Iterate over a stream of attributes
// @arg pos	loop counter, set to current attribute
// @arg head	head of attribute stream
// @arg len	length of attribute stream
// @arg rem	initialized to len, holds bytes currently remaining in stream
//

//
// libbpf_nla_data - head of payload
// @nla: netlink attribute
//
// libbpf_nla_len - length of payload
// @nla: netlink attribute
//
extern "C" {
    pub fn libbpf_nla_dump_errormsg(nlh: *mut nlmsghdr) -> c_int;
}
