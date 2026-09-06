//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_netdev.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const DRV_MODULE_GEN_MAJOR: c_int = 2;
pub const DRV_MODULE_GEN_MINOR: c_int = 1;
pub const DRV_MODULE_GEN_SUBMINOR: c_int = 0;

// 1 for AENQ + ADMIN
pub const ENA_ADMIN_MSIX_VEC: c_int = 1;

// The ENA buffer length fields is 16 bit long. So when PAGE_SIZE == 64kB the
// driver passes 0.
// Since the max packet size the ENA handles is ~9kB limit the buffer length to
// 16kB.
//

pub const ENA_MIN_MSIX_VEC: c_int = 2;
pub const ENA_REG_BAR: c_int = 0;
pub const ENA_MEM_BAR: c_int = 2;

pub const ENA_MIN_MTU: c_int = 128;
pub const ENA_NAME_MAX_LEN: c_int = 20;
pub const ENA_IRQNAME_SIZE: c_int = 40;
pub const ENA_PKT_MAX_BUFS: c_int = 19;
pub const ENA_RX_RSS_TABLE_LOG_SIZE: c_int = 7;

// The number of tx packet completions that will be handled each NAPI poll
// cycle is ring_size / ENA_TX_POLL_BUDGET_DIVIDER.
//
pub const ENA_TX_POLL_BUDGET_DIVIDER: c_int = 4;
// Refill Rx queue when number of required descriptors is above
// QUEUE_SIZE / ENA_RX_REFILL_THRESH_DIVIDER or ENA_RX_REFILL_THRESH_PACKET
//
pub const ENA_RX_REFILL_THRESH_DIVIDER: c_int = 8;
pub const ENA_RX_REFILL_THRESH_PACKET: c_int = 256;
// Number of queues to check for missing queues per timer service
pub const ENA_MONITORED_TX_QUEUES: c_int = 4;
// Max timeout packets before device reset
pub const MAX_NUM_OF_TIMEOUTED_PACKETS: c_int = 128;

pub const ENA_MGMNT_IRQ_IDX: c_int = 0;
pub const ENA_IO_IRQ_FIRST_IDX: c_int = 1;

pub const ENA_ADMIN_POLL_DELAY_US: c_int = 100;
// ENA device should send keep alive msg every 1 sec.
// We wait for 6 sec just to be on the safe side.
//

pub const ENA_MAX_NO_INTERRUPT_ITERATIONS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_irq {
    pub handler: irq_handler_t,
    pub data: *mut c_void,
    pub cpu: c_int,
    pub vector: u32,
    pub affinity_hint_mask: cpumask_t,
    pub name: [c_char; ENA_IRQNAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_napi {
    pub ____cacheline_aligned: u8 first_interrupt,
    pub interrupts_masked: u8,
    pub napi: napi_struct,
    pub tx_ring: *mut ena_ring,
    pub rx_ring: *mut ena_ring,
    pub qid: u32,
    pub dim: dim,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_tx_buffer {
    pub skb: *mut sk_buff,
// XDP buffer structure which is used for sending packets in
// the xdp queues
//
    pub xdpf: *mut xdp_frame,
}

// num of ena desc for this specific skb
// (includes data desc and metadata desc)
//
// num of buffers used by this skb
// Total size of all buffers in bytes
// Indicate if bufs[0] map the linear data of the skb.
// Used for detect missing tx packets to limit the number of prints
// Save the last jiffies to detect missing tx packets
//
// sets to non zero value on ena_start_xmit and set to zero on
// napi and timer_Service_routine.
//
// while this value is not protected by lock,
// a given packet is not expected to be handled by ena_start_xmit
// and by napi/timer_service at the same time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_rx_buffer {
    pub skb: *mut sk_buff,
    pub page: *mut page,
    pub dma_addr: dma_addr_t,
    pub page_offset: u32,
    pub buf_offset: u32,
    pub ena_buf: ena_com_buf,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_stats_tx {
    pub cnt: u64,
    pub bytes: u64,
    pub queue_stop: u64,
    pub prepare_ctx_err: u64,
    pub queue_wakeup: u64,
    pub dma_mapping_err: u64,
    pub linearize: u64,
    pub linearize_failed: u64,
    pub napi_comp: u64,
    pub tx_poll: u64,
    pub doorbells: u64,
    pub bad_req_id: u64,
    pub llq_buffer_copy: u64,
    pub missed_tx: u64,
    pub unmask_interrupt: u64,
    pub last_napi_jiffies: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_stats_rx {
    pub cnt: u64,
    pub bytes: u64,
    pub rx_copybreak_pkt: u64,
    pub csum_good: u64,
    pub refil_partial: u64,
    pub csum_bad: u64,
    pub page_alloc_fail: u64,
    pub skb_alloc_fail: u64,
    pub dma_mapping_err: u64,
    pub bad_desc_num: u64,
    pub bad_req_id: u64,
    pub empty_rx_ring: u64,
    pub csum_unchecked: u64,
    pub xdp_aborted: u64,
    pub xdp_drop: u64,
    pub xdp_pass: u64,
    pub xdp_tx: u64,
    pub xdp_invalid: u64,
    pub xdp_redirect: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_ring {
// Holds the empty requests for TX/RX
// out of order completions
//
    pub free_ids: *mut u16,
    pub tx_buffer_info: *mut ena_tx_buffer,
    pub rx_buffer_info: *mut ena_rx_buffer,
}

// cache ptr to avoid using the adapter
// Used for rx queues only to point to the xdp tx ring, to
// which traffic should be redirected from this rx ring.
//
// The maximum header length the device can handle
// cpu and NUMA for TPH
// number of tx/rx_buffer_info's entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_stats_dev {
    pub tx_timeout: u64,
    pub suspend: u64,
    pub resume: u64,
    pub wd_expired: u64,
    pub interface_up: u64,
    pub interface_down: u64,
    pub admin_q_pause: u64,
    pub rx_drops: u64,
    pub tx_drops: u64,
    pub reset_fail: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_flags_t {
    ENA_FLAG_DEVICE_RUNNING,
    ENA_FLAG_DEV_UP,
    ENA_FLAG_LINK_UP,
    ENA_FLAG_MSIX_ENABLED,
    ENA_FLAG_TRIGGER_RESET,
    ENA_FLAG_ONGOING_RESET
}

// adapter specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_adapter {
    pub ena_dev: *mut ena_com_dev,
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
// rx packets that shorter that this len will be copied to the skb
// header
//
    pub rx_copybreak: u32,
    pub max_mtu: u32,
    pub num_io_queues: u32,
    pub max_num_io_queues: u32,
    pub msix_vecs: c_int,
    pub missing_tx_completion_threshold: u32,
    pub requested_tx_ring_size: u32,
    pub requested_rx_ring_size: u32,
    pub max_tx_ring_size: u32,
    pub max_rx_ring_size: u32,
    pub msg_enable: u32,
// large_llq_header_enabled is used for two purposes:
// 1. Indicates that large LLQ has been requested.
// 2. Indicates whether large LLQ is set or not after device
// initialization / configuration.
//
    pub large_llq_header_enabled: bool,
    pub large_llq_header_supported: bool,
    pub max_tx_sgl_size: u16,
    pub max_rx_sgl_size: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub keep_alive_timeout: c_ulong,
    pub missing_tx_completion_to: c_ulong,
    pub name: [c_char; ENA_NAME_MAX_LEN],
    pub phc_info: *mut ena_phc_info,
    pub flags: c_ulong,
// TX
// RX
    pub ena_napi: [ena_napi; ENA_MAX_NUM_IO_QUEUES],
    pub irq_tbl: [ena_irq; ENA_MAX_MSIX_VEC(ENA_MAX_NUM_IO_QUEUES)],
// timer service
    pub reset_task: work_struct,
    pub timer_service: timer_list,
    pub wd_state: bool,
    pub dev_up_before_reset: bool,
    pub disable_meta_caching: bool,
    pub last_keep_alive_jiffies: c_ulong,
    pub syncp: u64_stats_sync,
    pub dev_stats: ena_stats_dev,
    pub eni_stats: ena_admin_eni_stats,
    pub ena_srd_info: ena_admin_ena_srd_info,
// last queue index that was checked for uncompleted tx packets
    pub last_monitored_tx_qid: u32,
    pub reset_reason: ena_regs_reset_reason_types,
    pub xdp_bpf_prog: *mut bpf_prog,
    pub xdp_first_ring: u32,
    pub xdp_num_queues: u32,
    pub devlink: *mut devlink,
    pub devlink_port: devlink_port,

    pub debugfs_base: *mut dentry,

}

extern "C" {
    pub fn ena_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ena_dump_stats_to_dmesg(adapter: *mut ena_adapter);
}
extern "C" {
    pub fn ena_dump_stats_to_buf(adapter: *mut ena_adapter, buf: *mut u8);
}
extern "C" {
    pub fn ena_update_queue_count(adapter: *mut ena_adapter, new_channel_count: u32) -> c_int;
}
extern "C" {
    pub fn ena_set_rx_copybreak(adapter: *mut ena_adapter, rx_copybreak: u32) -> c_int;
}
extern "C" {
    pub fn ena_get_sset_count(netdev: *mut net_device, sset: c_int) -> c_int;
}
// Make sure reset reason is set before triggering the reset
extern "C" {
    pub fn ena_destroy_device(adapter: *mut ena_adapter, graceful: bool) -> c_int;
}
extern "C" {
    pub fn ena_restore_device(adapter: *mut ena_adapter) -> c_int;
}
// Increase a stat by cnt while holding syncp seqlock on 32bit machines
extern "C" {
    pub fn ena_free_all_io_tx_resources(adapter: *mut ena_adapter);
}
extern "C" {
    pub fn ena_down(adapter: *mut ena_adapter);
}
extern "C" {
    pub fn ena_up(adapter: *mut ena_adapter) -> c_int;
}
extern "C" {
    pub fn ena_unmask_interrupt(tx_ring: *mut ena_ring, rx_ring: *mut ena_ring);
}
