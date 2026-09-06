//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_defines.h
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

// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const IGC_CTRL_EXT_SDP2_DIR: c_uint = 0x00000400 /* SDP2 Data direction */;
pub const IGC_CTRL_EXT_SDP3_DIR: c_uint = 0x00000800 /* SDP3 Data direction */;
pub const IGC_CTRL_EXT_DRV_LOAD: c_uint = 0x10000000 /* Drv loaded bit for FW */;
// Definitions for power management and wakeup registers
// Wake Up Control
pub const IGC_WUC_PME_EN: c_uint = 0x00000002 /* PME Enable */;
// Wake Up Filter Control
pub const IGC_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const IGC_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const IGC_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const IGC_WUFC_MC: c_uint = 0x00000008 /* Directed Multicast Wakeup Enable */;
pub const IGC_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;

pub const IGC_CTRL_ADVD3WUC: c_uint = 0x00100000  /* D3 WUC */;
// Wake Up Status
pub const IGC_WUS_EX: c_uint = 0x00000004 /* Directed Exact */;
pub const IGC_WUS_ARPD: c_uint = 0x00000020 /* Directed ARP Request */;
pub const IGC_WUS_IPV4: c_uint = 0x00000040 /* Directed IPv4 */;
pub const IGC_WUS_IPV6: c_uint = 0x00000080 /* Directed IPv6 */;
pub const IGC_WUS_NSD: c_uint = 0x00000400 /* Directed IPv6 Neighbor Solicitation */;
// Packet types that are enabled for wake packet delivery

// Wake Up Packet Length
pub const IGC_WUPL_MASK: c_uint = 0x00000FFF;
// Wake Up Packet Memory stores the first 128 bytes of the wake up packet
pub const IGC_WUPM_BYTES: c_int = 128;
// Wakeup Filter Control Extended

// Loop limit on how long we wait for auto-negotiation to complete
pub const COPPER_LINK_UP_LIMIT: c_int = 10;
pub const PHY_AUTO_NEG_LIMIT: c_int = 45;
// Number of 100 microseconds we wait for PCI Express master disable
pub const MASTER_DISABLE_TIMEOUT: c_int = 800;
// Blocks new Master requests
pub const IGC_CTRL_GIO_MASTER_DISABLE: c_uint = 0x00000004;
// Status of Master requests.
pub const IGC_STATUS_GIO_MASTER_ENABLE: c_uint = 0x00080000;
// Receive Address
// Number of high/low register pairs in the RAR. The RAR (Receive Address
// Registers) holds the directed and multicast addresses that we monitor.
// Technically, we have 16 spots.  However, we reserve one of these spots
// (RAR[15]) for our directed address used by controllers with
// manageability enabled, allowing us room for 15 multicast addresses.
//
pub const IGC_RAH_RAH_MASK: c_uint = 0x0000FFFF;
pub const IGC_RAH_ASEL_MASK: c_uint = 0x00030000;

pub const IGC_RAH_QSEL_MASK: c_uint = 0x000C0000;
pub const IGC_RAH_QSEL_SHIFT: c_int = 18;

pub const IGC_RAH_AV: c_uint = 0x80000000 /* Receive descriptor valid */;
pub const IGC_RAL_MAC_ADDR_LEN: c_int = 4;
pub const IGC_RAH_MAC_ADDR_LEN: c_int = 2;
// Error Codes
pub const IGC_SUCCESS: c_int = 0;
pub const IGC_ERR_NVM: c_int = 1;
pub const IGC_ERR_PHY: c_int = 2;
pub const IGC_ERR_CONFIG: c_int = 3;
pub const IGC_ERR_PARAM: c_int = 4;
pub const IGC_ERR_MAC_INIT: c_int = 5;
pub const IGC_ERR_RESET: c_int = 9;
pub const IGC_ERR_MASTER_REQUESTS_PENDING: c_int = 10;
pub const IGC_ERR_BLK_PHY_RESET: c_int = 12;
pub const IGC_ERR_SWFW_SYNC: c_int = 13;
// Device Control

pub const IGC_CTRL_RST: c_uint = 0x04000000  /* Global reset */;
pub const IGC_CTRL_PHY_RST: c_uint = 0x80000000  /* PHY Reset */;
pub const IGC_CTRL_SLU: c_uint = 0x00000040  /* Set link up (Force Link) */;

pub const IGC_CTRL_FRCSPD: c_uint = 0x00000800  /* Force Speed */;
pub const IGC_CTRL_FRCDPX: c_uint = 0x00001000  /* Force Duplex */;
pub const IGC_CTRL_VME: c_uint = 0x40000000  /* IEEE VLAN mode enable */;
pub const IGC_CTRL_RFCE: c_uint = 0x08000000  /* Receive Flow Control enable */;
pub const IGC_CTRL_TFCE: c_uint = 0x10000000  /* Transmit flow control enable */;
pub const IGC_CTRL_SDP0_DIR: c_uint = 0x00400000  /* SDP0 Data direction */;
pub const IGC_CTRL_SDP1_DIR: c_uint = 0x00800000  /* SDP1 Data direction */;
// As per the EAS the maximum supported size is 9.5KB (9728 bytes)
pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x2600;
// PBA constants
pub const IGC_PBA_34K: c_uint = 0x0022;
// SW Semaphore Register
pub const IGC_SWSM_SMBI: c_uint = 0x00000001 /* Driver Semaphore bit */;
pub const IGC_SWSM_SWESMBI: c_uint = 0x00000002 /* FW Semaphore bit */;
// SWFW_SYNC Definitions
pub const IGC_SWFW_EEP_SM: c_uint = 0x1;
pub const IGC_SWFW_PHY0_SM: c_uint = 0x2;
// Autoneg Advertisement Register
pub const NWAY_AR_10T_HD_CAPS: c_uint = 0x0020   /* 10T   Half Duplex Capable */;
pub const NWAY_AR_10T_FD_CAPS: c_uint = 0x0040   /* 10T   Full Duplex Capable */;
pub const NWAY_AR_100TX_HD_CAPS: c_uint = 0x0080   /* 100TX Half Duplex Capable */;
pub const NWAY_AR_100TX_FD_CAPS: c_uint = 0x0100   /* 100TX Full Duplex Capable */;
pub const NWAY_AR_PAUSE: c_uint = 0x0400   /* Pause operation desired */;
pub const NWAY_AR_ASM_DIR: c_uint = 0x0800   /* Asymmetric Pause Direction bit */;
// Link Partner Ability Register (Base Page)
pub const NWAY_LPAR_PAUSE: c_uint = 0x0400 /* LP Pause operation desired */;
pub const NWAY_LPAR_ASM_DIR: c_uint = 0x0800 /* LP Asymmetric Pause Direction bit */;
// 1000BASE-T Control Register
pub const CR_1000T_HD_CAPS: c_uint = 0x0100 /* Advertise 1000T HD capability */;
pub const CR_1000T_FD_CAPS: c_uint = 0x0200 /* Advertise 1000T FD capability  */;
// 1000BASE-T Status Register
pub const SR_1000T_REMOTE_RX_STATUS: c_uint = 0x1000 /* Remote receiver OK */;
// PHY GPY 211 registers
pub const STANDARD_AN_REG_MASK: c_uint = 0x0007 /* MMD */;

pub const CR_2500T_FD_CAPS: c_uint = 0x0080 /* Advertise 2500T FD capability */;
// NVM Control
// Number of milliseconds for NVM auto read done after MAC reset.
pub const AUTO_READ_DONE_TIMEOUT: c_int = 10;
pub const IGC_EECD_AUTO_RD: c_uint = 0x00000200  /* NVM Auto Read done */;
pub const IGC_EECD_REQ: c_uint = 0x00000040 /* NVM Access Request */;
pub const IGC_EECD_GNT: c_uint = 0x00000080 /* NVM Access Grant */;
// NVM Addressing bits based on type 0=small, 1=large
pub const IGC_EECD_ADDR_BITS: c_uint = 0x00000400;

pub const IGC_EECD_SIZE_EX_MASK: c_uint = 0x00007800  /* NVM Size */;
pub const IGC_EECD_SIZE_EX_SHIFT: c_int = 11;
pub const IGC_EECD_FLUPD_I225: c_uint = 0x00800000 /* Update FLASH */;
pub const IGC_EECD_FLUDONE_I225: c_uint = 0x04000000 /* Update FLASH done*/;
pub const IGC_EECD_FLASH_DETECTED_I225: c_uint = 0x00080000 /* FLASH detected */;
pub const IGC_FLUDONE_ATTEMPTS: c_int = 20000;

// Offset to data in NVM read/write registers
pub const IGC_NVM_RW_REG_DATA: c_int = 16;

// NVM Word Offsets
pub const NVM_CHECKSUM_REG: c_uint = 0x003F;
// For checksumming, the sum of all words in the NVM should equal 0xBABA.
pub const NVM_SUM: c_uint = 0xBABA;
pub const NVM_WORD_SIZE_BASE_SHIFT: c_int = 6;
// Collision related configuration parameters
pub const IGC_COLLISION_THRESHOLD: c_int = 15;
pub const IGC_CT_SHIFT: c_int = 4;
pub const IGC_COLLISION_DISTANCE: c_int = 63;
pub const IGC_COLD_SHIFT: c_int = 12;
// Device Status
pub const IGC_STATUS_FD: c_uint = 0x00000001      /* Full duplex.0=half,1=full */;
pub const IGC_STATUS_LU: c_uint = 0x00000002      /* Link up.0=no,1=link */;
pub const IGC_STATUS_FUNC_MASK: c_uint = 0x0000000C      /* PCI Function Mask */;
pub const IGC_STATUS_FUNC_SHIFT: c_int = 2;
pub const IGC_STATUS_TXOFF: c_uint = 0x00000010      /* transmission paused */;
pub const IGC_STATUS_SPEED_100: c_uint = 0x00000040      /* Speed 100Mb/s */;
pub const IGC_STATUS_SPEED_1000: c_uint = 0x00000080      /* Speed 1000Mb/s */;
pub const IGC_STATUS_SPEED_2500: c_uint = 0x00400000	/* Speed 2.5Gb/s */;
pub const SPEED_10: c_int = 10;
pub const SPEED_100: c_int = 100;
pub const SPEED_1000: c_int = 1000;
pub const SPEED_2500: c_int = 2500;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
// 1Gbps and 2.5Gbps half duplex is not supported, nor spec-compliant.
pub const ADVERTISE_10_HALF: c_uint = 0x0001;
pub const ADVERTISE_10_FULL: c_uint = 0x0002;
pub const ADVERTISE_100_HALF: c_uint = 0x0004;
pub const ADVERTISE_100_FULL: c_uint = 0x0008;
pub const ADVERTISE_1000_HALF: c_uint = 0x0010 /* Not used, just FYI */;
pub const ADVERTISE_1000_FULL: c_uint = 0x0020;
pub const ADVERTISE_2500_HALF: c_uint = 0x0040 /* Not used, just FYI */;
pub const ADVERTISE_2500_FULL: c_uint = 0x0080;

// Interrupt Cause Read

// If this bit asserted, the driver should claim the interrupt

// Interrupt Mask Set

pub const IGC_QVECTOR_MASK: c_uint = 0x7FFC		/* Q-vector mask */;
pub const IGC_ITR_VAL_MASK: c_uint = 0x04		/* ITR value mask */;
// Interrupt Cause Set

pub const IGC_ICR_DOUTSYNC: c_uint = 0x10000000 /* NIC DMA out of sync */;
pub const IGC_EITR_CNT_IGNR: c_uint = 0x80000000 /* Don't reset counters on write */;
pub const IGC_IVAR_VALID: c_uint = 0x80;
pub const IGC_GPIE_NSICR: c_uint = 0x00000001;
pub const IGC_GPIE_MSIX_MODE: c_uint = 0x00000010;
pub const IGC_GPIE_EIAME: c_uint = 0x40000000;
pub const IGC_GPIE_PBA: c_uint = 0x80000000;
// Receive Descriptor bit definitions
pub const IGC_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
// Transmit Descriptor bit definitions
pub const IGC_TXD_DTYP_D: c_uint = 0x00100000 /* Data Descriptor */;
pub const IGC_TXD_DTYP_C: c_uint = 0x00000000 /* Context Descriptor */;
pub const IGC_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const IGC_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const IGC_TXD_POPTS_SMD_MASK: c_uint = 0x3000     /* Indicates whether it's SMD-V or SMD-R */;
pub const IGC_TXD_CMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const IGC_TXD_CMD_IC: c_uint = 0x04000000 /* Insert Checksum */;
pub const IGC_TXD_CMD_DEXT: c_uint = 0x20000000 /* Desc extension (0 = legacy) */;
pub const IGC_TXD_CMD_VLE: c_uint = 0x40000000 /* Add VLAN tag */;
pub const IGC_TXD_STAT_DD: c_uint = 0x00000001 /* Descriptor Done */;
pub const IGC_TXD_CMD_TCP: c_uint = 0x01000000 /* TCP packet */;
pub const IGC_TXD_CMD_IP: c_uint = 0x02000000 /* IP packet */;
pub const IGC_TXD_CMD_TSE: c_uint = 0x04000000 /* TCP Seg enable */;
pub const IGC_TXD_EXTCMD_TSTAMP: c_uint = 0x00000010 /* IEEE1588 Timestamp packet */;
pub const IGC_TXD_PTP2_TIMER_1: c_uint = 0x00000020;
// IPSec Encrypt Enable

pub const IGC_ADVTXD_TSN_CNTX_FIRST: c_uint = 0x00000080;
// Transmit Control
pub const IGC_TCTL_EN: c_uint = 0x00000002 /* enable Tx */;
pub const IGC_TCTL_PSP: c_uint = 0x00000008 /* pad short packets */;
pub const IGC_TCTL_CT: c_uint = 0x00000ff0 /* collision threshold */;
pub const IGC_TCTL_COLD: c_uint = 0x003ff000 /* collision distance */;
pub const IGC_TCTL_RTLC: c_uint = 0x01000000 /* Re-transmit on late collision */;
// Flow Control Constants
pub const FLOW_CONTROL_ADDRESS_LOW: c_uint = 0x00C28001;
pub const FLOW_CONTROL_ADDRESS_HIGH: c_uint = 0x00000100;
pub const FLOW_CONTROL_TYPE: c_uint = 0x8808;
// Enable XON frame transmission
pub const IGC_FCRTL_XONE: c_uint = 0x80000000;
// Management Control
pub const IGC_MANC_RCV_TCO_EN: c_uint = 0x00020000 /* Receive TCO Packets Enabled */;
pub const IGC_MANC_BLK_PHY_RST_ON_IDE: c_uint = 0x00040000 /* Block phy resets */;
// Receive Control
pub const IGC_RCTL_RST: c_uint = 0x00000001 /* Software reset */;
pub const IGC_RCTL_EN: c_uint = 0x00000002 /* enable */;
pub const IGC_RCTL_SBP: c_uint = 0x00000004 /* store bad packet */;
pub const IGC_RCTL_UPE: c_uint = 0x00000008 /* unicast promisc enable */;
pub const IGC_RCTL_MPE: c_uint = 0x00000010 /* multicast promisc enable */;
pub const IGC_RCTL_LPE: c_uint = 0x00000020 /* long packet enable */;
pub const IGC_RCTL_LBM_MAC: c_uint = 0x00000040 /* MAC loopback mode */;
pub const IGC_RCTL_LBM_TCVR: c_uint = 0x000000C0 /* tcvr loopback mode */;
pub const IGC_RCTL_RDMTS_HALF: c_uint = 0x00000000 /* Rx desc min thresh size */;
pub const IGC_RCTL_BAM: c_uint = 0x00008000 /* broadcast enable */;
// Split Replication Receive Control
pub const IGC_SRRCTL_TIMESTAMP: c_uint = 0x40000000;

// Receive Descriptor bit definitions
pub const IGC_RXD_STAT_SMD_TYPE_V: c_uint = 0x01	/* SMD-V Packet */;
pub const IGC_RXD_STAT_SMD_TYPE_R: c_uint = 0x02	/* SMD-R Packet */;
pub const IGC_RXD_STAT_EOP: c_uint = 0x02	/* End of Packet */;
pub const IGC_RXD_STAT_IXSM: c_uint = 0x04	/* Ignore checksum */;
pub const IGC_RXD_STAT_UDPCS: c_uint = 0x10	/* UDP xsum calculated */;
pub const IGC_RXD_STAT_TCPCS: c_uint = 0x20	/* TCP xsum calculated */;
pub const IGC_RXD_STAT_VP: c_uint = 0x08	/* IEEE VLAN Packet */;
pub const IGC_RXDEXT_STATERR_LB: c_uint = 0x00040000;
// Advanced Receive Descriptor bit definitions
pub const IGC_RXDADV_STAT_SMD_TYPE_MASK: c_uint = 0x06000;
pub const IGC_RXDADV_STAT_TSIP: c_uint = 0x08000 /* timestamp in packet */;
pub const IGC_RXDEXT_STATERR_L4E: c_uint = 0x20000000;
pub const IGC_RXDEXT_STATERR_IPE: c_uint = 0x40000000;
pub const IGC_RXDEXT_STATERR_RXE: c_uint = 0x80000000;
pub const IGC_MRQC_ENABLE_RSS_MQ: c_uint = 0x00000002;
pub const IGC_MRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const IGC_MRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const IGC_MRQC_RSS_FIELD_IPV6_TCP_EX: c_uint = 0x00040000;
pub const IGC_MRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const IGC_MRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;
pub const IGC_MRQC_RSS_FIELD_IPV4_UDP: c_uint = 0x00400000;
pub const IGC_MRQC_RSS_FIELD_IPV6_UDP: c_uint = 0x00800000;

// Header split receive
pub const IGC_RFCTL_IPV6_EX_DIS: c_uint = 0x00010000;
pub const IGC_RFCTL_LEF: c_uint = 0x00040000;
pub const IGC_RCTL_SZ_256: c_uint = 0x00030000 /* Rx buffer size 256 */;

pub const IGC_RCTL_CFIEN: c_uint = 0x00080000 /* canonical form enable */;
pub const IGC_RCTL_DPF: c_uint = 0x00400000 /* discard pause frames */;
pub const IGC_RCTL_PMCF: c_uint = 0x00800000 /* pass MAC control frames */;
pub const IGC_RCTL_SECRC: c_uint = 0x04000000 /* Strip Ethernet CRC */;
// Mask for RX packet buffer size

// Mask for timestamp in RX buffer

// High-priority RX packet buffer size (KB). Used for Express traffic when preemption is enabled

// BMC to OS packet buffer size in KB

// Low-priority RX packet buffer size (KB). Used for BE traffic when preemption is enabled

// Enable RX packet buffer for timestamp descriptor, saving 16 bytes per packet if set

// Default value following I225/I226 SW User Manual Section 8.3.1

// Mask for TX packet buffer size

// Mask for OS to BMC packet buffer size

// TX Packet buffer size in KB

// OS to BMC packet buffer size in KB

// Default value following I225/I226 SW User Manual Section 8.3.2

// TSN value following I225/I226 SW User Manual Section 7.5.4

pub const IGC_DTXMXPKTSZ_TSN: c_uint = 0x19 /* 1600 bytes of max TX DMA packet size */;
pub const IGC_DTXMXPKTSZ_DEFAULT: c_uint = 0x98 /* 9728-byte Jumbo frames */;
// Retry Buffer Control
pub const IGC_RETX_CTL: c_uint = 0x041C;
pub const IGC_RETX_CTL_WATERMARK_MASK: c_uint = 0xF;

pub const IGC_RETX_CTL_QBVFULLEN: c_uint = 0x1000 /* Enable QBV Retry Buffer Full Threshold */;
// Transmit Scheduling Latency
// Latency between transmission scheduling (LaunchTime) and the time
// the packet is transmitted to the network in nanosecond.
//
pub const IGC_TXOFFSET_SPEED_10: c_uint = 0x000034BC;
pub const IGC_TXOFFSET_SPEED_100: c_uint = 0x00000578;
pub const IGC_TXOFFSET_SPEED_1000: c_uint = 0x0000012C;
pub const IGC_TXOFFSET_SPEED_2500: c_uint = 0x00000578;
// Time Sync Interrupt Causes

pub const IGC_FTQF_VF_BP: c_uint = 0x00008000;
pub const IGC_FTQF_1588_TIME_STAMP: c_uint = 0x08000000;
pub const IGC_FTQF_MASK: c_uint = 0xF0000000;
pub const IGC_FTQF_MASK_PROTO_BP: c_uint = 0x10000000;
// Time Sync Receive Control bit definitions
pub const IGC_TSYNCRXCTL_TYPE_MASK: c_uint = 0x0000000E  /* Rx type mask */;
pub const IGC_TSYNCRXCTL_TYPE_L2_V2: c_uint = 0x00;
pub const IGC_TSYNCRXCTL_TYPE_L4_V1: c_uint = 0x02;
pub const IGC_TSYNCRXCTL_TYPE_L2_L4_V2: c_uint = 0x04;
pub const IGC_TSYNCRXCTL_TYPE_ALL: c_uint = 0x08;
pub const IGC_TSYNCRXCTL_TYPE_EVENT_V2: c_uint = 0x0A;
pub const IGC_TSYNCRXCTL_ENABLED: c_uint = 0x00000010  /* enable Rx timestamping */;
pub const IGC_TSYNCRXCTL_SYSCFI: c_uint = 0x00000020  /* Sys clock frequency */;
pub const IGC_TSYNCRXCTL_RXSYNSIG: c_uint = 0x00000400  /* Sample RX tstamp in PHY sop */;
// Time Sync Receive Configuration
pub const IGC_TSYNCRXCFG_PTP_V1_CTRLT_MASK: c_uint = 0x000000FF;
pub const IGC_TSYNCRXCFG_PTP_V1_SYNC_MESSAGE: c_uint = 0x00;
pub const IGC_TSYNCRXCFG_PTP_V1_DELAY_REQ_MESSAGE: c_uint = 0x01;
// Immediate Interrupt Receive
pub const IGC_IMIR_CLEAR_MASK: c_uint = 0xF001FFFF /* IMIR Reg Clear Mask */;
pub const IGC_IMIR_PORT_BYPASS: c_uint = 0x20000 /* IMIR Port Bypass Bit */;

pub const IGC_IMIREXT_CLEAR_MASK: c_uint = 0x7FFFF /* IMIREXT Reg Clear Mask */;
// Immediate Interrupt Receive Extended
pub const IGC_IMIREXT_CTRL_BP: c_uint = 0x00080000  /* Bypass check of ctrl bits */;
pub const IGC_IMIREXT_SIZE_BP: c_uint = 0x00001000  /* Packet size bypass */;
// Time Sync Transmit Control bit definitions
pub const IGC_TSYNCTXCTL_TXTT_0: c_uint = 0x00000001  /* Tx timestamp reg 0 valid */;
pub const IGC_TSYNCTXCTL_TXTT_1: c_uint = 0x00000002  /* Tx timestamp reg 1 valid */;
pub const IGC_TSYNCTXCTL_TXTT_2: c_uint = 0x00000004  /* Tx timestamp reg 2 valid */;
pub const IGC_TSYNCTXCTL_TXTT_3: c_uint = 0x00000008  /* Tx timestamp reg 3 valid */;
pub const IGC_TSYNCTXCTL_ENABLED: c_uint = 0x00000010  /* enable Tx timestamping */;
pub const IGC_TSYNCTXCTL_MAX_ALLOWED_DLY_MASK: c_uint = 0x0000F000  /* max delay */;
pub const IGC_TSYNCTXCTL_SYNC_COMP_ERR: c_uint = 0x20000000  /* sync err */;
pub const IGC_TSYNCTXCTL_SYNC_COMP: c_uint = 0x40000000  /* sync complete */;
pub const IGC_TSYNCTXCTL_START_SYNC: c_uint = 0x80000000  /* initiate sync */;
pub const IGC_TSYNCTXCTL_TXSYNSIG: c_uint = 0x00000020  /* Sample TX tstamp in PHY sop */;

// Timer selection bits

// TSAUXC Configuration Bits

// SDP Configuration Bits

// Transmit Scheduling
pub const IGC_TQAVCTRL_TRANSMIT_MODE_TSN: c_uint = 0x00000001;
pub const IGC_TQAVCTRL_PREEMPT_ENA: c_uint = 0x00000002;
pub const IGC_TQAVCTRL_ENHANCED_QAV: c_uint = 0x00000008;
pub const IGC_TQAVCTRL_FUTSCDDIS: c_uint = 0x00000080;
pub const IGC_TQAVCTRL_MIN_FRAG_MASK: c_uint = 0x0000C000;
pub const IGC_TXQCTL_QUEUE_MODE_LAUNCHT: c_uint = 0x00000001;
pub const IGC_TXQCTL_STRICT_CYCLE: c_uint = 0x00000002;
pub const IGC_TXQCTL_STRICT_END: c_uint = 0x00000004;
pub const IGC_TXQCTL_PREEMPTIBLE: c_uint = 0x00000008;
pub const IGC_TXQCTL_QAV_SEL_MASK: c_uint = 0x000000C0;
pub const IGC_TXQCTL_QAV_SEL_CBS0: c_uint = 0x00000080;
pub const IGC_TXQCTL_QAV_SEL_CBS1: c_uint = 0x000000C0;
pub const IGC_TQAVCC_IDLESLOPE_MASK: c_uint = 0xFFFF;

pub const IGC_MAX_SR_QUEUES: c_int = 2;

// Receive Checksum Control
pub const IGC_RXCSUM_CRCOFL: c_uint = 0x00000800   /* CRC32 offload enable */;
pub const IGC_RXCSUM_PCSD: c_uint = 0x00002000   /* packet checksum disabled */;
// PCIe PTM Control

// A short cycle time of 1us theoretically should work, but appears to be too
// short in practice.
//

// PCIe Digital Delay
pub const IGC_PCIE_DIG_DELAY_DEFAULT: c_uint = 0x01440000;
// PCIe PHY Delay
pub const IGC_PCIE_PHY_DELAY_DEFAULT: c_uint = 0x40900000;
pub const IGC_TIMADJ_ADJUST_METH: c_uint = 0x40000000;
// PCIe PTM Status

// PCIe PTM Cycle Control

// GPY211 - I225 defines
pub const GPY_MMD_MASK: c_uint = 0xFFFF0000;
pub const GPY_MMD_SHIFT: c_int = 16;
pub const GPY_REG_MASK: c_uint = 0x0000FFFF;
pub const IGC_MMDAC_FUNC_DATA: c_uint = 0x4000 /* Data, no post increment */;
// MAC definitions
pub const IGC_FACTPS_MNGCG: c_uint = 0x20000000;
pub const IGC_FWSM_MODE_MASK: c_uint = 0xE;
pub const IGC_FWSM_MODE_SHIFT: c_int = 1;
// Management Control
pub const IGC_MANC_SMBUS_EN: c_uint = 0x00000001 /* SMBus Enabled - RO */;
pub const IGC_MANC_ASF_EN: c_uint = 0x00000002 /* ASF Enabled - RO */;
// PHY
pub const PHY_REVISION_MASK: c_uint = 0xFFFFFFF0;
pub const MAX_PHY_REG_ADDRESS: c_uint = 0x1F  /* 5 bit address bus (0-0x1F) */;
pub const IGC_GEN_POLL_TIMEOUT: c_int = 1920;
// PHY Control Register

pub const MII_CR_SPEED_10: c_uint = 0x0000	/* SSM=0, SSL=0: 10 Mb/s */;

pub const MII_CR_RESTART_AUTO_NEG: c_uint = 0x0200  /* Restart auto negotiation */;
pub const MII_CR_POWER_DOWN: c_uint = 0x0800  /* Power down */;
pub const MII_CR_AUTO_NEG_EN: c_uint = 0x1000  /* Auto Neg Enable */;
// PHY Status Register
pub const MII_SR_LINK_STATUS: c_uint = 0x0004 /* Link Status 1 = link */;
pub const MII_SR_AUTONEG_COMPLETE: c_uint = 0x0020 /* Auto Neg Complete */;
pub const IGC_PHY_RST_COMP: c_uint = 0x0100 /* Internal PHY reset completion */;
// PHY 1000 MII Register/Bit Definitions
// PHY Registers defined by IEEE
pub const PHY_CONTROL: c_uint = 0x00 /* Control Register */;
pub const PHY_STATUS: c_uint = 0x01 /* Status Register */;
pub const PHY_ID1: c_uint = 0x02 /* Phy Id Reg (word 1) */;
pub const PHY_ID2: c_uint = 0x03 /* Phy Id Reg (word 2) */;
pub const PHY_AUTONEG_ADV: c_uint = 0x04 /* Autoneg Advertisement */;
pub const PHY_LP_ABILITY: c_uint = 0x05 /* Link Partner Ability (Base Page) */;
pub const PHY_1000T_CTRL: c_uint = 0x09 /* 1000Base-T Control Reg */;
pub const PHY_1000T_STATUS: c_uint = 0x0A /* 1000Base-T Status Reg */;
// MDI Control
pub const IGC_MDIC_DATA_MASK: c_uint = 0x0000FFFF;
pub const IGC_MDIC_REG_MASK: c_uint = 0x001F0000;
pub const IGC_MDIC_REG_SHIFT: c_int = 16;
pub const IGC_MDIC_PHY_MASK: c_uint = 0x03E00000;
pub const IGC_MDIC_PHY_SHIFT: c_int = 21;
pub const IGC_MDIC_OP_WRITE: c_uint = 0x04000000;
pub const IGC_MDIC_OP_READ: c_uint = 0x08000000;
pub const IGC_MDIC_READY: c_uint = 0x10000000;
pub const IGC_MDIC_ERROR: c_uint = 0x40000000;
// EEE Link Ability

// EEE Link-Partner Ability

pub const IGC_MAX_MAC_HDR_LEN: c_int = 127;
pub const IGC_MAX_NETWORK_HDR_LEN: c_int = 511;

pub const IGC_VLANPQF_QUEUE_MASK: c_uint = 0x03;

pub const IGC_ADVTXD_TUCMD_IPV4: c_uint = 0x00000400  /* IP Packet Type:1=IPv4 */;
pub const IGC_ADVTXD_TUCMD_L4T_TCP: c_uint = 0x00000800  /* L4 Packet Type of TCP */;
pub const IGC_ADVTXD_TUCMD_L4T_SCTP: c_uint = 0x00001000 /* L4 packet TYPE of SCTP */;
// Maximum size of the MTA register table in all supported adapters
pub const MAX_MTA_REG: c_int = 128;
// EEE defines
pub const IGC_IPCNFG_EEE_2_5G_AN: c_uint = 0x00000010 /* IPCNFG EEE Ena 2.5G AN */;
pub const IGC_IPCNFG_EEE_1G_AN: c_uint = 0x00000008 /* IPCNFG EEE Ena 1G AN */;
pub const IGC_IPCNFG_EEE_100M_AN: c_uint = 0x00000004 /* IPCNFG EEE Ena 100M AN */;
pub const IGC_EEER_EEE_NEG: c_uint = 0x20000000 /* EEE capability nego */;
pub const IGC_EEER_TX_LPI_EN: c_uint = 0x00010000 /* EEER Tx LPI Enable */;
pub const IGC_EEER_RX_LPI_EN: c_uint = 0x00020000 /* EEER Rx LPI Enable */;
pub const IGC_EEER_LPI_FC: c_uint = 0x00040000 /* EEER Ena on Flow Cntrl */;
pub const IGC_EEE_SU_LPI_CLK_STP: c_uint = 0x00800000 /* EEE LPI Clock Stop */;
// LTR defines
pub const IGC_LTRC_EEEMS_EN: c_uint = 0x00000020 /* Enable EEE LTR max send */;
pub const IGC_RXPBS_SIZE_I225_MASK: c_uint = 0x0000003F /* Rx packet buffer size */;
pub const IGC_TW_SYSTEM_1000_MASK: c_uint = 0x000000FF;
// Minimum time for 100BASE-T where no data will be transmit following move out
// of EEE LPI Tx state
//
pub const IGC_TW_SYSTEM_100_MASK: c_uint = 0x0000FF00;
pub const IGC_TW_SYSTEM_100_SHIFT: c_int = 8;
// Reg val to set scale to 1024 nsec
pub const IGC_LTRMINV_SCALE_1024: c_int = 2;
// Reg val to set scale to 32768 nsec
pub const IGC_LTRMINV_SCALE_32768: c_int = 3;
// Reg val to set scale to 1024 nsec
pub const IGC_LTRMAXV_SCALE_1024: c_int = 2;
// Reg val to set scale to 32768 nsec
pub const IGC_LTRMAXV_SCALE_32768: c_int = 3;
pub const IGC_LTRMINV_LTRV_MASK: c_uint = 0x000003FF /* LTR minimum value */;
pub const IGC_LTRMAXV_LTRV_MASK: c_uint = 0x000003FF /* LTR maximum value */;
pub const IGC_LTRMINV_LSNP_REQ: c_uint = 0x00008000 /* LTR Snoop Requirement */;
pub const IGC_LTRMINV_SCALE_SHIFT: c_int = 10;
pub const IGC_LTRMAXV_LSNP_REQ: c_uint = 0x00008000 /* LTR Snoop Requirement */;
pub const IGC_LTRMAXV_SCALE_SHIFT: c_int = 10;
