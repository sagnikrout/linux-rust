//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic_hdr.h
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
// QLogic qlcnic NIC Driver
// Copyright (c) 2009-2013 QLogic Corporation
//

//
// The basic unit of access when reading/writing control registers.
//
// Hub 0
// Hub 1
// Hub 2
// Hub 3
// Hub 4
// Hub 5
// Hub 6
// Floaters - non existent modules
pub const QLCNIC_HW_EFC_RPMX0_CRB_AGT_ADR: c_uint = 0x67;
// This field defines PCI/X adr [25:20] of agents on the CRB
pub const BIT_0: c_uint = 0x1;
pub const BIT_1: c_uint = 0x2;
pub const BIT_2: c_uint = 0x4;
pub const BIT_3: c_uint = 0x8;
pub const BIT_4: c_uint = 0x10;
pub const BIT_5: c_uint = 0x20;
pub const BIT_6: c_uint = 0x40;
pub const BIT_7: c_uint = 0x80;
pub const BIT_8: c_uint = 0x100;
pub const BIT_9: c_uint = 0x200;
pub const BIT_10: c_uint = 0x400;
pub const BIT_11: c_uint = 0x800;
pub const BIT_12: c_uint = 0x1000;
pub const BIT_13: c_uint = 0x2000;
pub const BIT_14: c_uint = 0x4000;
pub const BIT_15: c_uint = 0x8000;
pub const BIT_16: c_uint = 0x10000;
pub const BIT_17: c_uint = 0x20000;
pub const BIT_18: c_uint = 0x40000;
pub const BIT_19: c_uint = 0x80000;
pub const BIT_20: c_uint = 0x100000;
pub const BIT_21: c_uint = 0x200000;
pub const BIT_22: c_uint = 0x400000;
pub const BIT_23: c_uint = 0x800000;
pub const BIT_24: c_uint = 0x1000000;
pub const BIT_25: c_uint = 0x2000000;
pub const BIT_26: c_uint = 0x4000000;
pub const BIT_27: c_uint = 0x8000000;
pub const BIT_28: c_uint = 0x10000000;
pub const BIT_29: c_uint = 0x20000000;
pub const BIT_30: c_uint = 0x40000000;
pub const BIT_31: c_uint = 0x80000000;
// This field defines CRB adr [31:20] of the agents

//
// Definitions specific to M25P flash
//
// all are 1MB windows
pub const QLCNIC_PCI_CRB_WINDOWSIZE: c_uint = 0x00100000;

//
// Register offsets for MN
//

// 200ms delay in each loop
pub const QLCNIC_NIU_PHY_WAITLEN: c_int = 200000;
// 10 seconds before we give up
pub const QLCNIC_NIU_PHY_WAITMAX: c_int = 50;
pub const QLCNIC_NIU_MAX_GBE_PORTS: c_int = 4;
pub const QLCNIC_NIU_MAX_XG_PORTS: c_int = 2;

pub const MAX_CTL_CHECK: c_int = 1000;

// XG Link status
pub const XG_LINK_UP: c_uint = 0x10;
pub const XG_LINK_DOWN: c_uint = 0x20;
pub const XG_LINK_UP_P3P: c_uint = 0x01;
pub const XG_LINK_DOWN_P3P: c_uint = 0x02;
pub const XG_LINK_STATE_P3P_MASK: c_uint = 0xf;

pub const P3P_LINK_SPEED_MHZ: c_int = 100;
pub const P3P_LINK_SPEED_MASK: c_uint = 0xff;

pub const QLCNIC_CDRP_MAX_ARGS: c_int = 4;

//
// CrbPortPhanCntrHi/Lo is used to pass the address of HostPhantomIndex address
// which can be read by the Phantom host to get producer/consumer indexes from
// Phantom/Casper. If it is not HOST_SHARED_MEMORY, then the following
// registers will be used for the addresses of the ring's shared memory
// on the Phantom.
//

//
// Temperature control.
//
// Lock IDs for PHY lock
pub const PHY_LOCK_DRIVER: c_uint = 0x44524956;

pub const QLCNIC_PORT_MODE_NONE: c_int = 0;
pub const QLCNIC_PORT_MODE_XG: c_int = 1;
pub const QLCNIC_PORT_MODE_GB: c_int = 2;
pub const QLCNIC_PORT_MODE_802_3_AP: c_int = 3;
pub const QLCNIC_PORT_MODE_AUTO_NEG: c_int = 4;
pub const QLCNIC_PORT_MODE_AUTO_NEG_1G: c_int = 5;
pub const QLCNIC_PORT_MODE_AUTO_NEG_XG: c_int = 6;

pub const QLCNIC_PEG_TUNE_MN_PRESENT: c_uint = 0x1;

// Device State
pub const QLCNIC_DEV_COLD: c_uint = 0x1;
pub const QLCNIC_DEV_INITIALIZING: c_uint = 0x2;
pub const QLCNIC_DEV_READY: c_uint = 0x3;
pub const QLCNIC_DEV_NEED_RESET: c_uint = 0x4;
pub const QLCNIC_DEV_NEED_QUISCENT: c_uint = 0x5;
pub const QLCNIC_DEV_FAILED: c_uint = 0x6;
pub const QLCNIC_DEV_QUISCENT: c_uint = 0x7;
pub const QLCNIC_DEV_BADBAD: c_uint = 0xbad0bad0;

pub const QLCNIC_TYPE_NIC: c_int = 1;
pub const QLCNIC_TYPE_FCOE: c_int = 2;
pub const QLCNIC_TYPE_ISCSI: c_int = 3;
pub const QLCNIC_RCODE_DRIVER_INFO: c_uint = 0x20000000;

pub const QLCNIC_FWERROR_FAN_FAILURE: c_uint = 0x16;

pub const FW_FAIL_THRESH: c_int = 2;
pub const QLCNIC_RESET_TIMEOUT_SECS: c_int = 10;
pub const QLCNIC_INIT_TIMEOUT_SECS: c_int = 30;
pub const QLCNIC_RCVPEG_CHECK_RETRY_COUNT: c_int = 2000;
pub const QLCNIC_RCVPEG_CHECK_DELAY: c_int = 10;
pub const QLCNIC_CMDPEG_CHECK_RETRY_COUNT: c_int = 60;
pub const QLCNIC_CMDPEG_CHECK_DELAY: c_int = 500;
pub const QLCNIC_HEARTBEAT_PERIOD_MSECS: c_int = 200;
pub const QLCNIC_HEARTBEAT_CHECK_RETRY_COUNT: c_int = 10;
pub const QLCNIC_MAX_MC_COUNT: c_int = 38;
pub const QLCNIC_MAX_UC_COUNT: c_int = 512;
pub const QLCNIC_WATCHDOG_TIMEOUTVALUE: c_int = 5;

//
// PCI Interrupt Vector Values.
//
pub const PCIX_INT_VECTOR_BIT_F0: c_uint = 0x0080;
pub const PCIX_INT_VECTOR_BIT_F1: c_uint = 0x0100;
pub const PCIX_INT_VECTOR_BIT_F2: c_uint = 0x0200;
pub const PCIX_INT_VECTOR_BIT_F3: c_uint = 0x0400;
pub const PCIX_INT_VECTOR_BIT_F4: c_uint = 0x0800;
pub const PCIX_INT_VECTOR_BIT_F5: c_uint = 0x1000;
pub const PCIX_INT_VECTOR_BIT_F6: c_uint = 0x2000;
pub const PCIX_INT_VECTOR_BIT_F7: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_legacy_intr_set {
    pub int_vec_bit: u32,
    pub tgt_status_reg: u32,
    pub tgt_mask_reg: u32,
    pub pci_int_reg: u32,
}

pub const QLCNIC_MSIX_BASE: c_uint = 0x132110;
pub const QLCNIC_MAX_VLAN_FILTERS: c_int = 64;
pub const FLASH_ROM_WINDOW: c_uint = 0x42110030;
pub const FLASH_ROM_DATA: c_uint = 0x42150000;
pub const QLCNIC_FW_DUMP_REG1: c_uint = 0x00130060;
pub const QLCNIC_FW_DUMP_REG2: c_uint = 0x001e0000;
pub const QLCNIC_FLASH_SEM2_LK: c_uint = 0x0013C010;
pub const QLCNIC_FLASH_SEM2_ULK: c_uint = 0x0013C014;
pub const QLCNIC_FLASH_LOCK_ID: c_uint = 0x001B2100;
// PCI function operational mode
pub const QLC_DEV_DRV_DEFAULT: c_uint = 0x11111111;

pub const QLCNIC_MS_CTRL: c_uint = 0x41000090;
pub const QLCNIC_MS_ADDR_LO: c_uint = 0x41000094;
pub const QLCNIC_MS_ADDR_HI: c_uint = 0x41000098;
pub const QLCNIC_MS_WRTDATA_LO: c_uint = 0x410000A0;
pub const QLCNIC_MS_WRTDATA_HI: c_uint = 0x410000A4;
pub const QLCNIC_MS_WRTDATA_ULO: c_uint = 0x410000B0;
pub const QLCNIC_MS_WRTDATA_UHI: c_uint = 0x410000B4;
pub const QLCNIC_MS_RDDATA_LO: c_uint = 0x410000A8;
pub const QLCNIC_MS_RDDATA_HI: c_uint = 0x410000AC;
pub const QLCNIC_MS_RDDATA_ULO: c_uint = 0x410000B8;
pub const QLCNIC_MS_RDDATA_UHI: c_uint = 0x410000BC;

// NIU REGS

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
// NIU GB Pause Ctl Register
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
// PHY-Specific MII control/status registers.
//
pub const QLCNIC_NIU_GB_MII_MGMT_ADDR_AUTONEG: c_int = 4;
pub const QLCNIC_NIU_GB_MII_MGMT_ADDR_PHY_STATUS: c_int = 17;
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

pub const QLCNIC_NIU_NON_PROMISC_MODE: c_int = 0;
pub const QLCNIC_NIU_PROMISC_MODE: c_int = 1;
pub const QLCNIC_NIU_ALLMULTI_MODE: c_int = 2;
pub const QLCNIC_PCIE_SEM_TIMEOUT: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crb_128M_2M_sub_block_map {
    pub valid: unsigned,
    pub start_128M: unsigned,
    pub end_128M: unsigned,
    pub start_2M: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crb_128M_2M_block_map {
    pub sub_block: [crb_128M_2M_sub_block_map; 16],
}
