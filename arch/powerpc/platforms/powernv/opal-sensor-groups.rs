//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-sensor-groups.c
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
// PowerNV OPAL Sensor-groups interface
//
// Copyright 2017 IBM Corp.
//

    static DEFINE_MUTEX(sg_mutex);
    static struct kobject *sg_kobj;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_attr {
    pub handle: u32,
    pub attr: kobj_attribute,
}

    static struct sensor_group {
    char name[20];
    struct attribute_group sg;
    struct sg_attr *sgattrs;
    } *sgs;
#[no_mangle]
pub unsafe extern "C" fn sensor_group_enable(handle: u32, enable: bool) -> c_int {
    int sensor_group_enable(u32 handle, bool enable)
    {
    struct opal_msg msg;
    int token, ret;
    token = opal_async_get_token_interruptible();
    if (token < 0)
    return token;
    ret = opal_sensor_group_enable(handle, token, enable);
    if (ret == OPAL_ASYNC_COMPLETION) {
    ret = opal_async_wait_response(token, &msg);
    if (ret) {
    pr_devel("Failed to wait for the async response\n");
    ret = -EIO;
    goto out;
    }
    ret = opal_error_code(opal_get_async_rc(msg));
    } else {
    ret = opal_error_code(ret);
    }
    out:
    opal_async_release_token(token);
    return ret;
    }
    EXPORT_SYMBOL_GPL(sensor_group_enable);
    static ssize_t sg_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct sg_attr *sattr = container_of(attr, struct sg_attr, attr);
    struct opal_msg msg;
    u32 data;
    int ret, token;
    ret = kstrtoint(buf, 0, &data);
    if (ret)
    return ret;
    if (data != 1)
    return -EINVAL;
    token = opal_async_get_token_interruptible();
    if (token < 0) {
    pr_devel("Failed to get token\n");
    return token;
    }
    ret = mutex_lock_interruptible(&sg_mutex);
    if (ret)
    goto out_token;
    ret = opal_sensor_group_clear(sattr.handle, token);
    switch (ret) {
    case OPAL_ASYNC_COMPLETION:
    ret = opal_async_wait_response(token, &msg);
    if (ret) {
    pr_devel("Failed to wait for the async response\n");
    ret = -EIO;
    goto out;
    }
    ret = opal_error_code(opal_get_async_rc(msg));
    if (!ret)
    ret = count;
    break;
    case OPAL_SUCCESS:
    ret = count;
    break;
    default:
    ret = opal_error_code(ret);
    }
    out:
    mutex_unlock(&sg_mutex);
    out_token:
    opal_async_release_token(token);
    return ret;
    }
    static struct sg_ops_info {
    int opal_no;
    const char *attr_name;
    ssize_t (*store)(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count);
    } ops_info[] = {
    { OPAL_SENSOR_GROUP_CLEAR, "clear", sg_store },
    };
#[no_mangle]
unsafe extern "C" fn add_attr(handle: c_int, attr: *mut sg_attr, index: c_int) {
    static void add_attr(int handle, struct sg_attr *attr, int index)
    {
    attr.handle = handle;
    sysfs_attr_init(&attr.attr.attr);
    attr.attr.attr.name = ops_info[index].attr_name;
    attr.attr.attr.mode = 0220;
    attr.attr.store = ops_info[index].store;
    }
    static int __init add_attr_group(const __be32 *ops, int len, struct sensor_group *sg,
    u32 handle)
    {
    int i, j;
    let mut count: c_int = 0;
    for (i = 0; i < len; i++)
    for (j = 0; j < ARRAY_SIZE(ops_info); j++)
    if (be32_to_cpu(ops[i]) == ops_info[j].opal_no) {
    add_attr(handle, &sg.sgattrs[count], j);
    sg.sg.attrs[count] =
    &sg.sgattrs[count].attr.attr;
    count++;
    }
    return sysfs_create_group(sg_kobj, &sg.sg);
    }
#[no_mangle]
unsafe extern "C" fn get_nr_attrs(ops: *const __be32, len: c_int) -> int __init {
    static int __init get_nr_attrs(const __be32 *ops, int len)
    {
    int i, j;
    let mut nr_attrs: c_int = 0;
    for (i = 0; i < len; i++)
    for (j = 0; j < ARRAY_SIZE(ops_info); j++)
    if (be32_to_cpu(ops[i]) == ops_info[j].opal_no)
    nr_attrs++;
    return nr_attrs;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_sensor_groups_init() -> void __init {
    void __init opal_sensor_groups_init(void)
    {
    struct device_node *sg, *node;
    let mut i: c_int = 0;
    sg = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opal-sensor-group");
    if (!sg) {
    pr_devel("Sensor groups node not found\n");
    return;
    }
    sgs = kzalloc_objs(*sgs, of_get_child_count(sg));
    if (!sgs)
    goto out_sg_put;
    sg_kobj = kobject_create_and_add("sensor_groups", opal_kobj);
    if (!sg_kobj) {
    pr_warn("Failed to create sensor group kobject\n");
    goto out_sgs;
    }
    for_each_child_of_node(sg, node) {
    const __be32 *ops;
    u32 sgid, len, nr_attrs, chipid;
    ops = of_get_property(node, "ops", &len);
    if (!ops)
    continue;
    nr_attrs = get_nr_attrs(ops, len);
    if (!nr_attrs)
    continue;
    sgs[i].sgattrs = kzalloc_objs(*sgs[i].sgattrs, nr_attrs);
    if (!sgs[i].sgattrs)
    goto out_sgs_sgattrs;
    sgs[i].sg.attrs = kzalloc_objs(*sgs[i].sg.attrs, nr_attrs + 1);
    if (!sgs[i].sg.attrs) {
    kfree(sgs[i].sgattrs);
    goto out_sgs_sgattrs;
    }
    if (of_property_read_u32(node, "sensor-group-id", &sgid)) {
    pr_warn("sensor-group-id property not found\n");
    goto out_sgs_sgattrs;
    }
    if (!of_property_read_u32(node, "ibm,chip-id", &chipid))
    sprintf(sgs[i].name, "%pOFn%d", node, chipid);
    else
    sprintf(sgs[i].name, "%pOFn", node);
    sgs[i].sg.name = sgs[i].name;
    if (add_attr_group(ops, len, &sgs[i], sgid)) {
    pr_warn("Failed to create sensor attribute group %s\n",
    sgs[i].sg.name);
    goto out_sgs_sgattrs;
    }
    i++;
    }
    of_node_put(sg);
    return;
    out_sgs_sgattrs:
    while (--i >= 0) {
    kfree(sgs[i].sgattrs);
    kfree(sgs[i].sg.attrs);
    }
    kobject_put(sg_kobj);
    of_node_put(node);
    out_sgs:
    kfree(sgs);
    out_sg_put:
    of_node_put(sg);
    }
