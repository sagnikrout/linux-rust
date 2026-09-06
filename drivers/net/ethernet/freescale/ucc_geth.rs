//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/ucc_geth.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) Freescale Semicondutor, Inc. 2006-2009. All rights reserved.
//
// Author: Shlomi Gridish <gridish@freescale.com>
//
// Description:
// Internal header file for UCC Gigabit Ethernet unit routines.
//
// Changelog:
// Jun 28, 2006 Li Yang <LeoLi@freescale.com>
// - Rearrange code and style fixes
//

pub const NUM_TX_QUEUES: c_int = 8;
pub const NUM_RX_QUEUES: c_int = 8;
pub const NUM_BDS_IN_PREFETCHED_BDS: c_int = 4;
pub const TX_IP_OFFSET_ENTRY_MAX: c_int = 8;
pub const NUM_OF_PADDRS: c_int = 4;
pub const ENET_INIT_PARAM_MAX_ENTRIES_RX: c_int = 9;
pub const ENET_INIT_PARAM_MAX_ENTRIES_TX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth {
    pub uccf: ucc_fast,
    pub ucc_fast)]: u8 res0[0x100 - sizeof(struct,
    pub /: *mut *mut u32 maccfg1; / mac configuration reg. 1,
    pub /: *mut *mut u32 maccfg2; / mac configuration reg. 2,
    pub /: *mut *mut u32 ipgifg; / interframe gap reg.,
    pub /: *mut *mut u32 hafdup; / half-duplex reg.,
    pub res1: [u8; 0x10],
    pub /: *mut *mut u8 miimng[0x18]; / MII management structure moved to _mii.h,
    pub /: *mut *mut u32 ifctl; / interface control reg,
    pub /: *mut *mut u32 ifstat; / interface statux reg,
    pub /: *mut *mut u32 macstnaddr1; / mac station address part 1 reg,
    pub /: *mut *mut u32 macstnaddr2; / mac station address part 2 reg,
    pub res2: [u8; 0x8],
    pub /: *mut *mut u32 uempr; / UCC Ethernet Mac parameter reg,
    pub /: *mut *mut u32 utbipar; / UCC tbi address reg,
    pub /: *mut *mut u16 uescr; / UCC Ethernet statistics control reg,
    pub 0x15A]: u8 res3[0x180 -,
    pub bad: *mut *mut u32 tx64; / Total number of frames (including,
    pub bad: *mut *mut u32 tx127; / Total number of frames (including,
    pub bad: *mut *mut u32 tx255; / Total number of frames (including,
    pub including: *mut *mut u32 rx64; / Total number of frames received,
    pub bad: *mut *mut u32 rx127; / Total number of frames (including,
    pub bad: *mut *mut u32 rx255; / Total number of frames (including,
    pub frames: *mut *mut u32 txok; / Total number of octets residing in,
    pub frames: *mut *mut u16 txcf; / Total number of PAUSE control,
    pub res4: [u8; 0x2],
    pub transmitted: *mut *mut u32 tmca; / Total number of frames that were,
    pub transmitted: *mut *mut u32 tbca; / Total number of frames,
    pub /: *mut *mut u32 rxfok; / Total number of frames received OK,
    pub /: *mut *mut u32 rxbok; / Total number of octets received OK,
    pub including: *mut *mut u32 rbyt; / Total number of octets received,
    pub received: *mut *mut u32 rmca; / Total number of frames that were,
    pub successfully: *mut *mut u32 rbca; / Total number of frames received,
    pub /: *mut *mut u32 scar; / Statistics carry register,
    pub /: *mut *mut u32 scam; / Statistics caryy mask register,
    pub 0x1c4]: u8 res5[0x200 -,
    pub __packed: },
// UCC GETH TEMODR Register
pub const TEMODER_TX_RMON_STATISTICS_ENABLE: c_uint = 0x0100	/* enable Tx statistics;
//
pub const TEMODER_SCHEDULER_ENABLE: c_uint = 0x2000	/* enable scheduler */;
pub const TEMODER_IP_CHECKSUM_GENERATE: c_uint = 0x0400	/* generate IPv4;
pub const TEMODER_PERFORMANCE_OPTIMIZATION_MODE1: c_uint = 0x0200	/* enable performance;
pub const TEMODER_RMON_STATISTICS: c_uint = 0x0100	/* enable tx statistics;
//

// UCC GETH TEMODR Register
pub const REMODER_RX_RMON_STATISTICS_ENABLE: c_uint = 0x00001000	/* enable Rx;
pub const REMODER_RX_EXTENDED_FEATURES: c_uint = 0x80000000	/* enable;

//
pub const REMODER_RMON_STATISTICS: c_uint = 0x00001000	/* enable rx;
pub const REMODER_RX_EXTENDED_FILTERING: c_uint = 0x00000800	/* extended;

pub const REMODER_DYNAMIC_MAX_FRAME_LENGTH: c_uint = 0x00000008	/* enable;
//
pub const REMODER_DYNAMIC_MIN_FRAME_LENGTH: c_uint = 0x00000004	/* enable;
//
pub const REMODER_IP_CHECKSUM_CHECK: c_uint = 0x00000002	/* check IPv4;
pub const REMODER_IP_ADDRESS_ALIGNMENT: c_uint = 0x00000001	/* align ip;
// UCC GETH Event Register

// TBI defines
pub const ENET_TBI_MII_CR: c_uint = 0x00	/* Control */;
pub const ENET_TBI_MII_SR: c_uint = 0x01	/* Status */;
pub const ENET_TBI_MII_ANA: c_uint = 0x04	/* AN advertisement */;
pub const ENET_TBI_MII_ANLPBPA: c_uint = 0x05	/* AN link partner base page ability */;
pub const ENET_TBI_MII_ANEX: c_uint = 0x06	/* AN expansion */;
pub const ENET_TBI_MII_ANNPT: c_uint = 0x07	/* AN next page transmit */;
pub const ENET_TBI_MII_ANLPANP: c_uint = 0x08	/* AN link partner ability next page */;
pub const ENET_TBI_MII_EXST: c_uint = 0x0F	/* Extended status */;
pub const ENET_TBI_MII_JD: c_uint = 0x10	/* Jitter diagnostics */;
pub const ENET_TBI_MII_TBICON: c_uint = 0x11	/* TBI control */;
// TBI MDIO register bit fields
pub const TBISR_LSTATUS: c_uint = 0x0004;
pub const TBICON_CLK_SELECT: c_uint = 0x0020;
pub const TBIANA_ASYMMETRIC_PAUSE: c_uint = 0x0100;
pub const TBIANA_SYMMETRIC_PAUSE: c_uint = 0x0080;
pub const TBIANA_HALF_DUPLEX: c_uint = 0x0040;
pub const TBIANA_FULL_DUPLEX: c_uint = 0x0020;
pub const TBICR_PHY_RESET: c_uint = 0x8000;
pub const TBICR_ANEG_ENABLE: c_uint = 0x1000;
pub const TBICR_RESTART_ANEG: c_uint = 0x0200;
pub const TBICR_FULL_DUPLEX: c_uint = 0x0100;
pub const TBICR_SPEED1_SET: c_uint = 0x0040;

// UCC GETH MACCFG1 (MAC Configuration 1 Register)
pub const MACCFG1_FLOW_RX: c_uint = 0x00000020	/* Flow Control;
pub const MACCFG1_FLOW_TX: c_uint = 0x00000010	/* Flow Control;
pub const MACCFG1_ENABLE_SYNCHED_RX: c_uint = 0x00000008	/* Rx Enable;
//
pub const MACCFG1_ENABLE_RX: c_uint = 0x00000004	/* Enable Rx */;
pub const MACCFG1_ENABLE_SYNCHED_TX: c_uint = 0x00000002	/* Tx Enable;
//
pub const MACCFG1_ENABLE_TX: c_uint = 0x00000001	/* Enable Tx */;
// UCC GETH MACCFG2 (MAC Configuration 2 Register)

pub const MACCFG2_PREL_MASK: c_uint = 0x0000f000	/* Preamble;
pub const MACCFG2_SRP: c_uint = 0x00000080	/* Soft Receive;
pub const MACCFG2_STP: c_uint = 0x00000040	/* Soft;
pub const MACCFG2_RESERVED_1: c_uint = 0x00000020	/* Reserved -;
pub const MACCFG2_LC: c_uint = 0x00000010	/* Length Check;
//
pub const MACCFG2_MPE: c_uint = 0x00000008	/* Magic packet;
pub const MACCFG2_FDX: c_uint = 0x00000001	/* Full Duplex */;
pub const MACCFG2_FDX_MASK: c_uint = 0x00000001	/* Full Duplex;
pub const MACCFG2_PAD_CRC: c_uint = 0x00000004;
pub const MACCFG2_CRC_EN: c_uint = 0x00000002;
pub const MACCFG2_PAD_AND_CRC_MODE_NONE: c_uint = 0x00000000	/* Neither;
pub const MACCFG2_PAD_AND_CRC_MODE_CRC_ONLY: c_uint = 0x00000002	/* Append CRC;
pub const MACCFG2_PAD_AND_CRC_MODE_PAD_AND_CRC: c_uint = 0x00000004;
pub const MACCFG2_INTERFACE_MODE_NIBBLE: c_uint = 0x00000100	/* nibble mode;
pub const MACCFG2_INTERFACE_MODE_BYTE: c_uint = 0x00000200	/* byte mode;
pub const MACCFG2_INTERFACE_MODE_MASK: c_uint = 0x00000300	/* mask;
// UCC GETH IPGIFG (Inter-frame Gap / Inter-Frame Gap Register)

//

pub const IPGIFG_NBTB_CS_IPG_MASK: c_uint = 0x7F000000;
pub const IPGIFG_NBTB_IPG_MASK: c_uint = 0x007F0000;
pub const IPGIFG_MIN_IFG_MASK: c_uint = 0x0000FF00;
pub const IPGIFG_BTB_IPG_MASK: c_uint = 0x0000007F;
// UCC GETH HAFDUP (Half Duplex Register)

pub const HALFDUP_ALT_BEB_TRUNCATION_MAX: c_uint = 0xf	/* Alternate Binary;
pub const HALFDUP_ALT_BEB: c_uint = 0x00080000	/* Alternate;
pub const HALFDUP_BACK_PRESSURE_NO_BACKOFF: c_uint = 0x00040000	/* Back;
pub const HALFDUP_NO_BACKOFF: c_uint = 0x00020000	/* No Backoff */;
pub const HALFDUP_EXCESSIVE_DEFER: c_uint = 0x00010000	/* Excessive;

pub const HALFDUP_MAX_RETRANSMISSION_MAX: c_uint = 0xf	/* Maximum;

pub const HALFDUP_COLLISION_WINDOW_MAX: c_uint = 0x3f	/* Collision Window max;
pub const HALFDUP_ALT_BEB_TR_MASK: c_uint = 0x00F00000;
pub const HALFDUP_RETRANS_MASK: c_uint = 0x0000F000;
pub const HALFDUP_COL_WINDOW_MASK: c_uint = 0x0000003F;
// UCC GETH UCCS (Ethernet Status Register)
pub const UCCS_BPR: c_uint = 0x02	/* Back pressure (in;
pub const UCCS_PAU: c_uint = 0x02	/* Pause state (in full;
pub const UCCS_MPD: c_uint = 0x01	/* Magic Packet;
// UCC GETH IFSTAT (Interface Status Register)
pub const IFSTAT_EXCESS_DEFER: c_uint = 0x00000200	/* Excessive;
// UCC GETH MACSTNADDR1 (Station Address Part 1 Register)

// UCC GETH MACSTNADDR2 (Station Address Part 2 Register)

// UCC GETH UEMPR (Ethernet Mac Parameter Register)

// UCC GETH UTBIPAR (Ten Bit Interface Physical Address Register)

pub const UTBIPAR_PHY_ADDRESS_MASK: c_uint = 0x0000001f	/* Phy address;
// UCC GETH UESCR (Ethernet Statistics Control Register)
pub const UESCR_AUTOZ: c_uint = 0x8000	/* Automatically zero;
pub const UESCR_CLRCNT: c_uint = 0x4000	/* Clear all statistics;

// UCC GETH UDSR (Data Synchronization Register)
pub const UDSR_MAGIC: c_uint = 0x067E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_thread_data_tx {
    pub res0: [u8; 104],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_thread_data_rx {
    pub res0: [u8; 40],
    pub __packed: },
// Send Queue Queue-Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_send_queue_qd {
    pub /: *mut *mut u32 bd_ring_base; / pointer to BD ring base address,
    pub res0: [u8; 0x8],
    pub /: *mut *mut u32 last_bd_completed_address;/ initialize to last entry in BD ring,
    pub res1: [u8; 0x30],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_send_queue_mem_region {
    pub sqqd: [ucc_geth_send_queue_qd; NUM_TX_QUEUES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_thread_tx_pram {
    pub res0: [u8; 64],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_thread_rx_pram {
    pub res0: [u8; 128],
    pub __packed: },
pub const THREAD_RX_PRAM_ADDITIONAL_FOR_EXTENDED_FILTERING: c_int = 64;
pub const THREAD_RX_PRAM_ADDITIONAL_FOR_EXTENDED_FILTERING_8: c_int = 64;
pub const THREAD_RX_PRAM_ADDITIONAL_FOR_EXTENDED_FILTERING_16: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_scheduler {
    pub /: *mut *mut u16 cpucount0; / CPU packet counter,
    pub /: *mut *mut u16 cpucount1; / CPU packet counter,
    pub /: *mut *mut u16 cecount0; / QE packet counter,
    pub /: *mut *mut u16 cecount1; / QE packet counter,
    pub /: *mut *mut u16 cpucount2; / CPU packet counter,
    pub /: *mut *mut u16 cpucount3; / CPU packet counter,
    pub /: *mut *mut u16 cecount2; / QE packet counter,
    pub /: *mut *mut u16 cecount3; / QE packet counter,
    pub /: *mut *mut u16 cpucount4; / CPU packet counter,
    pub /: *mut *mut u16 cpucount5; / CPU packet counter,
    pub /: *mut *mut u16 cecount4; / QE packet counter,
    pub /: *mut *mut u16 cecount5; / QE packet counter,
    pub /: *mut *mut u16 cpucount6; / CPU packet counter,
    pub /: *mut *mut u16 cpucount7; / CPU packet counter,
    pub /: *mut *mut u16 cecount6; / QE packet counter,
    pub /: *mut *mut u16 cecount7; / QE packet counter,
    pub /: *mut *mut u32 weightstatus[NUM_TX_QUEUES]; / accumulated weight factor,
    pub /: *mut *mut u32 rtsrshadow; / temporary variable handled by QE,
    pub /: *mut *mut u32 time; / temporary variable handled by QE,
    pub /: *mut *mut u32 ttl; / temporary variable handled by QE,
    pub /: *mut *mut u32 mblinterval; / max burst length interval,
    pub /: *mut *mut u16 nortsrbytetime; / normalized value of byte time in tsr units,
    pub of: *mut *mut u8 fracsiz; / radix 2 log value of denom.,
    pub res0: [u8; 1],
    pub /: *mut *mut u8 strictpriorityq; / Strict Priority Mask register,
    pub /: *mut *mut u8 txasap; / Transmit ASAP register,
    pub /: *mut *mut u8 extrabw; / Extra BandWidth register,
    pub /: *mut *mut u8 oldwfqmask; / temporary variable handled by QE,
    pub weightfactor: [u8; NUM_TX_QUEUES],
// < weight factor for queues
    pub /: *mut *mut u32 minw; / temporary variable handled by QE,
    pub 0x64]: u8 res1[0x70 -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_tx_firmware_statistics_pram {
    pub /: *mut *mut u32 sicoltx; / single collision,
    pub /: *mut *mut u32 mulcoltx; / multiple collision,
    pub /: *mut *mut u32 latecoltxfr; / late collision,
    pub /: *mut *mut u32 frabortduecol; / frames aborted due to transmit collision,
    pub error: *mut *mut u32 frlostinmactxer; / frames lost due to internal MAC,
    pub /: *mut *mut u32 carriersenseertx; / carrier sense error,
    pub /: *mut *mut u32 frtxok; / frames transmitted OK,
    pub than: *mut *mut u32 txfrexcessivedefer; / frames with defferal time greater,
    pub 256: *mut *mut u32 txpkts256; / total packets (including bad) between,
    pub 512: *mut *mut u32 txpkts512; / total packets (including bad) between,
    pub 1024: *mut *mut u32 txpkts1024; / total packets (including bad) between,
    pub 1024: *mut *mut u32 txpktsjumbo; / total packets (including bad) between,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_firmware_statistics_pram {
    pub /: *mut *mut u32 frrxfcser; / frames with crc error,
    pub /: *mut *mut u32 fraligner; / frames with alignment error,
    pub /: *mut *mut u32 inrangelenrxer; / in range length error,
    pub /: *mut *mut u32 outrangelenrxer; / out of range length error,
    pub /: *mut *mut u32 frtoolong; / frame too long,
    pub /: *mut *mut u32 runt; / runt,
    pub /: *mut *mut u32 verylongevent; / very long event,
    pub /: *mut *mut u32 symbolerror; / symbol error,
    pub /: *mut *mut u32 dropbsy; / drop because of BD not ready,
    pub res0: [u8; 0x8],
    pub address: *mut *mut u32 mismatchdrop; / drop because of MAC filtering (e.g.,
    pub /: *mut *mut u32 underpkts; / total frames less than 64 octets,
    pub and: *mut *mut u32 pkts256; / total frames (including bad) between 256,
    pub and: *mut *mut u32 pkts512; / total frames (including bad) between 512,
    pub 1024: *mut *mut u32 pkts1024; / total frames (including bad) between,
    pub 1024: *mut *mut u32 pktsjumbo; / total frames (including bad) between,
    pub error: *mut *mut u32 frlossinmacer; / frames lost because of internal MAC,
    pub /: *mut *mut u32 pausefr; / pause frames,
    pub res1: [u8; 0x4],
    pub removed: *mut *mut u32 removevlan; / total frames that had their VLAN tag,
//
    pub tag: *mut *mut u32 replacevlan; / total frames that had their VLAN,
    pub tag: *mut *mut u32 insertvlan; / total frames that had their VLAN,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_interrupt_coalescing_entry {
    pub max: *mut *mut u32 interruptcoalescingmaxvalue; / interrupt coalescing,
    pub counter,: *mut *mut u32 interruptcoalescingcounter; / interrupt coalescing,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_interrupt_coalescing_table {
    pub coalescingentry: [ucc_geth_rx_interrupt_coalescing_entry; NUM_RX_QUEUES],
// < interrupt coalescing entry
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_prefetched_bds {
    pub /: *mut *mut qe_bd bd[NUM_BDS_IN_PREFETCHED_BDS]; / prefetched bd,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_bd_queues_entry {
    pub /: *mut *mut u32 bdbaseptr; / BD base pointer,
    pub /: *mut *mut u32 bdptr; / BD pointer,
    pub /: *mut *mut u32 externalbdbaseptr; / external BD base pointer,
    pub /: *mut *mut u32 externalbdptr; / external BD pointer,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_tx_global_pram {
    pub temoder: u16,
    pub 0x02]: u8 res0[0x38 -,
    pub /: *mut *mut u32 sqptr; / a base pointer to send queue memory region,
    pub memory: *mut *mut u32 schedulerbasepointer; / a base pointer to scheduler,
    pub /: *mut *mut u32 txrmonbaseptr; / base pointer to Tx RMON statistics counter,
    pub contains: *mut *mut u32 tstate; / tx internal state. High byte,
    pub iphoffset: [u8; TX_IP_OFFSET_ENTRY_MAX],
    pub /: *mut *mut u32 vtagtable[0x8]; / 8 4-byte VLAN tags,
    pub Memory: *mut *mut u32 tqptr; / a base pointer to the Tx Queues,
    pub 0x74]: u8 res2[0x78 -,
    pub snums_en: u64,
    pub /: *mut *mut u32 l2l3baseptr; / top byte consists of a few other bit fields,
    pub mtu: [u16; 8],
    pub 0x94]: u8 res3[0xa8 -,
    pub /: *mut *mut u32 wrrtablebase; / top byte is reserved,
    pub 0xac]: u8 res4[0xc0 -,
    pub __packed: },
// structure representing Extended Filtering Global Parameters in PRAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_exf_global_pram {
    pub /: *mut *mut u32 l2pcdptr; / individual address filter, high,
    pub 0x04]: u8 res0[0x10 -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_global_pram {
    pub /: *mut *mut u32 remoder; / ethernet mode reg.,
    pub Region*/: *mut *mut u32 rqptr; / base pointer to the Rx Queues Memory,
    pub res0: [u32; 0x1],
    pub 0xC]: u8 res1[0x20 -,
    pub field: *mut *mut u16 typeorlen; / cutoff point less than which, type/len,
    pub res2: [u8; 0x1],
    pub command*/: *mut *mut u8 rxgstpack; / acknowledgement on GRACEFUL STOP RX,
    pub /: *mut *mut u32 rxrmonbaseptr; / base pointer to Rx RMON statistics counter,
    pub 0x28]: u8 res3[0x30 -,
    pub /: *mut *mut u32 intcoalescingptr; / Interrupt coalescing table pointer,
    pub 0x34]: u8 res4[0x36 -,
    pub contains: *mut *mut u8 rstate; / rx internal state. High byte,
    pub 0x37]: u8 res5[0x46 -,
    pub /: *mut *mut u16 mrblr; / max receive buffer length reg.,
    pub table: *mut *mut u32 rbdqptr; / base pointer to RxBD parameter,
    pub /: *mut *mut u16 mflr; / max frame length reg.,
    pub /: *mut *mut u16 minflr; / min frame length reg.,
    pub /: *mut *mut u16 maxd1; / max dma1 length reg.,
    pub /: *mut *mut u16 maxd2; / max dma2 length reg.,
    pub /: *mut *mut u32 ecamptr; / external CAM address,
    pub /: *mut *mut u32 l2qt; / VLAN priority mapping table.,
    pub /: *mut *mut u32 l3qt[0x8]; / IP priority mapping table.,
    pub /: *mut *mut u16 vlantype; / vlan type,
    pub /: *mut *mut u16 vlantci; / default vlan tci,
    pub /: *mut *mut u8 addressfiltering[64]; / address filtering data structure,
    pub global: *mut *mut u32 exfGlobalParam; / base address for extended filtering,
    pub /: *mut *mut u8 res6[0x100 - 0xC4]; / Initialize to zero,
    pub __packed: },
pub const GRACEFUL_STOP_ACKNOWLEDGE_RX: c_uint = 0x01;
// structure representing InitEnet command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_init_pram {
    pub resinit1: u8,
    pub resinit2: u8,
    pub resinit3: u8,
    pub resinit4: u8,
    pub resinit5: u16,
    pub res1: [u8; 0x1],
    pub largestexternallookupkeysize: u8,
    pub rgftgfrxglobal: u32,
    pub /: *mut *mut u32 rxthread[ENET_INIT_PARAM_MAX_ENTRIES_RX]; / rx threads,
    pub 0x30]: u8 res2[0x38 -,
    pub /: *mut *mut u32 txglobal; / tx global,
    pub /: *mut *mut u32 txthread[ENET_INIT_PARAM_MAX_ENTRIES_TX]; / tx threads,
    pub res3: [u8; 0x1],
    pub __packed: },

pub const ENET_INIT_PARAM_RISC_MASK: c_uint = 0x0000003f;
pub const ENET_INIT_PARAM_PTR_MASK: c_uint = 0x00ffffc0;
pub const ENET_INIT_PARAM_SNUM_MASK: c_uint = 0xff000000;
pub const ENET_INIT_PARAM_SNUM_SHIFT: c_int = 24;
pub const ENET_INIT_PARAM_MAGIC_RES_INIT1: c_uint = 0x06;
pub const ENET_INIT_PARAM_MAGIC_RES_INIT2: c_uint = 0x30;
pub const ENET_INIT_PARAM_MAGIC_RES_INIT3: c_uint = 0xff;
pub const ENET_INIT_PARAM_MAGIC_RES_INIT4: c_uint = 0x00;
pub const ENET_INIT_PARAM_MAGIC_RES_INIT5: c_uint = 0x0400;
// structure representing 82xx Address Filtering Enet Address in PRAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_82xx_enet_address {
    pub res1: [u8; 0x2],
    pub /: *mut *mut u16 h; / address (MSB),
    pub /: *mut *mut u16 m; / address,
    pub /: *mut *mut u16 l; / address (LSB),
    pub __packed: },
// structure representing 82xx Address Filtering PRAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_82xx_address_filtering_pram {
    pub /: *mut *mut u32 iaddr_h; / individual address filter, high,
    pub /: *mut *mut u32 iaddr_l; / individual address filter, low,
    pub /: *mut *mut u32 gaddr_h; / group address filter, high,
    pub /: *mut *mut u32 gaddr_l; / group address filter, low,
    pub taddr: ucc_geth_82xx_enet_address __iomem,
    pub paddr: [ucc_geth_82xx_enet_address __iomem; NUM_OF_PADDRS],
    pub 0x38]: u8 res0[0x40 -,
    pub __packed: },
// GETH Tx firmware statistics structure, used when calling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_tx_firmware_statistics {
    pub /: *mut *mut u32 sicoltx; / single collision,
    pub /: *mut *mut u32 mulcoltx; / multiple collision,
    pub /: *mut *mut u32 latecoltxfr; / late collision,
    pub /: *mut *mut u32 frabortduecol; / frames aborted due to transmit collision,
    pub error: *mut *mut u32 frlostinmactxer; / frames lost due to internal MAC,
    pub /: *mut *mut u32 carriersenseertx; / carrier sense error,
    pub /: *mut *mut u32 frtxok; / frames transmitted OK,
    pub than: *mut *mut u32 txfrexcessivedefer; / frames with defferal time greater,
    pub 256: *mut *mut u32 txpkts256; / total packets (including bad) between,
    pub 512: *mut *mut u32 txpkts512; / total packets (including bad) between,
    pub 1024: *mut *mut u32 txpkts1024; / total packets (including bad) between,
    pub 1024: *mut *mut u32 txpktsjumbo; / total packets (including bad) between,
    pub __packed: },
// GETH Rx firmware statistics structure, used when calling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_rx_firmware_statistics {
    pub /: *mut *mut u32 frrxfcser; / frames with crc error,
    pub /: *mut *mut u32 fraligner; / frames with alignment error,
    pub /: *mut *mut u32 inrangelenrxer; / in range length error,
    pub /: *mut *mut u32 outrangelenrxer; / out of range length error,
    pub /: *mut *mut u32 frtoolong; / frame too long,
    pub /: *mut *mut u32 runt; / runt,
    pub /: *mut *mut u32 verylongevent; / very long event,
    pub /: *mut *mut u32 symbolerror; / symbol error,
    pub /: *mut *mut u32 dropbsy; / drop because of BD not ready,
    pub res0: [u8; 0x8],
    pub address: *mut *mut u32 mismatchdrop; / drop because of MAC filtering (e.g.,
    pub /: *mut *mut u32 underpkts; / total frames less than 64 octets,
    pub and: *mut *mut u32 pkts256; / total frames (including bad) between 256,
    pub and: *mut *mut u32 pkts512; / total frames (including bad) between 512,
    pub 1024: *mut *mut u32 pkts1024; / total frames (including bad) between,
    pub 1024: *mut *mut u32 pktsjumbo; / total frames (including bad) between,
    pub error: *mut *mut u32 frlossinmacer; / frames lost because of internal MAC,
    pub /: *mut *mut u32 pausefr; / pause frames,
    pub res1: [u8; 0x4],
    pub removed: *mut *mut u32 removevlan; / total frames that had their VLAN tag,
//
    pub tag: *mut *mut u32 replacevlan; / total frames that had their VLAN,
    pub tag: *mut *mut u32 insertvlan; / total frames that had their VLAN,
    pub __packed: },
// GETH hardware statistics structure, used when calling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_hardware_statistics {
    pub bad: *mut *mut u32 tx64; / Total number of frames (including,
    pub bad: *mut *mut u32 tx127; / Total number of frames (including,
    pub bad: *mut *mut u32 tx255; / Total number of frames (including,
    pub including: *mut *mut u32 rx64; / Total number of frames received,
    pub bad: *mut *mut u32 rx127; / Total number of frames (including,
    pub bad: *mut *mut u32 rx255; / Total number of frames (including,
    pub frames: *mut *mut u32 txok; / Total number of octets residing in,
    pub frames: *mut *mut u16 txcf; / Total number of PAUSE control,
    pub transmitted: *mut *mut u32 tmca; / Total number of frames that were,
    pub transmitted: *mut *mut u32 tbca; / Total number of frames,
    pub /: *mut *mut u32 rxfok; / Total number of frames received OK,
    pub /: *mut *mut u32 rxbok; / Total number of octets received OK,
    pub including: *mut *mut u32 rbyt; / Total number of octets received,
    pub received: *mut *mut u32 rmca; / Total number of frames that were,
    pub successfully: *mut *mut u32 rbca; / Total number of frames received,
    pub __packed: },
// UCC GETH Tx errors returned via TxConf callback
pub const TX_ERRORS_DEF: c_uint = 0x0200;
pub const TX_ERRORS_EXDEF: c_uint = 0x0100;
pub const TX_ERRORS_LC: c_uint = 0x0080;
pub const TX_ERRORS_RL: c_uint = 0x0040;
pub const TX_ERRORS_RC_MASK: c_uint = 0x003C;
pub const TX_ERRORS_RC_SHIFT: c_int = 2;
pub const TX_ERRORS_UN: c_uint = 0x0002;
pub const TX_ERRORS_CSL: c_uint = 0x0001;
// UCC GETH Rx errors returned via RxStore callback
pub const RX_ERRORS_CMR: c_uint = 0x0200;
pub const RX_ERRORS_M: c_uint = 0x0100;
pub const RX_ERRORS_BC: c_uint = 0x0080;
pub const RX_ERRORS_MC: c_uint = 0x0040;
// Transmit BD. These are in addition to values defined in uccf.
pub const T_VID: c_uint = 0x003c0000	/* insert VLAN id index mask. */;

// Receive BD. These are in addition to values defined in uccf.
pub const R_LG: c_uint = 0x00200000	/* Frame length violation.  */;
pub const R_NO: c_uint = 0x00100000	/* Non-octet aligned frame.  */;
pub const R_SH: c_uint = 0x00080000	/* Short frame.  */;
pub const R_CR: c_uint = 0x00040000	/* CRC error.  */;
pub const R_OV: c_uint = 0x00020000	/* Overrun.  */;
pub const R_IPCH: c_uint = 0x00010000	/* IP checksum check failed. */;

// Alignments
pub const UCC_GETH_RX_GLOBAL_PRAM_ALIGNMENT: c_int = 256;
pub const UCC_GETH_TX_GLOBAL_PRAM_ALIGNMENT: c_int = 128;
pub const UCC_GETH_THREAD_RX_PRAM_ALIGNMENT: c_int = 128;
pub const UCC_GETH_THREAD_TX_PRAM_ALIGNMENT: c_int = 64;

pub const UCC_GETH_SEND_QUEUE_QUEUE_DESCRIPTOR_ALIGNMENT: c_int = 32;

pub const UCC_GETH_RX_INTERRUPT_COALESCING_ALIGNMENT: c_int = 64;

//
pub const UCC_GETH_RX_BD_RING_ALIGNMENT: c_int = 32;
pub const UCC_GETH_TX_BD_RING_ALIGNMENT: c_int = 32;
pub const UCC_GETH_MRBLR_ALIGNMENT: c_int = 128;
pub const UCC_GETH_RX_BD_RING_SIZE_ALIGNMENT: c_int = 4;
pub const UCC_GETH_TX_BD_RING_SIZE_MEMORY_ALIGNMENT: c_int = 32;
pub const UCC_GETH_RX_DATA_BUF_ALIGNMENT: c_int = 64;
pub const UCC_GETH_TAD_EF: c_uint = 0x80;
pub const UCC_GETH_TAD_V: c_uint = 0x40;
pub const UCC_GETH_TAD_REJ: c_uint = 0x20;
pub const UCC_GETH_TAD_VTAG_OP_RIGHT_SHIFT: c_int = 2;
pub const UCC_GETH_TAD_VTAG_OP_SHIFT: c_int = 6;
pub const UCC_GETH_TAD_V_NON_VTAG_OP: c_uint = 0x20;
pub const UCC_GETH_TAD_RQOS_SHIFT: c_int = 0;
pub const UCC_GETH_TAD_V_PRIORITY_SHIFT: c_int = 5;
pub const UCC_GETH_TAD_CFI: c_uint = 0x10;
pub const UCC_GETH_VLAN_PRIORITY_MAX: c_int = 8;
pub const UCC_GETH_IP_PRIORITY_MAX: c_int = 64;
pub const UCC_GETH_TX_VTAG_TABLE_ENTRY_MAX: c_int = 8;
pub const UCC_GETH_RX_BD_RING_SIZE_MIN: c_int = 8;
pub const UCC_GETH_TX_BD_RING_SIZE_MIN: c_int = 2;
pub const UCC_GETH_BD_RING_SIZE_MAX: c_uint = 0xffff;

// Driver definitions
pub const TX_BD_RING_LEN: c_uint = 0x10;
pub const RX_BD_RING_LEN: c_uint = 0x20;

pub const ENET_GROUP_ADDR: c_uint = 0x01	/* Group address mask;

// Fast Ethernet (10/100 Mbps)

//

//

// Gigabit Ethernet (1000 Mbps)

pub const UCC_GETH_TEMODER_INIT: c_uint = 0xC000	/* bits that must */;
// Initial value for UPSMR

pub const UCC_GETH_MACCFG1_INIT: c_int = 0;

// Ethernet Address Type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enet_addr_type {
    ENET_ADDR_TYPE_INDIVIDUAL,
    ENET_ADDR_TYPE_GROUP,
    ENET_ADDR_TYPE_BROADCAST
}

// UCC GETH 82xx Ethernet Address Recognition Location
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_enet_address_recognition_location {
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_STATION_ADDRESS,/* station
    address */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_PADDR_FIRST,	/* additional
    station
    address
    paddr1 */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_PADDR2,	/* additional
    station
    address
    paddr2 */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_PADDR3,	/* additional
    station
    address
    paddr3 */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_PADDR_LAST,	/* additional
    station
    address
    paddr4 */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_GROUP_HASH,	/* group hash */
    UCC_GETH_ENET_ADDRESS_RECOGNITION_LOCATION_INDIVIDUAL_HASH /* individual
    hash */
}

// UCC GETH vlan operation tagged
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_vlan_operation_tagged {
    UCC_GETH_VLAN_OPERATION_TAGGED_NOP = 0x0,	/* Tagged - nop */
    UCC_GETH_VLAN_OPERATION_TAGGED_REPLACE_VID_PORTION_OF_Q_TAG
    = 0x1,	/* Tagged - replace vid portion of q tag */
    UCC_GETH_VLAN_OPERATION_TAGGED_IF_VID0_REPLACE_VID_WITH_DEFAULT_VALUE
    = 0x2,	/* Tagged - if vid0 replace vid with default value  */
    UCC_GETH_VLAN_OPERATION_TAGGED_EXTRACT_Q_TAG_FROM_FRAME
    = 0x3	/* Tagged - extract q tag from frame */
}

// UCC GETH vlan operation non-tagged
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_vlan_operation_non_tagged {
    UCC_GETH_VLAN_OPERATION_NON_TAGGED_NOP = 0x0,	/* Non tagged - nop */
    UCC_GETH_VLAN_OPERATION_NON_TAGGED_Q_TAG_INSERT = 0x1	/* Non tagged -
    q tag insert
//
}

// UCC GETH Rx Quality of Service Mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_qos_mode {
    UCC_GETH_QOS_MODE_DEFAULT = 0x0,	/* default queue */
    UCC_GETH_QOS_MODE_QUEUE_NUM_FROM_L2_CRITERIA = 0x1,	/* queue
    determined
    by L2
    criteria */
    UCC_GETH_QOS_MODE_QUEUE_NUM_FROM_L3_CRITERIA = 0x2	/* queue
    determined
    by L3
    criteria */
}

// UCC GETH Statistics Gathering Mode - These are bit flags, 'or' them together
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_statistics_gathering_mode {
    UCC_GETH_STATISTICS_GATHERING_MODE_NONE = 0x00000000,	/* No
    statistics
    gathering */
    UCC_GETH_STATISTICS_GATHERING_MODE_HARDWARE = 0x00000001,/* Enable
    hardware
    statistics
    gathering
//
    UCC_GETH_STATISTICS_GATHERING_MODE_FIRMWARE_TX = 0x00000004,/*Enable
    firmware
    tx
    statistics
    gathering
//
    UCC_GETH_STATISTICS_GATHERING_MODE_FIRMWARE_RX = 0x00000008/* Enable
    firmware
    rx
    statistics
    gathering
//
}

// UCC GETH Pad and CRC Mode - Note, Padding without CRC is not possible
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_maccfg2_pad_and_crc_mode {
    UCC_GETH_PAD_AND_CRC_MODE_NONE
    = MACCFG2_PAD_AND_CRC_MODE_NONE,	/* Neither Padding
    short frames
    nor CRC */
    UCC_GETH_PAD_AND_CRC_MODE_CRC_ONLY
    = MACCFG2_PAD_AND_CRC_MODE_CRC_ONLY,	/* Append
    CRC only */
    UCC_GETH_PAD_AND_CRC_MODE_PAD_AND_CRC =
    MACCFG2_PAD_AND_CRC_MODE_PAD_AND_CRC
}

// UCC GETH upsmr Flow Control Mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_flow_control_mode {
    UPSMR_AUTOMATIC_FLOW_CONTROL_MODE_NONE = 0x00000000,	/* No automatic
    flow control
//
    UPSMR_AUTOMATIC_FLOW_CONTROL_MODE_PAUSE_WHEN_EMERGENCY
    = 0x00004000	/* Send pause frame when RxFIFO reaches its
    emergency threshold */
}

// UCC GETH number of threads
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_num_of_threads {
    UCC_GETH_NUM_OF_THREADS_1 = 0x1,	/* 1 */
    UCC_GETH_NUM_OF_THREADS_2 = 0x2,	/* 2 */
    UCC_GETH_NUM_OF_THREADS_4 = 0x0,	/* 4 */
    UCC_GETH_NUM_OF_THREADS_6 = 0x3,	/* 6 */
    UCC_GETH_NUM_OF_THREADS_8 = 0x4	/* 8 */
}

// UCC GETH number of station addresses
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_geth_num_of_station_addresses {
    UCC_GETH_NUM_OF_STATION_ADDRESSES_1,	/* 1 */
    UCC_GETH_NUM_OF_STATION_ADDRESSES_5	/* 5 */
}

// UCC GETH 82xx Ethernet Address Container
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enet_addr_container {
    pub /: *mut *mut u8 address[ETH_ALEN]; / ethernet address,
    pub in: *mut *mut ucc_geth_enet_address_recognition_location location; / location,
    pub node: list_head,
}

// UCC GETH Termination Action Descriptor (TAD) structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_tad_params {
    pub rx_non_dynamic_extended_features_mode: c_int,
    pub reject_frame: c_int,
    pub vtag_op: ucc_geth_vlan_operation_tagged,
    pub vnontag_op: ucc_geth_vlan_operation_non_tagged,
    pub rqos: ucc_geth_qos_mode,
    pub vpri: u8,
    pub vid: u16,
}

// GETH protocol initialization structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_info {
    pub uf_info: ucc_fast_info,
    pub ipCheckSumCheck: c_int,
    pub ipCheckSumGenerate: c_int,
    pub rxExtendedFiltering: c_int,
    pub extendedFilteringChainPointer: u32,
    pub typeorlen: u16,
    pub dynamicMaxFrameLength: c_int,
    pub dynamicMinFrameLength: c_int,
    pub nonBackToBackIfgPart1: u8,
    pub nonBackToBackIfgPart2: u8,
    pub miminumInterFrameGapEnforcement: u8,
    pub backToBackInterFrameGap: u8,
    pub ipAddressAlignment: c_int,
    pub mblinterval: u32,
    pub nortsrbytetime: u16,
    pub fracsiz: u8,
    pub strictpriorityq: u8,
    pub txasap: u8,
    pub extrabw: u8,
    pub miiPreambleSupress: c_int,
    pub altBebTruncation: u8,
    pub altBeb: c_int,
    pub backPressureNoBackoff: c_int,
    pub noBackoff: c_int,
    pub excessDefer: c_int,
    pub maxRetransmission: u8,
    pub collisionWindow: u8,
    pub pro: c_int,
    pub cap: c_int,
    pub rsh: c_int,
    pub rlpb: c_int,
    pub cam: c_int,
    pub bro: c_int,
    pub ecm: c_int,
    pub receiveFlowControl: c_int,
    pub transmitFlowControl: c_int,
    pub maxGroupAddrInHash: u8,
    pub maxIndAddrInHash: u8,
    pub maxFrameLength: u16,
    pub minFrameLength: u16,
    pub maxD1Length: u16,
    pub maxD2Length: u16,
    pub vlantype: u16,
    pub vlantci: u16,
    pub ecamptr: u32,
    pub eventRegMask: u32,
    pub pausePeriod: u16,
    pub extensionField: u16,
    pub tbi_node: *mut device_node,
    pub weightfactor: [u8; NUM_TX_QUEUES],
    pub interruptcoalescingmaxvalue: [u8; NUM_RX_QUEUES],
    pub l2qt: [u8; UCC_GETH_VLAN_PRIORITY_MAX],
    pub l3qt: [u8; UCC_GETH_IP_PRIORITY_MAX],
    pub vtagtable: [u32; UCC_GETH_TX_VTAG_TABLE_ENTRY_MAX],
    pub iphoffset: [u8; TX_IP_OFFSET_ENTRY_MAX],
    pub bdRingLenTx: [u16; NUM_TX_QUEUES],
    pub bdRingLenRx: [u16; NUM_RX_QUEUES],
    pub numStationAddresses: ucc_geth_num_of_station_addresses,
    pub statisticsMode: ucc_geth_statistics_gathering_mode,
    pub vlanOperationTagged: ucc_geth_vlan_operation_tagged,
    pub vlanOperationNonTagged: ucc_geth_vlan_operation_non_tagged,
    pub rxQoSMode: ucc_geth_qos_mode,
    pub aufc: ucc_geth_flow_control_mode,
    pub padAndCrc: ucc_geth_maccfg2_pad_and_crc_mode,
    pub numThreadsTx: ucc_geth_num_of_threads,
    pub numThreadsRx: ucc_geth_num_of_threads,
    pub riscTx: c_uint,
    pub riscRx: c_uint,
}

// structure representing UCC GETH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_geth_private {
    pub ug_info: *mut ucc_geth_info,
    pub uccf: *mut ucc_fast_private,
    pub dev: *mut device,
    pub ndev: *mut net_device,
    pub napi: napi_struct,
    pub timeout_work: work_struct,
    pub ug_regs: *mut ucc_geth __iomem,
    pub p_init_enet_param_shadow: *mut ucc_geth_init_pram,
    pub p_exf_glbl_param: *mut ucc_geth_exf_global_pram __iomem,
    pub exf_glbl_param_offset: u32,
    pub p_rx_glbl_pram: *mut ucc_geth_rx_global_pram __iomem,
    pub p_tx_glbl_pram: *mut ucc_geth_tx_global_pram __iomem,
    pub p_send_q_mem_reg: *mut ucc_geth_send_queue_mem_region __iomem,
    pub send_q_mem_reg_offset: u32,
    pub p_thread_data_tx: *mut ucc_geth_thread_data_tx __iomem,
    pub thread_dat_tx_offset: u32,
    pub p_thread_data_rx: *mut ucc_geth_thread_data_rx __iomem,
    pub thread_dat_rx_offset: u32,
    pub p_scheduler: *mut ucc_geth_scheduler __iomem,
    pub scheduler_offset: u32,
    pub p_tx_fw_statistics_pram: *mut ucc_geth_tx_firmware_statistics_pram __iomem,
    pub tx_fw_statistics_pram_offset: u32,
    pub p_rx_fw_statistics_pram: *mut ucc_geth_rx_firmware_statistics_pram __iomem,
    pub rx_fw_statistics_pram_offset: u32,
    pub p_rx_irq_coalescing_tbl: *mut ucc_geth_rx_interrupt_coalescing_table __iomem,
    pub rx_irq_coalescing_tbl_offset: u32,
    pub p_rx_bd_qs_tbl: *mut ucc_geth_rx_bd_queues_entry __iomem,
    pub rx_bd_qs_tbl_offset: u32,
    pub p_tx_bd_ring: [*mut u8 __iomem; NUM_TX_QUEUES],
    pub p_rx_bd_ring: [*mut u8 __iomem; NUM_RX_QUEUES],
    pub confBd: [*mut u8 __iomem; NUM_TX_QUEUES],
    pub txBd: [*mut u8 __iomem; NUM_TX_QUEUES],
    pub rxBd: [*mut u8 __iomem; NUM_RX_QUEUES],
    pub badFrame: [c_int; NUM_RX_QUEUES],
    pub cpucount: [u16; NUM_TX_QUEUES],
    pub p_cpucount: [*mut u16 __iomem; NUM_TX_QUEUES],
    pub indAddrRegUsed: [c_int; NUM_OF_PADDRS],
    pub /: *mut *mut u8 paddr[NUM_OF_PADDRS][ETH_ALEN]; / ethernet address,
    pub numGroupAddrInHash: u8,
    pub numIndAddrInHash: u8,
    pub numIndAddrInReg: u8,
    pub rx_extended_features: c_int,
    pub rx_non_dynamic_extended_features: c_int,
    pub conf_skbs: list_head,
    pub group_hash_q: list_head,
    pub ind_hash_q: list_head,
    pub saved_uccm: u32,
    pub lock: spinlock_t,
// pointers to arrays of skbuffs for tx and rx
    pub tx_skbuff: [*mut sk_buff; NUM_TX_QUEUES],
    pub rx_skbuff: [*mut sk_buff; NUM_RX_QUEUES],
// indices pointing to the next free sbk in skb arrays
    pub skb_curtx: [u16; NUM_TX_QUEUES],
    pub skb_currx: [u16; NUM_RX_QUEUES],
// index of the first skb which hasn't been transmitted yet.
    pub skb_dirtytx: [u16; NUM_TX_QUEUES],
    pub mii_info: *mut ugeth_mii_info,
    pub msg_enable: u32,
    pub wol_en: u32,
    pub phy_wol_en: u32,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub node: *mut device_node,
}

extern "C" {
    pub fn uec_set_ethtool_ops(netdev: *mut net_device);
}
