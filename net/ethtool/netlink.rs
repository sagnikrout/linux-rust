//! Automatically rewritten from C Header to Rust Module
//! Source: net/ethtool/netlink.h
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


// SPDX-License-Identifier: GPL-2.0-only

extern "C" {
    pub fn ethnl_multicast(skb: *mut sk_buff, dev: *mut net_device) -> c_int;
}
//
// ethnl_strz_size() - calculate attribute length for fixed size string
// @s: ETH_GSTRING_LEN sized string (may not be null terminated)
//
// Return: total length of an attribute with null terminated string from @s
//
extern "C" {
    pub fn nla_total_size(_arg: strnlen(s, 1: ETH_GSTRING_LEN) +) -> return;
}
//
// ethnl_put_strz() - put string attribute with fixed size string
// @skb:      skb with the message
// @attrtype: attribute type
// @s:        ETH_GSTRING_LEN sized string (may not be null terminated)
//
// Puts an attribute with null terminated string from @s into the message.
//
// Return: 0 on success, negative error code on failure
//
// ethnl_update_u32() - update u32 value from NLA_U32 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Copy the u32 value from NLA_U32 netlink attribute @attr into variable
// pointed to by @dst; do nothing if @attr is null. Bool pointed to by @mod
// is set to true if this function changed the value of *dst, otherwise it
// is left as is.
//
// dst = val;
// mod = true;
//
// ethnl_update_u8() - update u8 value from NLA_U8 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Copy the u8 value from NLA_U8 netlink attribute @attr into variable
// pointed to by @dst; do nothing if @attr is null. Bool pointed to by @mod
// is set to true if this function changed the value of *dst, otherwise it
// is left as is.
//
// dst = val;
// mod = true;
//
// ethnl_update_u8_u32() - update u8 value from an NLA_U32 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Some attributes are NLA_U32 on the wire but are stored in a u8. Read the
// full 32-bit value from NLA_U32 netlink attribute @attr and narrow it into
// the u8 pointed to by @dst; do nothing if @attr is null.
// Bool pointed to by @mod is set to true if this function changed the value
// of *dst, otherwise it is left as is.
//
// dst = val;
// mod = true;
//
// ethnl_update_bool32() - update u32 used as bool from NLA_U8 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Use the u8 value from NLA_U8 netlink attribute @attr to set u32 variable
// pointed to by @dst to 0 (if zero) or 1 (if not); do nothing if @attr is
// null. Bool pointed to by @mod is set to true if this function changed the
// logical value of *dst, otherwise it is left as is.
//
// dst = val;
// mod = true;
//
// ethnl_update_bool() - updateb bool used as bool from NLA_U8 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Use the bool value from NLA_U8 netlink attribute @attr to set bool variable
// pointed to by @dst to 0 (if zero) or 1 (if not); do nothing if @attr is
// null. Bool pointed to by @mod is set to true if this function changed the
// logical value of *dst, otherwise it is left as is.
//
// dst = val;
// mod = true;
//
// ethnl_update_binary() - update binary data from NLA_BINARY attribute
// @dst:  value to update
// @len:  destination buffer length
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Use the u8 value from NLA_U8 netlink attribute @attr to rewrite data block
// of length @len at @dst by attribute payload; do nothing if @attr is null.
// Bool pointed to by @mod is set to true if this function changed the logical
// value of *dst, otherwise it is left as is.
//
// mod = true;
//
// ethnl_update_bitfield32() - update u32 value from NLA_BITFIELD32 attribute
// @dst:  value to update
// @attr: netlink attribute with new value or null
// @mod:  pointer to bool for modification tracking
//
// Update bits in u32 value which are set in attribute's mask to values from
// attribute's value. Do nothing if @attr is null or the value wouldn't change;
// otherwise, set bool pointed to by @mod to true.
//
// dst = newval;
// mod = true;
//
// ethnl_reply_header_size() - total size of reply header
//
// This is an upper estimate so that we do not need to hold RTNL lock longer
// than necessary (to prevent rename between size estimate and composing the
// message). Accounts only for device ifindex and name as those are the only
// attributes ethnl_fill_reply_header() puts into the reply header.
//
// GET request handling
// Unified processing of GET requests uses two data structures: request info
// and reply data. Request info holds information parsed from client request
// and its stays constant through all request processing. Reply data holds data
// retrieved from ethtool_ops callbacks or other internal sources which is used
// to compose the reply. When processing a dump request, request info is filled
// only once (when the request message is parsed) but reply data is filled for
// each reply message.
//
// Both structures consist of part common for all request types (struct
// ethnl_req_info and struct ethnl_reply_data defined below) and optional
// parts specific for each request type. Common part always starts at offset 0.
//
// struct ethnl_req_info - base type of request information for GET requests
// @dev:   network device the request is for (may be null)
// @dev_tracker: refcount tracker for @dev reference
// @flags: request flags common for all request types
// @phy_index: phy_device index connected to @dev this request is for. Can be
// 0 if the request doesn't target a phy, or if the @dev's attached
// phy is targeted.
//
// This is a common base for request specific structures holding data from
// parsed userspace request. These always embed struct ethnl_req_info at
// zero offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_req_info {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub flags: u32,
    pub phy_index: u32,
}

//
// ethnl_req_get_phydev() - Gets the phy_device targeted by this request,
// if any.
// @req_info:	The ethnl request to get the phy from.
// @tb:		The netlink attributes array, for error reporting.
// @header:	The netlink header index, used for error reporting.
// @extack:	The netlink extended ACK, for error reporting.
//
// If a phy_device is returned the caller must hold rtnl_lock when calling
// this function, and until it's done interacting with the returned phy_device.
// IOW caller must hold rtnl_lock unless they know netdev has no phy_device.
//
// Return: A phy_device pointer corresponding either to the passed phy_index
// if one is provided. If not, the phy_device attached to the
// net_device targeted by this request is returned. If there's no
// targeted net_device, or no phy_device is attached, NULL is
// returned. If the provided phy_index is invalid, an error pointer
// is returned.
//
// struct ethnl_reply_data - base type of reply data for GET requests
// @dev:       device for current reply message; in single shot requests it is
// equal to &ethnl_req_info.dev; in dumps it's different for each
// reply message
//
// This is a common base for request specific structures holding data for
// kernel reply message. These always embed struct ethnl_reply_data at zero
// offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_reply_data {
    pub dev: *mut net_device,
}

extern "C" {
    pub fn ethnl_ops_begin(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ethnl_ops_complete(dev: *mut net_device);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethnl_sock_type {
    ETHTOOL_SOCK_TYPE_MODULE_FW_FLASH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_sock_priv {
    pub net: *mut net,
    pub portid: u32,
    pub type: ethnl_sock_type,
}

//
// struct ethnl_request_ops - unified handling of GET and SET requests
// @request_cmd:      command id for request (GET)
// @reply_cmd:        command id for reply (GET_REPLY)
// @hdr_attr:         attribute type for request header
// @req_info_size:    size of request info
// @reply_data_size:  size of reply data
// @allow_nodev_do:
// Allow non-dump request with no device identification.
// Note that locks (rtnl_lock etc.) are only taken if device is set.
// @set_ntf_cmd:      notification to generate on changes (SET)
// @parse_request:
// Parse request except common header (struct ethnl_req_info). Common
// header is already filled on entry, the rest up to @repdata_offset
// is zero initialized. This callback should only modify type specific
// request info by parsed attributes from request message.
// Called for both GET and SET. Information parsed for SET will
// be conveyed to the req_info used during NTF generation.
// @prepare_data:
// Retrieve and prepare data needed to compose a reply message. Calls to
// ethtool_ops handlers are limited to this callback. Common reply data
// (struct ethnl_reply_data) is filled on entry, type specific part after
// it is zero initialized. This callback should only modify the type
// specific part of reply data. Device identification from struct
// ethnl_reply_data is to be used as for dump requests, it iterates
// through network devices while dev member of struct ethnl_req_info
// points to the device from client request.
// @reply_size:
// Estimate reply message size. Returned value must be sufficient for
// message payload without common reply header. The callback may returned
// estimate higher than actual message size if exact calculation would
// not be worth the saved memory space.
// @fill_reply:
// Fill reply message payload (except for common header) from reply data.
// The callback must not generate more payload than previously called
// ->reply_size() estimated.
// @cleanup_data:
// Optional cleanup called when reply data is no longer needed. Can be
// used e.g. to free any additional data structures outside the main
// structure which were allocated by ->prepare_data(). When processing
// dump requests, ->cleanup() is called for each message.
// @set_validate:
// Check if set operation is supported for a given device, and perform
// extra input checks. Expected return values:
// - 0 if the operation is a noop for the device (rare)
// - 1 if operation should proceed to calling @set
// - negative errno on errors
// Called without any locks, just a reference on the netdev.
// @set:
// Execute the set operation. The implementation should return
// - 0 if no configuration has changed
// - 1 if configuration changed and notification should be generated
// - negative errno on errors
//
// Description of variable parts of GET request handling when using the
// unified infrastructure. When used, a pointer to an instance of this
// structure is to be added to &ethnl_default_requests array and generic
// handlers ethnl_default_doit(), ethnl_default_dumpit(),
// ethnl_default_start() and ethnl_default_done() used in @ethtool_genl_ops;
// ethnl_default_notify() can be used in @ethnl_notify_handlers to send
// notifications of the corresponding type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_request_ops {
    pub request_cmd: u8,
    pub reply_cmd: u8,
    pub hdr_attr: u16,
    pub req_info_size: c_uint,
    pub reply_data_size: c_uint,
    pub allow_nodev_do: bool,
    pub set_ntf_cmd: u8,
    pub extack): *mut netlink_ext_ack,
    pub info): *const genl_info,
    pub reply_data): *const ethnl_reply_data,
    pub reply_data): *const ethnl_reply_data,
    pub reply_data): *mut *mut void (cleanup_data)(struct ethnl_reply_data,
    pub info): *mut genl_info,
    pub info): *mut genl_info,
}

// request handlers
extern "C" {
    pub fn ethnl_set_features(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_act_cable_test(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_act_cable_test_tdr(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_tunnel_info_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_tunnel_info_start(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_tunnel_info_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_act_module_fw_flash(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_rss_dump_start(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_rss_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_tsinfo_start(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_tsinfo_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_tsinfo_done(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ethnl_rss_create_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ethnl_rss_delete_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
