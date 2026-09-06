//! Automatically rewritten from C to Rust
//! Source: drivers/net/bonding/bond_sysfs_slave.c
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
// Sysfs attributes of bond slaves
//
// Copyright (c) 2014 Scott Feldman <sfeldma@cumulusnetworks.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slave_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct slave , char,
}

    const struct slave_attribute slave_attr_##_name = __ATTR_RO(_name)
#[no_mangle]
unsafe extern "C" fn state_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t state_show(struct slave *slave, char *buf)
    {
    switch (bond_slave_state(slave)) {
    case BOND_STATE_ACTIVE:
    return sysfs_emit(buf, "active\n");
    case BOND_STATE_BACKUP:
    return sysfs_emit(buf, "backup\n");
    default:
    return sysfs_emit(buf, "UNKNOWN\n");
    }
    }
    static SLAVE_ATTR_RO(state);
#[no_mangle]
unsafe extern "C" fn mii_status_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t mii_status_show(struct slave *slave, char *buf)
    {
    return sysfs_emit(buf, "%s\n", bond_slave_link_status(slave.link));
    }
    static SLAVE_ATTR_RO(mii_status);
#[no_mangle]
unsafe extern "C" fn link_failure_count_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t link_failure_count_show(struct slave *slave, char *buf)
    {
    return sysfs_emit(buf, "%d\n", slave.link_failure_count);
    }
    static SLAVE_ATTR_RO(link_failure_count);
#[no_mangle]
unsafe extern "C" fn perm_hwaddr_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t perm_hwaddr_show(struct slave *slave, char *buf)
    {
    return sysfs_emit(buf, "%*phC\n",
    slave.dev.addr_len,
    slave.perm_hwaddr);
    }
    static SLAVE_ATTR_RO(perm_hwaddr);
#[no_mangle]
unsafe extern "C" fn queue_id_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t queue_id_show(struct slave *slave, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(slave.queue_id));
    }
    static SLAVE_ATTR_RO(queue_id);
#[no_mangle]
unsafe extern "C" fn ad_aggregator_id_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t ad_aggregator_id_show(struct slave *slave, char *buf)
    {
    const struct aggregator *agg;
    if (BOND_MODE(slave.bond) == BOND_MODE_8023AD) {
    rcu_read_lock();
    agg = rcu_dereference(SLAVE_AD_INFO(slave).port.aggregator);
    if (agg) {
    ssize_t res = sysfs_emit(buf, "%d\n",
    agg.aggregator_identifier);
    rcu_read_unlock();
    return res;
    }
    rcu_read_unlock();
    }
    return sysfs_emit(buf, "N/A\n");
    }
    static SLAVE_ATTR_RO(ad_aggregator_id);
#[no_mangle]
unsafe extern "C" fn ad_actor_oper_port_state_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t ad_actor_oper_port_state_show(struct slave *slave, char *buf)
    {
    const struct port *ad_port;
    if (BOND_MODE(slave.bond) == BOND_MODE_8023AD) {
    ad_port = &SLAVE_AD_INFO(slave).port;
    if (rcu_access_pointer(ad_port.aggregator))
    return sysfs_emit(buf, "%u\n",
    ad_port.actor_oper_port_state);
    }
    return sysfs_emit(buf, "N/A\n");
    }
    static SLAVE_ATTR_RO(ad_actor_oper_port_state);
#[no_mangle]
unsafe extern "C" fn ad_partner_oper_port_state_show(slave: *mut slave, buf: *mut c_char) -> isize {
    static ssize_t ad_partner_oper_port_state_show(struct slave *slave, char *buf)
    {
    const struct port *ad_port;
    if (BOND_MODE(slave.bond) == BOND_MODE_8023AD) {
    ad_port = &SLAVE_AD_INFO(slave).port;
    if (rcu_access_pointer(ad_port.aggregator))
    return sysfs_emit(buf, "%u\n",
    ad_port.partner_oper.port_state);
    }
    return sysfs_emit(buf, "N/A\n");
    }
    static SLAVE_ATTR_RO(ad_partner_oper_port_state);
    static const struct attribute *slave_attrs[] = {
    &slave_attr_state.attr,
    &slave_attr_mii_status.attr,
    &slave_attr_link_failure_count.attr,
    &slave_attr_perm_hwaddr.attr,
    &slave_attr_queue_id.attr,
    &slave_attr_ad_aggregator_id.attr,
    &slave_attr_ad_actor_oper_port_state.attr,
    &slave_attr_ad_partner_oper_port_state.attr,
    core::ptr::null_mut()
    };

    static ssize_t slave_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct slave_attribute *slave_attr = to_slave_attr(attr);
    struct slave *slave = to_slave(kobj);
    return slave_attr.show(slave, buf);
    }
    const struct sysfs_ops slave_sysfs_ops = {
    .show = slave_show,
    };
#[no_mangle]
pub unsafe extern "C" fn bond_sysfs_slave_add(slave: *mut slave) -> c_int {
    int bond_sysfs_slave_add(struct slave *slave)
    {
    return sysfs_create_files(&slave.kobj, slave_attrs);
    }
#[no_mangle]
pub unsafe extern "C" fn bond_sysfs_slave_del(slave: *mut slave) {
    void bond_sysfs_slave_del(struct slave *slave)
    {
    sysfs_remove_files(&slave.kobj, slave_attrs);
    }
