//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/btmrvl_debugfs.c
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
// Marvell Bluetooth driver: debugfs related functions
//
// Copyright (C) 2009, Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_debugfs_data {
    pub config_dir: *mut dentry,
    pub status_dir: *mut dentry,
}

    static ssize_t btmrvl_hscfgcmd_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    long result, ret;
    ret = kstrtol_from_user(ubuf, count, 10, &result);
    if (ret)
    return ret;
    priv.btmrvl_dev.hscfgcmd = result;
    if (priv.btmrvl_dev.hscfgcmd) {
    btmrvl_prepare_command(priv);
    wake_up_interruptible(&priv.main_thread.wait_q);
    }
    return count;
    }
    static ssize_t btmrvl_hscfgcmd_read(struct file *file, char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    char buf[16];
    int ret;
    ret = snprintf(buf, sizeof(buf) - 1, "%d\n",
    priv.btmrvl_dev.hscfgcmd);
    return simple_read_from_buffer(userbuf, count, ppos, buf, ret);
    }
    static const struct file_operations btmrvl_hscfgcmd_fops = {
    .read	= btmrvl_hscfgcmd_read,
    .write	= btmrvl_hscfgcmd_write,
    .open	= simple_open,
    .llseek = default_llseek,
    };
    static ssize_t btmrvl_pscmd_write(struct file *file, const char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    long result, ret;
    ret = kstrtol_from_user(ubuf, count, 10, &result);
    if (ret)
    return ret;
    priv.btmrvl_dev.pscmd = result;
    if (priv.btmrvl_dev.pscmd) {
    btmrvl_prepare_command(priv);
    wake_up_interruptible(&priv.main_thread.wait_q);
    }
    return count;
    }
    static ssize_t btmrvl_pscmd_read(struct file *file, char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    char buf[16];
    int ret;
    ret = snprintf(buf, sizeof(buf) - 1, "%d\n", priv.btmrvl_dev.pscmd);
    return simple_read_from_buffer(userbuf, count, ppos, buf, ret);
    }
    static const struct file_operations btmrvl_pscmd_fops = {
    .read = btmrvl_pscmd_read,
    .write = btmrvl_pscmd_write,
    .open = simple_open,
    .llseek = default_llseek,
    };
    static ssize_t btmrvl_hscmd_write(struct file *file, const char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    long result, ret;
    ret = kstrtol_from_user(ubuf, count, 10, &result);
    if (ret)
    return ret;
    priv.btmrvl_dev.hscmd = result;
    if (priv.btmrvl_dev.hscmd) {
    btmrvl_prepare_command(priv);
    wake_up_interruptible(&priv.main_thread.wait_q);
    }
    return count;
    }
    static ssize_t btmrvl_hscmd_read(struct file *file, char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct btmrvl_private *priv = file.private_data;
    char buf[16];
    int ret;
    ret = snprintf(buf, sizeof(buf) - 1, "%d\n", priv.btmrvl_dev.hscmd);
    return simple_read_from_buffer(userbuf, count, ppos, buf, ret);
    }
    static const struct file_operations btmrvl_hscmd_fops = {
    .read	= btmrvl_hscmd_read,
    .write	= btmrvl_hscmd_write,
    .open	= simple_open,
    .llseek = default_llseek,
    };
#[no_mangle]
pub unsafe extern "C" fn btmrvl_debugfs_init(hdev: *mut hci_dev) {
    void btmrvl_debugfs_init(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    struct btmrvl_debugfs_data *dbg;
    if (!hdev.debugfs)
    return;
    dbg = kzalloc_obj(*dbg);
    priv.debugfs_data = dbg;
    if (!dbg) {
    BT_ERR("Can not allocate memory for btmrvl_debugfs_data.");
    return;
    }
    dbg.config_dir = debugfs_create_dir("config", hdev.debugfs);
    debugfs_create_u8("psmode", 0644, dbg.config_dir,
    &priv.btmrvl_dev.psmode);
    debugfs_create_file("pscmd", 0644, dbg.config_dir,
    priv, &btmrvl_pscmd_fops);
    debugfs_create_x16("gpiogap", 0644, dbg.config_dir,
    &priv.btmrvl_dev.gpio_gap);
    debugfs_create_u8("hsmode", 0644, dbg.config_dir,
    &priv.btmrvl_dev.hsmode);
    debugfs_create_file("hscmd", 0644, dbg.config_dir,
    priv, &btmrvl_hscmd_fops);
    debugfs_create_file("hscfgcmd", 0644, dbg.config_dir,
    priv, &btmrvl_hscfgcmd_fops);
    dbg.status_dir = debugfs_create_dir("status", hdev.debugfs);
    debugfs_create_u8("curpsmode", 0444, dbg.status_dir,
    &priv.adapter.psmode);
    debugfs_create_u8("psstate", 0444, dbg.status_dir,
    &priv.adapter.ps_state);
    debugfs_create_u8("hsstate", 0444, dbg.status_dir,
    &priv.adapter.hs_state);
    debugfs_create_u8("txdnldready", 0444, dbg.status_dir,
    &priv.btmrvl_dev.tx_dnld_rdy);
    }
#[no_mangle]
pub unsafe extern "C" fn btmrvl_debugfs_remove(hdev: *mut hci_dev) {
    void btmrvl_debugfs_remove(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    struct btmrvl_debugfs_data *dbg = priv.debugfs_data;
    if (!dbg)
    return;
    debugfs_remove_recursive(dbg.config_dir);
    debugfs_remove_recursive(dbg.status_dir);
    kfree(dbg);
    }
