//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/net_dropmon.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_dm_drop_point {
    pub pc: [__u8; 8],
    pub count: __u32,
}

pub const NET_DM_CFG_VERSION: c_int = 0;
pub const NET_DM_CFG_ALERT_COUNT: c_int = 1;
pub const NET_DM_CFG_ALERT_DELAY: c_int = 2;
pub const NET_DM_CFG_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_dm_config_entry {
    pub type: __u32,
    pub __attribute__((aligned(8))): __u64 data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_dm_config_msg {
    pub entries: __u32,
    pub options: [net_dm_config_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_dm_alert_msg {
    pub entries: __u32,
    pub points: [net_dm_drop_point; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_dm_user_msg {
    pub user: net_dm_config_msg,
    pub alert: net_dm_alert_msg,
    pub u: },
}

// These are the netlink message types for this protocol

//
// Our group identifiers
//
pub const NET_DM_GRP_ALERT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_dm_attr {
    NET_DM_ATTR_UNSPEC,

    NET_DM_ATTR_ALERT_MODE,			/* u8 */
    NET_DM_ATTR_PC,				/* u64 */
    NET_DM_ATTR_SYMBOL,			/* string */
    NET_DM_ATTR_IN_PORT,			/* nested */
    NET_DM_ATTR_TIMESTAMP,			/* u64 */
    NET_DM_ATTR_PROTO,			/* u16 */
    NET_DM_ATTR_PAYLOAD,			/* binary */
    NET_DM_ATTR_PAD,
    NET_DM_ATTR_TRUNC_LEN,			/* u32 */
    NET_DM_ATTR_ORIG_LEN,			/* u32 */
    NET_DM_ATTR_QUEUE_LEN,			/* u32 */
    NET_DM_ATTR_STATS,			/* nested */
    NET_DM_ATTR_HW_STATS,			/* nested */
    NET_DM_ATTR_ORIGIN,			/* u16 */
    NET_DM_ATTR_HW_TRAP_GROUP_NAME,		/* string */
    NET_DM_ATTR_HW_TRAP_NAME,		/* string */
    NET_DM_ATTR_HW_ENTRIES,			/* nested */
    NET_DM_ATTR_HW_ENTRY,			/* nested */
    NET_DM_ATTR_HW_TRAP_COUNT,		/* u32 */
    NET_DM_ATTR_SW_DROPS,			/* flag */
    NET_DM_ATTR_HW_DROPS,			/* flag */
    NET_DM_ATTR_FLOW_ACTION_COOKIE,		/* binary */
    NET_DM_ATTR_REASON,			/* string */

    __NET_DM_ATTR_MAX,
    NET_DM_ATTR_MAX = __NET_DM_ATTR_MAX - 1
}

//
// enum net_dm_alert_mode - Alert mode.
// @NET_DM_ALERT_MODE_SUMMARY: A summary of recent drops is sent to user space.
// @NET_DM_ALERT_MODE_PACKET: Each dropped packet is sent to user space along
// with metadata.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_dm_alert_mode {
    NET_DM_ALERT_MODE_SUMMARY,
    NET_DM_ALERT_MODE_PACKET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_dm_origin {
    NET_DM_ORIGIN_SW,
    NET_DM_ORIGIN_HW,
}
