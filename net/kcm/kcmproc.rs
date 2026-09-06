//! Automatically rewritten from C to Rust
//! Source: net/kcm/kcmproc.c
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

    static struct kcm_mux *kcm_get_first(struct seq_file *seq)
    {
    struct net *net = seq_file_net(seq);
    struct kcm_net *knet = net_generic(net, kcm_net_id);
    return list_first_or_null_rcu(&knet.mux_list,
    struct kcm_mux, kcm_mux_list);
    }
    static struct kcm_mux *kcm_get_next(struct kcm_mux *mux)
    {
    struct kcm_net *knet = mux.knet;
    return list_next_or_null_rcu(&knet.mux_list, &mux.kcm_mux_list,
    struct kcm_mux, kcm_mux_list);
    }
    static struct kcm_mux *kcm_get_idx(struct seq_file *seq, loff_t pos)
    {
    struct net *net = seq_file_net(seq);
    struct kcm_net *knet = net_generic(net, kcm_net_id);
    struct kcm_mux *m;
    list_for_each_entry_rcu(m, &knet.mux_list, kcm_mux_list) {
    if (!pos)
    return m;
    --pos;
    }
    return core::ptr::null_mut();
    }
    static void *kcm_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    void *p;
    if (v == SEQ_START_TOKEN)
    p = kcm_get_first(seq);
    else
    p = kcm_get_next(v);
    ++*pos;
    return p;
    }
    static void *kcm_seq_start(struct seq_file *seq, loff_t *pos)
    __acquires(rcu)
    {
    rcu_read_lock();
    if (!*pos)
    return SEQ_START_TOKEN;
    else
    return kcm_get_idx(seq, *pos - 1);
    }
#[no_mangle]
unsafe extern "C" fn kcm_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void kcm_seq_stop(struct seq_file *seq, void *v)
    __releases(rcu)
    {
    rcu_read_unlock();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_proc_mux_state {
    pub p: seq_net_private,
    pub idx: c_int,
}

#[no_mangle]
unsafe extern "C" fn kcm_format_mux_header(seq: *mut seq_file) {
    static void kcm_format_mux_header(struct seq_file *seq)
    {
    struct net *net = seq_file_net(seq);
    struct kcm_net *knet = net_generic(net, kcm_net_id);
    seq_printf(seq,
    "*** KCM statistics (%d MUX) ****\n",
    knet.count);
    seq_printf(seq,
    "%-14s %-10s %-16s %-10s %-16s %-8s %-8s %-8s %-8s %s",
    "Object",
    "RX-Msgs",
    "RX-Bytes",
    "TX-Msgs",
    "TX-Bytes",
    "Recv-Q",
    "Rmem",
    "Send-Q",
    "Smem",
    "Status");
// XXX: pdsts header stuff here
    seq_puts(seq, "\n");
    }
    static void kcm_format_sock(struct kcm_sock *kcm, struct seq_file *seq,
    int i, int *len)
    {
    seq_printf(seq,
    "   kcm-%-7u %-10llu %-16llu %-10llu %-16llu %-8d %-8d %-8d %-8s ",
    kcm.index,
    kcm.stats.rx_msgs,
    kcm.stats.rx_bytes,
    kcm.stats.tx_msgs,
    kcm.stats.tx_bytes,
    kcm.sk.sk_receive_queue.qlen,
    sk_rmem_alloc_get(&kcm.sk),
    kcm.sk.sk_write_queue.qlen,
    "-");
    if (kcm.tx_psock)
    seq_printf(seq, "Psck-%u ", kcm.tx_psock.index);
    if (kcm.tx_wait)
    seq_puts(seq, "TxWait ");
    if (kcm.tx_wait_more)
    seq_puts(seq, "WMore ");
    if (kcm.rx_wait)
    seq_puts(seq, "RxWait ");
    seq_puts(seq, "\n");
    }
    static void kcm_format_psock(struct kcm_psock *psock, struct seq_file *seq,
    int i, int *len)
    {
    seq_printf(seq,
    "   psock-%-5u %-10llu %-16llu %-10llu %-16llu %-8d %-8d %-8d %-8d ",
    psock.index,
    psock.strp.stats.msgs,
    psock.strp.stats.bytes,
    psock.stats.tx_msgs,
    psock.stats.tx_bytes,
    psock.sk.sk_receive_queue.qlen,
    atomic_read(&psock.sk.sk_rmem_alloc),
    psock.sk.sk_write_queue.qlen,
    refcount_read(&psock.sk.sk_wmem_alloc));
    if (psock.done)
    seq_puts(seq, "Done ");
    if (psock.tx_stopped)
    seq_puts(seq, "TxStop ");
    if (psock.strp.stopped)
    seq_puts(seq, "RxStop ");
    if (psock.tx_kcm)
    seq_printf(seq, "Rsvd-%d ", psock.tx_kcm.index);
    if (!psock.strp.paused && !psock.ready_rx_msg) {
    if (psock.sk.sk_receive_queue.qlen) {
    if (psock.strp.need_bytes)
    seq_printf(seq, "RxWait=%u ",
    psock.strp.need_bytes);
    else
    seq_printf(seq, "RxWait ");
    }
    } else  {
    if (psock.strp.paused)
    seq_puts(seq, "RxPause ");
    if (psock.ready_rx_msg)
    seq_puts(seq, "RdyRx ");
    }
    seq_puts(seq, "\n");
    }
    static void
    kcm_format_mux(struct kcm_mux *mux, loff_t idx, struct seq_file *seq)
    {
    int i, len;
    struct kcm_sock *kcm;
    struct kcm_psock *psock;
// mux information
    seq_printf(seq,
    "%-6s%-8s %-10llu %-16llu %-10llu %-16llu %-8s %-8s %-8s %-8s ",
    "mux", "",
    mux.stats.rx_msgs,
    mux.stats.rx_bytes,
    mux.stats.tx_msgs,
    mux.stats.tx_bytes,
    "-", "-", "-", "-");
    seq_printf(seq, "KCMs: %d, Psocks %d\n",
    mux.kcm_socks_cnt, mux.psocks_cnt);
// kcm sock information
    i = 0;
    spin_lock_bh(&mux.lock);
    list_for_each_entry(kcm, &mux.kcm_socks, kcm_sock_list) {
    kcm_format_sock(kcm, seq, i, &len);
    i++;
    }
    i = 0;
    list_for_each_entry(psock, &mux.psocks, psock_list) {
    kcm_format_psock(psock, seq, i, &len);
    i++;
    }
    spin_unlock_bh(&mux.lock);
    }
#[no_mangle]
unsafe extern "C" fn kcm_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int kcm_seq_show(struct seq_file *seq, void *v)
    {
    struct kcm_proc_mux_state *mux_state;
    mux_state = seq.private;
    if (v == SEQ_START_TOKEN) {
    mux_state.idx = 0;
    kcm_format_mux_header(seq);
    } else {
    kcm_format_mux(v, mux_state.idx, seq);
    mux_state.idx++;
    }
    return 0;
    }
    static const struct seq_operations kcm_seq_ops = {
    .show	= kcm_seq_show,
    .start	= kcm_seq_start,
    .next	= kcm_seq_next,
    .stop	= kcm_seq_stop,
    };
#[no_mangle]
unsafe extern "C" fn kcm_stats_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int kcm_stats_seq_show(struct seq_file *seq, void *v)
    {
    struct kcm_psock_stats psock_stats;
    struct kcm_mux_stats mux_stats;
    struct strp_aggr_stats strp_stats;
    struct kcm_mux *mux;
    struct kcm_psock *psock;
    struct net *net = seq.private;
    struct kcm_net *knet = net_generic(net, kcm_net_id);
    memset(&mux_stats, 0, sizeof(mux_stats));
    memset(&psock_stats, 0, sizeof(psock_stats));
    memset(&strp_stats, 0, sizeof(strp_stats));
    mutex_lock(&knet.mutex);
    aggregate_mux_stats(&knet.aggregate_mux_stats, &mux_stats);
    aggregate_psock_stats(&knet.aggregate_psock_stats,
    &psock_stats);
    aggregate_strp_stats(&knet.aggregate_strp_stats,
    &strp_stats);
    list_for_each_entry(mux, &knet.mux_list, kcm_mux_list) {
    spin_lock_bh(&mux.lock);
    aggregate_mux_stats(&mux.stats, &mux_stats);
    aggregate_psock_stats(&mux.aggregate_psock_stats,
    &psock_stats);
    aggregate_strp_stats(&mux.aggregate_strp_stats,
    &strp_stats);
    list_for_each_entry(psock, &mux.psocks, psock_list) {
    aggregate_psock_stats(&psock.stats, &psock_stats);
    save_strp_stats(&psock.strp, &strp_stats);
    }
    spin_unlock_bh(&mux.lock);
    }
    mutex_unlock(&knet.mutex);
    seq_printf(seq,
    "%-8s %-10s %-16s %-10s %-16s %-10s %-10s %-10s %-10s %-10s\n",
    "MUX",
    "RX-Msgs",
    "RX-Bytes",
    "TX-Msgs",
    "TX-Bytes",
    "TX-Retries",
    "Attach",
    "Unattach",
    "UnattchRsvd",
    "RX-RdyDrops");
    seq_printf(seq,
    "%-8s %-10llu %-16llu %-10llu %-16llu %-10u %-10u %-10u %-10u %-10u\n",
    "",
    mux_stats.rx_msgs,
    mux_stats.rx_bytes,
    mux_stats.tx_msgs,
    mux_stats.tx_bytes,
    mux_stats.tx_retries,
    mux_stats.psock_attach,
    mux_stats.psock_unattach_rsvd,
    mux_stats.psock_unattach,
    mux_stats.rx_ready_drops);
    seq_printf(seq,
    "%-8s %-10s %-16s %-10s %-16s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s\n",
    "Psock",
    "RX-Msgs",
    "RX-Bytes",
    "TX-Msgs",
    "TX-Bytes",
    "Reserved",
    "Unreserved",
    "RX-Aborts",
    "RX-Intr",
    "RX-Unrecov",
    "RX-MemFail",
    "RX-NeedMor",
    "RX-BadLen",
    "RX-TooBig",
    "RX-Timeout",
    "TX-Aborts");
    seq_printf(seq,
    "%-8s %-10llu %-16llu %-10llu %-16llu %-10llu %-10llu %-10u %-10u %-10u %-10u %-10u %-10u %-10u %-10u %-10u\n",
    "",
    strp_stats.msgs,
    strp_stats.bytes,
    psock_stats.tx_msgs,
    psock_stats.tx_bytes,
    psock_stats.reserved,
    psock_stats.unreserved,
    strp_stats.aborts,
    strp_stats.interrupted,
    strp_stats.unrecov_intr,
    strp_stats.mem_fail,
    strp_stats.need_more_hdr,
    strp_stats.bad_hdr_len,
    strp_stats.msg_too_big,
    strp_stats.msg_timeouts,
    psock_stats.tx_aborts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kcm_proc_init_net(net: *mut net) -> c_int {
    static int kcm_proc_init_net(struct net *net)
    {
    if (!proc_create_net_single("kcm_stats", 0444, net.proc_net,
    kcm_stats_seq_show, core::ptr::null_mut()))
    goto out_kcm_stats;
    if (!proc_create_net("kcm", 0444, net.proc_net, &kcm_seq_ops,
    sizeof(struct kcm_proc_mux_state)))
    goto out_kcm;
    return 0;
    out_kcm:
    remove_proc_entry("kcm_stats", net.proc_net);
    out_kcm_stats:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn kcm_proc_exit_net(net: *mut net) {
    static void kcm_proc_exit_net(struct net *net)
    {
    remove_proc_entry("kcm", net.proc_net);
    remove_proc_entry("kcm_stats", net.proc_net);
    }
    static struct pernet_operations kcm_net_ops = {
    .init = kcm_proc_init_net,
    .exit = kcm_proc_exit_net,
    };
#[no_mangle]
pub unsafe extern "C" fn kcm_proc_init() -> int __init {
    int __init kcm_proc_init(void)
    {
    return register_pernet_subsys(&kcm_net_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn kcm_proc_exit() -> void __exit {
    void __exit kcm_proc_exit(void)
    {
    unregister_pernet_subsys(&kcm_net_ops);
    }
