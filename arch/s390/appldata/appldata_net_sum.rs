//! Automatically rewritten from C to Rust
//! Source: arch/s390/appldata/appldata_net_sum.c
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
// Data gathering module for Linux-VM Monitor Stream, Stage 1.
// Collects accumulated network statistics (Packets received/transmitted,
// dropped, errors, ...).
//
// Copyright IBM Corp. 2003, 2006
//
// Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

//
// Network data
//
// This is accessed as binary data by z/VM. If changes to it can't be avoided,
// the structure version (product ID, see appldata_base.c) needs to be changed
// as well and all documentation and z/VM applications using it must be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_net_sum_data {
    pub timestamp: u64,
    pub /: *mut *mut u32 sync_count_1; / after VM collected the record data,,
    pub the: *mut *mut u32 sync_count_2; / sync_count_1 and sync_count_2 should be,
    same. If not, the record has been updated on
    the Linux side while VM was collecting the
    (possibly corrupt) data */
    pub /: *mut *mut u32 nr_interfaces; / nr. of network interfaces being monitored,
    pub /: *mut *mut u32 padding; / next value is 64-bit aligned, so these,
// 4 byte would be padded out by compiler
    pub /: *mut *mut u64 rx_packets; / total packets received,
    pub /: *mut *mut u64 tx_packets; / total packets transmitted,
    pub /: *mut *mut u64 rx_bytes; / total bytes received,
    pub /: *mut *mut u64 tx_bytes; / total bytes transmitted,
    pub /: *mut *mut u64 rx_errors; / bad packets received,
    pub /: *mut *mut u64 tx_errors; / packet transmit problems,
    pub /: *mut *mut u64 rx_dropped; / no space in linux buffers,
    pub /: *mut *mut u64 tx_dropped; / no space available in linux,
    pub /: *mut *mut u64 collisions; / collisions while transmitting,
    pub __packed: },
//
// appldata_get_net_sum_data()
//
// gather accumulated network statistics
//
#[no_mangle]
unsafe extern "C" fn appldata_get_net_sum_data(data: *mut c_void) {
    static void appldata_get_net_sum_data(void *data)
    {
    pub i: c_int,
    pub net_data: *mut appldata_net_sum_data,
    pub dev: *mut net_device,
    unsigned long rx_packets, tx_packets, rx_bytes, tx_bytes, rx_errors,
    pub collisions: tx_errors, rx_dropped, tx_dropped,,
    pub data: net_data =,
    pub 0: i =,
    pub 0: rx_packets =,
    pub 0: tx_packets =,
    pub 0: rx_bytes =,
    pub 0: tx_bytes =,
    pub 0: rx_errors =,
    pub 0: tx_errors =,
    pub 0: rx_dropped =,
    pub 0: tx_dropped =,
    pub 0: collisions =,
    for_each_netdev_rcu(&init_net, dev) {
    pub stats: *const rtnl_link_stats64,
    pub temp: rtnl_link_stats64,
    pub &temp): stats = dev_get_stats(dev,,
    pub stats->rx_packets: rx_packets +=,
    pub stats->tx_packets: tx_packets +=,
    pub stats->rx_bytes: rx_bytes +=,
    pub stats->tx_bytes: tx_bytes +=,
    pub stats->rx_errors: rx_errors +=,
    pub stats->tx_errors: tx_errors +=,
    pub stats->rx_dropped: rx_dropped +=,
    pub stats->tx_dropped: tx_dropped +=,
    pub stats->collisions: collisions +=,
    }
    pub i: net_data->nr_interfaces =,
    pub rx_packets: net_data->rx_packets =,
    pub tx_packets: net_data->tx_packets =,
    pub rx_bytes: net_data->rx_bytes =,
    pub tx_bytes: net_data->tx_bytes =,
    pub rx_errors: net_data->rx_errors =,
    pub tx_errors: net_data->tx_errors =,
    pub rx_dropped: net_data->rx_dropped =,
    pub tx_dropped: net_data->tx_dropped =,
    pub collisions: net_data->collisions =,
    pub get_tod_clock(): net_data->timestamp =,
    }
    static struct appldata_ops ops = {
    .name	   = "net_sum",
    .record_nr = APPLDATA_RECORD_NET_SUM_ID,
    .size	   = sizeof(struct appldata_net_sum_data),
    .callback  = &appldata_get_net_sum_data,
    .owner     = THIS_MODULE,
    .mod_lvl   = {0xF0, 0xF0},		/* EBCDIC "00" */
}

//
// appldata_net_init()
//
// init data, register ops
//
#[no_mangle]
unsafe extern "C" fn appldata_net_init() -> int __init {
    static int __init appldata_net_init(void)
    {
    int ret;
    ops.data = kzalloc_obj(struct appldata_net_sum_data);
    if (!ops.data)
    return -ENOMEM;
    ret = appldata_register_ops(&ops);
    if (ret)
    kfree(ops.data);
    return ret;
    }
//
// appldata_net_exit()
//
// unregister ops
//
#[no_mangle]
unsafe extern "C" fn appldata_net_exit() -> void __exit {
    static void __exit appldata_net_exit(void)
    {
    appldata_unregister_ops(&ops);
    kfree(ops.data);
    }
    module_init(appldata_net_init);
    module_exit(appldata_net_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Gerald Schaefer");
    MODULE_DESCRIPTION("Linux-VM Monitor Stream, accumulated network statistics");
