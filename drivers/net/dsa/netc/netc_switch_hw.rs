//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/netc/netc_switch_hw.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2025-2026 NXP
//

pub const NETC_SWITCH_VENDOR_ID: c_uint = 0x1131;
pub const NETC_SWITCH_DEVICE_ID: c_uint = 0xeef2;
// Definition of Switch base registers
pub const NETC_BPCAPR: c_uint = 0x0008;

pub const NETC_PBPMCR0: c_uint = 0x0400;
pub const NETC_PBPMCR1: c_uint = 0x0404;

pub const NETC_SWCR: c_uint = 0x1018;

pub const NETC_DOSL2CR: c_uint = 0x1220;

pub const NETC_DOSL3CR: c_uint = 0x1224;

pub const NETC_ETTCAPR: c_uint = 0x18c4;
pub const NETC_ECTCAPR: c_uint = 0x18ec;
// Index table NUM_ENTRIES mask

// Hash table memory capability register, the memory is shared by
// the following tables:
//
// - Ingress Stream Identification table
// - Ingress Stream Filter table
// - VLAN Filter table
// - FDB table
// - L2 IPv4 Multicast Filter table
//
// Each hash table entry is one word in size.
//
pub const NETC_HTMCAPR: c_uint = 0x1900;

pub const NETC_VFHTDECR1: c_uint = 0x2014;
pub const NETC_VFHTDECR2: c_uint = 0x2018;

// Definition of Switch port registers
pub const NETC_PCAPR: c_uint = 0x0000;

pub const NETC_PMCAPR: c_uint = 0x0004;

pub const FP_SUPPORT: c_int = 2;
pub const NETC_PCR: c_uint = 0x0010;

pub const NETC_PQOSMR: c_uint = 0x0054;

pub const NETC_PIPFCR: c_uint = 0x0084;

pub const NETC_POR: c_uint = 0x100;

pub const NETC_PSR: c_uint = 0x104;

pub const NETC_PTGSLACR: c_uint = 0x130;
pub const NETC_PRXDCR: c_uint = 0x1c0;
pub const NETC_PRXDCRRR: c_uint = 0x1c4;
pub const NETC_PRXDCRR0: c_uint = 0x1c8;
pub const NETC_PRXDCRR1: c_uint = 0x1cc;
pub const NETC_PTXDCR: c_uint = 0x1e0;

pub const SDU_TYPE_PPDU: c_int = 0;
pub const SDU_TYPE_MPDU: c_int = 1;
pub const SDU_TYPE_MSDU: c_int = 2;
pub const NETC_PSDFTCR: c_uint = 0x4c4;
pub const NETC_PSDFDDCR: c_uint = 0x4c8;
pub const NETC_BPCR: c_uint = 0x500;

// MAC learning options, see BPCR[MLO], VFHTDECR2[MLO] and
// VLAN Filter Table CFGE_DATA[MLO]
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netc_mlo {
    MLO_NOT_OVERRIDE = 0,
    MLO_DISABLE,
    MLO_HW,
    MLO_SW_SEC,
    MLO_SW_UNSEC,
    MLO_DISABLE_SMAC,
}

// MAC forwarding options, see VFHTDECR2[MFO] and VLAN
// Filter Table CFGE_DATA[MFO]
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netc_mfo {
    MFO_NO_FDB_LOOKUP = 1,
    MFO_NO_MATCH_FLOOD,
    MFO_NO_MATCH_DISCARD,
}

pub const NETC_BPDVR: c_uint = 0x510;

pub const NETC_BPSTGSR: c_uint = 0x520;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netc_stg_stage {
    NETC_STG_STATE_DISABLED = 0,
    NETC_STG_STATE_LEARNING,
    NETC_STG_STATE_FORWARDING,
}

pub const NETC_BPDCR: c_uint = 0x580;
// Definition of Switch ethernet MAC port registers
pub const NETC_PMAC_OFFSET: c_uint = 0x400;

pub const IFMODE_MII: c_int = 1;
pub const IFMODE_RMII: c_int = 3;
pub const IFMODE_RGMII: c_int = 4;
pub const IFMODE_SGMII: c_int = 5;

pub const SSP_100M: c_int = 0;
pub const SSP_10M: c_int = 1;
pub const SSP_1G: c_int = 2;
// Port MAC 0/1 Receive Ethernet Octets Counter

// Port MAC 0/1 Receive Octets Counter

// Port MAC 0/1 Receive Alignment Error Counter Register

// Port MAC 0/1 Receive Valid Pause Frame Counter

// Port MAC 0/1 Receive Frame Counter

// Port MAC 0/1 Receive Frame Check Sequence Error Counter

// Port MAC 0/1 Receive VLAN Frame Counter

// Port MAC 0/1 Receive Frame Error Counter

// Port MAC 0/1 Receive Unicast Frame Counter

// Port MAC 0/1 Receive Multicast Frame Counter

// Port MAC 0/1 Receive Broadcast Frame Counter

// Port MAC 0/1 Receive Dropped Packets Counter

// Port MAC 0/1 Receive Packets Counter

// Port MAC 0/1 Receive Undersized Packet Counter

// Port MAC 0/1 Receive 64-Octet Packet Counter

// Port MAC 0/1 Receive 65 to 127-Octet Packet Counter

// Port MAC 0/1 Receive 128 to 255-Octet Packet Counter

// Port MAC 0/1 Receive 256 to 511-Octet Packet Counter

// Port MAC 0/1 Receive 512 to 1023-Octet Packet Counter

// Port MAC 0/1 Receive 1024 to 1522-Octet Packet Counter

// Port MAC 0/1 Receive 1523 to Max-Octet Packet Counter

// Port MAC 0/1 Receive Oversized Packet Counter

// Port MAC 0/1 Receive Jabber Packet Counter

// Port MAC 0/1 Receive Fragment Packet Counter

// Port MAC 0/1 Receive Control Packet Counter

// Port MAC 0/1 Receive Dropped Not Truncated Packets Counter

// Port MAC 0/1 Transmit Ethernet Octets Counter

// Port MAC 0/1 Transmit Octets Counter

// Port MAC 0/1 Transmit Excessive Deferral Packet Counter

// Port MAC 0/1 Transmit Valid Pause Frame Counter

// Port MAC 0/1 Transmit Frame Counter

// Port MAC 0/1 Transmit Frame Check Sequence Error Counter

// Port MAC 0/1 Transmit VLAN Frame Counter

// Port MAC 0/1 Transmit Frame Error Counter

// Port MAC 0/1 Transmit Unicast Frame Counter

// Port MAC 0/1 Transmit Multicast Frame Counter

// Port MAC 0/1 Transmit Broadcast Frame Counter

// Port MAC 0/1 Transmit Packets Counter

// Port MAC 0/1 Transmit Undersized Packet Counter

// Port MAC 0/1 Transmit 64-Octet Packet Counter

// Port MAC 0/1 Transmit 65 to 127-Octet Packet Counter

// Port MAC 0/1 Transmit 128 to 255-Octet Packet Counter

// Port MAC 0/1 Transmit 256 to 511-Octet Packet Counter

// Port MAC 0/1 Transmit 512 to 1023-Octet Packet Counter

// Port MAC 0/1 Transmit 1024 to 1522-Octet Packet Counter

// Port MAC 0/1 Transmit 1523 to TX_MTU-Octet Packet Counter

// Port MAC 0/1 Transmit Control Packet Counter

// Port MAC 0/1 Transmit Deferred Packet Counter

// Port MAC 0/1 Transmit Multiple Collisions Counter

// Port MAC 0/1 Transmit Single Collision

// Port MAC 0/1 Transmit Late Collision Counter

// Port MAC 0/1 Transmit Excessive Collisions Counter

// Port MAC 0/1 Transmit Invalid Octets Counter

pub const NETC_PEMDIOCR: c_uint = 0x1c00;

// Definition of global registers (read only)
pub const NETC_IPBRR0: c_uint = 0x0bf8;

