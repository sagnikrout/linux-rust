//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/chips/map_rom.c
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
// Common code to handle map devices which are simple ROM
// (C) 2000 Red Hat.
//

    static int maprom_read (struct mtd_info *, loff_t, size_t, size_t *, u_char *);
    static int maprom_write (struct mtd_info *, loff_t, size_t, size_t *, const u_char *);
    static void maprom_nop (struct mtd_info *);
    static struct mtd_info *map_rom_probe(struct map_info *map);
    static int maprom_erase (struct mtd_info *mtd, struct erase_info *info);
    static int maprom_point (struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys);
    static int maprom_unpoint(struct mtd_info *mtd, loff_t from, size_t len);
    static struct mtd_chip_driver maprom_chipdrv = {
    .probe	= map_rom_probe,
    .name	= "map_rom",
    .module	= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn default_erasesize(map: *mut map_info) -> c_uint {
    static unsigned int default_erasesize(struct map_info *map)
    {
    const __be32 *erase_size = core::ptr::null_mut();
    erase_size = of_get_property(map.device_node, "erase-size", core::ptr::null_mut());
    return !erase_size ? map.size : be32_to_cpu(*erase_size);
    }
    static struct mtd_info *map_rom_probe(struct map_info *map)
    {
    struct mtd_info *mtd;
    mtd = kzalloc_obj(*mtd);
    if (!mtd)
    return core::ptr::null_mut();
    map.fldrv = &maprom_chipdrv;
    mtd.priv = map;
    mtd.name = map.name;
    mtd.type = MTD_ROM;
    mtd.size = map.size;
    mtd._point = maprom_point;
    mtd._unpoint = maprom_unpoint;
    mtd._read = maprom_read;
    mtd._write = maprom_write;
    mtd._sync = maprom_nop;
    mtd._erase = maprom_erase;
    mtd.flags = MTD_CAP_ROM;
    mtd.erasesize = default_erasesize(map);
    mtd.writesize = 1;
    mtd.writebufsize = 1;
    __module_get(THIS_MODULE);
    return mtd;
    }
    static int maprom_point(struct mtd_info *mtd, loff_t from, size_t len,
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
unsafe extern "C" fn maprom_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int {
    static int maprom_unpoint(struct mtd_info *mtd, loff_t from, size_t len)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn maprom_read(mtd: *mut mtd_info, from: loff_t, len: usize, retlen: *mut usize, buf: *mut u_char) -> c_int {
    static int maprom_read (struct mtd_info *mtd, loff_t from, size_t len, size_t *retlen, u_char *buf)
    {
    struct map_info *map = mtd.priv;
    map_copy_from(map, buf, from, len);
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn maprom_nop(mtd: *mut mtd_info) {
    static void maprom_nop(struct mtd_info *mtd)
    {
// Nothing to see here
    }
#[no_mangle]
unsafe extern "C" fn maprom_write(mtd: *mut mtd_info, to: loff_t, len: usize, retlen: *mut usize, buf: *const u_char) -> c_int {
    static int maprom_write (struct mtd_info *mtd, loff_t to, size_t len, size_t *retlen, const u_char *buf)
    {
    return -EROFS;
    }
#[no_mangle]
unsafe extern "C" fn maprom_erase(mtd: *mut mtd_info, info: *mut erase_info) -> c_int {
    static int maprom_erase (struct mtd_info *mtd, struct erase_info *info)
    {
// We do our best 8)
    return -EROFS;
    }
#[no_mangle]
unsafe extern "C" fn map_rom_init() -> int __init {
    static int __init map_rom_init(void)
    {
    register_mtd_chip_driver(&maprom_chipdrv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_rom_exit() -> void __exit {
    static void __exit map_rom_exit(void)
    {
    unregister_mtd_chip_driver(&maprom_chipdrv);
    }
    module_init(map_rom_init);
    module_exit(map_rom_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("MTD chip driver for ROM chips");
