//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/net_driver.h
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
//
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2005-2013 Solarflare Communications Inc.
//
// Common definitions for all Efx net driver code

//
// Build definitions
//

//
// Efx data structures
//

pub const EFX_EXTRA_CHANNEL_IOV: c_int = 0;
pub const EFX_EXTRA_CHANNEL_PTP: c_int = 1;

// Checksum generation is a per-queue option in hardware, so each
// queue visible to the networking core is backed by two hardware TX
// queues.
pub const EFX_MAX_TX_TC: c_int = 2;

pub const EFX_TXQ_TYPES: c_int = 8;
// HIGHPRI is Siena-only, and INNER_CSUM is EF10, so no need for both
pub const EFX_MAX_TXQ_PER_CHANNEL: c_int = 4;

// Maximum possible MTU the driver supports

// Minimum MTU, from RFC791 (IP)
pub const EFX_MIN_MTU: c_int = 68;
// Maximum total header length for TSOv2
pub const EFX_TSO2_MAX_HDRLEN: c_int = 208;
// Size of an RX scatter buffer.  Small enough to pack 2 into a 4K page,
// and should be a multiple of the cache line size.
//

// If possible, we should ensure cache line alignment at start and end
// of every buffer.  Otherwise, we just need to ensure 4-byte
// alignment of the network header.
//

pub const EFX_RX_BUF_ALIGNMENT: c_int = 4;

// Non-standard XDP_PACKET_HEADROOM and tailroom to satisfy XDP_REDIRECT and
// still fit two standard MTU size packets into a single 4K page.
//
pub const EFX_XDP_HEADROOM: c_int = 128;

// Forward declare Precision Time Protocol (PTP) support structure.
//
// struct efx_buffer - A general-purpose DMA buffer
// @addr: host base address of the buffer
// @dma_addr: DMA base address of the buffer
// @len: Buffer length, in bytes
//
// The NIC uses these buffers for its interrupt status registers and
// MAC stats dumps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_buffer {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub len: c_uint,
}

//
// struct efx_special_buffer - DMA buffer entered into buffer table
// @buf: Standard &struct efx_buffer
// @index: Buffer index within controller;s buffer table
// @entries: Number of buffer table entries
//
// The NIC has a buffer table that maps buffers of size %EFX_BUF_SIZE.
// Event and descriptor rings are addressed via one or more buffer
// table entries (and so can be physically non-contiguous, although we
// currently do not take advantage of that).  On Falcon and Siena we
// have to take care of allocating and initialising the entries
// ourselves.  On later hardware this is managed by the firmware and
// @index and @entries are left as 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_special_buffer {
    pub buf: efx_buffer,
    pub index: c_uint,
    pub entries: c_uint,
}

//
// struct efx_tx_buffer - buffer state for a TX descriptor
// @skb: When @flags & %EFX_TX_BUF_SKB, the associated socket buffer to be
// freed when descriptor completes
// @xdpf: When @flags & %EFX_TX_BUF_XDP, the XDP frame information; its @data
// member is the associated buffer to drop a page reference on.
// @option: When @flags & %EFX_TX_BUF_OPTION, an EF10-specific option
// descriptor.
// @dma_addr: DMA address of the fragment.
// @flags: Flags for allocation and DMA mapping type
// @len: Length of this fragment.
// This field is zero when the queue slot is empty.
// @unmap_len: Length of this fragment to unmap
// @dma_offset: Offset of @dma_addr from the address of the backing DMA mapping.
// Only valid if @unmap_len != 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tx_buffer {
    pub skb: *const sk_buff,
    pub xdpf: *mut xdp_frame,
}

pub const EFX_TX_BUF_OPTION: c_uint = 0x10	/* empty buffer for option descriptor */;
pub const EFX_TX_BUF_XDP: c_uint = 0x20	/* buffer was sent with XDP */;
pub const EFX_TX_BUF_TSO_V3: c_uint = 0x40	/* empty buffer for a TSO_V3 descriptor */;
//
// struct efx_tx_queue - An Efx TX queue
//
// This is a ring buffer of TX fragments.
// Since the TX completion path always executes on the same
// CPU and the xmit path can operate on different CPUs,
// performance is increased by ensuring that the completion
// path and the xmit path operate on different cache lines.
// This is particularly important if the xmit path is always
// executing on one CPU which is different from the completion
// path.  There is also a cache line for members which are
// read but not written on the fast path.
//
// @efx: The associated Efx NIC
// @queue: DMA queue number
// @label: Label for TX completion events.
// Is our index within @channel->tx_queue array.
// @type: configuration type of this TX queue.  A bitmask of %EFX_TXQ_TYPE_* flags.
// @tso_version: Version of TSO in use for this queue.
// @tso_encap: Is encapsulated TSO supported? Supported in TSOv2 on 8000 series.
// @channel: The associated channel
// @core_txq: The networking core TX queue structure
// @buffer: The software buffer ring
// @cb_page: Array of pages of copy buffers.  Carved up according to
// %EFX_TX_CB_ORDER into %EFX_TX_CB_SIZE-sized chunks.
// @txd: The hardware descriptor ring
// @ptr_mask: The size of the ring minus 1.
// @piobuf: PIO buffer region for this TX queue (shared with its partner).
// @piobuf_offset: Buffer offset to be specified in PIO descriptors
// @initialised: Has hardware queue been initialised?
// @timestamping: Is timestamping enabled for this channel?
// @xdp_tx: Is this an XDP tx queue?
// @read_count: Current read pointer.
// This is the number of buffers that have been removed from both rings.
// @old_write_count: The value of @write_count when last checked.
// This is here for performance reasons.  The xmit path will
// only get the up-to-date value of @write_count if this
// variable indicates that the queue is empty.  This is to
// avoid cache-line ping-pong between the xmit path and the
// completion path.
// @merge_events: Number of TX merged completion events
// @completed_timestamp_major: Top part of the most recent tx timestamp.
// @completed_timestamp_minor: Low part of the most recent tx timestamp.
// @insert_count: Current insert pointer
// This is the number of buffers that have been added to the
// software ring.
// @write_count: Current write pointer
// This is the number of buffers that have been added to the
// hardware ring.
// @packet_write_count: Completable write pointer
// This is the write pointer of the last packet written.
// Normally this will equal @write_count, but as option descriptors
// don't produce completion events, they won't update this.
// Filled in iff @efx->type->option_descriptors; only used for PIO.
// Thus, this is written and used on EF10, and neither on farch.
// @old_read_count: The value of read_count when last checked.
// This is here for performance reasons.  The xmit path will
// only get the up-to-date value of read_count if this
// variable indicates that the queue is full.  This is to
// avoid cache-line ping-pong between the xmit path and the
// completion path.
// @tso_bursts: Number of times TSO xmit invoked by kernel
// @tso_long_headers: Number of packets with headers too long for standard
// blocks
// @tso_packets: Number of packets via the TSO xmit path
// @tso_fallbacks: Number of times TSO fallback used
// @pushes: Number of times the TX push feature has been used
// @pio_packets: Number of times the TX PIO feature has been used
// @xmit_pending: Are any packets waiting to be pushed to the NIC
// @cb_packets: Number of times the TX copybreak feature has been used
// @notify_count: Count of notified descriptors to the NIC
// @empty_read_count: If the completion path has seen the queue as empty
// and the transmission path has not yet checked this, the value of
// @read_count bitwise-added to %EFX_EMPTY_COUNT_VALID; otherwise 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tx_queue {
// Members which don't change on the fast path
    pub ____cacheline_aligned_in_smp: *mut *mut efx_nic efx,
    pub queue: c_uint,
    pub label: c_uint,
    pub type: c_uint,
    pub tso_version: c_uint,
    pub tso_encap: bool,
    pub channel: *mut efx_channel,
    pub core_txq: *mut netdev_queue,
    pub buffer: *mut efx_tx_buffer,
    pub cb_page: *mut efx_buffer,
    pub txd: efx_special_buffer,
    pub ptr_mask: c_uint,
    pub piobuf: *mut void __iomem,
    pub piobuf_offset: c_uint,
    pub initialised: bool,
    pub timestamping: bool,
    pub xdp_tx: bool,
// Members used mainly on the completion path
    pub ____cacheline_aligned_in_smp: unsigned int read_count,
    pub old_write_count: c_uint,
    pub merge_events: c_uint,
    pub bytes_compl: c_uint,
    pub pkts_compl: c_uint,
    pub completed_timestamp_major: u32,
    pub completed_timestamp_minor: u32,
// Members used only on the xmit path
    pub ____cacheline_aligned_in_smp: unsigned int insert_count,
    pub write_count: c_uint,
    pub packet_write_count: c_uint,
    pub old_read_count: c_uint,
    pub tso_bursts: c_uint,
    pub tso_long_headers: c_uint,
    pub tso_packets: c_uint,
    pub tso_fallbacks: c_uint,
    pub pushes: c_uint,
    pub pio_packets: c_uint,
    pub xmit_pending: bool,
    pub cb_packets: c_uint,
    pub notify_count: c_uint,
// Statistics to supplement MAC stats
    pub tx_packets: c_ulong,
// Members shared between paths and sometimes updated
    pub ____cacheline_aligned_in_smp: unsigned int empty_read_count,
pub const EFX_EMPTY_COUNT_VALID: c_uint = 0x80000000;
    pub flush_outstanding: core::sync::atomic::AtomicI32,
}

pub const EFX_TX_CB_ORDER: c_int = 7;

//
// struct efx_rx_buffer - An Efx RX data buffer
// @dma_addr: DMA base address of the buffer
// @page: The associated page buffer.
// Will be %NULL if the buffer slot is currently free.
// @page_offset: If pending: offset in @page of DMA base address.
// If completed: offset in @page of Ethernet header.
// @len: If pending: length for DMA descriptor.
// If completed: received length, excluding hash prefix.
// @flags: Flags for buffer and packet state.  These are only set on the
// first buffer of a scattered packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rx_buffer {
    pub dma_addr: dma_addr_t,
    pub page: *mut page,
    pub page_offset: u16,
    pub len: u16,
    pub flags: u16,
}

pub const EFX_RX_BUF_LAST_IN_PAGE: c_uint = 0x0001;
pub const EFX_RX_PKT_CSUMMED: c_uint = 0x0002;
pub const EFX_RX_PKT_DISCARD: c_uint = 0x0004;
pub const EFX_RX_PKT_TCP: c_uint = 0x0040;
pub const EFX_RX_PKT_PREFIX_LEN: c_uint = 0x0080	/* length is in prefix only */;
pub const EFX_RX_PKT_CSUM_LEVEL: c_uint = 0x0200;
//
// struct efx_rx_page_state - Page-based rx buffer state
//
// Inserted at the start of every page allocated for receive buffers.
// Used to facilitate sharing dma mappings between recycled rx buffers
// and those passed up to the kernel.
//
// @dma_addr: The dma address of this page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rx_page_state {
    pub dma_addr: dma_addr_t,
    pub ____cacheline_aligned: unsigned int __pad[],
}

//
// struct efx_rx_queue - An Efx RX queue
// @efx: The associated Efx NIC
// @core_index:  Index of network core RX queue.  Will be >= 0 iff this
// is associated with a real RX queue.
// @buffer: The software buffer ring
// @rxd: The hardware descriptor ring
// @ptr_mask: The size of the ring minus 1.
// @refill_enabled: Enable refill whenever fill level is low
// @flush_pending: Set when a RX flush is pending. Has the same lifetime as
// @rxq_flush_pending.
// @added_count: Number of buffers added to the receive queue.
// @notified_count: Number of buffers given to NIC (<= @added_count).
// @removed_count: Number of buffers removed from the receive queue.
// @scatter_n: Used by NIC specific receive code.
// @scatter_len: Used by NIC specific receive code.
// @page_ring: The ring to store DMA mapped pages for reuse.
// @page_add: Counter to calculate the write pointer for the recycle ring.
// @page_remove: Counter to calculate the read pointer for the recycle ring.
// @page_recycle_count: The number of pages that have been recycled.
// @page_recycle_failed: The number of pages that couldn't be recycled because
// the kernel still held a reference to them.
// @page_recycle_full: The number of pages that were released because the
// recycle ring was full.
// @page_ptr_mask: The number of pages in the RX recycle ring minus 1.
// @max_fill: RX descriptor maximum fill level (<= ring size)
// @fast_fill_trigger: RX descriptor fill level that will trigger a fast fill
// (<= @max_fill)
// @min_fill: RX descriptor minimum non-zero fill level.
// This records the minimum fill level observed when a ring
// refill was triggered.
// @recycle_count: RX buffer recycle counter.
// @slow_fill: Timer used to defer efx_nic_generate_fill_event().
// @xdp_rxq_info: XDP specific RX queue information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rx_queue {
    pub efx: *mut efx_nic,
    pub core_index: c_int,
    pub buffer: *mut efx_rx_buffer,
    pub rxd: efx_special_buffer,
    pub ptr_mask: c_uint,
    pub refill_enabled: bool,
    pub flush_pending: bool,
    pub added_count: c_uint,
    pub notified_count: c_uint,
    pub removed_count: c_uint,
    pub scatter_n: c_uint,
    pub scatter_len: c_uint,
    pub page_ring: *mut page,
    pub page_add: c_uint,
    pub page_remove: c_uint,
    pub page_recycle_count: c_uint,
    pub page_recycle_failed: c_uint,
    pub page_recycle_full: c_uint,
    pub page_ptr_mask: c_uint,
    pub max_fill: c_uint,
    pub fast_fill_trigger: c_uint,
    pub min_fill: c_uint,
    pub min_overfill: c_uint,
    pub recycle_count: c_uint,
    pub slow_fill: timer_list,
    pub slow_fill_count: c_uint,
// Statistics to supplement MAC stats
    pub rx_packets: c_ulong,
    pub xdp_rxq_info: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_sync_events_state {
    SYNC_EVENTS_DISABLED = 0,
    SYNC_EVENTS_QUIESCENT,
    SYNC_EVENTS_REQUESTED,
    SYNC_EVENTS_VALID,
}

//
// struct efx_channel - An Efx channel
//
// A channel comprises an event queue, at least one TX queue, at least
// one RX queue, and an associated tasklet for processing the event
// queue.
//
// @efx: Associated Efx NIC
// @channel: Channel instance number
// @type: Channel type definition
// @eventq_init: Event queue initialised flag
// @enabled: Channel enabled indicator
// @irq: IRQ number (MSI and MSI-X only)
// @irq_moderation_us: IRQ moderation value (in microseconds)
// @napi_dev: Net device used with NAPI
// @napi_str: NAPI control structure
// @state: state for NAPI vs busy polling
// @state_lock: lock protecting @state
// @eventq: Event queue buffer
// @eventq_mask: Event queue pointer mask
// @eventq_read_ptr: Event queue read pointer
// @event_test_cpu: Last CPU to handle interrupt or test event for this channel
// @irq_count: Number of IRQs since last adaptive moderation decision
// @irq_mod_score: IRQ moderation score
// @rfs_filter_count: number of accelerated RFS filters currently in place;
// equals the count of @rps_flow_id slots filled
// @rfs_last_expiry: value of jiffies last time some accelerated RFS filters
// were checked for expiry
// @rfs_expire_index: next accelerated RFS filter ID to check for expiry
// @n_rfs_succeeded: number of successful accelerated RFS filter insertions
// @n_rfs_failed: number of failed accelerated RFS filter insertions
// @filter_work: Work item for efx_filter_rfs_expire()
// @rps_flow_id: Flow IDs of filters allocated for accelerated RFS,
// indexed by filter ID
// @n_rx_tobe_disc: Count of RX_TOBE_DISC errors
// @n_rx_ip_hdr_chksum_err: Count of RX IP header checksum errors
// @n_rx_tcp_udp_chksum_err: Count of RX TCP and UDP checksum errors
// @n_rx_mcast_mismatch: Count of unmatched multicast frames
// @n_rx_frm_trunc: Count of RX_FRM_TRUNC errors
// @n_rx_overlength: Count of RX_OVERLENGTH errors
// @n_skbuff_leaks: Count of skbuffs leaked due to RX overrun
// @n_rx_nodesc_trunc: Number of RX packets truncated and then dropped due to
// lack of descriptors
// @n_rx_merge_events: Number of RX merged completion events
// @n_rx_merge_packets: Number of RX packets completed by merged events
// @n_rx_xdp_drops: Count of RX packets intentionally dropped due to XDP
// @n_rx_xdp_bad_drops: Count of RX packets dropped due to XDP errors
// @n_rx_xdp_tx: Count of RX packets retransmitted due to XDP
// @n_rx_xdp_redirect: Count of RX packets redirected to a different NIC by XDP
// @rx_pkt_n_frags: Number of fragments in next packet to be delivered by
// __efx_siena_rx_packet(), or zero if there is none
// @rx_pkt_index: Ring index of first buffer for next packet to be delivered
// by __efx_siena_rx_packet(), if @rx_pkt_n_frags != 0
// @rx_list: list of SKBs from current RX, awaiting processing
// @rx_queue: RX queue for this channel
// @tx_queue: TX queues for this channel
// @tx_queue_by_type: pointers into @tx_queue, or %NULL, indexed by txq type
// @sync_events_state: Current state of sync events on this channel
// @sync_timestamp_major: Major part of the last ptp sync event
// @sync_timestamp_minor: Minor part of the last ptp sync event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_channel {
    pub efx: *mut efx_nic,
    pub channel: c_int,
    pub type: *const efx_channel_type,
    pub eventq_init: bool,
    pub enabled: bool,
    pub irq: c_int,
    pub irq_moderation_us: c_uint,
    pub napi_dev: *mut net_device,
    pub napi_str: napi_struct,

    pub busy_poll_state: c_ulong,

    pub eventq: efx_special_buffer,
    pub eventq_mask: c_uint,
    pub eventq_read_ptr: c_uint,
    pub event_test_cpu: c_int,
    pub irq_count: c_uint,
    pub irq_mod_score: c_uint,

    pub rfs_filter_count: c_uint,
    pub rfs_last_expiry: c_uint,
    pub rfs_expire_index: c_uint,
    pub n_rfs_succeeded: c_uint,
    pub n_rfs_failed: c_uint,
    pub filter_work: delayed_work,
pub const RPS_FLOW_ID_INVALID: c_uint = 0xFFFFFFFF;
    pub rps_flow_id: *mut u32,

    pub n_rx_tobe_disc: c_uint,
    pub n_rx_ip_hdr_chksum_err: c_uint,
    pub n_rx_tcp_udp_chksum_err: c_uint,
    pub n_rx_outer_ip_hdr_chksum_err: c_uint,
    pub n_rx_outer_tcp_udp_chksum_err: c_uint,
    pub n_rx_inner_ip_hdr_chksum_err: c_uint,
    pub n_rx_inner_tcp_udp_chksum_err: c_uint,
    pub n_rx_eth_crc_err: c_uint,
    pub n_rx_mcast_mismatch: c_uint,
    pub n_rx_frm_trunc: c_uint,
    pub n_rx_overlength: c_uint,
    pub n_skbuff_leaks: c_uint,
    pub n_rx_nodesc_trunc: c_uint,
    pub n_rx_merge_events: c_uint,
    pub n_rx_merge_packets: c_uint,
    pub n_rx_xdp_drops: c_uint,
    pub n_rx_xdp_bad_drops: c_uint,
    pub n_rx_xdp_tx: c_uint,
    pub n_rx_xdp_redirect: c_uint,
    pub rx_pkt_n_frags: c_uint,
    pub rx_pkt_index: c_uint,
    pub rx_list: *mut list_head,
    pub rx_queue: efx_rx_queue,
    pub tx_queue: [efx_tx_queue; EFX_MAX_TXQ_PER_CHANNEL],
    pub tx_queue_by_type: [*mut efx_tx_queue; EFX_TXQ_TYPES],
    pub sync_events_state: efx_sync_events_state,
    pub sync_timestamp_major: u32,
    pub sync_timestamp_minor: u32,
}

//
// struct efx_msi_context - Context for each MSI
// @efx: The associated NIC
// @index: Index of the channel/IRQ
// @name: Name of the channel/IRQ
//
// Unlike &struct efx_channel, this is never reallocated and is always
// safe for the IRQ handler to access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_msi_context {
    pub efx: *mut efx_nic,
    pub index: c_uint,
    pub 6]: char name[IFNAMSIZ +,
}

//
// struct efx_channel_type - distinguishes traffic and extra channels
// @handle_no_channel: Handle failure to allocate an extra channel
// @pre_probe: Set up extra state prior to initialisation
// @post_remove: Tear down extra state after finalisation, if allocated.
// May be called on channels that have not been probed.
// @get_name: Generate the channel's name (used for its IRQ handler)
// @copy: Copy the channel state prior to reallocation.  May be %NULL if
// reallocation is not supported.
// @receive_skb: Handle an skb ready to be passed to netif_receive_skb()
// @want_txqs: Determine whether this channel should have TX queues
// created.  If %NULL, TX queues are not created.
// @keep_eventq: Flag for whether event queue should be kept initialised
// while the device is stopped
// @want_pio: Flag for whether PIO buffers should be linked to this
// channel's TX queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_channel_type {
    pub ): *mut *mut void (handle_no_channel)(struct efx_nic,
    pub ): *mut *mut int (pre_probe)(struct efx_channel,
    pub ): *mut *mut void (post_remove)(struct efx_channel,
    pub len): *mut *mut *mut *mut void (get_name)(struct efx_channel , char buf, size_t,
    pub ): *const *const *const efx_channel (copy)(efx_channel,
    pub ): *mut *mut *mut bool (receive_skb)(struct efx_channel , struct sk_buff,
    pub ): *mut *mut bool (want_txqs)(struct efx_channel,
    pub keep_eventq: bool,
    pub want_pio: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_led_mode {
    EFX_LED_OFF	= 0,
    EFX_LED_ON	= 1,
    EFX_LED_DEFAULT	= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_int_mode {
// Be careful if altering to correct macro below
    EFX_INT_MODE_MSIX = 0,
    EFX_INT_MODE_MSI = 1,
    EFX_INT_MODE_LEGACY = 2,
    EFX_INT_MODE_MAX	/* Insert any new items before this */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nic_state {
    STATE_UNINIT = 0,	/* device being probed/removed or is frozen */
    STATE_READY = 1,	/* hardware ready and netdev registered */
    STATE_DISABLED = 2,	/* device disabled due to hardware errors */
    STATE_RECOVERY = 3,	/* device recovering from PCI error */
}

// Forward declaration
// Pseudo bit-mask flow control field

pub const EFX_FC_AUTO: c_int = 4;
//
// struct efx_link_state - Current state of the link
// @up: Link is up
// @fd: Link is full-duplex
// @fc: Actual flow control flags
// @speed: Link speed (Mbps)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_link_state {
    pub up: bool,
    pub fd: bool,
    pub fc: u8,
    pub speed: c_uint,
}

//
// enum efx_phy_mode - PHY operating mode flags
// @PHY_MODE_NORMAL: on and should pass traffic
// @PHY_MODE_TX_DISABLED: on with TX disabled
// @PHY_MODE_LOW_POWER: set to low power through MDIO
// @PHY_MODE_OFF: switched off through external control
// @PHY_MODE_SPECIAL: on but will not pass traffic
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_phy_mode {
    PHY_MODE_NORMAL		= 0,
    PHY_MODE_TX_DISABLED	= 1,
    PHY_MODE_LOW_POWER	= 2,
    PHY_MODE_OFF		= 4,
    PHY_MODE_SPECIAL	= 8,
}

//
// struct efx_hw_stat_desc - Description of a hardware statistic
// @name: Name of the statistic as visible through ethtool, or %NULL if
// it should not be exposed
// @dma_width: Width in bits (0 for non-DMA statistics)
// @offset: Offset within stats (ignored for non-DMA statistics)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_hw_stat_desc {
    pub name: *const c_char,
    pub dma_width: u16,
    pub offset: u16,
}

// Number of bits used in a multicast filter hash address
pub const EFX_MCAST_HASH_BITS: c_int = 8;
// Number of (single-bit) entries in a multicast filter hash

// An Efx multicast filter hash
#[repr(C)]
#[derive(Copy, Clone)]
pub union efx_multicast_hash {
    pub 8]: u8 byte[EFX_MCAST_HASH_ENTRIES /,
    pub 8]: efx_oword_t oword[EFX_MCAST_HASH_ENTRIES / sizeof(efx_oword_t) /,
}

// The reserved RSS context value
pub const EFX_MCDI_RSS_CONTEXT_INVALID: c_uint = 0xffffffff;
//
// struct efx_rss_context - An RSS context for filtering
// @context_id: 0 if RSS is active, else %EFX_MCDI_RSS_CONTEXT_INVALID.
// @rx_hash_udp_4tuple: UDP 4-tuple hashing enabled
// @rx_hash_key: Toeplitz hash key for this RSS context
// @indir_table: Indirection table for this RSS context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rss_context {
    pub context_id: u32,
    pub rx_hash_udp_4tuple: bool,
    pub rx_hash_key: [u8; 40],
    pub rx_indir_table: [u32; 128],
}

// Order of these is important, since filter_id >= %EFX_ARFS_FILTER_ID_PENDING
// is used to test if filter does or will exist.
//

//
// struct efx_arfs_rule - record of an ARFS filter and its IDs
// @node: linkage into hash table
// @spec: details of the filter (used as key for hash table).  Use efx->type to
// determine which member to use.
// @rxq_index: channel to which the filter will steer traffic.
// @arfs_id: filter ID which was returned to ARFS
// @filter_id: index in software filter table.  May be
// %EFX_ARFS_FILTER_ID_PENDING if filter was not inserted yet,
// %EFX_ARFS_FILTER_ID_ERROR if filter insertion failed, or
// %EFX_ARFS_FILTER_ID_REMOVING if expiry is currently removing the filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_arfs_rule {
    pub node: hlist_node,
    pub spec: efx_filter_spec,
    pub rxq_index: u16,
    pub arfs_id: u16,
    pub filter_id: i32,
}

// Size chosen so that the table is one page (4kB)
pub const EFX_ARFS_HASH_TABLE_SIZE: c_int = 512;
//
// struct efx_async_filter_insertion - Request to asynchronously insert a filter
// @net_dev: Reference to the netdevice
// @net_dev_tracker: reference tracker entry for @net_dev
// @spec: The filter to insert
// @work: Workitem for this request
// @rxq_index: Identifies the channel for which this request was made
// @flow_id: Identifies the kernel-side flow for which this request was made
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_async_filter_insertion {
    pub net_dev: *mut net_device,
    pub net_dev_tracker: netdevice_tracker,
    pub spec: efx_filter_spec,
    pub work: work_struct,
    pub rxq_index: u16,
    pub flow_id: u32,
}

// Maximum number of ARFS workitems that may be in flight on an efx_nic
pub const EFX_RPS_MAX_IN_FLIGHT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_xdp_tx_queues_mode {
    EFX_XDP_TX_QUEUES_DEDICATED,	/* one queue per core, locking not needed */
    EFX_XDP_TX_QUEUES_SHARED,	/* each queue used by more than 1 core */
    EFX_XDP_TX_QUEUES_BORROWED	/* queues borrowed from net stack */
}

//
// struct efx_nic - an Efx NIC
// @name: Device name (net device name or bus id before net device registered)
// @pci_dev: The PCI device
// @node: List node for maintaning primary/secondary function lists
// @primary: &struct efx_nic instance for the primary function of this
// controller.  May be the same structure, and may be %NULL if no
// primary function is bound.  Serialised by rtnl_lock.
// @secondary_list: List of &struct efx_nic instances for the secondary PCI
// functions of the controller, if this is for the primary function.
// Serialised by rtnl_lock.
// @type: Controller type attributes
// @legacy_irq: IRQ number
// @workqueue: Workqueue for port reconfigures and the HW monitor.
// Work items do not hold and must not acquire RTNL.
// @workqueue_name: Name of workqueue
// @reset_work: Scheduled reset workitem
// @membase_phys: Memory BAR value as physical address
// @membase: Memory BAR value
// @vi_stride: step between per-VI registers / memory regions
// @interrupt_mode: Interrupt mode
// @timer_quantum_ns: Interrupt timer quantum, in nanoseconds
// @timer_max_ns: Interrupt timer maximum value, in nanoseconds
// @irq_rx_adaptive: Adaptive IRQ moderation enabled for RX event queues
// @irqs_hooked: Channel interrupts are hooked
// @irq_rx_mod_step_us: Step size for IRQ moderation for RX event queues
// @irq_rx_moderation_us: IRQ moderation time for RX event queues
// @msg_enable: Log message enable flags
// @state: Device state number (%STATE_*). Serialised by the rtnl_lock.
// @reset_pending: Bitmask for pending resets
// @tx_queue: TX DMA queues
// @rx_queue: RX DMA queues
// @channel: Channels
// @msi_context: Context for each MSI
// @extra_channel_types: Types of extra (non-traffic) channels that
// should be allocated for this NIC
// @xdp_tx_queue_count: Number of entries in %xdp_tx_queues.
// @xdp_tx_queues: Array of pointers to tx queues used for XDP transmit.
// @xdp_txq_queues_mode: XDP TX queues sharing strategy.
// @rxq_entries: Size of receive queues requested by user.
// @txq_entries: Size of transmit queues requested by user.
// @txq_stop_thresh: TX queue fill level at or above which we stop it.
// @txq_wake_thresh: TX queue fill level at or below which we wake it.
// @tx_dc_base: Base qword address in SRAM of TX queue descriptor caches
// @rx_dc_base: Base qword address in SRAM of RX queue descriptor caches
// @sram_lim_qw: Qword address limit of SRAM
// @next_buffer_table: First available buffer table id
// @n_channels: Number of channels in use
// @n_rx_channels: Number of channels used for RX (= number of RX queues)
// @n_tx_channels: Number of channels used for TX
// @n_extra_tx_channels: Number of extra channels with TX queues
// @tx_queues_per_channel: number of TX queues probed on each channel
// @n_xdp_channels: Number of channels used for XDP TX
// @xdp_channel_offset: Offset of zeroth channel used for XPD TX.
// @xdp_tx_per_channel: Max number of TX queues on an XDP TX channel.
// @rx_ip_align: RX DMA address offset to have IP header aligned in
// accordance with NET_IP_ALIGN
// @rx_dma_len: Current maximum RX DMA length
// @rx_buffer_order: Order (log2) of number of pages for each RX buffer
// @rx_buffer_truesize: Amortised allocation size of an RX buffer,
// for use in sk_buff::truesize
// @rx_prefix_size: Size of RX prefix before packet data
// @rx_packet_hash_offset: Offset of RX flow hash from start of packet data
// (valid only if @rx_prefix_size != 0; always negative)
// @rx_packet_len_offset: Offset of RX packet length from start of packet data
// (valid only for NICs that set %EFX_RX_PKT_PREFIX_LEN; always negative)
// @rx_packet_ts_offset: Offset of timestamp from start of packet data
// (valid only if channel->sync_timestamps_enabled; always negative)
// @rx_scatter: Scatter mode enabled for receives
// @rss_context: Main RSS context
// @vport_id: The function's vport ID, only relevant for PFs
// @int_error_count: Number of internal errors seen recently
// @int_error_expire: Time at which error count will be expired
// @must_realloc_vis: Flag: VIs have yet to be reallocated after MC reboot
// @irq_soft_enabled: Are IRQs soft-enabled? If not, IRQ handler will
// acknowledge but do nothing else.
// @irq_status: Interrupt status buffer
// @irq_zero_count: Number of legacy IRQs seen with queue flags == 0
// @irq_level: IRQ level/index for IRQs not triggered by an event queue
// @selftest_work: Work item for asynchronous self-test
// @mtd_list: List of MTDs attached to the NIC
// @nic_data: Hardware dependent state
// @mcdi: Management-Controller-to-Driver Interface state
// @mac_lock: MAC access lock. Protects @port_enabled, @phy_mode,
// efx_monitor() and efx_siena_reconfigure_port()
// @port_enabled: Port enabled indicator.
// Serialises efx_siena_stop_all(), efx_siena_start_all(),
// efx_monitor() and efx_mac_work() with kernel interfaces.
// Safe to read under any one of the rtnl_lock, mac_lock, or netif_tx_lock,
// but all three must be held to modify it.
// @port_initialized: Port initialized?
// @net_dev: Operating system network device. Consider holding the rtnl lock
// @fixed_features: Features which cannot be turned off
// @num_mac_stats: Number of MAC stats reported by firmware (MAC_STATS_NUM_STATS
// field of %MC_CMD_GET_CAPABILITIES_V4 response, or %MC_CMD_MAC_NSTATS)
// @stats_buffer: DMA buffer for statistics
// @phy_type: PHY type
// @phy_data: PHY private data (including PHY-specific stats)
// @mdio: PHY MDIO interface
// @mdio_bus: PHY MDIO bus ID (only used by Siena)
// @phy_mode: PHY operating mode. Serialised by @mac_lock.
// @link_advertising: Autonegotiation advertising flags
// @fec_config: Forward Error Correction configuration flags.  For bit positions
// see &enum ethtool_fec_config_bits.
// @link_state: Current state of the link
// @n_link_state_changes: Number of times the link has changed state
// @unicast_filter: Flag for Falcon-arch simple unicast filter.
// Protected by @mac_lock.
// @multicast_hash: Multicast hash table for Falcon-arch.
// Protected by @mac_lock.
// @wanted_fc: Wanted flow control flags
// @fc_disable: When non-zero flow control is disabled. Typically used to
// ensure that network back pressure doesn't delay dma queue flushes.
// Serialised by the rtnl lock.
// @mac_work: Work item for changing MAC promiscuity and multicast hash
// @loopback_mode: Loopback status
// @loopback_modes: Supported loopback mode bitmask
// @loopback_selftest: Offline self-test private state
// @xdp_prog: Current XDP programme for this interface
// @filter_sem: Filter table rw_semaphore, protects existence of @filter_state
// @filter_state: Architecture-dependent filter table state
// @rps_mutex: Protects RPS state of all channels
// @rps_slot_map: bitmap of in-flight entries in @rps_slot
// @rps_slot: array of ARFS insertion requests for efx_filter_rfs_work()
// @rps_hash_lock: Protects ARFS filter mapping state (@rps_hash_table and
// @rps_next_id).
// @rps_hash_table: Mapping between ARFS filters and their various IDs
// @rps_next_id: next arfs_id for an ARFS filter
// @active_queues: Count of RX and TX queues that haven't been flushed and drained.
// @rxq_flush_pending: Count of number of receive queues that need to be flushed.
// Decremented when the efx_flush_rx_queue() is called.
// @rxq_flush_outstanding: Count of number of RX flushes started but not yet
// completed (either success or failure). Not used when MCDI is used to
// flush receive queues.
// @flush_wq: wait queue used by efx_nic_flush_queues() to wait for flush completions.
// @vf_count: Number of VFs intended to be enabled.
// @vf_init_count: Number of VFs that have been fully initialised.
// @vi_scale: log2 number of vnics per VF.
// @ptp_data: PTP state data
// @ptp_warned: has this NIC seen and warned about unexpected PTP events?
// @vpd_sn: Serial number read from VPD
// @xdp_rxq_info_failed: Have any of the rx queues failed to initialise their
// xdp_rxq_info structures?
// @netdev_notifier: Netdevice notifier.
// @mem_bar: The BAR that is mapped into membase.
// @reg_base: Offset from the start of the bar to the function control window.
// @monitor_work: Hardware monitor workitem
// @biu_lock: BIU (bus interface unit) lock
// @last_irq_cpu: Last CPU to handle a possible test interrupt.  This
// field is used by efx_test_interrupts() to verify that an
// interrupt has occurred.
// @stats_lock: Statistics update lock. Must be held when calling
// efx_nic_type::{update,start,stop}_stats.
// @n_rx_noskb_drops: Count of RX packets dropped due to failure to allocate an skb
//
// This is stored in the private area of the &struct net_device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_nic {
// The following fields should be written very rarely
    pub name: [c_char; IFNAMSIZ],
    pub node: list_head,
    pub primary: *mut efx_nic,
    pub secondary_list: list_head,
    pub pci_dev: *mut pci_dev,
    pub port_num: c_uint,
    pub type: *const efx_nic_type,
    pub legacy_irq: c_int,
    pub eeh_disabled_legacy_irq: bool,
    pub workqueue: *mut workqueue_struct,
    pub workqueue_name: [c_char; 16],
    pub reset_work: work_struct,
    pub membase_phys: resource_size_t,
    pub membase: *mut void __iomem,
    pub vi_stride: c_uint,
    pub interrupt_mode: efx_int_mode,
    pub timer_quantum_ns: c_uint,
    pub timer_max_ns: c_uint,
    pub irq_rx_adaptive: bool,
    pub irqs_hooked: bool,
    pub irq_mod_step_us: c_uint,
    pub irq_rx_moderation_us: c_uint,
    pub msg_enable: u32,
    pub state: nic_state,
    pub reset_pending: c_ulong,
    pub channel: [*mut efx_channel; EFX_MAX_CHANNELS],
    pub msi_context: [efx_msi_context; EFX_MAX_CHANNELS],
    pub xdp_tx_queue_count: c_uint,
    pub xdp_tx_queues: *mut efx_tx_queue,
    pub xdp_txq_queues_mode: efx_xdp_tx_queues_mode,
    pub rxq_entries: unsigned,
    pub txq_entries: unsigned,
    pub txq_stop_thresh: c_uint,
    pub txq_wake_thresh: c_uint,
    pub tx_dc_base: unsigned,
    pub rx_dc_base: unsigned,
    pub sram_lim_qw: unsigned,
    pub next_buffer_table: unsigned,
    pub max_channels: c_uint,
    pub max_vis: c_uint,
    pub max_tx_channels: c_uint,
    pub n_channels: unsigned,
    pub n_rx_channels: unsigned,
    pub rss_spread: unsigned,
    pub tx_channel_offset: unsigned,
    pub n_tx_channels: unsigned,
    pub n_extra_tx_channels: unsigned,
    pub tx_queues_per_channel: c_uint,
    pub n_xdp_channels: c_uint,
    pub xdp_channel_offset: c_uint,
    pub xdp_tx_per_channel: c_uint,
    pub rx_ip_align: c_uint,
    pub rx_dma_len: c_uint,
    pub rx_buffer_order: c_uint,
    pub rx_buffer_truesize: c_uint,
    pub rx_page_buf_step: c_uint,
    pub rx_bufs_per_page: c_uint,
    pub rx_pages_per_batch: c_uint,
    pub rx_prefix_size: c_uint,
    pub rx_packet_hash_offset: c_int,
    pub rx_packet_len_offset: c_int,
    pub rx_packet_ts_offset: c_int,
    pub rx_scatter: bool,
    pub rss_context: efx_rss_context,
    pub vport_id: u32,
    pub int_error_count: unsigned,
    pub int_error_expire: c_ulong,
    pub must_realloc_vis: bool,
    pub irq_soft_enabled: bool,
    pub irq_status: efx_buffer,
    pub irq_zero_count: unsigned,
    pub irq_level: unsigned,
    pub selftest_work: delayed_work,

    pub mtd_list: list_head,

    pub nic_data: *mut c_void,
    pub mcdi: *mut efx_mcdi_data,
    pub mac_lock: mutex,
    pub mac_work: work_struct,
    pub port_enabled: bool,
    pub mc_bist_for_other_fn: bool,
    pub port_initialized: bool,
    pub net_dev: *mut net_device,
    pub fixed_features: netdev_features_t,
    pub num_mac_stats: u16,
    pub stats_buffer: efx_buffer,
    pub rx_nodesc_drops_total: u64,
    pub rx_nodesc_drops_while_down: u64,
    pub rx_nodesc_drops_prev_state: bool,
    pub phy_type: c_uint,
    pub phy_data: *mut c_void,
    pub mdio: mdio_if_info,
    pub mdio_bus: c_uint,
    pub phy_mode: efx_phy_mode,
    pub fec_config: u32,
    pub link_state: efx_link_state,
    pub n_link_state_changes: c_uint,
    pub unicast_filter: bool,
    pub multicast_hash: efx_multicast_hash,
    pub wanted_fc: u8,
    pub fc_disable: unsigned,
    pub rx_reset: core::sync::atomic::AtomicI32,
    pub loopback_mode: efx_loopback_mode,
    pub loopback_modes: u64,
    pub loopback_selftest: *mut c_void,
// We access loopback_selftest immediately before running XDP,
// so we want them next to each other.
//
    pub xdp_prog: *mut bpf_prog __rcu,
    pub filter_sem: rw_semaphore,
    pub filter_state: *mut c_void,

    pub rps_mutex: mutex,
    pub rps_slot_map: c_ulong,
    pub rps_slot: [efx_async_filter_insertion; EFX_RPS_MAX_IN_FLIGHT],
    pub rps_hash_lock: spinlock_t,
    pub rps_hash_table: *mut hlist_head,
    pub rps_next_id: u32,

    pub active_queues: core::sync::atomic::AtomicI32,
    pub rxq_flush_pending: core::sync::atomic::AtomicI32,
    pub rxq_flush_outstanding: core::sync::atomic::AtomicI32,
    pub flush_wq: wait_queue_head_t,

    pub vf_count: unsigned,
    pub vf_init_count: unsigned,
    pub vi_scale: unsigned,

    pub ptp_data: *mut efx_ptp_data,
    pub ptp_warned: bool,
    pub vpd_sn: *mut c_char,
    pub xdp_rxq_info_failed: bool,
    pub netdev_notifier: notifier_block,
    pub mem_bar: c_uint,
    pub reg_base: u32,
// The following fields may be written more often
    pub ____cacheline_aligned_in_smp: delayed_work monitor_work,
    pub biu_lock: spinlock_t,
    pub last_irq_cpu: c_int,
    pub stats_lock: spinlock_t,
    pub n_rx_noskb_drops: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mtd_partition {
    pub node: list_head,
    pub mtd: mtd_info,
    pub dev_type_name: *const c_char,
    pub type_name: *const c_char,
    pub 20]: char name[IFNAMSIZ +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_udp_tunnel {
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_INVALID: c_uint = 0xffff;
    pub /: *mut *mut u16 type; / TUNNEL_ENCAP_UDP_PORT_ENTRY_foo, see mcdi_pcol.h,
    pub port: __be16,
}

//
// struct efx_nic_type - Efx device type definition
// @mem_bar: Get the memory BAR
// @mem_map_size: Get memory BAR mapped size
// @probe: Probe the controller
// @remove: Free resources allocated by probe()
// @init: Initialise the controller
// @dimension_resources: Dimension controller resources (buffer table,
// and VIs once the available interrupt resources are clear)
// @fini: Shut down the controller
// @monitor: Periodic function for polling link state and hardware monitor
// @map_reset_reason: Map ethtool reset reason to a reset method
// @map_reset_flags: Map ethtool reset flags to a reset method, if possible
// @reset: Reset the controller hardware and possibly the PHY.  This will
// be called while the controller is uninitialised.
// @probe_port: Probe the MAC and PHY
// @remove_port: Free resources allocated by probe_port()
// @handle_global_event: Handle a "global" event (may be %NULL)
// @fini_dmaq: Flush and finalise DMA queues (RX and TX queues)
// @prepare_flush: Prepare the hardware for flushing the DMA queues
// (for Falcon architecture)
// @finish_flush: Clean up after flushing the DMA queues (for Falcon
// architecture)
// @prepare_flr: Prepare for an FLR
// @finish_flr: Clean up after an FLR
// @describe_stats: Describe statistics for ethtool
// @update_stats: Update statistics not provided by event handling.
// Either argument may be %NULL.
// @update_stats_atomic: Update statistics while in atomic context, if that
// is more limiting than @update_stats.  Otherwise, leave %NULL and
// driver core will call @update_stats.
// @start_stats: Start the regular fetching of statistics
// @pull_stats: Pull stats from the NIC and wait until they arrive.
// @stop_stats: Stop the regular fetching of statistics
// @push_irq_moderation: Apply interrupt moderation value
// @reconfigure_port: Push loopback/power/txdis changes to the MAC and PHY
// @prepare_enable_fc_tx: Prepare MAC to enable pause frame TX (may be %NULL)
// @reconfigure_mac: Push MAC address, MTU, flow control and filter settings
// to the hardware.  Serialised by the mac_lock.
// @check_mac_fault: Check MAC fault state. True if fault present.
// @get_wol: Get WoL configuration from driver state
// @set_wol: Push WoL configuration to the NIC
// @resume_wol: Synchronise WoL state between driver and MC (e.g. after resume)
// @get_fec_stats: Get standard FEC statistics.
// @test_chip: Test registers.  May use efx_farch_test_registers(), and is
// expected to reset the NIC.
// @test_nvram: Test validity of NVRAM contents
// @mcdi_request: Send an MCDI request with the given header and SDU.
// The SDU length may be any value from 0 up to the protocol-
// defined maximum, but its buffer will be padded to a multiple
// of 4 bytes.
// @mcdi_poll_response: Test whether an MCDI response is available.
// @mcdi_read_response: Read the MCDI response PDU.  The offset will
// be a multiple of 4.  The length may not be, but the buffer
// will be padded so it is safe to round up.
// @mcdi_poll_reboot: Test whether the MCDI has rebooted.  If so,
// return an appropriate error code for aborting any current
// request; otherwise return 0.
// @irq_enable_master: Enable IRQs on the NIC.  Each event queue must
// be separately enabled after this.
// @irq_test_generate: Generate a test IRQ
// @irq_disable_non_ev: Disable non-event IRQs on the NIC.  Each event
// queue must be separately disabled before this.
// @irq_handle_msi: Handle MSI for a channel.  The @dev_id argument is
// a pointer to the &struct efx_msi_context for the channel.
// @irq_handle_legacy: Handle legacy interrupt.  The @dev_id argument
// is a pointer to the &struct efx_nic.
// @tx_probe: Allocate resources for TX queue (and select TXQ type)
// @tx_init: Initialise TX queue on the NIC
// @tx_remove: Free resources for TX queue
// @tx_write: Write TX descriptors and doorbell
// @tx_enqueue: Add an SKB to TX queue
// @rx_push_rss_config: Write RSS hash key and indirection table to the NIC
// @rx_pull_rss_config: Read RSS hash key and indirection table back from the NIC
// @rx_probe: Allocate resources for RX queue
// @rx_init: Initialise RX queue on the NIC
// @rx_remove: Free resources for RX queue
// @rx_write: Write RX descriptors and doorbell
// @rx_defer_refill: Generate a refill reminder event
// @rx_packet: Receive the queued RX buffer on a channel
// @rx_buf_hash_valid: Determine whether the RX prefix contains a valid hash
// @ev_probe: Allocate resources for event queue
// @ev_init: Initialise event queue on the NIC
// @ev_fini: Deinitialise event queue on the NIC
// @ev_remove: Free resources for event queue
// @ev_process: Process events for a queue, up to the given NAPI quota
// @ev_read_ack: Acknowledge read events on a queue, rearming its IRQ
// @ev_test_generate: Generate a test event
// @filter_table_probe: Probe filter capabilities and set up filter software state
// @filter_table_restore: Restore filters removed from hardware
// @filter_table_remove: Remove filters from hardware and tear down software state
// @filter_update_rx_scatter: Update filters after change to rx scatter setting
// @filter_insert: add or replace a filter
// @filter_remove_safe: remove a filter by ID, carefully
// @filter_get_safe: retrieve a filter by ID, carefully
// @filter_clear_rx: Remove all RX filters whose priority is less than or
// equal to the given priority and is not %EFX_FILTER_PRI_AUTO
// @filter_count_rx_used: Get the number of filters in use at a given priority
// @filter_get_rx_id_limit: Get maximum value of a filter id, plus 1
// @filter_get_rx_ids: Get list of RX filters at a given priority
// @filter_rfs_expire_one: Consider expiring a filter inserted for RFS.
// This must check whether the specified table entry is used by RFS
// and that rps_may_expire_flow() returns true for it.
// @mtd_probe: Probe and add MTD partitions associated with this net device,
// using efx_siena_mtd_add()
// @mtd_rename: Set an MTD partition name using the net device name
// @mtd_read: Read from an MTD partition
// @mtd_erase: Erase part of an MTD partition
// @mtd_write: Write to an MTD partition
// @mtd_sync: Wait for write-back to complete on MTD partition.  This
// also notifies the driver that a writer has finished using this
// partition.
// @ptp_write_host_time: Send host time to MC as part of sync protocol
// @ptp_set_ts_sync_events: Enable or disable sync events for inline RX
// timestamping, possibly only temporarily for the purposes of a reset.
// @ptp_set_ts_config: Set hardware timestamp configuration.  The flags
// and tx_type will already have been validated but this operation
// must validate and update rx_filter.
// @get_phys_port_id: Get the underlying physical port id.
// @set_mac_address: Set the MAC address of the device
// @tso_versions: Returns mask of firmware-assisted TSO versions supported.
// If %NULL, then device does not support any TSO version.
// @udp_tnl_push_ports: Push the list of UDP tunnel ports to the NIC if required.
// @udp_tnl_has_port: Check if a port has been added as UDP tunnel
// @print_additional_fwver: Dump NIC-specific additional FW version info
// @sensor_event: Handle a sensor event from MCDI
// @rx_recycle_ring_size: Size of the RX recycle ring
// @revision: Hardware architecture revision
// @txd_ptr_tbl_base: TX descriptor ring base address
// @rxd_ptr_tbl_base: RX descriptor ring base address
// @buf_tbl_base: Buffer table base address
// @evq_ptr_tbl_base: Event queue pointer table base address
// @evq_rptr_tbl_base: Event queue read-pointer table base address
// @max_dma_mask: Maximum possible DMA mask
// @rx_prefix_size: Size of RX prefix before packet data
// @rx_hash_offset: Offset of RX flow hash within prefix
// @rx_ts_offset: Offset of timestamp within prefix
// @rx_buffer_padding: Size of padding at end of RX packet
// @can_rx_scatter: NIC is able to scatter packets to multiple buffers
// @always_rx_scatter: NIC will always scatter packets to multiple buffers
// @option_descriptors: NIC supports TX option descriptors
// @min_interrupt_mode: Lowest capability interrupt mode supported
// from &enum efx_int_mode.
// @timer_period_max: Maximum period of interrupt timer (in ticks)
// @offload_features: net_device feature flags for protocol offload
// features implemented in hardware
// @mcdi_max_ver: Maximum MCDI version supported
// @hwtstamp_filters: Mask of hardware timestamp filter types supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_nic_type {
    pub is_vf: bool,
    pub efx): *mut *mut unsigned int (mem_bar)(struct efx_nic,
    pub efx): *mut *mut unsigned int (mem_map_size)(struct efx_nic,
    pub efx): *mut *mut int (probe)(struct efx_nic,
    pub efx): *mut *mut void (remove)(struct efx_nic,
    pub efx): *mut *mut int (init)(struct efx_nic,
    pub efx): *mut *mut int (dimension_resources)(struct efx_nic,
    pub efx): *mut *mut void (fini)(struct efx_nic,
    pub efx): *mut *mut void (monitor)(struct efx_nic,
    pub reason): *mut *mut reset_type (map_reset_reason)(reset_type,
    pub flags): *mut *mut int (map_reset_flags)(u32,
    pub method): *mut *mut *mut int (reset)(struct efx_nic efx, enum reset_type,
    pub efx): *mut *mut int (probe_port)(struct efx_nic,
    pub efx): *mut *mut void (remove_port)(struct efx_nic,
    pub ): *mut *mut *mut bool (handle_global_event)(struct efx_channel channel, efx_qword_t,
    pub efx): *mut *mut int (fini_dmaq)(struct efx_nic,
    pub efx): *mut *mut void (prepare_flush)(struct efx_nic,
    pub efx): *mut *mut void (finish_flush)(struct efx_nic,
    pub efx): *mut *mut void (prepare_flr)(struct efx_nic,
    pub efx): *mut *mut void (finish_flr)(struct efx_nic,
    pub names): *mut *mut *mut size_t (describe_stats)(struct efx_nic efx, u8,
    pub core_stats): *mut rtnl_link_stats64,
    pub core_stats): *mut rtnl_link_stats64,
    pub efx): *mut *mut void (start_stats)(struct efx_nic,
    pub efx): *mut *mut void (pull_stats)(struct efx_nic,
    pub efx): *mut *mut void (stop_stats)(struct efx_nic,
    pub channel): *mut *mut void (push_irq_moderation)(struct efx_channel,
    pub efx): *mut *mut int (reconfigure_port)(struct efx_nic,
    pub efx): *mut *mut void (prepare_enable_fc_tx)(struct efx_nic,
    pub mtu_only): *mut *mut *mut int (reconfigure_mac)(struct efx_nic efx, bool,
    pub efx): *mut *mut bool (check_mac_fault)(struct efx_nic,
    pub wol): *mut *mut *mut void (get_wol)(struct efx_nic efx, struct ethtool_wolinfo,
    pub type): *mut *mut *mut int (set_wol)(struct efx_nic efx, u32,
    pub efx): *mut *mut void (resume_wol)(struct efx_nic,
    pub fec_stats): *mut ethtool_fec_stats,
    pub offset): u32,
    pub tests): *mut *mut *mut int (test_chip)(struct efx_nic efx, struct efx_self_tests,
    pub efx): *mut *mut int (test_nvram)(struct efx_nic,
    pub sdu_len): *const *const efx_dword_t sdu, size_t,
    pub efx): *mut *mut bool (mcdi_poll_response)(struct efx_nic,
    pub pdu_len): size_t pdu_offset, size_t,
    pub efx): *mut *mut int (mcdi_poll_reboot)(struct efx_nic,
    pub efx): *mut *mut void (mcdi_reboot_detected)(struct efx_nic,
    pub efx): *mut *mut void (irq_enable_master)(struct efx_nic,
    pub efx): *mut *mut int (irq_test_generate)(struct efx_nic,
    pub efx): *mut *mut void (irq_disable_non_ev)(struct efx_nic,
    pub dev_id): *mut *mut irqreturn_t (irq_handle_msi)(int irq, void,
    pub dev_id): *mut *mut irqreturn_t (irq_handle_legacy)(int irq, void,
    pub tx_queue): *mut *mut int (tx_probe)(struct efx_tx_queue,
    pub tx_queue): *mut *mut void (tx_init)(struct efx_tx_queue,
    pub tx_queue): *mut *mut void (tx_remove)(struct efx_tx_queue,
    pub tx_queue): *mut *mut void (tx_write)(struct efx_tx_queue,
    pub skb): *mut *mut *mut netdev_tx_t (tx_enqueue)(struct efx_tx_queue tx_queue, struct sk_buff,
    pub len): dma_addr_t dma_addr, unsigned int,
    pub key): *const *const u32 rx_indir_table, u8,
    pub efx): *mut *mut int (rx_pull_rss_config)(struct efx_nic,
    pub rx_queue): *mut *mut int (rx_probe)(struct efx_rx_queue,
    pub rx_queue): *mut *mut void (rx_init)(struct efx_rx_queue,
    pub rx_queue): *mut *mut void (rx_remove)(struct efx_rx_queue,
    pub rx_queue): *mut *mut void (rx_write)(struct efx_rx_queue,
    pub rx_queue): *mut *mut void (rx_defer_refill)(struct efx_rx_queue,
    pub channel): *mut *mut void (rx_packet)(struct efx_channel,
    pub prefix): *const *const bool (rx_buf_hash_valid)(u8,
    pub channel): *mut *mut int (ev_probe)(struct efx_channel,
    pub channel): *mut *mut int (ev_init)(struct efx_channel,
    pub channel): *mut *mut void (ev_fini)(struct efx_channel,
    pub channel): *mut *mut void (ev_remove)(struct efx_channel,
    pub quota): *mut *mut *mut int (ev_process)(struct efx_channel channel, int,
    pub channel): *mut *mut void (ev_read_ack)(struct efx_channel,
    pub channel): *mut *mut void (ev_test_generate)(struct efx_channel,
    pub efx): *mut *mut int (filter_table_probe)(struct efx_nic,
    pub efx): *mut *mut void (filter_table_restore)(struct efx_nic,
    pub efx): *mut *mut void (filter_table_remove)(struct efx_nic,
    pub efx): *mut *mut void (filter_update_rx_scatter)(struct efx_nic,
    pub replace): *mut *mut efx_filter_spec spec, bool,
    pub filter_id): u32,
    pub ): *mut u32 filter_id, struct efx_filter_spec,
    pub priority): efx_filter_priority,
    pub priority): efx_filter_priority,
    pub efx): *mut *mut u32 (filter_get_rx_id_limit)(struct efx_nic,
    pub size): *mut *mut u32 buf, u32,

    pub index): c_uint,

    pub efx): *mut *mut int (mtd_probe)(struct efx_nic,
    pub part): *mut *mut void (mtd_rename)(struct efx_mtd_partition,
    pub buffer): *mut *mut size_t retlen, u8,
    pub len): *mut *mut *mut int (mtd_erase)(struct mtd_info mtd, loff_t start, size_t,
    pub buffer): *const *const size_t retlen, u8,
    pub mtd): *mut *mut int (mtd_sync)(struct mtd_info,

    pub host_time): *mut *mut *mut void (ptp_write_host_time)(struct efx_nic efx, u32,
    pub temp): *mut *mut *mut int (ptp_set_ts_sync_events)(struct efx_nic efx, bool en, bool,
    pub init): *mut kernel_hwtstamp_config,
    pub num_vfs): *mut *mut *mut int (sriov_configure)(struct efx_nic efx, int,
    pub vid): *mut *mut *mut int (vlan_rx_add_vid)(struct efx_nic efx, __be16 proto, u16,
    pub vid): *mut *mut *mut int (vlan_rx_kill_vid)(struct efx_nic efx, __be16 proto, u16,
    pub ppid): *mut netdev_phys_item_id,
    pub efx): *mut *mut int (sriov_init)(struct efx_nic,
    pub efx): *mut *mut void (sriov_fini)(struct efx_nic,
    pub efx): *mut *mut bool (sriov_wanted)(struct efx_nic,
    pub efx): *mut *mut void (sriov_reset)(struct efx_nic,
    pub vf_i): *mut *mut *mut void (sriov_flr)(struct efx_nic efx, unsigned,
    pub mac): *const *const *const int (sriov_set_vf_mac)(struct efx_nic efx, int vf_i, u8,
    pub qos): u8,
    pub spoofchk): bool,
    pub ivi): *mut ifla_vf_info,
    pub link_state): c_int,
    pub efx): *mut *mut int (vswitching_probe)(struct efx_nic,
    pub efx): *mut *mut int (vswitching_restore)(struct efx_nic,
    pub efx): *mut *mut void (vswitching_remove)(struct efx_nic,
    pub perm_addr): *mut *mut *mut int (get_mac_address)(struct efx_nic efx, unsigned char,
    pub efx): *mut *mut int (set_mac_address)(struct efx_nic,
    pub efx): *mut *mut u32 (tso_versions)(struct efx_nic,
    pub efx): *mut *mut int (udp_tnl_push_ports)(struct efx_nic,
    pub port): *mut *mut *mut bool (udp_tnl_has_port)(struct efx_nic efx, __be16,
    pub len): usize,
    pub ev): *mut *mut *mut void (sensor_event)(struct efx_nic efx, efx_qword_t,
    pub efx): *const *const unsigned int (rx_recycle_ring_size)(struct efx_nic,
    pub revision: c_int,
    pub txd_ptr_tbl_base: c_uint,
    pub rxd_ptr_tbl_base: c_uint,
    pub buf_tbl_base: c_uint,
    pub evq_ptr_tbl_base: c_uint,
    pub evq_rptr_tbl_base: c_uint,
    pub max_dma_mask: u64,
    pub rx_prefix_size: c_uint,
    pub rx_hash_offset: c_uint,
    pub rx_ts_offset: c_uint,
    pub rx_buffer_padding: c_uint,
    pub can_rx_scatter: bool,
    pub always_rx_scatter: bool,
    pub option_descriptors: bool,
    pub min_interrupt_mode: c_uint,
    pub timer_period_max: c_uint,
    pub offload_features: netdev_features_t,
    pub mcdi_max_ver: c_int,
    pub max_rx_ip_filters: c_uint,
    pub hwtstamp_filters: u32,
    pub rx_hash_key_size: c_uint,
}

//
// Prototypes and inline functions
//
// Iterate over all used channels

// Iterate over all used channels in reverse

extern "C" {
    pub fn efx_channel_get_tx_queue(_arg: channel, _arg: type) -> return;
}
// Iterate over all TX queues belonging to a channel

// Iterate over all RX queues belonging to a channel

extern "C" {
    pub fn container_of(_arg: rx_queue, efx_channel: struct, _arg: rx_queue) -> return;
}
// Returns a pointer to the specified receive buffer in the RX
// descriptor queue.
//
extern "C" {
    pub fn efx_rx_buffer(_arg: rx_queue, _arg: 0) -> return;
}
//
// EFX_MAX_FRAME_LEN - calculate maximum frame length
//
// This calculates the maximum frame length that will be used for a
// given MTU.  The frame length will be equal to the MTU plus a
// constant amount of header space and padding.  This is the quantity
// that the net driver will program into the MAC as the maximum frame
// length.
//
// The 10G MAC requires 8-byte alignment on the frame
// length, so we round up to the nearest 8.
//
// Re-clocking by the XGXS on RX can reduce an IPG to 32 bits (half an
// XGMII cycle).  If the frame length reaches the maximum value in the
// same cycle, the XMAC can miss the IPG altogether.  We work around
// this by adding a further 16 bytes.
//
pub const EFX_FRAME_PAD: c_int = 16;

// Get the max fill level of the TX queues on this channel
// Conservative approximation of efx_channel_tx_fill_level using cached value
// Get all supported features.
// If a feature is not fixed, it is present in hw_features.
// If a feature is fixed, it does not present in hw_features, but
// always in features.
//
// Get the current TX queue insert index.
// Get a TX buffer.
// Get a TX buffer, checking it's not currently in use.
