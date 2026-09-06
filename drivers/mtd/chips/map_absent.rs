//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/chips/map_absent.c
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
// Common code to handle absent "placeholder" devices
// Copyright 2001 Resilience Corporation <ebrower@resilience.com>
//
// This map driver is used to allocate "placeholder" MTD
// devices on systems that have socketed/removable media.
// Use of this driver as a fallback preserves the expected
// registration of MTD device nodes regardless of probe outcome.
// A usage example is as follows:
//
// my_dev[i] = do_map_probe("cfi", &my_map[i]);
// if(NULL == my_dev[i]) {
// my_dev[i] = do_map_probe("map_absent", &my_map[i]);
// }
//
// Any device 'probed' with this driver will return -ENODEV
// upon open.
//

    static int map_absent_read (struct mtd_info *, loff_t, size_t, size_t *, u_char *);
    static int map_absent_write (struct mtd_info *, loff_t, size_t, size_t *, const u_char *);
    static int map_absent_erase (struct mtd_info *, struct erase_info *);
    static void map_absent_sync (struct mtd_info *);
    static struct mtd_info *map_absent_probe(struct map_info *map);
    static void map_absent_destroy (struct mtd_info *);
    static struct mtd_chip_driver map_absent_chipdrv = {
    .probe		= map_absent_probe,
    .destroy	= map_absent_destroy,
    .name		= "map_absent",
    .module		= THIS_MODULE
    };
    static struct mtd_info *map_absent_probe(struct map_info *map)
    {
    struct mtd_info *mtd;
    mtd = kzalloc_obj(*mtd);
    if (!mtd) {
    return core::ptr::null_mut();
    }
    map.fldrv 	= &map_absent_chipdrv;
    mtd.priv 	= map;
    mtd.name 	= map.name;
    mtd.type 	= MTD_ABSENT;
    mtd.size 	= map.size;
    mtd._erase 	= map_absent_erase;
    mtd._read 	= map_absent_read;
    mtd._write 	= map_absent_write;
    mtd._sync 	= map_absent_sync;
    mtd.flags 	= 0;
    mtd.erasesize  = PAGE_SIZE;
    mtd.writesize  = 1;
    __module_get(THIS_MODULE);
    return mtd;
    }
#[no_mangle]
unsafe extern "C" fn map_absent_read(mtd: *mut mtd_info, from: loff_t, len: usize, retlen: *mut usize, buf: *mut u_char) -> c_int {
    static int map_absent_read(struct mtd_info *mtd, loff_t from, size_t len, size_t *retlen, u_char *buf)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn map_absent_write(mtd: *mut mtd_info, to: loff_t, len: usize, retlen: *mut usize, buf: *const u_char) -> c_int {
    static int map_absent_write(struct mtd_info *mtd, loff_t to, size_t len, size_t *retlen, const u_char *buf)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn map_absent_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int map_absent_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn map_absent_sync(mtd: *mut mtd_info) {
    static void map_absent_sync(struct mtd_info *mtd)
    {
// nop
    }
#[no_mangle]
unsafe extern "C" fn map_absent_destroy(mtd: *mut mtd_info) {
    static void map_absent_destroy(struct mtd_info *mtd)
    {
// nop
    }
#[no_mangle]
unsafe extern "C" fn map_absent_init() -> int __init {
    static int __init map_absent_init(void)
    {
    register_mtd_chip_driver(&map_absent_chipdrv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_absent_exit() -> void __exit {
    static void __exit map_absent_exit(void)
    {
    unregister_mtd_chip_driver(&map_absent_chipdrv);
    }
    module_init(map_absent_init);
    module_exit(map_absent_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Resilience Corporation - Eric Brower <ebrower@resilience.com>");
    MODULE_DESCRIPTION("Placeholder MTD chip driver for 'absent' chips");
