//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_txrx.h
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
// Copyright (C) 2023 Intel Corporation

pub const IDPF_LARGE_MAX_Q: c_int = 256;
pub const IDPF_MAX_Q: c_int = 16;
pub const IDPF_MIN_Q: c_int = 2;
// Mailbox Queue
pub const IDPF_MAX_MBXQ: c_int = 1;
pub const IDPF_MIN_TXQ_DESC: c_int = 128;
pub const IDPF_MIN_RXQ_DESC: c_int = 64;
pub const IDPF_MIN_TXQ_COMPLQ_DESC: c_int = 256;
pub const IDPF_MAX_QIDS: c_int = 256;
// Number of descriptors in a queue should be a multiple of 32. RX queue
// descriptors alone should be a multiple of IDPF_REQ_RXQ_DESC_MULTIPLE
// to achieve BufQ descriptors aligned to 32
//
pub const IDPF_REQ_DESC_MULTIPLE: c_int = 32;

pub const IDPF_MAX_DESCS: c_int = 8160;

pub const IDPF_DFLT_SINGLEQ_TX_Q_GROUPS: c_int = 1;
pub const IDPF_DFLT_SINGLEQ_RX_Q_GROUPS: c_int = 1;
pub const IDPF_DFLT_SINGLEQ_TXQ_PER_GROUP: c_int = 4;
pub const IDPF_DFLT_SINGLEQ_RXQ_PER_GROUP: c_int = 4;
pub const IDPF_COMPLQ_PER_GROUP: c_int = 1;
pub const IDPF_SINGLE_BUFQ_PER_RXQ_GRP: c_int = 1;
pub const IDPF_MAX_BUFQS_PER_RXQ_GRP: c_int = 2;
pub const IDPF_BUFQ2_ENA: c_int = 1;
pub const IDPF_NUMQ_PER_CHUNK: c_int = 1;
pub const IDPF_DFLT_SPLITQ_TXQ_PER_GROUP: c_int = 1;
pub const IDPF_DFLT_SPLITQ_RXQ_PER_GROUP: c_int = 1;
// Default vector sharing
pub const IDPF_MBX_Q_VEC: c_int = 1;
pub const IDPF_MIN_Q_VEC: c_int = 1;
pub const IDPF_MIN_RDMA_VEC: c_int = 2;
// Data vector for NOIRQ queues
pub const IDPF_RESERVED_VECS: c_int = 1;
pub const IDPF_DFLT_TX_Q_DESC_COUNT: c_int = 512;
pub const IDPF_DFLT_TX_COMPLQ_DESC_COUNT: c_int = 512;
pub const IDPF_DFLT_RX_Q_DESC_COUNT: c_int = 512;
// IMPORTANT: We absolutely _cannot_ have more buffers in the system than a
// given RX completion queue has descriptors. This includes _ALL_ buffer
// queues. E.g.: If you have two buffer queues of 512 descriptors and buffers,
// you have a total of 1024 buffers so your RX queue _must_ have at least that
// many descriptors. This macro divides a given number of RX descriptors by
// number of buffer queues to calculate how many descriptors each buffer queue
// can have without overrunning the RX queue.
//
// If you give hardware more buffers than completion descriptors what will
// happen is that if hardware gets a chance to post more than ring wrap of
// descriptors before SW gets an interrupt and overwrites SW head, the gen bit
// in the descriptor will be wrong. Any overwritten descriptors' buffers will
// be gone forever and SW has no reasonable way to tell that this has happened.
// From SW perspective, when we finally get an interrupt, it looks like we're
// still waiting for descriptor to be done, stalling forever.
//

pub const IDPF_RX_BUF_STRIDE: c_int = 32;
pub const IDPF_RX_BUF_POST_STRIDE: c_int = 16;
pub const IDPF_LOW_WATERMARK: c_int = 64;
pub const IDPF_TX_TSO_MIN_MSS: c_int = 88;
// Minimum number of descriptors between 2 descriptors with the RE bit set;
// only relevant in flow scheduling mode
//
pub const IDPF_TX_SPLITQ_RE_MIN_GAP: c_int = 64;

// Determine the absolute number of completions pending, i.e. the number of
// completions that are expected to arrive on the TX completion queue.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union idpf_tx_flex_desc {
    pub /: *mut *mut idpf_flex_tx_desc q; / queue based scheduling,
    pub /: *mut *mut idpf_flex_tx_sched_desc flow; / flow based scheduling,
}

//
// struct idpf_tx_offload_params - Offload parameters for a given packet
// @tx_flags: Feature flags enabled for this packet
// @hdr_offsets: Offset parameter for single queue model
// @cd_tunneling: Type of tunneling enabled for single queue model
// @tso_len: Total length of payload to segment
// @mss: Segment size
// @tso_segs: Number of segments to be sent
// @tso_hdr_len: Length of headers to be duplicated
// @td_cmd: Command field to be inserted into descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_tx_offload_params {
    pub tx_flags: u32,
    pub hdr_offsets: u32,
    pub cd_tunneling: u32,
    pub tso_len: u32,
    pub mss: u16,
    pub tso_segs: u16,
    pub tso_hdr_len: u16,
    pub td_cmd: u16,
}

//
// struct idpf_tx_splitq_params
// @dtype: General descriptor info
// @eop_cmd: Type of EOP
// @compl_tag: Associated tag for completion
// @td_tag: Descriptor tunneling tag
// @offload: Offload parameters
// @prev_ntu: stored TxQ next_to_use in case of rollback
// @prev_refill_ntc: stored refillq next_to_clean in case of packet rollback
// @prev_refill_gen: stored refillq generation bit in case of packet rollback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_tx_splitq_params {
    pub dtype: idpf_tx_desc_dtype_value,
    pub eop_cmd: u16,
    pub compl_tag: u16,
    pub td_tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_ctx_desc_eipt_offload {
    IDPF_TX_CTX_EXT_IP_NONE         = 0x0,
    IDPF_TX_CTX_EXT_IP_IPV6         = 0x1,
    IDPF_TX_CTX_EXT_IP_IPV4_NO_CSUM = 0x2,
    IDPF_TX_CTX_EXT_IP_IPV4         = 0x3
}

pub const IDPF_TX_COMPLQ_CLEAN_BUDGET: c_int = 256;
pub const IDPF_TX_MIN_PKT_LEN: c_int = 17;
pub const IDPF_TX_DESCS_FOR_SKB_DATA_PTR: c_int = 1;

pub const IDPF_TX_DESCS_FOR_CTX: c_int = 1;
// TX descriptors needed, worst case

// The size limit for a transmit buffer in a descriptor is (16K - 1).
// In order to align with the read requests we will align the value to
// the nearest 4K which represents our maximum read request size.
//

pub const IDPF_RX_MAX_PTYPE_PROTO_IDS: c_int = 32;

pub const IDPF_RX_MAX_PTYPE: c_int = 1024;
pub const IDPF_RX_MAX_BASE_PTYPE: c_int = 256;
pub const IDPF_INVALID_PTYPE_ID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tunnel_state {
    IDPF_PTYPE_TUNNEL_IP                    = BIT(0),
    IDPF_PTYPE_TUNNEL_IP_GRENAT             = BIT(1),
    IDPF_PTYPE_TUNNEL_IP_GRENAT_MAC         = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptype_state {
    pub outer_ip:1: bool,
    pub outer_frag:1: bool,
    pub tunnel_state:6: u8,
}

//
// enum idpf_queue_flags_t
// @__IDPF_Q_GEN_CHK: Queues operating in splitq mode use a generation bit to
// identify new descriptor writebacks on the ring. HW sets
// the gen bit to 1 on the first writeback of any given
// descriptor. After the ring wraps, HW sets the gen bit of
// those descriptors to 0, and continues flipping
// 0->1 or 1->0 on each ring wrap. SW maintains its own
// gen bit to know what value will indicate writebacks on
// the next pass around the ring. E.g. it is initialized
// to 1 and knows that reading a gen bit of 1 in any
// descriptor on the initial pass of the ring indicates a
// writeback. It also flips on every ring wrap.
// @__IDPF_Q_RFL_GEN_CHK: Refill queues are SW only, so Q_GEN acts as the HW
// bit and Q_RFL_GEN is the SW bit.
// @__IDPF_Q_FLOW_SCH_EN: Enable flow scheduling
// @__IDPF_Q_SW_MARKER: Used to indicate TX queue marker completions
// @__IDPF_Q_CRC_EN: enable CRC offload in singleq mode
// @__IDPF_Q_RSC_EN: enable Receive Side Coalescing on Rx (splitq)
// @__IDPF_Q_HSPLIT_EN: enable header split on Rx (splitq)
// @__IDPF_Q_PTP: indicates whether the Rx timestamping is enabled for the
// queue
// @__IDPF_Q_NOIRQ: queue is polling-driven and has no interrupt
// @__IDPF_Q_XDP: this is an XDP queue
// @__IDPF_Q_XSK: the queue has an XSk pool installed
// @__IDPF_Q_FLAGS_NBITS: Must be last
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_queue_flags_t {
    __IDPF_Q_GEN_CHK,
    __IDPF_Q_RFL_GEN_CHK,
    __IDPF_Q_FLOW_SCH_EN,
    __IDPF_Q_SW_MARKER,
    __IDPF_Q_CRC_EN,
    __IDPF_Q_RSC_EN,
    __IDPF_Q_HSPLIT_EN,
    __IDPF_Q_PTP,
    __IDPF_Q_NOIRQ,
    __IDPF_Q_XDP,
    __IDPF_Q_XSK,

    __IDPF_Q_FLAGS_NBITS,
}

//
// struct idpf_vec_regs
// @dyn_ctl_reg: Dynamic control interrupt register offset
// @itrn_reg: Interrupt Throttling Rate register offset
// @itrn_index_spacing: Register spacing between ITR registers of the same
// vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vec_regs {
    pub dyn_ctl_reg: u32,
    pub itrn_reg: u32,
    pub itrn_index_spacing: u32,
}

//
// struct idpf_intr_reg
// @dyn_ctl: Dynamic control interrupt register
// @dyn_ctl_intena_m: Mask for dyn_ctl interrupt enable
// @dyn_ctl_intena_msk_m: Mask for dyn_ctl interrupt enable mask
// @dyn_ctl_itridx_s: Register bit offset for ITR index
// @dyn_ctl_itridx_m: Mask for ITR index
// @dyn_ctl_intrvl_s: Register bit offset for ITR interval
// @dyn_ctl_wb_on_itr_m: Mask for WB on ITR feature
// @dyn_ctl_sw_itridx_ena_m: Mask for SW ITR index
// @dyn_ctl_swint_trig_m: Mask for dyn_ctl SW triggered interrupt enable
// @rx_itr: RX ITR register
// @tx_itr: TX ITR register
// @icr_ena: Interrupt cause register offset
// @icr_ena_ctlq_m: Mask for ICR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_intr_reg {
    pub dyn_ctl: *mut void __iomem,
    pub dyn_ctl_intena_m: u32,
    pub dyn_ctl_intena_msk_m: u32,
    pub dyn_ctl_itridx_s: u32,
    pub dyn_ctl_itridx_m: u32,
    pub dyn_ctl_intrvl_s: u32,
    pub dyn_ctl_wb_on_itr_m: u32,
    pub dyn_ctl_sw_itridx_ena_m: u32,
    pub dyn_ctl_swint_trig_m: u32,
    pub rx_itr: *mut void __iomem,
    pub tx_itr: *mut void __iomem,
    pub icr_ena: *mut void __iomem,
    pub icr_ena_ctlq_m: u32,
}

//
// struct idpf_q_vector
// @vport: Vport back pointer
// @num_rxq: Number of RX queues
// @num_txq: Number of TX queues
// @num_bufq: Number of buffer queues
// @num_complq: number of completion queues
// @num_xsksq: number of XSk send queues
// @rx: Array of RX queues to service
// @tx: Array of TX queues to service
// @bufq: Array of buffer queues to service
// @complq: array of completion queues
// @xsksq: array of XSk send queues
// @intr_reg: See struct idpf_intr_reg
// @csd: XSk wakeup CSD
// @total_events: Number of interrupts processed
// @wb_on_itr: whether WB on ITR is enabled
// @napi: napi handler
// @tx_dim: Data for TX net_dim algorithm
// @tx_itr_value: TX interrupt throttling rate
// @tx_intr_mode: Dynamic ITR or not
// @tx_itr_idx: TX ITR index
// @rx_dim: Data for RX net_dim algorithm
// @rx_itr_value: RX interrupt throttling rate
// @rx_intr_mode: Dynamic ITR or not
// @rx_itr_idx: RX ITR index
// @v_idx: Vector index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_q_vector {
    pub vport: *mut idpf_vport,
    pub num_rxq: u16,
    pub num_txq: u16,
    pub num_bufq: u16,
    pub num_complq: u16,
    pub num_xsksq: u16,
    pub rx: *mut idpf_rx_queue,
    pub tx: *mut idpf_tx_queue,
    pub bufq: *mut idpf_buf_queue,
    pub complq: *mut idpf_compl_queue,
    pub xsksq: *mut idpf_tx_queue,
    pub intr_reg: idpf_intr_reg,
    pub csd: call_single_data_t,
    pub total_events: u16,
    pub wb_on_itr: bool,
    pub napi: napi_struct,
    pub tx_dim: dim,
    pub tx_itr_value: u16,
    pub tx_intr_mode: bool,
    pub tx_itr_idx: u32,
    pub rx_dim: dim,
    pub rx_itr_value: u16,
    pub rx_intr_mode: bool,
    pub rx_itr_idx: u32,
    pub v_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_rx_queue_stats {
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub rsc_pkts: u64_stats_t,
    pub hw_csum_err: u64_stats_t,
    pub hsplit_pkts: u64_stats_t,
    pub hsplit_buf_ovf: u64_stats_t,
    pub bad_descs: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_tx_queue_stats {
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub lso_pkts: u64_stats_t,
    pub linearize: u64_stats_t,
    pub q_busy: u64_stats_t,
    pub skb_drops: u64_stats_t,
    pub dma_map_errs: u64_stats_t,
    pub tstamp_skipped: u64_stats_t,
}

pub const IDPF_ITR_DYNAMIC: c_int = 1;
pub const IDPF_ITR_MAX: c_uint = 0x1FE0;
pub const IDPF_ITR_20K: c_uint = 0x0032;

pub const IDPF_ITR_MASK: c_uint = 0x1FFE  /* ITR register value alignment mask */;

// Index used for 'SW ITR' update in DYN_CTL register
pub const IDPF_SW_ITR_UPDATE_IDX: c_int = 2;
// Index used for 'No ITR' update in DYN_CTL register
pub const IDPF_NO_ITR_UPDATE_IDX: c_int = 3;

pub const IDPF_DIM_DEFAULT_PROFILE_IX: c_int = 1;
//
// struct idpf_rx_queue - software structure representing a receive queue
// @rx: universal receive descriptor array
// @single_buf: buffer descriptor array in singleq
// @desc_ring: virtual descriptor ring address
// @bufq_sets: Pointer to the array of buffer queues in splitq mode
// @napi: NAPI instance corresponding to this queue (splitq)
// @xdp_prog: attached XDP program
// @rx_buf: See struct &libeth_fqe
// @pp: Page pool pointer in singleq mode
// @tail: Tail offset. Used for both queue models single and split.
// @flags: See enum idpf_queue_flags_t
// @idx: For RX queue, it is used to index to total RX queue across groups and
// used for skb reporting.
// @desc_count: Number of descriptors
// @num_xdp_txq: total number of XDP Tx queues
// @xdpsqs: shortcut for XDP Tx queues array
// @rxdids: Supported RX descriptor ids
// @truesize: data buffer truesize in singleq
// @rx_ptype_lkup: LUT of Rx ptypes
// @xdp_rxq: XDP queue info
// @next_to_use: Next descriptor to use
// @next_to_clean: Next descriptor to clean
// @next_to_alloc: RX buffer to allocate at
// @xdp: XDP buffer with the current frame
// @xsk: current XDP buffer in XSk mode
// @pool: XSk pool if installed
// @cached_phc_time: Cached PHC time for the Rx queue
// @stats_sync: See struct u64_stats_sync
// @q_stats: See union idpf_rx_queue_stats
// @q_id: Queue id
// @size: Length of descriptor ring in bytes
// @dma: Physical address of ring
// @q_vector: Backreference to associated vector
// @rx_buffer_low_watermark: RX buffer low watermark
// @rx_hbuf_size: Header buffer size
// @rx_buf_size: Buffer size
// @rx_max_pkt_size: RX max packet size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_rx_queue {
    pub rx: *mut virtchnl2_rx_desc,
    pub single_buf: *mut virtchnl2_singleq_rx_buf_desc,
    pub desc_ring: *mut c_void,
}

//
// struct idpf_tx_queue - software structure representing a transmit queue
// @base_tx: base Tx descriptor array
// @base_ctx: base Tx context descriptor array
// @flex_tx: flex Tx descriptor array
// @flex_ctx: flex Tx context descriptor array
// @desc_ring: virtual descriptor ring address
// @tx_buf: See struct idpf_tx_buf
// @txq_grp: See struct idpf_txq_group
// @complq: corresponding completion queue in XDP mode
// @dev: Device back pointer for DMA mapping
// @pool: corresponding XSk pool if installed
// @tail: Tail offset. Used for both queue models single and split
// @flags: See enum idpf_queue_flags_t
// @idx: For TX queue, it is used as index to map between TX queue group and
// hot path TX pointers stored in vport. Used in both singleq/splitq.
// @desc_count: Number of descriptors
// @tx_min_pkt_len: Min supported packet length
// @thresh: XDP queue cleaning threshold
// @netdev: &net_device corresponding to this queue
// @next_to_use: Next descriptor to use
// @next_to_clean: Next descriptor to clean
// @last_re: last descriptor index that RE bit was set
// @tx_max_bufs: Max buffers that can be transmitted with scatter-gather
// @cleaned_bytes: Splitq only, TXQ only: When a TX completion is received on
// the TX completion queue, it can be for any TXQ associated
// with that completion queue. This means we can clean up to
// N TXQs during a single call to clean the completion queue.
// cleaned_bytes|pkts tracks the clean stats per TXQ during
// that single call to clean the completion queue. By doing so,
// we can update BQL with aggregate cleaned stats for each TXQ
// only once at the end of the cleaning routine.
// @clean_budget: singleq only, queue cleaning budget
// @cleaned_pkts: Number of packets cleaned for the above said case
// @refillq: Pointer to refill queue
// @pending: number of pending descriptors to send in QB
// @xdp_tx: number of pending &xdp_buff or &xdp_frame buffers
// @timer: timer for XDP Tx queue cleanup
// @xdp_lock: lock for XDP Tx queues sharing
// @cached_tstamp_caps: Tx timestamp capabilities negotiated with the CP
// @tstamp_task: Work that handles Tx timestamp read
// @stats_sync: See struct u64_stats_sync
// @q_stats: See union idpf_tx_queue_stats
// @q_id: Queue id
// @size: Length of descriptor ring in bytes
// @dma: Physical address of ring
// @q_vector: Backreference to associated vector
// @buf_pool_size: Total number of idpf_tx_buf
// @rel_q_id: relative virtchnl queue index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_tx_queue {
    pub base_tx: *mut idpf_base_tx_desc,
    pub base_ctx: *mut idpf_base_tx_ctx_desc,
    pub flex_tx: *mut idpf_tx_flex_desc,
    pub flex_ctx: *mut idpf_flex_tx_ctx_desc,
    pub desc_ring: *mut c_void,
}

//
// struct idpf_buf_queue - software structure representing a buffer queue
// @split_buf: buffer descriptor array
// @buf: &libeth_fqe for data buffers
// @pp: &page_pool for data buffers
// @xsk_buf: &xdp_buff for XSk Rx buffers
// @pool: &xsk_buff_pool on XSk queues
// @hdr_buf: &libeth_fqe for header buffers
// @hdr_pp: &page_pool for header buffers
// @tail: Tail offset
// @flags: See enum idpf_queue_flags_t
// @desc_count: Number of descriptors
// @thresh: refill threshold in XSk
// @next_to_use: Next descriptor to use
// @next_to_clean: Next descriptor to clean
// @next_to_alloc: RX buffer to allocate at
// @pending: number of buffers to refill (Xsk)
// @hdr_truesize: truesize for buffer headers
// @truesize: truesize for data buffers
// @q_id: Queue id
// @size: Length of descriptor ring in bytes
// @dma: Physical address of ring
// @q_vector: Backreference to associated vector
// @rx_buffer_low_watermark: RX buffer low watermark
// @rx_hbuf_size: Header buffer size
// @rx_buf_size: Buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_buf_queue {
    pub split_buf: *mut virtchnl2_splitq_rx_buf_desc,
    pub buf: *mut libeth_fqe,
    pub pp: *mut page_pool,
}

//
// struct idpf_compl_queue - software structure representing a completion queue
// @comp: 8-byte completion descriptor array
// @comp_4b: 4-byte completion descriptor array
// @desc_ring: virtual descriptor ring address
// @txq_grp: See struct idpf_txq_group
// @flags: See enum idpf_queue_flags_t
// @desc_count: Number of descriptors
// @clean_budget: queue cleaning budget
// @netdev: &net_device corresponding to this queue
// @next_to_use: Next descriptor to use. Relevant in both split & single txq
// and bufq.
// @next_to_clean: Next descriptor to clean
// @num_completions: Only relevant for TX completion queue. It tracks the
// number of completions received to compare against the
// number of completions pending, as accumulated by the
// TX queues.
// @q_id: Queue id
// @size: Length of descriptor ring in bytes
// @dma: Physical address of ring
// @q_vector: Backreference to associated vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_compl_queue {
    pub comp: *mut idpf_splitq_tx_compl_desc,
    pub comp_4b: *mut idpf_splitq_4b_tx_compl_desc,
    pub desc_ring: *mut c_void,
}

//
// struct idpf_sw_queue
// @ring: Pointer to the ring
// @flags: See enum idpf_queue_flags_t
// @desc_count: Descriptor count
// @next_to_use: Buffer to allocate at
// @next_to_clean: Next descriptor to clean
//
// Software queues are used in splitq mode to manage buffers between rxq
// producer and the bufq consumer.  These are required in order to maintain a
// lockless buffer management system and are strictly software only constructs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_sw_queue {
    pub ring: *mut u32,
    pub __IDPF_Q_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub desc_count: u32,
    pub next_to_use: u32,
    pub next_to_clean: u32,
}

//
// struct idpf_rxq_set
// @rxq: RX queue
// @refillq: pointers to refill queues
//
// Splitq only.  idpf_rxq_set associates an rxq with at an array of refillqs.
// Each rxq needs a refillq to return used buffers back to the respective bufq.
// Bufqs then clean these refillqs for buffers to give to hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_rxq_set {
    pub rxq: idpf_rx_queue,
    pub refillq: [*mut idpf_sw_queue; IDPF_MAX_BUFQS_PER_RXQ_GRP],
}

//
// struct idpf_bufq_set
// @bufq: Buffer queue
// @num_refillqs: Number of refill queues. This is always equal to num_rxq_sets
// in idpf_rxq_group.
// @refillqs: Pointer to refill queues array.
//
// Splitq only. idpf_bufq_set associates a bufq to an array of refillqs.
// In this bufq_set, there will be one refillq for each rxq in this rxq_group.
// Used buffers received by rxqs will be put on refillqs which bufqs will
// clean to return new buffers back to hardware.
//
// Buffers needed by some number of rxqs associated in this rxq_group are
// managed by at most two bufqs (depending on performance configuration).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_bufq_set {
    pub bufq: idpf_buf_queue,
    pub num_refillqs: c_int,
    pub refillqs: *mut idpf_sw_queue,
}

//
// struct idpf_rxq_group
// @vport: Vport back pointer
// @singleq: Struct with single queue related members
// @singleq.num_rxq: Number of RX queues associated
// @singleq.rxqs: Array of RX queue pointers
// @splitq: Struct with split queue related members
// @splitq.num_rxq_sets: Number of RX queue sets
// @splitq.num_rxq_sets: Number of Buffer queue sets
// @splitq.rxq_sets: Array of RX queue sets
// @splitq.bufq_sets: Buffer queue set pointer
//
// In singleq mode, an rxq_group is simply an array of rxqs.  In splitq, a
// rxq_group contains all the rxqs, bufqs and refillqs needed to
// manage buffers in splitq mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_rxq_group {
    pub vport: *mut idpf_vport,
    pub num_rxq: u16,
    pub rxqs: [*mut idpf_rx_queue; IDPF_LARGE_MAX_Q],
    pub singleq: },
    pub num_rxq_sets: u16,
    pub num_bufq_sets: u16,
    pub rxq_sets: [*mut idpf_rxq_set; IDPF_LARGE_MAX_Q],
    pub bufq_sets: *mut idpf_bufq_set,
    pub splitq: },
}

//
// struct idpf_txq_group
// @vport: Vport back pointer
// @num_txq: Number of TX queues associated
// @txqs: Array of TX queue pointers
// @complq: Associated completion queue pointer, split queue only
// @num_completions_pending: Total number of completions pending for the
// completion queue, acculumated for all TX queues
// associated with that completion queue.
//
// Between singleq and splitq, a txq_group is largely the same except for the
// complq. In splitq a single complq is responsible for handling completions
// for some number of txqs associated in this txq_group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_txq_group {
    pub vport: *mut idpf_vport,
    pub num_txq: u16,
    pub txqs: [*mut idpf_tx_queue; IDPF_LARGE_MAX_Q],
    pub complq: *mut idpf_compl_queue,
    pub num_completions_pending: aligned_u64,
}

//
// idpf_size_to_txd_count - Get number of descriptors needed for large Tx frag
// @size: transmit request size in bytes
//
// In the case where a large frag (>= 16K) needs to be split across multiple
// descriptors, we need to assume that we can have no more than 12K of data
// per descriptor due to hardware alignment restrictions (4K alignment).
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: size, _arg: IDPF_TX_MAX_DESC_DATA_ALIGNED) -> return;
}
//
// idpf_tx_singleq_build_ctob - populate command tag offset and size
// @td_cmd: Command to be filled in desc
// @td_offset: Offset to be filled in desc
// @size: Size of the buffer
// @td_tag: td tag to be filled
//
// Returns the 64 bit value populated with the input parameters
//
// idpf_tx_splitq_build_desc - determine which type of data descriptor to build
// @desc: descriptor to populate
// @params: pointer to tx params struct
// @td_cmd: command to be filled in desc
// @size: size of buffer
//
// idpf_vport_intr_set_wb_on_itr - enable descriptor writeback on disabled interrupts
// @q_vector: pointer to queue vector struct
//
// idpf_tx_splitq_get_free_bufs - get number of free buf_ids in refillq
// @refillq: pointer to refillq containing buf_ids
//
extern "C" {
    pub fn idpf_vport_singleq_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn idpf_vport_calc_num_q_groups(rsrc: *mut idpf_q_vec_rsrc);
}
extern "C" {
    pub fn idpf_vport_intr_rel(rsrc: *mut idpf_q_vec_rsrc);
}
extern "C" {
    pub fn idpf_vport_intr_update_itr_ena_irq(q_vector: *mut idpf_q_vector);
}
extern "C" {
    pub fn idpf_config_rss(vport: *mut idpf_vport, rss_data: *mut idpf_rss_data) -> c_int;
}
extern "C" {
    pub fn idpf_init_rss_lut(vport: *mut idpf_vport, rss_data: *mut idpf_rss_data) -> c_int;
}
extern "C" {
    pub fn idpf_deinit_rss_lut(rss_data: *mut idpf_rss_data);
}
extern "C" {
    pub fn idpf_qp_switch(vport: *mut idpf_vport, qid: u32, en: bool) -> c_int;
}
extern "C" {
    pub fn idpf_size_to_txd_count(size: c_uint) -> c_uint;
}
extern "C" {
    pub fn idpf_tx_drop_skb(tx_q: *mut idpf_tx_queue, skb: *mut sk_buff) -> netdev_tx_t;
}
extern "C" {
    pub fn idpf_tx_timeout(netdev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn idpf_tx_start(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn idpf_tso(skb: *mut sk_buff, off: *mut idpf_tx_offload_params) -> c_int;
}
extern "C" {
    pub fn idpf_wait_for_sw_marker_completion(txq: *const idpf_tx_queue);
}
