//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/ti/icssg/icssg_queues.c
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
// ICSSG Buffer queue helpers
//
// Copyright (C) 2021 Texas Instruments Incorporated - https://www.ti.com
//

pub const ICSSG_QUEUES_MAX: c_int = 64;
pub const ICSSG_QUEUE_OFFSET: c_uint = 0xd00;
pub const ICSSG_QUEUE_PEEK_OFFSET: c_uint = 0xe00;
pub const ICSSG_QUEUE_CNT_OFFSET: c_uint = 0xe40;
pub const ICSSG_QUEUE_RESET_OFFSET: c_uint = 0xf40;
#[no_mangle]
pub unsafe extern "C" fn icssg_queue_pop(prueth: *mut prueth, queue: u8) -> c_int {
    int icssg_queue_pop(struct prueth *prueth, u8 queue)
    {
    u32 val, cnt;
    if (queue >= ICSSG_QUEUES_MAX)
    return -EINVAL;
    regmap_read(prueth.miig_rt, ICSSG_QUEUE_CNT_OFFSET + 4 * queue, &cnt);
    if (!cnt)
    return -EINVAL;
    regmap_read(prueth.miig_rt, ICSSG_QUEUE_OFFSET + 4 * queue, &val);
    return val;
    }
    EXPORT_SYMBOL_GPL(icssg_queue_pop);
#[no_mangle]
pub unsafe extern "C" fn icssg_queue_push(prueth: *mut prueth, queue: c_int, addr: u16) {
    void icssg_queue_push(struct prueth *prueth, int queue, u16 addr)
    {
    if (queue >= ICSSG_QUEUES_MAX)
    return;
    regmap_write(prueth.miig_rt, ICSSG_QUEUE_OFFSET + 4 * queue, addr);
    }
    EXPORT_SYMBOL_GPL(icssg_queue_push);
#[no_mangle]
pub unsafe extern "C" fn icssg_queue_level(prueth: *mut prueth, queue: c_int) -> u32 {
    u32 icssg_queue_level(struct prueth *prueth, int queue)
    {
    u32 reg;
    if (queue >= ICSSG_QUEUES_MAX)
    return 0;
    regmap_read(prueth.miig_rt, ICSSG_QUEUE_CNT_OFFSET + 4 * queue, &reg);
    return reg;
    }
