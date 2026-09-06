//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/ipw2x00/libipw_spy.c
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


//
// This file implement the Wireless Extensions spy API.
//
// Authors :	Jean Tourrilhes - HPL - <jt@hpl.hp.com>
// Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
//
// (As all part of the Linux kernel, this file is GPL)
//

    static struct iw_spy_data *get_spydata(struct net_device *dev)
    {
    struct libipw_device *ieee = netdev_priv(dev);
    if (ieee.spy_enabled)
    return &ieee.spy_data;
    return core::ptr::null_mut();
    }
    int ipw_wx_set_spy(struct net_device *		dev,
    struct iw_request_info *	info,
    union iwreq_data *		wrqu,
    char *			extra)
    {
    let mut spydata: *mut iw_spy_data = get_spydata(dev);
    let mut address: *mut sockaddr = (struct sockaddr *) extra;
// Make sure driver is not buggy or using the old API
    if (!spydata)
    return -EOPNOTSUPP;
// Disable spy collection while we copy the addresses.
// While we copy addresses, any call to libipw_spy_update()
// will NOP. This is OK, as anyway the addresses are changing.
    spydata.spy_number = 0;
// We want to operate without locking, because libipw_spy_update()
// most likely will happen in the interrupt handler, and therefore
// have its own locking constraints and needs performance.
// The rtnl_lock() make sure we don't race with the other iw_handlers.
// This make sure libipw_spy_update() "see" that the spy list
// is temporarily disabled.
    smp_wmb();
// Are there are addresses to copy?
    if (wrqu.data.length > 0) {
    int i;
// Copy addresses
    for (i = 0; i < wrqu.data.length; i++)
    memcpy(spydata.spy_address[i], address[i].sa_data,
    ETH_ALEN);
// Reset stats
    memset(spydata.spy_stat, 0,
    sizeof(struct iw_quality) * IW_MAX_SPY);
    }
// Make sure above is updated before re-enabling
    smp_wmb();
// Enable addresses
    spydata.spy_number = wrqu.data.length;
    return 0;
    }
    EXPORT_SYMBOL(ipw_wx_set_spy);
    int ipw_wx_get_spy(struct net_device *		dev,
    struct iw_request_info *	info,
    union iwreq_data *		wrqu,
    char *			extra)
    {
    let mut spydata: *mut iw_spy_data = get_spydata(dev);
    let mut address: *mut sockaddr = (struct sockaddr *) extra;
    int			i;
// Make sure driver is not buggy or using the old API
    if (!spydata)
    return -EOPNOTSUPP;
    wrqu.data.length = spydata.spy_number;
// Copy addresses.
    for (i = 0; i < spydata.spy_number; i++) 	{
    memcpy(address[i].sa_data, spydata.spy_address[i], ETH_ALEN);
    address[i].sa_family = AF_UNIX;
    }
// Copy stats to the user buffer (just after).
    if (spydata.spy_number > 0)
    memcpy(extra  + (sizeof(struct sockaddr) *spydata.spy_number),
    spydata.spy_stat,
    sizeof(struct iw_quality) * spydata.spy_number);
// Reset updated flags.
    for (i = 0; i < spydata.spy_number; i++)
    spydata.spy_stat[i].updated &= ~IW_QUAL_ALL_UPDATED;
    return 0;
    }
    EXPORT_SYMBOL(ipw_wx_get_spy);
// ------------------------------------------------------------------
//
// Standard Wireless Handler : set spy threshold
//
    int ipw_wx_set_thrspy(struct net_device *	dev,
    struct iw_request_info *	info,
    union iwreq_data *	wrqu,
    char *			extra)
    {
    let mut spydata: *mut iw_spy_data = get_spydata(dev);
    let mut threshold: *mut iw_thrspy = (struct iw_thrspy *) extra;
// Make sure driver is not buggy or using the old API
    if (!spydata)
    return -EOPNOTSUPP;
// Just do it
    spydata.spy_thr_low = threshold.low;
    spydata.spy_thr_high = threshold.high;
// Clear flag
    memset(spydata.spy_thr_under, '\0', sizeof(spydata.spy_thr_under));
    return 0;
    }
    EXPORT_SYMBOL(ipw_wx_set_thrspy);
// ------------------------------------------------------------------
//
// Standard Wireless Handler : get spy threshold
//
    int ipw_wx_get_thrspy(struct net_device *	dev,
    struct iw_request_info *	info,
    union iwreq_data *	wrqu,
    char *			extra)
    {
    let mut spydata: *mut iw_spy_data = get_spydata(dev);
    let mut threshold: *mut iw_thrspy = (struct iw_thrspy *) extra;
// Make sure driver is not buggy or using the old API
    if (!spydata)
    return -EOPNOTSUPP;
// Just do it
    threshold.low = spydata.spy_thr_low;
    threshold.high = spydata.spy_thr_high;
    return 0;
    }
    EXPORT_SYMBOL(ipw_wx_get_thrspy);
// ------------------------------------------------------------------
//
// Prepare and send a Spy Threshold event
//
    static void iw_send_thrspy_event(struct net_device *	dev,
    struct iw_spy_data *	spydata,
    unsigned char *	address,
    struct iw_quality *	wstats)
    {
    union iwreq_data	wrqu;
    struct iw_thrspy	threshold;
// Init
    wrqu.data.length = 1;
    wrqu.data.flags = 0;
// Copy address
    memcpy(threshold.addr.sa_data, address, ETH_ALEN);
    threshold.addr.sa_family = ARPHRD_ETHER;
// Copy stats
    threshold.qual = *wstats;
// Copy also thresholds
    threshold.low = spydata.spy_thr_low;
    threshold.high = spydata.spy_thr_high;
// Send event to user space
    wireless_send_event(dev, SIOCGIWTHRSPY, &wrqu, (char *) &threshold);
    }
// ----------------------------------------------------------------
//
// Call for the driver to update the spy data.
// For now, the spy data is a simple array. As the size of the array is
// small, this is good enough. If we wanted to support larger number of
// spy addresses, we should use something more efficient...
//
    void libipw_spy_update(struct net_device *	dev,
    unsigned char *		address,
    struct iw_quality *	wstats)
    {
    let mut spydata: *mut iw_spy_data = get_spydata(dev);
    int			i;
    let mut match: c_int = -1;
// Make sure driver is not buggy or using the old API
    if (!spydata)
    return;
// Update all records that match
    for (i = 0; i < spydata.spy_number; i++)
    if (ether_addr_equal(address, spydata.spy_address[i])) {
    memcpy(&(spydata.spy_stat[i]), wstats,
    sizeof(struct iw_quality));
    match = i;
    }
// Generate an event if we cross the spy threshold.
// To avoid event storms, we have a simple hysteresis : we generate
// event only when we go under the low threshold or above the
// high threshold.
    if (match >= 0) {
    if (spydata.spy_thr_under[match]) {
    if (wstats.level > spydata.spy_thr_high.level) {
    spydata.spy_thr_under[match] = 0;
    iw_send_thrspy_event(dev, spydata,
    address, wstats);
    }
    } else {
    if (wstats.level < spydata.spy_thr_low.level) {
    spydata.spy_thr_under[match] = 1;
    iw_send_thrspy_event(dev, spydata,
    address, wstats);
    }
    }
    }
    }
