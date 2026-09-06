//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_reg.h
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//
// SXGBE MAC Registers
pub const SXGBE_CORE_TX_CONFIG_REG: c_uint = 0x0000;
pub const SXGBE_CORE_RX_CONFIG_REG: c_uint = 0x0004;
pub const SXGBE_CORE_PKT_FILTER_REG: c_uint = 0x0008;
pub const SXGBE_CORE_WATCHDOG_TIMEOUT_REG: c_uint = 0x000C;
pub const SXGBE_CORE_HASH_TABLE_REG0: c_uint = 0x0010;
pub const SXGBE_CORE_HASH_TABLE_REG1: c_uint = 0x0014;
pub const SXGBE_CORE_HASH_TABLE_REG2: c_uint = 0x0018;
pub const SXGBE_CORE_HASH_TABLE_REG3: c_uint = 0x001C;
pub const SXGBE_CORE_HASH_TABLE_REG4: c_uint = 0x0020;
pub const SXGBE_CORE_HASH_TABLE_REG5: c_uint = 0x0024;
pub const SXGBE_CORE_HASH_TABLE_REG6: c_uint = 0x0028;
pub const SXGBE_CORE_HASH_TABLE_REG7: c_uint = 0x002C;
// EEE-LPI Registers
pub const SXGBE_CORE_LPI_CTRL_STATUS: c_uint = 0x00D0;
pub const SXGBE_CORE_LPI_TIMER_CTRL: c_uint = 0x00D4;
// VLAN Specific Registers
pub const SXGBE_CORE_VLAN_TAG_REG: c_uint = 0x0050;
pub const SXGBE_CORE_VLAN_HASHTAB_REG: c_uint = 0x0058;
pub const SXGBE_CORE_VLAN_INSCTL_REG: c_uint = 0x0060;
pub const SXGBE_CORE_VLAN_INNERCTL_REG: c_uint = 0x0064;
pub const SXGBE_CORE_RX_ETHTYPE_MATCH_REG: c_uint = 0x006C;
// Flow Contol Registers
pub const SXGBE_CORE_TX_Q0_FLOWCTL_REG: c_uint = 0x0070;
pub const SXGBE_CORE_TX_Q1_FLOWCTL_REG: c_uint = 0x0074;
pub const SXGBE_CORE_TX_Q2_FLOWCTL_REG: c_uint = 0x0078;
pub const SXGBE_CORE_TX_Q3_FLOWCTL_REG: c_uint = 0x007C;
pub const SXGBE_CORE_TX_Q4_FLOWCTL_REG: c_uint = 0x0080;
pub const SXGBE_CORE_TX_Q5_FLOWCTL_REG: c_uint = 0x0084;
pub const SXGBE_CORE_TX_Q6_FLOWCTL_REG: c_uint = 0x0088;
pub const SXGBE_CORE_TX_Q7_FLOWCTL_REG: c_uint = 0x008C;
pub const SXGBE_CORE_RX_FLOWCTL_REG: c_uint = 0x0090;
pub const SXGBE_CORE_RX_CTL0_REG: c_uint = 0x00A0;
pub const SXGBE_CORE_RX_CTL1_REG: c_uint = 0x00A4;
pub const SXGBE_CORE_RX_CTL2_REG: c_uint = 0x00A8;
pub const SXGBE_CORE_RX_CTL3_REG: c_uint = 0x00AC;
pub const SXGBE_CORE_RXQ_ENABLE_MASK: c_uint = 0x0003;
pub const SXGBE_CORE_RXQ_ENABLE: c_uint = 0x0002;
pub const SXGBE_CORE_RXQ_DISABLE: c_uint = 0x0000;
// Interrupt Registers
pub const SXGBE_CORE_INT_STATUS_REG: c_uint = 0x00B0;
pub const SXGBE_CORE_INT_ENABLE_REG: c_uint = 0x00B4;
pub const SXGBE_CORE_RXTX_ERR_STATUS_REG: c_uint = 0x00B8;
pub const SXGBE_CORE_PMT_CTL_STATUS_REG: c_uint = 0x00C0;
pub const SXGBE_CORE_RWK_PKT_FILTER_REG: c_uint = 0x00C4;
pub const SXGBE_CORE_VERSION_REG: c_uint = 0x0110;
pub const SXGBE_CORE_DEBUG_REG: c_uint = 0x0114;

// SMA(MDIO) module registers
pub const SXGBE_MDIO_SCMD_ADD_REG: c_uint = 0x0200;
pub const SXGBE_MDIO_SCMD_DATA_REG: c_uint = 0x0204;
pub const SXGBE_MDIO_CCMD_WADD_REG: c_uint = 0x0208;
pub const SXGBE_MDIO_CCMD_WDATA_REG: c_uint = 0x020C;
pub const SXGBE_MDIO_CSCAN_PORT_REG: c_uint = 0x0210;
pub const SXGBE_MDIO_INT_STATUS_REG: c_uint = 0x0214;
pub const SXGBE_MDIO_INT_ENABLE_REG: c_uint = 0x0218;
pub const SXGBE_MDIO_PORT_CONDCON_REG: c_uint = 0x021C;
pub const SXGBE_MDIO_CLAUSE22_PORT_REG: c_uint = 0x0220;
// port specific, addr = 0-3
pub const SXGBE_MDIO_DEV_BASE_REG: c_uint = 0x0230;

pub const SXGBE_CORE_GPIO_CTL_REG: c_uint = 0x0278;
pub const SXGBE_CORE_GPIO_STATUS_REG: c_uint = 0x027C;
// Address registers for filtering
pub const SXGBE_CORE_ADD_BASE_REG: c_uint = 0x0300;
// addr = 0-31

// SXGBE MMC registers
pub const SXGBE_MMC_CTL_REG: c_uint = 0x0800;
pub const SXGBE_MMC_RXINT_STATUS_REG: c_uint = 0x0804;
pub const SXGBE_MMC_TXINT_STATUS_REG: c_uint = 0x0808;
pub const SXGBE_MMC_RXINT_ENABLE_REG: c_uint = 0x080C;
pub const SXGBE_MMC_TXINT_ENABLE_REG: c_uint = 0x0810;
// TX specific counters
pub const SXGBE_MMC_TXOCTETHI_GBCNT_REG: c_uint = 0x0814;
pub const SXGBE_MMC_TXOCTETLO_GBCNT_REG: c_uint = 0x0818;
pub const SXGBE_MMC_TXFRAMELO_GBCNT_REG: c_uint = 0x081C;
pub const SXGBE_MMC_TXFRAMEHI_GBCNT_REG: c_uint = 0x0820;
pub const SXGBE_MMC_TXBROADLO_GCNT_REG: c_uint = 0x0824;
pub const SXGBE_MMC_TXBROADHI_GCNT_REG: c_uint = 0x0828;
pub const SXGBE_MMC_TXMULTILO_GCNT_REG: c_uint = 0x082C;
pub const SXGBE_MMC_TXMULTIHI_GCNT_REG: c_uint = 0x0830;
pub const SXGBE_MMC_TX64LO_GBCNT_REG: c_uint = 0x0834;
pub const SXGBE_MMC_TX64HI_GBCNT_REG: c_uint = 0x0838;
pub const SXGBE_MMC_TX65TO127LO_GBCNT_REG: c_uint = 0x083C;
pub const SXGBE_MMC_TX65TO127HI_GBCNT_REG: c_uint = 0x0840;
pub const SXGBE_MMC_TX128TO255LO_GBCNT_REG: c_uint = 0x0844;
pub const SXGBE_MMC_TX128TO255HI_GBCNT_REG: c_uint = 0x0848;
pub const SXGBE_MMC_TX256TO511LO_GBCNT_REG: c_uint = 0x084C;
pub const SXGBE_MMC_TX256TO511HI_GBCNT_REG: c_uint = 0x0850;
pub const SXGBE_MMC_TX512TO1023LO_GBCNT_REG: c_uint = 0x0854;
pub const SXGBE_MMC_TX512TO1023HI_GBCNT_REG: c_uint = 0x0858;
pub const SXGBE_MMC_TX1023TOMAXLO_GBCNT_REG: c_uint = 0x085C;
pub const SXGBE_MMC_TX1023TOMAXHI_GBCNT_REG: c_uint = 0x0860;
pub const SXGBE_MMC_TXUNICASTLO_GBCNT_REG: c_uint = 0x0864;
pub const SXGBE_MMC_TXUNICASTHI_GBCNT_REG: c_uint = 0x0868;
pub const SXGBE_MMC_TXMULTILO_GBCNT_REG: c_uint = 0x086C;
pub const SXGBE_MMC_TXMULTIHI_GBCNT_REG: c_uint = 0x0870;
pub const SXGBE_MMC_TXBROADLO_GBCNT_REG: c_uint = 0x0874;
pub const SXGBE_MMC_TXBROADHI_GBCNT_REG: c_uint = 0x0878;
pub const SXGBE_MMC_TXUFLWLO_GBCNT_REG: c_uint = 0x087C;
pub const SXGBE_MMC_TXUFLWHI_GBCNT_REG: c_uint = 0x0880;
pub const SXGBE_MMC_TXOCTETLO_GCNT_REG: c_uint = 0x0884;
pub const SXGBE_MMC_TXOCTETHI_GCNT_REG: c_uint = 0x0888;
pub const SXGBE_MMC_TXFRAMELO_GCNT_REG: c_uint = 0x088C;
pub const SXGBE_MMC_TXFRAMEHI_GCNT_REG: c_uint = 0x0890;
pub const SXGBE_MMC_TXPAUSELO_CNT_REG: c_uint = 0x0894;
pub const SXGBE_MMC_TXPAUSEHI_CNT_REG: c_uint = 0x0898;
pub const SXGBE_MMC_TXVLANLO_GCNT_REG: c_uint = 0x089C;
pub const SXGBE_MMC_TXVLANHI_GCNT_REG: c_uint = 0x08A0;
// RX specific counters
pub const SXGBE_MMC_RXFRAMELO_GBCNT_REG: c_uint = 0x0900;
pub const SXGBE_MMC_RXFRAMEHI_GBCNT_REG: c_uint = 0x0904;
pub const SXGBE_MMC_RXOCTETLO_GBCNT_REG: c_uint = 0x0908;
pub const SXGBE_MMC_RXOCTETHI_GBCNT_REG: c_uint = 0x090C;
pub const SXGBE_MMC_RXOCTETLO_GCNT_REG: c_uint = 0x0910;
pub const SXGBE_MMC_RXOCTETHI_GCNT_REG: c_uint = 0x0914;
pub const SXGBE_MMC_RXBROADLO_GCNT_REG: c_uint = 0x0918;
pub const SXGBE_MMC_RXBROADHI_GCNT_REG: c_uint = 0x091C;
pub const SXGBE_MMC_RXMULTILO_GCNT_REG: c_uint = 0x0920;
pub const SXGBE_MMC_RXMULTIHI_GCNT_REG: c_uint = 0x0924;
pub const SXGBE_MMC_RXCRCERRLO_REG: c_uint = 0x0928;
pub const SXGBE_MMC_RXCRCERRHI_REG: c_uint = 0x092C;
pub const SXGBE_MMC_RXSHORT64BFRAME_ERR_REG: c_uint = 0x0930;
pub const SXGBE_MMC_RXJABBERERR_REG: c_uint = 0x0934;
pub const SXGBE_MMC_RXSHORT64BFRAME_COR_REG: c_uint = 0x0938;
pub const SXGBE_MMC_RXOVERMAXFRAME_COR_REG: c_uint = 0x093C;
pub const SXGBE_MMC_RX64LO_GBCNT_REG: c_uint = 0x0940;
pub const SXGBE_MMC_RX64HI_GBCNT_REG: c_uint = 0x0944;
pub const SXGBE_MMC_RX65TO127LO_GBCNT_REG: c_uint = 0x0948;
pub const SXGBE_MMC_RX65TO127HI_GBCNT_REG: c_uint = 0x094C;
pub const SXGBE_MMC_RX128TO255LO_GBCNT_REG: c_uint = 0x0950;
pub const SXGBE_MMC_RX128TO255HI_GBCNT_REG: c_uint = 0x0954;
pub const SXGBE_MMC_RX256TO511LO_GBCNT_REG: c_uint = 0x0958;
pub const SXGBE_MMC_RX256TO511HI_GBCNT_REG: c_uint = 0x095C;
pub const SXGBE_MMC_RX512TO1023LO_GBCNT_REG: c_uint = 0x0960;
pub const SXGBE_MMC_RX512TO1023HI_GBCNT_REG: c_uint = 0x0964;
pub const SXGBE_MMC_RX1023TOMAXLO_GBCNT_REG: c_uint = 0x0968;
pub const SXGBE_MMC_RX1023TOMAXHI_GBCNT_REG: c_uint = 0x096C;
pub const SXGBE_MMC_RXUNICASTLO_GCNT_REG: c_uint = 0x0970;
pub const SXGBE_MMC_RXUNICASTHI_GCNT_REG: c_uint = 0x0974;
pub const SXGBE_MMC_RXLENERRLO_REG: c_uint = 0x0978;
pub const SXGBE_MMC_RXLENERRHI_REG: c_uint = 0x097C;
pub const SXGBE_MMC_RXOUTOFRANGETYPELO_REG: c_uint = 0x0980;
pub const SXGBE_MMC_RXOUTOFRANGETYPEHI_REG: c_uint = 0x0984;
pub const SXGBE_MMC_RXPAUSELO_CNT_REG: c_uint = 0x0988;
pub const SXGBE_MMC_RXPAUSEHI_CNT_REG: c_uint = 0x098C;
pub const SXGBE_MMC_RXFIFOOVERFLOWLO_GBCNT_REG: c_uint = 0x0990;
pub const SXGBE_MMC_RXFIFOOVERFLOWHI_GBCNT_REG: c_uint = 0x0994;
pub const SXGBE_MMC_RXVLANLO_GBCNT_REG: c_uint = 0x0998;
pub const SXGBE_MMC_RXVLANHI_GBCNT_REG: c_uint = 0x099C;
pub const SXGBE_MMC_RXWATCHDOG_ERR_REG: c_uint = 0x09A0;
// L3/L4 function registers
pub const SXGBE_CORE_L34_ADDCTL_REG: c_uint = 0x0C00;
pub const SXGBE_CORE_L34_DATA_REG: c_uint = 0x0C04;
// ARP registers
pub const SXGBE_CORE_ARP_ADD_REG: c_uint = 0x0C10;
// RSS registers
pub const SXGBE_CORE_RSS_CTL_REG: c_uint = 0x0C80;
pub const SXGBE_CORE_RSS_ADD_REG: c_uint = 0x0C88;
pub const SXGBE_CORE_RSS_DATA_REG: c_uint = 0x0C8C;
// RSS control register bits

// IEEE 1588 registers
pub const SXGBE_CORE_TSTAMP_CTL_REG: c_uint = 0x0D00;
pub const SXGBE_CORE_SUBSEC_INC_REG: c_uint = 0x0D04;
pub const SXGBE_CORE_SYSTIME_SEC_REG: c_uint = 0x0D0C;
pub const SXGBE_CORE_SYSTIME_NSEC_REG: c_uint = 0x0D10;
pub const SXGBE_CORE_SYSTIME_SECUP_REG: c_uint = 0x0D14;
pub const SXGBE_CORE_TSTAMP_ADD_REG: c_uint = 0x0D18;
pub const SXGBE_CORE_SYSTIME_HWORD_REG: c_uint = 0x0D1C;
pub const SXGBE_CORE_TSTAMP_STATUS_REG: c_uint = 0x0D20;
pub const SXGBE_CORE_TXTIME_STATUSNSEC_REG: c_uint = 0x0D30;
pub const SXGBE_CORE_TXTIME_STATUSSEC_REG: c_uint = 0x0D34;
// Auxiliary registers
pub const SXGBE_CORE_AUX_CTL_REG: c_uint = 0x0D40;
pub const SXGBE_CORE_AUX_TSTAMP_NSEC_REG: c_uint = 0x0D48;
pub const SXGBE_CORE_AUX_TSTAMP_SEC_REG: c_uint = 0x0D4C;
pub const SXGBE_CORE_AUX_TSTAMP_INGCOR_REG: c_uint = 0x0D50;
pub const SXGBE_CORE_AUX_TSTAMP_ENGCOR_REG: c_uint = 0x0D54;
pub const SXGBE_CORE_AUX_TSTAMP_INGCOR_NSEC_REG: c_uint = 0x0D58;
pub const SXGBE_CORE_AUX_TSTAMP_INGCOR_SUBNSEC_REG: c_uint = 0x0D5C;
pub const SXGBE_CORE_AUX_TSTAMP_ENGCOR_NSEC_REG: c_uint = 0x0D60;
pub const SXGBE_CORE_AUX_TSTAMP_ENGCOR_SUBNSEC_REG: c_uint = 0x0D64;
// PPS registers
pub const SXGBE_CORE_PPS_CTL_REG: c_uint = 0x0D70;
pub const SXGBE_CORE_PPS_BASE: c_uint = 0x0D80;
// addr = 0 - 3

pub const SXGBE_CORE_PTO_CTL_REG: c_uint = 0x0DC0;
pub const SXGBE_CORE_SRCPORT_ITY0_REG: c_uint = 0x0DC4;
pub const SXGBE_CORE_SRCPORT_ITY1_REG: c_uint = 0x0DC8;
pub const SXGBE_CORE_SRCPORT_ITY2_REG: c_uint = 0x0DCC;
pub const SXGBE_CORE_LOGMSG_LEVEL_REG: c_uint = 0x0DD0;
// SXGBE MTL Registers
pub const SXGBE_MTL_BASE_REG: c_uint = 0x1000;

// TC/Queue registers, qnum=0-15

pub const SXGBE_MTL_FIFO_LSHIFT: c_int = 16;
pub const SXGBE_MTL_ENABLE_QUEUE: c_uint = 0x00000008;

pub const SXGBE_MTL_TC_RXBASE_REG: c_uint = 0x1140;

// SXGBE DMA Registers
pub const SXGBE_DMA_BASE_REG: c_uint = 0x3000;

// Channel Registers, cha_num = 0-15

// TX DMA control register specific

// sxgbe tx configuration register bitfields
pub const SXGBE_SPEED_10G: c_uint = 0x0;
pub const SXGBE_SPEED_2_5G: c_uint = 0x1;
pub const SXGBE_SPEED_1G: c_uint = 0x2;
pub const SXGBE_SPEED_LSHIFT: c_int = 29;

// sxgbe rx configuration register bitfields

// sxgbe vlan Tag Register bitfields

// XMAC VLAN Tag Inclusion Register(0x0060) bitfields
// Below fields same for  Inner VLAN Tag Inclusion
// Register(0x0064) register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlan_tag_ctl_tx {
    VLAN_TAG_TX_NOP,
    VLAN_TAG_TX_DEL,
    VLAN_TAG_TX_INSERT,
    VLAN_TAG_TX_REPLACE
}

// SXGBE TX Q Flow Control Register bitfields

// SXGBE RX Q Flow Control Register bitfields

// sxgbe rx Q control0 register bitfields
pub const SXGBE_RX_Q_ENABLE: c_uint = 0x2;
// SXGBE hardware features bitfield specific
// Capability Register 0

// Capability Register 1

// Capability Register 2

// DMAchannel interrupt enable specific
// DMA Normal interrupt

// DMA Abnormal interrupt

// DMA channel interrupt status specific

