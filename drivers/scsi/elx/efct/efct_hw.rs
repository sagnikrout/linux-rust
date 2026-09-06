//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_hw.h
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
//
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

//
// EFCT PCI IDs
//
pub const EFCT_VENDOR_ID: c_uint = 0x10df;
// LightPulse 16Gb x 4 FC (lancer-g6)
pub const EFCT_DEVICE_LANCER_G6: c_uint = 0xe307;
// LightPulse 32Gb x 4 FC (lancer-g7)
pub const EFCT_DEVICE_LANCER_G7: c_uint = 0xf407;
// Default RQ entries len used by driver
pub const EFCT_HW_RQ_ENTRIES_MIN: c_int = 512;
pub const EFCT_HW_RQ_ENTRIES_DEF: c_int = 1024;
pub const EFCT_HW_RQ_ENTRIES_MAX: c_int = 4096;
// Defines the size of the RQ buffers used for each RQ
pub const EFCT_HW_RQ_SIZE_HDR: c_int = 128;
pub const EFCT_HW_RQ_SIZE_PAYLOAD: c_int = 1024;
// Define the maximum number of multi-receive queues
pub const EFCT_HW_MAX_MRQS: c_int = 8;
//
// Define count of when to set the WQEC bit in a submitted
// WQE, causing a consummed/released completion to be posted.
//
pub const EFCT_HW_WQEC_SET_COUNT: c_int = 32;
// Send frame timeout in seconds
pub const EFCT_HW_SEND_FRAME_TIMEOUT: c_int = 10;
//
// FDT Transfer Hint value, reads greater than this value
// will be segmented to implement fairness. A value of zero disables
// the feature.
//
pub const EFCT_HW_FDT_XFER_HINT: c_int = 8192;
pub const EFCT_HW_TIMECHECK_ITERATIONS: c_int = 100;
pub const EFCT_HW_MAX_NUM_MQ: c_int = 1;
pub const EFCT_HW_MAX_NUM_RQ: c_int = 32;
pub const EFCT_HW_MAX_NUM_EQ: c_int = 16;
pub const EFCT_HW_MAX_NUM_WQ: c_int = 32;
pub const EFCT_HW_DEF_NUM_EQ: c_int = 1;
pub const OCE_HW_MAX_NUM_MRQ_PAIRS: c_int = 16;
pub const EFCT_HW_MQ_DEPTH: c_int = 128;
pub const EFCT_HW_EQ_DEPTH: c_int = 1024;
//
// A CQ will be assigned to each WQ
// (CQ must have 2X entries of the WQ for abort
// processing), plus a separate one for each RQ PAIR and one for MQ
//

pub const EFCT_HW_Q_HASH_SIZE: c_int = 128;
pub const EFCT_HW_RQ_HEADER_SIZE: c_int = 128;
pub const EFCT_HW_RQ_HEADER_INDEX: c_int = 0;
pub const EFCT_HW_REQUE_XRI_REGTAG: c_int = 65534;
// Options for efct_hw_command()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_cmd_opts {
// command executes synchronously and busy-waits for completion
    EFCT_CMD_POLL,
// command executes asynchronously. Uses callback
    EFCT_CMD_NOWAIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_reset {
    EFCT_HW_RESET_FUNCTION,
    EFCT_HW_RESET_FIRMWARE,
    EFCT_HW_RESET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_topo {
    EFCT_HW_TOPOLOGY_AUTO,
    EFCT_HW_TOPOLOGY_NPORT,
    EFCT_HW_TOPOLOGY_LOOP,
    EFCT_HW_TOPOLOGY_NONE,
    EFCT_HW_TOPOLOGY_MAX
}

// pack fw revision values into a single uint64_t

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_io_type {
    EFCT_HW_ELS_REQ,
    EFCT_HW_ELS_RSP,
    EFCT_HW_FC_CT,
    EFCT_HW_FC_CT_RSP,
    EFCT_HW_BLS_ACC,
    EFCT_HW_BLS_RJT,
    EFCT_HW_IO_TARGET_READ,
    EFCT_HW_IO_TARGET_WRITE,
    EFCT_HW_IO_TARGET_RSP,
    EFCT_HW_IO_DNRX_REQUEUE,
    EFCT_HW_IO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_io_state {
    EFCT_HW_IO_STATE_FREE,
    EFCT_HW_IO_STATE_INUSE,
    EFCT_HW_IO_STATE_WAIT_FREE,
    EFCT_HW_IO_STATE_WAIT_SEC_HIO,
}

pub const EFCT_TARGET_WRITE_SKIPS: c_int = 1;
pub const EFCT_TARGET_READ_SKIPS: c_int = 2;
pub const EFCT_CMD_CTX_POOL_SZ: c_int = 32;
//
// HW command context.
// Stores the state for the asynchronous commands sent to the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_command_ctx {
    pub list_entry: list_head,
    pub arg): *mut *mut *mut *mut int (cb)(struct efct_hw hw, int status, u8 mqe, void,
    pub /: *mut *mut *mut void arg; / Argument for callback,
// buffer holding command / results
    pub buf: [u8; SLI4_BMBX_SIZE],
    pub /: *mut *mut *mut void ctx; / upper layer context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_sgl {
    pub addr: uintptr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union efct_hw_io_param_u {
    pub bls: sli_bls_params,
    pub els: sli_els_params,
    pub fc_ct: sli_ct_params,
    pub fcp_tgt: sli_fcp_tgt_params,
}

// WQ steering mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_wq_steering {
    EFCT_HW_WQ_STEERING_CLASS,
    EFCT_HW_WQ_STEERING_REQUEST,
    EFCT_HW_WQ_STEERING_CPU,
}

// HW wqe object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_wqe {
    pub list_entry: list_head,
    pub abort_wqe_submit_needed: bool,
    pub send_abts: bool,
    pub id: u32,
    pub abort_reqtag: u32,
    pub wqebuf: *mut u8,
}

// Typedef for HW "done" callback
//
// HW IO object.
//
// Stores the per-IO information necessary
// for both SLI and efct.
// @ref:		reference counter for hw io object
// @state:		state of IO: free, busy, wait_free
// @list_entry		used for busy, wait_free, free lists
// @wqe			Work queue object, with link for pending
// @hw			pointer back to hardware context
// @xfer_rdy		transfer ready data
// @type		IO type
// @xbusy		Exchange is active in FW
// @abort_in_progress	if TRUE, abort is in progress
// @status_saved	if TRUE, latched status should be returned
// @wq_class		WQ class if steering mode is Class
// @reqtag		request tag for this HW IO
// @wq			WQ assigned to the exchange
// @done		Function called on IO completion
// @arg			argument passed to IO done callback
// @abort_done		Function called on abort completion
// @abort_arg		argument passed to abort done callback
// @wq_steering		WQ steering mode request
// @saved_status	Saved status
// @saved_len		Status length
// @saved_ext		Saved extended status
// @eq			EQ on which this HIO came up
// @sge_offset		SGE data offset
// @def_sgl_count	Count of SGEs in default SGL
// @abort_reqtag	request tag for an abort of this HW IO
// @indicator		Exchange indicator
// @def_sgl		default SGL
// @sgl			pointer to current active SGL
// @sgl_count		count of SGEs in io->sgl
// @first_data_sge	index of first data SGE
// @n_sge		number of active SGEs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_io {
    pub ref: kref,
    pub state: efct_hw_io_state,
    pub arg): *mut *mut void (release)(struct kref,
    pub list_entry: list_head,
    pub wqe: efct_hw_wqe,
    pub hw: *mut efct_hw,
    pub xfer_rdy: efc_dma,
    pub type: u16,
    pub xbusy: bool,
    pub abort_in_progress: c_int,
    pub status_saved: bool,
    pub wq_class: u8,
    pub reqtag: u16,
    pub wq: *mut hw_wq,
    pub done: efct_hw_done_t,
    pub arg: *mut c_void,
    pub abort_done: efct_hw_done_t,
    pub abort_arg: *mut c_void,
    pub wq_steering: efct_hw_wq_steering,
    pub saved_status: u32,
    pub saved_len: u32,
    pub saved_ext: u32,
    pub eq: *mut hw_eq,
    pub sge_offset: u32,
    pub def_sgl_count: u32,
    pub abort_reqtag: u32,
    pub indicator: u32,
    pub def_sgl: efc_dma,
    pub sgl: *mut efc_dma,
    pub sgl_count: u32,
    pub first_data_sge: u32,
    pub n_sge: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_port {
    EFCT_HW_PORT_INIT,
    EFCT_HW_PORT_SHUTDOWN,
}

// Node group rpi reference
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_rpi_ref {
    pub rpi_count: core::sync::atomic::AtomicI32,
    pub rpi_attached: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_link_stat {
    EFCT_HW_LINK_STAT_LINK_FAILURE_COUNT,
    EFCT_HW_LINK_STAT_LOSS_OF_SYNC_COUNT,
    EFCT_HW_LINK_STAT_LOSS_OF_SIGNAL_COUNT,
    EFCT_HW_LINK_STAT_PRIMITIVE_SEQ_COUNT,
    EFCT_HW_LINK_STAT_INVALID_XMIT_WORD_COUNT,
    EFCT_HW_LINK_STAT_CRC_COUNT,
    EFCT_HW_LINK_STAT_PRIMITIVE_SEQ_TIMEOUT_COUNT,
    EFCT_HW_LINK_STAT_ELASTIC_BUFFER_OVERRUN_COUNT,
    EFCT_HW_LINK_STAT_ARB_TIMEOUT_COUNT,
    EFCT_HW_LINK_STAT_ADVERTISED_RCV_B2B_CREDIT,
    EFCT_HW_LINK_STAT_CURR_RCV_B2B_CREDIT,
    EFCT_HW_LINK_STAT_ADVERTISED_XMIT_B2B_CREDIT,
    EFCT_HW_LINK_STAT_CURR_XMIT_B2B_CREDIT,
    EFCT_HW_LINK_STAT_RCV_EOFA_COUNT,
    EFCT_HW_LINK_STAT_RCV_EOFDTI_COUNT,
    EFCT_HW_LINK_STAT_RCV_EOFNI_COUNT,
    EFCT_HW_LINK_STAT_RCV_SOFF_COUNT,
    EFCT_HW_LINK_STAT_RCV_DROPPED_NO_AER_COUNT,
    EFCT_HW_LINK_STAT_RCV_DROPPED_NO_RPI_COUNT,
    EFCT_HW_LINK_STAT_RCV_DROPPED_NO_XRI_COUNT,
    EFCT_HW_LINK_STAT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_host_stat {
    EFCT_HW_HOST_STAT_TX_KBYTE_COUNT,
    EFCT_HW_HOST_STAT_RX_KBYTE_COUNT,
    EFCT_HW_HOST_STAT_TX_FRAME_COUNT,
    EFCT_HW_HOST_STAT_RX_FRAME_COUNT,
    EFCT_HW_HOST_STAT_TX_SEQ_COUNT,
    EFCT_HW_HOST_STAT_RX_SEQ_COUNT,
    EFCT_HW_HOST_STAT_TOTAL_EXCH_ORIG,
    EFCT_HW_HOST_STAT_TOTAL_EXCH_RESP,
    EFCT_HW_HOSY_STAT_RX_P_BSY_COUNT,
    EFCT_HW_HOST_STAT_RX_F_BSY_COUNT,
    EFCT_HW_HOST_STAT_DROP_FRM_DUE_TO_NO_RQ_BUF_COUNT,
    EFCT_HW_HOST_STAT_EMPTY_RQ_TIMEOUT_COUNT,
    EFCT_HW_HOST_STAT_DROP_FRM_DUE_TO_NO_XRI_COUNT,
    EFCT_HW_HOST_STAT_EMPTY_XRI_POOL_COUNT,
    EFCT_HW_HOST_STAT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_state {
    EFCT_HW_STATE_UNINITIALIZED,
    EFCT_HW_STATE_QUEUES_ALLOCATED,
    EFCT_HW_STATE_ACTIVE,
    EFCT_HW_STATE_RESET_IN_PROGRESS,
    EFCT_HW_STATE_TEARDOWN_IN_PROGRESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_link_stat_counts {
    pub overflow: u8,
    pub counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_host_stat_counts {
    pub counter: u32,
}

// Structure used for the hash lookup of queue IDs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_queue_hash {
    pub in_use: bool,
    pub id: u16,
    pub index: u16,
}

// WQ callback object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_wq_callback {
    pub /: *mut *mut u16 instance_index; / use for request tag,
    pub status): *mut *mut *mut *mut void (callback)(void arg, u8 cqe, int,
    pub arg: *mut c_void,
    pub list_entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reqtag_pool {
    pub /: *mut *mut spinlock_t lock; / pool lock,
    pub tags: [*mut hw_wq_callback; U16_MAX],
    pub freelist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_config {
    pub n_eq: u32,
    pub n_cq: u32,
    pub n_mq: u32,
    pub n_rq: u32,
    pub n_wq: u32,
    pub n_io: u32,
    pub n_sgl: u32,
    pub speed: u32,
    pub topology: u32,
// size of the buffers for first burst
    pub rq_default_buffer_size: u32,
    pub esoc: u8,
// MRQ RQ selection policy
    pub rq_selection_policy: u8,
// RQ quanta if rq_selection_policy == 2
    pub rr_quanta: u8,
    pub filter_def: [u32; SLI4_CMD_REG_FCFI_NUM_RQ_CFG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw {
    pub os: *mut efct,
    pub sli: sli4,
    pub ulp_start: u16,
    pub ulp_max: u16,
    pub dump_size: u32,
    pub state: efct_hw_state,
    pub hw_setup_called: bool,
    pub sliport_healthcheck: u8,
    pub fcf_indicator: u16,
// HW configuration
    pub config: efct_hw_config,
// calculated queue sizes for each type
    pub num_qentries: [u32; SLI4_QTYPE_MAX],
// Storage for SLI queue objects
    pub wq: [sli4_queue; EFCT_HW_MAX_NUM_WQ],
    pub rq: [sli4_queue; EFCT_HW_MAX_NUM_RQ],
    pub hw_rq_lookup: [u16; EFCT_HW_MAX_NUM_RQ],
    pub mq: [sli4_queue; EFCT_HW_MAX_NUM_MQ],
    pub cq: [sli4_queue; EFCT_HW_MAX_NUM_CQ],
    pub eq: [sli4_queue; EFCT_HW_MAX_NUM_EQ],
// HW queue
    pub eq_count: u32,
    pub cq_count: u32,
    pub mq_count: u32,
    pub wq_count: u32,
    pub rq_count: u32,
    pub cmd_head_count: u32,
    pub eq_list: list_head,
    pub cq_hash: [efct_queue_hash; EFCT_HW_Q_HASH_SIZE],
    pub rq_hash: [efct_queue_hash; EFCT_HW_Q_HASH_SIZE],
    pub wq_hash: [efct_queue_hash; EFCT_HW_Q_HASH_SIZE],
// Storage for HW queue objects
    pub hw_wq: [*mut hw_wq; EFCT_HW_MAX_NUM_WQ],
    pub hw_rq: [*mut hw_rq; EFCT_HW_MAX_NUM_RQ],
    pub hw_mq: [*mut hw_mq; EFCT_HW_MAX_NUM_MQ],
    pub hw_cq: [*mut hw_cq; EFCT_HW_MAX_NUM_CQ],
    pub hw_eq: [*mut hw_eq; EFCT_HW_MAX_NUM_EQ],
// count of hw_rq[] entries
    pub hw_rq_count: u32,
// count of multirq RQs
    pub hw_mrq_count: u32,
    pub wq_cpu_array: *mut hw_wq,
// Sequence objects used in incoming frame processing
    pub seq_pool: *mut efc_hw_sequence,
// Maintain an ordered, linked list of outstanding HW commands.
    pub bmbx_lock: mutex,
    pub cmd_lock: spinlock_t,
    pub cmd_head: list_head,
    pub cmd_pending: list_head,
    pub cmd_ctx_pool: *mut mempool_t,
    pub mbox_rqst_pool: *mut mempool_t,
    pub link: sli4_link_event,
// pointer array of IO objects
    pub io: *mut efct_hw_io,
// array of WQE buffs mapped to IO objects
    pub wqe_buffs: *mut u8,
// IO lock to synchronize list access
    pub io_lock: spinlock_t,
// List of IO objects in use
    pub io_inuse: list_head,
// List of IO objects waiting to be freed
    pub io_wait_free: list_head,
// List of IO objects available for allocation
    pub io_free: list_head,
    pub loop_map: efc_dma,
    pub xfer_rdy: efc_dma,
    pub rnode_mem: efc_dma,
    pub io_alloc_failed_count: core::sync::atomic::AtomicI32,
// stat: wq sumbit count
    pub tcmd_wq_submit: [u32; EFCT_HW_MAX_NUM_WQ],
// stat: wq complete count
    pub tcmd_wq_complete: [u32; EFCT_HW_MAX_NUM_WQ],
    pub send_frame_seq_id: core::sync::atomic::AtomicI32,
    pub wq_reqtag_pool: *mut reqtag_pool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_hw_io_count_type {
    EFCT_HW_IO_INUSE_COUNT,
    EFCT_HW_IO_FREE_COUNT,
    EFCT_HW_IO_WAIT_FREE_COUNT,
    EFCT_HW_IO_N_TOTAL_IO_COUNT,
}

// HW queue data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_eq {
    pub list_entry: list_head,
    pub type: sli4_qtype,
    pub instance: u32,
    pub entry_count: u32,
    pub entry_size: u32,
    pub hw: *mut efct_hw,
    pub queue: *mut sli4_queue,
    pub cq_list: list_head,
    pub use_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_cq {
    pub list_entry: list_head,
    pub type: sli4_qtype,
    pub instance: u32,
    pub entry_count: u32,
    pub entry_size: u32,
    pub eq: *mut hw_eq,
    pub queue: *mut sli4_queue,
    pub q_list: list_head,
    pub use_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_q {
    pub list_entry: list_head,
    pub type: sli4_qtype,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_mq {
    pub list_entry: list_head,
    pub type: sli4_qtype,
    pub instance: u32,
    pub entry_count: u32,
    pub entry_size: u32,
    pub cq: *mut hw_cq,
    pub queue: *mut sli4_queue,
    pub use_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_wq {
    pub list_entry: list_head,
    pub type: sli4_qtype,
    pub instance: u32,
    pub hw: *mut efct_hw,
    pub entry_count: u32,
    pub entry_size: u32,
    pub cq: *mut hw_cq,
    pub queue: *mut sli4_queue,
    pub class: u32,
// WQ consumed
    pub wqec_set_count: u32,
    pub wqec_count: u32,
    pub free_count: u32,
    pub total_submit_count: u32,
    pub pending_list: list_head,
// HW IO allocated for use with Send Frame
    pub send_frame_io: *mut efct_hw_io,
// Stats
    pub use_count: u32,
    pub wq_pending_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_rq {
    pub list_entry: list_head,
    pub type: sli4_qtype,
    pub instance: u32,
    pub entry_count: u32,
    pub use_count: u32,
    pub hdr_entry_size: u32,
    pub first_burst_entry_size: u32,
    pub data_entry_size: u32,
    pub is_mrq: bool,
    pub base_mrq_id: u32,
    pub cq: *mut hw_cq,
    pub filter_mask: u8,
    pub hdr: *mut sli4_queue,
    pub first_burst: *mut sli4_queue,
    pub data: *mut sli4_queue,
    pub hdr_buf: *mut efc_hw_rq_buffer,
    pub fb_buf: *mut efc_hw_rq_buffer,
    pub payload_buf: *mut efc_hw_rq_buffer,
// RQ tracker for this RQ
    pub rq_tracker: *mut efc_hw_sequence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_send_frame_context {
    pub hw: *mut efct_hw,
    pub wqcb: *mut hw_wq_callback,
    pub wqe: efct_hw_wqe,
    pub arg): *mut *mut void (callback)(int status, void,
    pub arg: *mut c_void,
// General purpose elements
    pub seq: *mut efc_hw_sequence,
    pub payload: efc_dma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_hw_grp_hdr {
    pub size: u32,
    pub magic_number: __be32,
    pub word2: u32,
    pub rev_name: [u8; 128],
    pub date: [u8; 12],
    pub revision: [u8; 32],
}

extern "C" {
    pub fn efct_hw_init(hw: *mut efct_hw) -> c_int;
}
extern "C" {
    pub fn efct_hw_rx_allocate(hw: *mut efct_hw) -> c_int;
}
extern "C" {
    pub fn efct_hw_rx_post(hw: *mut efct_hw) -> c_int;
}
extern "C" {
    pub fn efct_hw_rx_free(hw: *mut efct_hw);
}
extern "C" {
    pub fn efct_hw_io_free(hw: *mut efct_hw, io: *mut efct_hw_io) -> c_int;
}
extern "C" {
    pub fn efct_hw_io_inuse(hw: *mut efct_hw, io: *mut efct_hw_io) -> u8;
}
// efct_hw_io_lookup(struct efct_hw *hw, u32 indicator);
extern "C" {
    pub fn efct_hw_io_abort_all(hw: *mut efct_hw);
}
extern "C" {
    pub fn efct_hw_io_free_internal(arg: *mut kref);
}
// HW WQ request tag API
extern "C" {
    pub fn efct_hw_reqtag_pool_free(hw: *mut efct_hw);
}
// efct_hw_reqtag_alloc(struct efct_hw *hw,
// efct_hw_reqtag_get_instance(struct efct_hw *hw, u32 instance_index);
// RQ completion handlers for RQ pair mode
// Copy src to dst, then zero out the linked list link
// dst = *src;
// Only RQ pair mode is supported
extern "C" {
    pub fn efct_hw_rqpair_sequence_free(_arg: hw, _arg: seq) -> return;
}
extern "C" {
    pub fn efct_hw_cq_process(hw: *mut efct_hw, cq: *mut hw_cq);
}
extern "C" {
    pub fn efct_hw_wq_write(wq: *mut hw_wq, wqe: *mut efct_hw_wqe) -> c_int;
}
// Function for retrieving link statistics
// Function for retrieving host statistics
// efct_hw_new_wq(struct hw_cq *cq, u32 entry_count);
extern "C" {
    pub fn efct_hw_del_eq(eq: *mut hw_eq);
}
extern "C" {
    pub fn efct_hw_del_cq(cq: *mut hw_cq);
}
extern "C" {
    pub fn efct_hw_del_mq(mq: *mut hw_mq);
}
extern "C" {
    pub fn efct_hw_del_wq(wq: *mut hw_wq);
}
extern "C" {
    pub fn efct_hw_del_rq(rq: *mut hw_rq);
}
extern "C" {
    pub fn efct_hw_queue_teardown(hw: *mut efct_hw);
}
extern "C" {
    pub fn efct_hw_teardown(hw: *mut efct_hw);
}
