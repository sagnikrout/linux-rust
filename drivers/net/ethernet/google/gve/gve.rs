//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2024 Google LLC
//

pub const PCI_VENDOR_ID_GOOGLE: c_uint = 0x1ae0;

pub const PCI_DEV_ID_GVNIC: c_uint = 0x0042;
pub const GVE_REGISTER_BAR: c_int = 0;
pub const GVE_DOORBELL_BAR: c_int = 2;
// Driver can alloc up to 2 segments for the header and 2 for the payload.
pub const GVE_TX_MAX_IOVEC: c_int = 4;
// 1 for management, 1 for rx, 1 for tx
pub const GVE_MIN_MSIX: c_int = 3;
// Numbers of gve tx/rx stats in stats report.
pub const GVE_TX_STATS_REPORT_NUM: c_int = 6;
pub const GVE_RX_STATS_REPORT_NUM: c_int = 2;
// Interval to schedule a stats report update, 20000ms.
pub const GVE_STATS_REPORT_TIMER_PERIOD: c_int = 20000;

// Numbers of NIC tx/rx stats in stats report.
pub const NIC_TX_STATS_REPORT_NUM: c_int = 0;
pub const NIC_RX_STATS_REPORT_NUM: c_int = 4;
pub const GVE_ADMINQ_BUFFER_SIZE: c_int = 4096;

// PTYPEs are always 10 bits.
pub const GVE_NUM_PTYPES: c_int = 1024;
// Default minimum ring size
pub const GVE_DEFAULT_MIN_TX_RING_SIZE: c_int = 256;
pub const GVE_DEFAULT_MIN_RX_RING_SIZE: c_int = 512;
pub const GVE_DEFAULT_RX_BUFFER_SIZE: c_int = 2048;
pub const GVE_XDP_RX_BUFFER_SIZE_DQO: c_int = 4096;
pub const GVE_DEFAULT_RX_BUFFER_OFFSET: c_int = 2048;
pub const GVE_PAGE_POOL_SIZE_MULTIPLIER: c_int = 4;

pub const GVE_RSS_KEY_SIZE: c_int = 40;
pub const GVE_RSS_INDIR_SIZE: c_int = 128;
pub const GVE_XDP_ACTIONS: c_int = 5;
pub const GVE_GQ_TX_MIN_PKT_DESC_BYTES: c_int = 182;
pub const GVE_DEFAULT_HEADER_BUFFER_SIZE: c_int = 128;
// Maximum TSO size supported on DQO
pub const GVE_DQO_TX_MAX: c_uint = 0x3FFFF;
pub const GVE_TX_BUF_SHIFT_DQO: c_int = 11;
// 2K buffers for DQO-QPL

// If number of free/recyclable buffers are less than this threshold; driver
// allocs and uses a non-qpl page on the receive path of DQO QPL to free
// up buffers.
// Value is set big enough to post at least 3 64K LRO packet via 2K buffer to NIC.
//
pub const GVE_DQO_QPL_ONDEMAND_ALLOC_THRESHOLD: c_int = 96;
pub const GVE_DQO_RX_HWTSTAMP_VALID: c_uint = 0x1;
// Each slot in the desc ring has a 1:1 mapping to a slot in the data ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_desc_queue {
    pub /: *mut *mut *mut gve_rx_desc desc_ring; / the descriptor ring,
    pub /: *mut *mut dma_addr_t bus; / the bus for the desc_ring,
    pub desc*/: *mut *mut u8 seqno; / the next expected seqno for this,
}

// The page info for a single slot in the RX data queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_slot_page_info {
// netmem is used for DQO RDA mode
// page is used in all other modes
//
    pub page: *mut page,
    pub netmem: netmem_ref,
}

// A list of pages registered with the device during setup and used by a queue
// as buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_queue_page_list {
    pub /: *mut *mut u32 id; / unique id,
    pub num_entries: u32,
    pub /: *mut *mut *mut *mut page pages; / list of num_entries pages,
    pub /: *mut *mut *mut dma_addr_t page_buses; / the dma addrs of the pages,
}

// Each slot in the data ring has a 1:1 mapping to a slot in the desc ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_data_queue {
    pub /: *mut *mut *mut gve_rx_data_slot data_ring; / read by NIC,
    pub /: *mut *mut dma_addr_t data_bus; / dma mapping of the slots,
    pub /: *mut *mut *mut gve_rx_slot_page_info page_info; / page info of the buffers,
    pub /: *mut *mut *mut gve_queue_page_list qpl; / qpl assigned to this queue,
    pub /: *mut *mut u8 raw_addressing; / use raw_addressing?,
}

// RX buffer queue for posting buffers to HW.
// Each RX (completion) queue has a corresponding buffer queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_buf_queue_dqo {
    pub desc_ring: *mut gve_rx_desc_dqo,
    pub bus: dma_addr_t,
    pub /: *mut *mut u32 head; / Pointer to start cleaning buffers at.,
    pub /: *mut *mut u32 tail; / Last posted buffer index + 1,
    pub /: *mut *mut u32 mask; / Mask for indices to the size of the ring,
}

// RX completion queue to receive packets from HW.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_compl_queue_dqo {
    pub desc_ring: *mut gve_rx_compl_desc_dqo,
    pub bus: dma_addr_t,
// Number of slots which did not have a buffer posted yet. We should not
// post more buffers than the queue size to avoid HW overrunning the
// queue.
//
    pub num_free_slots: c_int,
// HW uses a "generation bit" to notify SW of new descriptors. When a
// descriptor's generation bit is different from the current generation,
// that descriptor is ready to be consumed by SW.
//
    pub cur_gen_bit: u8,
// Pointer into desc_ring where the next completion descriptor will be
// received.
//
    pub head: u32,
    pub /: *mut *mut u32 mask; / Mask for indices to the size of the ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_header_buf {
    pub data: *mut u8,
    pub addr: dma_addr_t,
}

// Stores state for tracking buffers posted to HW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_buf_state_dqo {
// The page posted to HW.
    pub page_info: gve_rx_slot_page_info,
// XSK buffer
    pub xsk_buff: *mut xdp_buff,
// The DMA address corresponding to `page_info`.
    pub addr: dma_addr_t,
// Last offset into the page when it only had a single reference, at
// which point every other offset is free to be reused.
//
    pub last_single_ref_offset: u32,
// Linked list index to next element in the list, or -1 if none
    pub next: i16,
}

// Wrapper for XDP Rx metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_xdp_buff {
    pub xdp: xdp_buff,
    pub gve: *mut gve_priv,
    pub compl_desc: *const gve_rx_compl_desc_dqo,
}

// `head` and `tail` are indices into an array, or -1 if empty.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_index_list {
    pub head: i16,
    pub tail: i16,
}

// A single received packet split across multiple buffers may be
// reconstructed using the information in this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_ctx {
// head and tail of skb chain for the current packet or NULL if none
    pub skb_head: *mut sk_buff,
    pub skb_tail: *mut sk_buff,
    pub total_size: u32,
    pub frag_cnt: u8,
    pub drop_pkt: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_cnts {
    pub ok_pkt_bytes: u32,
    pub ok_pkt_cnt: u16,
    pub total_pkt_cnt: u16,
    pub cont_pkt_cnt: u16,
    pub desc_err_pkt_cnt: u16,
}

// Contains datapath state used to represent an RX queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_ring {
    pub gve: *mut gve_priv,
    pub /: *mut *mut u16 packet_buffer_size; / Size of buffer posted to NIC,
    pub /: *mut *mut u16 packet_buffer_truesize; / Total size of RX buffer,
    pub rx_headroom: u16,
// GQI fields
    pub desc: gve_rx_desc_queue,
    pub data: gve_rx_data_queue,
// threshold for posting new buffs and descs
    pub db_threshold: u32,
    pub qpl_copy_pool_mask: u32,
    pub qpl_copy_pool_head: u32,
    pub qpl_copy_pool: *mut gve_rx_slot_page_info,
}

// DQO fields.
// Linked list of gve_rx_buf_state_dqo. Index into
// buf_states, or -1 if empty.
//
// Linked list of gve_rx_buf_state_dqo. Indexes into
// buf_states, or -1 if empty.
//
// This list contains buf_states which are pointing to
// valid buffers.
//
// We use a FIFO here in order to increase the
// probability that buffers can be reused by increasing
// the time between usages.
//
// Linked list of gve_rx_buf_state_dqo. Indexes into
// buf_states, or -1 if empty.
//
// This list contains buf_states which have buffers
// which cannot be reused yet.
//
// qpl assigned to this queue
// index into queue page list
// track number of used buffers
// Address info of the buffers for header-split
// free-running count of unsplit packets due to header buffer overflow or hdr_len is 0
// XDP stuff
// A TX desc ring entry
#[repr(C)]
#[derive(Copy, Clone)]
pub union gve_tx_desc {
    pub /: *mut *mut gve_tx_pkt_desc pkt; / first desc for a packet,
    pub /: *mut *mut gve_tx_mtd_desc mtd; / optional metadata descriptor,
    pub /: *mut *mut gve_tx_seg_desc seg; / subsequent descs for a packet,
}

// Tracks the memory in the fifo occupied by a segment of a packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_iovec {
    pub /: *mut *mut u32 iov_offset; / offset into this segment,
    pub /: *mut *mut u32 iov_len; / length,
    pub /: *mut *mut u32 iov_padding; / padding associated with this segment,
}

// Tracks the memory in the fifo occupied by the skb. Mapped 1:1 to a desc
// ring entry but only used for a pkt_desc not a seg_desc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_buffer_state {
    pub /: *mut *mut *mut sk_buff skb; / skb for this pkt,
    pub /: *mut *mut *mut xdp_frame xdp_frame; / xdp_frame,
}

// A TX buffer - each queue has one
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_fifo {
    pub /: *mut *mut *mut void base; / address of base of FIFO,
    pub /: *mut *mut u32 size; / total size,
    pub /: *mut *mut atomic_t available; / how much space is still available,
    pub /: *mut *mut u32 head; / offset to write at,
    pub /: *mut *mut *mut gve_queue_page_list qpl; / QPL mapped into this FIFO,
}

// TX descriptor for DQO format
#[repr(C)]
#[derive(Copy, Clone)]
pub union gve_tx_desc_dqo {
    pub pkt: gve_tx_pkt_desc_dqo,
    pub tso_ctx: gve_tx_tso_context_desc_dqo,
    pub general_ctx: gve_tx_general_context_desc_dqo,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_packet_state {
// Packet is in free list, available to be allocated.
// This should always be zero since state is not explicitly initialized.
//
    GVE_PACKET_STATE_UNALLOCATED,
// Packet is expecting a regular data completion or miss completion
    GVE_PACKET_STATE_PENDING_DATA_COMPL,
// Packet has received a miss completion and is expecting a
// re-injection completion.
//
    GVE_PACKET_STATE_PENDING_REINJECT_COMPL,
// No valid completion received within the specified timeout.
    GVE_PACKET_STATE_TIMED_OUT_COMPL,
// XSK pending packet has received a packet/reinjection completion, or
// has timed out. At this point, the pending packet can be counted by
// xsk_tx_complete and freed.
//
    GVE_PACKET_STATE_XSK_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_tx_pending_packet_dqo_type {
    GVE_TX_PENDING_PACKET_DQO_SKB,
    GVE_TX_PENDING_PACKET_DQO_XDP_FRAME,
    GVE_TX_PENDING_PACKET_DQO_XSK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_pending_packet_dqo {
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

// 0th element corresponds to the linear portion of `skb`, should be
// unmapped with `dma_unmap_single`.
//
// All others correspond to `skb`'s frags and should be unmapped with
// `dma_unmap_page`.
//
// Linked list index to next element in the list, or -1 if none
// Linked list index to prev element in the list, or -1 if none.
// Used for tracking either outstanding miss completions or prematurely
// freed packets.
//
// Identifies the current state of the packet as defined in
// `enum gve_packet_state`.
//
// gve_tx_pending_packet_dqo_type
// If packet is an outstanding miss completion, then the packet is
// freed if the corresponding re-injection completion is not received
// before kernel jiffies exceeds timeout_jiffies.
//
// Contains datapath state used to represent a TX queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_ring {
// Cacheline 0 -- Accessed & dirtied during transmit
// GQI fields
    pub tx_fifo: gve_tx_fifo,
    pub /: *mut *mut u32 req; / driver tracked head pointer,
    pub /: *mut *mut u32 done; / driver tracked tail pointer,
}

// DQO fields.
// Spinlock for XDP tx traffic
// Linked list of gve_tx_pending_packet_dqo. Index into
// pending_packets, or -1 if empty.
//
// This is a consumer list owned by the TX path. When it
// runs out, the producer list is stolen from the
// completion handling path
// (dqo_compl.free_pending_packets).
//
// Cached value of `dqo_compl.hw_tx_head`
// Index of the last descriptor with "report event" bit
// set.
//
// free running number of packet buf descriptors posted
// free running number of packet buf descriptors completed
// QPL fields
// Linked list of gve_tx_buf_dqo. Index into
// tx_qpl_buf_next, or -1 if empty.
//
// This is a consumer list owned by the TX path. When it
// runs out, the producer list is stolen from the
// completion handling path
// (dqo_compl.free_tx_qpl_buf_head).
//
// Free running count of the number of QPL tx buffers
// allocated
//
// Cached value of `dqo_compl.free_tx_qpl_buf_cnt`
// Cacheline 1 -- Accessed & dirtied during gve_clean_tx_done
// GQI fields
// Spinlock for when cleanup in progress
// Spinlock for XDP tx traffic
// DQO fields.
// Tracks the current gen bit of compl_q
// Linked list of gve_tx_pending_packet_dqo. Index into
// pending_packets, or -1 if empty.
//
// This is the producer list, owned by the completion
// handling path. When the consumer list
// (dqo_tx.free_pending_packets) is runs out, this list
// will be stolen.
//
// Last TX ring index fetched by HW
// List to track pending packets which received a miss
// completion but not a corresponding reinjection.
//
// List to track pending packets that were completed
// before receiving a valid completion because they
// reached a specified timeout.
//
// QPL fields
// Linked list of gve_tx_buf_dqo. Index into
// tx_qpl_buf_next, or -1 if empty.
//
// This is the producer list, owned by the completion
// handling path. When the consumer list
// (dqo_tx.free_tx_qpl_buf_head) is runs out, this list
// will be stolen.
//
// Free running count of the number of tx buffers
// freed
//
// Cacheline 2 -- Read-mostly fields
// GQI fields
// Maps 1:1 to a desc
// DQO fields.
// QPL fields
// qpl assigned to this queue
// Each QPL page is divided into TX bounce buffers
// of size GVE_TX_BUF_SIZE_DQO. tx_qpl_buf_next is
// an array to manage linked lists of TX buffers.
// An entry j at index i implies that j'th buffer
// is next on the list after i
//
// Slow-path fields
// Wraps the info for one irq including the napi struct and the queues
// associated with that irq.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_notify_block {
    pub /: *mut *mut *mut __be32 irq_db_index; / pointer to idx into Bar2,
    pub /: *mut *mut char name[IFNAMSIZ + 16]; / name registered with the kernel,
    pub /: *mut *mut napi_napi; / kernel napi for this block,
    pub priv: *mut gve_priv,
    pub /: *mut *mut *mut gve_tx_ring tx; / tx rings on this block,
    pub /: *mut *mut *mut gve_rx_ring rx; / rx rings on this block,
    pub irq: u32,
}

// Tracks allowed and current rx queue settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_queue_config {
    pub max_queues: u16,
    pub num_queues: u16,
    pub packet_buffer_size: u16,
}

// Tracks allowed and current tx queue settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_queue_config {
    pub max_queues: u16,
    pub /: *mut *mut u16 num_queues; / number of TX queues, excluding XDP queues,
    pub num_xdp_queues: u16,
}

// Tracks the available and used qpl IDs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_qpl_config {
    pub /: *mut *mut u32 qpl_map_size; / map memory size,
    pub /: *mut *mut *mut unsigned long qpl_id_map; / bitmap of used qpl ids,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_irq_db {
    pub index: __be32,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_ptype {
    pub /: *mut *mut u8 l3_type; / `gve_l3_type` in gve_adminq.h,
    pub /: *mut *mut u8 l4_type; / `gve_l4_type` in gve_adminq.h,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_ptype_lut {
    pub ptypes: [gve_ptype; GVE_NUM_PTYPES],
}

// Parameters for allocating resources for tx queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_alloc_rings_cfg {
    pub qcfg: *mut gve_tx_queue_config,
    pub pages_per_qpl: u16,
    pub num_xdp_rings: u16,
    pub ring_size: u16,
    pub raw_addressing: bool,
// Allocated resources are returned here
    pub tx: *mut gve_tx_ring,
}

// Parameters for allocating resources for rx queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_alloc_rings_cfg {
// tx config is also needed to determine QPL ids
    pub qcfg_rx: *mut gve_rx_queue_config,
    pub qcfg_tx: *mut gve_tx_queue_config,
    pub pages_per_qpl: u16,
    pub ring_size: u16,
    pub packet_buffer_size: u16,
    pub raw_addressing: bool,
    pub enable_header_split: bool,
    pub reset_rss: bool,
    pub xdp: bool,
// Allocated resources are returned here
    pub rx: *mut gve_rx_ring,
}

// GVE_QUEUE_FORMAT_UNSPECIFIED must be zero since 0 is the default value
// when the entire configure_device_resources command is zeroed out and the
// queue_format is not specified.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_queue_format {
    GVE_QUEUE_FORMAT_UNSPECIFIED	= 0x0,
    GVE_GQI_RDA_FORMAT		= 0x1,
    GVE_GQI_QPL_FORMAT		= 0x2,
    GVE_DQO_RDA_FORMAT		= 0x3,
    GVE_DQO_QPL_FORMAT		= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_flow_spec {
    pub src_ip: [__be32; 4],
    pub dst_ip: [__be32; 4],
    pub src_port: __be16,
    pub dst_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_flow_rule {
    pub location: u32,
    pub flow_type: u16,
    pub action: u16,
    pub key: gve_flow_spec,
    pub mask: gve_flow_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_flow_rules_cache {
    pub /: *mut *mut bool rules_cache_synced; / False if the driver's rules_cache is outdated,
    pub rules_cache: *mut gve_adminq_queried_flow_rule,
    pub rule_ids_cache: *mut __be32,
// The total number of queried rules that stored in the caches
    pub rules_cache_num: u32,
    pub rule_ids_cache_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rss_config {
    pub hash_key: *mut u8,
    pub hash_lut: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_ptp {
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub priv: *mut gve_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_priv {
    pub dev: *mut net_device,
    pub /: *mut *mut *mut gve_tx_ring tx; / array of tx_cfg.num_queues,
    pub /: *mut *mut *mut gve_rx_ring rx; / array of rx_cfg.num_queues,
    pub /: *mut *mut *mut gve_notify_block ntfy_blocks; / array of num_ntfy_blks,
    pub /: *mut *mut *mut gve_irq_db irq_db_indices; / array of num_ntfy_blks,
    pub irq_db_indices_bus: dma_addr_t,
    pub /: *mut *mut *mut msix_entry msix_vectors; / array of num_ntfy_blks + 1,
    pub 16]: char mgmt_msix_name[IFNAMSIZ +,
    pub mgmt_msix_idx: u32,
    pub /: *mut *mut *mut __be32 counter_array; / array of num_event_counters,
    pub counter_array_bus: dma_addr_t,
    pub num_event_counters: u16,
    pub /: *mut *mut u16 tx_desc_cnt; / num desc per ring,
    pub /: *mut *mut u16 rx_desc_cnt; / num desc per ring,
    pub max_tx_desc_cnt: u16,
    pub max_rx_desc_cnt: u16,
    pub min_tx_desc_cnt: u16,
    pub min_rx_desc_cnt: u16,
    pub modify_ring_size_enabled: bool,
    pub default_min_ring_size: bool,
    pub tx_pages_per_qpl: u16,
    pub rx_pages_per_qpl: u16,
    pub max_registered_pages: u64,
    pub /: *mut *mut u64 num_registered_pages; / num pages registered with NIC,
    pub /: *mut *mut *mut bpf_prog xdp_prog; / XDP BPF program,
    pub /: *mut *mut u32 rx_copybreak; / copy packets smaller than this,
    pub /: *mut *mut u16 default_num_queues; / default num queues to set up,
    pub tx_cfg: gve_tx_queue_config,
    pub rx_cfg: gve_rx_queue_config,
    pub /: *mut *mut *mut unsigned long xsk_pools; / bitmap of RX queues with XSK pools,
    pub /: *mut *mut u32 num_ntfy_blks; / split between TX and RX so must be even,
    pub numa_node: c_int,
    pub /: *mut *mut *mut gve_registers __iomem reg_bar0; / see gve_register.h,
    pub /: *mut *mut *mut __be32 __iomem db_bar2; / "array" of doorbells,
    pub /: *mut *mut *mut u32 msg_enable; / level for netif netdev print macros,
    pub pdev: *mut pci_dev,
// metrics
    pub tx_timeo_cnt: u32,
// Admin queue - see gve_adminq.h
    pub adminq: *mut gve_adminq_command,
    pub adminq_bus_addr: dma_addr_t,
    pub adminq_pool: *mut dma_pool,
    pub /: *mut *mut mutex adminq_lock; / Protects adminq command execution,
    pub /: *mut *mut u32 adminq_mask; / masks prod_cnt to adminq size,
    pub /: *mut *mut u32 adminq_prod_cnt; / free-running count of AQ cmds executed,
    pub /: *mut *mut u32 adminq_cmd_fail; / free-running count of AQ cmds failed,
    pub /: *mut *mut u32 adminq_timeouts; / free-running count of AQ cmds timeouts,
// free-running count of per AQ cmd executed
    pub adminq_describe_device_cnt: u32,
    pub adminq_cfg_device_resources_cnt: u32,
    pub adminq_register_page_list_cnt: u32,
    pub adminq_unregister_page_list_cnt: u32,
    pub adminq_create_tx_queue_cnt: u32,
    pub adminq_create_rx_queue_cnt: u32,
    pub adminq_destroy_tx_queue_cnt: u32,
    pub adminq_destroy_rx_queue_cnt: u32,
    pub adminq_dcfg_device_resources_cnt: u32,
    pub adminq_set_driver_parameter_cnt: u32,
    pub adminq_report_stats_cnt: u32,
    pub adminq_report_link_speed_cnt: u32,
    pub adminq_report_nic_timestamp_cnt: u32,
    pub adminq_get_ptype_map_cnt: u32,
    pub adminq_verify_driver_compatibility_cnt: u32,
    pub adminq_query_flow_rules_cnt: u32,
    pub adminq_cfg_flow_rule_cnt: u32,
    pub adminq_cfg_rss_cnt: u32,
    pub adminq_query_rss_cnt: u32,
// Global stats
    pub /: *mut *mut u32 interface_up_cnt; / count of times interface turned up since last reset,
    pub /: *mut *mut u32 interface_down_cnt; / count of times interface turned down since last reset,
    pub /: *mut *mut u32 reset_cnt; / count of reset,
    pub /: *mut *mut u32 page_alloc_fail; / count of page alloc fails,
    pub /: *mut *mut u32 dma_mapping_error; / count of dma mapping errors,
    pub /: *mut *mut u32 stats_report_trigger_cnt; / count of device-requested stats-reports since last reset,
    pub /: *mut *mut u32 suspend_cnt; / count of times suspended,
    pub /: *mut *mut u32 resume_cnt; / count of times resumed,
    pub gve_wq: *mut workqueue_struct,
    pub service_task: work_struct,
    pub stats_report_task: work_struct,
    pub service_task_flags: c_ulong,
    pub state_flags: c_ulong,
    pub stats_report: *mut gve_stats_report,
    pub stats_report_len: u64,
    pub /: *mut *mut dma_addr_t stats_report_bus; / dma address for the stats report,
    pub ethtool_flags: c_ulong,
    pub stats_report_timer_period: c_ulong,
    pub stats_report_timer: timer_list,
// Gvnic device link speed from hypervisor.
    pub link_speed: u64,
    pub /: *mut *mut bool up_before_suspend; / True if dev was up before suspend,
    pub ptype_lut_dqo: *mut gve_ptype_lut,
// Must be a power of two.
    pub /: *mut *mut u16 max_rx_buffer_size; / device limit,
    pub queue_format: gve_queue_format,
// Interrupt coalescing settings
    pub tx_coalesce_usecs: u32,
    pub rx_coalesce_usecs: u32,
    pub /: *mut *mut u16 header_buf_size; / device configured, header-split supported if non-zero,
    pub /: *mut *mut bool header_split_enabled; / True if the header split is enabled by the user,
    pub max_flow_rules: u32,
    pub num_flow_rules: u32,
    pub flow_rules_cache: gve_flow_rules_cache,
    pub rss_key_size: u16,
    pub rss_lut_size: u16,
    pub cache_rss_config: bool,
    pub rss_config: gve_rss_config,
// True if the device supports reading the nic clock
    pub nic_timestamp_supported: bool,
    pub ptp: *mut gve_ptp,
    pub ts_config: kernel_hwtstamp_config,
    pub nic_ts_report: *mut gve_nic_ts_report,
    pub nic_ts_report_bus: dma_addr_t,
    pub /: *mut *mut u64 last_sync_nic_counter; / Clock counter from last NIC TS report,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_service_task_flags_bit {
    GVE_PRIV_FLAGS_DO_RESET			= 1,
    GVE_PRIV_FLAGS_RESET_IN_PROGRESS	= 2,
    GVE_PRIV_FLAGS_PROBE_IN_PROGRESS	= 3,
    GVE_PRIV_FLAGS_DO_REPORT_STATS = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_state_flags_bit {
    GVE_PRIV_FLAGS_ADMIN_QUEUE_OK		= 1,
    GVE_PRIV_FLAGS_DEVICE_RESOURCES_OK	= 2,
    GVE_PRIV_FLAGS_DEVICE_RINGS_OK		= 3,
    GVE_PRIV_FLAGS_NAPI_ENABLED		= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_ethtool_flags_bit {
    GVE_PRIV_FLAGS_REPORT_STATS		= 0,
}

extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_DO_RESET, _arg: &priv->service_task_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_ADMIN_QUEUE_OK, _arg: &priv->state_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_DEVICE_RESOURCES_OK, _arg: &priv->state_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_DEVICE_RINGS_OK, _arg: &priv->state_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_NAPI_ENABLED, _arg: &priv->state_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GVE_PRIV_FLAGS_REPORT_STATS, _arg: &priv->ethtool_flags) -> return;
}
// Returns the address of the ntfy_blocks irq doorbell
//
// Returns the index into ntfy_blocks of the given tx ring's block
//
// Returns the index into ntfy_blocks of the given rx ring's block
//
// Returns the number of tx queue page lists
// Returns the number of rx queue page lists
extern "C" {
    pub fn gve_tx_qpl_id(_arg: priv, _arg: 0) -> return;
}
extern "C" {
    pub fn gve_get_rx_qpl_id(_arg: tx_cfg, _arg: 0) -> return;
}
// Returns the correct dma direction for tx and rx qpls
extern "C" {
    pub fn gve_xdp_tx_queue_id(_arg: priv, _arg: 0) -> return;
}
// gqi napi handler defined in gve_main.c
extern "C" {
    pub fn gve_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
// buffers
// qpls
// tx handling
extern "C" {
    pub fn gve_tx(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn gve_xdp_tx_flush(priv: *mut gve_priv, xdp_qid: u32);
}
extern "C" {
    pub fn gve_tx_poll(block: *mut gve_notify_block, budget: c_int) -> bool;
}
extern "C" {
    pub fn gve_xdp_poll(block: *mut gve_notify_block, budget: c_int) -> bool;
}
extern "C" {
    pub fn gve_xsk_tx_poll(block: *mut gve_notify_block, budget: c_int) -> c_int;
}
extern "C" {
    pub fn gve_tx_start_ring_gqi(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_tx_stop_ring_gqi(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_tx_clean_pending(priv: *mut gve_priv, tx: *mut gve_tx_ring) -> bool;
}
// rx handling
extern "C" {
    pub fn gve_rx_write_doorbell(priv: *mut gve_priv, rx: *mut gve_rx_ring);
}
extern "C" {
    pub fn gve_rx_poll(block: *mut gve_notify_block, budget: c_int) -> c_int;
}
extern "C" {
    pub fn gve_rx_work_pending(rx: *mut gve_rx_ring) -> bool;
}
extern "C" {
    pub fn gve_rx_start_ring_gqi(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_rx_stop_ring_gqi(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_header_split_supported(priv: *const gve_priv) -> bool;
}
// rx buffer handling
extern "C" {
    pub fn gve_buf_ref_cnt(bs: *mut gve_rx_buf_state_dqo) -> c_int;
}
extern "C" {
    pub fn gve_free_qpl_page_dqo(buf_state: *mut gve_rx_buf_state_dqo);
}
extern "C" {
    pub fn gve_alloc_buffer(rx: *mut gve_rx_ring, desc: *mut gve_rx_desc_dqo) -> c_int;
}
// Reset
extern "C" {
    pub fn gve_schedule_reset(priv: *mut gve_priv);
}
extern "C" {
    pub fn gve_reset(priv: *mut gve_priv, attempt_teardown: bool) -> c_int;
}
// flow steering rule
extern "C" {
    pub fn gve_get_flow_rule_entry(priv: *mut gve_priv, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn gve_get_flow_rule_ids(priv: *mut gve_priv, cmd: *mut ethtool_rxnfc, rule_locs: *mut u32) -> c_int;
}
extern "C" {
    pub fn gve_add_flow_rule(priv: *mut gve_priv, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn gve_del_flow_rule(priv: *mut gve_priv, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn gve_flow_rules_reset(priv: *mut gve_priv) -> c_int;
}
// RSS config
extern "C" {
    pub fn gve_init_rss_config(priv: *mut gve_priv, num_queues: u16) -> c_int;
}
// PTP and timestamping

extern "C" {
    pub fn gve_clock_nic_ts_read(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_init_clock(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_teardown_clock(priv: *mut gve_priv);
}

// report stats handling
extern "C" {
    pub fn gve_handle_report_stats(priv: *mut gve_priv);
}
// exported by ethtool.c
// needed by ethtool
