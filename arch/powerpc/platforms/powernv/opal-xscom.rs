//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-xscom.c
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
// PowerNV SCOM bus debugfs interface
//
// Copyright 2010 Benjamin Herrenschmidt, IBM Corp
// <benh@kernel.crashing.org>
// and        David Gibson, IBM Corporation.
// Copyright 2013 IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn opal_scom_unmangle(addr: u64) -> u64 {
    static u64 opal_scom_unmangle(u64 addr)
    {
    u64 tmp;
//
// XSCOM addresses use the top nibble to set indirect mode and
// its form.  Bits 4-11 are always 0.
//
// Because the debugfs interface uses signed offsets and shifts
// the address left by 3, we basically cannot use the top 4 bits
// of the 64-bit address, and thus cannot use the indirect bit.
//
// To deal with that, we support the indirect bits being in
// bits 4-7 (IBM notation) instead of bit 0-3 in this API, we
// do the conversion here.
//
// For in-kernel use, we don't need to do this mangling.  In
// kernel won't have bits 4-7 set.
//
// So:
// debugfs will always   set 0-3 = 0 and clear 4-7
// kernel will always clear 0-3 = 0 and   set 4-7
//
    tmp = addr;
    tmp  &= 0x0f00000000000000;
    addr &= 0xf0ffffffffffffff;
    addr |= tmp << 4;
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn opal_scom_read(chip: u32, addr: u64, reg: u64, value: *mut u64) -> c_int {
    static int opal_scom_read(uint32_t chip, uint64_t addr, u64 reg, u64 *value)
    {
    int64_t rc;
    __be64 v;
    reg = opal_scom_unmangle(addr + reg);
    rc = opal_xscom_read(chip, reg, (__be64 *)__pa(&v));
    if (rc) {
// value = 0xfffffffffffffffful;
    return -EIO;
    }
// value = be64_to_cpu(v);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opal_scom_write(chip: u32, addr: u64, reg: u64, value: u64) -> c_int {
    static int opal_scom_write(uint32_t chip, uint64_t addr, u64 reg, u64 value)
    {
    int64_t rc;
    reg = opal_scom_unmangle(addr + reg);
    rc = opal_xscom_write(chip, reg, value);
    if (rc)
    return -EIO;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scom_debug_entry {
    pub chip: u32,
    pub path: debugfs_blob_wrapper,
    pub name: [c_char; 16],
}

    static ssize_t scom_debug_read(struct file *filp, char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct scom_debug_entry *ent = filp.private_data;
    u64 __user *ubuf64 = (u64 __user *)ubuf;
    let mut off: loff_t = *ppos;
    let mut done: isize = 0;
    u64 reg, reg_base, reg_cnt, val;
    int rc;
    if (off < 0 || (off & 7) || (count & 7))
    return -EINVAL;
    reg_base = off >> 3;
    reg_cnt = count >> 3;
    for (reg = 0; reg < reg_cnt; reg++) {
    rc = opal_scom_read(ent.chip, reg_base, reg, &val);
    if (!rc)
    rc = put_user(val, ubuf64);
    if (rc) {
    if (!done)
    done = rc;
    break;
    }
    ubuf64++;
// ppos += 8;
    done += 8;
    }
    return done;
    }
    static ssize_t scom_debug_write(struct file *filp, const char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct scom_debug_entry *ent = filp.private_data;
    u64 __user *ubuf64 = (u64 __user *)ubuf;
    let mut off: loff_t = *ppos;
    let mut done: isize = 0;
    u64 reg, reg_base, reg_cnt, val;
    int rc;
    if (off < 0 || (off & 7) || (count & 7))
    return -EINVAL;
    reg_base = off >> 3;
    reg_cnt = count >> 3;
    for (reg = 0; reg < reg_cnt; reg++) {
    rc = get_user(val, ubuf64);
    if (!rc)
    rc = opal_scom_write(ent.chip, reg_base, reg,  val);
    if (rc) {
    if (!done)
    done = rc;
    break;
    }
    ubuf64++;
    done += 8;
    }
    return done;
    }
    static const struct file_operations scom_debug_fops = {
    .read =		scom_debug_read,
    .write =	scom_debug_write,
    .open =		simple_open,
    .llseek =	default_llseek,
    };
    static int scom_debug_init_one(struct dentry *root, struct device_node *dn,
    int chip)
    {
    struct scom_debug_entry *ent;
    struct dentry *dir;
    ent = kzalloc_obj(*ent);
    if (!ent)
    return -ENOMEM;
    ent.chip = chip;
    snprintf(ent.name, 16, "%08x", chip);
    ent.path.data = (void *)kasprintf(GFP_KERNEL, "%pOF", dn);
    if (!ent.path.data) {
    kfree(ent);
    return -ENOMEM;
    }
    ent.path.size = strlen((char *)ent.path.data);
    dir = debugfs_create_dir(ent.name, root);
    if (IS_ERR(dir)) {
    kfree(ent.path.data);
    kfree(ent);
    return -1;
    }
    debugfs_create_blob("devspec", 0400, dir, &ent.path);
    debugfs_create_file("access", 0600, dir, ent, &scom_debug_fops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scom_debug_init() -> c_int {
    static int scom_debug_init(void)
    {
    struct device_node *dn;
    struct dentry *root;
    int chip, rc;
    if (!firmware_has_feature(FW_FEATURE_OPAL))
    return 0;
    root = debugfs_create_dir("scom", arch_debugfs_dir);
    if (IS_ERR(root))
    return -1;
    rc = 0;
    for_each_node_with_property(dn, "scom-controller") {
    chip = of_get_ibm_chip_id(dn);
    WARN_ON(chip == -1);
    rc |= scom_debug_init_one(root, dn, chip);
    }
    return rc;
    }
    device_initcall(scom_debug_init);
