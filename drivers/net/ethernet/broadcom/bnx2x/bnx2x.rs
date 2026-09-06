//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x.h
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


// bnx2x.h: QLogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Eliezer Tamir
// Based on code from Michael Chan's bnx2 driver
//

// compilation time flags
// define this to make the driver freeze on error to allow getting debug info
// (you will need to reboot afterwards)
// #define BNX2X_STOP_ON_ERROR
// FIXME: Delete the DRV_MODULE_VERSION below, but please be warned
// that it is not an easy task because such change has all chances
// to break this driver due to amount of abuse of in-kernel interfaces
// between modules and FW.
//
// DO NOT UPDATE DRV_MODULE_VERSION below.
//

pub const BNX2X_BC_VER: c_uint = 0x040200;

// Macro flag: #define BCM_DCBNL

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_int_mode {
    BNX2X_INT_MODE_MSIX,
    BNX2X_INT_MODE_INTX,
    BNX2X_INT_MODE_MSI
}

// error/debug prints

// for messages that are currently off
pub const BNX2X_MSG_OFF: c_uint = 0x0;
pub const BNX2X_MSG_MCP: c_uint = 0x0010000 /* was: NETIF_MSG_HW */;
pub const BNX2X_MSG_STATS: c_uint = 0x0020000 /* was: NETIF_MSG_TIMER */;
pub const BNX2X_MSG_NVM: c_uint = 0x0040000 /* was: NETIF_MSG_HW */;
pub const BNX2X_MSG_DMAE: c_uint = 0x0080000 /* was: NETIF_MSG_HW */;
pub const BNX2X_MSG_SP: c_uint = 0x0100000 /* was: NETIF_MSG_INTR */;
pub const BNX2X_MSG_FP: c_uint = 0x0200000 /* was: NETIF_MSG_INTR */;
pub const BNX2X_MSG_IOV: c_uint = 0x0800000;
pub const BNX2X_MSG_PTP: c_uint = 0x1000000;
pub const BNX2X_MSG_IDLE: c_uint = 0x2000000 /* used for idle check*/;
pub const BNX2X_MSG_ETHTOOL: c_uint = 0x4000000;
pub const BNX2X_MSG_DCB: c_uint = 0x8000000;
// regular debug print

// errors debug print

// for errors (never masked)

// before we have a dev->name use dev_info()

// Error handling
extern "C" {
    pub fn bnx2x_panic_dump(bp: *mut bnx2x, disable_int: bool);
}

// SP SB indices
// General SP events - stats query, cfc delete, etc
pub const HC_SP_INDEX_ETH_DEF_CONS: c_int = 3;
// EQ completions
pub const HC_SP_INDEX_EQ_CONS: c_int = 7;
// FCoE L2 connection completions
pub const HC_SP_INDEX_ETH_FCOE_TX_CQ_CONS: c_int = 6;
pub const HC_SP_INDEX_ETH_FCOE_RX_CQ_CONS: c_int = 4;
// iSCSI L2
pub const HC_SP_INDEX_ETH_ISCSI_CQ_CONS: c_int = 5;
pub const HC_SP_INDEX_ETH_ISCSI_RX_CQ_CONS: c_int = 1;
// Special clients parameters
// SB indices
// FCoE L2

//
// CIDs and CLIDs:
// CLIDs below is a CLID for func 0, then the CLID for other
// functions will be calculated by the formula:
//
// FUNC_N_CLID_X = N * NUM_SPECIAL_CLIENTS + FUNC_0_CLID_X
//
// use a value high enough to be above all the PFs, which has least significant
// nibble as 8, so when cnic needs to come up with a CID for UIO to use to
// calculate doorbell address according to old doorbell configuration scheme
// (db_msg_sz 1 << 7 * cid + 0x40 DPM offset) it can come up with a valid number
// We must avoid coming up with cid 8 for iscsi since according to this method
// the designated UIO cid will come out 0 and it has a special handling for that
// case which doesn't suit us. Therefore will will cieling to closes cid which
// has least signigifcant nibble 8 and if it is 8 we will move forward to 0x18.
//

// amount of cids traversed by UIO's DPM addition to doorbell
pub const UIO_DPM: c_int = 8;
// roundup to DPM offset

// offset to nearest value which has lsb nibble matching DPM

// add offset to rounded-up cid to get a value which could be used with UIO

// but wait - avoid UIO special case for cid 0

// Properly DPM aligned CID dajusted to cid 0 secal case

// how many cids were wasted  - need this value for cid allocation

// iSCSI L2

// FCoE L2

pub const SM_RX_ID: c_int = 0;
pub const SM_TX_ID: c_int = 1;
// defines for multiple tx priority indices
pub const FIRST_TX_ONLY_COS_INDEX: c_int = 1;
pub const FIRST_TX_COS_INDEX: c_int = 0;
// rules for calculating the cids of tx-only connections

// fp index inside class of service range

// Indexes for transmission queues array:
// txdata for RSS i CoS j is at location i + (j * num of RSS)
// txdata for FCoE (if exist) is at location max cos * num of RSS
// txdata for FWD (if exist) is one location after FCoE
// txdata for OOO (if exist) is one location after FWD
//

// fast path
//
// This driver uses new build_skb() API :
// RX ring buffer contains pointer to kmalloc() data only,
// skb are built only after Hardware filled the frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_rx_bd {
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_tx_bd {
    pub skb: *mut sk_buff,
    pub first_bd: u16,
    pub flags: u8,
// Set on the first BD descriptor when there is a split BD

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_rx_page {
    pub page: *mut page,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union db_prod {
    pub data: doorbell_set_prod,
    pub raw: u32,
}

// dropless fc FW/HW related params

pub const FW_PREFETCH_CNT: c_int = 16;
pub const DROPLESS_FC_HEADROOM: c_int = 100;
// MC hsi
pub const BCM_PAGE_SHIFT: c_int = 12;

pub const PAGES_PER_SGE_SHIFT: c_int = 0;

pub const SGE_PAGE_SHIFT: c_int = 12;

// SGE ring related macros
pub const NUM_RX_SGE_PAGES: c_int = 2;

pub const NEXT_PAGE_SGE_DESC_CNT: c_int = 2;

// RX_SGE_CNT is promised to be a power of 2

//
// Number of required  SGEs is the sum of two:
// 1. Number of possible opened aggregations (next packet for
// these aggregations will probably consume SGE immediately)
// 2. Rest of BRB blocks divided by 2 (block will consume new SGE only
// after placement on BD for new TPA aggregation)
//
// Takes into account NEXT_PAGE_SGE_DESC_CNT "next" elements on each page
//

// Manipulate a bit vector defined as an array of u64
// Number of bits in one sge_mask array element
pub const BIT_VEC64_ELEM_SZ: c_int = 64;
pub const BIT_VEC64_ELEM_SHIFT: c_int = 6;

// Creates a bitmask of all ones in less significant bits.

//
// Number of u64 elements in SGE mask array

#[repr(C)]
#[derive(Copy, Clone)]
pub union host_hc_status_block {
// pointer to fp status block e1x
    pub e1x_sb: *mut host_hc_status_block_e1x,
// pointer to fp status block e2
    pub e2_sb: *mut host_hc_status_block_e2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_agg_info {
//
// First aggregation buffer is a data buffer, the following - are pages.
// We will preallocate the data buffer for each aggregation when
// we open the interface and will replace the BD at the consumer
// with this one when we receive the TPA_START CQE in order to
// keep the Rx BD ring consistent.
//
    pub first_buf: sw_rx_bd,
    pub tpa_state: u8,
pub const BNX2X_TPA_START: c_int = 1;
pub const BNX2X_TPA_STOP: c_int = 2;
pub const BNX2X_TPA_ERROR: c_int = 3;
    pub placement_offset: u8,
    pub parsing_flags: u16,
    pub vlan_tag: u16,
    pub len_on_bd: u16,
    pub rxhash: u32,
    pub rxhash_type: pkt_hash_types,
    pub gro_size: u16,
    pub full_page: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fp_txdata {
    pub tx_buf_ring: *mut sw_tx_bd,
    pub tx_desc_ring: *mut eth_tx_bd_types,
    pub tx_desc_mapping: dma_addr_t,
    pub cid: u32,
    pub tx_db: db_prod,
    pub tx_pkt_prod: u16,
    pub tx_pkt_cons: u16,
    pub tx_bd_prod: u16,
    pub tx_bd_cons: u16,
    pub tx_pkt: c_ulong,
    pub tx_cons_sb: *mut __le16,
    pub txq_index: c_int,
    pub parent_fp: *mut bnx2x_fastpath,
    pub tx_ring_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_tpa_mode_t {
    TPA_MODE_DISABLED,
    TPA_MODE_LRO,
    TPA_MODE_GRO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_alloc_pool {
    pub page: *mut page,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fastpath {
    pub /: *mut *mut *mut bnx2x bp; / parent,
    pub napi: napi_struct,
    pub status_blk: host_hc_status_block,
// chip independent shortcuts into sb structure
    pub sb_index_values: *mut __le16,
    pub sb_running_index: *mut __le16,
// chip independent shortcut into rx_prods_offset memory
    pub ustorm_rx_prods_offset: u32,
    pub rx_buf_size: u32,
    pub /: *mut *mut u32 rx_frag_size; / 0 if kmalloced(), or rx_buf_size + NET_SKB_PAD,
    pub status_blk_mapping: dma_addr_t,
    pub mode: bnx2x_tpa_mode_t,
    pub /: *mut *mut u8 max_cos; / actual number of active tx coses,
    pub txdata_ptr: [*mut bnx2x_fp_txdata; BNX2X_MULTI_TX_COS],
    pub /: *mut *mut *mut sw_rx_bd rx_buf_ring; / BDs mappings ring,
    pub /: *mut *mut *mut sw_rx_page rx_page_ring; / SGE pages mappings ring,
    pub rx_desc_ring: *mut eth_rx_bd,
    pub rx_desc_mapping: dma_addr_t,
    pub rx_comp_ring: *mut eth_rx_cqe,
    pub rx_comp_mapping: dma_addr_t,
// SGE ring
    pub rx_sge_ring: *mut eth_rx_sge,
    pub rx_sge_mapping: dma_addr_t,
    pub sge_mask: [u64; RX_SGE_MASK_LEN],
    pub cid: u32,
    pub fp_hc_idx: __le16,
    pub /: *mut *mut u8 index; / number in fp array,
    pub /: *mut *mut u8 rx_queue; / index for skb_record,
    pub /: *mut *mut u8 cl_id; / eth client id,
    pub cl_qzone_id: u8,
    pub /: *mut *mut u8 fw_sb_id; / status block number in FW,
    pub /: *mut *mut u8 igu_sb_id; / status block number in HW,
    pub rx_bd_prod: u16,
    pub rx_bd_cons: u16,
    pub rx_comp_prod: u16,
    pub rx_comp_cons: u16,
    pub rx_sge_prod: u16,
// The last maximal completed SGE
    pub last_max_sge: u16,
    pub rx_cons_sb: *mut __le16,
// TPA related
    pub tpa_info: *mut bnx2x_agg_info,

    pub tpa_queue_used: u64,

// The size is calculated using the following:
    pub name: [c_char; FP_NAME_SIZE],
    pub page_pool: bnx2x_alloc_pool,
}

// Use 2500 as a mini-jumbo MTU for FCoE
pub const BNX2X_FCOE_MINI_JUMBO_MTU: c_int = 2500;
pub const FCOE_IDX_OFFSET: c_int = 0;

// MC hsi

pub const RX_COPY_THRESH: c_int = 92;
pub const NUM_TX_RINGS: c_int = 16;

pub const NEXT_PAGE_TX_DESC_CNT: c_int = 1;

// number of NEXT_PAGE descriptors may be required during placement

// max BDs per tx packet w/o next_pages:
// START_BD		- describes packed
// START_BD(splitted)	- includes unpaged data segment for GSO
// PARSING_BD		- for TSO and CSUM data
// PARSING_BD2		- for encapsulation data
// Frag BDs		- describes pages for frags
//
pub const BDS_PER_TX_PKT: c_int = 4;

// max BDs per tx packet including next pages

// The RX BD ring is special, each bd is 8 bytes but the last one is 16
pub const NUM_RX_RINGS: c_int = 8;

pub const NEXT_PAGE_RX_DESC_CNT: c_int = 2;

// dropless fc calculations for BDs
//
// Number of BDs should as number of buffers in BRB:
// Low threshold takes into account NEXT_PAGE_RX_DESC_CNT
// "next" elements on each page
//

//
// As long as CQE is X times bigger than BD entry we have to allocate X times
// more pages for CQ ring in order to keep it balanced with BD ring
//

pub const NEXT_PAGE_RCQ_DESC_CNT: c_int = 1;

// dropless fc calculations for RCQs
//
// Number of RCQs should be as number of buffers in BRB:
// Low threshold takes into account NEXT_PAGE_RCQ_DESC_CNT
// "next" elements on each page
//

// This is needed for determining of last_max

pub const BNX2X_SWCID_SHIFT: c_int = 17;

// used on a CID received from the HW

// TX CSUM helpers

pub const XMIT_PLAIN: c_int = 0;

// stuff added to make the code fit 80Col

pub const HC_INDEX_ETH_RX_CQ_CONS: c_int = 1;
pub const HC_INDEX_OOO_TX_CQ_CONS: c_int = 4;
pub const HC_INDEX_ETH_TX_CQ_CONS_COS0: c_int = 5;
pub const HC_INDEX_ETH_TX_CQ_CONS_COS1: c_int = 6;
pub const HC_INDEX_ETH_TX_CQ_CONS_COS2: c_int = 7;

// end of fast path
// common
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_common {
    pub chip_id: u32,
// chip num:16-31, rev:12-15, metal:4-11, bond_id:0-3

pub const CHIP_NUM_57710: c_uint = 0x164e;
pub const CHIP_NUM_57711: c_uint = 0x164f;
pub const CHIP_NUM_57711E: c_uint = 0x1650;
pub const CHIP_NUM_57712: c_uint = 0x1662;
pub const CHIP_NUM_57712_MF: c_uint = 0x1663;
pub const CHIP_NUM_57712_VF: c_uint = 0x166f;
pub const CHIP_NUM_57713: c_uint = 0x1651;
pub const CHIP_NUM_57713E: c_uint = 0x1652;
pub const CHIP_NUM_57800: c_uint = 0x168a;
pub const CHIP_NUM_57800_MF: c_uint = 0x16a5;
pub const CHIP_NUM_57800_VF: c_uint = 0x16a9;
pub const CHIP_NUM_57810: c_uint = 0x168e;
pub const CHIP_NUM_57810_MF: c_uint = 0x16ae;
pub const CHIP_NUM_57810_VF: c_uint = 0x16af;
pub const CHIP_NUM_57811: c_uint = 0x163d;
pub const CHIP_NUM_57811_MF: c_uint = 0x163e;
pub const CHIP_NUM_57811_VF: c_uint = 0x163f;
pub const CHIP_NUM_57840_OBSOLETE: c_uint = 0x168d;
pub const CHIP_NUM_57840_MF_OBSOLETE: c_uint = 0x16ab;
pub const CHIP_NUM_57840_4_10: c_uint = 0x16a1;
pub const CHIP_NUM_57840_2_20: c_uint = 0x16a2;
pub const CHIP_NUM_57840_MF: c_uint = 0x16a4;
pub const CHIP_NUM_57840_VF: c_uint = 0x16ad;

pub const CHIP_REV_SHIFT: c_int = 12;

// assume maximum 5 revisions

// Emul versions are A=>0xe, B=>0xc, C=>0xa, D=>8, E=>6

// FPGA versions are A=>0xf, B=>0xd, C=>0xb, D=>9, E=>7

// This define is used in two main places:
// 1. In the early stages of nic_load, to know if to configure Parser / Searcher
// to nic-only mode or to offload mode. Offload mode is configured if either the
// chip is E1x (where MIC_MODE register is not applicable), or if cnic already
// registered for this port (which means that the user wants storage services).
// 2. During cnic-related load, to know if offload mode is already configured in
// the HW or needs to be configured.
// Since the transition from nic-mode to offload-mode in HW causes traffic
// corruption, nic-mode is configured only in ports on which storage services
// where never requested.
//

    pub flash_size: c_int,
pub const BNX2X_NVRAM_1MB_SIZE: c_uint = 0x20000	/* 1M bit in bytes */;
pub const BNX2X_NVRAM_TIMEOUT_COUNT: c_int = 30000;
pub const BNX2X_NVRAM_PAGE_SIZE: c_int = 256;
    pub shmem_base: u32,
    pub shmem2_base: u32,
    pub mf_cfg_base: u32,
    pub mf2_cfg_base: u32,
    pub hw_config: u32,
    pub bc_ver: u32,
    pub int_block: u8,
pub const INT_BLOCK_HC: c_int = 0;
pub const INT_BLOCK_IGU: c_int = 1;
pub const INT_BLOCK_MODE_NORMAL: c_int = 0;
pub const INT_BLOCK_MODE_BW_COMP: c_int = 2;

    pub chip_port_mode: u8,
pub const CHIP_4_PORT_MODE: c_uint = 0x0;
pub const CHIP_2_PORT_MODE: c_uint = 0x1;
pub const CHIP_PORT_MODE_NONE: c_uint = 0x2;

    pub boot_mode: u32,
}

// IGU MSIX STATISTICS on 57712: 64 for VFs; 4 for PFs; 4 for Attentions
pub const BNX2X_IGU_STAS_MSG_VF_CNT: c_int = 64;
pub const BNX2X_IGU_STAS_MSG_PF_CNT: c_int = 4;
pub const MAX_IGU_ATTN_ACK_TO: c_int = 100;
// end of common
// port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_port {
    pub pmf: u32,
    pub link_config: [u32; LINK_CONFIG_SIZE],
    pub supported: [u32; LINK_CONFIG_SIZE],
    pub advertising: [u32; LINK_CONFIG_SIZE],
    pub phy_addr: u32,
// used to synchronize phy accesses
    pub phy_mutex: mutex,
    pub port_stx: u32,
    pub old_nig_stats: nig_stats,
}

// end of port

// slow path
pub const BNX2X_MAX_NUM_OF_VFS: c_int = 64;

// We need to reserve doorbell addresses for all VF and queue combinations

// The doorbell is configured to have the same number of CIDs for PFs and for
// VFs. For this reason the PF CID zone is as large as the VF zone.
//

pub const BNX2X_MAX_NUM_VF_QUEUES: c_int = 64;
pub const BNX2X_VF_ID_INVALID: c_uint = 0xFF;
// the number of VF CIDS multiplied by the amount of bytes reserved for each
// cid must not exceed the size of the VF doorbell
//
pub const BNX2X_VF_BAR_SIZE: c_int = 512;

//
// The total number of L2 queues, MSIX vectors and HW contexts (CIDs) is
// control by the number of fast-path status blocks supported by the
// device (HW/FW). Each fast-path status block (FP-SB) aka non-default
// status block represents an independent interrupts context that can
// serve a regular L2 networking queue. However special L2 queues such
// as the FCoE queue do not require a FP-SB and other components like
// the CNIC may consume FP-SB reducing the number of possible L2 queues
//
// If the maximum number of FP-SB available is X then:
// a. If CNIC is supported it consumes 1 FP-SB thus the max number of
// regular L2 queues is Y=X-1
// b. In MF mode the actual number of L2 queues is Y= (X-1/MF_factor)
// c. If the FCoE L2 queue is supported the actual number of L2 queues
// is Y+1
// d. The number of irqs (MSIX vectors) is either Y+1 (one extra for
// slow-path interrupts) or Y+2 if CNIC is supported (one additional
// FP interrupt context for the CNIC).
// e. The number of HW context (CID count) is always X or X+1 if FCoE
// L2 queue is supported. The cid for the FCoE L2 queue is always X.
//
// fast-path interrupt contexts E1x
pub const FP_SB_MAX_E1x: c_int = 16;
// fast-path interrupt contexts E2

#[repr(C)]
#[derive(Copy, Clone)]
pub union cdu_context {
    pub eth: eth_context,
    pub pad: [c_char; 1024],
}

// CDU host DB constants
pub const CDU_ILT_PAGE_SZ_HW: c_int = 2;

pub const CNIC_ISCSI_CID_MAX: c_int = 256;
pub const CNIC_FCOE_CID_MAX: c_int = 2048;

pub const QM_ILT_PAGE_SZ_HW: c_int = 0;

pub const QM_CID_ROUND: c_int = 1024;
// TM (timers) host DB constants
pub const TM_ILT_PAGE_SZ_HW: c_int = 0;

// SRC (Searcher) host DB constants
pub const SRC_ILT_PAGE_SZ_HW: c_int = 0;

pub const SRC_HASH_BITS: c_int = 10;

pub const MAX_DMAE_C: c_int = 8;
// DMA memory not used in fastpath
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_slowpath {
    pub e1x: mac_configuration_cmd,
    pub e2: eth_classify_rules_ramrod_data,
    pub mac_rdata: },
    pub e2: eth_classify_rules_ramrod_data,
    pub vlan_rdata: },
    pub e1x: tstorm_eth_mac_filter_config,
    pub e2: eth_filter_rules_ramrod_data,
    pub rx_mode_rdata: },
    pub e1: mac_configuration_cmd,
    pub e2: eth_multicast_rules_ramrod_data,
    pub mcast_rdata: },
    pub rss_rdata: eth_rss_update_ramrod_data,
// Queue State related ramrods are always sent under rtnl_lock
    pub init_data: client_init_ramrod_data,
    pub update_data: client_update_ramrod_data,
    pub tpa_data: tpa_update_ramrod_data,
    pub q_rdata: },
    pub func_start: function_start_data,
// pfc configuration for DCBX ramrod
    pub pfc_config: flow_control_configuration,
    pub func_rdata: },
// afex ramrod can not be a part of func_rdata union because these
// events might arrive in parallel to other events from func_rdata.
// Therefore, if they would have been defined in the same union,
// data can get corrupted.
//
    pub viflist_data: afex_vif_list_ramrod_data,
    pub func_update: function_update_data,
    pub func_afex_rdata: },
// used by dmae command executer
    pub dmae: [dmae_command; MAX_DMAE_C],
    pub stats_comp: u32,
    pub mac_stats: mac_stats,
    pub nig_stats: nig_stats,
    pub port_stats: host_port_stats,
    pub func_stats: host_func_stats,
    pub wb_comp: u32,
    pub wb_data: [u32; 4],
    pub drv_info_to_mcp: drv_info_to_mcp,
}

// attn group wiring
pub const MAX_DYNAMIC_ATTN_GRPS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attn_route {
    pub sig: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iro {
    pub base: u32,
    pub m1: u16,
    pub m2: u16,
    pub m3: u16,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_context {
    pub vcxt: *mut cdu_context,
    pub cxt_mapping: dma_addr_t,
    pub size: usize,
}

// forward
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_recovery_state {
    BNX2X_RECOVERY_DONE,
    BNX2X_RECOVERY_INIT,
    BNX2X_RECOVERY_WAIT,
    BNX2X_RECOVERY_FAILED,
    BNX2X_RECOVERY_NIC_LOADING
}

//
// Event queue (EQ or event ring) MC hsi
// NUM_EQ_PAGES and EQ_DESC_CNT_PAGE must be power of 2
//
pub const NUM_EQ_PAGES: c_int = 1;

// depends on EQ_DESC_CNT_PAGE being a power of 2

// depends on the above and on NUM_EQ_PAGES being a power of 2

// This is a data that will be used to create a link report message.
// We will keep the data used for the last link report in order
// to prevent reporting the same link parameters twice.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_link_report_data {
    pub /: *mut *mut u16 line_speed; / Effective line speed,
    pub /: *mut *mut unsigned long link_report_flags;/ BNX2X_LINK_REPORT_XXX flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fw_stats_req {
    pub hdr: stats_query_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fw_stats_data {
    pub storm_counters: stats_counter,
    pub port: per_port_stats,
    pub pf: per_pf_stats,
    pub fcoe: fcoe_statistics_params,
    pub queue_stats: [per_queue_stats; ],
}

// Public slow path states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sp_rtnl_flag {
    BNX2X_SP_RTNL_SETUP_TC,
    BNX2X_SP_RTNL_TX_TIMEOUT,
    BNX2X_SP_RTNL_FAN_FAILURE,
    BNX2X_SP_RTNL_AFEX_F_UPDATE,
    BNX2X_SP_RTNL_ENABLE_SRIOV,
    BNX2X_SP_RTNL_VFPF_MCAST,
    BNX2X_SP_RTNL_VFPF_CHANNEL_DOWN,
    BNX2X_SP_RTNL_RX_MODE,
    BNX2X_SP_RTNL_HYPERVISOR_VLAN,
    BNX2X_SP_RTNL_TX_STOP,
    BNX2X_SP_RTNL_GET_DRV_VERSION,
    BNX2X_SP_RTNL_UPDATE_SVID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_iov_flag {
    BNX2X_IOV_HANDLE_VF_MSG,
    BNX2X_IOV_HANDLE_FLR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_prev_path_list {
    pub list: list_head,
    pub bus: u8,
    pub slot: u8,
    pub path: u8,
    pub aer: u8,
    pub undi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_sp_objs {
// MACs object
    pub mac_obj: bnx2x_vlan_mac_obj,
// Queue State object
    pub q_obj: bnx2x_queue_sp_obj,
// VLANs object
    pub vlan_obj: bnx2x_vlan_mac_obj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fp_stats {
    pub old_tclient: tstorm_per_queue_stats,
    pub old_uclient: ustorm_per_queue_stats,
    pub old_xclient: xstorm_per_queue_stats,
    pub eth_q_stats: bnx2x_eth_q_stats,
    pub eth_q_stats_old: bnx2x_eth_q_stats_old,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_entry {
    pub link: list_head,
    pub vid: u16,
    pub hw: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_udp_port_type {
    BNX2X_UDP_PORT_VXLAN,
    BNX2X_UDP_PORT_GENEVE,
    BNX2X_UDP_PORT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x {
// Fields used in the tx and intr/napi performance paths
// are grouped together in the beginning of the structure
//
    pub fp: *mut bnx2x_fastpath,
    pub sp_objs: *mut bnx2x_sp_objs,
    pub fp_stats: *mut bnx2x_fp_stats,
    pub bnx2x_txq: *mut bnx2x_fp_txdata,
    pub regview: *mut void __iomem,
    pub doorbells: *mut void __iomem,
    pub db_size: u16,
    pub /: *mut *mut u8 pf_num; / absolute PF number,
    pub /: *mut *mut u8 pfid; / per-path PF number,
    pub /**/: *mut int base_fw_ndsb;,

// protects vf2pf mailbox from simultaneous access
    pub vf2pf_mutex: mutex,
// vf pf channel mailbox contains request and response buffers
    pub vf2pf_mbox: *mut bnx2x_vf_mbx_msg,
    pub vf2pf_mbox_mapping: dma_addr_t,
// we set aside a copy of the acquire response
    pub acquire_resp: pfvf_acquire_resp_tlv,
// bulletin board for messages from pf to vf
    pub pf2vf_bulletin: *mut pf_vf_bulletin,
    pub pf2vf_bulletin_mapping: dma_addr_t,
    pub shadow_bulletin: pf_vf_bulletin,
    pub old_bulletin: pf_vf_bulletin_content,
    pub requested_nr_virtfn: u16,

    pub dev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub iro_arr: *const iro,

    pub recovery_state: bnx2x_recovery_state,
    pub is_leader: c_int,
    pub msix_table: *mut msix_entry,
    pub tx_ring_size: c_int,
// L2 header size + 2*VLANs (8 bytes) + LLC SNAP (8 bytes)

pub const ETH_MAX_JUMBO_PACKET_SIZE: c_int = 9600;
// TCP with Timestamp Option (32) + IPv6 (40)
pub const ETH_MAX_TPA_HEADER_SIZE: c_int = 72;
// Max supported alignment is 256 (8 shift)
// minimal alignment shift 6 is optimal for 57xxx HW performance
//

// FW uses 2 Cache lines Alignment for start packet and size
//
// We assume skb_build() uses sizeof(struct skb_shared_info) bytes
// at the end of skb->data, to avoid wasting a full cache line.
// This reduces memory use (skb->truesize).
//

    pub def_status_blk: *mut host_sp_status_block,
pub const DEF_SB_IGU_ID: c_int = 16;

    pub def_idx: __le16,
    pub def_att_idx: __le16,
    pub attn_state: u32,
    pub attn_group: [attn_route; MAX_DYNAMIC_ATTN_GRPS],
// slow path ring
    pub spq: *mut eth_spe,
    pub spq_mapping: dma_addr_t,
    pub spq_prod_idx: u16,
    pub spq_prod_bd: *mut eth_spe,
    pub spq_last_bd: *mut eth_spe,
    pub dsb_sp_prod: *mut __le16,
    pub /: *mut *mut atomic_t cq_spq_left; / ETH_XXX ramrods credit,
// used to synchronize spq accesses
    pub spq_lock: spinlock_t,
// event queue
    pub eq_ring: *mut event_ring_elem,
    pub eq_mapping: dma_addr_t,
    pub eq_prod: u16,
    pub eq_cons: u16,
    pub eq_cons_sb: *mut __le16,
    pub /: *mut *mut atomic_t eq_spq_left; / COMMON_XXX ramrods credit,
// Counter for marking that there is a STAT_QUERY ramrod pending
    pub stats_pending: u16,
// Counter for completed statistics ramrods
    pub stats_comp: u16,
// End of fields used in the performance code paths
    pub panic: c_int,
    pub msg_enable: c_int,
    pub flags: u32,

    pub cnic_support: u8,
    pub cnic_enabled: bool,
    pub cnic_loaded: bool,
    pub ): *mut *mut *mut cnic_eth_dev (cnic_probe)(net_device,
    pub nic_stopped: bool,
// Flag that indicates that we can start looking for FCoE L2 queue
// completions in the default status block.
//
    pub fcoe_init: bool,
    pub mrrs: c_int,
    pub sp_task: delayed_work,
    pub iov_task: delayed_work,
    pub interrupt_occurred: core::sync::atomic::AtomicI32,
    pub sp_rtnl_task: delayed_work,
    pub period_task: delayed_work,
    pub timer: timer_list,
    pub current_interval: c_int,
    pub fw_seq: u16,
    pub fw_drv_pulse_wr_seq: u16,
    pub func_stx: u32,
    pub link_params: link_params,
    pub link_vars: link_vars,
    pub link_cnt: u32,
    pub last_reported_link: bnx2x_link_report_data,
    pub force_link_down: bool,
    pub mdio: mdio_if_info,
    pub common: bnx2x_common,
    pub port: bnx2x_port,
    pub cmng: cmng_init,
    pub mf_config: [u32; E1HVN_MAX],
    pub mf_ext_config: u32,
    pub /: *mut *mut u32 path_has_ovlan; / E3,
    pub mf_ov: u16,
    pub mf_mode: u8,

    pub mf_sub_mode: u8,

    pub wol: u8,
    pub rx_ring_size: c_int,
    pub tx_quick_cons_trip_int: u16,
    pub tx_quick_cons_trip: u16,
    pub tx_ticks_int: u16,
    pub tx_ticks: u16,
    pub rx_quick_cons_trip_int: u16,
    pub rx_quick_cons_trip: u16,
    pub rx_ticks_int: u16,
    pub rx_ticks: u16,
// Maximal coalescing timeout in us

    pub lin_cnt: u32,
    pub state: u16,
pub const BNX2X_STATE_CLOSED: c_int = 0;
pub const BNX2X_STATE_OPENING_WAIT4_LOAD: c_uint = 0x1000;
pub const BNX2X_STATE_OPENING_WAIT4_PORT: c_uint = 0x2000;
pub const BNX2X_STATE_OPEN: c_uint = 0x3000;
pub const BNX2X_STATE_CLOSING_WAIT4_HALT: c_uint = 0x4000;
pub const BNX2X_STATE_CLOSING_WAIT4_DELETE: c_uint = 0x5000;
pub const BNX2X_STATE_DIAG: c_uint = 0xe000;
pub const BNX2X_STATE_ERROR: c_uint = 0xf000;
pub const BNX2X_MAX_PRIORITY: c_int = 8;
    pub num_queues: c_int,
    pub num_ethernet_queues: c_uint,
    pub num_cnic_queues: c_uint,
    pub disable_tpa: c_int,
    pub rx_mode: u32,
pub const BNX2X_RX_MODE_NONE: c_int = 0;
pub const BNX2X_RX_MODE_NORMAL: c_int = 1;
pub const BNX2X_RX_MODE_ALLMULTI: c_int = 2;
pub const BNX2X_RX_MODE_PROMISC: c_int = 3;
pub const BNX2X_MAX_MULTICAST: c_int = 64;
    pub igu_dsb_id: u8,
    pub igu_base_sb: u8,
    pub igu_sb_cnt: u8,
    pub min_msix_vec_cnt: u8,
    pub igu_base_addr: u32,
    pub def_status_blk_mapping: dma_addr_t,
    pub slowpath: *mut bnx2x_slowpath,
    pub slowpath_mapping: dma_addr_t,
// Mechanism protecting the drv_info_to_mcp
    pub drv_info_mutex: mutex,
    pub drv_info_mng_owner: bool,
// Total number of FW statistics requests
    pub fw_stats_num: u8,
//
// This is a memory buffer that will contain both statistics
// ramrod request and data.
//
    pub fw_stats: *mut c_void,
    pub fw_stats_mapping: dma_addr_t,
//
// FW statistics request shortcut (points at the
// beginning of fw_stats buffer).
//
    pub fw_stats_req: *mut bnx2x_fw_stats_req,
    pub fw_stats_req_mapping: dma_addr_t,
    pub fw_stats_req_sz: c_int,
//
// FW statistics data shortcut (points at the beginning of
// fw_stats buffer + fw_stats_req_sz).
//
    pub fw_stats_data: *mut bnx2x_fw_stats_data,
    pub fw_stats_data_mapping: dma_addr_t,
    pub fw_stats_data_sz: c_int,
// For max 1024 cids (VF RSS), 32KB ILT page size and 1KB
// context size we need 8 ILT entries.
//
pub const ILT_MAX_L2_LINES: c_int = 32;
    pub context: [hw_context; ILT_MAX_L2_LINES],
    pub ilt: *mut bnx2x_ilt,

pub const ILT_MAX_LINES: c_int = 256;
//
// Maximum supported number of RSS queues: number of IGU SBs minus one that goes
// to CNIC.
//

//
// Maximum CID count that might be required by the bnx2x:
// Max RSS * Max_Tx_Multi_Cos + FCoE + iSCSI
//

    pub qm_cid_count: c_int,
    pub dropless_fc: bool,
    pub t2: *mut c_void,
    pub t2_mapping: dma_addr_t,
    pub cnic_ops: *mut cnic_ops __rcu,
    pub cnic_data: *mut c_void,
    pub cnic_tag: u32,
    pub cnic_eth_dev: cnic_eth_dev,
    pub cnic_sb: host_hc_status_block,
    pub cnic_sb_mapping: dma_addr_t,
    pub cnic_kwq: *mut eth_spe,
    pub cnic_kwq_prod: *mut eth_spe,
    pub cnic_kwq_cons: *mut eth_spe,
    pub cnic_kwq_last: *mut eth_spe,
    pub cnic_kwq_pending: u16,
    pub cnic_spq_pending: u16,
    pub fip_mac: [u8; ETH_ALEN],
    pub cnic_mutex: mutex,
    pub iscsi_l2_mac_obj: bnx2x_vlan_mac_obj,
// Start index of the "special" (CNIC related) L2 clients
    pub cnic_base_cl_id: u8,
    pub dmae_ready: c_int,
// used to synchronize dmae accesses
    pub dmae_lock: spinlock_t,
// used to protect the FW mail box
    pub fw_mb_mutex: mutex,
// used to synchronize stats collecting
    pub stats_state: c_int,
// used for synchronization of concurrent threads statistics handling
    pub stats_lock: semaphore,
// used by dmae command loader
    pub stats_dmae: dmae_command,
    pub executer_idx: c_int,
    pub stats_counter: u16,
    pub eth_stats: bnx2x_eth_stats,
    pub func_stats: host_func_stats,
    pub eth_stats_old: bnx2x_eth_stats_old,
    pub net_stats_old: bnx2x_net_stats_old,
    pub fw_stats_old: bnx2x_fw_port_stats_old,
    pub stats_init: bool,
    pub strm: *mut z_stream_s,
    pub gunzip_buf: *mut c_void,
    pub gunzip_mapping: dma_addr_t,
    pub gunzip_outlen: c_int,
pub const FW_BUF_SIZE: c_uint = 0x8000;

    pub init_ops: *mut raw_op,
// Init blocks offsets inside init_ops
    pub init_ops_offsets: *mut u16,
// Data blob - has 32 bit granularity
    pub init_data: *mut u32,
    pub init_mode_flags: u32,

// Zipped PRAM blobs - raw data
    pub tsem_int_table_data: *const u8,
    pub tsem_pram_data: *const u8,
    pub usem_int_table_data: *const u8,
    pub usem_pram_data: *const u8,
    pub xsem_int_table_data: *const u8,
    pub xsem_pram_data: *const u8,
    pub csem_int_table_data: *const u8,
    pub csem_pram_data: *const u8,

pub const PHY_FW_VER_LEN: c_int = 20;
    pub fw_ver: [c_char; 32],
    pub firmware: *const firmware,
    pub vfdb: *mut bnx2x_vfdb,

// DCB support on/off
    pub dcb_state: u16,
pub const BNX2X_DCB_STATE_OFF: c_int = 0;
pub const BNX2X_DCB_STATE_ON: c_int = 1;
// DCBX engine mode
    pub dcbx_enabled: c_int,
pub const BNX2X_DCBX_ENABLED_OFF: c_int = 0;
pub const BNX2X_DCBX_ENABLED_ON_NEG_OFF: c_int = 1;
pub const BNX2X_DCBX_ENABLED_ON_NEG_ON: c_int = 2;

    pub dcbx_mode_uset: bool,
    pub dcbx_config_params: bnx2x_config_dcbx_params,
    pub dcbx_port_params: bnx2x_dcbx_port_params,
    pub dcb_version: c_int,
// CAM credit pools
    pub vlans_pool: bnx2x_credit_pool_obj,
    pub macs_pool: bnx2x_credit_pool_obj,
// RX_MODE object
    pub rx_mode_obj: bnx2x_rx_mode_obj,
// MCAST object
    pub mcast_obj: bnx2x_mcast_obj,
// RSS configuration object
    pub rss_conf_obj: bnx2x_rss_config_obj,
// Function State controlling object
    pub func_obj: bnx2x_func_sp_obj,
    pub sp_state: c_ulong,
// operation indication for the sp_rtnl task
    pub sp_rtnl_state: c_ulong,
// Indication of the IOV tasks
    pub iov_task_state: c_ulong,
// DCBX Negotiation results
    pub dcbx_local_feat: dcbx_features,
    pub dcbx_error: u32,

    pub dcbx_remote_feat: dcbx_features,
    pub dcbx_remote_flags: u32,

// AFEX: store default vlan used
    pub afex_def_vlan_tag: c_int,
    pub afex_vlan_mode: mf_cfg_afex_vlan_mode,
    pub pending_max: u32,
// multiple tx classes of service
    pub max_cos: u8,
// priority to cos mapping
    pub prio_to_cos: [u8; 8],
    pub fp_array_size: c_int,
    pub dump_preset_idx: u32,
    pub phys_port_id: [u8; ETH_ALEN],
// PTP related context
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub ptp_task: work_struct,
    pub cyclecounter: cyclecounter,
    pub timecounter: timecounter,
    pub timecounter_init_done: bool,
    pub ptp_tx_skb: *mut sk_buff,
    pub ptp_tx_start: c_ulong,
    pub hwtstamp_ioctl_called: bool,
    pub tx_type: u16,
    pub rx_filter: u16,
    pub vf_link_vars: bnx2x_link_report_data,
    pub vlan_reg: list_head,
    pub vlan_cnt: u16,
    pub vlan_credit: u16,
    pub accept_any_vlan: bool,
// Vxlan/Geneve related information
    pub udp_tunnel_ports: [u16; BNX2X_UDP_PORT_MAX],
    pub fw_cap: u32,
    pub fw_major: u32,
    pub fw_minor: u32,
    pub fw_rev: u32,
    pub fw_eng: u32,
}

// Tx queues may be less or equal to Rx queues

// #define is_eth_multi(bp)	(BNX2X_NUM_ETH_QUEUES(bp) > 1)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_init_params {
// dma
    pub spq_active: bool,
    pub spq_map: dma_addr_t,
    pub spq_prod: u16,
    pub /: *mut *mut u16 func_id; / abs fid,
    pub pf_id: u16,
}

// Skip forwarding FP

// Skip OOO FP

// skip rx queue
// if FCOE l2 support is disabled and this is the fcoe L2 queue
//

// skip tx queue
// if FCOE l2 support is disabled and this is the fcoe L2 queue
//

// self test
extern "C" {
    pub fn bnx2x_idle_chk(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_set_mac_one - configure a single MAC address
//
// @bp:			driver handle
// @mac:		MAC to configure
// @obj:		MAC object handle
// @set:		if 'true' add a new MAC, otherwise - delete
// @mac_type:		the type of the MAC to configure (e.g. ETH, UC list)
// @ramrod_flags:	RAMROD_XXX flags (e.g. RAMROD_CONT, RAMROD_COMP_WAIT)
//
// Configures one MAC according to provided parameters or continues the
// execution of previously scheduled commands if RAMROD_CONT is set in
// ramrod_flags.
//
// Returns zero if operation has successfully completed, a positive value if the
// operation has been successfully scheduled and a negative - if a requested
// operations has failed.
//
// bnx2x_del_all_macs - delete all MACs configured for the specific MAC object
//
// @bp:			driver handle
// @mac_obj:		MAC object handle
// @mac_type:		type of the MACs to clear (BNX2X_XXX_MAC)
// @wait_for_comp:	if 'true' block until completion
//
// Deletes all MACs of the specific type (e.g. ETH, UC list).
//
// Returns zero if operation has successfully completed, a positive value if the
// operation has been successfully scheduled and a negative - if a requested
// operations has failed.
//
// Init Function API
extern "C" {
    pub fn bnx2x_func_init(bp: *mut bnx2x, p: *mut bnx2x_func_init_params);
}
extern "C" {
    pub fn bnx2x_get_gpio(bp: *mut bnx2x, gpio_num: c_int, port: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_gpio(bp: *mut bnx2x, gpio_num: c_int, mode: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_mult_gpio(bp: *mut bnx2x, pins: u8, mode: u32) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_gpio_int(bp: *mut bnx2x, gpio_num: c_int, mode: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_read_mf_cfg(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_pretend_func(bp: *mut bnx2x, pretend_func_val: u16) -> c_int;
}
// dmae
extern "C" {
    pub fn bnx2x_read_dmae(bp: *mut bnx2x, src_addr: u32, len32: u32);
}
extern "C" {
    pub fn bnx2x_post_dmae(bp: *mut bnx2x, dmae: *mut dmae_command, idx: c_int);
}
extern "C" {
    pub fn bnx2x_dmae_opcode_add_comp(opcode: u32, comp_type: u8) -> u32;
}
extern "C" {
    pub fn bnx2x_dmae_opcode_clr_src_reset(opcode: u32) -> u32;
}
// FLR related routines
extern "C" {
    pub fn bnx2x_flr_clnup_poll_count(bp: *mut bnx2x) -> u32;
}
extern "C" {
    pub fn bnx2x_tx_hw_flushed(bp: *mut bnx2x, poll_count: u32);
}
extern "C" {
    pub fn bnx2x_send_final_clnup(bp: *mut bnx2x, clnup_func: u8, poll_cnt: u32) -> c_int;
}
extern "C" {
    pub fn bnx2x_is_pcie_pending(dev: *mut pci_dev) -> u8;
}
extern "C" {
    pub fn bnx2x_calc_fc_adv(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_update_coalesce(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_get_cur_phy_idx(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_port_after_undi(bp: *mut bnx2x) -> bool;
}

// In 57710/11 we use whole table since we have 8 func
// In 57712 we have only 4 func, but use same size per func, then only half of
// the table in use
//

//
// the phys address is shifted right 12 bits and has an added
// 1=valid bit added to the 53rd bit
// then since this is a wide register(TM)
// we split it into two 32 bit writes
//

// load/unload mode
pub const LOAD_NORMAL: c_int = 0;
pub const LOAD_OPEN: c_int = 1;
pub const LOAD_DIAG: c_int = 2;
pub const LOAD_LOOPBACK_EXT: c_int = 3;
pub const UNLOAD_NORMAL: c_int = 0;
pub const UNLOAD_CLOSE: c_int = 1;
pub const UNLOAD_RECOVERY: c_int = 2;
// DMAE command defines

pub const DMAE_PCI_ERR_FLAG: c_uint = 0x80000000;
pub const DMAE_SRC_PCI: c_int = 0;
pub const DMAE_SRC_GRC: c_int = 1;
pub const DMAE_DST_NONE: c_int = 0;
pub const DMAE_DST_PCI: c_int = 1;
pub const DMAE_DST_GRC: c_int = 2;
pub const DMAE_COMP_PCI: c_int = 0;
pub const DMAE_COMP_GRC: c_int = 1;
// E2 and onward - PCI error handling in the completion
pub const DMAE_COMP_REGULAR: c_int = 0;
pub const DMAE_COM_SET_ERR: c_int = 1;

pub const DMAE_CMD_PORT_0: c_int = 0;

pub const DMAE_SRC_PF: c_int = 0;
pub const DMAE_SRC_VF: c_int = 1;
pub const DMAE_DST_PF: c_int = 0;
pub const DMAE_DST_VF: c_int = 1;
pub const DMAE_C_SRC: c_int = 0;
pub const DMAE_C_DST: c_int = 1;
pub const DMAE_LEN32_RD_MAX: c_uint = 0x80;

pub const DMAE_COMP_VAL: c_uint = 0x60d0d0ae /* E2 and on - upper bit;
// indicates error
//
pub const MAX_DMAE_C_PER_PORT: c_int = 8;

// Following is the DMAE channel number allocation for the clients.
// MFW: OCBB/OCSD implementations use DMAE channels 14/15 respectively.
// Driver: 0-3 and 8-11 (for PF dmae operations)
// 4 and 12 (for stats requests)
//

// PCIE link and speed
pub const PCICFG_LINK_WIDTH: c_uint = 0x1f00000;
pub const PCICFG_LINK_WIDTH_SHIFT: c_int = 20;
pub const PCICFG_LINK_SPEED: c_uint = 0xf0000;
pub const PCICFG_LINK_SPEED_SHIFT: c_int = 16;
pub const BNX2X_NUM_TESTS_SF: c_int = 7;
pub const BNX2X_NUM_TESTS_MF: c_int = 3;

pub const BNX2X_PHY_LOOPBACK: c_int = 0;
pub const BNX2X_MAC_LOOPBACK: c_int = 1;
pub const BNX2X_EXT_LOOPBACK: c_int = 2;
pub const BNX2X_PHY_LOOPBACK_FAILED: c_int = 1;
pub const BNX2X_MAC_LOOPBACK_FAILED: c_int = 2;
pub const BNX2X_EXT_LOOPBACK_FAILED: c_int = 3;

pub const STROM_ASSERT_ARRAY_SIZE: c_int = 50;
// must be used on a CID before placing it on a HW ring

pub const BNX2X_BTR: c_int = 4;
pub const MAX_SPQ_PENDING: c_int = 8;
// CMNG constants, as derived from system spec calculations
// default MIN rate in case VNIC min rate is configured to zero - 100Mbps
pub const DEF_MIN_RATE: c_int = 100;
// resolution of the rate shaping timer - 400 usec
pub const RS_PERIODIC_TIMEOUT_USEC: c_int = 400;
// number of bytes in single QM arbitration cycle -
// coefficient for calculating the fairness timer
pub const QM_ARB_BYTES: c_int = 160000;
// resolution of Min algorithm 1:100
pub const MIN_RES: c_int = 100;
// how many bytes above threshold for the minimal credit of Min algorithm
pub const MIN_ABOVE_THRESH: c_int = 32768;
// Fairness algorithm integration time coefficient -
// for calculating the actual Tfair

// Memory of fairness algorithm . 2 cycles
pub const FAIR_MEM: c_int = 2;

pub const ATTN_HARD_WIRED_MASK: c_uint = 0xff00;
pub const ATTENTION_ID: c_int = 4;

// stuff added to make the code fit 80Col

pub const MULTI_MASK: c_uint = 0x7f;

// Number of u32 elements in MC hash array
pub const MC_HASH_SIZE: c_int = 8;

pub const VENDOR_ID_LEN: c_int = 4;
pub const VF_ACQUIRE_THRESH: c_int = 3;
pub const VF_ACQUIRE_MAC_FILTERS: c_int = 1;
pub const VF_ACQUIRE_MC_FILTERS: c_int = 10;

extern "C" {
    pub fn bnx2x_compare_fw_ver(bp: *mut bnx2x, load_code: u32, print_err: bool) -> c_int;
}
// Congestion management fairness mode
pub const CMNG_FNS_NONE: c_int = 0;
pub const CMNG_FNS_MINMAX: c_int = 1;

pub const HC_SEG_ACCESS_ATTN: c_int = 4;

extern "C" {
    pub fn bnx2x_set_ethtool_ops(bp: *mut bnx2x, netdev: *mut net_device);
}
extern "C" {
    pub fn bnx2x_notify_link_changed(bp: *mut bnx2x);
}

// Determines whether BW configuration arrives in 100Mb units or in
// percentages from actual physical link speed.
//

pub const NUM_MACS: c_int = 8;
extern "C" {
    pub fn bnx2x_set_local_cmng(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_update_mng_version(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_update_mfw_dump(bp: *mut bnx2x);
}

extern "C" {
    pub fn bnx2x_init_ptp(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_configure_ptp_filters(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_rx_ts(bp: *mut bnx2x, skb: *mut sk_buff);
}
extern "C" {
    pub fn bnx2x_register_phc(bp: *mut bnx2x);
}
pub const BNX2X_MAX_PHC_DRIFT: c_int = 31000000;
// Macro flag: #define BNX2X_PTP_TX_TIMEOUT
// Re-configure all previously configured vlan filters.
// Meant for implicit re-load flows.
//
extern "C" {
    pub fn bnx2x_vlan_reconfigure_vid(bp: *mut bnx2x) -> c_int;
}
