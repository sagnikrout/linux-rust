//! Automatically rewritten from C to Rust
//! Source: net/l2tp/l2tp_debugfs.c
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
// L2TP subsystem debugfs
//
// Copyright (c) 2010 Katalix Systems Ltd
//

    static struct dentry *rootdir;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_dfs_seq_data {
    pub net: *mut net,
    pub ns_tracker: netns_tracker,
    pub /: *mut *mut unsigned long tkey; / lookup key of current tunnel,
    pub /: *mut *mut unsigned long skey; / lookup key of current session,
    pub tunnel: *mut l2tp_tunnel,
    pub /: *mut *mut *mut l2tp_session session; / NULL means get next tunnel,
}

#[no_mangle]
unsafe extern "C" fn l2tp_dfs_next_tunnel(pd: *mut l2tp_dfs_seq_data) {
    static void l2tp_dfs_next_tunnel(struct l2tp_dfs_seq_data *pd)
    {
// Drop reference taken during previous invocation
    if (pd.tunnel)
    l2tp_tunnel_put(pd.tunnel);
    pd.tunnel = l2tp_tunnel_get_next(pd.net, &pd.tkey);
    pd.tkey++;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_next_session(pd: *mut l2tp_dfs_seq_data) {
    static void l2tp_dfs_next_session(struct l2tp_dfs_seq_data *pd)
    {
// Drop reference taken during previous invocation
    if (pd.session)
    l2tp_session_put(pd.session);
    pd.session = l2tp_session_get_next(pd.net, pd.tunnel.sock,
    pd.tunnel.version,
    pd.tunnel.tunnel_id, &pd.skey);
    pd.skey++;
    if (!pd.session) {
    pd.skey = 0;
    l2tp_dfs_next_tunnel(pd);
    }
    }
    static void *l2tp_dfs_seq_start(struct seq_file *m, loff_t *offs)
    {
    struct l2tp_dfs_seq_data *pd = SEQ_START_TOKEN;
    let mut pos: loff_t = *offs;
    if (!pos)
    goto out;
    if (WARN_ON(!m.private)) {
    pd = core::ptr::null_mut();
    goto out;
    }
    pd = m.private;
    if (!pd.tunnel)
    l2tp_dfs_next_tunnel(pd);
    else
    l2tp_dfs_next_session(pd);
// NULL tunnel and session indicates end of list
    if (!pd.tunnel && !pd.session)
    pd = core::ptr::null_mut();
    out:
    return pd;
    }
    static void *l2tp_dfs_seq_next(struct seq_file *m, void *v, loff_t *pos)
    {
    (*pos)++;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_stop(p: *mut seq_file, v: *mut c_void) {
    static void l2tp_dfs_seq_stop(struct seq_file *p, void *v)
    {
    struct l2tp_dfs_seq_data *pd = v;
    if (!pd || pd == SEQ_START_TOKEN)
    return;
// Drop reference taken by last invocation of l2tp_dfs_next_session()
// or l2tp_dfs_next_tunnel().
//
    if (pd.session) {
    l2tp_session_put(pd.session);
    pd.session = core::ptr::null_mut();
    }
    if (pd.tunnel) {
    l2tp_tunnel_put(pd.tunnel);
    pd.tunnel = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_tunnel_show(m: *mut seq_file, v: *mut c_void) {
    static void l2tp_dfs_seq_tunnel_show(struct seq_file *m, void *v)
    {
    struct l2tp_tunnel *tunnel = v;
    struct l2tp_session *session;
    let mut session_count: c_int = 0;
    rcu_read_lock_bh();
    list_for_each_entry_rcu(session, &tunnel.session_list, list) {
// Session ID of zero is a dummy/reserved value used by pppol2tp
    if (session.session_id == 0)
    continue;
    session_count++;
    }
    rcu_read_unlock_bh();
    seq_printf(m, "\nTUNNEL %u peer %u", tunnel.tunnel_id, tunnel.peer_tunnel_id);
    if (tunnel.sock) {
    struct inet_sock *inet = inet_sk(tunnel.sock);

    if (tunnel.sock.sk_family == AF_INET6) {
    const struct ipv6_pinfo *np = inet6_sk(tunnel.sock);
    seq_printf(m, " from %pI6c to %pI6c\n",
    &np.saddr, &tunnel.sock.sk_v6_daddr);
    }

    if (tunnel.sock.sk_family == AF_INET)
    seq_printf(m, " from %pI4 to %pI4\n",
    &inet.inet_saddr, &inet.inet_daddr);
    if (tunnel.encap == L2TP_ENCAPTYPE_UDP)
    seq_printf(m, " source port %hu, dest port %hu\n",
    ntohs(inet.inet_sport), ntohs(inet.inet_dport));
    }
    seq_printf(m, " L2TPv%d, %s\n", tunnel.version,
    tunnel.encap == L2TP_ENCAPTYPE_UDP ? "UDP" :
    tunnel.encap == L2TP_ENCAPTYPE_IP ? "IP" :
    "");
    seq_printf(m, " %d sessions, refcnt %d/%d\n", session_count,
    tunnel.sock ? refcount_read(&tunnel.sock.sk_refcnt) : 0,
    refcount_read(&tunnel.ref_count));
    seq_printf(m, " %08x tx %ld/%ld/%ld rx %ld/%ld/%ld\n",
    0,
    atomic_long_read(&tunnel.stats.tx_packets),
    atomic_long_read(&tunnel.stats.tx_bytes),
    atomic_long_read(&tunnel.stats.tx_errors),
    atomic_long_read(&tunnel.stats.rx_packets),
    atomic_long_read(&tunnel.stats.rx_bytes),
    atomic_long_read(&tunnel.stats.rx_errors));
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_session_show(m: *mut seq_file, v: *mut c_void) {
    static void l2tp_dfs_seq_session_show(struct seq_file *m, void *v)
    {
    struct l2tp_session *session = v;
    seq_printf(m, "  SESSION %u, peer %u, %s\n", session.session_id,
    session.peer_session_id,
    session.pwtype == L2TP_PWTYPE_ETH ? "ETH" :
    session.pwtype == L2TP_PWTYPE_PPP ? "PPP" :
    "");
    if (session.send_seq || session.recv_seq)
    seq_printf(m, "   nr %u, ns %u\n", session.nr, session.ns);
    seq_printf(m, "   refcnt %d\n", refcount_read(&session.ref_count));
    seq_printf(m, "   config 0/0/%c/%c/-/%s %08x %u\n",
    session.recv_seq ? 'R' : '-',
    session.send_seq ? 'S' : '-',
    session.lns_mode ? "LNS" : "LAC",
    0,
    jiffies_to_msecs(session.reorder_timeout));
    seq_printf(m, "   offset 0 l2specific %hu/%d\n",
    session.l2specific_type, l2tp_get_l2specific_len(session));
    if (session.cookie_len) {
    seq_printf(m, "   cookie %02x%02x%02x%02x",
    session.cookie[0], session.cookie[1],
    session.cookie[2], session.cookie[3]);
    if (session.cookie_len == 8)
    seq_printf(m, "%02x%02x%02x%02x",
    session.cookie[4], session.cookie[5],
    session.cookie[6], session.cookie[7]);
    seq_puts(m, "\n");
    }
    if (session.peer_cookie_len) {
    seq_printf(m, "   peer cookie %02x%02x%02x%02x",
    session.peer_cookie[0], session.peer_cookie[1],
    session.peer_cookie[2], session.peer_cookie[3]);
    if (session.peer_cookie_len == 8)
    seq_printf(m, "%02x%02x%02x%02x",
    session.peer_cookie[4], session.peer_cookie[5],
    session.peer_cookie[6], session.peer_cookie[7]);
    seq_puts(m, "\n");
    }
    seq_printf(m, "   %u/%u tx %ld/%ld/%ld rx %ld/%ld/%ld\n",
    session.nr, session.ns,
    atomic_long_read(&session.stats.tx_packets),
    atomic_long_read(&session.stats.tx_bytes),
    atomic_long_read(&session.stats.tx_errors),
    atomic_long_read(&session.stats.rx_packets),
    atomic_long_read(&session.stats.rx_bytes),
    atomic_long_read(&session.stats.rx_errors));
    if (session.show)
    session.show(m, session);
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int l2tp_dfs_seq_show(struct seq_file *m, void *v)
    {
    struct l2tp_dfs_seq_data *pd = v;
// display header on line 1
    if (v == SEQ_START_TOKEN) {
    seq_puts(m, "TUNNEL ID, peer ID from IP to IP\n");
    seq_puts(m, " L2TPv2/L2TPv3, UDP/IP\n");
    seq_puts(m, " sessions session-count, refcnt refcnt/sk.refcnt\n");
    seq_puts(m, " debug tx-pkts/bytes/errs rx-pkts/bytes/errs\n");
    seq_puts(m, "  SESSION ID, peer ID, PWTYPE\n");
    seq_puts(m, "   refcnt cnt\n");
    seq_puts(m, "   offset OFFSET l2specific TYPE/LEN\n");
    seq_puts(m, "   [ cookie ]\n");
    seq_puts(m, "   [ peer cookie ]\n");
    seq_puts(m, "   config mtu/mru/rcvseq/sendseq/dataseq/lns debug reorderto\n");
    seq_puts(m, "   nr/ns tx-pkts/bytes/errs rx-pkts/bytes/errs\n");
    goto out;
    }
    if (!pd.session)
    l2tp_dfs_seq_tunnel_show(m, pd.tunnel);
    else
    l2tp_dfs_seq_session_show(m, pd.session);
    out:
    return 0;
    }
    static const struct seq_operations l2tp_dfs_seq_ops = {
    .start		= l2tp_dfs_seq_start,
    .next		= l2tp_dfs_seq_next,
    .stop		= l2tp_dfs_seq_stop,
    .show		= l2tp_dfs_seq_show,
    };
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    static int l2tp_dfs_seq_open(struct inode *inode, struct file *file)
    {
    struct l2tp_dfs_seq_data *pd;
    struct seq_file *seq;
    let mut rc: c_int = -ENOMEM;
    pd = kzalloc_obj(*pd);
    if (!pd)
    goto out;
// Derive the network namespace from the pid opening the
// file.
//
    pd.net = get_net_ns_by_pid(current.pid);
    if (IS_ERR(pd.net)) {
    rc = PTR_ERR(pd.net);
    goto err_free_pd;
    }
    netns_tracker_alloc(pd.net, &pd.ns_tracker, GFP_KERNEL);
    rc = seq_open(file, &l2tp_dfs_seq_ops);
    if (rc)
    goto err_free_net;
    seq = file.private_data;
    seq.private = pd;
    out:
    return rc;
    err_free_net:
    put_net_track(pd.net, &pd.ns_tracker);
    err_free_pd:
    kfree(pd);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_dfs_seq_release(inode: *mut inode, file: *mut file) -> c_int {
    static int l2tp_dfs_seq_release(struct inode *inode, struct file *file)
    {
    struct l2tp_dfs_seq_data *pd;
    struct seq_file *seq;
    seq = file.private_data;
    pd = seq.private;
    if (pd.session)
    l2tp_session_put(pd.session);
    if (pd.tunnel)
    l2tp_tunnel_put(pd.tunnel);
    if (pd.net)
    put_net_track(pd.net, &pd.ns_tracker);
    kfree(pd);
    seq_release(inode, file);
    return 0;
    }
    static const struct file_operations l2tp_dfs_fops = {
    .owner		= THIS_MODULE,
    .open		= l2tp_dfs_seq_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= l2tp_dfs_seq_release,
    };
#[no_mangle]
unsafe extern "C" fn l2tp_debugfs_init() -> int __init {
    static int __init l2tp_debugfs_init(void)
    {
    rootdir = debugfs_create_dir("l2tp", core::ptr::null_mut());
    debugfs_create_file("tunnels", 0600, rootdir, core::ptr::null_mut(), &l2tp_dfs_fops);
    pr_info("L2TP debugfs support\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_debugfs_exit() -> void __exit {
    static void __exit l2tp_debugfs_exit(void)
    {
    debugfs_remove_recursive(rootdir);
    }
    module_init(l2tp_debugfs_init);
    module_exit(l2tp_debugfs_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Chapman <jchapman@katalix.com>");
    MODULE_DESCRIPTION("L2TP debugfs driver");
    MODULE_VERSION("1.0");
