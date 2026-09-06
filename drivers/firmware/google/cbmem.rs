//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/google/cbmem.c
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
// cbmem.c
//
// Driver for exporting cbmem entries in sysfs.
//
// Copyright 2022 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cbmem_entry {
    pub mem_file_buf: *mut c_char,
    pub size: u32,
}

    static struct cbmem_entry *to_cbmem_entry(struct kobject *kobj)
    {
    return dev_get_drvdata(kobj_to_dev(kobj));
    }
    static ssize_t mem_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf, loff_t pos,
    size_t count)
    {
    struct cbmem_entry *entry = to_cbmem_entry(kobj);
    return memory_read_from_buffer(buf, count, &pos, entry.mem_file_buf,
    entry.size);
    }
    static ssize_t mem_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf, loff_t pos,
    size_t count)
    {
    struct cbmem_entry *entry = to_cbmem_entry(kobj);
    if (pos < 0 || pos >= entry.size)
    return -EINVAL;
    if (count > entry.size - pos)
    count = entry.size - pos;
    memcpy(entry.mem_file_buf + pos, buf, count);
    return count;
    }
    static const BIN_ATTR_ADMIN_RW(mem, 0);
    static ssize_t address_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct coreboot_device *cbdev = dev_to_coreboot_device(dev);
    return sysfs_emit(buf, "0x%llx\n", cbdev.cbmem_entry.address);
    }
    static DEVICE_ATTR_RO(address);
    static ssize_t size_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct coreboot_device *cbdev = dev_to_coreboot_device(dev);
    return sysfs_emit(buf, "0x%x\n", cbdev.cbmem_entry.entry_size);
    }
    static DEVICE_ATTR_RO(size);
    static struct attribute *attrs[] = {
    &dev_attr_address.attr,
    &dev_attr_size.attr,
    core::ptr::null_mut(),
    };
    static const struct bin_attribute *const bin_attrs[] = {
    &bin_attr_mem,
    core::ptr::null_mut(),
    };
    static const struct attribute_group cbmem_entry_group = {
    .attrs = attrs,
    .bin_attrs = bin_attrs,
    };
    static const struct attribute_group *dev_groups[] = {
    &cbmem_entry_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn cbmem_entry_probe(dev: *mut coreboot_device) -> c_int {
    static int cbmem_entry_probe(struct coreboot_device *dev)
    {
    struct cbmem_entry *entry;
    entry = devm_kzalloc(&dev.dev, sizeof(*entry), GFP_KERNEL);
    if (!entry)
    return -ENOMEM;
    dev_set_drvdata(&dev.dev, entry);
    entry.mem_file_buf = devm_memremap(&dev.dev, dev.cbmem_entry.address,
    dev.cbmem_entry.entry_size,
    MEMREMAP_WB);
    if (IS_ERR(entry.mem_file_buf))
    return PTR_ERR(entry.mem_file_buf);
    entry.size = dev.cbmem_entry.entry_size;
    return 0;
    }
    static const struct coreboot_device_id cbmem_ids[] = {
    { .tag = LB_TAG_CBMEM_ENTRY },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(coreboot, cbmem_ids);
    static struct coreboot_driver cbmem_entry_driver = {
    .probe = cbmem_entry_probe,
    .drv = {
    .name = "cbmem",
    .dev_groups = dev_groups,
    },
    .id_table = cbmem_ids,
    };
    module_coreboot_driver(cbmem_entry_driver);
    MODULE_AUTHOR("Jack Rosenthal <jrosenth@chromium.org>");
    MODULE_DESCRIPTION("Driver for exporting CBMEM entries in sysfs");
    MODULE_LICENSE("GPL");
