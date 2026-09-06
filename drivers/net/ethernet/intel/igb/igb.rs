//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/igb.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.
// Linux PRO/1000 Ethernet Driver main header file

pub const E1000_PCS_CFG_IGN_SD: c_int = 1;
// Interrupt defines

pub const IGB_4K_ITR: c_int = 980;
pub const IGB_20K_ITR: c_int = 196;
pub const IGB_70K_ITR: c_int = 56;
// TX/RX descriptor defines
pub const IGB_DEFAULT_TXD: c_int = 256;
pub const IGB_DEFAULT_TX_WORK: c_int = 128;
pub const IGB_MIN_TXD: c_int = 64;
pub const IGB_MAX_TXD: c_int = 4096;
pub const IGB_DEFAULT_RXD: c_int = 256;
pub const IGB_MIN_RXD: c_int = 64;
pub const IGB_MAX_RXD: c_int = 4096;

pub const IGB_MAX_ITR_USECS: c_int = 10000;
pub const IGB_MIN_ITR_USECS: c_int = 10;
pub const NON_Q_VECTORS: c_int = 1;
pub const MAX_Q_VECTORS: c_int = 8;
pub const MAX_MSIX_ENTRIES: c_int = 10;
// Transmit and receive queues
pub const IGB_MAX_RX_QUEUES: c_int = 8;
pub const IGB_MAX_RX_QUEUES_82575: c_int = 4;
pub const IGB_MAX_RX_QUEUES_I211: c_int = 2;
pub const IGB_MAX_TX_QUEUES: c_int = 8;
pub const IGB_MAX_VF_MC_ENTRIES: c_int = 30;
pub const IGB_MAX_VF_FUNCTIONS: c_int = 8;
pub const IGB_MAX_VFTA_ENTRIES: c_int = 128;
pub const IGB_82576_VF_DEV_ID: c_uint = 0x10CA;
pub const IGB_I350_VF_DEV_ID: c_uint = 0x1520;
// NVM version defines
pub const IGB_MAJOR_MASK: c_uint = 0xF000;
pub const IGB_MINOR_MASK: c_uint = 0x0FF0;
pub const IGB_BUILD_MASK: c_uint = 0x000F;
pub const IGB_COMB_VER_MASK: c_uint = 0x00FF;
pub const IGB_MAJOR_SHIFT: c_int = 12;
pub const IGB_MINOR_SHIFT: c_int = 4;
pub const IGB_COMB_VER_SHFT: c_int = 8;
pub const IGB_NVM_VER_INVALID: c_uint = 0xFFFF;
pub const IGB_ETRACK_SHIFT: c_int = 16;
pub const NVM_ETRACK_WORD: c_uint = 0x0042;
pub const NVM_COMB_VER_OFF: c_uint = 0x0083;
pub const NVM_COMB_VER_PTR: c_uint = 0x003d;
// Transmit and receive latency (for PTP timestamps)
pub const IGB_I210_TX_LATENCY_10: c_int = 9542;
pub const IGB_I210_TX_LATENCY_100: c_int = 1024;
pub const IGB_I210_TX_LATENCY_1000: c_int = 178;
pub const IGB_I210_RX_LATENCY_10: c_int = 20662;
pub const IGB_I210_RX_LATENCY_100: c_int = 2213;
pub const IGB_I210_RX_LATENCY_1000: c_int = 448;
// XDP
pub const IGB_XDP_PASS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_data_storage {
    pub vf_mac_addresses: [c_uchar; ETH_ALEN],
    pub vf_mc_hashes: [u16; IGB_MAX_VF_MC_ENTRIES],
    pub num_vf_mc_hashes: u16,
    pub flags: u32,
    pub last_nack: c_ulong,
    pub /: *mut *mut u16 pf_vlan; / When set, guest VLAN config not allowed.,
    pub pf_qos: u16,
    pub tx_rate: u16,
    pub spoofchk_enabled: bool,
    pub trusted: bool,
}

// Number of unicast MAC filters reserved for the PF in the RAR registers
pub const IGB_PF_MAC_FILTERS_RESERVED: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_mac_filter {
    pub l: list_head,
    pub vf: c_int,
    pub free: bool,
    pub vf_mac: [u8; ETH_ALEN],
}

pub const IGB_VF_FLAG_CTS: c_uint = 0x00000001 /* VF is clear to send data */;
pub const IGB_VF_FLAG_UNI_PROMISC: c_uint = 0x00000002 /* VF has unicast promisc */;
pub const IGB_VF_FLAG_MULTI_PROMISC: c_uint = 0x00000004 /* VF has multicast promisc */;
pub const IGB_VF_FLAG_PF_SET_MAC: c_uint = 0x00000008 /* PF has set MAC address */;
// RX descriptor control thresholds.
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

pub const IGB_RX_HTHRESH: c_int = 8;

pub const IGB_TX_HTHRESH: c_int = 1;

// this is the size past which hardware will drop packets when setting LPE=0
pub const MAXIMUM_ETHERNET_VLAN_SIZE: c_int = 1522;

// Supported Rx Buffer Sizes
pub const IGB_RXBUFFER_256: c_int = 256;
pub const IGB_RXBUFFER_1536: c_int = 1536;
pub const IGB_RXBUFFER_2048: c_int = 2048;
pub const IGB_RXBUFFER_3072: c_int = 3072;

pub const IGB_TS_HDR_LEN: c_int = 16;
// Attempt to maximize the headroom available for incoming frames.  We
// use a 2K buffer for receives and need 1536/1534 to store the data for
// the frame.  This leaves us with 512 bytes of room.  From that we need
// to deduct the space needed for the shared info and the padding needed
// to IP align the frame.
//
// Note: For cache line sizes 256 or larger this value is going to end
// up negative.  In these cases we should fall back to the 3K
// buffers.
//

// If a 2K buffer cannot handle a standard Ethernet frame then
// optimize padding for a 3K buffer instead of a 1.5K buffer.
//
// For a 3K buffer we need to add enough padding to allow for
// tailroom due to NET_IP_ALIGN possibly shifting us out of
// cache-line alignment.
//
// if needed make room for NET_IP_ALIGN
extern "C" {
    pub fn igb_compute_pad(_arg: rx_buf_len) -> return;
}

// How many Rx Buffers do we bundle into one write to the hardware ?

pub const AUTO_ALL_MODES: c_int = 0;
pub const IGB_EEPROM_APME: c_uint = 0x0400;

// Switch to override PHY master/slave setting

pub const IGB_MNG_VLAN_NONE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igb_tx_flags {
// cmd_type flags
    IGB_TX_FLAGS_VLAN	= 0x01,
    IGB_TX_FLAGS_TSO	= 0x02,
    IGB_TX_FLAGS_TSTAMP	= 0x04,

// olinfo flags
    IGB_TX_FLAGS_IPV4	= 0x10,
    IGB_TX_FLAGS_CSUM	= 0x20,
}

// VLAN info
pub const IGB_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const IGB_TX_FLAGS_VLAN_SHIFT: c_int = 16;
// The largest size we can write to the descriptor is 65535.  In order to
// maintain a power of two alignment we have to limit ourselves to 32K.
//
pub const IGB_MAX_TXD_PWR: c_int = 15;

// Tx Descriptors needed, worst case

// EEPROM byte offsets
pub const IGB_SFF_8472_SWAP: c_uint = 0x5C;
pub const IGB_SFF_8472_COMP: c_uint = 0x5E;
// Bitmasks
pub const IGB_SFF_ADDRESSING_MODE: c_uint = 0x4;
pub const IGB_SFF_8472_UNSUP: c_uint = 0x00;
// TX resources are shared between XDP and netstack
// and we need to tag the buffer type to distinguish them
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igb_tx_buf_type {
    IGB_TYPE_SKB = 0,
    IGB_TYPE_XDP,
    IGB_TYPE_XSK
}

// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_tx_buffer {
    pub next_to_watch: *mut e1000_adv_tx_desc,
    pub time_stamp: c_ulong,
    pub type: igb_tx_buf_type,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_rx_buffer {
    pub dma: dma_addr_t,
    pub page: *mut page,

    pub page_offset: __u32,

    pub page_offset: __u16,

    pub pagecnt_bias: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_tx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub restart_queue: u64,
    pub restart_queue2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_rx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub drops: u64,
    pub csum_err: u64,
    pub alloc_failed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_ring_container {
    pub /: *mut *mut *mut igb_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u16 work_limit; / total work allowed per interrupt,
    pub /: *mut *mut u8 count; / total number of rings in vector,
    pub /: *mut *mut u8 itr; / current ITR setting for ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_ring {
    pub /: *mut *mut *mut igb_q_vector q_vector; / backlink to q_vector,
    pub /: *mut *mut *mut net_device netdev; / back pointer to net_device,
    pub xdp_prog: *mut bpf_prog,
    pub /: *mut *mut *mut device dev; / device pointer for dma mapping,
    pub tx_buffer_info: *mut igb_tx_buffer,
    pub rx_buffer_info: *mut igb_rx_buffer,
    pub rx_buffer_info_zc: *mut xdp_buff,
}

// everything past this point are written often
// TX
// RX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_q_vector {
    pub /: *mut *mut *mut igb_adapter adapter; / backlink,
    pub /: *mut *mut int cpu; / CPU for DCA,
    pub /: *mut *mut u32 eims_value; / EIMS mask value,
    pub itr_val: u16,
    pub set_itr: u8,
    pub itr_register: *mut void __iomem,
    pub tx: igb_ring_container rx,,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub 9]: char name[IFNAMSIZ +,
// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: igb_ring ring[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_ring_flags_t {
    IGB_RING_FLAG_RX_3K_BUFFER,
    IGB_RING_FLAG_RX_BUILD_SKB_ENABLED,
    IGB_RING_FLAG_RX_SCTP_CSUM,
    IGB_RING_FLAG_RX_LB_VLAN_BSWAP,
    IGB_RING_FLAG_TX_CTX_IDX,
    IGB_RING_FLAG_TX_DETECT_HANG,
    IGB_RING_FLAG_TX_DISABLED,
    IGB_RING_FLAG_RX_ALLOC_FAILED,
}

// igb_test_staterr - tests bits within Rx descriptor status and error fields
// igb_desc_unused - calculate if we have unused descriptors

pub const IGB_HWMON_TYPE_LOC: c_int = 0;
pub const IGB_HWMON_TYPE_TEMP: c_int = 1;
pub const IGB_HWMON_TYPE_CAUTION: c_int = 2;
pub const IGB_HWMON_TYPE_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_attr {
    pub dev_attr: device_attribute,
    pub hw: *mut e1000_hw,
    pub sensor: *mut e1000_thermal_diode_data,
    pub name: [c_char; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_buff {
    pub group: attribute_group,
    pub groups: [*const attribute_group; 2],
    pub 1]: *mut *mut *mut attribute attrs[E1000_MAX_SENSORS  4 +,
    pub 4]: *mut *mut hwmon_attr hwmon_list[E1000_MAX_SENSORS,
    pub n_hwmon: c_uint,
}

// The number of L2 ether-type filter registers, Index 3 is reserved
// for PTP 1588 timestamp
//

// ETQF filter list: one static filter per filter consumer. This is
// to avoid filter collisions later. Add new filters here!!
//
// Current filters:		Filter 3
//
pub const IGB_ETQF_FILTER_1588: c_int = 3;
pub const IGB_N_EXTTS: c_int = 2;
pub const IGB_N_PEROUT: c_int = 2;
pub const IGB_N_SDP: c_int = 4;
pub const IGB_RETA_SIZE: c_int = 128;
pub const IGB_RSS_KEY_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igb_filter_match_flags {
    IGB_FILTER_FLAG_ETHER_TYPE = 0x1,
    IGB_FILTER_FLAG_VLAN_TCI   = 0x2,
    IGB_FILTER_FLAG_SRC_MAC_ADDR   = 0x4,
    IGB_FILTER_FLAG_DST_MAC_ADDR   = 0x8,
}

pub const IGB_MAX_RXNFC_FILTERS: c_int = 16;
// RX network flow classification data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_nfc_input {
// Byte layout in order, all values with MSB first:
// match_flags - 1 byte
// etype - 2 bytes
// vlan_tci - 2 bytes
//
    pub match_flags: u8,
    pub etype: __be16,
    pub vlan_tci: __be16,
    pub src_addr: [u8; ETH_ALEN],
    pub dst_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_nfc_filter {
    pub nfc_node: hlist_node,
    pub filter: igb_nfc_input,
    pub cookie: c_ulong,
    pub etype_reg_index: u16,
    pub sw_idx: u16,
    pub action: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_mac_addr {
    pub addr: [u8; ETH_ALEN],
    pub queue: u8,
    pub /: *mut *mut u8 state; / bitmask,
}

pub const IGB_MAC_STATE_DEFAULT: c_uint = 0x1;
pub const IGB_MAC_STATE_IN_USE: c_uint = 0x2;
pub const IGB_MAC_STATE_SRC_ADDR: c_uint = 0x4;
pub const IGB_MAC_STATE_QUEUE_STEERING: c_uint = 0x8;
// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igb_adapter {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub netdev: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub state: c_ulong,
    pub flags: c_uint,
    pub num_q_vectors: c_uint,
    pub msix_entries: [msix_entry; MAX_MSIX_ENTRIES],
// Interrupt Throttle Rate
    pub rx_itr_setting: u32,
    pub tx_itr_setting: u32,
    pub tx_itr: u16,
    pub rx_itr: u16,
// TX
    pub tx_work_limit: u16,
    pub tx_timeout_count: u32,
    pub num_tx_queues: c_int,
    pub tx_ring: [*mut igb_ring; 16],
// RX
    pub num_rx_queues: c_int,
    pub rx_ring: [*mut igb_ring; 16],
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub watchdog_timer: timer_list,
    pub phy_info_timer: timer_list,
    pub mng_vlan_id: u16,
    pub bd_number: u32,
    pub wol: u32,
    pub en_mng_pt: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub /: *mut *mut *mut u8 __iomem io_addr; / Mainly for iounmap use,
    pub reset_task: work_struct,
    pub watchdog_task: work_struct,
    pub fc_autoneg: bool,
    pub tx_timeout_factor: u8,
    pub blink_timer: timer_list,
    pub led_status: c_ulong,
// OS defined structs
    pub pdev: *mut pci_dev,
    pub stats64_lock: spinlock_t,
    pub stats64: rtnl_link_stats64,
// structs defined in e1000_hw.h
    pub hw: e1000_hw,
    pub stats: e1000_hw_stats,
    pub phy_info: e1000_phy_info,
    pub test_icr: u32,
    pub test_tx_ring: igb_ring,
    pub test_rx_ring: igb_ring,
    pub msg_enable: c_int,
    pub q_vector: [*mut igb_q_vector; MAX_Q_VECTORS],
    pub eims_enable_mask: u32,
    pub eims_other: u32,
// to not mess up cache alignment, always add to the bottom
    pub tx_ring_count: u16,
    pub rx_ring_count: u16,
    pub vfs_allocated_count: c_uint,
    pub vf_data: *mut vf_data_storage,
    pub vf_rate_link_speed: c_int,
    pub rss_queues: u32,
    pub wvbr: u32,
    pub shadow_vfta: *mut u32,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
    pub ptp_overflow_work: delayed_work,
    pub ptp_tx_work: work_struct,
    pub ptp_tx_skb: *mut sk_buff,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_tx_start: c_ulong,
    pub last_rx_ptp_check: c_ulong,
    pub last_rx_timestamp: c_ulong,
    pub ptp_flags: c_uint,
    pub tmreg_lock: spinlock_t,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
    pub rx_hwtstamp_cleared: u32,
    pub pps_sys_wrap_on: bool,
    pub sdp_config: [ptp_pin_desc; IGB_N_SDP],
    pub start: timespec64,
    pub period: timespec64,
    pub perout: [}; IGB_N_PEROUT],
    pub fw_version: [c_char; 48],
    pub igb_hwmon_buff: *mut hwmon_buff,
    pub ets: bool,

    pub i2c_algo: i2c_algo_bit_data,
    pub i2c_adap: i2c_adapter,
    pub i2c_client: *mut i2c_client,
    pub rss_indir_tbl_init: u32,
    pub rss_indir_tbl: [u8; IGB_RETA_SIZE],
    pub rss_key: [u8; IGB_RSS_KEY_SIZE],
    pub link_check_timeout: c_ulong,
    pub copper_tries: c_int,
    pub ei: e1000_info,
    pub eee_advert: u16,
// RX network flow classification support
    pub nfc_filter_list: hlist_head,
    pub cls_flower_list: hlist_head,
    pub nfc_filter_count: c_uint,
// lock for RX network flow classification filter
    pub nfc_lock: spinlock_t,
    pub etype_bitmap: [bool; MAX_ETYPE_FILTER],
    pub mac_table: *mut igb_mac_addr,
    pub vf_macs: vf_mac_filter,
    pub vf_mac_list: *mut vf_mac_filter,
// lock for VF resources
    pub vfs_lock: spinlock_t,
}

// flags controlling PTP/1588 function

// Media Auto Sense

// DMA Coalescing defines
pub const IGB_MIN_TXPBSIZE: c_int = 20408;
pub const IGB_TX_BUF_4096: c_int = 4096;
pub const IGB_DMCTLX_DCFLUSH_DIS: c_uint = 0x80000000  /* Disable DMA Coal Flush */;
pub const IGB_82576_TSYNC_SHIFT: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_state_t {
    __IGB_TESTING,
    __IGB_RESETTING,
    __IGB_DOWN,
    __IGB_PTP_TX_IN_PROGRESS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igb_boards {
    board_82575,
}

extern "C" {
    pub fn igb_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn igb_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn igb_up(: *mut igb_adapter) -> c_int;
}
extern "C" {
    pub fn igb_down(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_reinit_locked(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_reset(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_reinit_queues(: *mut igb_adapter) -> c_int;
}
extern "C" {
    pub fn igb_write_rss_key(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_write_rss_indir_tbl(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_set_spd_dplx(: *mut igb_adapter, _arg: u32, _arg: u8) -> c_int;
}
extern "C" {
    pub fn igb_setup_tx_resources(: *mut igb_ring) -> c_int;
}
extern "C" {
    pub fn igb_setup_rx_resources(: *mut igb_ring) -> c_int;
}
extern "C" {
    pub fn igb_free_tx_resources(: *mut igb_ring);
}
extern "C" {
    pub fn igb_free_rx_resources(: *mut igb_ring);
}
extern "C" {
    pub fn igb_clean_tx_ring(tx_ring: *mut igb_ring);
}
extern "C" {
    pub fn igb_clean_rx_ring(rx_ring: *mut igb_ring);
}
extern "C" {
    pub fn igb_configure_tx_ring(: *mut igb_adapter, : *mut igb_ring);
}
extern "C" {
    pub fn igb_configure_rx_ring(: *mut igb_adapter, : *mut igb_ring);
}
extern "C" {
    pub fn igb_finalize_xdp(adapter: *mut igb_adapter, status: c_uint);
}
extern "C" {
    pub fn igb_setup_tctl(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_setup_rctl(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_setup_srrctl(: *mut igb_adapter, : *mut igb_ring);
}
extern "C" {
    pub fn igb_xmit_frame_ring(: *mut sk_buff, : *mut igb_ring) -> netdev_tx_t;
}
extern "C" {
    pub fn igb_xdp_xmit_back(adapter: *mut igb_adapter, xdp: *mut xdp_buff) -> c_int;
}
extern "C" {
    pub fn igb_alloc_rx_buffers(: *mut igb_ring, _arg: u16);
}
extern "C" {
    pub fn igb_update_stats(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_has_link(adapter: *mut igb_adapter) -> bool;
}
extern "C" {
    pub fn igb_set_ethtool_ops(: *mut net_device);
}
extern "C" {
    pub fn igb_power_up_link(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_set_fw_version(: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_init(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_stop(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_reset(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_suspend(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_rx_hang(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_tx_hang(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_ptp_rx_rgtstamp(q_vector: *mut igb_q_vector, skb: *mut sk_buff);
}
extern "C" {
    pub fn igb_set_flag_queue_pairs(: *mut igb_adapter, u32: const);
}
extern "C" {
    pub fn igb_get_max_rss_queues(: *mut igb_adapter) -> c_uint;
}

extern "C" {
    pub fn igb_sysfs_exit(adapter: *mut igb_adapter);
}
extern "C" {
    pub fn igb_sysfs_init(adapter: *mut igb_adapter) -> c_int;
}

extern "C" {
    pub fn netdev_get_tx_queue(_arg: tx_ring->netdev, _arg: tx_ring->queue_index) -> return;
}
// This function assumes __netif_tx_lock is held by the caller.
// Force memory writes to complete before letting h/w know there
// are new descriptors to fetch.
//
extern "C" {
    pub fn igb_clean_rx_ring_zc(rx_ring: *mut igb_ring);
}
extern "C" {
    pub fn igb_xmit_zc(tx_ring: *mut igb_ring, xsk_pool: *mut xsk_buff_pool) -> bool;
}
extern "C" {
    pub fn igb_xsk_wakeup(dev: *mut net_device, qid: u32, flags: u32) -> c_int;
}
