//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-powercap.c
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
// PowerNV OPAL Powercap interface
//
// Copyright 2017 IBM Corp.
//

    static DEFINE_MUTEX(powercap_mutex);
    static struct kobject *powercap_kobj;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_attr {
    pub handle: u32,
    pub attr: kobj_attribute,
}

    static struct pcap {
    struct attribute_group pg;
    struct powercap_attr *pattrs;
    } *pcaps;
    static ssize_t powercap_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    struct powercap_attr *pcap_attr = container_of(attr,
    struct powercap_attr, attr);
    struct opal_msg msg;
    u32 pcap;
    int ret, token;
    token = opal_async_get_token_interruptible();
    if (token < 0) {
    pr_devel("Failed to get token\n");
    return token;
    }
    ret = mutex_lock_interruptible(&powercap_mutex);
    if (ret)
    goto out_token;
    ret = opal_get_powercap(pcap_attr.handle, token, (u32 *)__pa(&pcap));
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
    ret = sysfs_emit(buf, "%u\n", be32_to_cpu(pcap));
    break;
    case OPAL_SUCCESS:
    ret = sysfs_emit(buf, "%u\n", be32_to_cpu(pcap));
    break;
    default:
    ret = opal_error_code(ret);
    }
    out:
    mutex_unlock(&powercap_mutex);
    out_token:
    opal_async_release_token(token);
    return ret;
    }
    static ssize_t powercap_store(struct kobject *kobj,
    struct kobj_attribute *attr, const char *buf,
    size_t count)
    {
    struct powercap_attr *pcap_attr = container_of(attr,
    struct powercap_attr, attr);
    struct opal_msg msg;
    u32 pcap;
    int ret, token;
    ret = kstrtoint(buf, 0, &pcap);
    if (ret)
    return ret;
    token = opal_async_get_token_interruptible();
    if (token < 0) {
    pr_devel("Failed to get token\n");
    return token;
    }
    ret = mutex_lock_interruptible(&powercap_mutex);
    if (ret)
    goto out_token;
    ret = opal_set_powercap(pcap_attr.handle, token, pcap);
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
    mutex_unlock(&powercap_mutex);
    out_token:
    opal_async_release_token(token);
    return ret;
    }
    static void __init powercap_add_attr(int handle, const char *name,
    struct powercap_attr *attr)
    {
    attr.handle = handle;
    sysfs_attr_init(&attr.attr.attr);
    attr.attr.attr.name = name;
    attr.attr.attr.mode = 0444;
    attr.attr.show = powercap_show;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_powercap_init() -> void __init {
    void __init opal_powercap_init(void)
    {
    struct device_node *powercap, *node;
    let mut i: c_int = 0;
    powercap = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opal-powercap");
    if (!powercap) {
    pr_devel("Powercap node not found\n");
    return;
    }
    pcaps = kzalloc_objs(*pcaps, of_get_child_count(powercap));
    if (!pcaps)
    goto out_put_powercap;
    powercap_kobj = kobject_create_and_add("powercap", opal_kobj);
    if (!powercap_kobj) {
    pr_warn("Failed to create powercap kobject\n");
    goto out_pcaps;
    }
    i = 0;
    for_each_child_of_node(powercap, node) {
    u32 cur, min, max;
    let mut j: c_int = 0;
    let mut has_cur: bool = false, has_min = false, has_max = false;
    if (!of_property_read_u32(node, "powercap-min", &min)) {
    j++;
    has_min = true;
    }
    if (!of_property_read_u32(node, "powercap-max", &max)) {
    j++;
    has_max = true;
    }
    if (!of_property_read_u32(node, "powercap-current", &cur)) {
    j++;
    has_cur = true;
    }
    pcaps[i].pattrs = kzalloc_objs(struct powercap_attr, j);
    if (!pcaps[i].pattrs)
    goto out_pcaps_pattrs;
    pcaps[i].pg.attrs = kzalloc_objs(struct attribute *, j + 1);
    if (!pcaps[i].pg.attrs) {
    kfree(pcaps[i].pattrs);
    goto out_pcaps_pattrs;
    }
    j = 0;
    pcaps[i].pg.name = kasprintf(GFP_KERNEL, "%pOFn", node);
    if (!pcaps[i].pg.name) {
    kfree(pcaps[i].pattrs);
    kfree(pcaps[i].pg.attrs);
    goto out_pcaps_pattrs;
    }
    if (has_min) {
    powercap_add_attr(min, "powercap-min",
    &pcaps[i].pattrs[j]);
    pcaps[i].pg.attrs[j] = &pcaps[i].pattrs[j].attr.attr;
    j++;
    }
    if (has_max) {
    powercap_add_attr(max, "powercap-max",
    &pcaps[i].pattrs[j]);
    pcaps[i].pg.attrs[j] = &pcaps[i].pattrs[j].attr.attr;
    j++;
    }
    if (has_cur) {
    powercap_add_attr(cur, "powercap-current",
    &pcaps[i].pattrs[j]);
    pcaps[i].pattrs[j].attr.attr.mode |= 0220;
    pcaps[i].pattrs[j].attr.store = powercap_store;
    pcaps[i].pg.attrs[j] = &pcaps[i].pattrs[j].attr.attr;
    j++;
    }
    if (sysfs_create_group(powercap_kobj, &pcaps[i].pg)) {
    pr_warn("Failed to create powercap attribute group %s\n",
    pcaps[i].pg.name);
    goto out_pcaps_pattrs;
    }
    i++;
    }
    of_node_put(powercap);
    return;
    out_pcaps_pattrs:
    while (--i >= 0) {
    kfree(pcaps[i].pattrs);
    kfree(pcaps[i].pg.attrs);
    kfree(pcaps[i].pg.name);
    }
    kobject_put(powercap_kobj);
    of_node_put(node);
    out_pcaps:
    kfree(pcaps);
    out_put_powercap:
    of_node_put(powercap);
    }
