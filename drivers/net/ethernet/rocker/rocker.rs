//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/rocker/rocker.h
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
// drivers/net/ethernet/rocker/rocker.h - Rocker switch device driver
// Copyright (c) 2014-2016 Jiri Pirko <jiri@mellanox.com>
// Copyright (c) 2014 Scott Feldman <sfeldma@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_desc_info {
    pub /: *mut *mut *mut char data; / mapped,
    pub data_size: usize,
    pub tlv_size: usize,
    pub desc: *mut rocker_desc,
    pub mapaddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_dma_ring_info {
    pub size: usize,
    pub head: u32,
    pub tail: u32,
    pub /: *mut *mut *mut rocker_desc desc; / mapped,
    pub mapaddr: dma_addr_t,
    pub desc_info: *mut rocker_desc_info,
    pub type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_port {
    pub dev: *mut net_device,
    pub rocker: *mut rocker,
    pub wpriv: *mut c_void,
    pub port_number: c_uint,
    pub pport: u32,
    pub napi_tx: napi_struct,
    pub napi_rx: napi_struct,
    pub tx_ring: rocker_dma_ring_info,
    pub rx_ring: rocker_dma_ring_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker {
    pub pdev: *mut pci_dev,
    pub hw_addr: *mut u8 __iomem,
    pub msix_entries: *mut msix_entry,
    pub port_count: c_uint,
    pub ports: *mut rocker_port,
    pub id: u64,
    pub hw: },
    pub /: *mut *mut spinlock_t cmd_ring_lock; / for cmd ring accesses,
    pub cmd_ring: rocker_dma_ring_info,
    pub event_ring: rocker_dma_ring_info,
    pub fib_nb: notifier_block,
    pub wops: *mut rocker_world_ops,
    pub rocker_owq: *mut workqueue_struct,
    pub wpriv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocker_world_ops {
    pub kind: *const c_char,
    pub priv_size: usize,
    pub port_priv_size: usize,
    pub mode: u8,
    pub rocker): *mut *mut int (init)(struct rocker,
    pub rocker): *mut *mut void (fini)(struct rocker,
    pub rocker_port): *mut *mut int (port_pre_init)(struct rocker_port,
    pub rocker_port): *mut *mut int (port_init)(struct rocker_port,
    pub rocker_port): *mut *mut void (port_fini)(struct rocker_port,
    pub rocker_port): *mut *mut void (port_post_fini)(struct rocker_port,
    pub rocker_port): *mut *mut int (port_open)(struct rocker_port,
    pub rocker_port): *mut *mut void (port_stop)(struct rocker_port,
    pub state): u8,
    pub brport_flags): c_ulong,
    pub ageing_time): u32,
    pub vlan): *const switchdev_obj_port_vlan,
    pub vlan): *const switchdev_obj_port_vlan,
    pub addr): *const u16 vid, unsigned char,
    pub addr): *const u16 vid, unsigned char,
    pub extack): *mut netlink_ext_ack,
    pub master): *mut net_device,
    pub n): *mut neighbour,
    pub n): *mut neighbour,
    pub vlan_id): __be16,
    pub fen_info): *const fib_entry_notifier_info,
    pub fen_info): *const fib_entry_notifier_info,
    pub rocker): *mut *mut void (fib4_abort)(struct rocker,
}
