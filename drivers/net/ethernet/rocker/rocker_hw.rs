//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/rocker/rocker_hw.h
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
// drivers/net/ethernet/rocker/rocker_hw.h - Rocker switch device driver
// Copyright (c) 2014-2016 Jiri Pirko <jiri@mellanox.com>
// Copyright (c) 2014 Scott Feldman <sfeldma@gmail.com>
//

// Return codes
pub const ROCKER_FP_PORTS_MAX: c_int = 62;
pub const PCI_DEVICE_ID_REDHAT_ROCKER: c_uint = 0x0006;
pub const ROCKER_PCI_BAR0_SIZE: c_uint = 0x2000;
// MSI-X vectors

// Rocker bogus registers
pub const ROCKER_BOGUS_REG0: c_uint = 0x0000;
pub const ROCKER_BOGUS_REG1: c_uint = 0x0004;
pub const ROCKER_BOGUS_REG2: c_uint = 0x0008;
pub const ROCKER_BOGUS_REG3: c_uint = 0x000c;
// Rocker test registers
pub const ROCKER_TEST_REG: c_uint = 0x0010;
pub const ROCKER_TEST_REG64: c_uint = 0x0018  /* 8-byte */;
pub const ROCKER_TEST_IRQ: c_uint = 0x0020;
pub const ROCKER_TEST_DMA_ADDR: c_uint = 0x0028  /* 8-byte */;
pub const ROCKER_TEST_DMA_SIZE: c_uint = 0x0030;
pub const ROCKER_TEST_DMA_CTRL: c_uint = 0x0034;
// Rocker test register ctrl

// Rocker DMA ring register offsets

// Rocker dma ctrl register bits

// Rocker DMA ring types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rocker_dma_type {
    ROCKER_DMA_CMD,
    ROCKER_DMA_EVENT,
    __ROCKER_DMA_TX,
    __ROCKER_DMA_RX,

}

// Rocker DMA ring size limits and default sizes

pub const ROCKER_DMA_TX_DESC_SIZE: c_int = 256;

pub const ROCKER_DMA_RX_DESC_SIZE: c_int = 256;
// Rocker DMA descriptor struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_desc {
    pub buf_addr: u64,
    pub cookie: u64,
    pub buf_size: u16,
    pub tlv_size: u16,
    pub resv: [u16; 5],
    pub comp_err: u16,
}

// Rocker DMA TLV struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_tlv {
    pub type: u32,
    pub len: u16,
}

// TLVs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rocker_port_mode {
    ROCKER_PORT_MODE_OF_DPA,
}

pub const ROCKER_TX_OFFLOAD_NONE: c_int = 0;
pub const ROCKER_TX_OFFLOAD_IP_CSUM: c_int = 1;
pub const ROCKER_TX_OFFLOAD_TCP_UDP_CSUM: c_int = 2;
pub const ROCKER_TX_OFFLOAD_L3_CSUM: c_int = 3;
pub const ROCKER_TX_OFFLOAD_TSO: c_int = 4;
pub const ROCKER_TX_FRAGS_MAX: c_int = 16;
// cmd info nested for OF-DPA msgs
// OF-DPA table IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rocker_of_dpa_table_id {
    ROCKER_OF_DPA_TABLE_ID_INGRESS_PORT = 0,
    ROCKER_OF_DPA_TABLE_ID_VLAN = 10,
    ROCKER_OF_DPA_TABLE_ID_TERMINATION_MAC = 20,
    ROCKER_OF_DPA_TABLE_ID_UNICAST_ROUTING = 30,
    ROCKER_OF_DPA_TABLE_ID_MULTICAST_ROUTING = 40,
    ROCKER_OF_DPA_TABLE_ID_BRIDGING = 50,
    ROCKER_OF_DPA_TABLE_ID_ACL_POLICY = 60,
}

// OF-DPA flow stats
// OF-DPA group types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rocker_of_dpa_group_type {
    ROCKER_OF_DPA_GROUP_TYPE_L2_INTERFACE = 0,
    ROCKER_OF_DPA_GROUP_TYPE_L2_REWRITE,
    ROCKER_OF_DPA_GROUP_TYPE_L3_UCAST,
    ROCKER_OF_DPA_GROUP_TYPE_L2_MCAST,
    ROCKER_OF_DPA_GROUP_TYPE_L2_FLOOD,
    ROCKER_OF_DPA_GROUP_TYPE_L3_INTERFACE,
    ROCKER_OF_DPA_GROUP_TYPE_L3_MCAST,
    ROCKER_OF_DPA_GROUP_TYPE_L3_ECMP,
    ROCKER_OF_DPA_GROUP_TYPE_L2_OVERLAY,
}

// OF-DPA group L2 overlay types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rocker_of_dpa_overlay_type {
    ROCKER_OF_DPA_OVERLAY_TYPE_FLOOD_UCAST = 0,
    ROCKER_OF_DPA_OVERLAY_TYPE_FLOOD_MCAST,
    ROCKER_OF_DPA_OVERLAY_TYPE_MCAST_UCAST,
    ROCKER_OF_DPA_OVERLAY_TYPE_MCAST_MCAST,
}

// OF-DPA group ID encoding
pub const ROCKER_GROUP_TYPE_SHIFT: c_int = 28;
pub const ROCKER_GROUP_TYPE_MASK: c_uint = 0xf0000000;
pub const ROCKER_GROUP_VLAN_SHIFT: c_int = 16;
pub const ROCKER_GROUP_VLAN_MASK: c_uint = 0x0fff0000;
pub const ROCKER_GROUP_PORT_SHIFT: c_int = 0;
pub const ROCKER_GROUP_PORT_MASK: c_uint = 0x0000ffff;
pub const ROCKER_GROUP_TUNNEL_ID_SHIFT: c_int = 12;
pub const ROCKER_GROUP_TUNNEL_ID_MASK: c_uint = 0x0ffff000;
pub const ROCKER_GROUP_SUBTYPE_SHIFT: c_int = 10;
pub const ROCKER_GROUP_SUBTYPE_MASK: c_uint = 0x00000c00;
pub const ROCKER_GROUP_INDEX_SHIFT: c_int = 0;
pub const ROCKER_GROUP_INDEX_MASK: c_uint = 0x0000ffff;
pub const ROCKER_GROUP_INDEX_LONG_SHIFT: c_int = 0;
pub const ROCKER_GROUP_INDEX_LONG_MASK: c_uint = 0x0fffffff;

pub const ROCKER_GROUP_NONE: c_int = 0;

// Rocker general purpose registers
pub const ROCKER_CONTROL: c_uint = 0x0300;
pub const ROCKER_PORT_PHYS_COUNT: c_uint = 0x0304;
pub const ROCKER_PORT_PHYS_LINK_STATUS: c_uint = 0x0310 /* 8-byte */;
pub const ROCKER_PORT_PHYS_ENABLE: c_uint = 0x0318 /* 8-byte */;
pub const ROCKER_SWITCH_ID: c_uint = 0x0320 /* 8-byte */;
// Rocker control bits

