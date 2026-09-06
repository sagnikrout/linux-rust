//! Automatically rewritten from C to Rust
//! Source: drivers/tty/n_null.c
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
// n_null.c - Null line discipline used in the failure path
//
// Copyright (C) Intel 2017
//
    static ssize_t n_null_read(struct tty_struct *tty, struct file *file, u8 *buf,
    size_t nr, void **cookie, unsigned long offset)
    {
    return -EOPNOTSUPP;
    }
    static ssize_t n_null_write(struct tty_struct *tty, struct file *file,
    const u8 *buf, size_t nr)
    {
    return -EOPNOTSUPP;
    }
    static struct tty_ldisc_ops null_ldisc = {
    .owner		=	THIS_MODULE,
    .num		=	N_NULL,
    .name		=	"n_null",
    .read		=	n_null_read,
    .write		=	n_null_write,
    };
#[no_mangle]
unsafe extern "C" fn n_null_init() -> int __init {
    static int __init n_null_init(void)
    {
    BUG_ON(tty_register_ldisc(&null_ldisc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn n_null_exit() -> void __exit {
    static void __exit n_null_exit(void)
    {
    tty_unregister_ldisc(&null_ldisc);
    }
    module_init(n_null_init);
    module_exit(n_null_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alan Cox");
    MODULE_ALIAS_LDISC(N_NULL);
    MODULE_DESCRIPTION("Null ldisc driver");
