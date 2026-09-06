//! Automatically rewritten from C to Rust
//! Source: net/wireless/wext-proc.c
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
// This file implement the Wireless Extensions proc API.
//
// Authors :	Jean Tourrilhes - HPL - <jt@hpl.hp.com>
// Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
//
// The /proc/net/wireless file is a human readable user-space interface
// exporting various wireless specific statistics from the wireless devices.
// This is the most popular part of the Wireless Extensions ;-)
//
// This interface is a pure clone of /proc/net/dev (in net/core/dev.c).
// The content of the file is basically the content of "struct iw_statistics".
//

    static void wireless_seq_printf_stats(struct seq_file *seq,
    struct net_device *dev)
    {
// Get stats from the driver
    struct iw_statistics *stats = get_wireless_stats(dev);
    let mut nullstats: static struct iw_statistics = {};
// show device if it's wireless regardless of current stats
    if (!stats) {

    if (dev.wireless_handlers)
    stats = &nullstats;

    if (dev.ieee80211_ptr)
    stats = &nullstats;

    }
    if (stats) {
    seq_printf(seq, "%6s: %04x  %3d%c  %3d%c  %3d%c  %6d %6d %6d "
    "%6d %6d   %6d\n",
    dev.name, stats.status, stats.qual.qual,
    stats.qual.updated & IW_QUAL_QUAL_UPDATED
    ? '.' : ' ',
    ((__s32) stats.qual.level) -
    ((stats.qual.updated & IW_QUAL_DBM) ? 0x100 : 0),
    stats.qual.updated & IW_QUAL_LEVEL_UPDATED
    ? '.' : ' ',
    ((__s32) stats.qual.noise) -
    ((stats.qual.updated & IW_QUAL_DBM) ? 0x100 : 0),
    stats.qual.updated & IW_QUAL_NOISE_UPDATED
    ? '.' : ' ',
    stats.discard.nwid, stats.discard.code,
    stats.discard.fragment, stats.discard.retries,
    stats.discard.misc, stats.miss.beacon);
    if (stats != &nullstats)
    stats.qual.updated &= ~IW_QUAL_ALL_UPDATED;
    }
    }
// ----------------------------------------------------------------
//
// Print info for /proc/net/wireless (print all entries)
//
#[no_mangle]
unsafe extern "C" fn wireless_dev_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int wireless_dev_seq_show(struct seq_file *seq, void *v)
    {
    might_sleep();
    if (v == SEQ_START_TOKEN)
    seq_printf(seq, "Inter-| sta-|   Quality        |   Discarded "
    "packets               | Missed | WE\n"
    " face | tus | link level noise |  nwid  "
    "crypt   frag  retry   misc | beacon | %d\n",
    WIRELESS_EXT);
    else
    wireless_seq_printf_stats(seq, v);
    return 0;
    }
    static void *wireless_dev_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct net *net = seq_file_net(seq);
    loff_t off;
    struct net_device *dev;
    rtnl_lock();
    if (!*pos)
    return SEQ_START_TOKEN;
    off = 1;
    for_each_netdev(net, dev)
    if (off++ == *pos)
    return dev;
    return core::ptr::null_mut();
    }
    static void *wireless_dev_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct net *net = seq_file_net(seq);
    ++*pos;
    return v == SEQ_START_TOKEN ?
    first_net_device(net) : next_net_device(v);
    }
#[no_mangle]
unsafe extern "C" fn wireless_dev_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void wireless_dev_seq_stop(struct seq_file *seq, void *v)
    {
    rtnl_unlock();
    }
    static const struct seq_operations wireless_seq_ops = {
    .start = wireless_dev_seq_start,
    .next  = wireless_dev_seq_next,
    .stop  = wireless_dev_seq_stop,
    .show  = wireless_dev_seq_show,
    };
#[no_mangle]
pub unsafe extern "C" fn wext_proc_init(net: *mut net) -> int __net_init {
    int __net_init wext_proc_init(struct net *net)
    {
// Create /proc/net/wireless entry
    if (!proc_create_net("wireless", 0444, net.proc_net,
    &wireless_seq_ops, sizeof(struct seq_net_private)))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn wext_proc_exit(net: *mut net) -> void __net_exit {
    void __net_exit wext_proc_exit(struct net *net)
    {
    remove_proc_entry("wireless", net.proc_net);
    }
