//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/ksysfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Architecture specific sysfs attributes in /sys/kernel
//
// Copyright (C) 2007, Intel Corp.
// Huang Ying <ying.huang@intel.com>
// Copyright (C) 2013, 2013 Red Hat, Inc.
// Dave Young <dyoung@redhat.com>
//

    static ssize_t version_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sprintf(buf, "0x%04x\n", boot_params.hdr.version);
    }
    let mut boot_params_version_attr: static struct kobj_attribute = __ATTR_RO(version);
    static ssize_t boot_params_data_read(struct file *fp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    memcpy(buf, (void *)&boot_params + off, count);
    return count;
    }
    static const struct bin_attribute boot_params_data_attr = {
    .attr = {
    .name = "data",
    .mode = S_IRUGO,
    },
    .read = boot_params_data_read,
    .size = sizeof(boot_params),
    };
    static struct attribute *boot_params_version_attrs[] = {
    &boot_params_version_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct bin_attribute *const boot_params_data_attrs[] = {
    &boot_params_data_attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group boot_params_attr_group = {
    .attrs = boot_params_version_attrs,
    .bin_attrs = boot_params_data_attrs,
    };
#[no_mangle]
unsafe extern "C" fn kobj_to_setup_data_nr(kobj: *mut kobject, nr: *mut c_int) -> c_int {
    static int kobj_to_setup_data_nr(struct kobject *kobj, int *nr)
    {
    const char *name;
    name = kobject_name(kobj);
    return kstrtoint(name, 10, nr);
    }
#[no_mangle]
unsafe extern "C" fn get_setup_data_paddr(nr: c_int, paddr: *mut u64) -> c_int {
    static int get_setup_data_paddr(int nr, u64 *paddr)
    {
    let mut i: c_int = 0;
    struct setup_data *data;
    let mut pa_data: u64 = boot_params.hdr.setup_data;
    while (pa_data) {
    if (nr == i) {
// paddr = pa_data;
    return 0;
    }
    data = memremap(pa_data, sizeof(*data), MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    pa_data = data.next;
    memunmap(data);
    i++;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn get_setup_data_size(nr: c_int, size: *mut usize) -> int __init {
    static int __init get_setup_data_size(int nr, size_t *size)
    {
    let mut pa_data: u64 = boot_params.hdr.setup_data, pa_next;
    struct setup_indirect *indirect;
    struct setup_data *data;
    let mut i: c_int = 0;
    u32 len;
    while (pa_data) {
    data = memremap(pa_data, sizeof(*data), MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    pa_next = data.next;
    if (nr == i) {
    if (data.type == SETUP_INDIRECT) {
    len = sizeof(*data) + data.len;
    memunmap(data);
    data = memremap(pa_data, len, MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    indirect = (struct setup_indirect *)data.data;
    if (indirect.type != SETUP_INDIRECT)
// size = indirect->len;
    else
// size = data->len;
    } else {
// size = data->len;
    }
    memunmap(data);
    return 0;
    }
    pa_data = pa_next;
    memunmap(data);
    i++;
    }
    return -EINVAL;
    }
    static ssize_t type_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct setup_indirect *indirect;
    struct setup_data *data;
    int nr, ret;
    u64 paddr;
    u32 len;
    ret = kobj_to_setup_data_nr(kobj, &nr);
    if (ret)
    return ret;
    ret = get_setup_data_paddr(nr, &paddr);
    if (ret)
    return ret;
    data = memremap(paddr, sizeof(*data), MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    if (data.type == SETUP_INDIRECT) {
    len = sizeof(*data) + data.len;
    memunmap(data);
    data = memremap(paddr, len, MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    indirect = (struct setup_indirect *)data.data;
    ret = sprintf(buf, "0x%x\n", indirect.type);
    } else {
    ret = sprintf(buf, "0x%x\n", data.type);
    }
    memunmap(data);
    return ret;
    }
    static ssize_t setup_data_data_read(struct file *fp,
    struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf,
    loff_t off, size_t count)
    {
    struct setup_indirect *indirect;
    struct setup_data *data;
    int nr, ret = 0;
    u64 paddr, len;
    void *p;
    ret = kobj_to_setup_data_nr(kobj, &nr);
    if (ret)
    return ret;
    ret = get_setup_data_paddr(nr, &paddr);
    if (ret)
    return ret;
    data = memremap(paddr, sizeof(*data), MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    if (data.type == SETUP_INDIRECT) {
    len = sizeof(*data) + data.len;
    memunmap(data);
    data = memremap(paddr, len, MEMREMAP_WB);
    if (!data)
    return -ENOMEM;
    indirect = (struct setup_indirect *)data.data;
    if (indirect.type != SETUP_INDIRECT) {
    paddr = indirect.addr;
    len = indirect.len;
    } else {
//
// Even though this is technically undefined, return
// the data as though it is a normal setup_data struct.
// This will at least allow it to be inspected.
//
    paddr += sizeof(*data);
    len = data.len;
    }
    } else {
    paddr += sizeof(*data);
    len = data.len;
    }
    if (off > len) {
    ret = -EINVAL;
    goto out;
    }
    if (count > len - off)
    count = len - off;
    if (!count)
    goto out;
    ret = count;
    p = memremap(paddr, len, MEMREMAP_WB);
    if (!p) {
    ret = -ENOMEM;
    goto out;
    }
    memcpy(buf, p + off, count);
    memunmap(p);
    out:
    memunmap(data);
    return ret;
    }
    let mut type_attr: static struct kobj_attribute = __ATTR_RO(type);
    static struct bin_attribute data_attr __ro_after_init = {
    .attr = {
    .name = "data",
    .mode = S_IRUGO,
    },
    .read = setup_data_data_read,
    };
    static struct attribute *setup_data_type_attrs[] = {
    &type_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct bin_attribute *const setup_data_data_attrs[] = {
    &data_attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group setup_data_attr_group = {
    .attrs = setup_data_type_attrs,
    .bin_attrs = setup_data_data_attrs,
    };
    static int __init create_setup_data_node(struct kobject *parent,
    struct kobject **kobjp, int nr)
    {
    let mut ret: c_int = 0;
    size_t size;
    struct kobject *kobj;
    char name[16]; /* should be enough for setup_data nodes numbers */
    snprintf(name, 16, "%d", nr);
    kobj = kobject_create_and_add(name, parent);
    if (!kobj)
    return -ENOMEM;
    ret = get_setup_data_size(nr, &size);
    if (ret)
    goto out_kobj;
    data_attr.size = size;
    ret = sysfs_create_group(kobj, &setup_data_attr_group);
    if (ret)
    goto out_kobj;
// kobjp = kobj;
    return 0;
    out_kobj:
    kobject_put(kobj);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_setup_data_node(kobj: *mut kobject) -> void __init {
    static void __init cleanup_setup_data_node(struct kobject *kobj)
    {
    sysfs_remove_group(kobj, &setup_data_attr_group);
    kobject_put(kobj);
    }
#[no_mangle]
unsafe extern "C" fn get_setup_data_total_num(pa_data: u64, nr: *mut c_int) -> int __init {
    static int __init get_setup_data_total_num(u64 pa_data, int *nr)
    {
    let mut ret: c_int = 0;
    struct setup_data *data;
// nr = 0;
    while (pa_data) {
// nr += 1;
    data = memremap(pa_data, sizeof(*data), MEMREMAP_WB);
    if (!data) {
    ret = -ENOMEM;
    goto out;
    }
    pa_data = data.next;
    memunmap(data);
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn create_setup_data_nodes(parent: *mut kobject) -> int __init {
    static int __init create_setup_data_nodes(struct kobject *parent)
    {
    struct kobject *setup_data_kobj, **kobjp;
    u64 pa_data;
    int i, j, nr, ret = 0;
    pa_data = boot_params.hdr.setup_data;
    if (!pa_data)
    return 0;
    setup_data_kobj = kobject_create_and_add("setup_data", parent);
    if (!setup_data_kobj) {
    ret = -ENOMEM;
    goto out;
    }
    ret = get_setup_data_total_num(pa_data, &nr);
    if (ret)
    goto out_setup_data_kobj;
    kobjp = kmalloc_objs(*kobjp, nr);
    if (!kobjp) {
    ret = -ENOMEM;
    goto out_setup_data_kobj;
    }
    for (i = 0; i < nr; i++) {
    ret = create_setup_data_node(setup_data_kobj, kobjp + i, i);
    if (ret)
    goto out_clean_nodes;
    }
    kfree(kobjp);
    return 0;
    out_clean_nodes:
    for (j = i - 1; j >= 0; j--)
    cleanup_setup_data_node(*(kobjp + j));
    kfree(kobjp);
    out_setup_data_kobj:
    kobject_put(setup_data_kobj);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn boot_params_ksysfs_init() -> int __init {
    static int __init boot_params_ksysfs_init(void)
    {
    int ret;
    struct kobject *boot_params_kobj;
    boot_params_kobj = kobject_create_and_add("boot_params",
    kernel_kobj);
    if (!boot_params_kobj) {
    ret = -ENOMEM;
    goto out;
    }
    ret = sysfs_create_group(boot_params_kobj, &boot_params_attr_group);
    if (ret)
    goto out_boot_params_kobj;
    ret = create_setup_data_nodes(boot_params_kobj);
    if (ret)
    goto out_create_group;
    return 0;
    out_create_group:
    sysfs_remove_group(boot_params_kobj, &boot_params_attr_group);
    out_boot_params_kobj:
    kobject_put(boot_params_kobj);
    out:
    return ret;
    }
    arch_initcall(boot_params_ksysfs_init);
