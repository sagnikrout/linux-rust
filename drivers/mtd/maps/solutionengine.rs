//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/solutionengine.c
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
// Flash and EPROM on Hitachi Solution Engine and similar boards.
//
// (C) 2001 Red Hat, Inc.
//
// GPL'd
//

    static struct mtd_info *flash_mtd;
    static struct mtd_info *eprom_mtd;
    struct map_info soleng_eprom_map = {
    .name = "Solution Engine EPROM",
    .size = 0x400000,
    .bankwidth = 4,
    };
    struct map_info soleng_flash_map = {
    .name = "Solution Engine FLASH",
    .size = 0x400000,
    .bankwidth = 4,
    };
    static const char * const probes[] = { "RedBoot", "cmdlinepart", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn init_soleng_maps() -> int __init {
    static int __init init_soleng_maps(void)
    {
// First probe at offset 0
    soleng_flash_map.phys = 0;
    soleng_flash_map.virt = (void __iomem *)P2SEGADDR(0);
    soleng_eprom_map.phys = 0x01000000;
    soleng_eprom_map.virt = (void __iomem *)P1SEGADDR(0x01000000);
    simple_map_init(&soleng_eprom_map);
    simple_map_init(&soleng_flash_map);
    printk(KERN_NOTICE "Probing for flash chips at 0x00000000:\n");
    flash_mtd = do_map_probe("cfi_probe", &soleng_flash_map);
    if (!flash_mtd) {
// Not there. Try swapping
    printk(KERN_NOTICE "Probing for flash chips at 0x01000000:\n");
    soleng_flash_map.phys = 0x01000000;
    soleng_flash_map.virt = P2SEGADDR(0x01000000);
    soleng_eprom_map.phys = 0;
    soleng_eprom_map.virt = P1SEGADDR(0);
    flash_mtd = do_map_probe("cfi_probe", &soleng_flash_map);
    if (!flash_mtd) {
// Eep.
    printk(KERN_NOTICE "Flash chips not detected at either possible location.\n");
    return -ENXIO;
    }
    }
    printk(KERN_NOTICE "Solution Engine: Flash at 0x%pap, EPROM at 0x%pap\n",
    &soleng_flash_map.phys,
    &soleng_eprom_map.phys);
    flash_mtd.owner = THIS_MODULE;
    eprom_mtd = do_map_probe("map_rom", &soleng_eprom_map);
    if (eprom_mtd) {
    eprom_mtd.owner = THIS_MODULE;
    mtd_device_register(eprom_mtd, core::ptr::null_mut(), 0);
    }
    mtd_device_parse_register(flash_mtd, probes, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_soleng_maps() -> void __exit {
    static void __exit cleanup_soleng_maps(void)
    {
    if (eprom_mtd) {
    mtd_device_unregister(eprom_mtd);
    map_destroy(eprom_mtd);
    }
    mtd_device_unregister(flash_mtd);
    map_destroy(flash_mtd);
    }
    module_init(init_soleng_maps);
    module_exit(cleanup_soleng_maps);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("MTD map driver for Hitachi SolutionEngine (and similar) boards");
