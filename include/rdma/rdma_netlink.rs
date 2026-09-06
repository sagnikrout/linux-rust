//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdma_netlink.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_nl_cbs {
    pub extack): *mut netlink_ext_ack,
    pub nlcb): *mut *mut *mut int (dump)(struct sk_buff skb, struct netlink_callback,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_flags {
// Require CAP_NET_ADMIN
    RDMA_NL_ADMIN_PERM	= 1 << 0,
}

// Define this module as providing netlink services for NETLINK_RDMA, with
// index _index.  Since the client indexes were setup in a uapi header as an
// enum and we do no want to change that, the user must supply the expanded
// constant as well and the compiler checks they are the same.
//

//
// Register client in RDMA netlink.
// @index: Index of the added client
// @cb_table: A table for op->callback
//
// Remove a client from IB netlink.
// @index: Index of the removed IB client.
//
extern "C" {
    pub fn rdma_nl_unregister(index: c_uint);
}
//
// Put a new message in a supplied skb.
// @skb: The netlink skb.
// @nlh: Pointer to put the header of the new netlink message.
// @seq: The message sequence number.
// @len: The requested message length to allocate.
// @client: Calling IB netlink client.
// @op: message content op.
// Returns the allocated buffer on success and NULL on failure.
//
// Put a new attribute in a supplied skb.
// @skb: The netlink skb.
// @nlh: Header of the netlink message to append the attribute to.
// @len: The length of the attribute data.
// @data: The attribute data to put.
// @type: The attribute type.
// Returns the 0 and a negative error code on failure.
//
// Send the supplied skb to a specific userspace PID.
// @net: Net namespace in which to send the skb
// @skb: The netlink skb
// @pid: Userspace netlink process ID
// Returns 0 on success or a negative error code.
//
extern "C" {
    pub fn rdma_nl_unicast(net: *mut net, skb: *mut sk_buff, pid: u32) -> c_int;
}
//
// Send, with wait/1 retry, the supplied skb to a specific userspace PID.
// @net: Net namespace in which to send the skb
// @skb: The netlink skb
// @pid: Userspace netlink process ID
// Returns 0 on success or a negative error code.
//
extern "C" {
    pub fn rdma_nl_unicast_wait(net: *mut net, skb: *mut sk_buff, pid: __u32) -> c_int;
}
//
// Send the supplied skb to a netlink group.
// @net: Net namespace in which to send the skb
// @skb: The netlink skb
// @group: Netlink group ID
// @flags: allocation flags
// Returns 0 on success or a negative error code.
//
// Check if there are any listeners to the netlink group
// @group: the netlink group ID
// Returns true on success or false if no listeners.
//
extern "C" {
    pub fn rdma_nl_chk_listeners(group: c_uint) -> bool;
}
//
// Prepare and send an event message
// @ib: the IB device which triggered the event
// @port_num: the port number which triggered the event - 0 if unused
// @type: the event type
// Returns 0 on success or a negative error code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_link_ops {
    pub list: list_head,
    pub type: *const c_char,
    pub ndev): *const *const *const int (newlink)(char ibdev_name, struct net_device,
    pub dev): *mut *mut int (dellink)(struct ib_device,
}

extern "C" {
    pub fn rdma_link_register(ops: *mut rdma_link_ops);
}
extern "C" {
    pub fn rdma_link_unregister(ops: *mut rdma_link_ops);
}

