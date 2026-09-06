//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_team.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/linux/if_team.h - Network team device driver header
// Copyright (c) 2011 Jiri Pirko <jpirko@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_pcpu_stats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_multicast: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub rx_nohandler: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_port {
    pub dev: *mut net_device,
    pub /: *mut *mut hlist_node tx_hlist; / node in tx-enabled ports hash list,
    pub /: *mut *mut list_head list; / node in ordinary list,
    pub team: *mut team,
    pub /: *mut *mut int tx_index; / index of tx enabled port. If disabled, -1,
    pub rx_enabled: bool,
    pub /: *mut *mut bool linkup; / either state.linkup or user.linkup,
    pub linkup: bool,
    pub speed: u32,
    pub duplex: u8,
    pub state: },
// Values set by userspace
    pub linkup: bool,
    pub linkup_enabled: bool,
    pub user: },
// Custom gennetlink interface related flags
    pub changed: bool,
    pub removed: bool,
//
// A place for storing original values of the device before it
// become a port.
//
    pub dev_addr: [c_uchar; MAX_ADDR_LEN],
    pub mtu: c_uint,
    pub orig: },

    pub np: *mut netpoll,

    pub /: *mut *mut s32 priority; / lower number ~ higher priority,
    pub queue_id: u16,
    pub /: *mut *mut list_head qom_list; / node in queue override mapping list,
    pub rcu: rcu_head,
    pub mode_priv: [c_long; ],
}

extern "C" {
    pub fn rcu_dereference(_arg: dev->rx_handler_data) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: port->rx_enabled) -> return;
}
extern "C" {
    pub fn team_port_rx_enabled(team_port_tx_enabled(port: port) &&) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_mode_ops {
    pub team): *mut *mut int (init)(struct team,
    pub team): *mut *mut void (exit)(struct team,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut *mut bool (transmit)(struct team team, struct sk_buff,
    pub port): *mut *mut *mut int (port_enter)(struct team team, struct team_port,
    pub port): *mut *mut *mut void (port_leave)(struct team team, struct team_port,
    pub port): *mut *mut *mut void (port_change_dev_addr)(struct team team, struct team_port,
    pub port): *mut *mut *mut void (port_tx_disabled)(struct team team, struct team_port,
}

extern "C" {
    pub fn team_modeop_port_enter(team: *mut team, port: *mut team_port) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum team_option_type {
    TEAM_OPTION_TYPE_U32,
    TEAM_OPTION_TYPE_STRING,
    TEAM_OPTION_TYPE_BINARY,
    TEAM_OPTION_TYPE_BOOL,
    TEAM_OPTION_TYPE_S32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_option_inst_info {
    pub array_index: u32,
    pub /: *mut *mut *mut team_port port; / != NULL if per-port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_gsetter_ctx {
    pub u32_val: u32,
    pub str_val: *const c_char,
    pub ptr: *const c_void,
    pub len: u32,
    pub bin_val: },
    pub bool_val: bool,
    pub s32_val: i32,
    pub data: },
    pub info: *mut team_option_inst_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_option {
    pub list: list_head,
    pub name: *const c_char,
    pub per_port: bool,
    pub /: *mut *mut unsigned int array_size; / != 0 means the option is array,
    pub type: team_option_type,
    pub info): *mut *mut *mut void (init)(struct team team, struct team_option_inst_info,
    pub ctx): *mut *mut *mut void (getter)(struct team team, struct team_gsetter_ctx,
    pub ctx): *mut *mut *mut int (setter)(struct team team, struct team_gsetter_ctx,
}

extern "C" {
    pub fn team_option_inst_set_change(opt_inst_info: *mut team_option_inst_info);
}
extern "C" {
    pub fn team_options_change_check(team: *mut team);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct team_mode {
    pub kind: *const c_char,
    pub owner: *mut module,
    pub priv_size: usize,
    pub port_priv_size: usize,
    pub ops: *const team_mode_ops,
    pub lag_tx_type: netdev_lag_tx_type,
}

pub const TEAM_PORT_HASHBITS: c_int = 4;

pub const TEAM_MODE_PRIV_LONGS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct team {
    pub pcpu_stats: *mut team_pcpu_stats __percpu,
    pub header_ops_cache: *const header_ops,
//
// List of tx-enabled ports and counts of rx and tx-enabled ports.
//
    pub tx_en_port_count: c_int,
    pub rx_en_port_count: c_int,
    pub tx_en_port_hlist: [hlist_head; TEAM_PORT_HASHENTRIES],
    pub /: *mut *mut list_head port_list; / list of all ports,
    pub option_list: list_head,
    pub /: *mut *mut list_head option_inst_list; / list of option instances,
    pub mode: *const team_mode,
    pub ops: team_mode_ops,
    pub user_carrier_enabled: bool,
    pub queue_override_enabled: bool,
    pub /: *mut *mut *mut list_head qom_lists; / array of queue override mapping lists,
    pub port_mtu_change_allowed: bool,
    pub notifier_ctx: bool,
    pub count: c_uint,
    pub /: *mut *mut unsigned int interval; / in ms,
    pub count_pending: core::sync::atomic::AtomicI32,
    pub dw: delayed_work,
    pub notify_peers: },
    pub count: c_uint,
    pub /: *mut *mut unsigned int interval; / in ms,
    pub count_pending: core::sync::atomic::AtomicI32,
    pub dw: delayed_work,
    pub mcast_rejoin: },
    pub mode_priv: [c_long; TEAM_MODE_PRIV_LONGS],
}

extern "C" {
    pub fn dev_queue_xmit(_arg: skb) -> return;
}
extern "C" {
    pub fn team_mode_register(mode: *const team_mode) -> c_int;
}
extern "C" {
    pub fn team_mode_unregister(mode: *const team_mode);
}
pub const TEAM_DEFAULT_NUM_TX_QUEUES: c_int = 16;
pub const TEAM_DEFAULT_NUM_RX_QUEUES: c_int = 16;

