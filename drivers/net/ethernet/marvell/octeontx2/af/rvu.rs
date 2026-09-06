//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rvu.h
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

// PCI device IDs
pub const PCI_DEVID_OCTEONTX2_RVU_AF: c_uint = 0xA065;
pub const PCI_DEVID_OCTEONTX2_LBK: c_uint = 0xA061;
// Subsystem Device ID
pub const PCI_SUBSYS_DEVID_98XX: c_uint = 0xB100;
pub const PCI_SUBSYS_DEVID_96XX: c_uint = 0xB200;
pub const PCI_SUBSYS_DEVID_CN10K_A: c_uint = 0xB900;
pub const PCI_SUBSYS_DEVID_CNF10K_A: c_uint = 0xBA00;
pub const PCI_SUBSYS_DEVID_CNF10K_B: c_uint = 0xBC00;
pub const PCI_SUBSYS_DEVID_CN10K_B: c_uint = 0xBD00;
pub const PCI_SUBSYS_DEVID_CN20KA: c_uint = 0xC220;
pub const PCI_SUBSYS_DEVID_CNF20KA: c_uint = 0xC320;
// PCI BAR nos
pub const PCI_AF_REG_BAR_NUM: c_int = 0;
pub const PCI_PF_REG_BAR_NUM: c_int = 2;
pub const PCI_MBOX_BAR_NUM: c_int = 4;
pub const NAME_SIZE: c_int = 32;
pub const MAX_NIX_BLKS: c_int = 2;
pub const MAX_CPT_BLKS: c_int = 2;
// PF_FUNC
pub const RVU_OTX2_PFVF_PF_SHIFT: c_int = 10;
pub const RVU_OTX2_PFVF_PF_MASK: c_uint = 0x3F;
pub const RVU_PFVF_FUNC_SHIFT: c_int = 0;
pub const RVU_PFVF_FUNC_MASK: c_uint = 0x3FF;
pub const RVU_CN20K_PFVF_PF_SHIFT: c_int = 9;
pub const RVU_CN20K_PFVF_PF_MASK: c_uint = 0x7F;
pub const RVU_AFPF: c_int = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dump_ctx {
    pub lf: c_int,
    pub id: c_int,
    pub all: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_ctx {
    pub blkaddr: c_int,
    pub rvu: *mut rvu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_debugfs {
    pub root: *mut dentry,
    pub cgx_root: *mut dentry,
    pub cgx: *mut dentry,
    pub lmac: *mut dentry,
    pub npa: *mut dentry,
    pub nix: *mut dentry,
    pub npc: *mut dentry,
    pub cpt: *mut dentry,
    pub mcs_root: *mut dentry,
    pub mcs: *mut dentry,
    pub mcs_rx: *mut dentry,
    pub mcs_tx: *mut dentry,
    pub npa_aura_ctx: dump_ctx,
    pub npa_pool_ctx: dump_ctx,
    pub nix_cq_ctx: dump_ctx,
    pub nix_rq_ctx: dump_ctx,
    pub nix_sq_ctx: dump_ctx,
    pub nix_tm_ctx: dump_ctx,
    pub cpt_ctx: [cpt_ctx; MAX_CPT_BLKS],
    pub npa_qsize_id: c_int,
    pub nix_qsize_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_work {
    pub work: work_struct,
    pub rvu: *mut rvu,
    pub num_msgs: c_int,
    pub up_num_msgs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsrc_bmap {
    pub /: *mut *mut *mut unsigned long bmap; / Pointer to resource bitmap,
    pub /: *mut *mut u16 max; / Max resource id or count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_block {
    pub lf: rsrc_bmap,
    pub /: *mut *mut *mut admin_queue aq; / NIX/NPA AQ,
    pub /: *mut *mut *mut u16 fn_map; / LF to pcifunc mapping,
    pub multislot: bool,
    pub implemented: bool,
    pub /: *mut *mut u8 addr; / RVU_BLOCK_ADDR_E,
    pub /: *mut *mut u8 type; / RVU_BLOCK_TYPE_E,
    pub lfshift: u8,
    pub lookup_reg: u64,
    pub pf_lfcnt_reg: u64,
    pub vf_lfcnt_reg: u64,
    pub lfcfg_reg: u64,
    pub msixcfg_reg: u64,
    pub lfreset_reg: u64,
    pub name: [c_uchar; NAME_SIZE],
    pub rvu: *mut rvu,
    pub cpt_flt_eng_map: [u64; 3],
    pub cpt_rcvrd_eng_map: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast {
    pub mce_ctx: *mut qmem,
    pub mcast_buf: *mut qmem,
    pub replay_pkind: c_int,
    pub mce_counter: [rsrc_bmap; 2],
// Counters for both ingress and egress mcast lists
    pub /: *mut *mut mutex mce_lock; / Serialize MCE updates,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mce_list {
    pub head: hlist_head,
    pub count: c_int,
    pub max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp_elem {
    pub mcast_mce_list: nix_mce_list,
    pub mcast_grp_idx: u32,
    pub pcifunc: u32,
    pub mcam_index: c_int,
    pub mce_start_index: c_int,
    pub list: list_head,
    pub dir: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mcast_grp {
    pub mcast_grp_head: list_head,
    pub count: c_int,
    pub next_grp_index: c_int,
    pub /: *mut *mut mutex mcast_grp_lock; / Serialize MCE updates,
}

// layer metadata to uniquely identify a packet header field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_layer_mdata {
    pub lid: u8,
    pub ltype: u8,
    pub hdr: u8,
    pub key: u8,
    pub len: u8,
}

// Structure to represent a field present in the
// generated key. A key field may present anywhere and can
// be of any size in the generated key. Once this structure
// is populated for fields of interest then field's presence
// and location (if present) can be known.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_key_field {
// Masks where all set bits indicate position
// of a field in the key
//
    pub kw_mask: [u64; NPC_KWS_IN_KEY_SZ_MAX],
// Number of words in the key a field spans. If a field is
// of 16 bytes and key offset is 4 then the field will use
// 4 bytes in KW0, 8 bytes in KW1 and 4 bytes in KW2 and
// nr_kws will be 3(KW0, KW1 and KW2).
//
    pub nr_kws: c_int,
// used by packet header fields
    pub layer_mdata: npc_layer_mdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam {
    pub counters: rsrc_bmap,
    pub /: *mut *mut mutex lock; / MCAM entries and counters update lock,
    pub /: *mut *mut *mut unsigned long bmap; / bitmap, 0 => bmap_entries,
    pub /: *mut *mut *mut unsigned long bmap_reverse; / Reverse bitmap, bmap_entries => 0,
    pub /: *mut *mut u16 bmap_entries; / Number of unreserved MCAM entries,
    pub /: *mut *mut u16 bmap_fcnt; / MCAM entries free count,
    pub entry2pfvf_map: *mut u16,
    pub entry2cntr_map: *mut u16,
    pub cntr2pfvf_map: *mut u16,
    pub cntr_refcnt: *mut u16,
    pub entry2target_pffunc: *mut u16,
    pub /: *mut *mut u8 keysize; / MCAM keysize 112/224/448 bits,
    pub /: *mut *mut u8 banks; / Number of MCAM banks,
    pub /: *mut *mut u8 banks_per_entry;/ Number of keywords in key,
    pub /: *mut *mut u16 banksize; / Number of MCAM entries in each bank,
    pub /: *mut *mut u16 total_entries; / Total number of MCAM entries,
    pub /: *mut *mut u16 nixlf_offset; / Offset of nixlf rsvd uncast entries,
    pub /: *mut *mut u16 pf_offset; / Offset of PF's rsvd bcast, promisc entries,
    pub lprio_count: u16,
    pub lprio_start: u16,
    pub hprio_count: u16,
    pub hprio_end: u16,
    pub /: *mut *mut u16 rx_miss_act_cntr; / Counter for RX MISS action,
// fields present in the generated key
    pub tx_key_fields: [npc_key_field; NPC_KEY_FIELDS_MAX],
    pub rx_key_fields: [npc_key_field; NPC_KEY_FIELDS_MAX],
    pub tx_features: u64,
    pub rx_features: u64,
    pub mcam_rules: list_head,
}

// Structure for per RVU func info ie PF/VF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_pfvf {
    pub /: *mut *mut bool npalf; / Only one NPALF per RVU_FUNC,
    pub /: *mut *mut bool nixlf; / Only one NIXLF per RVU_FUNC,
    pub sso: u16,
    pub ssow: u16,
    pub cptlfs: u16,
    pub timlfs: u16,
    pub cpt1_lfs: u16,
    pub cgx_lmac: u8,
// Block LF's MSIX vector info
    pub /: *mut *mut rsrc_bmap msix; / Bitmap for MSIX vector alloc,

    pub /: *mut *mut *mut u16 msix_lfmap; / Vector to block LF mapping,
// NPA contexts
    pub aura_ctx: *mut qmem,
    pub pool_ctx: *mut qmem,
    pub npa_qints_ctx: *mut qmem,
    pub aura_bmap: *mut c_ulong,
    pub pool_bmap: *mut c_ulong,
// NIX contexts
    pub rq_ctx: *mut qmem,
    pub sq_ctx: *mut qmem,
    pub cq_ctx: *mut qmem,
    pub rss_ctx: *mut qmem,
    pub cq_ints_ctx: *mut qmem,
    pub nix_qints_ctx: *mut qmem,
    pub sq_bmap: *mut c_ulong,
    pub rq_bmap: *mut c_ulong,
    pub cq_bmap: *mut c_ulong,
    pub rx_chan_base: u16,
    pub tx_chan_base: u16,
    pub /: *mut *mut u8 rx_chan_cnt; / total number of RX channels,
    pub /: *mut *mut u8 tx_chan_cnt; / total number of TX channels,
    pub maxlen: u16,
    pub minlen: u16,
    pub /: *mut *mut bool hw_rx_tstamp_en; / Is rx_tstamp enabled,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / MAC address of this PF/VF,
    pub /: *mut *mut u8 default_mac[ETH_ALEN]; / MAC address from FWdata,
// Broadcast/Multicast/Promisc pkt replication info
    pub bcast_mce_idx: u16,
    pub mcast_mce_idx: u16,
    pub promisc_mce_idx: u16,
    pub bcast_mce_list: nix_mce_list,
    pub mcast_mce_list: nix_mce_list,
    pub promisc_mce_list: nix_mce_list,
    pub use_mce_list: bool,
    pub def_ucast_rule: *mut rvu_npc_mcam_rule,
    pub /: *mut *mut bool cgx_in_use; / this PF/VF using CGX?,
    pub /: *mut *mut int cgx_users; / number of cgx users - used only by PFs,
    pub intf_mode: c_int,
    pub /: *mut *mut u8 nix_blkaddr; / BLKADDR_NIX0/1 assigned to this PF,
    pub /: *mut *mut u8 nix_rx_intf; / NIX0_RX/NIX1_RX interface to NPC,
    pub /: *mut *mut u8 nix_tx_intf; / NIX0_TX/NIX1_TX interface to NPC,
    pub /: *mut *mut u8 lbkid; / NIX0/1 lbk link ID,
    pub addr*/: *mut *mut u64 lmt_base_addr; / Preseving the pcifunc's lmtst base,
    pub entry*/: *mut *mut u64 lmt_map_ent_w1; / Preseving the word1 of lmtst map table,
    pub flags: c_ulong,
    pub sdp_info: *mut sdp_node_info,
    pub /: *mut *mut u8 hw_prio; / Hw priority of default rules,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_pfvf_flags {
    NIXLF_INITIALIZED = 0,
    PF_SET_VF_MAC,
    PF_SET_VF_CFG,
    PF_SET_VF_TRUSTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bp {
    pub /: *mut *mut rsrc_bmap bpids; / free bpids bitmap,
    pub cgx_bpid_cnt: u16,
    pub sdp_bpid_cnt: u16,
    pub free_pool_base: u16,
    pub /: *mut *mut *mut u16 fn_map; / pcifunc mapping,
    pub /: *mut *mut *mut u8 intf_map; / interface type map,
    pub ref_cnt: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txsch {
    pub schq: rsrc_bmap,
    pub lvl: u8,

    pub pfvf_map: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_mark_format {
    pub total: u8,
    pub in_use: u8,
    pub cfg: *mut u32,
}

// smq(flush) to tl1 cir/pir info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_smq_tree_ctx {
    pub schq: u16,
    pub cir_off: u64,
    pub cir_val: u64,
    pub pir_off: u64,
    pub pir_val: u64,
}

// smq flush context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_smq_flush_ctx {
    pub smq: c_int,
    pub smq_tree_ctx: [nix_smq_tree_ctx; NIX_TXSCH_LVL_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_pkind {
    pub rsrc: rsrc_bmap,
    pub pfchan_map: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_flowkey {
pub const NIX_FLOW_KEY_ALG_MAX: c_int = 32;
    pub flowkey: [u32; NIX_FLOW_KEY_ALG_MAX],
    pub in_use: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lso {
    pub total: u8,
    pub in_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_txvlan {
pub const NIX_TX_VTAG_DEF_MAX: c_uint = 0x400;
    pub rsrc: rsrc_bmap,
    pub entry2pfvf_map: *mut u16,
    pub /: *mut *mut mutex rsrc_lock; / Serialize resource alloc/free,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_ipolicer {
    pub band_prof: rsrc_bmap,
    pub pfvf_map: *mut u16,
    pub match_id: *mut u16,
    pub ref_count: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_hw {
    pub blkaddr: c_int,
    pub rvu: *mut rvu,
    pub /: *mut *mut nix_txsch txsch[NIX_TXSCH_LVL_CNT]; / Tx schedulers,
    pub mcast: nix_mcast,
    pub mcast_grp: nix_mcast_grp,
    pub flowkey: nix_flowkey,
    pub mark_format: nix_mark_format,
    pub lso: nix_lso,
    pub txvlan: nix_txvlan,
    pub ipolicer: *mut nix_ipolicer,
    pub bp: nix_bp,
    pub tx_credits: *mut u64,
    pub cc_mcs_cnt: u8,
}

// RVU block's capabilities or functionality,
// which vary by silicon version/skew.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_cap {
// Transmit side supported functionality
    pub /: *mut *mut u8 nix_tx_aggr_lvl; / Tx link's traffic aggregation level,
    pub /: *mut *mut u16 nix_txsch_per_cgx_lmac; / Max Q's transmitting to CGX LMAC,
    pub /: *mut *mut u16 nix_txsch_per_lbk_lmac; / Max Q's transmitting to LBK LMAC,
    pub /: *mut *mut u16 nix_txsch_per_sdp_lmac; / Max Q's transmitting to SDP LMAC,
    pub /: *mut *mut bool nix_fixed_txschq_mapping; / Schq mapping fixed or flexible,
    pub /: *mut *mut bool nix_shaping; / Is shaping and coloring supported,
    pub /: *mut *mut bool nix_shaper_toggle_wait; / Shaping toggle needs poll/wait,
    pub /: *mut *mut bool nix_tx_link_bp; / Can link backpressure TL queues ?,
    pub /: *mut *mut bool nix_rx_multicast; / Rx packet replication support,
    pub /: *mut *mut bool nix_common_dwrr_mtu; / Common DWRR MTU for quantum config,
    pub /: *mut *mut bool per_pf_mbox_regs; / PF mbox specified in per PF registers ?,
    pub /: *mut *mut bool programmable_chans; / Channels programmable ?,
    pub ipolicer: bool,
    pub /: *mut *mut bool nix_multiple_dwrr_mtu; / Multiple DWRR_MTU to choose from,
    pub /: *mut *mut bool npc_hash_extract; / Hash extract enabled ?,
    pub /: *mut *mut bool npc_exact_match_enabled; / Exact match supported ?,
    pub /: *mut *mut bool cpt_rxc; / Is CPT-RXC supported,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_hwinfo {
    pub /: *mut *mut u8 total_pfs; / MAX RVU PFs HW supports,
    pub /: *mut *mut u16 total_vfs; / Max RVU VFs HW supports,
    pub /: *mut *mut u16 max_vfs_per_pf; / Max VFs that can be attached to a PF,
    pub cgx: u8,
    pub lmac_per_cgx: u8,
    pub /: *mut *mut u16 cgx_chan_base; / CGX base channel number,
    pub /: *mut *mut u16 lbk_chan_base; / LBK base channel number,
    pub /: *mut *mut u16 sdp_chan_base; / SDP base channel number,
    pub /: *mut *mut u16 cpt_chan_base; / CPT base channel number,
    pub cgx_links: u8,
    pub lbk_links: u8,
    pub sdp_links: u8,
    pub /: *mut *mut u8 cpt_links; / Number of CPT links,
    pub /: *mut *mut u8 npc_kpus; / No of parser units,
    pub /: *mut *mut u8 npc_kpms; / Number of enhanced parser units,
    pub /: *mut *mut u8 npc_kex_extr; / Number of LDATA extractors per KEX,
    pub /: *mut *mut u8 npc_pkinds; / No of port kinds,
    pub /: *mut *mut u8 npc_intfs; / No of interfaces,
    pub /: *mut *mut u16 npc_kpu_entries; / No of KPU entries,
    pub /: *mut *mut u16 npc_counters; / No of match stats counters,
    pub /: *mut *mut u32 lbk_bufsize; / FIFO size supported by LBK,
    pub /: *mut *mut bool npc_ext_set; / Extended register set,
    pub /: *mut *mut u64 npc_stat_ena; / Match stats enable bit,
    pub cap: hw_cap,
    pub /: *mut *mut rvu_block block[BLK_COUNT]; / Block info,
    pub nix: *mut nix_hw,
    pub rvu: *mut rvu,
    pub pkind: npc_pkind,
    pub mcam: npc_mcam,
    pub table: *mut npc_exact_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_wq_info {
    pub mbox: otx2_mbox,
    pub mbox_wrk: *mut rvu_work,
    pub mbox_up: otx2_mbox,
    pub mbox_wrk_up: *mut rvu_work,
    pub mbox_wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_irq_data {
    pub intr_status: u64,
    pub intr): int mdevs, u64,
    pub intr): int mdevs, u64,
    pub rvu: *mut rvu,
    pub vec_num: c_int,
    pub start: c_int,
    pub mdevs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_ops {
    pub rvu_irq): *mut *mut irqreturn_t (pf_intr_handler)(int irq, void,
    pub rvu_irq): *mut *mut irqreturn_t (afvf_intr_handler)(int irq, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_fwdata {
    pub info: sdp_node_info,
    pub valid: u8,
pub const RVU_CHANL_INFO_RESERVED: c_int = 379;
    pub reserved: [u8; RVU_CHANL_INFO_RESERVED],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altaf_intr_notify {
    pub flr_pf_bmap: [c_ulong; 2],
    pub flr_vf_bmap: [c_ulong; 2],
    pub gint_paddr: c_ulong,
    pub gint_iova_addr: c_ulong,
    pub reserved: [c_ulong; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_fwdata {
pub const RVU_FWDATA_HEADER_MAGIC: c_uint = 0xCFDA	/* Custom Firmware Data*/;
pub const RVU_FWDATA_VERSION: c_uint = 0x0001;
    pub header_magic: u32,
    pub /: *mut *mut u32 version; / version id,
// MAC address
pub const PF_MACNUM_MAX: c_int = 32;
pub const VF_MACNUM_MAX: c_int = 256;
    pub pf_macs: [u64; PF_MACNUM_MAX],
    pub vf_macs: [u64; VF_MACNUM_MAX],
    pub sclk: u64,
    pub rclk: u64,
    pub mcam_addr: u64,
    pub mcam_sz: u64,
    pub msixtr_base: u64,
    pub ptp_ext_clk_rate: u32,
    pub ptp_ext_tstamp: u32,
    pub channel_data: channel_fwdata,
    pub altaf_intr_info: altaf_intr_notify,
pub const FWDATA_RESERVED_MEM: c_int = 946;
    pub reserved: [u64; FWDATA_RESERVED_MEM],
pub const CGX_MAX: c_int = 9;
pub const CGX_LMACS_MAX: c_int = 4;
pub const CGX_LMACS_USX: c_int = 8;
pub const FWDATA_CGX_LMAC_OFFSET: c_int = 10536;
}

// Do not add new fields below this line
// KPU profile adapter structure gathering all KPU configuration data and abstracting out the
// source where it came from.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile_adapter {
    pub name: *const c_char,
    pub version: u64,
    pub lt_def: *const npc_lt_def_cfg,
    pub /: *mut *mut *mut npc_kpu_profile_action ikpu; / array[pkinds],
    pub /: *mut *mut *mut npc_kpu_profile_action ikpu2; / array[pkinds],
    pub /: *mut *mut *mut npc_kpu_profile kpu; / array[kpus],
#[repr(C)]
#[derive(Copy, Clone)]
pub union npc_mcam_key_prfl {
    pub mkex: *const npc_mcam_kex,
// used for cn9k and cn10k
    pub /: *const *const *const npc_mcam_kex_extr mkex_extr; / used for cn20k,
    pub mcam_kex_prfl: },
    pub mkex_hash: *mut npc_mcam_kex_hash,
    pub custom: bool,
    pub pkinds: usize,
    pub kpus: usize,
    pub from_fs: bool,
}

pub const RVU_SWITCH_LBK_CHAN: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_switch {
    pub /: *mut *mut mutex switch_lock; / Serialize flow installation,
    pub used_entries: u32,
    pub entry2pcifunc: *mut u16,
    pub mode: u16,
    pub start_entry: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rep_evtq_ent {
    pub node: list_head,
    pub event: rep_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu {
    pub afreg_base: *mut void __iomem,
    pub pfreg_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub hw: *mut rvu_hwinfo,
    pub pf: *mut rvu_pfvf,
    pub hwvf: *mut rvu_pfvf,
    pub /: *mut *mut mutex rsrc_lock; / Serialize resource alloc/free,
    pub /: *mut *mut mutex alias_lock; / Serialize bar2 alias access,
    pub /: *mut *mut int vfs; / Number of VFs attached to RVU,
    pub /: *mut *mut u16 vf_devid; / VF devices id,
    pub def_rule_cntr_en: bool,
    pub nix_blkaddr: [c_int; MAX_NIX_BLKS],
// Mbox
    pub afpf_wq_info: mbox_wq_info,
    pub afvf_wq_info: mbox_wq_info,
// PF FLR
    pub flr_wrk: *mut rvu_work,
    pub flr_wq: *mut workqueue_struct,
    pub /: *mut *mut mutex flr_lock; / Serialize FLRs,
// MSI-X
    pub num_vec: u16,
    pub irq_name: *mut c_char,
    pub irq_allocated: *mut bool,
    pub msix_base_iova: dma_addr_t,
    pub /: *mut *mut u64 msixtr_base_phy; / Register reset value,
// CGX

    pub /: *mut *mut u16 cgx_mapped_vfs; / maximum CGX mapped VFs,
    pub cgx_mapped_pfs: u8,
    pub /: *mut *mut u8 cgx_cnt_max; / CGX port count max,
    pub /: *mut *mut *mut u8 pf2cgxlmac_map; / pf to cgx_lmac map,
    pub for: *mut *mut *mut u64 cgxlmac2pf_map; / bitmap of mapped pfs,
// every cgx lmac port
//
    pub /: *mut *mut unsigned long pf_notify_bmap; / Flags for PF notification,
    pub /: *mut *mut *mut *mut void cgx_idmap; / cgx id to cgx data map table,
    pub cgx_evh_work: work_struct,
    pub cgx_evh_wq: *mut workqueue_struct,
    pub /: *mut *mut spinlock_t cgx_evq_lock; / cgx event queue lock,
    pub /: *mut *mut list_head cgx_evq_head; / cgx event queue head,
    pub /: *mut *mut mutex cgx_cfg_lock; / serialize cgx configuration,
    pub /: *mut *mut char mkex_pfl_name[MKEX_NAME_LEN]; / Configured MKEX profile name,
    pub /: *mut *mut char kpu_pfl_name[KPU_NAME_LEN]; / Configured KPU profile name,
// Firmware data
    pub fwdata: *mut rvu_fwdata,
    pub kpu_fwdata: *const c_void,
    pub kpu_fwdata_sz: usize,
    pub kpu_prfl_addr: *mut void __iomem,
// NPC KPU data
    pub kpu: npc_kpu_profile_adapter,
    pub ptp: *mut ptp,
    pub mcs_blk_cnt: c_int,
    pub cpt_pf_num: c_int,

    pub rvu_dbg: rvu_debugfs,

    pub rvu_dl: *mut rvu_devlink,
// RVU switch implementation over NPC with DMAC rules
    pub rswitch: rvu_switch,
    pub mcs_intr_work: work_struct,
    pub mcs_intr_wq: *mut workqueue_struct,
    pub mcs_intrq_head: list_head,
// mcs interrupt queue lock
    pub mcs_intrq_lock: spinlock_t,
// CPT interrupt lock
    pub cpt_intr_lock: spinlock_t,
    pub /: *mut *mut mutex mbox_lock; / Serialize mbox up and down msgs,
    pub rep_pcifunc: u16,
    pub altaf_ready: bool,
    pub rep_cnt: c_int,
    pub rep2pfvf_map: *mut u16,
    pub rep_mode: u8,
    pub rep_evt_work: work_struct,
    pub rep_evt_wq: *mut workqueue_struct,
    pub rep_evtq_head: list_head,
// Representor event lock
    pub rep_evtq_lock: spinlock_t,
    pub ng_rvu: *mut ng_rvu,
}

extern "C" {
    pub fn readq(offset): rvu->afreg_base + ((block << 28) |) -> return;
}
extern "C" {
    pub fn readq(offset: rvu->pfreg_base +) -> return;
}
// HW requires read back of RVU_AF_BAR2_SEL register to make sure completion of
// write operation.
//
// Barrier to ensure read completes before accessing LF registers
// Silicon revisions
// 96XX A0/B0, 95XX A0/A1/B0 chips
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
// On CNF10KA and CNF10KB silicons only two CGX blocks are connected
// to NIX.
//
extern "C" {
    pub fn NIX_CHAN_CGX_LMAC_CHX(_arg: cgxid, _arg: lmacid, _arg: chan) -> return;
}
extern "C" {
    pub fn NIX_CHAN_LBK_CHX(_arg: lbkid, _arg: chan) -> return;
}
extern "C" {
    pub fn NIX_CHAN_SDP_CHX(_arg: chan) -> return;
}
// Function Prototypes
// RVU
//
pub const RVU_LBK_VF_DEVID: c_uint = 0xA0F8;
// check if PF_FUNC is AF
extern "C" {
    pub fn rvu_alloc_bitmap(rsrc: *mut rsrc_bmap) -> c_int;
}
extern "C" {
    pub fn rvu_free_bitmap(rsrc: *mut rsrc_bmap);
}
extern "C" {
    pub fn rvu_alloc_rsrc(rsrc: *mut rsrc_bmap) -> c_int;
}
extern "C" {
    pub fn rvu_free_rsrc(rsrc: *mut rsrc_bmap, id: c_int);
}
extern "C" {
    pub fn is_rsrc_free(rsrc: *mut rsrc_bmap, id: c_int) -> bool;
}
extern "C" {
    pub fn rvu_rsrc_free_count(rsrc: *mut rsrc_bmap) -> c_int;
}
extern "C" {
    pub fn rvu_alloc_rsrc_contig(rsrc: *mut rsrc_bmap, nrsrc: c_int) -> c_int;
}
extern "C" {
    pub fn rvu_free_rsrc_contig(rsrc: *mut rsrc_bmap, nrsrc: c_int, start: c_int);
}
extern "C" {
    pub fn rvu_rsrc_check_contig(rsrc: *mut rsrc_bmap, nrsrc: c_int) -> bool;
}
extern "C" {
    pub fn rvu_get_rsrc_mapcount(pfvf: *mut rvu_pfvf, blkaddr: c_int) -> u16;
}
extern "C" {
    pub fn rvu_get_pf_numvfs(rvu: *mut rvu, pf: c_int, numvfs: *mut c_int, hwvf: *mut c_int);
}
extern "C" {
    pub fn is_block_implemented(hw: *mut rvu_hwinfo, blkaddr: c_int) -> bool;
}
extern "C" {
    pub fn is_pf_func_valid(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn is_pffunc_map_valid(rvu: *mut rvu, pcifunc: u16, blktype: c_int) -> bool;
}
extern "C" {
    pub fn rvu_get_lf(rvu: *mut rvu, block: *mut rvu_block, pcifunc: u16, slot: u16) -> c_int;
}
extern "C" {
    pub fn rvu_lf_reset(rvu: *mut rvu, block: *mut rvu_block, lf: c_int) -> c_int;
}
extern "C" {
    pub fn rvu_get_blkaddr(rvu: *mut rvu, blktype: c_int, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn rvu_poll_reg(rvu: *mut rvu, block: u64, offset: u64, mask: u64, zero: bool) -> c_int;
}
extern "C" {
    pub fn rvu_get_num_lbk_chans() -> c_int;
}
extern "C" {
    pub fn rvu_ndc_sync(rvu: *mut rvu, lfblkid: c_int, lfidx: c_int, lfoffset: u64) -> c_int;
}
// RVU HW reg validation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regmap_block {
    TXSCHQ_HWREGMAP = 0,
    MAX_HWREGMAP,
}

extern "C" {
    pub fn rvu_check_valid_reg(regmap: c_int, regblk: c_int, reg: u64) -> bool;
}
// NPA/NIX AQ APIs
extern "C" {
    pub fn rvu_aq_free(rvu: *mut rvu, aq: *mut admin_queue);
}
// SDP APIs
extern "C" {
    pub fn rvu_sdp_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn is_sdp_pfvf(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn is_sdp_pf(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn is_sdp_vf(rvu: *mut rvu, pcifunc: u16) -> bool;
}
// CGX APIs
// cgx_id = (map >> 4) & 0xF;
// lmac_id = (map & 0xF);

// Mbox APIs
extern "C" {
    pub fn rvu_cgx_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_exit(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_config_rxtx(rvu: *mut rvu, pcifunc: u16, start: bool) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_enadis_rx_bp(rvu: *mut rvu, pf: c_int, enable: bool);
}
extern "C" {
    pub fn rvu_cgx_start_stop_io(rvu: *mut rvu, pcifunc: u16, start: bool) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_disable_dmac_entries(rvu: *mut rvu, pcifunc: u16);
}
// NPA APIs
extern "C" {
    pub fn rvu_npa_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_npa_freemem(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_npa_lf_teardown(rvu: *mut rvu, pcifunc: u16, npalf: c_int);
}
// NIX APIs
extern "C" {
    pub fn is_nixlf_attached(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn rvu_nix_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_nix_freemem(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_get_nixlf_count(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_nix_lf_teardown(rvu: *mut rvu, pcifunc: u16, blkaddr: c_int, npalf: c_int);
}
extern "C" {
    pub fn nix_get_nixlf(rvu: *mut rvu, pcifunc: u16, nixlf: *mut c_int, nix_blkaddr: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rvu_get_next_nix_blkaddr(rvu: *mut rvu, blkaddr: c_int) -> c_int;
}
extern "C" {
    pub fn rvu_nix_reset_mac(pfvf: *mut rvu_pfvf, pcifunc: c_int);
}
extern "C" {
    pub fn rvu_get_nix_blkaddr(rvu: *mut rvu, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn nix_get_dwrr_mtu_reg(hw: *mut rvu_hwinfo, smq_link_type: c_int) -> c_int;
}
extern "C" {
    pub fn convert_dwrr_mtu_to_bytes(dwrr_mtu: u8) -> u32;
}
extern "C" {
    pub fn convert_bytes_to_dwrr_mtu(bytes: u32) -> u32;
}
extern "C" {
    pub fn rvu_nix_mcast_flr_free_entries(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_nix_flr_free_bpids(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_block_bcast_xon(rvu: *mut rvu, blkaddr: c_int);
}
// NPC APIs
extern "C" {
    pub fn rvu_npc_freemem(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_npc_get_pkind(rvu: *mut rvu, pf: u16) -> c_int;
}
extern "C" {
    pub fn rvu_npc_set_pkind(rvu: *mut rvu, pkind: c_int, pfvf: *mut rvu_pfvf);
}
extern "C" {
    pub fn npc_config_ts_kpuaction(rvu: *mut rvu, pf: c_int, pcifunc: u16, en: bool) -> c_int;
}
extern "C" {
    pub fn rvu_npc_disable_mcam_entries(rvu: *mut rvu, pcifunc: u16, nixlf: c_int);
}
extern "C" {
    pub fn rvu_npc_enable_mcam_by_entry_index(rvu: *mut rvu, entry: c_int, intf: c_int, enable: bool) -> bool;
}
extern "C" {
    pub fn rvu_npc_free_mcam_entries(rvu: *mut rvu, pcifunc: u16, nixlf: c_int);
}
extern "C" {
    pub fn rvu_npc_disable_default_entries(rvu: *mut rvu, pcifunc: u16, nixlf: c_int);
}
extern "C" {
    pub fn rvu_npc_enable_default_entries(rvu: *mut rvu, pcifunc: u16, nixlf: c_int);
}
extern "C" {
    pub fn rvu_npc_clear_ucast_entry(rvu: *mut rvu, pcifunc: c_int, nixlf: c_int);
}
extern "C" {
    pub fn is_npc_intf_tx(intf: u8) -> bool;
}
extern "C" {
    pub fn is_npc_intf_rx(intf: u8) -> bool;
}
extern "C" {
    pub fn is_npc_interface_valid(rvu: *mut rvu, intf: u8) -> bool;
}
extern "C" {
    pub fn rvu_npc_get_tx_nibble_cfg(rvu: *mut rvu, nibble_ena: u64) -> c_int;
}
extern "C" {
    pub fn npc_flow_steering_init(rvu: *mut rvu, blkaddr: c_int) -> c_int;
}
extern "C" {
    pub fn npc_get_bank(mcam: *mut npc_mcam, index: c_int) -> c_int;
}
extern "C" {
    pub fn npc_mcam_enable_flows(rvu: *mut rvu, target: u16);
}
extern "C" {
    pub fn npc_mcam_disable_flows(rvu: *mut rvu, target: u16);
}
extern "C" {
    pub fn npc_config_cntr_default_entries(rvu: *mut rvu, enable: bool) -> c_int;
}
extern "C" {
    pub fn is_cgx_config_permitted(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn rvu_cgx_check_permission_and_set_pkind(rvu: *mut rvu, pcifunc: u16, pkind: c_int) -> bool;
}
extern "C" {
    pub fn rvu_cgx_is_pkind_config_permitted(rvu: *mut rvu, pcifunc: u16) -> bool;
}
extern "C" {
    pub fn is_mac_feature_supported(rvu: *mut rvu, pf: c_int, feature: c_int) -> bool;
}
extern "C" {
    pub fn rvu_cgx_get_fifolen(rvu: *mut rvu) -> u32;
}
extern "C" {
    pub fn cgxlmac_to_pf(rvu: *mut rvu, cgx_id: c_int, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_config_tx(cgxd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_tx_enable(rvu: *mut rvu, pcifunc: u16, enable: bool) -> c_int;
}
extern "C" {
    pub fn rvu_cgx_cfg_pause_frm(rvu: *mut rvu, pcifunc: u16, tx_pause: u8, rx_pause: u8) -> c_int;
}
extern "C" {
    pub fn rvu_mac_reset(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_cgx_get_lmac_fifolen(rvu: *mut rvu, cgx: c_int, lmac: c_int) -> u32;
}
extern "C" {
    pub fn cgx_start_linkup(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_npc_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn npc_mcam_rsrcs_reserve(rvu: *mut rvu, blkaddr: c_int, entry_idx: c_int);
}
extern "C" {
    pub fn npc_is_feature_supported(rvu: *mut rvu, features: u64, intf: u8) -> bool;
}
extern "C" {
    pub fn npc_mcam_rsrcs_init(rvu: *mut rvu, blkaddr: c_int) -> c_int;
}
extern "C" {
    pub fn npc_mcam_rsrcs_deinit(rvu: *mut rvu);
}
// CPT APIs
extern "C" {
    pub fn rvu_cpt_register_interrupts(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_cpt_unregister_interrupts(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_cpt_ctx_flush(rvu: *mut rvu, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn rvu_cpt_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_get_cpt_chan_mask(rvu: *mut rvu) -> u32;
}

// CN10K RVU
extern "C" {
    pub fn rvu_set_channels_base(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_program_channels(rvu: *mut rvu);
}
// CN10K NIX
extern "C" {
    pub fn rvu_nix_block_cn10k_init(rvu: *mut rvu, nix_hw: *mut nix_hw);
}
// CN10K RVU - LMT
extern "C" {
    pub fn rvu_reset_lmt_map_tbl(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_apr_block_cn10k_init(rvu: *mut rvu);
}

extern "C" {
    pub fn rvu_dbg_init(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_dbg_exit(rvu: *mut rvu);
}

extern "C" {
    pub fn rvu_ndc_fix_locked_cacheline(rvu: *mut rvu, blkaddr: c_int) -> c_int;
}
// RVU Switch
extern "C" {
    pub fn rvu_switch_enable(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_switch_disable(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_switch_update_rules(rvu: *mut rvu, pcifunc: u16, ena: bool);
}
extern "C" {
    pub fn rvu_switch_enable_lbk_link(rvu: *mut rvu, pcifunc: u16, ena: bool);
}
extern "C" {
    pub fn rvu_get_hwvf(rvu: *mut rvu, pcifunc: c_int) -> c_int;
}
// CN10K MCS
extern "C" {
    pub fn rvu_mcs_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_mcs_flr_handler(rvu: *mut rvu, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn rvu_mcs_ptp_cfg(rvu: *mut rvu, rpm_id: u8, lmac_id: u8, ena: bool);
}
extern "C" {
    pub fn rvu_mcs_exit(rvu: *mut rvu);
}
// Representor APIs
extern "C" {
    pub fn rvu_rep_pf_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_rep_install_mcam_rules(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_rep_update_rules(rvu: *mut rvu, pcifunc: u16, ena: bool);
}
extern "C" {
    pub fn rvu_rep_notify_pfvf_state(rvu: *mut rvu, pcifunc: u16, enable: bool) -> c_int;
}
extern "C" {
    pub fn npc_mcam_verify_entry(mcam: *mut npc_mcam, pcifunc: u16, entry: c_int) -> c_int;
}
