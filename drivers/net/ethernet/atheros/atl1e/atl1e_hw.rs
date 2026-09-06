//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atl1e/atl1e_hw.h
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
// Copyright(c) 2007 Atheros Corporation. All rights reserved.
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

// function prototype
extern "C" {
    pub fn atl1e_reset_hw(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_read_mac_addr(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_init_hw(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_phy_commit(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_get_speed_and_duplex(hw: *mut atl1e_hw, speed: *mut u16, duplex: *mut u16) -> i32;
}
extern "C" {
    pub fn atl1e_auto_get_fc(adapter: *mut atl1e_adapter, duplex: u16) -> u32;
}
extern "C" {
    pub fn atl1e_hash_mc_addr(hw: *mut atl1e_hw, mc_addr: *mut u8) -> u32;
}
extern "C" {
    pub fn atl1e_hash_set(hw: *mut atl1e_hw, hash_value: u32);
}
extern "C" {
    pub fn atl1e_read_phy_reg(hw: *mut atl1e_hw, reg_addr: u16, phy_data: *mut u16) -> i32;
}
extern "C" {
    pub fn atl1e_write_phy_reg(hw: *mut atl1e_hw, reg_addr: u32, phy_data: u16) -> i32;
}
extern "C" {
    pub fn atl1e_validate_mdi_setting(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_hw_set_mac_addr(hw: *mut atl1e_hw);
}
extern "C" {
    pub fn atl1e_read_eeprom(hw: *mut atl1e_hw, offset: u32, p_value: *mut u32) -> bool;
}
extern "C" {
    pub fn atl1e_write_eeprom(hw: *mut atl1e_hw, offset: u32, value: u32) -> bool;
}
extern "C" {
    pub fn atl1e_phy_enter_power_saving(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_phy_leave_power_saving(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_phy_init(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_check_eeprom_exist(hw: *mut atl1e_hw) -> c_int;
}
extern "C" {
    pub fn atl1e_force_ps(hw: *mut atl1e_hw);
}
extern "C" {
    pub fn atl1e_restart_autoneg(hw: *mut atl1e_hw) -> i32;
}
// register definition
pub const REG_PM_CTRLSTAT: c_uint = 0x44;
pub const REG_PCIE_CAP_LIST: c_uint = 0x58;
pub const REG_DEVICE_CAP: c_uint = 0x5C;
pub const DEVICE_CAP_MAX_PAYLOAD_MASK: c_uint = 0x7;
pub const DEVICE_CAP_MAX_PAYLOAD_SHIFT: c_int = 0;
pub const REG_DEVICE_CTRL: c_uint = 0x60;
pub const DEVICE_CTRL_MAX_PAYLOAD_MASK: c_uint = 0x7;
pub const DEVICE_CTRL_MAX_PAYLOAD_SHIFT: c_int = 5;
pub const DEVICE_CTRL_MAX_RREQ_SZ_MASK: c_uint = 0x7;
pub const DEVICE_CTRL_MAX_RREQ_SZ_SHIFT: c_int = 12;
pub const REG_VPD_CAP: c_uint = 0x6C;
pub const VPD_CAP_ID_MASK: c_uint = 0xff;
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
// Macro flag: #define     TWSI_CTRL_SMB_SLV_ADDR
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
pub const REG_LTSSM_TEST_MODE: c_uint = 0x12FC;
pub const LTSSM_TEST_MODE_DEF: c_uint = 0xE000;
// Selene Master Control Register
pub const REG_MASTER_CTRL: c_uint = 0x1400;
pub const MASTER_CTRL_SOFT_RST: c_uint = 0x1;
pub const MASTER_CTRL_MTIMER_EN: c_uint = 0x2;
pub const MASTER_CTRL_ITIMER_EN: c_uint = 0x4;
pub const MASTER_CTRL_MANUAL_INT: c_uint = 0x8;
pub const MASTER_CTRL_ITIMER2_EN: c_uint = 0x20;
pub const MASTER_CTRL_INT_RDCLR: c_uint = 0x40;
pub const MASTER_CTRL_LED_MODE: c_uint = 0x200;
pub const MASTER_CTRL_REV_NUM_SHIFT: c_int = 16;
pub const MASTER_CTRL_REV_NUM_MASK: c_uint = 0xff;
pub const MASTER_CTRL_DEV_ID_SHIFT: c_int = 24;
pub const MASTER_CTRL_DEV_ID_MASK: c_uint = 0xff;
// Timer Initial Value Register
pub const REG_MANUAL_TIMER_INIT: c_uint = 0x1404;
// IRQ ModeratorTimer Initial Value Register
pub const REG_IRQ_MODU_TIMER_INIT: c_uint = 0x1408   /* w */;
pub const REG_IRQ_MODU_TIMER2_INIT: c_uint = 0x140A   /* w */;
pub const REG_GPHY_CTRL: c_uint = 0x140C;
pub const GPHY_CTRL_EXT_RESET: c_int = 1;
pub const GPHY_CTRL_PIPE_MOD: c_int = 2;
pub const GPHY_CTRL_TEST_MODE_MASK: c_int = 3;
pub const GPHY_CTRL_TEST_MODE_SHIFT: c_int = 2;
pub const GPHY_CTRL_BERT_START: c_uint = 0x10;
pub const GPHY_CTRL_GATE_25M_EN: c_uint = 0x20;
pub const GPHY_CTRL_LPW_EXIT: c_uint = 0x40;
pub const GPHY_CTRL_PHY_IDDQ: c_uint = 0x80;
pub const GPHY_CTRL_PHY_IDDQ_DIS: c_uint = 0x100;
pub const GPHY_CTRL_PCLK_SEL_DIS: c_uint = 0x200;
pub const GPHY_CTRL_HIB_EN: c_uint = 0x400;
pub const GPHY_CTRL_HIB_PULSE: c_uint = 0x800;
pub const GPHY_CTRL_SEL_ANA_RST: c_uint = 0x1000;
pub const GPHY_CTRL_PHY_PLL_ON: c_uint = 0x2000;
pub const GPHY_CTRL_PWDOWN_HW: c_uint = 0x4000;

// IRQ Anti-Lost Timer Initial Value Register
pub const REG_CMBDISDMA_TIMER: c_uint = 0x140E;
// Block IDLE Status Register
pub const REG_IDLE_STATUS: c_uint = 0x1410;

pub const IDLE_STATUS_DMAR: c_uint = 0x10 /* 1: DMAR state machine is in non-IDLE state.  0: DMAR is idling  */;
pub const IDLE_STATUS_DMAW: c_uint = 0x20 /* 1: DMAW state machine is in non-IDLE state.  0: DMAW is idling  */;
pub const IDLE_STATUS_SMB: c_uint = 0x40 /* 1: SMB state machine is in non-IDLE state.   0: SMB is idling   */;
pub const IDLE_STATUS_CMB: c_uint = 0x80 /* 1: CMB state machine is in non-IDLE state.   0: CMB is idling   */;
// MDIO Control Register
pub const REG_MDIO_CTRL: c_uint = 0x1414;
pub const MDIO_DATA_MASK: c_uint = 0xffff  /* On MDIO write, the 16-bit control data to write to PHY MII management register */;

pub const MDIO_REG_ADDR_MASK: c_uint = 0x1f    /* MDIO register address */;
pub const MDIO_REG_ADDR_SHIFT: c_int = 16;
pub const MDIO_RW: c_uint = 0x200000      /* 1: read, 0: write */;
pub const MDIO_SUP_PREAMBLE: c_uint = 0x400000      /* Suppress preamble */;
pub const MDIO_START: c_uint = 0x800000      /* Write 1 to initiate the MDIO master. And this bit is self cleared after one cycle*/;
pub const MDIO_CLK_SEL_SHIFT: c_int = 24;
pub const MDIO_CLK_25_4: c_int = 0;
pub const MDIO_CLK_25_6: c_int = 2;
pub const MDIO_CLK_25_8: c_int = 3;
pub const MDIO_CLK_25_10: c_int = 4;
pub const MDIO_CLK_25_14: c_int = 5;
pub const MDIO_CLK_25_20: c_int = 6;
pub const MDIO_CLK_25_28: c_int = 7;
pub const MDIO_BUSY: c_uint = 0x8000000;
pub const MDIO_AP_EN: c_uint = 0x10000000;
pub const MDIO_WAIT_TIMES: c_int = 10;
// MII PHY Status Register
pub const REG_PHY_STATUS: c_uint = 0x1418;
pub const PHY_STATUS_100M: c_uint = 0x20000;
pub const PHY_STATUS_EMI_CA: c_uint = 0x40000;
// BIST Control and Status Register0 (for the Packet Memory)
pub const REG_BIST0_CTRL: c_uint = 0x141c;
pub const BIST0_NOW: c_uint = 0x1 /* 1: To trigger BIST0 logic. This bit stays high during the */;
// BIST process and reset to zero when BIST is done
pub const BIST0_SRAM_FAIL: c_uint = 0x2 /* 1: The SRAM failure is un-repairable because it has address */;
// decoder failure or more than 1 cell stuck-to-x failure
pub const BIST0_FUSE_FLAG: c_uint = 0x4 /* 1: Indicating one cell has been fixed */;
// BIST Control and Status Register1(for the retry buffer of PCI Express)
pub const REG_BIST1_CTRL: c_uint = 0x1420;
pub const BIST1_NOW: c_uint = 0x1 /* 1: To trigger BIST0 logic. This bit stays high during the */;
// BIST process and reset to zero when BIST is done
pub const BIST1_SRAM_FAIL: c_uint = 0x2 /* 1: The SRAM failure is un-repairable because it has address */;
// decoder failure or more than 1 cell stuck-to-x failure.
pub const BIST1_FUSE_FLAG: c_uint = 0x4;
// SerDes Lock Detect Control and Status Register
pub const REG_SERDES_LOCK: c_uint = 0x1424;

// MAC Control Register
pub const REG_MAC_CTRL: c_uint = 0x1480;

pub const MAC_CTRL_LOOPBACK: c_uint = 0x10      /* 1: Loop back at G/MII Interface */;
pub const MAC_CTRL_DUPLX: c_uint = 0x20      /* 1: Full-duplex mode  0: Half-duplex mode */;
pub const MAC_CTRL_ADD_CRC: c_uint = 0x40      /* 1: Instruct MAC to attach CRC on all egress Ethernet frames */;
pub const MAC_CTRL_PAD: c_uint = 0x80      /* 1: Instruct MAC to pad short frames to 60-bytes, and then attach CRC. This bit has higher priority over CRC_EN */;
pub const MAC_CTRL_LENCHK: c_uint = 0x100     /* 1: Instruct MAC to check if length field matches the real packet length */;
pub const MAC_CTRL_HUGE_EN: c_uint = 0x200     /* 1: receive Jumbo frame enable */;

pub const MAC_CTRL_PRMLEN_MASK: c_uint = 0xf;
pub const MAC_CTRL_RMV_VLAN: c_uint = 0x4000    /* 1: to remove VLAN Tag automatically from all receive packets */;
pub const MAC_CTRL_PROMIS_EN: c_uint = 0x8000    /* 1: Promiscuous Mode Enable */;
pub const MAC_CTRL_TX_PAUSE: c_uint = 0x10000   /* 1: transmit test pause */;
pub const MAC_CTRL_SCNT: c_uint = 0x20000   /* 1: shortcut slot time counter */;
pub const MAC_CTRL_SRST_TX: c_uint = 0x40000   /* 1: synchronized reset Transmit MAC module */;
pub const MAC_CTRL_TX_SIMURST: c_uint = 0x80000   /* 1: transmit simulation reset */;

pub const MAC_CTRL_SPEED_MASK: c_uint = 0x300000;
pub const MAC_CTRL_SPEED_1000: c_int = 2;
pub const MAC_CTRL_SPEED_10_100: c_int = 1;
pub const MAC_CTRL_DBG_TX_BKPRESURE: c_uint = 0x400000  /* 1: transmit maximum backoff (half-duplex test bit) */;
pub const MAC_CTRL_TX_HUGE: c_uint = 0x800000  /* 1: transmit huge enable */;
pub const MAC_CTRL_RX_CHKSUM_EN: c_uint = 0x1000000 /* 1: RX checksum enable */;
pub const MAC_CTRL_MC_ALL_EN: c_uint = 0x2000000 /* 1: upload all multicast frame without error to system */;
pub const MAC_CTRL_BC_EN: c_uint = 0x4000000 /* 1: upload all broadcast frame without error to system */;
pub const MAC_CTRL_DBG: c_uint = 0x8000000 /* 1: upload all received frame to system (Debug Mode) */;
// MAC IPG/IFG Control Register
pub const REG_MAC_IPG_IFG: c_uint = 0x1484;

pub const MAC_IPG_IFG_IPGT_MASK: c_uint = 0x7f;

pub const MAC_IPG_IFG_MIFG_MASK: c_uint = 0xff  /* Frame gap below such IFP is dropped */;

pub const MAC_IPG_IFG_IPGR1_MASK: c_uint = 0x7f;

pub const MAC_IPG_IFG_IPGR2_MASK: c_uint = 0x7f;
// MAC STATION ADDRESS
pub const REG_MAC_STA_ADDR: c_uint = 0x1488;
// Hash table for multicast address
pub const REG_RX_HASH_TABLE: c_uint = 0x1490;
// MAC Half-Duplex Control Register
pub const REG_MAC_HALF_DUPLX_CTRL: c_uint = 0x1498;

pub const MAC_HALF_DUPLX_CTRL_LCOL_MASK: c_uint = 0x3ff;

pub const MAC_HALF_DUPLX_CTRL_RETRY_MASK: c_uint = 0xf;
pub const MAC_HALF_DUPLX_CTRL_EXC_DEF_EN: c_uint = 0x10000 /* 1: Allow the transmission of a packet which has been excessively deferred */;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_C: c_uint = 0x20000 /* 1: No back-off on collision, immediately start the retransmission */;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_P: c_uint = 0x40000 /* 1: No back-off on backpressure, immediately start the transmission after back pressure */;
pub const MAC_HALF_DUPLX_CTRL_ABEBE: c_uint = 0x80000 /* 1: Alternative Binary Exponential Back-off Enabled */;

pub const MAC_HALF_DUPLX_CTRL_ABEBT_MASK: c_uint = 0xf;

pub const MAC_HALF_DUPLX_CTRL_JAMIPG_MASK: c_uint = 0xf     /* mode. In unit of 8-bit time */;
// Maximum Frame Length Control Register
pub const REG_MTU: c_uint = 0x149c;
// Wake-On-Lan control register
pub const REG_WOL_CTRL: c_uint = 0x14a0;
pub const WOL_PATTERN_EN: c_uint = 0x00000001;
pub const WOL_PATTERN_PME_EN: c_uint = 0x00000002;
pub const WOL_MAGIC_EN: c_uint = 0x00000004;
pub const WOL_MAGIC_PME_EN: c_uint = 0x00000008;
pub const WOL_LINK_CHG_EN: c_uint = 0x00000010;
pub const WOL_LINK_CHG_PME_EN: c_uint = 0x00000020;
pub const WOL_PATTERN_ST: c_uint = 0x00000100;
pub const WOL_MAGIC_ST: c_uint = 0x00000200;
pub const WOL_LINKCHG_ST: c_uint = 0x00000400;
pub const WOL_CLK_SWITCH_EN: c_uint = 0x00008000;
pub const WOL_PT0_EN: c_uint = 0x00010000;
pub const WOL_PT1_EN: c_uint = 0x00020000;
pub const WOL_PT2_EN: c_uint = 0x00040000;
pub const WOL_PT3_EN: c_uint = 0x00080000;
pub const WOL_PT4_EN: c_uint = 0x00100000;
pub const WOL_PT5_EN: c_uint = 0x00200000;
pub const WOL_PT6_EN: c_uint = 0x00400000;
// WOL Length ( 2 DWORD )
pub const REG_WOL_PATTERN_LEN: c_uint = 0x14a4;
pub const WOL_PT_LEN_MASK: c_uint = 0x7f;
pub const WOL_PT0_LEN_SHIFT: c_int = 0;
pub const WOL_PT1_LEN_SHIFT: c_int = 8;
pub const WOL_PT2_LEN_SHIFT: c_int = 16;
pub const WOL_PT3_LEN_SHIFT: c_int = 24;
pub const WOL_PT4_LEN_SHIFT: c_int = 0;
pub const WOL_PT5_LEN_SHIFT: c_int = 8;
pub const WOL_PT6_LEN_SHIFT: c_int = 16;
// Internal SRAM Partition Register
pub const REG_SRAM_TRD_ADDR: c_uint = 0x1518;
pub const REG_SRAM_TRD_LEN: c_uint = 0x151C;
pub const REG_SRAM_RXF_ADDR: c_uint = 0x1520;
pub const REG_SRAM_RXF_LEN: c_uint = 0x1524;
pub const REG_SRAM_TXF_ADDR: c_uint = 0x1528;
pub const REG_SRAM_TXF_LEN: c_uint = 0x152C;
pub const REG_SRAM_TCPH_ADDR: c_uint = 0x1530;
pub const REG_SRAM_PKTH_ADDR: c_uint = 0x1532;
// Load Ptr Register
pub const REG_LOAD_PTR: c_uint = 0x1534  /* Software sets this bit after the initialization of the head and tail */;
//
// addresses of all descriptors, as well as the following descriptor
// control register, which triggers each function block to load the head
// pointer to prepare for the operation. This bit is then self-cleared
// after one cycle.
//
// Descriptor Control register
pub const REG_RXF3_BASE_ADDR_HI: c_uint = 0x153C;
pub const REG_DESC_BASE_ADDR_HI: c_uint = 0x1540;
pub const REG_RXF0_BASE_ADDR_HI: c_uint = 0x1540 /* share with DESC BASE ADDR HI */;
pub const REG_HOST_RXF0_PAGE0_LO: c_uint = 0x1544;
pub const REG_HOST_RXF0_PAGE1_LO: c_uint = 0x1548;
pub const REG_TPD_BASE_ADDR_LO: c_uint = 0x154C;
pub const REG_RXF1_BASE_ADDR_HI: c_uint = 0x1550;
pub const REG_RXF2_BASE_ADDR_HI: c_uint = 0x1554;
pub const REG_HOST_RXFPAGE_SIZE: c_uint = 0x1558;
pub const REG_TPD_RING_SIZE: c_uint = 0x155C;
// RSS about
pub const REG_RSS_KEY0: c_uint = 0x14B0;
pub const REG_RSS_KEY1: c_uint = 0x14B4;
pub const REG_RSS_KEY2: c_uint = 0x14B8;
pub const REG_RSS_KEY3: c_uint = 0x14BC;
pub const REG_RSS_KEY4: c_uint = 0x14C0;
pub const REG_RSS_KEY5: c_uint = 0x14C4;
pub const REG_RSS_KEY6: c_uint = 0x14C8;
pub const REG_RSS_KEY7: c_uint = 0x14CC;
pub const REG_RSS_KEY8: c_uint = 0x14D0;
pub const REG_RSS_KEY9: c_uint = 0x14D4;
pub const REG_IDT_TABLE4: c_uint = 0x14E0;
pub const REG_IDT_TABLE5: c_uint = 0x14E4;
pub const REG_IDT_TABLE6: c_uint = 0x14E8;
pub const REG_IDT_TABLE7: c_uint = 0x14EC;
pub const REG_IDT_TABLE0: c_uint = 0x1560;
pub const REG_IDT_TABLE1: c_uint = 0x1564;
pub const REG_IDT_TABLE2: c_uint = 0x1568;
pub const REG_IDT_TABLE3: c_uint = 0x156C;

pub const REG_RSS_HASH_VALUE: c_uint = 0x1570;
pub const REG_RSS_HASH_FLAG: c_uint = 0x1574;
pub const REG_BASE_CPU_NUMBER: c_uint = 0x157C;
// TXQ Control Register
pub const REG_TXQ_CTRL: c_uint = 0x1580;
pub const TXQ_CTRL_NUM_TPD_BURST_MASK: c_uint = 0xF;
pub const TXQ_CTRL_NUM_TPD_BURST_SHIFT: c_int = 0;
pub const TXQ_CTRL_EN: c_uint = 0x20  /* 1: Enable TXQ */;
pub const TXQ_CTRL_ENH_MODE: c_uint = 0x40  /* Performance enhancement mode, in which up to two back-to-back DMA read commands might be dispatched. */;

pub const TXQ_CTRL_TXF_BURST_NUM_MASK: c_uint = 0xffff;
// Jumbo packet Threshold for task offload
pub const REG_TX_EARLY_TH: c_uint = 0x1584 /* Jumbo frame threshold in QWORD unit. Packet greater than */;
// JUMBO_TASK_OFFLOAD_THRESHOLD will not be task offloaded.
pub const TX_TX_EARLY_TH_MASK: c_uint = 0x7ff;
pub const TX_TX_EARLY_TH_SHIFT: c_int = 0;
// RXQ Control Register
pub const REG_RXQ_CTRL: c_uint = 0x15A0;

pub const RXQ_CTRL_PBA_ALIGN_64: c_int = 1;
pub const RXQ_CTRL_PBA_ALIGN_128: c_int = 2;
pub const RXQ_CTRL_PBA_ALIGN_256: c_int = 3;
pub const RXQ_CTRL_Q1_EN: c_uint = 0x10;
pub const RXQ_CTRL_Q2_EN: c_uint = 0x20;
pub const RXQ_CTRL_Q3_EN: c_uint = 0x40;
pub const RXQ_CTRL_IPV6_XSUM_VERIFY_EN: c_uint = 0x80;
pub const RXQ_CTRL_HASH_TLEN_SHIFT: c_int = 8;
pub const RXQ_CTRL_HASH_TLEN_MASK: c_uint = 0xFF;
pub const RXQ_CTRL_HASH_TYPE_IPV4: c_uint = 0x10000;
pub const RXQ_CTRL_HASH_TYPE_IPV4_TCP: c_uint = 0x20000;
pub const RXQ_CTRL_HASH_TYPE_IPV6: c_uint = 0x40000;
pub const RXQ_CTRL_HASH_TYPE_IPV6_TCP: c_uint = 0x80000;
pub const RXQ_CTRL_RSS_MODE_DISABLE: c_int = 0;
pub const RXQ_CTRL_RSS_MODE_SQSINT: c_uint = 0x4000000;
pub const RXQ_CTRL_RSS_MODE_MQUESINT: c_uint = 0x8000000;
pub const RXQ_CTRL_RSS_MODE_MQUEMINT: c_uint = 0xC000000;
pub const RXQ_CTRL_NIP_QUEUE_SEL_TBL: c_uint = 0x10000000;
pub const RXQ_CTRL_HASH_ENABLE: c_uint = 0x20000000;
pub const RXQ_CTRL_CUT_THRU_EN: c_uint = 0x40000000;
pub const RXQ_CTRL_EN: c_uint = 0x80000000;
// Rx jumbo packet threshold and rrd  retirement timer
pub const REG_RXQ_JMBOSZ_RRDTIM: c_uint = 0x15A4;
//
// Jumbo packet threshold for non-VLAN packet, in QWORD (64-bit) unit.
// When the packet length greater than or equal to this value, RXQ
// shall start cut-through forwarding of the received packet.
//
pub const RXQ_JMBOSZ_TH_MASK: c_uint = 0x7ff;

pub const RXQ_JMBO_LKAH_MASK: c_uint = 0xf;
pub const RXQ_JMBO_LKAH_SHIFT: c_int = 11;
// RXF flow control register
pub const REG_RXQ_RXF_PAUSE_THRESH: c_uint = 0x15A8;
pub const RXQ_RXF_PAUSE_TH_HI_SHIFT: c_int = 0;
pub const RXQ_RXF_PAUSE_TH_HI_MASK: c_uint = 0xfff;
pub const RXQ_RXF_PAUSE_TH_LO_SHIFT: c_int = 16;
pub const RXQ_RXF_PAUSE_TH_LO_MASK: c_uint = 0xfff;
// DMA Engine Control Register
pub const REG_DMA_CTRL: c_uint = 0x15C0;
pub const DMA_CTRL_DMAR_IN_ORDER: c_uint = 0x1;
pub const DMA_CTRL_DMAR_ENH_ORDER: c_uint = 0x2;
pub const DMA_CTRL_DMAR_OUT_ORDER: c_uint = 0x4;
pub const DMA_CTRL_RCB_VALUE: c_uint = 0x8;
pub const DMA_CTRL_DMAR_BURST_LEN_SHIFT: c_int = 4;
pub const DMA_CTRL_DMAR_BURST_LEN_MASK: c_int = 7;
pub const DMA_CTRL_DMAW_BURST_LEN_SHIFT: c_int = 7;
pub const DMA_CTRL_DMAW_BURST_LEN_MASK: c_int = 7;
pub const DMA_CTRL_DMAR_REQ_PRI: c_uint = 0x400;
pub const DMA_CTRL_DMAR_DLY_CNT_MASK: c_uint = 0x1F;
pub const DMA_CTRL_DMAR_DLY_CNT_SHIFT: c_int = 11;
pub const DMA_CTRL_DMAW_DLY_CNT_MASK: c_uint = 0xF;
pub const DMA_CTRL_DMAW_DLY_CNT_SHIFT: c_int = 16;
pub const DMA_CTRL_TXCMB_EN: c_uint = 0x100000;
pub const DMA_CTRL_RXCMB_EN: c_uint = 0x200000;
// CMB/SMB Control Register
pub const REG_SMB_STAT_TIMER: c_uint = 0x15C4;
pub const REG_TRIG_RRD_THRESH: c_uint = 0x15CA;
pub const REG_TRIG_TPD_THRESH: c_uint = 0x15C8;
pub const REG_TRIG_TXTIMER: c_uint = 0x15CC;
pub const REG_TRIG_RXTIMER: c_uint = 0x15CE;
// HOST RXF Page 1,2,3 address
pub const REG_HOST_RXF1_PAGE0_LO: c_uint = 0x15D0;
pub const REG_HOST_RXF1_PAGE1_LO: c_uint = 0x15D4;
pub const REG_HOST_RXF2_PAGE0_LO: c_uint = 0x15D8;
pub const REG_HOST_RXF2_PAGE1_LO: c_uint = 0x15DC;
pub const REG_HOST_RXF3_PAGE0_LO: c_uint = 0x15E0;
pub const REG_HOST_RXF3_PAGE1_LO: c_uint = 0x15E4;
// Mail box
pub const REG_MB_RXF1_RADDR: c_uint = 0x15B4;
pub const REG_MB_RXF2_RADDR: c_uint = 0x15B8;
pub const REG_MB_RXF3_RADDR: c_uint = 0x15BC;
pub const REG_MB_TPD_PROD_IDX: c_uint = 0x15F0;
// RXF-Page 0-3  PageNo & Valid bit
pub const REG_HOST_RXF0_PAGE0_VLD: c_uint = 0x15F4;
pub const HOST_RXF_VALID: c_int = 1;
pub const HOST_RXF_PAGENO_SHIFT: c_int = 1;
pub const HOST_RXF_PAGENO_MASK: c_uint = 0x7F;
pub const REG_HOST_RXF0_PAGE1_VLD: c_uint = 0x15F5;
pub const REG_HOST_RXF1_PAGE0_VLD: c_uint = 0x15F6;
pub const REG_HOST_RXF1_PAGE1_VLD: c_uint = 0x15F7;
pub const REG_HOST_RXF2_PAGE0_VLD: c_uint = 0x15F8;
pub const REG_HOST_RXF2_PAGE1_VLD: c_uint = 0x15F9;
pub const REG_HOST_RXF3_PAGE0_VLD: c_uint = 0x15FA;
pub const REG_HOST_RXF3_PAGE1_VLD: c_uint = 0x15FB;
// Interrupt Status Register
pub const REG_ISR: c_uint = 0x1600;
pub const ISR_SMB: c_int = 1;

//
// Software manual interrupt, for debug. Set when SW_MAN_INT_EN is set
// in Table 51 Selene Master Control Register (Offset 0x1400).
//
pub const ISR_MANUAL: c_int = 4;

pub const ISR_HOST_RXF0_OV: c_uint = 0x10;
pub const ISR_HOST_RXF1_OV: c_uint = 0x20;
pub const ISR_HOST_RXF2_OV: c_uint = 0x40;
pub const ISR_HOST_RXF3_OV: c_uint = 0x80;
pub const ISR_TXF_UN: c_uint = 0x100;
pub const ISR_RX0_PAGE_FULL: c_uint = 0x200;
pub const ISR_DMAR_TO_RST: c_uint = 0x400;
pub const ISR_DMAW_TO_RST: c_uint = 0x800;
pub const ISR_GPHY: c_uint = 0x1000;
pub const ISR_TX_CREDIT: c_uint = 0x2000;
pub const ISR_GPHY_LPW: c_uint = 0x4000    /* GPHY low power state interrupt */;
pub const ISR_RX_PKT: c_uint = 0x10000   /* One packet received, triggered by RFD */;
pub const ISR_TX_PKT: c_uint = 0x20000   /* One packet transmitted, triggered by TPD */;
pub const ISR_TX_DMA: c_uint = 0x40000;
pub const ISR_RX_PKT_1: c_uint = 0x80000;
pub const ISR_RX_PKT_2: c_uint = 0x100000;
pub const ISR_RX_PKT_3: c_uint = 0x200000;
pub const ISR_MAC_RX: c_uint = 0x400000;
pub const ISR_MAC_TX: c_uint = 0x800000;
pub const ISR_UR_DETECTED: c_uint = 0x1000000;
pub const ISR_FERR_DETECTED: c_uint = 0x2000000;
pub const ISR_NFERR_DETECTED: c_uint = 0x4000000;
pub const ISR_CERR_DETECTED: c_uint = 0x8000000;
pub const ISR_PHY_LINKDOWN: c_uint = 0x10000000;
pub const ISR_DIS_INT: c_uint = 0x80000000;
// Interrupt Mask Register
pub const REG_IMR: c_uint = 0x1604;

pub const REG_MAC_RX_STATUS_BIN: c_uint = 0x1700;
pub const REG_MAC_RX_STATUS_END: c_uint = 0x175c;
pub const REG_MAC_TX_STATUS_BIN: c_uint = 0x1760;
pub const REG_MAC_TX_STATUS_END: c_uint = 0x17c0;
// Hardware Offset Register
pub const REG_HOST_RXF0_PAGEOFF: c_uint = 0x1800;
pub const REG_TPD_CONS_IDX: c_uint = 0x1804;
pub const REG_HOST_RXF1_PAGEOFF: c_uint = 0x1808;
pub const REG_HOST_RXF2_PAGEOFF: c_uint = 0x180C;
pub const REG_HOST_RXF3_PAGEOFF: c_uint = 0x1810;
// RXF-Page 0-3 Offset DMA Address
pub const REG_HOST_RXF0_MB0_LO: c_uint = 0x1820;
pub const REG_HOST_RXF0_MB1_LO: c_uint = 0x1824;
pub const REG_HOST_RXF1_MB0_LO: c_uint = 0x1828;
pub const REG_HOST_RXF1_MB1_LO: c_uint = 0x182C;
pub const REG_HOST_RXF2_MB0_LO: c_uint = 0x1830;
pub const REG_HOST_RXF2_MB1_LO: c_uint = 0x1834;
pub const REG_HOST_RXF3_MB0_LO: c_uint = 0x1838;
pub const REG_HOST_RXF3_MB1_LO: c_uint = 0x183C;
// Tpd CMB DMA Address
pub const REG_HOST_TX_CMB_LO: c_uint = 0x1840;
pub const REG_HOST_SMB_ADDR_LO: c_uint = 0x1844;
// DEBUG ADDR
pub const REG_DEBUG_DATA0: c_uint = 0x1900;
pub const REG_DEBUG_DATA1: c_uint = 0x1904;
// MII definition
// PHY Common Register
pub const MII_AT001_PSCR: c_uint = 0x10;
pub const MII_AT001_PSSR: c_uint = 0x11;
pub const MII_INT_CTRL: c_uint = 0x12;
pub const MII_INT_STATUS: c_uint = 0x13;
pub const MII_SMARTSPEED: c_uint = 0x14;
pub const MII_LBRERROR: c_uint = 0x18;
pub const MII_RESV2: c_uint = 0x1a;
pub const MII_DBG_ADDR: c_uint = 0x1D;
pub const MII_DBG_DATA: c_uint = 0x1E;
// Autoneg Advertisement Register
pub const MII_AR_DEFAULT_CAP_MASK: c_int = 0;
// 1000BASE-T Control Register

// AT001 PHY Specific Control Register
pub const MII_AT001_PSCR_JABBER_DISABLE: c_uint = 0x0001  /* 1=Jabber Function disabled */;
pub const MII_AT001_PSCR_POLARITY_REVERSAL: c_uint = 0x0002  /* 1=Polarity Reversal enabled */;
pub const MII_AT001_PSCR_SQE_TEST: c_uint = 0x0004  /* 1=SQE Test enabled */;
pub const MII_AT001_PSCR_MAC_POWERDOWN: c_uint = 0x0008;
pub const MII_AT001_PSCR_CLK125_DISABLE: c_uint = 0x0010  /* 1=CLK125 low,;
// 0=CLK125 toggling
//
pub const MII_AT001_PSCR_MDI_MANUAL_MODE: c_uint = 0x0000  /* MDI Crossover Mode bits 6:5 */;
// Manual MDI configuration
pub const MII_AT001_PSCR_MDIX_MANUAL_MODE: c_uint = 0x0020  /* Manual MDIX configuration */;
pub const MII_AT001_PSCR_AUTO_X_1000T: c_uint = 0x0040  /* 1000BASE-T: Auto crossover,;
// 100BASE-TX/10BASE-T:
// MDI Mode
//
pub const MII_AT001_PSCR_AUTO_X_MODE: c_uint = 0x0060  /* Auto crossover enabled;
// all speeds.
//
pub const MII_AT001_PSCR_10BT_EXT_DIST_ENABLE: c_uint = 0x0080;
// 1=Enable Extended 10BASE-T distance
// (Lower 10BASE-T RX Threshold)
// 0=Normal 10BASE-T RX Threshold
pub const MII_AT001_PSCR_MII_5BIT_ENABLE: c_uint = 0x0100;
// 1=5-Bit interface in 100BASE-TX
// 0=MII interface in 100BASE-TX
pub const MII_AT001_PSCR_SCRAMBLER_DISABLE: c_uint = 0x0200  /* 1=Scrambler disable */;
pub const MII_AT001_PSCR_FORCE_LINK_GOOD: c_uint = 0x0400  /* 1=Force link good */;
pub const MII_AT001_PSCR_ASSERT_CRS_ON_TX: c_uint = 0x0800  /* 1=Assert CRS on Transmit */;
pub const MII_AT001_PSCR_POLARITY_REVERSAL_SHIFT: c_int = 1;
pub const MII_AT001_PSCR_AUTO_X_MODE_SHIFT: c_int = 5;
pub const MII_AT001_PSCR_10BT_EXT_DIST_ENABLE_SHIFT: c_int = 7;
// AT001 PHY Specific Status Register
pub const MII_AT001_PSSR_SPD_DPLX_RESOLVED: c_uint = 0x0800  /* 1=Speed & Duplex resolved */;
pub const MII_AT001_PSSR_DPLX: c_uint = 0x2000  /* 1=Duplex 0=Half Duplex */;
pub const MII_AT001_PSSR_SPEED: c_uint = 0xC000  /* Speed, bits 14:15 */;
pub const MII_AT001_PSSR_10MBS: c_uint = 0x0000  /* 00=10Mbs */;
pub const MII_AT001_PSSR_100MBS: c_uint = 0x4000  /* 01=100Mbs */;
pub const MII_AT001_PSSR_1000MBS: c_uint = 0x8000  /* 10=1000Mbs */;
