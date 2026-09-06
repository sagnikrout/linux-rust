//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_common.h
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
// Marvell RVU Ethernet driver
//
// Copyright (C) 2020 Marvell.
//

// IPv4 flag more fragment bit
pub const IPV4_FLAG_MORE: c_uint = 0x20;
// PCI device IDs
pub const PCI_DEVID_OCTEONTX2_RVU_PF: c_uint = 0xA063;
pub const PCI_DEVID_OCTEONTX2_RVU_VF: c_uint = 0xA064;
pub const PCI_DEVID_OCTEONTX2_RVU_AFVF: c_uint = 0xA0F8;
pub const PCI_SUBSYS_DEVID_96XX_RVU_PFVF: c_uint = 0xB200;
pub const PCI_SUBSYS_DEVID_CN10K_A_RVU_PFVF: c_uint = 0xB900;
pub const PCI_SUBSYS_DEVID_CN10K_B_RVU_PFVF: c_uint = 0xBD00;
pub const PCI_DEVID_OCTEONTX2_SDP_REP: c_uint = 0xA0F7;
// PCI BAR nos
pub const PCI_CFG_REG_BAR_NUM: c_int = 2;
pub const PCI_MBOX_BAR_NUM: c_int = 4;
pub const NAME_SIZE: c_int = 32;

// Max priority supported for PFC
pub const NIX_PF_PFC_PRIO_MAX: c_int = 8;

// Number of segments per SG structure
pub const MAX_SEGS_PER_SG: c_int = 3;
extern "C" {
    pub fn otx2_pfaf_mbox_intr_handler(irq: c_int, pf_irq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cn20k_pfaf_mbox_intr_handler(irq: c_int, pf_irq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cn20k_vfaf_mbox_intr_handler(irq: c_int, vf_irq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cn20k_pfvf_mbox_intr_handler(irq: c_int, pf_irq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn otx2_pfvf_mbox_intr_handler(irq: c_int, pf_irq: *mut c_void) -> irqreturn_t;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arua_mapped_qtypes {
    AURA_NIX_RQ,
    AURA_NIX_SQ,
}

// NIX LF interrupts range
pub const NIX_LF_QINT_VEC_START: c_uint = 0x00;
pub const NIX_LF_CINT_VEC_START: c_uint = 0x40;
pub const NIX_LF_GINT_VEC: c_uint = 0x80;
pub const NIX_LF_ERR_VEC: c_uint = 0x81;
pub const NIX_LF_POISON_VEC: c_uint = 0x82;
// Send skid of 2000 packets required for CQ size of 4K CQEs.
pub const SEND_CQ_SKID: c_int = 2000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_lmt_info {
    pub lmt_addr: u64,
    pub lmt_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_rss_info {
    pub enable: u8,
    pub flowkey_cfg: u32,
    pub rss_size: u16,
    pub key: [u8; RSS_HASH_KEY_SIZE],
    pub ind_tbl: [u32; MAX_RSS_INDIR_TBL_SIZE],
}

// NIX (or NPC) RX errors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_errlvl {
    NPC_ERRLVL_RE,
    NPC_ERRLVL_LID_LA,
    NPC_ERRLVL_LID_LB,
    NPC_ERRLVL_LID_LC,
    NPC_ERRLVL_LID_LD,
    NPC_ERRLVL_LID_LE,
    NPC_ERRLVL_LID_LF,
    NPC_ERRLVL_LID_LG,
    NPC_ERRLVL_LID_LH,
    NPC_ERRLVL_NIX = 0x0F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_errcodes_re {
// NPC_ERRLVL_RE errcodes
    ERRCODE_FCS = 0x7,
    ERRCODE_FCS_RCV = 0x8,
    ERRCODE_UNDERSIZE = 0x10,
    ERRCODE_OVERSIZE = 0x11,
    ERRCODE_OL2_LEN_MISMATCH = 0x12,
// NPC_ERRLVL_NIX errcodes
    ERRCODE_OL3_LEN = 0x10,
    ERRCODE_OL4_LEN = 0x11,
    ERRCODE_OL4_CSUM = 0x12,
    ERRCODE_IL3_LEN = 0x20,
    ERRCODE_IL4_LEN = 0x21,
    ERRCODE_IL4_CSUM = 0x22,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_xdp_action {
    OTX2_XDP_TX	  = BIT(0),
    OTX2_XDP_REDIRECT = BIT(1),
    OTX2_AF_XDP_FRAME = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_dev_stats {
    pub rx_bytes: u64,
    pub rx_frames: u64,
    pub rx_ucast_frames: u64,
    pub rx_bcast_frames: u64,
    pub rx_mcast_frames: u64,
    pub rx_drops: u64,
    pub tx_bytes: u64,
    pub tx_frames: u64,
    pub tx_ucast_frames: u64,
    pub tx_bcast_frames: u64,
    pub tx_mcast_frames: u64,
    pub tx_drops: u64,
    pub tx_discards: atomic_long_t,
}

// Driver counted stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_drv_stats {
    pub rx_fcs_errs: core::sync::atomic::AtomicI32,
    pub rx_oversize_errs: core::sync::atomic::AtomicI32,
    pub rx_undersize_errs: core::sync::atomic::AtomicI32,
    pub rx_csum_errs: core::sync::atomic::AtomicI32,
    pub rx_len_errs: core::sync::atomic::AtomicI32,
    pub rx_other_errs: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox {
    pub mbox: otx2_mbox,
    pub mbox_wrk: work_struct,
    pub mbox_up: otx2_mbox,
    pub mbox_up_wrk: work_struct,
    pub pfvf: *mut otx2_nic,
    pub /: *mut *mut *mut void bbuf_base; / Bounce buffer for mbox memory,
    pub /: *mut *mut mutex lock; / serialize mailbox access,
    pub /: *mut *mut int num_msgs; / mbox number of messages,
    pub /: *mut *mut int up_num_msgs; / mbox_up number of messages,
}

// Egress rate limiting definitions
pub const MAX_BURST_EXPONENT: c_uint = 0x0FULL;
pub const MAX_BURST_MANTISSA: c_uint = 0xFFULL;

pub const MAX_RATE_EXPONENT: c_uint = 0x0FULL;
pub const MAX_RATE_MANTISSA: c_uint = 0xFFULL;
// Bitfields in NIX_TLX_PIR register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_hw {
    pub pdev: *mut pci_dev,
    pub rss_info: otx2_rss_info,
    pub rx_queues: u16,
    pub tx_queues: u16,
    pub xdp_queues: u16,
    pub tc_tx_queues: u16,
    pub /: *mut *mut u16 non_qos_queues; / tx queues plus xdp queues,
    pub max_queues: u16,
    pub pool_cnt: u16,
    pub rqpool_cnt: u16,
    pub sqpool_cnt: u16,
pub const OTX2_DEFAULT_RBUF_LEN: c_int = 2048;
    pub rbuf_len: u16,
    pub xqe_size: u32,
// NPA
    pub /: *mut *mut u32 stack_pg_ptrs; / No of ptrs per stack page,
    pub /: *mut *mut u32 stack_pg_bytes; / Size of stack page,
    pub sqb_size: u16,
// NIX
    pub txschq_link_cfg_lvl: u8,
    pub txschq_cnt: [u8; NIX_TXSCH_LVL_CNT],
    pub txschq_aggr_lvl_rr_prio: u8,
    pub txschq_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub matchall_ipolicer: u16,
    pub dwrr_mtu: u32,
    pub max_mtu: u32,
    pub smq_link_type: u8,
// HW settings, coalescing etc
    pub rx_chan_base: u16,
    pub tx_chan_base: u16,
    pub rx_chan_cnt: u8,
    pub tx_chan_cnt: u8,
    pub cq_qcount_wait: u16,
    pub cq_ecount_wait: u16,
    pub rq_skid: u16,
    pub cq_time_wait: u8,
// Segmentation
    pub lso_tsov4_idx: u8,
    pub lso_tsov6_idx: u8,
    pub lso_udpv4_idx: u8,
    pub lso_udpv6_idx: u8,
// RSS
    pub flowkey_alg_idx: u8,
// MSI-X
    pub /: *mut *mut u8 cint_cnt; / CQ interrupt count,
    pub /: *mut *mut u16 npa_msixoff; / Offset of NPA vectors,
    pub /: *mut *mut u16 nix_msixoff; / Offset of NIX vectors,
    pub irq_name: *mut c_char,
    pub affinity_mask: *mut cpumask_var_t,
    pub pfvf_irq_devid: [*mut pf_irq_data; 4],
// Stats
    pub dev_stats: otx2_dev_stats,
    pub drv_stats: otx2_drv_stats,
    pub cgx_rx_stats: [u64; CGX_RX_STATS_COUNT],
    pub cgx_tx_stats: [u64; CGX_TX_STATS_COUNT],
    pub cgx_fec_corr_blks: u64,
    pub cgx_fec_uncorr_blks: u64,
    pub /: *mut *mut u8 cgx_links; / No. of CGX links present in HW,
    pub /: *mut *mut u8 lbk_links; / No. of LBK links present in HW,
    pub /: *mut *mut u8 tx_link; / Transmit channel link number,
pub const HW_TSO: c_int = 0;
pub const CN10K_MBOX: c_int = 1;
pub const CN10K_LMTST: c_int = 2;
pub const CN10K_RPM: c_int = 3;
pub const CN10K_PTP_ONESTEP: c_int = 4;
pub const CN10K_HW_MACSEC: c_int = 5;
pub const QOS_CIR_PIR_SUPPORT: c_int = 6;
    pub cap_flag: c_ulong,
pub const LMT_LINE_SIZE: c_int = 128;

    pub lmt_base: *mut u64,
    pub lmt_info: *mut otx2_lmt_info __percpu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfperm {
    OTX2_RESET_VF_PERM,
    OTX2_TRUSTED_VF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_vf_config {
    pub pf: *mut otx2_nic,
    pub link_event_work: delayed_work,
    pub /: *mut *mut bool intf_down; / interface was either configured or not,
    pub mac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub tx_vtag_idx: c_int,
    pub trusted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flr_work {
    pub work: work_struct,
    pub pf: *mut otx2_nic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refill_work {
    pub pool_refill_work: delayed_work,
    pub pf: *mut otx2_nic,
    pub napi: *mut napi_struct,
}

// PTPv2 originTimestamp structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptpv2_tstamp {
    pub /: *mut *mut __be16 seconds_msb; / 16 bits +,
    pub bits*/: *mut *mut __be32 seconds_lsb; / 32 bits = 48,
    pub nanoseconds: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_ptp {
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub nic: *mut otx2_nic,
    pub cycle_counter: cyclecounter,
    pub time_counter: timecounter,
    pub extts_work: delayed_work,
    pub last_extts: u64,
    pub thresh: u64,
    pub extts_config: ptp_pin_desc,
    pub timestamp): *mut *mut u64 (convert_rx_ptp_tstmp)(u64,
    pub timestamp): *mut *mut u64 (convert_tx_ptp_tstmp)(u64,
    pub timestamp): *const *const *const u64 (ptp_tstamp2nsec)(struct timecounter time_counter, u64,
    pub synctstamp_work: delayed_work,
    pub tstamp: u64,
    pub base_ns: u32,
}

pub const OTX2_HW_TIMESTAMP_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_mac_table {
    pub addr: [u8; ETH_ALEN],
    pub mcam_entry: u16,
    pub inuse: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_flow_config {
    pub flow_ent: *mut u16,
    pub def_ent: *mut u16,
    pub nr_flows: u16,
pub const OTX2_DEFAULT_FLOWCOUNT: c_int = 16;
pub const OTX2_DEFAULT_UNICAST_FLOWS: c_int = 4;
pub const OTX2_MAX_VLAN_FLOWS: c_int = 1;

    pub unicast_offset: u16,
    pub rx_vlan_offset: u16,
    pub vf_vlan_offset: u16,

pub const OTX2_VF_VLAN_RX_INDEX: c_int = 0;
pub const OTX2_VF_VLAN_TX_INDEX: c_int = 1;
    pub bmap_to_dmacindex: *mut u32,
    pub dmacflt_bmap: *mut c_ulong,
    pub flow_list: list_head,
    pub dmacflt_max_flows: u32,
    pub max_flows: u16,
    pub mark_flows: refcount_t,
    pub flow_list_tc: list_head,
    pub ucast_flt_cnt: u8,
    pub ntuple: bool,
    pub ntuple_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_tc_flow_stats {
    pub bytes: u64,
    pub pkts: u64,
    pub used: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_tc_flow {
    pub list: list_head,
    pub cookie: c_ulong,
    pub rcu: rcu_head,
    pub stats: otx2_tc_flow_stats,
    pub /: *mut *mut spinlock_t lock; / lock for stats,
    pub rq: u16,
    pub entry: u16,
    pub leaf_profile: u16,
    pub is_act_police: bool,
    pub prio: u32,
    pub req: npc_install_flow_req,
    pub rate: u64,
    pub burst: u32,
    pub mcast_grp_idx: u32,
    pub is_pps: bool,
    pub /: *mut *mut u8 kw_type; / X2/X4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_hw_ops {
    pub sqb_aura): u16,
    pub qidx): int size, int,
    pub cq): *mut *mut *mut int (refill_pool_ptrs)(void dev, struct otx2_cq_queue,
    pub buf): *mut *mut *mut void (aura_freeptr)(void dev, int aura, u64,
    pub pf_irq): *mut *mut irqreturn_t (pfaf_mbox_intr_handler)(int irq, void,
    pub pf_irq): *mut *mut irqreturn_t (vfaf_mbox_intr_handler)(int irq, void,
    pub pf_irq): *mut *mut irqreturn_t (pfvf_mbox_intr_handler)(int irq, void,
    pub numptrs): int pool_id, int,
    pub type): c_int,
}

pub const CN10K_MCS_SA_PER_SC: c_int = 4;
// Stats which need to be accumulated in software because
// of shared counters in hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_txsc_stats {
    pub InPktsUntagged: u64,
    pub InPktsNoTag: u64,
    pub InPktsBadTag: u64,
    pub InPktsUnknownSCI: u64,
    pub InPktsNoSCI: u64,
    pub InPktsOverrun: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_rxsc_stats {
    pub InOctetsValidated: u64,
    pub InOctetsDecrypted: u64,
    pub InPktsUnchecked: u64,
    pub InPktsDelayed: u64,
    pub InPktsOK: u64,
    pub InPktsInvalid: u64,
    pub InPktsLate: u64,
    pub InPktsNotValid: u64,
    pub InPktsNotUsingSA: u64,
    pub InPktsUnusedSA: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_mcs_txsc {
    pub sw_secy: *mut macsec_secy,
    pub stats: cn10k_txsc_stats,
    pub entry: list_head,
    pub last_validate_frames: macsec_validation_type,
    pub last_replay_protect: bool,
    pub hw_secy_id_tx: u16,
    pub hw_secy_id_rx: u16,
    pub hw_flow_id: u16,
    pub hw_sc_id: u16,
    pub hw_sa_id: [u16; CN10K_MCS_SA_PER_SC],
    pub sa_bmap: u8,
    pub sa_key: [u8; CN10K_MCS_SA_PER_SC][MACSEC_MAX_KEY_LEN],
    pub encoding_sa: u8,
    pub salt: [u8; CN10K_MCS_SA_PER_SC][MACSEC_SALT_LEN],
    pub ssci: [ssci_t; CN10K_MCS_SA_PER_SC],
    pub /: *mut *mut bool vlan_dev; / macsec running on VLAN ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_mcs_rxsc {
    pub sw_secy: *mut macsec_secy,
    pub sw_rxsc: *mut macsec_rx_sc,
    pub stats: cn10k_rxsc_stats,
    pub entry: list_head,
    pub hw_flow_id: u16,
    pub hw_sc_id: u16,
    pub hw_sa_id: [u16; CN10K_MCS_SA_PER_SC],
    pub sa_bmap: u8,
    pub sa_key: [u8; CN10K_MCS_SA_PER_SC][MACSEC_MAX_KEY_LEN],
    pub salt: [u8; CN10K_MCS_SA_PER_SC][MACSEC_SALT_LEN],
    pub ssci: [ssci_t; CN10K_MCS_SA_PER_SC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_mcs_cfg {
    pub txsc_list: list_head,
    pub rxsc_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_irq_data {
    pub intr_status: u64,
    pub intr): int first, int mdevs, u64,
    pub pf: *mut otx2_nic,
    pub vec_num: c_int,
    pub start: c_int,
    pub mdevs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_nic {
    pub reg_base: *mut void __iomem,
    pub netdev: *mut net_device,
    pub hw_ops: *mut dev_hw_ops,
    pub iommu_domain: *mut c_void,
    pub tx_max_pktlen: u16,
    pub /: *mut *mut u16 rbsize; / Receive buffer size,

    pub flags: u64,
    pub cq_op_addr: *mut u64,
    pub xdp_prog: *mut bpf_prog,
    pub qset: otx2_qset,
    pub hw: otx2_hw,
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
// Mbox
    pub mbox: mbox,
    pub mbox_pfvf: *mut mbox,
    pub mbox_wq: *mut workqueue_struct,
    pub mbox_pfvf_wq: *mut workqueue_struct,
    pub pfvf_mbox_addr: *mut qmem,
    pub total_vfs: u8,
    pub /: *mut *mut u16 pcifunc; / RVU PF_FUNC,
    pub bpid: [u16; NIX_MAX_BPID_CHAN],
    pub vf_configs: *mut otx2_vf_config,
    pub linfo: cgx_link_user_info,
// NPC MCAM
    pub flow_cfg: *mut otx2_flow_config,
    pub mac_table: *mut otx2_mac_table,
    pub reset_count: u64,
    pub reset_task: work_struct,
    pub flr_wq: *mut workqueue_struct,
    pub flr_wrk: *mut flr_work,
    pub refill_wrk: *mut refill_work,
    pub otx2_wq: *mut workqueue_struct,
    pub rx_mode_work: work_struct,
// Ethtool stuff
    pub msg_enable: u32,
// Block address of NIX either BLKADDR_NIX0 or BLKADDR_NIX1
    pub nix_blkaddr: c_int,
// LMTST Lines info
    pub dync_lmt: *mut qmem,
    pub tot_lmt_lines: u16,
    pub npa_lmt_lines: u16,
    pub nix_lmt_size: u32,
    pub ptp: *mut otx2_ptp,
    pub tstamp: kernel_hwtstamp_config,
    pub rq_bmap: c_ulong,
// Devlink
    pub dl: *mut otx2_devlink,
// PFC
    pub pfc_en: u8,

    pub queue_to_pfc_map: *mut u8,
    pub pfc_schq_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub pfc_alloc_status: [bool; NIX_PF_PFC_PRIO_MAX],
// qos
    pub qos: otx2_qos,
// napi event count. It is needed for adaptive irq coalescing.
    pub napi_events: u32,

    pub macsec_cfg: *mut cn10k_mcs_cfg,

    pub reps: *mut rep_dev,
    pub rep_cnt: c_int,
    pub rep_pf_map: [u16; RVU_MAX_REP],
    pub esw_mode: u16,

// Inline ipsec
    pub ipsec: cn10k_ipsec,
// af_xdp zero-copy
    pub af_xdp_zc_qidx: *mut c_ulong,
}

// REVID for PCIe devices.
// Bits 0..1: minor pass, bit 3..2: major pass
// bits 7..4: midr id
//
pub const PCI_REVISION_ID_96XX: c_uint = 0x00;
pub const PCI_REVISION_ID_95XX: c_uint = 0x10;
pub const PCI_REVISION_ID_95XXN: c_uint = 0x20;
pub const PCI_REVISION_ID_98XX: c_uint = 0x30;
pub const PCI_REVISION_ID_95XXMM: c_uint = 0x40;
pub const PCI_REVISION_ID_95XXO: c_uint = 0xE0;
// Time based irq coalescing is not supported
// Due to HW issue previous silicons required minimum
// 600 unused CQE to avoid CQ overflow.
//
// Register read/write APIs
extern "C" {
    pub fn readq(_arg: addr) -> return;
}
// Mbox bounce buffer APIs
// Overwrite mbox mbase to point to bounce buffer, so that PF/VF
// prepare all mbox messages in bounce buffer instead of directly
// in hw mbox memory.
//
// Copy mbox messages from mbox memory to bounce buffer
// With the absence of API for 128-bit IO memory access for arm64,
// implement required operations at place.
//

// LMTID is same as AURA Id
// Meaning of count_eot
// CN10K: count_eot = 0 if the number of pointers to free is even,
// count_eot = 1 if the number of pointers to free is odd.
//
// CN20K: count_eot represents the least significant 2 bits of the
// total number of valid pointers to free.
// Example: if 7 pointers are freed (0b111), count_eot = 0b11.
//
// Set AURA ID to free pointer
// Target address for LMTST flush tells HW how many 128bit
// words are valid from NPA_LF_AURA_BATCH_FREE0.
//
// tar_addr[6:4] is LMTST size-1 in units of 128b.
//
// Perform LMTST flush
// Free only one buffer at time during init and teardown
// Alloc pointer from pool/aura
extern "C" {
    pub fn otx2_atomic64_add(_arg: incr, _arg: ptr) -> return;
}
// Free pointer to a pool/aura
// AURA_NIX_RQ
// Mbox APIs
extern "C" {
    pub fn otx2_mbox_check_rsp_msgs(_arg: &mbox->mbox, _arg: 0) -> return;
}
extern "C" {
    pub fn otx2_mbox_check_rsp_msgs(_arg: &mbox->mbox_up, _arg: devid) -> return;
}
// Use this API to send mbox msgs in atomic context
// where sleeping is not allowed
//
extern "C" {
    pub fn otx2_mbox_check_rsp_msgs(_arg: &mbox->mbox, _arg: 0) -> return;
}

// otx2_mbox_alloc_msg_ ## _fn_name(struct mbox *mbox)                    \

// Time to wait before watchdog kicks off

// check if qidx falls under QOS queues
// Convert bytes per second to Mbps
// return here if MCAM entries not allocated
// MSI-X APIs
extern "C" {
    pub fn otx2_free_cints(pfvf: *mut otx2_nic, n: c_int);
}
extern "C" {
    pub fn otx2_set_cints_affinity(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_set_mac_address(netdev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn otx2_hw_set_mtu(pfvf: *mut otx2_nic, mtu: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_tx_timeout(netdev: *mut net_device, txq: c_uint);
}
extern "C" {
    pub fn otx2_get_mac_from_af(netdev: *mut net_device);
}
extern "C" {
    pub fn otx2_config_irq_coalescing(pfvf: *mut otx2_nic, qidx: c_int);
}
extern "C" {
    pub fn otx2_config_pause_frm(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_setup_segmentation(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_reset_mac_stats(pfvf: *mut otx2_nic) -> c_int;
}
// RVU block related APIs
extern "C" {
    pub fn otx2_attach_npa_nix(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_detach_resources(mbox: *mut mbox) -> c_int;
}
extern "C" {
    pub fn otx2_config_npa(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_sq_aura_pool_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_rq_aura_pool_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_aura_pool_free(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_free_aura_ptr(pfvf: *mut otx2_nic, type: c_int);
}
extern "C" {
    pub fn otx2_sq_free_sqbs(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_config_nix(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_config_nix_queues(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_txschq_config(pfvf: *mut otx2_nic, lvl: c_int, prio: c_int, pfc_en: bool) -> c_int;
}
extern "C" {
    pub fn otx2_txsch_alloc(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_txschq_stop(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_txschq_free_one(pfvf: *mut otx2_nic, lvl: u16, schq: u16);
}
extern "C" {
    pub fn otx2_free_pending_sqe(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_sqb_flush(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_rxtx_enable(pfvf: *mut otx2_nic, enable: bool) -> c_int;
}
extern "C" {
    pub fn otx2_ctx_disable(mbox: *mut mbox, type: c_int, npa: bool);
}
extern "C" {
    pub fn otx2_nix_config_bp(pfvf: *mut otx2_nic, enable: bool) -> c_int;
}
extern "C" {
    pub fn otx2_nix_cpt_config_bp(pfvf: *mut otx2_nic, enable: bool) -> c_int;
}
extern "C" {
    pub fn otx2_cleanup_rx_cqes(pfvf: *mut otx2_nic, cq: *mut otx2_cq_queue, qidx: c_int);
}
extern "C" {
    pub fn otx2_cleanup_tx_cqes(pfvf: *mut otx2_nic, cq: *mut otx2_cq_queue);
}
extern "C" {
    pub fn otx2_sq_init(pfvf: *mut otx2_nic, qidx: u16, sqb_aura: u16) -> c_int;
}
extern "C" {
    pub fn otx2_sq_aq_init(dev: *mut c_void, qidx: u16, chan_offset: u8, sqb_aura: u16) -> c_int;
}
extern "C" {
    pub fn cn10k_sq_aq_init(dev: *mut c_void, qidx: u16, chan_offset: u8, sqb_aura: u16) -> c_int;
}
extern "C" {
    pub fn otx2_init_rsrc(pdev: *mut pci_dev, pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_free_queue_mem(qset: *mut otx2_qset);
}
extern "C" {
    pub fn otx2_alloc_queue_mem(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_init_hw_resources(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_free_hw_resources(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_wq_init(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_check_pf_usable(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_pfaf_mbox_init(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_register_mbox_intr(pf: *mut otx2_nic, probe_af: bool) -> c_int;
}
extern "C" {
    pub fn otx2_realloc_msix_vectors(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_pfaf_mbox_destroy(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_disable_mbox_intr(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_disable_napi(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_cq_intr_handler(irq: c_int, cq_irq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn otx2_rq_init(pfvf: *mut otx2_nic, qidx: u16, lpb_aura: u16) -> c_int;
}
extern "C" {
    pub fn otx2_cq_init(pfvf: *mut otx2_nic, qidx: u16) -> c_int;
}
extern "C" {
    pub fn otx2_set_hw_capabilities(pfvf: *mut otx2_nic) -> c_int;
}
// RSS configuration APIs
extern "C" {
    pub fn otx2_rss_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_set_flowkey_cfg(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_set_rss_key(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_set_rss_table(pfvf: *mut otx2_nic, ctx_id: c_int, ind_tbl: *const u32) -> c_int;
}
// Mbox handlers
extern "C" {
    pub fn otx2_set_fec_stats_count(pfvf: *mut otx2_nic);
}
// Device stats APIs
extern "C" {
    pub fn otx2_get_dev_stats(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_update_lmac_stats(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_update_lmac_fec_stats(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_update_rq_stats(pfvf: *mut otx2_nic, qidx: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_update_sq_stats(pfvf: *mut otx2_nic, qidx: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn otx2vf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn otx2_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn otx2_stop(netdev: *mut net_device) -> c_int;
}
// MCAM filter related APIs
extern "C" {
    pub fn otx2_mcam_flow_init(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2vf_mcam_flow_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_alloc_mcam_entries(pfvf: *mut otx2_nic, count: u16) -> c_int;
}
extern "C" {
    pub fn otx2_mcam_flow_del(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_destroy_ntuple_flows(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_destroy_mcam_flows(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_remove_flow(pfvf: *mut otx2_nic, location: u32) -> c_int;
}
extern "C" {
    pub fn otx2_get_maxflows(flow_cfg: *mut otx2_flow_config) -> c_int;
}
extern "C" {
    pub fn otx2_rss_ctx_flow_del(pfvf: *mut otx2_nic, ctx_id: c_int);
}
extern "C" {
    pub fn otx2_del_macfilter(netdev: *mut net_device, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn otx2_add_macfilter(netdev: *mut net_device, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn otx2_enable_rxvlan(pf: *mut otx2_nic, enable: bool) -> c_int;
}
extern "C" {
    pub fn otx2_install_rxvlan_offload_flow(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_get_max_mtu(pfvf: *mut otx2_nic) -> u16;
}
extern "C" {
    pub fn otx2_smq_flush(pfvf: *mut otx2_nic, smq: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_mcam_entry_init(pfvf: *mut otx2_nic) -> c_int;
}
// tc support
extern "C" {
    pub fn otx2_init_tc(nic: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_shutdown_tc(nic: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_tc_apply_ingress_police_rules(nic: *mut otx2_nic);
}
// CGX/RPM DMAC filters support
extern "C" {
    pub fn otx2_dmacflt_get_max_cnt(pf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_dmacflt_add(pf: *mut otx2_nic, mac: *const u8, bit_pos: u32) -> c_int;
}
extern "C" {
    pub fn otx2_dmacflt_remove(pf: *mut otx2_nic, mac: *const u8, bit_pos: u32) -> c_int;
}
extern "C" {
    pub fn otx2_dmacflt_update(pf: *mut otx2_nic, mac: *mut u8, bit_pos: u32) -> c_int;
}
extern "C" {
    pub fn otx2_dmacflt_reinstall_flows(pf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_dmacflt_update_pfmac_flow(pfvf: *mut otx2_nic);
}

// DCB support
extern "C" {
    pub fn otx2_update_bpid_in_rqctx(pfvf: *mut otx2_nic, vlan_prio: c_int, qidx: c_int, pfc_enable: bool);
}
extern "C" {
    pub fn otx2_config_priority_flow_ctrl(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_dcbnl_set_ops(dev: *mut net_device) -> c_int;
}
// PFC support
extern "C" {
    pub fn otx2_pfc_txschq_config(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_pfc_txschq_alloc(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_pfc_txschq_update(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_pfc_txschq_stop(pfvf: *mut otx2_nic) -> c_int;
}

// MACSEC offload support
extern "C" {
    pub fn cn10k_mcs_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn cn10k_mcs_free(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn cn10k_handle_mcs_event(pfvf: *mut otx2_nic, event: *mut mcs_intr_info);
}

// qos support
extern "C" {
    pub fn otx2_get_txq_by_classid(pfvf: *mut otx2_nic, classid: u16) -> c_int;
}
extern "C" {
    pub fn otx2_qos_config_txschq(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn otx2_clean_qos_queues(pfvf: *mut otx2_nic);
}
extern "C" {
    pub fn rvu_event_up_notify(pf: *mut otx2_nic, info: *mut rep_event) -> c_int;
}
extern "C" {
    pub fn otx2_dma_unmap_skb_frags(pfvf: *mut otx2_nic, sg: *mut sg_list);
}
extern "C" {
    pub fn otx2_read_free_sqe(pfvf: *mut otx2_nic, qidx: u16) -> c_int;
}
