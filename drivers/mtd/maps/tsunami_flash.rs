//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/tsunami_flash.c
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
// tsunami_flash.c
//
// flash chip on alpha ds10...
//

pub const FLASH_ENABLE_PORT: c_uint = 0x00C00001;
pub const FLASH_ENABLE_BYTE: c_uint = 0x01;
pub const FLASH_DISABLE_BYTE: c_uint = 0x00;

#[no_mangle]
pub unsafe extern "C" fn tsunami_flash_read8(map: *mut map_info, offset: c_ulong) -> map_word {
    static inline map_word tsunami_flash_read8(struct map_info *map, unsigned long offset)
    {
    map_word val;
    val.x[0] = tsunami_tig_readb(offset);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn tsunami_flash_write8(map: *mut map_info, value: map_word, offset: c_ulong) {
    static void tsunami_flash_write8(struct map_info *map, map_word value, unsigned long offset)
    {
    tsunami_tig_writeb(value.x[0], offset);
    }
    static void tsunami_flash_copy_from(
    struct map_info *map, void *addr, unsigned long offset, ssize_t len)
    {
    unsigned char *dest;
    dest = addr;
    while(len && (offset < MAX_TIG_FLASH_SIZE)) {
// dest = tsunami_tig_readb(offset);
    offset++;
    dest++;
    len--;
    }
    }
    static void tsunami_flash_copy_to(
    struct map_info *map, unsigned long offset,
    const void *addr, ssize_t len)
    {
    const unsigned char *src;
    src = addr;
    while(len && (offset < MAX_TIG_FLASH_SIZE)) {
    tsunami_tig_writeb(*src, offset);
    offset++;
    src++;
    len--;
    }
    }
//
// Deliberately don't provide operations wider than 8 bits.  I don't
// have then and it scares me to think how you could mess up if
// you tried to use them.   Buswidth is correctly so I'm safe.
//
    static struct map_info tsunami_flash_map = {
    .name = "flash chip on the Tsunami TIG bus",
    .size = MAX_TIG_FLASH_SIZE,
    .phys = NO_XIP,
    .bankwidth = 1,
    .read = tsunami_flash_read8,
    .copy_from = tsunami_flash_copy_from,
    .write = tsunami_flash_write8,
    .copy_to = tsunami_flash_copy_to,
    };
    static struct mtd_info *tsunami_flash_mtd;
#[no_mangle]
unsafe extern "C" fn cleanup_tsunami_flash() -> void __exit {
    static void __exit  cleanup_tsunami_flash(void)
    {
    struct mtd_info *mtd;
    mtd = tsunami_flash_mtd;
    if (mtd) {
    mtd_device_unregister(mtd);
    map_destroy(mtd);
    }
    tsunami_flash_mtd = 0;
    }
    static const char * const rom_probe_types[] = {
    "cfi_probe", "jedec_probe", "map_rom", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn init_tsunami_flash() -> int __init {
    static int __init init_tsunami_flash(void)
    {
    const char * const *type;
    tsunami_tig_writeb(FLASH_ENABLE_BYTE, FLASH_ENABLE_PORT);
    tsunami_flash_mtd = 0;
    type = rom_probe_types;
    for(; !tsunami_flash_mtd && *type; type++) {
    tsunami_flash_mtd = do_map_probe(*type, &tsunami_flash_map);
    }
    if (tsunami_flash_mtd) {
    tsunami_flash_mtd.owner = THIS_MODULE;
    mtd_device_register(tsunami_flash_mtd, core::ptr::null_mut(), 0);
    return 0;
    }
    return -ENXIO;
    }
    module_init(init_tsunami_flash);
    module_exit(cleanup_tsunami_flash);
