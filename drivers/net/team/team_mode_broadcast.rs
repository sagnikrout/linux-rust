//! Automatically rewritten from C to Rust
//! Source: drivers/net/team/team_mode_broadcast.c
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
// drivers/net/team/team_mode_broadcast.c - Broadcast mode for team
// Copyright (c) 2012 Jiri Pirko <jpirko@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn bc_transmit(team: *mut team, skb: *mut sk_buff) -> bool {
    static bool bc_transmit(struct team *team, struct sk_buff *skb)
    {
    struct team_port *cur;
    struct team_port *last = core::ptr::null_mut();
    struct sk_buff *skb2;
    bool ret;
    let mut sum_ret: bool = false;
    list_for_each_entry_rcu(cur, &team.port_list, list) {
    if (team_port_txable(cur)) {
    if (last) {
    skb2 = skb_clone(skb, GFP_ATOMIC);
    if (skb2) {
    ret = !team_dev_queue_xmit(team, last,
    skb2);
    if (!sum_ret)
    sum_ret = ret;
    }
    }
    last = cur;
    }
    }
    if (last) {
    ret = !team_dev_queue_xmit(team, last, skb);
    if (!sum_ret)
    sum_ret = ret;
    }
    return sum_ret;
    }
    static const struct team_mode_ops bc_mode_ops = {
    .transmit		= bc_transmit,
    .port_enter		= team_modeop_port_enter,
    .port_change_dev_addr	= team_modeop_port_change_dev_addr,
    };
    static const struct team_mode bc_mode = {
    .kind		= "broadcast",
    .owner		= THIS_MODULE,
    .ops		= &bc_mode_ops,
    .lag_tx_type	= NETDEV_LAG_TX_TYPE_BROADCAST,
    };
#[no_mangle]
unsafe extern "C" fn bc_init_module() -> int __init {
    static int __init bc_init_module(void)
    {
    return team_mode_register(&bc_mode);
    }
#[no_mangle]
unsafe extern "C" fn bc_cleanup_module() -> void __exit {
    static void __exit bc_cleanup_module(void)
    {
    team_mode_unregister(&bc_mode);
    }
    module_init(bc_init_module);
    module_exit(bc_cleanup_module);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jiri Pirko <jpirko@redhat.com>");
    MODULE_DESCRIPTION("Broadcast mode for team");
    MODULE_ALIAS_TEAM_MODE("broadcast");
