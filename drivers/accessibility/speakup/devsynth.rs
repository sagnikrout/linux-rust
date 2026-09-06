//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/devsynth.c
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

    static int synth_registered, synthu_registered;
    static int dev_opened;
// Latin1 version
    static ssize_t speakup_file_write(struct file *fp, const char __user *buffer,
    size_t nbytes, loff_t *ppos)
    {
    let mut count: usize = nbytes;
    const char __user *ptr = buffer;
    size_t bytes;
    unsigned long flags;
    u_char buf[256];
    if (!synth)
    return -ENODEV;
    while (count > 0) {
    bytes = min(count, sizeof(buf));
    if (copy_from_user(buf, ptr, bytes))
    return -EFAULT;
    count -= bytes;
    ptr += bytes;
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    synth_write(buf, bytes);
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    }
    return (ssize_t)nbytes;
    }
// UTF-8 version
    static ssize_t speakup_file_writeu(struct file *fp, const char __user *buffer,
    size_t nbytes, loff_t *ppos)
    {
    let mut count: usize = nbytes, consumed, want;
    const char __user *ptr = buffer;
    size_t bytes;
    unsigned long flags;
    unsigned char buf[256];
    u16 ubuf[256];
    size_t in, out;
    if (!synth)
    return -ENODEV;
    want = 1;
    while (count >= want) {
// Copy some UTF-8 piece from userland
    bytes = min(count, sizeof(buf));
    if (copy_from_user(buf, ptr, bytes))
    return -EFAULT;
// Convert to u16
    for (in = 0, out = 0; in < bytes; in += consumed) {
    s32 value;
    value = synth_utf8_get(buf + in, bytes - in, &consumed, &want);
    if (value == -1) {
// Invalid or incomplete
    if (want > bytes - in)
// We don't have it all yet, stop here
// and wait for the rest
//
    bytes = in;
    continue;
    }
    if (value < 0x10000)
    ubuf[out++] = value;
    }
    count -= bytes;
    ptr += bytes;
// And speak this up
    if (out) {
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    for (in = 0; in < out; in++)
    synth_buffer_add(ubuf[in]);
    synth_start();
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    }
    }
    return (ssize_t)(nbytes - count);
    }
    static ssize_t speakup_file_read(struct file *fp, char __user *buf,
    size_t nbytes, loff_t *ppos)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn speakup_file_open(ip: *mut inode, fp: *mut file) -> c_int {
    static int speakup_file_open(struct inode *ip, struct file *fp)
    {
    if (!synth)
    return -ENODEV;
    if (xchg(&dev_opened, 1))
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn speakup_file_release(ip: *mut inode, fp: *mut file) -> c_int {
    static int speakup_file_release(struct inode *ip, struct file *fp)
    {
    dev_opened = 0;
    return 0;
    }
    static const struct file_operations synth_fops = {
    .read = speakup_file_read,
    .write = speakup_file_write,
    .open = speakup_file_open,
    .release = speakup_file_release,
    };
    static const struct file_operations synthu_fops = {
    .read = speakup_file_read,
    .write = speakup_file_writeu,
    .open = speakup_file_open,
    .release = speakup_file_release,
    };
    static struct miscdevice synth_device = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "synth",
    .fops = &synth_fops,
    };
    static struct miscdevice synthu_device = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "synthu",
    .fops = &synthu_fops,
    };
#[no_mangle]
pub unsafe extern "C" fn speakup_register_devsynth() {
    void speakup_register_devsynth(void)
    {
    if (!synth_registered) {
    if (misc_register(&synth_device)) {
    pr_warn("Couldn't initialize miscdevice /dev/synth.\n");
    } else {
    pr_info("initialized device: /dev/synth, node (MAJOR %d, MINOR %d)\n",
    MISC_MAJOR, synth_device.minor);
    synth_registered = 1;
    }
    }
    if (!synthu_registered) {
    if (misc_register(&synthu_device)) {
    pr_warn("Couldn't initialize miscdevice /dev/synthu.\n");
    } else {
    pr_info("initialized device: /dev/synthu, node (MAJOR %d, MINOR %d)\n",
    MISC_MAJOR, synthu_device.minor);
    synthu_registered = 1;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_unregister_devsynth() {
    void speakup_unregister_devsynth(void)
    {
    if (synth_registered) {
    pr_info("speakup: unregistering synth device /dev/synth\n");
    misc_deregister(&synth_device);
    synth_registered = 0;
    }
    if (synthu_registered) {
    pr_info("speakup: unregistering synth device /dev/synthu\n");
    misc_deregister(&synthu_device);
    synthu_registered = 0;
    }
    }
