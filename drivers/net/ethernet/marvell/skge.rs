//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/skge.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Definitions for the new Marvell Yukon / SysKonnect driver.
//

// PCI config registers
pub const PCI_DEV_REG1: c_uint = 0x40;
pub const PCI_PHY_COMA: c_uint = 0x8000000;
pub const PCI_VIO: c_uint = 0x2000000;
pub const PCI_DEV_REG2: c_uint = 0x44;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csr_regs {
    B0_RAP	= 0x0000,
    B0_CTST	= 0x0004,
    B0_LED	= 0x0006,
    B0_POWER_CTRL	= 0x0007,
    B0_ISRC	= 0x0008,
    B0_IMSK	= 0x000c,
    B0_HWE_ISRC	= 0x0010,
    B0_HWE_IMSK	= 0x0014,
    B0_SP_ISRC	= 0x0018,
    B0_XM1_IMSK	= 0x0020,
    B0_XM1_ISRC	= 0x0028,
    B0_XM1_PHY_ADDR	= 0x0030,
    B0_XM1_PHY_DATA	= 0x0034,
    B0_XM2_IMSK	= 0x0040,
    B0_XM2_ISRC	= 0x0048,
    B0_XM2_PHY_ADDR	= 0x0050,
    B0_XM2_PHY_DATA	= 0x0054,
    B0_R1_CSR	= 0x0060,
    B0_R2_CSR	= 0x0064,
    B0_XS1_CSR	= 0x0068,
    B0_XA1_CSR	= 0x006c,
    B0_XS2_CSR	= 0x0070,
    B0_XA2_CSR	= 0x0074,

    B2_MAC_1	= 0x0100,
    B2_MAC_2	= 0x0108,
    B2_MAC_3	= 0x0110,
    B2_CONN_TYP	= 0x0118,
    B2_PMD_TYP	= 0x0119,
    B2_MAC_CFG	= 0x011a,
    B2_CHIP_ID	= 0x011b,
    B2_E_0		= 0x011c,
    B2_E_1		= 0x011d,
    B2_E_2		= 0x011e,
    B2_E_3		= 0x011f,
    B2_FAR		= 0x0120,
    B2_FDP		= 0x0124,
    B2_LD_CTRL	= 0x0128,
    B2_LD_TEST	= 0x0129,
    B2_TI_INI	= 0x0130,
    B2_TI_VAL	= 0x0134,
    B2_TI_CTRL	= 0x0138,
    B2_TI_TEST	= 0x0139,
    B2_IRQM_INI	= 0x0140,
    B2_IRQM_VAL	= 0x0144,
    B2_IRQM_CTRL	= 0x0148,
    B2_IRQM_TEST	= 0x0149,
    B2_IRQM_MSK	= 0x014c,
    B2_IRQM_HWE_MSK	= 0x0150,
    B2_TST_CTRL1	= 0x0158,
    B2_TST_CTRL2	= 0x0159,
    B2_GP_IO	= 0x015c,
    B2_I2C_CTRL	= 0x0160,
    B2_I2C_DATA	= 0x0164,
    B2_I2C_IRQ	= 0x0168,
    B2_I2C_SW	= 0x016c,
    B2_BSC_INI	= 0x0170,
    B2_BSC_VAL	= 0x0174,
    B2_BSC_CTRL	= 0x0178,
    B2_BSC_STAT	= 0x0179,
    B2_BSC_TST	= 0x017a,

    B3_RAM_ADDR	= 0x0180,
    B3_RAM_DATA_LO	= 0x0184,
    B3_RAM_DATA_HI	= 0x0188,
    B3_RI_WTO_R1	= 0x0190,
    B3_RI_WTO_XA1	= 0x0191,
    B3_RI_WTO_XS1	= 0x0192,
    B3_RI_RTO_R1	= 0x0193,
    B3_RI_RTO_XA1	= 0x0194,
    B3_RI_RTO_XS1	= 0x0195,
    B3_RI_WTO_R2	= 0x0196,
    B3_RI_WTO_XA2	= 0x0197,
    B3_RI_WTO_XS2	= 0x0198,
    B3_RI_RTO_R2	= 0x0199,
    B3_RI_RTO_XA2	= 0x019a,
    B3_RI_RTO_XS2	= 0x019b,
    B3_RI_TO_VAL	= 0x019c,
    B3_RI_CTRL	= 0x01a0,
    B3_RI_TEST	= 0x01a2,
    B3_MA_TOINI_RX1	= 0x01b0,
    B3_MA_TOINI_RX2	= 0x01b1,
    B3_MA_TOINI_TX1	= 0x01b2,
    B3_MA_TOINI_TX2	= 0x01b3,
    B3_MA_TOVAL_RX1	= 0x01b4,
    B3_MA_TOVAL_RX2	= 0x01b5,
    B3_MA_TOVAL_TX1	= 0x01b6,
    B3_MA_TOVAL_TX2	= 0x01b7,
    B3_MA_TO_CTRL	= 0x01b8,
    B3_MA_TO_TEST	= 0x01ba,
    B3_MA_RCINI_RX1	= 0x01c0,
    B3_MA_RCINI_RX2	= 0x01c1,
    B3_MA_RCINI_TX1	= 0x01c2,
    B3_MA_RCINI_TX2	= 0x01c3,
    B3_MA_RCVAL_RX1	= 0x01c4,
    B3_MA_RCVAL_RX2	= 0x01c5,
    B3_MA_RCVAL_TX1	= 0x01c6,
    B3_MA_RCVAL_TX2	= 0x01c7,
    B3_MA_RC_CTRL	= 0x01c8,
    B3_MA_RC_TEST	= 0x01ca,
    B3_PA_TOINI_RX1	= 0x01d0,
    B3_PA_TOINI_RX2	= 0x01d4,
    B3_PA_TOINI_TX1	= 0x01d8,
    B3_PA_TOINI_TX2	= 0x01dc,
    B3_PA_TOVAL_RX1	= 0x01e0,
    B3_PA_TOVAL_RX2	= 0x01e4,
    B3_PA_TOVAL_TX1	= 0x01e8,
    B3_PA_TOVAL_TX2	= 0x01ec,
    B3_PA_CTRL	= 0x01f0,
    B3_PA_TEST	= 0x01f2,
}

// B0_CTST			16 bit	Control/Status register
// B0_LED			 8 Bit	LED register
// Bit  7.. 2:	reserved
// B0_POWER_CTRL	 8 Bit	Power Control reg (YUKON only)
// B2_IRQM_MSK 	32 bit	IRQ Moderation Mask
// Bit 30:	reserved
// IRQ from PHY (YUKON only)
// Receive Queue 1
// Receive Queue 2
// Synchronous Transmit Queue 1
// Asynchronous Transmit Queue 1
// Synchronous Transmit Queue 2
// Asynchronous Transmit Queue 2
// B2_IRQM_HWE_MSK	32 bit	IRQ Moderation HW Error Mask
// B2_TST_CTRL1	 8 bit	Test Control Register 1
// B2_MAC_CFG		 8 bit	MAC Configuration / Chip Revision
// Bit 3.. 2:	reserved
// B2_CHIP_ID		 8 bit 	Chip Identification Number
// B2_TI_CTRL		 8 bit	Timer control
// B2_IRQM_CTRL	 8 bit	IRQ Moderation Timer Control
// B2_TI_TEST		 8 Bit	Timer Test
// B2_IRQM_TEST	 8 bit	IRQ Moderation Timer Test
// B28_DPT_TST		 8 bit	Descriptor Poll Timer Test Reg
// B2_GP_IO		32 bit	General Purpose I/O Register
// Descriptor Bit Definition
// TxCtrl		Transmit Buffer Control Field
// RxCtrl		Receive  Buffer Control Field
// TxCtrl specific bits
// RxCtrl specific bits
// Bit 23..16:	BMU Check Opcodes
// B2_BSC_CTRL		 8 bit	Blink Source Counter Control
// B2_BSC_STAT		 8 bit	Blink Source Counter Status
// B2_BSC_TST		16 bit	Blink Source Counter Test Reg
// B3_RAM_ADDR		32 bit	RAM Address, to read or write
// Bit 31..19:	reserved
pub const RAM_ADR_RAN: c_uint = 0x0007ffffL	/* Bit 18.. 0:	RAM Address Range */;
// RAM Interface Registers
// B3_RI_CTRL		16 bit	RAM Iface Control Register
// MAC Arbiter Registers
// B3_MA_TO_CTRL	16 bit	MAC Arbiter Timeout Ctrl Reg
// Timeout values

pub const SK_PKT_TO_53: c_uint = 0x2000		/* Packet arbiter timeout */;
pub const SK_PKT_TO_MAX: c_uint = 0xffff		/* Maximum value */;

// Packet Arbiter Registers
// B3_PA_CTRL		16 bit	Packet Arbiter Ctrl Register

// Transmit Arbiter Registers MAC 1 and 2, use SK_REG() to access
// TXA_ITI_INI		32 bit	Tx Arb Interval Timer Init Val
// TXA_ITI_VAL		32 bit	Tx Arb Interval Timer Value
// TXA_LIM_INI		32 bit	Tx Arb Limit Counter Init Val
// TXA_LIM_VAL		32 bit	Tx Arb Limit Counter Value
pub const TXA_MAX_VAL: c_uint = 0x00ffffffUL	/* Bit 23.. 0:	Max TXA Timer/Cnt Val */;
// TXA_CTRL		 8 bit	Tx Arbiter Control Register
//
// Bank 4 - 5
//
// Transmit Arbiter Registers MAC 1 and 2, use SK_REG() to access
// Queue Register Offsets, use Q_ADDR() to access

// RAM Buffer Register Offsets
// 0x10 - 0x1f:	reserved at Tx RAM Buffer Registers
// Receive and Transmit Queues
// Different MAC Types
// Different PHY Types
// PHY addresses (bits 12..8 of PHY address reg)
// GPHY address (bits 15..11 of SMI control reg)

// Receive MAC FIFO, Receive LED, and Link_Sync regs (GENESIS only)
// Receive and Transmit MAC FIFO Registers (GENESIS only)
// RX_MFF_CTRL1	16 bit	Receive MAC FIFO Control Reg 1
// TX_MFF_CTRL1	16 bit	Transmit MAC FIFO Control Reg 1
// RX_MFF_TST2	 	 8 bit	Receive MAC FIFO Test Register 2
// TX_MFF_TST2	 	 8 bit	Transmit MAC FIFO Test Register 2
// RX_MFF_TST1	 	 8 bit	Receive MAC FIFO Test Register 1
// TX_MFF_TST1	 	 8 bit	Transmit MAC FIFO Test Register 1
// RX_MFF_CTRL2	 8 bit	Receive MAC FIFO Control Reg 2
// TX_MFF_CTRL2	 8 bit	Transmit MAC FIFO Control Reg 2
// Link LED Counter Registers (GENESIS only)
// RX_LED_CTRL		 8 bit	Receive LED Cnt Control Reg
// TX_LED_CTRL		 8 bit	Transmit LED Cnt Control Reg
// LNK_SYNC_CTRL	 8 bit	Link Sync Cnt Control Register
// RX_LED_TST		 8 bit	Receive LED Cnt Test Register
// TX_LED_TST		 8 bit	Transmit LED Cnt Test Register
// LNK_SYNC_TST	 8 bit	Link Sync Cnt Test Register
// LNK_LED_REG	 	 8 bit	Link LED Register
// Receive GMAC FIFO (YUKON)
// TXA_TEST		 8 bit	Tx Arbiter Test Register
// TXA_STAT		 8 bit	Tx Arbiter Status Register
// Q_BC			32 bit	Current Byte Counter
// BMU Control Status Registers
// B0_R1_CSR		32 bit	BMU Ctrl/Stat Rx Queue 1
// B0_R2_CSR		32 bit	BMU Ctrl/Stat Rx Queue 2
// B0_XA1_CSR		32 bit	BMU Ctrl/Stat Sync Tx Queue 1
// B0_XS1_CSR		32 bit	BMU Ctrl/Stat Async Tx Queue 1
// B0_XA2_CSR		32 bit	BMU Ctrl/Stat Sync Tx Queue 2
// B0_XS2_CSR		32 bit	BMU Ctrl/Stat Async Tx Queue 2
// Q_CSR			32 bit	BMU Control/Status Register

// Q_F				32 bit	Flag Register
// RAM Buffer Register Offsets, use RB_ADDR(Queue, Offs) to access
// RB_START		32 bit	RAM Buffer Start Address
// RB_END			32 bit	RAM Buffer End Address
// RB_WP			32 bit	RAM Buffer Write Pointer
// RB_RP			32 bit	RAM Buffer Read Pointer
// RB_RX_UTPP		32 bit	Rx Upper Threshold, Pause Pack
// RB_RX_LTPP		32 bit	Rx Lower Threshold, Pause Pack
// RB_RX_UTHP		32 bit	Rx Upper Threshold, High Prio
// RB_RX_LTHP		32 bit	Rx Lower Threshold, High Prio
// RB_PC			32 bit	RAM Buffer Packet Counter
// RB_LEV			32 bit	RAM Buffer Level Register
pub const RB_MSK: c_uint = 0x0007ffff	/* Bit 18.. 0:	RAM Buffer Pointer Bits */;
// RB_TST2			 8 bit	RAM Buffer Test Register 2
// RB_TST1			 8 bit	RAM Buffer Test Register 1
// RB_CTRL			 8 bit	RAM Buffer Control Register
// Transmit MAC FIFO and Transmit LED Registers (GENESIS only),
// Counter and Timer constants, for a host clock of 62.5 MHz
pub const SK_XMIT_DUR: c_uint = 0x002faf08UL	/*  50 ms */;
pub const SK_BLK_DUR: c_uint = 0x01dcd650UL	/* 500 ms */;
pub const SK_DPOLL_DEF: c_uint = 0x00ee6b28UL	/* 250 ms at 62.5 MHz */;
pub const SK_DPOLL_MAX: c_uint = 0x00ffffffUL	/* 268 ms at 62.5 MHz */;
// 215 ms at 78.12 MHz

// Transmit GMAC FIFO (YUKON only)
// Descriptor Poll Timer Registers
// Time Stamp Timer Registers (YUKON only)
// GMAC and GPHY Control Registers (YUKON only)
// Wake-up Frame Pattern Match Control Registers (YUKON only)
// WOL Pattern Length Registers (YUKON only)
// WOL Pattern Counter Registers (YUKON only)

//
// Receive Frame Status Encoding
//
// XMR_FS_ERR will be set if
// XMR_FS_FCS_ERR, XMR_FS_LNG_ERR, XMR_FS_RUNT,
// XMR_FS_FRA_ERR, XMR_FS_LEN_ERR, or XMR_FS_CEX_ERR
// is set. XMR_FS_LNG_ERR and XMR_FS_LEN_ERR will issue
// XMR_FS_ERR unless the corresponding bit in the Receive Command
// Register is set.
//
// Broadcom-PHY Registers, indirect addressed over XMAC
//
// Broadcom-specific registers
//
// Marvel-PHY Registers, indirect addressed over GMAC
//
// Marvel-specific registers
// for 10/100 Fast Ethernet PHY (88E3082 only)
// different Broadcom PHY Ids
// different Marvell PHY Ids
// Advertisement register bits
// Xmac Specific
// Pause Bits (PHY_X_AN_PAUSE and PHY_X_RS_PAUSE) encoding
// PHY_XMAC_EXT_STAT	16 bit r/w	Extended Status Register
// PHY_XMAC_RES_ABI	16 bit r/o	PHY Resolved Ability
// Remote Fault Bits (PHY_X_AN_RFB) encoding
// Broadcom-Specific
// PHY_BCOM_1000T_CTRL	16 bit r/w	1000Base-T Control Reg
// PHY_BCOM_1000T_STAT	16 bit r/o	1000Base-T Status Reg
// PHY_MARV_1000T_STAT	16 bit r/o	1000Base-T Status Reg
// Bit  9..8:	reserved
// PHY_BCOM_EXT_STAT	16 bit r/o	Extended Status Register
// PHY_BCOM_P_EXT_CTRL	16 bit r/w	PHY Extended Control Reg
// PHY_BCOM_P_EXT_STAT	16 bit r/o	PHY Extended Status Reg
// PHY_BCOM_AUNE_ADV	16 bit r/w	Auto-Negotiation Advertisement
// PHY_BCOM_AUNE_LP	16 bit r/o	Link Partner Ability Reg
// PHY_BCOM_FC_CTR		16 bit r/w	False Carrier Counter
// PHY_BCOM_RNO_CTR	16 bit r/w	Receive NOT_OK Counter
// PHY_BCOM_AUX_CTRL	16 bit r/w	Auxiliary Control Reg
// Bit 11:	reserved
// Bit  9.. 8:	reserved
// Bit  6:	reserved
// Bit  4:	reserved
// PHY_BCOM_AUX_STAT	16 bit r/o	Auxiliary Status Reg

// PHY_BCOM_INT_STAT	16 bit r/o	Interrupt Status Reg
// PHY_BCOM_INT_MASK	16 bit r/w	Interrupt Mask Reg

// Pause Bits (PHY_B_AN_ASP and PHY_B_AN_PC) encoding
//
// Resolved Duplex mode and Capabilities (Aux Status Summary Reg)
//
// Marvell-Specific
// special defines for FIBER (88E1011S only)
// Pause Bits (PHY_M_AN_ASP_X and PHY_M_AN_PC_X) encoding
// PHY_MARV_1000T_CTRL	16 bit r/w	1000Base-T Control Reg
// PHY_MARV_PHY_CTRL	16 bit r/w	PHY Specific Ctrl Reg
// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_PHY_STAT	16 bit r/o	PHY Specific Status Reg

// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_EXT_CTRL	16 bit r/w	Ext. PHY Specific Ctrl
// (88E1011 only)
// (88E1111 only)
// !!! Errata in spec. (1 = disable)

// 100=5x; 101=6x; 110=7x; 111=8x
// PHY_MARV_LED_CTRL	16 bit r/w	LED Control Reg
// (88E1111 only)

// (88E1011 only)
// PHY_MARV_LED_OVER	16 bit r/w	Manual LED Override Reg

// Bit 13..12:	reserved

// PHY_MARV_EXT_CTRL_2	16 bit r/w	Ext. PHY Specific Ctrl 2
// PHY_MARV_EXT_P_STAT 16 bit r/w	Ext. PHY Specific Status
// (88E1111 only)
// Bit  9.. 4: reserved (88E1011 only)
// PHY_MARV_CABLE_DIAG	16 bit r/o	Cable Diagnostic Reg
// (88E1111 only)
// values for Cable Diagnostic Status (11=fail; 00=OK; 10=open; 01=short)
// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_FE_LED_PAR		16 bit r/w	LED Parallel Select Reg.
// Bit 15..12: reserved (used internally)

// ,PHY_MARV_FE_SPEC_2		16 bit r/w	Specific Control Reg. 2
// PHY_MARV_PHY_CTRL (page 3)		16 bit r/w	LED Control Reg.

// GMAC registers
// Port Registers
// Source Address Registers
// Multicast Address Hash Registers
// Interrupt Source Registers
// Interrupt Mask Registers
// Serial Management Interface (SMI) Registers
// MIB Counters
pub const GM_MIB_CNT_BASE: c_uint = 0x0100		/* Base Address of MIB Counters */;

//
// MIB Counters base address definitions (low word) -
// use offset 4 for access to high word	(32 bit r/o)
//
// GM_MIB_CNT_BASE + 40:	reserved
// GM_MIB_CNT_BASE + 168:	reserved
// GM_MIB_CNT_BASE + 184:	reserved
// GMAC Bit Definitions
// GM_GP_STAT	16 bit r/o	General Purpose Status Register
// GM_GP_CTRL	16 bit r/w	General Purpose Control Register

// GM_TX_CTRL			16 bit r/w	Transmit Control Register

pub const TX_COL_DEF: c_uint = 0x04	/* late collision after 64 byte */;
// GM_RX_CTRL			16 bit r/w	Receive Control Register
// GM_TX_PARAM		16 bit r/w	Transmit Parameter Register

// GM_SERIAL_MODE			16 bit r/w	Serial Mode Register

pub const DATA_BLIND_DEF: c_uint = 0x04;

pub const IPG_DATA_DEF: c_uint = 0x1e;
// GM_SMI_CTRL			16 bit r/w	SMI Control Register

// GM_PHY_ADDR				16 bit r/w	GPHY Address Register
// Receive Frame Status Encoding
//
// GMR_FS_ANY_ERR (analogous to XMR_FS_ANY_ERR)
//
// Rx GMAC FIFO Flush Mask (default)
// RX_GMF_CTRL_T	32 bit	Rx GMAC FIFO Control/Test
// TX_GMF_CTRL_T	32 bit	Tx GMAC FIFO Control/Test
// GMAC_TI_ST_CTRL	 8 bit	Time Stamp Timer Ctrl Reg (YUKON only)
// GMAC_CTRL		32 bit	GMAC Control Reg (YUKON only)
// GPHY_CTRL		32 bit	GPHY Control Reg (YUKON only)
// Bits  7..2:	reserved

// forced speed and duplex mode (don't mix with other ANEG bits)
pub const GPC_FRC10MBIT_HALF: c_int = 0;

// auto-negotiation with limited advertised speeds
// mix only with master/slave settings (for copper)

// master/slave settings
// only for copper with 1000 Mbps
pub const GPC_FORCE_MASTER: c_int = 0;

// GMAC_IRQ_SRC	 8 bit	GMAC Interrupt Source Reg (YUKON only)
// GMAC_IRQ_MSK	 8 bit	GMAC Interrupt Mask   Reg (YUKON only)

// GMAC_LINK_CTRL	16 bit	GMAC Link Control Reg (YUKON only)
// Bits 15.. 2:	reserved
// WOL_CTRL_STAT	16 bit	WOL Control/Status Reg

// WOL_MATCH_CTL	 8 bit	WOL Match Control Reg

// XMAC II registers

// XM_MMU_CMD	16 bit r/w	MMU Command Register
// XM_TX_CMD	16 bit r/w	Transmit Command Register
// XM_TX_RT_LIM	16 bit r/w	Transmit Retry Limit Register
pub const XM_RT_LIM_MSK: c_uint = 0x1f	/* Bit  4..0:	Tx Retry Limit */;
// XM_TX_STIME	16 bit r/w	Transmit Slottime Register
pub const XM_STIME_MSK: c_uint = 0x7f	/* Bit  6..0:	Tx Slottime bits */;
// XM_TX_IPG	16 bit r/w	Transmit Inter Packet Gap
pub const XM_IPG_MSK: c_uint = 0xff	/* Bit  7..0:	IPG value bits */;
// XM_RX_CMD	16 bit r/w	Receive Command Register
// inrange error packets
// jumbo packets
// XM_GP_PORT	32 bit r/w	General Purpose Port Register
// XM_IMSK		16 bit r/w	Interrupt Mask Register
// XM_ISRC		16 bit r/o	Interrupt Status Register
// XM_HW_CFG	16 bit r/w	Hardware Config Register
// XM_TX_LO_WM	16 bit r/w	Tx FIFO Low Water Mark
// XM_TX_HI_WM	16 bit r/w	Tx FIFO High Water Mark
pub const XM_TX_WM_MSK: c_uint = 0x01ff	/* Bit  9.. 0	Tx FIFO Watermark bits */;
// XM_TX_THR	16 bit r/w	Tx Request Threshold
// XM_HT_THR	16 bit r/w	Host Request Threshold
// XM_RX_THR	16 bit r/w	Rx Request Threshold
pub const XM_THR_MSK: c_uint = 0x03ff	/* Bit 10.. 0	Rx/Tx Request Threshold bits */;
// XM_TX_STAT	32 bit r/o	Tx Status LIFO Register
// XM_RX_LO_WM	16 bit r/w	Receive Low Water Mark
// XM_RX_HI_WM	16 bit r/w	Receive High Water Mark
pub const XM_RX_WM_MSK: c_uint = 0x03ff		/* Bit 11.. 0:	Rx FIFO Watermark bits */;
// XM_DEV_ID	32 bit r/o	Device ID Register

// XM_MODE		32 bit r/w	Mode Register
// extern generated
// intern generated

// XM_STAT_CMD	16 bit r/w	Statistics Command Register
// XM_RX_CNT_EV	32 bit r/o	Rx Counter Event Register
// XM_RX_EV_MSK	32 bit r/w	Rx Counter Event Mask

// XM_TX_CNT_EV	32 bit r/o	Tx Counter Event Register
// XM_TX_EV_MSK	32 bit r/w	Tx Counter Event Mask

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_rx_desc {
    pub control: u32,
    pub next_offset: u32,
    pub dma_lo: u32,
    pub dma_hi: u32,
    pub status: u32,
    pub timestamp: u32,
    pub csum2: u16,
    pub csum1: u16,
    pub csum2_start: u16,
    pub csum1_start: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_tx_desc {
    pub control: u32,
    pub next_offset: u32,
    pub dma_lo: u32,
    pub dma_hi: u32,
    pub status: u32,
    pub csum_offs: u32,
    pub csum_write: u16,
    pub csum_start: u16,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_element {
    pub next: *mut skge_element,
    pub desc: *mut c_void,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_ring {
    pub to_clean: *mut skge_element,
    pub to_use: *mut skge_element,
    pub start: *mut skge_element,
    pub count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_hw {
    pub regs: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub hw_lock: spinlock_t,
    pub intr_mask: u32,
    pub dev: [*mut net_device; 2],
    pub chip_id: u8,
    pub chip_rev: u8,
    pub copper: u8,
    pub ports: u8,
    pub phy_type: u8,
    pub ram_size: u32,
    pub ram_offset: u32,
    pub phy_addr: u16,
    pub phy_lock: spinlock_t,
    pub phy_task: tasklet_struct,
    pub /: *mut *mut char irq_name[]; / skge@pci:000:04:00.0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pause_control {
    FLOW_MODE_NONE 		= 1, /* No Flow-Control */
    FLOW_MODE_LOC_SEND	= 2, /* Local station sends PAUSE */
    FLOW_MODE_SYMMETRIC	= 3, /* Both stations may send PAUSE */
    FLOW_MODE_SYM_OR_REM	= 4, /* Both stations may send PAUSE or
// just the remote station may send PAUSE
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pause_status {
    FLOW_STAT_INDETERMINATED=0,	/* indeterminated */
    FLOW_STAT_NONE,			/* No Flow Control */
    FLOW_STAT_REM_SEND,		/* Remote Station sends PAUSE */
    FLOW_STAT_LOC_SEND,		/* Local station sends PAUSE */
    FLOW_STAT_SYMMETRIC,		/* Both station may send PAUSE */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skge_port {
    pub hw: *mut skge_hw,
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub port: c_int,
    pub msg_enable: u32,
    pub tx_ring: skge_ring,
    pub ____cacheline_aligned_in_smp: skge_ring rx_ring,
    pub rx_buf_size: c_uint,
    pub link_timer: timer_list,
    pub flow_control: pause_control,
    pub flow_status: pause_status,
    pub blink_on: u8,
    pub wol: u8,
    pub /: *mut *mut u8 autoneg; / AUTONEG_ENABLE, AUTONEG_DISABLE,
    pub /: *mut *mut u8 duplex; / DUPLEX_HALF, DUPLEX_FULL,
    pub /: *mut *mut u16 speed; / SPEED_1000, SPEED_100, ...,
    pub advertising: u32,
    pub /: *mut *mut *mut void mem; / PCI memory for rings,
    pub dma: dma_addr_t,
    pub mem_size: c_ulong,

    pub debugfs: *mut dentry,

}

// Register accessor for memory mapped device
extern "C" {
    pub fn readl(reg: hw->regs +) -> return;
}
extern "C" {
    pub fn readw(reg: hw->regs +) -> return;
}
extern "C" {
    pub fn readb(reg: hw->regs +) -> return;
}
// MAC Related Registers inside the device.

extern "C" {
    pub fn skge_read16(_arg: hw, _arg: SK_XMAC_REG(port, _arg: reg)) -> return;
}

extern "C" {
    pub fn skge_read16(_arg: hw, _arg: SK_GMAC_REG(port, _arg: reg)) -> return;
}
