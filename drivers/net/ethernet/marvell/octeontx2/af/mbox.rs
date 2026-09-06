//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/mbox.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2018 Marvell.
//

pub const MBOX_DOWN_MSG: c_int = 1;
pub const MBOX_UP_MSG: c_int = 2;
// AF/PF: PF initiated, PF/VF VF initiated
pub const MBOX_DOWN_RX_START: c_int = 0;

// AF/PF: AF initiated, PF/VF PF initiated

// Mailbox directions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_mbox_dev {
    pub /: *mut *mut *mut void mbase; / This dev's mbox region,
    pub hwbase: *mut c_void,
    pub mbox_lock: spinlock_t,
    pub /: *mut *mut u16 msg_size; / Total msg size to be sent,
    pub /: *mut *mut u16 rsp_size; / Total rsp size to be sure the reply is ok,
    pub /: *mut *mut u16 num_msgs; / No of msgs sent or waiting for response,
    pub /: *mut *mut u16 msgs_acked; / No of msgs for which response is received,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_mbox {
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut void hwbase; / Mbox region advertised by HW,
    pub /: *mut *mut *mut void reg_base;/ CSR base for this dev,
    pub /: *mut *mut u64 trigger; / Trigger mbox notification,
    pub /: *mut *mut u16 tr_shift; / Mbox trigger shift,
    pub /: *mut *mut u64 rx_start; / Offset of Rx region in mbox memory,
    pub /: *mut *mut u64 tx_start; / Offset of Tx region in mbox memory,
    pub /: *mut *mut u16 rx_size; / Size of Rx region,
    pub /: *mut *mut u16 tx_size; / Size of Tx region,
    pub /: *mut *mut u16 ndevs; / The number of peers,
    pub dev: *mut otx2_mbox_dev,
}

// Header which precedes all mbox messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_hdr {
    pub /: *mut *mut u64 msg_size; / Total msgs size embedded,
    pub /: *mut *mut u16 num_msgs; / No of msgs embedded,
    pub opt_msg: u16,
    pub sig: u8,
}

// Header which precedes every msg and is also part of it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_msghdr {
    pub /: *mut *mut u16 pcifunc; / Who's sending this msg,
    pub /: *mut *mut u16 id; / Mbox message ID,

    pub /: *mut *mut u16 sig; / Signature, for validating corrupted msgs,

    pub /: *mut *mut u16 ver; / Version of msg's structure for this ID,
    pub /: *mut *mut u16 next_msgoff; / Offset of next msg within mailbox region,
    pub /: *mut *mut int rc; / Msg process'ed response code,
}

extern "C" {
    pub fn otx2_mbox_reset(mbox: *mut otx2_mbox, devid: c_int);
}
extern "C" {
    pub fn __otx2_mbox_reset(mbox: *mut otx2_mbox, devid: c_int);
}
extern "C" {
    pub fn otx2_mbox_destroy(mbox: *mut otx2_mbox);
}
extern "C" {
    pub fn otx2_mbox_msg_send(mbox: *mut otx2_mbox, devid: c_int);
}
extern "C" {
    pub fn otx2_mbox_msg_send_up(mbox: *mut otx2_mbox, devid: c_int);
}
extern "C" {
    pub fn otx2_mbox_wait_for_rsp(mbox: *mut otx2_mbox, devid: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_mbox_busy_poll_for_rsp(mbox: *mut otx2_mbox, devid: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_mbox_check_rsp_msgs(mbox: *mut otx2_mbox, devid: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_mbox_nonempty(mbox: *mut otx2_mbox, devid: c_int) -> bool;
}
extern "C" {
    pub fn otx2_mbox_alloc_msg_rsp(_arg: mbox, _arg: devid, _arg: size, _arg: 0) -> return;
}
extern "C" {
    pub fn otx2_mbox_wait_for_zero(mbox: *mut otx2_mbox, devid: c_int) -> bool;
}
// Mailbox message types
pub const MBOX_MSG_MASK: c_uint = 0xFFFF;
pub const MBOX_MSG_INVALID: c_uint = 0xFFFE;
pub const MBOX_MSG_MAX: c_uint = 0xFFFF;

// Generic mbox IDs (range 0x000 - 0x1FF) */				\
// CGX mbox IDs (range 0x200 - 0x3FF) */				\
// NPA mbox IDs (range 0x400 - 0x5FF) */				\
// SSO/SSOW mbox IDs (range 0x600 - 0x7FF) */				\
// TIM mbox IDs (range 0x800 - 0x9FF) */				\
// CPT mbox IDs (range 0xA00 - 0xBFF) */				\
// SDP mbox IDs (range 0x1000 - 0x11FF) */				\
// NPC mbox IDs (range 0x6000 - 0x7FFF) */				\
// NIX mbox IDs (range 0x8000 - 0xFFFF) */				\
// MCS mbox IDs (range 0xA000 - 0xBFFF) */					\
// Messages initiated by AF (range 0xC00 - 0xEFF)

// Mailbox message formats
pub const RVU_DEFAULT_PF_FUNC: c_uint = 0xFFFF;
// Generic request msg used for those mbox messages which
// don't send any data in the request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_req {
    pub hdr: mbox_msghdr,
}

// Generic response msg used an ack or response for those mbox
// messages which don't have a specific rsp msg format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_rsp {
    pub hdr: mbox_msghdr,
}

// RVU mailbox error codes
// Range 256 - 300.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_af_status {
    RVU_INVALID_VF_ID           = -256,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ready_msg_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 sclk_freq; / SCLK frequency (in MHz),
    pub /: *mut *mut u16 rclk_freq; / RCLK frequency (in MHz),
}

// Structure for requesting resource provisioning.
// 'modify' flag to be used when either requesting more
// or to detach partial of a certain resource type.
// Rest of the fields specify how many of what type to
// be attached.
// To request LFs from two blocks of same type this mailbox
// can be sent twice as below:
// struct rsrc_attach *attach;
// .. Allocate memory for message ..
// attach->cptlfs = 3; <3 LFs from CPT0>
// .. Send message ..
// .. Allocate memory for message ..
// attach->modify = 1;
// attach->cpt_blkaddr = BLKADDR_CPT1;
// attach->cptlfs = 2; <2 LFs from CPT1>
// .. Send message ..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsrc_attach {
    pub hdr: mbox_msghdr,
    pub modify:1: u8,
    pub npalf:1: u8,
    pub nixlf:1: u8,
    pub sso: u16,
    pub ssow: u16,
    pub timlfs: u16,
    pub cptlfs: u16,
    pub /: *mut *mut int cpt_blkaddr; / BLKADDR_CPT0/BLKADDR_CPT1 or 0 for BLKADDR_CPT0,
}

// Structure for relinquishing resources.
// 'partial' flag to be used when relinquishing all resources
// but only of a certain type. If not set, all resources of all
// types provisioned to the RVU function will be detached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsrc_detach {
    pub hdr: mbox_msghdr,
    pub partial:1: u8,
    pub npalf:1: u8,
    pub nixlf:1: u8,
    pub sso:1: u8,
    pub ssow:1: u8,
    pub timlfs:1: u8,
    pub cptlfs:1: u8,
}

// Number of resources available to the caller.
// In reply to MBOX_MSG_FREE_RSRC_CNT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct free_rsrcs_rsp {
    pub hdr: mbox_msghdr,
    pub schq: [u16; NIX_TXSCH_LVL_CNT],
    pub sso: u16,
    pub tim: u16,
    pub ssow: u16,
    pub cpt: u16,
    pub npa: u8,
    pub nix: u8,
    pub schq_nix1: [u16; NIX_TXSCH_LVL_CNT],
    pub nix1: u8,
    pub cpt1: u8,
    pub ree0: u8,
    pub ree1: u8,
}

pub const MSIX_VECTOR_INVALID: c_uint = 0xFFFF;
pub const MAX_RVU_BLKLF_CNT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msix_offset_rsp {
    pub hdr: mbox_msghdr,
    pub npa_msixoff: u16,
    pub nix_msixoff: u16,
    pub sso: u16,
    pub ssow: u16,
    pub timlfs: u16,
    pub cptlfs: u16,
    pub sso_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub ssow_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub timlf_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub cptlf_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub cpt1_lfs: u16,
    pub ree0_lfs: u16,
    pub ree1_lfs: u16,
    pub cpt1_lf_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub ree0_lf_msixoff: [u16; MAX_RVU_BLKLF_CNT],
    pub ree1_lf_msixoff: [u16; MAX_RVU_BLKLF_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_hw_cap_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 nix_fixed_txschq_mapping; / Schq mapping fixed or flexible,
    pub /: *mut *mut u8 nix_shaping; / Is shaping and coloring supported,
    pub /: *mut *mut u8 npc_hash_extract; / Is hash extract supported,

    pub hw_caps: u64,
}

// CGX mbox message formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_stats_rsp {
    pub hdr: mbox_msghdr,
pub const CGX_RX_STATS_COUNT: c_int = 9;
pub const CGX_TX_STATS_COUNT: c_int = 18;
    pub rx_stats: [u64; CGX_RX_STATS_COUNT],
    pub tx_stats: [u64; CGX_TX_STATS_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_fec_stats_rsp {
    pub hdr: mbox_msghdr,
    pub fec_corr_blks: u64,
    pub fec_uncorr_blks: u64,
}

// Structure for requesting the operation for
// setting/getting mac address in the CGX interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_set_or_get {
    pub hdr: mbox_msghdr,
    pub mac_addr: [u8; ETH_ALEN],
    pub index: u32,
}

// Structure for requesting the operation to
// add DMAC filter entry into CGX interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_add_req {
    pub hdr: mbox_msghdr,
    pub mac_addr: [u8; ETH_ALEN],
}

// Structure for response against the operation to
// add DMAC filter entry into CGX interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_add_rsp {
    pub hdr: mbox_msghdr,
    pub index: u32,
}

// Structure for requesting the operation to
// delete DMAC filter entry from CGX interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_del_req {
    pub hdr: mbox_msghdr,
    pub index: u32,
}

// Structure for response against the operation to
// get maximum supported DMAC filter entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_max_dmac_entries_get_rsp {
    pub hdr: mbox_msghdr,
    pub max_dmac_filters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_link_user_info {
    pub link_up:1: u64,
    pub full_duplex:1: u64,
    pub lmac_type_id:4: u64,
    pub /: *mut *mut uint64_t speed:20; / speed in Mbps,
    pub /: *mut *mut uint64_t an:1; / AN supported or not,
    pub /: *mut *mut uint64_t fec:2; / FEC type if enabled else 0,
pub const LMACTYPE_STR_LEN: c_int = 16;
    pub lmac_type: [c_char; LMACTYPE_STR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_link_info_msg {
    pub hdr: mbox_msghdr,
    pub link_info: cgx_link_user_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_pause_frm_cfg {
    pub hdr: mbox_msghdr,
    pub set: u8,
// set = 1 if the request is to config pause frames
// set = 0 if the request is to fetch pause frames config
    pub rx_pause: u8,
    pub tx_pause: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fec_type {
    OTX2_FEC_NONE,
    OTX2_FEC_BASER,
    OTX2_FEC_RS,
    OTX2_FEC_STATS_CNT = 2,
    OTX2_FEC_OFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_mode {
    pub hdr: mbox_msghdr,
    pub fec: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_eeprom_s {
pub const SFP_EEPROM_SIZE: c_int = 256;
    pub sff_id: u16,
    pub buf: [u8; SFP_EEPROM_SIZE],
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_s {
    pub can_change_mod_type:1: u64,
    pub mod_type:1: u64,
    pub has_fec_stats:1: u64,
    pub misc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_stats_s {
    pub rsfec_corr_cws: u32,
    pub rsfec_uncorr_cws: u32,
    pub brfec_corr_blks: u32,
    pub brfec_uncorr_blks: u32,
    pub fec_stats: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_lmac_fwdata_s {
    pub rw_valid: u16,
    pub supported_fec: u64,
    pub supported_an: u64,
    pub supported_link_modes: u64,
// only applicable if AN is supported
    pub advertised_fec: u64,
    pub /: *mut *mut u64 advertised_link_modes_own:1; / CGX_CMD_OWN,
    pub advertised_link_modes:63: u64,
// Only applicable if SFP/QSFP slot is present
    pub sfp_eeprom: sfp_eeprom_s,
    pub phy: phy_s,
    pub lmac_type: u32,
    pub portm_idx: u32,
    pub mgmt_port:1: u64,
    pub advertised_an:1: u64,
    pub port: u64,
pub const LMAC_FWDATA_RESERVED_MEM: c_int = 1018;
    pub reserved: [u64; LMAC_FWDATA_RESERVED_MEM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_fw_data {
    pub hdr: mbox_msghdr,
    pub fwdata: cgx_lmac_fwdata_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_set_link_mode_args {
    pub speed: u32,
    pub duplex: u8,
    pub an: u8,
    pub mode_baseidx: u8,
    pub multimode: u8,
    pub mode: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_set_link_mode_req {
    pub hdr: mbox_msghdr,
    pub args: cgx_set_link_mode_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_set_link_mode_rsp {
    pub hdr: mbox_msghdr,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_reset_req {
    pub hdr: mbox_msghdr,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_update_req {
    pub hdr: mbox_msghdr,
    pub mac_addr: [u8; ETH_ALEN],
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_mac_addr_update_rsp {
    pub hdr: mbox_msghdr,
    pub index: u32,
}

// flow control from physical link higig2 messages

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_features_info_msg {
    pub hdr: mbox_msghdr,
    pub lmac_features: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpm_stats_rsp {
    pub hdr: mbox_msghdr,
pub const RPM_RX_STATS_COUNT: c_int = 43;
pub const RPM_TX_STATS_COUNT: c_int = 34;
    pub rx_stats: [u64; RPM_RX_STATS_COUNT],
    pub tx_stats: [u64; RPM_TX_STATS_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_pfc_cfg {
    pub hdr: mbox_msghdr,
    pub rx_pause: u8,
    pub tx_pause: u8,
    pub /: *mut *mut u16 pfc_en; / bitmap indicating pfc enabled traffic classes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_pfc_rsp {
    pub hdr: mbox_msghdr,
    pub rx_pause: u8,
    pub tx_pause: u8,
}

// NPA mbox message formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_set_pkind {
    pub hdr: mbox_msghdr,

    pub mode: u64,

    pub dir: u8,
    pub /: *mut *mut u8 pkind; / valid only in case custom flag,
    pub field.: *mut *mut u8 var_len_off; / Offset of custom header length,
// Valid only for pkind NPC_RX_CUSTOM_PRE_L2_PKIND
//
    pub /: *mut *mut u8 var_len_off_mask; / Mask for length with in offset,
    pub /: *mut *mut u8 shift_dir; / shift direction to get length of the header at var_len_off,
    pub /: *mut *mut u8 skip_size; / l2 size to skip,
}

// NPA mbox message formats
// NPA mailbox error codes
// Range 301 - 400.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_af_status {
    NPA_AF_ERR_PARAM            = -301,
    NPA_AF_ERR_AQ_FULL          = -302,
    NPA_AF_ERR_AQ_ENQUEUE       = -303,
    NPA_AF_ERR_AF_LF_INVALID    = -304,
    NPA_AF_ERR_AF_LF_ALLOC      = -305,
    NPA_AF_ERR_LF_RESET         = -306,
}

// For NPA LF context alloc and init
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_lf_alloc_req {
    pub hdr: mbox_msghdr,
    pub node: c_int,
    pub /: *mut *mut int aura_sz; / No of auras,
    pub /: *mut *mut u32 nr_pools; / No of pools,
    pub way_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_lf_alloc_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u32 stack_pg_ptrs; / No of ptrs per stack page,
    pub /: *mut *mut u32 stack_pg_bytes; / Size of stack page,
    pub /: *mut *mut u16 qints; / NPA_AF_CONST::QINTS,
    pub /: *mut *mut u8 cache_lines; /BATCH ALLOC DMA,
}

// NPA AQ enqueue msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_enq_req {
    pub hdr: mbox_msghdr,
    pub aura_id: u32,
    pub ctype: u8,
    pub op: u8,
// Valid when op == WRITE/INIT and ctype == AURA.
// LF fills the pool_id in aura.pool_addr. AF will translate
// the pool_id to pool context pointer.
//
    pub aura: npa_aura_s,
// Valid when op == WRITE/INIT and ctype == POOL
    pub pool: npa_pool_s,
}

// Mask data when op == WRITE (1=write, 0=don't write)
// Valid when op == WRITE and ctype == AURA
// Valid when op == WRITE and ctype == POOL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_enq_rsp {
    pub hdr: mbox_msghdr,
// Valid when op == READ and ctype == AURA
    pub aura: npa_aura_s,
// Valid when op == READ and ctype == POOL
    pub pool: npa_pool_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_cn20k_aq_enq_req {
    pub hdr: mbox_msghdr,
    pub aura_id: u32,
    pub ctype: u8,
    pub op: u8,
// Valid when op == WRITE/INIT and ctype == AURA.
// LF fills the pool_id in aura.pool_addr. AF will translate
// the pool_id to pool context pointer.
//
    pub aura: npa_cn20k_aura_s,
// Valid when op == WRITE/INIT and ctype == POOL
    pub pool: npa_cn20k_pool_s,
}

// Mask data when op == WRITE (1=write, 0=don't write)
// Valid when op == WRITE and ctype == AURA
// Valid when op == WRITE and ctype == POOL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_cn20k_aq_enq_rsp {
    pub hdr: mbox_msghdr,
// Valid when op == READ and ctype == AURA
    pub aura: npa_cn20k_aura_s,
// Valid when op == READ and ctype == POOL
    pub pool: npa_cn20k_pool_s,
}

// Disable all contexts of type 'ctype'
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwctx_disable_req {
    pub hdr: mbox_msghdr,
    pub ctype: u8,
}

// NIX mbox message formats
// NIX mailbox error codes
// Range 401 - 500.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_af_status {
    NIX_AF_ERR_PARAM            = -401,
    NIX_AF_ERR_AQ_FULL          = -402,
    NIX_AF_ERR_AQ_ENQUEUE       = -403,
    NIX_AF_ERR_AF_LF_INVALID    = -404,
    NIX_AF_ERR_AF_LF_ALLOC      = -405,
    NIX_AF_ERR_TLX_ALLOC_FAIL   = -406,
    NIX_AF_ERR_TLX_INVALID      = -407,
    NIX_AF_ERR_RSS_SIZE_INVALID = -408,
    NIX_AF_ERR_RSS_GRPS_INVALID = -409,
    NIX_AF_ERR_FRS_INVALID      = -410,
    NIX_AF_ERR_RX_LINK_INVALID  = -411,
    NIX_AF_INVAL_TXSCHQ_CFG     = -412,
    NIX_AF_SMQ_FLUSH_FAILED     = -413,
    NIX_AF_ERR_LF_RESET         = -414,
    NIX_AF_ERR_RSS_NOSPC_FIELD  = -415,
    NIX_AF_ERR_RSS_NOSPC_ALGO   = -416,
    NIX_AF_ERR_MARK_CFG_FAIL    = -417,
    NIX_AF_ERR_LSO_CFG_FAIL     = -418,
    NIX_AF_INVAL_NPA_PF_FUNC    = -419,
    NIX_AF_INVAL_SSO_PF_FUNC    = -420,
    NIX_AF_ERR_TX_VTAG_NOSPC    = -421,
    NIX_AF_ERR_RX_VTAG_INUSE    = -422,
    NIX_AF_ERR_PTP_CONFIG_FAIL  = -423,
    NIX_AF_ERR_NPC_KEY_NOT_SUPP = -424,
    NIX_AF_ERR_INVALID_NIXBLK   = -425,
    NIX_AF_ERR_INVALID_BANDPROF = -426,
    NIX_AF_ERR_IPOLICER_NOTSUPP = -427,
    NIX_AF_ERR_BANDPROF_INVAL_REQ  = -428,
    NIX_AF_ERR_CQ_CTX_WRITE_ERR  = -429,
    NIX_AF_ERR_AQ_CTX_RETRY_WRITE  = -430,
    NIX_AF_ERR_LINK_CREDITS  = -431,
    NIX_AF_ERR_INVALID_BPID         = -434,
    NIX_AF_ERR_INVALID_BPID_REQ     = -435,
    NIX_AF_ERR_INVALID_MCAST_GRP	= -436,
    NIX_AF_ERR_INVALID_MCAST_DEL_REQ = -437,
    NIX_AF_ERR_NON_CONTIG_MCE_LIST = -438,
    NIX_AF_ERR_RX_SW_SYNC_FAIL	= -439,
}

// For NIX RX vtag action
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_rx_vtag0_type {
    NIX_AF_LFX_RX_VTAG_TYPE0, /* reserved for rx vlan offload */
    NIX_AF_LFX_RX_VTAG_TYPE1,
    NIX_AF_LFX_RX_VTAG_TYPE2,
    NIX_AF_LFX_RX_VTAG_TYPE3,
    NIX_AF_LFX_RX_VTAG_TYPE4,
    NIX_AF_LFX_RX_VTAG_TYPE5,
    NIX_AF_LFX_RX_VTAG_TYPE6,
    NIX_AF_LFX_RX_VTAG_TYPE7,
}

// For NIX LF context alloc and init
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lf_alloc_req {
    pub hdr: mbox_msghdr,
    pub node: c_int,
    pub /: *mut *mut u32 rq_cnt; / No of receive queues,
    pub /: *mut *mut u32 sq_cnt; / No of send queues,
    pub /: *mut *mut u32 cq_cnt; / No of completion queues,
    pub xqe_sz: u8,
    pub rss_sz: u16,
    pub rss_grps: u8,
    pub npa_func: u16,
    pub sso_func: u16,
    pub /: *mut *mut u64 rx_cfg; / See NIX_AF_LF(0..127)_RX_CFG,
    pub way_mask: u64,

    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lf_alloc_rsp {
    pub hdr: mbox_msghdr,
    pub sqb_size: u16,
    pub rx_chan_base: u16,
    pub tx_chan_base: u16,
    pub /: *mut *mut u8 rx_chan_cnt; / total number of RX channels,
    pub /: *mut *mut u8 tx_chan_cnt; / total number of TX channels,
    pub lso_tsov4_idx: u8,
    pub lso_tsov6_idx: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 lf_rx_stats; / NIX_AF_CONST1::LF_RX_STATS,
    pub /: *mut *mut u8 lf_tx_stats; / NIX_AF_CONST1::LF_TX_STATS,
    pub /: *mut *mut u16 cints; / NIX_AF_CONST2::CINTS,
    pub /: *mut *mut u16 qints; / NIX_AF_CONST2::QINTS,
    pub /: *mut *mut u8 cgx_links; / No. of CGX links present in HW,
    pub /: *mut *mut u8 lbk_links; / No. of LBK links present in HW,
    pub /: *mut *mut u8 sdp_links; / No. of SDP links present in HW,
    pub /: *mut *mut u8 tx_link; / Transmit channel link number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lf_free_req {
    pub hdr: mbox_msghdr,

    pub flags: u64,
}

// CN20K NIX AQ enqueue msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn20k_aq_enq_req {
    pub hdr: mbox_msghdr,
    pub qidx: u32,
    pub ctype: u8,
    pub op: u8,
    pub rq: nix_cn20k_rq_ctx_s,
    pub sq: nix_cn20k_sq_ctx_s,
    pub cq: nix_cn20k_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn20k_aq_enq_rsp {
    pub hdr: mbox_msghdr,
    pub rq: nix_cn20k_rq_ctx_s,
    pub sq: nix_cn20k_sq_ctx_s,
    pub cq: nix_cn20k_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

// CN10K NIX AQ enqueue msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn10k_aq_enq_req {
    pub hdr: mbox_msghdr,
    pub qidx: u32,
    pub ctype: u8,
    pub op: u8,
    pub rq: nix_cn10k_rq_ctx_s,
    pub sq: nix_cn10k_sq_ctx_s,
    pub cq: nix_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn10k_aq_enq_rsp {
    pub hdr: mbox_msghdr,
    pub rq: nix_cn10k_rq_ctx_s,
    pub sq: nix_cn10k_sq_ctx_s,
    pub cq: nix_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

// NIX AQ enqueue msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_aq_enq_req {
    pub hdr: mbox_msghdr,
    pub qidx: u32,
    pub ctype: u8,
    pub op: u8,
    pub rq: nix_rq_ctx_s,
    pub sq: nix_sq_ctx_s,
    pub cq: nix_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_aq_enq_rsp {
    pub hdr: mbox_msghdr,
    pub rq: nix_rq_ctx_s,
    pub sq: nix_sq_ctx_s,
    pub cq: nix_cq_ctx_s,
    pub rss: nix_rsse_s,
    pub mce: nix_rx_mce_s,
    pub prof: nix_bandprof_s,
}

// Tx scheduler/shaper mailbox messages
pub const MAX_TXSCHQ_PER_FUNC: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txsch_alloc_req {
    pub hdr: mbox_msghdr,
// Scheduler queue count request at each level
    pub /: *mut *mut u16 schq_contig[NIX_TXSCH_LVL_CNT]; / No of contiguous queues,
    pub /: *mut *mut u16 schq[NIX_TXSCH_LVL_CNT]; / No of non-contiguous queues,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txsch_alloc_rsp {
    pub hdr: mbox_msghdr,
// Scheduler queue count allocated at each level
    pub schq_contig: [u16; NIX_TXSCH_LVL_CNT],
    pub schq: [u16; NIX_TXSCH_LVL_CNT],
// Scheduler queue list allocated at each level
    pub schq_contig_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub schq_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub /: *mut *mut u8 aggr_level; / Traffic aggregation scheduler level,
    pub /: *mut *mut u8 aggr_lvl_rr_prio; / Aggregation lvl's RR_PRIO config,
    pub /: *mut *mut u8 link_cfg_lvl; / LINKX_CFG CSRs mapped to TL3 or TL2's index ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txsch_free_req {
    pub hdr: mbox_msghdr,

    pub flags: u16,
// Scheduler queue level to be freed
    pub schq_lvl: u16,
// List of scheduler queues to be freed
    pub schq: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txschq_config {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 lvl; / SMQ/MDQ/TL4/TL3/TL2/TL1,
    pub read: u8,
pub const TXSCHQ_IDX_SHIFT: c_int = 16;

    pub num_regs: u8,
pub const MAX_REGS_PER_MBOX_MSG: c_int = 20;
    pub reg: [u64; MAX_REGS_PER_MBOX_MSG],
    pub regval: [u64; MAX_REGS_PER_MBOX_MSG],
// All 0's => overwrite with new value
    pub regval_mask: [u64; MAX_REGS_PER_MBOX_MSG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_vtag_config {
    pub hdr: mbox_msghdr,
// '0' for 4 octet VTAG, '1' for 8 octet VTAG
    pub vtag_size: u8,
// cfg_type is '0' for tx vlan cfg
// cfg_type is '1' for rx vlan cfg
//
    pub cfg_type: u8,
// valid when cfg_type is '0'
    pub vtag0: u64,
    pub vtag1: u64,
// cfg_vtag0 & cfg_vtag1 fields are valid
// when free_vtag0 & free_vtag1 are '0's.
//
// cfg_vtag0 = 1 to configure vtag0
    pub :1: u8 cfg_vtag0,
// cfg_vtag1 = 1 to configure vtag1
    pub :1: u8 cfg_vtag1,
// vtag0_idx & vtag1_idx are only valid when
// both cfg_vtag0 & cfg_vtag1 are '0's,
// these fields are used along with free_vtag0
// & free_vtag1 to free the nix lf's tx_vlan
// configuration.
//
// Denotes the indices of tx_vtag def registers
// that needs to be cleared and freed.
//
    pub vtag0_idx: c_int,
    pub vtag1_idx: c_int,
// free_vtag0 & free_vtag1 fields are valid
// when cfg_vtag0 & cfg_vtag1 are '0's.
//
// free_vtag0 = 1 clears vtag0 configuration
// vtag0_idx denotes the index to be cleared.
//
    pub :1: u8 free_vtag0,
// free_vtag1 = 1 clears vtag1 configuration
// vtag1_idx denotes the index to be cleared.
//
    pub :1: u8 free_vtag1,
    pub tx: },
// valid when cfg_type is '1'
// rx vtag type index, valid values are in 0..7 range
    pub vtag_type: u8,
// rx vtag strip
    pub :1: u8 strip_vtag,
// rx vtag capture
    pub :1: u8 capture_vtag,
    pub rx: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_vtag_config_rsp {
    pub hdr: mbox_msghdr,
    pub vtag0_idx: c_int,
    pub vtag1_idx: c_int,
// Indices of tx_vtag def registers used to configure
// tx vtag0 & vtag1 headers, these indices are valid
// when nix_vtag_config mbox requested for vtag0 and
// or vtag1 configuration.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rss_flowkey_cfg {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut int mcam_index; / MCAM entry index to modify,

    pub /: *mut *mut u32 flowkey_cfg; / Flowkey types selected,
    pub /: *mut *mut u8 group; / RSS context or group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rss_flowkey_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 alg_idx; / Selected algo index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_set_mac_addr {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / MAC address to be set for this pcifunc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_get_mac_addr_rsp {
    pub hdr: mbox_msghdr,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mark_format_cfg {
    pub hdr: mbox_msghdr,
    pub offset: u8,
    pub y_mask: u8,
    pub y_val: u8,
    pub r_mask: u8,
    pub r_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mark_format_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub mark_format_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_mode {
    pub hdr: mbox_msghdr,

    pub mode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_cfg {
    pub hdr: mbox_msghdr,

    pub /: *mut *mut u8 len_verify; / Outer L3/L4 len check,

    pub /: *mut *mut u8 csum_verify; / Outer L4 checksum verification,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_frs_cfg {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 update_smq; / Update SMQ's min/max lens,
    pub /: *mut *mut u8 update_minlen; / Set minlen also,
    pub /: *mut *mut u8 sdp_link; / Set SDP RX link,
    pub maxlen: u16,
    pub minlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lso_format_cfg {
    pub hdr: mbox_msghdr,
    pub field_mask: u64,
pub const NIX_LSO_FIELD_MAX: c_int = 8;
    pub fields: [u64; NIX_LSO_FIELD_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lso_format_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub lso_format_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bp_cfg_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 chan_base; / Starting channel number,
    pub /: *mut *mut u8 chan_cnt; / Number of channels,
    pub bpid_per_chan: u8,
// bpid_per_chan = 0 assigns single bp id for range of channels
// bpid_per_chan = 1 assigns separate bp id for each channel
}

// Maximum channels any single NIX interface can have
pub const NIX_MAX_BPID_CHAN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bp_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 chan_bpid[NIX_MAX_BPID_CHAN]; / Channel and bpid mapping,
    pub /: *mut *mut u8 chan_cnt; / Number of channel for which bpids are assigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_create_req {
    pub hdr: mbox_msghdr,
pub const NIX_MCAST_INGRESS: c_int = 0;
pub const NIX_MCAST_EGRESS: c_int = 1;
    pub dir: u8,
    pub reserved: [u8; 11],
// Reserving few bytes for future requirement
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_create_rsp {
    pub hdr: mbox_msghdr,
// This mcast_grp_idx should be passed during MCAM
// write entry for multicast. AF will identify the
// corresponding multicast table index associated
// with the group id and program the same to MCAM entry.
// This group id is also needed during group delete
// and update request.
//
    pub mcast_grp_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_destroy_req {
    pub hdr: mbox_msghdr,
// Group id returned by nix_mcast_grp_create_rsp
    pub mcast_grp_idx: u32,
// If AF is requesting for destroy, then set
// it to '1'. Otherwise keep it to '0'
//
    pub is_af: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_update_req {
    pub hdr: mbox_msghdr,
// Group id returned by nix_mcast_grp_create_rsp
    pub mcast_grp_idx: u32,
// Number of multicast/mirror entries requested
    pub num_mce_entry: u32,
pub const NIX_MCE_ENTRY_MAX: c_int = 64;
pub const NIX_RX_RQ: c_int = 0;
pub const NIX_RX_RSS: c_int = 1;
// Receive queue or RSS index within pf_func
    pub rq_rss_index: [u32; NIX_MCE_ENTRY_MAX],
// pcifunc is required for both ingress and egress multicast
    pub pcifunc: [u16; NIX_MCE_ENTRY_MAX],
// channel is required for egress multicast
    pub channel: [u16; NIX_MCE_ENTRY_MAX],
pub const NIX_MCAST_OP_ADD_ENTRY: c_int = 0;
pub const NIX_MCAST_OP_DEL_ENTRY: c_int = 1;
// Destination type. 0:Receive queue, 1:RSS
    pub dest_type: [u8; NIX_MCE_ENTRY_MAX],
    pub op: u8,
// If AF is requesting for update, then set
// it to '1'. Otherwise keep it to '0'
//
    pub is_af: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_update_rsp {
    pub hdr: mbox_msghdr,
    pub mce_start_index: u32,
}

// Global NIX inline IPSec configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_inline_ipsec_cfg {
    pub hdr: mbox_msghdr,
    pub cpt_credit: u32,
    pub egrp: u8,
    pub opcode: u16,
    pub param1: u16,
    pub param2: u16,
    pub gen_cfg: },
    pub cpt_pf_func: u16,
    pub cpt_slot: u8,
    pub inst_qsel: },
    pub enable: u8,
    pub bpid: u16,
    pub credit_th: u32,
}

// Per NIX LF inline IPSec configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_inline_ipsec_lf_cfg {
    pub hdr: mbox_msghdr,
    pub sa_base_addr: u64,
    pub tag_const: u32,
    pub lenm1_max: u16,
    pub sa_pow2_size: u8,
    pub tt: u8,
    pub ipsec_cfg0: },
    pub sa_idx_max: u32,
    pub sa_idx_w: u8,
    pub ipsec_cfg1: },
    pub enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_hw_info {
    pub hdr: mbox_msghdr,
    pub vwqe_delay: u16,
    pub max_mtu: u16,
    pub min_mtu: u16,
    pub rpm_dwrr_mtu: u32,
    pub sdp_dwrr_mtu: u32,
    pub lbk_dwrr_mtu: u32,
    pub rsvd32: [u32; 1],
    pub /: *mut *mut u64 rsvd[15]; / Add reserved fields for future expansion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bandprof_alloc_req {
    pub hdr: mbox_msghdr,
// Count of profiles needed per layer
    pub prof_count: [u16; BAND_PROF_NUM_LAYERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bandprof_alloc_rsp {
    pub hdr: mbox_msghdr,
    pub prof_count: [u16; BAND_PROF_NUM_LAYERS],
// There is no need to allocate morethan 1 bandwidth profile
// per RQ of a PF_FUNC's NIXLF. So limit the maximum
// profiles to 64 per PF_FUNC.
//
pub const MAX_BANDPROF_PER_PFFUNC: c_int = 64;
    pub prof_idx: [u16; BAND_PROF_NUM_LAYERS][MAX_BANDPROF_PER_PFFUNC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bandprof_free_req {
    pub hdr: mbox_msghdr,
    pub free_all: u8,
    pub prof_count: [u16; BAND_PROF_NUM_LAYERS],
    pub prof_idx: [u16; BAND_PROF_NUM_LAYERS][MAX_BANDPROF_PER_PFFUNC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bandprof_get_hwinfo_rsp {
    pub hdr: mbox_msghdr,
    pub prof_count: [u16; BAND_PROF_NUM_LAYERS],
    pub policer_timeunit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_stats_req {
    pub hdr: mbox_msghdr,
    pub reset: u8,
    pub pcifunc: u16,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_stats_rsp {
    pub hdr: mbox_msghdr,
    pub pcifunc: u16,
    pub octs: u64,
    pub ucast: u64,
    pub bcast: u64,
    pub mcast: u64,
    pub drop: u64,
    pub drop_octs: u64,
    pub drop_mcast: u64,
    pub drop_bcast: u64,
    pub err: u64,
    pub rsvd: [u64; 5],
    pub rx: },
    pub ucast: u64,
    pub bcast: u64,
    pub mcast: u64,
    pub drop: u64,
    pub octs: u64,
    pub tx: },
}

// NPC mbox message structs
pub const NPC_MCAM_ENTRY_INVALID: c_uint = 0xFFFF;
pub const NPC_MCAM_INVALID_MAP: c_uint = 0xFFFF;
// NPC mailbox error codes
// Range 701 - 800.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_af_status {
    NPC_MCAM_INVALID_REQ	= -701,
    NPC_MCAM_ALLOC_DENIED	= -702,
    NPC_MCAM_ALLOC_FAILED	= -703,
    NPC_MCAM_PERM_DENIED	= -704,
    NPC_FLOW_INTF_INVALID	= -707,
    NPC_FLOW_CHAN_INVALID	= -708,
    NPC_FLOW_NO_NIXLF	= -709,
    NPC_FLOW_NOT_SUPPORTED	= -710,
    NPC_FLOW_VF_PERM_DENIED	= -711,
    NPC_FLOW_VF_NOT_INIT	= -712,
    NPC_FLOW_VF_OVERLAP	= -713,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_entry_req {
    pub hdr: mbox_msghdr,
pub const NPC_MAX_NONCONTIG_ENTRIES: c_int = 256;
    pub /: *mut *mut u8 contig; / Contiguous entries ?,
pub const NPC_MCAM_ANY_PRIO: c_int = 0;
pub const NPC_MCAM_LOWER_PRIO: c_int = 1;
pub const NPC_MCAM_HIGHER_PRIO: c_int = 2;
    pub /: *mut *mut u8 ref_prio; / Lower or higher w.r.t ref_entry,
    pub ref_entry: u16,
    pub /: *mut *mut u16 count; / Number of entries requested,
    pub /: *mut *mut u8 kw_type; / entry key type, valid for cn20k,
    pub /: *mut *mut u8 virt; / Request virtual index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_entry_rsp {
    pub hdr: mbox_msghdr,
    pub contiguous.: *mut *mut u16 entry; / Entry allocated or start index if,
// Invalid incase of non-contiguous.
//
    pub /: *mut *mut u16 count; / Number of entries allocated,
    pub /: *mut *mut u16 free_count; / Number of entries available,
    pub entry_list: [u16; NPC_MAX_NONCONTIG_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_free_entry_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 entry; / Entry index to be freed,
    pub /: *mut *mut u8 all; / If all entries allocated to this PFVF to be freed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcam_entry_mdata {
    pub kw: *mut u64,
    pub kw_mask: *mut u64,
    pub action: *mut u64,
    pub vtag_action: *mut u64,
    pub max_kw: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kws_in_key_sz {
    NPC_KWS_IN_KEY_SZ_7 = 7,
    NPC_KWS_IN_KEY_SZ_8 = 8,
    NPC_KWS_IN_KEY_SZ_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcam_entry {
    pub kw: [u64; NPC_KWS_IN_KEY_SZ_7],
    pub kw_mask: [u64; NPC_KWS_IN_KEY_SZ_7],
    pub action: u64,
    pub vtag_action: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn20k_mcam_entry {
    pub kw: [u64; NPC_KWS_IN_KEY_SZ_8],
    pub kw_mask: [u64; NPC_KWS_IN_KEY_SZ_8],
    pub action: u64,
    pub vtag_action: u64,
    pub action2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_mcam_write_entry_req {
    pub hdr: mbox_msghdr,
    pub entry_data: cn20k_mcam_entry,
    pub /: *mut *mut u16 entry; / MCAM entry to write this match key,
    pub /: *mut *mut u16 cntr; / Counter for this MCAM entry,
    pub /: *mut *mut u8 intf; / Rx or Tx interface,
    pub /: *mut *mut u8 enable_entry;/ Enable this MCAM entry ?,
    pub /: *mut *mut u8 hw_prio; / hardware priority, valid for cn20k,
    pub /: *mut *mut u8 req_kw_type; / Type of kw which should be written,
    pub /: *mut *mut u64 reserved; / reserved for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_write_entry_req {
    pub hdr: mbox_msghdr,
    pub entry_data: mcam_entry,
    pub /: *mut *mut u16 entry; / MCAM entry to write this match key,
    pub /: *mut *mut u16 cntr; / Counter for this MCAM entry,
    pub /: *mut *mut u8 intf; / Rx or Tx interface,
    pub /: *mut *mut u8 enable_entry;/ Enable this MCAM entry ?,
    pub /: *mut *mut u8 set_cntr; / Set counter for this entry ?,
    pub /: *mut *mut u8 hw_prio; / hardware priority, valid for cn20k,
    pub /: *mut *mut u64 reserved; / reserved for future use,
}

// Enable/Disable a given entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_ena_dis_entry_req {
    pub hdr: mbox_msghdr,
    pub entry: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_shift_entry_req {
    pub hdr: mbox_msghdr,
pub const NPC_MCAM_MAX_SHIFTS: c_int = 64;
    pub curr_entry: [u16; NPC_MCAM_MAX_SHIFTS],
    pub new_entry: [u16; NPC_MCAM_MAX_SHIFTS],
    pub /: *mut *mut u16 shift_count; / Number of entries to shift,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_shift_entry_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 failed_entry_idx; / Index in 'curr_entry', not entry itself,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_counter_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 contig; / Contiguous counters ?,
pub const NPC_MAX_NONCONTIG_COUNTERS: c_int = 64;
    pub /: *mut *mut u16 count; / Number of counters requested,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_counter_rsp {
    pub hdr: mbox_msghdr,
    pub contiguous.: *mut *mut u16 cntr; / Counter allocated or start index if,
// Invalid incase of non-contiguous.
//
    pub /: *mut *mut u16 count; / Number of counters allocated,
    pub cntr_list: [u16; NPC_MAX_NONCONTIG_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_oper_counter_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 cntr; / Free a counter or clear/fetch it's stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_oper_counter_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 stat; / valid only while fetching counter's stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_unmap_counter_req {
    pub hdr: mbox_msghdr,
    pub cntr: u16,
    pub /: *mut *mut u16 entry; / Entry and counter to be unmapped,
    pub /: *mut *mut u8 all; / Unmap all entries using this counter ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_and_write_entry_req {
    pub hdr: mbox_msghdr,
    pub entry_data: mcam_entry,
    pub ref_entry: u16,
    pub /: *mut *mut u8 ref_prio; / Lower or higher w.r.t ref_entry,
    pub /: *mut *mut u8 intf; / Rx or Tx interface,
    pub /: *mut *mut u8 enable_entry;/ Enable this MCAM entry ?,
    pub /: *mut *mut u8 alloc_cntr; / Allocate counter and map ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_mcam_alloc_and_write_entry_req {
    pub hdr: mbox_msghdr,
    pub entry_data: cn20k_mcam_entry,
    pub ref_entry: u16,
    pub /: *mut *mut u8 ref_prio; / Lower or higher w.r.t ref_entry,
    pub /: *mut *mut u8 intf; / Rx or Tx interface,
    pub /: *mut *mut u8 enable_entry;/ Enable this MCAM entry ?,
    pub /: *mut *mut u8 hw_prio; / hardware priority, valid for cn20k,
    pub /: *mut *mut u8 virt; / Allocate virtual index,
    pub /: *mut *mut u8 req_kw_type; / Key type to be written,
    pub /: *mut *mut u16 reserved[4]; / reserved for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_mcam_read_entry_rsp {
    pub hdr: mbox_msghdr,
    pub entry_data: cn20k_mcam_entry,
    pub intf: u8,
    pub enable: u8,
    pub /: *mut *mut u8 hw_prio; / valid for cn20k,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_mcam_read_base_rule_rsp {
    pub hdr: mbox_msghdr,
    pub entry: cn20k_mcam_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_alloc_and_write_entry_rsp {
    pub hdr: mbox_msghdr,
    pub entry: u16,
    pub cntr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_kex_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 rx_keyx_cfg; / NPC_AF_INTF(0)_KEX_CFG,
    pub /: *mut *mut u64 tx_keyx_cfg; / NPC_AF_INTF(1)_KEX_CFG,
pub const NPC_MAX_INTF: c_int = 2;
pub const NPC_MAX_LID: c_int = 8;
pub const NPC_MAX_LT: c_int = 16;
pub const NPC_MAX_LD: c_int = 2;
pub const NPC_MAX_LFL: c_int = 16;
// NPC_AF_KEX_LDATA(0..1)_FLAGS_CFG
    pub kex_ld_flags: [u64; NPC_MAX_LD],
// NPC_AF_INTF(0..1)_LID(0..7)_LT(0..15)_LD(0..1)_CFG
    pub intf_lid_lt_ld: [u64; NPC_MAX_INTF][NPC_MAX_LID][NPC_MAX_LT][NPC_MAX_LD],
// NPC_AF_INTF(0..1)_LDATA(0..1)_FLAGS(0..15)_CFG
    pub intf_ld_flags: [u64; NPC_MAX_INTF][NPC_MAX_LD][NPC_MAX_LFL],
pub const MKEX_NAME_LEN: c_int = 128;
    pub mkex_pfl_name: [u8; MKEX_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_get_kex_cfg_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 rx_keyx_cfg; / NPC_AF_INTF(0)_KEX_CFG,
    pub /: *mut *mut u64 tx_keyx_cfg; / NPC_AF_INTF(1)_KEX_CFG,
pub const NPC_MAX_EXTRACTOR: c_int = 24;
// MKEX Extractor data
    pub intf_extr_lid: [u64; NPC_MAX_INTF][NPC_MAX_EXTRACTOR],
// KEX configuration per extractor
    pub intf_extr_lt: [u64; NPC_MAX_INTF][NPC_MAX_EXTRACTOR][NPC_MAX_LT],
pub const MKEX_NAME_LEN: c_int = 128;
    pub mkex_pfl_name: [u8; MKEX_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_get_cap_rsp {
    pub hdr: mbox_msghdr,

    pub cap: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_rep_cnt_rsp {
    pub hdr: mbox_msghdr,
    pub rep_cnt: u16,
    pub rep_pf_map: [u16; 64],
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw_cfg_req {
    pub hdr: mbox_msghdr,
    pub ena: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rep_evt_data {
    pub port_state: u8,
    pub vf_state: u8,
    pub rx_mode: u16,
    pub rx_flags: u16,
    pub mtu: u16,
    pub mac: [u8; ETH_ALEN],
    pub rsvd: [u64; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rep_event {
    pub hdr: mbox_msghdr,
    pub pcifunc: u16,

    pub event: u16,
    pub evt_data: rep_evt_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_msg {
    pub dmac: [c_uchar; 6],
    pub smac: [c_uchar; 6],
    pub etype: __be16,
    pub vlan_etype: __be16,
    pub vlan_tci: __be16,
    pub ip4src: __be32,
    pub ip6src: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_install_flow_req {
    pub hdr: mbox_msghdr,
    pub packet: flow_msg,
    pub mask: flow_msg,
    pub features: u64,
    pub entry: u16,
    pub channel: u16,
    pub chan_mask: u16,
    pub intf: u8,
    pub /: *mut *mut u8 set_cntr; / If counter is available set counter for this entry ?,
    pub default_rule: u8,
    pub /: *mut *mut u8 append; / overwrite(0) or append(1) flow to default rule?,
    pub vf: u16,
// action
    pub index: u32,
    pub match_id: u16,
    pub flow_key_alg: u8,
    pub op: u8,
// vtag rx action
    pub vtag0_type: u8,
    pub vtag0_valid: u8,
    pub vtag1_type: u8,
    pub vtag1_valid: u8,
// vtag tx action
    pub vtag0_def: u16,
    pub vtag0_op: u8,
    pub vtag1_def: u16,
    pub vtag1_op: u8,
// old counter value
    pub cntr_val: u16,
    pub hw_prio: u8,
    pub /: *mut *mut u8 req_kw_type; / Key type to be written,
    pub /: *mut *mut u8 alloc_entry; / only for cn20k,
// For now use any priority, once AF driver is changed to
// allocate least priority entry instead of mid zone then make
// NPC_MCAM_LEAST_PRIO as 3
//

    pub ref_prio: u16,
    pub ref_entry: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_install_flow_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut int counter; / negative if no counter else counter number,
    pub entry: u16,
    pub kw_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_num_kws_req {
    pub hdr: mbox_msghdr,
    pub fl: npc_install_flow_req,
    pub rsvd: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_num_kws_rsp {
    pub hdr: mbox_msghdr,
    pub kws: c_int,
    pub rsvd: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_dft_rl_idxs_rsp {
    pub hdr: mbox_msghdr,
    pub bcast: u16,
    pub mcast: u16,
    pub promisc: u16,
    pub ucast: u16,
    pub vf_ucast: u16,
    pub rsvd: [u16; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_pfl_info_rsp {
    pub hdr: mbox_msghdr,
    pub x4_slots: u16,
    pub kw_type: u8,
    pub rsvd1: [u8; 3],
    pub rsvd2: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_delete_flow_req {
    pub hdr: mbox_msghdr,
    pub entry: u16,
    pub /: *mut *mut u16 start;/Disable range of entries,
    pub end: u16,
    pub /: *mut *mut u8 all; / PF + VFs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_delete_flow_rsp {
    pub hdr: mbox_msghdr,
    pub cntr_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_read_entry_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 entry; / MCAM entry to read,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_read_entry_rsp {
    pub hdr: mbox_msghdr,
    pub entry_data: mcam_entry,
    pub intf: u8,
    pub enable: u8,
    pub /: *mut *mut u8 hw_prio; / valid for cn20k,
}

// Available entries to use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_get_fcnt_rsp {
    pub hdr: mbox_msghdr,
    pub free_x2: c_int,
    pub free_x4: c_int,
    pub free_subbanks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_read_base_rule_rsp {
    pub hdr: mbox_msghdr,
    pub entry: mcam_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_get_stats_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u16 entry; / mcam entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_get_stats_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 stat; / counter stats,
    pub /: *mut *mut u8 stat_ena; / enabled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_field_hash_info_req {
    pub hdr: mbox_msghdr,
    pub intf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_field_hash_info_rsp {
    pub hdr: mbox_msghdr,
    pub secret_key: [u64; 3],
pub const NPC_MAX_HASH: c_int = 2;
pub const NPC_MAX_HASH_MASK: c_int = 2;
// NPC_AF_INTF(0..1)_HASH(0..1)_MASK(0..1)
    pub hash_mask: [u64; NPC_MAX_INTF][NPC_MAX_HASH][NPC_MAX_HASH_MASK],
// NPC_AF_INTF(0..1)_HASH(0..1)_RESULT_CTRL
    pub hash_ctrl: [u64; NPC_MAX_INTF][NPC_MAX_HASH],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_op {
    PTP_OP_ADJFINE = 0,
    PTP_OP_GET_CLOCK = 1,
    PTP_OP_GET_TSTMP = 2,
    PTP_OP_SET_THRESH = 3,
    PTP_OP_PPS_ON = 4,
    PTP_OP_ADJTIME = 5,
    PTP_OP_SET_CLOCK = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_req {
    pub hdr: mbox_msghdr,
    pub op: u8,
    pub scaled_ppm: i64,
    pub thresh: u64,
    pub period: u64,
    pub pps_on: c_int,
    pub delta: i64,
    pub clk: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_rsp {
    pub hdr: mbox_msghdr,
    pub clk: u64,
    pub tsc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_field_status_req {
    pub hdr: mbox_msghdr,
    pub intf: u8,
    pub field: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_get_field_status_rsp {
    pub hdr: mbox_msghdr,
    pub enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_vf_perm {
    pub hdr: mbox_msghdr,
    pub vf: u16,

    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lmtst_tbl_setup_req {
    pub hdr: mbox_msghdr,
    pub :1: u64 dis_sched_early_comp,
    pub :1: u64 sch_ena,
    pub :1: u64 dis_line_pref,
    pub :13: u64 ssow_pf_func,
    pub base_pcifunc: u16,
    pub use_local_lmt_region: u8,
    pub lmt_iova: u64,
    pub rsvd: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndc_sync_op {
    pub hdr: mbox_msghdr,
    pub nix_lf_tx_sync: u8,
    pub nix_lf_rx_sync: u8,
    pub npa_lf_sync: u8,
}

// CPT mailbox error codes
// Range 901 - 1000.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpt_af_status {
    CPT_AF_ERR_PARAM		= -901,
    CPT_AF_ERR_GRP_INVALID		= -902,
    CPT_AF_ERR_LF_INVALID		= -903,
    CPT_AF_ERR_ACCESS_DENIED	= -904,
    CPT_AF_ERR_SSO_PF_FUNC_INVALID	= -905,
    CPT_AF_ERR_NIX_PF_FUNC_INVALID	= -906,
    CPT_AF_ERR_INLINE_IPSEC_INB_ENA	= -907,
    CPT_AF_ERR_INLINE_IPSEC_OUT_ENA	= -908
}

// CPT mbox message formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_rd_wr_reg_msg {
    pub hdr: mbox_msghdr,
    pub reg_offset: u64,
    pub ret_val: *mut u64,
    pub val: u64,
    pub is_write: u8,
    pub blkaddr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_lf_alloc_req_msg {
    pub hdr: mbox_msghdr,
    pub nix_pf_func: u16,
    pub sso_pf_func: u16,
    pub eng_grpmsk: u16,
    pub blkaddr: u8,
    pub 1: u8 ctx_ilen_valid :,
    pub 7: u8 ctx_ilen :,
}

pub const CPT_INLINE_INBOUND: c_int = 0;
pub const CPT_INLINE_OUTBOUND: c_int = 1;
// Mailbox message request format for CPT IPsec
// inline inbound and outbound configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_inline_ipsec_cfg_msg {
    pub hdr: mbox_msghdr,
    pub enable: u8,
    pub slot: u8,
    pub dir: u8,
    pub sso_pf_func_ovrd: u8,
    pub /: *mut *mut u16 sso_pf_func; / inbound path SSO_PF_FUNC,
    pub /: *mut *mut u16 nix_pf_func; / outbound path NIX_PF_FUNC,
}

// Mailbox message request and response format for CPT stats.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_sts_req {
    pub hdr: mbox_msghdr,
    pub blkaddr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_sts_rsp {
    pub hdr: mbox_msghdr,
    pub inst_req_pc: u64,
    pub inst_lat_pc: u64,
    pub rd_req_pc: u64,
    pub rd_lat_pc: u64,
    pub rd_uc_pc: u64,
    pub active_cycles_pc: u64,
    pub ctx_mis_pc: u64,
    pub ctx_hit_pc: u64,
    pub ctx_aop_pc: u64,
    pub ctx_aop_lat_pc: u64,
    pub ctx_ifetch_pc: u64,
    pub ctx_ifetch_lat_pc: u64,
    pub ctx_ffetch_pc: u64,
    pub ctx_ffetch_lat_pc: u64,
    pub ctx_wback_pc: u64,
    pub ctx_wback_lat_pc: u64,
    pub ctx_psh_pc: u64,
    pub ctx_psh_lat_pc: u64,
    pub ctx_err: u64,
    pub ctx_enc_id: u64,
    pub ctx_flush_timer: u64,
    pub rxc_time: u64,
    pub rxc_time_cfg: u64,
    pub rxc_active_sts: u64,
    pub rxc_zombie_sts: u64,
    pub busy_sts_ae: u64,
    pub free_sts_ae: u64,
    pub busy_sts_se: u64,
    pub free_sts_se: u64,
    pub busy_sts_ie: u64,
    pub free_sts_ie: u64,
    pub exe_err_info: u64,
    pub cptclk_cnt: u64,
    pub diag: u64,
    pub rxc_dfrg: u64,
    pub x2p_link_cfg0: u64,
    pub x2p_link_cfg1: u64,
}

// Mailbox message request format to configure reassembly timeout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_rxc_time_cfg_req {
    pub hdr: mbox_msghdr,
    pub blkaddr: c_int,
    pub step: u32,
    pub zombie_thres: u16,
    pub zombie_limit: u16,
    pub active_thres: u16,
    pub active_limit: u16,
}

// Mailbox message request format to request for CPT_INST_S lmtst.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_inst_lmtst_req {
    pub hdr: mbox_msghdr,
    pub inst: [u64; 8],
    pub rsvd: u64,
}

// Mailbox message format to request for CPT LF reset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_lf_rst_req {
    pub hdr: mbox_msghdr,
    pub slot: u32,
    pub rsvd: u32,
}

// Mailbox message format to request for CPT faulted engines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_flt_eng_info_req {
    pub hdr: mbox_msghdr,
    pub blkaddr: c_int,
    pub reset: bool,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_flt_eng_info_rsp {
    pub hdr: mbox_msghdr,
pub const CPT_AF_MAX_FLT_INT_VECS: c_int = 3;
    pub flt_eng_map: [u64; CPT_AF_MAX_FLT_INT_VECS],
    pub rcvrd_eng_map: [u64; CPT_AF_MAX_FLT_INT_VECS],
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_node_info {
// Node to which this PF belons to
    pub node_id: u8,
    pub max_vfs: u8,
    pub num_pf_rings: u8,
    pub pf_srn: u8,
pub const SDP_MAX_VFS: c_int = 128;
    pub vf_rings: [u8; SDP_MAX_VFS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_chan_info_msg {
    pub hdr: mbox_msghdr,
    pub info: sdp_node_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_get_chan_info_msg {
    pub hdr: mbox_msghdr,
    pub chan_base: u16,
    pub num_chan: u16,
}

// CGX mailbox error codes
// Range 1101 - 1200.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_af_status {
    LMAC_AF_ERR_INVALID_PARAM	= -1101,
    LMAC_AF_ERR_PF_NOT_MAPPED	= -1102,
    LMAC_AF_ERR_PERM_DENIED		= -1103,
    LMAC_AF_ERR_PFC_ENADIS_PERM_DENIED       = -1104,
    LMAC_AF_ERR_8023PAUSE_ENADIS_PERM_DENIED = -1105,
    LMAC_AF_ERR_CMD_TIMEOUT = -1106,
    LMAC_AF_ERR_FIRMWARE_DATA_NOT_MAPPED = -1107,
    LMAC_AF_ERR_EXACT_MATCH_TBL_ADD_FAILED = -1108,
    LMAC_AF_ERR_EXACT_MATCH_TBL_DEL_FAILED = -1109,
    LMAC_AF_ERR_EXACT_MATCH_TBL_LOOK_UP_FAILED = -1110,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcs_direction {
    MCS_RX,
    MCS_TX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcs_rsrc_type {
    MCS_RSRC_TYPE_FLOWID,
    MCS_RSRC_TYPE_SECY,
    MCS_RSRC_TYPE_SC,
    MCS_RSRC_TYPE_SA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_alloc_rsrc_req {
    pub hdr: mbox_msghdr,
    pub rsrc_type: u8,
    pub /: *mut *mut u8 rsrc_cnt; / Resources count,
    pub /: *mut *mut u8 mcs_id; / MCS block ID,
    pub /: *mut *mut u8 dir; / Macsec ingress or egress side,
    pub /: *mut *mut u8 all; / Allocate all resource type one each,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_alloc_rsrc_rsp {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 flow_ids[128]; / Index of reserved entries,
    pub secy_ids: [u8; 128],
    pub sc_ids: [u8; 128],
    pub sa_ids: [u8; 256],
    pub rsrc_type: u8,
    pub /: *mut *mut u8 rsrc_cnt; / No of entries reserved,
    pub mcs_id: u8,
    pub dir: u8,
    pub all: u8,
    pub /: *mut *mut u8 rsvd[256]; / reserved fields for future expansion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_free_rsrc_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 rsrc_id; / Index of the entry to be freed,
    pub rsrc_type: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub /: *mut *mut u8 all; / Free all the cam resources,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_flowid_entry_write_req {
    pub hdr: mbox_msghdr,
    pub data: [u64; 4],
    pub mask: [u64; 4],
    pub /: *mut *mut u64 sci; / CNF10K-B for tx_secy_mem_map,
    pub flow_id: u8,
    pub /: *mut *mut u8 secy_id; / secyid for which flowid is mapped,
    pub /: *mut *mut u8 sc_id; / Valid if dir = MCS_TX, SC_CAM id mapped to flowid,
    pub /: *mut *mut u8 ena; / Enable tcam entry,
    pub ctrl_pkt: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_secy_plcy_write_req {
    pub hdr: mbox_msghdr,
    pub plcy: u64,
    pub secy_id: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

// RX SC_CAM mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_rx_sc_cam_write_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 sci; / SCI,
    pub /: *mut *mut u64 secy_id; / secy index mapped to SC,
    pub /: *mut *mut u8 sc_id; / SC CAM entry index,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_sa_plcy_write_req {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u64 plcy[2][9]; / Support 2 SA policy,
    pub sa_index: [u8; 2],
    pub sa_cnt: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_tx_sc_sa_map {
    pub hdr: mbox_msghdr,
    pub sa_index0: u8,
    pub sa_index1: u8,
    pub rekey_ena: u8,
    pub sa_index0_vld: u8,
    pub sa_index1_vld: u8,
    pub tx_sa_active: u8,
    pub sectag_sci: u64,
    pub /: *mut *mut u8 sc_id; / used as index for SA_MEM_MAP,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_rx_sc_sa_map {
    pub hdr: mbox_msghdr,
    pub sa_index: u8,
    pub sa_in_use: u8,
    pub sc_id: u8,
    pub /: *mut *mut u8 an; / value range 0-3, sc_id + an used as index SA_MEM_MAP,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_flowid_ena_dis_entry {
    pub hdr: mbox_msghdr,
    pub flow_id: u8,
    pub ena: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_pn_table_write_req {
    pub hdr: mbox_msghdr,
    pub next_pn: u64,
    pub pn_id: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_hw_info {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 num_mcs_blks; / Number of MCS blocks,
    pub /: *mut *mut u8 tcam_entries; / RX/TX Tcam entries per mcs block,
    pub /: *mut *mut u8 secy_entries; / RX/TX SECY entries per mcs block,
    pub /: *mut *mut u8 sc_entries; / RX/TX SC CAM entries per mcs block,
    pub /: *mut *mut u16 sa_entries; / PN table entries = SA entries,
    pub rsvd: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_set_active_lmac {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u32 lmac_bmap; / bitmap of active lmac per mcs block,
    pub mcs_id: u8,
    pub /: *mut *mut u16 chan_base; / MCS channel base,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_set_lmac_mode {
    pub hdr: mbox_msghdr,
    pub /: *mut *mut u8 mode; / 1:Bypass 0:Operational,
    pub lmac_id: u8,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_port_reset_req {
    pub hdr: mbox_msghdr,
    pub reset: u8,
    pub mcs_id: u8,
    pub port_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_port_cfg_set_req {
    pub hdr: mbox_msghdr,
    pub cstm_tag_rel_mode_sel: u8,
    pub custom_hdr_enb: u8,
    pub fifo_skid: u8,
    pub port_mode: u8,
    pub port_id: u8,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_port_cfg_get_req {
    pub hdr: mbox_msghdr,
    pub port_id: u8,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_port_cfg_get_rsp {
    pub hdr: mbox_msghdr,
    pub cstm_tag_rel_mode_sel: u8,
    pub custom_hdr_enb: u8,
    pub fifo_skid: u8,
    pub port_mode: u8,
    pub port_id: u8,
    pub mcs_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_custom_tag_cfg_get_req {
    pub hdr: mbox_msghdr,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_custom_tag_cfg_get_rsp {
    pub hdr: mbox_msghdr,
    pub cstm_etype: [u16; 8],
    pub cstm_indx: [u8; 8],
    pub cstm_etype_en: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

// MCS mailbox error codes
// Range 1201 - 1300.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcs_af_status {
    MCS_AF_ERR_INVALID_MCSID        = -1201,
    MCS_AF_ERR_NOT_MAPPED           = -1202,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_set_pn_threshold {
    pub hdr: mbox_msghdr,
    pub threshold: u64,
    pub /: *mut *mut u8 xpn; / '1' for setting xpn threshold,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcs_ctrl_pkt_rulew_type {
    MCS_CTRL_PKT_RULE_TYPE_ETH,
    MCS_CTRL_PKT_RULE_TYPE_DA,
    MCS_CTRL_PKT_RULE_TYPE_RANGE,
    MCS_CTRL_PKT_RULE_TYPE_COMBO,
    MCS_CTRL_PKT_RULE_TYPE_MAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_alloc_ctrl_pkt_rule_req {
    pub hdr: mbox_msghdr,
    pub rule_type: u8,
    pub /: *mut *mut u8 mcs_id; / MCS block ID,
    pub /: *mut *mut u8 dir; / Macsec ingress or egress side,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_alloc_ctrl_pkt_rule_rsp {
    pub hdr: mbox_msghdr,
    pub rule_idx: u8,
    pub rule_type: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_free_ctrl_pkt_rule_req {
    pub hdr: mbox_msghdr,
    pub rule_idx: u8,
    pub rule_type: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub all: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_ctrl_pkt_rule_write_req {
    pub hdr: mbox_msghdr,
    pub data0: u64,
    pub data1: u64,
    pub data2: u64,
    pub rule_idx: u8,
    pub rule_type: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_stats_req {
    pub hdr: mbox_msghdr,
    pub id: u8,
    pub mcs_id: u8,
    pub dir: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_flowid_stats {
    pub hdr: mbox_msghdr,
    pub tcam_hit_cnt: u64,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_secy_stats {
    pub hdr: mbox_msghdr,
    pub ctl_pkt_bcast_cnt: u64,
    pub ctl_pkt_mcast_cnt: u64,
    pub ctl_pkt_ucast_cnt: u64,
    pub ctl_octet_cnt: u64,
    pub unctl_pkt_bcast_cnt: u64,
    pub unctl_pkt_mcast_cnt: u64,
    pub unctl_pkt_ucast_cnt: u64,
    pub unctl_octet_cnt: u64,
// Valid only for RX
    pub octet_decrypted_cnt: u64,
    pub octet_validated_cnt: u64,
    pub pkt_port_disabled_cnt: u64,
    pub pkt_badtag_cnt: u64,
    pub pkt_nosa_cnt: u64,
    pub pkt_nosaerror_cnt: u64,
    pub pkt_tagged_ctl_cnt: u64,
    pub pkt_untaged_cnt: u64,
    pub /: *mut *mut u64 pkt_ctl_cnt; / CN10K-B,
    pub /: *mut *mut u64 pkt_notag_cnt; / CNF10K-B,
// Valid only for TX
    pub octet_encrypted_cnt: u64,
    pub octet_protected_cnt: u64,
    pub pkt_noactivesa_cnt: u64,
    pub pkt_toolong_cnt: u64,
    pub pkt_untagged_cnt: u64,
    pub rsvd: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_port_stats {
    pub hdr: mbox_msghdr,
    pub tcam_miss_cnt: u64,
    pub parser_err_cnt: u64,
    pub /: *mut *mut u64 preempt_err_cnt; / CNF10K-B,
    pub sectag_insert_err_cnt: u64,
    pub rsvd: [u64; 4],
}

// Only for CN10K-B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_sa_stats {
    pub hdr: mbox_msghdr,
// RX
    pub pkt_invalid_cnt: u64,
    pub pkt_nosaerror_cnt: u64,
    pub pkt_notvalid_cnt: u64,
    pub pkt_ok_cnt: u64,
    pub pkt_nosa_cnt: u64,
// TX
    pub pkt_encrypt_cnt: u64,
    pub pkt_protected_cnt: u64,
    pub rsvd: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_sc_stats {
    pub hdr: mbox_msghdr,
// RX
    pub hit_cnt: u64,
    pub pkt_invalid_cnt: u64,
    pub pkt_late_cnt: u64,
    pub pkt_notvalid_cnt: u64,
    pub pkt_unchecked_cnt: u64,
    pub /: *mut *mut u64 pkt_delay_cnt; / CNF10K-B,
    pub /: *mut *mut u64 pkt_ok_cnt; / CNF10K-B,
    pub /: *mut *mut u64 octet_decrypt_cnt; / CN10K-B,
    pub /: *mut *mut u64 octet_validate_cnt; / CN10K-B,
// TX
    pub pkt_encrypt_cnt: u64,
    pub pkt_protected_cnt: u64,
    pub /: *mut *mut u64 octet_encrypt_cnt; / CN10K-B,
    pub /: *mut *mut u64 octet_protected_cnt; / CN10K-B,
    pub rsvd: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_clear_stats {
    pub hdr: mbox_msghdr,
pub const MCS_FLOWID_STATS: c_int = 0;
pub const MCS_SECY_STATS: c_int = 1;
pub const MCS_SC_STATS: c_int = 2;
pub const MCS_SA_STATS: c_int = 3;
pub const MCS_PORT_STATS: c_int = 4;
    pub /: *mut *mut u8 type; / FLOWID, SECY, SC, SA, PORT,
    pub /: *mut *mut u8 id; / type = PORT, If id = FF(invalid) port no is derived from pcifunc,
    pub mcs_id: u8,
    pub dir: u8,
    pub /: *mut *mut u8 all; / All resources stats mapped to PF are cleared,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_intr_cfg {
    pub hdr: mbox_msghdr,

    pub /: *mut *mut u64 intr_mask; / Interrupt enable mask,
    pub mcs_id: u8,
    pub lmac_id: u8,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_intr_info {
    pub hdr: mbox_msghdr,
    pub intr_mask: u64,
    pub sa_id: c_int,
    pub mcs_id: u8,
    pub lmac_id: u8,
    pub rsvd: u64,
}
