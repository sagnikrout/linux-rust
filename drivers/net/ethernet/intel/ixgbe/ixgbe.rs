//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe.h
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
// Copyright(c) 1999 - 2024 Intel Corporation.

// Macro flag: #define IXGBE_FCOE

// common prefix used by pr_<> macros

// TX/RX descriptor defines
pub const IXGBE_DEFAULT_TXD: c_int = 512;
pub const IXGBE_DEFAULT_TX_WORK: c_int = 256;
pub const IXGBE_MAX_TXD_82598: c_int = 4096;
pub const IXGBE_MAX_TXD_82599: c_int = 8192;
pub const IXGBE_MAX_TXD_X540: c_int = 8192;
pub const IXGBE_MAX_TXD_X550: c_int = 32768;
pub const IXGBE_MIN_TXD: c_int = 64;

pub const IXGBE_DEFAULT_RXD: c_int = 512;

pub const IXGBE_DEFAULT_RXD: c_int = 128;

pub const IXGBE_MAX_RXD_82598: c_int = 4096;
pub const IXGBE_MAX_RXD_82599: c_int = 8192;
pub const IXGBE_MAX_RXD_X540: c_int = 8192;
pub const IXGBE_MAX_RXD_X550: c_int = 32768;
pub const IXGBE_MIN_RXD: c_int = 64;
// flow control
pub const IXGBE_MIN_FCRTL: c_uint = 0x40;
pub const IXGBE_MAX_FCRTL: c_uint = 0x7FF80;
pub const IXGBE_MIN_FCRTH: c_uint = 0x600;
pub const IXGBE_MAX_FCRTH: c_uint = 0x7FFF0;
pub const IXGBE_DEFAULT_FCPAUSE: c_uint = 0xFFFF;
pub const IXGBE_MIN_FCPAUSE: c_int = 0;
pub const IXGBE_MAX_FCPAUSE: c_uint = 0xFFFF;
// Supported Rx Buffer Sizes

pub const IXGBE_RXBUFFER_1536: c_int = 1536;
pub const IXGBE_RXBUFFER_2K: c_int = 2048;
pub const IXGBE_RXBUFFER_3K: c_int = 3072;
pub const IXGBE_RXBUFFER_4K: c_int = 4096;

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
    pub fn ixgbe_compute_pad(_arg: rx_buf_len) -> return;
}

//
// NOTE: netdev_alloc_skb reserves up to 64 bytes, NET_IP_ALIGN means we
// reserve 64 more, and skb_shared_info adds an additional 320 bytes more,
// this adds up to 448 bytes of extra data.
//
// Since netdev_alloc_skb now allocates a page fragment we can use a value
// of 256 and the resultant skb will have a truesize of 960 or less.
//

// How many Rx Buffers do we bundle into one write to the hardware ?

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_tx_flags {
// cmd_type flags
    IXGBE_TX_FLAGS_HW_VLAN	= 0x01,
    IXGBE_TX_FLAGS_TSO	= 0x02,
    IXGBE_TX_FLAGS_TSTAMP	= 0x04,

// olinfo flags
    IXGBE_TX_FLAGS_CC	= 0x08,
    IXGBE_TX_FLAGS_IPV4	= 0x10,
    IXGBE_TX_FLAGS_CSUM	= 0x20,
    IXGBE_TX_FLAGS_IPSEC	= 0x40,

// software defined flags
    IXGBE_TX_FLAGS_SW_VLAN	= 0x80,
    IXGBE_TX_FLAGS_FCOE	= 0x100,
}

// VLAN info
pub const IXGBE_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const IXGBE_TX_FLAGS_VLAN_PRIO_MASK: c_uint = 0xe0000000;
pub const IXGBE_TX_FLAGS_VLAN_PRIO_SHIFT: c_int = 29;
pub const IXGBE_TX_FLAGS_VLAN_SHIFT: c_int = 16;
pub const IXGBE_MAX_VF_MC_ENTRIES: c_int = 30;
pub const IXGBE_MAX_VF_FUNCTIONS: c_int = 64;
pub const IXGBE_MAX_VFTA_ENTRIES: c_int = 128;
pub const MAX_EMULATION_MAC_ADDRS: c_int = 16;
pub const IXGBE_MAX_PF_MACVLANS: c_int = 15;

pub const IXGBE_82599_VF_DEVICE_ID: c_uint = 0x10ED;
pub const IXGBE_X540_VF_DEVICE_ID: c_uint = 0x1515;
pub const IXGBE_E610_VF_DEVICE_ID: c_uint = 0x57AD;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_stats {
    pub gprc: u64,
    pub gorc: u64,
    pub gptc: u64,
    pub gotc: u64,
    pub mprc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_data_storage {
    pub vfdev: *mut pci_dev,
    pub vf_mac_addresses: [c_uchar; ETH_ALEN],
    pub vf_mc_hashes: [u16; IXGBE_MAX_VF_MC_ENTRIES],
    pub num_vf_mc_hashes: u16,
    pub clear_to_send: bool,
    pub vfstats: vf_stats,
    pub last_vfstats: vf_stats,
    pub saved_rst_vfstats: vf_stats,
    pub pf_set_mac: bool,
    pub /: *mut *mut u16 pf_vlan; / When set, guest VLAN config not allowed.,
    pub pf_qos: u16,
    pub tx_rate: u16,
    pub link_enable: c_int,
    pub link_state: c_int,
    pub spoofchk_enabled: u8,
    pub rss_query_enabled: bool,
    pub trusted: u8,
    pub xcast_mode: c_int,
    pub vf_api: c_uint,
    pub primary_abort_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbevf_xcast_modes {
    IXGBEVF_XCAST_MODE_NONE = 0,
    IXGBEVF_XCAST_MODE_MULTI,
    IXGBEVF_XCAST_MODE_ALLMULTI,
    IXGBEVF_XCAST_MODE_PROMISC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_macvlans {
    pub l: list_head,
    pub vf: c_int,
    pub free: bool,
    pub is_macvlan: bool,
    pub vf_macvlan: [u8; ETH_ALEN],
}

pub const IXGBE_MAX_TXD_PWR: c_int = 14;

// Tx Descriptors needed, worst case

// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_tx_buffer {
    pub next_to_watch: *mut ixgbe_adv_tx_desc,
    pub time_stamp: c_ulong,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_rx_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: __u32,
    pub pagecnt_bias: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_queue_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_tx_queue_stats {
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_done_old: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_rx_queue_stats {
    pub rsc_count: u64,
    pub rsc_flush: u64,
    pub non_eop_descs: u64,
    pub alloc_rx_page: u64,
    pub alloc_rx_page_failed: u64,
    pub alloc_rx_buff_failed: u64,
    pub csum_err: u64,
}

pub const IXGBE_TS_HDR_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_ring_state_t {
    __IXGBE_RX_3K_BUFFER,
    __IXGBE_RX_BUILD_SKB_ENABLED,
    __IXGBE_RX_RSC_ENABLED,
    __IXGBE_RX_CSUM_UDP_ZERO_ERR,
    __IXGBE_RX_FCOE,
    __IXGBE_TX_FDIR_INIT_DONE,
    __IXGBE_TX_XPS_INIT_DONE,
    __IXGBE_TX_DETECT_HANG,
    __IXGBE_HANG_CHECK_ARMED,
    __IXGBE_TX_XDP_RING,
    __IXGBE_TX_DISABLED,
    __IXGBE_RING_STATE_NBITS, /* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fwd_adapter {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub netdev: *mut net_device,
    pub tx_base_queue: c_uint,
    pub rx_base_queue: c_uint,
    pub pool: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_ring {
    pub /: *mut *mut *mut ixgbe_ring next; / pointer to next ring in q_vector,
    pub /: *mut *mut *mut ixgbe_q_vector q_vector; / backpointer to host q_vector,
    pub /: *mut *mut *mut net_device netdev; / netdev ring belongs to,
    pub xdp_prog: *mut bpf_prog,
    pub /: *mut *mut *mut device dev; / device for DMA mapping,
    pub /: *mut *mut *mut void desc; / descriptor ring memory,
    pub tx_buffer_info: *mut ixgbe_tx_buffer,
    pub rx_buffer_info: *mut ixgbe_rx_buffer,
}

// the hardware register offset
// associated with this ring, which is
// different for DCB and RSS modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_ring_f_enum {
    RING_F_NONE = 0,
    RING_F_VMDQ,  /* SR-IOV uses the same ring feature */
    RING_F_RSS,
    RING_F_FDIR,

    RING_F_FCOE,

    RING_F_ARRAY_SIZE      /* must be last in enum set */
}

pub const IXGBE_MAX_RSS_INDICES: c_int = 16;
pub const IXGBE_MAX_RSS_INDICES_X550: c_int = 63;
pub const IXGBE_MAX_VMDQ_INDICES: c_int = 64;

pub const IXGBE_MAX_FCOE_INDICES: c_int = 8;

pub const IXGBE_MAX_L2A_QUEUES: c_int = 4;
pub const IXGBE_BAD_L2A_QUEUE: c_int = 3;
pub const IXGBE_MAX_MACVLANS: c_int = 63;
pub const IXGBE_MAX_TX_QUEUES: c_int = 128;
pub const IXGBE_MAX_TX_DESCRIPTORS: c_int = 40;
pub const IXGBE_MAX_TX_VF_HANGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_ring_feature {
    pub /: *mut *mut u16 limit; / upper limit on feature indices,
    pub /: *mut *mut u16 indices; / current value of indices,
    pub /: *mut *mut u16 mask; / Mask used for feature to ring mapping,
    pub /: *mut *mut u16 offset; / offset to start of feature,
    pub ____cacheline_internodealigned_in_smp: },
pub const IXGBE_82599_VMDQ_8Q_MASK: c_uint = 0x78;
pub const IXGBE_82599_VMDQ_4Q_MASK: c_uint = 0x7C;
pub const IXGBE_82599_VMDQ_2Q_MASK: c_uint = 0x7E;
//
// FCoE requires that all Rx buffers be over 2200 bytes in length.  Since
// this is twice the size of a half page we need to double the page order
// for FCoE enabled Rx queues.
//
    pub IXGBE_RXBUFFER_3K: return,

    pub IXGBE_MAX_2K_FRAME_BUILD_SKB: return,

    pub IXGBE_RXBUFFER_2K: return,

    pub 1: return,

    pub 0: return,

pub const IXGBE_ITR_ADAPTIVE_MIN_INC: c_int = 2;
pub const IXGBE_ITR_ADAPTIVE_MIN_USECS: c_int = 10;
pub const IXGBE_ITR_ADAPTIVE_MAX_USECS: c_int = 126;
pub const IXGBE_ITR_ADAPTIVE_LATENCY: c_uint = 0x80;
pub const IXGBE_ITR_ADAPTIVE_BULK: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_ring_container {
    pub /: *mut *mut *mut ixgbe_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned long next_update; / jiffies value of last update,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u16 work_limit; / total work allowed per interrupt,
    pub /: *mut *mut u8 count; / total number of rings in vector,
    pub /: *mut *mut u8 itr; / current ITR setting for ring,
}

// iterator for handling rings in ring container

// MAX_Q_VECTORS of these are allocated,
// but we only use one per queue-specific vector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_q_vector {
    pub adapter: *mut ixgbe_adapter,

    pub /: *mut *mut int cpu; / CPU for DCA,

    pub for: *mut *mut u16 v_idx; / index of q_vector within array, also used,
// finding the bit in EICR and friends that
// represents the vector for this ring
    pub /: *mut *mut u16 itr; / Interrupt throttle rate written to EITR,
    pub tx: ixgbe_ring_container rx,,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub affinity_mask: cpumask_t,
    pub numa_node: c_int,
    pub 9]: char name[IFNAMSIZ +,
// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: ixgbe_ring ring[],
}

pub const IXGBE_HWMON_TYPE_LOC: c_int = 0;
pub const IXGBE_HWMON_TYPE_TEMP: c_int = 1;
pub const IXGBE_HWMON_TYPE_CAUTION: c_int = 2;
pub const IXGBE_HWMON_TYPE_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_attr {
    pub dev_attr: device_attribute,
    pub hw: *mut ixgbe_hw,
    pub sensor: *mut ixgbe_thermal_diode_data,
    pub name: [c_char; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_buff {
    pub group: attribute_group,
    pub groups: [*const attribute_group; 2],
    pub 1]: *mut *mut *mut attribute attrs[IXGBE_MAX_SENSORS  4 +,
    pub 4]: *mut *mut hwmon_attr hwmon_list[IXGBE_MAX_SENSORS,
    pub n_hwmon: c_uint,
}

//
// microsecond values for various ITR rates shifted by 2 to fit itr register
// with the first 3 bits reserved 0
//
pub const IXGBE_MIN_RSC_ITR: c_int = 24;
pub const IXGBE_100K_ITR: c_int = 40;
pub const IXGBE_20K_ITR: c_int = 200;
pub const IXGBE_12K_ITR: c_int = 336;
// ixgbe_test_staterr - tests bits in Rx descriptor status and error fields

// Use 3K as the baby jumbo frame size for FCoE
pub const IXGBE_FCOE_JUMBO_FRAME_SIZE: c_int = 3072;

pub const OTHER_VECTOR: c_int = 1;

pub const MAX_MSIX_VECTORS_82599: c_int = 64;
pub const MAX_Q_VECTORS_82599: c_int = 64;
pub const MAX_MSIX_VECTORS_82598: c_int = 18;
pub const MAX_Q_VECTORS_82598: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mac_addr {
    pub addr: [u8; ETH_ALEN],
    pub pool: u16,
    pub /: *mut *mut u16 state; / bitmask,
}

pub const IXGBE_MAC_STATE_DEFAULT: c_uint = 0x1;
pub const IXGBE_MAC_STATE_MODIFIED: c_uint = 0x2;
pub const IXGBE_MAC_STATE_IN_USE: c_uint = 0x4;

pub const MIN_MSIX_Q_VECTORS: c_int = 1;

// default to trying for four seconds

pub const IXGBE_PRIMARY_ABORT_LIMIT: c_int = 5;
// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_adapter {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
// OS defined structs
    pub netdev: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub pdev: *mut pci_dev,
    pub mii_bus: *mut mii_bus,
    pub devlink: *mut devlink,
    pub devlink_port: devlink_port,
    pub nvm_region: *mut devlink_region,
    pub sram_region: *mut devlink_region,
    pub devcaps_region: *mut devlink_region,
    pub state: c_ulong,
// Some features need tri-state capability,
// thus the additional *_CAPABLE flags.
//
    pub flags: u32,

    pub flags2: u32,

// Tx fast path data
    pub num_tx_queues: c_int,
    pub tx_itr_setting: u16,
    pub tx_work_limit: u16,
    pub tx_ipsec: u64,
// Rx fast path data
    pub num_rx_queues: c_int,
    pub rx_itr_setting: u16,
    pub rx_ipsec: u64,
// Port number used to identify VXLAN traffic
    pub vxlan_port: __be16,
    pub geneve_port: __be16,
// XDP
    pub num_xdp_queues: c_int,
    pub xdp_ring: [*mut ixgbe_ring; IXGBE_MAX_XDP_QS],
    pub /: *mut *mut *mut unsigned long af_xdp_zc_qps; / tracks AF_XDP ZC enabled rings,
// TX
    pub ____cacheline_aligned_in_smp: *mut *mut ixgbe_ring tx_ring[MAX_TX_QUEUES],
    pub restart_queue: u64,
    pub lsc_int: u64,
    pub tx_timeout_count: u32,
// RX
    pub rx_ring: [*mut ixgbe_ring; MAX_RX_QUEUES],
    pub /: *mut *mut int num_rx_pools; / == num_rx_queues in 82598,
    pub /: *mut *mut int num_rx_queues_per_pool; / 1 if 82598, can be many if 82599,
    pub hw_csum_rx_error: u64,
    pub hw_rx_no_dma_resources: u64,
    pub rsc_total_count: u64,
    pub rsc_total_flush: u64,
    pub non_eop_descs: u64,
    pub alloc_rx_page: u32,
    pub alloc_rx_page_failed: u32,
    pub alloc_rx_buff_failed: u32,
    pub q_vector: [*mut ixgbe_q_vector; MAX_Q_VECTORS],
// DCB parameters
    pub ixgbe_ieee_pfc: *mut ieee_pfc,
    pub ixgbe_ieee_ets: *mut ieee_ets,
    pub dcb_cfg: ixgbe_dcb_config,
    pub temp_dcb_cfg: ixgbe_dcb_config,
    pub hw_tcs: u8,
    pub dcb_set_bitmap: u8,
    pub dcbx_cap: u8,
    pub last_lfc_mode: ixgbe_fc_mode,
    pub /: *mut *mut int num_q_vectors; / current number of q_vectors for device,
    pub /: *mut *mut int max_q_vectors; / true count of q_vectors for device,
    pub ring_feature: [ixgbe_ring_feature; RING_F_ARRAY_SIZE],
    pub msix_entries: *mut msix_entry,
    pub test_icr: u32,
    pub test_tx_ring: ixgbe_ring,
    pub test_rx_ring: ixgbe_ring,
// structs defined in ixgbe_hw.h
    pub hw: ixgbe_hw,
    pub msg_enable: u16,
    pub stats: ixgbe_hw_stats,
    pub tx_busy: u64,
    pub tx_ring_count: c_uint,
    pub xdp_ring_count: c_uint,
    pub rx_ring_count: c_uint,
    pub link_speed: u32,
    pub link_up: bool,
    pub sfp_poll_time: c_ulong,
    pub link_check_timeout: c_ulong,
    pub link_down_events: u32,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub fdir_filter_list: hlist_head,
    pub /: *mut *mut unsigned long fdir_overflow; / number of times ATR was backed off,
    pub fdir_mask: ixgbe_atr_input,
    pub fdir_filter_count: c_int,
    pub fdir_pballoc: u32,
    pub atr_sample_rate: u32,
    pub fdir_perfect_lock: spinlock_t,
    pub fw_emp_reset_disabled: bool,

    pub fcoe: ixgbe_fcoe,

    pub /: *mut *mut *mut u8 __iomem io_addr; / Mainly for iounmap use,
    pub wol: u32,
    pub bridge_mode: u16,
    pub eeprom_id: [c_char; NVM_VER_SIZE],
    pub eeprom_cap: u16,
    pub interrupt_event: u32,
    pub led_reg: u32,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
    pub ptp_tx_work: work_struct,
    pub ptp_tx_skb: *mut sk_buff,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_tx_start: c_ulong,
    pub last_overflow_check: c_ulong,
    pub last_rx_ptp_check: c_ulong,
    pub last_rx_timestamp: c_ulong,
    pub tmreg_lock: spinlock_t,
    pub hw_cc: cyclecounter,
    pub hw_tc: timecounter,
    pub base_incval: u32,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
    pub rx_hwtstamp_cleared: u32,
    pub ): *mut *mut void (ptp_setup_sdp)(struct ixgbe_adapter,
// SR-IOV
    pub IXGBE_MAX_VF_FUNCTIONS): DECLARE_BITMAP(active_vfs,,
    pub num_vfs: c_uint,
    pub vfinfo: *mut vf_data_storage,
    pub vf_rate_link_speed: c_int,
    pub vf_mvs: vf_macvlans,
    pub mv_list: *mut vf_macvlans,
    pub timer_event_accumulator: u32,
    pub vferr_refcount: u32,
    pub mac_table: *mut ixgbe_mac_addr,
    pub tx_hang_count: [u8; IXGBE_MAX_TX_QUEUES],
    pub info_kobj: *mut kobject,
    pub lse_mask: u16,

    pub ixgbe_hwmon_buff: *mut hwmon_buff,

    pub ixgbe_dbg_adapter: *mut dentry,
    pub default_up: u8,
// Bitmask indicating in use pools
    pub 1): DECLARE_BITMAP(fwd_bitmask, IXGBE_MAX_MACVLANS +,
pub const IXGBE_MAX_LINK_HANDLE: c_int = 10;
    pub jump_tables: [*mut ixgbe_jump_table; IXGBE_MAX_LINK_HANDLE],
    pub tables: c_ulong,
// maximum number of RETA entries among all devices supported by ixgbe
// driver: currently it's x550 device in non-SRIOV mode
//
pub const IXGBE_MAX_RETA_ENTRIES: c_int = 512;
    pub rss_indir_tbl: [u8; IXGBE_MAX_RETA_ENTRIES],
    pub rss_key: *mut u32,

    pub ipsec: *mut ixgbe_ipsec,

    pub vfs_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_netdevice_priv {
    pub adapter: *mut ixgbe_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_fdir_filter {
    pub fdir_node: hlist_node,
    pub filter: ixgbe_atr_input,
    pub sw_idx: u16,
    pub action: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_state_t {
    __IXGBE_TESTING,
    __IXGBE_RESETTING,
    __IXGBE_DOWN,
    __IXGBE_DISABLED,
    __IXGBE_REMOVING,
    __IXGBE_SERVICE_SCHED,
    __IXGBE_SERVICE_INITED,
    __IXGBE_IN_SFP_INIT,
    __IXGBE_PTP_RUNNING,
    __IXGBE_PTP_TX_IN_PROGRESS,
    __IXGBE_RESET_REQUESTED,
    __IXGBE_PHY_INIT_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_cb {
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_boards {
    board_82598,
    board_82599,
    board_X540,
    board_X550,
    board_X550EM_x,
    board_x550em_x_fw,
    board_x550em_a,
    board_x550em_a_fw,
    board_e610,
}

extern "C" {
    pub fn ixgbe_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbe_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbe_up(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_down(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_reinit_locked(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_reset(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ixgbe_setup_rx_resources(: *mut ixgbe_adapter, : *mut ixgbe_ring) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_tx_resources(: *mut ixgbe_ring) -> c_int;
}
extern "C" {
    pub fn ixgbe_free_rx_resources(: *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_free_tx_resources(: *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_configure_rx_ring(: *mut ixgbe_adapter, : *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_configure_tx_ring(: *mut ixgbe_adapter, : *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_disable_rx(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_disable_tx(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_update_stats(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_init_interrupt_scheme(adapter: *mut ixgbe_adapter) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_fw_version_e610(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_refresh_fw_version(adapter: *mut ixgbe_adapter) -> c_int;
}

extern "C" {
    pub fn ixgbe_full_sync_mac_table(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_update_pf_promisc_vlvf(adapter: *mut ixgbe_adapter, vid: u32);
}
extern "C" {
    pub fn ixgbe_clear_interrupt_scheme(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_alloc_rx_buffers(: *mut ixgbe_ring, _arg: u16);
}
extern "C" {
    pub fn ixgbe_write_eitr(: *mut ixgbe_q_vector);
}
extern "C" {
    pub fn ixgbe_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn ethtool_ioctl(ifr: *mut ifreq) -> c_int;
}
extern "C" {
    pub fn ixgbe_reinit_fdir_tables_82599(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_fdir_signature_82599(hw: *mut ixgbe_hw, fdirctrl: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_fdir_perfect_82599(hw: *mut ixgbe_hw, fdirctrl: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_rx_mode(netdev: *mut net_device);
}

extern "C" {
    pub fn ixgbe_set_rx_drop_en(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_setup_tc(dev: *mut net_device, tc: u8) -> c_int;
}
extern "C" {
    pub fn ixgbe_tx_ctxtdesc(: *mut ixgbe_ring, _arg: u32, _arg: u32, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn ixgbe_do_reset(netdev: *mut net_device);
}

extern "C" {
    pub fn ixgbe_sysfs_exit(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_sysfs_init(adapter: *mut ixgbe_adapter) -> c_int;
}

extern "C" {
    pub fn ixgbe_configure_fcoe(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_fcoe_ddp_put(netdev: *mut net_device, xid: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_fcoe_ddp_resources(adapter: *mut ixgbe_adapter) -> c_int;
}
extern "C" {
    pub fn ixgbe_free_fcoe_ddp_resources(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_fcoe_enable(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbe_fcoe_disable(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbe_fcoe_get_wwn(netdev: *mut net_device, wwn: *mut u64, type: c_int) -> c_int;
}
extern "C" {
    pub fn ixgbe_fcoe_get_tc(adapter: *mut ixgbe_adapter) -> u8;
}

extern "C" {
    pub fn ixgbe_dbg_adapter_init(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_dbg_adapter_exit(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_dbg_init();
}
extern "C" {
    pub fn ixgbe_dbg_exit();
}

extern "C" {
    pub fn netdev_get_tx_queue(_arg: ring->netdev, _arg: ring->queue_index) -> return;
}
extern "C" {
    pub fn ixgbe_ptp_init(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_suspend(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_stop(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_overflow_check(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_rx_hang(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_tx_hang(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_rx_pktstamp(: *mut ixgbe_q_vector, : *mut sk_buff);
}
extern "C" {
    pub fn ixgbe_ptp_rx_rgtstamp(: *mut ixgbe_q_vector, skb: *mut sk_buff);
}
// Update the last_rx_timestamp timer in order to enable watchdog check
// for error case of latched timestamp on a dropped packet.
//
extern "C" {
    pub fn ixgbe_ptp_start_cyclecounter(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_reset(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ptp_check_pps_event(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_sriov_reinit(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_rss_indir_tbl_entries(adapter: *mut ixgbe_adapter) -> u32;
}
extern "C" {
    pub fn ixgbe_store_key(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_store_reta(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_init_ipsec_offload(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_stop_ipsec_offload(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ipsec_restore(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ipsec_vf_clear(adapter: *mut ixgbe_adapter, vf: u32);
}
extern "C" {
    pub fn ixgbe_ipsec_vf_add_sa(adapter: *mut ixgbe_adapter, mbuf: *mut u32, vf: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_ipsec_vf_del_sa(adapter: *mut ixgbe_adapter, mbuf: *mut u32, vf: u32) -> c_int;
}

