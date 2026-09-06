//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/bgrt.c
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
// BGRT boot graphic support
// Authors: Matthew Garrett, Josh Triplett <josh@joshtriplett.org>
// Copyright 2012 Red Hat, Inc <mjg@redhat.com>
// Copyright 2012 Intel Corporation
//

    static void *bgrt_image;
    static struct kobject *bgrt_kobj;

    static ssize_t _name##_show(struct kobject *kobj,			\
    struct kobj_attribute *attr, char *buf)	\
    {									\
    return sysfs_emit(buf, "%d\n", bgrt_tab._member);		\
    }									\
    static struct kobj_attribute bgrt_attr_##_name = __ATTR_RO(_name)
    BGRT_SHOW(version, version);
    BGRT_SHOW(status, status);
    BGRT_SHOW(type, image_type);
    BGRT_SHOW(xoffset, image_offset_x);
    BGRT_SHOW(yoffset, image_offset_y);
    static __ro_after_init BIN_ATTR_SIMPLE_RO(image);
    static struct attribute *bgrt_attributes[] = {
    &bgrt_attr_version.attr,
    &bgrt_attr_status.attr,
    &bgrt_attr_type.attr,
    &bgrt_attr_xoffset.attr,
    &bgrt_attr_yoffset.attr,
    core::ptr::null_mut(),
    };
    static const struct bin_attribute *const bgrt_bin_attributes[] = {
    &bin_attr_image,
    core::ptr::null_mut(),
    };
    static const struct attribute_group bgrt_attribute_group = {
    .attrs = bgrt_attributes,
    .bin_attrs = bgrt_bin_attributes,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_parse_bgrt(table: *mut acpi_table_header) -> int __init {
    int __init acpi_parse_bgrt(struct acpi_table_header *table)
    {
    efi_bgrt_init(table);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bgrt_init() -> int __init {
    static int __init bgrt_init(void)
    {
    int ret;
    if (!bgrt_tab.image_address)
    return -ENODEV;
    bgrt_image = memremap(bgrt_tab.image_address, bgrt_image_size,
    MEMREMAP_WB);
    if (!bgrt_image) {
    pr_notice("Ignoring BGRT: failed to map image memory\n");
    return -ENOMEM;
    }
    bin_attr_image.private = bgrt_image;
    bin_attr_image.size = bgrt_image_size;
    bgrt_kobj = kobject_create_and_add("bgrt", acpi_kobj);
    if (!bgrt_kobj) {
    ret = -EINVAL;
    goto out_memmap;
    }
    ret = sysfs_create_group(bgrt_kobj, &bgrt_attribute_group);
    if (ret)
    goto out_kobject;
    return 0;
    out_kobject:
    kobject_put(bgrt_kobj);
    out_memmap:
    memunmap(bgrt_image);
    return ret;
    }
    device_initcall(bgrt_init);
