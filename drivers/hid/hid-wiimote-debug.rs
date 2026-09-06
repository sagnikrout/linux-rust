//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-wiimote-debug.c
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
// Debug support for HID Nintendo Wii / Wii U peripherals
// Copyright (c) 2011-2013 David Herrmann <dh.herrmann@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimote_debug {
    pub wdata: *mut wiimote_data,
    pub eeprom: *mut dentry,
    pub drm: *mut dentry,
}

    static ssize_t wiidebug_eeprom_read(struct file *f, char __user *u, size_t s,
    loff_t *off)
    {
    struct wiimote_debug *dbg = f.private_data;
    struct wiimote_data *wdata = dbg.wdata;
    unsigned long flags;
    ssize_t ret;
    char buf[16];
    let mut size: __u16 = 0;
    if (s == 0)
    return -EINVAL;
    if (*off > 0xffffff)
    return 0;
    if (s > 16)
    s = 16;
    ret = wiimote_cmd_acquire(wdata);
    if (ret)
    return ret;
    spin_lock_irqsave(&wdata.state.lock, flags);
    wdata.state.cmd_read_size = s;
    wdata.state.cmd_read_buf = buf;
    wiimote_cmd_set(wdata, WIIPROTO_REQ_RMEM, *off & 0xffff);
    wiiproto_req_reeprom(wdata, *off, s);
    spin_unlock_irqrestore(&wdata.state.lock, flags);
    ret = wiimote_cmd_wait(wdata);
    if (!ret)
    size = wdata.state.cmd_read_size;
    spin_lock_irqsave(&wdata.state.lock, flags);
    wdata.state.cmd_read_buf = core::ptr::null_mut();
    spin_unlock_irqrestore(&wdata.state.lock, flags);
    wiimote_cmd_release(wdata);
    if (ret)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(0: size ==) -> else {
    else if (size == 0)
    return -EIO;
    if (copy_to_user(u, buf, size))
    return -EFAULT;
// off += size;
    ret = size;
    return ret;
    }
    static const struct file_operations wiidebug_eeprom_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = wiidebug_eeprom_read,
    .llseek = generic_file_llseek,
    };
    static const char *wiidebug_drmmap[] = {
    [WIIPROTO_REQ_NULL] = "core::ptr::null_mut()",
    [WIIPROTO_REQ_DRM_K] = "K",
    [WIIPROTO_REQ_DRM_KA] = "KA",
    [WIIPROTO_REQ_DRM_KE] = "KE",
    [WIIPROTO_REQ_DRM_KAI] = "KAI",
    [WIIPROTO_REQ_DRM_KEE] = "KEE",
    [WIIPROTO_REQ_DRM_KAE] = "KAE",
    [WIIPROTO_REQ_DRM_KIE] = "KIE",
    [WIIPROTO_REQ_DRM_KAIE] = "KAIE",
    [WIIPROTO_REQ_DRM_E] = "E",
    [WIIPROTO_REQ_DRM_SKAI1] = "SKAI1",
    [WIIPROTO_REQ_DRM_SKAI2] = "SKAI2",
    [WIIPROTO_REQ_MAX] = core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn wiidebug_drm_show(f: *mut seq_file, p: *mut c_void) -> c_int {
    static int wiidebug_drm_show(struct seq_file *f, void *p)
    {
    struct wiimote_debug *dbg = f.private;
    const char *str = core::ptr::null_mut();
    unsigned long flags;
    __u8 drm;
    spin_lock_irqsave(&dbg.wdata.state.lock, flags);
    drm = dbg.wdata.state.drm;
    spin_unlock_irqrestore(&dbg.wdata.state.lock, flags);
    if (drm < WIIPROTO_REQ_MAX)
    str = wiidebug_drmmap[drm];
    if (!str)
    str = "unknown";
    seq_printf(f, "%s\n", str);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wiidebug_drm_open(i: *mut inode, f: *mut file) -> c_int {
    static int wiidebug_drm_open(struct inode *i, struct file *f)
    {
    return single_open(f, wiidebug_drm_show, i.i_private);
    }
    static ssize_t wiidebug_drm_write(struct file *f, const char __user *u,
    size_t s, loff_t *off)
    {
    struct seq_file *sf = f.private_data;
    struct wiimote_debug *dbg = sf.private;
    unsigned long flags;
    char buf[16];
    ssize_t len;
    int i;
    if (s == 0)
    return -EINVAL;
    len = min((size_t) 15, s);
    if (copy_from_user(buf, u, len))
    return -EFAULT;
    buf[len] = 0;
    for (i = 0; i < WIIPROTO_REQ_MAX; ++i) {
    if (!wiidebug_drmmap[i])
    continue;
    if (!strcasecmp(buf, wiidebug_drmmap[i]))
    break;
    }
    if (i == WIIPROTO_REQ_MAX)
    i = simple_strtoul(buf, core::ptr::null_mut(), 16);
    spin_lock_irqsave(&dbg.wdata.state.lock, flags);
    dbg.wdata.state.flags &= ~WIIPROTO_FLAG_DRM_LOCKED;
    wiiproto_req_drm(dbg.wdata, (__u8) i);
    if (i != WIIPROTO_REQ_NULL)
    dbg.wdata.state.flags |= WIIPROTO_FLAG_DRM_LOCKED;
    spin_unlock_irqrestore(&dbg.wdata.state.lock, flags);
    return len;
    }
    static const struct file_operations wiidebug_drm_fops = {
    .owner = THIS_MODULE,
    .open = wiidebug_drm_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .write = wiidebug_drm_write,
    .release = single_release,
    };
#[no_mangle]
pub unsafe extern "C" fn wiidebug_init(wdata: *mut wiimote_data) -> c_int {
    int wiidebug_init(struct wiimote_data *wdata)
    {
    struct wiimote_debug *dbg;
    unsigned long flags;
    dbg = kzalloc_obj(*dbg);
    if (!dbg)
    return -ENOMEM;
    dbg.wdata = wdata;
    dbg.eeprom = debugfs_create_file("eeprom", S_IRUSR,
    dbg.wdata.hdev.debug_dir, dbg, &wiidebug_eeprom_fops);
    dbg.drm = debugfs_create_file("drm", S_IRUSR,
    dbg.wdata.hdev.debug_dir, dbg, &wiidebug_drm_fops);
    spin_lock_irqsave(&wdata.state.lock, flags);
    wdata.debug = dbg;
    spin_unlock_irqrestore(&wdata.state.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn wiidebug_deinit(wdata: *mut wiimote_data) {
    void wiidebug_deinit(struct wiimote_data *wdata)
    {
    struct wiimote_debug *dbg = wdata.debug;
    unsigned long flags;
    if (!dbg)
    return;
    spin_lock_irqsave(&wdata.state.lock, flags);
    wdata.debug = core::ptr::null_mut();
    spin_unlock_irqrestore(&wdata.state.lock, flags);
    debugfs_remove(dbg.drm);
    debugfs_remove(dbg.eeprom);
    kfree(dbg);
    }
