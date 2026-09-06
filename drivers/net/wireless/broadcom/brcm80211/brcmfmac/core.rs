//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/core.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//
// Common types
//

// Macro flag: #define BRCMF_EXPORT_SYMBOL_GPL(__sym)

pub const TOE_TX_CSUM_OL: c_uint = 0x00000001;
pub const TOE_RX_CSUM_OL: c_uint = 0x00000002;
// For supporting multiple interfaces
pub const BRCMF_MAX_IFS: c_int = 16;
// Small, medium and maximum buffer size for dcmd
//
pub const BRCMF_DCMD_SMLEN: c_int = 256;
pub const BRCMF_DCMD_MEDLEN: c_int = 1536;
pub const BRCMF_DCMD_MAXLEN: c_int = 8192;
// IOCTL from host to device are limited in length. A device can only handle
// ethernet frame size. This limitation is to be applied by protocol layer.
//

pub const BRCMF_AMPDU_RX_REORDER_MAXFLOWS: c_int = 256;
// Length of firmware version string stored for
// ethtool driver info which uses 32 bytes as well.
//
pub const BRCMF_DRIVER_FIRMWARE_VERSION_LEN: c_int = 32;
pub const NDOL_MAX_ENTRIES: c_int = 8;
//
// struct brcmf_ampdu_rx_reorder - AMPDU receive reorder info
//
// @flow_id: AMPDU flow identifier.
// @cur_idx: last AMPDU index from firmware.
// @exp_idx: expected next AMPDU index.
// @max_idx: maximum amount of packets per AMPDU.
// @pend_pkts: number of packets currently in @pktslots.
// @pktslots: array for ordering AMPDU packets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_ampdu_rx_reorder {
    pub flow_id: u8,
    pub cur_idx: u8,
    pub exp_idx: u8,
    pub max_idx: u8,
    pub pend_pkts: u8,
    pub pktslots: [*mut sk_buff; ],
}

// Forward decls for struct brcmf_pub (see below)
//
// struct brcmf_rev_info
//
// The result field stores the error code of the
// revision info request from firmware. For the
// other fields see struct brcmf_rev_info_le in
// fwil_types.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_rev_info {
    pub result: c_int,
    pub vendorid: u32,
    pub deviceid: u32,
    pub radiorev: u32,
    pub corerev: u32,
    pub boardid: u32,
    pub boardvendor: u32,
    pub boardrev: u32,
    pub driverrev: u32,
    pub ucoderev: u32,
    pub bus: u32,
    pub chipname: [c_char; 12],
    pub phytype: u32,
    pub phyrev: u32,
    pub anarev: u32,
    pub chippkg: u32,
    pub nvramrev: u32,
}

// Common structure for module and instance linkage
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pub {
// Linkage ponters
    pub bus_if: *mut brcmf_bus,
    pub proto: *mut brcmf_proto,
    pub wiphy: *mut wiphy,
    pub ops: *mut cfg80211_ops,
    pub config: *mut brcmf_cfg80211_info,
// Internal brcmf items
    pub /: *mut *mut uint hdrlen; / Total BRCMF header length (proto + bus),
// Dongle media info
    pub fwver: [c_char; BRCMF_DRIVER_FIRMWARE_VERSION_LEN],
    pub /: *mut *mut u8 mac[ETH_ALEN]; / MAC address obtained from dongle,
    pub addresses: [mac_address; BRCMF_MAX_IFS],
    pub iflist: [*mut brcmf_if; BRCMF_MAX_IFS],
    pub if2bss: [i32; BRCMF_MAX_IFS],
    pub mon_if: *mut brcmf_if,
    pub proto_block: mutex,
    pub proto_buf: [c_uchar; BRCMF_DCMD_MAXLEN],
    pub fweh: *mut brcmf_fweh_info,
// reorder_flows[BRCMF_AMPDU_RX_REORDER_MAXFLOWS];
    pub feat_flags: u32,
    pub chip_quirks: u32,
    pub revinfo: brcmf_rev_info,

    pub dbgfs_dir: *mut dentry,

    pub inetaddr_notifier: notifier_block,
    pub inet6addr_notifier: notifier_block,
    pub settings: *mut brcmf_mp_device,
    pub bus_reset: work_struct,
    pub clmver: [u8; BRCMF_DCMD_SMLEN],
    pub sta_mac_idx: u8,
    pub vops: *const brcmf_fwvid_ops,
    pub vdata: *mut c_void,
}

// forward declarations
//
// enum brcmf_netif_stop_reason - reason for stopping netif queue.
//
// @BRCMF_NETIF_STOP_REASON_FWS_FC:
// netif stopped due to firmware signalling flow control.
// @BRCMF_NETIF_STOP_REASON_FLOW:
// netif stopped due to flowring full.
// @BRCMF_NETIF_STOP_REASON_DISCONNECTED:
// netif stopped due to not being connected (STA mode).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_netif_stop_reason {
    BRCMF_NETIF_STOP_REASON_FWS_FC = BIT(0),
    BRCMF_NETIF_STOP_REASON_FLOW = BIT(1),
    BRCMF_NETIF_STOP_REASON_DISCONNECTED = BIT(2)
}

//
// struct brcmf_if - interface control information.
//
// @drvr: points to device related information.
// @vif: points to cfg80211 specific interface information.
// @ndev: associated network device.
// @multicast_work: worker object for multicast provisioning.
// @ndoffload_work: worker object for neighbor discovery offload configuration.
// @fws_desc: interface specific firmware-signalling descriptor.
// @ifidx: interface index in device firmware.
// @bsscfgidx: index of bss associated with this interface.
// @mac_addr: assigned mac address.
// @netif_stop: bitmap indicates reason why netif queues are stopped.
// @netif_stop_lock: spinlock for update netif_stop from multiple sources.
// @pend_8021x_cnt: tracks outstanding number of 802.1x frames.
// @pend_8021x_wait: used for signalling change in count.
// @fwil_fwerr: flag indicating fwil layer should return firmware error codes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_if {
    pub drvr: *mut brcmf_pub,
    pub vif: *mut brcmf_cfg80211_vif,
    pub ndev: *mut net_device,
    pub multicast_work: work_struct,
    pub ndoffload_work: work_struct,
    pub fws_desc: *mut brcmf_fws_mac_descriptor,
    pub ifidx: c_int,
    pub bsscfgidx: i32,
    pub mac_addr: [u8; ETH_ALEN],
    pub netif_stop: u8,
    pub netif_stop_lock: spinlock_t,
    pub pend_8021x_cnt: core::sync::atomic::AtomicI32,
    pub pend_8021x_wait: wait_queue_head_t,
    pub ipv6_addr_tbl: [in6_addr; NDOL_MAX_ENTRIES],
    pub ipv6addr_idx: u8,
    pub fwil_fwerr: bool,
}

extern "C" {
    pub fn brcmf_netdev_wait_pend8021x(ifp: *mut brcmf_if) -> c_int;
}
// Return pointer to interface name
extern "C" {
    pub fn brcmf_configure_arp_nd_offload(ifp: *mut brcmf_if, enable: bool);
}
extern "C" {
    pub fn brcmf_net_attach(ifp: *mut brcmf_if, locked: bool) -> c_int;
}
extern "C" {
    pub fn brcmf_remove_interface(ifp: *mut brcmf_if, locked: bool);
}
extern "C" {
    pub fn brcmf_txfinalize(ifp: *mut brcmf_if, txp: *mut sk_buff, success: bool);
}
extern "C" {
    pub fn brcmf_netif_rx(ifp: *mut brcmf_if, skb: *mut sk_buff);
}
extern "C" {
    pub fn brcmf_netif_mon_rx(ifp: *mut brcmf_if, skb: *mut sk_buff);
}
extern "C" {
    pub fn brcmf_net_detach(ndev: *mut net_device, locked: bool);
}
extern "C" {
    pub fn brcmf_net_mon_attach(ifp: *mut brcmf_if) -> c_int;
}
extern "C" {
    pub fn brcmf_net_setcarrier(ifp: *mut brcmf_if, on: bool);
}
extern "C" {
    pub fn brcmf_core_init() -> int __init;
}
extern "C" {
    pub fn brcmf_core_exit() -> void __exit;
}
