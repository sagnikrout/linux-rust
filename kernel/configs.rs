//! Automatically rewritten from C to Rust
//! Source: kernel/configs.c
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
// kernel/configs.c
// Echo the kernel .config file used to build the kernel
//
// Copyright (C) 2002 Khalid Aziz <khalid_aziz@hp.com>
// Copyright (C) 2002 Randy Dunlap <rdunlap@xenotime.net>
// Copyright (C) 2002 Al Stone <ahs3@fc.hp.com>
// Copyright (C) 2002 Hewlett-Packard Company
//

//
// "IKCFG_ST" and "IKCFG_ED" are used to extract the config data from
// a binary kernel image or a module. See scripts/extract-ikconfig.
//
    asm (
    "	.pushsection .rodata, \"a\"		\n"
    "	.ascii \"IKCFG_ST\"			\n"
    "	.global kernel_config_data		\n"
    "kernel_config_data:				\n"
    "	.incbin \"kernel/config_data.gz\"	\n"
    "	.global kernel_config_data_end		\n"
    "kernel_config_data_end:			\n"
    "	.ascii \"IKCFG_ED\"			\n"
    "	.popsection				\n"
    );

    extern char kernel_config_data;
    extern char kernel_config_data_end;
    static ssize_t
    ikconfig_read_current(struct file *file, char __user *buf,
    size_t len, loff_t * offset)
    {
    return simple_read_from_buffer(buf, len, offset,
    &kernel_config_data,
    &kernel_config_data_end -
    &kernel_config_data);
    }
    static const struct proc_ops config_gz_proc_ops = {
    .proc_read	= ikconfig_read_current,
    .proc_lseek	= default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn ikconfig_init() -> int __init {
    static int __init ikconfig_init(void)
    {
    struct proc_dir_entry *entry;
// create the current config file
    entry = proc_create("config.gz", S_IFREG | S_IRUGO, core::ptr::null_mut(),
    &config_gz_proc_ops);
    if (!entry)
    return -ENOMEM;
    proc_set_size(entry, &kernel_config_data_end - &kernel_config_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ikconfig_cleanup() -> void __exit {
    static void __exit ikconfig_cleanup(void)
    {
    remove_proc_entry("config.gz", core::ptr::null_mut());
    }
    module_init(ikconfig_init);
    module_exit(ikconfig_cleanup);

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Randy Dunlap");
    MODULE_DESCRIPTION("Echo the kernel .config file used to build the kernel");
