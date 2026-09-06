//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc4_hw.h
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
// This header file defines the register offsets and bit fields
// of ENETC4 PF and VFs. Note that the same registers as ENETC
// version 1.0 are defined in the enetc_hw.h file.
//
// Copyright 2024 NXP
//
pub const NXP_ENETC_VENDOR_ID: c_uint = 0x1131;
pub const NXP_ENETC_PF_DEV_ID: c_uint = 0xe101;
pub const NXP_ENETC_PPM_DEV_ID: c_uint = 0xe110;
// Station interface registers
// Station interface LSO segmentation flag mask register 0/1
pub const ENETC4_SILSOSFMR0: c_uint = 0x1300;

pub const ENETC4_SILSOSFMR1: c_uint = 0x1304;

// According to tso_build_hdr(), clear all special flags for not last packet.

// ENETC port registers
pub const ENETC4_ECAPR0: c_uint = 0x0;

pub const ENETC4_ECAPR1: c_uint = 0x4;

pub const ENETC4_ECAPR2: c_uint = 0x8;

pub const ENETC4_PMR: c_uint = 0x10;

// Port Pause ON/OFF threshold register
pub const ENETC4_PPAUONTR: c_uint = 0x108;
pub const ENETC4_PPAUOFFTR: c_uint = 0x10c;
// Port ingress congestion DRa (a=0,1,2,3) discard count register

// Port Station interface promiscuous MAC mode register
pub const ENETC4_PSIPMMR: c_uint = 0x200;
// Port Station interface promiscuous VLAN mode register
pub const ENETC4_PSIPVMR: c_uint = 0x204;
// Port broadcast frames dropped due to MAC filtering register
pub const ENETC4_PBFDSIR: c_uint = 0x208;
// Port frame drop MAC source address pruning register
pub const ENETC4_PFDMSAPR: c_uint = 0x20c;
// Port RSS key register n. n = 0,1,2,...,9

// Port station interface MAC address filtering capability register
pub const ENETC4_PSIMAFCAPR: c_uint = 0x280;

// Port unicast frames dropped due to MAC filtering register
pub const ENETC4_PUFDMFR: c_uint = 0x284;
// Port multicast frames dropped due to MAC filtering register
pub const ENETC4_PMFDMFR: c_uint = 0x288;
// Port station interface VLAN filtering capability register
pub const ENETC4_PSIVLANFCAPR: c_uint = 0x2c0;

// Port station interface VLAN filtering mode register
pub const ENETC4_PSIVLANFMR: c_uint = 0x2c4;

// Port unicast frames dropped VLAN filtering register
pub const ENETC4_PUFDVFR: c_uint = 0x2d0;
// Port multicast frames dropped VLAN filtering register
pub const ENETC4_PMFDVFR: c_uint = 0x2d4;
// Port broadcast frames dropped VLAN filtering register
pub const ENETC4_PBFDVFR: c_uint = 0x2d8;
// Port Station interface a primary MAC address registers

// Port station interface a configuration register 0/2

// Port station interface a unicast MAC hash filter register 0/1

// Port station interface a multicast MAC hash filter register 0/1

// Port station interface a VLAN hash filter register 0/1

pub const ENETC4_PMCAPR: c_uint = 0x4004;

// Port capability register
pub const ENETC4_PCAPR: c_uint = 0x4000;

// Port configuration register
pub const ENETC4_PCR: c_uint = 0x4010;

// Port MAC address register 0/1
pub const ENETC4_PMAR0: c_uint = 0x4020;
pub const ENETC4_PMAR1: c_uint = 0x4024;
// Port operational register
pub const ENETC4_POR: c_uint = 0x4100;

// Port status register
pub const ENETC4_PSR: c_uint = 0x4104;

// Port Rx discard count register
pub const ENETC4_PRXDCR: c_uint = 0x41c0;
// Port Rx discard count read-reset register
pub const ENETC4_PRXDCRRR: c_uint = 0x41c4;
// Port Rx discard count reason register 0
pub const ENETC4_PRXDCRR0: c_uint = 0x41c8;
// Port Rx discard count reason register 1
pub const ENETC4_PRXDCRR1: c_uint = 0x41cc;
// Port traffic class a transmit maximum SDU register

pub const SDU_TYPE_PPDU: c_int = 0;
pub const SDU_TYPE_MPDU: c_int = 1;
pub const SDU_TYPE_MSDU: c_int = 2;
pub const ENETC4_PMAC_OFFSET: c_uint = 0x400;

pub const LPBCK_MODE_EXT_TX_CLK: c_int = 0;
pub const LPBCK_MODE_MAC_LEVEL: c_int = 1;
pub const LPBCK_MODE_INT_TX_CLK: c_int = 2;

// Port MAC 0/1 Maximum Frame Length Register

// Port internal MDIO base address, use to access PCS
pub const ENETC4_PM_IMDIO_BASE: c_uint = 0x5030;
// Port MAC 0/1 Interrupt Event Register

// Port MAC 0/1 Pause Quanta Register

// Port MAC 0/1 Pause Quanta Threshold Register

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

// Port MAC 0 Interface Mode Control Register

pub const IFMODE_XGMII: c_int = 0;
pub const IFMODE_RMII: c_int = 3;
pub const IFMODE_RGMII: c_int = 4;
pub const IFMODE_SGMII: c_int = 5;

pub const SSP_100M: c_int = 0;
pub const SSP_10M: c_int = 1;
pub const SSP_1G: c_int = 2;

// Port external MDIO Base address, use to access off-chip PHY
pub const ENETC4_EMDIO_BASE: c_uint = 0x5c00;
// ENETC Pseudo MAC port registers
// Port pseudo MAC receive octets counter (64-bit)
pub const ENETC4_PPMROCR: c_uint = 0x5080;
// Port pseudo MAC receive unicast frame counter register (64-bit)
pub const ENETC4_PPMRUFCR: c_uint = 0x5088;
// Port pseudo MAC receive multicast frame counter register (64-bit)
pub const ENETC4_PPMRMFCR: c_uint = 0x5090;
// Port pseudo MAC receive broadcast frame counter register (64-bit)
pub const ENETC4_PPMRBFCR: c_uint = 0x5098;
// Port pseudo MAC transmit octets counter (64-bit)
pub const ENETC4_PPMTOCR: c_uint = 0x50c0;
// Port pseudo MAC transmit unicast frame counter register (64-bit)
pub const ENETC4_PPMTUFCR: c_uint = 0x50c8;
// Port pseudo MAC transmit multicast frame counter register (64-bit)
pub const ENETC4_PPMTMFCR: c_uint = 0x50d0;
// Port pseudo MAC transmit broadcast frame counter register (64-bit)
pub const ENETC4_PPMTBFCR: c_uint = 0x50d8;
