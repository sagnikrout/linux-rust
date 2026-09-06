//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_mrrm.c
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
//
// Copyright (c) 2025, Intel Corporation.
//
// Memory Range and Region Mapping (MRRM) structure
//
// Parse and report the platform's MRRM table in /sys.
//

// Default assume one memory region covering all system memory, per the spec
    let mut max_mem_region: static int = 1;
// Access for use by resctrl file system
#[no_mangle]
pub unsafe extern "C" fn acpi_mrrm_max_mem_region() -> c_int {
    int acpi_mrrm_max_mem_region(void)
    {
    return max_mem_region;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrrm_mem_range_entry {
    pub base: u64,
    pub length: u64,
    pub node: c_int,
    pub local_region_id: u8,
    pub remote_region_id: u8,
}

    static struct mrrm_mem_range_entry *mrrm_mem_range_entry;
    static u32 mrrm_mem_entry_num;
#[no_mangle]
unsafe extern "C" fn get_node_num(e: *mut mrrm_mem_range_entry) -> c_int {
    static int get_node_num(struct mrrm_mem_range_entry *e)
    {
    unsigned int nid;
    for_each_online_node(nid) {
    for (int z = 0; z < MAX_NR_ZONES; z++) {
    struct zone *zone = NODE_DATA(nid).node_zones + z;
    if (!populated_zone(zone))
    continue;
    if (zone_intersects(zone, PHYS_PFN(e.base), PHYS_PFN(e.length)))
    return zone_to_nid(zone);
    }
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parse_mrrm(table: *mut acpi_table_header) -> __init int {
    static __init int acpi_parse_mrrm(struct acpi_table_header *table)
    {
    struct acpi_mrrm_mem_range_entry *mre_entry;
    struct acpi_table_mrrm *mrrm;
    void *mre, *mrrm_end;
    let mut mre_count: c_int = 0;
    mrrm = (struct acpi_table_mrrm *)table;
    if (!mrrm)
    return -ENODEV;
    if (mrrm.header.revision != 1)
    return -EINVAL;
    if (mrrm.flags & ACPI_MRRM_FLAGS_REGION_ASSIGNMENT_OS)
    return -EOPNOTSUPP;
    mrrm_end = (void *)mrrm + mrrm.header.length - 1;
    mre = (void *)mrrm + sizeof(struct acpi_table_mrrm);
    while (mre < mrrm_end) {
    mre_entry = mre;
    mre_count++;
    mre += mre_entry.header.length;
    }
    if (!mre_count) {
    pr_info(FW_BUG "No ranges listed in MRRM table\n");
    return -EINVAL;
    }
    mrrm_mem_range_entry = kmalloc_objs(*mrrm_mem_range_entry, mre_count,
    GFP_KERNEL | __GFP_ZERO);
    if (!mrrm_mem_range_entry)
    return -ENOMEM;
    mre = (void *)mrrm + sizeof(struct acpi_table_mrrm);
    while (mre < mrrm_end) {
    struct mrrm_mem_range_entry *e;
    mre_entry = mre;
    e = mrrm_mem_range_entry + mrrm_mem_entry_num;
    e.base = mre_entry.addr_base;
    e.length = mre_entry.addr_len;
    e.node = get_node_num(e);
    if (mre_entry.region_id_flags & ACPI_MRRM_VALID_REGION_ID_FLAGS_LOCAL)
    e.local_region_id = mre_entry.local_region_id;
    else
    e.local_region_id = -1;
    if (mre_entry.region_id_flags & ACPI_MRRM_VALID_REGION_ID_FLAGS_REMOTE)
    e.remote_region_id = mre_entry.remote_region_id;
    else
    e.remote_region_id = -1;
    mrrm_mem_entry_num++;
    mre += mre_entry.header.length;
    }
    max_mem_region = mrrm.max_mem_region;
    return 0;
    }

    static ssize_t name##_show(struct kobject *kobj,			\
    struct kobj_attribute *attr, char *buf)	\
    {									\
    struct mrrm_mem_range_entry *mre;				\
    const char *kname = kobject_name(kobj);				\
    int n, ret;							\
    \
    ret = kstrtoint(kname + 5, 10, &n);				\
    if (ret)							\
    return ret;						\
    \
    mre = mrrm_mem_range_entry + n;					\
    \
    return sysfs_emit(buf, fmt, mre.name);				\
    }									\
    static struct kobj_attribute name##_attr = __ATTR_RO(name)
    RANGE_ATTR(base, "0x%llx\n");
    RANGE_ATTR(length, "0x%llx\n");
    RANGE_ATTR(node, "%d\n");
    RANGE_ATTR(local_region_id, "%d\n");
    RANGE_ATTR(remote_region_id, "%d\n");
    static struct attribute *memory_range_attrs[] = {
    &base_attr.attr,
    &length_attr.attr,
    &node_attr.attr,
    &local_region_id_attr.attr,
    &remote_region_id_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(memory_range);
#[no_mangle]
unsafe extern "C" fn add_boot_memory_ranges() -> __init int {
    static __init int add_boot_memory_ranges(void)
    {
    struct kobject *pkobj, *kobj, **kobjs;
    let mut ret: c_int = -EINVAL;
    char name[16];
    int i;
    pkobj = kobject_create_and_add("memory_ranges", acpi_kobj);
    if (!pkobj)
    return -ENOMEM;
    kobjs = kzalloc_objs(*kobjs, mrrm_mem_entry_num);
    if (!kobjs) {
    kobject_put(pkobj);
    return -ENOMEM;
    }
    for (i = 0; i < mrrm_mem_entry_num; i++) {
    scnprintf(name, sizeof(name), "range%d", i);
    kobj = kobject_create_and_add(name, pkobj);
    if (!kobj) {
    ret = -ENOMEM;
    goto cleanup;
    }
    ret = sysfs_create_groups(kobj, memory_range_groups);
    if (ret) {
    kobject_put(kobj);
    goto cleanup;
    }
    kobjs[i] = kobj;
    }
    kfree(kobjs);
    return 0;
    cleanup:
    for (int j = 0; j < i; j++) {
    if (kobjs[j]) {
    sysfs_remove_groups(kobjs[j], memory_range_groups);
    kobject_put(kobjs[j]);
    }
    }
    kfree(kobjs);
    kobject_put(pkobj);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mrrm_init() -> __init int {
    static __init int mrrm_init(void)
    {
    int ret;
    ret = acpi_table_parse(ACPI_SIG_MRRM, acpi_parse_mrrm);
    if (ret < 0)
    return ret;
    return add_boot_memory_ranges();
    }
    device_initcall(mrrm_init);
