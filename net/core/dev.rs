//! Automatically rewritten from C Header to Rust Module
//! Source: net/core/dev.h
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

// Random bits of netdevice that don't need to be exposed

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_flow_limit {
    pub rcu: rcu_head,
    pub count: c_uint,
    pub log_buckets: u8,
    pub history_head: c_uint,
    pub history: [u16; FLOW_LIMIT_HISTORY],
    pub buckets: [u8; ],
}

extern "C" {
    pub fn netdev_put_lock(_arg: dev, _arg: net, _arg: NULL) -> return;
}

extern "C" {
    pub fn dev_proc_init() -> int __init;
}

pub const dev_proc_init(): c_int = 0;

extern "C" {
    pub fn linkwatch_init_dev(dev: *mut net_device);
}
extern "C" {
    pub fn linkwatch_run_queue();
}
extern "C" {
    pub fn dev_addr_flush(dev: *mut net_device);
}
extern "C" {
    pub fn dev_addr_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_addr_check(dev: *mut net_device);
}
extern "C" {
    pub fn __hw_addr_flush(list: *mut netdev_hw_addr_list);
}

extern "C" {
    pub fn net_shaper_flush_netdev(dev: *mut net_device);
}

// sysctls not referred to from outside net/core/
// rtnl helpers
extern "C" {
    pub fn netdev_run_todo();
}
extern "C" {
    pub fn netif_rxq_has_mp(dev: *mut net_device, rxq_idx: c_uint) -> bool;
}
extern "C" {
    pub fn netif_rxq_is_leased(dev: *mut net_device, rxq_idx: c_uint) -> bool;
}
extern "C" {
    pub fn netif_is_queue_leasee(dev: *const net_device) -> bool;
}
// netdev management, shared between various uAPI entry points
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_name_node {
    pub hlist: hlist_node,
    pub list: list_head,
    pub dev: *mut net_device,
    pub name: *const c_char,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn netdev_get_name(net: *mut net, name: *mut c_char, ifindex: c_int) -> c_int;
}
extern "C" {
    pub fn netif_change_name(dev: *mut net_device, newname: *const c_char) -> c_int;
}
extern "C" {
    pub fn dev_change_name(dev: *mut net_device, newname: *const c_char) -> c_int;
}

extern "C" {
    pub fn netdev_name_node_alt_create(dev: *mut net_device, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn netdev_name_node_alt_destroy(dev: *mut net_device, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn netif_change_proto_down(dev: *mut net_device, proto_down: bool) -> c_int;
}
extern "C" {
    pub fn dev_change_proto_down(dev: *mut net_device, proto_down: bool) -> c_int;
}
extern "C" {
    pub fn int(dev: *mut *mut bpf_op_t)(struct net_device, bpf: *mut netdev_bpf) -> typedef;
}
extern "C" {
    pub fn netif_change_tx_queue_len(dev: *mut net_device, new_len: c_ulong) -> c_int;
}
extern "C" {
    pub fn dev_change_tx_queue_len(dev: *mut net_device, new_len: c_ulong) -> c_int;
}
extern "C" {
    pub fn netif_set_group(dev: *mut net_device, new_group: c_int);
}
extern "C" {
    pub fn dev_set_group(dev: *mut net_device, new_group: c_int);
}
extern "C" {
    pub fn netif_change_carrier(dev: *mut net_device, new_carrier: bool) -> c_int;
}
extern "C" {
    pub fn dev_change_carrier(dev: *mut net_device, new_carrier: bool) -> c_int;
}
extern "C" {
    pub fn __dev_set_rx_mode(dev: *mut net_device);
}
extern "C" {
    pub fn __dev_set_promiscuity(dev: *mut net_device, inc: c_int, notify: bool) -> c_int;
}
extern "C" {
    pub fn netif_rx_mode_init(dev: *mut net_device);
}
extern "C" {
    pub fn netif_rx_mode_run(dev: *mut net_device);
}
extern "C" {
    pub fn netif_rx_mode_sync(dev: *mut net_device);
}
extern "C" {
    pub fn netif_rx_mode_cancel_retry(dev: *mut net_device);
}
// Events for the async netdev work, tracked in netdev->work_core_pending.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_work_core {
    NETDEV_WORK_RX_MODE	= BIT(0),	/* run the rx_mode update */
}

extern "C" {
    pub fn __netdev_work_core_sched(dev: *mut net_device, event: c_ulong);
}
extern "C" {
    pub fn netdev_work_cancel_all(dev: *mut net_device);
}
// dev->gso_max_size is read locklessly from sk_setup_caps()
// dev->gso_max_segs is read locklessly from sk_setup_caps()
// This pairs with the READ_ONCE() in skb_gro_receive()
// dev->gso_ipv4_max_size is read locklessly from sk_setup_caps()
// This pairs with the READ_ONCE() in skb_gro_receive()
//
// napi_get_defer_hard_irqs - get the NAPI's defer_hard_irqs
// @n: napi struct to get the defer_hard_irqs field from
//
// Return: the per-NAPI value of the defar_hard_irqs field.
//
extern "C" {
    pub fn READ_ONCE(_arg: n->defer_hard_irqs) -> return;
}
//
// napi_set_defer_hard_irqs - set the defer_hard_irqs for a napi
// @n: napi_struct to set the defer_hard_irqs field
// @defer: the value the field should be set to
//
// netdev_set_defer_hard_irqs - set defer_hard_irqs for all NAPIs of a netdev
// @netdev: the net_device for which all NAPIs will have defer_hard_irqs set
// @defer: the defer_hard_irqs value to set
//
// napi_get_gro_flush_timeout - get the gro_flush_timeout
// @n: napi struct to get the gro_flush_timeout from
//
// Return: the per-NAPI value of the gro_flush_timeout field.
//
extern "C" {
    pub fn READ_ONCE(_arg: n->gro_flush_timeout) -> return;
}
//
// napi_set_gro_flush_timeout - set the gro_flush_timeout for a napi
// @n: napi struct to set the gro_flush_timeout
// @timeout: timeout value to set
//
// napi_set_gro_flush_timeout sets the per-NAPI gro_flush_timeout
//
// netdev_set_gro_flush_timeout - set gro_flush_timeout of a netdev's NAPIs
// @netdev: the net_device for which all NAPIs will have gro_flush_timeout set
// @timeout: the timeout value to set
//
// napi_get_irq_suspend_timeout - get the irq_suspend_timeout
// @n: napi struct to get the irq_suspend_timeout from
//
// Return: the per-NAPI value of the irq_suspend_timeout field.
//
extern "C" {
    pub fn READ_ONCE(_arg: n->irq_suspend_timeout) -> return;
}
//
// napi_set_irq_suspend_timeout - set the irq_suspend_timeout for a napi
// @n: napi struct to set the irq_suspend_timeout
// @timeout: timeout value to set
//
// napi_set_irq_suspend_timeout sets the per-NAPI irq_suspend_timeout
//
extern "C" {
    pub fn rps_cpumask_housekeeping(mask: *mut cpumask) -> c_int;
}

extern "C" {
    pub fn xdp_do_check_flushed(napi: *mut napi_struct);
}

// Best effort check that NAPI is not idle (can't be scheduled to run)
// uninitialized instance, can't race
// SCHED bit is set on disabled instances
extern "C" {
    pub fn kick_defer_list_purge(cpu: c_uint);
}
extern "C" {
    pub fn net_hwtstamp_validate(cfg: *const kernel_hwtstamp_config) -> c_int;
}
// Caller holds RTNL, netdev->lock or RCU
