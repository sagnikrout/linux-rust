//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/gelic_udbg.c
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
// udbg debug output routine via GELIC UDP broadcasts
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2006, 2007 Sony Corporation
// Copyright (C) 2010 Hector Martin <hector@marcansoft.com>
// Copyright (C) 2011 Andre Heider <a.heider@gmail.com>
//

pub const GELIC_BUS_ID: c_int = 1;
pub const GELIC_DEVICE_ID: c_int = 0;
pub const GELIC_DEBUG_PORT: c_int = 18194;
pub const GELIC_MAX_MESSAGE_SIZE: c_int = 1000;
pub const GELIC_LV1_GET_MAC_ADDRESS: c_int = 1;
pub const GELIC_LV1_GET_VLAN_ID: c_int = 4;
pub const GELIC_LV1_VLAN_TX_ETHERNET_0: c_int = 2;
pub const GELIC_DESCR_DMA_STAT_MASK: c_uint = 0xf0000000;
pub const GELIC_DESCR_DMA_CARDOWNED: c_uint = 0xa0000000;
pub const GELIC_DESCR_TX_DMA_IKE: c_uint = 0x00080000;
pub const GELIC_DESCR_TX_DMA_NO_CHKSUM: c_uint = 0x00000000;
pub const GELIC_DESCR_TX_DMA_FRAME_TAIL: c_uint = 0x00040000;

    GELIC_DESCR_TX_DMA_IKE | \
    GELIC_DESCR_TX_DMA_NO_CHKSUM)
    static u64 bus_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_descr {
// as defined by the hardware
    pub buf_addr: __be32,
    pub buf_size: __be32,
    pub next_descr_addr: __be32,
    pub dmac_cmd_status: __be32,
    pub result_size: __be32,
    pub /: *mut *mut __be32 valid_size; / all zeroes for tx,
    pub data_status: __be32,
    pub /: *mut *mut __be32 data_error; / all zeroes for tx,
    pub __attribute__((aligned(32))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_block {
    pub descr: gelic_descr,
    pub pkt: [u8; 1520],
    pub __packed: },
    pub h_eth: *mut static __iomem struct ethhdr,
    pub h_vlan: *mut static __iomem struct vlan_hdr,
    pub h_ip: *mut static __iomem struct iphdr,
    pub h_udp: *mut static __iomem struct udphdr,
    pub pmsg: *mut static __iomem char,
    pub pmsgc: *mut static __iomem char,
    pub __attribute__((aligned(32))): static __iomem struct debug_block dbg,
    pub header_size: static int,
    static void map_dma_mem(int bus_id, int dev_id, void *start, size_t len,
    u64 *real_bus_addr)
    {
    pub result: i64,
    pub 0x0fffffffffffffffUL: u64 real_addr = ((u64)start) &,
    pub len: u64 real_end = real_addr +,
    pub ~0xfff: u64 map_start = real_addr &,
    pub ~0xfff: u64 map_end = (real_end + 0xfff) &,
    pub 0: u64 bus_addr =,
    pub 0xf800000000000000UL: u64 flags =,
    result = lv1_allocate_device_dma_region(bus_id, dev_id,
    map_end - map_start, 12, 0,
    if (result)
    result = lv1_map_device_dma_region(bus_id, dev_id, map_start,
    bus_addr, map_end - map_start,
    if (result)
// real_bus_addr = bus_addr + real_addr - map_start;
    }
#[no_mangle]
unsafe extern "C" fn unmap_dma_mem(bus_id: c_int, dev_id: c_int, bus_addr: u64, len: usize) -> c_int {
    static int unmap_dma_mem(int bus_id, int dev_id, u64 bus_addr, size_t len)
    {
    pub result: i64,
    pub real_bus_addr: u64,
    pub ~0xfff: real_bus_addr = bus_addr &,
    pub real_bus_addr: len += bus_addr -,
    pub ~0xfff: len = (len + 0xfff) &,
    result = lv1_unmap_device_dma_region(bus_id, dev_id, real_bus_addr,
    if (result)
    pub result: return,
    pub real_bus_addr): return lv1_free_device_dma_region(bus_id, dev_id,,
    }
#[no_mangle]
unsafe extern "C" fn gelic_debug_init() -> void __init {
    static void __init gelic_debug_init(void)
    {
    pub result: i64,
    pub v2: u64,
    pub mac: u64,
    pub vlan_id: u64,
    pub 0): result = lv1_open_device(GELIC_BUS_ID, GELIC_DEVICE_ID,,
    if (result)
    map_dma_mem(GELIC_BUS_ID, GELIC_DEVICE_ID, &dbg, sizeof(dbg),
    pub sizeof(dbg)): memset(&dbg, 0,,
    pub pkt): dbg.descr.buf_addr = bus_addr + offsetof(struct debug_block,,
    result = lv1_net_control(GELIC_BUS_ID, GELIC_DEVICE_ID,
    GELIC_LV1_GET_MAC_ADDRESS, 0, 0, 0,
    pub &v2): &mac,,
    if (result)
    pub 16: mac <<=,
    pub )dbg.pkt: *mut h_eth = (struct ethhdr,
    pub ETH_ALEN): memcpy(&h_eth->h_source, &mac,,
    pub ethhdr): header_size = sizeof(struct,
    result = lv1_net_control(GELIC_BUS_ID, GELIC_DEVICE_ID,
    GELIC_LV1_GET_VLAN_ID,
    GELIC_LV1_VLAN_TX_ETHERNET_0, 0, 0,
    pub &v2): &vlan_id,,
    if (!result) {
    pub ETH_P_8021Q: h_eth->h_proto=,
    pub vlan_hdr): header_size += sizeof(struct,
    pub 1): *mut *mut h_vlan = (struct vlan_hdr )(h_eth +,
    pub vlan_id: h_vlan->h_vlan_TCI =,
    pub ETH_P_IP: h_vlan->h_vlan_encapsulated_proto =,
    pub 1): *mut *mut h_ip = (struct iphdr )(h_vlan +,
    } else {
    pub 0x0800: h_eth->h_proto=,
    pub 1): *mut *mut h_ip = (struct iphdr )(h_eth +,
    }
    pub iphdr): header_size += sizeof(struct,
    pub 4: h_ip->version =,
    pub 5: h_ip->ihl =,
    pub 10: h_ip->ttl =,
    pub 0x11: h_ip->protocol =,
    pub 0x00000000: h_ip->saddr =,
    pub 0xffffffff: h_ip->daddr =,
    pub udphdr): header_size += sizeof(struct,
    pub 1): *mut *mut h_udp = (struct udphdr )(h_ip +,
    pub GELIC_DEBUG_PORT: h_udp->source =,
    pub GELIC_DEBUG_PORT: h_udp->dest =,
    pub 1): *mut *mut pmsgc = pmsg = (char )(h_udp +,
    }
#[no_mangle]
unsafe extern "C" fn gelic_debug_shutdown() {
    static void gelic_debug_shutdown(void)
    {
    if (bus_addr)
    unmap_dma_mem(GELIC_BUS_ID, GELIC_DEVICE_ID,
    pub sizeof(dbg)): bus_addr,,
    pub GELIC_DEVICE_ID): lv1_close_device(GELIC_BUS_ID,,
    }
#[no_mangle]
unsafe extern "C" fn gelic_sendbuf(msgsize: c_int) {
    static void gelic_sendbuf(int msgsize)
    {
    pub p: *mut u16,
    pub sum: u32,
    pub i: c_int,
    pub msgsize: dbg.descr.buf_size = header_size +,
    h_ip.tot_len = msgsize + sizeof(struct udphdr) +
    pub iphdr): sizeof(struct,
    pub udphdr): h_udp->len = msgsize + sizeof(struct,
    pub 0: h_ip->check =,
    pub 0: sum =,
    pub )h_ip: *mut p = (u16,
    pub i++): for (i = 0; i < 5;,
    pub p++: *mut sum +=,
    pub 16)): h_ip->check = ~(sum + (sum >>,
    dbg.descr.dmac_cmd_status = GELIC_DESCR_DMA_CMD_NO_CHKSUM |
    pub 0: dbg.descr.result_size =,
    pub 0: dbg.descr.data_status =,
    pub 0): lv1_net_start_tx_dma(GELIC_BUS_ID, GELIC_DEVICE_ID, bus_addr,,
    while ((dbg.descr.dmac_cmd_status & GELIC_DESCR_DMA_STAT_MASK) ==
    GELIC_DESCR_DMA_CARDOWNED)
    }
#[no_mangle]
unsafe extern "C" fn ps3gelic_udbg_putc(ch: c_char) {
    static void ps3gelic_udbg_putc(char ch)
    {
// pmsgc++ = ch;
    if (ch == '\n' || (pmsgc-pmsg) >= GELIC_MAX_MESSAGE_SIZE) {
    pub pmsg: pmsgc =,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_ps3gelic() -> void __init {
    void __init udbg_init_ps3gelic(void)
    {
    pub ps3gelic_udbg_putc: udbg_putc =,
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_shutdown_ps3gelic() {
    void udbg_shutdown_ps3gelic(void)
    {
    pub NULL: udbg_putc =,
    }
