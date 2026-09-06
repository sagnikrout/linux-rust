//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/smsc/smsc911x.h
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
// Copyright (C) 2004-2008 SMSC
// Copyright (C) 2005-2008 ARM
//
// Chip ID
pub const LAN9115: c_uint = 0x01150000;
pub const LAN9116: c_uint = 0x01160000;
pub const LAN9117: c_uint = 0x01170000;
pub const LAN9118: c_uint = 0x01180000;
pub const LAN9215: c_uint = 0x115A0000;
pub const LAN9216: c_uint = 0x116A0000;
pub const LAN9217: c_uint = 0x117A0000;
pub const LAN9218: c_uint = 0x118A0000;
pub const LAN9210: c_uint = 0x92100000;
pub const LAN9211: c_uint = 0x92110000;
pub const LAN9220: c_uint = 0x92200000;
pub const LAN9221: c_uint = 0x92210000;
pub const LAN9250: c_uint = 0x92500000;
pub const LAN89218: c_uint = 0x218A0000;

pub const USE_DEBUG: c_int = 0;
// This is the maximum number of packets to be received every
// NAPI poll
pub const SMSC_NAPI_WEIGHT: c_int = 16;
// implements a PHY loopback test at initialisation time, to ensure a packet
// can be successfully looped back
// Macro flag: #define USE_PHY_WORK_AROUND

// SMSC911x registers and bitfields
pub const RX_DATA_FIFO: c_uint = 0x00;
pub const TX_DATA_FIFO: c_uint = 0x20;
pub const TX_CMD_A_ON_COMP_: c_uint = 0x80000000;
pub const TX_CMD_A_BUF_END_ALGN_: c_uint = 0x03000000;
pub const TX_CMD_A_4_BYTE_ALGN_: c_uint = 0x00000000;
pub const TX_CMD_A_16_BYTE_ALGN_: c_uint = 0x01000000;
pub const TX_CMD_A_32_BYTE_ALGN_: c_uint = 0x02000000;
pub const TX_CMD_A_DATA_OFFSET_: c_uint = 0x001F0000;
pub const TX_CMD_A_FIRST_SEG_: c_uint = 0x00002000;
pub const TX_CMD_A_LAST_SEG_: c_uint = 0x00001000;
pub const TX_CMD_A_BUF_SIZE_: c_uint = 0x000007FF;
pub const TX_CMD_B_PKT_TAG_: c_uint = 0xFFFF0000;
pub const TX_CMD_B_ADD_CRC_DISABLE_: c_uint = 0x00002000;
pub const TX_CMD_B_DISABLE_PADDING_: c_uint = 0x00001000;
pub const TX_CMD_B_PKT_BYTE_LENGTH_: c_uint = 0x000007FF;
pub const RX_STATUS_FIFO: c_uint = 0x40;
pub const RX_STS_ES_: c_uint = 0x00008000;
pub const RX_STS_LENGTH_ERR_: c_uint = 0x00001000;
pub const RX_STS_MCAST_: c_uint = 0x00000400;
pub const RX_STS_FRAME_TYPE_: c_uint = 0x00000020;
pub const RX_STS_CRC_ERR_: c_uint = 0x00000002;
pub const RX_STATUS_FIFO_PEEK: c_uint = 0x44;
pub const TX_STATUS_FIFO: c_uint = 0x48;
pub const TX_STS_ES_: c_uint = 0x00008000;
pub const TX_STS_LOST_CARRIER_: c_uint = 0x00000800;
pub const TX_STS_NO_CARRIER_: c_uint = 0x00000400;
pub const TX_STS_LATE_COL_: c_uint = 0x00000200;
pub const TX_STS_EXCESS_COL_: c_uint = 0x00000100;
pub const TX_STATUS_FIFO_PEEK: c_uint = 0x4C;
pub const ID_REV: c_uint = 0x50;
pub const ID_REV_CHIP_ID_: c_uint = 0xFFFF0000;
pub const ID_REV_REV_ID_: c_uint = 0x0000FFFF;
pub const INT_CFG: c_uint = 0x54;
pub const INT_CFG_INT_DEAS_: c_uint = 0xFF000000;
pub const INT_CFG_INT_DEAS_CLR_: c_uint = 0x00004000;
pub const INT_CFG_INT_DEAS_STS_: c_uint = 0x00002000;
pub const INT_CFG_IRQ_INT_: c_uint = 0x00001000;
pub const INT_CFG_IRQ_EN_: c_uint = 0x00000100;
pub const INT_CFG_IRQ_POL_: c_uint = 0x00000010;
pub const INT_CFG_IRQ_TYPE_: c_uint = 0x00000001;
pub const INT_STS: c_uint = 0x58;
pub const INT_STS_SW_INT_: c_uint = 0x80000000;
pub const INT_STS_TXSTOP_INT_: c_uint = 0x02000000;
pub const INT_STS_RXSTOP_INT_: c_uint = 0x01000000;
pub const INT_STS_RXDFH_INT_: c_uint = 0x00800000;
pub const INT_STS_RXDF_INT_: c_uint = 0x00400000;
pub const INT_STS_TX_IOC_: c_uint = 0x00200000;
pub const INT_STS_RXD_INT_: c_uint = 0x00100000;
pub const INT_STS_GPT_INT_: c_uint = 0x00080000;
pub const INT_STS_PHY_INT_: c_uint = 0x00040000;
pub const INT_STS_PME_INT_: c_uint = 0x00020000;
pub const INT_STS_TXSO_: c_uint = 0x00010000;
pub const INT_STS_RWT_: c_uint = 0x00008000;
pub const INT_STS_RXE_: c_uint = 0x00004000;
pub const INT_STS_TXE_: c_uint = 0x00002000;
pub const INT_STS_TDFU_: c_uint = 0x00000800;
pub const INT_STS_TDFO_: c_uint = 0x00000400;
pub const INT_STS_TDFA_: c_uint = 0x00000200;
pub const INT_STS_TSFF_: c_uint = 0x00000100;
pub const INT_STS_TSFL_: c_uint = 0x00000080;
pub const INT_STS_RXDF_: c_uint = 0x00000040;
pub const INT_STS_RDFL_: c_uint = 0x00000020;
pub const INT_STS_RSFF_: c_uint = 0x00000010;
pub const INT_STS_RSFL_: c_uint = 0x00000008;
pub const INT_STS_GPIO2_INT_: c_uint = 0x00000004;
pub const INT_STS_GPIO1_INT_: c_uint = 0x00000002;
pub const INT_STS_GPIO0_INT_: c_uint = 0x00000001;
pub const INT_EN: c_uint = 0x5C;
pub const INT_EN_SW_INT_EN_: c_uint = 0x80000000;
pub const INT_EN_TXSTOP_INT_EN_: c_uint = 0x02000000;
pub const INT_EN_RXSTOP_INT_EN_: c_uint = 0x01000000;
pub const INT_EN_RXDFH_INT_EN_: c_uint = 0x00800000;
pub const INT_EN_TIOC_INT_EN_: c_uint = 0x00200000;
pub const INT_EN_RXD_INT_EN_: c_uint = 0x00100000;
pub const INT_EN_GPT_INT_EN_: c_uint = 0x00080000;
pub const INT_EN_PHY_INT_EN_: c_uint = 0x00040000;
pub const INT_EN_PME_INT_EN_: c_uint = 0x00020000;
pub const INT_EN_TXSO_EN_: c_uint = 0x00010000;
pub const INT_EN_RWT_EN_: c_uint = 0x00008000;
pub const INT_EN_RXE_EN_: c_uint = 0x00004000;
pub const INT_EN_TXE_EN_: c_uint = 0x00002000;
pub const INT_EN_TDFU_EN_: c_uint = 0x00000800;
pub const INT_EN_TDFO_EN_: c_uint = 0x00000400;
pub const INT_EN_TDFA_EN_: c_uint = 0x00000200;
pub const INT_EN_TSFF_EN_: c_uint = 0x00000100;
pub const INT_EN_TSFL_EN_: c_uint = 0x00000080;
pub const INT_EN_RXDF_EN_: c_uint = 0x00000040;
pub const INT_EN_RDFL_EN_: c_uint = 0x00000020;
pub const INT_EN_RSFF_EN_: c_uint = 0x00000010;
pub const INT_EN_RSFL_EN_: c_uint = 0x00000008;
pub const INT_EN_GPIO2_INT_: c_uint = 0x00000004;
pub const INT_EN_GPIO1_INT_: c_uint = 0x00000002;
pub const INT_EN_GPIO0_INT_: c_uint = 0x00000001;
pub const BYTE_TEST: c_uint = 0x64;
pub const FIFO_INT: c_uint = 0x68;
pub const FIFO_INT_TX_AVAIL_LEVEL_: c_uint = 0xFF000000;
pub const FIFO_INT_TX_STS_LEVEL_: c_uint = 0x00FF0000;
pub const FIFO_INT_RX_AVAIL_LEVEL_: c_uint = 0x0000FF00;
pub const FIFO_INT_RX_STS_LEVEL_: c_uint = 0x000000FF;
pub const RX_CFG: c_uint = 0x6C;
pub const RX_CFG_RX_END_ALGN_: c_uint = 0xC0000000;
pub const RX_CFG_RX_END_ALGN4_: c_uint = 0x00000000;
pub const RX_CFG_RX_END_ALGN16_: c_uint = 0x40000000;
pub const RX_CFG_RX_END_ALGN32_: c_uint = 0x80000000;
pub const RX_CFG_RX_DMA_CNT_: c_uint = 0x0FFF0000;
pub const RX_CFG_RX_DUMP_: c_uint = 0x00008000;
pub const RX_CFG_RXDOFF_: c_uint = 0x00001F00;
pub const TX_CFG: c_uint = 0x70;
pub const TX_CFG_TXS_DUMP_: c_uint = 0x00008000;
pub const TX_CFG_TXD_DUMP_: c_uint = 0x00004000;
pub const TX_CFG_TXSAO_: c_uint = 0x00000004;
pub const TX_CFG_TX_ON_: c_uint = 0x00000002;
pub const TX_CFG_STOP_TX_: c_uint = 0x00000001;
pub const HW_CFG: c_uint = 0x74;
pub const HW_CFG_TTM_: c_uint = 0x00200000;
pub const HW_CFG_SF_: c_uint = 0x00100000;
pub const HW_CFG_TX_FIF_SZ_: c_uint = 0x000F0000;
pub const HW_CFG_TR_: c_uint = 0x00003000;
pub const HW_CFG_SRST_: c_uint = 0x00000001;
// only available on 115/117
pub const HW_CFG_PHY_CLK_SEL_: c_uint = 0x00000060;
pub const HW_CFG_PHY_CLK_SEL_INT_PHY_: c_uint = 0x00000000;
pub const HW_CFG_PHY_CLK_SEL_EXT_PHY_: c_uint = 0x00000020;
pub const HW_CFG_PHY_CLK_SEL_CLK_DIS_: c_uint = 0x00000040;
pub const HW_CFG_SMI_SEL_: c_uint = 0x00000010;
pub const HW_CFG_EXT_PHY_DET_: c_uint = 0x00000008;
pub const HW_CFG_EXT_PHY_EN_: c_uint = 0x00000004;
pub const HW_CFG_SRST_TO_: c_uint = 0x00000002;
// only available  on 116/118
pub const HW_CFG_32_16_BIT_MODE_: c_uint = 0x00000004;
pub const RX_DP_CTRL: c_uint = 0x78;
pub const RX_DP_CTRL_RX_FFWD_: c_uint = 0x80000000;
pub const RX_FIFO_INF: c_uint = 0x7C;
pub const RX_FIFO_INF_RXSUSED_: c_uint = 0x00FF0000;
pub const RX_FIFO_INF_RXDUSED_: c_uint = 0x0000FFFF;
pub const TX_FIFO_INF: c_uint = 0x80;
pub const TX_FIFO_INF_TSUSED_: c_uint = 0x00FF0000;
pub const TX_FIFO_INF_TDFREE_: c_uint = 0x0000FFFF;
pub const PMT_CTRL: c_uint = 0x84;
pub const PMT_CTRL_PM_MODE_: c_uint = 0x00003000;
pub const PMT_CTRL_PM_MODE_D0_: c_uint = 0x00000000;
pub const PMT_CTRL_PM_MODE_D1_: c_uint = 0x00001000;
pub const PMT_CTRL_PM_MODE_D2_: c_uint = 0x00002000;
pub const PMT_CTRL_PM_MODE_D3_: c_uint = 0x00003000;
pub const PMT_CTRL_PHY_RST_: c_uint = 0x00000400;
pub const PMT_CTRL_WOL_EN_: c_uint = 0x00000200;
pub const PMT_CTRL_ED_EN_: c_uint = 0x00000100;
pub const PMT_CTRL_PME_TYPE_: c_uint = 0x00000040;
pub const PMT_CTRL_WUPS_: c_uint = 0x00000030;
pub const PMT_CTRL_WUPS_NOWAKE_: c_uint = 0x00000000;
pub const PMT_CTRL_WUPS_ED_: c_uint = 0x00000010;
pub const PMT_CTRL_WUPS_WOL_: c_uint = 0x00000020;
pub const PMT_CTRL_WUPS_MULTI_: c_uint = 0x00000030;
pub const PMT_CTRL_PME_IND_: c_uint = 0x00000008;
pub const PMT_CTRL_PME_POL_: c_uint = 0x00000004;
pub const PMT_CTRL_PME_EN_: c_uint = 0x00000002;
pub const PMT_CTRL_READY_: c_uint = 0x00000001;
pub const GPIO_CFG: c_uint = 0x88;
pub const GPIO_CFG_LED3_EN_: c_uint = 0x40000000;
pub const GPIO_CFG_LED2_EN_: c_uint = 0x20000000;
pub const GPIO_CFG_LED1_EN_: c_uint = 0x10000000;
pub const GPIO_CFG_GPIO2_INT_POL_: c_uint = 0x04000000;
pub const GPIO_CFG_GPIO1_INT_POL_: c_uint = 0x02000000;
pub const GPIO_CFG_GPIO0_INT_POL_: c_uint = 0x01000000;
pub const GPIO_CFG_EEPR_EN_: c_uint = 0x00700000;
pub const GPIO_CFG_GPIOBUF2_: c_uint = 0x00040000;
pub const GPIO_CFG_GPIOBUF1_: c_uint = 0x00020000;
pub const GPIO_CFG_GPIOBUF0_: c_uint = 0x00010000;
pub const GPIO_CFG_GPIODIR2_: c_uint = 0x00000400;
pub const GPIO_CFG_GPIODIR1_: c_uint = 0x00000200;
pub const GPIO_CFG_GPIODIR0_: c_uint = 0x00000100;
pub const GPIO_CFG_GPIOD4_: c_uint = 0x00000020;
pub const GPIO_CFG_GPIOD3_: c_uint = 0x00000010;
pub const GPIO_CFG_GPIOD2_: c_uint = 0x00000004;
pub const GPIO_CFG_GPIOD1_: c_uint = 0x00000002;
pub const GPIO_CFG_GPIOD0_: c_uint = 0x00000001;
pub const GPT_CFG: c_uint = 0x8C;
pub const GPT_CFG_TIMER_EN_: c_uint = 0x20000000;
pub const GPT_CFG_GPT_LOAD_: c_uint = 0x0000FFFF;
pub const GPT_CNT: c_uint = 0x90;
pub const GPT_CNT_GPT_CNT_: c_uint = 0x0000FFFF;
pub const WORD_SWAP: c_uint = 0x98;
pub const FREE_RUN: c_uint = 0x9C;
pub const RX_DROP: c_uint = 0xA0;
pub const MAC_CSR_CMD: c_uint = 0xA4;
pub const MAC_CSR_CMD_CSR_BUSY_: c_uint = 0x80000000;
pub const MAC_CSR_CMD_R_NOT_W_: c_uint = 0x40000000;
pub const MAC_CSR_CMD_CSR_ADDR_: c_uint = 0x000000FF;
pub const MAC_CSR_DATA: c_uint = 0xA8;
pub const AFC_CFG: c_uint = 0xAC;
pub const AFC_CFG_AFC_HI_: c_uint = 0x00FF0000;
pub const AFC_CFG_AFC_LO_: c_uint = 0x0000FF00;
pub const AFC_CFG_BACK_DUR_: c_uint = 0x000000F0;
pub const AFC_CFG_FCMULT_: c_uint = 0x00000008;
pub const AFC_CFG_FCBRD_: c_uint = 0x00000004;
pub const AFC_CFG_FCADD_: c_uint = 0x00000002;
pub const AFC_CFG_FCANY_: c_uint = 0x00000001;
pub const E2P_CMD: c_uint = 0xB0;
pub const E2P_CMD_EPC_BUSY_: c_uint = 0x80000000;
pub const E2P_CMD_EPC_CMD_: c_uint = 0x70000000;
pub const E2P_CMD_EPC_CMD_READ_: c_uint = 0x00000000;
pub const E2P_CMD_EPC_CMD_EWDS_: c_uint = 0x10000000;
pub const E2P_CMD_EPC_CMD_EWEN_: c_uint = 0x20000000;
pub const E2P_CMD_EPC_CMD_WRITE_: c_uint = 0x30000000;
pub const E2P_CMD_EPC_CMD_WRAL_: c_uint = 0x40000000;
pub const E2P_CMD_EPC_CMD_ERASE_: c_uint = 0x50000000;
pub const E2P_CMD_EPC_CMD_ERAL_: c_uint = 0x60000000;
pub const E2P_CMD_EPC_CMD_RELOAD_: c_uint = 0x70000000;
pub const E2P_CMD_EPC_TIMEOUT_: c_uint = 0x00000200;
pub const E2P_CMD_MAC_ADDR_LOADED_: c_uint = 0x00000100;
pub const E2P_CMD_EPC_ADDR_: c_uint = 0x000000FF;
pub const E2P_DATA: c_uint = 0xB4;
pub const E2P_DATA_EEPROM_DATA_: c_uint = 0x000000FF;
pub const LAN_REGISTER_EXTENT: c_uint = 0x00000100;
pub const RESET_CTL: c_uint = 0x1F8;
pub const RESET_CTL_DIGITAL_RST_: c_uint = 0x00000001;
//
// MAC Control and Status Register (Indirect Address)
// Offset (through the MAC_CSR CMD and DATA port)
//
pub const MAC_CR: c_uint = 0x01;
pub const MAC_CR_RXALL_: c_uint = 0x80000000;
pub const MAC_CR_HBDIS_: c_uint = 0x10000000;
pub const MAC_CR_RCVOWN_: c_uint = 0x00800000;
pub const MAC_CR_LOOPBK_: c_uint = 0x00200000;
pub const MAC_CR_FDPX_: c_uint = 0x00100000;
pub const MAC_CR_MCPAS_: c_uint = 0x00080000;
pub const MAC_CR_PRMS_: c_uint = 0x00040000;
pub const MAC_CR_INVFILT_: c_uint = 0x00020000;
pub const MAC_CR_PASSBAD_: c_uint = 0x00010000;
pub const MAC_CR_HFILT_: c_uint = 0x00008000;
pub const MAC_CR_HPFILT_: c_uint = 0x00002000;
pub const MAC_CR_LCOLL_: c_uint = 0x00001000;
pub const MAC_CR_BCAST_: c_uint = 0x00000800;
pub const MAC_CR_DISRTY_: c_uint = 0x00000400;
pub const MAC_CR_PADSTR_: c_uint = 0x00000100;
pub const MAC_CR_BOLMT_MASK_: c_uint = 0x000000C0;
pub const MAC_CR_DFCHK_: c_uint = 0x00000020;
pub const MAC_CR_TXEN_: c_uint = 0x00000008;
pub const MAC_CR_RXEN_: c_uint = 0x00000004;
pub const ADDRH: c_uint = 0x02;
pub const ADDRL: c_uint = 0x03;
pub const HASHH: c_uint = 0x04;
pub const HASHL: c_uint = 0x05;
pub const MII_ACC: c_uint = 0x06;
pub const MII_ACC_PHY_ADDR_: c_uint = 0x0000F800;
pub const MII_ACC_MIIRINDA_: c_uint = 0x000007C0;
pub const MII_ACC_MII_WRITE_: c_uint = 0x00000002;
pub const MII_ACC_MII_BUSY_: c_uint = 0x00000001;
pub const MII_DATA: c_uint = 0x07;
pub const FLOW: c_uint = 0x08;
pub const FLOW_FCPT_: c_uint = 0xFFFF0000;
pub const FLOW_FCPASS_: c_uint = 0x00000004;
pub const FLOW_FCEN_: c_uint = 0x00000002;
pub const FLOW_FCBSY_: c_uint = 0x00000001;
pub const VLAN1: c_uint = 0x09;
pub const VLAN2: c_uint = 0x0A;
pub const WUFF: c_uint = 0x0B;
pub const WUCSR: c_uint = 0x0C;
pub const WUCSR_GUE_: c_uint = 0x00000200;
pub const WUCSR_WUFR_: c_uint = 0x00000040;
pub const WUCSR_MPR_: c_uint = 0x00000020;
pub const WUCSR_WAKE_EN_: c_uint = 0x00000004;
pub const WUCSR_MPEN_: c_uint = 0x00000002;
//
// Phy definitions (vendor-specific)
//
pub const LAN9118_PHY_ID: c_uint = 0x00C0001C;
pub const MII_INTSTS: c_uint = 0x1D;
pub const MII_INTMSK: c_uint = 0x1E;

//
// Provide hooks to let the arch add to the initialisation procedure
// and to override the source of the MAC address.
//

