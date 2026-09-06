//! Automatically rewritten from C to Rust
//! Source: init/do_mounts_initrd.c
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

    unsigned long initrd_start, initrd_end;
    int initrd_below_start_ok;
    let mut mount_initrd: static int __initdata = 1;
    phys_addr_t phys_initrd_start __initdata;
    unsigned long phys_initrd_size __initdata;
#[no_mangle]
unsafe extern "C" fn no_initrd(str: *mut c_char) -> int __init {
    static int __init no_initrd(char *str)
    {
    pr_warn("noinitrd option is deprecated and will be removed soon\n");
    mount_initrd = 0;
    return 1;
    }
    __setup("noinitrd", no_initrd);
#[no_mangle]
unsafe extern "C" fn early_initrdmem(p: *mut c_char) -> int __init {
    static int __init early_initrdmem(char *p)
    {
    phys_addr_t start;
    unsigned long size;
    char *endp;
    start = memparse(p, &endp);
    if (*endp == ',') {
    size = memparse(endp + 1, core::ptr::null_mut());
    phys_initrd_start = start;
    phys_initrd_size = size;
    }
    return 0;
    }
    early_param("initrdmem", early_initrdmem);
#[no_mangle]
unsafe extern "C" fn early_initrd(p: *mut c_char) -> int __init {
    static int __init early_initrd(char *p)
    {
    return early_initrdmem(p);
    }
    early_param("initrd", early_initrd);
#[no_mangle]
pub unsafe extern "C" fn initrd_load() -> void __init {
    void __init initrd_load(void)
    {
    if (mount_initrd) {
    create_dev("/dev/ram", Root_RAM0);
//
// Load the initrd data into /dev/ram0.
//
    if (rd_load_image()) {
    pr_warn("using deprecated initrd support, will be removed in January 2027; "
    "use initramfs instead or (as a last resort) /sys/firmware/initrd; "
    "see section \"Workaround\" in "
    "https://lore.kernel.org/lkml/20251010094047.3111495-1-safinaskar@gmail.com\n");
    }
    }
    init_unlink("/initrd.image");
    }
