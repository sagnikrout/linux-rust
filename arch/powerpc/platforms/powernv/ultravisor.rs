//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/ultravisor.c
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
// Ultravisor high level interfaces
//
// Copyright 2019, IBM Corporation.
//

    static struct kobject *ultravisor_kobj;
    int __init early_init_dt_scan_ultravisor(unsigned long node, const char *uname,
    int depth, void *data)
    {
    if (!of_flat_dt_is_compatible(node, "ibm,ultravisor"))
    return 0;
    powerpc_firmware_features |= FW_FEATURE_ULTRAVISOR;
    pr_debug("Ultravisor detected!\n");
    return 1;
    }
    static struct memcons *uv_memcons;
    static ssize_t uv_msglog_read(struct file *file, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *to,
    loff_t pos, size_t count)
    {
    return memcons_copy(uv_memcons, to, pos, count);
    }
    static struct bin_attribute uv_msglog_attr __ro_after_init = {
    .attr = {.name = "msglog", .mode = 0400},
    .read = uv_msglog_read
    };
#[no_mangle]
unsafe extern "C" fn uv_init() -> int __init {
    static int __init uv_init(void)
    {
    struct device_node *node;
    if (!firmware_has_feature(FW_FEATURE_ULTRAVISOR))
    return 0;
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,uv-firmware");
    if (!node)
    return -ENODEV;
    uv_memcons = memcons_init(node, "memcons");
    of_node_put(node);
    if (!uv_memcons)
    return -ENOENT;
    uv_msglog_attr.size = memcons_get_size(uv_memcons);
    ultravisor_kobj = kobject_create_and_add("ultravisor", firmware_kobj);
    if (!ultravisor_kobj)
    return -ENOMEM;
    return sysfs_create_bin_file(ultravisor_kobj, &uv_msglog_attr);
    }
    machine_subsys_initcall(powernv, uv_init);
