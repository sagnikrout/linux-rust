//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/uvc/uvc_debugfs.c
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
// uvc_debugfs.c --  USB Video Class driver - Debugging support
//
// Copyright (C) 2011
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// -----------------------------------------------------------------------------
// Statistics
//
pub const UVC_DEBUGFS_BUF_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_debugfs_buffer {
    pub count: usize,
    pub data: [c_char; UVC_DEBUGFS_BUF_SIZE],
}

#[no_mangle]
unsafe extern "C" fn uvc_debugfs_stats_open(inode: *mut inode, file: *mut file) -> c_int {
    static int uvc_debugfs_stats_open(struct inode *inode, struct file *file)
    {
    struct uvc_streaming *stream = inode.i_private;
    struct uvc_debugfs_buffer *buf;
    buf = kmalloc_obj(*buf);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    buf.count = uvc_video_stats_dump(stream, buf.data, sizeof(buf.data));
    file.private_data = buf;
    return 0;
    }
    static ssize_t uvc_debugfs_stats_read(struct file *file, char __user *user_buf,
    size_t nbytes, loff_t *ppos)
    {
    struct uvc_debugfs_buffer *buf = file.private_data;
    return simple_read_from_buffer(user_buf, nbytes, ppos, buf.data,
    buf.count);
    }
#[no_mangle]
unsafe extern "C" fn uvc_debugfs_stats_release(inode: *mut inode, file: *mut file) -> c_int {
    static int uvc_debugfs_stats_release(struct inode *inode, struct file *file)
    {
    kfree(file.private_data);
    file.private_data = core::ptr::null_mut();
    return 0;
    }
    static const struct file_operations uvc_debugfs_stats_fops = {
    .owner = THIS_MODULE,
    .open = uvc_debugfs_stats_open,
    .read = uvc_debugfs_stats_read,
    .release = uvc_debugfs_stats_release,
    };
// -----------------------------------------------------------------------------
// Global and stream initialization/cleanup
//
    static struct dentry *uvc_debugfs_root_dir;
#[no_mangle]
pub unsafe extern "C" fn uvc_debugfs_init_stream(stream: *mut uvc_streaming) {
    void uvc_debugfs_init_stream(struct uvc_streaming *stream)
    {
    struct usb_device *udev = stream.dev.udev;
    char dir_name[33];
    if (uvc_debugfs_root_dir == core::ptr::null_mut())
    return;
    snprintf(dir_name, sizeof(dir_name), "%u-%u-%u", udev.bus.busnum,
    udev.devnum, stream.intfnum);
    stream.debugfs_dir = debugfs_create_dir(dir_name,
    uvc_debugfs_root_dir);
    debugfs_create_file("stats", 0444, stream.debugfs_dir, stream,
    &uvc_debugfs_stats_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_debugfs_cleanup_stream(stream: *mut uvc_streaming) {
    void uvc_debugfs_cleanup_stream(struct uvc_streaming *stream)
    {
    debugfs_remove_recursive(stream.debugfs_dir);
    stream.debugfs_dir = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_debugfs_init() {
    void uvc_debugfs_init(void)
    {
    uvc_debugfs_root_dir = debugfs_create_dir("uvcvideo", usb_debug_root);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_debugfs_cleanup() {
    void uvc_debugfs_cleanup(void)
    {
    debugfs_remove_recursive(uvc_debugfs_root_dir);
    }
