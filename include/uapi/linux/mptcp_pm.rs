//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mptcp_pm.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/mptcp_pm.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const MPTCP_PM_VER: c_int = 1;
//
// enum mptcp_event_type - Netlink MPTCP event types
// @MPTCP_EVENT_UNSPEC: unused event
// @MPTCP_EVENT_CREATED: A new MPTCP connection has been created. It is the
// good time to allocate memory and send ADD_ADDR if needed. Depending on the
// traffic-patterns it can take a long time until the MPTCP_EVENT_ESTABLISHED
// is sent. Attributes: token, family, saddr4 | saddr6, daddr4 | daddr6,
// sport, dport, [server-side], [flags].
// @MPTCP_EVENT_ESTABLISHED: A MPTCP connection is established (can start new
// subflows). Attributes: token, family, saddr4 | saddr6, daddr4 | daddr6,
// sport, dport, [server-side], [flags].
// @MPTCP_EVENT_CLOSED: A MPTCP connection has stopped. Attribute: token.
// @MPTCP_EVENT_ANNOUNCED: A new address has been announced by the peer.
// Attributes: token, rem_id, family, daddr4 | daddr6 [, dport].
// @MPTCP_EVENT_REMOVED: An address has been lost by the peer. Attributes:
// token, rem_id.
// @MPTCP_EVENT_SUB_ESTABLISHED: A new subflow has been established. 'error'
// should not be set. Attributes: token, family, loc_id, rem_id, saddr4 |
// saddr6, daddr4 | daddr6, sport, dport, backup, if-idx [, error].
// @MPTCP_EVENT_SUB_CLOSED: A subflow has been closed. An error (copy of
// sk_err) could be set if an error has been detected for this subflow.
// Attributes: token, family, loc_id, rem_id, saddr4 | saddr6, daddr4 |
// daddr6, sport, dport, backup, if-idx [, error].
// @MPTCP_EVENT_SUB_PRIORITY: The priority of a subflow has changed. 'error'
// should not be set. Attributes: token, family, loc_id, rem_id, saddr4 |
// saddr6, daddr4 | daddr6, sport, dport, backup, if-idx [, error].
// @MPTCP_EVENT_LISTENER_CREATED: A new PM listener is created. Attributes:
// family, sport, saddr4 | saddr6.
// @MPTCP_EVENT_LISTENER_CLOSED: A PM listener is closed. Attributes: family,
// sport, saddr4 | saddr6.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptcp_event_type {
    MPTCP_EVENT_UNSPEC,
    MPTCP_EVENT_CREATED,
    MPTCP_EVENT_ESTABLISHED,
    MPTCP_EVENT_CLOSED,
    MPTCP_EVENT_ANNOUNCED = 6,
    MPTCP_EVENT_REMOVED,
    MPTCP_EVENT_SUB_ESTABLISHED = 10,
    MPTCP_EVENT_SUB_CLOSED,
    MPTCP_EVENT_SUB_PRIORITY = 13,
    MPTCP_EVENT_LISTENER_CREATED = 15,
    MPTCP_EVENT_LISTENER_CLOSED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptcp_event_attr {
    MPTCP_ATTR_UNSPEC,
    MPTCP_ATTR_TOKEN,
    MPTCP_ATTR_FAMILY,
    MPTCP_ATTR_LOC_ID,
    MPTCP_ATTR_REM_ID,
    MPTCP_ATTR_SADDR4,
    MPTCP_ATTR_SADDR6,
    MPTCP_ATTR_DADDR4,
    MPTCP_ATTR_DADDR6,
    MPTCP_ATTR_SPORT,
    MPTCP_ATTR_DPORT,
    MPTCP_ATTR_BACKUP,
    MPTCP_ATTR_ERROR,
    MPTCP_ATTR_FLAGS,
    MPTCP_ATTR_TIMEOUT,
    MPTCP_ATTR_IF_IDX,
    MPTCP_ATTR_RESET_REASON,
    MPTCP_ATTR_RESET_FLAGS,
    MPTCP_ATTR_SERVER_SIDE,

    __MPTCP_ATTR_MAX
}

