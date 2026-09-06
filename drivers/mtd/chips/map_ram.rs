//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/chips/map_ram.c
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
// Common code to handle map devices which are simple RAM
// (C) 2000 Red Hat.
//

    static int mapram_read (struct mtd_info *, loff_t, size_t, size_t *, u_char *);
    static int mapram_write (struct mtd_info *, loff_t, size_t, size_t *, const u_char *);
    static int mapram_erase (struct mtd_info *, struct erase_info *);
    static void mapram_nop (struct mtd_info *);
    static struct mtd_info *map_ram_probe(struct map_info *map);
    static int mapram_point (struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys);
    static int mapram_unpoint(struct mtd_info *mtd, loff_t from, size_t len);
    static struct mtd_chip_driver mapram_chipdrv = {
    .probe	= map_ram_probe,
    .name	= "map_ram",
    .module	= THIS_MODULE
    };
    static struct mtd_info *map_ram_probe(struct map_info *map)
    {
    struct mtd_info *mtd;
// Check the first byte is RAM

    map_write8(map, 0x55, 0);
    if (map_read8(map, 0) != 0x55)
    return core::ptr::null_mut();
    map_write8(map, 0xAA, 0);
    if (map_read8(map, 0) != 0xAA)
    return core::ptr::null_mut();
// Check the last byte is RAM
    map_write8(map, 0x55, map.size-1);
    if (map_read8(map, map.size-1) != 0x55)
    return core::ptr::null_mut();
    map_write8(map, 0xAA, map.size-1);
    if (map_read8(map, map.size-1) != 0xAA)
    return core::ptr::null_mut();

// OK. It seems to be RAM.
    mtd = kzalloc_obj(*mtd);
    if (!mtd)
    return core::ptr::null_mut();
    map.fldrv = &mapram_chipdrv;
    mtd.priv = map;
    mtd.name = map.name;
    mtd.type = MTD_RAM;
    mtd.size = map.size;
    mtd._erase = mapram_erase;
    mtd._read = mapram_read;
    mtd._write = mapram_write;
    mtd._panic_write = mapram_write;
    mtd._sync = mapram_nop;
    mtd.flags = MTD_CAP_RAM;
    mtd.writesize = 1;
// Disable direct access when NO_XIP is set
    if (map.phys != NO_XIP) {
    mtd._point = mapram_point;
    mtd._unpoint = mapram_unpoint;
    }
    mtd.erasesize = PAGE_SIZE;
    while(mtd.size & (mtd.erasesize - 1))
    mtd.erasesize >>= 1;
    __module_get(THIS_MODULE);
    return mtd;
    }
    static int mapram_point(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys)
    {
    struct map_info *map = mtd.priv;
    if (!map.virt)
    return -EINVAL;
// virt = map->virt + from;
    if (phys)
// phys = map->phys + from;
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mapram_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int {
    static int mapram_unpoint(struct mtd_info *mtd, loff_t from, size_t len)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mapram_read(mtd: *mut mtd_info, from: loff_t, len: usize, retlen: *mut usize, buf: *mut u_char) -> c_int {
    static int mapram_read (struct mtd_info *mtd, loff_t from, size_t len, size_t *retlen, u_char *buf)
    {
    struct map_info *map = mtd.priv;
    map_copy_from(map, buf, from, len);
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mapram_write(mtd: *mut mtd_info, to: loff_t, len: usize, retlen: *mut usize, buf: *const u_char) -> c_int {
    static int mapram_write (struct mtd_info *mtd, loff_t to, size_t len, size_t *retlen, const u_char *buf)
    {
    struct map_info *map = mtd.priv;
    map_copy_to(map, to, buf, len);
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mapram_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int mapram_erase (struct mtd_info *mtd, struct erase_info *instr)
    {
// Yeah, it's inefficient. Who cares? It's faster than a _real_
    flash erase. */
    struct map_info *map = mtd.priv;
    map_word allff;
    unsigned long i;
    allff = map_word_ff(map);
    for (i=0; i<instr.len; i += map_bankwidth(map))
    map_write(map, allff, instr.addr + i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mapram_nop(mtd: *mut mtd_info) {
    static void mapram_nop(struct mtd_info *mtd)
    {
// Nothing to see here
    }
#[no_mangle]
unsafe extern "C" fn map_ram_init() -> int __init {
    static int __init map_ram_init(void)
    {
    register_mtd_chip_driver(&mapram_chipdrv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_ram_exit() -> void __exit {
    static void __exit map_ram_exit(void)
    {
    unregister_mtd_chip_driver(&mapram_chipdrv);
    }
    module_init(map_ram_init);
    module_exit(map_ram_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("MTD chip driver for RAM chips");
