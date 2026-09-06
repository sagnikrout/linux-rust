//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/siw/siw.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Copyright (c) 2008-2019, IBM Corporation

pub const SIW_VENDOR_ID: c_uint = 0x626d74 /* ascii 'bmt' for now */;
pub const SIW_VENDORT_PART_ID: c_int = 0;

pub const SIW_MAX_ORD_QP: c_int = 128;
pub const SIW_MAX_IRD_QP: c_int = 128;

// Min number of bytes for using zero copy transmit

// Maximum number of frames which can be send in one SQ processing
pub const SQ_USER_MAXBURST: c_int = 100;
// Maximum number of consecutive IRQ elements which get served
// if SQ has pending work. Prevents starving local SQ processing
// by serving peer Read Requests.
//
pub const SIW_IRQ_MAXBURST_SQ_ACTIVE: c_int = 4;
// There is always only a port 1 per siw device
pub const SIW_PORT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_dev_cap {
    pub max_qp: c_int,
    pub max_qp_wr: c_int,
    pub /: *mut *mut int max_ord; / max. outbound read queue depth,
    pub /: *mut *mut int max_ird; / max. inbound read queue depth,
    pub max_sge: c_int,
    pub max_sge_rd: c_int,
    pub max_cq: c_int,
    pub max_cqe: c_int,
    pub max_mr: c_int,
    pub max_pd: c_int,
    pub max_mw: c_int,
    pub max_srq: c_int,
    pub max_srq_wr: c_int,
    pub max_srq_sge: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_pd {
    pub base_pd: ib_pd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_device {
    pub base_dev: ib_device,
    pub attrs: siw_dev_cap,
    pub vendor_part_id: u32,
    pub numa_node: c_int,
    pub raw_gid: [c_char; ETH_ALEN],
    pub lock: spinlock_t,
    pub qp_xa: xarray,
    pub mem_xa: xarray,
    pub cep_list: list_head,
    pub qp_list: list_head,
// active objects statistics to enforce limits
    pub num_qp: core::sync::atomic::AtomicI32,
    pub num_cq: core::sync::atomic::AtomicI32,
    pub num_pd: core::sync::atomic::AtomicI32,
    pub num_mr: core::sync::atomic::AtomicI32,
    pub num_srq: core::sync::atomic::AtomicI32,
    pub num_ctx: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_ucontext {
    pub base_ucontext: ib_ucontext,
    pub sdev: *mut siw_device,
}

//
// The RDMA core does not define LOCAL_READ access, which is always
// enabled implictely.
//

//
// siw presentation of user memory registered as source
// or target of RDMA operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_page_chunk {
    pub plist: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_umem {
    pub base_mem: *mut ib_umem,
    pub num_pages: c_uint,
    pub num_chunks: c_uint,
    pub /: *mut *mut u64 fp_addr; / First page base address,
    pub __counted_by(num_chunks): siw_page_chunk page_chunk[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_pble {
    pub /: *mut *mut dma_addr_t addr; / Address of assigned buffer,
    pub /: *mut *mut unsigned int size; / Size of this entry,
    pub /: *mut *mut unsigned long pbl_off; / Total offset from start of PBL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_pbl {
    pub num_buf: c_uint,
    pub max_buf: c_uint,
    pub __counted_by(max_buf): siw_pble pbe[],
}

//
// Generic memory representation for registered siw memory.
// Memory lookup always via higher 24 bit of STag (STag index).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_mem {
    pub sdev: *mut siw_device,
    pub ref: kref,
    pub /: *mut *mut u64 va; / VA of memory,
    pub /: *mut *mut u64 len; / length of the memory buffer in bytes,
    pub /: *mut *mut u32 stag; / iWarp memory access steering tag,
    pub /: *mut *mut u8 stag_valid; / VALID or INVALID,
    pub /: *mut *mut u8 is_pbl; / PBL or user space mem,
    pub /: *mut *mut u8 is_mw; / Memory Region or Memory Window,
    pub /: *mut *mut ib_access_flags perms; / local/remote READ & WRITE,
    pub umem: *mut siw_umem,
    pub pbl: *mut siw_pbl,
    pub mem_obj: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_mr {
    pub base_mr: ib_mr,
    pub mem: *mut siw_mem,
    pub rcu: rcu_head,
}

//
// Error codes for local or remote
// access to registered memory
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_access_state {
    E_ACCESS_OK,
    E_STAG_INVALID,
    E_BASE_BOUNDS,
    E_ACCESS_PERM,
    E_PD_MISMATCH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_wr_state {
    SIW_WR_IDLE,
    SIW_WR_QUEUED, /* processing has not started yet */
    SIW_WR_INPROGRESS /* initiated processing of the WR */
}

// The WQE currently being processed (RX or TX)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_wqe {
// Copy of applications SQE or RQE
    pub sqe: siw_sqe,
    pub rqe: siw_rqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_cq {
    pub base_cq: ib_cq,
    pub lock: spinlock_t,
    pub notify: *mut siw_cq_ctrl,
    pub queue: *mut siw_cqe,
    pub cq_put: u32,
    pub cq_get: u32,
    pub num_cqe: u32,
    pub /: *mut *mut *mut rdma_user_mmap_entry cq_entry; / mmap info for CQE array,
    pub /: *mut *mut u32 id; / For debugging only,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_qp_state {
    SIW_QP_STATE_IDLE,
    SIW_QP_STATE_RTR,
    SIW_QP_STATE_RTS,
    SIW_QP_STATE_CLOSING,
    SIW_QP_STATE_TERMINATE,
    SIW_QP_STATE_ERROR,
    SIW_QP_STATE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_qp_flags {
    SIW_RDMA_BIND_ENABLED = (1 << 0),
    SIW_RDMA_WRITE_ENABLED = (1 << 1),
    SIW_RDMA_READ_ENABLED = (1 << 2),
    SIW_SIGNAL_ALL_WR = (1 << 3),
    SIW_MPA_CRC = (1 << 4),
    SIW_QP_IN_DESTROY = (1 << 5)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_qp_attr_mask {
    SIW_QP_ATTR_STATE = (1 << 0),
    SIW_QP_ATTR_ACCESS_FLAGS = (1 << 1),
    SIW_QP_ATTR_LLP_HANDLE = (1 << 2),
    SIW_QP_ATTR_ORD = (1 << 3),
    SIW_QP_ATTR_IRD = (1 << 4),
    SIW_QP_ATTR_SQ_SIZE = (1 << 5),
    SIW_QP_ATTR_RQ_SIZE = (1 << 6),
    SIW_QP_ATTR_MPA = (1 << 7)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_srq {
    pub base_srq: ib_srq,
    pub lock: spinlock_t,
    pub max_sge: u32,
    pub /: *mut *mut u32 limit; / low watermark for async event,
    pub recvq: *mut siw_rqe,
    pub rq_put: u32,
    pub rq_get: u32,
    pub /: *mut *mut u32 num_rqe; / max # of wqe's allowed,
    pub /: *mut *mut *mut rdma_user_mmap_entry srq_entry; / mmap info for SRQ array,
    pub /: *mut *mut bool armed:1; / inform user if limit hit,
    pub /: *mut *mut bool is_kernel_res:1; / true if kernel client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_qp_attrs {
    pub state: siw_qp_state,
    pub sq_size: u32,
    pub rq_size: u32,
    pub orq_size: u32,
    pub irq_size: u32,
    pub sq_max_sges: u32,
    pub rq_max_sges: u32,
    pub flags: siw_qp_flags,
    pub sk: *mut socket,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_tx_ctx {
    SIW_SEND_HDR, /* start or continue sending HDR */
    SIW_SEND_DATA, /* start or continue sending DDP payload */
    SIW_SEND_TRAILER, /* start or continue sending TRAILER */
    SIW_SEND_SHORT_FPDU/* send whole FPDU hdr|data|trailer at once */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_rx_state {
    SIW_GET_HDR, /* await new hdr or within hdr */
    SIW_GET_DATA_START, /* start of inbound DDP payload */
    SIW_GET_DATA_MORE, /* continuation of (misaligned) DDP payload */
    SIW_GET_TRAILER/* await new trailer or within trailer */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_rx_stream {
    pub skb: *mut sk_buff,
    pub /: *mut *mut int skb_new; / pending unread bytes in skb,
    pub /: *mut *mut int skb_offset; / offset in skb,
    pub /: *mut *mut int skb_copied; / processed bytes in skb,
    pub state: siw_rx_state,
    pub hdr: iwarp_hdr,
    pub trailer: mpa_trailer,
    pub mpa_crc: u32,
    pub mpa_crc_enabled: bool,
//
// For each FPDU, main RX loop runs through 3 stages:
// Receiving protocol headers, placing DDP payload and receiving
// trailer information (CRC + possibly padding).
// Next two variables keep state on receive status of the
// current FPDU part (hdr, data, trailer).
//
    pub /: *mut *mut int fpdu_part_rcvd; / bytes in pkt part copied,
    pub /: *mut *mut int fpdu_part_rem; / bytes in pkt part not seen,
//
// Next expected DDP MSN for each QN +
// expected steering tag +
// expected DDP tagget offset (all HBO)
//
    pub ddp_msn: [u32; RDMAP_UNTAGGED_QN_COUNT],
    pub ddp_stag: u32,
    pub ddp_to: u64,
    pub /: *mut *mut u32 inval_stag; / Stag to be invalidated,
    pub 1: u8 rx_suspend :,
    pub /: *mut *mut u8 pad : 2; / # of pad bytes expected,
    pub /: *mut *mut u8 rdmap_op : 4; / opcode of current frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_rx_fpdu {
//
// Local destination memory of inbound RDMA operation.
// Valid, according to wqe->wr_status
//
    pub wqe_active: siw_wqe,
    pub /: *mut *mut unsigned int pbl_idx; / Index into current PBL,
    pub /: *mut *mut unsigned int sge_idx; / current sge in rx,
    pub /: *mut *mut unsigned int sge_off; / already rcvd in curr. sge,
    pub /: *mut *mut char first_ddp_seg; / this is the first DDP seg,
    pub /: *mut *mut char more_ddp_segs; / more DDP segs expected,
    pub /: *mut *mut u8 prev_rdmap_op : 4; / opcode of prev frame,
}

//
// Shorthands for short packets w/o payload
// to be transmitted more efficient.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_send_pkt {
    pub send: iwarp_send,
    pub crc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_write_pkt {
    pub write: iwarp_rdma_write,
    pub crc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_rreq_pkt {
    pub rreq: iwarp_rdma_rreq,
    pub crc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_rresp_pkt {
    pub rresp: iwarp_rdma_rresp,
    pub crc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_iwarp_tx {
    pub hdr: iwarp_hdr,
// Generic part of FPDU header
    pub ctrl: iwarp_ctrl,
    pub c_untagged: iwarp_ctrl_untagged,
    pub c_tagged: iwarp_ctrl_tagged,
// FPDU headers
    pub rwrite: iwarp_rdma_write,
    pub rreq: iwarp_rdma_rreq,
    pub rresp: iwarp_rdma_rresp,
    pub terminate: iwarp_terminate,
    pub send: iwarp_send,
    pub send_inv: iwarp_send_inv,
// complete short FPDUs
    pub send_pkt: siw_send_pkt,
    pub write_pkt: siw_write_pkt,
    pub rreq_pkt: siw_rreq_pkt,
    pub rresp_pkt: siw_rresp_pkt,
    pub pkt: },
    pub trailer: mpa_trailer,
// DDP MSN for untagged messages
    pub ddp_msn: [u32; RDMAP_UNTAGGED_QN_COUNT],
    pub state: siw_tx_ctx,
    pub /: *mut *mut u16 ctrl_len; / ddp+rdmap hdr,
    pub ctrl_sent: u16,
    pub burst: c_int,
    pub /: *mut *mut int bytes_unsent; / ddp payload bytes,
    pub mpa_crc: u32,
    pub mpa_crc_enabled: bool,
    pub /: *mut *mut u8 do_crc : 1; / do crc for segment,
    pub /: *mut *mut u8 use_sendpage : 1; / send w/o copy,
    pub /: *mut *mut u8 tx_suspend : 1; / stop sending DDP segs.,
    pub /: *mut *mut u8 pad : 2; / # pad in current fpdu,
    pub /: *mut *mut u8 orq_fence : 1; / ORQ full or Send fenced,
    pub /: *mut *mut u8 in_syscall : 1; / TX out of user context,
    pub /: *mut *mut u8 zcopy_tx : 1; / Use TCP_SENDPAGE if possible,
    pub /: *mut *mut u8 gso_seg_limit; / Maximum segments for GSO, 0 = unbound,
    pub /: *mut *mut u16 fpdu_len; / len of FPDU to tx,
    pub /: *mut *mut unsigned int tcp_seglen; / remaining tcp seg space,
    pub wqe_active: siw_wqe,
    pub /: *mut *mut int pbl_idx; / Index into current PBL,
    pub /: *mut *mut int sge_idx; / current sge in tx,
    pub /: *mut *mut u32 sge_off; / already sent in curr. sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_qp {
    pub base_qp: ib_qp,
    pub sdev: *mut siw_device,
    pub tx_cpu: c_int,
    pub ref: kref,
    pub qp_free: completion,
    pub devq: list_head,
    pub attrs: siw_qp_attrs,
    pub cep: *mut siw_cep,
    pub state_lock: rw_semaphore,
    pub pd: *mut ib_pd,
    pub scq: *mut siw_cq,
    pub rcq: *mut siw_cq,
    pub srq: *mut siw_srq,
    pub /: *mut *mut siw_iwarp_tx tx_ctx; / Transmit context,
    pub sq_lock: spinlock_t,
    pub /: *mut *mut *mut siw_sqe sendq; / send queue element array,
    pub /: *mut *mut uint32_t sq_get; / consumer index into sq array,
    pub /: *mut *mut uint32_t sq_put; / kernel prod. index into sq array,
    pub tx_list: llist_node,
    pub /: *mut *mut *mut siw_sqe orq; / outbound read queue element array,
    pub orq_lock: spinlock_t,
    pub /: *mut *mut uint32_t orq_get; / consumer index into orq array,
    pub /: *mut *mut uint32_t orq_put; / shared producer index for ORQ,
    pub rx_stream: siw_rx_stream,
    pub rx_fpdu: *mut siw_rx_fpdu,
    pub rx_tagged: siw_rx_fpdu,
    pub rx_untagged: siw_rx_fpdu,
    pub rq_lock: spinlock_t,
    pub /: *mut *mut *mut siw_rqe recvq; / recv queue element array,
    pub /: *mut *mut uint32_t rq_get; / consumer index into rq array,
    pub /: *mut *mut uint32_t rq_put; / kernel prod. index into rq array,
    pub /: *mut *mut *mut siw_sqe irq; / inbound read queue element array,
    pub /: *mut *mut uint32_t irq_get; / consumer index into irq array,
    pub /: *mut *mut uint32_t irq_put; / producer index into irq array,
    pub irq_burst: c_int,
    pub valid: u8,
    pub in_tx: u8,
    pub 4: u8 layer : 4, etype :,
    pub ecode: u8,
    pub term_info: },
    pub /: *mut *mut *mut rdma_user_mmap_entry sq_entry; / mmap info for SQE array,
    pub /: *mut *mut *mut rdma_user_mmap_entry rq_entry; / mmap info for RQE array,
}

// helper macros

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_msg_info {
    pub hdr_len: c_int,
    pub ctrl: iwarp_ctrl,
    pub qp): *mut *mut int (rx_data)(struct siw_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub address: *mut c_void,
}

// Global siw parameters. Currently set in siw_main.c
// QP general functions
extern "C" {
    pub fn siw_qp_mpa_rts(qp: *mut siw_qp, ctrl: mpa_v2_ctrl) -> c_int;
}
extern "C" {
    pub fn siw_qp_llp_close(qp: *mut siw_qp);
}
extern "C" {
    pub fn siw_qp_cm_drop(qp: *mut siw_qp, schedule: c_int);
}
extern "C" {
    pub fn siw_send_terminate(qp: *mut siw_qp);
}
extern "C" {
    pub fn siw_qp_get_ref(qp: *mut ib_qp);
}
extern "C" {
    pub fn siw_qp_put_ref(qp: *mut ib_qp);
}
extern "C" {
    pub fn siw_qp_add(sdev: *mut siw_device, qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_free_qp(ref: *mut kref);
}
extern "C" {
    pub fn siw_tagged_error(state: siw_access_state) -> ddp_ecode;
}
extern "C" {
    pub fn siw_rdmap_error(state: siw_access_state) -> rdmap_ecode;
}
extern "C" {
    pub fn siw_read_to_orq(rreq: *mut siw_sqe, sqe: *mut siw_sqe);
}
extern "C" {
    pub fn siw_qp_llp_data_ready(sk: *mut sock);
}
extern "C" {
    pub fn siw_qp_llp_write_space(sk: *mut sock);
}
// QP TX path functions
extern "C" {
    pub fn siw_create_tx_threads() -> c_int;
}
extern "C" {
    pub fn siw_stop_tx_threads();
}
extern "C" {
    pub fn siw_run_sq(arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn siw_qp_sq_process(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_sq_start(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_activate_tx(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_get_tx_cpu(sdev: *mut siw_device) -> c_int;
}
extern "C" {
    pub fn siw_put_tx_cpu(cpu: c_int);
}
// QP RX path functions
extern "C" {
    pub fn siw_proc_send(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_proc_rreq(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_proc_rresp(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_proc_write(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn siw_proc_terminate(qp: *mut siw_qp) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: base_ctx, siw_ucontext: struct, _arg: base_ucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: base_qp, siw_qp: struct, _arg: base_qp) -> return;
}
extern "C" {
    pub fn container_of(_arg: base_cq, siw_cq: struct, _arg: base_cq) -> return;
}
extern "C" {
    pub fn container_of(_arg: base_srq, siw_srq: struct, _arg: base_srq) -> return;
}
extern "C" {
    pub fn container_of(_arg: base_dev, siw_device: struct, _arg: base_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: base_mr, siw_mr: struct, _arg: base_mr) -> return;
}
extern "C" {
    pub fn container_of(_arg: rdma_mmap, siw_user_mmap_entry: struct, _arg: rdma_entry) -> return;
}
// crc = ~0;
// crc = crc32c(*crc, data, len);
extern "C" {
    pub fn siw_crc_final(_arg: &crc, _arg: out) -> return;
}

extern "C" {
    pub fn siw_cq_flush(cq: *mut siw_cq);
}
extern "C" {
    pub fn siw_sq_flush(qp: *mut siw_qp);
}
extern "C" {
    pub fn siw_rq_flush(qp: *mut siw_qp);
}
extern "C" {
    pub fn siw_reap_cqe(cq: *mut siw_cq, wc: *mut ib_wc) -> c_int;
}
