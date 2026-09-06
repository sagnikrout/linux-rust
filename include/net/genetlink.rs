//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/genetlink.h
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

// Non-parallel generic netlink requests are serialized by a global lock.
extern "C" {
    pub fn genl_lock();
}
extern "C" {
    pub fn genl_unlock();
}

// Binding to multicast group requires %CAP_NET_ADMIN

// Binding to multicast group requires %CAP_SYS_ADMIN

//
// struct genl_multicast_group - generic netlink multicast group
// @name: name of the multicast group, names are per-family
// @flags: GENL_MCAST_* flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_multicast_group {
    pub name: [c_char; GENL_NAMSIZ],
    pub flags: u8,
}

//
// struct genl_family - generic netlink family
// @hdrsize: length of user specific header in bytes
// @name: name of family
// @version: protocol version
// @maxattr: maximum number of attributes supported
// @policy: netlink policy
// @netnsok: set to true if the family can handle network
// namespaces and should be presented in all of them
// @parallel_ops: operations can be called in parallel and aren't
// synchronized by the core genetlink code
// @pre_doit: called before an operation's doit callback, it may
// do additional, common, filtering and return an error
// @post_doit: called after an operation's doit callback, it may
// undo operations done by pre_doit, for example release locks
// @bind: called when family multicast group is added to a netlink socket
// @unbind: called when family multicast group is removed from a netlink socket
// @module: pointer to the owning module (set to THIS_MODULE)
// @mcgrps: multicast groups used by this family
// @n_mcgrps: number of multicast groups
// @resv_start_op: first operation for which reserved fields of the header
// can be validated and policies are required (see below);
// new families should leave this field at zero
// @ops: the operations supported by this family
// @n_ops: number of operations supported by this family
// @small_ops: the small-struct operations supported by this family
// @n_small_ops: number of small-struct operations supported by this family
// @split_ops: the split do/dump form of operation definition
// @n_split_ops: number of entries in @split_ops, note that with split do/dump
// ops the number of entries is not the same as number of commands
// @sock_priv_size: the size of per-socket private memory
// @sock_priv_init: the per-socket private memory initializer
// @sock_priv_destroy: the per-socket private memory destructor
//
// Attribute policies (the combination of @policy and @maxattr fields)
// can be attached at the family level or at the operation level.
// If both are present the per-operation policy takes precedence.
// For operations before @resv_start_op lack of policy means that the core
// will perform no attribute parsing or validation. For newer operations
// if policy is not provided core will reject all TLV attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_family {
    pub hdrsize: c_uint,
    pub name: [c_char; GENL_NAMSIZ],
    pub version: c_uint,
    pub maxattr: c_uint,
    pub netnsok:1: u8,
    pub parallel_ops:1: u8,
    pub n_ops: u8,
    pub n_small_ops: u8,
    pub n_split_ops: u8,
    pub n_mcgrps: u8,
    pub resv_start_op: u8,
    pub policy: *const nla_policy,
    pub info): *mut genl_info,
    pub info): *mut genl_info,
    pub mcgrp): *mut *mut int (bind)(int,
    pub mcgrp): *mut *mut void (unbind)(int,
    pub ops: *const *const genl_ops,
    pub small_ops: *const genl_small_ops,
    pub split_ops: *const genl_split_ops,
    pub mcgrps: *const genl_multicast_group,
    pub module: *mut module,
    pub sock_priv_size: usize,
    pub priv): *mut *mut void (sock_priv_init)(void,
    pub priv): *mut *mut void (sock_priv_destroy)(void,
// private: internal use only
// protocol family identifier
    pub id: c_int,
// starting number of multicast group IDs in this family
    pub mcgrp_offset: c_uint,
// list of per-socket privs
    pub sock_privs: *mut xarray,
}

//
// struct genl_info - receiving information
// @snd_seq: sending sequence number
// @snd_portid: netlink portid of sender
// @family: generic netlink family
// @nlhdr: netlink message header
// @genlhdr: generic netlink message header
// @attrs: netlink attributes
// @_net: network namespace
// @ctx: storage space for the use by the family
// @user_ptr: user pointers (deprecated, use ctx instead)
// @extack: extended ACK report struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_info {
    pub snd_seq: u32,
    pub snd_portid: u32,
    pub family: *const genl_family,
    pub nlhdr: *const *const nlmsghdr,
    pub genlhdr: *mut *mut genlmsghdr,
    pub attrs: *mut *mut *mut nlattr,
    pub _net: possible_net_t,
    pub ctx: [u8; NETLINK_CTX_SIZE],
    pub user_ptr: [*mut *mut c_void; 2],
}

extern "C" {
    pub fn read_pnet(_arg: &info->_net) -> return;
}

// Report that a root attribute is missing

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genl_validate_flags {
    GENL_DONT_VALIDATE_STRICT		= BIT(0),
    GENL_DONT_VALIDATE_DUMP			= BIT(1),
    GENL_DONT_VALIDATE_DUMP_STRICT		= BIT(2),
}

//
// struct genl_small_ops - generic netlink operations (small version)
// @cmd: command identifier
// @internal_flags: flags used by the family
// @flags: GENL_* flags (%GENL_ADMIN_PERM or %GENL_UNS_ADMIN_PERM)
// @validate: validation flags from enum genl_validate_flags
// @doit: standard command callback
// @dumpit: callback for dumpers
//
// This is a cut-down version of struct genl_ops for users who don't need
// most of the ancillary infra and want to save space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_small_ops {
    pub info): *mut *mut *mut int (doit)(struct sk_buff skb, struct genl_info,
    pub cb): *mut *mut *mut int (dumpit)(struct sk_buff skb, struct netlink_callback,
    pub cmd: u8,
    pub internal_flags: u8,
    pub flags: u8,
    pub validate: u8,
}

//
// struct genl_ops - generic netlink operations
// @cmd: command identifier
// @internal_flags: flags used by the family
// @flags: GENL_* flags (%GENL_ADMIN_PERM or %GENL_UNS_ADMIN_PERM)
// @maxattr: maximum number of attributes supported
// @policy: netlink policy (takes precedence over family policy)
// @validate: validation flags from enum genl_validate_flags
// @doit: standard command callback
// @start: start callback for dumps
// @dumpit: callback for dumpers
// @done: completion callback for dumps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_ops {
    pub info): *mut genl_info,
    pub cb): *mut *mut int (start)(struct netlink_callback,
    pub cb): *mut netlink_callback,
    pub cb): *mut *mut int (done)(struct netlink_callback,
    pub policy: *const nla_policy,
    pub maxattr: c_uint,
    pub cmd: u8,
    pub internal_flags: u8,
    pub flags: u8,
    pub validate: u8,
}

//
// struct genl_split_ops - generic netlink operations (do/dump split version)
// @cmd: command identifier
// @internal_flags: flags used by the family
// @flags: GENL_* flags (%GENL_ADMIN_PERM or %GENL_UNS_ADMIN_PERM)
// @validate: validation flags from enum genl_validate_flags
// @policy: netlink policy (takes precedence over family policy)
// @maxattr: maximum number of attributes supported
//
// Do callbacks:
// @pre_doit: called before an operation's @doit callback, it may
// do additional, common, filtering and return an error
// @doit: standard command callback
// @post_doit: called after an operation's @doit callback, it may
// undo operations done by pre_doit, for example release locks
//
// Dump callbacks:
// @start: start callback for dumps
// @dumpit: callback for dumpers
// @done: completion callback for dumps
//
// Do callbacks can be used if %GENL_CMD_CAP_DO is set in @flags.
// Dump callbacks can be used if %GENL_CMD_CAP_DUMP is set in @flags.
// Exactly one of those flags must be set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_split_ops {
    pub info): *mut genl_info,
    pub info): *mut genl_info,
    pub info): *mut genl_info,
}

//
// struct genl_dumpit_info - info that is available during dumpit op call
// @op: generic netlink ops - for internal genl code usage
// @attrs: netlink attributes
// @info: struct genl_info describing the request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genl_dumpit_info {
    pub op: genl_split_ops,
    pub info: genl_info,
}

//
// genl_info_init_ntf() - initialize genl_info for notifications
// @info:   genl_info struct to set up
// @family: pointer to the genetlink family
// @cmd:    command to be used in the notification
//
// Initialize a locally declared struct genl_info to pass to various APIs.
// Intended to be used when creating notifications.
//
extern "C" {
    pub fn genl_register_family(family: *mut genl_family) -> c_int;
}
extern "C" {
    pub fn genl_unregister_family(family: *const genl_family) -> c_int;
}
//
// genlmsg_iput - start genetlink message based on genl_info
// @skb: skb in which message header will be placed
// @info: genl_info as provided to do/dump handlers
//
// Convenience wrapper which starts a genetlink message based on
// information in user request. @info should be either the struct passed
// by genetlink core to do/dump handlers (when constructing replies to
// such requests) or a struct initialized by genl_info_init_ntf()
// when constructing notifications.
//
// Returns: pointer to new genetlink header.
//
extern "C" {
    pub fn __genlmsg_iput(_arg: skb, _arg: info, _arg: 0) -> return;
}
//
// genlmsg_nlhdr - Obtain netlink header from user specified header
// @user_hdr: user header as returned from genlmsg_put()
//
// Returns: pointer to netlink header.
//
// genlmsg_parse_deprecated - parse attributes of a genetlink message
// @nlh: netlink message header
// @family: genetlink message family
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// genlmsg_parse - parse attributes of a genetlink message
// @nlh: netlink message header
// @family: genetlink message family
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// genl_dump_check_consistent - check if sequence is consistent and advertise if not
// @cb: netlink callback structure that stores the sequence number
// @user_hdr: user header as returned from genlmsg_put()
//
// Cf. nl_dump_check_consistent(), this just provides a wrapper to make it
// simpler to use with generic netlink.
//
// genlmsg_put_reply - Add generic netlink header to a reply message
// @skb: socket buffer holding the message
// @info: receiver info
// @family: generic netlink family
// @flags: netlink message flags
// @cmd: generic netlink command
//
// Returns: pointer to user specific header
//
// genlmsg_end - Finalize a generic netlink message
// @skb: socket buffer the message is stored in
// @hdr: user specific header
//
// genlmsg_cancel - Cancel construction of a generic netlink message
// @skb: socket buffer the message is stored in
// @hdr: generic netlink message header
//
// genlmsg_multicast_netns_filtered - multicast a netlink message
// to a specific netns with filter
// function
// @family: the generic netlink family
// @net: the net namespace
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: offset of multicast group in groups array
// @flags: allocation flags
// @filter: filter function
// @filter_data: filter function private data
//
// Return: 0 on success, negative error code for failure.
//
// genlmsg_multicast_netns - multicast a netlink message to a specific netns
// @family: the generic netlink family
// @net: the net namespace
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: offset of multicast group in groups array
// @flags: allocation flags
//
// genlmsg_multicast - multicast a netlink message to the default netns
// @family: the generic netlink family
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: offset of multicast group in groups array
// @flags: allocation flags
//
// genlmsg_multicast_allns - multicast a netlink message to all net namespaces
// @family: the generic netlink family
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: offset of multicast group in groups array
//
// This function must hold the RTNL or rcu_read_lock().
//
// genlmsg_unicast - unicast a netlink message
// @net: network namespace to look up @portid in
// @skb: netlink message as socket buffer
// @portid: netlink portid of the destination socket
//
extern "C" {
    pub fn nlmsg_unicast(_arg: net->genl_sock, _arg: skb, _arg: portid) -> return;
}
//
// genlmsg_reply - reply to a request
// @skb: netlink message to be sent back
// @info: receiver information
//
extern "C" {
    pub fn genlmsg_unicast(_arg: genl_info_net(info), _arg: skb, _arg: info->snd_portid) -> return;
}
//
// genlmsg_data - head of message payload
// @gnlh: genetlink message header
//
// genlmsg_len - length of message payload
// @gnlh: genetlink message header
//
// genlmsg_msg_size - length of genetlink message not including padding
// @payload: length of message payload
//
// genlmsg_total_size - length of genetlink message including padding
// @payload: length of message payload
//
extern "C" {
    pub fn NLMSG_ALIGN(_arg: genlmsg_msg_size(payload)) -> return;
}
//
// genlmsg_new - Allocate a new generic netlink message
// @payload: size of the message payload
// @flags: the type of memory to allocate.
//
extern "C" {
    pub fn nlmsg_new(_arg: genlmsg_total_size(payload), _arg: flags) -> return;
}
//
// genl_set_err - report error to genetlink broadcast listeners
// @family: the generic netlink family
// @net: the network namespace to report the error to
// @portid: the PORTID of a process that we want to skip (if any)
// @group: the broadcast group that will notice the error
// (this is the offset of the multicast group in the groups array)
// @code: error code, must be negative (as usual in kernelspace)
//
// This function returns the number of broadcast listeners that have set the
// NETLINK_RECV_NO_ENOBUFS socket option.
//
extern "C" {
    pub fn netlink_set_err(_arg: net->genl_sock, _arg: portid, _arg: group, _arg: code) -> return;
}
extern "C" {
    pub fn netlink_has_listeners(_arg: net->genl_sock, _arg: group) -> return;
}
