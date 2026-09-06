//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpaa2-eth.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2016-2022 NXP
//

pub const DPAA2_ETH_STORE_SIZE: c_int = 16;
// Maximum number of scatter-gather entries in an ingress frame,
// considering the maximum receive frame size is 64K
//

// Maximum acceptable MTU value. It is in direct relation with the hardware
// enforced Max Frame Length (currently 10k).
//

// Convert L3 MTU to L2 MFL

// Set the taildrop threshold (in bytes) to allow the enqueue of a large
// enough number of jumbo frames in the Rx queues (length of the current
// frame is not taken into account when making the taildrop decision)
//

// Maximum burst size value for Tx shaping
pub const DPAA2_ETH_MAX_BURST_SIZE: c_uint = 0xF7FF;
// Maximum number of Tx confirmation frames to be processed
// in a single NAPI call
//
pub const DPAA2_ETH_TXCONF_PER_NAPI: c_int = 256;
// Maximum number of Tx frames to be processed in a single NAPI
// call when AF_XDP is running. Bind it to DPAA2_ETH_TXCONF_PER_NAPI
// to maximize the throughput.
//

// Buffer qouta per channel. We want to keep in check number of ingress frames
// in flight: for small sized frames, congestion group taildrop may kick in
// first; for large sizes, Rx FQ taildrop threshold will ensure only a
// reasonable number of frames will be pending at any given time.
// Ingress frame drop due to buffer pool depletion should be a corner case only
//
pub const DPAA2_ETH_NUM_BUFS: c_int = 1280;

// Congestion group taildrop threshold: number of frames allowed to accumulate
// at any moment in a group of Rx queues belonging to the same traffic class.
// Choose value such that we don't risk depleting the buffer pool before the
// taildrop kicks in
//

// Congestion group notification threshold: when this many frames accumulate
// on the Rx queues belonging to the same TC, the MAC is instructed to send
// PFC frames for that TC.
// When number of pending frames drops below exit threshold transmission of
// PFC frames is stopped.
//

// Maximum number of buffers that can be acquired/released through a single
// QBMan command
//
pub const DPAA2_ETH_BUFS_PER_CMD: c_int = 7;
// Hardware requires alignment for ingress/egress buffer addresses
pub const DPAA2_ETH_TX_BUF_ALIGN: c_int = 64;

// Hardware annotation area in RX/TX buffers
pub const DPAA2_ETH_RX_HWA_SIZE: c_int = 64;
pub const DPAA2_ETH_TX_HWA_SIZE: c_int = 128;
// PTP nominal frequency 1GHz
pub const DPAA2_PTP_CLK_PERIOD_NS: c_int = 1;
// Due to a limitation in WRIOP 1.0.0, the RX buffer data must be aligned
// to 256B. For newer revisions, the requirement is only for 64B alignment
//
pub const DPAA2_ETH_RX_BUF_ALIGN_REV1: c_int = 256;
pub const DPAA2_ETH_RX_BUF_ALIGN: c_int = 64;
// The firmware allows assigning multiple buffer pools to a single DPNI -
// maximum 8 DPBP objects. By default, only the first DPBP (idx 0) is used for
// all queues. Thus, when enabling AF_XDP we must accommodate up to 9 DPBPs
// object: the default and 8 other distinct buffer pools, one for each queue.
//
pub const DPAA2_ETH_DEFAULT_BP_IDX: c_int = 0;
pub const DPAA2_ETH_MAX_BPS: c_int = 9;
// We are accommodating a skb backpointer and some S/G info
// in the frame's software annotation. The hardware
// options are either 0 or 64, so we choose the latter.
//
pub const DPAA2_ETH_SWA_SIZE: c_int = 64;
// We store different information in the software annotation area of a Tx frame
// based on what type of frame it is
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_eth_swa_type {
    DPAA2_ETH_SWA_SINGLE,
    DPAA2_ETH_SWA_SG,
    DPAA2_ETH_SWA_XDP,
    DPAA2_ETH_SWA_XSK,
    DPAA2_ETH_SWA_SW_TSO,
}

// Must keep this struct smaller than DPAA2_ETH_SWA_SIZE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_swa {
    pub type: dpaa2_eth_swa_type,
    pub skb: *mut sk_buff,
    pub sgt_size: c_int,
    pub single: },
    pub skb: *mut sk_buff,
    pub scl: *mut scatterlist,
    pub num_sg: c_int,
    pub sgt_size: c_int,
    pub sg: },
    pub dma_size: c_int,
    pub xdpf: *mut xdp_frame,
    pub xdp: },
    pub xdp_buff: *mut xdp_buff,
    pub sgt_size: c_int,
    pub xsk: },
    pub skb: *mut sk_buff,
    pub num_sg: c_int,
    pub sgt_size: c_int,
    pub is_last_fd: c_int,
    pub tso: },
}

// Annotation valid bits in FD FRC
pub const DPAA2_FD_FRC_FASV: c_uint = 0x8000;
pub const DPAA2_FD_FRC_FAEADV: c_uint = 0x4000;
pub const DPAA2_FD_FRC_FAPRV: c_uint = 0x2000;
pub const DPAA2_FD_FRC_FAIADV: c_uint = 0x1000;
pub const DPAA2_FD_FRC_FASWOV: c_uint = 0x0800;
pub const DPAA2_FD_FRC_FAICFDV: c_uint = 0x0400;
// Error bits in FD CTRL

// Annotation bits in FD CTRL
pub const DPAA2_FD_CTRL_ASAL: c_uint = 0x00020000	/* ASAL = 128B */;
// Frame annotation status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fas {
    pub reserved: u8,
    pub ppid: u8,
    pub ifpid: __le16,
    pub status: __le32,
}

// Frame annotation status word is located in the first 8 bytes
// of the buffer's hardware annoatation area
//
pub const DPAA2_FAS_OFFSET: c_int = 0;

// Timestamp is located in the next 8 bytes of the buffer's
// hardware annotation area
//
pub const DPAA2_TS_OFFSET: c_uint = 0x8;
// Frame annotation parse results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fapr {
// 64-bit word 1
    pub faf_lo: __le32,
    pub faf_ext: __le16,
    pub nxt_hdr: __le16,
// 64-bit word 2
    pub faf_hi: __le64,
// 64-bit word 3
    pub last_ethertype_offset: u8,
    pub vlan_tci_offset_n: u8,
    pub vlan_tci_offset_1: u8,
    pub llc_snap_offset: u8,
    pub eth_offset: u8,
    pub ip1_pid_offset: u8,
    pub shim_offset_2: u8,
    pub shim_offset_1: u8,
// 64-bit word 4
    pub l5_offset: u8,
    pub l4_offset: u8,
    pub gre_offset: u8,
    pub l3_offset_n: u8,
    pub l3_offset_1: u8,
    pub mpls_offset_n: u8,
    pub mpls_offset_1: u8,
    pub pppoe_offset: u8,
// 64-bit word 5
    pub running_sum: __le16,
    pub gross_running_sum: __le16,
    pub ipv6_frag_offset: u8,
    pub nxt_hdr_offset: u8,
    pub routing_hdr_offset_2: u8,
    pub routing_hdr_offset_1: u8,
// 64-bit word 6
    pub /: *mut *mut u8 reserved[5]; / Soft-parsing context,
    pub ip_proto_offset_n: u8,
    pub nxt_hdr_frag_offset: u8,
    pub parse_error_code: u8,
}

pub const DPAA2_FAPR_OFFSET: c_uint = 0x10;

// Frame annotation egress action descriptor
pub const DPAA2_FAEAD_OFFSET: c_uint = 0x58;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_faead {
    pub conf_fqid: __le32,
    pub ctrl: __le32,
}

pub const DPAA2_FAEAD_A2V: c_uint = 0x20000000;
pub const DPAA2_FAEAD_A4V: c_uint = 0x08000000;
pub const DPAA2_FAEAD_UPDV: c_uint = 0x00001000;
pub const DPAA2_FAEAD_EBDDV: c_uint = 0x00002000;
pub const DPAA2_FAEAD_UPD: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_tstamp {
    pub sec_msb: u16,
    pub sec_lsb: u32,
    pub nsec: u32,
}

// Accessors for the hardware annotation fields that we use
// Error and status bits in the frame annotation status word
// Debug frame, otherwise supposed to be discarded
pub const DPAA2_FAS_DISC: c_uint = 0x80000000;
// MACSEC frame
pub const DPAA2_FAS_MS: c_uint = 0x40000000;
pub const DPAA2_FAS_PTP: c_uint = 0x08000000;
// Ethernet multicast frame
pub const DPAA2_FAS_MC: c_uint = 0x04000000;
// Ethernet broadcast frame
pub const DPAA2_FAS_BC: c_uint = 0x02000000;
pub const DPAA2_FAS_KSE: c_uint = 0x00040000;
pub const DPAA2_FAS_EOFHE: c_uint = 0x00020000;
pub const DPAA2_FAS_MNLE: c_uint = 0x00010000;
pub const DPAA2_FAS_TIDE: c_uint = 0x00008000;
pub const DPAA2_FAS_PIEE: c_uint = 0x00004000;
// Frame length error
pub const DPAA2_FAS_FLE: c_uint = 0x00002000;
// Frame physical error
pub const DPAA2_FAS_FPE: c_uint = 0x00001000;
pub const DPAA2_FAS_PTE: c_uint = 0x00000080;
pub const DPAA2_FAS_ISP: c_uint = 0x00000040;
pub const DPAA2_FAS_PHE: c_uint = 0x00000020;
pub const DPAA2_FAS_BLE: c_uint = 0x00000010;
// L3 csum validation performed
pub const DPAA2_FAS_L3CV: c_uint = 0x00000008;
// L3 csum error
pub const DPAA2_FAS_L3CE: c_uint = 0x00000004;
// L4 csum validation performed
pub const DPAA2_FAS_L4CV: c_uint = 0x00000002;
// L4 csum error
pub const DPAA2_FAS_L4CE: c_uint = 0x00000001;
// Possible errors on the ingress path

// Time in milliseconds between link state updates
pub const DPAA2_ETH_LINK_STATE_REFRESH: c_int = 1000;
// Number of times to retry a frame enqueue before giving up.
// Value determined empirically, in order to minimize the number
// of frames dropped on Tx
//
pub const DPAA2_ETH_ENQUEUE_RETRIES: c_int = 10;
// Number of times to retry DPIO portal operations while waiting
// for portal to finish executing current command and become
// available. We want to avoid being stuck in a while loop in case
// hardware becomes unresponsive, but not give up too easily if
// the portal really is busy for valid reasons
//
pub const DPAA2_ETH_SWP_BUSY_RETRIES: c_int = 1000;
// Driver statistics, other than those in struct rtnl_link_stats64.
// These are usually collected per-CPU and aggregated by ethtool.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_drv_stats {
    pub tx_conf_frames: __u64,
    pub tx_conf_bytes: __u64,
    pub tx_sg_frames: __u64,
    pub tx_sg_bytes: __u64,
    pub tx_tso_frames: __u64,
    pub tx_tso_bytes: __u64,
    pub rx_sg_frames: __u64,
    pub rx_sg_bytes: __u64,
// Linear skbs sent as a S/G FD due to insufficient headroom
    pub tx_converted_sg_frames: __u64,
    pub tx_converted_sg_bytes: __u64,
// Enqueues retried due to portal busy
    pub tx_portal_busy: __u64,
}

// Per-FQ statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_fq_stats {
// Number of frames received on this queue
    pub frames: __u64,
}

// Per-channel statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_ch_stats {
// Volatile dequeues retried due to portal busy
    pub dequeue_portal_busy: __u64,
// Pull errors
    pub pull_err: __u64,
// Number of CDANs; useful to estimate avg NAPI len
    pub cdan: __u64,
// XDP counters
    pub xdp_drop: __u64,
    pub xdp_tx: __u64,
    pub xdp_tx_err: __u64,
    pub xdp_redirect: __u64,
// Must be last, does not show up in ethtool stats
    pub frames: __u64,
    pub frames_per_cdan: __u64,
    pub bytes_per_cdan: __u64,
}

pub const DPAA2_ETH_CH_STATS: c_int = 7;
// Maximum number of queues associated with a DPNI
pub const DPAA2_ETH_MAX_TCS: c_int = 8;
pub const DPAA2_ETH_MAX_RX_QUEUES_PER_TC: c_int = 16;

pub const DPAA2_ETH_MAX_TX_QUEUES: c_int = 16;
pub const DPAA2_ETH_MAX_RX_ERR_QUEUES: c_int = 1;

pub const DPAA2_ETH_MAX_DPCONS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_eth_fq_type {
    DPAA2_RX_FQ = 0,
    DPAA2_TX_CONF_FQ,
    DPAA2_RX_ERR_FQ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_xdp_fds {
    pub fds: [dpaa2_fd; DEV_MAP_BULK_SIZE],
    pub num: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_fq {
    pub fqid: u32,
    pub tx_qdbin: u32,
    pub tx_fqid: [u32; DPAA2_ETH_MAX_TCS],
    pub flowid: u16,
    pub tc: u8,
    pub target_cpu: c_int,
    pub dq_frames: u32,
    pub dq_bytes: u32,
    pub channel: *mut dpaa2_eth_channel,
    pub type: dpaa2_eth_fq_type,
    pub consume: *mut dpaa2_eth_consume_cb_t,
    pub stats: dpaa2_eth_fq_stats,
    pub xdp_redirect_fds: dpaa2_eth_xdp_fds,
    pub xdp_tx_fds: dpaa2_eth_xdp_fds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_ch_xdp {
    pub prog: *mut bpf_prog,
    pub res: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_bp {
    pub dev: *mut fsl_mc_device,
    pub bpid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_channel {
    pub nctx: dpaa2_io_notification_ctx,
    pub dpcon: *mut fsl_mc_device,
    pub dpcon_id: c_int,
    pub ch_id: c_int,
    pub napi: napi_struct,
    pub dpio: *mut dpaa2_io,
    pub store: *mut dpaa2_io_store,
    pub priv: *mut dpaa2_eth_priv,
    pub buf_count: c_int,
    pub stats: dpaa2_eth_ch_stats,
    pub xdp: dpaa2_eth_ch_xdp,
    pub xdp_rxq: xdp_rxq_info,
    pub rx_list: *mut list_head,
// Buffers to be recycled back in the buffer pool
    pub recycled_bufs: [u64; DPAA2_ETH_BUFS_PER_CMD],
    pub recycled_bufs_cnt: c_int,
    pub xsk_zc: bool,
    pub xsk_tx_pkts_sent: c_int,
    pub xsk_pool: *mut xsk_buff_pool,
    pub bp: *mut dpaa2_eth_bp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_dist_fields {
    pub rxnfc_field: u64,
    pub cls_prot: net_prot,
    pub cls_field: c_int,
    pub size: c_int,
    pub id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_cls_rule {
    pub fs: ethtool_rx_flow_spec,
    pub in_use: u8,
}

pub const DPAA2_ETH_SGT_CACHE_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_sgt_cache {
    pub buf: [*mut c_void; DPAA2_ETH_SGT_CACHE_SIZE],
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_trap_item {
    pub trap_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_trap_data {
    pub trap_items_arr: *mut dpaa2_eth_trap_item,
    pub priv: *mut dpaa2_eth_priv,
}

pub const DPAA2_ETH_DEFAULT_COPYBREAK: c_int = 512;
pub const DPAA2_ETH_ENQUEUE_MAX_FDS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_fds {
    pub array: [dpaa2_fd; DPAA2_ETH_ENQUEUE_MAX_FDS],
}

// Driver private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_priv {
    pub net_dev: *mut net_device,
    pub num_fqs: u8,
    pub fq: [dpaa2_eth_fq; DPAA2_ETH_MAX_QUEUES],
    pub frames_enqueued): *mut c_int,
    pub num_channels: u8,
    pub channel: [*mut dpaa2_eth_channel; DPAA2_ETH_MAX_DPCONS],
    pub sgt_cache: *mut dpaa2_eth_sgt_cache __percpu,
    pub features: c_ulong,
    pub dpni_attrs: dpni_attr,
    pub dpni_ver_major: u16,
    pub dpni_ver_minor: u16,
    pub tx_data_offset: u16,
    pub onestep_reg_base: *mut void __iomem,
    pub ptp_correction_off: u8,
    pub udp): u32 offset, u8,
    pub rx_buf_size: u16,
    pub iommu_domain: *mut iommu_domain,
    pub /: *mut *mut hwtstamp_tx_types tx_tstamp_type; / Tx timestamping type,
    pub /: *mut *mut bool rx_tstamp; / Rx timestamping enabled,
// Buffer pool management
    pub bp: [*mut dpaa2_eth_bp; DPAA2_ETH_MAX_BPS],
    pub num_bps: c_int,
    pub tx_qdid: u16,
    pub mc_io: *mut fsl_mc_io,
// Cores which have an affine DPIO/DPCON.
// This is the cpu set on which Rx and Tx conf frames are processed
//
    pub dpio_cpumask: cpumask,
// Standard statistics
    pub percpu_stats: *mut rtnl_link_stats64 __percpu,
// Extra stats, in addition to the ones known by the kernel
    pub percpu_extras: *mut dpaa2_eth_drv_stats __percpu,
    pub mc_token: u16,
    pub rx_fqtd_enabled: u8,
    pub rx_cgtd_enabled: u8,
    pub link_state: dpni_link_state,
    pub do_link_poll: bool,
    pub poll_thread: *mut task_struct,
// enabled ethtool hashing bits
    pub rx_hash_fields: u64,
    pub rx_cls_fields: u64,
    pub cls_rules: *mut dpaa2_eth_cls_rule,
    pub rx_cls_enabled: u8,
    pub vlan_cls_enabled: u8,
    pub pfc_enabled: u8,

    pub dcbx_mode: u8,
    pub pfc: ieee_pfc,

    pub xdp_prog: *mut bpf_prog,

    pub dbg: dpaa2_debugfs,

    pub mac: *mut dpaa2_mac,
// Serializes changes to priv->mac
    pub mac_lock: mutex,
    pub dpaa2_ptp_wq: *mut workqueue_struct,
    pub tx_onestep_tstamp: work_struct,
    pub tx_skbs: sk_buff_head,
// The one-step timestamping configuration on hardware
// registers could only be done when no one-step
// timestamping frames are in flight. So we use a mutex
// lock here to make sure the lock is released by last
// one-step timestamping packet through TX confirmation
// queue before transmit current packet.
//
    pub onestep_tstamp_lock: mutex,
    pub devlink: *mut devlink,
    pub trap_data: *mut dpaa2_eth_trap_data,
    pub devlink_port: devlink_port,
    pub rx_copybreak: u32,
    pub fd: *mut dpaa2_eth_fds __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_eth_devlink_priv {
    pub dpaa2_priv: *mut dpaa2_eth_priv,
}

pub const TX_TSTAMP: c_uint = 0x1;
pub const TX_TSTAMP_ONESTEP_SYNC: c_uint = 0x2;

// default Rx hash options, set during probing

// Required by struct dpni_rx_tc_dist_cfg::key_cfg_iova
pub const DPAA2_CLASSIFIER_DMA_SIZE: c_int = 256;
// Minimum firmware version that supports a more flexible API
// for configuring the Rx flow hash key
//
pub const DPNI_RX_DIST_KEY_VER_MAJOR: c_int = 7;
pub const DPNI_RX_DIST_KEY_VER_MINOR: c_int = 5;

// We have exactly one {Rx, Tx conf} queue per channel

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_eth_rx_dist {
    DPAA2_ETH_RX_DIST_HASH,
    DPAA2_ETH_RX_DIST_CLS
}

// Unique IDs for the supported Rx classification header fields

pub const DPNI_PTP_ONESTEP_VER_MAJOR: c_int = 8;
pub const DPNI_PTP_ONESTEP_VER_MINOR: c_int = 2;

pub const DPNI_PAUSE_VER_MAJOR: c_int = 7;
pub const DPNI_PAUSE_VER_MINOR: c_int = 13;

// If we don't have an skb (e.g. XDP buffer), we only need space for
// the software annotation area
//
// For non-linear skbs we have no headroom requirement, as we build a
// SG frame with a newly allocated SGT buffer
//
// If we have Tx timestamping, need 128B hardware annotation
// Extra headroom space requested to hardware, in order to make sure there's
// no realloc'ing in forwarding scenarios
//
extern "C" {
    pub fn dpaa2_mac_is_type_phy(_arg: priv->mac) -> return;
}
extern "C" {
    pub fn dpaa2_eth_set_hash(net_dev: *mut net_device, flags: u64) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_set_cls(net_dev: *mut net_device, key: u64) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_cls_key_size(key: u64) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_cls_fld_off(prot: c_int, field: c_int) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_cls_trim_rule(key_mem: *mut c_void, fields: u64);
}
extern "C" {
    pub fn dpaa2_eth_dl_alloc(priv: *mut dpaa2_eth_priv) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_dl_free(priv: *mut dpaa2_eth_priv);
}
extern "C" {
    pub fn dpaa2_eth_dl_register(priv: *mut dpaa2_eth_priv);
}
extern "C" {
    pub fn dpaa2_eth_dl_unregister(priv: *mut dpaa2_eth_priv);
}
extern "C" {
    pub fn dpaa2_eth_dl_port_add(priv: *mut dpaa2_eth_priv) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_dl_port_del(priv: *mut dpaa2_eth_priv);
}
extern "C" {
    pub fn dpaa2_eth_dl_traps_register(priv: *mut dpaa2_eth_priv) -> c_int;
}
extern "C" {
    pub fn dpaa2_eth_dl_traps_unregister(priv: *mut dpaa2_eth_priv);
}
extern "C" {
    pub fn dpaa2_eth_free_dpbp(priv: *mut dpaa2_eth_priv, bp: *mut dpaa2_eth_bp);
}
extern "C" {
    pub fn dpaa2_xsk_wakeup(dev: *mut net_device, qid: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn dpaa2_xsk_setup_pool(dev: *mut net_device, pool: *mut xsk_buff_pool, qid: u16) -> c_int;
}
// SGT (Scatter-Gather Table) cache management
extern "C" {
    pub fn dpaa2_eth_sgt_recycle(priv: *mut dpaa2_eth_priv, sgt_buf: *mut c_void);
}
