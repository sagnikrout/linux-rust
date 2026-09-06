//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/falcon/net_driver.h
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

pub const EF4_EXTRA_CHANNEL_IOV: c_int = 0;
pub const EF4_EXTRA_CHANNEL_PTP: c_int = 1;

// Checksum generation is a per-queue option in hardware, so each
// queue visible to the networking core is backed by two hardware TX
// queues.
pub const EF4_MAX_TX_TC: c_int = 2;

pub const EF4_TXQ_TYPES: c_int = 4;

// Maximum possible MTU the driver supports

// Minimum MTU, from RFC791 (IP)
pub const EF4_MIN_MTU: c_int = 68;
// Size of an RX scatter buffer.  Small enough to pack 2 into a 4K page,
// and should be a multiple of the cache line size.
//

// If possible, we should ensure cache line alignment at start and end
// of every buffer.  Otherwise, we just need to ensure 4-byte
// alignment of the network header.
//

pub const EF4_RX_BUF_ALIGNMENT: c_int = 4;

//
// struct ef4_buffer - A general-purpose DMA buffer
// @addr: host base address of the buffer
// @dma_addr: DMA base address of the buffer
// @len: Buffer length, in bytes
//
// The NIC uses these buffers for its interrupt status registers and
// MAC stats dumps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_buffer {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub len: c_uint,
}

//
// struct ef4_special_buffer - DMA buffer entered into buffer table
// @buf: Standard &struct ef4_buffer
// @index: Buffer index within controller;s buffer table
// @entries: Number of buffer table entries
//
// The NIC has a buffer table that maps buffers of size %EF4_BUF_SIZE.
// Event and descriptor rings are addressed via one or more buffer
// table entries (and so can be physically non-contiguous, although we
// currently do not take advantage of that).  On Falcon and Siena we
// have to take care of allocating and initialising the entries
// ourselves.  On later hardware this is managed by the firmware and
// @index and @entries are left as 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_special_buffer {
    pub buf: ef4_buffer,
    pub index: c_uint,
    pub entries: c_uint,
}

//
// struct ef4_tx_buffer - buffer state for a TX descriptor
// @skb: When @flags & %EF4_TX_BUF_SKB, the associated socket buffer to be
// freed when descriptor completes
// @option: When @flags & %EF4_TX_BUF_OPTION, a NIC-specific option descriptor.
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
pub struct ef4_tx_buffer {
    pub skb: *const sk_buff,
    pub option: ef4_qword_t,
    pub dma_addr: dma_addr_t,
}

pub const EF4_TX_BUF_OPTION: c_uint = 0x10	/* empty buffer for option descriptor */;
//
// struct ef4_tx_queue - An Efx TX queue
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
// @channel: The associated channel
// @core_txq: The networking core TX queue structure
// @buffer: The software buffer ring
// @cb_page: Array of pages of copy buffers.  Carved up according to
// %EF4_TX_CB_ORDER into %EF4_TX_CB_SIZE-sized chunks.
// @txd: The hardware descriptor ring
// @ptr_mask: The size of the ring minus 1.
// @initialised: Has hardware queue been initialised?
// @tx_min_size: Minimum transmit size for this queue. Depends on HW.
// @read_count: Current read pointer.
// This is the number of buffers that have been removed from both rings.
// @old_write_count: The value of @write_count when last checked.
// This is here for performance reasons.  The xmit path will
// only get the up-to-date value of @write_count if this
// variable indicates that the queue is empty.  This is to
// avoid cache-line ping-pong between the xmit path and the
// completion path.
// @merge_events: Number of TX merged completion events
// @insert_count: Current insert pointer
// This is the number of buffers that have been added to the
// software ring.
// @write_count: Current write pointer
// This is the number of buffers that have been added to the
// hardware ring.
// @old_read_count: The value of read_count when last checked.
// This is here for performance reasons.  The xmit path will
// only get the up-to-date value of read_count if this
// variable indicates that the queue is full.  This is to
// avoid cache-line ping-pong between the xmit path and the
// completion path.
// @pushes: Number of times the TX push feature has been used
// @xmit_more_available: Are any packets waiting to be pushed to the NIC
// @cb_packets: Number of times the TX copybreak feature has been used
// @empty_read_count: If the completion path has seen the queue as empty
// and the transmission path has not yet checked this, the value of
// @read_count bitwise-added to %EF4_EMPTY_COUNT_VALID; otherwise 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_tx_queue {
// Members which don't change on the fast path
    pub ____cacheline_aligned_in_smp: *mut *mut ef4_nic efx,
    pub queue: unsigned,
    pub channel: *mut ef4_channel,
    pub core_txq: *mut netdev_queue,
    pub buffer: *mut ef4_tx_buffer,
    pub cb_page: *mut ef4_buffer,
    pub txd: ef4_special_buffer,
    pub ptr_mask: c_uint,
    pub initialised: bool,
    pub tx_min_size: c_uint,
// Function pointers used in the fast path.
    pub ): *mut *mut *mut *mut int (handle_tso)(struct ef4_tx_queue, struct sk_buff, bool,
// Members used mainly on the completion path
    pub ____cacheline_aligned_in_smp: unsigned int read_count,
    pub old_write_count: c_uint,
    pub merge_events: c_uint,
    pub bytes_compl: c_uint,
    pub pkts_compl: c_uint,
// Members used only on the xmit path
    pub ____cacheline_aligned_in_smp: unsigned int insert_count,
    pub write_count: c_uint,
    pub old_read_count: c_uint,
    pub pushes: c_uint,
    pub xmit_more_available: bool,
    pub cb_packets: c_uint,
// Statistics to supplement MAC stats
    pub tx_packets: c_ulong,
// Members shared between paths and sometimes updated
    pub ____cacheline_aligned_in_smp: unsigned int empty_read_count,
pub const EF4_EMPTY_COUNT_VALID: c_uint = 0x80000000;
    pub flush_outstanding: core::sync::atomic::AtomicI32,
}

pub const EF4_TX_CB_ORDER: c_int = 7;

//
// struct ef4_rx_buffer - An Efx RX data buffer
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
pub struct ef4_rx_buffer {
    pub dma_addr: dma_addr_t,
    pub page: *mut page,
    pub page_offset: u16,
    pub len: u16,
    pub flags: u16,
}

pub const EF4_RX_BUF_LAST_IN_PAGE: c_uint = 0x0001;
pub const EF4_RX_PKT_CSUMMED: c_uint = 0x0002;
pub const EF4_RX_PKT_DISCARD: c_uint = 0x0004;
pub const EF4_RX_PKT_TCP: c_uint = 0x0040;
pub const EF4_RX_PKT_PREFIX_LEN: c_uint = 0x0080	/* length is in prefix only */;
//
// struct ef4_rx_page_state - Page-based rx buffer state
//
// Inserted at the start of every page allocated for receive buffers.
// Used to facilitate sharing dma mappings between recycled rx buffers
// and those passed up to the kernel.
//
// @dma_addr: The dma address of this page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_rx_page_state {
    pub dma_addr: dma_addr_t,
    pub ____cacheline_aligned: unsigned int __pad[],
}

//
// struct ef4_rx_queue - An Efx RX queue
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
// @slow_fill: Timer used to defer ef4_nic_generate_fill_event().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_rx_queue {
    pub efx: *mut ef4_nic,
    pub core_index: c_int,
    pub buffer: *mut ef4_rx_buffer,
    pub rxd: ef4_special_buffer,
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
}

//
// struct ef4_channel - An Efx channel
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
// @rx_pkt_n_frags: Number of fragments in next packet to be delivered by
// __ef4_rx_packet(), or zero if there is none
// @rx_pkt_index: Ring index of first buffer for next packet to be delivered
// by __ef4_rx_packet(), if @rx_pkt_n_frags != 0
// @rx_queue: RX queue for this channel
// @tx_queue: TX queues for this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_channel {
    pub efx: *mut ef4_nic,
    pub channel: c_int,
    pub type: *const ef4_channel_type,
    pub eventq_init: bool,
    pub enabled: bool,
    pub irq: c_int,
    pub irq_moderation_us: c_uint,
    pub napi_dev: *mut net_device,
    pub napi_str: napi_struct,

    pub busy_poll_state: c_ulong,

    pub eventq: ef4_special_buffer,
    pub eventq_mask: c_uint,
    pub eventq_read_ptr: c_uint,
    pub event_test_cpu: c_int,
    pub irq_count: c_uint,
    pub irq_mod_score: c_uint,

    pub rfs_filters_added: c_uint,
pub const RPS_FLOW_ID_INVALID: c_uint = 0xFFFFFFFF;
    pub rps_flow_id: *mut u32,

    pub n_rx_tobe_disc: unsigned,
    pub n_rx_ip_hdr_chksum_err: unsigned,
    pub n_rx_tcp_udp_chksum_err: unsigned,
    pub n_rx_mcast_mismatch: unsigned,
    pub n_rx_frm_trunc: unsigned,
    pub n_rx_overlength: unsigned,
    pub n_skbuff_leaks: unsigned,
    pub n_rx_nodesc_trunc: c_uint,
    pub n_rx_merge_events: c_uint,
    pub n_rx_merge_packets: c_uint,
    pub rx_pkt_n_frags: c_uint,
    pub rx_pkt_index: c_uint,
    pub rx_queue: ef4_rx_queue,
    pub tx_queue: [ef4_tx_queue; EF4_TXQ_TYPES],
}

//
// struct ef4_msi_context - Context for each MSI
// @efx: The associated NIC
// @index: Index of the channel/IRQ
// @name: Name of the channel/IRQ
//
// Unlike &struct ef4_channel, this is never reallocated and is always
// safe for the IRQ handler to access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_msi_context {
    pub efx: *mut ef4_nic,
    pub index: c_uint,
    pub 6]: char name[IFNAMSIZ +,
}

//
// struct ef4_channel_type - distinguishes traffic and extra channels
// @handle_no_channel: Handle failure to allocate an extra channel
// @pre_probe: Set up extra state prior to initialisation
// @post_remove: Tear down extra state after finalisation, if allocated.
// May be called on channels that have not been probed.
// @get_name: Generate the channel's name (used for its IRQ handler)
// @copy: Copy the channel state prior to reallocation.  May be %NULL if
// reallocation is not supported.
// @receive_skb: Handle an skb ready to be passed to netif_receive_skb()
// @keep_eventq: Flag for whether event queue should be kept initialised
// while the device is stopped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_channel_type {
    pub ): *mut *mut void (handle_no_channel)(struct ef4_nic,
    pub ): *mut *mut int (pre_probe)(struct ef4_channel,
    pub ): *mut *mut void (post_remove)(struct ef4_channel,
    pub len): *mut *mut *mut *mut void (get_name)(struct ef4_channel , char buf, size_t,
    pub ): *const *const *const ef4_channel (copy)(ef4_channel,
    pub ): *mut *mut *mut bool (receive_skb)(struct ef4_channel , struct sk_buff,
    pub keep_eventq: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ef4_led_mode {
    EF4_LED_OFF	= 0,
    EF4_LED_ON	= 1,
    EF4_LED_DEFAULT	= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ef4_int_mode {
// Be careful if altering to correct macro below
    EF4_INT_MODE_MSIX = 0,
    EF4_INT_MODE_MSI = 1,
    EF4_INT_MODE_LEGACY = 2,
    EF4_INT_MODE_MAX	/* Insert any new items before this */
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

pub const EF4_FC_AUTO: c_int = 4;
//
// struct ef4_link_state - Current state of the link
// @up: Link is up
// @fd: Link is full-duplex
// @fc: Actual flow control flags
// @speed: Link speed (Mbps)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_link_state {
    pub up: bool,
    pub fd: bool,
    pub fc: u8,
    pub speed: c_uint,
}

//
// struct ef4_phy_operations - Efx PHY operations table
// @probe: Probe PHY and initialise efx->mdio.mode_support, efx->mdio.mmds,
// efx->loopback_modes.
// @init: Initialise PHY
// @fini: Shut down PHY
// @reconfigure: Reconfigure PHY (e.g. for new link parameters)
// @poll: Update @link_state and report whether it changed.
// Serialised by the mac_lock.
// @get_link_ksettings: Get ethtool settings. Serialised by the mac_lock.
// @set_link_ksettings: Set ethtool settings. Serialised by the mac_lock.
// @set_npage_adv: Set abilities advertised in (Extended) Next Page
// (only needed where AN bit is set in mmds)
// @test_alive: Test that PHY is 'alive' (online)
// @test_name: Get the name of a PHY-specific test/result
// @run_tests: Run tests and record results as appropriate (offline).
// Flags are the ethtool tests flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_phy_operations {
    pub efx): *mut *mut int (probe) (struct ef4_nic,
    pub efx): *mut *mut int (init) (struct ef4_nic,
    pub efx): *mut *mut void (fini) (struct ef4_nic,
    pub efx): *mut *mut void (remove) (struct ef4_nic,
    pub efx): *mut *mut int (reconfigure) (struct ef4_nic,
    pub efx): *mut *mut bool (poll) (struct ef4_nic,
    pub cmd): *mut ethtool_link_ksettings,
    pub cmd): *const ethtool_link_ksettings,
    pub u32): *mut *mut *mut void (set_npage_adv) (struct ef4_nic efx,,
    pub efx): *mut *mut int (test_alive) (struct ef4_nic,
    pub index): *const *const *const *const char (test_name) (struct ef4_nic efx, unsigned int,
    pub flags): *mut *mut *mut *mut int (run_tests) (struct ef4_nic efx, int results, unsigned,
    pub data): *mut u8,
    pub modinfo): *mut ethtool_modinfo,
}

//
// enum ef4_phy_mode - PHY operating mode flags
// @PHY_MODE_NORMAL: on and should pass traffic
// @PHY_MODE_TX_DISABLED: on with TX disabled
// @PHY_MODE_LOW_POWER: set to low power through MDIO
// @PHY_MODE_OFF: switched off through external control
// @PHY_MODE_SPECIAL: on but will not pass traffic
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ef4_phy_mode {
    PHY_MODE_NORMAL		= 0,
    PHY_MODE_TX_DISABLED	= 1,
    PHY_MODE_LOW_POWER	= 2,
    PHY_MODE_OFF		= 4,
    PHY_MODE_SPECIAL	= 8,
}

//
// struct ef4_hw_stat_desc - Description of a hardware statistic
// @name: Name of the statistic as visible through ethtool, or %NULL if
// it should not be exposed
// @dma_width: Width in bits (0 for non-DMA statistics)
// @offset: Offset within stats (ignored for non-DMA statistics)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_hw_stat_desc {
    pub name: *const c_char,
    pub dma_width: u16,
    pub offset: u16,
}

// Number of bits used in a multicast filter hash address
pub const EF4_MCAST_HASH_BITS: c_int = 8;
// Number of (single-bit) entries in a multicast filter hash

// An Efx multicast filter hash
#[repr(C)]
#[derive(Copy, Clone)]
pub union ef4_multicast_hash {
    pub 8]: u8 byte[EF4_MCAST_HASH_ENTRIES /,
    pub 8]: ef4_oword_t oword[EF4_MCAST_HASH_ENTRIES / sizeof(ef4_oword_t) /,
}

//
// struct ef4_nic - an Efx NIC
// @name: Device name (net device name or bus id before net device registered)
// @pci_dev: The PCI device
// @node: List node for maintaining primary/secondary function lists
// @primary: &struct ef4_nic instance for the primary function of this
// controller.  May be the same structure, and may be %NULL if no
// primary function is bound.  Serialised by rtnl_lock.
// @secondary_list: List of &struct ef4_nic instances for the secondary PCI
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
// @interrupt_mode: Interrupt mode
// @timer_quantum_ns: Interrupt timer quantum, in nanoseconds
// @timer_max_ns: Interrupt timer maximum value, in nanoseconds
// @irq_rx_adaptive: Adaptive IRQ moderation enabled for RX event queues
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
// (valid only for NICs that set %EF4_RX_PKT_PREFIX_LEN; always negative)
// @rx_packet_ts_offset: Offset of timestamp from start of packet data
// (valid only if channel->sync_timestamps_enabled; always negative)
// @rx_hash_key: Toeplitz hash key for RSS
// @rx_indir_table: Indirection table for RSS
// @rx_scatter: Scatter mode enabled for receives
// @int_error_count: Number of internal errors seen recently
// @int_error_expire: Time at which error count will be expired
// @irq_soft_enabled: Are IRQs soft-enabled? If not, IRQ handler will
// acknowledge but do nothing else.
// @irq_status: Interrupt status buffer
// @irq_zero_count: Number of legacy IRQs seen with queue flags == 0
// @irq_level: IRQ level/index for IRQs not triggered by an event queue
// @selftest_work: Work item for asynchronous self-test
// @mtd_list: List of MTDs attached to the NIC
// @nic_data: Hardware dependent state
// @mac_lock: MAC access lock. Protects @port_enabled, @phy_mode,
// ef4_monitor() and ef4_reconfigure_port()
// @port_enabled: Port enabled indicator.
// Serialises ef4_stop_all(), ef4_start_all(), ef4_monitor() and
// ef4_mac_work() with kernel interfaces. Safe to read under any
// one of the rtnl_lock, mac_lock, or netif_tx_lock, but all three must
// be held to modify it.
// @port_initialized: Port initialized?
// @net_dev: Operating system network device. Consider holding the rtnl lock
// @fixed_features: Features which cannot be turned off
// @stats_buffer: DMA buffer for statistics
// @phy_type: PHY type
// @phy_op: PHY interface
// @phy_data: PHY private data (including PHY-specific stats)
// @mdio: PHY MDIO interface
// @phy_mode: PHY operating mode. Serialised by @mac_lock.
// @link_advertising: Autonegotiation advertising flags
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
// @filter_sem: Filter table rw_semaphore, for freeing the table
// @filter_lock: Filter table lock, for mere content changes
// @filter_state: Architecture-dependent filter table state
// @rps_expire_channel: Next channel to check for expiry
// @rps_expire_index: Next index to check for expiry in
// @rps_expire_channel's @rps_flow_id
// @active_queues: Count of RX and TX queues that haven't been flushed and drained.
// @rxq_flush_pending: Count of number of receive queues that need to be flushed.
// Decremented when the ef4_flush_rx_queue() is called.
// @rxq_flush_outstanding: Count of number of RX flushes started but not yet
// completed (either success or failure). Not used when MCDI is used to
// flush receive queues.
// @flush_wq: wait queue used by ef4_nic_flush_queues() to wait for flush completions.
// @vpd_sn: Serial number read from VPD
// @monitor_work: Hardware monitor workitem
// @biu_lock: BIU (bus interface unit) lock
// @last_irq_cpu: Last CPU to handle a possible test interrupt.  This
// field is used by ef4_test_interrupts() to verify that an
// interrupt has occurred.
// @stats_lock: Statistics update lock. Must be held when calling
// ef4_nic_type::{update,start,stop}_stats.
// @n_rx_noskb_drops: Count of RX packets dropped due to failure to allocate an skb
//
// This is stored in the private area of the &struct net_device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_nic {
// The following fields should be written very rarely
    pub name: [c_char; IFNAMSIZ],
    pub node: list_head,
    pub primary: *mut ef4_nic,
    pub secondary_list: list_head,
    pub pci_dev: *mut pci_dev,
    pub port_num: c_uint,
    pub type: *const ef4_nic_type,
    pub legacy_irq: c_int,
    pub eeh_disabled_legacy_irq: bool,
    pub workqueue: *mut workqueue_struct,
    pub workqueue_name: [c_char; 16],
    pub reset_work: work_struct,
    pub membase_phys: resource_size_t,
    pub membase: *mut void __iomem,
    pub interrupt_mode: ef4_int_mode,
    pub timer_quantum_ns: c_uint,
    pub timer_max_ns: c_uint,
    pub irq_rx_adaptive: bool,
    pub irq_mod_step_us: c_uint,
    pub irq_rx_moderation_us: c_uint,
    pub msg_enable: u32,
    pub state: nic_state,
    pub reset_pending: c_ulong,
    pub channel: [*mut ef4_channel; EF4_MAX_CHANNELS],
    pub msi_context: [ef4_msi_context; EF4_MAX_CHANNELS],
    pub rxq_entries: unsigned,
    pub txq_entries: unsigned,
    pub txq_stop_thresh: c_uint,
    pub txq_wake_thresh: c_uint,
    pub tx_dc_base: unsigned,
    pub rx_dc_base: unsigned,
    pub sram_lim_qw: unsigned,
    pub next_buffer_table: unsigned,
    pub max_channels: c_uint,
    pub max_tx_channels: c_uint,
    pub n_channels: unsigned,
    pub n_rx_channels: unsigned,
    pub rss_spread: unsigned,
    pub tx_channel_offset: unsigned,
    pub n_tx_channels: unsigned,
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
    pub rx_hash_key: [u8; 40],
    pub rx_indir_table: [u32; 128],
    pub rx_scatter: bool,
    pub int_error_count: unsigned,
    pub int_error_expire: c_ulong,
    pub irq_soft_enabled: bool,
    pub irq_status: ef4_buffer,
    pub irq_zero_count: unsigned,
    pub irq_level: unsigned,
    pub selftest_work: delayed_work,

    pub mtd_list: list_head,

    pub nic_data: *mut c_void,
    pub mac_lock: mutex,
    pub mac_work: work_struct,
    pub port_enabled: bool,
    pub mc_bist_for_other_fn: bool,
    pub port_initialized: bool,
    pub net_dev: *mut net_device,
    pub fixed_features: netdev_features_t,
    pub stats_buffer: ef4_buffer,
    pub rx_nodesc_drops_total: u64,
    pub rx_nodesc_drops_while_down: u64,
    pub rx_nodesc_drops_prev_state: bool,
    pub phy_type: c_uint,
    pub phy_op: *const ef4_phy_operations,
    pub phy_data: *mut c_void,
    pub mdio: mdio_if_info,
    pub phy_mode: ef4_phy_mode,
    pub link_advertising: u32,
    pub link_state: ef4_link_state,
    pub n_link_state_changes: c_uint,
    pub unicast_filter: bool,
    pub multicast_hash: ef4_multicast_hash,
    pub wanted_fc: u8,
    pub fc_disable: unsigned,
    pub rx_reset: core::sync::atomic::AtomicI32,
    pub loopback_mode: ef4_loopback_mode,
    pub loopback_modes: u64,
    pub loopback_selftest: *mut c_void,
    pub filter_sem: rw_semaphore,
    pub filter_lock: spinlock_t,
    pub filter_state: *mut c_void,

    pub rps_expire_channel: c_uint,
    pub rps_expire_index: c_uint,

    pub active_queues: core::sync::atomic::AtomicI32,
    pub rxq_flush_pending: core::sync::atomic::AtomicI32,
    pub rxq_flush_outstanding: core::sync::atomic::AtomicI32,
    pub flush_wq: wait_queue_head_t,
    pub vpd_sn: *mut c_char,
// The following fields may be written more often
    pub ____cacheline_aligned_in_smp: delayed_work monitor_work,
    pub biu_lock: spinlock_t,
    pub last_irq_cpu: c_int,
    pub stats_lock: spinlock_t,
    pub n_rx_noskb_drops: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_mtd_partition {
    pub node: list_head,
    pub mtd: mtd_info,
    pub dev_type_name: *const c_char,
    pub type_name: *const c_char,
    pub 20]: char name[IFNAMSIZ +,
}

//
// struct ef4_nic_type - Efx device type definition
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
// @start_stats: Start the regular fetching of statistics
// @pull_stats: Pull stats from the NIC and wait until they arrive.
// @stop_stats: Stop the regular fetching of statistics
// @set_id_led: Set state of identifying LED or revert to automatic function
// @push_irq_moderation: Apply interrupt moderation value
// @reconfigure_port: Push loopback/power/txdis changes to the MAC and PHY
// @prepare_enable_fc_tx: Prepare MAC to enable pause frame TX (may be %NULL)
// @reconfigure_mac: Push MAC address, MTU, flow control and filter settings
// to the hardware.  Serialised by the mac_lock.
// @check_mac_fault: Check MAC fault state. True if fault present.
// @get_wol: Get WoL configuration from driver state
// @set_wol: Push WoL configuration to the NIC
// @resume_wol: Synchronise WoL state between driver and MC (e.g. after resume)
// @test_chip: Test registers.  May use ef4_farch_test_registers(), and is
// expected to reset the NIC.
// @test_nvram: Test validity of NVRAM contents
// @irq_enable_master: Enable IRQs on the NIC.  Each event queue must
// be separately enabled after this.
// @irq_test_generate: Generate a test IRQ
// @irq_disable_non_ev: Disable non-event IRQs on the NIC.  Each event
// queue must be separately disabled before this.
// @irq_handle_msi: Handle MSI for a channel.  The @dev_id argument is
// a pointer to the &struct ef4_msi_context for the channel.
// @irq_handle_legacy: Handle legacy interrupt.  The @dev_id argument
// is a pointer to the &struct ef4_nic.
// @tx_probe: Allocate resources for TX queue
// @tx_init: Initialise TX queue on the NIC
// @tx_remove: Free resources for TX queue
// @tx_write: Write TX descriptors and doorbell
// @rx_push_rss_config: Write RSS hash key and indirection table to the NIC
// @rx_probe: Allocate resources for RX queue
// @rx_init: Initialise RX queue on the NIC
// @rx_remove: Free resources for RX queue
// @rx_write: Write RX descriptors and doorbell
// @rx_defer_refill: Generate a refill reminder event
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
// equal to the given priority and is not %EF4_FILTER_PRI_AUTO
// @filter_count_rx_used: Get the number of filters in use at a given priority
// @filter_get_rx_id_limit: Get maximum value of a filter id, plus 1
// @filter_get_rx_ids: Get list of RX filters at a given priority
// @filter_rfs_insert: Add or replace a filter for RFS.  This must be
// atomic.  The hardware change may be asynchronous but should
// not be delayed for long.  It may fail if this can't be done
// atomically.
// @filter_rfs_expire_one: Consider expiring a filter inserted for RFS.
// This must check whether the specified table entry is used by RFS
// and that rps_may_expire_flow() returns true for it.
// @mtd_probe: Probe and add MTD partitions associated with this net device,
// using ef4_mtd_add()
// @mtd_rename: Set an MTD partition name using the net device name
// @mtd_read: Read from an MTD partition
// @mtd_erase: Erase part of an MTD partition
// @mtd_write: Write to an MTD partition
// @mtd_sync: Wait for write-back to complete on MTD partition.  This
// also notifies the driver that a writer has finished using this
// partition.
// @set_mac_address: Set the MAC address of the device
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
// @max_interrupt_mode: Highest capability interrupt mode supported
// from &enum ef4_init_mode.
// @timer_period_max: Maximum period of interrupt timer (in ticks)
// @offload_features: net_device feature flags for protocol offload
// features implemented in hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_nic_type {
    pub mem_bar: c_uint,
    pub efx): *mut *mut unsigned int (mem_map_size)(struct ef4_nic,
    pub efx): *mut *mut int (probe)(struct ef4_nic,
    pub efx): *mut *mut void (remove)(struct ef4_nic,
    pub efx): *mut *mut int (init)(struct ef4_nic,
    pub efx): *mut *mut int (dimension_resources)(struct ef4_nic,
    pub efx): *mut *mut void (fini)(struct ef4_nic,
    pub efx): *mut *mut void (monitor)(struct ef4_nic,
    pub reason): *mut *mut reset_type (map_reset_reason)(reset_type,
    pub flags): *mut *mut int (map_reset_flags)(u32,
    pub method): *mut *mut *mut int (reset)(struct ef4_nic efx, enum reset_type,
    pub efx): *mut *mut int (probe_port)(struct ef4_nic,
    pub efx): *mut *mut void (remove_port)(struct ef4_nic,
    pub ): *mut *mut *mut bool (handle_global_event)(struct ef4_channel channel, ef4_qword_t,
    pub efx): *mut *mut int (fini_dmaq)(struct ef4_nic,
    pub efx): *mut *mut void (prepare_flush)(struct ef4_nic,
    pub efx): *mut *mut void (finish_flush)(struct ef4_nic,
    pub efx): *mut *mut void (prepare_flr)(struct ef4_nic,
    pub efx): *mut *mut void (finish_flr)(struct ef4_nic,
    pub names): *mut *mut *mut size_t (describe_stats)(struct ef4_nic efx, u8,
    pub core_stats): *mut rtnl_link_stats64,
    pub efx): *mut *mut void (start_stats)(struct ef4_nic,
    pub efx): *mut *mut void (pull_stats)(struct ef4_nic,
    pub efx): *mut *mut void (stop_stats)(struct ef4_nic,
    pub mode): *mut *mut *mut void (set_id_led)(struct ef4_nic efx, enum ef4_led_mode,
    pub channel): *mut *mut void (push_irq_moderation)(struct ef4_channel,
    pub efx): *mut *mut int (reconfigure_port)(struct ef4_nic,
    pub efx): *mut *mut void (prepare_enable_fc_tx)(struct ef4_nic,
    pub efx): *mut *mut int (reconfigure_mac)(struct ef4_nic,
    pub efx): *mut *mut bool (check_mac_fault)(struct ef4_nic,
    pub wol): *mut *mut *mut void (get_wol)(struct ef4_nic efx, struct ethtool_wolinfo,
    pub type): *mut *mut *mut int (set_wol)(struct ef4_nic efx, u32,
    pub efx): *mut *mut void (resume_wol)(struct ef4_nic,
    pub tests): *mut *mut *mut int (test_chip)(struct ef4_nic efx, struct ef4_self_tests,
    pub efx): *mut *mut int (test_nvram)(struct ef4_nic,
    pub efx): *mut *mut void (irq_enable_master)(struct ef4_nic,
    pub efx): *mut *mut int (irq_test_generate)(struct ef4_nic,
    pub efx): *mut *mut void (irq_disable_non_ev)(struct ef4_nic,
    pub dev_id): *mut *mut irqreturn_t (irq_handle_msi)(int irq, void,
    pub dev_id): *mut *mut irqreturn_t (irq_handle_legacy)(int irq, void,
    pub tx_queue): *mut *mut int (tx_probe)(struct ef4_tx_queue,
    pub tx_queue): *mut *mut void (tx_init)(struct ef4_tx_queue,
    pub tx_queue): *mut *mut void (tx_remove)(struct ef4_tx_queue,
    pub tx_queue): *mut *mut void (tx_write)(struct ef4_tx_queue,
    pub len): dma_addr_t dma_addr, unsigned int,
    pub rx_indir_table): *const u32,
    pub rx_queue): *mut *mut int (rx_probe)(struct ef4_rx_queue,
    pub rx_queue): *mut *mut void (rx_init)(struct ef4_rx_queue,
    pub rx_queue): *mut *mut void (rx_remove)(struct ef4_rx_queue,
    pub rx_queue): *mut *mut void (rx_write)(struct ef4_rx_queue,
    pub rx_queue): *mut *mut void (rx_defer_refill)(struct ef4_rx_queue,
    pub channel): *mut *mut int (ev_probe)(struct ef4_channel,
    pub channel): *mut *mut int (ev_init)(struct ef4_channel,
    pub channel): *mut *mut void (ev_fini)(struct ef4_channel,
    pub channel): *mut *mut void (ev_remove)(struct ef4_channel,
    pub quota): *mut *mut *mut int (ev_process)(struct ef4_channel channel, int,
    pub channel): *mut *mut void (ev_read_ack)(struct ef4_channel,
    pub channel): *mut *mut void (ev_test_generate)(struct ef4_channel,
    pub efx): *mut *mut int (filter_table_probe)(struct ef4_nic,
    pub efx): *mut *mut void (filter_table_restore)(struct ef4_nic,
    pub efx): *mut *mut void (filter_table_remove)(struct ef4_nic,
    pub efx): *mut *mut void (filter_update_rx_scatter)(struct ef4_nic,
    pub replace): *mut *mut ef4_filter_spec spec, bool,
    pub filter_id): u32,
    pub ): *mut u32 filter_id, struct ef4_filter_spec,
    pub priority): ef4_filter_priority,
    pub priority): ef4_filter_priority,
    pub efx): *mut *mut u32 (filter_get_rx_id_limit)(struct ef4_nic,
    pub size): *mut *mut u32 buf, u32,

    pub spec): *mut ef4_filter_spec,
    pub index): c_uint,

    pub efx): *mut *mut int (mtd_probe)(struct ef4_nic,
    pub part): *mut *mut void (mtd_rename)(struct ef4_mtd_partition,
    pub buffer): *mut *mut size_t retlen, u8,
    pub len): *mut *mut *mut int (mtd_erase)(struct mtd_info mtd, loff_t start, size_t,
    pub buffer): *const *const size_t retlen, u8,
    pub mtd): *mut *mut int (mtd_sync)(struct mtd_info,

    pub perm_addr): *mut *mut *mut int (get_mac_address)(struct ef4_nic efx, unsigned char,
    pub efx): *mut *mut int (set_mac_address)(struct ef4_nic,
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
    pub max_interrupt_mode: c_uint,
    pub timer_period_max: c_uint,
    pub offload_features: netdev_features_t,
    pub max_rx_ip_filters: c_uint,
}

//
// Prototypes and inline functions
//
// Iterate over all used channels

// Iterate over all used channels in reverse

// Iterate over all TX queues belonging to a channel

// Iterate over all possible TX queues belonging to a channel

// Iterate over all RX queues belonging to a channel

extern "C" {
    pub fn container_of(_arg: rx_queue, ef4_channel: struct, _arg: rx_queue) -> return;
}
// Returns a pointer to the specified receive buffer in the RX
// descriptor queue.
//
// EF4_MAX_FRAME_LEN - calculate maximum frame length
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
pub const EF4_FRAME_PAD: c_int = 16;

// Get all supported features.
// If a feature is not fixed, it is present in hw_features.
// If a feature is fixed, it does not present in hw_features, but
// always in features.
//
// Get the current TX queue insert index.
// Get a TX buffer.
// Get a TX buffer, checking it's not currently in use.
