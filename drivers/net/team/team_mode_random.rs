//! Automatically rewritten from C to Rust
//! Source: drivers/net/team/team_mode_random.c
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
// drivers/net/team/team_mode_random.c - Random mode for team
// Copyright (c) 2013 Jiri Pirko <jiri@resnulli.us>
//

#[no_mangle]
unsafe extern "C" fn rnd_transmit(team: *mut team, skb: *mut sk_buff) -> bool {
    static bool rnd_transmit(struct team *team, struct sk_buff *skb)
    {
    struct team_port *port;
    int port_index;
    port_index = get_random_u32_below(READ_ONCE(team.tx_en_port_count));
    port = team_get_port_by_tx_index_rcu(team, port_index);
    if (unlikely(!port))
    goto drop;
    port = team_get_first_port_txable_rcu(team, port);
    if (unlikely(!port))
    goto drop;
    if (team_dev_queue_xmit(team, port, skb))
    return false;
    return true;
    drop:
    dev_kfree_skb_any(skb);
    return false;
    }
    static const struct team_mode_ops rnd_mode_ops = {
    .transmit		= rnd_transmit,
    .port_enter		= team_modeop_port_enter,
    .port_change_dev_addr	= team_modeop_port_change_dev_addr,
    };
    static const struct team_mode rnd_mode = {
    .kind		= "random",
    .owner		= THIS_MODULE,
    .ops		= &rnd_mode_ops,
    .lag_tx_type	= NETDEV_LAG_TX_TYPE_RANDOM,
    };
#[no_mangle]
unsafe extern "C" fn rnd_init_module() -> int __init {
    static int __init rnd_init_module(void)
    {
    return team_mode_register(&rnd_mode);
    }
#[no_mangle]
unsafe extern "C" fn rnd_cleanup_module() -> void __exit {
    static void __exit rnd_cleanup_module(void)
    {
    team_mode_unregister(&rnd_mode);
    }
    module_init(rnd_init_module);
    module_exit(rnd_cleanup_module);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jiri Pirko <jiri@resnulli.us>");
    MODULE_DESCRIPTION("Random mode for team");
    MODULE_ALIAS_TEAM_MODE("random");
