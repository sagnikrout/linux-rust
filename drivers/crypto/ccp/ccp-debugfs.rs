//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/ccp-debugfs.c
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
// AMD Cryptographic Coprocessor (CCP) driver
//
// Copyright (C) 2017 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <gary.hook@amd.com>
//

// DebugFS helpers

pub const OBUFLEN: c_int = 512;

    scnprintf(OBUFP, OBUFSPC, fmt, ## __VA_ARGS__)
pub const BUFLEN: c_int = 63;
pub const RI_VERSION_NUM: c_uint = 0x0000003F;
pub const RI_AES_PRESENT: c_uint = 0x00000040;
pub const RI_3DES_PRESENT: c_uint = 0x00000080;
pub const RI_SHA_PRESENT: c_uint = 0x00000100;
pub const RI_RSA_PRESENT: c_uint = 0x00000200;
pub const RI_ECC_PRESENT: c_uint = 0x00000400;
pub const RI_ZDE_PRESENT: c_uint = 0x00000800;
pub const RI_ZCE_PRESENT: c_uint = 0x00001000;
pub const RI_TRNG_PRESENT: c_uint = 0x00002000;
pub const RI_ELFC_PRESENT: c_uint = 0x00004000;
pub const RI_ELFC_SHIFT: c_int = 14;
pub const RI_NUM_VQM: c_uint = 0x00078000;
pub const RI_NVQM_SHIFT: c_int = 15;

pub const RI_LSB_ENTRIES: c_uint = 0x0FF80000;
pub const RI_NLSB_SHIFT: c_int = 19;

    static ssize_t ccp5_debugfs_info_read(struct file *filp, char __user *ubuf,
    size_t count, loff_t *offp)
    {
    struct ccp_device *ccp = filp.private_data;
    let mut oboff: c_uint = 0;
    unsigned int regval;
    ssize_t ret;
    char *obuf;
    if (!ccp)
    return 0;
    obuf = kmalloc(OBUFLEN, GFP_KERNEL);
    if (!obuf)
    return -ENOMEM;
    oboff += OSCNPRINTF("Device name: %s\n", ccp.name);
    oboff += OSCNPRINTF("   RNG name: %s\n", ccp.rngname);
    oboff += OSCNPRINTF("   # Queues: %d\n", ccp.cmd_q_count);
    oboff += OSCNPRINTF("     # Cmds: %d\n", ccp.cmd_count);
    regval = ioread32(ccp.io_regs + CMD5_PSP_CCP_VERSION);
    oboff += OSCNPRINTF("    Version: %d\n", regval & RI_VERSION_NUM);
    oboff += OSCNPRINTF("    Engines:");
    if (regval & RI_AES_PRESENT)
    oboff += OSCNPRINTF(" AES");
    if (regval & RI_3DES_PRESENT)
    oboff += OSCNPRINTF(" 3DES");
    if (regval & RI_SHA_PRESENT)
    oboff += OSCNPRINTF(" SHA");
    if (regval & RI_RSA_PRESENT)
    oboff += OSCNPRINTF(" RSA");
    if (regval & RI_ECC_PRESENT)
    oboff += OSCNPRINTF(" ECC");
    if (regval & RI_ZDE_PRESENT)
    oboff += OSCNPRINTF(" ZDE");
    if (regval & RI_ZCE_PRESENT)
    oboff += OSCNPRINTF(" ZCE");
    if (regval & RI_TRNG_PRESENT)
    oboff += OSCNPRINTF(" TRNG");
    oboff += OSCNPRINTF("\n");
    oboff += OSCNPRINTF("     Queues: %d\n",
    (regval & RI_NUM_VQM) >> RI_NVQM_SHIFT);
    oboff += OSCNPRINTF("LSB Entries: %d\n",
    (regval & RI_LSB_ENTRIES) >> RI_NLSB_SHIFT);
    ret = simple_read_from_buffer(ubuf, count, offp, obuf, oboff);
    kfree(obuf);
    return ret;
    }
// Return a formatted buffer containing the current
// statistics across all queues for a CCP.
//
    static ssize_t ccp5_debugfs_stats_read(struct file *filp, char __user *ubuf,
    size_t count, loff_t *offp)
    {
    struct ccp_device *ccp = filp.private_data;
    let mut total_xts_aes_ops: c_ulong = 0;
    let mut total_3des_ops: c_ulong = 0;
    let mut total_aes_ops: c_ulong = 0;
    let mut total_sha_ops: c_ulong = 0;
    let mut total_rsa_ops: c_ulong = 0;
    let mut total_ecc_ops: c_ulong = 0;
    let mut total_pt_ops: c_ulong = 0;
    let mut total_ops: c_ulong = 0;
    let mut oboff: c_uint = 0;
    let mut ret: isize = 0;
    unsigned int i;
    char *obuf;
    for (i = 0; i < ccp.cmd_q_count; i++) {
    struct ccp_cmd_queue *cmd_q = &ccp.cmd_q[i];
    total_ops += cmd_q.total_ops;
    total_aes_ops += cmd_q.total_aes_ops;
    total_xts_aes_ops += cmd_q.total_xts_aes_ops;
    total_3des_ops += cmd_q.total_3des_ops;
    total_sha_ops += cmd_q.total_sha_ops;
    total_rsa_ops += cmd_q.total_rsa_ops;
    total_pt_ops += cmd_q.total_pt_ops;
    total_ecc_ops += cmd_q.total_ecc_ops;
    }
    obuf = kmalloc(OBUFLEN, GFP_KERNEL);
    if (!obuf)
    return -ENOMEM;
    oboff += OSCNPRINTF("Total Interrupts Handled: %ld\n",
    ccp.total_interrupts);
    oboff += OSCNPRINTF("        Total Operations: %ld\n",
    total_ops);
    oboff += OSCNPRINTF("                     AES: %ld\n",
    total_aes_ops);
    oboff += OSCNPRINTF("                 XTS AES: %ld\n",
    total_xts_aes_ops);
    oboff += OSCNPRINTF("                     SHA: %ld\n",
    total_3des_ops);
    oboff += OSCNPRINTF("                     SHA: %ld\n",
    total_sha_ops);
    oboff += OSCNPRINTF("                     RSA: %ld\n",
    total_rsa_ops);
    oboff += OSCNPRINTF("               Pass-Thru: %ld\n",
    total_pt_ops);
    oboff += OSCNPRINTF("                     ECC: %ld\n",
    total_ecc_ops);
    ret = simple_read_from_buffer(ubuf, count, offp, obuf, oboff);
    kfree(obuf);
    return ret;
    }
// Reset the counters in a queue
//
#[no_mangle]
unsafe extern "C" fn ccp5_debugfs_reset_queue_stats(cmd_q: *mut ccp_cmd_queue) {
    static void ccp5_debugfs_reset_queue_stats(struct ccp_cmd_queue *cmd_q)
    {
    cmd_q.total_ops = 0L;
    cmd_q.total_aes_ops = 0L;
    cmd_q.total_xts_aes_ops = 0L;
    cmd_q.total_3des_ops = 0L;
    cmd_q.total_sha_ops = 0L;
    cmd_q.total_rsa_ops = 0L;
    cmd_q.total_pt_ops = 0L;
    cmd_q.total_ecc_ops = 0L;
    }
// A value was written to the stats variable, which
// should be used to reset the queue counters across
// that device.
//
    static ssize_t ccp5_debugfs_stats_write(struct file *filp,
    const char __user *ubuf,
    size_t count, loff_t *offp)
    {
    struct ccp_device *ccp = filp.private_data;
    int i;
    for (i = 0; i < ccp.cmd_q_count; i++)
    ccp5_debugfs_reset_queue_stats(&ccp.cmd_q[i]);
    ccp.total_interrupts = 0L;
    return count;
    }
// Return a formatted buffer containing the current information
// for that queue
//
    static ssize_t ccp5_debugfs_queue_read(struct file *filp, char __user *ubuf,
    size_t count, loff_t *offp)
    {
    struct ccp_cmd_queue *cmd_q = filp.private_data;
    let mut oboff: c_uint = 0;
    unsigned int regval;
    ssize_t ret;
    char *obuf;
    if (!cmd_q)
    return 0;
    obuf = kmalloc(OBUFLEN, GFP_KERNEL);
    if (!obuf)
    return -ENOMEM;
    oboff += OSCNPRINTF("  Total Queue Operations: %ld\n",
    cmd_q.total_ops);
    oboff += OSCNPRINTF("                     AES: %ld\n",
    cmd_q.total_aes_ops);
    oboff += OSCNPRINTF("                 XTS AES: %ld\n",
    cmd_q.total_xts_aes_ops);
    oboff += OSCNPRINTF("                     SHA: %ld\n",
    cmd_q.total_3des_ops);
    oboff += OSCNPRINTF("                     SHA: %ld\n",
    cmd_q.total_sha_ops);
    oboff += OSCNPRINTF("                     RSA: %ld\n",
    cmd_q.total_rsa_ops);
    oboff += OSCNPRINTF("               Pass-Thru: %ld\n",
    cmd_q.total_pt_ops);
    oboff += OSCNPRINTF("                     ECC: %ld\n",
    cmd_q.total_ecc_ops);
    regval = ioread32(cmd_q.reg_int_enable);
    oboff += OSCNPRINTF("      Enabled Interrupts:");
    if (regval & INT_EMPTY_QUEUE)
    oboff += OSCNPRINTF(" EMPTY");
    if (regval & INT_QUEUE_STOPPED)
    oboff += OSCNPRINTF(" STOPPED");
    if (regval & INT_ERROR)
    oboff += OSCNPRINTF(" ERROR");
    if (regval & INT_COMPLETION)
    oboff += OSCNPRINTF(" COMPLETION");
    oboff += OSCNPRINTF("\n");
    ret = simple_read_from_buffer(ubuf, count, offp, obuf, oboff);
    kfree(obuf);
    return ret;
    }
// A value was written to the stats variable for a
// queue. Reset the queue counters to this value.
//
    static ssize_t ccp5_debugfs_queue_write(struct file *filp,
    const char __user *ubuf,
    size_t count, loff_t *offp)
    {
    struct ccp_cmd_queue *cmd_q = filp.private_data;
    ccp5_debugfs_reset_queue_stats(cmd_q);
    return count;
    }
    static const struct file_operations ccp_debugfs_info_ops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = ccp5_debugfs_info_read,
    .write = core::ptr::null_mut(),
    };
    static const struct file_operations ccp_debugfs_queue_ops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = ccp5_debugfs_queue_read,
    .write = ccp5_debugfs_queue_write,
    };
    static const struct file_operations ccp_debugfs_stats_ops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = ccp5_debugfs_stats_read,
    .write = ccp5_debugfs_stats_write,
    };
    static struct dentry *ccp_debugfs_dir;
    static DEFINE_MUTEX(ccp_debugfs_lock);
pub const MAX_NAME_LEN: c_int = 20;
#[no_mangle]
pub unsafe extern "C" fn ccp5_debugfs_setup(ccp: *mut ccp_device) {
    void ccp5_debugfs_setup(struct ccp_device *ccp)
    {
    struct ccp_cmd_queue *cmd_q;
    char name[MAX_NAME_LEN + 1];
    struct dentry *debugfs_q_instance;
    int i;
    if (!debugfs_initialized())
    return;
    mutex_lock(&ccp_debugfs_lock);
    if (!ccp_debugfs_dir)
    ccp_debugfs_dir = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    mutex_unlock(&ccp_debugfs_lock);
    ccp.debugfs_instance = debugfs_create_dir(ccp.name, ccp_debugfs_dir);
    debugfs_create_file("info", 0400, ccp.debugfs_instance, ccp,
    &ccp_debugfs_info_ops);
    debugfs_create_file("stats", 0600, ccp.debugfs_instance, ccp,
    &ccp_debugfs_stats_ops);
    for (i = 0; i < ccp.cmd_q_count; i++) {
    cmd_q = &ccp.cmd_q[i];
    snprintf(name, MAX_NAME_LEN - 1, "q%d", cmd_q.id);
    debugfs_q_instance =
    debugfs_create_dir(name, ccp.debugfs_instance);
    debugfs_create_file("stats", 0600, debugfs_q_instance, cmd_q,
    &ccp_debugfs_queue_ops);
    }
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn ccp5_debugfs_destroy() {
    void ccp5_debugfs_destroy(void)
    {
    mutex_lock(&ccp_debugfs_lock);
    debugfs_remove_recursive(ccp_debugfs_dir);
    ccp_debugfs_dir = core::ptr::null_mut();
    mutex_unlock(&ccp_debugfs_lock);
    }
