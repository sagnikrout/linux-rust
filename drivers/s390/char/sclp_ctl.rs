//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_ctl.c
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
// IOCTL interface for SCLP
//
// Copyright IBM Corp. 2012
//
// Author: Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

//
// Supported command words
//
    static unsigned int sclp_ctl_sccb_wlist[] = {
    0x00400002,
    0x00410002,
    };
//
// Check if command word is supported
//
#[no_mangle]
unsafe extern "C" fn sclp_ctl_cmdw_supported(cmdw: c_uint) -> c_int {
    static int sclp_ctl_cmdw_supported(unsigned int cmdw)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(sclp_ctl_sccb_wlist); i++) {
    if (cmdw == sclp_ctl_sccb_wlist[i])
    return 1;
    }
    return 0;
    }
    static void __user *u64_to_uptr(u64 value)
    {
    return (void __user *)(unsigned long)value;
    }
//
// Start SCLP request
//
#[no_mangle]
unsafe extern "C" fn sclp_ctl_ioctl_sccb(user_area: *mut void __user) -> c_int {
    static int sclp_ctl_ioctl_sccb(void __user *user_area)
    {
    struct sclp_ctl_sccb ctl_sccb;
    struct sccb_header *sccb;
    unsigned long copied;
    int rc;
    if (copy_from_user(&ctl_sccb, user_area, sizeof(ctl_sccb)))
    return -EFAULT;
    if (!sclp_ctl_cmdw_supported(ctl_sccb.cmdw))
    return -EOPNOTSUPP;
    sccb = (void *) get_zeroed_page(GFP_KERNEL | GFP_DMA);
    if (!sccb)
    return -ENOMEM;
    copied = PAGE_SIZE -
    copy_from_user(sccb, u64_to_uptr(ctl_sccb.sccb), PAGE_SIZE);
    if (offsetof(struct sccb_header, length) +
    sizeof(sccb.length) > copied || sccb.length > copied) {
    rc = -EFAULT;
    goto out_free;
    }
    if (sccb.length < 8) {
    rc = -EINVAL;
    goto out_free;
    }
    rc = sclp_sync_request(ctl_sccb.cmdw, sccb);
    if (rc)
    goto out_free;
    if (copy_to_user(u64_to_uptr(ctl_sccb.sccb), sccb, sccb.length))
    rc = -EFAULT;
    out_free:
    free_page((unsigned long) sccb);
    return rc;
    }
//
// SCLP SCCB ioctl function
//
    static long sclp_ctl_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    void __user *argp;
    argp = (void __user *)arg;
    switch (cmd) {
    case SCLP_CTL_SCCB:
    return sclp_ctl_ioctl_sccb(argp);
    default: /* unknown ioctl number */
    return -ENOTTY;
    }
    }
//
// File operations
//
    static const struct file_operations sclp_ctl_fops = {
    .owner = THIS_MODULE,
    .open = nonseekable_open,
    .unlocked_ioctl = sclp_ctl_ioctl,
    };
//
// Misc device definition
//
    static struct miscdevice sclp_ctl_device = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "sclp",
    .fops = &sclp_ctl_fops,
    };
    builtin_misc_device(sclp_ctl_device);
