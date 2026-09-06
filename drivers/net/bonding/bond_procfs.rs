//! Automatically rewritten from C to Rust
//! Source: drivers/net/bonding/bond_procfs.c
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

    static void *bond_info_seq_start(struct seq_file *seq, loff_t *pos)
    __acquires(RCU)
    {
    struct bonding *bond = pde_data(file_inode(seq.file));
    struct list_head *iter;
    struct slave *slave;
    let mut off: loff_t = 0;
    rcu_read_lock();
    if (*pos == 0)
    return SEQ_START_TOKEN;
    bond_for_each_slave_rcu(bond, slave, iter)
    if (++off == *pos)
    return slave;
    return core::ptr::null_mut();
    }
    static void *bond_info_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bonding *bond = pde_data(file_inode(seq.file));
    struct list_head *iter;
    struct slave *slave;
    let mut found: bool = false;
    ++*pos;
    if (v == SEQ_START_TOKEN)
    return bond_first_slave_rcu(bond);
    bond_for_each_slave_rcu(bond, slave, iter) {
    if (found)
    return slave;
    if (slave == v)
    found = true;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bond_info_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void bond_info_seq_stop(struct seq_file *seq, void *v)
    __releases(RCU)
    {
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn bond_info_show_master(seq: *mut seq_file) {
    static void bond_info_show_master(struct seq_file *seq)
    {
    struct bonding *bond = pde_data(file_inode(seq.file));
    const struct bond_opt_value *optval;
    struct slave *curr, *primary;
    int arp_interval, fail_over_mac, miimon, i;
    curr = rcu_dereference(bond.curr_active_slave);
    seq_printf(seq, "Bonding Mode: %s",
    bond_mode_name(BOND_MODE(bond)));
    fail_over_mac = READ_ONCE(bond.params.fail_over_mac);
    if (BOND_MODE(bond) == BOND_MODE_ACTIVEBACKUP && fail_over_mac) {
    optval = bond_opt_get_val(BOND_OPT_FAIL_OVER_MAC,
    fail_over_mac);
    seq_printf(seq, " (fail_over_mac %s)", optval.string);
    }
    seq_printf(seq, "\n");
    if (bond_mode_uses_xmit_hash(bond)) {
    let mut xmit_policy: c_int = READ_ONCE(bond.params.xmit_policy);
    optval = bond_opt_get_val(BOND_OPT_XMIT_HASH, xmit_policy);
    seq_printf(seq, "Transmit Hash Policy: %s (%d)\n",
    optval.string, xmit_policy);
    }
    if (bond_uses_primary(bond)) {
    primary = rcu_dereference(bond.primary_slave);
    seq_printf(seq, "Primary Slave: %s",
    primary ? primary.dev.name : "None");
    if (primary) {
    optval = bond_opt_get_val(BOND_OPT_PRIMARY_RESELECT,
    READ_ONCE(bond.params.primary_reselect));
    seq_printf(seq, " (primary_reselect %s)",
    optval.string);
    }
    seq_printf(seq, "\nCurrently Active Slave: %s\n",
    (curr) ? curr.dev.name : "None");
    }
    seq_printf(seq, "MII Status: %s\n", netif_carrier_ok(bond.dev) ?
    "up" : "down");
    miimon = READ_ONCE(bond.params.miimon);
    seq_printf(seq, "MII Polling Interval (ms): %d\n", miimon);
    seq_printf(seq, "Up Delay (ms): %d\n",
    READ_ONCE(bond.params.updelay) * miimon);
    seq_printf(seq, "Down Delay (ms): %d\n",
    READ_ONCE(bond.params.downdelay) * miimon);
    seq_printf(seq, "Peer Notification Delay (ms): %d\n",
    READ_ONCE(bond.params.peer_notif_delay) * miimon);
// ARP information
    arp_interval = READ_ONCE(bond.params.arp_interval);
    if (arp_interval > 0) {
    let mut printed: c_int = 0;
    seq_printf(seq, "ARP Polling Interval (ms): %d\n",
    arp_interval);
    seq_printf(seq, "ARP Missed Max: %u\n",
    READ_ONCE(bond.params.missed_max));
    seq_printf(seq, "ARP IP target/s (n.n.n.n form):");
    for (i = 0; (i < BOND_MAX_ARP_TARGETS); i++) {
    let mut t: __be32 = READ_ONCE(bond.params.arp_targets[i]);
    if (!t)
    break;
    if (printed)
    seq_printf(seq, ",");
    seq_printf(seq, " %pI4", &t);
    printed = 1;
    }
    seq_printf(seq, "\n");

    printed = 0;
    seq_printf(seq, "NS IPv6 target/s (xx::xx form):");
    for (i = 0; (i < BOND_MAX_NS_TARGETS); i++) {
    if (ipv6_addr_any(&bond.params.ns_targets[i]))
    break;
    if (printed)
    seq_printf(seq, ",");
    seq_printf(seq, " %pI6c", &bond.params.ns_targets[i]);
    printed = 1;
    }
    seq_printf(seq, "\n");

    }
    if (BOND_MODE(bond) == BOND_MODE_8023AD) {
    struct ad_info ad_info;
    seq_puts(seq, "\n802.3ad info\n");
    seq_printf(seq, "LACP active: %s\n",
    READ_ONCE(bond.params.lacp_active) ? "on" : "off");
    seq_printf(seq, "LACP rate: %s\n",
    READ_ONCE(bond.params.lacp_fast) ? "fast" : "slow");
    seq_printf(seq, "Min links: %d\n",
    READ_ONCE(bond.params.min_links));
    optval = bond_opt_get_val(BOND_OPT_AD_SELECT,
    READ_ONCE(bond.params.ad_select));
    seq_printf(seq, "Aggregator selection policy (ad_select): %s\n",
    optval.string);
    if (capable(CAP_NET_ADMIN)) {
    seq_printf(seq, "System priority: %d\n",
    BOND_AD_INFO(bond).system.sys_priority);
    seq_printf(seq, "System MAC address: %pM\n",
    &BOND_AD_INFO(bond).system.sys_mac_addr);
    if (__bond_3ad_get_active_agg_info(bond, &ad_info)) {
    seq_printf(seq,
    "bond %s has no active aggregator\n",
    bond.dev.name);
    } else {
    seq_printf(seq, "Active Aggregator Info:\n");
    seq_printf(seq, "\tAggregator ID: %d\n",
    ad_info.aggregator_id);
    seq_printf(seq, "\tNumber of ports: %d\n",
    ad_info.ports);
    seq_printf(seq, "\tActor Key: %d\n",
    ad_info.actor_key);
    seq_printf(seq, "\tPartner Key: %d\n",
    ad_info.partner_key);
    seq_printf(seq, "\tPartner Mac Address: %pM\n",
    ad_info.partner_system);
    }
    }
    }
    }
// Note: runs under rcu_read_lock()
    static void bond_info_show_slave(struct seq_file *seq,
    const struct slave *slave)
    {
    struct bonding *bond = pde_data(file_inode(seq.file));
    seq_printf(seq, "\nSlave Interface: %s\n", slave.dev.name);
    seq_printf(seq, "MII Status: %s\n", bond_slave_link_status(slave.link));
    if (slave.speed == SPEED_UNKNOWN)
    seq_printf(seq, "Speed: %s\n", "Unknown");
    else
    seq_printf(seq, "Speed: %d Mbps\n", slave.speed);
    if (slave.duplex == DUPLEX_UNKNOWN)
    seq_printf(seq, "Duplex: %s\n", "Unknown");
    else
    seq_printf(seq, "Duplex: %s\n", slave.duplex ? "full" : "half");
    seq_printf(seq, "Link Failure Count: %u\n",
    slave.link_failure_count);
    seq_printf(seq, "Permanent HW addr: %*phC\n",
    slave.dev.addr_len, slave.perm_hwaddr);
    seq_printf(seq, "Slave queue ID: %d\n", READ_ONCE(slave.queue_id));
    if (BOND_MODE(bond) == BOND_MODE_8023AD) {
    const struct port *port = &SLAVE_AD_INFO(slave).port;
    const struct aggregator *agg = rcu_dereference(port.aggregator);
    if (agg) {
    seq_printf(seq, "Aggregator ID: %d\n",
    agg.aggregator_identifier);
    seq_printf(seq, "Actor Churn State: %s\n",
    bond_3ad_churn_desc(READ_ONCE(port.sm_churn_actor_state)));
    seq_printf(seq, "Partner Churn State: %s\n",
    bond_3ad_churn_desc(READ_ONCE(port.sm_churn_partner_state)));
    seq_printf(seq, "Actor Churned Count: %d\n",
    READ_ONCE(port.churn_actor_count));
    seq_printf(seq, "Partner Churned Count: %d\n",
    READ_ONCE(port.churn_partner_count));
    if (capable(CAP_NET_ADMIN)) {
    seq_puts(seq, "details actor lacp pdu:\n");
    seq_printf(seq, "    system priority: %d\n",
    port.actor_system_priority);
    seq_printf(seq, "    system mac address: %pM\n",
    &port.actor_system);
    seq_printf(seq, "    port key: %d\n",
    port.actor_oper_port_key);
    seq_printf(seq, "    port priority: %d\n",
    port.actor_port_priority);
    seq_printf(seq, "    port number: %d\n",
    port.actor_port_number);
    seq_printf(seq, "    port state: %d\n",
    port.actor_oper_port_state);
    seq_puts(seq, "details partner lacp pdu:\n");
    seq_printf(seq, "    system priority: %d\n",
    port.partner_oper.system_priority);
    seq_printf(seq, "    system mac address: %pM\n",
    &port.partner_oper.system);
    seq_printf(seq, "    oper key: %d\n",
    port.partner_oper.key);
    seq_printf(seq, "    port priority: %d\n",
    port.partner_oper.port_priority);
    seq_printf(seq, "    port number: %d\n",
    port.partner_oper.port_number);
    seq_printf(seq, "    port state: %d\n",
    port.partner_oper.port_state);
    }
    } else {
    seq_puts(seq, "Aggregator ID: N/A\n");
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bond_info_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int bond_info_seq_show(struct seq_file *seq, void *v)
    {
    if (v == SEQ_START_TOKEN) {
    seq_printf(seq, "%s\n", bond_version);
    bond_info_show_master(seq);
    } else
    bond_info_show_slave(seq, v);
    return 0;
    }
    static const struct seq_operations bond_info_seq_ops = {
    .start = bond_info_seq_start,
    .next  = bond_info_seq_next,
    .stop  = bond_info_seq_stop,
    .show  = bond_info_seq_show,
    };
#[no_mangle]
pub unsafe extern "C" fn bond_create_proc_entry(bond: *mut bonding) {
    void bond_create_proc_entry(struct bonding *bond)
    {
    struct net_device *bond_dev = bond.dev;
    struct bond_net *bn = net_generic(dev_net(bond_dev), bond_net_id);
    if (bn.proc_dir) {
    bond.proc_entry = proc_create_seq_data(bond_dev.name, 0444,
    bn.proc_dir, &bond_info_seq_ops, bond);
    if (bond.proc_entry == core::ptr::null_mut())
    netdev_warn(bond_dev, "Cannot create /proc/net/%s/%s\n",
    KBUILD_MODNAME, bond_dev.name);
    else
    memcpy(bond.proc_file_name, bond_dev.name, IFNAMSIZ);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bond_remove_proc_entry(bond: *mut bonding) {
    void bond_remove_proc_entry(struct bonding *bond)
    {
    struct net_device *bond_dev = bond.dev;
    struct bond_net *bn = net_generic(dev_net(bond_dev), bond_net_id);
    if (bn.proc_dir && bond.proc_entry) {
    remove_proc_entry(bond.proc_file_name, bn.proc_dir);
    memset(bond.proc_file_name, 0, IFNAMSIZ);
    bond.proc_entry = core::ptr::null_mut();
    }
    }
// Create the bonding directory under /proc/net, if doesn't exist yet.
// Caller must hold rtnl_lock.
//
#[no_mangle]
pub unsafe extern "C" fn bond_create_proc_dir(bn: *mut bond_net) -> void __net_init {
    void __net_init bond_create_proc_dir(struct bond_net *bn)
    {
    if (!bn.proc_dir) {
    bn.proc_dir = proc_mkdir(KBUILD_MODNAME, bn.net.proc_net);
    if (!bn.proc_dir)
    pr_warn("Warning: Cannot create /proc/net/%s\n",
    KBUILD_MODNAME);
    }
    }
// Destroy the bonding directory under /proc/net, if empty.
//
#[no_mangle]
pub unsafe extern "C" fn bond_destroy_proc_dir(bn: *mut bond_net) -> void __net_exit {
    void __net_exit bond_destroy_proc_dir(struct bond_net *bn)
    {
    if (bn.proc_dir) {
    remove_proc_entry(KBUILD_MODNAME, bn.net.proc_net);
    bn.proc_dir = core::ptr::null_mut();
    }
    }
