//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_vlan.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// VLAN		An implementation of 802.1Q VLAN tagging.
//
// Authors:	Ben Greear <greearb@candelatech.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// VLAN IOCTLs are found in sockios.h
// Passed in vlan_ioctl_args structure to determine behaviour.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlan_ioctl_cmds {
    ADD_VLAN_CMD,
    DEL_VLAN_CMD,
    SET_VLAN_INGRESS_PRIORITY_CMD,
    SET_VLAN_EGRESS_PRIORITY_CMD,
    GET_VLAN_INGRESS_PRIORITY_CMD,
    GET_VLAN_EGRESS_PRIORITY_CMD,
    SET_VLAN_NAME_TYPE_CMD,
    SET_VLAN_FLAG_CMD,
    GET_VLAN_REALDEV_NAME_CMD, /* If this works, you know it's a VLAN device, btw */
    GET_VLAN_VID_CMD /* Get the VID of this VLAN (specified by name) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlan_flags {
    VLAN_FLAG_REORDER_HDR		= 0x1,
    VLAN_FLAG_GVRP			= 0x2,
    VLAN_FLAG_LOOSE_BINDING		= 0x4,
    VLAN_FLAG_MVRP			= 0x8,
    VLAN_FLAG_BRIDGE_BINDING	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlan_name_types {
    VLAN_NAME_TYPE_PLUS_VID, /* Name will look like:  vlan0005 */
    VLAN_NAME_TYPE_RAW_PLUS_VID, /* name will look like:  eth1.0005 */
    VLAN_NAME_TYPE_PLUS_VID_NO_PAD, /* Name will look like:  vlan5 */
    VLAN_NAME_TYPE_RAW_PLUS_VID_NO_PAD, /* Name will look like:  eth0.5 */
    VLAN_NAME_TYPE_HIGHEST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_ioctl_args {
    pub /: *mut *mut int cmd; / Should be one of the vlan_ioctl_cmds enum above.,
    pub device1: [c_char; 24],
    pub device2: [c_char; 24],
    pub VID: c_int,
    pub skb_priority: c_uint,
    pub name_type: c_uint,
    pub bind_type: c_uint,
    pub /: *mut *mut unsigned int flag; / Matches vlan_dev_priv flags,
    pub u: },
    pub vlan_qos: c_short,
}
