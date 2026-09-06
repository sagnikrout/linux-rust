//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/sbc_gxx.c
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
// sbc_gxx.c -- MTD map driver for Arcom Control Systems SBC-MediaGX,
    SBC-GXm and SBC-GX1 series boards.
    Copyright (C) 2001 Arcom Control System Ltd
    The SBC-MediaGX / SBC-GXx has up to 16 MiB of
    Intel StrataFlash (28F320/28F640) in x8 mode.
    This driver uses the CFI probe and Intel Extended Command Set drivers.
    The flash is accessed as follows:
    16 KiB memory window at 0xdc000-0xdffff
    Two IO address locations for paging
    0x258
    bit 0-7: address bit 14-21
    0x259
    bit 0-1: address bit 22-23
    bit 7:   0 - reset/powered down
    1 - device enabled
    The single flash device is divided into 3 partition which appear as
    separate MTD devices.
    25/04/2001 AJL (Arcom)  Modified signon strings and partition sizes
    (to support bzImages up to 638KiB-ish)
//
// Includes

// Defines
// - Hardware specific
pub const WINDOW_START: c_uint = 0xdc000;
// Number of bits in offset.
pub const WINDOW_SHIFT: c_int = 14;

// The bits for the offset into the window.

pub const PAGE_IO: c_uint = 0x258;
pub const PAGE_IO_SIZE: c_int = 2;
// bit 7 of 0x259 must be 1 to enable device.
pub const DEVICE_ENABLE: c_uint = 0x8000;
// - Flash / Partition sizing
pub const MAX_SIZE_KiB: c_int = 16384;
pub const BOOT_PARTITION_SIZE_KiB: c_int = 768;
pub const DATA_PARTITION_SIZE_KiB: c_int = 1280;
pub const APP_PARTITION_SIZE_KiB: c_int = 6144;
// Globals
    static volatile int page_in_window = -1; // Current page in window.
    static void __iomem *iomapadr;
    static DEFINE_SPINLOCK(sbc_gxx_spin);
// partition_info gives details on the logical partitions that the split the
// single flash device into. If the size if zero we use up to the end of the
// device.
    static const struct mtd_partition partition_info[] = {
    { .name = "SBC-GXx flash boot partition",
    .offset = 0,
    .size =   BOOT_PARTITION_SIZE_KiB*1024 },
    { .name = "SBC-GXx flash data partition",
    .offset = BOOT_PARTITION_SIZE_KiB*1024,
    .size = (DATA_PARTITION_SIZE_KiB)*1024 },
    { .name = "SBC-GXx flash application partition",
    .offset = (BOOT_PARTITION_SIZE_KiB+DATA_PARTITION_SIZE_KiB)*1024 }
    };
pub const NUM_PARTITIONS: c_int = 3;
#[no_mangle]
pub unsafe extern "C" fn sbc_gxx_page(map: *mut map_info, ofs: c_ulong) {
    static inline void sbc_gxx_page(struct map_info *map, unsigned long ofs)
    {
    let mut page: c_ulong = ofs >> WINDOW_SHIFT;
    if( page!=page_in_window ) {
    outw( page | DEVICE_ENABLE, PAGE_IO );
    page_in_window = page;
    }
    }
#[no_mangle]
unsafe extern "C" fn sbc_gxx_read8(map: *mut map_info, ofs: c_ulong) -> map_word {
    static map_word sbc_gxx_read8(struct map_info *map, unsigned long ofs)
    {
    map_word ret;
    spin_lock(&sbc_gxx_spin);
    sbc_gxx_page(map, ofs);
    ret.x[0] = readb(iomapadr + (ofs & WINDOW_MASK));
    spin_unlock(&sbc_gxx_spin);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sbc_gxx_copy_from(map: *mut map_info, to: *mut c_void, from: c_ulong, len: isize) {
    static void sbc_gxx_copy_from(struct map_info *map, void *to, unsigned long from, ssize_t len)
    {
    while(len) {
    let mut thislen: c_ulong = len;
    if (len > (WINDOW_LENGTH - (from & WINDOW_MASK)))
    thislen = WINDOW_LENGTH-(from & WINDOW_MASK);
    spin_lock(&sbc_gxx_spin);
    sbc_gxx_page(map, from);
    memcpy_fromio(to, iomapadr + (from & WINDOW_MASK), thislen);
    spin_unlock(&sbc_gxx_spin);
    to += thislen;
    from += thislen;
    len -= thislen;
    }
    }
#[no_mangle]
unsafe extern "C" fn sbc_gxx_write8(map: *mut map_info, d: map_word, adr: c_ulong) {
    static void sbc_gxx_write8(struct map_info *map, map_word d, unsigned long adr)
    {
    spin_lock(&sbc_gxx_spin);
    sbc_gxx_page(map, adr);
    writeb(d.x[0], iomapadr + (adr & WINDOW_MASK));
    spin_unlock(&sbc_gxx_spin);
    }
#[no_mangle]
unsafe extern "C" fn sbc_gxx_copy_to(map: *mut map_info, to: c_ulong, from: *const c_void, len: isize) {
    static void sbc_gxx_copy_to(struct map_info *map, unsigned long to, const void *from, ssize_t len)
    {
    while(len) {
    let mut thislen: c_ulong = len;
    if (len > (WINDOW_LENGTH - (to & WINDOW_MASK)))
    thislen = WINDOW_LENGTH-(to & WINDOW_MASK);
    spin_lock(&sbc_gxx_spin);
    sbc_gxx_page(map, to);
    memcpy_toio(iomapadr + (to & WINDOW_MASK), from, thislen);
    spin_unlock(&sbc_gxx_spin);
    to += thislen;
    from += thislen;
    len -= thislen;
    }
    }
    static struct map_info sbc_gxx_map = {
    .name = "SBC-GXx flash",
    .phys = NO_XIP,
    .size = MAX_SIZE_KiB*1024, /* this must be set to a maximum possible amount
    of flash so the cfi probe routines find all
    the chips */
    .bankwidth = 1,
    .read = sbc_gxx_read8,
    .copy_from = sbc_gxx_copy_from,
    .write = sbc_gxx_write8,
    .copy_to = sbc_gxx_copy_to
    };
// MTD device for all of the flash.
    static struct mtd_info *all_mtd;
#[no_mangle]
unsafe extern "C" fn cleanup_sbc_gxx() {
    static void cleanup_sbc_gxx(void)
    {
    if( all_mtd ) {
    mtd_device_unregister(all_mtd);
    map_destroy( all_mtd );
    }
    iounmap(iomapadr);
    release_region(PAGE_IO,PAGE_IO_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn init_sbc_gxx() -> int __init {
    static int __init init_sbc_gxx(void)
    {
    iomapadr = ioremap(WINDOW_START, WINDOW_LENGTH);
    if (!iomapadr) {
    printk( KERN_ERR"%s: failed to ioremap memory region\n",
    sbc_gxx_map.name );
    return -EIO;
    }
    if (!request_region( PAGE_IO, PAGE_IO_SIZE, "SBC-GXx flash")) {
    printk( KERN_ERR"%s: IO ports 0x%x-0x%x in use\n",
    sbc_gxx_map.name,
    PAGE_IO, PAGE_IO+PAGE_IO_SIZE-1 );
    iounmap(iomapadr);
    return -EAGAIN;
    }
    printk( KERN_INFO"%s: IO:0x%x-0x%x MEM:0x%x-0x%x\n",
    sbc_gxx_map.name,
    PAGE_IO, PAGE_IO+PAGE_IO_SIZE-1,
    WINDOW_START, WINDOW_START+WINDOW_LENGTH-1 );
// Probe for chip.
    all_mtd = do_map_probe( "cfi_probe", &sbc_gxx_map );
    if( !all_mtd ) {
    cleanup_sbc_gxx();
    return -ENXIO;
    }
    all_mtd.owner = THIS_MODULE;
// Create MTD devices for each partition.
    mtd_device_register(all_mtd, partition_info, NUM_PARTITIONS);
    return 0;
    }
    module_init(init_sbc_gxx);
    module_exit(cleanup_sbc_gxx);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Arcom Control Systems Ltd.");
    MODULE_DESCRIPTION("MTD map driver for SBC-GXm and SBC-GX1 series boards");
