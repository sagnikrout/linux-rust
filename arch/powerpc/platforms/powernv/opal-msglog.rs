//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-msglog.c
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
// PowerNV OPAL in-memory console interface
//
// Copyright 2014 IBM Corp.
//

// OPAL in-memory console. Defined in OPAL source at core/console.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcons {
    pub magic: __be64,
pub const MEMCONS_MAGIC: c_uint = 0x6630696567726173L;
    pub obuf_phys: __be64,
    pub ibuf_phys: __be64,
    pub obuf_size: __be32,
    pub ibuf_size: __be32,
    pub out_pos: __be32,
pub const MEMCONS_OUT_POS_WRAP: c_uint = 0x80000000u;
pub const MEMCONS_OUT_POS_MASK: c_uint = 0x00ffffffu;
    pub in_prod: __be32,
    pub in_cons: __be32,
}

    static struct memcons *opal_memcons = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn memcons_copy(mc: *mut memcons, to: *mut c_char, pos: loff_t, count: usize) -> isize {
    ssize_t memcons_copy(struct memcons *mc, char *to, loff_t pos, size_t count)
    {
    const char *conbuf;
    ssize_t ret;
    let mut first_read: usize = 0;
    uint32_t out_pos, avail;
    if (!mc)
    return -ENODEV;
    out_pos = be32_to_cpu(READ_ONCE(mc.out_pos));
// Now we've read out_pos, put a barrier in before reading the new
// data it points to in conbuf.
    smp_rmb();
    conbuf = phys_to_virt(be64_to_cpu(mc.obuf_phys));
// When the buffer has wrapped, read from the out_pos marker to the end
// of the buffer, and then read the remaining data as in the un-wrapped
// case.
    if (out_pos & MEMCONS_OUT_POS_WRAP) {
    out_pos &= MEMCONS_OUT_POS_MASK;
    avail = be32_to_cpu(mc.obuf_size) - out_pos;
    ret = memory_read_from_buffer(to, count, &pos,
    conbuf + out_pos, avail);
    if (ret < 0)
    goto out;
    first_read = ret;
    to += first_read;
    count -= first_read;
    pos -= avail;
    if (count <= 0)
    goto out;
    }
// Sanity check. The firmware should not do this to us.
    if (out_pos > be32_to_cpu(mc.obuf_size)) {
    pr_err("OPAL: memory console corruption. Aborting read.\n");
    return -EINVAL;
    }
    ret = memory_read_from_buffer(to, count, &pos, conbuf, out_pos);
    if (ret < 0)
    goto out;
    ret += first_read;
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_msglog_copy(to: *mut c_char, pos: loff_t, count: usize) -> isize {
    ssize_t opal_msglog_copy(char *to, loff_t pos, size_t count)
    {
    return memcons_copy(opal_memcons, to, pos, count);
    }
    static ssize_t opal_msglog_read(struct file *file, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *to,
    loff_t pos, size_t count)
    {
    return opal_msglog_copy(to, pos, count);
    }
    static struct bin_attribute opal_msglog_attr __ro_after_init = {
    .attr = {.name = "msglog", .mode = 0400},
    .read = opal_msglog_read
    };
#[no_mangle]
pub unsafe extern "C" fn memcons_init(node: *mut device_node, mc_prop_name: *const c_char) -> *mut memcons __init {
    struct memcons *__init memcons_init(struct device_node *node, const char *mc_prop_name)
    {
    u64 mcaddr;
    struct memcons *mc;
    if (of_property_read_u64(node, mc_prop_name, &mcaddr)) {
    pr_warn("%s property not found, no message log\n",
    mc_prop_name);
    goto out_err;
    }
    mc = phys_to_virt(mcaddr);
    if (!mc) {
    pr_warn("memory console address is invalid\n");
    goto out_err;
    }
    if (be64_to_cpu(mc.magic) != MEMCONS_MAGIC) {
    pr_warn("memory console version is invalid\n");
    goto out_err;
    }
    return mc;
    out_err:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn memcons_get_size(mc: *mut memcons) -> u32 __init {
    u32 __init memcons_get_size(struct memcons *mc)
    {
    return be32_to_cpu(mc.ibuf_size) + be32_to_cpu(mc.obuf_size);
    }
#[no_mangle]
pub unsafe extern "C" fn opal_msglog_init() -> void __init {
    void __init opal_msglog_init(void)
    {
    opal_memcons = memcons_init(opal_node, "ibm,opal-memcons");
    if (!opal_memcons) {
    pr_warn("OPAL: memcons failed to load from ibm,opal-memcons\n");
    return;
    }
    opal_msglog_attr.size = memcons_get_size(opal_memcons);
    }
#[no_mangle]
pub unsafe extern "C" fn opal_msglog_sysfs_init() -> void __init {
    void __init opal_msglog_sysfs_init(void)
    {
    if (!opal_memcons) {
    pr_warn("OPAL: message log initialisation failed, not creating sysfs entry\n");
    return;
    }
    if (sysfs_create_bin_file(opal_kobj, &opal_msglog_attr) != 0)
    pr_warn("OPAL: sysfs file creation failed\n");
    }
