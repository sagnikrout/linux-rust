//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/spi/mcp251xfd/mcp251xfd-timestamp.c
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
// mcp251xfd - Microchip MCP251xFD Family CAN controller driver
//
// Copyright (c) 2021, 2023 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

#[no_mangle]
unsafe extern "C" fn mcp251xfd_timestamp_raw_read(cc: *mut cyclecounter) -> u64 {
    static u64 mcp251xfd_timestamp_raw_read(struct cyclecounter *cc)
    {
    const struct mcp251xfd_priv *priv;
    let mut ts_raw: u32 = 0;
    int err;
    priv = container_of(cc, struct mcp251xfd_priv, cc);
    err = mcp251xfd_get_timestamp_raw(priv, &ts_raw);
    if (err)
    netdev_err(priv.ndev,
    "Error %d while reading timestamp. HW timestamps may be inaccurate.",
    err);
    return ts_raw;
    }
#[no_mangle]
unsafe extern "C" fn mcp251xfd_timestamp_work(work: *mut work_struct) {
    static void mcp251xfd_timestamp_work(struct work_struct *work)
    {
    struct delayed_work *delayed_work = to_delayed_work(work);
    struct mcp251xfd_priv *priv;
    priv = container_of(delayed_work, struct mcp251xfd_priv, timestamp);
    timecounter_read(&priv.tc);
    schedule_delayed_work(&priv.timestamp,
    MCP251XFD_TIMESTAMP_WORK_DELAY_SEC * HZ);
    }
#[no_mangle]
pub unsafe extern "C" fn mcp251xfd_timestamp_init(priv: *mut mcp251xfd_priv) {
    void mcp251xfd_timestamp_init(struct mcp251xfd_priv *priv)
    {
    struct cyclecounter *cc = &priv.cc;
    cc.read = mcp251xfd_timestamp_raw_read;
    cc.mask = CYCLECOUNTER_MASK(32);
    cc.shift = 1;
    cc.mult = clocksource_hz2mult(priv.can.clock.freq, cc.shift);
    INIT_DELAYED_WORK(&priv.timestamp, mcp251xfd_timestamp_work);
    }
#[no_mangle]
pub unsafe extern "C" fn mcp251xfd_timestamp_start(priv: *mut mcp251xfd_priv) {
    void mcp251xfd_timestamp_start(struct mcp251xfd_priv *priv)
    {
    timecounter_init(&priv.tc, &priv.cc, ktime_get_real_ns());
    schedule_delayed_work(&priv.timestamp,
    MCP251XFD_TIMESTAMP_WORK_DELAY_SEC * HZ);
    }
#[no_mangle]
pub unsafe extern "C" fn mcp251xfd_timestamp_stop(priv: *mut mcp251xfd_priv) {
    void mcp251xfd_timestamp_stop(struct mcp251xfd_priv *priv)
    {
    cancel_delayed_work_sync(&priv.timestamp);
    }
