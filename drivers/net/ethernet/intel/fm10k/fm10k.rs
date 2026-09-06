//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k.h
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
// Copyright(c) 2013 - 2019 Intel Corporation.

pub const FM10K_MIN_RXD: c_int = 128;
pub const FM10K_MAX_RXD: c_int = 4096;
pub const FM10K_DEFAULT_RXD: c_int = 256;
pub const FM10K_MIN_TXD: c_int = 128;
pub const FM10K_MAX_TXD: c_int = 4096;
pub const FM10K_DEFAULT_TXD: c_int = 256;
pub const FM10K_DEFAULT_TX_WORK: c_int = 256;
pub const FM10K_RXBUFFER_256: c_int = 256;

pub const FM10K_RXBUFFER_2048: c_int = 2048;

// How many Rx Buffers do we bundle into one write to the hardware ?

pub const FM10K_MAX_STATIONS: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_l2_accel {
    pub size: c_int,
    pub count: u16,
    pub dglort: u16,
    pub rcu: rcu_head,
    pub macvlan: [*mut net_device; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_ring_state_t {
    __FM10K_TX_DETECT_HANG,
    __FM10K_HANG_CHECK_ARMED,
    __FM10K_TX_XPS_INIT_DONE,
// This must be last and is used to calculate BITMAP size
    __FM10K_TX_STATE_SIZE__,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_tx_buffer {
    pub next_to_watch: *mut fm10k_tx_desc,
    pub skb: *mut sk_buff,
    pub bytecount: c_uint,
    pub gso_segs: u16,
    pub tx_flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_rx_buffer {
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_queue_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_tx_queue_stats {
    pub restart_queue: u64,
    pub csum_err: u64,
    pub tx_busy: u64,
    pub tx_done_old: u64,
    pub csum_good: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_rx_queue_stats {
    pub alloc_failed: u64,
    pub csum_err: u64,
    pub errors: u64,
    pub csum_good: u64,
    pub switch_errors: u64,
    pub drops: u64,
    pub pp_errors: u64,
    pub link_errors: u64,
    pub length_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_ring {
    pub /: *mut *mut *mut fm10k_q_vector q_vector;/ backpointer to host q_vector,
    pub /: *mut *mut *mut net_device netdev; / netdev ring belongs to,
    pub /: *mut *mut *mut device dev; / device for DMA mapping,
    pub /: *mut *mut *mut fm10k_l2_accel __rcu l2_accel; / L2 acceleration list,
    pub /: *mut *mut *mut void desc; / descriptor ring memory,
    pub tx_buffer: *mut fm10k_tx_buffer,
    pub rx_buffer: *mut fm10k_rx_buffer,
}

// the hardware register offset
// associated with this ring, which is
// different for DCB and RSS modes
//
// Tx
// Rx
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_ring_container {
    pub /: *mut *mut *mut fm10k_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u16 work_limit; / total work allowed per interrupt,
    pub /: *mut *mut u16 itr; / interrupt throttle rate value,
    pub /: *mut *mut u8 itr_scale; / ITR adjustment based on PCI speed,
    pub /: *mut *mut u8 count; / total number of rings in vector,
}

pub const FM10K_ITR_MAX: c_uint = 0x0FFF	/* maximum value for ITR */;

pub const FM10K_ITR_ADAPTIVE: c_uint = 0x8000	/* adaptive interrupt moderation flag */;

// iterator for handling rings in ring container

pub const MAX_Q_VECTORS: c_int = 256;
pub const MIN_Q_VECTORS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_non_q_vectors {
    FM10K_MBX_VECTOR,
    NON_Q_VECTORS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_q_vector {
    pub interface: *mut fm10k_intfc,
    pub /: *mut *mut *mut u32 __iomem itr; / pointer to ITR register for this vector,
    pub /: *mut *mut u16 v_idx; / index of q_vector within interface array,
    pub tx: fm10k_ring_container rx,,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub affinity_mask: cpumask_t,
    pub 9]: char name[IFNAMSIZ +,

    pub dbg_q_vector: *mut dentry,

// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: fm10k_ring ring[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_ring_f_enum {
    RING_F_RSS,
    RING_F_QOS,
    RING_F_ARRAY_SIZE  /* must be last in enum set */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_ring_feature {
    pub /: *mut *mut u16 limit; / upper limit on feature indices,
    pub /: *mut *mut u16 indices; / current value of indices,
    pub /: *mut *mut u16 mask; / Mask used for feature to ring mapping,
    pub /: *mut *mut u16 offset; / offset to start of feature,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_iov_data {
    pub num_vfs: c_uint,
    pub next_vf_mbx: c_uint,
    pub rcu: rcu_head,
    pub vf_info: [fm10k_vf_info; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_macvlan_request_type {
    FM10K_UC_MAC_REQUEST,
    FM10K_MC_MAC_REQUEST,
    FM10K_VLAN_REQUEST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_macvlan_request {
    pub type: fm10k_macvlan_request_type,
    pub list: list_head,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mac_request {
    pub addr: [u8; ETH_ALEN],
    pub glort: u16,
    pub vid: u16,
    pub mac: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_vlan_request {
    pub vid: u32,
    pub vsi: u8,
    pub vlan: },
}

// one work queue for entire driver
// The following enumeration contains flags which indicate or enable modified
// driver behaviors. To avoid race conditions, the flags are stored in
// a BITMAP in the fm10k_intfc structure. The BITMAP should be accessed using
// atomic *_bit() operations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_flags_t {
    FM10K_FLAG_RESET_REQUESTED,
    FM10K_FLAG_RSS_FIELD_IPV4_UDP,
    FM10K_FLAG_RSS_FIELD_IPV6_UDP,
    FM10K_FLAG_SWPRI_CONFIG,
// __FM10K_FLAGS_SIZE__ is used to calculate the size of
// interface->flags and must be the last value in this
// enumeration.
//
    __FM10K_FLAGS_SIZE__
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_state_t {
    __FM10K_RESETTING,
    __FM10K_RESET_DETACHED,
    __FM10K_RESET_SUSPENDED,
    __FM10K_DOWN,
    __FM10K_SERVICE_SCHED,
    __FM10K_SERVICE_REQUEST,
    __FM10K_SERVICE_DISABLE,
    __FM10K_MACVLAN_SCHED,
    __FM10K_MACVLAN_REQUEST,
    __FM10K_MACVLAN_DISABLE,
    __FM10K_LINK_DOWN,
    __FM10K_UPDATING_STATS,
// This value must be last and determines the BITMAP size
    __FM10K_STATE_SIZE__,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_intfc {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub netdev: *mut net_device,
    pub /: *mut *mut *mut fm10k_l2_accel l2_accel; / pointer to L2 acceleration list,
    pub pdev: *mut pci_dev,
    pub __FM10K_STATE_SIZE__): DECLARE_BITMAP(state,,
// Access flag values using atomic *_bit() operations
    pub __FM10K_FLAGS_SIZE__): DECLARE_BITMAP(flags,,
    pub xcast_mode: c_int,
// Tx fast path data
    pub num_tx_queues: c_int,
    pub tx_itr: u16,
// Rx fast path data
    pub num_rx_queues: c_int,
    pub rx_itr: u16,
// TX
    pub ____cacheline_aligned_in_smp: *mut *mut fm10k_ring tx_ring[MAX_QUEUES],
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_csum_errors: u64,
    pub alloc_failed: u64,
    pub rx_csum_errors: u64,
    pub tx_bytes_nic: u64,
    pub tx_packets_nic: u64,
    pub rx_bytes_nic: u64,
    pub rx_packets_nic: u64,
    pub rx_drops_nic: u64,
    pub rx_overrun_pf: u64,
    pub rx_overrun_vf: u64,
// Debug Statistics
    pub hw_sm_mbx_full: u64,
    pub hw_csum_tx_good: u64,
    pub hw_csum_rx_good: u64,
    pub rx_switch_errors: u64,
    pub rx_drops: u64,
    pub rx_pp_errors: u64,
    pub rx_link_errors: u64,
    pub rx_length_errors: u64,
    pub tx_timeout_count: u32,
// RX
    pub rx_ring: [*mut fm10k_ring; MAX_QUEUES],
// Queueing vectors
    pub q_vector: [*mut fm10k_q_vector; MAX_Q_VECTORS],
    pub msix_entries: *mut msix_entry,
    pub /: *mut *mut int num_q_vectors; / current number of q_vectors for device,
    pub ring_feature: [fm10k_ring_feature; RING_F_ARRAY_SIZE],
// SR-IOV information management structure
    pub iov_data: *mut fm10k_iov_data,
    pub stats: fm10k_hw_stats,
    pub hw: fm10k_hw,
// Mailbox lock
    pub mbx_lock: spinlock_t,
    pub uc_addr: *mut u32 __iomem,
    pub sw_addr: *mut u32 __iomem,
    pub msg_enable: u16,
    pub tx_ring_count: u16,
    pub rx_ring_count: u16,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub next_stats_update: c_ulong,
    pub next_tx_hang_check: c_ulong,
    pub last_reset: c_ulong,
    pub link_down_event: c_ulong,
    pub host_ready: bool,
    pub lport_map_failed: bool,
    pub reta: [u32; FM10K_RETA_SIZE],
    pub rssrk: [u32; FM10K_RSSRK_SIZE],
// UDP encapsulation port tracking information
    pub vxlan_port: __be16,
    pub geneve_port: __be16,
// MAC/VLAN update queue
    pub macvlan_requests: list_head,
    pub macvlan_task: delayed_work,
// MAC/VLAN update queue lock
    pub macvlan_lock: spinlock_t,

    pub dbg_intfc: *mut dentry,

    pub pfc_en: u8,

    pub rx_pause: u8,
// GLORT resources in use by PF
    pub glort: u16,
    pub glort_count: u16,
// VLAN ID for updating multicast/unicast lists
    pub vid: u16,
}

extern "C" {
    pub fn spin_trylock(_arg: &interface->mbx_lock) -> return;
}
// fm10k_test_staterr - test bits in Rx descriptor status and error fields
// fm10k_desc_unused - calculate if we have unused descriptors

pub const FM10K_MAX_TXD_PWR: c_int = 14;

// Tx Descriptors needed, worst case

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_tx_flags {
// Tx offload flags
    FM10K_TX_FLAGS_CSUM	= 0x01,
}

// This structure is stored as little endian values as that is the native
// format of the Rx descriptor.  The ordering of these fields is reversed
// from the actual ftag header to allow for a single bswap to take care
// of placing all of the values in network order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fm10k_ftag_info {
    pub ftag: __le64,
// dglort and sglort combined into a single 32bit desc read
    pub glort: __le32,
// upper 16 bits of VLAN are reserved 0 for swpri_type_user
    pub vlan: __le32,
    pub d: },
    pub dglort: __le16,
    pub sglort: __le16,
    pub vlan: __le16,
    pub swpri_type_user: __le16,
    pub w: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_cb {
    pub tstamp: __le64,
    pub ts_tx_timeout: c_ulong,
}

// main
extern "C" {
    pub fn fm10k_init_queueing_scheme(interface: *mut fm10k_intfc) -> c_int;
}
extern "C" {
    pub fn fm10k_clear_queueing_scheme(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_tx_encap_offload(skb: *mut sk_buff) -> __be16;
}
extern "C" {
    pub fn fm10k_tx_timeout_reset(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_get_tx_pending(ring: *mut fm10k_ring, in_sw: bool) -> u64;
}
extern "C" {
    pub fn fm10k_check_tx_hang(tx_ring: *mut fm10k_ring) -> bool;
}
extern "C" {
    pub fn fm10k_alloc_rx_buffers(rx_ring: *mut fm10k_ring, cleaned_count: u16);
}
// PCI
extern "C" {
    pub fn fm10k_mbx_free_irq(: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_mbx_request_irq(: *mut fm10k_intfc) -> c_int;
}
extern "C" {
    pub fn fm10k_qv_free_irq(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_qv_request_irq(interface: *mut fm10k_intfc) -> c_int;
}
extern "C" {
    pub fn fm10k_register_pci_driver() -> c_int;
}
extern "C" {
    pub fn fm10k_unregister_pci_driver();
}
extern "C" {
    pub fn fm10k_up(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_down(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_update_stats(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_service_event_schedule(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_macvlan_schedule(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_update_rx_drop_en(interface: *mut fm10k_intfc);
}
// Netdev
extern "C" {
    pub fn fm10k_setup_rx_resources(: *mut fm10k_ring) -> c_int;
}
extern "C" {
    pub fn fm10k_setup_tx_resources(: *mut fm10k_ring) -> c_int;
}
extern "C" {
    pub fn fm10k_free_rx_resources(: *mut fm10k_ring);
}
extern "C" {
    pub fn fm10k_free_tx_resources(: *mut fm10k_ring);
}
extern "C" {
    pub fn fm10k_clean_all_rx_rings(: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_clean_all_tx_rings(: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_restore_rx_state(: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_reset_rx_state(: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_setup_tc(dev: *mut net_device, tc: u8) -> c_int;
}
extern "C" {
    pub fn fm10k_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fm10k_close(netdev: *mut net_device) -> c_int;
}
// Ethtool
extern "C" {
    pub fn fm10k_set_ethtool_ops(dev: *mut net_device);
}
extern "C" {
    pub fn fm10k_write_reta(interface: *mut fm10k_intfc, indir: *const u32);
}
// IOV
extern "C" {
    pub fn fm10k_iov_event(interface: *mut fm10k_intfc) -> i32;
}
extern "C" {
    pub fn fm10k_iov_mbx(interface: *mut fm10k_intfc) -> i32;
}
extern "C" {
    pub fn fm10k_iov_suspend(pdev: *mut pci_dev);
}
extern "C" {
    pub fn fm10k_iov_resume(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn fm10k_iov_disable(pdev: *mut pci_dev);
}
extern "C" {
    pub fn fm10k_iov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn fm10k_iov_update_stats(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_iov_update_pvid(interface: *mut fm10k_intfc, glort: u16, pvid: u16) -> i32;
}
extern "C" {
    pub fn fm10k_ndo_set_vf_mac(netdev: *mut net_device, vf_idx: c_int, mac: *mut u8) -> c_int;
}
// DebugFS

extern "C" {
    pub fn fm10k_dbg_q_vector_init(q_vector: *mut fm10k_q_vector);
}
extern "C" {
    pub fn fm10k_dbg_q_vector_exit(q_vector: *mut fm10k_q_vector);
}
extern "C" {
    pub fn fm10k_dbg_intfc_init(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_dbg_intfc_exit(interface: *mut fm10k_intfc);
}
extern "C" {
    pub fn fm10k_dbg_init();
}
extern "C" {
    pub fn fm10k_dbg_exit();
}

// DCB

extern "C" {
    pub fn fm10k_dcbnl_set_ops(dev: *mut net_device);
}

