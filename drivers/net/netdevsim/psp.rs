//! Automatically rewritten from C to Rust
//! Source: drivers/net/netdevsim/psp.c
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

#[no_mangle]
pub unsafe extern "C" fn nsim_psp_handle_ext(skb: *mut sk_buff, psp_ext: *mut skb_ext) {
    void nsim_psp_handle_ext(struct sk_buff *skb, struct skb_ext *psp_ext)
    {
    if (psp_ext)
    __skb_ext_set(skb, SKB_EXT_PSP, psp_ext);
    }
    enum skb_drop_reason
    nsim_do_psp(struct sk_buff *skb, struct netdevsim *ns,
    struct netdevsim *peer_ns, struct skb_ext **psp_ext)
    {
    let mut rc: enum skb_drop_reason = 0;
    struct psp_dev *peer_psd;
    struct psp_assoc *pas;
    struct net *net;
    int psp_len;
    void **ptr;
    rcu_read_lock();
    pas = psp_skb_get_assoc_rcu(skb);
    if (!pas) {
    rc = SKB_NOT_DROPPED_YET;
    goto out_unlock;
    }
    if (!skb_transport_header_was_set(skb)) {
    rc = SKB_DROP_REASON_PSP_OUTPUT;
    goto out_unlock;
    }
    ptr = psp_assoc_drv_data(pas);
    if (*ptr != ns) {
    rc = SKB_DROP_REASON_PSP_OUTPUT;
    goto out_unlock;
    }
    net = sock_net(skb.sk);
    if (!psp_dev_encapsulate(net, skb, pas.tx.spi, pas.version, 0)) {
    rc = SKB_DROP_REASON_PSP_OUTPUT;
    goto out_unlock;
    }
    psp_len = skb.len - skb_inner_transport_offset(skb);
    atomic64_inc(&ns.psp.tx_packets);
    atomic64_add(psp_len, &ns.psp.tx_bytes);
// Now pretend we just received this frame
    peer_psd = rcu_dereference(peer_ns.psp.dev);
    if (peer_psd && peer_psd.config.versions & (1 << pas.version)) {
    let mut strip_icv: bool = false;
    u8 generation;
// We cheat a bit and put the generation in the key.
// In real life if generation was too old, then decryption would
// fail. Here, we just make it so a bad key causes a bad
// generation too, and psp_sk_rx_policy_check() will fail.
//
    generation = pas.tx.key[0];
    skb_ext_reset(skb);
    skb.mac_len = ETH_HLEN;
    if (psp_dev_rcv(skb, peer_psd.id, generation, strip_icv)) {
    rc = SKB_DROP_REASON_PSP_OUTPUT;
    goto out_unlock;
    }
// psp_ext = skb->extensions;
    refcount_inc(&(*psp_ext).refcnt);
    skb.decrypted = 1;
    atomic64_inc(&peer_ns.psp.rx_packets);
    atomic64_add(psp_len, &peer_ns.psp.rx_bytes);
    } else {
    struct ipv6hdr *ip6h __maybe_unused;
    struct iphdr *iph;
    struct udphdr *uh;
    __wsum csum;
    int udplen;
// Do not decapsulate. Receive the skb with the udp and psp
// headers still there as if this is a normal udp packet.
// psp_dev_encapsulate() sets udp checksum to 0, so we need to
// provide a valid checksum here, so the skb isn't dropped.
//
    uh = udp_hdr(skb);
    udplen = udp_get_len(skb, uh, skb_transport_offset(skb));
    csum = skb_checksum(skb, skb_transport_offset(skb),
    udplen, 0);
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    iph = ip_hdr(skb);
    uh.check = udp_v4_check(udplen, iph.saddr,
    iph.daddr, csum);
    break;

    case htons(ETH_P_IPV6):
    ip6h = ipv6_hdr(skb);
    uh.check = udp_v6_check(udplen, &ip6h.saddr,
    &ip6h.daddr, csum);
    break;

    }
    uh.check	= uh.check ?: CSUM_MANGLED_0;
    skb.ip_summed	= CHECKSUM_NONE;
    }
    out_unlock:
    rcu_read_unlock();
    return rc;
    }
    static int
    nsim_psp_set_config(struct psp_dev *psd, struct psp_dev_config *conf,
    struct netlink_ext_ack *extack)
    {
    return 0;
    }
    static int
    nsim_rx_spi_alloc(struct psp_dev *psd, u32 version,
    struct psp_key_parsed *assoc,
    struct netlink_ext_ack *extack)
    {
    struct netdevsim *ns = psd.drv_priv;
    int i;
// Check if incrementing the spi would change the phase bit
    if ((ns.psp.spi & PSP_SPI_KEY_ID) == PSP_SPI_KEY_ID) {
    NL_SET_ERR_MSG(extack, "SPI space exhausted");
    return -ENOSPC;
    }
    assoc.spi = cpu_to_be32(++ns.psp.spi);
    assoc.key[0] = psd.generation;
    for (i = 1; i < PSP_MAX_KEY; i++)
    assoc.key[i] = ns.psp.spi + i;
    return 0;
    }
    static int nsim_assoc_add(struct psp_dev *psd, struct psp_assoc *pas,
    struct netlink_ext_ack *extack)
    {
    struct netdevsim *ns = psd.drv_priv;
    void **ptr = psp_assoc_drv_data(pas);
// Copy drv_priv from psd to assoc
// ptr = psd->drv_priv;
    ns.psp.assoc_cnt++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nsim_key_rotate(psd: *mut psp_dev, extack: *mut netlink_ext_ack) -> c_int {
    static int nsim_key_rotate(struct psp_dev *psd, struct netlink_ext_ack *extack)
    {
    struct netdevsim *ns = psd.drv_priv;
// Flip key phase and reset SPI to 0 within that space
// (will be pre-incremented, as 0 is an invalid SPI).
//
    if (ns.psp.spi & PSP_SPI_KEY_PHASE)
    ns.psp.spi = 0;
    else
    ns.psp.spi = PSP_SPI_KEY_PHASE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nsim_assoc_del(psd: *mut psp_dev, pas: *mut psp_assoc) {
    static void nsim_assoc_del(struct psp_dev *psd, struct psp_assoc *pas)
    {
    struct netdevsim *ns = psd.drv_priv;
    void **ptr = psp_assoc_drv_data(pas);
// ptr = NULL;
    ns.psp.assoc_cnt--;
    }
#[no_mangle]
unsafe extern "C" fn nsim_get_stats(psd: *mut psp_dev, stats: *mut psp_dev_stats) {
    static void nsim_get_stats(struct psp_dev *psd, struct psp_dev_stats *stats)
    {
    struct netdevsim *ns = psd.drv_priv;
// WARNING: do *not* blindly zero stats in real drivers!
// All required stats must be reported by the device!
//
    memset(stats, 0, sizeof(struct psp_dev_stats));
    stats.rx_bytes = atomic64_read(&ns.psp.rx_bytes);
    stats.rx_packets = atomic64_read(&ns.psp.rx_packets);
    stats.tx_bytes = atomic64_read(&ns.psp.tx_bytes);
    stats.tx_packets = atomic64_read(&ns.psp.tx_packets);
    }
    static struct psp_dev_ops nsim_psp_ops = {
    .set_config	= nsim_psp_set_config,
    .rx_spi_alloc	= nsim_rx_spi_alloc,
    .tx_key_add	= nsim_assoc_add,
    .tx_key_del	= nsim_assoc_del,
    .key_rotate	= nsim_key_rotate,
    .get_stats	= nsim_get_stats,
    };
    static struct psp_dev_caps nsim_psp_caps = {
    .versions = 1 << PSP_VERSION_HDR0_AES_GCM_128 |
    1 << PSP_VERSION_HDR0_AES_GMAC_128 |
    1 << PSP_VERSION_HDR0_AES_GCM_256 |
    1 << PSP_VERSION_HDR0_AES_GMAC_256,
    .assoc_drv_spc = sizeof(void *),
    };
#[no_mangle]
unsafe extern "C" fn __nsim_psp_uninit(ns: *mut netdevsim, teardown: bool) {
    static void __nsim_psp_uninit(struct netdevsim *ns, bool teardown)
    {
    struct psp_dev *psd;
    psd = rcu_dereference_protected(ns.psp.dev,
    teardown ||
    lockdep_is_held(&ns.psp.rereg_lock));
    if (psd) {
    rcu_assign_pointer(ns.psp.dev, core::ptr::null_mut());
    synchronize_rcu();
    psp_dev_unregister(psd);
    }
    WARN_ON(ns.psp.assoc_cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn nsim_psp_uninit(ns: *mut netdevsim) {
    void nsim_psp_uninit(struct netdevsim *ns)
    {
    debugfs_remove(ns.psp.rereg);
    mutex_destroy(&ns.psp.rereg_lock);
    __nsim_psp_uninit(ns, true);
    }
    static ssize_t
    nsim_psp_rereg_write(struct file *file, const char __user *data, size_t count,
    loff_t *ppos)
    {
    struct netdevsim *ns = file.private_data;
    struct psp_dev *psd;
    ssize_t ret;
    mutex_lock(&ns.psp.rereg_lock);
    __nsim_psp_uninit(ns, false);
    psd = psp_dev_create(ns.netdev, &nsim_psp_ops, &nsim_psp_caps, ns);
    if (IS_ERR(psd)) {
    ret = PTR_ERR(psd);
    goto out;
    }
    rcu_assign_pointer(ns.psp.dev, psd);
    ret = count;
    out:
    mutex_unlock(&ns.psp.rereg_lock);
    return ret;
    }
    static const struct file_operations nsim_psp_rereg_fops = {
    .open = simple_open,
    .write = nsim_psp_rereg_write,
    .llseek = generic_file_llseek,
    .owner = THIS_MODULE,
    };
#[no_mangle]
pub unsafe extern "C" fn nsim_psp_init(ns: *mut netdevsim) -> c_int {
    int nsim_psp_init(struct netdevsim *ns)
    {
    struct dentry *ddir = ns.nsim_dev_port.ddir;
    struct psp_dev *psd;
    psd = psp_dev_create(ns.netdev, &nsim_psp_ops, &nsim_psp_caps, ns);
    if (IS_ERR(psd))
    return PTR_ERR(psd);
    rcu_assign_pointer(ns.psp.dev, psd);
    mutex_init(&ns.psp.rereg_lock);
    ns.psp.rereg = debugfs_create_file("psp_rereg", 0200, ddir, ns,
    &nsim_psp_rereg_fops);
    return 0;
    }
