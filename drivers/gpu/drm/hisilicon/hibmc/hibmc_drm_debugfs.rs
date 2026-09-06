//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hisilicon/hibmc/hibmc_drm_debugfs.c
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
// Copyright (c) 2024 Hisilicon Limited.

pub const MAX_BUF_SIZE: c_int = 12;
    static ssize_t hibmc_control_write(struct file *file, const char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    struct hibmc_drm_private *priv = file_inode(file).i_private;
    struct hibmc_dp_cbar_cfg *cfg = &priv.dp.cfg;
    int ret, idx;
    u8 buf[MAX_BUF_SIZE];
    if (count >= MAX_BUF_SIZE)
    return -EINVAL;
    if (copy_from_user(buf, user_buf, count))
    return -EFAULT;
    buf[count] = '\0';
// Only 4 parameters is allowed, the ranger are as follow:
// [0] enable/disable colorbar feature
    0: enable colorbar, 1: disable colorbar
// [1] the timing source of colorbar displaying
    0: timing follows XDP, 1: internal self timing
// [2] the movment of colorbar displaying
    0: static colorbar image,
// 1~255: right shifting a type of color per (1~255)frames
// [3] the color type of colorbar displaying
    0~9: color bar, white, red, orange,
// yellow, green, cyan, bule, pupper, black
//
    if (sscanf(buf, "%hhu %hhu %hhu %u", &cfg.enable, &cfg.self_timing,
    &cfg.dynamic_rate, &cfg.pattern) != 4) {
    return -EINVAL;
    }
    if (cfg.pattern > 9 || cfg.enable > 1 || cfg.self_timing > 1)
    return -EINVAL;
    ret = drm_dev_enter(&priv.dev, &idx);
    if (!ret)
    return -ENODEV;
    hibmc_dp_set_cbar(&priv.dp, cfg);
    drm_dev_exit(idx);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn hibmc_dp_dbgfs_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int hibmc_dp_dbgfs_show(struct seq_file *m, void *arg)
    {
    struct hibmc_drm_private *priv = m.private;
    struct hibmc_dp_cbar_cfg *cfg = &priv.dp.cfg;
    int idx;
    if (!drm_dev_enter(&priv.dev, &idx))
    return -ENODEV;
    seq_printf(m, "hibmc dp colorbar cfg: %u %u %u %u\n", cfg.enable, cfg.self_timing,
    cfg.dynamic_rate, cfg.pattern);
    drm_dev_exit(idx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hibmc_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int hibmc_open(struct inode *inode, struct file *filp)
    {
    return single_open(filp, hibmc_dp_dbgfs_show, inode.i_private);
    }
    static const struct file_operations hibmc_dbg_fops = {
    .owner   = THIS_MODULE,
    .write   = hibmc_control_write,
    .read    = seq_read,
    .open    = hibmc_open,
    .llseek  = seq_lseek,
    .release = single_release,
    };
#[no_mangle]
pub unsafe extern "C" fn hibmc_debugfs_init(connector: *mut drm_connector, root: *mut dentry) {
    void hibmc_debugfs_init(struct drm_connector *connector, struct dentry *root)
    {
    struct drm_device *dev = connector.dev;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(dev);
// create the file in drm directory, so we don't need to remove manually
    debugfs_create_file("colorbar-cfg", 0200,
    root, priv, &hibmc_dbg_fops);
    }
