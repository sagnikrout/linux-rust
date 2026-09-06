//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/regs.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
pub const E1000_CTRL: c_uint = 0x00000	/* Device Control - RW */;
pub const E1000_STATUS: c_uint = 0x00008	/* Device Status - RO */;
pub const E1000_EECD: c_uint = 0x00010	/* EEPROM/Flash Control - RW */;
pub const E1000_EERD: c_uint = 0x00014	/* EEPROM Read - RW */;
pub const E1000_CTRL_EXT: c_uint = 0x00018	/* Extended Device Control - RW */;
pub const E1000_FLA: c_uint = 0x0001C	/* Flash Access - RW */;
pub const E1000_MDIC: c_uint = 0x00020	/* MDI Control - RW */;
pub const E1000_SCTL: c_uint = 0x00024	/* SerDes Control - RW */;
pub const E1000_FCAL: c_uint = 0x00028	/* Flow Control Address Low - RW */;
pub const E1000_FCAH: c_uint = 0x0002C	/* Flow Control Address High -RW */;
pub const E1000_FEXT: c_uint = 0x0002C	/* Future Extended - RW */;
pub const E1000_FEXTNVM: c_uint = 0x00028	/* Future Extended NVM - RW */;
pub const E1000_FEXTNVM3: c_uint = 0x0003C	/* Future Extended NVM 3 - RW */;
pub const E1000_FEXTNVM4: c_uint = 0x00024	/* Future Extended NVM 4 - RW */;
pub const E1000_FEXTNVM5: c_uint = 0x00014	/* Future Extended NVM 5 - RW */;
pub const E1000_FEXTNVM6: c_uint = 0x00010	/* Future Extended NVM 6 - RW */;
pub const E1000_FEXTNVM7: c_uint = 0x000E4	/* Future Extended NVM 7 - RW */;
pub const E1000_FEXTNVM8: c_uint = 0x5BB0	/* Future Extended NVM 8 - RW */;
pub const E1000_FEXTNVM9: c_uint = 0x5BB4	/* Future Extended NVM 9 - RW */;
pub const E1000_FEXTNVM11: c_uint = 0x5BBC	/* Future Extended NVM 11 - RW */;
pub const E1000_FEXTNVM12: c_uint = 0x5BC0	/* Future Extended NVM 12 - RW */;
pub const E1000_PCIEANACFG: c_uint = 0x00F18	/* PCIE Analog Config */;
pub const E1000_DPGFR: c_uint = 0x00FAC	/* Dynamic Power Gate Force Control Register */;
pub const E1000_FCT: c_uint = 0x00030	/* Flow Control Type - RW */;
pub const E1000_VET: c_uint = 0x00038	/* VLAN Ether Type - RW */;
pub const E1000_ICR: c_uint = 0x000C0	/* Interrupt Cause Read - R/clr */;
pub const E1000_ITR: c_uint = 0x000C4	/* Interrupt Throttling Rate - RW */;
pub const E1000_ICS: c_uint = 0x000C8	/* Interrupt Cause Set - WO */;
pub const E1000_IMS: c_uint = 0x000D0	/* Interrupt Mask Set - RW */;
pub const E1000_IMC: c_uint = 0x000D8	/* Interrupt Mask Clear - WO */;
pub const E1000_IAM: c_uint = 0x000E0	/* Interrupt Acknowledge Auto Mask */;
pub const E1000_IVAR: c_uint = 0x000E4	/* Interrupt Vector Allocation Register - RW */;
pub const E1000_SVCR: c_uint = 0x000F0;
pub const E1000_SVT: c_uint = 0x000F4;
pub const E1000_LPIC: c_uint = 0x000FC	/* Low Power IDLE control */;
pub const E1000_RCTL: c_uint = 0x00100	/* Rx Control - RW */;
pub const E1000_FCTTV: c_uint = 0x00170	/* Flow Control Transmit Timer Value - RW */;
pub const E1000_TXCW: c_uint = 0x00178	/* Tx Configuration Word - RW */;
pub const E1000_RXCW: c_uint = 0x00180	/* Rx Configuration Word - RO */;
pub const E1000_PBA_ECC: c_uint = 0x01100	/* PBA ECC Register */;
pub const E1000_TCTL: c_uint = 0x00400	/* Tx Control - RW */;
pub const E1000_TCTL_EXT: c_uint = 0x00404	/* Extended Tx Control - RW */;
pub const E1000_TIPG: c_uint = 0x00410	/* Tx Inter-packet gap -RW */;
pub const E1000_AIT: c_uint = 0x00458	/* Adaptive Interframe Spacing Throttle - RW */;
pub const E1000_LEDCTL: c_uint = 0x00E00	/* LED Control - RW */;
pub const E1000_EXTCNF_CTRL: c_uint = 0x00F00	/* Extended Configuration Control */;
pub const E1000_EXTCNF_SIZE: c_uint = 0x00F08	/* Extended Configuration Size */;
pub const E1000_PHY_CTRL: c_uint = 0x00F10	/* PHY Control Register in CSR */;

pub const E1000_PBA: c_uint = 0x01000	/* Packet Buffer Allocation - RW */;
pub const E1000_PBS: c_uint = 0x01008	/* Packet Buffer Size */;
pub const E1000_PBECCSTS: c_uint = 0x0100C	/* Packet Buffer ECC Status - RW */;
pub const E1000_IOSFPC: c_uint = 0x00F28	/* TX corrupted data  */;
pub const E1000_EEMNGCTL: c_uint = 0x01010	/* MNG EEprom Control */;
pub const E1000_EEWR: c_uint = 0x0102C	/* EEPROM Write Register - RW */;
pub const E1000_FLOP: c_uint = 0x0103C	/* FLASH Opcode Register */;
pub const E1000_ERT: c_uint = 0x02008	/* Early Rx Threshold - RW */;
pub const E1000_FCRTL: c_uint = 0x02160	/* Flow Control Receive Threshold Low - RW */;
pub const E1000_FCRTH: c_uint = 0x02168	/* Flow Control Receive Threshold High - RW */;
pub const E1000_PSRCTL: c_uint = 0x02170	/* Packet Split Receive Control - RW */;
pub const E1000_RDFH: c_uint = 0x02410	/* Rx Data FIFO Head - RW */;
pub const E1000_RDFT: c_uint = 0x02418	/* Rx Data FIFO Tail - RW */;
pub const E1000_RDFHS: c_uint = 0x02420	/* Rx Data FIFO Head Saved - RW */;
pub const E1000_RDFTS: c_uint = 0x02428	/* Rx Data FIFO Tail Saved - RW */;
pub const E1000_RDFPC: c_uint = 0x02430	/* Rx Data FIFO Packet Count - RW */;
// Split and Replication Rx Control - RW
pub const E1000_RDTR: c_uint = 0x02820	/* Rx Delay Timer - RW */;
pub const E1000_RADV: c_uint = 0x0282C	/* Rx Interrupt Absolute Delay Timer - RW */;
// Convenience macros
//
// Note: "_n" is the queue number of the register to be written to.
//
// Example usage:
// E1000_RDBAL_REG(current_rx_queue)
//

pub const E1000_KABGTXD: c_uint = 0x03004	/* AFE Band Gap Transmit Ref Data */;

pub const E1000_TDFH: c_uint = 0x03410	/* Tx Data FIFO Head - RW */;
pub const E1000_TDFT: c_uint = 0x03418	/* Tx Data FIFO Tail - RW */;
pub const E1000_TDFHS: c_uint = 0x03420	/* Tx Data FIFO Head Saved - RW */;
pub const E1000_TDFTS: c_uint = 0x03428	/* Tx Data FIFO Tail Saved - RW */;
pub const E1000_TDFPC: c_uint = 0x03430	/* Tx Data FIFO Packet Count - RW */;
pub const E1000_TIDV: c_uint = 0x03820	/* Tx Interrupt Delay Value - RW */;
pub const E1000_TADV: c_uint = 0x0382C	/* Tx Interrupt Absolute Delay Val - RW */;
pub const E1000_CRCERRS: c_uint = 0x04000	/* CRC Error Count - R/clr */;
pub const E1000_ALGNERRC: c_uint = 0x04004	/* Alignment Error Count - R/clr */;
pub const E1000_SYMERRS: c_uint = 0x04008	/* Symbol Error Count - R/clr */;
pub const E1000_RXERRC: c_uint = 0x0400C	/* Receive Error Count - R/clr */;
pub const E1000_MPC: c_uint = 0x04010	/* Missed Packet Count - R/clr */;
pub const E1000_SCC: c_uint = 0x04014	/* Single Collision Count - R/clr */;
pub const E1000_ECOL: c_uint = 0x04018	/* Excessive Collision Count - R/clr */;
pub const E1000_MCC: c_uint = 0x0401C	/* Multiple Collision Count - R/clr */;
pub const E1000_LATECOL: c_uint = 0x04020	/* Late Collision Count - R/clr */;
pub const E1000_COLC: c_uint = 0x04028	/* Collision Count - R/clr */;
pub const E1000_DC: c_uint = 0x04030	/* Defer Count - R/clr */;
pub const E1000_TNCRS: c_uint = 0x04034	/* Tx-No CRS - R/clr */;
pub const E1000_SEC: c_uint = 0x04038	/* Sequence Error Count - R/clr */;
pub const E1000_CEXTERR: c_uint = 0x0403C	/* Carrier Extension Error Count - R/clr */;
pub const E1000_RLEC: c_uint = 0x04040	/* Receive Length Error Count - R/clr */;
pub const E1000_XONRXC: c_uint = 0x04048	/* XON Rx Count - R/clr */;
pub const E1000_XONTXC: c_uint = 0x0404C	/* XON Tx Count - R/clr */;
pub const E1000_XOFFRXC: c_uint = 0x04050	/* XOFF Rx Count - R/clr */;
pub const E1000_XOFFTXC: c_uint = 0x04054	/* XOFF Tx Count - R/clr */;
pub const E1000_FCRUC: c_uint = 0x04058	/* Flow Control Rx Unsupported Count- R/clr */;
pub const E1000_PRC64: c_uint = 0x0405C	/* Packets Rx (64 bytes) - R/clr */;
pub const E1000_PRC127: c_uint = 0x04060	/* Packets Rx (65-127 bytes) - R/clr */;
pub const E1000_PRC255: c_uint = 0x04064	/* Packets Rx (128-255 bytes) - R/clr */;
pub const E1000_PRC511: c_uint = 0x04068	/* Packets Rx (255-511 bytes) - R/clr */;
pub const E1000_PRC1023: c_uint = 0x0406C	/* Packets Rx (512-1023 bytes) - R/clr */;
pub const E1000_PRC1522: c_uint = 0x04070	/* Packets Rx (1024-1522 bytes) - R/clr */;
pub const E1000_GPRC: c_uint = 0x04074	/* Good Packets Rx Count - R/clr */;
pub const E1000_BPRC: c_uint = 0x04078	/* Broadcast Packets Rx Count - R/clr */;
pub const E1000_MPRC: c_uint = 0x0407C	/* Multicast Packets Rx Count - R/clr */;
pub const E1000_GPTC: c_uint = 0x04080	/* Good Packets Tx Count - R/clr */;
pub const E1000_GORCL: c_uint = 0x04088	/* Good Octets Rx Count Low - R/clr */;
pub const E1000_GORCH: c_uint = 0x0408C	/* Good Octets Rx Count High - R/clr */;
pub const E1000_GOTCL: c_uint = 0x04090	/* Good Octets Tx Count Low - R/clr */;
pub const E1000_GOTCH: c_uint = 0x04094	/* Good Octets Tx Count High - R/clr */;
pub const E1000_RNBC: c_uint = 0x040A0	/* Rx No Buffers Count - R/clr */;
pub const E1000_RUC: c_uint = 0x040A4	/* Rx Undersize Count - R/clr */;
pub const E1000_RFC: c_uint = 0x040A8	/* Rx Fragment Count - R/clr */;
pub const E1000_ROC: c_uint = 0x040AC	/* Rx Oversize Count - R/clr */;
pub const E1000_RJC: c_uint = 0x040B0	/* Rx Jabber Count - R/clr */;
pub const E1000_MGTPRC: c_uint = 0x040B4	/* Management Packets Rx Count - R/clr */;
pub const E1000_MGTPDC: c_uint = 0x040B8	/* Management Packets Dropped Count - R/clr */;
pub const E1000_MGTPTC: c_uint = 0x040BC	/* Management Packets Tx Count - R/clr */;
pub const E1000_TORL: c_uint = 0x040C0	/* Total Octets Rx Low - R/clr */;
pub const E1000_TORH: c_uint = 0x040C4	/* Total Octets Rx High - R/clr */;
pub const E1000_TOTL: c_uint = 0x040C8	/* Total Octets Tx Low - R/clr */;
pub const E1000_TOTH: c_uint = 0x040CC	/* Total Octets Tx High - R/clr */;
pub const E1000_TPR: c_uint = 0x040D0	/* Total Packets Rx - R/clr */;
pub const E1000_TPT: c_uint = 0x040D4	/* Total Packets Tx - R/clr */;
pub const E1000_PTC64: c_uint = 0x040D8	/* Packets Tx (64 bytes) - R/clr */;
pub const E1000_PTC127: c_uint = 0x040DC	/* Packets Tx (65-127 bytes) - R/clr */;
pub const E1000_PTC255: c_uint = 0x040E0	/* Packets Tx (128-255 bytes) - R/clr */;
pub const E1000_PTC511: c_uint = 0x040E4	/* Packets Tx (256-511 bytes) - R/clr */;
pub const E1000_PTC1023: c_uint = 0x040E8	/* Packets Tx (512-1023 bytes) - R/clr */;
pub const E1000_PTC1522: c_uint = 0x040EC	/* Packets Tx (1024-1522 Bytes) - R/clr */;
pub const E1000_MPTC: c_uint = 0x040F0	/* Multicast Packets Tx Count - R/clr */;
pub const E1000_BPTC: c_uint = 0x040F4	/* Broadcast Packets Tx Count - R/clr */;
pub const E1000_TSCTC: c_uint = 0x040F8	/* TCP Segmentation Context Tx - R/clr */;
pub const E1000_TSCTFC: c_uint = 0x040FC	/* TCP Segmentation Context Tx Fail - R/clr */;
pub const E1000_IAC: c_uint = 0x04100	/* Interrupt Assertion Count */;
pub const E1000_ICRXPTC: c_uint = 0x04104	/* Interrupt Cause Rx Pkt Timer Expire Count */;
pub const E1000_ICRXATC: c_uint = 0x04108	/* Interrupt Cause Rx Abs Timer Expire Count */;
pub const E1000_ICTXPTC: c_uint = 0x0410C	/* Interrupt Cause Tx Pkt Timer Expire Count */;
pub const E1000_ICTXATC: c_uint = 0x04110	/* Interrupt Cause Tx Abs Timer Expire Count */;
pub const E1000_ICTXQEC: c_uint = 0x04118	/* Interrupt Cause Tx Queue Empty Count */;
pub const E1000_ICTXQMTC: c_uint = 0x0411C	/* Interrupt Cause Tx Queue Min Thresh Count */;
pub const E1000_ICRXDMTC: c_uint = 0x04120	/* Interrupt Cause Rx Desc Min Thresh Count */;
pub const E1000_ICRXOC: c_uint = 0x04124	/* Interrupt Cause Receiver Overrun Count */;
pub const E1000_CRC_OFFSET: c_uint = 0x05F50	/* CRC Offset register */;
pub const E1000_PCS_LCTL: c_uint = 0x04208	/* PCS Link Control - RW */;
pub const E1000_PCS_LSTAT: c_uint = 0x0420C	/* PCS Link Status - RO */;
pub const E1000_PCS_ANADV: c_uint = 0x04218	/* AN advertisement - RW */;
pub const E1000_PCS_LPAB: c_uint = 0x0421C	/* Link Partner Ability - RW */;
pub const E1000_RXCSUM: c_uint = 0x05000	/* Rx Checksum Control - RW */;
pub const E1000_RFCTL: c_uint = 0x05008	/* Receive Filter Control */;
pub const E1000_MTA: c_uint = 0x05200	/* Multicast Table Array - RW Array */;
pub const E1000_RA: c_uint = 0x05400	/* Receive Address - RW Array */;
pub const E1000_VFTA: c_uint = 0x05600	/* VLAN Filter Table Array - RW Array */;
pub const E1000_WUC: c_uint = 0x05800	/* Wakeup Control - RW */;
pub const E1000_WUFC: c_uint = 0x05808	/* Wakeup Filter Control - RW */;
pub const E1000_WUS: c_uint = 0x05810	/* Wakeup Status - RO */;
pub const E1000_MANC: c_uint = 0x05820	/* Management Control - RW */;
pub const E1000_FFLT: c_uint = 0x05F00	/* Flexible Filter Length Table - RW Array */;
pub const E1000_HOST_IF: c_uint = 0x08800	/* Host Interface */;
pub const E1000_KMRNCTRLSTA: c_uint = 0x00034	/* MAC-PHY interface - RW */;
pub const E1000_MANC2H: c_uint = 0x05860	/* Management Control To Host - RW */;
// Management Decision Filters

pub const E1000_SW_FW_SYNC: c_uint = 0x05B5C	/* SW-FW Synchronization - RW */;
pub const E1000_GCR: c_uint = 0x05B00	/* PCI-Ex Control */;
pub const E1000_GCR2: c_uint = 0x05B64	/* PCI-Ex Control #2 */;
pub const E1000_FACTPS: c_uint = 0x05B30	/* Function Active and Power State to MNG */;
pub const E1000_SWSM: c_uint = 0x05B50	/* SW Semaphore */;
pub const E1000_FWSM: c_uint = 0x05B54	/* FW Semaphore */;
pub const E1000_EXFWSM: c_uint = 0x05B58	/* Extended FW Semaphore */;
// Driver-only SW semaphore (not used by BOOT agents)
pub const E1000_SWSM2: c_uint = 0x05B58;
pub const E1000_FFLT_DBG: c_uint = 0x05F04	/* Debug Register */;
pub const E1000_HICR: c_uint = 0x08F00	/* Host Interface Control */;
// RSS registers
pub const E1000_MRQC: c_uint = 0x05818	/* Multiple Receive Control - RW */;

pub const E1000_TSYNCRXCTL: c_uint = 0x0B620	/* Rx Time Sync Control register - RW */;
pub const E1000_TSYNCTXCTL: c_uint = 0x0B614	/* Tx Time Sync Control register - RW */;
pub const E1000_RXSTMPL: c_uint = 0x0B624	/* Rx timestamp Low - RO */;
pub const E1000_RXSTMPH: c_uint = 0x0B628	/* Rx timestamp High - RO */;
pub const E1000_TXSTMPL: c_uint = 0x0B618	/* Tx timestamp value Low - RO */;
pub const E1000_TXSTMPH: c_uint = 0x0B61C	/* Tx timestamp value High - RO */;
pub const E1000_SYSTIML: c_uint = 0x0B600	/* System time register Low - RO */;
pub const E1000_SYSTIMH: c_uint = 0x0B604	/* System time register High - RO */;
pub const E1000_TIMINCA: c_uint = 0x0B608	/* Increment attributes register - RW */;
pub const E1000_SYSSTMPL: c_uint = 0x0B648 /* HH Timesync system stamp low register */;
pub const E1000_SYSSTMPH: c_uint = 0x0B64C /* HH Timesync system stamp hi register */;
pub const E1000_PLTSTMPL: c_uint = 0x0B640 /* HH Timesync platform stamp low register */;
pub const E1000_PLTSTMPH: c_uint = 0x0B644 /* HH Timesync platform stamp hi register */;
pub const E1000_RXMTRL: c_uint = 0x0B634	/* Time sync Rx EtherType and Msg Type - RW */;
pub const E1000_RXUDP: c_uint = 0x0B638	/* Time Sync Rx UDP Port - RW */;
// PHY registers

