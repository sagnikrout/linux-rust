//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atl1c/atl1c_hw.h
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
// Copyright(c) 2008 - 2009 Atheros Corporation. All rights reserved.
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

// function prototype
extern "C" {
    pub fn atl1c_phy_disable(hw: *mut atl1c_hw);
}
extern "C" {
    pub fn atl1c_hw_set_mac_addr(hw: *mut atl1c_hw, mac_addr: *mut u8);
}
extern "C" {
    pub fn atl1c_phy_reset(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_read_mac_addr(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_get_link_status(hw: *mut atl1c_hw) -> bool;
}
extern "C" {
    pub fn atl1c_get_speed_and_duplex(hw: *mut atl1c_hw, speed: *mut u16, duplex: *mut u16) -> c_int;
}
extern "C" {
    pub fn atl1c_hash_mc_addr(hw: *mut atl1c_hw, mc_addr: *mut u8) -> u32;
}
extern "C" {
    pub fn atl1c_hash_set(hw: *mut atl1c_hw, hash_value: u32);
}
extern "C" {
    pub fn atl1c_read_phy_reg(hw: *mut atl1c_hw, reg_addr: u16, phy_data: *mut u16) -> c_int;
}
extern "C" {
    pub fn atl1c_write_phy_reg(hw: *mut atl1c_hw, reg_addr: u32, phy_data: u16) -> c_int;
}
extern "C" {
    pub fn atl1c_read_eeprom(hw: *mut atl1c_hw, offset: u32, p_value: *mut u32) -> bool;
}
extern "C" {
    pub fn atl1c_phy_init(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_check_eeprom_exist(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_restart_autoneg(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_phy_to_ps_link(hw: *mut atl1c_hw) -> c_int;
}
extern "C" {
    pub fn atl1c_power_saving(hw: *mut atl1c_hw, wufc: u32) -> c_int;
}
extern "C" {
    pub fn atl1c_wait_mdio_idle(hw: *mut atl1c_hw) -> bool;
}
extern "C" {
    pub fn atl1c_stop_phy_polling(hw: *mut atl1c_hw);
}
extern "C" {
    pub fn atl1c_start_phy_polling(hw: *mut atl1c_hw, clk_sel: u16);
}
extern "C" {
    pub fn atl1c_read_phy_dbg(hw: *mut atl1c_hw, reg_addr: u16, phy_data: *mut u16) -> c_int;
}
extern "C" {
    pub fn atl1c_write_phy_dbg(hw: *mut atl1c_hw, reg_addr: u16, phy_data: u16) -> c_int;
}
extern "C" {
    pub fn atl1c_post_phy_linkchg(hw: *mut atl1c_hw, link_speed: u16);
}
// hw-ids
pub const PCI_DEVICE_ID_ATTANSIC_L2C: c_uint = 0x1062;
pub const PCI_DEVICE_ID_ATTANSIC_L1C: c_uint = 0x1063;
pub const PCI_DEVICE_ID_ATHEROS_L2C_B: c_uint = 0x2060 /* AR8152 v1.1 Fast 10/100 */;
pub const PCI_DEVICE_ID_ATHEROS_L2C_B2: c_uint = 0x2062 /* AR8152 v2.0 Fast 10/100 */;
pub const PCI_DEVICE_ID_ATHEROS_L1D: c_uint = 0x1073 /* AR8151 v1.0 Gigabit 1000 */;
pub const PCI_DEVICE_ID_ATHEROS_L1D_2_0: c_uint = 0x1083 /* AR8151 v2.0 Gigabit 1000 */;
pub const L2CB_V10: c_uint = 0xc0;
pub const L2CB_V11: c_uint = 0xc1;
pub const L2CB_V20: c_uint = 0xc0;
pub const L2CB_V21: c_uint = 0xc1;
// register definition
pub const REG_DEVICE_CAP: c_uint = 0x5C;
pub const DEVICE_CAP_MAX_PAYLOAD_MASK: c_uint = 0x7;
pub const DEVICE_CAP_MAX_PAYLOAD_SHIFT: c_int = 0;
pub const DEVICE_CTRL_MAXRRS_MIN: c_int = 2;
pub const REG_LINK_CTRL: c_uint = 0x68;
pub const LINK_CTRL_L0S_EN: c_uint = 0x01;
pub const LINK_CTRL_L1_EN: c_uint = 0x02;
pub const LINK_CTRL_EXT_SYNC: c_uint = 0x80;
pub const REG_PCIE_IND_ACC_ADDR: c_uint = 0x80;
pub const REG_PCIE_IND_ACC_DATA: c_uint = 0x84;
pub const REG_DEV_SERIALNUM_CTRL: c_uint = 0x200;
pub const REG_DEV_MAC_SEL_MASK: c_uint = 0x0 /* 0:EUI; 1:MAC */;
pub const REG_DEV_MAC_SEL_SHIFT: c_int = 0;
pub const REG_DEV_SERIAL_NUM_EN_MASK: c_uint = 0x1;
pub const REG_DEV_SERIAL_NUM_EN_SHIFT: c_int = 1;
pub const REG_TWSI_CTRL: c_uint = 0x218;
pub const TWSI_CTLR_FREQ_MASK: c_uint = 0x3UL;
pub const TWSI_CTRL_FREQ_SHIFT: c_int = 24;
pub const TWSI_CTRL_FREQ_100K: c_int = 0;
pub const TWSI_CTRL_FREQ_200K: c_int = 1;
pub const TWSI_CTRL_FREQ_300K: c_int = 2;
pub const TWSI_CTRL_FREQ_400K: c_int = 3;

pub const TWSI_CTRL_LD_OFFSET_MASK: c_uint = 0xFF;
pub const TWSI_CTRL_LD_OFFSET_SHIFT: c_int = 0;
pub const REG_PCIE_DEV_MISC_CTRL: c_uint = 0x21C;
pub const PCIE_DEV_MISC_EXT_PIPE: c_uint = 0x2;
pub const PCIE_DEV_MISC_RETRY_BUFDIS: c_uint = 0x1;
pub const PCIE_DEV_MISC_SPIROM_EXIST: c_uint = 0x4;
pub const PCIE_DEV_MISC_SERDES_ENDIAN: c_uint = 0x8;
pub const PCIE_DEV_MISC_SERDES_SEL_DIN: c_uint = 0x10;
pub const REG_PCIE_PHYMISC: c_uint = 0x1000;

pub const PCIE_PHYMISC_NFTS_MASK: c_uint = 0xFFUL;
pub const PCIE_PHYMISC_NFTS_SHIFT: c_int = 16;
pub const REG_PCIE_PHYMISC2: c_uint = 0x1004;
pub const PCIE_PHYMISC2_L0S_TH_MASK: c_uint = 0x3UL;
pub const PCIE_PHYMISC2_L0S_TH_SHIFT: c_int = 18;
pub const L2CB1_PCIE_PHYMISC2_L0S_TH: c_int = 3;
pub const PCIE_PHYMISC2_CDR_BW_MASK: c_uint = 0x3UL;
pub const PCIE_PHYMISC2_CDR_BW_SHIFT: c_int = 16;
pub const L2CB1_PCIE_PHYMISC2_CDR_BW: c_int = 3;
pub const REG_TWSI_DEBUG: c_uint = 0x1108;

pub const REG_DMA_DBG: c_uint = 0x1114;

pub const REG_EEPROM_CTRL: c_uint = 0x12C0;
pub const EEPROM_CTRL_DATA_HI_MASK: c_uint = 0xFFFF;
pub const EEPROM_CTRL_DATA_HI_SHIFT: c_int = 0;
pub const EEPROM_CTRL_ADDR_MASK: c_uint = 0x3FF;
pub const EEPROM_CTRL_ADDR_SHIFT: c_int = 16;
pub const EEPROM_CTRL_ACK: c_uint = 0x40000000;
pub const EEPROM_CTRL_RW: c_uint = 0x80000000;
pub const REG_EEPROM_DATA_LO: c_uint = 0x12C4;
pub const REG_OTP_CTRL: c_uint = 0x12F0;

pub const REG_PM_CTRL: c_uint = 0x12F8;

// thrghput(setting in 15A0)

pub const PM_CTRL_LCKDET_TIMER_MASK: c_uint = 0xFUL;
pub const PM_CTRL_LCKDET_TIMER_SHIFT: c_int = 24;
pub const PM_CTRL_LCKDET_TIMER_DEF: c_uint = 0xC;
pub const PM_CTRL_PM_REQ_TIMER_MASK: c_uint = 0xFUL;

// ->L0s not L1
pub const PM_CTRL_PM_REQ_TO_DEF: c_uint = 0xF;

pub const L1D_PMCTRL_L1_ENTRY_TM_SHIFT: c_int = 16;
pub const L1D_PMCTRL_L1_ENTRY_TM_DIS: c_int = 0;
pub const L1D_PMCTRL_L1_ENTRY_TM_2US: c_int = 1;
pub const L1D_PMCTRL_L1_ENTRY_TM_4US: c_int = 2;
pub const L1D_PMCTRL_L1_ENTRY_TM_8US: c_int = 3;
pub const L1D_PMCTRL_L1_ENTRY_TM_16US: c_int = 4;
pub const L1D_PMCTRL_L1_ENTRY_TM_24US: c_int = 5;
pub const L1D_PMCTRL_L1_ENTRY_TM_32US: c_int = 6;
pub const L1D_PMCTRL_L1_ENTRY_TM_63US: c_int = 7;
pub const PM_CTRL_L1_ENTRY_TIMER_MASK: c_uint = 0xFUL  /* l1C 4bits */;
pub const PM_CTRL_L1_ENTRY_TIMER_SHIFT: c_int = 16;
pub const L2CB1_PM_CTRL_L1_ENTRY_TM: c_int = 7;
pub const L1C_PM_CTRL_L1_ENTRY_TM: c_uint = 0xF;

pub const L1D_PMCTRL_L0S_TIMER_SHIFT: c_int = 8;
pub const PM_CTRL_L0S_ENTRY_TIMER_MASK: c_uint = 0xFUL	/* l1c, 4bits */;
pub const PM_CTRL_L0S_ENTRY_TIMER_SHIFT: c_int = 8;

pub const REG_LTSSM_ID_CTRL: c_uint = 0x12FC;
pub const LTSSM_ID_EN_WRO: c_uint = 0x1000;
// Selene Master Control Register
pub const REG_MASTER_CTRL: c_uint = 0x1400;

pub const MASTER_DEV_NUM_MASK: c_uint = 0x7FUL;
pub const MASTER_DEV_NUM_SHIFT: c_int = 24;
pub const MASTER_REV_NUM_MASK: c_uint = 0xFFUL;
pub const MASTER_REV_NUM_SHIFT: c_int = 16;

// serdes, not sw to 25M

pub const MASTER_PCIE_TSTMOD_SHIFT: c_int = 2;

pub const DMA_MAC_RST_TO: c_int = 50;
// Timer Initial Value Register
pub const REG_MANUAL_TIMER_INIT: c_uint = 0x1404;
// IRQ ModeratorTimer Initial Value Register
pub const REG_IRQ_MODRT_TIMER_INIT: c_uint = 0x1408;
pub const IRQ_MODRT_TIMER_MASK: c_uint = 0xffff;
pub const IRQ_MODRT_TX_TIMER_SHIFT: c_int = 0;
pub const IRQ_MODRT_RX_TIMER_SHIFT: c_int = 16;
pub const REG_GPHY_CTRL: c_uint = 0x140C;
pub const GPHY_CTRL_ADDR_MASK: c_uint = 0x1FUL;
pub const GPHY_CTRL_ADDR_SHIFT: c_int = 19;

// Block IDLE Status Register
pub const REG_IDLE_STATUS: c_uint = 0x1410;
pub const IDLE_STATUS_SFORCE_MASK: c_uint = 0xFUL;
pub const IDLE_STATUS_SFORCE_SHIFT: c_int = 14;

pub const IDLE_STATUS_CALIB_RES_MASK: c_uint = 0x1FUL;
pub const IDLE_STATUS_CALIB_RES_SHIFT: c_int = 8;
pub const IDLE_STATUS_CALIBERR_MASK: c_uint = 0xFUL;
pub const IDLE_STATUS_CALIBERR_SHIFT: c_int = 4;

// MDIO Control Register
pub const REG_MDIO_CTRL: c_uint = 0x1414;

pub const MDIO_CTRL_CLK_SEL_MASK: c_uint = 0x7UL;
pub const MDIO_CTRL_CLK_SEL_SHIFT: c_int = 24;

pub const MDIO_CTRL_CLK_25_6: c_int = 2;
pub const MDIO_CTRL_CLK_25_8: c_int = 3;
pub const MDIO_CTRL_CLK_25_10: c_int = 4;
pub const MDIO_CTRL_CLK_25_32: c_int = 5;
pub const MDIO_CTRL_CLK_25_64: c_int = 6;
pub const MDIO_CTRL_CLK_25_128: c_int = 7;

pub const MDIO_CTRL_REG_MASK: c_uint = 0x1FUL;
pub const MDIO_CTRL_REG_SHIFT: c_int = 16;
pub const MDIO_CTRL_DATA_MASK: c_uint = 0xFFFFUL;
pub const MDIO_CTRL_DATA_SHIFT: c_int = 0;

// for extension reg access
pub const REG_MDIO_EXTN: c_uint = 0x1448;
pub const MDIO_EXTN_PORTAD_MASK: c_uint = 0x1FUL;
pub const MDIO_EXTN_PORTAD_SHIFT: c_int = 21;
pub const MDIO_EXTN_DEVAD_MASK: c_uint = 0x1FUL;
pub const MDIO_EXTN_DEVAD_SHIFT: c_int = 16;
pub const MDIO_EXTN_REG_MASK: c_uint = 0xFFFFUL;
pub const MDIO_EXTN_REG_SHIFT: c_int = 0;
// BIST Control and Status Register0 (for the Packet Memory)
pub const REG_BIST0_CTRL: c_uint = 0x141c;
pub const BIST0_NOW: c_uint = 0x1;
pub const BIST0_SRAM_FAIL: c_uint = 0x2 /* 1: The SRAM failure is;
// un-repairable  because
// it has address decoder
// failure or more than 1 cell
// stuck-to-x failure
pub const BIST0_FUSE_FLAG: c_uint = 0x4;
// BIST Control and Status Register1(for the retry buffer of PCI Express)
pub const REG_BIST1_CTRL: c_uint = 0x1420;
pub const BIST1_NOW: c_uint = 0x1;
pub const BIST1_SRAM_FAIL: c_uint = 0x2;
pub const BIST1_FUSE_FLAG: c_uint = 0x4;
// SerDes Lock Detect Control and Status Register
pub const REG_SERDES: c_uint = 0x1424;

pub const SERDES_SELFB_PLL_MASK: c_uint = 0x3UL;
pub const SERDES_SELFB_PLL_SHIFT: c_int = 14;

pub const SERDES_SELFB_PLL_CSR_MASK: c_uint = 0x3UL;
pub const SERDES_SELFB_PLL_CSR_SHIFT: c_int = 4;

pub const REG_LPI_DECISN_TIMER: c_uint = 0x143C;
pub const L2CB_LPI_DESISN_TIMER: c_uint = 0x7D00;
pub const REG_LPI_CTRL: c_uint = 0x1440;

pub const LPI_CTRL_ENH_TO_MASK: c_uint = 0x1FFFUL;
pub const LPI_CTRL_ENH_TO_SHIFT: c_int = 12;
pub const LPI_CTRL_ENH_TH_MASK: c_uint = 0x1FUL;
pub const LPI_CTRL_ENH_TH_SHIFT: c_int = 6;

pub const REG_LPI_WAIT: c_uint = 0x1444;
pub const LPI_WAIT_TIMER_MASK: c_uint = 0xFFFFUL;
pub const LPI_WAIT_TIMER_SHIFT: c_int = 0;
// MAC Control Register
pub const REG_MAC_CTRL: c_uint = 0x1480;

pub const MAC_CTRL_SPEED_SHIFT: c_int = 20;
pub const MAC_CTRL_SPEED_10_100: c_int = 1;
pub const MAC_CTRL_SPEED_1000: c_int = 2;

pub const MAC_CTRL_PRMLEN_MASK: c_uint = 0xFUL;
pub const MAC_CTRL_PRMLEN_SHIFT: c_int = 10;

// MAC IPG/IFG Control Register
pub const REG_MAC_IPG_IFG: c_uint = 0x1484;

// inter-packet gap. The
// default is 96-bit time
pub const MAC_IPG_IFG_IPGT_MASK: c_uint = 0x7f;

// enforce in between RX frames
pub const MAC_IPG_IFG_MIFG_MASK: c_uint = 0xff  	/* Frame gap below such IFP is dropped */;

pub const MAC_IPG_IFG_IPGR1_MASK: c_uint = 0x7f;

pub const MAC_IPG_IFG_IPGR2_MASK: c_uint = 0x7f;
// MAC STATION ADDRESS
pub const REG_MAC_STA_ADDR: c_uint = 0x1488;
// Hash table for multicast address
pub const REG_RX_HASH_TABLE: c_uint = 0x1490;
// MAC Half-Duplex Control Register
pub const REG_MAC_HALF_DUPLX_CTRL: c_uint = 0x1498;

pub const MAC_HALF_DUPLX_CTRL_LCOL_MASK: c_uint = 0x3ff;
pub const MAC_HALF_DUPLX_CTRL_RETRY_SHIFT: c_int = 12;
pub const MAC_HALF_DUPLX_CTRL_RETRY_MASK: c_uint = 0xf;
pub const MAC_HALF_DUPLX_CTRL_EXC_DEF_EN: c_uint = 0x10000;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_C: c_uint = 0x20000;
pub const MAC_HALF_DUPLX_CTRL_NO_BACK_P: c_uint = 0x40000 /* No back-off on backpressure,;
// immediately start the
// transmission after back pressure
pub const MAC_HALF_DUPLX_CTRL_ABEBE: c_uint = 0x80000 /* 1: Alternative Binary Exponential Back-off Enabled */;

pub const MAC_HALF_DUPLX_CTRL_ABEBT_MASK: c_uint = 0xf;

pub const MAC_HALF_DUPLX_CTRL_JAMIPG_MASK: c_uint = 0xf     /* mode. In unit of 8-bit time */;
// Maximum Frame Length Control Register
pub const REG_MTU: c_uint = 0x149c;
// Wake-On-Lan control register
pub const REG_WOL_CTRL: c_uint = 0x14a0;

// WOL Length ( 2 DWORD )
pub const REG_WOL_PTLEN1: c_uint = 0x14A4;
pub const WOL_PTLEN1_3_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN1_3_SHIFT: c_int = 24;
pub const WOL_PTLEN1_2_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN1_2_SHIFT: c_int = 16;
pub const WOL_PTLEN1_1_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN1_1_SHIFT: c_int = 8;
pub const WOL_PTLEN1_0_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN1_0_SHIFT: c_int = 0;
pub const REG_WOL_PTLEN2: c_uint = 0x14A8;
pub const WOL_PTLEN2_7_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN2_7_SHIFT: c_int = 24;
pub const WOL_PTLEN2_6_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN2_6_SHIFT: c_int = 16;
pub const WOL_PTLEN2_5_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN2_5_SHIFT: c_int = 8;
pub const WOL_PTLEN2_4_MASK: c_uint = 0xFFUL;
pub const WOL_PTLEN2_4_SHIFT: c_int = 0;
// Internal SRAM Partition Register
pub const RFDX_HEAD_ADDR_MASK: c_uint = 0x03FF;
pub const RFDX_HARD_ADDR_SHIFT: c_int = 0;
pub const RFDX_TAIL_ADDR_MASK: c_uint = 0x03FF;
pub const RFDX_TAIL_ADDR_SHIFT: c_int = 16;
pub const REG_SRAM_RFD0_INFO: c_uint = 0x1500;
pub const REG_SRAM_RFD1_INFO: c_uint = 0x1504;
pub const REG_SRAM_RFD2_INFO: c_uint = 0x1508;
pub const REG_SRAM_RFD3_INFO: c_uint = 0x150C;
pub const REG_RFD_NIC_LEN: c_uint = 0x1510 /* In 8-bytes */;
pub const RFD_NIC_LEN_MASK: c_uint = 0x03FF;
pub const REG_SRAM_TRD_ADDR: c_uint = 0x1518;
pub const TPD_HEAD_ADDR_MASK: c_uint = 0x03FF;
pub const TPD_HEAD_ADDR_SHIFT: c_int = 0;
pub const TPD_TAIL_ADDR_MASK: c_uint = 0x03FF;
pub const TPD_TAIL_ADDR_SHIFT: c_int = 16;
pub const REG_SRAM_TRD_LEN: c_uint = 0x151C /* In 8-bytes */;
pub const TPD_NIC_LEN_MASK: c_uint = 0x03FF;
pub const REG_SRAM_RXF_ADDR: c_uint = 0x1520;
pub const REG_SRAM_RXF_LEN: c_uint = 0x1524;
pub const REG_SRAM_TXF_ADDR: c_uint = 0x1528;
pub const REG_SRAM_TXF_LEN: c_uint = 0x152C;
pub const REG_SRAM_TCPH_ADDR: c_uint = 0x1530;
pub const REG_SRAM_PKTH_ADDR: c_uint = 0x1532;
//
// Load Ptr Register
// Software sets this bit after the initialization of the head and tail
pub const REG_LOAD_PTR: c_uint = 0x1534;
//
// addresses of all descriptors, as well as the following descriptor
// control register, which triggers each function block to load the head
// pointer to prepare for the operation. This bit is then self-cleared
// after one cycle.
//
pub const REG_RX_BASE_ADDR_HI: c_uint = 0x1540;
pub const REG_TX_BASE_ADDR_HI: c_uint = 0x1544;
pub const REG_RFD0_HEAD_ADDR_LO: c_uint = 0x1550;
pub const REG_RFD1_HEAD_ADDR_LO: c_uint = 0x1554;
pub const REG_RFD2_HEAD_ADDR_LO: c_uint = 0x1558;
pub const REG_RFD3_HEAD_ADDR_LO: c_uint = 0x155C;
pub const REG_RFD_RING_SIZE: c_uint = 0x1560;
pub const RFD_RING_SIZE_MASK: c_uint = 0x0FFF;
pub const REG_RX_BUF_SIZE: c_uint = 0x1564;
pub const RX_BUF_SIZE_MASK: c_uint = 0xFFFF;
pub const REG_RRD0_HEAD_ADDR_LO: c_uint = 0x1568;
pub const REG_RRD1_HEAD_ADDR_LO: c_uint = 0x156C;
pub const REG_RRD2_HEAD_ADDR_LO: c_uint = 0x1570;
pub const REG_RRD3_HEAD_ADDR_LO: c_uint = 0x1574;
pub const REG_RRD_RING_SIZE: c_uint = 0x1578;
pub const RRD_RING_SIZE_MASK: c_uint = 0x0FFF;
pub const REG_TPD_PRI1_ADDR_LO: c_uint = 0x157C;
pub const REG_TPD_PRI0_ADDR_LO: c_uint = 0x1580;
pub const REG_TPD_PRI2_ADDR_LO: c_uint = 0x1F10;
pub const REG_TPD_PRI3_ADDR_LO: c_uint = 0x1F14;
pub const REG_TPD_RING_SIZE: c_uint = 0x1584;
pub const TPD_RING_SIZE_MASK: c_uint = 0xFFFF;
// TXQ Control Register
pub const REG_TXQ_CTRL: c_uint = 0x1590;
pub const TXQ_TXF_BURST_NUM_MASK: c_uint = 0xFFFFUL;
pub const TXQ_TXF_BURST_NUM_SHIFT: c_int = 16;
pub const L1C_TXQ_TXF_BURST_PREF: c_uint = 0x200;
pub const L2CB_TXQ_TXF_BURST_PREF: c_uint = 0x40;

pub const TXQ_NUM_TPD_BURST_MASK: c_uint = 0xFUL;
pub const TXQ_NUM_TPD_BURST_SHIFT: c_int = 0;
pub const TXQ_NUM_TPD_BURST_DEF: c_int = 5;

// Jumbo packet Threshold for task offload
pub const REG_TX_TSO_OFFLOAD_THRESH: c_uint = 0x1594 /* In 8-bytes */;
pub const TX_TSO_OFFLOAD_THRESH_MASK: c_uint = 0x07FF;

pub const REG_TXF_WATER_MARK: c_uint = 0x1598 /* In 8-bytes */;
pub const TXF_WATER_MARK_MASK: c_uint = 0x0FFF;
pub const TXF_LOW_WATER_MARK_SHIFT: c_int = 0;
pub const TXF_HIGH_WATER_MARK_SHIFT: c_int = 16;
pub const TXQ_CTRL_BURST_MODE_EN: c_uint = 0x80000000;
pub const REG_THRUPUT_MON_CTRL: c_uint = 0x159C;
pub const THRUPUT_MON_RATE_MASK: c_uint = 0x3;
pub const THRUPUT_MON_RATE_SHIFT: c_int = 0;
pub const THRUPUT_MON_EN: c_uint = 0x80;
// RXQ Control Register
pub const REG_RXQ_CTRL: c_uint = 0x15A0;
pub const ASPM_THRUPUT_LIMIT_MASK: c_uint = 0x3;
pub const ASPM_THRUPUT_LIMIT_SHIFT: c_int = 0;
pub const ASPM_THRUPUT_LIMIT_NO: c_uint = 0x00;
pub const ASPM_THRUPUT_LIMIT_1M: c_uint = 0x01;
pub const ASPM_THRUPUT_LIMIT_10M: c_uint = 0x02;
pub const ASPM_THRUPUT_LIMIT_100M: c_uint = 0x03;

pub const RXQ_RFD_BURST_NUM_MASK: c_uint = 0x003F;
pub const RXQ_RFD_BURST_NUM_SHIFT: c_int = 20;
pub const RXQ_NUM_RFD_PREF_DEF: c_int = 8;

pub const RSS_MODE_SHIFT: c_int = 26;
pub const RSS_MODE_DIS: c_int = 0;
pub const RSS_MODE_SQSI: c_int = 1;
pub const RSS_MODE_MQSI: c_int = 2;
pub const RSS_MODE_MQMI: c_int = 3;

pub const REG_RFD_FREE_THRESH: c_uint = 0x15A4;
pub const RFD_FREE_THRESH_MASK: c_uint = 0x003F;
pub const RFD_FREE_HI_THRESH_SHIFT: c_int = 0;
pub const RFD_FREE_LO_THRESH_SHIFT: c_int = 6;
// RXF flow control register
pub const REG_RXQ_RXF_PAUSE_THRESH: c_uint = 0x15A8;
pub const RXQ_RXF_PAUSE_TH_HI_SHIFT: c_int = 0;
pub const RXQ_RXF_PAUSE_TH_HI_MASK: c_uint = 0x0FFF;
pub const RXQ_RXF_PAUSE_TH_LO_SHIFT: c_int = 16;
pub const RXQ_RXF_PAUSE_TH_LO_MASK: c_uint = 0x0FFF;
pub const REG_RXD_DMA_CTRL: c_uint = 0x15AC;
pub const RXD_DMA_THRESH_MASK: c_uint = 0x0FFF	/* In 8-bytes */;
pub const RXD_DMA_THRESH_SHIFT: c_int = 0;
pub const RXD_DMA_DOWN_TIMER_MASK: c_uint = 0xFFFF;
pub const RXD_DMA_DOWN_TIMER_SHIFT: c_int = 16;
// DMA Engine Control Register
pub const REG_DMA_CTRL: c_uint = 0x15C0;

pub const DMA_CTRL_WDLY_CNT_MASK: c_uint = 0xFUL;
pub const DMA_CTRL_WDLY_CNT_SHIFT: c_int = 16;
pub const DMA_CTRL_WDLY_CNT_DEF: c_int = 4;
pub const DMA_CTRL_RDLY_CNT_MASK: c_uint = 0x1FUL;
pub const DMA_CTRL_RDLY_CNT_SHIFT: c_int = 11;
pub const DMA_CTRL_RDLY_CNT_DEF: c_int = 15;

pub const DMA_CTRL_WREQ_BLEN_SHIFT: c_int = 7;

pub const DMA_CTRL_RREQ_BLEN_SHIFT: c_int = 4;

pub const DMA_CTRL_RORDER_MODE_SHIFT: c_int = 0;
pub const DMA_CTRL_RORDER_MODE_OUT: c_int = 4;
pub const DMA_CTRL_RORDER_MODE_ENHANCE: c_int = 2;
pub const DMA_CTRL_RORDER_MODE_IN: c_int = 1;
// INT-triggle/SMB Control Register
pub const REG_SMB_STAT_TIMER: c_uint = 0x15C4	/* 2us resolution */;
pub const SMB_STAT_TIMER_MASK: c_uint = 0xFFFFFF;
pub const REG_TINT_TPD_THRESH: c_uint = 0x15C8 /* tpd th to trig intrrupt */;
// Mail box
pub const MB_RFDX_PROD_IDX_MASK: c_uint = 0xFFFF;
pub const REG_MB_RFD0_PROD_IDX: c_uint = 0x15E0;
pub const REG_MB_RFD1_PROD_IDX: c_uint = 0x15E4;
pub const REG_MB_RFD2_PROD_IDX: c_uint = 0x15E8;
pub const REG_MB_RFD3_PROD_IDX: c_uint = 0x15EC;
pub const REG_TPD_PRI1_PIDX: c_uint = 0x15F0	/* 16bit,hi-tpd producer idx */;
pub const REG_TPD_PRI0_PIDX: c_uint = 0x15F2	/* 16bit,lo-tpd producer idx */;
pub const REG_TPD_PRI1_CIDX: c_uint = 0x15F4	/* 16bit,hi-tpd consumer idx */;
pub const REG_TPD_PRI0_CIDX: c_uint = 0x15F6	/* 16bit,lo-tpd consumer idx */;
pub const REG_TPD_PRI3_PIDX: c_uint = 0x1F18;
pub const REG_TPD_PRI2_PIDX: c_uint = 0x1F1A;
pub const REG_TPD_PRI3_CIDX: c_uint = 0x1F1C;
pub const REG_TPD_PRI2_CIDX: c_uint = 0x1F1E;
pub const REG_MB_RFD01_CONS_IDX: c_uint = 0x15F8;
pub const MB_RFD0_CONS_IDX_MASK: c_uint = 0x0000FFFF;
pub const MB_RFD1_CONS_IDX_MASK: c_uint = 0xFFFF0000;
pub const REG_MB_RFD23_CONS_IDX: c_uint = 0x15FC;
pub const MB_RFD2_CONS_IDX_MASK: c_uint = 0x0000FFFF;
pub const MB_RFD3_CONS_IDX_MASK: c_uint = 0xFFFF0000;
// Interrupt Status Register
pub const REG_ISR: c_uint = 0x1600;
pub const ISR_SMB: c_uint = 0x00000001;
pub const ISR_TIMER: c_uint = 0x00000002;
//
// Software manual interrupt, for debug. Set when SW_MAN_INT_EN is set
// in Table 51 Selene Master Control Register (Offset 0x1400).
//
pub const ISR_MANUAL: c_uint = 0x00000004;
pub const ISR_HW_RXF_OV: c_uint = 0x00000008 /* RXF overflow interrupt */;
pub const ISR_RFD0_UR: c_uint = 0x00000010 /* RFD0 under run */;
pub const ISR_RFD1_UR: c_uint = 0x00000020;
pub const ISR_RFD2_UR: c_uint = 0x00000040;
pub const ISR_RFD3_UR: c_uint = 0x00000080;
pub const ISR_TXF_UR: c_uint = 0x00000100;
pub const ISR_DMAR_TO_RST: c_uint = 0x00000200;
pub const ISR_DMAW_TO_RST: c_uint = 0x00000400;
pub const ISR_TX_CREDIT: c_uint = 0x00000800;
pub const ISR_GPHY: c_uint = 0x00001000;
// GPHY low power state interrupt
pub const ISR_GPHY_LPW: c_uint = 0x00002000;
pub const ISR_TXQ_TO_RST: c_uint = 0x00004000;
pub const ISR_TX_PKT_0: c_uint = 0x00008000;
pub const ISR_RX_PKT_0: c_uint = 0x00010000;
pub const ISR_RX_PKT_1: c_uint = 0x00020000;
pub const ISR_RX_PKT_2: c_uint = 0x00040000;
pub const ISR_RX_PKT_3: c_uint = 0x00080000;
pub const ISR_MAC_RX: c_uint = 0x00100000;
pub const ISR_MAC_TX: c_uint = 0x00200000;
pub const ISR_UR_DETECTED: c_uint = 0x00400000;
pub const ISR_FERR_DETECTED: c_uint = 0x00800000;
pub const ISR_NFERR_DETECTED: c_uint = 0x01000000;
pub const ISR_CERR_DETECTED: c_uint = 0x02000000;
pub const ISR_PHY_LINKDOWN: c_uint = 0x04000000;
pub const ISR_TX_PKT_1: c_uint = 0x10000000;
pub const ISR_TX_PKT_2: c_uint = 0x20000000;
pub const ISR_TX_PKT_3: c_uint = 0x40000000;
pub const ISR_DIS_INT: c_uint = 0x80000000;
// Interrupt Mask Register
pub const REG_IMR: c_uint = 0x1604;

pub const REG_INT_RETRIG_TIMER: c_uint = 0x1608;
pub const INT_RETRIG_TIMER_MASK: c_uint = 0xFFFF;
pub const REG_MAC_RX_STATUS_BIN: c_uint = 0x1700;
pub const REG_MAC_RX_STATUS_END: c_uint = 0x175c;
pub const REG_MAC_TX_STATUS_BIN: c_uint = 0x1760;
pub const REG_MAC_TX_STATUS_END: c_uint = 0x17c0;
pub const REG_CLK_GATING_CTRL: c_uint = 0x1814;
pub const CLK_GATING_DMAW_EN: c_uint = 0x0001;
pub const CLK_GATING_DMAR_EN: c_uint = 0x0002;
pub const CLK_GATING_TXQ_EN: c_uint = 0x0004;
pub const CLK_GATING_RXQ_EN: c_uint = 0x0008;
pub const CLK_GATING_TXMAC_EN: c_uint = 0x0010;
pub const CLK_GATING_RXMAC_EN: c_uint = 0x0020;

// DEBUG ADDR
pub const REG_DEBUG_DATA0: c_uint = 0x1900;
pub const REG_DEBUG_DATA1: c_uint = 0x1904;
pub const REG_MT_MAGIC: c_uint = 0x1F00;
pub const REG_MT_MODE: c_uint = 0x1F04;
pub const REG_MT_SPEED: c_uint = 0x1F08;
pub const REG_MT_VERSION: c_uint = 0x1F0C;
pub const MT_MAGIC: c_uint = 0xaabb1234;

pub const L1D_MPW_PHYID1: c_uint = 0xD01C  /* V7 */;
pub const L1D_MPW_PHYID2: c_uint = 0xD01D  /* V1-V6 */;
pub const L1D_MPW_PHYID3: c_uint = 0xD01E  /* V8 */;
// Autoneg Advertisement Register

// 1000BASE-T Control Register
pub const GIGA_CR_1000T_REPEATER_DTE: c_uint = 0x0400  /* 1=Repeater/switch device port 0=DTE device */;
pub const GIGA_CR_1000T_MS_VALUE: c_uint = 0x0800  /* 1=Configure PHY as Master 0=Configure PHY as Slave */;
pub const GIGA_CR_1000T_MS_ENABLE: c_uint = 0x1000  /* 1=Master/Slave manual config value 0=Automatic Master/Slave config */;
pub const GIGA_CR_1000T_TEST_MODE_NORMAL: c_uint = 0x0000  /* Normal Operation */;
pub const GIGA_CR_1000T_TEST_MODE_1: c_uint = 0x2000  /* Transmit Waveform test */;
pub const GIGA_CR_1000T_TEST_MODE_2: c_uint = 0x4000  /* Master Transmit Jitter test */;
pub const GIGA_CR_1000T_TEST_MODE_3: c_uint = 0x6000  /* Slave Transmit Jitter test */;
pub const GIGA_CR_1000T_TEST_MODE_4: c_uint = 0x8000	/* Transmitter Distortion test */;
pub const GIGA_CR_1000T_SPEED_MASK: c_uint = 0x0300;
pub const GIGA_CR_1000T_DEFAULT_CAP: c_uint = 0x0300;
// PHY Specific Status Register
pub const MII_GIGA_PSSR: c_uint = 0x11;
pub const GIGA_PSSR_SPD_DPLX_RESOLVED: c_uint = 0x0800  /* 1=Speed & Duplex resolved */;
pub const GIGA_PSSR_DPLX: c_uint = 0x2000  /* 1=Duplex 0=Half Duplex */;
pub const GIGA_PSSR_SPEED: c_uint = 0xC000  /* Speed, bits 14:15 */;
pub const GIGA_PSSR_10MBS: c_uint = 0x0000  /* 00=10Mbs */;
pub const GIGA_PSSR_100MBS: c_uint = 0x4000  /* 01=100Mbs */;
pub const GIGA_PSSR_1000MBS: c_uint = 0x8000  /* 10=1000Mbs */;
// PHY Interrupt Enable Register
pub const MII_IER: c_uint = 0x12;
pub const IER_LINK_UP: c_uint = 0x0400;
pub const IER_LINK_DOWN: c_uint = 0x0800;
// PHY Interrupt Status Register
pub const MII_ISR: c_uint = 0x13;
pub const ISR_LINK_UP: c_uint = 0x0400;
pub const ISR_LINK_DOWN: c_uint = 0x0800;
// Cable-Detect-Test Control Register
pub const MII_CDTC: c_uint = 0x16;

pub const CDTC_EN_BITS: c_int = 1;
pub const CDTC_PAIR_OFF: c_int = 8;
pub const CDTC_PAIR_BIT: c_int = 2;
// Cable-Detect-Test Status Register
pub const MII_CDTS: c_uint = 0x1C;
pub const CDTS_STATUS_OFF: c_int = 8;
pub const CDTS_STATUS_BITS: c_int = 2;
pub const CDTS_STATUS_NORMAL: c_int = 0;
pub const CDTS_STATUS_SHORT: c_int = 1;
pub const CDTS_STATUS_OPEN: c_int = 2;
pub const CDTS_STATUS_INVALID: c_int = 3;
pub const MII_DBG_ADDR: c_uint = 0x1D;
pub const MII_DBG_DATA: c_uint = 0x1E;
// debug port
pub const MIIDBG_ANACTRL: c_uint = 0x00;
pub const ANACTRL_CLK125M_DELAY_EN: c_uint = 0x8000;
pub const ANACTRL_VCO_FAST: c_uint = 0x4000;
pub const ANACTRL_VCO_SLOW: c_uint = 0x2000;
pub const ANACTRL_AFE_MODE_EN: c_uint = 0x1000;
pub const ANACTRL_LCKDET_PHY: c_uint = 0x800;
pub const ANACTRL_LCKDET_EN: c_uint = 0x400;
pub const ANACTRL_OEN_125M: c_uint = 0x200;
pub const ANACTRL_HBIAS_EN: c_uint = 0x100;
pub const ANACTRL_HB_EN: c_uint = 0x80;
pub const ANACTRL_SEL_HSP: c_uint = 0x40;
pub const ANACTRL_CLASSA_EN: c_uint = 0x20;

pub const ANACTRL_MANUSWON_SWR_SHIFT: c_int = 2;
pub const ANACTRL_MANUSWON_SWR_2V: c_int = 0;
pub const ANACTRL_MANUSWON_SWR_1P9V: c_int = 1;
pub const ANACTRL_MANUSWON_SWR_1P8V: c_int = 2;
pub const ANACTRL_MANUSWON_SWR_1P7V: c_int = 3;
pub const ANACTRL_MANUSWON_BW3_4M: c_uint = 0x2;
pub const ANACTRL_RESTART_CAL: c_uint = 0x1;
pub const ANACTRL_DEF: c_uint = 0x02EF;
pub const MIIDBG_SYSMODCTRL: c_uint = 0x04;
pub const SYSMODCTRL_IECHOADJ_PFMH_PHY: c_uint = 0x8000;
pub const SYSMODCTRL_IECHOADJ_BIASGEN: c_uint = 0x4000;
pub const SYSMODCTRL_IECHOADJ_PFML_PHY: c_uint = 0x2000;

pub const SYSMODCTRL_IECHOADJ_PS_SHIFT: c_int = 10;
pub const SYSMODCTRL_IECHOADJ_PS_40: c_int = 3;
pub const SYSMODCTRL_IECHOADJ_PS_20: c_int = 2;
pub const SYSMODCTRL_IECHOADJ_PS_0: c_int = 1;
pub const SYSMODCTRL_IECHOADJ_10BT_100MV: c_uint = 0x40 /* 1:100mv, 0:200mv */;

pub const SYSMODCTRL_IECHOADJ_HLFAP_SHIFT: c_int = 4;
pub const SYSMODCTRL_IECHOADJ_VDFULBW: c_uint = 0x8;
pub const SYSMODCTRL_IECHOADJ_VDBIASHLF: c_uint = 0x4;
pub const SYSMODCTRL_IECHOADJ_VDAMPHLF: c_uint = 0x2;
pub const SYSMODCTRL_IECHOADJ_VDLANSW: c_uint = 0x1;
pub const SYSMODCTRL_IECHOADJ_DEF: c_uint = 0x88BB /* ???? */;
// for l1d & l2cb
pub const SYSMODCTRL_IECHOADJ_CUR_ADD: c_uint = 0x8000;

pub const SYSMODCTRL_IECHOADJ_CUR_SHIFT: c_int = 12;
pub const SYSMODCTRL_IECHOADJ_VOL_MASK: c_uint = 0xFU;
pub const SYSMODCTRL_IECHOADJ_VOL_SHIFT: c_int = 8;
pub const SYSMODCTRL_IECHOADJ_VOL_17ALL: c_int = 3;
pub const SYSMODCTRL_IECHOADJ_VOL_100M15: c_int = 1;
pub const SYSMODCTRL_IECHOADJ_VOL_10M17: c_int = 0;
pub const SYSMODCTRL_IECHOADJ_BIAS1_MASK: c_uint = 0xFU;
pub const SYSMODCTRL_IECHOADJ_BIAS1_SHIFT: c_int = 4;
pub const SYSMODCTRL_IECHOADJ_BIAS2_MASK: c_uint = 0xFU;
pub const SYSMODCTRL_IECHOADJ_BIAS2_SHIFT: c_int = 0;
pub const L1D_SYSMODCTRL_IECHOADJ_DEF: c_uint = 0x4FBB;
pub const MIIDBG_SRDSYSMOD: c_uint = 0x05;
pub const SRDSYSMOD_LCKDET_EN: c_uint = 0x2000;
pub const SRDSYSMOD_PLL_EN: c_uint = 0x800;
pub const SRDSYSMOD_SEL_HSP: c_uint = 0x400;
pub const SRDSYSMOD_HLFTXDR: c_uint = 0x200;
pub const SRDSYSMOD_TXCLK_DELAY_EN: c_uint = 0x100;
pub const SRDSYSMOD_TXELECIDLE: c_uint = 0x80;
pub const SRDSYSMOD_DEEMP_EN: c_uint = 0x40;
pub const SRDSYSMOD_MS_PAD: c_uint = 0x4;
pub const SRDSYSMOD_CDR_ADC_VLTG: c_uint = 0x2;
pub const SRDSYSMOD_CDR_DAC_1MA: c_uint = 0x1;
pub const SRDSYSMOD_DEF: c_uint = 0x2C46;
pub const MIIDBG_CFGLPSPD: c_uint = 0x0A;

pub const CFGLPSPD_RSTCNT_SHIFT: c_int = 14;
pub const CFGLPSPD_RSTCNT_CLK125SW: c_uint = 0x2000;
pub const MIIDBG_HIBNEG: c_uint = 0x0B;
pub const HIBNEG_PSHIB_EN: c_uint = 0x8000;
pub const HIBNEG_WAKE_BOTH: c_uint = 0x4000;
pub const HIBNEG_ONOFF_ANACHG_SUDEN: c_uint = 0x2000;
pub const HIBNEG_HIB_PULSE: c_uint = 0x1000;
pub const HIBNEG_GATE_25M_EN: c_uint = 0x800;
pub const HIBNEG_RST_80U: c_uint = 0x400;

pub const HIBNEG_RST_TIMER_SHIFT: c_int = 8;

pub const HIBNEG_GTX_CLK_DELAY_SHIFT: c_int = 5;
pub const HIBNEG_BYPSS_BRKTIMER: c_uint = 0x10;
pub const HIBNEG_DEF: c_uint = 0xBC40;
pub const MIIDBG_TST10BTCFG: c_uint = 0x12;

pub const TST10BTCFG_INTV_TIMER_SHIFT: c_int = 14;

pub const TST10BTCFG_TRIGER_TIMER_SHIFT: c_int = 12;
pub const TST10BTCFG_DIV_MAN_MLT3_EN: c_uint = 0x800;
pub const TST10BTCFG_OFF_DAC_IDLE: c_uint = 0x400;
pub const TST10BTCFG_LPBK_DEEP: c_uint = 0x4 /* 1:deep,0:shallow */;
pub const TST10BTCFG_DEF: c_uint = 0x4C04;
pub const MIIDBG_AZ_ANADECT: c_uint = 0x15;
pub const AZ_ANADECT_10BTRX_TH: c_uint = 0x8000;
pub const AZ_ANADECT_BOTH_01CHNL: c_uint = 0x4000;
pub const AZ_ANADECT_INTV_MASK: c_uint = 0x3FU;
pub const AZ_ANADECT_INTV_SHIFT: c_int = 8;
pub const AZ_ANADECT_THRESH_MASK: c_uint = 0xFU;
pub const AZ_ANADECT_THRESH_SHIFT: c_int = 4;
pub const AZ_ANADECT_CHNL_MASK: c_uint = 0xFU;
pub const AZ_ANADECT_CHNL_SHIFT: c_int = 0;
pub const AZ_ANADECT_DEF: c_uint = 0x3220;
pub const AZ_ANADECT_LONG: c_uint = 0xb210;
pub const MIIDBG_MSE16DB: c_uint = 0x18	/* l1d */;
pub const L1D_MSE16DB_UP: c_uint = 0x05EA;
pub const L1D_MSE16DB_DOWN: c_uint = 0x02EA;
pub const MIIDBG_LEGCYPS: c_uint = 0x29;
pub const LEGCYPS_EN: c_uint = 0x8000;

pub const LEGCYPS_DAC_AMP1000_SHIFT: c_int = 12;

pub const LEGCYPS_DAC_AMP100_SHIFT: c_int = 9;

pub const LEGCYPS_DAC_AMP10_SHIFT: c_int = 6;

pub const LEGCYPS_UNPLUG_TIMER_SHIFT: c_int = 3;
pub const LEGCYPS_UNPLUG_DECT_EN: c_uint = 0x4;
pub const LEGCYPS_ECNC_PS_EN: c_uint = 0x1;
pub const L1D_LEGCYPS_DEF: c_uint = 0x129D;
pub const L1C_LEGCYPS_DEF: c_uint = 0x36DD;
pub const MIIDBG_TST100BTCFG: c_uint = 0x36;
pub const TST100BTCFG_NORMAL_BW_EN: c_uint = 0x8000;
pub const TST100BTCFG_BADLNK_BYPASS: c_uint = 0x4000;
pub const TST100BTCFG_SHORTCABL_TH_MASK: c_uint = 0x3FU;
pub const TST100BTCFG_SHORTCABL_TH_SHIFT: c_int = 8;
pub const TST100BTCFG_LITCH_EN: c_uint = 0x80;
pub const TST100BTCFG_VLT_SW: c_uint = 0x40;
pub const TST100BTCFG_LONGCABL_TH_MASK: c_uint = 0x3FU;
pub const TST100BTCFG_LONGCABL_TH_SHIFT: c_int = 0;
pub const TST100BTCFG_DEF: c_uint = 0xE12C;
pub const MIIDBG_VOLT_CTRL: c_uint = 0x3B	/* only for l2cb 1 & 2 */;
pub const VOLT_CTRL_CABLE1TH_MASK: c_uint = 0x1FFU;
pub const VOLT_CTRL_CABLE1TH_SHIFT: c_int = 7;

pub const VOLT_CTRL_AMPCTRL_SHIFT: c_int = 5;
pub const VOLT_CTRL_SW_BYPASS: c_uint = 0x10;
pub const VOLT_CTRL_SWLOWEST: c_uint = 0x8;

pub const VOLT_CTRL_DACAMP10_SHIFT: c_int = 0;
pub const MIIDBG_CABLE1TH_DET: c_uint = 0x3E;
pub const CABLE1TH_DET_EN: c_uint = 0x8000;
// dev 3
pub const MIIEXT_PCS: c_int = 3;
pub const MIIEXT_CLDCTRL3: c_uint = 0x8003;
pub const CLDCTRL3_BP_CABLE1TH_DET_GT: c_uint = 0x8000;
pub const CLDCTRL3_AZ_DISAMP: c_uint = 0x1000;
pub const L2CB_CLDCTRL3: c_uint = 0x4D19;
pub const L1D_CLDCTRL3: c_uint = 0xDD19;
pub const MIIEXT_CLDCTRL6: c_uint = 0x8006;
pub const CLDCTRL6_CAB_LEN_MASK: c_uint = 0x1FFU;
pub const CLDCTRL6_CAB_LEN_SHIFT: c_int = 0;
pub const CLDCTRL6_CAB_LEN_SHORT: c_uint = 0x50;
// dev 7
pub const MIIEXT_ANEG: c_int = 7;
pub const MIIEXT_LOCAL_EEEADV: c_uint = 0x3C;
pub const LOCAL_EEEADV_1000BT: c_uint = 0x4;
pub const LOCAL_EEEADV_100BT: c_uint = 0x2;
pub const MIIEXT_REMOTE_EEEADV: c_uint = 0x3D;
pub const REMOTE_EEEADV_1000BT: c_uint = 0x4;
pub const REMOTE_EEEADV_100BT: c_uint = 0x2;
pub const MIIEXT_EEE_ANEG: c_uint = 0x8000;
pub const EEE_ANEG_1000M: c_uint = 0x4;
pub const EEE_ANEG_100M: c_uint = 0x2;
