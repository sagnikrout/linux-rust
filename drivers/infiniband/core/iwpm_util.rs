//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/iwpm_util.h
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
// Copyright (c) 2014 Intel Corporation. All rights reserved.
// Copyright (c) 2014 Chelsio, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const IWPM_NL_RETRANS: c_int = 3;

pub const IWPM_MAPINFO_SKB_COUNT: c_int = 20;

pub const IWPM_REG_UNDEF: c_uint = 0x01;
pub const IWPM_REG_VALID: c_uint = 0x02;
pub const IWPM_REG_INCOMPL: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_nlmsg_request {
    pub inprocess_list: list_head,
    pub nlmsg_seq: __u32,
    pub req_buffer: *mut c_void,
    pub nl_client: u8,
    pub request_done: u8,
    pub err_code: u16,
    pub sem: semaphore,
    pub kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_mapping_info {
    pub hlist_node: hlist_node,
    pub local_sockaddr: sockaddr_storage,
    pub mapped_sockaddr: sockaddr_storage,
    pub nl_client: u8,
    pub map_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_remote_info {
    pub hlist_node: hlist_node,
    pub remote_sockaddr: sockaddr_storage,
    pub mapped_loc_sockaddr: sockaddr_storage,
    pub mapped_rem_sockaddr: sockaddr_storage,
    pub nl_client: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwpm_admin_data {
    pub nlmsg_seq: core::sync::atomic::AtomicI32,
    pub reg_list: [u32; RDMA_NL_NUM_CLIENTS],
}

//
// iwpm_get_nlmsg_request - Allocate and initialize netlink message request
// @nlmsg_seq: Sequence number of the netlink message
// @nl_client: The index of the netlink client
// @gfp: Indicates how the memory for the request should be allocated
//
// Returns the newly allocated netlink request object if successful,
// otherwise returns NULL
//
// iwpm_free_nlmsg_request - Deallocate netlink message request
// @kref: Holds reference of netlink message request
//
extern "C" {
    pub fn iwpm_free_nlmsg_request(kref: *mut kref);
}
//
// iwpm_find_nlmsg_request - Find netlink message request in the request list
// @echo_seq: Sequence number of the netlink request to find
//
// Returns the found netlink message request,
// if not found, returns NULL
//
// iwpm_wait_complete_req - Block while servicing the netlink request
// @nlmsg_request: Netlink message request to service
//
// Wakes up, after the request is completed or expired
// Returns 0 if the request is complete without error
//
extern "C" {
    pub fn iwpm_wait_complete_req(nlmsg_request: *mut iwpm_nlmsg_request) -> c_int;
}
//
// iwpm_get_nlmsg_seq - Get the sequence number for a netlink
// message to send to the port mapper
//
// Returns the sequence number for the netlink message.
//
extern "C" {
    pub fn iwpm_get_nlmsg_seq() -> c_int;
}
//
// iwpm_add_remote_info - Add remote address info of the connecting peer
// to the remote info hash table
// @reminfo: The remote info to be added
//
extern "C" {
    pub fn iwpm_add_remote_info(reminfo: *mut iwpm_remote_info);
}
//
// iwpm_check_registration - Check if the client registration
// matches the given one
// @nl_client: The index of the netlink client
// @reg: The given registration type to compare with
//
// Call iwpm_register_pid() to register a client
// Returns true if the client registration matches reg,
// otherwise returns false
//
extern "C" {
    pub fn iwpm_check_registration(nl_client: u8, reg: u32) -> u32;
}
//
// iwpm_set_registration - Set the client registration
// @nl_client: The index of the netlink client
// @reg: Registration type to set
//
extern "C" {
    pub fn iwpm_set_registration(nl_client: u8, reg: u32);
}
//
// iwpm_get_registration - Get the client registration
// @nl_client: The index of the netlink client
//
// Returns the client registration type
//
extern "C" {
    pub fn iwpm_get_registration(nl_client: u8) -> u32;
}
//
// iwpm_send_mapinfo - Send local and mapped IPv4/IPv6 address info of
// a client to the user space port mapper
// @nl_client: The index of the netlink client
// @iwpm_pid: The pid of the user space port mapper
//
// If successful, returns the number of sent mapping info records
//
extern "C" {
    pub fn iwpm_send_mapinfo(nl_client: u8, iwpm_pid: c_int) -> c_int;
}
//
// iwpm_mapinfo_available - Check if any mapping info records is available
// in the hash table
//
// Returns 1 if mapping information is available, otherwise returns 0
//
extern "C" {
    pub fn iwpm_mapinfo_available() -> c_int;
}
//
// iwpm_compare_sockaddr - Compare two sockaddr storage structs
// @a_sockaddr: first sockaddr to compare
// @b_sockaddr: second sockaddr to compare
//
// Return: 0 if they are holding the same ip/tcp address info,
// otherwise returns 1
//
// iwpm_validate_nlmsg_attr - Check for NULL netlink attributes
// @nltb: Holds address of each netlink message attributes
// @nla_count: Number of netlink message attributes
//
// Returns error if any of the nla_count attributes is NULL
//
// iwpm_create_nlmsg - Allocate skb and form a netlink message
// @nl_op: Netlink message opcode
// @nlh: Holds address of the netlink message header in skb
// @nl_client: The index of the netlink client
//
// Returns the newly allcated skb, or NULL if the tailroom of the skb
// is insufficient to store the message header and payload
//
// iwpm_parse_nlmsg - Validate and parse the received netlink message
// @cb: Netlink callback structure
// @policy_max: Maximum attribute type to be expected
// @nlmsg_policy: Validation policy
// @nltb: Array to store policy_max parsed elements
// @msg_type: Type of netlink message
//
// Returns 0 on success or a negative error code
//
// iwpm_print_sockaddr - Print IPv4/IPv6 address and TCP port
// @sockaddr: Socket address to print
// @msg: Message to print
//
extern "C" {
    pub fn iwpm_print_sockaddr(sockaddr: *mut sockaddr_storage, msg: *mut c_char);
}
//
// iwpm_send_hello - Send hello response to iwpmd
//
// @nl_client: The index of the netlink client
// @iwpm_pid: The pid of the user space port mapper
// @abi_version: The kernel's abi_version
//
// Returns 0 on success or a negative error code
//
extern "C" {
    pub fn iwpm_send_hello(nl_client: u8, iwpm_pid: c_int, abi_version: u16) -> c_int;
}
