//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/dc395x.h
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
// dc395x.h
//
// Device Driver for Tekram DC395(U/UW/F), DC315(U)
// PCI SCSI Bus Master Host Adapter
// (SCSI chip set used Tekram ASIC TRM-S1040)
//

// Macro flag: #define DC395x_H
//
// Initial values
//
pub const DC395x_MAX_CMD_QUEUE: c_int = 32;
// #define DC395x_MAX_QTAGS		32
pub const DC395x_MAX_QTAGS: c_int = 16;
pub const DC395x_MAX_SCSI_ID: c_int = 16;

// item
pub const DC395x_MAX_SRB_CNT: c_int = 63;
// #define DC395x_MAX_CAN_QUEUE		7 * DC395x_MAX_QTAGS

pub const DC395x_END_SCAN: c_int = 2;

pub const DC395x_MAX_RETRIES: c_int = 3;

// Macro flag: #define SYNC_FIRST

pub const NORM_REC_LVL: c_int = 0;
//
// Various definitions
//
pub const BIT31: c_uint = 0x80000000;
pub const BIT30: c_uint = 0x40000000;
pub const BIT29: c_uint = 0x20000000;
pub const BIT28: c_uint = 0x10000000;
pub const BIT27: c_uint = 0x08000000;
pub const BIT26: c_uint = 0x04000000;
pub const BIT25: c_uint = 0x02000000;
pub const BIT24: c_uint = 0x01000000;
pub const BIT23: c_uint = 0x00800000;
pub const BIT22: c_uint = 0x00400000;
pub const BIT21: c_uint = 0x00200000;
pub const BIT20: c_uint = 0x00100000;
pub const BIT19: c_uint = 0x00080000;
pub const BIT18: c_uint = 0x00040000;
pub const BIT17: c_uint = 0x00020000;
pub const BIT16: c_uint = 0x00010000;
pub const BIT15: c_uint = 0x00008000;
pub const BIT14: c_uint = 0x00004000;
pub const BIT13: c_uint = 0x00002000;
pub const BIT12: c_uint = 0x00001000;
pub const BIT11: c_uint = 0x00000800;
pub const BIT10: c_uint = 0x00000400;
pub const BIT9: c_uint = 0x00000200;
pub const BIT8: c_uint = 0x00000100;
pub const BIT7: c_uint = 0x00000080;
pub const BIT6: c_uint = 0x00000040;
pub const BIT5: c_uint = 0x00000020;
pub const BIT4: c_uint = 0x00000010;
pub const BIT3: c_uint = 0x00000008;
pub const BIT2: c_uint = 0x00000004;
pub const BIT1: c_uint = 0x00000002;
pub const BIT0: c_uint = 0x00000001;
// UnitCtrlFlag

// UnitFlags

// SRBState machine definition
pub const SRB_FREE: c_uint = 0x0000;
pub const SRB_WAIT: c_uint = 0x0001;
pub const SRB_READY: c_uint = 0x0002;
pub const SRB_MSGOUT: c_uint = 0x0004	/* arbitration+msg_out 1st byte		*/;
pub const SRB_MSGIN: c_uint = 0x0008;
pub const SRB_EXTEND_MSGIN: c_uint = 0x0010;
pub const SRB_COMMAND: c_uint = 0x0020;
pub const SRB_START_: c_uint = 0x0040	/* arbitration+msg_out+command_out	*/;
pub const SRB_DISCONNECT: c_uint = 0x0080;
pub const SRB_DATA_XFER: c_uint = 0x0100;
pub const SRB_XFERPAD: c_uint = 0x0200;
pub const SRB_STATUS: c_uint = 0x0400;
pub const SRB_COMPLETED: c_uint = 0x0800;
pub const SRB_ABORT_SENT: c_uint = 0x1000;
pub const SRB_DO_SYNC_NEGO: c_uint = 0x2000;
pub const SRB_DO_WIDE_NEGO: c_uint = 0x4000;
pub const SRB_UNEXPECT_RESEL: c_uint = 0x8000;
//
// ACB Config
//
pub const HCC_WIDE_CARD: c_uint = 0x20;
pub const HCC_SCSI_RESET: c_uint = 0x10;
pub const HCC_PARITY: c_uint = 0x08;
pub const HCC_AUTOTERM: c_uint = 0x04;
pub const HCC_LOW8TERM: c_uint = 0x02;
pub const HCC_UP8TERM: c_uint = 0x01;
// ACBFlag

// DCBFlag

// SRBstatus

// SRBFlag

// Adapter status
pub const H_STATUS_GOOD: c_int = 0;
pub const H_SEL_TIMEOUT: c_uint = 0x11;
pub const H_OVER_UNDER_RUN: c_uint = 0x12;
pub const H_UNEXP_BUS_FREE: c_uint = 0x13;
pub const H_TARGET_PHASE_F: c_uint = 0x14;
pub const H_INVALID_CCB_OP: c_uint = 0x16;
pub const H_LINK_CCB_BAD: c_uint = 0x17;
pub const H_BAD_TARGET_DIR: c_uint = 0x18;
pub const H_DUPLICATE_CCB: c_uint = 0x19;
pub const H_BAD_CCB_OR_SG: c_uint = 0x1A;
pub const H_ABORT: c_uint = 0x0FF;
// SCSI BUS Status byte codes
pub const SCSI_STAT_UNEXP_BUS_F: c_uint = 0xFD	/* Unexpect Bus Free			*/;
pub const SCSI_STAT_BUS_RST_DETECT: c_uint = 0xFE	/* Scsi Bus Reset detected		*/;
pub const SCSI_STAT_SEL_TIMEOUT: c_uint = 0xFF	/* Selection Time out			*/;
// Sync_Mode
pub const SYNC_WIDE_TAG_ATNT_DISABLE: c_int = 0;

pub const SYNC_NEGO_OFFSET: c_int = 15;
// cmd->result
pub const STATUS_MASK_: c_uint = 0xFF;
pub const MSG_MASK: c_uint = 0xFF00;
pub const RETURN_MASK: c_uint = 0xFF0000;
//
// Inquiry Data format
//
// Inquiry byte 0 masks
pub const SCSI_DEVTYPE: c_uint = 0x1F	/* Peripheral Device Type		*/;
pub const SCSI_PERIPHQUAL: c_uint = 0xE0	/* Peripheral Qualifier			*/;
// Inquiry byte 1 mask
pub const SCSI_REMOVABLE_MEDIA: c_uint = 0x80	/* Removable Media bit (1=removable)	*/;
// Peripheral Device Type definitions
// See include/scsi/scsi.h

//
// Inquiry flag definitions (Inq data byte 7)
//
pub const SCSI_INQ_RELADR: c_uint = 0x80	/* device supports relative addressing	*/;
pub const SCSI_INQ_WBUS32: c_uint = 0x40	/* device supports 32 bit data xfers	*/;
pub const SCSI_INQ_WBUS16: c_uint = 0x20	/* device supports 16 bit data xfers	*/;
pub const SCSI_INQ_SYNC: c_uint = 0x10	/* device supports synchronous xfer	*/;
pub const SCSI_INQ_LINKED: c_uint = 0x08	/* device supports linked commands	*/;
pub const SCSI_INQ_CMDQUEUE: c_uint = 0x02	/* device supports command queueing	*/;
pub const SCSI_INQ_SFTRE: c_uint = 0x01	/* device supports soft resets		*/;
pub const ENABLE_CE: c_int = 1;
pub const DISABLE_CE: c_int = 0;
pub const EEPROM_READ: c_uint = 0x80;
//
// The PCI configuration register offset for TRM_S1040
//
pub const TRM_S1040_ID: c_uint = 0x00	/* Vendor and Device ID			*/;
pub const TRM_S1040_COMMAND: c_uint = 0x04	/* PCI command register			*/;
pub const TRM_S1040_IOBASE: c_uint = 0x10	/* I/O Space base address		*/;
pub const TRM_S1040_ROMBASE: c_uint = 0x30	/* Expansion ROM Base Address		*/;
pub const TRM_S1040_INTLINE: c_uint = 0x3C	/* Interrupt line			*/;
//
// The SCSI register offset for TRM_S1040
//
pub const TRM_S1040_SCSI_STATUS: c_uint = 0x80	/* SCSI Status (R)			*/;
pub const COMMANDPHASEDONE: c_uint = 0x2000	/* SCSI command phase done		*/;
pub const SCSIXFERDONE: c_uint = 0x0800	/* SCSI SCSI transfer done		*/;
pub const SCSIXFERCNT_2_ZERO: c_uint = 0x0100	/* SCSI SCSI transfer count to zero	*/;
pub const SCSIINTERRUPT: c_uint = 0x0080	/* SCSI interrupt pending		*/;
pub const COMMANDABORT: c_uint = 0x0040	/* SCSI command abort			*/;
pub const SEQUENCERACTIVE: c_uint = 0x0020	/* SCSI sequencer active		*/;
pub const PHASEMISMATCH: c_uint = 0x0010	/* SCSI phase mismatch			*/;
pub const PARITYERROR: c_uint = 0x0008	/* SCSI parity error			*/;
pub const PHASEMASK: c_uint = 0x0007	/* Phase MSG/CD/IO			*/;
pub const PH_DATA_OUT: c_uint = 0x00	/* Data out phase			*/;
pub const PH_DATA_IN: c_uint = 0x01	/* Data in phase			*/;
pub const PH_COMMAND: c_uint = 0x02	/* Command phase			*/;
pub const PH_STATUS: c_uint = 0x03	/* Status phase				*/;
pub const PH_BUS_FREE: c_uint = 0x05	/* Invalid phase used as bus free	*/;
pub const PH_MSG_OUT: c_uint = 0x06	/* Message out phase			*/;
pub const PH_MSG_IN: c_uint = 0x07	/* Message in phase			*/;
pub const TRM_S1040_SCSI_CONTROL: c_uint = 0x80	/* SCSI Control (W)			*/;
pub const DO_CLRATN: c_uint = 0x0400	/* Clear ATN				*/;
pub const DO_SETATN: c_uint = 0x0200	/* Set ATN				*/;
pub const DO_CMDABORT: c_uint = 0x0100	/* Abort SCSI command			*/;
pub const DO_RSTMODULE: c_uint = 0x0010	/* Reset SCSI chip			*/;
pub const DO_RSTSCSI: c_uint = 0x0008	/* Reset SCSI bus			*/;
pub const DO_CLRFIFO: c_uint = 0x0004	/* Clear SCSI transfer FIFO		*/;
pub const DO_DATALATCH: c_uint = 0x0002	/* Enable SCSI bus data input (latched)	*/;
// #define DO_DATALATCH			0x0000 */	/* KG: DISable SCSI bus data latch
pub const DO_HWRESELECT: c_uint = 0x0001	/* Enable hardware reselection		*/;
pub const TRM_S1040_SCSI_FIFOCNT: c_uint = 0x82	/* SCSI FIFO Counter 5bits(R)		*/;
pub const TRM_S1040_SCSI_SIGNAL: c_uint = 0x83	/* SCSI low level signal (R/W)		*/;
pub const TRM_S1040_SCSI_INTSTATUS: c_uint = 0x84	/* SCSI Interrupt Status (R)		*/;
pub const INT_SCAM: c_uint = 0x80	/* SCAM selection interrupt		*/;
pub const INT_SELECT: c_uint = 0x40	/* Selection interrupt			*/;
pub const INT_SELTIMEOUT: c_uint = 0x20	/* Selection timeout interrupt		*/;
pub const INT_DISCONNECT: c_uint = 0x10	/* Bus disconnected interrupt		*/;
pub const INT_RESELECTED: c_uint = 0x08	/* Reselected interrupt			*/;
pub const INT_SCSIRESET: c_uint = 0x04	/* SCSI reset detected interrupt	*/;
pub const INT_BUSSERVICE: c_uint = 0x02	/* Bus service interrupt		*/;
pub const INT_CMDDONE: c_uint = 0x01	/* SCSI command done interrupt		*/;
pub const TRM_S1040_SCSI_OFFSET: c_uint = 0x84	/* SCSI Offset Count (W)		*/;
//
// Bit		Name		Definition
// ---------	-------------	----------------------------
// 07-05	0	RSVD		Reversed. Always 0.
// 04	0	OFFSET4		Reversed for LVDS. Always 0.
// 03-00	0	OFFSET[03:00]	Offset number from 0 to 15
//
pub const TRM_S1040_SCSI_SYNC: c_uint = 0x85	/* SCSI Synchronous Control (R/W)	*/;
pub const LVDS_SYNC: c_uint = 0x20	/* Enable LVDS synchronous		*/;
pub const WIDE_SYNC: c_uint = 0x10	/* Enable WIDE synchronous		*/;
pub const ALT_SYNC: c_uint = 0x08	/* Enable Fast-20 alternate synchronous	*/;
//
// SYNCM	7    6    5    4    3       2       1       0
// Name	RSVD RSVD LVDS WIDE ALTPERD PERIOD2 PERIOD1 PERIOD0
// Default	0    0    0    0    0       0       0       0
//
// Bit		Name		Definition
// ---------	-------------	---------------------------
// 07-06	0	RSVD		Reversed. Always read 0
// 05	0	LVDS		Reversed. Always read 0
// 04	0	WIDE/WSCSI	Enable wide (16-bits) SCSI
// transfer.
// 03	0	ALTPERD/ALTPD	Alternate (Sync./Period) mode.
//
// @@ When this bit is set,
// the synchronous period bits 2:0
// in the Synchronous Mode register
// are used to transfer data
// at the Fast-20 rate.
// @@ When this bit is unset,
// the synchronous period bits 2:0
// in the Synchronous Mode Register
// are used to transfer data
// at the Fast-10 rate (or Fast-40 w/ LVDS).
//
// 02-00	0	PERIOD[2:0]/	Synchronous SCSI Transfer Rate.
// SXPD[02:00]	These 3 bits specify
// the Synchronous SCSI Transfer
// Rate for Fast-20 and Fast-10.
// These bits are also reset
// by a SCSI Bus reset.
//
// For Fast-10 bit ALTPD = 0 and LVDS = 0
// and bit2,bit1,bit0 is defined as follows :
//
// 000	100ns, 10.0 MHz
// 001	150ns,  6.6 MHz
// 010	200ns,  5.0 MHz
// 011	250ns,  4.0 MHz
// 100	300ns,  3.3 MHz
// 101	350ns,  2.8 MHz
// 110	400ns,  2.5 MHz
// 111	450ns,  2.2 MHz
//
// For Fast-20 bit ALTPD = 1 and LVDS = 0
// and bit2,bit1,bit0 is defined as follows :
//
// 000	 50ns, 20.0 MHz
// 001	 75ns, 13.3 MHz
// 010	100ns, 10.0 MHz
// 011	125ns,  8.0 MHz
// 100	150ns,  6.6 MHz
// 101	175ns,  5.7 MHz
// 110	200ns,  5.0 MHz
// 111	250ns,  4.0 MHz   KG: Maybe 225ns, 4.4 MHz
//
// For Fast-40 bit ALTPD = 0 and LVDS = 1
// and bit2,bit1,bit0 is defined as follows :
//
// 000	 25ns, 40.0 MHz
// 001	 50ns, 20.0 MHz
// 010	 75ns, 13.3 MHz
// 011	100ns, 10.0 MHz
// 100	125ns,  8.0 MHz
// 101	150ns,  6.6 MHz
// 110	175ns,  5.7 MHz
// 111	200ns,  5.0 MHz
//
pub const TRM_S1040_SCSI_TARGETID: c_uint = 0x86	/* SCSI Target ID (R/W)			*/;
pub const TRM_S1040_SCSI_IDMSG: c_uint = 0x87	/* SCSI Identify Message (R)		*/;
pub const TRM_S1040_SCSI_HOSTID: c_uint = 0x87	/* SCSI Host ID (W)			*/;
pub const TRM_S1040_SCSI_COUNTER: c_uint = 0x88	/* SCSI Transfer Counter 24bits(R/W)	*/;
pub const TRM_S1040_SCSI_INTEN: c_uint = 0x8C	/* SCSI Interrupt Enable (R/W)		*/;
pub const EN_SCAM: c_uint = 0x80	/* Enable SCAM selection interrupt	*/;
pub const EN_SELECT: c_uint = 0x40	/* Enable selection interrupt		*/;
pub const EN_SELTIMEOUT: c_uint = 0x20	/* Enable selection timeout interrupt	*/;
pub const EN_DISCONNECT: c_uint = 0x10	/* Enable bus disconnected interrupt	*/;
pub const EN_RESELECTED: c_uint = 0x08	/* Enable reselected interrupt		*/;
pub const EN_SCSIRESET: c_uint = 0x04	/* Enable SCSI reset detected interrupt	*/;
pub const EN_BUSSERVICE: c_uint = 0x02	/* Enable bus service interrupt		*/;
pub const EN_CMDDONE: c_uint = 0x01	/* Enable SCSI command done interrupt	*/;
pub const TRM_S1040_SCSI_CONFIG0: c_uint = 0x8D	/* SCSI Configuration 0 (R/W)		*/;
pub const PHASELATCH: c_uint = 0x40	/* Enable phase latch			*/;
pub const INITIATOR: c_uint = 0x20	/* Enable initiator mode		*/;
pub const PARITYCHECK: c_uint = 0x10	/* Enable parity check			*/;
pub const BLOCKRST: c_uint = 0x01	/* Disable SCSI reset1			*/;
pub const TRM_S1040_SCSI_CONFIG1: c_uint = 0x8E	/* SCSI Configuration 1 (R/W)		*/;
pub const ACTIVE_NEGPLUS: c_uint = 0x10	/* Enhance active negation		*/;
pub const FILTER_DISABLE: c_uint = 0x08	/* Disable SCSI data filter		*/;
pub const FAST_FILTER: c_uint = 0x04	/* ?					*/;
pub const ACTIVE_NEG: c_uint = 0x02	/* Enable active negation		*/;
pub const TRM_S1040_SCSI_CONFIG2: c_uint = 0x8F	/* SCSI Configuration 2 (R/W)		*/;
pub const CFG2_WIDEFIFO: c_uint = 0x02	/*					*/;
pub const TRM_S1040_SCSI_COMMAND: c_uint = 0x90	/* SCSI Command (R/W)			*/;
pub const SCMD_COMP: c_uint = 0x12	/* Command complete			*/;
pub const SCMD_SEL_ATN: c_uint = 0x60	/* Selection with ATN			*/;
pub const SCMD_SEL_ATN3: c_uint = 0x64	/* Selection with ATN3			*/;
pub const SCMD_SEL_ATNSTOP: c_uint = 0xB8	/* Selection with ATN and Stop		*/;
pub const SCMD_FIFO_OUT: c_uint = 0xC0	/* SCSI FIFO transfer out		*/;
pub const SCMD_DMA_OUT: c_uint = 0xC1	/* SCSI DMA transfer out		*/;
pub const SCMD_FIFO_IN: c_uint = 0xC2	/* SCSI FIFO transfer in		*/;
pub const SCMD_DMA_IN: c_uint = 0xC3	/* SCSI DMA transfer in			*/;
pub const SCMD_MSGACCEPT: c_uint = 0xD8	/* Message accept			*/;
//
// Code	Command Description
// ----	----------------------------------------
// 02	Enable reselection with FIFO
// 40	Select without ATN with FIFO
// 60	Select with ATN with FIFO
// 64	Select with ATN3 with FIFO
// A0	Select with ATN and stop with FIFO
// C0	Transfer information out with FIFO
// C1	Transfer information out with DMA
// C2	Transfer information in with FIFO
// C3	Transfer information in with DMA
// 12	Initiator command complete with FIFO
// 50	Initiator transfer information out sequence without ATN
// with FIFO
// 70	Initiator transfer information out sequence with ATN
// with FIFO
// 74	Initiator transfer information out sequence with ATN3
// with FIFO
// 52	Initiator transfer information in sequence without ATN
// with FIFO
// 72	Initiator transfer information in sequence with ATN
// with FIFO
// 76	Initiator transfer information in sequence with ATN3
// with FIFO
// 90	Initiator transfer information out command complete
// with FIFO
// 92	Initiator transfer information in command complete
// with FIFO
// D2	Enable selection
// 08	Reselection
// 48	Disconnect command with FIFO
// 88	Terminate command with FIFO
// C8	Target command complete with FIFO
// 18	SCAM Arbitration/ Selection
// 5A	Enable reselection
// 98	Select without ATN with FIFO
// B8	Select with ATN with FIFO
// D8	Message Accepted
// 58	NOP
//
pub const TRM_S1040_SCSI_TIMEOUT: c_uint = 0x91	/* SCSI Time Out Value (R/W)		*/;
pub const TRM_S1040_SCSI_FIFO: c_uint = 0x98	/* SCSI FIFO (R/W)			*/;
pub const TRM_S1040_SCSI_TCR0: c_uint = 0x9C	/* SCSI Target Control 0 (R/W)		*/;
pub const TCR0_WIDE_NEGO_DONE: c_uint = 0x8000	/* Wide nego done			*/;
pub const TCR0_SYNC_NEGO_DONE: c_uint = 0x4000	/* Synchronous nego done		*/;
pub const TCR0_ENABLE_LVDS: c_uint = 0x2000	/* Enable LVDS synchronous		*/;
pub const TCR0_ENABLE_WIDE: c_uint = 0x1000	/* Enable WIDE synchronous		*/;
pub const TCR0_ENABLE_ALT: c_uint = 0x0800	/* Enable alternate synchronous		*/;
pub const TCR0_PERIOD_MASK: c_uint = 0x0700	/* Transfer rate			*/;
pub const TCR0_DO_WIDE_NEGO: c_uint = 0x0080	/* Do wide NEGO				*/;
pub const TCR0_DO_SYNC_NEGO: c_uint = 0x0040	/* Do sync NEGO				*/;
pub const TCR0_DISCONNECT_EN: c_uint = 0x0020	/* Disconnection enable			*/;
pub const TCR0_OFFSET_MASK: c_uint = 0x001F	/* Offset number			*/;
pub const TRM_S1040_SCSI_TCR1: c_uint = 0x9E	/* SCSI Target Control 1 (R/W)		*/;
pub const MAXTAG_MASK: c_uint = 0x7F00	/* Maximum tags (127)			*/;
pub const NON_TAG_BUSY: c_uint = 0x0080	/* Non tag command active		*/;
pub const ACTTAG_MASK: c_uint = 0x007F	/* Active tags				*/;
//
// The DMA register offset for TRM_S1040
//
pub const TRM_S1040_DMA_COMMAND: c_uint = 0xA0	/* DMA Command (R/W)			*/;
pub const DMACMD_SG: c_uint = 0x02	/* Enable HW S/G support		*/;
pub const DMACMD_DIR: c_uint = 0x01	/* 1 = read from SCSI write to Host	*/;
pub const XFERDATAIN_SG: c_uint = 0x0103	/* Transfer data in  w/  SG		*/;
pub const XFERDATAOUT_SG: c_uint = 0x0102	/* Transfer data out w/  SG		*/;
pub const XFERDATAIN: c_uint = 0x0101	/* Transfer data in  w/o SG		*/;
pub const XFERDATAOUT: c_uint = 0x0100	/* Transfer data out w/o SG		*/;
pub const TRM_S1040_DMA_FIFOCNT: c_uint = 0xA1	/* DMA FIFO Counter (R)			*/;
pub const TRM_S1040_DMA_CONTROL: c_uint = 0xA1	/* DMA Control (W)			*/;
pub const DMARESETMODULE: c_uint = 0x10	/* Reset PCI/DMA module			*/;
pub const STOPDMAXFER: c_uint = 0x08	/* Stop  DMA transfer			*/;
pub const ABORTXFER: c_uint = 0x04	/* Abort DMA transfer			*/;
pub const CLRXFIFO: c_uint = 0x02	/* Clear DMA transfer FIFO		*/;
pub const STARTDMAXFER: c_uint = 0x01	/* Start DMA transfer			*/;
pub const TRM_S1040_DMA_FIFOSTAT: c_uint = 0xA2	/* DMA FIFO Status (R)			*/;
pub const TRM_S1040_DMA_STATUS: c_uint = 0xA3	/* DMA Interrupt Status (R/W)		*/;
pub const XFERPENDING: c_uint = 0x80	/* Transfer pending			*/;
pub const SCSIBUSY: c_uint = 0x40	/* SCSI busy				*/;
pub const GLOBALINT: c_uint = 0x20	/* DMA_INTEN bit 0-4 set		*/;
pub const FORCEDMACOMP: c_uint = 0x10	/* Force DMA transfer complete		*/;
pub const DMAXFERERROR: c_uint = 0x08	/* DMA transfer error			*/;
pub const DMAXFERABORT: c_uint = 0x04	/* DMA transfer abort			*/;
pub const DMAXFERCOMP: c_uint = 0x02	/* Bus Master XFER Complete status	*/;
pub const SCSICOMP: c_uint = 0x01	/* SCSI complete interrupt		*/;
pub const TRM_S1040_DMA_INTEN: c_uint = 0xA4	/* DMA Interrupt Enable (R/W)		*/;
pub const EN_FORCEDMACOMP: c_uint = 0x10	/* Force DMA transfer complete		*/;
pub const EN_DMAXFERERROR: c_uint = 0x08	/* DMA transfer error			*/;
pub const EN_DMAXFERABORT: c_uint = 0x04	/* DMA transfer abort			*/;
pub const EN_DMAXFERCOMP: c_uint = 0x02	/* Bus Master XFER Complete status	*/;
pub const EN_SCSIINTR: c_uint = 0x01	/* Enable SCSI complete interrupt	*/;
pub const TRM_S1040_DMA_CONFIG: c_uint = 0xA6	/* DMA Configuration (R/W)		*/;
pub const DMA_ENHANCE: c_uint = 0x8000	/* Enable DMA enhance feature (SG?)	*/;
pub const DMA_PCI_DUAL_ADDR: c_uint = 0x4000	/*					*/;
pub const DMA_CFG_RES: c_uint = 0x2000	/* Always 1				*/;
pub const DMA_AUTO_CLR_FIFO: c_uint = 0x1000	/* DISable DMA auto clear FIFO		*/;
pub const DMA_MEM_MULTI_READ: c_uint = 0x0800	/*					*/;
pub const DMA_MEM_WRITE_INVAL: c_uint = 0x0400	/* Memory write and invalidate		*/;
pub const DMA_FIFO_CTRL: c_uint = 0x0300	/* Control FIFO operation with DMA	*/;
pub const DMA_FIFO_HALF_HALF: c_uint = 0x0200	/* Keep half filled on both read/write	*/;
pub const TRM_S1040_DMA_XCNT: c_uint = 0xA8	/* DMA Transfer Counter (R/W), 24bits	*/;
pub const TRM_S1040_DMA_CXCNT: c_uint = 0xAC	/* DMA Current Transfer Counter (R)	*/;
pub const TRM_S1040_DMA_XLOWADDR: c_uint = 0xB0	/* DMA Transfer Physical Low Address	*/;
pub const TRM_S1040_DMA_XHIGHADDR: c_uint = 0xB4	/* DMA Transfer Physical High Address	*/;
//
// The general register offset for TRM_S1040
//
pub const TRM_S1040_GEN_CONTROL: c_uint = 0xD4	/* Global Control			*/;
pub const CTRL_LED: c_uint = 0x80	/* Control onboard LED			*/;
pub const EN_EEPROM: c_uint = 0x10	/* Enable EEPROM programming		*/;
pub const DIS_TERM: c_uint = 0x08	/* Disable onboard termination		*/;
pub const AUTOTERM: c_uint = 0x04	/* Enable Auto SCSI terminator		*/;
pub const LOW8TERM: c_uint = 0x02	/* Enable Lower 8 bit SCSI terminator	*/;
pub const UP8TERM: c_uint = 0x01	/* Enable Upper 8 bit SCSI terminator	*/;
pub const TRM_S1040_GEN_STATUS: c_uint = 0xD5	/* Global Status			*/;
pub const GTIMEOUT: c_uint = 0x80	/* Global timer reach 0			*/;
pub const EXT68HIGH: c_uint = 0x40	/* Higher 8 bit connected externally	*/;
pub const INT68HIGH: c_uint = 0x20	/* Higher 8 bit connected internally	*/;
pub const CON5068: c_uint = 0x10	/* External 50/68 pin connected (low)	*/;
pub const CON68: c_uint = 0x08	/* Internal 68 pin connected (low)	*/;
pub const CON50: c_uint = 0x04	/* Internal 50 pin connected (low!)	*/;
pub const WIDESCSI: c_uint = 0x02	/* Wide SCSI card			*/;
pub const STATUS_LOAD_DEFAULT: c_uint = 0x01	/*					*/;
pub const TRM_S1040_GEN_NVRAM: c_uint = 0xD6	/* Serial NON-VOLATILE RAM port		*/;
pub const NVR_BITOUT: c_uint = 0x08	/* Serial data out			*/;
pub const NVR_BITIN: c_uint = 0x04	/* Serial data in			*/;
pub const NVR_CLOCK: c_uint = 0x02	/* Serial clock				*/;
pub const NVR_SELECT: c_uint = 0x01	/* Serial select			*/;
pub const TRM_S1040_GEN_EDATA: c_uint = 0xD7	/* Parallel EEPROM data port		*/;
pub const TRM_S1040_GEN_EADDRESS: c_uint = 0xD8	/* Parallel EEPROM address		*/;
pub const TRM_S1040_GEN_TIMER: c_uint = 0xDB	/* Global timer				*/;
//
// NvmTarCfg0: Target configuration byte 0 :..pDCB->DevMode
//
pub const NTC_DO_WIDE_NEGO: c_uint = 0x20	/* Wide negotiate			*/;
pub const NTC_DO_TAG_QUEUEING: c_uint = 0x10	/* Enable SCSI tag queuing		*/;
pub const NTC_DO_SEND_START: c_uint = 0x08	/* Send start command SPINUP		*/;
pub const NTC_DO_DISCONNECT: c_uint = 0x04	/* Enable SCSI disconnect		*/;
pub const NTC_DO_SYNC_NEGO: c_uint = 0x02	/* Sync negotiation			*/;
pub const NTC_DO_PARITY_CHK: c_uint = 0x01	/* (it should define at NAC)		*/;
// Parity check enable
//
// Nvram Initiater bits definition
//

//
// Nvram Adapter Cfg bits definition
//
pub const NAC_SCANLUN: c_uint = 0x20	/* Include LUN as BIOS device		*/;
pub const NAC_POWERON_SCSI_RESET: c_uint = 0x04	/* Power on reset enable		*/;
pub const NAC_GREATER_1G: c_uint = 0x02	/* > 1G support enable			*/;
pub const NAC_GT2DRIVES: c_uint = 0x01	/* Support more than 2 drives		*/;
// #define NAC_DO_PARITY_CHK		0x08 */	/* Parity check enable
