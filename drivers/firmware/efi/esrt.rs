//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/esrt.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// esrt.c
//
// This module exports EFI System Resource Table (ESRT) entries into userspace
// through the sysfs file system. The ESRT provides a read-only catalog of
// system components for which the system accepts firmware upgrades via UEFI's
// "Capsule Update" feature. This module allows userland utilities to evaluate
// what firmware updates can be applied to this system, and potentially arrange
// for those updates to occur.
//
// Data is currently found below /sys/firmware/efi/esrt/...
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_system_resource_entry_v1 {
    pub fw_class: efi_guid_t,
    pub fw_type: u32,
    pub fw_version: u32,
    pub lowest_supported_fw_version: u32,
    pub capsule_flags: u32,
    pub last_attempt_version: u32,
    pub last_attempt_status: u32,
}

//
// _count and _version are what they seem like.  _max is actually just
// accounting info for the firmware when creating the table; it should never
// have been exposed to us.  To wit, the spec says:
// The maximum number of resource array entries that can be within the
// table without reallocating the table, must not be zero.
// Since there's no guidance about what that means in terms of memory layout,
// it means nothing to us.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_system_resource_table {
    pub fw_resource_count: u32,
    pub fw_resource_count_max: u32,
    pub fw_resource_version: u64,
    pub entries: [u8; ],
}

    static phys_addr_t esrt_data;
    static size_t esrt_data_size;
    static struct efi_system_resource_table *esrt;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esre_entry {
    union {
    pub esre1: *mut efi_system_resource_entry_v1,
    pub esre: },
    pub kobj: kobject,
    pub list: list_head,
}

// global list of esre_entry.
    static LIST_HEAD(entry_list);
// entry attribute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esre_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct esre_entry entry, char,
}

    static struct esre_entry *to_entry(struct kobject *kobj)
    {
    return container_of(kobj, struct esre_entry, kobj);
    }
    static struct esre_attribute *to_attr(struct attribute *attr)
    {
    return container_of(attr, struct esre_attribute, attr);
    }
    static ssize_t esre_attr_show(struct kobject *kobj,
    struct attribute *_attr, char *buf)
    {
    struct esre_entry *entry = to_entry(kobj);
    struct esre_attribute *attr = to_attr(_attr);
    return attr.show(entry, buf);
    }
    static const struct sysfs_ops esre_attr_ops = {
    .show = esre_attr_show,
    };
// Generic ESRT Entry ("ESRE") support.
#[no_mangle]
unsafe extern "C" fn fw_class_show(entry: *mut esre_entry, buf: *mut c_char) -> isize {
    static ssize_t fw_class_show(struct esre_entry *entry, char *buf)
    {
    char *str = buf;
    efi_guid_to_str(&entry.esre.esre1.fw_class, str);
    str += strlen(str);
    str += sprintf(str, "\n");
    return str - buf;
    }
    let mut esre_fw_class: static struct esre_attribute = __ATTR_RO_MODE(fw_class, 0400);

    static ssize_t name##_show(struct esre_entry *entry, char *buf) \
    { \
    return sprintf(buf, fmt "\n", \
    le##size##_to_cpu(entry.esre.esre1.name)); \
    } \
    \
    static struct esre_attribute esre_##name = __ATTR_RO_MODE(name, 0400)
    esre_attr_decl(fw_type, 32, "%u");
    esre_attr_decl(fw_version, 32, "%u");
    esre_attr_decl(lowest_supported_fw_version, 32, "%u");
    esre_attr_decl(capsule_flags, 32, "0x%x");
    esre_attr_decl(last_attempt_version, 32, "%u");
    esre_attr_decl(last_attempt_status, 32, "%u");
    static struct attribute *esre1_attrs[] = {
    &esre_fw_class.attr,
    &esre_fw_type.attr,
    &esre_fw_version.attr,
    &esre_lowest_supported_fw_version.attr,
    &esre_capsule_flags.attr,
    &esre_last_attempt_version.attr,
    &esre_last_attempt_status.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(esre1);
#[no_mangle]
unsafe extern "C" fn esre_release(kobj: *mut kobject) {
    static void esre_release(struct kobject *kobj)
    {
    struct esre_entry *entry = to_entry(kobj);
    list_del(&entry.list);
    kfree(entry);
    }
    static const struct kobj_type esre1_ktype = {
    .release = esre_release,
    .sysfs_ops = &esre_attr_ops,
    .default_groups = esre1_groups,
    };
    static struct kobject *esrt_kobj;
    static struct kset *esrt_kset;
#[no_mangle]
unsafe extern "C" fn esre_create_sysfs_entry(esre: *mut c_void, entry_num: c_int) -> c_int {
    static int esre_create_sysfs_entry(void *esre, int entry_num)
    {
    struct esre_entry *entry;
    entry = kzalloc_obj(*entry);
    if (!entry)
    return -ENOMEM;
    entry.kobj.kset = esrt_kset;
    if (esrt.fw_resource_version == 1) {
    let mut rc: c_int = 0;
    entry.esre.esre1 = esre;
    rc = kobject_init_and_add(&entry.kobj, &esre1_ktype, core::ptr::null_mut(),
    "entry%d", entry_num);
    if (rc) {
    kobject_put(&entry.kobj);
    return rc;
    }
    }
    list_add_tail(&entry.list, &entry_list);
    return 0;
    }
// support for displaying ESRT fields at the top level

    static ssize_t name##_show(struct kobject *kobj, \
    struct kobj_attribute *attr, char *buf)\
    { \
    return sprintf(buf, fmt "\n", le##size##_to_cpu(esrt.name)); \
    } \
    \
    static struct kobj_attribute esrt_##name = __ATTR_RO_MODE(name, 0400)
    esrt_attr_decl(fw_resource_count, 32, "%u");
    esrt_attr_decl(fw_resource_count_max, 32, "%u");
    esrt_attr_decl(fw_resource_version, 64, "%llu");
    static struct attribute *esrt_attrs[] = {
    &esrt_fw_resource_count.attr,
    &esrt_fw_resource_count_max.attr,
    &esrt_fw_resource_version.attr,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn esrt_table_exists() -> c_int {
    static inline int esrt_table_exists(void)
    {
    if (!efi_enabled(EFI_CONFIG_TABLES))
    return 0;
    if (efi.esrt == EFI_INVALID_TABLE_ADDR)
    return 0;
    return 1;
    }
    static umode_t esrt_attr_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    if (!esrt_table_exists())
    return 0;
    return attr.mode;
    }
    static const struct attribute_group esrt_attr_group = {
    .attrs = esrt_attrs,
    .is_visible = esrt_attr_is_visible,
    };
//
// remap the table, validate it, mark it reserved and unmap it.
//
#[no_mangle]
pub unsafe extern "C" fn efi_esrt_init() -> void __init {
    void __init efi_esrt_init(void)
    {
    void *va;
    struct efi_system_resource_table tmpesrt;
    size_t size, max, entry_size, entries_size;
    efi_memory_desc_t md;
    int rc;
    phys_addr_t end;
    if (!efi_enabled(EFI_MEMMAP) && !efi_enabled(EFI_PARAVIRT))
    return;
    pr_debug("esrt-init: loading.\n");
    if (!esrt_table_exists())
    return;
    rc = efi_mem_desc_lookup(efi.esrt, &md);
    if (rc < 0 ||
    (!(md.attribute & EFI_MEMORY_RUNTIME) &&
    md.type != EFI_BOOT_SERVICES_DATA &&
    md.type != EFI_RUNTIME_SERVICES_DATA &&
    md.type != EFI_ACPI_RECLAIM_MEMORY &&
    md.type != EFI_ACPI_MEMORY_NVS)) {
    pr_warn("ESRT header is not in the memory map.\n");
    return;
    }
    max = efi_mem_desc_end(&md) - efi.esrt;
    size = sizeof(*esrt);
    if (max < size) {
    pr_err("ESRT header doesn't fit on single memory map entry. (size: %zu max: %zu)\n",
    size, max);
    return;
    }
    va = early_memremap(efi.esrt, size);
    if (!va) {
    pr_err("early_memremap(%p, %zu) failed.\n", (void *)efi.esrt,
    size);
    return;
    }
    memcpy(&tmpesrt, va, sizeof(tmpesrt));
    early_memunmap(va, size);
    if (tmpesrt.fw_resource_version != 1) {
    pr_err("Unsupported ESRT version %lld.\n",
    tmpesrt.fw_resource_version);
    return;
    }
    entry_size = sizeof(struct efi_system_resource_entry_v1);
    if (tmpesrt.fw_resource_count > 0 && max - size < entry_size) {
    pr_err("ESRT memory map entry can only hold the header. (max: %zu size: %zu)\n",
    max - size, entry_size);
    return;
    }
//
// The format doesn't really give us any boundary to test here,
// so I'm making up 128 as the max number of individually updatable
// components we support.
// 128 should be pretty excessive, but there's still some chance
// somebody will do that someday and we'll need to raise this.
//
    if (tmpesrt.fw_resource_count > 128) {
    pr_err("ESRT says fw_resource_count has very large value %d.\n",
    tmpesrt.fw_resource_count);
    return;
    }
//
// We know it can't be larger than N * sizeof() here, and N is limited
// by the previous test to a small number, so there's no overflow.
//
    entries_size = tmpesrt.fw_resource_count * entry_size;
    if (max < size + entries_size) {
    pr_err("ESRT does not fit on single memory map entry (size: %zu max: %zu)\n",
    size, max);
    return;
    }
    size += entries_size;
    esrt_data = (phys_addr_t)efi.esrt;
    esrt_data_size = size;
    end = esrt_data + size;
    pr_info("Reserving ESRT space from %pa to %pa.\n", &esrt_data, &end);
    if (md.type == EFI_BOOT_SERVICES_DATA)
    efi_mem_reserve(esrt_data, esrt_data_size);
    pr_debug("esrt-init: loaded.\n");
    }
#[no_mangle]
unsafe extern "C" fn register_entries() -> int __init {
    static int __init register_entries(void)
    {
    struct efi_system_resource_entry_v1 *v1_entries = (void *)esrt.entries;
    int i, rc;
    if (!esrt_table_exists())
    return 0;
    for (i = 0; i < le32_to_cpu(esrt.fw_resource_count); i++) {
    void *esre = core::ptr::null_mut();
    if (esrt.fw_resource_version == 1) {
    esre = &v1_entries[i];
    } else {
    pr_err("Unsupported ESRT version %lld.\n",
    esrt.fw_resource_version);
    return -EINVAL;
    }
    rc = esre_create_sysfs_entry(esre, i);
    if (rc < 0) {
    pr_err("ESRT entry creation failed with error %d.\n",
    rc);
    return rc;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_entry_list() {
    static void cleanup_entry_list(void)
    {
    struct esre_entry *entry, *next;
    list_for_each_entry_safe(entry, next, &entry_list, list) {
    kobject_put(&entry.kobj);
    }
    }
#[no_mangle]
unsafe extern "C" fn esrt_sysfs_init() -> int __init {
    static int __init esrt_sysfs_init(void)
    {
    int error;
    pr_debug("esrt-sysfs: loading.\n");
    if (!esrt_data || !esrt_data_size)
    return -ENOSYS;
    esrt = memremap(esrt_data, esrt_data_size, MEMREMAP_WB);
    if (!esrt) {
    pr_err("memremap(%pa, %zu) failed.\n", &esrt_data,
    esrt_data_size);
    return -ENOMEM;
    }
    esrt_kobj = kobject_create_and_add("esrt", efi_kobj);
    if (!esrt_kobj) {
    pr_err("Firmware table registration failed.\n");
    error = -ENOMEM;
    goto err;
    }
    error = sysfs_create_group(esrt_kobj, &esrt_attr_group);
    if (error) {
    pr_err("Sysfs attribute export failed with error %d.\n",
    error);
    goto err_remove_esrt;
    }
    esrt_kset = kset_create_and_add("entries", core::ptr::null_mut(), esrt_kobj);
    if (!esrt_kset) {
    pr_err("kset creation failed.\n");
    error = -ENOMEM;
    goto err_remove_group;
    }
    error = register_entries();
    if (error)
    goto err_cleanup_list;
    pr_debug("esrt-sysfs: loaded.\n");
    return 0;
    err_cleanup_list:
    cleanup_entry_list();
    kset_unregister(esrt_kset);
    err_remove_group:
    sysfs_remove_group(esrt_kobj, &esrt_attr_group);
    err_remove_esrt:
    kobject_put(esrt_kobj);
    err:
    memunmap(esrt);
    esrt = core::ptr::null_mut();
    return error;
    }
    device_initcall(esrt_sysfs_init);
//
    MODULE_AUTHOR("Peter Jones <pjones@redhat.com>");
    MODULE_DESCRIPTION("EFI System Resource Table support");
    MODULE_LICENSE("GPL");
//
