//! Automatically rewritten from C to Rust
//! Source: drivers/net/team/team_mode_roundrobin.c
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
// drivers/net/team/team_mode_roundrobin.c - Round-robin mode for team
// Copyright (c) 2011 Jiri Pirko <jpirko@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rr_priv {
    pub sent_packets: c_uint,
}

    static struct rr_priv *rr_priv(struct team *team)
    {
    return (struct rr_priv *) &team.mode_priv;
    }
#[no_mangle]
unsafe extern "C" fn rr_transmit(team: *mut team, skb: *mut sk_buff) -> bool {
    static bool rr_transmit(struct team *team, struct sk_buff *skb)
    {
    struct team_port *port;
    int port_index;
    port_index = team_num_to_port_index(team,
    rr_priv(team).sent_packets++);
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
    static const struct team_mode_ops rr_mode_ops = {
    .transmit		= rr_transmit,
    .port_enter		= team_modeop_port_enter,
    .port_change_dev_addr	= team_modeop_port_change_dev_addr,
    };
    static const struct team_mode rr_mode = {
    .kind		= "roundrobin",
    .owner		= THIS_MODULE,
    .priv_size	= sizeof(struct rr_priv),
    .ops		= &rr_mode_ops,
    .lag_tx_type	= NETDEV_LAG_TX_TYPE_ROUNDROBIN,
    };
#[no_mangle]
unsafe extern "C" fn rr_init_module() -> int __init {
    static int __init rr_init_module(void)
    {
    return team_mode_register(&rr_mode);
    }
#[no_mangle]
unsafe extern "C" fn rr_cleanup_module() -> void __exit {
    static void __exit rr_cleanup_module(void)
    {
    team_mode_unregister(&rr_mode);
    }
    module_init(rr_init_module);
    module_exit(rr_cleanup_module);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jiri Pirko <jpirko@redhat.com>");
    MODULE_DESCRIPTION("Round-robin mode for team");
    MODULE_ALIAS_TEAM_MODE("roundrobin");
