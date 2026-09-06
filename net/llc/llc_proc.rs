//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_proc.c
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
// proc_llc.c - proc interface for LLC
//
// Copyright (c) 2001 by Jay Schulist <jschlst@samba.org>
// 2002-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

#[no_mangle]
unsafe extern "C" fn llc_ui_format_mac(seq: *mut seq_file, addr: *const u8) {
    static void llc_ui_format_mac(struct seq_file *seq, const u8 *addr)
    {
    seq_printf(seq, "%pM", addr);
    }
    static struct sock *llc_get_sk_idx(loff_t pos)
    {
    struct llc_sap *sap;
    struct sock *sk = core::ptr::null_mut();
    int i;
    list_for_each_entry_rcu(sap, &llc_sap_list, node) {
    spin_lock_bh(&sap.sk_lock);
    for (i = 0; i < LLC_SK_LADDR_HASH_ENTRIES; i++) {
    struct hlist_nulls_head *head = &sap.sk_laddr_hash[i];
    struct hlist_nulls_node *node;
    sk_nulls_for_each(sk, node, head) {
    if (!pos)
    goto found; /* keep the lock */
    --pos;
    }
    }
    spin_unlock_bh(&sap.sk_lock);
    }
    sk = core::ptr::null_mut();
    found:
    return sk;
    }
    static void *llc_seq_start(struct seq_file *seq, loff_t *pos) __acquires(RCU)
    {
    let mut l: loff_t = *pos;
    rcu_read_lock_bh();
    return l ? llc_get_sk_idx(--l) : SEQ_START_TOKEN;
    }
    static struct sock *laddr_hash_next(struct llc_sap *sap, int bucket)
    {
    struct hlist_nulls_node *node;
    struct sock *sk = core::ptr::null_mut();
    while (++bucket < LLC_SK_LADDR_HASH_ENTRIES)
    sk_nulls_for_each(sk, node, &sap.sk_laddr_hash[bucket])
    goto out;
    out:
    return sk;
    }
    static void *llc_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct sock* sk, *next;
    struct llc_sock *llc;
    struct llc_sap *sap;
    ++*pos;
    if (v == SEQ_START_TOKEN) {
    sk = llc_get_sk_idx(0);
    goto out;
    }
    sk = v;
    next = sk_nulls_next(sk);
    if (next) {
    sk = next;
    goto out;
    }
    llc = llc_sk(sk);
    sap = llc.sap;
    sk = laddr_hash_next(sap, llc_sk_laddr_hashfn(sap, &llc.laddr));
    if (sk)
    goto out;
    spin_unlock_bh(&sap.sk_lock);
    list_for_each_entry_continue_rcu(sap, &llc_sap_list, node) {
    spin_lock_bh(&sap.sk_lock);
    sk = laddr_hash_next(sap, -1);
    if (sk)
    break; /* keep the lock */
    spin_unlock_bh(&sap.sk_lock);
    }
    out:
    return sk;
    }
#[no_mangle]
unsafe extern "C" fn llc_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void llc_seq_stop(struct seq_file *seq, void *v)
    {
    if (v && v != SEQ_START_TOKEN) {
    struct sock *sk = v;
    struct llc_sock *llc = llc_sk(sk);
    struct llc_sap *sap = llc.sap;
    spin_unlock_bh(&sap.sk_lock);
    }
    rcu_read_unlock_bh();
    }
#[no_mangle]
unsafe extern "C" fn llc_seq_socket_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int llc_seq_socket_show(struct seq_file *seq, void *v)
    {
    struct sock* sk;
    struct llc_sock *llc;
    if (v == SEQ_START_TOKEN) {
    seq_puts(seq, "SKt Mc local_mac_sap        remote_mac_sap   "
    "    tx_queue rx_queue st uid link\n");
    goto out;
    }
    sk = v;
    llc = llc_sk(sk);
// FIXME: check if the address is multicast
    seq_printf(seq, "%2X  %2X ", sk.sk_type, 0);
    if (llc.dev)
    llc_ui_format_mac(seq, llc.dev.dev_addr);
    else {
    u8 addr[6] = {0,0,0,0,0,0};
    llc_ui_format_mac(seq, addr);
    }
    seq_printf(seq, "@%02X ", llc.sap.laddr.lsap);
    llc_ui_format_mac(seq, llc.daddr.mac);
    seq_printf(seq, "@%02X %8d %8d %2d %3u %4d\n", llc.daddr.lsap,
    sk_wmem_alloc_get(sk),
    sk_rmem_alloc_get(sk) - llc.copied_seq,
    sk.sk_state,
    from_kuid_munged(seq_user_ns(seq), sk_uid(sk)),
    llc.link);
    out:
    return 0;
    }
    static const char *const llc_conn_state_names[] = {
    [LLC_CONN_STATE_ADM] =        "adm",
    [LLC_CONN_STATE_SETUP] =      "setup",
    [LLC_CONN_STATE_NORMAL] =     "normal",
    [LLC_CONN_STATE_BUSY] =       "busy",
    [LLC_CONN_STATE_REJ] =        "rej",
    [LLC_CONN_STATE_AWAIT] =      "await",
    [LLC_CONN_STATE_AWAIT_BUSY] = "await_busy",
    [LLC_CONN_STATE_AWAIT_REJ] =  "await_rej",
    [LLC_CONN_STATE_D_CONN]	=     "d_conn",
    [LLC_CONN_STATE_RESET] =      "reset",
    [LLC_CONN_STATE_ERROR] =      "error",
    [LLC_CONN_STATE_TEMP] =       "temp",
    };
#[no_mangle]
unsafe extern "C" fn llc_seq_core_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int llc_seq_core_show(struct seq_file *seq, void *v)
    {
    struct sock* sk;
    struct llc_sock *llc;
    if (v == SEQ_START_TOKEN) {
    seq_puts(seq, "Connection list:\n"
    "dsap state      retr txw rxw pf ff sf df rs cs "
    "tack tpfc trs tbs blog busr\n");
    goto out;
    }
    sk = v;
    llc = llc_sk(sk);
    seq_printf(seq, " %02X  %-10s %3d  %3d %3d %2d %2d %2d %2d %2d %2d "
    "%4d %4d %3d %3d %4d %4d\n",
    llc.daddr.lsap, llc_conn_state_names[llc.state],
    llc.retry_count, llc.k, llc.rw, llc.p_flag, llc.f_flag,
    llc.s_flag, llc.data_flag, llc.remote_busy_flag,
    llc.cause_flag, timer_pending(&llc.ack_timer.timer),
    timer_pending(&llc.pf_cycle_timer.timer),
    timer_pending(&llc.rej_sent_timer.timer),
    timer_pending(&llc.busy_state_timer.timer),
    !!sk.sk_backlog.tail, sock_owned_by_user_nocheck(sk));
    out:
    return 0;
    }
    static const struct seq_operations llc_seq_socket_ops = {
    .start  = llc_seq_start,
    .next   = llc_seq_next,
    .stop   = llc_seq_stop,
    .show   = llc_seq_socket_show,
    };
    static const struct seq_operations llc_seq_core_ops = {
    .start  = llc_seq_start,
    .next   = llc_seq_next,
    .stop   = llc_seq_stop,
    .show   = llc_seq_core_show,
    };
    static struct proc_dir_entry *llc_proc_dir;
#[no_mangle]
pub unsafe extern "C" fn llc_proc_init() -> int __init {
    int __init llc_proc_init(void)
    {
    let mut rc: c_int = -ENOMEM;
    struct proc_dir_entry *p;
    llc_proc_dir = proc_mkdir("llc", init_net.proc_net);
    if (!llc_proc_dir)
    goto out;
    p = proc_create_seq("socket", 0444, llc_proc_dir, &llc_seq_socket_ops);
    if (!p)
    goto out_socket;
    p = proc_create_seq("core", 0444, llc_proc_dir, &llc_seq_core_ops);
    if (!p)
    goto out_core;
    rc = 0;
    out:
    return rc;
    out_core:
    remove_proc_entry("socket", llc_proc_dir);
    out_socket:
    remove_proc_entry("llc", init_net.proc_net);
    goto out;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_proc_exit() {
    void llc_proc_exit(void)
    {
    remove_proc_entry("socket", llc_proc_dir);
    remove_proc_entry("core", llc_proc_dir);
    remove_proc_entry("llc", init_net.proc_net);
    }
