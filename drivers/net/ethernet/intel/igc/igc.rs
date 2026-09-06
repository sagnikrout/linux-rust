//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc.h
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
// Copyright (c)  2018 Intel Corporation

extern "C" {
    pub fn igc_ethtool_set_ops(: *mut net_device);
}
// Transmit and receive queues
pub const IGC_MAX_RX_QUEUES: c_int = 4;
pub const IGC_MAX_TX_QUEUES: c_int = 4;
pub const MAX_Q_VECTORS: c_int = 8;
pub const MAX_STD_JUMBO_FRAME_SIZE: c_int = 9216;
pub const MAX_ETYPE_FILTER: c_int = 8;
pub const IGC_RETA_SIZE: c_int = 128;
pub const IGC_RSS_KEY_SIZE: c_int = 40;
// SDP support
pub const IGC_N_EXTTS: c_int = 2;
pub const IGC_N_PEROUT: c_int = 2;
pub const IGC_N_SDP: c_int = 4;
pub const MAX_FLEX_FILTER: c_int = 32;
pub const IGC_MAX_TX_TSTAMP_REGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_fpe_t {
    pub mmsv: ethtool_mmsv,
    pub tx_min_frag_size: u32,
    pub tx_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_mac_filter_type {
    IGC_MAC_FILTER_TYPE_DST = 0,
    IGC_MAC_FILTER_TYPE_SRC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_tx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub restart_queue: u64,
    pub restart_queue2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_rx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub drops: u64,
    pub csum_err: u64,
    pub alloc_failed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_rx_packet_stats {
    pub /: *mut *mut u64 ipv4_packets; / IPv4 headers processed,
    pub /: *mut *mut u64 ipv4e_packets; / IPv4E headers with extensions processed,
    pub /: *mut *mut u64 ipv6_packets; / IPv6 headers processed,
    pub /: *mut *mut u64 ipv6e_packets; / IPv6E headers with extensions processed,
    pub /: *mut *mut u64 tcp_packets; / TCP headers processed,
    pub /: *mut *mut u64 udp_packets; / UDP headers processed,
    pub /: *mut *mut u64 sctp_packets; / SCTP headers processed,
    pub /: *mut *mut u64 nfs_packets; / NFS headers processe,
    pub other_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_tx_buffer_type {
    IGC_TX_BUFFER_TYPE_SKB,
    IGC_TX_BUFFER_TYPE_XDP,
    IGC_TX_BUFFER_TYPE_XSK,
}

// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_tx_buffer {
    pub next_to_watch: *mut igc_adv_tx_desc,
    pub time_stamp: c_ulong,
    pub type: igc_tx_buffer_type,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_tx_timestamp_request {
    pub skb: *mut sk_buff,
    pub xsk_tx_buffer: *mut igc_tx_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_inline_rx_tstamps {
// Timestamps are saved in little endian at the beginning of the packet
// buffer following the layout:
//
// DWORD: | 0              | 1              | 2              | 3              |
// Field: | Timer1 SYSTIML | Timer1 SYSTIMH | Timer0 SYSTIML | Timer0 SYSTIMH |
//
// SYSTIML holds the nanoseconds part while SYSTIMH holds the seconds
// part of the timestamp.
//
    pub timer1: [__le32; 2],
    pub timer0: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_ring_container {
    pub /: *mut *mut *mut igc_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u16 work_limit; / total work allowed per interrupt,
    pub /: *mut *mut u8 count; / total number of rings in vector,
    pub /: *mut *mut u8 itr; / current ITR setting for ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_ring {
    pub /: *mut *mut *mut igc_q_vector q_vector; / backlink to q_vector,
    pub /: *mut *mut *mut net_device netdev; / back pointer to net_device,
    pub /: *mut *mut *mut device dev; / device for dma mapping,
    pub tx_buffer_info: *mut igc_tx_buffer,
    pub rx_buffer_info: *mut igc_rx_buffer,
}

// CBS parameters
// everything past this point are written often
// TX
// RX
// Board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_adapter {
    pub netdev: *mut net_device,
    pub eee: ethtool_keee,
    pub state: c_ulong,
    pub flags: c_uint,
    pub num_q_vectors: c_uint,
    pub msix_entries: *mut msix_entry,
// TX
    pub tx_work_limit: u16,
    pub tx_timeout_count: u32,
    pub num_tx_queues: c_int,
    pub tx_ring: [*mut igc_ring; IGC_MAX_TX_QUEUES],
// RX
    pub num_rx_queues: c_int,
    pub rx_ring: [*mut igc_ring; IGC_MAX_RX_QUEUES],
    pub watchdog_timer: timer_list,
    pub dma_err_timer: timer_list,
    pub phy_info_timer: timer_list,
    pub hrtimer: hrtimer,
    pub wol: u32,
    pub en_mng_pt: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub port_num: u8,
    pub io_addr: *mut u8 __iomem,
// Interrupt Throttle Rate
    pub rx_itr_setting: u32,
    pub tx_itr_setting: u32,
    pub reset_task: work_struct,
    pub watchdog_task: work_struct,
    pub dma_err_task: work_struct,
    pub fc_autoneg: bool,
    pub tx_timeout_factor: u8,
    pub msg_enable: c_int,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub tc_setup_type: c_int,
    pub base_time: ktime_t,
    pub cycle_time: ktime_t,
    pub taprio_offload_enable: bool,
    pub qbv_config_change_errors: u32,
    pub qbv_transition: bool,
    pub qbv_count: c_uint,
// Access to oper_gate_closed, admin_gate_closed and qbv_transition
// are protected by the qbv_tx_lock.
//
    pub qbv_tx_lock: spinlock_t,
    pub strict_priority_enable: bool,
    pub num_tc: u8,
    pub queue_per_tc: [u16; IGC_MAX_TX_QUEUES],
// OS defined structs
    pub pdev: *mut pci_dev,
// lock for statistics
    pub stats64_lock: spinlock_t,
    pub stats64: rtnl_link_stats64,
// structs defined in igc_hw.h
    pub hw: igc_hw,
    pub stats: igc_hw_stats,
    pub q_vector: [*mut igc_q_vector; MAX_Q_VECTORS],
    pub eims_enable_mask: u32,
    pub eims_other: u32,
    pub tx_ring_count: u16,
    pub rx_ring_count: u16,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
    pub rx_hwtstamp_cleared: u32,
    pub rss_queues: u32,
    pub rss_indir_tbl_init: u32,
// Any access to elements in nfc_rule_list is protected by the
// nfc_rule_lock.
//
    pub nfc_rule_lock: mutex,
    pub nfc_rule_list: list_head,
    pub nfc_rule_count: c_uint,
    pub rss_indir_tbl: [u8; IGC_RETA_SIZE],
    pub rss_key: [u8; IGC_RSS_KEY_SIZE],
    pub link_check_timeout: c_ulong,
    pub ei: igc_info,
    pub test_icr: u32,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
// Access to ptp_tx_skb and ptp_tx_start are protected by the
// ptp_tx_lock.
//
    pub ptp_tx_lock: spinlock_t,
    pub tx_tstamp: [igc_tx_timestamp_request; IGC_MAX_TX_TSTAMP_REGS],
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_flags: c_uint,
// System time value lock
    pub tmreg_lock: spinlock_t,
// Free-running timer lock
    pub free_timer_lock: spinlock_t,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub /: *mut *mut timespec64 prev_ptp_time; / Pre-reset PTP clock,
    pub /: *mut *mut ktime_t ptp_reset_start; / Reset time in clock mono,
    pub snapshot: system_time_snapshot,
    pub snapshot_clock_id: clockid_t,
    pub /: *mut *mut mutex ptm_lock; / Only allow one PTM transaction at a time,
    pub fw_version: [c_char; 32],
    pub xdp_prog: *mut bpf_prog,
    pub pps_sys_wrap_on: bool,
    pub sdp_config: [ptp_pin_desc; IGC_N_SDP],
    pub start: timespec64,
    pub period: timespec64,
    pub perout: [}; IGC_N_PEROUT],
    pub fpe: igc_fpe_t,
// LEDs
    pub led_mutex: mutex,
    pub leds: *mut igc_led_classdev,
    pub leds_available: bool,
}

extern "C" {
    pub fn igc_up(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_down(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn igc_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn igc_setup_tx_resources(ring: *mut igc_ring) -> c_int;
}
extern "C" {
    pub fn igc_setup_rx_resources(ring: *mut igc_ring) -> c_int;
}
extern "C" {
    pub fn igc_free_tx_resources(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_free_rx_resources(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_get_max_rss_queues(adapter: *mut igc_adapter) -> c_uint;
}
extern "C" {
    pub fn igc_reinit_queues(adapter: *mut igc_adapter) -> c_int;
}
extern "C" {
    pub fn igc_write_rss_key(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_write_rss_indir_tbl(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_has_link(adapter: *mut igc_adapter) -> bool;
}
extern "C" {
    pub fn igc_reset(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_update_stats(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_disable_rx_ring(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_enable_rx_ring(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_disable_tx_ring(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_enable_tx_ring(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_xsk_wakeup(dev: *mut net_device, queue_id: u32, flags: u32) -> c_int;
}
// AF_XDP TX metadata operations
// igc_dump declarations
extern "C" {
    pub fn igc_rings_dump(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_regs_dump(adapter: *mut igc_adapter);
}
pub const IGC_REGS_LEN: c_int = 740;
// flags controlling PTP/1588 function

// Flags definitions

// RX-desc Write-Back format RSS Type's
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_rss_type_num {
    IGC_RSS_TYPE_NO_HASH		= 0,
    IGC_RSS_TYPE_HASH_TCP_IPV4	= 1,
    IGC_RSS_TYPE_HASH_IPV4		= 2,
    IGC_RSS_TYPE_HASH_TCP_IPV6	= 3,
    IGC_RSS_TYPE_HASH_IPV6_EX	= 4,
    IGC_RSS_TYPE_HASH_IPV6		= 5,
    IGC_RSS_TYPE_HASH_TCP_IPV6_EX	= 6,
    IGC_RSS_TYPE_HASH_UDP_IPV4	= 7,
    IGC_RSS_TYPE_HASH_UDP_IPV6	= 8,
    IGC_RSS_TYPE_HASH_UDP_IPV6_EX	= 9,
    IGC_RSS_TYPE_MAX		= 10,
}

pub const IGC_RSS_TYPE_MAX_TABLE: c_int = 16;

// igc_rss_type - Rx descriptor RSS type field
// RSS Type 4-bits (3:0) number: 0-9 (above 9 is reserved)
// Accessing the same bits via u16 (wb.lower.lo_dword.hs_rss.pkt_info)
// is slightly slower than via u32 (wb.lower.lo_dword.data)
//
extern "C" {
    pub fn le32_get_bits(_arg: rx_desc->wb.lower.lo_dword.data, _arg: IGC_RSS_TYPE_MASK) -> return;
}
// Interrupt defines

pub const IGC_4K_ITR: c_int = 980;
pub const IGC_20K_ITR: c_int = 196;
pub const IGC_70K_ITR: c_int = 56;

pub const IGC_MAX_ITR_USECS: c_int = 10000;
pub const IGC_MIN_ITR_USECS: c_int = 10;
pub const NON_Q_VECTORS: c_int = 1;
pub const MAX_MSIX_ENTRIES: c_int = 10;
// TX/RX descriptor defines
pub const IGC_DEFAULT_TXD: c_int = 256;
pub const IGC_DEFAULT_TX_WORK: c_int = 128;
pub const IGC_MIN_TXD: c_int = 64;
pub const IGC_MAX_TXD: c_int = 4096;
pub const IGC_DEFAULT_RXD: c_int = 256;
pub const IGC_MIN_RXD: c_int = 64;
pub const IGC_MAX_RXD: c_int = 4096;
// Supported Rx Buffer Sizes
pub const IGC_RXBUFFER_256: c_int = 256;
pub const IGC_RXBUFFER_2048: c_int = 2048;
pub const IGC_RXBUFFER_3072: c_int = 3072;
pub const AUTO_ALL_MODES: c_int = 0;

// Transmit and receive latency (for PTP timestamps)
pub const IGC_I225_TX_LATENCY_10: c_int = 240;
pub const IGC_I225_TX_LATENCY_100: c_int = 58;
pub const IGC_I225_TX_LATENCY_1000: c_int = 80;
pub const IGC_I225_TX_LATENCY_2500: c_int = 1325;
pub const IGC_I225_RX_LATENCY_10: c_int = 6450;
pub const IGC_I225_RX_LATENCY_100: c_int = 185;
pub const IGC_I225_RX_LATENCY_1000: c_int = 300;
pub const IGC_I225_RX_LATENCY_2500: c_int = 1485;
// RX and TX descriptor control thresholds.
// PTHRESH - MAC will consider prefetch if it has fewer than this number of
// descriptors available in its onboard memory.
// Setting this to 0 disables RX descriptor prefetch.
// HTHRESH - MAC will only prefetch if there are at least this many descriptors
// available in host memory.
// If PTHRESH is 0, this should also be 0.
// WTHRESH - RX descriptor writeback threshold - MAC will delay writing back
// descriptors until either it has this many to write back, or the
// ITR timer expires.
//
pub const IGC_RXDCTL_PTHRESH: c_int = 8;
pub const IGC_RXDCTL_HTHRESH: c_int = 8;
pub const IGC_RXDCTL_WTHRESH: c_int = 4;
// Ena specific Rx Queue
pub const IGC_RXDCTL_QUEUE_ENABLE: c_uint = 0x02000000;
// Receive Software Flush
pub const IGC_RXDCTL_SWFLUSH: c_uint = 0x04000000;

// Ena specific Tx Queue

// Transmit Software Flush

pub const IGC_TS_HDR_LEN: c_int = 16;

// How many Rx Buffers do we bundle into one write to the hardware ?

// VLAN info
pub const IGC_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const IGC_TX_FLAGS_VLAN_SHIFT: c_int = 16;
// igc_test_staterr - tests bits within Rx descriptor status and error fields
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_state_t {
    __IGC_TESTING,
    __IGC_RESETTING,
    __IGC_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_tx_flags {
// cmd_type flags
    IGC_TX_FLAGS_VLAN	= 0x01,
    IGC_TX_FLAGS_TSO	= 0x02,
    IGC_TX_FLAGS_TSTAMP	= 0x04,

// olinfo flags
    IGC_TX_FLAGS_IPV4	= 0x10,
    IGC_TX_FLAGS_CSUM	= 0x20,

    IGC_TX_FLAGS_TSTAMP_1	= 0x100,
    IGC_TX_FLAGS_TSTAMP_2	= 0x200,
    IGC_TX_FLAGS_TSTAMP_3	= 0x400,

    IGC_TX_FLAGS_TSTAMP_TIMER_1 = 0x800,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_boards {
    board_base,
}

// The largest size we can write to the descriptor is 65535.  In order to
// maintain a power of two alignment we have to limit ourselves to 32K.
//
pub const IGC_MAX_TXD_PWR: c_int = 15;

// Tx Descriptors needed, worst case

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_rx_buffer {
    pub dma: dma_addr_t,
    pub page: *mut page,

    pub page_offset: __u32,

    pub page_offset: __u16,

    pub pagecnt_bias: __u16,
}

// context wrapper around xdp_buff to provide access to descriptor metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_xdp_buff {
    pub xdp: xdp_buff,
    pub rx_desc: *mut igc_adv_rx_desc,
    pub /: *mut *mut *mut igc_inline_rx_tstamps rx_ts; / data indication bit IGC_RXDADV_STAT_TSIP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_metadata_request {
    pub tx_buffer: *mut igc_tx_buffer,
    pub meta: *mut xsk_tx_metadata,
    pub tx_ring: *mut igc_ring,
    pub cmd_type: u32,
    pub used_desc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_q_vector {
    pub /: *mut *mut *mut igc_adapter adapter; / backlink,
    pub itr_register: *mut void __iomem,
    pub /: *mut *mut u32 eims_value; / EIMS mask value,
    pub itr_val: u16,
    pub set_itr: u8,
    pub tx: igc_ring_container rx,,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub 9]: char name[IFNAMSIZ +,
// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: igc_ring ring[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_filter_match_flags {
    IGC_FILTER_FLAG_ETHER_TYPE =	BIT(0),
    IGC_FILTER_FLAG_VLAN_TCI   =	BIT(1),
    IGC_FILTER_FLAG_SRC_MAC_ADDR =	BIT(2),
    IGC_FILTER_FLAG_DST_MAC_ADDR =	BIT(3),
    IGC_FILTER_FLAG_USER_DATA =	BIT(4),
    IGC_FILTER_FLAG_VLAN_ETYPE =	BIT(5),
    IGC_FILTER_FLAG_DEFAULT_QUEUE = BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_nfc_filter {
    pub match_flags: u8,
    pub etype: u16,
    pub vlan_etype: u16,
    pub vlan_tci: u16,
    pub vlan_tci_mask: u16,
    pub src_addr: [u8; ETH_ALEN],
    pub dst_addr: [u8; ETH_ALEN],
    pub user_data: [u8; 8],
    pub user_mask: [u8; 8],
    pub flex_index: u8,
    pub rx_queue: u8,
    pub prio: u8,
    pub immediate_irq: u8,
    pub drop: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_nfc_rule {
    pub list: list_head,
    pub filter: igc_nfc_filter,
    pub location: u32,
    pub action: u16,
    pub flex: bool,
}

// IGC supports a total of 65 NFC rules, listed below in order of priority:
// - 16 MAC address based filtering rules (highest priority)
// - 8 ethertype based filtering rules
// - 32 Flex filter based filtering rules
// - 8 VLAN priority based filtering rules
// - 1 default queue rule (lowest priority)
//
pub const IGC_MAX_RXNFC_RULES: c_int = 65;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_flex_filter {
    pub index: u8,
    pub data: [u8; 128],
    pub mask: [u8; 16],
    pub length: u8,
    pub rx_queue: u8,
    pub prio: u8,
    pub immediate_irq: u8,
    pub drop: u8,
}

// igc_desc_unused - calculate if we have unused descriptors
extern "C" {
    pub fn netdev_get_tx_queue(_arg: tx_ring->netdev, _arg: tx_ring->queue_index) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_ring_flags_t {
    IGC_RING_FLAG_RX_3K_BUFFER,
    IGC_RING_FLAG_RX_BUILD_SKB_ENABLED,
    IGC_RING_FLAG_RX_SCTP_CSUM,
    IGC_RING_FLAG_RX_LB_VLAN_BSWAP,
    IGC_RING_FLAG_TX_CTX_IDX,
    IGC_RING_FLAG_TX_DETECT_HANG,
    IGC_RING_FLAG_AF_XDP_ZC,
    IGC_RING_FLAG_TX_HWTSTAMP,
    IGC_RING_FLAG_RX_ALLOC_FAILED,
}

extern "C" {
    pub fn igc_reinit_locked(: *mut igc_adapter);
}
extern "C" {
    pub fn igc_add_nfc_rule(adapter: *mut igc_adapter, rule: *mut igc_nfc_rule) -> c_int;
}
extern "C" {
    pub fn igc_del_nfc_rule(adapter: *mut igc_adapter, rule: *mut igc_nfc_rule);
}
extern "C" {
    pub fn igc_disable_empty_addr_recv(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_enable_empty_addr_recv(adapter: *mut igc_adapter) -> c_int;
}
extern "C" {
    pub fn igc_flush_tx_descriptors(ring: *mut igc_ring);
}
extern "C" {
    pub fn igc_ptp_init(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_ptp_reset(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_ptp_suspend(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_ptp_stop(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_ptp_rx_pktstamp(adapter: *mut igc_adapter, buf: *mut __le32) -> ktime_t;
}
extern "C" {
    pub fn igc_ptp_tx_hang(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_ptp_read(adapter: *mut igc_adapter, ts: *mut timespec64);
}
extern "C" {
    pub fn igc_ptp_tx_tstamp_event(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_led_setup(adapter: *mut igc_adapter) -> c_int;
}
extern "C" {
    pub fn igc_led_free(adapter: *mut igc_adapter);
}

