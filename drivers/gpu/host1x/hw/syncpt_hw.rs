//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/hw/syncpt_hw.c
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
//
// Tegra host1x Syncpoints
//
// Copyright (c) 2010-2013, NVIDIA Corporation.
//

//
// Write the current syncpoint value back to hw.
//
#[no_mangle]
unsafe extern "C" fn syncpt_restore(sp: *mut host1x_syncpt) {
    static void syncpt_restore(struct host1x_syncpt *sp)
    {
    let mut min: u32 = host1x_syncpt_read_min(sp);
    struct host1x *host = sp.host;
    host1x_sync_writel(host, min, HOST1X_SYNC_SYNCPT(sp.id));
    }
//
// Write the current waitbase value back to hw.
//
#[no_mangle]
unsafe extern "C" fn syncpt_restore_wait_base(sp: *mut host1x_syncpt) {
    static void syncpt_restore_wait_base(struct host1x_syncpt *sp)
    {

    struct host1x *host = sp.host;
    host1x_sync_writel(host, sp.base_val,
    HOST1X_SYNC_SYNCPT_BASE(sp.id));

    }
//
// Read waitbase value from hw.
//
#[no_mangle]
unsafe extern "C" fn syncpt_read_wait_base(sp: *mut host1x_syncpt) {
    static void syncpt_read_wait_base(struct host1x_syncpt *sp)
    {

    struct host1x *host = sp.host;
    sp.base_val =
    host1x_sync_readl(host, HOST1X_SYNC_SYNCPT_BASE(sp.id));

    }
//
// Updates the last value read from hardware.
//
#[no_mangle]
unsafe extern "C" fn syncpt_load(sp: *mut host1x_syncpt) -> u32 {
    static u32 syncpt_load(struct host1x_syncpt *sp)
    {
    struct host1x *host = sp.host;
    u32 old, live;
// Loop in case there's a race writing to min_val
    do {
    old = host1x_syncpt_read_min(sp);
    live = host1x_sync_readl(host, HOST1X_SYNC_SYNCPT(sp.id));
    } while ((u32)atomic_cmpxchg(&sp.min_val, old, live) != old);
    if (!host1x_syncpt_check_max(sp, live))
    dev_err(host.dev, "%s failed: id=%u, min=%d, max=%d\n",
    __func__, sp.id, host1x_syncpt_read_min(sp),
    host1x_syncpt_read_max(sp));
    return live;
    }
//
// Write a cpu syncpoint increment to the hardware, without touching
// the cache.
//
#[no_mangle]
unsafe extern "C" fn syncpt_cpu_incr(sp: *mut host1x_syncpt) -> c_int {
    static int syncpt_cpu_incr(struct host1x_syncpt *sp)
    {
    struct host1x *host = sp.host;
    let mut reg_offset: u32 = sp.id / 32;
    if (!host1x_syncpt_client_managed(sp) &&
    host1x_syncpt_idle(sp))
    return -EINVAL;
    host1x_sync_writel(host, BIT(sp.id % 32),
    HOST1X_SYNC_SYNCPT_CPU_INCR(reg_offset));
    wmb();
    return 0;
    }
//
// syncpt_assign_to_channel() - Assign syncpoint to channel
// @sp: syncpoint
// @ch: channel
//
// On chips with the syncpoint protection feature (Tegra186+), assign @sp to
// @ch, preventing other channels from incrementing the syncpoints. If @ch is
// NULL, unassigns the syncpoint.
//
// On older chips, do nothing.
//
    static void syncpt_assign_to_channel(struct host1x_syncpt *sp,
    struct host1x_channel *ch)
    {

    struct host1x *host = sp.host;
    host1x_sync_writel(host,
    HOST1X_SYNC_SYNCPT_CH_APP_CH(ch ? ch.id : 0xff),
    HOST1X_SYNC_SYNCPT_CH_APP(sp.id));

    }
//
// syncpt_enable_protection() - Enable syncpoint protection
// @host: host1x instance
//
// On chips with the syncpoint protection feature (Tegra186+), enable this
// feature. On older chips, do nothing.
//
#[no_mangle]
unsafe extern "C" fn syncpt_enable_protection(host: *mut host1x) {
    static void syncpt_enable_protection(struct host1x *host)
    {

    if (!host.hv_regs)
    return;
    host1x_hypervisor_writel(host, HOST1X_HV_SYNCPT_PROT_EN_CH_EN,
    HOST1X_HV_SYNCPT_PROT_EN);

    }
    static const struct host1x_syncpt_ops host1x_syncpt_ops = {
    .restore = syncpt_restore,
    .restore_wait_base = syncpt_restore_wait_base,
    .load_wait_base = syncpt_read_wait_base,
    .load = syncpt_load,
    .cpu_incr = syncpt_cpu_incr,
    .assign_to_channel = syncpt_assign_to_channel,
    .enable_protection = syncpt_enable_protection,
    };
