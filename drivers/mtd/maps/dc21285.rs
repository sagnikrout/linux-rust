//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/dc21285.c
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
// MTD map driver for flash on the DC21285 (the StrongARM-110 companion chip)
//
// (C) 2000  Nicolas Pitre <nico@fluxnic.net>
//
// This code is GPL
//

    static struct mtd_info *dc21285_mtd;

//
// This is really ugly, but it seams to be the only
// realiable way to do it, as the cpld state machine
// is unpredictible. So we have a 25us penalty per
// write access.
//
#[no_mangle]
unsafe extern "C" fn nw_en_write() {
    static void nw_en_write(void)
    {
    unsigned long flags;
//
// we want to write a bit pattern XXX1 to Xilinx to enable
// the write gate, which will be open for about the next 2ms.
//
    raw_spin_lock_irqsave(&nw_gpio_lock, flags);
    nw_cpld_modify(CPLD_FLASH_WR_ENABLE, CPLD_FLASH_WR_ENABLE);
    raw_spin_unlock_irqrestore(&nw_gpio_lock, flags);
//
// let the ISA bus to catch on...
//
    udelay(25);
    }

#[no_mangle]
unsafe extern "C" fn dc21285_read8(map: *mut map_info, ofs: c_ulong) -> map_word {
    static map_word dc21285_read8(struct map_info *map, unsigned long ofs)
    {
    map_word val;
    val.x[0] = *(uint8_t*)(map.virt + ofs);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dc21285_read16(map: *mut map_info, ofs: c_ulong) -> map_word {
    static map_word dc21285_read16(struct map_info *map, unsigned long ofs)
    {
    map_word val;
    val.x[0] = *(uint16_t*)(map.virt + ofs);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dc21285_read32(map: *mut map_info, ofs: c_ulong) -> map_word {
    static map_word dc21285_read32(struct map_info *map, unsigned long ofs)
    {
    map_word val;
    val.x[0] = *(uint32_t*)(map.virt + ofs);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dc21285_copy_from(map: *mut map_info, to: *mut c_void, from: c_ulong, len: isize) {
    static void dc21285_copy_from(struct map_info *map, void *to, unsigned long from, ssize_t len)
    {
    memcpy(to, (void*)(map.virt + from), len);
    }
#[no_mangle]
unsafe extern "C" fn dc21285_write8(map: *mut map_info, d: map_word, adr: c_ulong) {
    static void dc21285_write8(struct map_info *map, const map_word d, unsigned long adr)
    {
    if (machine_is_netwinder())
    nw_en_write();
// CSR_ROMWRITEREG = adr & 3;
    adr &= ~3;
// (uint8_t*)(map->virt + adr) = d.x[0];
    }
#[no_mangle]
unsafe extern "C" fn dc21285_write16(map: *mut map_info, d: map_word, adr: c_ulong) {
    static void dc21285_write16(struct map_info *map, const map_word d, unsigned long adr)
    {
    if (machine_is_netwinder())
    nw_en_write();
// CSR_ROMWRITEREG = adr & 3;
    adr &= ~3;
// (uint16_t*)(map->virt + adr) = d.x[0];
    }
#[no_mangle]
unsafe extern "C" fn dc21285_write32(map: *mut map_info, d: map_word, adr: c_ulong) {
    static void dc21285_write32(struct map_info *map, const map_word d, unsigned long adr)
    {
    if (machine_is_netwinder())
    nw_en_write();
// (uint32_t*)(map->virt + adr) = d.x[0];
    }
#[no_mangle]
unsafe extern "C" fn dc21285_copy_to_32(map: *mut map_info, to: c_ulong, from: *const c_void, len: isize) {
    static void dc21285_copy_to_32(struct map_info *map, unsigned long to, const void *from, ssize_t len)
    {
    while (len > 0) {
    map_word d;
    d.x[0] = *((uint32_t*)from);
    dc21285_write32(map, d, to);
    from += 4;
    to += 4;
    len -= 4;
    }
    }
#[no_mangle]
unsafe extern "C" fn dc21285_copy_to_16(map: *mut map_info, to: c_ulong, from: *const c_void, len: isize) {
    static void dc21285_copy_to_16(struct map_info *map, unsigned long to, const void *from, ssize_t len)
    {
    while (len > 0) {
    map_word d;
    d.x[0] = *((uint16_t*)from);
    dc21285_write16(map, d, to);
    from += 2;
    to += 2;
    len -= 2;
    }
    }
#[no_mangle]
unsafe extern "C" fn dc21285_copy_to_8(map: *mut map_info, to: c_ulong, from: *const c_void, len: isize) {
    static void dc21285_copy_to_8(struct map_info *map, unsigned long to, const void *from, ssize_t len)
    {
    map_word d;
    d.x[0] = *((uint8_t*)from);
    dc21285_write8(map, d, to);
    from++;
    to++;
    len--;
    }
    static struct map_info dc21285_map = {
    .name = "DC21285 flash",
    .phys = NO_XIP,
    .size = 16*1024*1024,
    .copy_from = dc21285_copy_from,
    };
// Partition stuff
    static const char * const probes[] = { "RedBoot", "cmdlinepart", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn init_dc21285() -> int __init {
    static int __init init_dc21285(void)
    {
// Determine bankwidth
    switch (*CSR_SA110_CNTL & (3<<14)) {
    case SA110_CNTL_ROMWIDTH_8:
    dc21285_map.bankwidth = 1;
    dc21285_map.read = dc21285_read8;
    dc21285_map.write = dc21285_write8;
    dc21285_map.copy_to = dc21285_copy_to_8;
    break;
    case SA110_CNTL_ROMWIDTH_16:
    dc21285_map.bankwidth = 2;
    dc21285_map.read = dc21285_read16;
    dc21285_map.write = dc21285_write16;
    dc21285_map.copy_to = dc21285_copy_to_16;
    break;
    case SA110_CNTL_ROMWIDTH_32:
    dc21285_map.bankwidth = 4;
    dc21285_map.read = dc21285_read32;
    dc21285_map.write = dc21285_write32;
    dc21285_map.copy_to = dc21285_copy_to_32;
    break;
    default:
    printk (KERN_ERR "DC21285 flash: undefined bankwidth\n");
    return -ENXIO;
    }
    printk (KERN_NOTICE "DC21285 flash support (%d-bit bankwidth)\n",
    dc21285_map.bankwidth*8);
// Let's map the flash area
    dc21285_map.virt = ioremap(DC21285_FLASH, 16*1024*1024);
    if (!dc21285_map.virt) {
    printk("Failed to ioremap\n");
    return -EIO;
    }
    if (machine_is_ebsa285()) {
    dc21285_mtd = do_map_probe("cfi_probe", &dc21285_map);
    } else {
    dc21285_mtd = do_map_probe("jedec_probe", &dc21285_map);
    }
    if (!dc21285_mtd) {
    iounmap(dc21285_map.virt);
    return -ENXIO;
    }
    dc21285_mtd.owner = THIS_MODULE;
    mtd_device_parse_register(dc21285_mtd, probes, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if(machine_is_ebsa285()) {
//
// Flash timing is determined with bits 19-16 of the
// CSR_SA110_CNTL.  The value is the number of wait cycles, or
// 0 for 16 cycles (the default).  Cycles are 20 ns.
// Here we use 7 for 140 ns flash chips.
//
// access time
// CSR_SA110_CNTL = ((*CSR_SA110_CNTL & ~0x000f0000) | (7 << 16));
// burst time
// CSR_SA110_CNTL = ((*CSR_SA110_CNTL & ~0x00f00000) | (7 << 20));
// tristate time
// CSR_SA110_CNTL = ((*CSR_SA110_CNTL & ~0x0f000000) | (7 << 24));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_dc21285() -> void __exit {
    static void __exit cleanup_dc21285(void)
    {
    mtd_device_unregister(dc21285_mtd);
    map_destroy(dc21285_mtd);
    iounmap(dc21285_map.virt);
    }
    module_init(init_dc21285);
    module_exit(cleanup_dc21285);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nicolas Pitre <nico@fluxnic.net>");
    MODULE_DESCRIPTION("MTD map driver for DC21285 boards");
