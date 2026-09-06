//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_uld.h
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


//
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const MAX_ULD_QSETS: c_int = 16;
pub const MAX_ULD_NPORTS: c_int = 4;
// ulp_mem_io + ulptx_idata + payload + padding

// CPL message priority levels

// Special asynchronous notification message

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serv_entry {
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aopen_entry {
    pub data: *mut c_void,
    pub next: *mut aopen_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eotid_entry {
    pub data: *mut c_void,
}

//
// Holds the size, base address, free list start, etc of the TID, server TID,
// and active-open TID tables.  The tables themselves are allocated dynamically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_info {
    pub tid_tab: *mut c_void,
    pub tid_base: c_uint,
    pub ntids: c_uint,
    pub stid_tab: *mut serv_entry,
    pub stid_bmap: *mut c_ulong,
    pub nstids: c_uint,
    pub stid_base: c_uint,
    pub nhash: c_uint,
    pub hash_base: c_uint,
    pub atid_tab: *mut aopen_entry,
    pub natids: c_uint,
    pub atid_base: c_uint,
    pub hpftid_tab: *mut filter_entry,
    pub hpftid_bmap: *mut c_ulong,
    pub nhpftids: c_uint,
    pub hpftid_base: c_uint,
    pub ftid_tab: *mut filter_entry,
    pub ftid_bmap: *mut c_ulong,
    pub nftids: c_uint,
    pub ftid_base: c_uint,
    pub aftid_base: c_uint,
    pub aftid_end: c_uint,
// Server filter region
    pub sftid_base: c_uint,
    pub nsftids: c_uint,
    pub ____cacheline_aligned_in_smp: spinlock_t atid_lock,
    pub afree: *mut aopen_entry,
    pub atids_in_use: c_uint,
    pub stid_lock: spinlock_t,
    pub stids_in_use: c_uint,
    pub v6_stids_in_use: c_uint,
    pub sftids_in_use: c_uint,
// ETHOFLD range
    pub eotid_tab: *mut eotid_entry,
    pub eotid_bmap: *mut c_ulong,
    pub eotid_base: c_uint,
    pub neotids: c_uint,
// TIDs in the TCAM
    pub tids_in_use: core::sync::atomic::AtomicI32,
// TIDs in the HASH
    pub hash_tids_in_use: core::sync::atomic::AtomicI32,
    pub conns_in_use: core::sync::atomic::AtomicI32,
// ETHOFLD TIDs used for rate limiting
    pub eotids_in_use: core::sync::atomic::AtomicI32,
// lock for setting/clearing filter bitmap
    pub ftid_lock: spinlock_t,
    pub tc_hash_tids_max_prio: c_uint,
}

// Is it a server filter TID?
extern "C" {
    pub fn cxgb4_alloc_atid(t: *mut tid_info, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cxgb4_alloc_stid(t: *mut tid_info, family: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cxgb4_alloc_sftid(t: *mut tid_info, family: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cxgb4_free_atid(t: *mut tid_info, atid: c_uint);
}
extern "C" {
    pub fn cxgb4_free_stid(t: *mut tid_info, stid: c_uint, family: c_int);
}
// Filter operation context to allow callers of cxgb4_set_filter() and
// cxgb4_del_filter() to wait for an asynchronous completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_ctx {
    pub /: *mut *mut completion completion; / completion rendezvous,
    pub /: *mut *mut *mut void closure; / caller's opaque information,
    pub /: *mut *mut int result; / result of operation,
    pub /: *mut *mut u32 tid; / to store tid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ktls {
    pub ktls_refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_uld {
    CXGB4_ULD_INIT,
    CXGB4_ULD_RDMA,
    CXGB4_ULD_ISCSI,
    CXGB4_ULD_ISCSIT,
    CXGB4_ULD_CRYPTO,
    CXGB4_ULD_IPSEC,
    CXGB4_ULD_TLS,
    CXGB4_ULD_KTLS,
    CXGB4_ULD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_tx_uld {
    CXGB4_TX_OFLD,
    CXGB4_TX_CRYPTO,
    CXGB4_TX_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_txq_type {
    CXGB4_TXQ_ETH,
    CXGB4_TXQ_ULD,
    CXGB4_TXQ_CTRL,
    CXGB4_TXQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_state {
    CXGB4_STATE_UP,
    CXGB4_STATE_START_RECOVERY,
    CXGB4_STATE_DOWN,
    CXGB4_STATE_DETACH,
    CXGB4_STATE_FATAL_ERROR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_control {
    CXGB4_CONTROL_DB_FULL,
    CXGB4_CONTROL_DB_EMPTY,
    CXGB4_CONTROL_DB_DROP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_range {
    pub start: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_virt_res {
    pub ddp: cxgb4_range,
    pub iscsi: cxgb4_range,
    pub stag: cxgb4_range,
    pub rq: cxgb4_range,
    pub srq: cxgb4_range,
    pub pbl: cxgb4_range,
    pub qp: cxgb4_range,
    pub cq: cxgb4_range,
    pub ocq: cxgb4_range,
    pub key: cxgb4_range,
    pub ncrypto_fc: c_uint,
    pub ppod_edram: cxgb4_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_ktls_port_stats_debug {
    pub ktls_tx_connection_open: core::sync::atomic::AtomicI64,
    pub ktls_tx_connection_fail: core::sync::atomic::AtomicI64,
    pub ktls_tx_connection_close: core::sync::atomic::AtomicI64,
    pub ktls_tx_encrypted_packets: core::sync::atomic::AtomicI64,
    pub ktls_tx_encrypted_bytes: core::sync::atomic::AtomicI64,
    pub ktls_tx_ctx: core::sync::atomic::AtomicI64,
    pub ktls_tx_ooo: core::sync::atomic::AtomicI64,
    pub ktls_tx_skip_no_sync_data: core::sync::atomic::AtomicI64,
    pub ktls_tx_drop_no_sync_data: core::sync::atomic::AtomicI64,
    pub ktls_tx_drop_bypass_req: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_ktls_stats_debug {
    pub ktls_port: [ch_ktls_port_stats_debug; MAX_ULD_NPORTS],
    pub ktls_tx_send_records: core::sync::atomic::AtomicI64,
    pub ktls_tx_end_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_start_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_middle_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_retransmit_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_complete_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_trimmed_pkts: core::sync::atomic::AtomicI64,
    pub ktls_tx_fallback: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_stats_debug {
    pub cipher_rqst: core::sync::atomic::AtomicI32,
    pub digest_rqst: core::sync::atomic::AtomicI32,
    pub aead_rqst: core::sync::atomic::AtomicI32,
    pub complete: core::sync::atomic::AtomicI32,
    pub error: core::sync::atomic::AtomicI32,
    pub fallback: core::sync::atomic::AtomicI32,
    pub tls_pdu_tx: core::sync::atomic::AtomicI32,
    pub tls_pdu_rx: core::sync::atomic::AtomicI32,
    pub tls_key: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_ipsec_stats_debug {
    pub ipsec_cnt: core::sync::atomic::AtomicI32,
}

//
// Block of information the LLD provides to ULDs attaching to a device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_lld_info {
    pub /: *mut *mut *mut pci_dev pdev; / associated PCI device,
    pub /: *mut *mut *mut l2t_data l2t; / L2 table,
    pub /: *mut *mut *mut tid_info tids; / TID table,
    pub /: *mut *mut *mut *mut net_device ports; / device ports,
    pub /: *const *const *const cxgb4_virt_res vr; / assorted HW resources,
    pub /: *const *const *const unsigned short mtus; / MTU table,
    pub /: *const *const *const unsigned short rxq_ids; / the ULD's Rx queue ids,
    pub /: *const *const *const unsigned short ciq_ids; / the ULD's concentrator IQ ids,
    pub /: *mut *mut unsigned short nrxq; / # of Rx queues,
    pub /: *mut *mut unsigned short ntxq; / # of Tx queues,
    pub /: *mut *mut unsigned short nciq; / # of concentrator IQ,
    pub /: *mut *mut unsigned char nchan:4; / # of channels,
    pub /: *mut *mut unsigned char nports:4; / # of ports,
    pub /: *mut *mut unsigned char wr_cred; / WR 16-byte credits,
    pub /: *mut *mut unsigned char adapter_type; / type of adapter,
    pub /: *mut *mut unsigned char fw_api_ver; / FW API version,
    pub /: *mut *mut unsigned char crypto; / crypto support,
    pub /: *mut *mut unsigned int fw_vers; / FW version,
    pub /: *mut *mut unsigned int iscsi_iolen; / iSCSI max I/O length,
    pub /: *mut *mut unsigned int cclk_ps; / Core clock period in psec,
    pub /: *mut *mut unsigned short udb_density; / # of user DB/page,
    pub /: *mut *mut unsigned short ucq_density; / # of user CQs/page,
    pub /: *mut *mut unsigned int sge_host_page_size; / SGE host page size,
    pub /: *mut *mut unsigned short filt_mode; / filter optional components,
    pub /: *mut *mut unsigned short tx_modq[NCHAN]; / maps each tx channel to a,
// scheduler queue
    pub /: *mut *mut *mut void __iomem gts_reg; / address of GTS register,
    pub /: *mut *mut *mut void __iomem db_reg; / address of kernel doorbell,
    pub /: *mut *mut int dbfifo_int_thresh; / doorbell fifo int threshold,
    pub /: *mut *mut unsigned int sge_ingpadboundary; / SGE ingress padding boundary,
    pub /: *mut *mut unsigned int sge_egrstatuspagesize; / SGE egress status page size,
    pub /: *mut *mut unsigned int sge_pktshift; / Padding between CPL and,
// packet data
    pub /: *mut *mut unsigned int pf; / Physical Function we're using,
    pub /: *mut *mut bool enable_fw_ofld_conn; / Enable connection through fw,
// WR
    pub /: *mut *mut unsigned int max_ordird_qp; / Max ORD/IRD depth per RDMA QP,
    pub /: *mut *mut unsigned int max_ird_adapter; / Max IRD memory per adapter,
    pub /: *mut *mut bool ulptx_memwrite_dsgl; / use of T5 DSGL allowed,
    pub /: *mut *mut unsigned int iscsi_tagmask; / iscsi ddp tag mask,
    pub /: *mut *mut unsigned int iscsi_pgsz_order; / iscsi ddp page size orders,
    pub /: *mut *mut unsigned int iscsi_llimit; / chip's iscsi region llimit,
    pub /: *mut *mut unsigned int ulp_crypto; / crypto lookaside support,
    pub /: *mut *mut *mut *mut void iscsi_ppm; / iscsi page pod manager,
    pub /: *mut *mut int nodeid; / device numa node id,
    pub /: *mut *mut bool fr_nsmr_tpte_wr_support; / FW supports FR_NSMR_TPTE_WR,
    pub /: *mut *mut bool write_w_imm_support; / FW supports WRITE_WITH_IMMEDIATE,
    pub /: *mut *mut bool write_cmpl_support; / FW supports WRITE_CMPL WR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_uld_info {
    pub name: [c_char; IFNAMSIZ],
    pub handle: *mut c_void,
    pub nrxq: c_uint,
    pub rxq_size: c_uint,
    pub ntxq: c_uint,
    pub ciq: bool,
    pub lro: bool,
    pub p): *const *const *const void (add)(struct cxgb4_lld_info,
    pub gl): *const pkt_gl,
    pub new_state): *mut *mut *mut int (state_change)(void handle, enum cxgb4_state,
    pub ...): *mut *mut *mut int (control)(void handle, enum cxgb4_control control,,
    pub napi): *mut napi_struct,
    pub ): *mut *mut void (lro_flush)(struct t4_lro_mgr,
    pub dev): *mut *mut *mut int (tx_handler)(struct sk_buff skb, struct net_device,

    pub tlsdev_ops: *const tlsdev_ops,

    pub xfrmdev_ops: *const xfrmdev_ops,

}

extern "C" {
    pub fn cxgb4_uld_enable(adap: *mut adapter);
}
extern "C" {
    pub fn cxgb4_register_uld(type: cxgb4_uld, p: *const cxgb4_uld_info);
}
extern "C" {
    pub fn cxgb4_unregister_uld(type: cxgb4_uld) -> c_int;
}
extern "C" {
    pub fn cxgb4_ofld_send(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cxgb4_crypto_send(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cxgb4_dbfifo_count(dev: *const net_device, lpfifo: c_int) -> c_uint;
}
extern "C" {
    pub fn cxgb4_port_chan(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn cxgb4_port_e2cchan(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn cxgb4_port_viid(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn cxgb4_port_idx(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn cxgb4_sync_txq_pidx(dev: *mut net_device, qid: u16, pidx: u16, size: u16) -> c_int;
}
extern "C" {
    pub fn cxgb4_flush_eq_cache(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cxgb4_read_tpte(dev: *mut net_device, stag: u32, tpte: *mut __be32) -> c_int;
}
extern "C" {
    pub fn cxgb4_read_sge_timestamp(dev: *mut net_device) -> u64;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_bar2_qtype {
    int cxgb4_bar2_sge_qregs(struct net_device *dev,
    unsigned int qid,
    enum cxgb4_bar2_qtype qtype,
    int user,
    u64 *pbar2_qoffset,
    unsigned int *pbar2_qid);
