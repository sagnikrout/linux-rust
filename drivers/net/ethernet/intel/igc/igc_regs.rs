//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_regs.h
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
// Copyright (c)  2018 Intel Corporation
// General Register Descriptions
pub const IGC_CTRL: c_uint = 0x00000  /* Device Control - RW */;
pub const IGC_STATUS: c_uint = 0x00008  /* Device Status - RO */;
pub const IGC_EECD: c_uint = 0x00010  /* EEPROM/Flash Control - RW */;
pub const IGC_CTRL_EXT: c_uint = 0x00018  /* Extended Device Control - RW */;
pub const IGC_MDIC: c_uint = 0x00020  /* MDI Control - RW */;
pub const IGC_CONNSW: c_uint = 0x00034  /* Copper/Fiber switch control - RW */;
pub const IGC_VET: c_uint = 0x00038  /* VLAN Ether Type - RW */;
pub const IGC_LEDCTL: c_uint = 0x00E00	 /* LED Control - RW */;
pub const IGC_I225_PHPM: c_uint = 0x00E14  /* I225 PHY Power Management */;
pub const IGC_GPHY_VERSION: c_uint = 0x0001E  /* I225 gPHY Firmware Version */;
// Internal Packet Buffer Size Registers
pub const IGC_RXPBS: c_uint = 0x02404  /* Rx Packet Buffer Size - RW */;
pub const IGC_TXPBS: c_uint = 0x03404  /* Tx Packet Buffer Size - RW */;
// NVM  Register Descriptions
pub const IGC_EERD: c_uint = 0x12014  /* EEprom mode read - RW */;
pub const IGC_EEWR: c_uint = 0x12018  /* EEprom mode write - RW */;
// Flow Control Register Descriptions
pub const IGC_FCAL: c_uint = 0x00028  /* FC Address Low - RW */;
pub const IGC_FCAH: c_uint = 0x0002C  /* FC Address High - RW */;
pub const IGC_FCT: c_uint = 0x00030  /* FC Type - RW */;
pub const IGC_FCTTV: c_uint = 0x00170  /* FC Transmit Timer - RW */;
pub const IGC_FCRTL: c_uint = 0x02160  /* FC Receive Threshold Low - RW */;
pub const IGC_FCRTH: c_uint = 0x02168  /* FC Receive Threshold High - RW */;
pub const IGC_FCRTV: c_uint = 0x02460  /* FC Refresh Timer Value - RW */;
// Semaphore registers
pub const IGC_SW_FW_SYNC: c_uint = 0x05B5C  /* SW-FW Synchronization - RW */;
pub const IGC_SWSM: c_uint = 0x05B50  /* SW Semaphore */;
pub const IGC_FWSM: c_uint = 0x05B54  /* FW Semaphore */;
// Function Active and Power State to MNG
pub const IGC_FACTPS: c_uint = 0x05B30;
// Interrupt Register Description
pub const IGC_EICR: c_uint = 0x01580  /* Ext. Interrupt Cause read - W0 */;
pub const IGC_EICS: c_uint = 0x01520  /* Ext. Interrupt Cause Set - W0 */;
pub const IGC_EIMS: c_uint = 0x01524  /* Ext. Interrupt Mask Set/Read - RW */;
pub const IGC_EIMC: c_uint = 0x01528  /* Ext. Interrupt Mask Clear - WO */;
pub const IGC_EIAC: c_uint = 0x0152C  /* Ext. Interrupt Auto Clear - RW */;
pub const IGC_EIAM: c_uint = 0x01530  /* Ext. Interrupt Auto Mask - RW */;
pub const IGC_ICR: c_uint = 0x01500  /* Intr Cause Read - RC/W1C */;
pub const IGC_ICS: c_uint = 0x01504  /* Intr Cause Set - WO */;
pub const IGC_IMS: c_uint = 0x01508  /* Intr Mask Set/Read - RW */;
pub const IGC_IMC: c_uint = 0x0150C  /* Intr Mask Clear - WO */;
pub const IGC_IAM: c_uint = 0x01510  /* Intr Ack Auto Mask- RW */;
// Intr Throttle - RW

// Interrupt Vector Allocation - RW
pub const IGC_IVAR0: c_uint = 0x01700;
pub const IGC_IVAR_MISC: c_uint = 0x01740  /* IVAR for "other" causes - RW */;
pub const IGC_GPIE: c_uint = 0x01514  /* General Purpose Intr Enable - RW */;
// RSS registers
pub const IGC_MRQC: c_uint = 0x05818 /* Multiple Receive Control - RW */;
// Filtering Registers

pub const IGC_FHFTSL: c_uint = 0x05804 /* Flex Filter indirect table select */;
// ETQF register bit definitions

pub const IGC_ETQF_QUEUE_SHIFT: c_int = 16;
pub const IGC_ETQF_QUEUE_MASK: c_uint = 0x00070000;
pub const IGC_ETQF_ETYPE_MASK: c_uint = 0x0000FFFF;
// FHFT register bit definitions

pub const IGC_FHFT_QUEUE_SHIFT: c_int = 8;

pub const IGC_FHFT_PRIO_SHIFT: c_int = 16;

// FHFTSL register bit definitions
pub const IGC_FHFTSL_FTSL_SHIFT: c_int = 0;

// Redirection Table - RW Array

// RSS Random Key - RW Array

// Receive Register Descriptions
pub const IGC_RCTL: c_uint = 0x00100  /* Rx Control - RW */;

pub const IGC_RXCSUM: c_uint = 0x05000  /* Rx Checksum Control - RW */;
pub const IGC_RLPML: c_uint = 0x05004  /* Rx Long Packet Max Length */;
pub const IGC_RFCTL: c_uint = 0x05008  /* Receive Filter Control*/;
pub const IGC_MTA: c_uint = 0x05200  /* Multicast Table Array - RW Array */;
pub const IGC_RA: c_uint = 0x05400  /* Receive Address - RW Array */;
pub const IGC_UTA: c_uint = 0x0A000  /* Unicast Table Array - RW */;

pub const IGC_VLANPQF: c_uint = 0x055B0  /* VLAN Priority Queue Filter - RW */;
// Transmit Register Descriptions
pub const IGC_TCTL: c_uint = 0x00400  /* Tx Control - RW */;
pub const IGC_TIPG: c_uint = 0x00410  /* Tx Inter-packet gap - RW */;

// MMD Register Descriptions

// Statistics Register Descriptions
pub const IGC_CRCERRS: c_uint = 0x04000  /* CRC Error Count - R/clr */;
pub const IGC_ALGNERRC: c_uint = 0x04004  /* Alignment Error Count - R/clr */;
pub const IGC_RXERRC: c_uint = 0x0400C  /* Receive Error Count - R/clr */;
pub const IGC_MPC: c_uint = 0x04010  /* Missed Packet Count - R/clr */;
pub const IGC_SCC: c_uint = 0x04014  /* Single Collision Count - R/clr */;
pub const IGC_ECOL: c_uint = 0x04018  /* Excessive Collision Count - R/clr */;
pub const IGC_MCC: c_uint = 0x0401C  /* Multiple Collision Count - R/clr */;
pub const IGC_LATECOL: c_uint = 0x04020  /* Late Collision Count - R/clr */;
pub const IGC_COLC: c_uint = 0x04028  /* Collision Count - R/clr */;
pub const IGC_RERC: c_uint = 0x0402C  /* Receive Error Count - R/clr */;
pub const IGC_DC: c_uint = 0x04030  /* Defer Count - R/clr */;
pub const IGC_TNCRS: c_uint = 0x04034  /* Tx-No CRS - R/clr */;
pub const IGC_HTDPMC: c_uint = 0x0403C  /* Host Transmit Discarded by MAC - R/clr */;
pub const IGC_RLEC: c_uint = 0x04040  /* Receive Length Error Count - R/clr */;
pub const IGC_XONRXC: c_uint = 0x04048  /* XON Rx Count - R/clr */;
pub const IGC_XONTXC: c_uint = 0x0404C  /* XON Tx Count - R/clr */;
pub const IGC_XOFFRXC: c_uint = 0x04050  /* XOFF Rx Count - R/clr */;
pub const IGC_XOFFTXC: c_uint = 0x04054  /* XOFF Tx Count - R/clr */;
pub const IGC_FCRUC: c_uint = 0x04058  /* Flow Control Rx Unsupported Count- R/clr */;
pub const IGC_PRC64: c_uint = 0x0405C  /* Packets Rx (64 bytes) - R/clr */;
pub const IGC_PRC127: c_uint = 0x04060  /* Packets Rx (65-127 bytes) - R/clr */;
pub const IGC_PRC255: c_uint = 0x04064  /* Packets Rx (128-255 bytes) - R/clr */;
pub const IGC_PRC511: c_uint = 0x04068  /* Packets Rx (255-511 bytes) - R/clr */;
pub const IGC_PRC1023: c_uint = 0x0406C  /* Packets Rx (512-1023 bytes) - R/clr */;
pub const IGC_PRC1522: c_uint = 0x04070  /* Packets Rx (1024-1522 bytes) - R/clr */;
pub const IGC_GPRC: c_uint = 0x04074  /* Good Packets Rx Count - R/clr */;
pub const IGC_BPRC: c_uint = 0x04078  /* Broadcast Packets Rx Count - R/clr */;
pub const IGC_MPRC: c_uint = 0x0407C  /* Multicast Packets Rx Count - R/clr */;
pub const IGC_GPTC: c_uint = 0x04080  /* Good Packets Tx Count - R/clr */;
pub const IGC_GORCL: c_uint = 0x04088  /* Good Octets Rx Count Low - R/clr */;
pub const IGC_GORCH: c_uint = 0x0408C  /* Good Octets Rx Count High - R/clr */;
pub const IGC_GOTCL: c_uint = 0x04090  /* Good Octets Tx Count Low - R/clr */;
pub const IGC_GOTCH: c_uint = 0x04094  /* Good Octets Tx Count High - R/clr */;
pub const IGC_RNBC: c_uint = 0x040A0  /* Rx No Buffers Count - R/clr */;
pub const IGC_RUC: c_uint = 0x040A4  /* Rx Undersize Count - R/clr */;
pub const IGC_RFC: c_uint = 0x040A8  /* Rx Fragment Count - R/clr */;
pub const IGC_ROC: c_uint = 0x040AC  /* Rx Oversize Count - R/clr */;
pub const IGC_RJC: c_uint = 0x040B0  /* Rx Jabber Count - R/clr */;
pub const IGC_MGTPRC: c_uint = 0x040B4  /* Management Packets Rx Count - R/clr */;
pub const IGC_MGTPDC: c_uint = 0x040B8  /* Management Packets Dropped Count - R/clr */;
pub const IGC_MGTPTC: c_uint = 0x040BC  /* Management Packets Tx Count - R/clr */;
pub const IGC_TORL: c_uint = 0x040C0  /* Total Octets Rx Low - R/clr */;
pub const IGC_TORH: c_uint = 0x040C4  /* Total Octets Rx High - R/clr */;
pub const IGC_TOTL: c_uint = 0x040C8  /* Total Octets Tx Low - R/clr */;
pub const IGC_TOTH: c_uint = 0x040CC  /* Total Octets Tx High - R/clr */;
pub const IGC_TPR: c_uint = 0x040D0  /* Total Packets Rx - R/clr */;
pub const IGC_TPT: c_uint = 0x040D4  /* Total Packets Tx - R/clr */;
pub const IGC_PTC64: c_uint = 0x040D8  /* Packets Tx (64 bytes) - R/clr */;
pub const IGC_PTC127: c_uint = 0x040DC  /* Packets Tx (65-127 bytes) - R/clr */;
pub const IGC_PTC255: c_uint = 0x040E0  /* Packets Tx (128-255 bytes) - R/clr */;
pub const IGC_PTC511: c_uint = 0x040E4  /* Packets Tx (256-511 bytes) - R/clr */;
pub const IGC_PTC1023: c_uint = 0x040E8  /* Packets Tx (512-1023 bytes) - R/clr */;
pub const IGC_PTC1522: c_uint = 0x040EC  /* Packets Tx (1024-1522 Bytes) - R/clr */;
pub const IGC_MPTC: c_uint = 0x040F0  /* Multicast Packets Tx Count - R/clr */;
pub const IGC_BPTC: c_uint = 0x040F4  /* Broadcast Packets Tx Count - R/clr */;
pub const IGC_TSCTC: c_uint = 0x040F8  /* TCP Segmentation Context Tx - R/clr */;
pub const IGC_IAC: c_uint = 0x04100  /* Interrupt Assertion Count */;
pub const IGC_RPTHC: c_uint = 0x04104  /* Rx Packets To Host */;
pub const IGC_TLPIC: c_uint = 0x04148  /* EEE Tx LPI Count */;
pub const IGC_RLPIC: c_uint = 0x0414C  /* EEE Rx LPI Count */;
pub const IGC_HGPTC: c_uint = 0x04118  /* Host Good Packets Tx Count */;
pub const IGC_RXDMTC: c_uint = 0x04120  /* Rx Descriptor Minimum Threshold Count */;
pub const IGC_HGORCL: c_uint = 0x04128  /* Host Good Octets Received Count Low */;
pub const IGC_HGORCH: c_uint = 0x0412C  /* Host Good Octets Received Count High */;
pub const IGC_HGOTCL: c_uint = 0x04130  /* Host Good Octets Transmit Count Low */;
pub const IGC_HGOTCH: c_uint = 0x04134  /* Host Good Octets Transmit Count High */;
pub const IGC_LENERRS: c_uint = 0x04138  /* Length Errors Count */;
// Time sync registers
pub const IGC_TSICR: c_uint = 0x0B66C  /* Time Sync Interrupt Cause */;
pub const IGC_TSIM: c_uint = 0x0B674  /* Time Sync Interrupt Mask Register */;
pub const IGC_TSAUXC: c_uint = 0x0B640  /* Timesync Auxiliary Control register */;
pub const IGC_TSYNCRXCTL: c_uint = 0x0B620  /* Rx Time Sync Control register - RW */;
pub const IGC_TSYNCTXCTL: c_uint = 0x0B614  /* Tx Time Sync Control register - RW */;
pub const IGC_TSYNCRXCFG: c_uint = 0x05F50  /* Time Sync Rx Configuration - RW */;
pub const IGC_TSSDP: c_uint = 0x0003C  /* Time Sync SDP Configuration Register - RW */;
pub const IGC_TRGTTIML0: c_uint = 0x0B644 /* Target Time Register 0 Low  - RW */;
pub const IGC_TRGTTIMH0: c_uint = 0x0B648 /* Target Time Register 0 High - RW */;
pub const IGC_TRGTTIML1: c_uint = 0x0B64C /* Target Time Register 1 Low  - RW */;
pub const IGC_TRGTTIMH1: c_uint = 0x0B650 /* Target Time Register 1 High - RW */;
pub const IGC_FREQOUT0: c_uint = 0x0B654 /* Frequency Out 0 Control Register - RW */;
pub const IGC_FREQOUT1: c_uint = 0x0B658 /* Frequency Out 1 Control Register - RW */;
pub const IGC_AUXSTMPL0: c_uint = 0x0B65C /* Auxiliary Time Stamp 0 Register Low  - RO */;
pub const IGC_AUXSTMPH0: c_uint = 0x0B660 /* Auxiliary Time Stamp 0 Register High - RO */;
pub const IGC_AUXSTMPL1: c_uint = 0x0B664 /* Auxiliary Time Stamp 1 Register Low  - RO */;
pub const IGC_AUXSTMPH1: c_uint = 0x0B668 /* Auxiliary Time Stamp 1 Register High - RO */;

// Time sync registers - preemption statistics
pub const IGC_PRMPTDRCNT: c_uint = 0x04284	/* Good RX Preempted Packets */;
pub const IGC_PRMEVNTTCNT: c_uint = 0x04298	/* TX Preemption event counter */;
pub const IGC_PRMEVNTRCNT: c_uint = 0x0429C	/* RX Preemption event counter */;
// Preemption Exception Counter
pub const IGC_PRMEXCPRCNT: c_uint = 0x42A0;
// Received out of order packets with SMD-C
pub const IGC_PRMEXCPRCNT_OOO_SMDC: c_uint = 0x000000FF;
// Received out of order packets with SMD-C and wrong Frame CNT
pub const IGC_PRMEXCPRCNT_OOO_FRAME_CNT: c_uint = 0x0000FF00;
// Received out of order packets with SMD-C and wrong Frag CNT
pub const IGC_PRMEXCPRCNT_OOO_FRAG_CNT: c_uint = 0x00FF0000;
// Received packets with SMD-S and wrong Frag CNT and Frame CNT
pub const IGC_PRMEXCPRCNT_MISS_FRAME_FRAG_CNT: c_uint = 0xFF000000;
// Transmit Scheduling Registers
pub const IGC_TQAVCTRL: c_uint = 0x3570;

pub const IGC_GTXOFFSET: c_uint = 0x3310;
pub const IGC_BASET_L: c_uint = 0x3314;
pub const IGC_BASET_H: c_uint = 0x3318;
pub const IGC_QBVCYCLET: c_uint = 0x331C;
pub const IGC_QBVCYCLET_S: c_uint = 0x3320;

pub const IGC_DTXMXPKTSZ: c_uint = 0x355C;

pub const IGC_TXARB: c_uint = 0x3354 /* Tx Arbitration Control TxARB - RW */;
// System Time Registers
pub const IGC_SYSTIML: c_uint = 0x0B600  /* System time register Low - RO */;
pub const IGC_SYSTIMH: c_uint = 0x0B604  /* System time register High - RO */;
pub const IGC_SYSTIMR: c_uint = 0x0B6F8  /* System time register Residue */;
pub const IGC_TIMINCA: c_uint = 0x0B608  /* Increment attributes register - RW */;
pub const IGC_SYSTIML_1: c_uint = 0x0B688  /* System time register Low - RO (timer 1) */;
pub const IGC_SYSTIMH_1: c_uint = 0x0B68C  /* System time register High - RO (timer 1) */;
pub const IGC_SYSTIMR_1: c_uint = 0x0B684  /* System time register Residue (timer 1) */;
pub const IGC_TIMINCA_1: c_uint = 0x0B690  /* Increment attributes register - RW (timer 1) */;
// TX Timestamp Low
pub const IGC_TXSTMPL_0: c_uint = 0x0B618;
pub const IGC_TXSTMPL_1: c_uint = 0x0B698;
pub const IGC_TXSTMPL_2: c_uint = 0x0B6B8;
pub const IGC_TXSTMPL_3: c_uint = 0x0B6D8;
// TX Timestamp High
pub const IGC_TXSTMPH_0: c_uint = 0x0B61C;
pub const IGC_TXSTMPH_1: c_uint = 0x0B69C;
pub const IGC_TXSTMPH_2: c_uint = 0x0B6BC;
pub const IGC_TXSTMPH_3: c_uint = 0x0B6DC;
pub const IGC_TXSTMPL: c_uint = 0x0B618  /* Tx timestamp value Low - RO */;
pub const IGC_TXSTMPH: c_uint = 0x0B61C  /* Tx timestamp value High - RO */;
pub const IGC_TIMADJ: c_uint = 0x0B60C  /* Time Adjustment Offset Register */;
// PCIe Registers
pub const IGC_PTM_CTRL: c_uint = 0x12540  /* PTM Control */;
pub const IGC_PTM_STAT: c_uint = 0x12544  /* PTM Status */;
pub const IGC_PTM_CYCLE_CTRL: c_uint = 0x1254C  /* PTM Cycle Control */;
// PTM Time registers
pub const IGC_PTM_T1_TIM0_L: c_uint = 0x12558  /* T1 on Timer 0 Low */;
pub const IGC_PTM_T1_TIM0_H: c_uint = 0x1255C  /* T1 on Timer 0 High */;
pub const IGC_PTM_CURR_T2_L: c_uint = 0x1258C  /* Current T2 Low */;
pub const IGC_PTM_CURR_T2_H: c_uint = 0x12590  /* Current T2 High */;
pub const IGC_PTM_PREV_T2_L: c_uint = 0x12584  /* Previous T2 Low */;
pub const IGC_PTM_PREV_T2_H: c_uint = 0x12588  /* Previous T2 High */;
pub const IGC_PTM_PREV_T4M1: c_uint = 0x12578  /* T4 Minus T1 on previous PTM Cycle */;
pub const IGC_PTM_CURR_T4M1: c_uint = 0x1257C  /* T4 Minus T1 on this PTM Cycle */;
pub const IGC_PTM_PREV_T3M2: c_uint = 0x12580  /* T3 Minus T2 on previous PTM Cycle */;
pub const IGC_PTM_TDELAY: c_uint = 0x12594  /* PTM PCIe Link Delay */;
pub const IGC_PCIE_DIG_DELAY: c_uint = 0x12550  /* PCIe Digital Delay */;
pub const IGC_PCIE_PHY_DELAY: c_uint = 0x12554  /* PCIe PHY Delay */;
// Management registers
pub const IGC_MANC: c_uint = 0x05820  /* Management Control - RW */;
// Shadow Ram Write Register - RW
pub const IGC_SRWR: c_uint = 0x12018;
// Wake Up registers
pub const IGC_WUC: c_uint = 0x05800  /* Wakeup Control - RW */;
pub const IGC_WUFC: c_uint = 0x05808  /* Wakeup Filter Control - RW */;
pub const IGC_WUS: c_uint = 0x05810  /* Wakeup Status - R/W1C */;
pub const IGC_WUPL: c_uint = 0x05900  /* Wakeup Packet Length - RW */;
pub const IGC_WUFC_EXT: c_uint = 0x0580C  /* Wakeup Filter Control Register Extended - RW */;
// Wake Up packet memory

// Energy Efficient Ethernet "EEE" registers
pub const IGC_EEER: c_uint = 0x0E30 /* Energy Efficient Ethernet "EEE"*/;
pub const IGC_IPCNFG: c_uint = 0x0E38 /* Internal PHY Configuration */;
pub const IGC_EEE_SU: c_uint = 0x0E34 /* EEE Setup */;
// MULTI GBT AN Control Register - reg. 7.32
pub const IGC_ANEG_MULTIGBT_AN_CTRL: c_uint = 0x0020;
// EEE ANeg Advertisement Register - reg 7.60 and reg 7.62
pub const IGC_ANEG_EEE_AB1: c_uint = 0x003c;
pub const IGC_ANEG_EEE_AB2: c_uint = 0x003e;
// EEE ANeg Link-Partner Advertisement Register - reg 7.61 and reg 7.63
pub const IGC_ANEG_EEE_LP_AB1: c_uint = 0x003d;
pub const IGC_ANEG_EEE_LP_AB2: c_uint = 0x003f;
// LTR registers
pub const IGC_LTRC: c_uint = 0x01A0 /* Latency Tolerance Reporting Control */;
pub const IGC_LTRMINV: c_uint = 0x5BB0 /* LTR Minimum Value */;
pub const IGC_LTRMAXV: c_uint = 0x5BB4 /* LTR Maximum Value */;
// forward declaration
extern "C" {
    pub fn igc_rd32(hw: *mut igc_hw, reg: u32) -> u32;
}
// write operations, indexed using DWORDS

