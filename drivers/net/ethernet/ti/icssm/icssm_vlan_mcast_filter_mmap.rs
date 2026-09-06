//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssm/icssm_vlan_mcast_filter_mmap.h
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
// Copyright (C) 2015-2021 Texas Instruments Incorporated - https://www.ti.com
//
// This file contains VLAN/Multicast filtering feature memory map
//
// VLAN/Multicast filter defines & offsets,
// present on both PRU0 and PRU1 DRAM
//
// Feature enable/disable values for multicast filtering
pub const ICSS_EMAC_FW_MULTICAST_FILTER_CTRL_DISABLED: c_uint = 0x00;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_CTRL_ENABLED: c_uint = 0x01;
// Feature enable/disable values for VLAN filtering
pub const ICSS_EMAC_FW_VLAN_FILTER_CTRL_DISABLED: c_uint = 0x00;
pub const ICSS_EMAC_FW_VLAN_FILTER_CTRL_ENABLED: c_uint = 0x01;
// Add/remove multicast mac id for filtering bin
pub const ICSS_EMAC_FW_MULTICAST_FILTER_HOST_RCV_ALLOWED: c_uint = 0x01;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_HOST_RCV_NOT_ALLOWED: c_uint = 0x00;
// Default HASH value for the multicast filtering Mask
pub const ICSS_EMAC_FW_MULTICAST_FILTER_INIT_VAL: c_uint = 0xFF;
// Size requirements for Multicast filtering feature
pub const ICSS_EMAC_FW_MULTICAST_TABLE_SIZE_BYTES: c_int = 256;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_MASK_SIZE_BYTES: c_int = 6;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_CTRL_SIZE_BYTES: c_int = 1;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_MASK_OVERRIDE_STATUS_SIZE_BYTES: c_int = 1;
pub const ICSS_EMAC_FW_MULTICAST_FILTER_DROP_CNT_SIZE_BYTES: c_int = 4;
// Size requirements for VLAN filtering feature : 4096 bits = 512 bytes
pub const ICSS_EMAC_FW_VLAN_FILTER_TABLE_SIZE_BYTES: c_int = 512;
pub const ICSS_EMAC_FW_VLAN_FILTER_CTRL_SIZE_BYTES: c_int = 1;
pub const ICSS_EMAC_FW_VLAN_FILTER_DROP_CNT_SIZE_BYTES: c_int = 4;
// Mask override set status
pub const ICSS_EMAC_FW_MULTICAST_FILTER_MASK_OVERRIDE_SET: c_int = 1;
// Mask override not set status
pub const ICSS_EMAC_FW_MULTICAST_FILTER_MASK_OVERRIDE_NOT_SET: c_int = 0;
// 6 bytes HASH Mask for the MAC
pub const ICSS_EMAC_FW_MULTICAST_FILTER_MASK_OFFSET: c_uint = 0xF4;
// 0 -> multicast filtering disabled | 1 -> multicast filtering enabled

// Status indicating if the HASH override is done or not: 0: no, 1: yes

// Multicast drop statistics

// Multicast table

// Multicast filter defines & offsets for LRE
//
pub const ICSS_LRE_FW_MULTICAST_TABLE_SEARCH_OP_CONTROL_BIT: c_uint = 0xE0;
// one byte field :
// 0 -> multicast filtering disabled
// 1 -> multicast filtering enabled
//
pub const ICSS_LRE_FW_MULTICAST_FILTER_MASK: c_uint = 0xE4;
pub const ICSS_LRE_FW_MULTICAST_FILTER_TABLE: c_uint = 0x100;
// VLAN table Offsets
pub const ICSS_EMAC_FW_VLAN_FLTR_TBL_BASE_ADDR: c_uint = 0x200;
pub const ICSS_EMAC_FW_VLAN_FILTER_CTRL_BITMAP_OFFSET: c_uint = 0xEF;

// VLAN filter Control Bit maps
// one bit field, bit 0: | 0 : VLAN filter disabled (default),
// 1: VLAN filter enabled
//
pub const ICSS_EMAC_FW_VLAN_FILTER_CTRL_ENABLE_BIT: c_int = 0;
// one bit field, bit 1: | 0 : untagged host rcv allowed (default),
// 1: untagged host rcv not allowed
//
pub const ICSS_EMAC_FW_VLAN_FILTER_UNTAG_HOST_RCV_ALLOW_CTRL_BIT: c_int = 1;
// one bit field, bit 1: | 0 : priotag host rcv allowed (default),
// 1: priotag host rcv not allowed
//
pub const ICSS_EMAC_FW_VLAN_FILTER_PRIOTAG_HOST_RCV_ALLOW_CTRL_BIT: c_int = 2;
// one bit field, bit 1: | 0 : skip sv vlan flow
// :1 : take sv vlan flow  (not applicable for dual emac )
//
pub const ICSS_EMAC_FW_VLAN_FILTER_SV_VLAN_FLOW_HOST_RCV_ALLOW_CTRL_BIT: c_int = 3;
// VLAN IDs
pub const ICSS_EMAC_FW_VLAN_FILTER_PRIOTAG_VID: c_int = 0;
pub const ICSS_EMAC_FW_VLAN_FILTER_VID_MIN: c_uint = 0x0000;
pub const ICSS_EMAC_FW_VLAN_FILTER_VID_MAX: c_uint = 0x0FFF;
// VLAN Filtering Commands
pub const ICSS_EMAC_FW_VLAN_FILTER_ADD_VLAN_VID_CMD: c_uint = 0x00;
pub const ICSS_EMAC_FW_VLAN_FILTER_REMOVE_VLAN_VID_CMD: c_uint = 0x01;
// Switch defines for VLAN/MC filtering
// SRAM
// VLAN filter defines & offsets
//
pub const ICSS_LRE_FW_VLAN_FLTR_CTRL_BYTE: c_uint = 0x1FE;
// one bit field | 0 : VLAN filter disabled
// | 1 : VLAN filter enabled
//
pub const ICSS_LRE_FW_VLAN_FLTR_TBL_BASE_ADDR: c_uint = 0x200;
