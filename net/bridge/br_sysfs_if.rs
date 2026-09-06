//! Automatically rewritten from C to Rust
//! Source: net/bridge/br_sysfs_if.c
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
// Sysfs attributes of bridge ports
// Linux ethernet bridge
//
// Authors:
// Stephen Hemminger		<shemminger@osdl.org>
//

// IMPORTANT: new bridge port options must be added with netlink support only
// please do not add new sysfs entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brport_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct net_bridge_port , char,
    pub long): *mut *mut *mut int (store)(struct net_bridge_port , unsigned,
    pub ): *mut *mut *mut int (store_raw)(struct net_bridge_port , char,
}

    const struct brport_attribute brport_attr_##_name = {			\
    .attr		= {.name = __stringify(_name),			\
    .mode = _mode },				\
    .show		= _show,					\
    .store_raw	= _store,					\
    };

    const struct brport_attribute brport_attr_##_name = { 	        \
    .attr = {.name = __stringify(_name), 			\
    .mode = _mode },				\
    .show	= _show,					\
    .store	= _store,					\
    };

    static ssize_t show_##_name(struct net_bridge_port *p, char *buf) \
    {								\
    return sysfs_emit(buf, "%d\n", test_bit(_bitnr, &p.flags));	\
    }								\
    static int store_##_name(struct net_bridge_port *p, unsigned long v) \
    {								\
    return store_flag(p, v, _bitnr);				\
    }								\
    static BRPORT_ATTR(_name, 0644,					\
    show_##_name, store_##_name)
    static int store_flag(struct net_bridge_port *p, unsigned long v,
    unsigned long bitnr)
    {
    unsigned long oflags, flags = READ_ONCE(p.flags);
    let mut extack: netlink_ext_ack = {0};
    int err;
    oflags = flags;
    if (v)
    __set_bit(bitnr, &flags);
    else
    __clear_bit(bitnr, &flags);
    if (flags == oflags)
    return 0;
    err = br_switchdev_set_port_flag(p, flags, BIT(bitnr), &extack);
    if (err) {
    netdev_err(p.dev, "%s\n", extack._msg);
    return err;
    }
    if (v)
    set_bit(bitnr, &p.flags);
    else
    clear_bit(bitnr, &p.flags);
    br_port_flags_change(p, BIT(bitnr));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_path_cost(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_path_cost(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(p.path_cost));
    }
#[no_mangle]
unsafe extern "C" fn store_path_cost(p: *mut net_bridge_port, v: c_ulong) -> c_int {
    static int store_path_cost(struct net_bridge_port *p, unsigned long v)
    {
    int ret;
    spin_lock_bh(&p.br.lock);
    ret = br_stp_set_path_cost(p, v);
    spin_unlock_bh(&p.br.lock);
    return ret;
    }
    static BRPORT_ATTR(path_cost, 0644, show_path_cost, store_path_cost);
#[no_mangle]
unsafe extern "C" fn show_priority(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_priority(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(p.priority));
    }
#[no_mangle]
unsafe extern "C" fn store_priority(p: *mut net_bridge_port, v: c_ulong) -> c_int {
    static int store_priority(struct net_bridge_port *p, unsigned long v)
    {
    int ret;
    spin_lock_bh(&p.br.lock);
    ret = br_stp_set_port_priority(p, v);
    spin_unlock_bh(&p.br.lock);
    return ret;
    }
    static BRPORT_ATTR(priority, 0644, show_priority, store_priority);
#[no_mangle]
unsafe extern "C" fn show_designated_root(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_designated_root(struct net_bridge_port *p, char *buf)
    {
    return br_show_bridge_id(buf, &p.designated_root);
    }
    static BRPORT_ATTR(designated_root, 0444, show_designated_root, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_designated_bridge(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_designated_bridge(struct net_bridge_port *p, char *buf)
    {
    return br_show_bridge_id(buf, &p.designated_bridge);
    }
    static BRPORT_ATTR(designated_bridge, 0444, show_designated_bridge, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_designated_port(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_designated_port(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(p.designated_port));
    }
    static BRPORT_ATTR(designated_port, 0444, show_designated_port, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_designated_cost(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_designated_cost(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(p.designated_cost));
    }
    static BRPORT_ATTR(designated_cost, 0444, show_designated_cost, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_port_id(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_port_id(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "0x%x\n", READ_ONCE(p.port_id));
    }
    static BRPORT_ATTR(port_id, 0444, show_port_id, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_port_no(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_port_no(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "0x%x\n", p.port_no);
    }
    static BRPORT_ATTR(port_no, 0444, show_port_no, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_change_ack(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_change_ack(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", p.topology_change_ack);
    }
    static BRPORT_ATTR(change_ack, 0444, show_change_ack, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_config_pending(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_config_pending(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(p.config_pending));
    }
    static BRPORT_ATTR(config_pending, 0444, show_config_pending, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn show_port_state(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_port_state(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", p.state);
    }
    static BRPORT_ATTR(state, 0444, show_port_state, core::ptr::null_mut());
    static ssize_t show_message_age_timer(struct net_bridge_port *p,
    char *buf)
    {
    return sysfs_emit(buf, "%ld\n", br_timer_value(&p.message_age_timer));
    }
    static BRPORT_ATTR(message_age_timer, 0444, show_message_age_timer, core::ptr::null_mut());
    static ssize_t show_forward_delay_timer(struct net_bridge_port *p,
    char *buf)
    {
    return sysfs_emit(buf, "%ld\n", br_timer_value(&p.forward_delay_timer));
    }
    static BRPORT_ATTR(forward_delay_timer, 0444, show_forward_delay_timer, core::ptr::null_mut());
    static ssize_t show_hold_timer(struct net_bridge_port *p,
    char *buf)
    {
    return sysfs_emit(buf, "%ld\n", br_timer_value(&p.hold_timer));
    }
    static BRPORT_ATTR(hold_timer, 0444, show_hold_timer, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn store_flush(p: *mut net_bridge_port, v: c_ulong) -> c_int {
    static int store_flush(struct net_bridge_port *p, unsigned long v)
    {
    br_fdb_delete_by_port(p.br, p, 0, 0); // Don't delete local entry
    return 0;
    }
    static BRPORT_ATTR(flush, 0200, core::ptr::null_mut(), store_flush);
#[no_mangle]
unsafe extern "C" fn show_group_fwd_mask(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_group_fwd_mask(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%#x\n", p.group_fwd_mask);
    }
    static int store_group_fwd_mask(struct net_bridge_port *p,
    unsigned long v)
    {
    if (v & BR_GROUPFWD_MACPAUSE)
    return -EINVAL;
    p.group_fwd_mask = v;
    return 0;
    }
    static BRPORT_ATTR(group_fwd_mask, 0644, show_group_fwd_mask,
    store_group_fwd_mask);
#[no_mangle]
unsafe extern "C" fn show_backup_port(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_backup_port(struct net_bridge_port *p, char *buf)
    {
    struct net_bridge_port *backup_p;
    let mut ret: c_int = 0;
    rcu_read_lock();
    backup_p = rcu_dereference(p.backup_port);
    if (backup_p)
    ret = sysfs_emit(buf, "%s\n", backup_p.dev.name);
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn store_backup_port(p: *mut net_bridge_port, buf: *mut c_char) -> c_int {
    static int store_backup_port(struct net_bridge_port *p, char *buf)
    {
    struct net_device *backup_dev = core::ptr::null_mut();
    char *nl = strchr(buf, '\n');
    if (nl)
// nl = '\0';
    if (strlen(buf) > 0) {
    backup_dev = __dev_get_by_name(dev_net(p.dev), buf);
    if (!backup_dev)
    return -ENOENT;
    }
    return nbp_backup_change(p, backup_dev);
    }
    static BRPORT_ATTR_RAW(backup_port, 0644, show_backup_port, store_backup_port);
    BRPORT_ATTR_FLAG(hairpin_mode, BR_HAIRPIN_MODE_BIT);
    BRPORT_ATTR_FLAG(bpdu_guard, BR_BPDU_GUARD_BIT);
    BRPORT_ATTR_FLAG(root_block, BR_ROOT_BLOCK_BIT);
    BRPORT_ATTR_FLAG(learning, BR_LEARNING_BIT);
    BRPORT_ATTR_FLAG(unicast_flood, BR_FLOOD_BIT);
    BRPORT_ATTR_FLAG(proxyarp, BR_PROXYARP_BIT);
    BRPORT_ATTR_FLAG(proxyarp_wifi, BR_PROXYARP_WIFI_BIT);
    BRPORT_ATTR_FLAG(multicast_flood, BR_MCAST_FLOOD_BIT);
    BRPORT_ATTR_FLAG(broadcast_flood, BR_BCAST_FLOOD_BIT);
    BRPORT_ATTR_FLAG(neigh_suppress, BR_NEIGH_SUPPRESS_BIT);
    BRPORT_ATTR_FLAG(isolated, BR_ISOLATED_BIT);

#[no_mangle]
unsafe extern "C" fn show_multicast_router(p: *mut net_bridge_port, buf: *mut c_char) -> isize {
    static ssize_t show_multicast_router(struct net_bridge_port *p, char *buf)
    {
    return sysfs_emit(buf, "%d\n", p.multicast_ctx.multicast_router);
    }
    static int store_multicast_router(struct net_bridge_port *p,
    unsigned long v)
    {
    return br_multicast_set_port_router(&p.multicast_ctx, v);
    }
    static BRPORT_ATTR(multicast_router, 0644, show_multicast_router,
    store_multicast_router);
    BRPORT_ATTR_FLAG(multicast_fast_leave, BR_MULTICAST_FAST_LEAVE_BIT);
    BRPORT_ATTR_FLAG(multicast_to_unicast, BR_MULTICAST_TO_UNICAST_BIT);

    static const struct brport_attribute *brport_attrs[] = {
    &brport_attr_path_cost,
    &brport_attr_priority,
    &brport_attr_port_id,
    &brport_attr_port_no,
    &brport_attr_designated_root,
    &brport_attr_designated_bridge,
    &brport_attr_designated_port,
    &brport_attr_designated_cost,
    &brport_attr_state,
    &brport_attr_change_ack,
    &brport_attr_config_pending,
    &brport_attr_message_age_timer,
    &brport_attr_forward_delay_timer,
    &brport_attr_hold_timer,
    &brport_attr_flush,
    &brport_attr_hairpin_mode,
    &brport_attr_bpdu_guard,
    &brport_attr_root_block,
    &brport_attr_learning,
    &brport_attr_unicast_flood,

    &brport_attr_multicast_router,
    &brport_attr_multicast_fast_leave,
    &brport_attr_multicast_to_unicast,

    &brport_attr_proxyarp,
    &brport_attr_proxyarp_wifi,
    &brport_attr_multicast_flood,
    &brport_attr_broadcast_flood,
    &brport_attr_group_fwd_mask,
    &brport_attr_neigh_suppress,
    &brport_attr_isolated,
    &brport_attr_backup_port,
    core::ptr::null_mut()
    };

    static ssize_t brport_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct brport_attribute *brport_attr = to_brport_attr(attr);
    struct net_bridge_port *p = kobj_to_brport(kobj);
    if (!brport_attr.show)
    return -EINVAL;
    return brport_attr.show(p, buf);
    }
    static ssize_t brport_store(struct kobject *kobj,
    struct attribute *attr,
    const char *buf, size_t count)
    {
    struct brport_attribute *brport_attr = to_brport_attr(attr);
    struct net_bridge_port *p = kobj_to_brport(kobj);
    let mut ret: isize = -EINVAL;
    unsigned long val;
    char *endp;
    if (!ns_capable(dev_net(p.dev).user_ns, CAP_NET_ADMIN))
    return -EPERM;
    if (!rtnl_trylock())
    return restart_syscall();
    if (brport_attr.store_raw) {
    char *buf_copy;
    buf_copy = kstrndup(buf, count, GFP_KERNEL);
    if (!buf_copy) {
    ret = -ENOMEM;
    goto out_unlock;
    }
    ret = brport_attr.store_raw(p, buf_copy);
    kfree(buf_copy);
    } else if (brport_attr.store) {
    val = simple_strtoul(buf, &endp, 0);
    if (endp == buf)
    goto out_unlock;
    ret = brport_attr.store(p, val);
    }
    if (!ret) {
    br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
    ret = count;
    }
    out_unlock:
    rtnl_unlock();
    return ret;
    }
    const struct sysfs_ops brport_sysfs_ops = {
    .show = brport_show,
    .store = brport_store,
    };
//
// Add sysfs entries to ethernet device added to a bridge.
// Creates a brport subdirectory with bridge attributes.
// Puts symlink in bridge's brif subdirectory
//
#[no_mangle]
pub unsafe extern "C" fn br_sysfs_addif(p: *mut net_bridge_port) -> c_int {
    int br_sysfs_addif(struct net_bridge_port *p)
    {
    struct net_bridge *br = p.br;
    const struct brport_attribute **a;
    int err;
    err = sysfs_create_link(&p.kobj, &br.dev.dev.kobj,
    SYSFS_BRIDGE_PORT_LINK);
    if (err)
    return err;
    for (a = brport_attrs; *a; ++a) {
    err = sysfs_create_file(&p.kobj, &((*a).attr));
    if (err)
    return err;
    }
    strscpy(p.sysfs_name, p.dev.name, IFNAMSIZ);
    return sysfs_create_link(br.ifobj, &p.kobj, p.sysfs_name);
    }
// Rename bridge's brif symlink
#[no_mangle]
pub unsafe extern "C" fn br_sysfs_renameif(p: *mut net_bridge_port) -> c_int {
    int br_sysfs_renameif(struct net_bridge_port *p)
    {
    struct net_bridge *br = p.br;
    int err;
// If a rename fails, the rollback will cause another
// rename call with the existing name.
//
    if (!strncmp(p.sysfs_name, p.dev.name, IFNAMSIZ))
    return 0;
    err = sysfs_rename_link(br.ifobj, &p.kobj,
    p.sysfs_name, p.dev.name);
    if (err)
    netdev_notice(br.dev, "unable to rename link %s to %s",
    p.sysfs_name, p.dev.name);
    else
    strscpy(p.sysfs_name, p.dev.name, IFNAMSIZ);
    return err;
    }
