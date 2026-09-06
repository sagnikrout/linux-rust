//! Automatically rewritten from C to Rust
//! Source: samples/kfifo/record-example.c
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
// Sample dynamic sized record fifo implementation
//
// Copyright (C) 2010 Stefani Seibold <stefani@seibold.net>
//

//
// This module shows how to create a variable sized record fifo.
//
// fifo size in elements (bytes)
pub const FIFO_SIZE: c_int = 128;
// name of the proc entry

// lock for procfs read access
    static DEFINE_MUTEX(read_access);
// lock for procfs write access
    static DEFINE_MUTEX(write_access);
//
// define DYNAMIC in this example for a dynamically allocated fifo.
//
// Otherwise the fifo storage will be a part of the fifo structure.
//

// Macro flag: #define DYNAMIC

//
// struct kfifo_rec_ptr_1 and  STRUCT_KFIFO_REC_1 can handle records of a
// length between 0 and 255 bytes.
//
// struct kfifo_rec_ptr_2 and  STRUCT_KFIFO_REC_2 can handle records of a
// length between 0 and 65535 bytes.
//

    struct kfifo_rec_ptr_1 test;

    typedef STRUCT_KFIFO_REC_1(FIFO_SIZE) mytest;
    static mytest test;

    static const char *expected_result[] = {
    "a",
    "bb",
    "ccc",
    "dddd",
    "eeeee",
    "ffffff",
    "ggggggg",
    "hhhhhhhh",
    "iiiiiiiii",
    "jjjjjjjjjj",
    };
#[no_mangle]
unsafe extern "C" fn testfunc() -> int __init {
    static int __init testfunc(void)
    {
    char		buf[100];
    unsigned int	i;
    unsigned int	ret;
    struct { unsigned char buf[6]; } hello = { "hello" };
    printk(KERN_INFO "record fifo test start\n");
    kfifo_in(&test, &hello, sizeof(hello));
// show the size of the next record in the fifo
    printk(KERN_INFO "fifo peek len: %u\n", kfifo_peek_len(&test));
// put in variable length data
    for (i = 0; i < 10; i++) {
    memset(buf, 'a' + i, i + 1);
    kfifo_in(&test, buf, i + 1);
    }
// skip first element of the fifo
    printk(KERN_INFO "skip 1st element\n");
    kfifo_skip(&test);
    printk(KERN_INFO "fifo len: %u\n", kfifo_len(&test));
// show the first record without removing from the fifo
    ret = kfifo_out_peek(&test, buf, sizeof(buf));
    if (ret)
    printk(KERN_INFO "%.*s\n", ret, buf);
// check the correctness of all values in the fifo
    i = 0;
    while (!kfifo_is_empty(&test)) {
    ret = kfifo_out(&test, buf, sizeof(buf));
    buf[ret] = '\0';
    printk(KERN_INFO "item = %.*s\n", ret, buf);
    if (strcmp(buf, expected_result[i++])) {
    printk(KERN_WARNING "value mismatch: test failed\n");
    return -EIO;
    }
    }
    if (i != ARRAY_SIZE(expected_result)) {
    printk(KERN_WARNING "size mismatch: test failed\n");
    return -EIO;
    }
    printk(KERN_INFO "test passed\n");
    return 0;
    }
    static ssize_t fifo_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    int ret;
    unsigned int copied;
    if (mutex_lock_interruptible(&write_access))
    return -ERESTARTSYS;
    ret = kfifo_from_user(&test, buf, count, &copied);
    mutex_unlock(&write_access);
    if (ret)
    return ret;
    return copied;
    }
    static ssize_t fifo_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    int ret;
    unsigned int copied;
    if (mutex_lock_interruptible(&read_access))
    return -ERESTARTSYS;
    ret = kfifo_to_user(&test, buf, count, &copied);
    mutex_unlock(&read_access);
    if (ret)
    return ret;
    return copied;
    }
    static const struct proc_ops fifo_proc_ops = {
    .proc_read	= fifo_read,
    .proc_write	= fifo_write,
    .proc_lseek	= noop_llseek,
    };
#[no_mangle]
unsafe extern "C" fn example_init() -> int __init {
    static int __init example_init(void)
    {

    int ret;
    ret = kfifo_alloc(&test, FIFO_SIZE, GFP_KERNEL);
    if (ret) {
    printk(KERN_ERR "error kfifo_alloc\n");
    return ret;
    }

    INIT_KFIFO(test);

    if (testfunc() < 0) {

    kfifo_free(&test);

    return -EIO;
    }
    if (proc_create(PROC_FIFO, 0, core::ptr::null_mut(), &fifo_proc_ops) == core::ptr::null_mut()) {

    kfifo_free(&test);

    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn example_exit() -> void __exit {
    static void __exit example_exit(void)
    {
    remove_proc_entry(PROC_FIFO, core::ptr::null_mut());

    kfifo_free(&test);

    }
    module_init(example_init);
    module_exit(example_exit);
    MODULE_DESCRIPTION("Sample dynamic sized record fifo implementation");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Stefani Seibold <stefani@seibold.net>");
