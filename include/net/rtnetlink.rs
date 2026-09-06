//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/rtnetlink.h
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

extern "C" {
    pub fn int(: *mut *mut rtnl_dumpit_func)(struct sk_buff, : *mut netlink_callback) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtnl_link_flags {
    RTNL_FLAG_DOIT_UNLOCKED		= BIT(0),

    RTNL_FLAG_BULK_DEL_SUPPORTED	= BIT(1),
    RTNL_FLAG_DUMP_UNLOCKED		= BIT(2),
    RTNL_FLAG_DUMP_SPLIT_NLM_DONE	= BIT(3),	/* legacy behavior */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtnl_kinds {
    RTNL_KIND_NEW,
    RTNL_KIND_DEL,
    RTNL_KIND_GET,
    RTNL_KIND_SET
}

pub const RTNL_KIND_MASK: c_uint = 0x3;
//
// struct rtnl_msg_handler - rtnetlink message type and handlers
//
// @owner: NULL for built-in, THIS_MODULE for module
// @protocol: Protocol family or PF_UNSPEC
// @msgtype: rtnetlink message type
// @doit: Function pointer called for each request message
// @dumpit: Function pointer called for each dump request (NLM_F_DUMP) message
// @flags: rtnl_link_flags to modify behaviour of doit/dumpit functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_msg_handler {
    pub owner: *mut module,
    pub protocol: c_int,
    pub msgtype: c_int,
    pub doit: rtnl_doit_func,
    pub dumpit: rtnl_dumpit_func,
    pub flags: c_int,
}

extern "C" {
    pub fn rtnl_unregister_all(protocol: c_int);
}
extern "C" {
    pub fn __rtnl_register_many(handlers: *const rtnl_msg_handler, n: c_int) -> c_int;
}
extern "C" {
    pub fn __rtnl_unregister_many(handlers: *const rtnl_msg_handler, n: c_int);
}

//
// struct rtnl_newlink_params - parameters of rtnl_link_ops::newlink()
//
// @src_net: Source netns of rtnetlink socket
// @link_net: Link netns by IFLA_LINK_NETNSID, NULL if not specified
// @peer_net: Peer netns
// @tb: IFLA_* attributes
// @data: IFLA_INFO_DATA attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_newlink_params {
    pub src_net: *mut net,
    pub link_net: *mut net,
    pub peer_net: *mut net,
    pub tb: *mut nlattr,
    pub data: *mut nlattr,
}

// Get effective link netns from newlink params. Generally, this is link_net
// and falls back to src_net. But for compatibility, a driver may * choose to
// use dev_net(dev) instead.
//
// Get peer netns from newlink params. Fallback to link netns if peer netns is
// not specified explicitly.
//
// struct rtnl_link_ops - rtnetlink link operations
//
// @list: Used internally, protected by link_ops_mutex and SRCU
// @srcu: Used internally
// @kind: Identifier
// @netns_refund: Physical device, move to init_net on netns exit
// @peer_type: Peer device specific netlink attribute number (e.g. VETH_INFO_PEER)
// @maxtype: Highest device specific netlink attribute number
// @policy: Netlink policy for device specific attribute validation
// @validate: Optional validation function for netlink/changelink parameters
// @alloc: netdev allocation function, can be %NULL and is then used
// in place of alloc_netdev_mqs(), in this case @priv_size
// and @setup are unused. Returns a netdev or ERR_PTR().
// @priv_size: sizeof net_device private space
// @setup: net_device setup function
// @newlink: Function for configuring and registering a new device
// @changelink: Function for changing parameters of an existing device
// @dellink: Function to remove a device
// @get_size: Function to calculate required room for dumping device
// specific netlink attributes
// @fill_info: Function to dump device specific netlink attributes
// @get_xstats_size: Function to calculate required room for dumping device
// specific statistics
// @fill_xstats: Function to dump device specific statistics
// @get_num_tx_queues: Function to determine number of transmit queues
// to create when creating a new device.
// @get_num_rx_queues: Function to determine number of receive queues
// to create when creating a new device.
// @get_link_net: Function to get the i/o netns of the device
// @get_linkxstats_size: Function to calculate the required room for
// dumping device-specific extended link stats
// @fill_linkxstats: Function to dump device-specific extended link stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_link_ops {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub kind: *const c_char,
    pub priv_size: usize,
    pub num_rx_queues): c_uint,
    pub dev): *mut *mut void (setup)(struct net_device,
    pub netns_refund: bool,
    pub peer_type: u16,
    pub maxtype: c_uint,
    pub policy: *const nla_policy,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub head): *mut list_head,
    pub dev): *const *const size_t (get_size)(struct net_device,
    pub dev): *const net_device,
    pub dev): *const *const size_t (get_xstats_size)(struct net_device,
    pub dev): *const net_device,
    pub (*get_num_tx_queues)(void): *mut c_uint,
    pub (*get_num_rx_queues)(void): *mut c_uint,
    pub slave_maxtype: c_uint,
    pub slave_policy: *const nla_policy,
    pub extack): *mut netlink_ext_ack,
    pub slave_dev): *const net_device,
    pub slave_dev): *const net_device,
    pub dev): *const *const *const net (get_link_net)(net_device,
    pub attr): c_int,
    pub attr): *mut *mut int prividx, int,
}

extern "C" {
    pub fn rtnl_link_register(ops: *mut rtnl_link_ops) -> c_int;
}
extern "C" {
    pub fn rtnl_link_unregister(ops: *mut rtnl_link_ops);
}
//
// struct rtnl_af_ops - rtnetlink address family operations
//
// @list: Used internally, protected by RTNL and SRCU
// @srcu: Used internally
// @family: Address family
// @fill_link_af: Function to fill IFLA_AF_SPEC with address family
// specific netlink attributes.
// @get_link_af_size: Function to calculate size of address family specific
// netlink attributes.
// @validate_link_af: Validate a IFLA_AF_SPEC attribute, must check attr
// for invalid configuration settings.
// @set_link_af: Function to parse a IFLA_AF_SPEC attribute and modify
// net_device accordingly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_af_ops {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub family: c_int,
    pub ext_filter_mask): u32,
    pub ext_filter_mask): u32,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub dev): *const net_device,
    pub dev): *const *const size_t (get_stats_af_size)(struct net_device,
}

extern "C" {
    pub fn rtnl_af_register(ops: *mut rtnl_af_ops) -> c_int;
}
extern "C" {
    pub fn rtnl_af_unregister(ops: *mut rtnl_af_ops);
}
extern "C" {
    pub fn rtnl_delete_link(dev: *mut net_device, portid: u32, nlh: *const nlmsghdr) -> c_int;
}

