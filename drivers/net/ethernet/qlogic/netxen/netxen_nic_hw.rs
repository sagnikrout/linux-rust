//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/netxen/netxen_nic_hw.h
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
// Copyright (C) 2003 - 2009 NetXen, Inc.
// Copyright (C) 2009 - QLogic Corporation.
// All rights reserved.
//
// Hardware memory size of 128 meg

extern "C" {
    pub fn netxen_nic_set_link_parameters(adapter: *mut netxen_adapter);
}
// Nibble or Byte mode for phy interface (GbE mode only)

//
// NIU GB MAC Config Register 0 (applies to GB0, GB1, GB2, GB3)
//
// Bit 0 : enable_tx => 1:enable frame xmit, 0:disable
// Bit 1 : tx_synced => R/O: xmit enable synched to xmit stream
// Bit 2 : enable_rx => 1:enable frame recv, 0:disable
// Bit 3 : rx_synced => R/O: recv enable synched to recv stream
// Bit 4 : tx_flowctl => 1:enable pause frame generation, 0:disable
// Bit 5 : rx_flowctl => 1:act on recv'd pause frames, 0:ignore
// Bit 8 : loopback => 1:loop MAC xmits to MAC recvs, 0:normal
// Bit 16: tx_reset_pb => 1:reset frame xmit protocol blk, 0:no-op
// Bit 17: rx_reset_pb => 1:reset frame recv protocol blk, 0:no-op
// Bit 18: tx_reset_mac => 1:reset data/ctl multiplexer blk, 0:no-op
// Bit 19: rx_reset_mac => 1:reset ctl frames & timers blk, 0:no-op
// Bit 31: soft_reset => 1:reset the MAC and the SERDES, 0:no-op
//

//
// NIU GB MII Mgmt Command Register (applies to GB0, GB1, GB2, GB3)
// Bit 0 : read_cycle => 1:perform single read cycle, 0:no-op
// Bit 1 : scan_cycle => 1:perform continuous read cycles, 0:no-op
//

//
// NIU GB MII Mgmt Indicators Register (applies to GB0, GB1, GB2, GB3)
// Read-only register.
// Bit 0 : busy => 1:performing an MII mgmt cycle, 0:idle
// Bit 1 : scanning => 1:scan operation in progress, 0:idle
// Bit 2 : notvalid => :mgmt result data not yet valid, 0:idle
//

//
// NIU XG Pause Ctl Register
//
// Bit 0       : xg0_mask => 1:disable tx pause frames
// Bit 1       : xg0_request => 1:request single pause frame
// Bit 2       : xg0_on_off => 1:request is pause on, 0:off
// Bit 3       : xg1_mask => 1:disable tx pause frames
// Bit 4       : xg1_request => 1:request single pause frame
// Bit 5       : xg1_on_off => 1:request is pause on, 0:off
//

//
// NIU XG Pause Ctl Register
//
// Bit 0       : xg0_mask => 1:disable tx pause frames
// Bit 1       : xg0_request => 1:request single pause frame
// Bit 2       : xg0_on_off => 1:request is pause on, 0:off
// Bit 3       : xg1_mask => 1:disable tx pause frames
// Bit 4       : xg1_request => 1:request single pause frame
// Bit 5       : xg1_on_off => 1:request is pause on, 0:off
//

//
// PHY-Specific MII control/status registers.
//
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_CONTROL: c_int = 0;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_STATUS: c_int = 1;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_ID_0: c_int = 2;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_ID_1: c_int = 3;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_AUTONEG: c_int = 4;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_LNKPART: c_int = 5;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_AUTONEG_MORE: c_int = 6;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_NEXTPAGE_XMIT: c_int = 7;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_LNKPART_NEXTPAGE: c_int = 8;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_1000BT_CONTROL: c_int = 9;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_1000BT_STATUS: c_int = 10;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_EXTENDED_STATUS: c_int = 15;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_CONTROL: c_int = 16;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_STATUS: c_int = 17;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_INT_ENABLE: c_int = 18;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_INT_STATUS: c_int = 19;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_CONTROL_MORE: c_int = 20;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_RECV_ERROR_COUNT: c_int = 21;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_LED_CONTROL: c_int = 24;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_LED_OVERRIDE: c_int = 25;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_CONTROL_MORE_YET: c_int = 26;
pub const NETXEN_NIU_GB_MII_MGMT_ADDR_PHY_STATUS_MORE: c_int = 27;
//
// PHY-Specific Status Register (reg 17).
//
// Bit 0      : jabber => 1:jabber detected, 0:not
// Bit 1      : polarity => 1:polarity reversed, 0:normal
// Bit 2      : recvpause => 1:receive pause enabled, 0:disabled
// Bit 3      : xmitpause => 1:transmit pause enabled, 0:disabled
// Bit 4      : energydetect => 1:sleep, 0:active
// Bit 5      : downshift => 1:downshift, 0:no downshift
// Bit 6      : crossover => 1:MDIX (crossover), 0:MDI (no crossover)
// Bits 7-9   : cablelen => not valid in 10Mb/s mode
// 0:<50m, 1:50-80m, 2:80-110m, 3:110-140m, 4:>140m
// Bit 10     : link => 1:link up, 0:link down
// Bit 11     : resolved => 1:speed and duplex resolved, 0:not yet
// Bit 12     : pagercvd => 1:page received, 0:page not received
// Bit 13     : duplex => 1:full duplex, 0:half duplex
// Bits 14-15 : speed => 0:10Mb/s, 1:100Mb/s, 2:1000Mb/s, 3:rsvd
//

//
// NIU Mode Register.
// Bit 0 : enable FibreChannel
// Bit 1 : enable 10/100/1000 Ethernet
// Bit 2 : enable 10Gb Ethernet
//

pub const NETXEN_NIU_NON_PROMISC_MODE: c_int = 0;
pub const NETXEN_NIU_PROMISC_MODE: c_int = 1;
pub const NETXEN_NIU_ALLMULTI_MODE: c_int = 2;
//
// NIU XG MAC Config Register
//
// Bit 0 : tx_enable => 1:enable frame xmit, 0:disable
// Bit 2 : rx_enable => 1:enable frame recv, 0:disable
// Bit 4 : soft_reset => 1:reset the MAC , 0:no-op
// Bit 27: xaui_framer_reset
// Bit 28: xaui_rx_reset
// Bit 29: xaui_tx_reset
// Bit 30: xg_ingress_afifo_reset
// Bit 31: xg_egress_afifo_reset
//

