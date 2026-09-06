//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/tehuti/tn40_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) Tehuti Networks Ltd.
// Register region size
pub const TN40_REGS_SIZE: c_uint = 0x10000;
// Registers from 0x0000-0x00fc were remapped to 0x4000-0x40fc
pub const TN40_REG_TXD_CFG1_0: c_uint = 0x4000;
pub const TN40_REG_TXD_CFG1_1: c_uint = 0x4004;
pub const TN40_REG_TXD_CFG1_2: c_uint = 0x4008;
pub const TN40_REG_TXD_CFG1_3: c_uint = 0x400C;
pub const TN40_REG_RXF_CFG1_0: c_uint = 0x4010;
pub const TN40_REG_RXF_CFG1_1: c_uint = 0x4014;
pub const TN40_REG_RXF_CFG1_2: c_uint = 0x4018;
pub const TN40_REG_RXF_CFG1_3: c_uint = 0x401C;
pub const TN40_REG_RXD_CFG1_0: c_uint = 0x4020;
pub const TN40_REG_RXD_CFG1_1: c_uint = 0x4024;
pub const TN40_REG_RXD_CFG1_2: c_uint = 0x4028;
pub const TN40_REG_RXD_CFG1_3: c_uint = 0x402C;
pub const TN40_REG_TXF_CFG1_0: c_uint = 0x4030;
pub const TN40_REG_TXF_CFG1_1: c_uint = 0x4034;
pub const TN40_REG_TXF_CFG1_2: c_uint = 0x4038;
pub const TN40_REG_TXF_CFG1_3: c_uint = 0x403C;
pub const TN40_REG_TXD_CFG0_0: c_uint = 0x4040;
pub const TN40_REG_TXD_CFG0_1: c_uint = 0x4044;
pub const TN40_REG_TXD_CFG0_2: c_uint = 0x4048;
pub const TN40_REG_TXD_CFG0_3: c_uint = 0x404C;
pub const TN40_REG_RXF_CFG0_0: c_uint = 0x4050;
pub const TN40_REG_RXF_CFG0_1: c_uint = 0x4054;
pub const TN40_REG_RXF_CFG0_2: c_uint = 0x4058;
pub const TN40_REG_RXF_CFG0_3: c_uint = 0x405C;
pub const TN40_REG_RXD_CFG0_0: c_uint = 0x4060;
pub const TN40_REG_RXD_CFG0_1: c_uint = 0x4064;
pub const TN40_REG_RXD_CFG0_2: c_uint = 0x4068;
pub const TN40_REG_RXD_CFG0_3: c_uint = 0x406C;
pub const TN40_REG_TXF_CFG0_0: c_uint = 0x4070;
pub const TN40_REG_TXF_CFG0_1: c_uint = 0x4074;
pub const TN40_REG_TXF_CFG0_2: c_uint = 0x4078;
pub const TN40_REG_TXF_CFG0_3: c_uint = 0x407C;
pub const TN40_REG_TXD_WPTR_0: c_uint = 0x4080;
pub const TN40_REG_TXD_WPTR_1: c_uint = 0x4084;
pub const TN40_REG_TXD_WPTR_2: c_uint = 0x4088;
pub const TN40_REG_TXD_WPTR_3: c_uint = 0x408C;
pub const TN40_REG_RXF_WPTR_0: c_uint = 0x4090;
pub const TN40_REG_RXF_WPTR_1: c_uint = 0x4094;
pub const TN40_REG_RXF_WPTR_2: c_uint = 0x4098;
pub const TN40_REG_RXF_WPTR_3: c_uint = 0x409C;
pub const TN40_REG_RXD_WPTR_0: c_uint = 0x40A0;
pub const TN40_REG_RXD_WPTR_1: c_uint = 0x40A4;
pub const TN40_REG_RXD_WPTR_2: c_uint = 0x40A8;
pub const TN40_REG_RXD_WPTR_3: c_uint = 0x40AC;
pub const TN40_REG_TXF_WPTR_0: c_uint = 0x40B0;
pub const TN40_REG_TXF_WPTR_1: c_uint = 0x40B4;
pub const TN40_REG_TXF_WPTR_2: c_uint = 0x40B8;
pub const TN40_REG_TXF_WPTR_3: c_uint = 0x40BC;
pub const TN40_REG_TXD_RPTR_0: c_uint = 0x40C0;
pub const TN40_REG_TXD_RPTR_1: c_uint = 0x40C4;
pub const TN40_REG_TXD_RPTR_2: c_uint = 0x40C8;
pub const TN40_REG_TXD_RPTR_3: c_uint = 0x40CC;
pub const TN40_REG_RXF_RPTR_0: c_uint = 0x40D0;
pub const TN40_REG_RXF_RPTR_1: c_uint = 0x40D4;
pub const TN40_REG_RXF_RPTR_2: c_uint = 0x40D8;
pub const TN40_REG_RXF_RPTR_3: c_uint = 0x40DC;
pub const TN40_REG_RXD_RPTR_0: c_uint = 0x40E0;
pub const TN40_REG_RXD_RPTR_1: c_uint = 0x40E4;
pub const TN40_REG_RXD_RPTR_2: c_uint = 0x40E8;
pub const TN40_REG_RXD_RPTR_3: c_uint = 0x40EC;
pub const TN40_REG_TXF_RPTR_0: c_uint = 0x40F0;
pub const TN40_REG_TXF_RPTR_1: c_uint = 0x40F4;
pub const TN40_REG_TXF_RPTR_2: c_uint = 0x40F8;
pub const TN40_REG_TXF_RPTR_3: c_uint = 0x40FC;
// Hardware versioning
pub const TN40_FPGA_VER: c_uint = 0x5030;
// Registers from 0x0100-0x0150 were remapped to 0x5100-0x5150

pub const TN40_REG_ISR0: c_uint = 0x5100;

pub const TN40_REG_IMR0: c_uint = 0x5110;
pub const TN40_REG_RDINTCM0: c_uint = 0x5120;
pub const TN40_REG_RDINTCM2: c_uint = 0x5128;
pub const TN40_REG_TDINTCM0: c_uint = 0x5130;
pub const TN40_REG_ISR_MSK0: c_uint = 0x5140;
pub const TN40_REG_INIT_SEMAPHORE: c_uint = 0x5170;
pub const TN40_REG_INIT_STATUS: c_uint = 0x5180;
pub const TN40_REG_MAC_LNK_STAT: c_uint = 0x0200;
pub const TN40_MAC_LINK_STAT: c_uint = 0x0004 /* Link state */;
pub const TN40_REG_BLNK_LED: c_uint = 0x0210;
pub const TN40_REG_GMAC_RXF_A: c_uint = 0x1240;
pub const TN40_REG_UNC_MAC0_A: c_uint = 0x1250;
pub const TN40_REG_UNC_MAC1_A: c_uint = 0x1260;
pub const TN40_REG_UNC_MAC2_A: c_uint = 0x1270;
pub const TN40_REG_VLAN_0: c_uint = 0x1800;
pub const TN40_REG_MAX_FRAME_A: c_uint = 0x12C0;
pub const TN40_REG_RX_MAC_MCST0: c_uint = 0x1A80;
pub const TN40_REG_RX_MAC_MCST1: c_uint = 0x1A84;
pub const TN40_MAC_MCST_NUM: c_int = 15;
pub const TN40_REG_RX_MCST_HASH0: c_uint = 0x1A00;
pub const TN40_MAC_MCST_HASH_NUM: c_int = 8;
pub const TN40_REG_VPC: c_uint = 0x2300;
pub const TN40_REG_VIC: c_uint = 0x2320;
pub const TN40_REG_VGLB: c_uint = 0x2340;
pub const TN40_REG_CLKPLL: c_uint = 0x5000;
// MDIO interface
pub const TN40_REG_MDIO_CMD_STAT: c_uint = 0x6030;
pub const TN40_REG_MDIO_CMD: c_uint = 0x6034;
pub const TN40_REG_MDIO_DATA: c_uint = 0x6038;
pub const TN40_REG_MDIO_ADDR: c_uint = 0x603C;

pub const TN40_REG_REVISION: c_uint = 0x6000;
pub const TN40_REG_SCRATCH: c_uint = 0x6004;
pub const TN40_REG_CTRLST: c_uint = 0x6008;
pub const TN40_REG_MAC_ADDR_0: c_uint = 0x600C;
pub const TN40_REG_MAC_ADDR_1: c_uint = 0x6010;
pub const TN40_REG_FRM_LENGTH: c_uint = 0x6014;
pub const TN40_REG_PAUSE_QUANT: c_uint = 0x6054;
pub const TN40_REG_RX_FIFO_SECTION: c_uint = 0x601C;
pub const TN40_REG_TX_FIFO_SECTION: c_uint = 0x6020;
pub const TN40_REG_RX_FULLNESS: c_uint = 0x6024;
pub const TN40_REG_TX_FULLNESS: c_uint = 0x6028;
pub const TN40_REG_HASHTABLE: c_uint = 0x602C;
pub const TN40_REG_RST_PORT: c_uint = 0x7000;
pub const TN40_REG_DIS_PORT: c_uint = 0x7010;
pub const TN40_REG_RST_QU: c_uint = 0x7020;
pub const TN40_REG_DIS_QU: c_uint = 0x7030;
pub const TN40_REG_CTRLST_TX_ENA: c_uint = 0x0001;
pub const TN40_REG_CTRLST_RX_ENA: c_uint = 0x0002;
pub const TN40_REG_CTRLST_PRM_ENA: c_uint = 0x0010;
pub const TN40_REG_CTRLST_PAD_ENA: c_uint = 0x0020;

// TXD TXF RXF RXD  CONFIG 0x0000 --- 0x007c
pub const TN40_TX_RX_CFG1_BASE: c_uint = 0xffffffff /*0-31 */;
pub const TN40_TX_RX_CFG0_BASE: c_uint = 0xfffff000 /*31:12 */;
pub const TN40_TX_RX_CFG0_RSVD: c_uint = 0x00000ffc /*11:2 */;
pub const TN40_TX_RX_CFG0_SIZE: c_uint = 0x00000003 /*1:0 */;
// TXD TXF RXF RXD  WRITE 0x0080 --- 0x00BC
pub const TN40_TXF_WPTR_WR_PTR: c_uint = 0x00007ff8 /*14:3 */;
// TXD TXF RXF RXD  READ  0x00CO --- 0x00FC
pub const TN40_TXF_RPTR_RD_PTR: c_uint = 0x00007ff8 /*14:3 */;
// The last 4 bits are dropped size is rounded to 16
pub const TN40_TXF_WPTR_MASK: c_uint = 0x7ff0;
// regISR 0x0100
// regIMR 0x0110
pub const TN40_IMR_INPROG: c_uint = 0x80000000 /*31 */;
pub const TN40_IR_LNKCHG1: c_uint = 0x10000000 /*28 */;
pub const TN40_IR_LNKCHG0: c_uint = 0x08000000 /*27 */;
pub const TN40_IR_GPIO: c_uint = 0x04000000 /*26 */;
pub const TN40_IR_RFRSH: c_uint = 0x02000000 /*25 */;
pub const TN40_IR_RSVD: c_uint = 0x01000000 /*24 */;
pub const TN40_IR_SWI: c_uint = 0x00800000 /*23 */;
pub const TN40_IR_RX_FREE_3: c_uint = 0x00400000 /*22 */;
pub const TN40_IR_RX_FREE_2: c_uint = 0x00200000 /*21 */;
pub const TN40_IR_RX_FREE_1: c_uint = 0x00100000 /*20 */;
pub const TN40_IR_RX_FREE_0: c_uint = 0x00080000 /*19 */;
pub const TN40_IR_TX_FREE_3: c_uint = 0x00040000 /*18 */;
pub const TN40_IR_TX_FREE_2: c_uint = 0x00020000 /*17 */;
pub const TN40_IR_TX_FREE_1: c_uint = 0x00010000 /*16 */;
pub const TN40_IR_TX_FREE_0: c_uint = 0x00008000 /*15 */;
pub const TN40_IR_RX_DESC_3: c_uint = 0x00004000 /*14 */;
pub const TN40_IR_RX_DESC_2: c_uint = 0x00002000 /*13 */;
pub const TN40_IR_RX_DESC_1: c_uint = 0x00001000 /*12 */;
pub const TN40_IR_RX_DESC_0: c_uint = 0x00000800 /*11 */;
pub const TN40_IR_PSE: c_uint = 0x00000400 /*10 */;
pub const TN40_IR_TMR3: c_uint = 0x00000200 /* 9 */;
pub const TN40_IR_TMR2: c_uint = 0x00000100 /* 8 */;
pub const TN40_IR_TMR1: c_uint = 0x00000080 /* 7 */;
pub const TN40_IR_TMR0: c_uint = 0x00000040 /* 6 */;
pub const TN40_IR_VNT: c_uint = 0x00000020 /* 5 */;
pub const TN40_IR_RxFL: c_uint = 0x00000010 /* 4 */;
pub const TN40_IR_SDPERR: c_uint = 0x00000008 /* 3 */;
pub const TN40_IR_TR: c_uint = 0x00000004 /* 2 */;
pub const TN40_IR_PCIE_LINK: c_uint = 0x00000002 /* 1 */;
pub const TN40_IR_PCIE_TOUT: c_uint = 0x00000001 /* 0 */;

pub const TN40_GMAC_RX_FILTER_OSEN: c_uint = 0x1000 /* shared OS enable */;
pub const TN40_GMAC_RX_FILTER_TXFC: c_uint = 0x0400 /* Tx flow control */;
pub const TN40_GMAC_RX_FILTER_RSV0: c_uint = 0x0200 /* reserved */;
pub const TN40_GMAC_RX_FILTER_FDA: c_uint = 0x0100 /* filter out direct address */;
pub const TN40_GMAC_RX_FILTER_AOF: c_uint = 0x0080 /* accept over run */;
pub const TN40_GMAC_RX_FILTER_ACF: c_uint = 0x0040 /* accept control frames */;
pub const TN40_GMAC_RX_FILTER_ARUNT: c_uint = 0x0020 /* accept under run */;
pub const TN40_GMAC_RX_FILTER_ACRC: c_uint = 0x0010 /* accept crc error */;
pub const TN40_GMAC_RX_FILTER_AM: c_uint = 0x0008 /* accept multicast */;
pub const TN40_GMAC_RX_FILTER_AB: c_uint = 0x0004 /* accept broadcast */;
pub const TN40_GMAC_RX_FILTER_PRM: c_uint = 0x0001 /* [0:1] promiscuous mode */;
pub const TN40_MAX_FRAME_AB_VAL: c_uint = 0x3fff /* 13:0 */;
pub const TN40_CLKPLL_PLLLKD: c_uint = 0x0200 /* 9 */;
pub const TN40_CLKPLL_RSTEND: c_uint = 0x0100 /* 8 */;
pub const TN40_CLKPLL_SFTRST: c_uint = 0x0001 /* 0 */;

