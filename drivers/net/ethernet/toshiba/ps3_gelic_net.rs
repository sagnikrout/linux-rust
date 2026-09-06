//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/toshiba/ps3_gelic_net.h
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
// PS3 Platfom gelic network driver.
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2006, 2007 Sony Corporation.
//
// This file is based on: spider_net.h
//
// (C) Copyright IBM Corp. 2005
//
// Authors : Utz Bacher <utz.bacher@de.ibm.com>
// Jens Osterkamp <Jens.Osterkamp@de.ibm.com>
//
// descriptors

pub const GELIC_NET_MAX_FRAME: c_int = 2312;
pub const GELIC_NET_MAX_MTU: c_int = 2294;
pub const GELIC_NET_MIN_MTU: c_int = 64;
pub const GELIC_NET_RXBUF_ALIGN: c_int = 128;

pub const GELIC_NET_BROADCAST_ADDR: c_uint = 0xffffffffffffL;

// virtual interrupt status register bits
// INT1
pub const GELIC_CARD_TX_RAM_FULL_ERR: c_uint = 0x0000000000000001L;
pub const GELIC_CARD_RX_RAM_FULL_ERR: c_uint = 0x0000000000000002L;
pub const GELIC_CARD_TX_SHORT_FRAME_ERR: c_uint = 0x0000000000000004L;
pub const GELIC_CARD_TX_INVALID_DESCR_ERR: c_uint = 0x0000000000000008L;
pub const GELIC_CARD_RX_FIFO_FULL_ERR: c_uint = 0x0000000000002000L;
pub const GELIC_CARD_RX_DESCR_CHAIN_END: c_uint = 0x0000000000004000L;
pub const GELIC_CARD_RX_INVALID_DESCR_ERR: c_uint = 0x0000000000008000L;
pub const GELIC_CARD_TX_RESPONCE_ERR: c_uint = 0x0000000000010000L;
pub const GELIC_CARD_RX_RESPONCE_ERR: c_uint = 0x0000000000100000L;
pub const GELIC_CARD_TX_PROTECTION_ERR: c_uint = 0x0000000000400000L;
pub const GELIC_CARD_RX_PROTECTION_ERR: c_uint = 0x0000000004000000L;
pub const GELIC_CARD_TX_TCP_UDP_CHECKSUM_ERR: c_uint = 0x0000000008000000L;
pub const GELIC_CARD_PORT_STATUS_CHANGED: c_uint = 0x0000000020000000L;
pub const GELIC_CARD_WLAN_EVENT_RECEIVED: c_uint = 0x0000000040000000L;
pub const GELIC_CARD_WLAN_COMMAND_COMPLETED: c_uint = 0x0000000080000000L;
// INT 0
pub const GELIC_CARD_TX_FLAGGED_DESCR: c_uint = 0x0004000000000000L;
pub const GELIC_CARD_RX_FLAGGED_DESCR: c_uint = 0x0040000000000000L;
pub const GELIC_CARD_TX_TRANSFER_END: c_uint = 0x0080000000000000L;
pub const GELIC_CARD_TX_DESCR_CHAIN_END: c_uint = 0x0100000000000000L;
pub const GELIC_CARD_NUMBER_OF_RX_FRAME: c_uint = 0x1000000000000000L;
pub const GELIC_CARD_ONE_TIME_COUNT_TIMER: c_uint = 0x4000000000000000L;
pub const GELIC_CARD_FREE_RUN_COUNT_TIMER: c_uint = 0x8000000000000000L;
// initial interrupt mask

// RX descriptor data_status bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_rx_status {
    GELIC_DESCR_RXDMADU	= 0x80000000, /* destination MAC addr unknown */
    GELIC_DESCR_RXLSTFBF	= 0x40000000, /* last frame buffer            */
    GELIC_DESCR_RXIPCHK	= 0x20000000, /* IP checksum performed        */
    GELIC_DESCR_RXTCPCHK	= 0x10000000, /* TCP/UDP checksup performed   */
    GELIC_DESCR_RXWTPKT	= 0x00C00000, /*
// wakeup trigger packet
// 01: Magic Packet (TM)
// 10: ARP packet
// 11: Multicast MAC addr
//
    GELIC_DESCR_RXVLNPKT	= 0x00200000, /* VLAN packet */
// bit 20..16 reserved
    GELIC_DESCR_RXRRECNUM	= 0x0000ff00, /* reception receipt number */
// bit 7..0 reserved
}

// TX descriptor data_status bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_tx_status {
    GELIC_DESCR_TX_TAIL	= 0x00000001, /* gelic treated this
// descriptor was end of
// a tx frame
//
}

// RX descriptor data error bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_rx_error {
// bit 31 reserved
    GELIC_DESCR_RXALNERR	= 0x40000000, /* alignement error 10/100M */
    GELIC_DESCR_RXOVERERR	= 0x20000000, /* oversize error */
    GELIC_DESCR_RXRNTERR	= 0x10000000, /* Runt error */
    GELIC_DESCR_RXIPCHKERR	= 0x08000000, /* IP checksum  error */
    GELIC_DESCR_RXTCPCHKERR	= 0x04000000, /* TCP/UDP checksum  error */
    GELIC_DESCR_RXDRPPKT	= 0x00100000, /* drop packet */
    GELIC_DESCR_RXIPFMTERR	= 0x00080000, /* IP packet format error */
// bit 18 reserved
    GELIC_DESCR_RXDATAERR	= 0x00020000, /* IP packet format error */
    GELIC_DESCR_RXCALERR	= 0x00010000, /* cariier extension length
// error
    GELIC_DESCR_RXCREXERR	= 0x00008000, /* carrier extension error */
    GELIC_DESCR_RXMLTCST	= 0x00004000, /* multicast address frame */
// bit 13..0 reserved
}

// DMA command and status (RX and TX)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_dma_status {
    GELIC_DESCR_DMA_COMPLETE            = 0x00000000, /* used in tx */
    GELIC_DESCR_DMA_BUFFER_FULL         = 0x00000000, /* used in rx */
    GELIC_DESCR_DMA_RESPONSE_ERROR      = 0x10000000, /* used in rx, tx */
    GELIC_DESCR_DMA_PROTECTION_ERROR    = 0x20000000, /* used in rx, tx */
    GELIC_DESCR_DMA_FRAME_END           = 0x40000000, /* used in rx */
    GELIC_DESCR_DMA_FORCE_END           = 0x50000000, /* used in rx, tx */
    GELIC_DESCR_DMA_CARDOWNED           = 0xa0000000, /* used in rx, tx */
    GELIC_DESCR_DMA_NOT_IN_USE          = 0xb0000000, /* any other value */
}

// tx descriptor command and status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_tx_dma_status {
// [19]
    GELIC_DESCR_TX_DMA_IKE		= 0x00080000, /* IPSEC off */
// [18]
    GELIC_DESCR_TX_DMA_FRAME_TAIL	= 0x00040000, /* last descriptor of
// the packet
//
// [17..16]
    GELIC_DESCR_TX_DMA_TCP_CHKSUM	= 0x00020000, /* TCP packet */
    GELIC_DESCR_TX_DMA_UDP_CHKSUM	= 0x00030000, /* UDP packet */
    GELIC_DESCR_TX_DMA_NO_CHKSUM	= 0x00000000, /* no checksum */

// [1]
    GELIC_DESCR_TX_DMA_CHAIN_END	= 0x00000002, /* DMA terminated
// due to chain end
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_descr_rx_dma_status {
// [ 1 ]
    GELIC_DESCR_RX_DMA_CHAIN_END	= 0x00000002, /* DMA terminated
// due to chain end
//
}

// for lv1_net_control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_net_control_code {
    GELIC_LV1_GET_MAC_ADDRESS	= 1,
    GELIC_LV1_GET_ETH_PORT_STATUS	= 2,
    GELIC_LV1_SET_NEGOTIATION_MODE	= 3,
    GELIC_LV1_GET_VLAN_ID		= 4,
    GELIC_LV1_SET_WOL		= 5,
    GELIC_LV1_GET_CHANNEL           = 6,
    GELIC_LV1_POST_WLAN_CMD		= 9,
    GELIC_LV1_GET_WLAN_CMD_RESULT	= 10,
    GELIC_LV1_GET_WLAN_EVENT	= 11,
}

// for GELIC_LV1_SET_WOL
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_wol_command {
    GELIC_LV1_WOL_MAGIC_PACKET	= 1,
    GELIC_LV1_WOL_ADD_MATCH_ADDR	= 6,
    GELIC_LV1_WOL_DELETE_MATCH_ADDR	= 7,
}

// for GELIC_LV1_WOL_MAGIC_PACKET
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_wol_mp_arg {
    GELIC_LV1_WOL_MP_DISABLE	= 0,
    GELIC_LV1_WOL_MP_ENABLE		= 1,
}

// for GELIC_LV1_WOL_{ADD,DELETE}_MATCH_ADDR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_wol_match_arg {
    GELIC_LV1_WOL_MATCH_INDIVIDUAL	= 0,
    GELIC_LV1_WOL_MATCH_ALL		= 1,
}

// status returened from GET_ETH_PORT_STATUS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_ether_port_status {
    GELIC_LV1_ETHER_LINK_UP		= 0x0000000000000001L,
    GELIC_LV1_ETHER_FULL_DUPLEX	= 0x0000000000000002L,
    GELIC_LV1_ETHER_AUTO_NEG	= 0x0000000000000004L,

    GELIC_LV1_ETHER_SPEED_10	= 0x0000000000000010L,
    GELIC_LV1_ETHER_SPEED_100	= 0x0000000000000020L,
    GELIC_LV1_ETHER_SPEED_1000	= 0x0000000000000040L,
    GELIC_LV1_ETHER_SPEED_MASK	= 0x0000000000000070L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_vlan_index {
// for outgoing packets
    GELIC_LV1_VLAN_TX_ETHERNET_0	= 0x0000000000000002L,
    GELIC_LV1_VLAN_TX_WIRELESS	= 0x0000000000000003L,

// for incoming packets
    GELIC_LV1_VLAN_RX_ETHERNET_0	= 0x0000000000000012L,
    GELIC_LV1_VLAN_RX_WIRELESS	= 0x0000000000000013L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_phy {
    GELIC_LV1_PHY_ETHERNET_0	= 0x0000000000000002L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_port_type {
    GELIC_PORT_ETHERNET_0	= 0,
    GELIC_PORT_WIRELESS	= 1,
    GELIC_PORT_MAX
}

// As defined by the gelic hardware device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_hw_regs {
    pub dev_addr: __be32,
    pub size: __be32,
    pub payload: } __packed,
    pub next_descr_addr: __be32,
    pub dmac_cmd_status: __be32,
    pub result_size: __be32,
    pub /: *mut *mut __be32 valid_size; / all zeroes for tx,
    pub data_status: __be32,
    pub /: *mut *mut __be32 data_error; / all zeroes for tx,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_chain_link {
    pub cpu_addr: dma_addr_t,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_descr {
    pub hw_regs: gelic_hw_regs,
    pub link: gelic_chain_link,
    pub skb: *mut sk_buff,
    pub next: *mut gelic_descr,
    pub prev: *mut gelic_descr,
    pub __attribute__((aligned(32))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_descr_chain {
// we walk from tail to head
    pub head: *mut gelic_descr,
    pub tail: *mut gelic_descr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_vlan_id {
    pub tx: u16,
    pub rx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_card {
    pub napi: napi_struct,
    pub netdev: [*mut net_device; GELIC_PORT_MAX],
    pub rx_oom_timer: timer_list,
//
// hypervisor requires irq_status should be
// 8 bytes aligned, but u64 member is
// always disposed in that manner
//
    pub irq_status: u64,
    pub irq_mask: u64,
    pub dev: *mut ps3_system_bus_device,
    pub vlan: [gelic_vlan_id; GELIC_PORT_MAX],
    pub vlan_required: c_int,
    pub tx_chain: gelic_descr_chain,
    pub rx_chain: gelic_descr_chain,
//
// tx_lock guards tx descriptor list and
// tx_dma_progress.
//
    pub tx_lock: spinlock_t,
    pub tx_dma_progress: c_int,
    pub tx_timeout_task: work_struct,
    pub tx_timeout_task_counter: core::sync::atomic::AtomicI32,
    pub waitq: wait_queue_head_t,
// only first user should up the card
    pub updown_lock: mutex,
    pub users: core::sync::atomic::AtomicI32,
    pub ether_port_status: u64,
    pub link_mode: c_int,
// original address returned by kzalloc
    pub unalign: *mut c_void,
//
// each netdevice has copy of irq
//
    pub irq: c_uint,
    pub rx_top: *mut *mut gelic_descr tx_top,,
    pub /: *mut *mut gelic_descr descr[]; / must be the last,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_port {
    pub card: *mut gelic_card,
    pub netdev: *mut net_device,
    pub type: gelic_port_type,
    pub /: *mut *mut long priv[]; / long for alignment,
}

extern "C" {
    pub fn gelic_card_set_irq_mask(card: *mut gelic_card, mask: u64) -> c_int;
}
// shared netdev ops
extern "C" {
    pub fn gelic_card_up(card: *mut gelic_card);
}
extern "C" {
    pub fn gelic_card_down(card: *mut gelic_card);
}
extern "C" {
    pub fn gelic_net_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn gelic_net_stop(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn gelic_net_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn gelic_net_set_multi(netdev: *mut net_device);
}
extern "C" {
    pub fn gelic_net_tx_timeout(netdev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn gelic_net_setup_netdev(netdev: *mut net_device, card: *mut gelic_card) -> c_int;
}
// shared ethtool ops
extern "C" {
    pub fn gelic_net_poll_controller(netdev: *mut net_device);
}
