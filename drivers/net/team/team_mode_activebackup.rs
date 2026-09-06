//! Automatically rewritten from C to Rust
//! Source: drivers/net/team/team_mode_activebackup.c
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
// drivers/net/team/team_mode_activebackup.c - Active-backup mode for team
// Copyright (c) 2011 Jiri Pirko <jpirko@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab_priv {
    pub active_port: *mut team_port __rcu,
    pub ap_opt_inst_info: *mut team_option_inst_info,
}

    static struct ab_priv *ab_priv(struct team *team)
    {
    return (struct ab_priv *) &team.mode_priv;
    }
    static rx_handler_result_t ab_receive(struct team *team, struct team_port *port,
    struct sk_buff *skb) {
    struct team_port *active_port;
    active_port = rcu_dereference(ab_priv(team).active_port);
    if (active_port != port)
    return RX_HANDLER_EXACT;
    return RX_HANDLER_ANOTHER;
    }
#[no_mangle]
unsafe extern "C" fn ab_transmit(team: *mut team, skb: *mut sk_buff) -> bool {
    static bool ab_transmit(struct team *team, struct sk_buff *skb)
    {
    struct team_port *active_port;
    active_port = rcu_dereference_bh(ab_priv(team).active_port);
    if (unlikely(!active_port))
    goto drop;
    if (team_dev_queue_xmit(team, active_port, skb))
    return false;
    return true;
    drop:
    dev_kfree_skb_any(skb);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ab_port_leave(team: *mut team, port: *mut team_port) {
    static void ab_port_leave(struct team *team, struct team_port *port)
    {
    if (ab_priv(team).active_port == port) {
    RCU_INIT_POINTER(ab_priv(team).active_port, core::ptr::null_mut());
    team_option_inst_set_change(ab_priv(team).ap_opt_inst_info);
    }
    }
    static void ab_active_port_init(struct team *team,
    struct team_option_inst_info *info)
    {
    ab_priv(team).ap_opt_inst_info = info;
    }
#[no_mangle]
unsafe extern "C" fn ab_active_port_get(team: *mut team, ctx: *mut team_gsetter_ctx) {
    static void ab_active_port_get(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct team_port *active_port;
    active_port = rtnl_dereference(ab_priv(team).active_port);
    if (active_port)
    ctx.data.u32_val = active_port.dev.ifindex;
    else
    ctx.data.u32_val = 0;
    }
#[no_mangle]
unsafe extern "C" fn ab_active_port_set(team: *mut team, ctx: *mut team_gsetter_ctx) -> c_int {
    static int ab_active_port_set(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct team_port *port;
    list_for_each_entry(port, &team.port_list, list) {
    if (port.dev.ifindex == ctx.data.u32_val) {
    rcu_assign_pointer(ab_priv(team).active_port, port);
    return 0;
    }
    }
    return -ENOENT;
    }
    static const struct team_option ab_options[] = {
    {
    .name = "activeport",
    .type = TEAM_OPTION_TYPE_U32,
    .init = ab_active_port_init,
    .getter = ab_active_port_get,
    .setter = ab_active_port_set,
    },
    };
#[no_mangle]
unsafe extern "C" fn ab_init(team: *mut team) -> c_int {
    static int ab_init(struct team *team)
    {
    return team_options_register(team, ab_options, ARRAY_SIZE(ab_options));
    }
#[no_mangle]
unsafe extern "C" fn ab_exit(team: *mut team) {
    static void ab_exit(struct team *team)
    {
    team_options_unregister(team, ab_options, ARRAY_SIZE(ab_options));
    }
    static const struct team_mode_ops ab_mode_ops = {
    .init			= ab_init,
    .exit			= ab_exit,
    .receive		= ab_receive,
    .transmit		= ab_transmit,
    .port_leave		= ab_port_leave,
    };
    static const struct team_mode ab_mode = {
    .kind		= "activebackup",
    .owner		= THIS_MODULE,
    .priv_size	= sizeof(struct ab_priv),
    .ops		= &ab_mode_ops,
    .lag_tx_type	= NETDEV_LAG_TX_TYPE_ACTIVEBACKUP,
    };
#[no_mangle]
unsafe extern "C" fn ab_init_module() -> int __init {
    static int __init ab_init_module(void)
    {
    return team_mode_register(&ab_mode);
    }
#[no_mangle]
unsafe extern "C" fn ab_cleanup_module() -> void __exit {
    static void __exit ab_cleanup_module(void)
    {
    team_mode_unregister(&ab_mode);
    }
    module_init(ab_init_module);
    module_exit(ab_cleanup_module);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jiri Pirko <jpirko@redhat.com>");
    MODULE_DESCRIPTION("Active-backup mode for team");
    MODULE_ALIAS_TEAM_MODE("activebackup");
