//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_regs.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.
pub const E1000_CTRL: c_uint = 0x00000  /* Device Control - RW */;
pub const E1000_STATUS: c_uint = 0x00008  /* Device Status - RO */;
pub const E1000_EECD: c_uint = 0x00010  /* EEPROM/Flash Control - RW */;
pub const E1000_EERD: c_uint = 0x00014  /* EEPROM Read - RW */;
pub const E1000_CTRL_EXT: c_uint = 0x00018  /* Extended Device Control - RW */;
pub const E1000_MDIC: c_uint = 0x00020  /* MDI Control - RW */;
pub const E1000_MDICNFG: c_uint = 0x00E04  /* MDI Config - RW */;
pub const E1000_SCTL: c_uint = 0x00024  /* SerDes Control - RW */;
pub const E1000_FCAL: c_uint = 0x00028  /* Flow Control Address Low - RW */;
pub const E1000_FCAH: c_uint = 0x0002C  /* Flow Control Address High -RW */;
pub const E1000_FCT: c_uint = 0x00030  /* Flow Control Type - RW */;
pub const E1000_CONNSW: c_uint = 0x00034  /* Copper/Fiber switch control - RW */;
pub const E1000_VET: c_uint = 0x00038  /* VLAN Ether Type - RW */;
pub const E1000_TSSDP: c_uint = 0x0003C  /* Time Sync SDP Configuration Register - RW */;
pub const E1000_ICR: c_uint = 0x000C0  /* Interrupt Cause Read - R/clr */;
pub const E1000_ITR: c_uint = 0x000C4  /* Interrupt Throttling Rate - RW */;
pub const E1000_ICS: c_uint = 0x000C8  /* Interrupt Cause Set - WO */;
pub const E1000_IMS: c_uint = 0x000D0  /* Interrupt Mask Set - RW */;
pub const E1000_IMC: c_uint = 0x000D8  /* Interrupt Mask Clear - WO */;
pub const E1000_IAM: c_uint = 0x000E0  /* Interrupt Acknowledge Auto Mask */;
pub const E1000_RCTL: c_uint = 0x00100  /* RX Control - RW */;
pub const E1000_FCTTV: c_uint = 0x00170  /* Flow Control Transmit Timer Value - RW */;
pub const E1000_TXCW: c_uint = 0x00178  /* TX Configuration Word - RW */;
pub const E1000_EICR: c_uint = 0x01580  /* Ext. Interrupt Cause Read - R/clr */;

pub const E1000_EICS: c_uint = 0x01520  /* Ext. Interrupt Cause Set - W0 */;
pub const E1000_EIMS: c_uint = 0x01524  /* Ext. Interrupt Mask Set/Read - RW */;
pub const E1000_EIMC: c_uint = 0x01528  /* Ext. Interrupt Mask Clear - WO */;
pub const E1000_EIAC: c_uint = 0x0152C  /* Ext. Interrupt Auto Clear - RW */;
pub const E1000_EIAM: c_uint = 0x01530  /* Ext. Interrupt Ack Auto Clear Mask - RW */;
pub const E1000_GPIE: c_uint = 0x01514  /* General Purpose Interrupt Enable - RW */;
pub const E1000_IVAR0: c_uint = 0x01700  /* Interrupt Vector Allocation (array) - RW */;
pub const E1000_IVAR_MISC: c_uint = 0x01740 /* IVAR for "other" causes - RW */;
pub const E1000_TCTL: c_uint = 0x00400  /* TX Control - RW */;
pub const E1000_TCTL_EXT: c_uint = 0x00404  /* Extended TX Control - RW */;
pub const E1000_TIPG: c_uint = 0x00410  /* TX Inter-packet gap -RW */;
pub const E1000_AIT: c_uint = 0x00458  /* Adaptive Interframe Spacing Throttle - RW */;
pub const E1000_LEDCTL: c_uint = 0x00E00  /* LED Control - RW */;
pub const E1000_LEDMUX: c_uint = 0x08130  /* LED MUX Control */;
pub const E1000_PBA: c_uint = 0x01000  /* Packet Buffer Allocation - RW */;
pub const E1000_PBS: c_uint = 0x01008  /* Packet Buffer Size */;
pub const E1000_EEMNGCTL: c_uint = 0x01010  /* MNG EEprom Control */;
pub const E1000_EEMNGCTL_I210: c_uint = 0x12030  /* MNG EEprom Control */;
pub const E1000_EEARBC_I210: c_uint = 0x12024  /* EEPROM Auto Read Bus Control */;
pub const E1000_EEWR: c_uint = 0x0102C  /* EEPROM Write Register - RW */;
pub const E1000_I2CCMD: c_uint = 0x01028  /* SFPI2C Command Register - RW */;
pub const E1000_FRTIMER: c_uint = 0x01048  /* Free Running Timer - RW */;
pub const E1000_TCPTIMER: c_uint = 0x0104C  /* TCP Timer - RW */;
pub const E1000_FCRTL: c_uint = 0x02160  /* Flow Control Receive Threshold Low - RW */;
pub const E1000_FCRTH: c_uint = 0x02168  /* Flow Control Receive Threshold High - RW */;
pub const E1000_FCRTV: c_uint = 0x02460  /* Flow Control Refresh Timer Value - RW */;
pub const E1000_I2CPARAMS: c_uint = 0x0102C /* SFPI2C Parameters Register - RW */;
pub const E1000_I2CBB_EN: c_uint = 0x00000100  /* I2C - Bit Bang Enable */;
pub const E1000_I2C_CLK_OUT: c_uint = 0x00000200  /* I2C- Clock */;
pub const E1000_I2C_DATA_OUT: c_uint = 0x00000400  /* I2C- Data Out */;
pub const E1000_I2C_DATA_OE_N: c_uint = 0x00000800  /* I2C- Data Output Enable */;
pub const E1000_I2C_DATA_IN: c_uint = 0x00001000  /* I2C- Data In */;
pub const E1000_I2C_CLK_OE_N: c_uint = 0x00002000  /* I2C- Clock Output Enable */;
pub const E1000_I2C_CLK_IN: c_uint = 0x00004000  /* I2C- Clock In */;
pub const E1000_MPHY_ADDR_CTRL: c_uint = 0x0024 /* GbE MPHY Address Control */;
pub const E1000_MPHY_DATA: c_uint = 0x0E10 /* GBE MPHY Data */;
pub const E1000_MPHY_STAT: c_uint = 0x0E0C /* GBE MPHY Statistics */;
// IEEE 1588 TIMESYNCH
pub const E1000_TSYNCRXCTL: c_uint = 0x0B620 /* Rx Time Sync Control register - RW */;
pub const E1000_TSYNCTXCTL: c_uint = 0x0B614 /* Tx Time Sync Control register - RW */;
pub const E1000_TSYNCRXCFG: c_uint = 0x05F50 /* Time Sync Rx Configuration - RW */;
pub const E1000_RXSTMPL: c_uint = 0x0B624 /* Rx timestamp Low - RO */;
pub const E1000_RXSTMPH: c_uint = 0x0B628 /* Rx timestamp High - RO */;
pub const E1000_RXSATRL: c_uint = 0x0B62C /* Rx timestamp attribute low - RO */;
pub const E1000_RXSATRH: c_uint = 0x0B630 /* Rx timestamp attribute high - RO */;
pub const E1000_TXSTMPL: c_uint = 0x0B618 /* Tx timestamp value Low - RO */;
pub const E1000_TXSTMPH: c_uint = 0x0B61C /* Tx timestamp value High - RO */;
pub const E1000_SYSTIML: c_uint = 0x0B600 /* System time register Low - RO */;
pub const E1000_SYSTIMH: c_uint = 0x0B604 /* System time register High - RO */;
pub const E1000_TIMINCA: c_uint = 0x0B608 /* Increment attributes register - RW */;
pub const E1000_TSAUXC: c_uint = 0x0B640 /* Timesync Auxiliary Control register */;
pub const E1000_TRGTTIML0: c_uint = 0x0B644 /* Target Time Register 0 Low  - RW */;
pub const E1000_TRGTTIMH0: c_uint = 0x0B648 /* Target Time Register 0 High - RW */;
pub const E1000_TRGTTIML1: c_uint = 0x0B64C /* Target Time Register 1 Low  - RW */;
pub const E1000_TRGTTIMH1: c_uint = 0x0B650 /* Target Time Register 1 High - RW */;
pub const E1000_FREQOUT0: c_uint = 0x0B654 /* Frequency Out 0 Control Register - RW */;
pub const E1000_FREQOUT1: c_uint = 0x0B658 /* Frequency Out 1 Control Register - RW */;
pub const E1000_AUXSTMPL0: c_uint = 0x0B65C /* Auxiliary Time Stamp 0 Register Low  - RO */;
pub const E1000_AUXSTMPH0: c_uint = 0x0B660 /* Auxiliary Time Stamp 0 Register High - RO */;
pub const E1000_AUXSTMPL1: c_uint = 0x0B664 /* Auxiliary Time Stamp 1 Register Low  - RO */;
pub const E1000_AUXSTMPH1: c_uint = 0x0B668 /* Auxiliary Time Stamp 1 Register High - RO */;
pub const E1000_SYSTIMR: c_uint = 0x0B6F8 /* System time register Residue */;
pub const E1000_TSICR: c_uint = 0x0B66C /* Interrupt Cause Register */;
pub const E1000_TSIM: c_uint = 0x0B674 /* Interrupt Mask Register */;
// Filtering Registers

// DMA Coalescing registers
pub const E1000_DMACR: c_uint = 0x02508 /* Control Register */;
pub const E1000_DMCTXTH: c_uint = 0x03550 /* Transmit Threshold */;
pub const E1000_DMCTLX: c_uint = 0x02514 /* Time to Lx Request */;
pub const E1000_DMCRTRH: c_uint = 0x05DD0 /* Receive Packet Rate Threshold */;
pub const E1000_DMCCNT: c_uint = 0x05DD4 /* Current Rx Count */;
pub const E1000_FCRTC: c_uint = 0x02170 /* Flow Control Rx high watermark */;
// TX Rate Limit Registers
pub const E1000_RTTDQSEL: c_uint = 0x3604 /* Tx Desc Plane Queue Select - WO */;
pub const E1000_RTTBCNRM: c_uint = 0x3690 /* Tx BCN Rate-scheduler MMW */;
pub const E1000_RTTBCNRC: c_uint = 0x36B0 /* Tx BCN Rate-Scheduler Config - WO */;
// Split and Replication RX Control - RW
pub const E1000_RXPBS: c_uint = 0x02404 /* Rx Packet Buffer Size - RW */;
// Thermal sensor configuration and status registers
pub const E1000_THMJT: c_uint = 0x08100 /* Junction Temperature */;
pub const E1000_THLOWTC: c_uint = 0x08104 /* Low Threshold Control */;
pub const E1000_THMIDTC: c_uint = 0x08108 /* Mid Threshold Control */;
pub const E1000_THHIGHTC: c_uint = 0x0810C /* High Threshold Control */;
pub const E1000_THSTAT: c_uint = 0x08110 /* Thermal Sensor Status */;
// Convenience macros
//
// Note: "_n" is the queue number of the register to be written to.
//
// Example usage:
// E1000_RDBAL_REG(current_rx_queue)
//

pub const E1000_RXPBS: c_uint = 0x02404  /* Rx Packet Buffer Size - RW */;
pub const E1000_TXPBS: c_uint = 0x03404  /* Tx Packet Buffer Size - RW */;
pub const E1000_TDFH: c_uint = 0x03410  /* TX Data FIFO Head - RW */;
pub const E1000_TDFT: c_uint = 0x03418  /* TX Data FIFO Tail - RW */;
pub const E1000_TDFHS: c_uint = 0x03420  /* TX Data FIFO Head Saved - RW */;
pub const E1000_TDFPC: c_uint = 0x03430  /* TX Data FIFO Packet Count - RW */;
pub const E1000_DTXCTL: c_uint = 0x03590  /* DMA TX Control - RW */;
pub const E1000_CRCERRS: c_uint = 0x04000  /* CRC Error Count - R/clr */;
pub const E1000_ALGNERRC: c_uint = 0x04004  /* Alignment Error Count - R/clr */;
pub const E1000_SYMERRS: c_uint = 0x04008  /* Symbol Error Count - R/clr */;
pub const E1000_RXERRC: c_uint = 0x0400C  /* Receive Error Count - R/clr */;
pub const E1000_MPC: c_uint = 0x04010  /* Missed Packet Count - R/clr */;
pub const E1000_SCC: c_uint = 0x04014  /* Single Collision Count - R/clr */;
pub const E1000_ECOL: c_uint = 0x04018  /* Excessive Collision Count - R/clr */;
pub const E1000_MCC: c_uint = 0x0401C  /* Multiple Collision Count - R/clr */;
pub const E1000_LATECOL: c_uint = 0x04020  /* Late Collision Count - R/clr */;
pub const E1000_COLC: c_uint = 0x04028  /* Collision Count - R/clr */;
pub const E1000_DC: c_uint = 0x04030  /* Defer Count - R/clr */;
pub const E1000_TNCRS: c_uint = 0x04034  /* TX-No CRS - R/clr */;
pub const E1000_SEC: c_uint = 0x04038  /* Sequence Error Count - R/clr */;
pub const E1000_CEXTERR: c_uint = 0x0403C  /* Carrier Extension Error Count - R/clr */;
pub const E1000_RLEC: c_uint = 0x04040  /* Receive Length Error Count - R/clr */;
pub const E1000_XONRXC: c_uint = 0x04048  /* XON RX Count - R/clr */;
pub const E1000_XONTXC: c_uint = 0x0404C  /* XON TX Count - R/clr */;
pub const E1000_XOFFRXC: c_uint = 0x04050  /* XOFF RX Count - R/clr */;
pub const E1000_XOFFTXC: c_uint = 0x04054  /* XOFF TX Count - R/clr */;
pub const E1000_FCRUC: c_uint = 0x04058  /* Flow Control RX Unsupported Count- R/clr */;
pub const E1000_PRC64: c_uint = 0x0405C  /* Packets RX (64 bytes) - R/clr */;
pub const E1000_PRC127: c_uint = 0x04060  /* Packets RX (65-127 bytes) - R/clr */;
pub const E1000_PRC255: c_uint = 0x04064  /* Packets RX (128-255 bytes) - R/clr */;
pub const E1000_PRC511: c_uint = 0x04068  /* Packets RX (255-511 bytes) - R/clr */;
pub const E1000_PRC1023: c_uint = 0x0406C  /* Packets RX (512-1023 bytes) - R/clr */;
pub const E1000_PRC1522: c_uint = 0x04070  /* Packets RX (1024-1522 bytes) - R/clr */;
pub const E1000_GPRC: c_uint = 0x04074  /* Good Packets RX Count - R/clr */;
pub const E1000_BPRC: c_uint = 0x04078  /* Broadcast Packets RX Count - R/clr */;
pub const E1000_MPRC: c_uint = 0x0407C  /* Multicast Packets RX Count - R/clr */;
pub const E1000_GPTC: c_uint = 0x04080  /* Good Packets TX Count - R/clr */;
pub const E1000_GORCL: c_uint = 0x04088  /* Good Octets RX Count Low - R/clr */;
pub const E1000_GORCH: c_uint = 0x0408C  /* Good Octets RX Count High - R/clr */;
pub const E1000_GOTCL: c_uint = 0x04090  /* Good Octets TX Count Low - R/clr */;
pub const E1000_GOTCH: c_uint = 0x04094  /* Good Octets TX Count High - R/clr */;
pub const E1000_RNBC: c_uint = 0x040A0  /* RX No Buffers Count - R/clr */;
pub const E1000_RUC: c_uint = 0x040A4  /* RX Undersize Count - R/clr */;
pub const E1000_RFC: c_uint = 0x040A8  /* RX Fragment Count - R/clr */;
pub const E1000_ROC: c_uint = 0x040AC  /* RX Oversize Count - R/clr */;
pub const E1000_RJC: c_uint = 0x040B0  /* RX Jabber Count - R/clr */;
pub const E1000_MGTPRC: c_uint = 0x040B4  /* Management Packets RX Count - R/clr */;
pub const E1000_MGTPDC: c_uint = 0x040B8  /* Management Packets Dropped Count - R/clr */;
pub const E1000_MGTPTC: c_uint = 0x040BC  /* Management Packets TX Count - R/clr */;
pub const E1000_TORL: c_uint = 0x040C0  /* Total Octets RX Low - R/clr */;
pub const E1000_TORH: c_uint = 0x040C4  /* Total Octets RX High - R/clr */;
pub const E1000_TOTL: c_uint = 0x040C8  /* Total Octets TX Low - R/clr */;
pub const E1000_TOTH: c_uint = 0x040CC  /* Total Octets TX High - R/clr */;
pub const E1000_TPR: c_uint = 0x040D0  /* Total Packets RX - R/clr */;
pub const E1000_TPT: c_uint = 0x040D4  /* Total Packets TX - R/clr */;
pub const E1000_PTC64: c_uint = 0x040D8  /* Packets TX (64 bytes) - R/clr */;
pub const E1000_PTC127: c_uint = 0x040DC  /* Packets TX (65-127 bytes) - R/clr */;
pub const E1000_PTC255: c_uint = 0x040E0  /* Packets TX (128-255 bytes) - R/clr */;
pub const E1000_PTC511: c_uint = 0x040E4  /* Packets TX (256-511 bytes) - R/clr */;
pub const E1000_PTC1023: c_uint = 0x040E8  /* Packets TX (512-1023 bytes) - R/clr */;
pub const E1000_PTC1522: c_uint = 0x040EC  /* Packets TX (1024-1522 Bytes) - R/clr */;
pub const E1000_MPTC: c_uint = 0x040F0  /* Multicast Packets TX Count - R/clr */;
pub const E1000_BPTC: c_uint = 0x040F4  /* Broadcast Packets TX Count - R/clr */;
pub const E1000_TSCTC: c_uint = 0x040F8  /* TCP Segmentation Context TX - R/clr */;
pub const E1000_TSCTFC: c_uint = 0x040FC  /* TCP Segmentation Context TX Fail - R/clr */;
pub const E1000_IAC: c_uint = 0x04100  /* Interrupt Assertion Count */;
// Interrupt Cause Rx Packet Timer Expire Count
pub const E1000_ICRXPTC: c_uint = 0x04104;
// Interrupt Cause Rx Absolute Timer Expire Count
pub const E1000_ICRXATC: c_uint = 0x04108;
// Interrupt Cause Tx Packet Timer Expire Count
pub const E1000_ICTXPTC: c_uint = 0x0410C;
// Interrupt Cause Tx Absolute Timer Expire Count
pub const E1000_ICTXATC: c_uint = 0x04110;
// Interrupt Cause Tx Queue Empty Count
pub const E1000_ICTXQEC: c_uint = 0x04118;
// Interrupt Cause Tx Queue Minimum Threshold Count
pub const E1000_ICTXQMTC: c_uint = 0x0411C;
// Interrupt Cause Rx Descriptor Minimum Threshold Count
pub const E1000_ICRXDMTC: c_uint = 0x04120;
pub const E1000_ICRXOC: c_uint = 0x04124  /* Interrupt Cause Receiver Overrun Count */;
pub const E1000_PCS_CFG0: c_uint = 0x04200  /* PCS Configuration 0 - RW */;
pub const E1000_PCS_LCTL: c_uint = 0x04208  /* PCS Link Control - RW */;
pub const E1000_PCS_LSTAT: c_uint = 0x0420C  /* PCS Link Status - RO */;
pub const E1000_CBTMPC: c_uint = 0x0402C  /* Circuit Breaker TX Packet Count */;
pub const E1000_HTDPMC: c_uint = 0x0403C  /* Host Transmit Discarded Packets */;
pub const E1000_CBRMPC: c_uint = 0x040FC  /* Circuit Breaker RX Packet Count */;
pub const E1000_RPTHC: c_uint = 0x04104  /* Rx Packets To Host */;
pub const E1000_HGPTC: c_uint = 0x04118  /* Host Good Packets TX Count */;
pub const E1000_HTCBDPC: c_uint = 0x04124  /* Host TX Circuit Breaker Dropped Count */;
pub const E1000_HGORCL: c_uint = 0x04128  /* Host Good Octets Received Count Low */;
pub const E1000_HGORCH: c_uint = 0x0412C  /* Host Good Octets Received Count High */;
pub const E1000_HGOTCL: c_uint = 0x04130  /* Host Good Octets Transmit Count Low */;
pub const E1000_HGOTCH: c_uint = 0x04134  /* Host Good Octets Transmit Count High */;
pub const E1000_LENERRS: c_uint = 0x04138  /* Length Errors Count */;
pub const E1000_SCVPC: c_uint = 0x04228  /* SerDes/SGMII Code Violation Pkt Count */;
pub const E1000_PCS_ANADV: c_uint = 0x04218  /* AN advertisement - RW */;
pub const E1000_PCS_LPAB: c_uint = 0x0421C  /* Link Partner Ability - RW */;
pub const E1000_PCS_NPTX: c_uint = 0x04220  /* AN Next Page Transmit - RW */;
pub const E1000_PCS_LPABNP: c_uint = 0x04224  /* Link Partner Ability Next Page - RW */;
pub const E1000_RXCSUM: c_uint = 0x05000  /* RX Checksum Control - RW */;
pub const E1000_RLPML: c_uint = 0x05004  /* RX Long Packet Max Length */;
pub const E1000_RFCTL: c_uint = 0x05008  /* Receive Filter Control*/;
pub const E1000_MTA: c_uint = 0x05200  /* Multicast Table Array - RW Array */;
pub const E1000_RA: c_uint = 0x05400  /* Receive Address - RW Array */;
pub const E1000_RA2: c_uint = 0x054E0  /* 2nd half of Rx address array - RW Array */;

pub const E1000_VLAPQF: c_uint = 0x055B0  /* VLAN Priority Queue Filter VLAPQF */;

pub const E1000_VFTA: c_uint = 0x05600  /* VLAN Filter Table Array - RW Array */;
pub const E1000_VT_CTL: c_uint = 0x0581C  /* VMDq Control - RW */;
pub const E1000_WUC: c_uint = 0x05800  /* Wakeup Control - RW */;
pub const E1000_WUFC: c_uint = 0x05808  /* Wakeup Filter Control - RW */;
pub const E1000_WUS: c_uint = 0x05810  /* Wakeup Status - R/W1C */;
pub const E1000_MANC: c_uint = 0x05820  /* Management Control - RW */;
pub const E1000_IPAV: c_uint = 0x05838  /* IP Address Valid - RW */;
pub const E1000_WUPL: c_uint = 0x05900  /* Wakeup Packet Length - RW */;
pub const E1000_SW_FW_SYNC: c_uint = 0x05B5C /* Software-Firmware Synchronization - RW */;
pub const E1000_CCMCTL: c_uint = 0x05B48 /* CCM Control Register */;
pub const E1000_GIOCTL: c_uint = 0x05B44 /* GIO Analog Control Register */;
pub const E1000_SCCTL: c_uint = 0x05B4C /* PCIc PLL Configuration Register */;
pub const E1000_GCR: c_uint = 0x05B00 /* PCI-Ex Control */;
pub const E1000_FACTPS: c_uint = 0x05B30 /* Function Active and Power State to MNG */;
pub const E1000_SWSM: c_uint = 0x05B50 /* SW Semaphore */;
pub const E1000_FWSM: c_uint = 0x05B54 /* FW Semaphore */;
pub const E1000_DCA_CTRL: c_uint = 0x05B74 /* DCA Control - RW */;
// RSS registers
pub const E1000_MRQC: c_uint = 0x05818 /* Multiple Receive Control - RW */;

pub const E1000_IMIRVP: c_uint = 0x05AC0 /* Immediate Interrupt RX VLAN Priority - RW */;
// MSI-X Allocation Register (_i) - RW

// Redirection Table - RW Array

// VT Registers
pub const E1000_MBVFICR: c_uint = 0x00C80 /* Mailbox VF Cause - RWC */;
pub const E1000_MBVFIMR: c_uint = 0x00C84 /* Mailbox VF int Mask - RW */;
pub const E1000_VFLRE: c_uint = 0x00C88 /* VF Register Events - RWC */;
pub const E1000_VFRE: c_uint = 0x00C8C /* VF Receive Enables */;
pub const E1000_VFTE: c_uint = 0x00C90 /* VF Transmit Enables */;
pub const E1000_QDE: c_uint = 0x02408 /* Queue Drop Enable - RW */;
pub const E1000_DTXSWC: c_uint = 0x03500 /* DMA Tx Switch Control - RW */;
pub const E1000_WVBR: c_uint = 0x03554 /* VM Wrong Behavior - RWS */;
pub const E1000_RPLOLR: c_uint = 0x05AF0 /* Replication Offload - RW */;
pub const E1000_UTA: c_uint = 0x0A000 /* Unicast Table Array - RW */;
pub const E1000_IOVTCL: c_uint = 0x05BBC /* IOV Control Register */;
pub const E1000_TXSWC: c_uint = 0x05ACC /* Tx Switch Control */;
pub const E1000_LVMMC: c_uint = 0x03548 /* Last VM Misbehavior cause */;
// These act per VF so an array friendly macro is used

extern "C" {
    pub fn igb_rd32(hw: *mut e1000_hw, reg: u32) -> u32;
}
// write operations, indexed using DWORDS

// DMA Coalescing registers
pub const E1000_PCIEMISC: c_uint = 0x05BB8 /* PCIE misc config register */;
// Energy Efficient Ethernet "EEE" register
pub const E1000_IPCNFG: c_uint = 0x0E38 /* Internal PHY Configuration */;
pub const E1000_EEER: c_uint = 0x0E30 /* Energy Efficient Ethernet */;

pub const E1000_EMIADD: c_uint = 0x10   /* Extended Memory Indirect Address */;
pub const E1000_EMIDATA: c_uint = 0x11   /* Extended Memory Indirect Data */;

// Thermal Sensor Register
pub const E1000_THSTAT: c_uint = 0x08110 /* Thermal Sensor Status */;
// OS2BMC Registers
pub const E1000_B2OSPC: c_uint = 0x08FE0 /* BMC2OS packets sent by BMC */;
pub const E1000_B2OGPRC: c_uint = 0x04158 /* BMC2OS packets received by host */;
pub const E1000_O2BGPTC: c_uint = 0x08FE4 /* OS2BMC packets received by BMC */;
pub const E1000_O2BSPC: c_uint = 0x0415C /* OS2BMC packets transmitted by host */;
pub const E1000_SRWR: c_uint = 0x12018  /* Shadow Ram Write Register - RW */;
pub const E1000_I210_FLMNGCTL: c_uint = 0x12038;
pub const E1000_I210_FLMNGDATA: c_uint = 0x1203C;
pub const E1000_I210_FLMNGCNT: c_uint = 0x12040;
pub const E1000_I210_FLSWCTL: c_uint = 0x12048;
pub const E1000_I210_FLSWDATA: c_uint = 0x1204C;
pub const E1000_I210_FLSWCNT: c_uint = 0x12050;
pub const E1000_I210_FLA: c_uint = 0x1201C;
pub const E1000_I210_DTXMXPKTSZ: c_uint = 0x355C;

pub const E1000_I210_TQAVCTRL: c_uint = 0x3570;

pub const E1000_I210_RR2DCDELAY: c_uint = 0x5BF4;

