//! Automatically rewritten from C to Rust
//! Source: kernel/kheaders.c
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
// Provide kernel headers useful to build tracing programs
// such as for running eBPF tracing tools.
//
// (Borrowed code from kernel/configs.c)
//

//
// Define kernel_headers_data and kernel_headers_data_end, within which the
// compressed kernel headers are stored. The file is first compressed with xz.
//
    asm (
    "	.pushsection .rodata, \"a\"		\n"
    "	.global kernel_headers_data		\n"
    "kernel_headers_data:				\n"
    "	.incbin \"kernel/kheaders_data.tar.xz\"	\n"
    "	.global kernel_headers_data_end		\n"
    "kernel_headers_data_end:			\n"
    "	.popsection				\n"
    );
    extern char kernel_headers_data[];
    extern char kernel_headers_data_end[];
    static struct bin_attribute kheaders_attr __ro_after_init =
    __BIN_ATTR_SIMPLE_RO(kheaders.tar.xz, 0444);
#[no_mangle]
unsafe extern "C" fn ikheaders_init() -> int __init {
    static int __init ikheaders_init(void)
    {
    kheaders_attr.private = kernel_headers_data;
    kheaders_attr.size = (kernel_headers_data_end -
    kernel_headers_data);
    return sysfs_create_bin_file(kernel_kobj, &kheaders_attr);
    }
#[no_mangle]
unsafe extern "C" fn ikheaders_cleanup() -> void __exit {
    static void __exit ikheaders_cleanup(void)
    {
    sysfs_remove_bin_file(kernel_kobj, &kheaders_attr);
    }
    module_init(ikheaders_init);
    module_exit(ikheaders_cleanup);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Joel Fernandes");
    MODULE_DESCRIPTION("Echo the kernel header artifacts used to build the kernel");
