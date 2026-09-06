//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_nvm_iscsi_cfg.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

// ISCSI IBFT constraint
//

// assuming 4 port card
//
pub const NVM_ISCSI_CFG_DHCP_NAME_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_dhcp_vendor_id {
    pub 4]: u32 value[NVM_ISCSI_CFG_DHCP_NAME_MAX_LEN /,
    pub byte: [u8; NVM_ISCSI_CFG_DHCP_NAME_MAX_LEN],
}

pub const NVM_ISCSI_IPV4_ADDR_BYTE_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_ipv4_addr {
    pub addr: u32,
    pub byte: [u8; NVM_ISCSI_IPV4_ADDR_BYTE_LEN],
}

pub const NVM_ISCSI_IPV6_ADDR_BYTE_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_ipv6_addr {
    pub addr: [u32; 4],
    pub byte: [u8; NVM_ISCSI_IPV6_ADDR_BYTE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_initiator_ipv4 {
    pub /: *mut *mut nvm_iscsi_ipv4_addr addr; / 0x0,
    pub /: *mut *mut nvm_iscsi_ipv4_addr subnet_mask; / 0x4,
    pub /: *mut *mut nvm_iscsi_ipv4_addr gateway; / 0x8,
    pub /: *mut *mut nvm_iscsi_ipv4_addr primary_dns; / 0xC,
    pub /: *mut *mut nvm_iscsi_ipv4_addr secondary_dns; / 0x10,
    pub /: *mut *mut nvm_iscsi_ipv4_addr dhcp_addr; / 0x14,
    pub /: *mut *mut nvm_iscsi_ipv4_addr isns_server; / 0x18,
    pub /: *mut *mut nvm_iscsi_ipv4_addr slp_server; / 0x1C,
    pub /: *mut *mut nvm_iscsi_ipv4_addr primay_radius_server; / 0x20,
    pub /: *mut *mut nvm_iscsi_ipv4_addr secondary_radius_server; / 0x24,
    pub /: *mut *mut nvm_iscsi_ipv4_addr rsvd[4]; / 0x28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_initiator_ipv6 {
    pub /: *mut *mut nvm_iscsi_ipv6_addr addr; / 0x0,
    pub /: *mut *mut nvm_iscsi_ipv6_addr subnet_mask; / 0x10,
    pub /: *mut *mut nvm_iscsi_ipv6_addr gateway; / 0x20,
    pub /: *mut *mut nvm_iscsi_ipv6_addr primary_dns; / 0x30,
    pub /: *mut *mut nvm_iscsi_ipv6_addr secondary_dns; / 0x40,
    pub /: *mut *mut nvm_iscsi_ipv6_addr dhcp_addr; / 0x50,
    pub /: *mut *mut nvm_iscsi_ipv6_addr isns_server; / 0x60,
    pub /: *mut *mut nvm_iscsi_ipv6_addr slp_server; / 0x70,
    pub /: *mut *mut nvm_iscsi_ipv6_addr primay_radius_server; / 0x80,
    pub /: *mut *mut nvm_iscsi_ipv6_addr secondary_radius_server; / 0x90,
    pub /: *mut *mut nvm_iscsi_ipv6_addr rsvd[3]; / 0xA0,
    pub /: *mut *mut u32 config; / 0xD0,
pub const NVM_ISCSI_CFG_INITIATOR_IPV6_SUBNET_MASK_PREFIX_MASK: c_uint = 0x000000FF;
pub const NVM_ISCSI_CFG_INITIATOR_IPV6_SUBNET_MASK_PREFIX_OFFSET: c_int = 0;
    pub rsvd_1: [u32; 3],
}

pub const NVM_ISCSI_CFG_ISCSI_NAME_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_name {
    pub 4]: u32 value[NVM_ISCSI_CFG_ISCSI_NAME_MAX_LEN /,
    pub byte: [u8; NVM_ISCSI_CFG_ISCSI_NAME_MAX_LEN],
}

pub const NVM_ISCSI_CFG_CHAP_NAME_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_chap_name {
    pub 4]: u32 value[NVM_ISCSI_CFG_CHAP_NAME_MAX_LEN /,
    pub byte: [u8; NVM_ISCSI_CFG_CHAP_NAME_MAX_LEN],
}

// is 16 octets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_chap_password {
    pub 4]: u32 value[NVM_ISCSI_CFG_CHAP_PWD_MAX_LEN /,
    pub byte: [u8; NVM_ISCSI_CFG_CHAP_PWD_MAX_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_iscsi_lun {
    pub byte: [u8; 8],
    pub value: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_generic {
    pub /: *mut *mut u32 ctrl_flags; / 0x0,

    pub /: *mut *mut u32 timeout; / 0x4,
pub const NVM_ISCSI_CFG_GEN_DHCP_REQUEST_TIMEOUT_MASK: c_uint = 0x0000FFFF;
pub const NVM_ISCSI_CFG_GEN_DHCP_REQUEST_TIMEOUT_OFFSET: c_int = 0;
pub const NVM_ISCSI_CFG_GEN_PORT_LOGIN_TIMEOUT_MASK: c_uint = 0xFFFF0000;
pub const NVM_ISCSI_CFG_GEN_PORT_LOGIN_TIMEOUT_OFFSET: c_int = 16;
    pub /: *mut *mut nvm_iscsi_dhcp_vendor_id dhcp_vendor_id; / 0x8,
    pub /: *mut *mut u32 rsvd[62]; / 0x108,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_initiator {
    pub /: *mut *mut nvm_iscsi_initiator_ipv4 ipv4; / 0x0,
    pub /: *mut *mut nvm_iscsi_initiator_ipv6 ipv6; / 0x38,
    pub /: *mut *mut nvm_iscsi_name initiator_name; / 0x118,
    pub /: *mut *mut nvm_iscsi_chap_name chap_name; / 0x218,
    pub /: *mut *mut nvm_iscsi_chap_password chap_password; / 0x318,
    pub /: *mut *mut u32 generic_cont0; / 0x398,
pub const NVM_ISCSI_CFG_INITIATOR_VLAN_MASK: c_uint = 0x0000FFFF;
pub const NVM_ISCSI_CFG_INITIATOR_VLAN_OFFSET: c_int = 0;
pub const NVM_ISCSI_CFG_INITIATOR_IP_VERSION_MASK: c_uint = 0x00030000;
pub const NVM_ISCSI_CFG_INITIATOR_IP_VERSION_OFFSET: c_int = 16;
pub const NVM_ISCSI_CFG_INITIATOR_IP_VERSION_4: c_int = 1;
pub const NVM_ISCSI_CFG_INITIATOR_IP_VERSION_6: c_int = 2;
pub const NVM_ISCSI_CFG_INITIATOR_IP_VERSION_4_AND_6: c_int = 3;
    pub ctrl_flags: u32,

    pub /: *mut *mut u32 rsvd[116]; / 0x32C,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_target {
    pub /: *mut *mut u32 ctrl_flags; / 0x0,

    pub /: *mut *mut u32 generic_cont0; / 0x4,
pub const NVM_ISCSI_CFG_TARGET_TCP_PORT_MASK: c_uint = 0x0000FFFF;
pub const NVM_ISCSI_CFG_TARGET_TCP_PORT_OFFSET: c_int = 0;
    pub ip_ver: u32,
pub const NVM_ISCSI_CFG_IPv4: c_int = 4;
pub const NVM_ISCSI_CFG_IPv6: c_int = 6;
    pub /: *mut *mut u32 rsvd_1[7]; / 0x24,
    pub /: *mut *mut nvm_iscsi_ipv4_addr ipv4_addr; / 0x28,
    pub /: *mut *mut nvm_iscsi_ipv6_addr ipv6_addr; / 0x2C,
    pub /: *mut *mut nvm_iscsi_lun lun; / 0x3C,
    pub /: *mut *mut nvm_iscsi_name target_name; / 0x44,
    pub /: *mut *mut nvm_iscsi_chap_name chap_name; / 0x144,
    pub /: *mut *mut nvm_iscsi_chap_password chap_password; / 0x244,
    pub /: *mut *mut u32 rsvd_2[107]; / 0x2C4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_block {
    pub /: *mut *mut u32 id; / 0x0,
pub const NVM_ISCSI_CFG_BLK_MAPPED_PF_ID_MASK: c_uint = 0x0000000F;
pub const NVM_ISCSI_CFG_BLK_MAPPED_PF_ID_OFFSET: c_int = 0;
pub const NVM_ISCSI_CFG_BLK_CTRL_FLAG_MASK: c_uint = 0x00000FF0;
pub const NVM_ISCSI_CFG_BLK_CTRL_FLAG_OFFSET: c_int = 4;

    pub /: *mut *mut u32 rsvd_1[5]; / 0x4,
    pub /: *mut *mut nvm_iscsi_generic generic; / 0x18,
    pub /: *mut *mut nvm_iscsi_initiator initiator; / 0x218,
    pub target: [nvm_iscsi_target; NUM_OF_ISCSI_TARGET_PER_PF],
// 0x718
    pub /: *mut *mut u32 rsvd_2[58]; / 0x1718,
// total size - 0x1800 - 6K block
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_iscsi_cfg {
    pub /: *mut *mut u32 id; / 0x0,
pub const NVM_ISCSI_CFG_BLK_VERSION_MINOR_MASK: c_uint = 0x000000FF;
pub const NVM_ISCSI_CFG_BLK_VERSION_MAJOR_MASK: c_uint = 0x0000FF00;
pub const NVM_ISCSI_CFG_BLK_SIGNATURE_MASK: c_uint = 0xFFFF0000;
pub const NVM_ISCSI_CFG_BLK_SIGNATURE: c_uint = 0x49430000 /* IC - Iscsi;
// Config
//
pub const NVM_ISCSI_CFG_BLK_VERSION_MAJOR: c_int = 0;
pub const NVM_ISCSI_CFG_BLK_VERSION_MINOR: c_int = 10;

    pub /: *mut *mut nvm_iscsi_block block[NUM_OF_ISCSI_PF_SUPPORTED]; / 0x4,
}
