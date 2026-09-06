//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cirrus/cs89x0.h
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


// Copyright, 1988-1992, Russell Nelson, Crynwr Software
//
pub const PP_ChipID: c_uint = 0x0000	/* offset   0h -> Corp -ID              */;
// offset   2h -> Model/Product Number
// offset   3h -> Chip Revision Number
pub const PP_ISAIOB: c_uint = 0x0020	/*  IO base address */;
pub const PP_CS8900_ISAINT: c_uint = 0x0022	/*  ISA interrupt select */;
pub const PP_CS8920_ISAINT: c_uint = 0x0370	/*  ISA interrupt select */;
pub const PP_CS8900_ISADMA: c_uint = 0x0024	/*  ISA Rec DMA channel */;
pub const PP_CS8920_ISADMA: c_uint = 0x0374	/*  ISA Rec DMA channel */;
pub const PP_ISASOF: c_uint = 0x0026	/*  ISA DMA offset */;
pub const PP_DmaFrameCnt: c_uint = 0x0028	/*  ISA DMA Frame count */;
pub const PP_DmaByteCnt: c_uint = 0x002A	/*  ISA DMA Byte count */;
pub const PP_CS8900_ISAMemB: c_uint = 0x002C	/*  Memory base */;
pub const PP_CS8920_ISAMemB: c_uint = 0x0348 /*  */;
pub const PP_ISABootBase: c_uint = 0x0030	/*  Boot Prom base  */;
pub const PP_ISABootMask: c_uint = 0x0034	/*  Boot Prom Mask */;
// EEPROM data and command registers
pub const PP_EECMD: c_uint = 0x0040		/*  NVR Interface Command register */;
pub const PP_EEData: c_uint = 0x0042	/*  NVR Interface Data Register */;
pub const PP_DebugReg: c_uint = 0x0044	/*  Debug Register */;
pub const PP_RxCFG: c_uint = 0x0102		/*  Rx Bus config */;
pub const PP_RxCTL: c_uint = 0x0104		/*  Receive Control Register */;
pub const PP_TxCFG: c_uint = 0x0106		/*  Transmit Config Register */;
pub const PP_TxCMD: c_uint = 0x0108		/*  Transmit Command Register */;
pub const PP_BufCFG: c_uint = 0x010A	/*  Bus configuration Register */;
pub const PP_LineCTL: c_uint = 0x0112	/*  Line Config Register */;
pub const PP_SelfCTL: c_uint = 0x0114	/*  Self Command Register */;
pub const PP_BusCTL: c_uint = 0x0116	/*  ISA bus control Register */;
pub const PP_TestCTL: c_uint = 0x0118	/*  Test Register */;
pub const PP_AutoNegCTL: c_uint = 0x011C	/*  Auto Negotiation Ctrl */;
pub const PP_ISQ: c_uint = 0x0120		/*  Interrupt Status */;
pub const PP_RxEvent: c_uint = 0x0124	/*  Rx Event Register */;
pub const PP_TxEvent: c_uint = 0x0128	/*  Tx Event Register */;
pub const PP_BufEvent: c_uint = 0x012C	/*  Bus Event Register */;
pub const PP_RxMiss: c_uint = 0x0130	/*  Receive Miss Count */;
pub const PP_TxCol: c_uint = 0x0132		/*  Transmit Collision Count */;
pub const PP_LineST: c_uint = 0x0134	/*  Line State Register */;
pub const PP_SelfST: c_uint = 0x0136	/*  Self State register */;
pub const PP_BusST: c_uint = 0x0138		/*  Bus Status */;
pub const PP_TDR: c_uint = 0x013C		/*  Time Domain Reflectometry */;
pub const PP_AutoNegST: c_uint = 0x013E	/*  Auto Neg Status */;
pub const PP_TxCommand: c_uint = 0x0144	/*  Tx Command */;
pub const PP_TxLength: c_uint = 0x0146	/*  Tx Length */;
pub const PP_LAF: c_uint = 0x0150		/*  Hash Table */;
pub const PP_IA: c_uint = 0x0158		/*  Physical Address Register */;
pub const PP_RxStatus: c_uint = 0x0400	/*  Receive start of frame */;
pub const PP_RxLength: c_uint = 0x0402	/*  Receive Length of frame */;
pub const PP_RxFrame: c_uint = 0x0404	/*  Receive frame pointer */;
pub const PP_TxFrame: c_uint = 0x0A00	/*  Transmit frame pointer */;
// Primary I/O Base Address. If no I/O base is supplied by the user, then this
// can be used as the default I/O base to access the PacketPage Area.
pub const DEFAULTIOBASE: c_uint = 0x0300;
pub const FIRST_IO: c_uint = 0x020C		/*  First I/O port to check */;
pub const LAST_IO: c_uint = 0x037C		/*  Last I/O port to check (+10h) */;
pub const ADD_MASK: c_uint = 0x3000		/*  Mask it use of the ADD_PORT register */;
pub const ADD_SIG: c_uint = 0x3000		/*  Expected ID signature */;
// On Macs, we only need use the ISA I/O stuff until we do MEMORY_ON

pub const LCSLOTBASE: c_uint = 0xfee00000;
pub const MMIOBASE: c_uint = 0x40000;

pub const CHIP_EISA_ID_SIG: c_uint = 0x630E   /*  Product ID Code for Crystal Chip (CS8900 spec 4.3) */;

pub const EISA_ID_SIG: c_uint = 0x4D24	/*  IBM */;
pub const PART_NO_SIG: c_uint = 0x1010	/*  IBM */;
pub const MONGOOSE_BIT: c_uint = 0x0000	/*  IBM */;

pub const EISA_ID_SIG: c_uint = 0x630E	/*  PnP Vendor ID (same as chip id for Crystal board) */;
pub const PART_NO_SIG: c_uint = 0x4000	/*  ID code CS8920 board (PnP Vendor Product code) */;
pub const MONGOOSE_BIT: c_uint = 0x2000	/*  PART_NO_SIG + MONGOOSE_BUT => ID of mongoose */;

pub const PRODUCT_ID_ADD: c_uint = 0x0002   /*  Address of product ID */;
// Mask to find out the types of  registers
pub const REG_TYPE_MASK: c_uint = 0x001F;
// Eeprom Commands
pub const ERSE_WR_ENBL: c_uint = 0x00F0;
pub const ERSE_WR_DISABLE: c_uint = 0x0000;
// Defines Control/Config register quintuplet numbers
pub const RX_BUF_CFG: c_uint = 0x0003;
pub const RX_CONTROL: c_uint = 0x0005;
pub const TX_CFG: c_uint = 0x0007;
pub const TX_COMMAND: c_uint = 0x0009;
pub const BUF_CFG: c_uint = 0x000B;
pub const LINE_CONTROL: c_uint = 0x0013;
pub const SELF_CONTROL: c_uint = 0x0015;
pub const BUS_CONTROL: c_uint = 0x0017;
pub const TEST_CONTROL: c_uint = 0x0019;
// Defines Status/Count registers quintuplet numbers
pub const RX_EVENT: c_uint = 0x0004;
pub const TX_EVENT: c_uint = 0x0008;
pub const BUF_EVENT: c_uint = 0x000C;
pub const RX_MISS_COUNT: c_uint = 0x0010;
pub const TX_COL_COUNT: c_uint = 0x0012;
pub const LINE_STATUS: c_uint = 0x0014;
pub const SELF_STATUS: c_uint = 0x0016;
pub const BUS_STATUS: c_uint = 0x0018;
pub const TDR: c_uint = 0x001C;
// PP_RxCFG - Receive  Configuration and Interrupt Mask bit definition -  Read/write
pub const SKIP_1: c_uint = 0x0040;
pub const RX_STREAM_ENBL: c_uint = 0x0080;
pub const RX_OK_ENBL: c_uint = 0x0100;
pub const RX_DMA_ONLY: c_uint = 0x0200;
pub const AUTO_RX_DMA: c_uint = 0x0400;
pub const BUFFER_CRC: c_uint = 0x0800;
pub const RX_CRC_ERROR_ENBL: c_uint = 0x1000;
pub const RX_RUNT_ENBL: c_uint = 0x2000;
pub const RX_EXTRA_DATA_ENBL: c_uint = 0x4000;
// PP_RxCTL - Receive Control bit definition - Read/write
pub const RX_IA_HASH_ACCEPT: c_uint = 0x0040;
pub const RX_PROM_ACCEPT: c_uint = 0x0080;
pub const RX_OK_ACCEPT: c_uint = 0x0100;
pub const RX_MULTCAST_ACCEPT: c_uint = 0x0200;
pub const RX_IA_ACCEPT: c_uint = 0x0400;
pub const RX_BROADCAST_ACCEPT: c_uint = 0x0800;
pub const RX_BAD_CRC_ACCEPT: c_uint = 0x1000;
pub const RX_RUNT_ACCEPT: c_uint = 0x2000;
pub const RX_EXTRA_DATA_ACCEPT: c_uint = 0x4000;

// Default receive mode - individually addressed, broadcast, and error free

// PP_TxCFG - Transmit Configuration Interrupt Mask bit definition - Read/write
pub const TX_LOST_CRS_ENBL: c_uint = 0x0040;
pub const TX_SQE_ERROR_ENBL: c_uint = 0x0080;
pub const TX_OK_ENBL: c_uint = 0x0100;
pub const TX_LATE_COL_ENBL: c_uint = 0x0200;
pub const TX_JBR_ENBL: c_uint = 0x0400;
pub const TX_ANY_COL_ENBL: c_uint = 0x0800;
pub const TX_16_COL_ENBL: c_uint = 0x8000;
// PP_TxCMD - Transmit Command bit definition - Read-only
pub const TX_START_4_BYTES: c_uint = 0x0000;
pub const TX_START_64_BYTES: c_uint = 0x0040;
pub const TX_START_128_BYTES: c_uint = 0x0080;
pub const TX_START_ALL_BYTES: c_uint = 0x00C0;
pub const TX_FORCE: c_uint = 0x0100;
pub const TX_ONE_COL: c_uint = 0x0200;
pub const TX_TWO_PART_DEFF_DISABLE: c_uint = 0x0400;
pub const TX_NO_CRC: c_uint = 0x1000;
pub const TX_RUNT: c_uint = 0x2000;
// PP_BufCFG - Buffer Configuration Interrupt Mask bit definition - Read/write
pub const GENERATE_SW_INTERRUPT: c_uint = 0x0040;
pub const RX_DMA_ENBL: c_uint = 0x0080;
pub const READY_FOR_TX_ENBL: c_uint = 0x0100;
pub const TX_UNDERRUN_ENBL: c_uint = 0x0200;
pub const RX_MISS_ENBL: c_uint = 0x0400;
pub const RX_128_BYTE_ENBL: c_uint = 0x0800;
pub const TX_COL_COUNT_OVRFLOW_ENBL: c_uint = 0x1000;
pub const RX_MISS_COUNT_OVRFLOW_ENBL: c_uint = 0x2000;
pub const RX_DEST_MATCH_ENBL: c_uint = 0x8000;
// PP_LineCTL - Line Control bit definition - Read/write
pub const SERIAL_RX_ON: c_uint = 0x0040;
pub const SERIAL_TX_ON: c_uint = 0x0080;
pub const AUI_ONLY: c_uint = 0x0100;
pub const AUTO_AUI_10BASET: c_uint = 0x0200;
pub const MODIFIED_BACKOFF: c_uint = 0x0800;
pub const NO_AUTO_POLARITY: c_uint = 0x1000;
pub const TWO_PART_DEFDIS: c_uint = 0x2000;
pub const LOW_RX_SQUELCH: c_uint = 0x4000;
// PP_SelfCTL - Software Self Control bit definition - Read/write
pub const POWER_ON_RESET: c_uint = 0x0040;
pub const SW_STOP: c_uint = 0x0100;
pub const SLEEP_ON: c_uint = 0x0200;
pub const AUTO_WAKEUP: c_uint = 0x0400;
pub const HCB0_ENBL: c_uint = 0x1000;
pub const HCB1_ENBL: c_uint = 0x2000;
pub const HCB0: c_uint = 0x4000;
pub const HCB1: c_uint = 0x8000;
// PP_BusCTL - ISA Bus Control bit definition - Read/write
pub const RESET_RX_DMA: c_uint = 0x0040;
pub const MEMORY_ON: c_uint = 0x0400;
pub const DMA_BURST_MODE: c_uint = 0x0800;
pub const IO_CHANNEL_READY_ON: c_uint = 0x1000;
pub const RX_DMA_SIZE_64K: c_uint = 0x2000;
pub const ENABLE_IRQ: c_uint = 0x8000;
// PP_TestCTL - Test Control bit definition - Read/write
pub const LINK_OFF: c_uint = 0x0080;
pub const ENDEC_LOOPBACK: c_uint = 0x0200;
pub const AUI_LOOPBACK: c_uint = 0x0400;
pub const BACKOFF_OFF: c_uint = 0x0800;
pub const FDX_8900: c_uint = 0x4000;
pub const FAST_TEST: c_uint = 0x8000;
// PP_RxEvent - Receive Event Bit definition - Read-only
pub const RX_IA_HASHED: c_uint = 0x0040;
pub const RX_DRIBBLE: c_uint = 0x0080;
pub const RX_OK: c_uint = 0x0100;
pub const RX_HASHED: c_uint = 0x0200;
pub const RX_IA: c_uint = 0x0400;
pub const RX_BROADCAST: c_uint = 0x0800;
pub const RX_CRC_ERROR: c_uint = 0x1000;
pub const RX_RUNT: c_uint = 0x2000;
pub const RX_EXTRA_DATA: c_uint = 0x4000;
pub const HASH_INDEX_MASK: c_uint = 0x0FC00;
// PP_TxEvent - Transmit Event Bit definition - Read-only
pub const TX_LOST_CRS: c_uint = 0x0040;
pub const TX_SQE_ERROR: c_uint = 0x0080;
pub const TX_OK: c_uint = 0x0100;
pub const TX_LATE_COL: c_uint = 0x0200;
pub const TX_JBR: c_uint = 0x0400;
pub const TX_16_COL: c_uint = 0x8000;

pub const TX_COL_COUNT_MASK: c_uint = 0x7800;
// PP_BufEvent - Buffer Event Bit definition - Read-only
pub const SW_INTERRUPT: c_uint = 0x0040;
pub const RX_DMA: c_uint = 0x0080;
pub const READY_FOR_TX: c_uint = 0x0100;
pub const TX_UNDERRUN: c_uint = 0x0200;
pub const RX_MISS: c_uint = 0x0400;
pub const RX_128_BYTE: c_uint = 0x0800;
pub const TX_COL_OVRFLW: c_uint = 0x1000;
pub const RX_MISS_OVRFLW: c_uint = 0x2000;
pub const RX_DEST_MATCH: c_uint = 0x8000;
// PP_LineST - Ethernet Line Status bit definition - Read-only
pub const LINK_OK: c_uint = 0x0080;
pub const AUI_ON: c_uint = 0x0100;
pub const TENBASET_ON: c_uint = 0x0200;
pub const POLARITY_OK: c_uint = 0x1000;
pub const CRS_OK: c_uint = 0x4000;
// PP_SelfST - Chip Software Status bit definition
pub const ACTIVE_33V: c_uint = 0x0040;
pub const INIT_DONE: c_uint = 0x0080;
pub const SI_BUSY: c_uint = 0x0100;
pub const EEPROM_PRESENT: c_uint = 0x0200;
pub const EEPROM_OK: c_uint = 0x0400;
pub const EL_PRESENT: c_uint = 0x0800;
pub const EE_SIZE_64: c_uint = 0x1000;
// PP_BusST - ISA Bus Status bit definition
pub const TX_BID_ERROR: c_uint = 0x0080;
pub const READY_FOR_TX_NOW: c_uint = 0x0100;
// PP_AutoNegCTL - Auto Negotiation Control bit definition
pub const RE_NEG_NOW: c_uint = 0x0040;
pub const ALLOW_FDX: c_uint = 0x0080;
pub const AUTO_NEG_ENABLE: c_uint = 0x0100;
pub const NLP_ENABLE: c_uint = 0x0200;
pub const FORCE_FDX: c_uint = 0x8000;

// PP_AutoNegST - Auto Negotiation Status bit definition
pub const AUTO_NEG_BUSY: c_uint = 0x0080;
pub const FLP_LINK: c_uint = 0x0100;
pub const FLP_LINK_GOOD: c_uint = 0x0800;
pub const LINK_FAULT: c_uint = 0x1000;
pub const HDX_ACTIVE: c_uint = 0x4000;
pub const FDX_ACTIVE: c_uint = 0x8000;
// The following block defines the ISQ event types
pub const ISQ_RECEIVER_EVENT: c_uint = 0x04;
pub const ISQ_TRANSMITTER_EVENT: c_uint = 0x08;
pub const ISQ_BUFFER_EVENT: c_uint = 0x0c;
pub const ISQ_RX_MISS_EVENT: c_uint = 0x10;
pub const ISQ_TX_COL_EVENT: c_uint = 0x12;
pub const ISQ_EVENT_MASK: c_uint = 0x003F   /*  ISQ mask to find out type of event */;

pub const AUTOINCREMENT: c_uint = 0x8000	/*  Bit mask to set bit-15 for autoincrement */;
pub const TXRXBUFSIZE: c_uint = 0x0600;
pub const RXDMABUFSIZE: c_uint = 0x8000;
pub const RXDMASIZE: c_uint = 0x4000;
pub const TXRX_LENGTH_MASK: c_uint = 0x07FF;
// rx options bits

pub const RCV_POLLING: c_uint = 0x10	/*  Poll RxEvent */;
pub const RCV_ISQ: c_uint = 0x20	/*  Use ISQ, int */;
pub const RCV_AUTO_DMA: c_uint = 0x100	/*  Set AutoRxDMAE */;
pub const RCV_DMA: c_uint = 0x200	/*  Set RxDMA only */;
pub const RCV_DMA_ALL: c_uint = 0x400	/*  Copy all DMA'ed */;
pub const RCV_FIXED_DATA: c_uint = 0x800	/*  Every frame same */;
pub const RCV_IO: c_uint = 0x1000	/*  Use ISA IO only */;
pub const RCV_MEMORY: c_uint = 0x2000	/*  Use ISA Memory */;
pub const RAM_SIZE: c_uint = 0x1000       /*  The card has 4k bytes or RAM */;

pub const RX_FRAME_PORT: c_uint = 0x0000;

pub const TX_CMD_PORT: c_uint = 0x0004;
pub const TX_NOW: c_uint = 0x0000       /*  Tx packet after   5 bytes copied */;
pub const TX_AFTER_381: c_uint = 0x0040       /*  Tx packet after 381 bytes copied */;
pub const TX_AFTER_ALL: c_uint = 0x00c0       /*  Tx packet after all bytes copied */;
pub const TX_LEN_PORT: c_uint = 0x0006;
pub const ISQ_PORT: c_uint = 0x0008;
pub const ADD_PORT: c_uint = 0x000A;
pub const DATA_PORT: c_uint = 0x000C;
pub const EEPROM_WRITE_EN: c_uint = 0x00F0;
pub const EEPROM_WRITE_DIS: c_uint = 0x0000;
pub const EEPROM_WRITE_CMD: c_uint = 0x0100;
pub const EEPROM_READ_CMD: c_uint = 0x0200;
// Receive Header
// Description of header of each packet in receive area of memory

pub const CHIP_READ: c_uint = 0x1   /*  Used to mark state of the repins code (chip or dma) */;
pub const DMA_READ: c_uint = 0x2   /*  Used to mark state of the repins code (chip or dma) */;
// for bios scan
//

// use these values for debugging bios scan
pub const BIOS_START_SEG: c_uint = 0x00000;
pub const BIOS_OFFSET_INC: c_uint = 0x0010;

pub const BIOS_START_SEG: c_uint = 0x0c000;
pub const BIOS_OFFSET_INC: c_uint = 0x0200;

pub const BIOS_LAST_OFFSET: c_uint = 0x0fc00;
// Byte offsets into the EEPROM configuration buffer
pub const ISA_CNF_OFFSET: c_uint = 0x6;

// the assumption here is that the bits in the eeprom are generally
// in the same position as those in the autonegctl register.
// Of course the IMM bit is not in that register so it must be
// masked out
pub const EE_FORCE_FDX: c_uint = 0x8000;
pub const EE_NLP_ENABLE: c_uint = 0x0200;
pub const EE_AUTO_NEG_ENABLE: c_uint = 0x0100;
pub const EE_ALLOW_FDX: c_uint = 0x0080;

pub const IMM_BIT: c_uint = 0x0040		/*  ignore missing media	 */;

pub const A_CNF_10B_T: c_uint = 0x0001;
pub const A_CNF_AUI: c_uint = 0x0002;
pub const A_CNF_10B_2: c_uint = 0x0004;
pub const A_CNF_MEDIA_TYPE: c_uint = 0x0070;
pub const A_CNF_MEDIA_AUTO: c_uint = 0x0070;
pub const A_CNF_MEDIA_10B_T: c_uint = 0x0020;
pub const A_CNF_MEDIA_AUI: c_uint = 0x0040;
pub const A_CNF_MEDIA_10B_2: c_uint = 0x0010;
pub const A_CNF_DC_DC_POLARITY: c_uint = 0x0080;
pub const A_CNF_NO_AUTO_POLARITY: c_uint = 0x2000;
pub const A_CNF_LOW_RX_SQUELCH: c_uint = 0x4000;
pub const A_CNF_EXTND_10B_2: c_uint = 0x8000;
pub const PACKET_PAGE_OFFSET: c_uint = 0x8;
// Bit definitions for the ISA configuration word from the EEPROM
pub const INT_NO_MASK: c_uint = 0x000F;
pub const DMA_NO_MASK: c_uint = 0x0070;
pub const ISA_DMA_SIZE: c_uint = 0x0200;
pub const ISA_AUTO_RxDMA: c_uint = 0x0400;
pub const ISA_RxDMA: c_uint = 0x0800;
pub const DMA_BURST: c_uint = 0x1000;
pub const STREAM_TRANSFER: c_uint = 0x2000;

// DMA controller registers
pub const DMA_BASE: c_uint = 0x00     /*  DMA controller base */;
pub const DMA_BASE_2: c_uint = 0x0C0    /*  DMA controller base */;
pub const DMA_STAT: c_uint = 0x0D0    /*  DMA controller status register */;
pub const DMA_MASK: c_uint = 0x0D4    /*  DMA controller mask register */;
pub const DMA_MODE: c_uint = 0x0D6    /*  DMA controller mode register */;
pub const DMA_RESETFF: c_uint = 0x0D8    /*  DMA controller first/last flip flop */;
// DMA data
pub const DMA_DISABLE: c_uint = 0x04     /*  Disable channel n */;
pub const DMA_ENABLE: c_uint = 0x00     /*  Enable channel n */;
// Demand transfers, incr. address, auto init, writes, ch. n
pub const DMA_RX_MODE: c_uint = 0x14;
// Demand transfers, incr. address, auto init, reads, ch. n
pub const DMA_TX_MODE: c_uint = 0x18;

pub const CS8900: c_uint = 0x0000;
pub const CS8920: c_uint = 0x4000;
pub const CS8920M: c_uint = 0x6000;
pub const REVISON_BITS: c_uint = 0x1F00;
pub const EEVER_NUMBER: c_uint = 0x12;
pub const CHKSUM_LEN: c_uint = 0x14;
pub const CHKSUM_VAL: c_uint = 0x0000;
pub const START_EEPROM_DATA: c_uint = 0x001c /*  Offset into eeprom for start of data */;
pub const IRQ_MAP_EEPROM_DATA: c_uint = 0x0046 /*  Offset into eeprom for the IRQ map */;
pub const IRQ_MAP_LEN: c_uint = 0x0004 /*  No of bytes to read for the IRQ map */;
pub const PNP_IRQ_FRMT: c_uint = 0x0022 /*  PNP small item IRQ format */;
pub const CS8900_IRQ_MAP: c_uint = 0x1c20 /*  This IRQ map is fixed */;
pub const CS8920_NO_INTS: c_uint = 0x0F   /*  Max CS8920 interrupt select # */;
pub const PNP_ADD_PORT: c_uint = 0x0279;
pub const PNP_WRITE_PORT: c_uint = 0x0A79;
pub const GET_PNP_ISA_STRUCT: c_uint = 0x40;
pub const PNP_ISA_STRUCT_LEN: c_uint = 0x06;
pub const PNP_CSN_CNT_OFF: c_uint = 0x01;
pub const PNP_RD_PORT_OFF: c_uint = 0x02;
pub const PNP_FUNCTION_OK: c_uint = 0x00;
pub const PNP_WAKE: c_uint = 0x03;
pub const PNP_RSRC_DATA: c_uint = 0x04;
pub const PNP_RSRC_READY: c_uint = 0x01;
pub const PNP_STATUS: c_uint = 0x05;
pub const PNP_ACTIVATE: c_uint = 0x30;
pub const PNP_CNF_IO_H: c_uint = 0x60;
pub const PNP_CNF_IO_L: c_uint = 0x61;
pub const PNP_CNF_INT: c_uint = 0x70;
pub const PNP_CNF_DMA: c_uint = 0x74;
pub const PNP_CNF_MEM: c_uint = 0x48;
