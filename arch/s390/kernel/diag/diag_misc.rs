//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/diag/diag_misc.c
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
// Provide diagnose information via misc device /dev/diag.
//
// Copyright IBM Corp. 2024
//

#[no_mangle]
unsafe extern "C" fn diag_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long diag_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    long rc;
    switch (cmd) {
    case DIAG324_GET_PIBLEN:
    rc = diag324_piblen(arg);
    break;
    case DIAG324_GET_PIBBUF:
    rc = diag324_pibbuf(arg);
    break;
    case DIAG310_GET_STRIDE:
    rc = diag310_memtop_stride(arg);
    break;
    case DIAG310_GET_MEMTOPLEN:
    rc = diag310_memtop_len(arg);
    break;
    case DIAG310_GET_MEMTOPBUF:
    rc = diag310_memtop_buf(arg);
    break;
    default:
    rc = -ENOIOCTLCMD;
    break;
    }
    return rc;
    }
    static const struct file_operations fops = {
    .owner		= THIS_MODULE,
    .open		= nonseekable_open,
    .unlocked_ioctl	= diag_ioctl,
    };
    static struct miscdevice diagdev = {
    .name	= "diag",
    .minor	= MISC_DYNAMIC_MINOR,
    .fops	= &fops,
    .mode	= 0444,
    };
#[no_mangle]
unsafe extern "C" fn diag_init() -> c_int {
    static int diag_init(void)
    {
    return misc_register(&diagdev);
    }
    device_initcall(diag_init);
