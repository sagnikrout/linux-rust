//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/ulp/rtrs/rtrs-clt-sysfs.c
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
// RDMA Transport Layer
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

pub const MAX_MAX_RECONN_ATT: c_int = 9999;
#[no_mangle]
unsafe extern "C" fn rtrs_clt_path_release(kobj: *mut kobject) {
    static void rtrs_clt_path_release(struct kobject *kobj)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    free_path(clt_path);
    }
    static struct kobj_type ktype_sess = {
    .sysfs_ops = &kobj_sysfs_ops,
    .release = rtrs_clt_path_release
    };
#[no_mangle]
unsafe extern "C" fn rtrs_clt_path_stats_release(kobj: *mut kobject) {
    static void rtrs_clt_path_stats_release(struct kobject *kobj)
    {
    struct rtrs_clt_stats *stats;
    stats = container_of(kobj, struct rtrs_clt_stats, kobj_stats);
    free_percpu(stats.pcpu_stats);
    }
    static struct kobj_type ktype_stats = {
    .sysfs_ops = &kobj_sysfs_ops,
    .release = rtrs_clt_path_stats_release,
    };
    static ssize_t max_reconnect_attempts_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct rtrs_clt_sess *clt = container_of(dev, struct rtrs_clt_sess,
    dev);
    return sysfs_emit(page, "%d\n",
    rtrs_clt_get_max_reconnect_attempts(clt));
    }
    static ssize_t max_reconnect_attempts_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    int value;
    int ret;
    struct rtrs_clt_sess *clt  = container_of(dev, struct rtrs_clt_sess,
    dev);
    ret = kstrtoint(buf, 10, &value);
    if (ret) {
    rtrs_err(clt, "%s: failed to convert string '%s' to int\n",
    attr.attr.name, buf);
    return ret;
    }
    if (value > MAX_MAX_RECONN_ATT ||
    value < MIN_MAX_RECONN_ATT) {
    rtrs_err(clt,
    "%s: invalid range (provided: '%s', accepted: min: %d, max: %d)\n",
    attr.attr.name, buf, MIN_MAX_RECONN_ATT,
    MAX_MAX_RECONN_ATT);
    return -EINVAL;
    }
    rtrs_clt_set_max_reconnect_attempts(clt, value);
    return count;
    }
    static DEVICE_ATTR_RW(max_reconnect_attempts);
    static ssize_t mpath_policy_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct rtrs_clt_sess *clt;
    clt = container_of(dev, struct rtrs_clt_sess, dev);
    switch (clt.mp_policy) {
    case MP_POLICY_RR:
    return sysfs_emit(page, "round-robin (RR: %d)\n",
    clt.mp_policy);
    case MP_POLICY_MIN_INFLIGHT:
    return sysfs_emit(page, "min-inflight (MI: %d)\n",
    clt.mp_policy);
    case MP_POLICY_MIN_LATENCY:
    return sysfs_emit(page, "min-latency (ML: %d)\n",
    clt.mp_policy);
    default:
    return sysfs_emit(page, "Unknown (%d)\n", clt.mp_policy);
    }
    }
    static ssize_t mpath_policy_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    struct rtrs_clt_sess *clt;
    int value;
    int ret;
    let mut len: usize = 0;
    clt = container_of(dev, struct rtrs_clt_sess, dev);
    ret = kstrtoint(buf, 10, &value);
    if (!ret && (value == MP_POLICY_RR ||
    value == MP_POLICY_MIN_INFLIGHT ||
    value == MP_POLICY_MIN_LATENCY)) {
    clt.mp_policy = value;
    return count;
    }
// distinguish "mi" and "min-latency" with length
    len = strnlen(buf, NAME_MAX);
    if (len && buf[len - 1] == '\n')
    len--;
    if (!strncasecmp(buf, "round-robin", 11) ||
    (len == 2 && !strncasecmp(buf, "rr", 2)))
    clt.mp_policy = MP_POLICY_RR;
    else if (!strncasecmp(buf, "min-inflight", 12) ||
    (len == 2 && !strncasecmp(buf, "mi", 2)))
    clt.mp_policy = MP_POLICY_MIN_INFLIGHT;
    else if (!strncasecmp(buf, "min-latency", 11) ||
    (len == 2 && !strncasecmp(buf, "ml", 2)))
    clt.mp_policy = MP_POLICY_MIN_LATENCY;
    else
    return -EINVAL;
    return count;
    }
    static DEVICE_ATTR_RW(mpath_policy);
    static ssize_t add_path_show(struct device *dev,
    struct device_attribute *attr, char *page)
    {
    return sysfs_emit(page,
    "Usage: echo [<source addr>@]<destination addr> > %s\n\n*addr ::= [ ip:<ipv4|ipv6> | gid:<gid> ]\n",
    attr.attr.name);
    }
    static ssize_t add_path_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct sockaddr_storage srcaddr, dstaddr;
    struct rtrs_addr addr = {
    .src = &srcaddr,
    .dst = &dstaddr
    };
    struct rtrs_clt_sess *clt;
    const char *nl;
    size_t len;
    int err;
    clt = container_of(dev, struct rtrs_clt_sess, dev);
    nl = strchr(buf, '\n');
    if (nl)
    len = nl - buf;
    else
    len = count;
    err = rtrs_addr_to_sockaddr(buf, len, clt.port, &addr);
    if (err)
    return -EINVAL;
    err = rtrs_clt_create_path_from_sysfs(clt, &addr);
    if (err)
    return err;
    return count;
    }
    static DEVICE_ATTR_RW(add_path);
    static ssize_t rtrs_clt_state_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *page)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    if (clt_path.state == RTRS_CLT_CONNECTED)
    return sysfs_emit(page, "connected\n");
    return sysfs_emit(page, "disconnected\n");
    }
    static struct kobj_attribute rtrs_clt_state_attr =
    __ATTR(state, 0444, rtrs_clt_state_show, core::ptr::null_mut());
    static ssize_t rtrs_clt_reconnect_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "Usage: echo 1 > %s\n", attr.attr.name);
    }
    static ssize_t rtrs_clt_reconnect_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct rtrs_clt_path *clt_path;
    int ret;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    if (!sysfs_streq(buf, "1")) {
    rtrs_err(clt_path.clt, "%s: unknown value: '%s'\n",
    attr.attr.name, buf);
    return -EINVAL;
    }
    ret = rtrs_clt_reconnect_from_sysfs(clt_path);
    if (ret)
    return ret;
    return count;
    }
    static struct kobj_attribute rtrs_clt_reconnect_attr =
    __ATTR(reconnect, 0644, rtrs_clt_reconnect_show,
    rtrs_clt_reconnect_store);
    static ssize_t rtrs_clt_disconnect_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "Usage: echo 1 > %s\n", attr.attr.name);
    }
    static ssize_t rtrs_clt_disconnect_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    if (!sysfs_streq(buf, "1")) {
    rtrs_err(clt_path.clt, "%s: unknown value: '%s'\n",
    attr.attr.name, buf);
    return -EINVAL;
    }
    rtrs_clt_close_conns(clt_path, true);
    return count;
    }
    static struct kobj_attribute rtrs_clt_disconnect_attr =
    __ATTR(disconnect, 0644, rtrs_clt_disconnect_show,
    rtrs_clt_disconnect_store);
    static ssize_t rtrs_clt_remove_path_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "Usage: echo 1 > %s\n", attr.attr.name);
    }
    static ssize_t rtrs_clt_remove_path_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct rtrs_clt_path *clt_path;
    int ret;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    if (!sysfs_streq(buf, "1")) {
    rtrs_err(clt_path.clt, "%s: unknown value: '%s'\n",
    attr.attr.name, buf);
    return -EINVAL;
    }
    ret = rtrs_clt_remove_path_from_sysfs(clt_path, &attr.attr);
    if (ret)
    return ret;
    return count;
    }
    static struct kobj_attribute rtrs_clt_remove_path_attr =
    __ATTR(remove_path, 0644, rtrs_clt_remove_path_show,
    rtrs_clt_remove_path_store);
    STAT_ATTR(struct rtrs_clt_stats, cpu_migration_from,
    rtrs_clt_stats_migration_from_cnt_to_str,
    rtrs_clt_reset_cpu_migr_stats);
    STAT_ATTR(struct rtrs_clt_stats, cpu_migration_to,
    rtrs_clt_stats_migration_to_cnt_to_str,
    rtrs_clt_reset_cpu_migr_stats);
    STAT_ATTR(struct rtrs_clt_stats, reconnects,
    rtrs_clt_stats_reconnects_to_str,
    rtrs_clt_reset_reconnects_stat);
    STAT_ATTR(struct rtrs_clt_stats, rdma,
    rtrs_clt_stats_rdma_to_str,
    rtrs_clt_reset_rdma_stats);
    STAT_ATTR(struct rtrs_clt_stats, reset_all,
    rtrs_clt_reset_all_help,
    rtrs_clt_reset_all_stats);
    static struct attribute *rtrs_clt_stats_attrs[] = {
    &cpu_migration_from_attr.attr,
    &cpu_migration_to_attr.attr,
    &reconnects_attr.attr,
    &rdma_attr.attr,
    &reset_all_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group rtrs_clt_stats_attr_group = {
    .attrs = rtrs_clt_stats_attrs,
    };
    static ssize_t rtrs_clt_hca_port_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, typeof(*clt_path), kobj);
    return sysfs_emit(page, "%u\n", clt_path.hca_port);
    }
    static struct kobj_attribute rtrs_clt_hca_port_attr =
    __ATTR(hca_port, 0444, rtrs_clt_hca_port_show, core::ptr::null_mut());
    static ssize_t rtrs_clt_hca_name_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    return sysfs_emit(page, "%s\n", clt_path.hca_name);
    }
    static struct kobj_attribute rtrs_clt_hca_name_attr =
    __ATTR(hca_name, 0444, rtrs_clt_hca_name_show, core::ptr::null_mut());
    static ssize_t rtrs_clt_cur_latency_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rtrs_clt_path *clt_path;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    return sysfs_emit(page, "%lld ns\n",
    ktime_to_ns(clt_path.s.hb_cur_latency));
    }
    static struct kobj_attribute rtrs_clt_cur_latency_attr =
    __ATTR(cur_latency, 0444, rtrs_clt_cur_latency_show, core::ptr::null_mut());
    static ssize_t rtrs_clt_src_addr_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rtrs_clt_path *clt_path;
    int len;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    len = sockaddr_to_str((struct sockaddr *)&clt_path.s.src_addr, page,
    PAGE_SIZE);
    len += sysfs_emit_at(page, len, "\n");
    return len;
    }
    static struct kobj_attribute rtrs_clt_src_addr_attr =
    __ATTR(src_addr, 0444, rtrs_clt_src_addr_show, core::ptr::null_mut());
    static ssize_t rtrs_clt_dst_addr_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rtrs_clt_path *clt_path;
    int len;
    clt_path = container_of(kobj, struct rtrs_clt_path, kobj);
    len = sockaddr_to_str((struct sockaddr *)&clt_path.s.dst_addr, page,
    PAGE_SIZE);
    len += sysfs_emit_at(page, len, "\n");
    return len;
    }
    static struct kobj_attribute rtrs_clt_dst_addr_attr =
    __ATTR(dst_addr, 0444, rtrs_clt_dst_addr_show, core::ptr::null_mut());
    static struct attribute *rtrs_clt_path_attrs[] = {
    &rtrs_clt_hca_name_attr.attr,
    &rtrs_clt_hca_port_attr.attr,
    &rtrs_clt_src_addr_attr.attr,
    &rtrs_clt_dst_addr_attr.attr,
    &rtrs_clt_state_attr.attr,
    &rtrs_clt_reconnect_attr.attr,
    &rtrs_clt_disconnect_attr.attr,
    &rtrs_clt_remove_path_attr.attr,
    &rtrs_clt_cur_latency_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group rtrs_clt_path_attr_group = {
    .attrs = rtrs_clt_path_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn rtrs_clt_create_path_files(clt_path: *mut rtrs_clt_path) -> c_int {
    int rtrs_clt_create_path_files(struct rtrs_clt_path *clt_path)
    {
    struct rtrs_clt_sess *clt = clt_path.clt;
    char str[NAME_MAX];
    int err;
    struct rtrs_addr path = {
    .src = &clt_path.s.src_addr,
    .dst = &clt_path.s.dst_addr,
    };
    rtrs_addr_to_str(&path, str, sizeof(str));
    err = kobject_init_and_add(&clt_path.kobj, &ktype_sess,
    clt.kobj_paths,
    "%s", str);
    if (err) {
    pr_err("kobject_init_and_add: %pe\n", ERR_PTR(err));
    kobject_put(&clt_path.kobj);
    return err;
    }
    err = sysfs_create_group(&clt_path.kobj, &rtrs_clt_path_attr_group);
    if (err) {
    pr_err("sysfs_create_group(): %pe\n", ERR_PTR(err));
    goto put_kobj;
    }
    err = kobject_init_and_add(&clt_path.stats.kobj_stats, &ktype_stats,
    &clt_path.kobj, "stats");
    if (err) {
    pr_err("kobject_init_and_add: %pe\n", ERR_PTR(err));
    kobject_put(&clt_path.stats.kobj_stats);
    goto remove_group;
    }
    err = sysfs_create_group(&clt_path.stats.kobj_stats,
    &rtrs_clt_stats_attr_group);
    if (err) {
    pr_err("failed to create stats sysfs group, err: %pe\n", ERR_PTR(err));
    goto put_kobj_stats;
    }
    return 0;
    put_kobj_stats:
    kobject_del(&clt_path.stats.kobj_stats);
    kobject_put(&clt_path.stats.kobj_stats);
    remove_group:
    sysfs_remove_group(&clt_path.kobj, &rtrs_clt_path_attr_group);
    put_kobj:
    kobject_del(&clt_path.kobj);
    kobject_put(&clt_path.kobj);
    return err;
    }
    void rtrs_clt_destroy_path_files(struct rtrs_clt_path *clt_path,
    const struct attribute *sysfs_self)
    {
    kobject_del(&clt_path.stats.kobj_stats);
    kobject_put(&clt_path.stats.kobj_stats);
    if (sysfs_self)
    sysfs_remove_file_self(&clt_path.kobj, sysfs_self);
    kobject_del(&clt_path.kobj);
    }
    static struct attribute *rtrs_clt_attrs[] = {
    &dev_attr_max_reconnect_attempts.attr,
    &dev_attr_mpath_policy.attr,
    &dev_attr_add_path.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group rtrs_clt_attr_group = {
    .attrs = rtrs_clt_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn rtrs_clt_create_sysfs_root_files(clt: *mut rtrs_clt_sess) -> c_int {
    int rtrs_clt_create_sysfs_root_files(struct rtrs_clt_sess *clt)
    {
    return sysfs_create_group(&clt.dev.kobj, &rtrs_clt_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn rtrs_clt_destroy_sysfs_root(clt: *mut rtrs_clt_sess) {
    void rtrs_clt_destroy_sysfs_root(struct rtrs_clt_sess *clt)
    {
    sysfs_remove_group(&clt.dev.kobj, &rtrs_clt_attr_group);
    if (clt.kobj_paths) {
    kobject_del(clt.kobj_paths);
    kobject_put(clt.kobj_paths);
    }
    }
