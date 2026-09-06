//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/google/memconsole-x86-legacy.c
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
// memconsole-x86-legacy.c
//
// EBDA specific parts of the memory based BIOS console.
//
// Copyright 2017 Google Inc.
//

pub const BIOS_MEMCONSOLE_V1_MAGIC: c_uint = 0xDEADBABE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct biosmemcon_ebda {
    pub signature: u32,
    union {
    struct {
    pub enabled: u8,
    pub buffer_addr: u32,
    pub start: u16,
    pub end: u16,
    pub num_chars: u16,
    pub wrapped: u8,
    pub v1: } __packed,
    struct {
    pub buffer_addr: u32,
// Misdocumented as number of pages!
    pub num_bytes: u16,
    pub start: u16,
    pub end: u16,
    pub v2: } __packed,
}

    } __packed;
    static char *memconsole_baseaddr;
    static size_t memconsole_length;
#[no_mangle]
unsafe extern "C" fn memconsole_read(buf: *mut c_char, pos: loff_t, count: usize) -> isize {
    static ssize_t memconsole_read(char *buf, loff_t pos, size_t count)
    {
    return memory_read_from_buffer(buf, count, &pos, memconsole_baseaddr,
    memconsole_length);
    }
#[no_mangle]
unsafe extern "C" fn found_v1_header(hdr: *mut biosmemcon_ebda) {
    static void found_v1_header(struct biosmemcon_ebda *hdr)
    {
    pr_info("memconsole: BIOS console v1 EBDA structure found at %p\n",
    hdr);
    pr_info("memconsole: BIOS console buffer at 0x%.8x, start = %d, end = %d, num = %d\n",
    hdr.v1.buffer_addr, hdr.v1.start,
    hdr.v1.end, hdr.v1.num_chars);
    memconsole_baseaddr = phys_to_virt(hdr.v1.buffer_addr);
    memconsole_length = hdr.v1.num_chars;
    memconsole_setup(memconsole_read);
    }
#[no_mangle]
unsafe extern "C" fn found_v2_header(hdr: *mut biosmemcon_ebda) {
    static void found_v2_header(struct biosmemcon_ebda *hdr)
    {
    pr_info("memconsole: BIOS console v2 EBDA structure found at %p\n",
    hdr);
    pr_info("memconsole: BIOS console buffer at 0x%.8x, start = %d, end = %d, num_bytes = %d\n",
    hdr.v2.buffer_addr, hdr.v2.start,
    hdr.v2.end, hdr.v2.num_bytes);
    memconsole_baseaddr = phys_to_virt(hdr.v2.buffer_addr + hdr.v2.start);
    memconsole_length = hdr.v2.end - hdr.v2.start;
    memconsole_setup(memconsole_read);
    }
//
// Search through the EBDA for the BIOS Memory Console, and
// set the global variables to point to it.  Return true if found.
//
#[no_mangle]
unsafe extern "C" fn memconsole_ebda_init() -> bool {
    static bool memconsole_ebda_init(void)
    {
    unsigned int address;
    size_t length, cur;
    address = get_bios_ebda();
    if (!address) {
    pr_info("memconsole: BIOS EBDA non-existent.\n");
    return false;
    }
// EBDA length is byte 0 of EBDA (in KB)
    length = *(u8 *)phys_to_virt(address);
    length <<= 10; /* convert to bytes */
//
// Search through EBDA for BIOS memory console structure
// note: signature is not necessarily dword-aligned
//
    for (cur = 0; cur < length; cur++) {
    struct biosmemcon_ebda *hdr = phys_to_virt(address + cur);
// memconsole v1
    if (hdr.signature == BIOS_MEMCONSOLE_V1_MAGIC) {
    found_v1_header(hdr);
    return true;
    }
// memconsole v2
    if (hdr.signature == BIOS_MEMCONSOLE_V2_MAGIC) {
    found_v2_header(hdr);
    return true;
    }
    }
    pr_info("memconsole: BIOS console EBDA structure not found!\n");
    return false;
    }
    static const struct dmi_system_id memconsole_dmi_table[] __initconst = {
    {
    .ident = "Google Board",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Google, Inc."),
    },
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, memconsole_dmi_table);
#[no_mangle]
unsafe extern "C" fn memconsole_find() -> bool __init {
    static bool __init memconsole_find(void)
    {
    if (!dmi_check_system(memconsole_dmi_table))
    return false;
    return memconsole_ebda_init();
    }
#[no_mangle]
unsafe extern "C" fn memconsole_x86_init() -> int __init {
    static int __init memconsole_x86_init(void)
    {
    if (!memconsole_find())
    return -ENODEV;
    return memconsole_sysfs_init();
    }
#[no_mangle]
unsafe extern "C" fn memconsole_x86_exit() -> void __exit {
    static void __exit memconsole_x86_exit(void)
    {
    memconsole_exit();
    }
    module_init(memconsole_x86_init);
    module_exit(memconsole_x86_exit);
    MODULE_AUTHOR("Google, Inc.");
    MODULE_DESCRIPTION("EBDA specific parts of the memory based BIOS console.");
    MODULE_LICENSE("GPL");
