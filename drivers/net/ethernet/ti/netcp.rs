//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/netcp.h
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
//
// NetCP driver local header
//
// Copyright (C) 2014 Texas Instruments Incorporated
// Authors:	Sandeep Nair <sandeep_n@ti.com>
// Sandeep Paulraj <s-paulraj@ti.com>
// Cyril Chemparathy <cyril@ti.com>
// Santosh Shilimkar <santosh.shilimkar@ti.com>
// Wingman Kwok <w-kwok2@ti.com>
// Murali Karicheri <m-karicheri2@ti.com>
//

// Maximum Ethernet frame size supported by Keystone switch
pub const NETCP_MAX_FRAME_SIZE: c_int = 9504;
pub const SGMII_LINK_MAC_MAC_AUTONEG: c_int = 0;
pub const SGMII_LINK_MAC_PHY: c_int = 1;
pub const SGMII_LINK_MAC_MAC_FORCED: c_int = 2;
pub const SGMII_LINK_MAC_FIBER: c_int = 3;
pub const SGMII_LINK_MAC_PHY_NO_MDIO: c_int = 4;
pub const RGMII_LINK_MAC_PHY: c_int = 5;
pub const RGMII_LINK_MAC_PHY_NO_MDIO: c_int = 7;
pub const XGMII_LINK_MAC_PHY: c_int = 10;
pub const XGMII_LINK_MAC_MAC_FORCED: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_tx_pipe {
    pub netcp_device: *mut netcp_device,
    pub dma_queue: *mut c_void,
    pub dma_queue_id: c_uint,
// To port for packet forwarded to switch. Used only by ethss
    pub switch_to_port: u8,

    pub flags: u8,
    pub dma_channel: *mut c_void,
    pub dma_chan_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netcp_addr_type {
    ADDR_ANY,
    ADDR_DEV,
    ADDR_UCAST,
    ADDR_MCAST,
    ADDR_BCAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_addr {
    pub netcp: *mut netcp_intf,
    pub addr: [c_uchar; ETH_ALEN],
    pub type: netcp_addr_type,
    pub flags: c_uint,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_stats {
    pub ____cacheline_aligned_in_smp: u64_stats_sync syncp_rx,
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_errors: u32,
    pub rx_dropped: u32,
    pub ____cacheline_aligned_in_smp: u64_stats_sync syncp_tx,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub tx_errors: u32,
    pub tx_dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_intf {
    pub dev: *mut device,
    pub ndev_dev: *mut device,
    pub ndev: *mut net_device,
    pub big_endian: bool,
    pub tx_compl_qid: c_uint,
    pub tx_pool: *mut c_void,
    pub txhook_list_head: list_head,
    pub tx_pause_threshold: c_uint,
    pub tx_compl_q: *mut c_void,
    pub tx_resume_threshold: c_uint,
    pub rx_queue: *mut c_void,
    pub rx_pool: *mut c_void,
    pub rxhook_list_head: list_head,
    pub rx_queue_id: c_uint,
    pub rx_fdq: [*mut c_void; KNAV_DMA_FDQ_PER_CHAN],
    pub rx_napi: napi_struct,
    pub tx_napi: napi_struct,

    pub hw_cap: u32,
// 64-bit netcp stats
    pub stats: netcp_stats,
    pub rx_channel: *mut c_void,
    pub dma_chan_name: *const c_char,
    pub rx_pool_size: u32,
    pub rx_pool_region_id: u32,
    pub tx_pool_size: u32,
    pub tx_pool_region_id: u32,
    pub module_head: list_head,
    pub interface_list: list_head,
    pub addr_list: list_head,
    pub netdev_registered: bool,
    pub primary_module_attached: bool,
// Lock used for protecting Rx/Tx hook list management
    pub lock: spinlock_t,
    pub netcp_device: *mut netcp_device,
    pub node_interface: *mut device_node,
// DMA configuration data
    pub msg_enable: u32,
    pub rx_queue_depths: [u32; KNAV_DMA_FDQ_PER_CHAN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_packet {
    pub skb: *mut sk_buff,
    pub epib: *mut __le32,
    pub psdata: *mut u32,
    pub eflags: u32,
    pub psdata_len: c_uint,
    pub netcp: *mut netcp_intf,
    pub tx_pipe: *mut netcp_tx_pipe,
    pub rxtstamp_complete: bool,
    pub ts_context: *mut c_void,
    pub skb): *mut *mut *mut void (txtstamp)(void ctx, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netcp_module {
    pub name: *const c_char,
    pub owner: *mut module,
    pub primary: bool,
// probe/remove: called once per NETCP instance
    pub inst_priv): *mut c_void,
    pub inst_priv): *mut *mut *mut int (remove)(struct netcp_device netcp_device, void,
// attach/release: called once per network interface
    pub intf_priv): *mut *mut device_node node, void,
    pub intf_priv): *mut *mut int (release)(void,
    pub ndev): *mut *mut *mut int (open)(void intf_priv, struct net_device,
    pub ndev): *mut *mut *mut int (close)(void intf_priv, struct net_device,
    pub naddr): *mut *mut *mut int (add_addr)(void intf_priv, struct netcp_addr,
    pub naddr): *mut *mut *mut int (del_addr)(void intf_priv, struct netcp_addr,
    pub vid): *mut *mut *mut int (add_vid)(void intf_priv, int,
    pub vid): *mut *mut *mut int (del_vid)(void intf_priv, int,
    pub cmd): *mut *mut *mut *mut int (ioctl)(void intf_priv, struct ifreq req, int,
    pub promisc): *mut *mut *mut int (set_rx_mode)(void intf_priv, bool,
    pub cfg): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
// used internally
    pub module_list: list_head,
    pub interface_list: list_head,
}

extern "C" {
    pub fn netcp_register_module(module: *mut netcp_module) -> c_int;
}
extern "C" {
    pub fn netcp_unregister_module(module: *mut netcp_module);
}
extern "C" {
    pub fn netcp_txpipe_open(tx_pipe: *mut netcp_tx_pipe) -> c_int;
}
extern "C" {
    pub fn netcp_txpipe_close(tx_pipe: *mut netcp_tx_pipe) -> c_int;
}
extern "C" {
    pub fn netcp_hook_rtn(order: c_int, data: *mut c_void, packet: *mut netcp_packet) -> typedef int;
}
// SGMII functions
extern "C" {
    pub fn netcp_sgmii_reset(sgmii_ofs: *mut void __iomem, port: c_int) -> c_int;
}
extern "C" {
    pub fn netcp_sgmii_rtreset(sgmii_ofs: *mut void __iomem, port: c_int, set: bool) -> bool;
}
extern "C" {
    pub fn netcp_sgmii_get_port_link(sgmii_ofs: *mut void __iomem, port: c_int) -> c_int;
}
extern "C" {
    pub fn netcp_sgmii_config(sgmii_ofs: *mut void __iomem, port: c_int, interface: u32) -> c_int;
}
// XGBE SERDES init functions
extern "C" {
    pub fn netcp_xgbe_serdes_init(serdes_regs: *mut void __iomem, xgbe_regs: *mut void __iomem) -> c_int;
}
