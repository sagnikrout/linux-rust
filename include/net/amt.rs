//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/amt.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2021 Taehee Yoo <ap420073@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_msg_type {
    AMT_MSG_DISCOVERY = 1,
    AMT_MSG_ADVERTISEMENT,
    AMT_MSG_REQUEST,
    AMT_MSG_MEMBERSHIP_QUERY,
    AMT_MSG_MEMBERSHIP_UPDATE,
    AMT_MSG_MULTICAST_DATA,
    AMT_MSG_TEARDOWN,
    __AMT_MSG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_ops {
// A*B
    AMT_OPS_INT,
// A+B
    AMT_OPS_UNI,
// A-B
    AMT_OPS_SUB,
// B-A
    AMT_OPS_SUB_REV,
    __AMT_OPS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_filter {
    AMT_FILTER_FWD,
    AMT_FILTER_D_FWD,
    AMT_FILTER_FWD_NEW,
    AMT_FILTER_D_FWD_NEW,
    AMT_FILTER_ALL,
    AMT_FILTER_NONE_NEW,
    AMT_FILTER_BOTH,
    AMT_FILTER_BOTH_NEW,
    __AMT_FILTER_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_act {
    AMT_ACT_GMI,
    AMT_ACT_GMI_ZERO,
    AMT_ACT_GT,
    AMT_ACT_STATUS_FWD_NEW,
    AMT_ACT_STATUS_D_FWD_NEW,
    AMT_ACT_STATUS_NONE_NEW,
    __AMT_ACT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_status {
    AMT_STATUS_INIT,
    AMT_STATUS_SENT_DISCOVERY,
    AMT_STATUS_RECEIVED_DISCOVERY,
    AMT_STATUS_SENT_ADVERTISEMENT,
    AMT_STATUS_RECEIVED_ADVERTISEMENT,
    AMT_STATUS_SENT_REQUEST,
    AMT_STATUS_RECEIVED_REQUEST,
    AMT_STATUS_SENT_QUERY,
    AMT_STATUS_RECEIVED_QUERY,
    AMT_STATUS_SENT_UPDATE,
    AMT_STATUS_RECEIVED_UPDATE,
    __AMT_STATUS_MAX,
}

// Gateway events only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_event {
    AMT_EVENT_NONE,
    AMT_EVENT_RECEIVE,
    AMT_EVENT_SEND_DISCOVERY,
    AMT_EVENT_SEND_REQUEST,
    __AMT_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header {

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_discovery {

    pub nonce: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_advertisement {

    pub nonce: __be32,
    pub ip4: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_request {

    pub nonce: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_membership_query {

    pub nonce: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_membership_update {

    pub nonce: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_header_mcast_data {

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_headers {
    pub discovery: amt_header_discovery,
    pub advertisement: amt_header_advertisement,
    pub request: amt_header_request,
    pub query: amt_header_membership_query,
    pub update: amt_header_membership_update,
    pub data: amt_header_mcast_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_gw_headers {
    pub discovery: amt_header_discovery,
    pub request: amt_header_request,
    pub update: amt_header_membership_update,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_relay_headers {
    pub advertisement: amt_header_advertisement,
    pub query: amt_header_membership_query,
    pub data: amt_header_mcast_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_skb_cb {
    pub tunnel: *mut amt_tunnel_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_tunnel_list {
    pub list: list_head,
// Protect All resources under an amt_tunne_list
    pub lock: spinlock_t,
    pub amt: *mut amt_dev,
    pub nr_groups: u32,
    pub nr_sources: u32,
    pub status: amt_status,
    pub gc_wq: delayed_work,
    pub source_port: __be16,
    pub ip4: __be32,
    pub nonce: __be32,
    pub key: siphash_key_t,
    pub rcu: rcu_head,
    pub groups: [hlist_head; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amt_addr {
    pub ip4: __be32,

    pub ip6: in6_addr,

}

// RFC 3810
//
// When the router is in EXCLUDE mode, the router state is represented
// by the notation EXCLUDE (X,Y), where X is called the "Requested List"
// and Y is called the "Exclude List".  All sources, except those from
// the Exclude List, will be forwarded by the router
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amt_source_status {
    AMT_SOURCE_STATUS_NONE,
// Node of Requested List
    AMT_SOURCE_STATUS_FWD,
// Node of Exclude List
    AMT_SOURCE_STATUS_D_FWD,
}

// protected by gnode->lock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_source_node {
    pub node: hlist_node,
    pub gnode: *mut amt_group_node,
    pub source_timer: delayed_work,
    pub source_addr: amt_addr,
    pub status: amt_source_status,
pub const AMT_SOURCE_OLD: c_int = 0;
pub const AMT_SOURCE_NEW: c_int = 1;
    pub flags: u8,
    pub rcu: rcu_head,
}

// Protected by amt_tunnel_list->lock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_group_node {
    pub amt: *mut amt_dev,
    pub group_addr: amt_addr,
    pub host_addr: amt_addr,
    pub v6: bool,
    pub filter_mode: u8,
    pub nr_sources: u32,
    pub tunnel_list: *mut amt_tunnel_list,
    pub node: hlist_node,
    pub group_timer: delayed_work,
    pub rcu: rcu_head,
    pub sources: [hlist_head; ],
}

pub const AMT_MAX_EVENTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_events {
    pub event: amt_event,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amt_dev {
    pub dev: *mut net_device,
    pub stream_dev: *mut net_device,
    pub net: *mut net,
// Global lock for amt device
    pub lock: spinlock_t,
// Used only in relay mode
    pub tunnel_list: list_head,
    pub gro_cells: gro_cells,
// Protected by RTNL
    pub discovery_wq: delayed_work,
// Protected by RTNL
    pub req_wq: delayed_work,
// Protected by RTNL
    pub secret_wq: delayed_work,
    pub event_wq: work_struct,
// AMT status
    pub status: amt_status,
// Generated key
    pub key: siphash_key_t,
    pub sk: *mut sock __rcu,
    pub max_groups: u32,
    pub max_sources: u32,
    pub hash_buckets: u32,
    pub hash_seed: u32,
// Default 128
    pub max_tunnels: u32,
// Default 128
    pub nr_tunnels: u32,
// Gateway or Relay mode
    pub mode: u32,
// Default 2268
    pub relay_port: __be16,
// Default 2268
    pub gw_port: __be16,
// Outer local ip
    pub local_ip: __be32,
// Outer remote ip
    pub remote_ip: __be32,
// Outer discovery ip
    pub discovery_ip: __be32,
// Only used in gateway mode
    pub nonce: __be32,
// Gateway sent request and received query
    pub ready4: bool,
    pub ready6: bool,
    pub req_cnt: u8,
    pub qi: u8,
    pub qrv: u64,
    pub qri: u64,
// Used only in gateway mode
// AMT gateway side message handler queue
    pub events: [amt_events; AMT_MAX_EVENTS],
    pub event_idx: u8,
    pub nr_events: u8,
}

pub const AMT_TOS: c_uint = 0xc0;
pub const AMT_IPHDR_OPTS: c_int = 4;
pub const AMT_IP6HDR_OPTS: c_int = 8;

pub const AMT_MAX_GROUP: c_int = 32;
pub const AMT_MAX_SOURCE: c_int = 128;
pub const AMT_HSIZE_SHIFT: c_int = 8;

pub const AMT_DISCOVERY_TIMEOUT: c_int = 5000;
pub const AMT_INIT_REQ_TIMEOUT: c_int = 1;
pub const AMT_INIT_QUERY_INTERVAL: c_int = 125;
pub const AMT_MAX_REQ_TIMEOUT: c_int = 120;
pub const AMT_MAX_REQ_COUNT: c_int = 3;
pub const AMT_SECRET_TIMEOUT: c_int = 60000;
pub const IANA_AMT_UDP_PORT: c_int = 2268;
pub const AMT_MAX_TUNNELS: c_int = 128;
pub const AMT_MAX_REQS: c_int = 128;

