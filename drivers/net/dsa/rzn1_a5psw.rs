//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/rzn1_a5psw.h
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
// Copyright (C) 2022 Schneider Electric
//
// Clément Léger <clement.leger@bootlin.com>
//

pub const A5PSW_REVISION: c_uint = 0x0;

pub const A5PSW_PORT_ENA: c_uint = 0x8;

pub const A5PSW_PORT_ENA_RX_SHIFT: c_int = 16;

pub const A5PSW_UCAST_DEF_MASK: c_uint = 0xC;
pub const A5PSW_VLAN_VERIFY: c_uint = 0x10;
pub const A5PSW_VLAN_VERI_SHIFT: c_int = 0;
pub const A5PSW_VLAN_DISC_SHIFT: c_int = 16;
pub const A5PSW_BCAST_DEF_MASK: c_uint = 0x14;
pub const A5PSW_MCAST_DEF_MASK: c_uint = 0x18;
pub const A5PSW_INPUT_LEARN: c_uint = 0x1C;

pub const A5PSW_MGMT_CFG: c_uint = 0x20;

pub const A5PSW_MODE_CFG: c_uint = 0x24;

pub const A5PSW_VLAN_IN_MODE: c_uint = 0x28;

pub const A5PSW_VLAN_IN_MODE_SINGLE_PASSTHROUGH: c_uint = 0x0;
pub const A5PSW_VLAN_IN_MODE_SINGLE_REPLACE: c_uint = 0x1;
pub const A5PSW_VLAN_IN_MODE_TAG_ALWAYS: c_uint = 0x2;
pub const A5PSW_VLAN_OUT_MODE: c_uint = 0x2C;

pub const A5PSW_VLAN_OUT_MODE_DIS: c_uint = 0x0;
pub const A5PSW_VLAN_OUT_MODE_STRIP: c_uint = 0x1;
pub const A5PSW_VLAN_OUT_MODE_TAG_THROUGH: c_uint = 0x2;
pub const A5PSW_VLAN_OUT_MODE_TRANSPARENT: c_uint = 0x3;
pub const A5PSW_VLAN_IN_MODE_ENA: c_uint = 0x30;
pub const A5PSW_VLAN_TAG_ID: c_uint = 0x34;

pub const A5PSW_LK_CTRL: c_uint = 0x400;

pub const A5PSW_LK_ADDR_CTRL: c_uint = 0x408;

pub const A5PSW_LK_DATA_LO: c_uint = 0x40C;
pub const A5PSW_LK_DATA_HI: c_uint = 0x410;

pub const A5PSW_LK_LEARNCOUNT: c_uint = 0x418;

pub const A5PSW_LK_LEARNCOUNT_MODE_SET: c_uint = 0x0;
pub const A5PSW_LK_LEARNCOUNT_MODE_INC: c_uint = 0x1;
pub const A5PSW_LK_LEARNCOUNT_MODE_DEC: c_uint = 0x2;
pub const A5PSW_MGMT_TAG_CFG: c_uint = 0x480;

pub const A5PSW_LK_AGETIME: c_uint = 0x41C;

pub const A5PSW_MDIO_CFG_STATUS: c_uint = 0x700;

pub const A5PSW_MDIO_COMMAND: c_uint = 0x704;
// Register is named TRAININIT in datasheet and should be set when reading

pub const A5PSW_MDIO_DATA: c_uint = 0x708;

pub const A5PSW_STATS_HIWORD: c_uint = 0x900;
// Stats
pub const A5PSW_aFramesTransmittedOK: c_uint = 0x868;
pub const A5PSW_aFramesReceivedOK: c_uint = 0x86C;
pub const A5PSW_aFrameCheckSequenceErrors: c_uint = 0x870;
pub const A5PSW_aAlignmentErrors: c_uint = 0x874;
pub const A5PSW_aOctetsTransmittedOK: c_uint = 0x878;
pub const A5PSW_aOctetsReceivedOK: c_uint = 0x87C;
pub const A5PSW_aTxPAUSEMACCtrlFrames: c_uint = 0x880;
pub const A5PSW_aRxPAUSEMACCtrlFrames: c_uint = 0x884;
// If
pub const A5PSW_ifInErrors: c_uint = 0x888;
pub const A5PSW_ifOutErrors: c_uint = 0x88C;
pub const A5PSW_ifInUcastPkts: c_uint = 0x890;
pub const A5PSW_ifInMulticastPkts: c_uint = 0x894;
pub const A5PSW_ifInBroadcastPkts: c_uint = 0x898;
pub const A5PSW_ifOutDiscards: c_uint = 0x89C;
pub const A5PSW_ifOutUcastPkts: c_uint = 0x8A0;
pub const A5PSW_ifOutMulticastPkts: c_uint = 0x8A4;
pub const A5PSW_ifOutBroadcastPkts: c_uint = 0x8A8;
// Ether
pub const A5PSW_etherStatsDropEvents: c_uint = 0x8AC;
pub const A5PSW_etherStatsOctets: c_uint = 0x8B0;
pub const A5PSW_etherStatsPkts: c_uint = 0x8B4;
pub const A5PSW_etherStatsUndersizePkts: c_uint = 0x8B8;
pub const A5PSW_etherStatsOversizePkts: c_uint = 0x8BC;
pub const A5PSW_etherStatsPkts64Octets: c_uint = 0x8C0;
pub const A5PSW_etherStatsPkts65to127Octets: c_uint = 0x8C4;
pub const A5PSW_etherStatsPkts128to255Octets: c_uint = 0x8C8;
pub const A5PSW_etherStatsPkts256to511Octets: c_uint = 0x8CC;
pub const A5PSW_etherStatsPkts512to1023Octets: c_uint = 0x8D0;
pub const A5PSW_etherStatsPkts1024to1518Octets: c_uint = 0x8D4;
pub const A5PSW_etherStatsPkts1519toXOctets: c_uint = 0x8D8;
pub const A5PSW_etherStatsJabbers: c_uint = 0x8DC;
pub const A5PSW_etherStatsFragments: c_uint = 0x8E0;
pub const A5PSW_VLANReceived: c_uint = 0x8E8;
pub const A5PSW_VLANTransmitted: c_uint = 0x8EC;
pub const A5PSW_aDeferred: c_uint = 0x910;
pub const A5PSW_aMultipleCollisions: c_uint = 0x914;
pub const A5PSW_aSingleCollisions: c_uint = 0x918;
pub const A5PSW_aLateCollisions: c_uint = 0x91C;
pub const A5PSW_aExcessiveCollisions: c_uint = 0x920;
pub const A5PSW_aCarrierSenseErrors: c_uint = 0x924;

pub const A5PSW_PORTS_NUM: c_int = 5;

pub const A5PSW_MDIO_DEF_FREQ: c_int = 2500000;
pub const A5PSW_MDIO_TIMEOUT: c_int = 100;

pub const A5PSW_MDIO_CLK_DIV_MIN: c_int = 5;
pub const A5PSW_TAG_LEN: c_int = 8;
pub const A5PSW_VLAN_COUNT: c_int = 32;
// Ensure enough space for 2 VLAN tags

pub const A5PSW_PATTERN_MGMTFWD: c_int = 0;
pub const A5PSW_LK_BUSY_USEC_POLL: c_int = 10;
pub const A5PSW_CTRL_TIMEOUT: c_int = 1000;
pub const A5PSW_TABLE_ENTRIES: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_entry {
    pub mac: [u8; ETH_ALEN],
    pub valid:1: u16,
    pub is_static:1: u16,
    pub prio:3: u16,
    pub port_mask:5: u16,
    pub reserved:6: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union lk_data {
    pub lo: u32,
    pub hi: u32,
}

//
// struct a5psw - switch struct
// @base: Base address of the switch
// @hclk: hclk_switch clock
// @clk: clk_switch clock
// @dev: Device associated to the switch
// @mii_bus: MDIO bus struct
// @mdio_freq: MDIO bus frequency requested
// @pcs: Array of PCS connected to the switch ports (not for the CPU)
// @ds: DSA switch struct
// @stats_lock: lock to access statistics (shared HI counter)
// @lk_lock: Lock for the lookup table
// @reg_lock: Lock for register read-modify-write operation
// @bridged_ports: Mask of ports that are bridged and should be flooded
// @br_dev: Bridge net device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a5psw {
    pub base: *mut void __iomem,
    pub hclk: *mut clk,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub mii_bus: *mut mii_bus,
    pub 1]: *mut *mut phylink_pcs pcs[A5PSW_PORTS_NUM -,
    pub ds: dsa_switch,
    pub lk_lock: mutex,
    pub reg_lock: spinlock_t,
    pub bridged_ports: u32,
    pub br_dev: *mut net_device,
}
