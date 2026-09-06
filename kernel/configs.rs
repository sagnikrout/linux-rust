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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























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
// asm (
    "	.pushsection .rodata, \"a\"		\n"
    "	.ascii \"IKCFG_ST\"			\n"
    "	.global kernel_config_data		\n"
    "kernel_config_data:				\n"
    "	.incbin \"kernel/config_data.gz\"	\n"
    "	.global kernel_config_data_end		\n"
    "kernel_config_data_end:			\n"
    "	.ascii \"IKCFG_ED\"			\n"
    "	.popsection				\n"
    ); */

extern "C" { pub static mut kernel_config_data: usize; }
extern "C" { pub static mut kernel_config_data_end: usize; }
#[no_mangle]
pub unsafe extern "C" fn ikconfig_read_current() {
    return simple_read_from_buffer(buf, len, offset,
    &kernel_config_data,
    &kernel_config_data_end -
    &kernel_config_data);
    }
pub static mut proc_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ikconfig_init() -> c_int {
    let mut entry = core::ptr::null_mut();
// create the current config file
    entry = proc_create("config.gz", S_IFREG | S_IRUGO, core::ptr::null_mut(),
    &config_gz_proc_ops);
    if (!entry) {
    return -ENOMEM;
    }
    proc_set_size(entry, &kernel_config_data_end - &kernel_config_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ikconfig_cleanup() -> c_int {
    remove_proc_entry("config.gz", core::ptr::null_mut());
    }
// module_init;
// module_exit;

// MODULE_LICENSE;
// MODULE_AUTHOR;
// MODULE_DESCRIPTION;