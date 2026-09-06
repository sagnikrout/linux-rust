//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iscsi_boot_sysfs.h
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
//
// Export the iSCSI boot info to userland via sysfs.
//
// Copyright (C) 2010 Red Hat, Inc.  All rights reserved.
// Copyright (C) 2010 Mike Christie
//
// The text attributes names for each of the kobjects.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_boot_eth_properties_enum {
    ISCSI_BOOT_ETH_INDEX,
    ISCSI_BOOT_ETH_FLAGS,
    ISCSI_BOOT_ETH_IP_ADDR,
    ISCSI_BOOT_ETH_PREFIX_LEN,
    ISCSI_BOOT_ETH_SUBNET_MASK,
    ISCSI_BOOT_ETH_ORIGIN,
    ISCSI_BOOT_ETH_GATEWAY,
    ISCSI_BOOT_ETH_PRIMARY_DNS,
    ISCSI_BOOT_ETH_SECONDARY_DNS,
    ISCSI_BOOT_ETH_DHCP,
    ISCSI_BOOT_ETH_VLAN,
    ISCSI_BOOT_ETH_MAC,
// eth_pci_bdf - this is replaced by link to the device itself.
    ISCSI_BOOT_ETH_HOSTNAME,
    ISCSI_BOOT_ETH_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_boot_tgt_properties_enum {
    ISCSI_BOOT_TGT_INDEX,
    ISCSI_BOOT_TGT_FLAGS,
    ISCSI_BOOT_TGT_IP_ADDR,
    ISCSI_BOOT_TGT_PORT,
    ISCSI_BOOT_TGT_LUN,
    ISCSI_BOOT_TGT_CHAP_TYPE,
    ISCSI_BOOT_TGT_NIC_ASSOC,
    ISCSI_BOOT_TGT_NAME,
    ISCSI_BOOT_TGT_CHAP_NAME,
    ISCSI_BOOT_TGT_CHAP_SECRET,
    ISCSI_BOOT_TGT_REV_CHAP_NAME,
    ISCSI_BOOT_TGT_REV_CHAP_SECRET,
    ISCSI_BOOT_TGT_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_boot_initiator_properties_enum {
    ISCSI_BOOT_INI_INDEX,
    ISCSI_BOOT_INI_FLAGS,
    ISCSI_BOOT_INI_ISNS_SERVER,
    ISCSI_BOOT_INI_SLP_SERVER,
    ISCSI_BOOT_INI_PRI_RADIUS_SERVER,
    ISCSI_BOOT_INI_SEC_RADIUS_SERVER,
    ISCSI_BOOT_INI_INITIATOR_NAME,
    ISCSI_BOOT_INI_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_boot_acpitbl_properties_enum {
    ISCSI_BOOT_ACPITBL_SIGNATURE,
    ISCSI_BOOT_ACPITBL_OEM_ID,
    ISCSI_BOOT_ACPITBL_OEM_TABLE_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_boot_kobj {
    pub kobj: kobject,
    pub attr_group: *mut attribute_group,
    pub list: list_head,
//
// Pointer to store driver specific info. If set this will
// be freed for the LLD when the kobj release function is called.
//
    pub data: *mut c_void,
//
// Driver specific show function.
//
// The enum of the type. This can be any value of the above
// properties.
//
    pub buf): *mut *mut *mut ssize_t (show) (void data, int type, char,
//
// Drivers specific visibility function.
// The function should return if they the attr should be readable
// writable or should not be shown.
//
// The enum of the type. This can be any value of the above
// properties.
//
    pub type): *mut *mut *mut umode_t (is_visible) (void data, int,
//
// Driver specific release function.
//
// The function should free the data passed in.
//
    pub data): *mut *mut void (release) (void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_boot_kset {
    pub kobj_list: list_head,
    pub kset: *mut kset,
}

extern "C" {
    pub fn iscsi_boot_destroy_kset(boot_kset: *mut iscsi_boot_kset);
}
