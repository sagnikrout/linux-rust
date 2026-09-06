//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cadence/macb.h
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
// Atmel MACB Ethernet Controller driver
//
// Copyright (C) 2004-2006 Atmel Corporation
//

pub const MACB_GREGS_NBR: c_int = 16;
pub const MACB_GREGS_VERSION: c_int = 2;
pub const MACB_MAX_QUEUES: c_int = 8;
// MACB register offsets
pub const MACB_NCR: c_uint = 0x0000 /* Network Control */;
pub const MACB_NCFGR: c_uint = 0x0004 /* Network Config */;
pub const MACB_NSR: c_uint = 0x0008 /* Network Status */;
pub const MACB_TAR: c_uint = 0x000c /* AT91RM9200 only */;
pub const MACB_TCR: c_uint = 0x0010 /* AT91RM9200 only */;
pub const MACB_TSR: c_uint = 0x0014 /* Transmit Status */;
pub const MACB_RBQP: c_uint = 0x0018 /* RX Q Base Address */;
pub const MACB_TBQP: c_uint = 0x001c /* TX Q Base Address */;
pub const MACB_RSR: c_uint = 0x0020 /* Receive Status */;
pub const MACB_ISR: c_uint = 0x0024 /* Interrupt Status */;
pub const MACB_IER: c_uint = 0x0028 /* Interrupt Enable */;
pub const MACB_IDR: c_uint = 0x002c /* Interrupt Disable */;
pub const MACB_IMR: c_uint = 0x0030 /* Interrupt Mask */;
pub const MACB_MAN: c_uint = 0x0034 /* PHY Maintenance */;
pub const MACB_PTR: c_uint = 0x0038;
pub const MACB_PFR: c_uint = 0x003c;
pub const MACB_FTO: c_uint = 0x0040;
pub const MACB_SCF: c_uint = 0x0044;
pub const MACB_MCF: c_uint = 0x0048;
pub const MACB_FRO: c_uint = 0x004c;
pub const MACB_FCSE: c_uint = 0x0050;
pub const MACB_ALE: c_uint = 0x0054;
pub const MACB_DTF: c_uint = 0x0058;
pub const MACB_LCOL: c_uint = 0x005c;
pub const MACB_EXCOL: c_uint = 0x0060;
pub const MACB_TUND: c_uint = 0x0064;
pub const MACB_CSE: c_uint = 0x0068;
pub const MACB_RRE: c_uint = 0x006c;
pub const MACB_ROVR: c_uint = 0x0070;
pub const MACB_RSE: c_uint = 0x0074;
pub const MACB_ELE: c_uint = 0x0078;
pub const MACB_RJA: c_uint = 0x007c;
pub const MACB_USF: c_uint = 0x0080;
pub const MACB_STE: c_uint = 0x0084;
pub const MACB_RLE: c_uint = 0x0088;
pub const MACB_TPF: c_uint = 0x008c;
pub const MACB_HRB: c_uint = 0x0090;
pub const MACB_HRT: c_uint = 0x0094;
pub const MACB_SA1B: c_uint = 0x0098;
pub const MACB_SA1T: c_uint = 0x009c;
pub const MACB_SA2B: c_uint = 0x00a0;
pub const MACB_SA2T: c_uint = 0x00a4;
pub const MACB_SA3B: c_uint = 0x00a8;
pub const MACB_SA3T: c_uint = 0x00ac;
pub const MACB_SA4B: c_uint = 0x00b0;
pub const MACB_SA4T: c_uint = 0x00b4;
pub const MACB_TID: c_uint = 0x00b8;
pub const MACB_TPQ: c_uint = 0x00bc;
pub const MACB_USRIO: c_uint = 0x00c0;
pub const MACB_WOL: c_uint = 0x00c4;
pub const MACB_MID: c_uint = 0x00fc;
pub const MACB_TBQPH: c_uint = 0x04C8;
pub const MACB_RBQPH: c_uint = 0x04D4;
// GEM register offsets.
pub const GEM_NCR: c_uint = 0x0000 /* Network Control */;
pub const GEM_NCFGR: c_uint = 0x0004 /* Network Config */;
pub const GEM_USRIO: c_uint = 0x000c /* User IO */;
pub const GEM_DMACFG: c_uint = 0x0010 /* DMA Configuration */;
pub const GEM_PBUFRXCUT: c_uint = 0x0044 /* RX Partial Store and Forward */;
pub const GEM_JML: c_uint = 0x0048 /* Jumbo Max Length */;
pub const GEM_HS_MAC_CONFIG: c_uint = 0x0050 /* GEM high speed config */;
pub const GEM_HRB: c_uint = 0x0080 /* Hash Bottom */;
pub const GEM_HRT: c_uint = 0x0084 /* Hash Top */;
pub const GEM_SA1B: c_uint = 0x0088 /* Specific1 Bottom */;
pub const GEM_SA1T: c_uint = 0x008C /* Specific1 Top */;
pub const GEM_SA2B: c_uint = 0x0090 /* Specific2 Bottom */;
pub const GEM_SA2T: c_uint = 0x0094 /* Specific2 Top */;
pub const GEM_SA3B: c_uint = 0x0098 /* Specific3 Bottom */;
pub const GEM_SA3T: c_uint = 0x009C /* Specific3 Top */;
pub const GEM_SA4B: c_uint = 0x00A0 /* Specific4 Bottom */;
pub const GEM_SA4T: c_uint = 0x00A4 /* Specific4 Top */;
pub const GEM_WOL: c_uint = 0x00b8 /* Wake on LAN */;
pub const GEM_RXPTPUNI: c_uint = 0x00D4 /* PTP RX Unicast address */;
pub const GEM_TXPTPUNI: c_uint = 0x00D8 /* PTP TX Unicast address */;
pub const GEM_EFTSH: c_uint = 0x00e8 /* PTP Event Frame Transmitted Seconds Register 47:32 */;
pub const GEM_EFRSH: c_uint = 0x00ec /* PTP Event Frame Received Seconds Register 47:32 */;
pub const GEM_PEFTSH: c_uint = 0x00f0 /* PTP Peer Event Frame Transmitted Seconds Register 47:32 */;
pub const GEM_PEFRSH: c_uint = 0x00f4 /* PTP Peer Event Frame Received Seconds Register 47:32 */;
pub const GEM_OTX: c_uint = 0x0100 /* Octets transmitted */;
pub const GEM_OCTTXL: c_uint = 0x0100 /* Octets transmitted [31:0] */;
pub const GEM_OCTTXH: c_uint = 0x0104 /* Octets transmitted [47:32] */;
pub const GEM_TXCNT: c_uint = 0x0108 /* Frames Transmitted counter */;
pub const GEM_TXBCCNT: c_uint = 0x010c /* Broadcast Frames counter */;
pub const GEM_TXMCCNT: c_uint = 0x0110 /* Multicast Frames counter */;
pub const GEM_TXPAUSECNT: c_uint = 0x0114 /* Pause Frames Transmitted Counter */;
pub const GEM_TX64CNT: c_uint = 0x0118 /* 64 byte Frames TX counter */;
pub const GEM_TX65CNT: c_uint = 0x011c /* 65-127 byte Frames TX counter */;
pub const GEM_TX128CNT: c_uint = 0x0120 /* 128-255 byte Frames TX counter */;
pub const GEM_TX256CNT: c_uint = 0x0124 /* 256-511 byte Frames TX counter */;
pub const GEM_TX512CNT: c_uint = 0x0128 /* 512-1023 byte Frames TX counter */;
pub const GEM_TX1024CNT: c_uint = 0x012c /* 1024-1518 byte Frames TX counter */;
pub const GEM_TX1519CNT: c_uint = 0x0130 /* 1519+ byte Frames TX counter */;
pub const GEM_TXURUNCNT: c_uint = 0x0134 /* TX under run error counter */;
pub const GEM_SNGLCOLLCNT: c_uint = 0x0138 /* Single Collision Frame Counter */;
pub const GEM_MULTICOLLCNT: c_uint = 0x013c /* Multiple Collision Frame Counter */;
pub const GEM_EXCESSCOLLCNT: c_uint = 0x0140 /* Excessive Collision Frame Counter */;
pub const GEM_LATECOLLCNT: c_uint = 0x0144 /* Late Collision Frame Counter */;
pub const GEM_TXDEFERCNT: c_uint = 0x0148 /* Deferred Transmission Frame Counter */;
pub const GEM_TXCSENSECNT: c_uint = 0x014c /* Carrier Sense Error Counter */;
pub const GEM_ORX: c_uint = 0x0150 /* Octets received */;
pub const GEM_OCTRXL: c_uint = 0x0150 /* Octets received [31:0] */;
pub const GEM_OCTRXH: c_uint = 0x0154 /* Octets received [47:32] */;
pub const GEM_RXCNT: c_uint = 0x0158 /* Frames Received Counter */;
pub const GEM_RXBROADCNT: c_uint = 0x015c /* Broadcast Frames Received Counter */;
pub const GEM_RXMULTICNT: c_uint = 0x0160 /* Multicast Frames Received Counter */;
pub const GEM_RXPAUSECNT: c_uint = 0x0164 /* Pause Frames Received Counter */;
pub const GEM_RX64CNT: c_uint = 0x0168 /* 64 byte Frames RX Counter */;
pub const GEM_RX65CNT: c_uint = 0x016c /* 65-127 byte Frames RX Counter */;
pub const GEM_RX128CNT: c_uint = 0x0170 /* 128-255 byte Frames RX Counter */;
pub const GEM_RX256CNT: c_uint = 0x0174 /* 256-511 byte Frames RX Counter */;
pub const GEM_RX512CNT: c_uint = 0x0178 /* 512-1023 byte Frames RX Counter */;
pub const GEM_RX1024CNT: c_uint = 0x017c /* 1024-1518 byte Frames RX Counter */;
pub const GEM_RX1519CNT: c_uint = 0x0180 /* 1519+ byte Frames RX Counter */;
pub const GEM_RXUNDRCNT: c_uint = 0x0184 /* Undersize Frames Received Counter */;
pub const GEM_RXOVRCNT: c_uint = 0x0188 /* Oversize Frames Received Counter */;
pub const GEM_RXJABCNT: c_uint = 0x018c /* Jabbers Received Counter */;
pub const GEM_RXFCSCNT: c_uint = 0x0190 /* Frame Check Sequence Error Counter */;
pub const GEM_RXLENGTHCNT: c_uint = 0x0194 /* Length Field Error Counter */;
pub const GEM_RXSYMBCNT: c_uint = 0x0198 /* Symbol Error Counter */;
pub const GEM_RXALIGNCNT: c_uint = 0x019c /* Alignment Error Counter */;
pub const GEM_RXRESERRCNT: c_uint = 0x01a0 /* Receive Resource Error Counter */;
pub const GEM_RXORCNT: c_uint = 0x01a4 /* Receive Overrun Counter */;
pub const GEM_RXIPCCNT: c_uint = 0x01a8 /* IP header Checksum Error Counter */;
pub const GEM_RXTCPCCNT: c_uint = 0x01ac /* TCP Checksum Error Counter */;
pub const GEM_RXUDPCCNT: c_uint = 0x01b0 /* UDP Checksum Error Counter */;
pub const GEM_TISUBN: c_uint = 0x01bc /* 1588 Timer Increment Sub-ns */;
pub const GEM_TSH: c_uint = 0x01c0 /* 1588 Timer Seconds High */;
pub const GEM_TSL: c_uint = 0x01d0 /* 1588 Timer Seconds Low */;
pub const GEM_TN: c_uint = 0x01d4 /* 1588 Timer Nanoseconds */;
pub const GEM_TA: c_uint = 0x01d8 /* 1588 Timer Adjust */;
pub const GEM_TI: c_uint = 0x01dc /* 1588 Timer Increment */;
pub const GEM_EFTSL: c_uint = 0x01e0 /* PTP Event Frame Tx Seconds Low */;
pub const GEM_EFTN: c_uint = 0x01e4 /* PTP Event Frame Tx Nanoseconds */;
pub const GEM_EFRSL: c_uint = 0x01e8 /* PTP Event Frame Rx Seconds Low */;
pub const GEM_EFRN: c_uint = 0x01ec /* PTP Event Frame Rx Nanoseconds */;
pub const GEM_PEFTSL: c_uint = 0x01f0 /* PTP Peer Event Frame Tx Secs Low */;
pub const GEM_PEFTN: c_uint = 0x01f4 /* PTP Peer Event Frame Tx Ns */;
pub const GEM_PEFRSL: c_uint = 0x01f8 /* PTP Peer Event Frame Rx Sec Low */;
pub const GEM_PEFRN: c_uint = 0x01fc /* PTP Peer Event Frame Rx Ns */;
pub const GEM_PCSCNTRL: c_uint = 0x0200 /* PCS Control */;
pub const GEM_PCSSTS: c_uint = 0x0204 /* PCS Status */;
pub const GEM_PCSPHYTOPID: c_uint = 0x0208 /* PCS PHY Top ID */;
pub const GEM_PCSPHYBOTID: c_uint = 0x020c /* PCS PHY Bottom ID */;
pub const GEM_PCSANADV: c_uint = 0x0210 /* PCS AN Advertisement */;
pub const GEM_PCSANLPBASE: c_uint = 0x0214 /* PCS AN Link Partner Base */;
pub const GEM_PCSANEXP: c_uint = 0x0218 /* PCS AN Expansion */;
pub const GEM_PCSANNPTX: c_uint = 0x021c /* PCS AN Next Page TX */;
pub const GEM_PCSANNPLP: c_uint = 0x0220 /* PCS AN Next Page LP */;
pub const GEM_PCSANEXTSTS: c_uint = 0x023c /* PCS AN Extended Status */;
pub const GEM_RXLPI: c_uint = 0x0270 /* RX LPI Transitions */;
pub const GEM_RXLPITIME: c_uint = 0x0274 /* RX LPI Time */;
pub const GEM_TXLPI: c_uint = 0x0278 /* TX LPI Transitions */;
pub const GEM_TXLPITIME: c_uint = 0x027c /* TX LPI Time */;
pub const GEM_DCFG1: c_uint = 0x0280 /* Design Config 1 */;
pub const GEM_DCFG2: c_uint = 0x0284 /* Design Config 2 */;
pub const GEM_DCFG3: c_uint = 0x0288 /* Design Config 3 */;
pub const GEM_DCFG4: c_uint = 0x028c /* Design Config 4 */;
pub const GEM_DCFG5: c_uint = 0x0290 /* Design Config 5 */;
pub const GEM_DCFG6: c_uint = 0x0294 /* Design Config 6 */;
pub const GEM_DCFG7: c_uint = 0x0298 /* Design Config 7 */;
pub const GEM_DCFG8: c_uint = 0x029C /* Design Config 8 */;
pub const GEM_DCFG10: c_uint = 0x02A4 /* Design Config 10 */;
pub const GEM_DCFG12: c_uint = 0x02AC /* Design Config 12 */;
pub const GEM_ENST_CONTROL: c_uint = 0x0880 /* ENST control register */;
pub const GEM_USX_CONTROL: c_uint = 0x0A80 /* High speed PCS control register */;
pub const GEM_USX_STATUS: c_uint = 0x0A88 /* High speed PCS status register */;
pub const GEM_TXBDCTRL: c_uint = 0x04cc /* TX Buffer Descriptor control register */;
pub const GEM_RXBDCTRL: c_uint = 0x04d0 /* RX Buffer Descriptor control register */;
// Screener Type 2 match registers
pub const GEM_SCRT2: c_uint = 0x540;
// EtherType registers
pub const GEM_ETHT: c_uint = 0x06E0;
// Type 2 compare registers
pub const GEM_T2CMPW0: c_uint = 0x0700;
pub const GEM_T2CMPW1: c_uint = 0x0704;

// type 2 compare registers
// each location requires 3 compare regs
//

// Which screening type 2 EtherType register will be used (0 - 7)
pub const SCRT2_ETHT: c_int = 0;

// Bitfields in ENST_CONTROL
pub const GEM_ENST_DISABLE_QUEUE_OFFSET: c_int = 16;
// Bitfields in NCR

pub const MACB_LB_SIZE: c_int = 1;

pub const MACB_LLB_SIZE: c_int = 1;

pub const MACB_RE_SIZE: c_int = 1;

pub const MACB_TE_SIZE: c_int = 1;

pub const MACB_MPE_SIZE: c_int = 1;

pub const MACB_CLRSTAT_SIZE: c_int = 1;

pub const MACB_INCSTAT_SIZE: c_int = 1;

pub const MACB_WESTAT_SIZE: c_int = 1;

pub const MACB_BP_SIZE: c_int = 1;

pub const MACB_TSTART_SIZE: c_int = 1;

pub const MACB_THALT_SIZE: c_int = 1;

pub const MACB_NCR_TPF_SIZE: c_int = 1;

pub const MACB_TZQ_SIZE: c_int = 1;

pub const MACB_PTPUNI_SIZE: c_int = 1;

pub const MACB_OSSMODE_SIZE: c_int = 1;

pub const MACB_MIIONRGMII_SIZE: c_int = 1;
// Bitfields in NCFGR

pub const MACB_SPD_SIZE: c_int = 1;

pub const MACB_FD_SIZE: c_int = 1;

pub const MACB_BIT_RATE_SIZE: c_int = 1;

pub const MACB_JFRAME_SIZE: c_int = 1;

pub const MACB_CAF_SIZE: c_int = 1;

pub const MACB_NBC_SIZE: c_int = 1;

pub const MACB_NCFGR_MTI_SIZE: c_int = 1;

pub const MACB_UNI_SIZE: c_int = 1;

pub const MACB_BIG_SIZE: c_int = 1;

pub const MACB_EAE_SIZE: c_int = 1;
pub const MACB_CLK_OFFSET: c_int = 10;
pub const MACB_CLK_SIZE: c_int = 2;

pub const MACB_RTY_SIZE: c_int = 1;

pub const MACB_PAE_SIZE: c_int = 1;

pub const MACB_RBOF_SIZE: c_int = 2;

pub const MACB_RLCE_SIZE: c_int = 1;

pub const MACB_DRFCS_SIZE: c_int = 1;
pub const MACB_EFRHD_OFFSET: c_int = 18;
pub const MACB_EFRHD_SIZE: c_int = 1;
pub const MACB_IRXFCS_OFFSET: c_int = 19;
pub const MACB_IRXFCS_SIZE: c_int = 1;
// GEM specific NCR bitfields.
pub const GEM_TXLPIEN_OFFSET: c_int = 19;
pub const GEM_TXLPIEN_SIZE: c_int = 1;
pub const GEM_ENABLE_HS_MAC_OFFSET: c_int = 31;
pub const GEM_ENABLE_HS_MAC_SIZE: c_int = 1;
// GEM specific NCFGR bitfields.

pub const GEM_FD_SIZE: c_int = 1;

pub const GEM_GBE_SIZE: c_int = 1;
pub const GEM_PCSSEL_OFFSET: c_int = 11;
pub const GEM_PCSSEL_SIZE: c_int = 1;

pub const GEM_PAE_SIZE: c_int = 1;

pub const GEM_CLK_SIZE: c_int = 3;

pub const GEM_DBW_SIZE: c_int = 2;
pub const GEM_RXCOEN_OFFSET: c_int = 24;
pub const GEM_RXCOEN_SIZE: c_int = 1;
pub const GEM_SGMIIEN_OFFSET: c_int = 27;
pub const GEM_SGMIIEN_SIZE: c_int = 1;
// Constants for data bus width.

// Bitfields in DMACFG.

pub const GEM_FBLDO_SIZE: c_int = 5;

pub const GEM_ENDIA_DESC_SIZE: c_int = 1;

pub const GEM_ENDIA_PKT_SIZE: c_int = 1;

pub const GEM_RXBMS_SIZE: c_int = 2;

pub const GEM_TXPBMS_SIZE: c_int = 1;

pub const GEM_TXCOEN_SIZE: c_int = 1;

pub const GEM_RXBS_SIZE: c_int = 8;

pub const GEM_DDRP_SIZE: c_int = 1;

pub const GEM_RXEXT_SIZE: c_int = 1;

pub const GEM_TXEXT_SIZE: c_int = 1;

pub const GEM_ADDR64_SIZE: c_int = 1;
// Bitfields in PBUFRXCUT

pub const GEM_ENCUTTHRU_SIZE: c_int = 1;
// Bitfields in NSR

pub const MACB_NSR_LINK_SIZE: c_int = 1;

pub const MACB_MDIO_SIZE: c_int = 1;

pub const MACB_IDLE_SIZE: c_int = 1;
// Bitfields in TSR

pub const MACB_UBR_SIZE: c_int = 1;

pub const MACB_COL_SIZE: c_int = 1;

pub const MACB_TSR_RLE_SIZE: c_int = 1;

pub const MACB_TGO_SIZE: c_int = 1;

pub const MACB_BEX_SIZE: c_int = 1;

pub const MACB_COMP_SIZE: c_int = 1;

pub const MACB_UND_SIZE: c_int = 1;
// Bitfields in RSR

pub const MACB_BNA_SIZE: c_int = 1;

pub const MACB_REC_SIZE: c_int = 1;

pub const MACB_OVR_SIZE: c_int = 1;
// Bitfields in ISR/IER/IDR/IMR

pub const MACB_MFD_SIZE: c_int = 1;

pub const MACB_RCOMP_SIZE: c_int = 1;

pub const MACB_RXUBR_SIZE: c_int = 1;

pub const MACB_TXUBR_SIZE: c_int = 1;

pub const MACB_ISR_TUND_SIZE: c_int = 1;

pub const MACB_ISR_RLE_SIZE: c_int = 1;

pub const MACB_TXERR_SIZE: c_int = 1;

pub const MACB_RM9200_TBRE_SIZE: c_int = 1;

pub const MACB_TCOMP_SIZE: c_int = 1;

pub const MACB_ISR_LINK_SIZE: c_int = 1;

pub const MACB_ISR_ROVR_SIZE: c_int = 1;

pub const MACB_HRESP_SIZE: c_int = 1;

pub const MACB_PFR_SIZE: c_int = 1;

pub const MACB_PTZ_SIZE: c_int = 1;

pub const MACB_WOL_SIZE: c_int = 1;

pub const MACB_DRQFR_SIZE: c_int = 1;

pub const MACB_SFR_SIZE: c_int = 1;

pub const MACB_DRQFT_SIZE: c_int = 1;

pub const MACB_SFT_SIZE: c_int = 1;

pub const MACB_PDRQFR_SIZE: c_int = 1;

pub const MACB_PDRSFR_SIZE: c_int = 1;

pub const MACB_PDRQFT_SIZE: c_int = 1;

pub const MACB_PDRSFT_SIZE: c_int = 1;

pub const MACB_SRI_SIZE: c_int = 1;

pub const GEM_WOL_SIZE: c_int = 1;
// Timer increment fields
pub const MACB_TI_CNS_OFFSET: c_int = 0;
pub const MACB_TI_CNS_SIZE: c_int = 8;
pub const MACB_TI_ACNS_OFFSET: c_int = 8;
pub const MACB_TI_ACNS_SIZE: c_int = 8;
pub const MACB_TI_NIT_OFFSET: c_int = 16;
pub const MACB_TI_NIT_SIZE: c_int = 8;
// Bitfields in MAN

pub const MACB_DATA_SIZE: c_int = 16;

pub const MACB_CODE_SIZE: c_int = 2;

pub const MACB_REGA_SIZE: c_int = 5;

pub const MACB_PHYA_SIZE: c_int = 5;

pub const MACB_RW_SIZE: c_int = 2;

pub const MACB_SOF_SIZE: c_int = 2;
// Bitfields in USRIO (AVR32)
pub const MACB_MII_OFFSET: c_int = 0;
pub const MACB_MII_SIZE: c_int = 1;
pub const MACB_EAM_OFFSET: c_int = 1;
pub const MACB_EAM_SIZE: c_int = 1;
pub const MACB_TX_PAUSE_OFFSET: c_int = 2;
pub const MACB_TX_PAUSE_SIZE: c_int = 1;
pub const MACB_TX_PAUSE_ZERO_OFFSET: c_int = 3;
pub const MACB_TX_PAUSE_ZERO_SIZE: c_int = 1;
// Bitfields in USRIO (AT91)
pub const MACB_RMII_OFFSET: c_int = 0;
pub const MACB_RMII_SIZE: c_int = 1;

pub const GEM_RGMII_SIZE: c_int = 1;
pub const MACB_CLKEN_OFFSET: c_int = 1;
pub const MACB_CLKEN_SIZE: c_int = 1;
// Bitfields in WOL
pub const MACB_IP_OFFSET: c_int = 0;
pub const MACB_IP_SIZE: c_int = 16;
pub const MACB_MAG_OFFSET: c_int = 16;
pub const MACB_MAG_SIZE: c_int = 1;
pub const MACB_ARP_OFFSET: c_int = 17;
pub const MACB_ARP_SIZE: c_int = 1;
pub const MACB_SA1_OFFSET: c_int = 18;
pub const MACB_SA1_SIZE: c_int = 1;
pub const MACB_WOL_MTI_OFFSET: c_int = 19;
pub const MACB_WOL_MTI_SIZE: c_int = 1;
// Bitfields in MID
pub const MACB_IDNUM_OFFSET: c_int = 16;
pub const MACB_IDNUM_SIZE: c_int = 12;
pub const MACB_REV_OFFSET: c_int = 0;
pub const MACB_REV_SIZE: c_int = 16;
// Bitfield in HS_MAC_CONFIG
pub const GEM_HS_MAC_SPEED_OFFSET: c_int = 0;
pub const GEM_HS_MAC_SPEED_SIZE: c_int = 3;
// Bitfields in PCSCNTRL
pub const GEM_PCSAUTONEG_OFFSET: c_int = 12;
pub const GEM_PCSAUTONEG_SIZE: c_int = 1;
// Bitfields in DCFG1.
pub const GEM_IRQCOR_OFFSET: c_int = 23;
pub const GEM_IRQCOR_SIZE: c_int = 1;
pub const GEM_DBWDEF_OFFSET: c_int = 25;
pub const GEM_DBWDEF_SIZE: c_int = 3;
pub const GEM_USERIO_OFFSET: c_int = 9;
pub const GEM_USERIO_SIZE: c_int = 1;
pub const GEM_NO_PCS_OFFSET: c_int = 0;
pub const GEM_NO_PCS_SIZE: c_int = 1;
// Bitfields in DCFG2.
pub const GEM_RX_PKT_BUFF_OFFSET: c_int = 20;
pub const GEM_RX_PKT_BUFF_SIZE: c_int = 1;
pub const GEM_TX_PKT_BUFF_OFFSET: c_int = 21;
pub const GEM_TX_PKT_BUFF_SIZE: c_int = 1;
pub const GEM_RX_PBUF_ADDR_OFFSET: c_int = 22;
pub const GEM_RX_PBUF_ADDR_SIZE: c_int = 4;
// Bitfields in DCFG5.
pub const GEM_TSU_OFFSET: c_int = 8;
pub const GEM_TSU_SIZE: c_int = 1;
// Bitfields in DCFG6.
pub const GEM_PBUF_LSO_OFFSET: c_int = 27;
pub const GEM_PBUF_LSO_SIZE: c_int = 1;
pub const GEM_PBUF_RSC_OFFSET: c_int = 26;
pub const GEM_PBUF_RSC_SIZE: c_int = 1;
pub const GEM_PBUF_CUTTHRU_OFFSET: c_int = 25;
pub const GEM_PBUF_CUTTHRU_SIZE: c_int = 1;
pub const GEM_DAW64_OFFSET: c_int = 23;
pub const GEM_DAW64_SIZE: c_int = 1;
// Bitfields in DCFG8.
pub const GEM_T1SCR_OFFSET: c_int = 24;
pub const GEM_T1SCR_SIZE: c_int = 8;
pub const GEM_T2SCR_OFFSET: c_int = 16;
pub const GEM_T2SCR_SIZE: c_int = 8;
pub const GEM_SCR2ETH_OFFSET: c_int = 8;
pub const GEM_SCR2ETH_SIZE: c_int = 8;
pub const GEM_SCR2CMP_OFFSET: c_int = 0;
pub const GEM_SCR2CMP_SIZE: c_int = 8;
// Bitfields in DCFG10
pub const GEM_TXBD_RDBUFF_OFFSET: c_int = 12;
pub const GEM_TXBD_RDBUFF_SIZE: c_int = 4;
pub const GEM_RXBD_RDBUFF_OFFSET: c_int = 8;
pub const GEM_RXBD_RDBUFF_SIZE: c_int = 4;
// Bitfields in DCFG12.
pub const GEM_HIGH_SPEED_OFFSET: c_int = 26;
pub const GEM_HIGH_SPEED_SIZE: c_int = 1;
// Bitfields in ENST_START_TIME_Qx.
pub const GEM_START_TIME_SEC_OFFSET: c_int = 30;
pub const GEM_START_TIME_SEC_SIZE: c_int = 2;
pub const GEM_START_TIME_NSEC_OFFSET: c_int = 0;
pub const GEM_START_TIME_NSEC_SIZE: c_int = 30;
// Bitfields in ENST_ON_TIME_Qx.
pub const GEM_ON_TIME_OFFSET: c_int = 0;
pub const GEM_ON_TIME_SIZE: c_int = 17;
// Bitfields in ENST_OFF_TIME_Qx.
pub const GEM_OFF_TIME_OFFSET: c_int = 0;
pub const GEM_OFF_TIME_SIZE: c_int = 17;
// Hardware ENST timing registers granularity
pub const ENST_TIME_GRANULARITY_NS: c_int = 8;
// Bitfields in USX_CONTROL.
pub const GEM_USX_CTRL_SPEED_OFFSET: c_int = 14;
pub const GEM_USX_CTRL_SPEED_SIZE: c_int = 3;
pub const GEM_SERDES_RATE_OFFSET: c_int = 12;
pub const GEM_SERDES_RATE_SIZE: c_int = 2;
pub const GEM_RX_SCR_BYPASS_OFFSET: c_int = 9;
pub const GEM_RX_SCR_BYPASS_SIZE: c_int = 1;
pub const GEM_TX_SCR_BYPASS_OFFSET: c_int = 8;
pub const GEM_TX_SCR_BYPASS_SIZE: c_int = 1;
pub const GEM_TX_EN_OFFSET: c_int = 1;
pub const GEM_TX_EN_SIZE: c_int = 1;
pub const GEM_SIGNAL_OK_OFFSET: c_int = 0;
pub const GEM_SIGNAL_OK_SIZE: c_int = 1;
// Bitfields in USX_STATUS.
pub const GEM_USX_BLOCK_LOCK_OFFSET: c_int = 0;
pub const GEM_USX_BLOCK_LOCK_SIZE: c_int = 1;
// Bitfields in TISUBN
pub const GEM_SUBNSINCR_OFFSET: c_int = 0;
pub const GEM_SUBNSINCRL_OFFSET: c_int = 24;
pub const GEM_SUBNSINCRL_SIZE: c_int = 8;
pub const GEM_SUBNSINCRH_OFFSET: c_int = 0;
pub const GEM_SUBNSINCRH_SIZE: c_int = 16;
pub const GEM_SUBNSINCR_SIZE: c_int = 24;
// Bitfields in TI
pub const GEM_NSINCR_OFFSET: c_int = 0;
pub const GEM_NSINCR_SIZE: c_int = 8;
// Bitfields in TSH

pub const GEM_TSH_SIZE: c_int = 16;
// Bitfields in TSL

pub const GEM_TSL_SIZE: c_int = 32;
// Bitfields in TN

pub const GEM_TN_SIZE: c_int = 30;
// Bitfields in TXBDCTRL

pub const GEM_TXTSMODE_SIZE: c_int = 2;
// Bitfields in RXBDCTRL

pub const GEM_RXTSMODE_SIZE: c_int = 2;
// Bitfields in SCRT2

pub const GEM_QUEUE_SIZE: c_int = 4;

pub const GEM_VLANPR_SIZE: c_int = 3;

pub const GEM_VLANEN_SIZE: c_int = 1;

pub const GEM_ETHT2IDX_SIZE: c_int = 3;

pub const GEM_ETHTEN_SIZE: c_int = 1;

pub const GEM_CMPA_SIZE: c_int = 5;

pub const GEM_CMPAEN_SIZE: c_int = 1;

pub const GEM_CMPB_SIZE: c_int = 5;

pub const GEM_CMPBEN_SIZE: c_int = 1;

pub const GEM_CMPC_SIZE: c_int = 5;

pub const GEM_CMPCEN_SIZE: c_int = 1;
// Bitfields in ETHT

pub const GEM_ETHTCMP_SIZE: c_int = 16;
// Bitfields in T2CMPW0

pub const GEM_T2CMP_SIZE: c_int = 16;

pub const GEM_T2MASK_SIZE: c_int = 16;
// Bitfields in T2CMPW1

pub const GEM_T2DISMSK_SIZE: c_int = 1;

pub const GEM_T2CMPOFST_SIZE: c_int = 2;

pub const GEM_T2OFST_SIZE: c_int = 7;
// Bitfields in queue pointer registers

pub const MACB_QUEUE_DISABLE_SIZE: c_int = 1;
// Offset for screener type 2 compare values (T2CMPOFST).
// Note the offset is applied after the specified point,
// e.g. GEM_T2COMPOFST_ETYPE denotes the EtherType field, so an offset
// of 12 bytes from this would be the source IP address in an IP header
//
pub const GEM_T2COMPOFST_SOF: c_int = 0;
pub const GEM_T2COMPOFST_ETYPE: c_int = 1;
pub const GEM_T2COMPOFST_IPHDR: c_int = 2;
pub const GEM_T2COMPOFST_TCPUDP: c_int = 3;
// offset from EtherType to IP address
pub const ETYPE_SRCIP_OFFSET: c_int = 12;
pub const ETYPE_DSTIP_OFFSET: c_int = 16;
// offset from IP header to port
pub const IPHDR_SRCPORT_OFFSET: c_int = 0;
pub const IPHDR_DSTPORT_OFFSET: c_int = 2;
// Transmit DMA buffer descriptor Word 1

pub const GEM_DMA_TXVALID_SIZE: c_int = 1;
// Receive DMA buffer descriptor Word 0

pub const GEM_DMA_RXVALID_SIZE: c_int = 1;
// DMA buffer descriptor Word 2 (32 bit addressing) or Word 4 (64 bit addressing)

pub const GEM_DMA_SECL_SIZE: c_int = 2;

pub const GEM_DMA_NSEC_SIZE: c_int = 30;
// DMA buffer descriptor Word 3 (32 bit addressing) or Word 5 (64 bit addressing)
// New hardware supports 12 bit precision of timestamp in DMA buffer descriptor.
// Old hardware supports only 6 bit precision but it is enough for PTP.
// Less accuracy is used always instead of checking hardware version.
//

pub const GEM_DMA_SECH_SIZE: c_int = 4;

// Bitfields in ADJ
pub const GEM_ADDSUB_OFFSET: c_int = 31;
pub const GEM_ADDSUB_SIZE: c_int = 1;
// Constants for CLK
pub const MACB_CLK_DIV8: c_int = 0;
pub const MACB_CLK_DIV16: c_int = 1;
pub const MACB_CLK_DIV32: c_int = 2;
pub const MACB_CLK_DIV64: c_int = 3;
// GEM specific constants for CLK.
pub const GEM_CLK_DIV8: c_int = 0;
pub const GEM_CLK_DIV16: c_int = 1;
pub const GEM_CLK_DIV32: c_int = 2;
pub const GEM_CLK_DIV48: c_int = 3;
pub const GEM_CLK_DIV64: c_int = 4;
pub const GEM_CLK_DIV96: c_int = 5;
pub const GEM_CLK_DIV128: c_int = 6;
pub const GEM_CLK_DIV224: c_int = 7;
// Constants for MAN register
pub const MACB_MAN_C22_SOF: c_int = 1;
pub const MACB_MAN_C22_WRITE: c_int = 1;
pub const MACB_MAN_C22_READ: c_int = 2;
pub const MACB_MAN_C22_CODE: c_int = 2;
pub const MACB_MAN_C45_SOF: c_int = 0;
pub const MACB_MAN_C45_ADDR: c_int = 0;
pub const MACB_MAN_C45_WRITE: c_int = 1;
pub const MACB_MAN_C45_POST_READ_INCR: c_int = 2;
pub const MACB_MAN_C45_READ: c_int = 3;
pub const MACB_MAN_C45_CODE: c_int = 2;
// Capability mask bits

// LSO settings
pub const MACB_LSO_UFO_ENABLE: c_uint = 0x01;
pub const MACB_LSO_TSO_ENABLE: c_uint = 0x02;
// Bit manipulation macros

// Macro flag: #define MACB_BFEXT(name,value)\

// Register access macros

// Conditional GEM/MACB macros.  These perform the operation to the correct
// register dependent on whether the device is a GEM or a MACB.  For registers
// and bitfields that are common across both devices, use macb_{read,write}l
// to avoid the cost of the conditional.
//

// struct macb_dma_desc - Hardware DMA descriptor
// @addr: DMA address of data buffer
// @ctrl: Control and status bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_dma_desc {
    pub addr: u32,
    pub ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_dma_desc_64 {
    pub addrh: u32,
    pub resvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_dma_desc_ptp {
    pub ts_1: u32,
    pub ts_2: u32,
}

// DMA descriptor bitfields
pub const MACB_RX_USED_OFFSET: c_int = 0;
pub const MACB_RX_USED_SIZE: c_int = 1;
pub const MACB_RX_WRAP_OFFSET: c_int = 1;
pub const MACB_RX_WRAP_SIZE: c_int = 1;
pub const MACB_RX_WADDR_OFFSET: c_int = 2;
pub const MACB_RX_WADDR_SIZE: c_int = 30;
pub const MACB_RX_FRMLEN_OFFSET: c_int = 0;
pub const MACB_RX_FRMLEN_SIZE: c_int = 12;
pub const MACB_RX_OFFSET_OFFSET: c_int = 12;
pub const MACB_RX_OFFSET_SIZE: c_int = 2;
pub const MACB_RX_SOF_OFFSET: c_int = 14;
pub const MACB_RX_SOF_SIZE: c_int = 1;
pub const MACB_RX_EOF_OFFSET: c_int = 15;
pub const MACB_RX_EOF_SIZE: c_int = 1;
pub const MACB_RX_CFI_OFFSET: c_int = 16;
pub const MACB_RX_CFI_SIZE: c_int = 1;
pub const MACB_RX_VLAN_PRI_OFFSET: c_int = 17;
pub const MACB_RX_VLAN_PRI_SIZE: c_int = 3;
pub const MACB_RX_PRI_TAG_OFFSET: c_int = 20;
pub const MACB_RX_PRI_TAG_SIZE: c_int = 1;
pub const MACB_RX_VLAN_TAG_OFFSET: c_int = 21;
pub const MACB_RX_VLAN_TAG_SIZE: c_int = 1;
pub const MACB_RX_TYPEID_MATCH_OFFSET: c_int = 22;
pub const MACB_RX_TYPEID_MATCH_SIZE: c_int = 1;
pub const MACB_RX_SA4_MATCH_OFFSET: c_int = 23;
pub const MACB_RX_SA4_MATCH_SIZE: c_int = 1;
pub const MACB_RX_SA3_MATCH_OFFSET: c_int = 24;
pub const MACB_RX_SA3_MATCH_SIZE: c_int = 1;
pub const MACB_RX_SA2_MATCH_OFFSET: c_int = 25;
pub const MACB_RX_SA2_MATCH_SIZE: c_int = 1;
pub const MACB_RX_SA1_MATCH_OFFSET: c_int = 26;
pub const MACB_RX_SA1_MATCH_SIZE: c_int = 1;
pub const MACB_RX_EXT_MATCH_OFFSET: c_int = 28;
pub const MACB_RX_EXT_MATCH_SIZE: c_int = 1;
pub const MACB_RX_UHASH_MATCH_OFFSET: c_int = 29;
pub const MACB_RX_UHASH_MATCH_SIZE: c_int = 1;
pub const MACB_RX_MHASH_MATCH_OFFSET: c_int = 30;
pub const MACB_RX_MHASH_MATCH_SIZE: c_int = 1;
pub const MACB_RX_BROADCAST_OFFSET: c_int = 31;
pub const MACB_RX_BROADCAST_SIZE: c_int = 1;
pub const MACB_RX_FRMLEN_MASK: c_uint = 0xFFF;
pub const MACB_RX_JFRMLEN_MASK: c_uint = 0x3FFF;
// RX checksum offload disabled: bit 24 clear in NCFGR
pub const GEM_RX_TYPEID_MATCH_OFFSET: c_int = 22;
pub const GEM_RX_TYPEID_MATCH_SIZE: c_int = 2;
// RX checksum offload enabled: bit 24 set in NCFGR
pub const GEM_RX_CSUM_OFFSET: c_int = 22;
pub const GEM_RX_CSUM_SIZE: c_int = 2;
pub const MACB_TX_FRMLEN_OFFSET: c_int = 0;
pub const MACB_TX_FRMLEN_SIZE: c_int = 11;
pub const MACB_TX_LAST_OFFSET: c_int = 15;
pub const MACB_TX_LAST_SIZE: c_int = 1;
pub const MACB_TX_NOCRC_OFFSET: c_int = 16;
pub const MACB_TX_NOCRC_SIZE: c_int = 1;
pub const MACB_MSS_MFS_OFFSET: c_int = 16;
pub const MACB_MSS_MFS_SIZE: c_int = 14;
pub const MACB_TX_LSO_OFFSET: c_int = 17;
pub const MACB_TX_LSO_SIZE: c_int = 2;
pub const MACB_TX_TCP_SEQ_SRC_OFFSET: c_int = 19;
pub const MACB_TX_TCP_SEQ_SRC_SIZE: c_int = 1;
pub const MACB_TX_BUF_EXHAUSTED_OFFSET: c_int = 27;
pub const MACB_TX_BUF_EXHAUSTED_SIZE: c_int = 1;
pub const MACB_TX_UNDERRUN_OFFSET: c_int = 28;
pub const MACB_TX_UNDERRUN_SIZE: c_int = 1;
pub const MACB_TX_ERROR_OFFSET: c_int = 29;
pub const MACB_TX_ERROR_SIZE: c_int = 1;
pub const MACB_TX_WRAP_OFFSET: c_int = 30;
pub const MACB_TX_WRAP_SIZE: c_int = 1;
pub const MACB_TX_USED_OFFSET: c_int = 31;
pub const MACB_TX_USED_SIZE: c_int = 1;
pub const GEM_TX_FRMLEN_OFFSET: c_int = 0;
pub const GEM_TX_FRMLEN_SIZE: c_int = 14;
// Buffer descriptor constants
pub const GEM_RX_CSUM_NONE: c_int = 0;
pub const GEM_RX_CSUM_IP_ONLY: c_int = 1;
pub const GEM_RX_CSUM_IP_TCP: c_int = 2;
pub const GEM_RX_CSUM_IP_UDP: c_int = 3;
// limit RX checksum offload to TCP and UDP packets
pub const GEM_RX_CSUM_CHECKED_MASK: c_int = 2;
// Scaled PPM fraction
pub const PPM_FRACTION: c_int = 16;
// struct macb_tx_skb - data about an skb which is being transmitted
// @skb: skb currently being transmitted, only set for the last buffer
// of the frame
// @mapping: DMA address of the skb's fragment buffer
// @size: size of the DMA mapped buffer
// @fcs_len: FCS bytes appended in software, 0 or ETH_FCS_LEN, only
// set for the last buffer of the frame
// @mapped_as_page: true when buffer was mapped with skb_frag_dma_map(),
// false when buffer was mapped with dma_map_single()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_tx_skb {
    pub skb: *mut sk_buff,
    pub mapping: dma_addr_t,
    pub size: usize,
    pub fcs_len: u8,
    pub mapped_as_page: bool,
}

// Hardware-collected statistics. Used when updating the network
// device stats by a periodic timer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_stats {
    pub rx_pause_frames: u64,
    pub tx_ok: u64,
    pub tx_single_cols: u64,
    pub tx_multiple_cols: u64,
    pub rx_ok: u64,
    pub rx_fcs_errors: u64,
    pub rx_align_errors: u64,
    pub tx_deferred: u64,
    pub tx_late_cols: u64,
    pub tx_excessive_cols: u64,
    pub tx_underruns: u64,
    pub tx_carrier_errors: u64,
    pub rx_resource_errors: u64,
    pub rx_overruns: u64,
    pub rx_symbol_errors: u64,
    pub rx_oversize_pkts: u64,
    pub rx_jabbers: u64,
    pub rx_undersize_pkts: u64,
    pub sqe_test_errors: u64,
    pub rx_length_mismatch: u64,
    pub tx_pause_frames: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_stats {
    pub tx_octets: u64,
    pub tx_frames: u64,
    pub tx_broadcast_frames: u64,
    pub tx_multicast_frames: u64,
    pub tx_pause_frames: u64,
    pub tx_64_byte_frames: u64,
    pub tx_65_127_byte_frames: u64,
    pub tx_128_255_byte_frames: u64,
    pub tx_256_511_byte_frames: u64,
    pub tx_512_1023_byte_frames: u64,
    pub tx_1024_1518_byte_frames: u64,
    pub tx_greater_than_1518_byte_frames: u64,
    pub tx_underrun: u64,
    pub tx_single_collision_frames: u64,
    pub tx_multiple_collision_frames: u64,
    pub tx_excessive_collisions: u64,
    pub tx_late_collisions: u64,
    pub tx_deferred_frames: u64,
    pub tx_carrier_sense_errors: u64,
    pub rx_octets: u64,
    pub rx_frames: u64,
    pub rx_broadcast_frames: u64,
    pub rx_multicast_frames: u64,
    pub rx_pause_frames: u64,
    pub rx_64_byte_frames: u64,
    pub rx_65_127_byte_frames: u64,
    pub rx_128_255_byte_frames: u64,
    pub rx_256_511_byte_frames: u64,
    pub rx_512_1023_byte_frames: u64,
    pub rx_1024_1518_byte_frames: u64,
    pub rx_greater_than_1518_byte_frames: u64,
    pub rx_undersized_frames: u64,
    pub rx_oversize_frames: u64,
    pub rx_jabbers: u64,
    pub rx_frame_check_sequence_errors: u64,
    pub rx_length_field_frame_errors: u64,
    pub rx_symbol_errors: u64,
    pub rx_alignment_errors: u64,
    pub rx_resource_errors: u64,
    pub rx_overruns: u64,
    pub rx_ip_header_checksum_errors: u64,
    pub rx_tcp_checksum_errors: u64,
    pub rx_udp_checksum_errors: u64,
    pub rx_lpi_transitions: u64,
    pub rx_lpi_time: u64,
    pub tx_lpi_transitions: u64,
    pub tx_lpi_time: u64,
}

// Describes the name and offset of an individual statistic register, as
// returned by `ethtool -S`. Also describes which net_device_stats statistics
// this register should contribute to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_statistic {
    pub __nonstring: char stat_string[ETH_GSTRING_LEN],
    pub offset: c_int,
    pub stat_bits: u32,
}

// Bitfield defs for net_device_stat statistics
pub const GEM_NDS_RXERR_OFFSET: c_int = 0;
pub const GEM_NDS_RXLENERR_OFFSET: c_int = 1;
pub const GEM_NDS_RXOVERERR_OFFSET: c_int = 2;
pub const GEM_NDS_RXCRCERR_OFFSET: c_int = 3;
pub const GEM_NDS_RXFRAMEERR_OFFSET: c_int = 4;
pub const GEM_NDS_RXFIFOERR_OFFSET: c_int = 5;
pub const GEM_NDS_TXERR_OFFSET: c_int = 6;
pub const GEM_NDS_TXABORTEDERR_OFFSET: c_int = 7;
pub const GEM_NDS_TXCARRIERERR_OFFSET: c_int = 8;
pub const GEM_NDS_TXFIFOERR_OFFSET: c_int = 9;
pub const GEM_NDS_COLLISIONS_OFFSET: c_int = 10;

// list of gem statistic registers. The names MUST match the
// corresponding GEM_* definitions.
//

// per queue statistics, each should be unsigned long type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_stats {
    pub first: c_ulong,
    pub rx_packets: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_or_gem_ops {
    pub bp): *mut *mut int (mog_alloc_rx_buffers)(struct macb,
    pub bp): *mut *mut void (mog_free_rx_buffers)(struct macb,
    pub bp): *mut *mut void (mog_init_rings)(struct macb,
    pub budget): c_int,
}

// MACB-PTP interface: adapt to platform needs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_ptp_info {
    pub netdev): *mut *mut void (ptp_init)(struct net_device,
    pub netdev): *mut *mut void (ptp_remove)(struct net_device,
    pub (*get_ptp_max_adj)(void): *mut i32,
    pub bp): *mut *mut unsigned int (get_tsu_rate)(struct macb,
    pub info): *mut kernel_ethtool_ts_info,
    pub tstamp_config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_pm_data {
    pub scrt2: u32,
    pub usrio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_usrio_config {
    pub mii: u32,
    pub rmii: u32,
    pub rgmii: u32,
    pub refclk: u32,
    pub clken: u32,
    pub hdfctlen: u32,
    pub tsu_source: u32,
    pub refclk_default_external: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_config {
    pub caps: u32,
    pub dma_burst_length: c_uint,
    pub tsu_clk): *mut *mut *mut clk rx_clk, clk,
    pub pdev): *mut *mut int (init)(struct platform_device,
    pub max_tx_length: c_uint,
    pub jumbo_max_len: c_int,
    pub usrio: *const macb_usrio_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsu_incr {
    pub sub_ns: u32,
    pub ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_queue {
    pub bp: *mut macb,
    pub irq: c_int,
    pub ISR: c_uint,
    pub IER: c_uint,
    pub IDR: c_uint,
    pub IMR: c_uint,
    pub TBQP: c_uint,
    pub RBQS: c_uint,
    pub RBQP: c_uint,
// ENST register offsets for this queue
    pub ENST_START_TIME: c_uint,
    pub ENST_ON_TIME: c_uint,
    pub ENST_OFF_TIME: c_uint,
// Lock to protect tx_head and tx_tail
    pub tx_ptr_lock: spinlock_t,
    pub tx_tail: unsigned int tx_head,,
    pub tx_ring: *mut macb_dma_desc,
    pub tx_skb: *mut macb_tx_skb,
    pub tx_ring_dma: dma_addr_t,
    pub tx_error_task: work_struct,
    pub txubr_pending: bool,
    pub napi_tx: napi_struct,
    pub rx_ring_dma: dma_addr_t,
    pub rx_buffers_dma: dma_addr_t,
    pub rx_tail: c_uint,
    pub rx_prepared_head: c_uint,
    pub rx_ring: *mut macb_dma_desc,
    pub rx_skbuff: *mut sk_buff,
    pub rx_buffers: *mut c_void,
    pub napi_rx: napi_struct,
    pub stats: queue_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rx_fs_item {
    pub fs: ethtool_rx_flow_spec,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rx_fs_list {
    pub list: list_head,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb {
    pub regs: *mut void __iomem,
    pub native_io: bool,
// hardware IO accessors
    pub offset): *mut *mut *mut u32 (macb_reg_readl)(struct macb bp, int,
    pub value): *mut *mut *mut void (macb_reg_writel)(struct macb bp, int offset, u32,
    pub rx_ring_tieoff: *mut macb_dma_desc,
    pub rx_ring_tieoff_dma: dma_addr_t,
    pub rx_buffer_size: usize,
    pub rx_ring_size: c_uint,
    pub tx_ring_size: c_uint,
    pub num_queues: c_uint,
    pub queues: [macb_queue; MACB_MAX_QUEUES],
    pub lock: spinlock_t,
    pub pdev: *mut platform_device,
    pub pclk: *mut clk,
    pub hclk: *mut clk,
    pub tx_clk: *mut clk,
    pub rx_clk: *mut clk,
    pub tsu_clk: *mut clk,
    pub netdev: *mut net_device,
// Protects hw_stats and ethtool_stats
    pub stats_lock: spinlock_t,
    pub macb: macb_stats,
    pub gem: gem_stats,
    pub hw_stats: },
    pub macbgem_ops: macb_or_gem_ops,
    pub mii_bus: *mut mii_bus,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub phylink_usx_pcs: phylink_pcs,
    pub phylink_sgmii_pcs: phylink_pcs,
    pub caps: u32,
    pub dma_burst_length: c_uint,
    pub phy_interface: phy_interface_t,
// AT91RM9200 transmit queue (1 on wire + 1 queued)
    pub rm9200_txq: [macb_tx_skb; 2],
    pub max_tx_length: c_uint,
    pub MACB_MAX_QUEUES]: *mut *mut u64 ethtool_stats[GEM_STATS_LEN + QUEUE_STATS_LEN,
    pub rx_frm_len_mask: c_uint,
    pub jumbo_max_len: c_uint,
    pub wol: u32,
    pub wolopts: u32,
// holds value of rx watermark value for pbuf_rxcutthru register
    pub rx_watermark: u32,
    pub /: *mut *mut *mut macb_ptp_info ptp_info; / macb-ptp interface,
    pub phy: *mut phy,
    pub /: *mut *mut spinlock_t tsu_clk_lock; / gem tsu clock locking,
    pub tsu_rate: c_uint,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub tsu_incr: tsu_incr,
    pub tstamp_config: kernel_hwtstamp_config,
// RX queue filer rule set
    pub rx_fs_list: ethtool_rx_fs_list,
    pub rx_fs_lock: spinlock_t,
    pub max_tuples: c_uint,
    pub hresp_err_bh_work: work_struct,
// EEE / LPI state
    pub eee_active: bool,
    pub tx_lpi_work: delayed_work,
    pub tx_lpi_timer: u32,
    pub rx_bd_rd_prefetch: c_int,
    pub tx_bd_rd_prefetch: c_int,
    pub rx_intr_mask: u32,
    pub pm_data: macb_pm_data,
    pub usrio: *const macb_usrio_config,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macb_bd_control {
    TSTAMP_DISABLED,
    TSTAMP_FRAME_PTP_EVENT_ONLY,
    TSTAMP_ALL_PTP_FRAMES,
    TSTAMP_ALL_FRAMES,
}

extern "C" {
    pub fn gem_ptp_init(netdev: *mut net_device);
}
extern "C" {
    pub fn gem_ptp_remove(netdev: *mut net_device);
}
extern "C" {
    pub fn gem_ptp_txstamp(bp: *mut macb, skb: *mut sk_buff, desc: *mut macb_dma_desc);
}
extern "C" {
    pub fn gem_ptp_rxstamp(bp: *mut macb, skb: *mut sk_buff, desc: *mut macb_dma_desc);
}

extern "C" {
    pub fn IS_ENABLED(MACB_CAPS_GEM_HAS_PTP: CONFIG_MACB_USE_HWSTAMP) && (bp->caps &) -> return;
}
// ENST Helper functions
//
// struct macb_platform_data - platform data for MACB Ethernet used for PCI registration
// @pclk:		platform clock
// @hclk:		AHB clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_platform_data {
    pub pclk: *mut clk,
    pub hclk: *mut clk,
}

//
// struct macb_queue_enst_config - Configuration for Enhanced Scheduled Traffic
// @start_time_mask:  Bitmask representing the start time for the queue
// @on_time_bytes:    "on" time nsec expressed in bytes
// @off_time_bytes:   "off" time nsec expressed in bytes
// @queue_id:         Identifier for the queue
//
// This structure holds the configuration parameters for an ENST queue,
// used to control time-based transmission scheduling in the MACB driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macb_queue_enst_config {
    pub start_time_mask: u32,
    pub on_time_bytes: u32,
    pub off_time_bytes: u32,
    pub queue_id: u8,
}
