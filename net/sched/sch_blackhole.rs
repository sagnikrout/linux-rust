//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_blackhole.c
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
// net/sched/sch_blackhole.c	Black hole queue
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//
// Note: Quantum tunneling is not supported.
//

    static int blackhole_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    qdisc_drop(skb, sch, to_free);
    return NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    }
    static struct sk_buff *blackhole_dequeue(struct Qdisc *sch)
    {
    return core::ptr::null_mut();
    }
    static struct Qdisc_ops blackhole_qdisc_ops __read_mostly = {
    .id		= "blackhole",
    .priv_size	= 0,
    .enqueue	= blackhole_enqueue,
    .dequeue	= blackhole_dequeue,
    .peek		= blackhole_dequeue,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn blackhole_init() -> int __init {
    static int __init blackhole_init(void)
    {
    return register_qdisc(&blackhole_qdisc_ops);
    }
    device_initcall(blackhole_init)
