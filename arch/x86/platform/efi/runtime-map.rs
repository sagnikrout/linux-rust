//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/efi/runtime-map.c
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
// Copyright (C) 2013 Red Hat, Inc., Dave Young <dyoung@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_runtime_map_entry {
    pub md: efi_memory_desc_t,
    pub /: *mut *mut kobject kobj; / kobject for each entry,
}

    static struct efi_runtime_map_entry **map_entries;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct efi_runtime_map_entry entry, char,
}

    static inline struct map_attribute *to_map_attr(struct attribute *attr)
    {
    return container_of(attr, struct map_attribute, attr);
    }
#[no_mangle]
unsafe extern "C" fn type_show(entry: *mut efi_runtime_map_entry, buf: *mut c_char) -> isize {
    static ssize_t type_show(struct efi_runtime_map_entry *entry, char *buf)
    {
    return snprintf(buf, PAGE_SIZE, "0x%x\n", entry.md.type);
    }

    static ssize_t name##_show(struct efi_runtime_map_entry *entry, char *buf) \
    { \
    return snprintf(buf, PAGE_SIZE, "0x%llx\n", EFI_RUNTIME_FIELD(name)); \
    }
    EFI_RUNTIME_U64_ATTR_SHOW(phys_addr);
    EFI_RUNTIME_U64_ATTR_SHOW(virt_addr);
    EFI_RUNTIME_U64_ATTR_SHOW(num_pages);
    EFI_RUNTIME_U64_ATTR_SHOW(attribute);
    static inline struct efi_runtime_map_entry *to_map_entry(struct kobject *kobj)
    {
    return container_of(kobj, struct efi_runtime_map_entry, kobj);
    }
    static ssize_t map_attr_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    struct efi_runtime_map_entry *entry = to_map_entry(kobj);
    struct map_attribute *map_attr = to_map_attr(attr);
    return map_attr.show(entry, buf);
    }
    let mut map_type_attr: static struct map_attribute = __ATTR_RO_MODE(type, 0400);
    let mut map_phys_addr_attr: static struct map_attribute = __ATTR_RO_MODE(phys_addr, 0400);
    let mut map_virt_addr_attr: static struct map_attribute = __ATTR_RO_MODE(virt_addr, 0400);
    let mut map_num_pages_attr: static struct map_attribute = __ATTR_RO_MODE(num_pages, 0400);
    let mut map_attribute_attr: static struct map_attribute = __ATTR_RO_MODE(attribute, 0400);
//
// These are default attributes that are added for every memmap entry.
//
    static struct attribute *def_attrs[] = {
    &map_type_attr.attr,
    &map_phys_addr_attr.attr,
    &map_virt_addr_attr.attr,
    &map_num_pages_attr.attr,
    &map_attribute_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(def);
    static const struct sysfs_ops map_attr_ops = {
    .show = map_attr_show,
    };
#[no_mangle]
unsafe extern "C" fn map_release(kobj: *mut kobject) {
    static void map_release(struct kobject *kobj)
    {
    struct efi_runtime_map_entry *entry;
    entry = to_map_entry(kobj);
    kfree(entry);
    }
    static const struct kobj_type __refconst map_ktype = {
    .sysfs_ops	= &map_attr_ops,
    .default_groups	= def_groups,
    .release	= map_release,
    };
    static struct kset *map_kset;
    static struct efi_runtime_map_entry *
    add_sysfs_runtime_map_entry(struct kobject *kobj, int nr,
    efi_memory_desc_t *md)
    {
    int ret;
    struct efi_runtime_map_entry *entry;
    if (!map_kset) {
    map_kset = kset_create_and_add("runtime-map", core::ptr::null_mut(), kobj);
    if (!map_kset)
    return ERR_PTR(-ENOMEM);
    }
    entry = kzalloc_obj(*entry);
    if (!entry) {
    kset_unregister(map_kset);
    map_kset = core::ptr::null_mut();
    return ERR_PTR(-ENOMEM);
    }
    memcpy(&entry.md, md, sizeof(efi_memory_desc_t));
    kobject_init(&entry.kobj, &map_ktype);
    entry.kobj.kset = map_kset;
    ret = kobject_add(&entry.kobj, core::ptr::null_mut(), "%d", nr);
    if (ret) {
    kobject_put(&entry.kobj);
    kset_unregister(map_kset);
    map_kset = core::ptr::null_mut();
    return ERR_PTR(ret);
    }
    return entry;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_get_runtime_map_size() -> c_int {
    int efi_get_runtime_map_size(void)
    {
    return efi.memmap.nr_map * efi.memmap.desc_size;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_get_runtime_map_desc_size() -> c_int {
    int efi_get_runtime_map_desc_size(void)
    {
    return efi.memmap.desc_size;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_runtime_map_copy(buf: *mut c_void, bufsz: usize) -> c_int {
    int efi_runtime_map_copy(void *buf, size_t bufsz)
    {
    let mut sz: usize = efi_get_runtime_map_size();
    if (sz > bufsz)
    sz = bufsz;
    memcpy(buf, efi.memmap.map, sz);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efi_runtime_map_init() -> int __init {
    static int __init efi_runtime_map_init(void)
    {
    int i, j, ret = 0;
    struct efi_runtime_map_entry *entry;
    efi_memory_desc_t *md;
    if (!efi_enabled(EFI_MEMMAP) || !efi_kobj)
    return 0;
    map_entries = kzalloc_objs(entry, efi.memmap.nr_map);
    if (!map_entries) {
    ret = -ENOMEM;
    goto out;
    }
    i = 0;
    for_each_efi_memory_desc(md) {
    entry = add_sysfs_runtime_map_entry(efi_kobj, i, md);
    if (IS_ERR(entry)) {
    ret = PTR_ERR(entry);
    goto out_add_entry;
    }
// (map_entries + i++) = entry;
    }
    return 0;
    out_add_entry:
    for (j = i - 1; j >= 0; j--) {
    entry = *(map_entries + j);
    kobject_put(&entry.kobj);
    }
    out:
    return ret;
    }
    subsys_initcall_sync(efi_runtime_map_init);
