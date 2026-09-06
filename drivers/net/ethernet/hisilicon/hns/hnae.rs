//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hnae.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2014-2015 Hisilicon Limited.
//
// Names used in this framework:
// ae handle (handle):
// a set of queues provided by AE
// ring buffer queue (rbq):
// the channel between upper layer and the AE, can do tx and rx
// ring:
// a tx or rx channel within a rbq
// ring description (desc):
// an element in the ring with packet information
// buffer:
// a memory region referred by desc with the full packet payload
//
// "num" means a static number set as a parameter, "count" mean a dynamic
// number set while running
// "cb" means control block
//

// Macro flag: #define assert(expr)

pub const AE_NAME_SIZE: c_int = 16;
pub const BD_SIZE_2048_MAX_MTU: c_int = 6000;
// some said the RX and TX RCB format should not be the same in the future. But
// it is the same now...
//
pub const RCB_REG_BASEADDR_L: c_uint = 0x00 /* P660 support only 32bit accessing */;
pub const RCB_REG_BASEADDR_H: c_uint = 0x04;
pub const RCB_REG_BD_NUM: c_uint = 0x08;
pub const RCB_REG_BD_LEN: c_uint = 0x0C;
pub const RCB_REG_PKTLINE: c_uint = 0x10;
pub const RCB_REG_TAIL: c_uint = 0x18;
pub const RCB_REG_HEAD: c_uint = 0x1C;
pub const RCB_REG_FBDNUM: c_uint = 0x20;
pub const RCB_REG_OFFSET: c_uint = 0x24 /* pkt num to be handled */;
pub const RCB_REG_PKTNUM_RECORD: c_uint = 0x2C /* total pkt received */;
pub const HNS_RX_HEAD_SIZE: c_int = 256;
pub const HNAE_AE_REGISTER: c_uint = 0x1;

pub const HNAE_LOWEST_LATENCY_COAL_PARAM: c_int = 30;
pub const HNAE_LOW_LATENCY_COAL_PARAM: c_int = 80;
pub const HNAE_BULK_LATENCY_COAL_PARAM: c_int = 150;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae_led_state {
    HNAE_LED_INACTIVE,
    HNAE_LED_ACTIVE,
    HNAE_LED_ON,
    HNAE_LED_OFF
}

pub const HNS_RX_FLAG_VLAN_PRESENT: c_uint = 0x1;
pub const HNS_RX_FLAG_L3ID_IPV4: c_uint = 0x0;
pub const HNS_RX_FLAG_L3ID_IPV6: c_uint = 0x1;
pub const HNS_RX_FLAG_L4ID_UDP: c_uint = 0x0;
pub const HNS_RX_FLAG_L4ID_TCP: c_uint = 0x1;
pub const HNS_RX_FLAG_L4ID_SCTP: c_uint = 0x3;
pub const HNS_TXD_ASID_S: c_int = 0;

pub const HNS_TXD_BUFNUM_S: c_int = 8;

pub const HNS_TXD_PORTID_S: c_int = 10;

pub const HNS_TXD_RA_B: c_int = 8;
pub const HNS_TXD_RI_B: c_int = 9;
pub const HNS_TXD_L4CS_B: c_int = 10;
pub const HNS_TXD_L3CS_B: c_int = 11;
pub const HNS_TXD_FE_B: c_int = 12;
pub const HNS_TXD_VLD_B: c_int = 13;
pub const HNS_TXD_IPOFFSET_S: c_int = 14;

pub const HNS_RXD_IPOFFSET_S: c_int = 0;

pub const HNS_RXD_BUFNUM_S: c_int = 8;

pub const HNS_RXD_PORTID_S: c_int = 10;

pub const HNS_RXD_DMAC_S: c_int = 13;

pub const HNS_RXD_VLAN_S: c_int = 15;

pub const HNS_RXD_L3ID_S: c_int = 17;

pub const HNS_RXD_L4ID_S: c_int = 21;

pub const HNS_RXD_FE_B: c_int = 25;
pub const HNS_RXD_FRAG_B: c_int = 26;
pub const HNS_RXD_VLD_B: c_int = 27;
pub const HNS_RXD_L2E_B: c_int = 28;
pub const HNS_RXD_L3E_B: c_int = 29;
pub const HNS_RXD_L4E_B: c_int = 30;
pub const HNS_RXD_DROP_B: c_int = 31;
pub const HNS_RXD_VLANID_S: c_int = 8;

pub const HNS_RXD_CFI_B: c_int = 20;
pub const HNS_RXD_PRI_S: c_int = 21;

pub const HNS_RXD_ASID_S: c_int = 24;

pub const HNSV2_TXD_BUFNUM_S: c_int = 0;

pub const HNSV2_TXD_PORTID_S: c_int = 4;

pub const HNSV2_TXD_RI_B: c_int = 1;
pub const HNSV2_TXD_L4CS_B: c_int = 2;
pub const HNSV2_TXD_L3CS_B: c_int = 3;
pub const HNSV2_TXD_FE_B: c_int = 4;
pub const HNSV2_TXD_VLD_B: c_int = 5;
pub const HNSV2_TXD_TSE_B: c_int = 0;
pub const HNSV2_TXD_VLAN_EN_B: c_int = 1;
pub const HNSV2_TXD_SNAP_B: c_int = 2;
pub const HNSV2_TXD_IPV6_B: c_int = 3;
pub const HNSV2_TXD_SCTP_B: c_int = 4;
// hardware spec ring buffer format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_desc_cb {
    pub /: *mut *mut dma_addr_t dma; / dma address of this desc,
    pub /: *mut *mut *mut void buf; / cpu addr for a desc,
// priv data for the desc, e.g. skb when use with ip stack
    pub priv: *mut c_void,
    pub page_offset: u32,
    pub /: *mut *mut u32 length; / length of the buffer,
    pub reuse_flag: u16,
// desc type, used by the ring user to mark the type of the priv data
    pub type: u16,
}

// hnae_ring->flags fields
pub const RINGF_DIR: c_uint = 0x1	    /* TX or RX ring, set if TX */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_stats {
    pub io_err_cnt: u64,
    pub sw_err_cnt: u64,
    pub seg_pkt_cnt: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
    pub tx_err_cnt: u64,
    pub restart_queue: u64,
    pub tx_busy: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_ring {
    pub /: *mut *mut *mut u8 __iomem io_base; / base io address for the ring,
    pub /: *mut *mut *mut hnae_desc desc; / dma map address space,
    pub desc_cb: *mut hnae_desc_cb,
    pub q: *mut hnae_queue,
    pub irq: c_int,
    pub ring_name: [c_char; RCB_RING_NAME_LEN],
// statistic
    pub stats: ring_stats,
    pub desc_dma_addr: dma_addr_t,
    pub /: *mut *mut u32 buf_size; / size for hnae_desc->addr, preset by AE,
    pub /: *mut *mut u16 desc_num; / total number of desc,
    pub max_desc_num_per_pkt: u16,
    pub max_raw_data_sz_per_desc: u16,
    pub max_pkt_size: u16,
    pub /: *mut *mut int next_to_use; / idx of next spare desc,
// idx of lastest sent desc, the ring is empty when equal to
// next_to_use
//
    pub next_to_clean: c_int,
    pub /: *mut *mut int flags; / ring attribute,
    pub irq_init_flag: c_int,
// total rx bytes after last rx rate calucated
    pub coal_last_rx_bytes: u64,
    pub coal_last_jiffies: c_ulong,
    pub coal_param: u32,
    pub /: *mut *mut u32 coal_rx_rate; / rx rate in MB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_desc_type {
    DESC_TYPE_SKB,
    DESC_TYPE_PAGE,
}

// the distance between [begin, end) in a ring buffer
// note: there is a unuse slot between the begin and the end
//

// allocate and dma map space for hnae desc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_buf_ops {
    pub cb): *mut *mut *mut int (alloc_buffer)(struct hnae_ring ring, struct hnae_desc_cb,
    pub cb): *mut *mut *mut void (free_buffer)(struct hnae_ring ring, struct hnae_desc_cb,
    pub cb): *mut *mut *mut int (map_buffer)(struct hnae_ring ring, struct hnae_desc_cb,
    pub cb): *mut *mut *mut void (unmap_buffer)(struct hnae_ring ring, struct hnae_desc_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_queue {
    pub io_base: *mut u8 __iomem,
    pub phy_base: phys_addr_t,
    pub /: *mut *mut *mut hnae_ae_dev dev; / the device who use this queue,
    pub ____cacheline_internodealigned_in_smp: hnae_ring rx_ring,
    pub ____cacheline_internodealigned_in_smp: hnae_ring tx_ring,
    pub handle: *mut hnae_handle,
}

// hnae loop mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae_loop {
    MAC_INTERNALLOOP_MAC = 0,
    MAC_INTERNALLOOP_SERDES,
    MAC_INTERNALLOOP_PHY,
    MAC_LOOP_PHY_NONE,
    MAC_LOOP_NONE,
}

// hnae port type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae_port_type {
    HNAE_PORT_SERVICE = 0,
    HNAE_PORT_DEBUG
}

// mac media type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hnae_media_type {
    HNAE_MEDIA_TYPE_UNKNOWN = 0,
    HNAE_MEDIA_TYPE_FIBER,
    HNAE_MEDIA_TYPE_COPPER,
    HNAE_MEDIA_TYPE_BACKPLANE,
}

// This struct defines the operation on the handle.
//
// get_handle(): (mandatory)
// Get a handle from AE according to its name and options.
// the AE driver should manage the space used by handle and its queues while
// the HNAE framework will allocate desc and desc_cb for all rings in the
// queues.
// put_handle():
// Release the handle.
// start():
// Enable the hardware, include all queues
// stop():
// Disable the hardware
// set_opts(): (mandatory)
// Set options to the AE
// get_opts(): (mandatory)
// Get options from the AE
// get_status():
// Get the carrier state of the back channel of the handle, 1 for ok, 0 for
// non-ok
// toggle_ring_irq(): (mandatory)
// Set the ring irq to be enabled(0) or disable(1)
// toggle_queue_status(): (mandatory)
// Set the queue to be enabled(1) or disable(0), this will not change the
// ring irq state
// adjust_link()
// adjust link status
// set_loopback()
// set loopback
// get_ring_bdnum_limit()
// get ring bd number limit
// get_pauseparam()
// get tx and rx of pause frame use
// set_pauseparam()
// set tx and rx of pause frame use
// get_coalesce_usecs()
// get usecs to delay a TX interrupt after a packet is sent
// get_rx_max_coalesced_frames()
// get Maximum number of packets to be sent before a TX interrupt.
// set_coalesce_usecs()
// set usecs to delay a TX interrupt after a packet is sent
// set_coalesce_frames()
// set Maximum number of packets to be sent before a TX interrupt.
// get_ringnum()
// get RX/TX ring number
// get_max_ringnum()
// get RX/TX ring maximum number
// get_mac_addr()
// get mac address
// set_mac_addr()
// set mac address
// clr_mc_addr()
// clear mcast tcam table
// set_mc_addr()
// set multicast mode
// add_uc_addr()
// add ucast address
// rm_uc_addr()
// remove ucast address
// set_mtu()
// set mtu
// update_stats()
// update Old network device statistics
// get_ethtool_stats()
// get ethtool network device statistics
// get_strings()
// get a set of strings that describe the requested objects
// get_sset_count()
// get number of strings that @get_strings will write
// update_led_status()
// update the led status
// set_led_id()
// set led id
// get_regs()
// get regs dump
// get_regs_len()
// get the len of the regs dump
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_ae_ops {
    pub port_id): u32,
    pub handle): *mut *mut void (put_handle)(struct hnae_handle,
    pub q): *mut *mut void (init_queue)(struct hnae_queue,
    pub q): *mut *mut void (fini_queue)(struct hnae_queue,
    pub handle): *mut *mut int (start)(struct hnae_handle,
    pub handle): *mut *mut void (stop)(struct hnae_handle,
    pub handle): *mut *mut void (reset)(struct hnae_handle,
    pub opts): *mut *mut *mut int (set_opts)(struct hnae_handle handle, int type, void,
    pub opts): *mut *mut *mut int (get_opts)(struct hnae_handle handle, int type, void,
    pub handle): *mut *mut int (get_status)(struct hnae_handle,
    pub duplex): *mut *mut *mut u8 auto_neg, u16 speed, u8,
    pub val): *mut *mut *mut void (toggle_ring_irq)(struct hnae_ring ring, u32,
    pub duplex): *mut *mut *mut void (adjust_link)(struct hnae_handle handle, int speed, int,
    pub duplex): int speed, int,
    pub en): hnae_loop loop_mode, int,
    pub uplimit): *mut u32,
    pub tx_en): *mut *mut *mut u32 auto_neg, u32 rx_en, u32,
    pub tx_en): u32 auto_neg, u32 rx_en, u32,
    pub rx_usecs): *mut *mut u32 tx_usecs, u32,
    pub rx_frames): *mut *mut u32 tx_frames, u32,
    pub timeout): *mut *mut *mut int (set_coalesce_usecs)(struct hnae_handle handle, u32,
    pub rx_frames): u32 tx_frames, u32,
    pub rx_usecs_high): *mut *mut u32 tx_usecs_high, u32,
    pub en): *mut *mut *mut void (set_promisc_mode)(struct hnae_handle handle, u32,
    pub p): *mut *mut *mut int (get_mac_addr)(struct hnae_handle handle, void,
    pub p): *const *const *const int (set_mac_addr)(struct hnae_handle handle, void,
    pub addr): *const c_uchar,
    pub addr): *const c_uchar,
    pub handle): *mut *mut int (clr_mc_addr)(struct hnae_handle,
    pub addr): *mut *mut *mut int (set_mc_addr)(struct hnae_handle handle, void,
    pub new_mtu): *mut *mut *mut int (set_mtu)(struct hnae_handle handle, int,
    pub enable): *mut *mut *mut void (set_tso_stats)(struct hnae_handle handle, int,
    pub net_stats): *mut net_device_stats,
    pub data): *mut *mut *mut void (get_stats)(struct hnae_handle handle, u64,
    pub data): *mut u32 stringset, u8,
    pub stringset): *mut *mut *mut int (get_sset_count)(struct hnae_handle handle, int,
    pub handle): *mut *mut void (update_led_status)(struct hnae_handle,
    pub status): hnae_led_state,
    pub data): *mut *mut *mut void (get_regs)(struct hnae_handle handle, void,
    pub handle): *mut *mut int (get_regs_len)(struct hnae_handle,
    pub handle): *mut *mut u32 (get_rss_key_size)(struct hnae_handle,
    pub handle): *mut *mut u32 (get_rss_indir_size)(struct hnae_handle,
    pub hfunc): *mut u8,
    pub hfunc): *const *const u8 key, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_ae_dev {
    pub /: *mut *mut device cls_dev; / the class dev,
    pub /: *mut *mut *mut device dev; / the presented dev,
    pub ops: *mut hnae_ae_ops,
    pub node: list_head,
    pub /: *mut *mut *mut module owner; / the module who provides this dev,
    pub id: c_int,
    pub name: [c_char; AE_NAME_SIZE],
    pub handle_list: list_head,
    pub /: *mut *mut spinlock_t lock; / lock to protect the handle_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_handle {
    pub /: *mut *mut *mut device owner_dev; / the device which make use of this handle,
    pub /: *mut *mut *mut hnae_ae_dev dev; / the device who provides this handle,
    pub phy_dev: *mut phy_device,
    pub phy_if: phy_interface_t,
    pub if_support: u32,
    pub q_num: c_int,
    pub vf_id: c_int,
    pub coal_last_jiffies: c_ulong,
    pub /: *mut *mut u32 coal_param; / self adapt coalesce param,
// the ring index of last ring that set coal param
    pub coal_ring_idx: u32,
    pub eport_id: u32,
    pub /: *mut *mut u32 dport_id; / v2 tx bd should fill the dport_id,
    pub coal_adapt_en: bool,
    pub port_type: hnae_port_type,
    pub media_type: hnae_media_type,
    pub /: *mut *mut list_head node; / list to hnae_ae_dev->handle_list,
    pub /: *mut *mut *mut hnae_buf_ops bops; / operation for the buffer,
    pub /: *mut *mut *mut hnae_queue qs[]; / flexible array of all queues,
}

extern "C" {
    pub fn hnae_put_handle(handle: *mut hnae_handle);
}
extern "C" {
    pub fn hnae_ae_register(dev: *mut hnae_ae_dev, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn hnae_ae_unregister(dev: *mut hnae_ae_dev);
}
extern "C" {
    pub fn hnae_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn hnae_unregister_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn hnae_reinit_handle(handle: *mut hnae_handle) -> c_int;
}

// Macro flag: #define assert(cond)

// detach a in-used buffer and replace with a reserved one
// when reinit buffer size, we should reinit buffer description
// when reinit buffer size, we should reinit page offset

