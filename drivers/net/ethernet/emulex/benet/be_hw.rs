//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/emulex/benet/be_hw.h
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
// Copyright (C) 2005-2016 Broadcom.
// All rights reserved.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//
// Mailbox door bell
// Used for driver communication with the FW.
// The software must write this register twice to post any command. First,
// it writes the register with hi=1 and the upper bits of the physical address
// for the MAILBOX structure. Software must poll the ready bit until this
// is acknowledged. Then, software writes the register with hi=0 with the lower
// bits in the address. It must poll the ready bit until the command is
// complete. Upon completion, the MAILBOX will contain a valid completion
// queue entry.
//
pub const MPU_MAILBOX_DB_OFFSET: c_uint = 0x160;
pub const MPU_MAILBOX_DB_RDY_MASK: c_uint = 0x1 	/* bit 0 */;
pub const MPU_MAILBOX_DB_HI_MASK: c_uint = 0x2	/* bit 1 */;
pub const MPU_EP_CONTROL: c_int = 0;
// MPU semaphore: used for SH & BE
pub const SLIPORT_SOFTRESET_OFFSET: c_uint = 0x5c	/* CSR BAR offset */;
pub const SLIPORT_SEMAPHORE_OFFSET_BEx: c_uint = 0xac  /* CSR BAR offset */;
pub const SLIPORT_SEMAPHORE_OFFSET_SH: c_uint = 0x94  /* PCI-CFG offset */;
pub const POST_STAGE_MASK: c_uint = 0x0000FFFF;
pub const POST_ERR_MASK: c_uint = 0x1;
pub const POST_ERR_SHIFT: c_int = 31;
pub const POST_ERR_RECOVERY_CODE_MASK: c_uint = 0xFFF;
// Soft Reset register masks
pub const SLIPORT_SOFTRESET_SR_MASK: c_uint = 0x00000080	/* SR bit */;
// MPU semaphore POST stage values
pub const POST_STAGE_AWAITING_HOST_RDY: c_uint = 0x1 /* FW awaiting goahead from host */;
pub const POST_STAGE_HOST_RDY: c_uint = 0x2 /* Host has given go-ahed to FW */;
pub const POST_STAGE_BE_RESET: c_uint = 0x3 /* Host wants to reset chip */;
pub const POST_STAGE_ARMFW_RDY: c_uint = 0xc000	/* FW is done with POST */;
pub const POST_STAGE_RECOVERABLE_ERR: c_uint = 0xE000	/* Recoverable err detected */;
// FW has detected a UE and is dumping FAT log data
pub const POST_STAGE_FAT_LOG_START: c_uint = 0x0D00;
pub const POST_STAGE_ARMFW_UE: c_uint = 0xF000  /*FW has asserted an UE*/;
// Lancer SLIPORT registers
pub const SLIPORT_STATUS_OFFSET: c_uint = 0x404;
pub const SLIPORT_CONTROL_OFFSET: c_uint = 0x408;
pub const SLIPORT_ERROR1_OFFSET: c_uint = 0x40C;
pub const SLIPORT_ERROR2_OFFSET: c_uint = 0x410;
pub const PHYSDEV_CONTROL_OFFSET: c_uint = 0x414;
pub const SLIPORT_STATUS_ERR_MASK: c_uint = 0x80000000;
pub const SLIPORT_STATUS_DIP_MASK: c_uint = 0x02000000;
pub const SLIPORT_STATUS_RN_MASK: c_uint = 0x01000000;
pub const SLIPORT_STATUS_RDY_MASK: c_uint = 0x00800000;
pub const SLI_PORT_CONTROL_IP_MASK: c_uint = 0x08000000;
pub const PHYSDEV_CONTROL_FW_RESET_MASK: c_uint = 0x00000002;
pub const PHYSDEV_CONTROL_DD_MASK: c_uint = 0x00000004;
pub const PHYSDEV_CONTROL_INP_MASK: c_uint = 0x40000000;
pub const SLIPORT_ERROR_NO_RESOURCE1: c_uint = 0x2;
pub const SLIPORT_ERROR_NO_RESOURCE2: c_uint = 0x9;
pub const SLIPORT_ERROR_FW_RESET1: c_uint = 0x2;
pub const SLIPORT_ERROR_FW_RESET2: c_uint = 0x0;
// Memory BAR register
pub const PCICFG_MEMBAR_CTRL_INT_CTRL_OFFSET: c_uint = 0xfc;
// Host Interrupt Enable, if set interrupts are enabled although "PCI Interrupt
// Disable" may still globally block interrupts in addition to individual
// interrupt masks; a mechanism for the device driver to block all interrupts
// atomically without having to arbitrate for the PCI Interrupt Disable bit
// with the OS.
//

// PCI Function Capability
pub const BE_FUNCTION_CAPS_RSS: c_uint = 0x2;
pub const BE_FUNCTION_CAPS_SUPER_NIC: c_uint = 0x40;
// Power management (WOL)
pub const PCICFG_PM_CONTROL_OFFSET: c_uint = 0x44;
pub const PCICFG_PM_CONTROL_MASK: c_uint = 0x108	/* bits 3 & 8 */;
// Online Control Registers
pub const PCICFG_ONLINE0: c_uint = 0xB0;
pub const PCICFG_ONLINE1: c_uint = 0xB4;
// UE Status and Mask Registers
pub const PCICFG_UE_STATUS_LOW: c_uint = 0xA0;
pub const PCICFG_UE_STATUS_HIGH: c_uint = 0xA4;
pub const PCICFG_UE_STATUS_LOW_MASK: c_uint = 0xA8;
pub const PCICFG_UE_STATUS_HI_MASK: c_uint = 0xAC;
// SLI_INTF
pub const SLI_INTF_REG_OFFSET: c_uint = 0x58;
pub const SLI_INTF_VALID_MASK: c_uint = 0xE0000000;
pub const SLI_INTF_VALID: c_uint = 0xC0000000;
pub const SLI_INTF_HINT2_MASK: c_uint = 0x1F000000;
pub const SLI_INTF_HINT2_SHIFT: c_int = 24;
pub const SLI_INTF_HINT1_MASK: c_uint = 0x00FF0000;
pub const SLI_INTF_HINT1_SHIFT: c_int = 16;
pub const SLI_INTF_FAMILY_MASK: c_uint = 0x00000F00;
pub const SLI_INTF_FAMILY_SHIFT: c_int = 8;
pub const SLI_INTF_IF_TYPE_MASK: c_uint = 0x0000F000;
pub const SLI_INTF_IF_TYPE_SHIFT: c_int = 12;
pub const SLI_INTF_REV_MASK: c_uint = 0x000000F0;
pub const SLI_INTF_REV_SHIFT: c_int = 4;
pub const SLI_INTF_FT_MASK: c_uint = 0x00000001;
pub const SLI_INTF_TYPE_2: c_int = 2;
pub const SLI_INTF_TYPE_3: c_int = 3;
// ISR0 Register offset
pub const CEV_ISR0_OFFSET: c_uint = 0xC18;
pub const CEV_ISR_SIZE: c_int = 4;
// Event Q door bell

pub const DB_EQ_RING_ID_MASK: c_uint = 0x1FF	/* bits 0 - 8 */;
pub const DB_EQ_RING_ID_EXT_MASK: c_uint = 0x3e00  /* bits 9-13 */;

// Clear the interrupt for this eq

// Must be 1

// Number of event entries processed

// Rearm bit

// Rearm to interrupt delay encoding

// Rearm to interrupt (R2I) delay multiplier encoding represents 3 different
// values configured in CEV_REARM2IRPT_DLY_MULT_CSR register. This value is
// programmed by host driver while ringing an EQ doorbell(EQ_DB) if a delay
// between rearming the EQ and next interrupt on this EQ is desired.
//

// Compl Q door bell
pub const DB_CQ_OFFSET: c_uint = 0x120;
pub const DB_CQ_RING_ID_MASK: c_uint = 0x3FF	/* bits 0 - 9 */;
pub const DB_CQ_RING_ID_EXT_MASK: c_uint = 0x7C00	/* bits 10-14 */;

// Number of event entries processed

// Rearm bit

// TX ULP door bell
pub const DB_TXULP1_OFFSET: c_uint = 0x60;
pub const DB_TXULP_RING_ID_MASK: c_uint = 0x7FF	/* bits 0 - 10 */;
// Number of tx entries posted

pub const DB_TXULP_NUM_POSTED_MASK: c_uint = 0x3FFF	/* bits 16 - 29 */;
// RQ(erx) door bell
pub const DB_RQ_OFFSET: c_uint = 0x100;
pub const DB_RQ_RING_ID_MASK: c_uint = 0x3FF	/* bits 0 - 9 */;
// Number of rx frags posted

// MCC door bell
pub const DB_MCCQ_OFFSET: c_uint = 0x140;
pub const DB_MCCQ_RING_ID_MASK: c_uint = 0x7FF	/* bits 0 - 10 */;
// Number of entries posted

// SRIOV VF PCICFG OFFSET

// FAT TABLE
pub const RETRIEVE_FAT: c_int = 0;
pub const QUERY_FAT: c_int = 1;
// Rx Packet Type Encoding
pub const BE_UNICAST_PACKET: c_int = 0;
pub const BE_MULTICAST_PACKET: c_int = 1;
pub const BE_BROADCAST_PACKET: c_int = 2;
pub const BE_RSVD_PACKET: c_int = 3;
//
// BE descriptors: host memory data structures whose formats
// are hardwired in BE silicon.
//
// Event Queue Descriptor
pub const EQ_ENTRY_VALID_MASK: c_uint = 0x1	/* bit 0 */;
pub const EQ_ENTRY_RES_ID_MASK: c_uint = 0xFFFF	/* bits 16 - 31 */;
pub const EQ_ENTRY_RES_ID_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eq_entry {
    pub evt: u32,
}

// TX Queue Descriptor
pub const ETH_WRB_FRAG_LEN_MASK: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_wrb {
    pub /: *mut *mut __le32 frag_pa_hi; / dword 0,
    pub /: *mut *mut __le32 frag_pa_lo; / dword 1,
    pub /: *mut *mut u32 rsvd0; / dword 2,
    pub /: *mut *mut __le32 frag_len; / dword 3: bits 0 - 15,
    pub __packed: },
// Pseudo amap definition for eth_hdr_wrb in which each bit of the
// actual structure is defined as a byte : used to calculate
// offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eth_hdr_wrb {
    pub /: *mut *mut u8 rsvd0[32]; / dword 0,
    pub /: *mut *mut u8 rsvd1[32]; / dword 1,
    pub /: *mut *mut u8 complete; / dword 2,
    pub event: u8,
    pub crc: u8,
    pub forward: u8,
    pub lso6: u8,
    pub mgmt: u8,
    pub ipcs: u8,
    pub udpcs: u8,
    pub tcpcs: u8,
    pub lso: u8,
    pub vlan: u8,
    pub gso: [u8; 2],
    pub num_wrb: [u8; 5],
    pub lso_mss: [u8; 14],
    pub /: *mut *mut u8 len[16]; / dword 3,
    pub vlan_tag: [u8; 16],
    pub __packed: },

pub const TX_HDR_WRB_NUM_MASK: c_uint = 0x1F		/* word 2: bits 13:17 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_hdr_wrb {
    pub dw: [__le32; 4],
}

// Tx Compl Status Encoding
pub const BE_TX_COMP_HDR_PARSE_ERR: c_uint = 0x2;
pub const BE_TX_COMP_NDMA_ERR: c_uint = 0x3;
pub const BE_TX_COMP_ACL_ERR: c_uint = 0x5;
pub const LANCER_TX_COMP_LSO_ERR: c_uint = 0x1;
pub const LANCER_TX_COMP_HSW_DROP_MAC_ERR: c_uint = 0x3;
pub const LANCER_TX_COMP_HSW_DROP_VLAN_ERR: c_uint = 0x5;
pub const LANCER_TX_COMP_QINQ_ERR: c_uint = 0x7;
pub const LANCER_TX_COMP_SGE_ERR: c_uint = 0x9;
pub const LANCER_TX_COMP_PARITY_ERR: c_uint = 0xb;
pub const LANCER_TX_COMP_DMA_ERR: c_uint = 0xd;
// TX Compl Queue Descriptor
// Pseudo amap definition for eth_tx_compl in which each bit of the
// actual structure is defined as a byte: used to calculate
// offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eth_tx_compl {
    pub /: *mut *mut u8 wrb_index[16]; / dword 0,
    pub /: *mut *mut u8 ct[2]; / dword 0,
    pub /: *mut *mut u8 port[2]; / dword 0,
    pub /: *mut *mut u8 rsvd0[8]; / dword 0,
    pub /: *mut *mut u8 status[4]; / dword 0,
    pub /: *mut *mut u8 user_bytes[16]; / dword 1,
    pub /: *mut *mut u8 nwh_bytes[8]; / dword 1,
    pub /: *mut *mut u8 lso; / dword 1,
    pub /: *mut *mut u8 cast_enc[2]; / dword 1,
    pub /: *mut *mut u8 rsvd1[5]; / dword 1,
    pub /: *mut *mut u8 rsvd2[32]; / dword 2,
    pub /: *mut *mut u8 pkts[16]; / dword 3,
    pub /: *mut *mut u8 ringid[11]; / dword 3,
    pub /: *mut *mut u8 hash_val[4]; / dword 3,
    pub /: *mut *mut u8 valid; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_tx_compl {
    pub dw: [u32; 4],
}

// RX Queue Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_rx_d {
    pub fragpa_hi: u32,
    pub fragpa_lo: u32,
}

// RX Compl Queue Descriptor
// Pseudo amap definition for BE2 and BE3 legacy mode eth_rx_compl in which
// each bit of the actual structure is defined as a byte: used to calculate
// offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eth_rx_compl_v0 {
    pub /: *mut *mut u8 vlan_tag[16]; / dword 0,
    pub /: *mut *mut u8 pktsize[14]; / dword 0,
    pub /: *mut *mut u8 port; / dword 0,
    pub /: *mut *mut u8 ip_opt; / dword 0,
    pub /: *mut *mut u8 err; / dword 1,
    pub /: *mut *mut u8 rsshp; / dword 1,
    pub /: *mut *mut u8 ipf; / dword 1,
    pub /: *mut *mut u8 tcpf; / dword 1,
    pub /: *mut *mut u8 udpf; / dword 1,
    pub /: *mut *mut u8 ipcksm; / dword 1,
    pub /: *mut *mut u8 l4_cksm; / dword 1,
    pub /: *mut *mut u8 ip_version; / dword 1,
    pub /: *mut *mut u8 macdst[6]; / dword 1,
    pub /: *mut *mut u8 vtp; / dword 1,
    pub /: *mut *mut u8 ip_frag; / dword 1,
    pub /: *mut *mut u8 fragndx[10]; / dword 1,
    pub /: *mut *mut u8 ct[2]; / dword 1,
    pub /: *mut *mut u8 sw; / dword 1,
    pub /: *mut *mut u8 numfrags[3]; / dword 1,
    pub /: *mut *mut u8 rss_flush; / dword 2,
    pub /: *mut *mut u8 cast_enc[2]; / dword 2,
    pub /: *mut *mut u8 qnq; / dword 2,
    pub /: *mut *mut u8 rss_bank; / dword 2,
    pub /: *mut *mut u8 rsvd1[23]; / dword 2,
    pub /: *mut *mut u8 lro_pkt; / dword 2,
    pub /: *mut *mut u8 rsvd2[2]; / dword 2,
    pub /: *mut *mut u8 valid; / dword 2,
    pub /: *mut *mut u8 rsshash[32]; / dword 3,
    pub __packed: },
// Pseudo amap definition for BE3 native mode eth_rx_compl in which
// each bit of the actual structure is defined as a byte: used to calculate
// offset/shift/mask of each field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eth_rx_compl_v1 {
    pub /: *mut *mut u8 vlan_tag[16]; / dword 0,
    pub /: *mut *mut u8 pktsize[14]; / dword 0,
    pub /: *mut *mut u8 vtp; / dword 0,
    pub /: *mut *mut u8 ip_opt; / dword 0,
    pub /: *mut *mut u8 err; / dword 1,
    pub /: *mut *mut u8 rsshp; / dword 1,
    pub /: *mut *mut u8 ipf; / dword 1,
    pub /: *mut *mut u8 tcpf; / dword 1,
    pub /: *mut *mut u8 udpf; / dword 1,
    pub /: *mut *mut u8 ipcksm; / dword 1,
    pub /: *mut *mut u8 l4_cksm; / dword 1,
    pub /: *mut *mut u8 ip_version; / dword 1,
    pub /: *mut *mut u8 macdst[7]; / dword 1,
    pub /: *mut *mut u8 rsvd0; / dword 1,
    pub /: *mut *mut u8 fragndx[10]; / dword 1,
    pub /: *mut *mut u8 ct[2]; / dword 1,
    pub /: *mut *mut u8 sw; / dword 1,
    pub /: *mut *mut u8 numfrags[3]; / dword 1,
    pub /: *mut *mut u8 rss_flush; / dword 2,
    pub /: *mut *mut u8 cast_enc[2]; / dword 2,
    pub /: *mut *mut u8 qnq; / dword 2,
    pub /: *mut *mut u8 rss_bank; / dword 2,
    pub /: *mut *mut u8 port[2]; / dword 2,
    pub /: *mut *mut u8 vntagp; / dword 2,
    pub /: *mut *mut u8 header_len[8]; / dword 2,
    pub /: *mut *mut u8 header_split[2]; / dword 2,
    pub /: *mut *mut u8 rsvd1[12]; / dword 2,
    pub tunneled: u8,
    pub /: *mut *mut u8 valid; / dword 2,
    pub /: *mut *mut u8 rsshash[32]; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_rx_compl {
    pub dw: [u32; 4],
}
