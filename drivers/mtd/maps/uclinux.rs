//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/uclinux.c
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


//
// uclinux.c -- generic memory mapped MTD driver for uclinux
//
// (C) Copyright 2002, Greg Ungerer (gerg@snapgear.com)
//
// License: GPL
//

//

    static struct map_info uclinux_ram_map = {
    .name = MAP_NAME,
    .size = 0,
    };
    let mut physaddr: static unsigned long = -1;
    module_param(physaddr, ulong, S_IRUGO);
    static struct mtd_info *uclinux_ram_mtdinfo;
//
    static const struct mtd_partition uclinux_romfs[] = {
    { .name = "ROMfs" }
    };

//
    static int uclinux_point(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys)
    {
    struct map_info *map = mtd.priv;
// virt = map->virt + from;
    if (phys)
// phys = map->phys + from;
// retlen = len;
    return(0);
    }
//
#[no_mangle]
unsafe extern "C" fn uclinux_mtd_init() -> int __init {
    static int __init uclinux_mtd_init(void)
    {
    struct mtd_info *mtd;
    struct map_info *mapp;
    mapp = &uclinux_ram_map;
    if (physaddr == -1)
    mapp.phys = (resource_size_t)__bss_stop;
    else
    mapp.phys = physaddr;
    if (!mapp.size)
    mapp.size = PAGE_ALIGN(ntohl(*((unsigned long *)(mapp.phys + 8))));
    mapp.bankwidth = 4;
    printk("uclinux[mtd]: probe address=0x%x size=0x%x\n",
    (int) mapp.phys, (int) mapp.size);
//
// The filesystem is guaranteed to be in direct mapped memory. It is
// directly following the kernels own bss region. Following the same
// mechanism used by architectures setting up traditional initrds we
// use phys_to_virt to get the virtual address of its start.
//
    mapp.virt = phys_to_virt(mapp.phys);
    if (mapp.virt == 0) {
    printk("uclinux[mtd]: no virtual mapping?\n");
    return(-EIO);
    }
    simple_map_init(mapp);
    mtd = do_map_probe("map_" MAP_NAME, mapp);
    if (!mtd) {
    printk("uclinux[mtd]: failed to find a mapping?\n");
    return(-ENXIO);
    }
    mtd.owner = THIS_MODULE;
    mtd._point = uclinux_point;
    mtd.priv = mapp;
    uclinux_ram_mtdinfo = mtd;
    mtd_device_register(mtd, uclinux_romfs, NUM_PARTITIONS);
    return(0);
    }
    device_initcall(uclinux_mtd_init);
//
