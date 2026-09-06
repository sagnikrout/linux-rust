//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atlx/atlx.h
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
// atlx_hw.h -- common hardware definitions for Attansic network drivers
//
// Copyright(c) 2005 - 2006 Attansic Corporation. All rights reserved.
// Copyright(c) 2006 - 2007 Chris Snook <csnook@redhat.com>
// Copyright(c) 2006 - 2008 Jay Cliburn <jcliburn@gmail.com>
// Copyright(c) 2007 Atheros Corporation. All rights reserved.
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

pub const ATLX_ERR_PHY: c_int = 2;
pub const ATLX_ERR_PHY_SPEED: c_int = 7;
pub const ATLX_ERR_PHY_RES: c_int = 8;
pub const SPEED_0: c_uint = 0xffff;
pub const SPEED_10: c_int = 10;
pub const SPEED_100: c_int = 100;
pub const SPEED_1000: c_int = 1000;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
pub const MEDIA_TYPE_AUTO_SENSOR: c_int = 0;
// register definitions
pub const REG_PM_CTRLSTAT: c_uint = 0x44;
pub const REG_PCIE_CAP_LIST: c_uint = 0x58;
pub const REG_VPD_CAP: c_uint = 0x6C;
pub const VPD_CAP_ID_MASK: c_uint = 0xFF;
pub const VPD_CAP_ID_SHIFT: c_int = 0;
pub const VPD_CAP_NEXT_PTR_MASK: c_uint = 0xFF;
pub const VPD_CAP_NEXT_PTR_SHIFT: c_int = 8;
pub const VPD_CAP_VPD_ADDR_MASK: c_uint = 0x7FFF;
pub const VPD_CAP_VPD_ADDR_SHIFT: c_int = 16;
pub const VPD_CAP_VPD_FLAG: c_uint = 0x80000000;
pub const REG_VPD_DATA: c_uint = 0x70;
pub const REG_SPI_FLASH_CTRL: c_uint = 0x200;
pub const SPI_FLASH_CTRL_STS_NON_RDY: c_uint = 0x1;
pub const SPI_FLASH_CTRL_STS_WEN: c_uint = 0x2;
pub const SPI_FLASH_CTRL_STS_WPEN: c_uint = 0x80;
pub const SPI_FLASH_CTRL_DEV_STS_MASK: c_uint = 0xFF;
pub const SPI_FLASH_CTRL_DEV_STS_SHIFT: c_int = 0;
pub const SPI_FLASH_CTRL_INS_MASK: c_uint = 0x7;
pub const SPI_FLASH_CTRL_INS_SHIFT: c_int = 8;
pub const SPI_FLASH_CTRL_START: c_uint = 0x800;
pub const SPI_FLASH_CTRL_EN_VPD: c_uint = 0x2000;
pub const SPI_FLASH_CTRL_LDSTART: c_uint = 0x8000;
pub const SPI_FLASH_CTRL_CS_HI_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_CS_HI_SHIFT: c_int = 16;
pub const SPI_FLASH_CTRL_CS_HOLD_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_CS_HOLD_SHIFT: c_int = 18;
pub const SPI_FLASH_CTRL_CLK_LO_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_CLK_LO_SHIFT: c_int = 20;
pub const SPI_FLASH_CTRL_CLK_HI_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_CLK_HI_SHIFT: c_int = 22;
pub const SPI_FLASH_CTRL_CS_SETUP_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_CS_SETUP_SHIFT: c_int = 24;
pub const SPI_FLASH_CTRL_EROM_PGSZ_MASK: c_uint = 0x3;
pub const SPI_FLASH_CTRL_EROM_PGSZ_SHIFT: c_int = 26;
pub const SPI_FLASH_CTRL_WAIT_READY: c_uint = 0x10000000;
pub const REG_SPI_ADDR: c_uint = 0x204;
pub const REG_SPI_DATA: c_uint = 0x208;
pub const REG_SPI_FLASH_CONFIG: c_uint = 0x20C;
pub const SPI_FLASH_CONFIG_LD_ADDR_MASK: c_uint = 0xFFFFFF;
pub const SPI_FLASH_CONFIG_LD_ADDR_SHIFT: c_int = 0;
pub const SPI_FLASH_CONFIG_VPD_ADDR_MASK: c_uint = 0x3;
pub const SPI_FLASH_CONFIG_VPD_ADDR_SHIFT: c_int = 24;
pub const SPI_FLASH_CONFIG_LD_EXIST: c_uint = 0x4000000;
pub const REG_SPI_FLASH_OP_PROGRAM: c_uint = 0x210;
pub const REG_SPI_FLASH_OP_SC_ERASE: c_uint = 0x211;
pub const REG_SPI_FLASH_OP_CHIP_ERASE: c_uint = 0x212;
pub const REG_SPI_FLASH_OP_RDID: c_uint = 0x213;
pub const REG_SPI_FLASH_OP_WREN: c_uint = 0x214;
pub const REG_SPI_FLASH_OP_RDSR: c_uint = 0x215;
pub const REG_SPI_FLASH_OP_WRSR: c_uint = 0x216;
pub const REG_SPI_FLASH_OP_READ: c_uint = 0x217;
pub const REG_TWSI_CTRL: c_uint = 0x218;
pub const TWSI_CTRL_LD_OFFSET_MASK: c_uint = 0xFF;
pub const TWSI_CTRL_LD_OFFSET_SHIFT: c_int = 0;
pub const TWSI_CTRL_LD_SLV_ADDR_MASK: c_uint = 0x7;
pub const TWSI_CTRL_LD_SLV_ADDR_SHIFT: c_int = 8;
pub const TWSI_CTRL_SW_LDSTART: c_uint = 0x800;
pub const TWSI_CTRL_HW_LDSTART: c_uint = 0x1000;
pub const TWSI_CTRL_SMB_SLV_ADDR_MASK: c_uint = 0x7F;
pub const TWSI_CTRL_SMB_SLV_ADDR_SHIFT: c_int = 15;
pub const TWSI_CTRL_LD_EXIST: c_uint = 0x400000;
pub const TWSI_CTRL_READ_FREQ_SEL_MASK: c_uint = 0x3;
pub const TWSI_CTRL_READ_FREQ_SEL_SHIFT: c_int = 23;
pub const TWSI_CTRL_FREQ_SEL_100K: c_int = 0;
pub const TWSI_CTRL_FREQ_SEL_200K: c_int = 1;
pub const TWSI_CTRL_FREQ_SEL_300K: c_int = 2;
pub const TWSI_CTRL_FREQ_SEL_400K: c_int = 3;

pub const TWSI_CTRL_WRITE_FREQ_SEL_MASK: c_uint = 0x3;
pub const TWSI_CTRL_WRITE_FREQ_SEL_SHIFT: c_int = 24;
pub const REG_PCIE_DEV_MISC_CTRL: c_uint = 0x21C;
pub const PCIE_DEV_MISC_CTRL_EXT_PIPE: c_uint = 0x2;
pub const PCIE_DEV_MISC_CTRL_RETRY_BUFDIS: c_uint = 0x1;
pub const PCIE_DEV_MISC_CTRL_SPIROM_EXIST: c_uint = 0x4;
pub const PCIE_DEV_MISC_CTRL_SERDES_ENDIAN: c_uint = 0x8;
pub const PCIE_DEV_MISC_CTRL_SERDES_SEL_DIN: c_uint = 0x10;
pub const REG_PCIE_PHYMISC: c_uint = 0x1000;
pub const PCIE_PHYMISC_FORCE_RCV_DET: c_uint = 0x4;
pub const REG_PCIE_DLL_TX_CTRL1: c_uint = 0x1104;
pub const PCIE_DLL_TX_CTRL1_SEL_NOR_CLK: c_uint = 0x400;
pub const PCIE_DLL_TX_CTRL1_DEF: c_uint = 0x568;
pub const REG_LTSSM_TEST_MODE: c_uint = 0x12FC;
pub const LTSSM_TEST_MODE_DEF: c_uint = 0x6500;
// Master Control Register
pub const REG_MASTER_CTRL: c_uint = 0x1400;
pub const MASTER_CTRL_SOFT_RST: c_uint = 0x1;
pub const MASTER_CTRL_MTIMER_EN: c_uint = 0x2;
pub const MASTER_CTRL_ITIMER_EN: c_uint = 0x4;
pub const MASTER_CTRL_MANUAL_INT: c_uint = 0x8;
pub const MASTER_CTRL_REV_NUM_SHIFT: c_int = 16;
pub const MASTER_CTRL_REV_NUM_MASK: c_uint = 0xFF;
pub const MASTER_CTRL_DEV_ID_SHIFT: c_int = 24;
pub const MASTER_CTRL_DEV_ID_MASK: c_uint = 0xFF;
// Timer Initial Value Register
pub const REG_MANUAL_TIMER_INIT: c_uint = 0x1404;
// IRQ Moderator Timer Initial Value Register
pub const REG_IRQ_MODU_TIMER_INIT: c_uint = 0x1408;
pub const REG_PHY_ENABLE: c_uint = 0x140C;
// IRQ Anti-Lost Timer Initial Value Register
pub const REG_CMBDISDMA_TIMER: c_uint = 0x140E;
// Block IDLE Status Register
pub const REG_IDLE_STATUS: c_uint = 0x1410;
// MDIO Control Register
pub const REG_MDIO_CTRL: c_uint = 0x1414;
pub const MDIO_DATA_MASK: c_uint = 0xFFFF;
pub const MDIO_DATA_SHIFT: c_int = 0;
pub const MDIO_REG_ADDR_MASK: c_uint = 0x1F;
pub const MDIO_REG_ADDR_SHIFT: c_int = 16;
pub const MDIO_RW: c_uint = 0x200000;
pub const MDIO_SUP_PREAMBLE: c_uint = 0x400000;
pub const MDIO_START: c_uint = 0x800000;
pub const MDIO_CLK_SEL_SHIFT: c_int = 24;
pub const MDIO_CLK_25_4: c_int = 0;
pub const MDIO_CLK_25_6: c_int = 2;
pub const MDIO_CLK_25_8: c_int = 3;
pub const MDIO_CLK_25_10: c_int = 4;
pub const MDIO_CLK_25_14: c_int = 5;
pub const MDIO_CLK_25_20: c_int = 6;
pub const MDIO_CLK_25_28: c_int = 7;
pub const MDIO_BUSY: c_uint = 0x8000000;
// MII PHY Status Register
pub const REG_PHY_STATUS: c_uint = 0x1418;
// BIST Control and Status Register0 (for the Packet Memory)
pub const REG_BIST0_CTRL: c_uint = 0x141C;
pub const BIST0_NOW: c_uint = 0x1;
pub const BIST0_SRAM_FAIL: c_uint = 0x2;
pub const BIST0_FUSE_FLAG: c_uint = 0x4;
pub const REG_BIST1_CTRL: c_uint = 0x1420;
pub const BIST1_NOW: c_uint = 0x1;
pub const BIST1_SRAM_FAIL: c_uint = 0x2;
pub const BIST1_FUSE_FLAG: c_uint = 0x4;
// SerDes Lock Detect Control and Status Register
pub const REG_SERDES_LOCK: c_uint = 0x1424;
pub const SERDES_LOCK_DETECT: c_int = 1;
pub const SERDES_LOCK_DETECT_EN: c_int = 2;
// MAC Control Register
pub const REG_MAC_CTRL: c_uint = 0x1480;
pub const MAC_CTRL_TX_EN: c_int = 1;
pub const MAC_CTRL_RX_EN: c_int = 2;
pub const MAC_CTRL_TX_FLOW: c_int = 4;
pub const MAC_CTRL_RX_FLOW: c_int = 8;
pub const MAC_CTRL_LOOPBACK: c_uint = 0x10;
pub const MAC_CTRL_DUPLX: c_uint = 0x20;
pub const MAC_CTRL_ADD_CRC: c_uint = 0x40;
pub const MAC_CTRL_PAD: c_uint = 0x80;
pub const MAC_CTRL_LENCHK: c_uint = 0x100;
pub const MAC_CTRL_HUGE_EN: c_uint = 0x200;
pub const MAC_CTRL_PRMLEN_SHIFT: c_int = 10;
pub const MAC_CTRL_PRMLEN_MASK: c_uint = 0xF;
pub const MAC_CTRL_RMV_VLAN: c_uint = 0x4000;
pub const MAC_CTRL_PROMIS_EN: c_uint = 0x8000;
pub const MAC_CTRL_MC_ALL_EN: c_uint = 0x2000000;
pub const MAC_CTRL_BC_EN: c_uint = 0x4000000;
// MAC IPG/IFG Control Register
pub const REG_MAC_IPG_IFG: c_uint = 0x1484;
pub const MAC_IPG_IFG_IPGT_SHIFT: c_int = 0;
pub const MAC_IPG_IFG_IPGT_MASK: c_uint = 0x7F;
pub const MAC_IPG_IFG_MIFG_SHIFT: c_int = 8;
pub const MAC_IPG_IFG_MIFG_MASK: c_uint = 0xFF;
pub const MAC_IPG_IFG_IPGR1_SHIFT: c_int = 16;
pub const MAC_IPG_IFG_IPGR1_MASK: c_uint = 0x7F;
pub const MAC_IPG_IFG_IPGR2_SHIFT: c_int = 24;
pub const MAC_IPG_IFG_IPGR2_MASK: c_uint = 0x7F;
// MAC STATION ADDRESS
pub const REG_MAC_STA_ADDR: c_uint = 0x1488;
// Hash table for multicast address
pub const REG_RX_HASH_TABLE: c_uint = 0x1490;
// MAC Half-Duplex Control Register
pub const REG_MAC_HALF_DUPLX_CTRL: c_uint = 0x1498;
pub const MAC_HALF_DUPLX_CTRL_LCOL_SHIFT: c_int = 0;
pub const MAC_HALF_DUPLX_CTRL_LCOL_MASK: c_uint = 0x3FF;
pub const MAC_HALF_DUPLX_CTRL_RETRY_SHIFT: c_int = 12;
pub const MAC_HALF_DUPLX_CTRL_RETRY_MASK: c_uint = 0xF;
pub const MAC_HALF_DUPLX_CTRL_EXC_DEF_EN: c_uint = 0x10000;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_C: c_uint = 0x20000;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_P: c_uint = 0x40000;
pub const MAC_HALF_DUPLX_CTRL_ABEBE: c_uint = 0x80000;
pub const MAC_HALF_DUPLX_CTRL_ABEBT_SHIFT: c_int = 20;
pub const MAC_HALF_DUPLX_CTRL_ABEBT_MASK: c_uint = 0xF;
pub const MAC_HALF_DUPLX_CTRL_JAMIPG_SHIFT: c_int = 24;
pub const MAC_HALF_DUPLX_CTRL_JAMIPG_MASK: c_uint = 0xF;
// Maximum Frame Length Control Register
pub const REG_MTU: c_uint = 0x149C;
// Wake-On-Lan control register
pub const REG_WOL_CTRL: c_uint = 0x14A0;
pub const WOL_PATTERN_EN: c_uint = 0x1;
pub const WOL_PATTERN_PME_EN: c_uint = 0x2;
pub const WOL_MAGIC_EN: c_uint = 0x4;
pub const WOL_MAGIC_PME_EN: c_uint = 0x8;
pub const WOL_LINK_CHG_EN: c_uint = 0x10;
pub const WOL_LINK_CHG_PME_EN: c_uint = 0x20;
pub const WOL_PATTERN_ST: c_uint = 0x100;
pub const WOL_MAGIC_ST: c_uint = 0x200;
pub const WOL_LINKCHG_ST: c_uint = 0x400;
pub const WOL_PT0_EN: c_uint = 0x10000;
pub const WOL_PT1_EN: c_uint = 0x20000;
pub const WOL_PT2_EN: c_uint = 0x40000;
pub const WOL_PT3_EN: c_uint = 0x80000;
pub const WOL_PT4_EN: c_uint = 0x100000;
pub const WOL_PT0_MATCH: c_uint = 0x1000000;
pub const WOL_PT1_MATCH: c_uint = 0x2000000;
pub const WOL_PT2_MATCH: c_uint = 0x4000000;
pub const WOL_PT3_MATCH: c_uint = 0x8000000;
pub const WOL_PT4_MATCH: c_uint = 0x10000000;
// Internal SRAM Partition Register, high 32 bits
pub const REG_SRAM_RFD_ADDR: c_uint = 0x1500;
// Descriptor Control register, high 32 bits
pub const REG_DESC_BASE_ADDR_HI: c_uint = 0x1540;
// Interrupt Status Register
pub const REG_ISR: c_uint = 0x1600;
pub const ISR_UR_DETECTED: c_uint = 0x1000000;
pub const ISR_FERR_DETECTED: c_uint = 0x2000000;
pub const ISR_NFERR_DETECTED: c_uint = 0x4000000;
pub const ISR_CERR_DETECTED: c_uint = 0x8000000;
pub const ISR_PHY_LINKDOWN: c_uint = 0x10000000;
pub const ISR_DIS_INT: c_uint = 0x80000000;
// Interrupt Mask Register
pub const REG_IMR: c_uint = 0x1604;
pub const REG_RFD_RRD_IDX: c_uint = 0x1800;
pub const REG_TPD_IDX: c_uint = 0x1804;
// MII definitions
// PHY Common Register
pub const MII_ATLX_CR: c_uint = 0x09;
pub const MII_ATLX_SR: c_uint = 0x0A;
pub const MII_ATLX_ESR: c_uint = 0x0F;
pub const MII_ATLX_PSCR: c_uint = 0x10;
pub const MII_ATLX_PSSR: c_uint = 0x11;
// PHY Control Register
pub const MII_CR_SPEED_SELECT_MSB: c_uint = 0x0040	/* bits 6,13: 10=1000, 01=100,;
// 00=10
//
pub const MII_CR_COLL_TEST_ENABLE: c_uint = 0x0080	/* Collision test enable */;
pub const MII_CR_FULL_DUPLEX: c_uint = 0x0100	/* FDX =1, half duplex =0 */;
pub const MII_CR_RESTART_AUTO_NEG: c_uint = 0x0200	/* Restart auto negotiation */;
pub const MII_CR_ISOLATE: c_uint = 0x0400	/* Isolate PHY from MII */;
pub const MII_CR_POWER_DOWN: c_uint = 0x0800	/* Power down */;
pub const MII_CR_AUTO_NEG_EN: c_uint = 0x1000	/* Auto Neg Enable */;
pub const MII_CR_SPEED_SELECT_LSB: c_uint = 0x2000	/* bits 6,13: 10=1000, 01=100,;
// 00=10
//
pub const MII_CR_LOOPBACK: c_uint = 0x4000	/* 0 = normal, 1 = loopback */;
pub const MII_CR_RESET: c_uint = 0x8000	/* 0 = normal, 1 = PHY reset */;
pub const MII_CR_SPEED_MASK: c_uint = 0x2040;
pub const MII_CR_SPEED_1000: c_uint = 0x0040;
pub const MII_CR_SPEED_100: c_uint = 0x2000;
pub const MII_CR_SPEED_10: c_uint = 0x0000;
// PHY Status Register
pub const MII_SR_EXTENDED_CAPS: c_uint = 0x0001	/* Ext register capabilities */;
pub const MII_SR_JABBER_DETECT: c_uint = 0x0002	/* Jabber Detected */;
pub const MII_SR_LINK_STATUS: c_uint = 0x0004	/* Link Status 1 = link */;
pub const MII_SR_AUTONEG_CAPS: c_uint = 0x0008	/* Auto Neg Capable */;
pub const MII_SR_REMOTE_FAULT: c_uint = 0x0010	/* Remote Fault Detect */;
pub const MII_SR_AUTONEG_COMPLETE: c_uint = 0x0020	/* Auto Neg Complete */;
pub const MII_SR_PREAMBLE_SUPPRESS: c_uint = 0x0040	/* Preamble may be suppressed */;
pub const MII_SR_EXTENDED_STATUS: c_uint = 0x0100	/* Ext stat info in Reg 0x0F */;
pub const MII_SR_100T2_HD_CAPS: c_uint = 0x0200	/* 100T2 Half Duplex Capable */;
pub const MII_SR_100T2_FD_CAPS: c_uint = 0x0400	/* 100T2 Full Duplex Capable */;
pub const MII_SR_10T_HD_CAPS: c_uint = 0x0800	/* 10T   Half Duplex Capable */;
pub const MII_SR_10T_FD_CAPS: c_uint = 0x1000	/* 10T   Full Duplex Capable */;
pub const MII_SR_100X_HD_CAPS: c_uint = 0x2000	/* 100X  Half Duplex Capable */;
pub const MII_SR_100X_FD_CAPS: c_uint = 0x4000	/* 100X  Full Duplex Capable */;
pub const MII_SR_100T4_CAPS: c_uint = 0x8000	/* 100T4 Capable */;
// Link partner ability register
pub const MII_LPA_SLCT: c_uint = 0x001f	/* Same as advertise selector */;
pub const MII_LPA_10HALF: c_uint = 0x0020	/* Can do 10mbps half-duplex */;
pub const MII_LPA_10FULL: c_uint = 0x0040	/* Can do 10mbps full-duplex */;
pub const MII_LPA_100HALF: c_uint = 0x0080	/* Can do 100mbps half-duplex */;
pub const MII_LPA_100FULL: c_uint = 0x0100	/* Can do 100mbps full-duplex */;
pub const MII_LPA_100BASE4: c_uint = 0x0200	/* 100BASE-T4 */;
pub const MII_LPA_PAUSE: c_uint = 0x0400	/* PAUSE */;
pub const MII_LPA_ASYPAUSE: c_uint = 0x0800	/* Asymmetrical PAUSE */;
pub const MII_LPA_RFAULT: c_uint = 0x2000	/* Link partner faulted */;
pub const MII_LPA_LPACK: c_uint = 0x4000	/* Link partner acked us */;
pub const MII_LPA_NPAGE: c_uint = 0x8000	/* Next page bit */;
// Autoneg Advertisement Register
pub const MII_AR_SELECTOR_FIELD: c_uint = 0x0001	/* IEEE 802.3 CSMA/CD */;
pub const MII_AR_10T_HD_CAPS: c_uint = 0x0020	/* 10T   Half Duplex Capable */;
pub const MII_AR_10T_FD_CAPS: c_uint = 0x0040	/* 10T   Full Duplex Capable */;
pub const MII_AR_100TX_HD_CAPS: c_uint = 0x0080	/* 100TX Half Duplex Capable */;
pub const MII_AR_100TX_FD_CAPS: c_uint = 0x0100	/* 100TX Full Duplex Capable */;
pub const MII_AR_100T4_CAPS: c_uint = 0x0200	/* 100T4 Capable */;
pub const MII_AR_PAUSE: c_uint = 0x0400	/* Pause operation desired */;
pub const MII_AR_ASM_DIR: c_uint = 0x0800	/* Asymmetric Pause Dir bit */;
pub const MII_AR_REMOTE_FAULT: c_uint = 0x2000	/* Remote Fault detected */;
pub const MII_AR_NEXT_PAGE: c_uint = 0x8000	/* Next Page ability support */;
pub const MII_AR_SPEED_MASK: c_uint = 0x01E0;
pub const MII_AR_DEFAULT_CAP_MASK: c_uint = 0x0DE0;
// 1000BASE-T Control Register
pub const MII_ATLX_CR_1000T_HD_CAPS: c_uint = 0x0100	/* Adv 1000T HD cap */;
pub const MII_ATLX_CR_1000T_FD_CAPS: c_uint = 0x0200	/* Adv 1000T FD cap */;
pub const MII_ATLX_CR_1000T_REPEATER_DTE: c_uint = 0x0400	/* 1=Repeater/switch device,;
// 0=DTE device
pub const MII_ATLX_CR_1000T_MS_VALUE: c_uint = 0x0800	/* 1=Config PHY as Master,;
// 0=Configure PHY as Slave
pub const MII_ATLX_CR_1000T_MS_ENABLE: c_uint = 0x1000	/* 1=Man Master/Slave config,;
// 0=Auto Master/Slave config
//
pub const MII_ATLX_CR_1000T_TEST_MODE_NORMAL: c_uint = 0x0000	/* Normal Operation */;
pub const MII_ATLX_CR_1000T_TEST_MODE_1: c_uint = 0x2000	/* Transmit Waveform test */;
pub const MII_ATLX_CR_1000T_TEST_MODE_2: c_uint = 0x4000	/* Master Xmit Jitter test */;
pub const MII_ATLX_CR_1000T_TEST_MODE_3: c_uint = 0x6000	/* Slave Xmit Jitter test */;
pub const MII_ATLX_CR_1000T_TEST_MODE_4: c_uint = 0x8000	/* Xmitter Distortion test */;
pub const MII_ATLX_CR_1000T_SPEED_MASK: c_uint = 0x0300;
pub const MII_ATLX_CR_1000T_DEFAULT_CAP_MASK: c_uint = 0x0300;
// 1000BASE-T Status Register
pub const MII_ATLX_SR_1000T_LP_HD_CAPS: c_uint = 0x0400	/* LP is 1000T HD capable */;
pub const MII_ATLX_SR_1000T_LP_FD_CAPS: c_uint = 0x0800	/* LP is 1000T FD capable */;
pub const MII_ATLX_SR_1000T_REMOTE_RX_STATUS: c_uint = 0x1000	/* Remote receiver OK */;
pub const MII_ATLX_SR_1000T_LOCAL_RX_STATUS: c_uint = 0x2000	/* Local receiver OK */;
pub const MII_ATLX_SR_1000T_MS_CONFIG_RES: c_uint = 0x4000	/* 1=Local TX is Master;
// 0=Slave
//
pub const MII_ATLX_SR_1000T_MS_CONFIG_FAULT: c_uint = 0x8000	/* Master/Slave config;
// fault
pub const MII_ATLX_SR_1000T_REMOTE_RX_STATUS_SHIFT: c_int = 12;
pub const MII_ATLX_SR_1000T_LOCAL_RX_STATUS_SHIFT: c_int = 13;
// Extended Status Register
pub const MII_ATLX_ESR_1000T_HD_CAPS: c_uint = 0x1000	/* 1000T HD capable */;
pub const MII_ATLX_ESR_1000T_FD_CAPS: c_uint = 0x2000	/* 1000T FD capable */;
pub const MII_ATLX_ESR_1000X_HD_CAPS: c_uint = 0x4000	/* 1000X HD capable */;
pub const MII_ATLX_ESR_1000X_FD_CAPS: c_uint = 0x8000	/* 1000X FD capable */;
// ATLX PHY Specific Control Register
pub const MII_ATLX_PSCR_JABBER_DISABLE: c_uint = 0x0001	/* 1=Jabber Func disabled */;
pub const MII_ATLX_PSCR_POLARITY_REVERSAL: c_uint = 0x0002	/* 1=Polarity Reversal enbld */;
pub const MII_ATLX_PSCR_SQE_TEST: c_uint = 0x0004	/* 1=SQE Test enabled */;
pub const MII_ATLX_PSCR_MAC_POWERDOWN: c_uint = 0x0008;
pub const MII_ATLX_PSCR_CLK125_DISABLE: c_uint = 0x0010	/* 1=CLK125 low;
// 0=CLK125 toggling
//
pub const MII_ATLX_PSCR_MDI_MANUAL_MODE: c_uint = 0x0000	/* MDI Crossover Mode bits 6:5,;
// Manual MDI configuration
//
pub const MII_ATLX_PSCR_MDIX_MANUAL_MODE: c_uint = 0x0020	/* Manual MDIX configuration */;
pub const MII_ATLX_PSCR_AUTO_X_1000T: c_uint = 0x0040	/* 1000BASE-T: Auto crossover;
// 100BASE-TX/10BASE-T: MDI
// Mode
pub const MII_ATLX_PSCR_AUTO_X_MODE: c_uint = 0x0060	/* Auto crossover enabled;
// all speeds.
//
pub const MII_ATLX_PSCR_10BT_EXT_DIST_ENABLE: c_uint = 0x0080	/* 1=Enable Extended;
// 10BASE-T distance
// (Lower 10BASE-T RX
// Threshold)
// 0=Normal 10BASE-T RX
// Threshold
//
pub const MII_ATLX_PSCR_MII_5BIT_ENABLE: c_uint = 0x0100	/* 1=5-Bit interface in;
// 100BASE-TX
// 0=MII interface in
// 100BASE-TX
//
pub const MII_ATLX_PSCR_SCRAMBLER_DISABLE: c_uint = 0x0200	/* 1=Scrambler dsbl */;
pub const MII_ATLX_PSCR_FORCE_LINK_GOOD: c_uint = 0x0400	/* 1=Force link good */;
pub const MII_ATLX_PSCR_ASSERT_CRS_ON_TX: c_uint = 0x0800	/* 1=Assert CRS on Transmit */;
pub const MII_ATLX_PSCR_POLARITY_REVERSAL_SHIFT: c_int = 1;
pub const MII_ATLX_PSCR_AUTO_X_MODE_SHIFT: c_int = 5;
pub const MII_ATLX_PSCR_10BT_EXT_DIST_ENABLE_SHIFT: c_int = 7;
// ATLX PHY Specific Status Register
pub const MII_ATLX_PSSR_SPD_DPLX_RESOLVED: c_uint = 0x0800	/* 1=Speed & Duplex resolved */;
pub const MII_ATLX_PSSR_DPLX: c_uint = 0x2000	/* 1=Duplex 0=Half Duplex */;
pub const MII_ATLX_PSSR_SPEED: c_uint = 0xC000	/* Speed, bits 14:15 */;
pub const MII_ATLX_PSSR_10MBS: c_uint = 0x0000	/* 00=10Mbs */;
pub const MII_ATLX_PSSR_100MBS: c_uint = 0x4000	/* 01=100Mbs */;
pub const MII_ATLX_PSSR_1000MBS: c_uint = 0x8000	/* 10=1000Mbs */;
pub const MII_DBG_ADDR: c_uint = 0x1D;
pub const MII_DBG_DATA: c_uint = 0x1E;
// PCI Command Register Bit Definitions
pub const PCI_REG_COMMAND: c_uint = 0x04	/* PCI Command Register */;
pub const CMD_IO_SPACE: c_uint = 0x0001;
pub const CMD_MEMORY_SPACE: c_uint = 0x0002;
pub const CMD_BUS_MASTER: c_uint = 0x0004;
// Wake Up Filter Control
pub const ATLX_WUFC_LNKC: c_uint = 0x00000001	/* Link Status Change Wakeup Enable */;
pub const ATLX_WUFC_MAG: c_uint = 0x00000002	/* Magic Packet Wakeup Enable */;
pub const ATLX_WUFC_EX: c_uint = 0x00000004	/* Directed Exact Wakeup Enable */;
pub const ATLX_WUFC_MC: c_uint = 0x00000008	/* Multicast Wakeup Enable */;
pub const ATLX_WUFC_BC: c_uint = 0x00000010	/* Broadcast Wakeup Enable */;
pub const ADVERTISE_10_HALF: c_uint = 0x0001;
pub const ADVERTISE_10_FULL: c_uint = 0x0002;
pub const ADVERTISE_100_HALF: c_uint = 0x0004;
pub const ADVERTISE_100_FULL: c_uint = 0x0008;
pub const ADVERTISE_1000_HALF: c_uint = 0x0010;
pub const ADVERTISE_1000_FULL: c_uint = 0x0020;
pub const AUTONEG_ADVERTISE_10_100_ALL: c_uint = 0x000F	/* All 10/100 speeds */;
pub const AUTONEG_ADVERTISE_10_ALL: c_uint = 0x0003	/* 10Mbps Full & Half speeds */;

// For checksumming, the sum of all words in the EEPROM should equal 0xBABA
pub const EEPROM_SUM: c_uint = 0xBABA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlx_spi_flash_dev {
    pub /: *const *const *const char manu_name; / manufacturer id,
// op-code
    pub cmd_wrsr: u8,
    pub cmd_read: u8,
    pub cmd_program: u8,
    pub cmd_wren: u8,
    pub cmd_wrdi: u8,
    pub cmd_rdsr: u8,
    pub cmd_rdid: u8,
    pub cmd_sector_erase: u8,
    pub cmd_chip_erase: u8,
}
