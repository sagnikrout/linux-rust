//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/adapter.h
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
// Copyright (c) 2003-2008 Chelsio, Inc. All rights reserved.
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
// This file should not be included directly.  Include common.h instead.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_idx_types {
    LAN_MAC_IDX	= 0,
    SAN_MAC_IDX,

    MAX_MAC_IDX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_config {
    pub mac_addr: [__u8; ETH_ALEN],
    pub flags: __u32,
    pub skb): *mut *mut *mut int (send)(struct port_info pi, struct sk_buff,
    pub skb): *mut *mut *mut int (recv)(struct port_info pi, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_info {
    pub adapter: *mut adapter,
    pub qs: *mut sge_qset,
    pub port_id: u8,
    pub nqsets: u8,
    pub first_qset: u8,
    pub phy: cphy,
    pub mac: cmac,
    pub link_config: link_config,
    pub activity: c_int,
    pub iscsi_ipv4addr: __be32,
    pub iscsic: iscsi_config,
    pub /: *mut *mut int link_fault; / link fault was detected,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fl_pg_chunk {
    pub page: *mut page,
    pub va: *mut c_void,
    pub offset: c_uint,
    pub p_cnt: *mut c_ulong,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_fl {
    pub /: *mut *mut unsigned int buf_size; / size of each Rx buffer,
    pub /: *mut *mut unsigned int credits; / # of available Rx buffers,
    pub /: *mut *mut unsigned int pend_cred; / new buffers since last FL DB ring,
    pub /: *mut *mut unsigned int size; / capacity of free list,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned int gen; / free list generation,
    pub /: *mut *mut fl_pg_chunk pg_chunk;/ page chunk cache,
    pub /: *mut *mut unsigned int use_pages; / whether FL uses pages or sk_buffs,
    pub /: *mut *mut unsigned int order; / order of page allocations,
    pub /: *mut *mut unsigned int alloc_size; / size of allocated buffer,
    pub /: *mut *mut *mut rx_desc desc; / address of HW Rx descriptor ring,
    pub /: *mut *mut *mut rx_sw_desc sdesc; / address of SW Rx descriptor ring,
    pub /: *mut *mut dma_addr_t phys_addr; / physical address of HW ring start,
    pub /: *mut *mut unsigned int cntxt_id; / SGE context id for the free list,
    pub /: *mut *mut unsigned long empty; / # of times queue ran out of buffers,
    pub /: *mut *mut unsigned long alloc_failed; / # of times buffer allocation failed,
}

//
// Bundle size for grouping offload RX packets for delivery to the stack.
// Don't make this too big as we do prefetch on each packet in a bundle.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_rspq {
    pub /: *mut *mut unsigned int credits; / # of pending response credits,
    pub /: *mut *mut unsigned int size; / capacity of response queue,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut unsigned int gen; / current generation bit,
    pub /: *mut *mut unsigned int polling; / is the queue serviced through NAPI?,
    pub /: *mut *mut unsigned int holdoff_tmr; / interrupt holdoff timer in 100ns,
    pub /: *mut *mut unsigned int next_holdoff; / holdoff time for next interrupt,
    pub occurred: *mut *mut unsigned int rx_recycle_buf; / whether recycling,
    pub /: *mut *mut *mut rsp_desc desc; / address of HW response ring,
    pub /: *mut *mut dma_addr_t phys_addr; / physical address of the ring,
    pub /: *mut *mut unsigned int cntxt_id; / SGE context id for the response q,
    pub /: *mut *mut spinlock_t lock; / guards response processing,
    pub /: *mut *mut sk_buff_head rx_queue; / offload packet receive queue,
    pub /: *mut *mut *mut sk_buff pg_skb; / used to build frag list in napi handler,
    pub offload_pkts: c_ulong,
    pub offload_bundles: c_ulong,
    pub /: *mut *mut unsigned long eth_pkts; / # of ethernet packets,
    pub /: *mut *mut unsigned long pure_rsps; / # of pure (non-data) responses,
    pub /: *mut *mut unsigned long imm_data; / responses with immediate data,
    pub /: *mut *mut unsigned long rx_drops; / # of packets dropped due to no mem,
    pub /: *mut *mut unsigned long async_notif; / # of asynchronous notification events,
    pub /: *mut *mut unsigned long empty; / # of times queue ran out of credits,
    pub /: *mut *mut unsigned long nomem; / # of responses deferred due to no mem,
    pub /: *mut *mut unsigned long unhandled_irqs; / # of spurious intrs,
    pub starved: c_ulong,
    pub restarted: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_txq {
    pub /: *mut *mut unsigned long flags; / HW DMA fetch status,
    pub /: *mut *mut unsigned int in_use; / # of in-use Tx descriptors,
    pub /: *mut *mut unsigned int size; / # of descriptors,
    pub /: *mut *mut unsigned int processed; / total # of descs HW has processed,
    pub /: *mut *mut unsigned int cleaned; / total # of descs SW has reclaimed,
    pub /: *mut *mut unsigned int stop_thres; / SW TX queue suspend threshold,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned int gen; / current value of generation bit,
    pub /: *mut *mut unsigned int unacked; / Tx descriptors used since last COMPL,
    pub /: *mut *mut *mut tx_desc desc; / address of HW Tx descriptor ring,
    pub /: *mut *mut *mut tx_sw_desc sdesc; / address of SW Tx descriptor ring,
    pub /: *mut *mut spinlock_t lock; / guards enqueueing of new packets,
    pub /: *mut *mut unsigned int token; / WR token,
    pub /: *mut *mut dma_addr_t phys_addr; / physical address of the ring,
    pub /: *mut *mut sk_buff_head sendq; / List of backpressured offload packets,
    pub /: *mut *mut work_qresume_task; / restarts the queue,
    pub /: *mut *mut unsigned int cntxt_id; / SGE context id for the Tx q,
    pub /: *mut *mut unsigned long stops; / # of times q has been stopped,
    pub /: *mut *mut unsigned long restarts; / # of queue restarts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_qset {
    pub adap: *mut adapter,
    pub napi: napi_struct,
    pub rspq: sge_rspq,
    pub fl: [sge_fl; SGE_RXQ_PER_SET],
    pub txq: [sge_txq; SGE_TXQ_PER_SET],
    pub nomem: c_int,
    pub lro_va: *mut c_void,
    pub netdev: *mut net_device,
    pub /: *mut *mut *mut netdev_queue tx_q; / associated netdev TX queue,
    pub /: *mut *mut unsigned long txq_stopped; / which Tx queues are stopped,
    pub /: *mut *mut timer_list tx_reclaim_timer; / reclaims TX buffers,
    pub /: *mut *mut timer_list rx_reclaim_timer; / reclaims RX buffers,
    pub port_stats: [c_ulong; SGE_PSTAT_MAX],
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge {
    pub qs: [sge_qset; SGE_QSETS],
    pub /: *mut *mut spinlock_t reg_lock; / guards non-atomic SGE registers (eg context),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter {
    pub tdev: t3cdev,
    pub adapter_list: list_head,
    pub regs: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub registered_device_map: c_ulong,
    pub open_device_map: c_ulong,
    pub flags: c_ulong,
    pub name: *const c_char,
    pub msg_enable: c_int,
    pub mmio_len: c_uint,
    pub params: adapter_params,
    pub slow_intr_mask: c_uint,
    pub irq_stats: [c_ulong; IRQ_NUM_STATS],
    pub msix_nvectors: c_int,
    pub vec: c_ushort,
    pub /: *mut *mut char desc[IFNAMSIZ + 1 + 12]; / Needs space for "%s-%d",
    pub 1]: } msix_info[SGE_QSETS +,
// T3 modules
    pub sge: sge,
    pub pmrx: mc7,
    pub pmtx: mc7,
    pub cm: mc7,
    pub mc5: mc5,
    pub port: [*mut net_device; MAX_NPORTS],
    pub check_task_cnt: c_uint,
    pub adap_check_task: delayed_work,
    pub ext_intr_handler_task: work_struct,
    pub fatal_error_handler_task: work_struct,
    pub link_fault_handler_task: work_struct,
    pub db_full_task: work_struct,
    pub db_empty_task: work_struct,
    pub db_drop_task: work_struct,
    pub debugfs_root: *mut dentry,
    pub mdio_lock: mutex,
    pub stats_lock: spinlock_t,
    pub work_lock: spinlock_t,
    pub nofail_skb: *mut sk_buff,
}

extern "C" {
    pub fn netdev_priv(_arg: adap->port[idx]) -> return;
}
pub const OFFLOAD_DEVMAP_BIT: c_int = 15;

extern "C" {
    pub fn test_bit(_arg: OFFLOAD_DEVMAP_BIT, _arg: &adapter->open_device_map) -> return;
}
extern "C" {
    pub fn t3_offload_tx(tdev: *mut t3cdev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t3_os_ext_intr_handler(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_os_phymod_changed(adap: *mut adapter, port_id: c_int);
}
extern "C" {
    pub fn t3_os_link_fault(adapter: *mut adapter, port_id: c_int, state: c_int);
}
extern "C" {
    pub fn t3_os_link_fault_handler(adapter: *mut adapter, port_id: c_int);
}
extern "C" {
    pub fn t3_sge_start(adap: *mut adapter);
}
extern "C" {
    pub fn t3_sge_stop_dma(adap: *mut adapter);
}
extern "C" {
    pub fn t3_sge_stop(adap: *mut adapter);
}
extern "C" {
    pub fn t3_start_sge_timers(adap: *mut adapter);
}
extern "C" {
    pub fn t3_stop_sge_timers(adap: *mut adapter);
}
extern "C" {
    pub fn t3_free_sge_resources(adap: *mut adapter);
}
extern "C" {
    pub fn t3_sge_err_intr_handler(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_intr_handler(adap: *mut adapter, polling: c_int) -> irq_handler_t;
}
extern "C" {
    pub fn t3_eth_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn t3_mgmt_tx(adap: *mut adapter, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t3_update_qset_coalesce(qs: *mut sge_qset, p: *const qset_params);
}
extern "C" {
    pub fn t3_get_edc_fw(phy: *mut cphy, edc_idx: c_int, size: c_int) -> c_int;
}
