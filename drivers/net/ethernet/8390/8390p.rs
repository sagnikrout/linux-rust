//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/8390/8390p.c
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


// SPDX-License-Identifier: GPL-2.0-only
// 8390 core for ISA devices needing bus delays
    static const char version[] =
    "8390p.c:v1.10cvs 9/23/94 Donald Becker (becker@cesdis.gsfc.nasa.gov)\n";

#[no_mangle]
pub unsafe extern "C" fn eip_open(dev: *mut net_device) -> c_int {
    int eip_open(struct net_device *dev)
    {
    return __ei_open(dev);
    }
    EXPORT_SYMBOL(eip_open);
#[no_mangle]
pub unsafe extern "C" fn eip_close(dev: *mut net_device) -> c_int {
    int eip_close(struct net_device *dev)
    {
    return __ei_close(dev);
    }
    EXPORT_SYMBOL(eip_close);
#[no_mangle]
pub unsafe extern "C" fn eip_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    netdev_tx_t eip_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    return __ei_start_xmit(skb, dev);
    }
    EXPORT_SYMBOL(eip_start_xmit);
    struct net_device_stats *eip_get_stats(struct net_device *dev)
    {
    return __ei_get_stats(dev);
    }
    EXPORT_SYMBOL(eip_get_stats);
#[no_mangle]
pub unsafe extern "C" fn eip_set_multicast_list(dev: *mut net_device) {
    void eip_set_multicast_list(struct net_device *dev)
    {
    __ei_set_multicast_list(dev);
    }
    EXPORT_SYMBOL(eip_set_multicast_list);
#[no_mangle]
pub unsafe extern "C" fn eip_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    void eip_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    __ei_tx_timeout(dev, txqueue);
    }
    EXPORT_SYMBOL(eip_tx_timeout);
#[no_mangle]
pub unsafe extern "C" fn eip_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    irqreturn_t eip_interrupt(int irq, void *dev_id)
    {
    return __ei_interrupt(irq, dev_id);
    }
    EXPORT_SYMBOL(eip_interrupt);

#[no_mangle]
pub unsafe extern "C" fn eip_poll(dev: *mut net_device) {
    void eip_poll(struct net_device *dev)
    {
    __ei_poll(dev);
    }
    EXPORT_SYMBOL(eip_poll);

    const struct net_device_ops eip_netdev_ops = {
    .ndo_open		= eip_open,
    .ndo_stop		= eip_close,
    .ndo_start_xmit		= eip_start_xmit,
    .ndo_tx_timeout		= eip_tx_timeout,
    .ndo_get_stats		= eip_get_stats,
    .ndo_set_rx_mode	= eip_set_multicast_list,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address 	= eth_mac_addr,

    .ndo_poll_controller	= eip_poll,

    };
    EXPORT_SYMBOL(eip_netdev_ops);
    struct net_device *__alloc_eip_netdev(int size)
    {
    struct net_device *dev = ____alloc_ei_netdev(size);
    if (dev)
    dev.netdev_ops = &eip_netdev_ops;
    return dev;
    }
    EXPORT_SYMBOL(__alloc_eip_netdev);
#[no_mangle]
pub unsafe extern "C" fn NS8390p_init(dev: *mut net_device, startp: c_int) {
    void NS8390p_init(struct net_device *dev, int startp)
    {
    __NS8390_init(dev, startp);
    }
    EXPORT_SYMBOL(NS8390p_init);
    MODULE_DESCRIPTION("National Semiconductor 8390 core for ISA driver");
    MODULE_LICENSE("GPL");
