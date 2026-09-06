//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netdev_rx_queue.h
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

// This structure contains an instance of an RX queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_rx_queue {
    pub xdp_rxq: xdp_rxq_info,

    pub rps_map: *mut rps_map __rcu,
    pub rps_flow_table: rps_tag_ptr,

    pub kobj: kobject,
    pub groups: *const attribute_group,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
// All fields below are "ops protected",
// see comment about net_device::lock
//

    pub pool: *mut xsk_buff_pool,

    pub napi: *mut napi_struct,
    pub qcfg: netdev_queue_config,
    pub mp_params: pp_memory_provider_params,
// If a queue is leased, then the lease pointer is always
// valid. From the physical device it points to the virtual
// queue, and from the virtual device it points to the
// physical queue.
//
    pub lease: *mut netdev_rx_queue,
    pub lease_tracker: netdevice_tracker,
    pub ____cacheline_aligned_in_smp: },
//
// RX queue sysfs structures and functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_queue_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct netdev_rx_queue queue, char,
    pub len): *const *const char buf, size_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netif_lease_dir {
    NETIF_VIRT_TO_PHYS,
    NETIF_PHYS_TO_VIRT,
}

extern "C" {
    pub fn netdev_rx_queue_restart(dev: *mut net_device, rxq: c_uint) -> c_int;
}
