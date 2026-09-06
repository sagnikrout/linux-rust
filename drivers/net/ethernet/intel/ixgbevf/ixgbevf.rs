//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbevf/ixgbevf.h
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

pub const IXGBE_MAX_TXD_PWR: c_int = 14;

// Tx Descriptors needed, worst case

// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_tx_buffer {
    pub next_to_watch: *mut ixgbe_adv_tx_desc,
    pub time_stamp: c_ulong,
    pub skb: *mut sk_buff,
// XDP uses address ptr on irq_clean
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_rx_buffer {
    pub dma: dma_addr_t,
    pub page: *mut page,

    pub page_offset: __u32,

    pub page_offset: __u16,

    pub pagecnt_bias: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_tx_queue_stats {
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_done_old: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_rx_queue_stats {
    pub alloc_rx_page_failed: u64,
    pub alloc_rx_buff_failed: u64,
    pub alloc_rx_page: u64,
    pub csum_err: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbevf_ring_state_t {
    __IXGBEVF_RX_3K_BUFFER,
    __IXGBEVF_RX_BUILD_SKB_ENABLED,
    __IXGBEVF_TX_DETECT_HANG,
    __IXGBEVF_HANG_CHECK_ARMED,
    __IXGBEVF_TX_XDP_RING,
    __IXGBEVF_TX_XDP_RING_PRIMED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_ring {
    pub next: *mut ixgbevf_ring,
    pub /: *mut *mut *mut ixgbevf_q_vector q_vector; / backpointer to q_vector,
    pub netdev: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub dev: *mut device,
    pub /: *mut *mut *mut void desc; / descriptor ring memory,
    pub /: *mut *mut dma_addr_t dma; / phys. address of descriptor ring,
    pub /: *mut *mut unsigned int size; / length in bytes,
    pub /: *mut *mut u16 count; / amount of descriptors,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub next_to_alloc: u16,
    pub tx_buffer_info: *mut ixgbevf_tx_buffer,
    pub rx_buffer_info: *mut ixgbevf_rx_buffer,
}

// holds the special value that gets the hardware register offset
// associated with this ring, which is different for DCB and RSS modes
//
// How many Rx Buffers do we bundle into one write to the hardware ?

pub const IXGBEVF_MAX_RSS_QUEUES: c_int = 2;

pub const IXGBEVF_RSS_HASH_KEY_SIZE: c_int = 40;

pub const IXGBEVF_DEFAULT_TXD: c_int = 1024;
pub const IXGBEVF_DEFAULT_RXD: c_int = 512;
pub const IXGBEVF_MAX_TXD: c_int = 4096;
pub const IXGBEVF_MIN_TXD: c_int = 64;
pub const IXGBEVF_MAX_RXD: c_int = 4096;
pub const IXGBEVF_MIN_RXD: c_int = 64;
// Supported Rx Buffer Sizes

pub const IXGBEVF_RXBUFFER_2048: c_int = 2048;
pub const IXGBEVF_RXBUFFER_3072: c_int = 3072;

pub const IXGBE_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const IXGBE_TX_FLAGS_VLAN_PRIO_MASK: c_uint = 0x0000e000;
pub const IXGBE_TX_FLAGS_VLAN_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_ring_container {
    pub /: *mut *mut *mut ixgbevf_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u8 count; / total number of rings in vector,
    pub /: *mut *mut u8 itr; / current ITR setting for ring,
}

// iterator for handling rings in ring container

// MAX_MSIX_Q_VECTORS of these are allocated,
// but we only use one per queue-specific vector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_q_vector {
    pub adapter: *mut ixgbevf_adapter,
// index of q_vector within array, also used for finding the bit in
// EICR and friends that represents the vector for this ring
//
    pub v_idx: u16,
    pub /: *mut *mut u16 itr; / Interrupt throttle rate written to EITR,
    pub napi: napi_struct,
    pub tx: ixgbevf_ring_container rx,,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub 9]: char name[IFNAMSIZ +,
// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: ixgbevf_ring ring[],
}

// microsecond values for various ITR rates shifted by 2 to fit itr register
// with the first 3 bits reserved 0
//
pub const IXGBE_MIN_RSC_ITR: c_int = 24;
pub const IXGBE_100K_ITR: c_int = 40;
pub const IXGBE_20K_ITR: c_int = 200;
pub const IXGBE_12K_ITR: c_int = 336;
// Helper macros to switch between ints/sec and what the register uses.
// And yes, it's the same math going both ways.  The lowest value
// supported by all of the ixgbe hardware is 8.
//

// ixgbevf_test_staterr - tests bits in Rx descriptor status and error fields

pub const OTHER_VECTOR: c_int = 1;

pub const MAX_MSIX_Q_VECTORS: c_int = 2;
pub const MIN_MSIX_Q_VECTORS: c_int = 1;

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_adapter {
// this field must be first, see ixgbevf_process_skb_fields
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub q_vector: [*mut ixgbevf_q_vector; MAX_MSIX_Q_VECTORS],
// Interrupt Throttle Rate
    pub rx_itr_setting: u16,
    pub tx_itr_setting: u16,
// interrupt masks
    pub eims_enable_mask: u32,
    pub eims_other: u32,
// XDP
    pub num_xdp_queues: c_int,
    pub xdp_ring: [*mut ixgbevf_ring; MAX_XDP_QUEUES],
// TX
    pub num_tx_queues: c_int,
    pub /: *mut *mut *mut ixgbevf_ring tx_ring[MAX_TX_QUEUES]; / One per active queue,
    pub restart_queue: u64,
    pub tx_timeout_count: u32,
    pub tx_ipsec: u64,
// RX
    pub num_rx_queues: c_int,
    pub /: *mut *mut *mut ixgbevf_ring rx_ring[MAX_TX_QUEUES]; / One per active queue,
    pub hw_csum_rx_error: u64,
    pub num_msix_vectors: c_int,
    pub alloc_rx_page_failed: u64,
    pub alloc_rx_buff_failed: u64,
    pub alloc_rx_page: u64,
    pub rx_ipsec: u64,
    pub msix_entries: *mut msix_entry,
// OS defined structs
    pub netdev: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub pdev: *mut pci_dev,
// structs defined in ixgbe_vf.h
    pub hw: ixgbe_hw,
    pub msg_enable: u16,
    pub pf_features: u32,

    pub stats: ixgbevf_hw_stats,
    pub state: c_ulong,
    pub tx_busy: u64,
    pub tx_ring_count: c_uint,
    pub xdp_ring_count: c_uint,
    pub rx_ring_count: c_uint,
    pub /: *mut *mut *mut u8 __iomem io_addr; / Mainly for iounmap use,
    pub link_speed: u32,
    pub link_up: bool,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub mbx_lock: spinlock_t,
    pub last_reset: c_ulong,
    pub rss_key: *mut u32,
    pub rss_indir_tbl: [u8; IXGBEVF_X550_VFRETA_SIZE],
    pub flags: u32,
    pub link_state: bool,

    pub ipsec: *mut ixgbevf_ipsec,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixbgevf_state_t {
    __IXGBEVF_TESTING,
    __IXGBEVF_RESETTING,
    __IXGBEVF_DOWN,
    __IXGBEVF_DISABLED,
    __IXGBEVF_REMOVING,
    __IXGBEVF_SERVICE_SCHED,
    __IXGBEVF_SERVICE_INITED,
    __IXGBEVF_RESET_REQUESTED,
    __IXGBEVF_QUEUE_RESET_REQUESTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbevf_boards {
    board_82599_vf,
    board_82599_vf_hv,
    board_X540_vf,
    board_X540_vf_hv,
    board_X550_vf,
    board_X550_vf_hv,
    board_X550EM_x_vf,
    board_X550EM_x_vf_hv,
    board_x550em_a_vf,
    board_e610_vf,
    board_e610_vf_hv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbevf_xcast_modes {
    IXGBEVF_XCAST_MODE_NONE = 0,
    IXGBEVF_XCAST_MODE_MULTI,
    IXGBEVF_XCAST_MODE_ALLMULTI,
    IXGBEVF_XCAST_MODE_PROMISC,
}

// needed by ethtool.c
extern "C" {
    pub fn ixgbevf_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbevf_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ixgbevf_up(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_down(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_reinit_locked(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_reset(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ixgbevf_setup_tx_resources(: *mut ixgbevf_ring) -> c_int;
}
extern "C" {
    pub fn ixgbevf_free_rx_resources(: *mut ixgbevf_ring);
}
extern "C" {
    pub fn ixgbevf_free_tx_resources(: *mut ixgbevf_ring);
}
extern "C" {
    pub fn ixgbevf_update_stats(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ethtool_ioctl(ifr: *mut ifreq) -> c_int;
}
extern "C" {
    pub fn ixgbevf_write_eitr(q_vector: *mut ixgbevf_q_vector);
}

extern "C" {
    pub fn ixgbevf_init_ipsec_offload(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_stop_ipsec_offload(adapter: *mut ixgbevf_adapter);
}
extern "C" {
    pub fn ixgbevf_ipsec_restore(adapter: *mut ixgbevf_adapter);
}

extern "C" {
    pub fn ixgbevf_poll_mbx(hw: *mut ixgbe_hw, msg: *mut u32, size: u16) -> i32;
}
extern "C" {
    pub fn ixgbevf_write_mbx(hw: *mut ixgbe_hw, msg: *mut u32, size: u16) -> i32;
}
