//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/vmxnet3/vmxnet3_int.h
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
// Linux driver for VMware's vmxnet3 ethernet NIC.
//
// Copyright (C) 2008-2024, VMware, Inc. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; version 2 of the License and no later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//
// The full GNU General Public License is included in this distribution in
// the file called "COPYING".
//
// Maintained by: pv-drivers@vmware.com
//

//
// Version numbers
//

// Each byte of this 32-bit integer encodes a version number in
// VMXNET3_DRIVER_VERSION_STRING.
//
pub const VMXNET3_DRIVER_VERSION_NUM: c_uint = 0x01090000;

// RSS only makes sense if MSI-X is supported.
// Macro flag: #define VMXNET3_RSS

//
// Capabilities
//
// IPv4
// offload
// for a pkt
// pages transmits
// pkts up to 256kB.
//
// Maximum devices supported.
//
pub const MAX_ETHERNET_CARDS: c_int = 10;
pub const MAX_PCI_PASSTHRU_DEVICE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_cmd_ring {
    pub base: *mut Vmxnet3_GenericDesc,
    pub size: u32,
    pub next2fill: u32,
    pub next2comp: u32,
    pub gen: u8,
    pub isOutOfOrder: u8,
    pub basePA: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_comp_ring {
    pub base: *mut Vmxnet3_GenericDesc,
    pub size: u32,
    pub next2proc: u32,
    pub gen: u8,
    pub intr_idx: u8,
    pub basePA: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tx_data_ring {
    pub base: *mut Vmxnet3_TxDataDesc,
    pub size: u32,
    pub basePA: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tx_ts_ring {
    pub base: *mut Vmxnet3_TxTSDesc,
    pub basePA: dma_addr_t,
}

pub const VMXNET3_MAP_NONE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tx_buf_info {
    pub map_type: u32,
    pub len: u16,
    pub sop_idx: u16,
    pub dma_addr: dma_addr_t,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tq_driver_stats {
    pub the: *mut *mut u64 drop_total; / # of pkts dropped by the driver,,
// counters below track droppings due to
// different reasons
//
    pub drop_too_many_frags: u64,
    pub drop_oversized_hdr: u64,
    pub drop_hdr_inspect_err: u64,
    pub drop_tso: u64,
    pub tx_ring_full: u64,
    pub /: *mut *mut u64 linearized; / # of pkts linearized,
    pub /: *mut *mut u64 copy_skb_header; / # of times we have to copy skb header,
    pub oversized_hdr: u64,
    pub xdp_xmit: u64,
    pub xdp_xmit_err: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tx_ctx {
    pub ipv4: bool,
    pub ipv6: bool,
    pub mss: u16,
    pub csum: *mut *mut u32 l4_offset; / only valid for pkts requesting tso or,
// offloading. For encap offload, it refers to
// inner L4 offset i.e. it includes outer header
// encap header and inner eth and ip header size
//
    pub 0: *mut *mut u32 l4_hdr_size; / only valid if mss !=,
// Refers to inner L4 hdr size for encap
// offload
//
    pub /: *mut *mut u32 copy_size; / # of bytes copied into the data ring,
    pub sop_txd: *mut Vmxnet3_GenericDesc,
    pub eop_txd: *mut Vmxnet3_GenericDesc,
    pub ts_txd: *mut Vmxnet3_TxTSDesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_tx_queue {
    pub /: *mut *mut char name[IFNAMSIZ+8]; / To identify interrupt,
    pub adapter: *mut vmxnet3_adapter,
    pub tx_lock: spinlock_t,
    pub tx_ring: vmxnet3_cmd_ring,
    pub buf_info: *mut vmxnet3_tx_buf_info,
    pub data_ring: vmxnet3_tx_data_ring,
    pub ts_ring: vmxnet3_tx_ts_ring,
    pub comp_ring: vmxnet3_comp_ring,
    pub shared: *mut Vmxnet3_TxQueueCtrl,
    pub stats: vmxnet3_tq_driver_stats,
    pub stopped: bool,
    pub is: *mut *mut int num_stop; / # of times the queue,
// stopped
    pub qid: c_int,
    pub txdata_desc_size: u16,
    pub tx_ts_desc_size: u16,
    pub tsPktCount: u16,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmxnet3_rx_buf_type {
    VMXNET3_RX_BUF_NONE = 0,
    VMXNET3_RX_BUF_SKB = 1,
    VMXNET3_RX_BUF_PAGE = 2,
    VMXNET3_RX_BUF_XDP = 3,
}

pub const VMXNET3_RXD_COMP_PENDING: c_int = 0;
pub const VMXNET3_RXD_COMP_DONE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rx_buf_info {
    pub buf_type: vmxnet3_rx_buf_type,
    pub len: u16,
    pub comp_state: u8,
    pub skb: *mut sk_buff,
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rx_ctx {
    pub skb: *mut sk_buff,
    pub sop_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rq_driver_stats {
    pub drop_total: u64,
    pub drop_err: u64,
    pub drop_fcs: u64,
    pub rx_buf_alloc_failure: u64,
    pub /: *mut *mut u64 xdp_packets; / Total packets processed by XDP.,
    pub xdp_tx: u64,
    pub xdp_redirects: u64,
    pub xdp_drops: u64,
    pub xdp_aborted: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rx_data_ring {
    pub base: *mut Vmxnet3_RxDataDesc,
    pub basePA: dma_addr_t,
    pub desc_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rx_ts_ring {
    pub base: *mut Vmxnet3_RxTSDesc,
    pub basePA: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_rx_queue {
    pub /: *mut *mut char name[IFNAMSIZ + 8]; / To identify interrupt,
    pub adapter: *mut vmxnet3_adapter,
    pub napi: napi_struct,
    pub rx_ring: [vmxnet3_cmd_ring; 2],
    pub data_ring: vmxnet3_rx_data_ring,
    pub comp_ring: vmxnet3_comp_ring,
    pub ts_ring: vmxnet3_rx_ts_ring,
    pub rx_ctx: vmxnet3_rx_ctx,
    pub /: *mut *mut u32 qid; / rqID in RCD for buffer from 1st ring,
    pub /: *mut *mut u32 qid2; / rqID in RCD for buffer from 2nd ring,
    pub /: *mut *mut u32 dataRingQid; / rqID in RCD for buffer from data ring,
    pub buf_info: [*mut vmxnet3_rx_buf_info; 2],
    pub shared: *mut Vmxnet3_RxQueueCtrl,
    pub stats: vmxnet3_rq_driver_stats,
    pub page_pool: *mut page_pool,
    pub xdp_rxq: xdp_rxq_info,
    pub rx_ts_desc_size: u16,
    pub ____cacheline_aligned: },
pub const VMXNET3_DEVICE_MAX_TX_QUEUES: c_int = 32;

pub const VMXNET3_DEVICE_DEFAULT_TX_QUEUES: c_int = 8;

// Should be less than UPT1_RSS_MAX_IND_TABLE_SIZE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_intr {
    pub mask_mode: vmxnet3_intr_mask_mode,
    pub /: *mut *mut vmxnet3_intr_type type; / MSI-X, MSI, or INTx?,
    pub /: *mut *mut u8 num_intrs; / # of intr vectors,
    pub /: *mut *mut u8 event_intr_idx; / idx of the intr vector for event,
    pub /: *mut *mut u8 mod_levels[VMXNET3_LINUX_MAX_MSIX_VECT]; / moderation level,
    pub event_msi_vector_name: [c_char; IFNAMSIZ+17],    pub msix_entries: [msix_entry; VMXNET3_LINUX_MAX_MSIX_VECT],
}

// Interrupt sharing schemes, share_intr

pub const VMXNET3_STATE_BIT_RESETTING: c_int = 0;
pub const VMXNET3_STATE_BIT_QUIESCED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmxnet3_adapter {
    pub tx_queue: [vmxnet3_tx_queue; VMXNET3_DEVICE_MAX_TX_QUEUES],
    pub rx_queue: [vmxnet3_rx_queue; VMXNET3_DEVICE_MAX_RX_QUEUES],
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub intr: vmxnet3_intr,
    pub cmd_lock: spinlock_t,
    pub shared: *mut Vmxnet3_DriverShared,
    pub pm_conf: *mut Vmxnet3_PMConf,
    pub /: *mut *mut *mut Vmxnet3_TxQueueDesc tqd_start; / all tx queue desc,
    pub /: *mut *mut *mut Vmxnet3_RxQueueDesc rqd_start; / all rx queue desc,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut u8 __iomem hw_addr0; / for BAR 0,
    pub /: *mut *mut *mut u8 __iomem hw_addr1; / for BAR 1,
    pub version: u8,

    pub rss_conf: *mut UPT1_RSSConf,
    pub rss: bool,

    pub num_rx_queues: u32,
    pub num_tx_queues: u32,
// rx buffer related
    pub skb_buf_size: unsigned,
    pub /: *mut *mut int rx_buf_per_pkt; / only apply to the 1st ring,
    pub shared_pa: dma_addr_t,
    pub queue_desc_pa: dma_addr_t,
    pub coal_conf_pa: dma_addr_t,
// Wake-on-LAN
    pub wol: u32,
// Link speed
    pub /: *mut *mut u32 link_speed; / in mbps,
    pub tx_timeout_count: u64,
// Ring sizes
    pub tx_ring_size: u32,
    pub rx_ring_size: u32,
    pub rx_ring2_size: u32,
// Size of buffer in the data ring
    pub txdata_desc_size: u16,
    pub rxdata_desc_size: u16,
    pub rxdataring_enabled: bool,
    pub default_rss_fields: bool,
    pub rss_fields: Vmxnet3_RSSField,
    pub work: work_struct,
    pub /: *mut *mut unsigned long state; / VMXNET3_STATE_BIT_xxx,
    pub share_intr: c_int,
    pub coal_conf: *mut Vmxnet3_CoalesceScheme,
    pub default_coal_mode: bool,
    pub adapter_pa: dma_addr_t,
    pub pm_conf_pa: dma_addr_t,
    pub rss_conf_pa: dma_addr_t,
    pub queuesExtEnabled: bool,
    pub ringBufSize: Vmxnet3_RingBufferSize,
    pub devcap_supported: [u32; 8],
    pub ptcap_supported: [u32; 8],
    pub dev_caps: [u32; 8],
    pub tx_prod_offset: u16,
    pub rx_prod_offset: u16,
    pub rx_prod2_offset: u16,
    pub xdp_bpf_prog: *mut bpf_prog __rcu,
    pub latencyConf: *mut Vmxnet3_LatencyConf,
// Size of buffer in the ts ring
    pub tx_ts_desc_size: u16,
    pub rx_ts_desc_size: u16,
    pub disabledOffloads: u32,
}

// must be a multiple of VMXNET3_RING_SIZE_ALIGN
pub const VMXNET3_DEF_TX_RING_SIZE: c_int = 512;
pub const VMXNET3_DEF_RX_RING_SIZE: c_int = 1024;
pub const VMXNET3_DEF_RX_RING2_SIZE: c_int = 512;
pub const VMXNET3_DEF_RXDATA_DESC_SIZE: c_int = 128;
pub const VMXNET3_MAX_ETH_HDR_SIZE: c_int = 22;

pub const VMXNET3_COAL_STATIC_DEFAULT_DEPTH: c_int = 64;

extern "C" {
    pub fn vmxnet3_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn vmxnet3_check_ptcapability(cap_supported: u32, cap: u32) -> bool;
}
