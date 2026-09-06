//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cifs/cifs_netlink.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// Netlink routines for CIFS
//
// Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
//

pub const CIFS_GENL_VERSION: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_genl_multicast_groups {
    CIFS_GENL_MCGRP_SWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_genl_attributes {
    CIFS_GENL_ATTR_UNSPEC,
    CIFS_GENL_ATTR_SWN_REGISTRATION_ID,
    CIFS_GENL_ATTR_SWN_NET_NAME,
    CIFS_GENL_ATTR_SWN_SHARE_NAME,
    CIFS_GENL_ATTR_SWN_IP,
    CIFS_GENL_ATTR_SWN_NET_NAME_NOTIFY,
    CIFS_GENL_ATTR_SWN_SHARE_NAME_NOTIFY,
    CIFS_GENL_ATTR_SWN_IP_NOTIFY,
    CIFS_GENL_ATTR_SWN_KRB_AUTH,
    CIFS_GENL_ATTR_SWN_USER_NAME,
    CIFS_GENL_ATTR_SWN_PASSWORD,
    CIFS_GENL_ATTR_SWN_DOMAIN_NAME,
    CIFS_GENL_ATTR_SWN_NOTIFICATION_TYPE,
    CIFS_GENL_ATTR_SWN_RESOURCE_STATE,
    CIFS_GENL_ATTR_SWN_RESOURCE_NAME,
    __CIFS_GENL_ATTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_genl_commands {
    CIFS_GENL_CMD_UNSPEC,
    CIFS_GENL_CMD_SWN_REGISTER,
    CIFS_GENL_CMD_SWN_UNREGISTER,
    CIFS_GENL_CMD_SWN_NOTIFY,
    __CIFS_GENL_CMD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_swn_notification_type {
    CIFS_SWN_NOTIFICATION_RESOURCE_CHANGE = 0x01,
    CIFS_SWN_NOTIFICATION_CLIENT_MOVE	 = 0x02,
    CIFS_SWN_NOTIFICATION_SHARE_MOVE	 = 0x03,
    CIFS_SWN_NOTIFICATION_IP_CHANGE	 = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_swn_resource_state {
    CIFS_SWN_RESOURCE_STATE_UNKNOWN     = 0x00,
    CIFS_SWN_RESOURCE_STATE_AVAILABLE   = 0x01,
    CIFS_SWN_RESOURCE_STATE_UNAVAILABLE = 0xFF
}
