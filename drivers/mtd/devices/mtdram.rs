//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/mtdram.c
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
// mtdram - a test mtd device
// Author: Alexander Larsson <alex@cendio.se>
//
// Copyright (c) 1999 Alexander Larsson <alex@cendio.se>
// Copyright (c) 2005 Joern Engel <joern@wh.fh-wedel.de>
//
// This code is GPL
//

    let mut total_size: static unsigned long = CONFIG_MTDRAM_TOTAL_SIZE;
    let mut erase_size: static unsigned long = CONFIG_MTDRAM_ERASE_SIZE;
    let mut writebuf_size: static unsigned long = 64;

    module_param(total_size, ulong, 0);
    MODULE_PARM_DESC(total_size, "Total device size in KiB");
    module_param(erase_size, ulong, 0);
    MODULE_PARM_DESC(erase_size, "Device erase block size in KiB");
    module_param(writebuf_size, ulong, 0);
    MODULE_PARM_DESC(writebuf_size, "Device write buf size in Bytes (Default: 64)");
// We could store these in the mtd structure, but we only support 1 device..
    static struct mtd_info *mtd_info;
#[no_mangle]
unsafe extern "C" fn check_offs_len(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int check_offs_len(struct mtd_info *mtd, loff_t ofs, uint64_t len)
    {
    let mut ret: c_int = 0;
// Start address must align on block boundary
    if (mtd_mod_by_eb(ofs, mtd)) {
    pr_debug("%s: unaligned address\n", __func__);
    ret = -EINVAL;
    }
// Length must align on block boundary
    if (mtd_mod_by_eb(len, mtd)) {
    pr_debug("%s: length not block aligned\n", __func__);
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ram_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int ram_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    if (check_offs_len(mtd, instr.addr, instr.len))
    return -EINVAL;
    memset((char *)mtd.priv + instr.addr, 0xff, instr.len);
    return 0;
    }
    static int ram_point(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, void **virt, resource_size_t *phys)
    {
// virt = mtd->priv + from;
// retlen = len;
    if (phys) {
// limit retlen to the number of contiguous physical pages
    let mut page_ofs: c_ulong = offset_in_page(*virt);
    void *addr = *virt - page_ofs;
    unsigned long pfn1, pfn0 = vmalloc_to_pfn(addr);
// phys = __pfn_to_phys(pfn0) + page_ofs;
    len += page_ofs;
    while (len > PAGE_SIZE) {
    len -= PAGE_SIZE;
    addr += PAGE_SIZE;
    pfn0++;
    pfn1 = vmalloc_to_pfn(addr);
    if (pfn1 != pfn0) {
// retlen = addr - *virt;
    break;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ram_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int {
    static int ram_unpoint(struct mtd_info *mtd, loff_t from, size_t len)
    {
    return 0;
    }
    static int ram_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, u_char *buf)
    {
    memcpy(buf, mtd.priv + from, len);
// retlen = len;
    return 0;
    }
    static int ram_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    memcpy((char *)mtd.priv + to, buf, len);
// retlen = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_mtdram() -> void __exit {
    static void __exit cleanup_mtdram(void)
    {
    if (mtd_info) {
    mtd_device_unregister(mtd_info);
    vfree(mtd_info.priv);
    kfree(mtd_info);
    }
    }
    int mtdram_init_device(struct mtd_info *mtd, void *mapped_address,
    unsigned long size, const char *name)
    {
    memset(mtd, 0, sizeof(*mtd));
// Setup the MTD structure
    mtd.name = name;
    mtd.type = MTD_RAM;
    mtd.flags = MTD_CAP_RAM;
    mtd.size = size;
    mtd.writesize = 1;
    mtd.writebufsize = writebuf_size;
    mtd.erasesize = MTDRAM_ERASE_SIZE;
    mtd.priv = mapped_address;
    mtd.owner = THIS_MODULE;
    mtd._erase = ram_erase;
    mtd._point = ram_point;
    mtd._unpoint = ram_unpoint;
    mtd._read = ram_read;
    mtd._write = ram_write;
    if (mtd_device_register(mtd, core::ptr::null_mut(), 0))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_mtdram() -> int __init {
    static int __init init_mtdram(void)
    {
    void *addr;
    int err;
    if (!total_size)
    return -EINVAL;
// Allocate some memory
    mtd_info = kmalloc_obj(struct mtd_info);
    if (!mtd_info)
    return -ENOMEM;
    addr = vmalloc(MTDRAM_TOTAL_SIZE);
    if (!addr) {
    kfree(mtd_info);
    mtd_info = core::ptr::null_mut();
    return -ENOMEM;
    }
    err = mtdram_init_device(mtd_info, addr, MTDRAM_TOTAL_SIZE, "mtdram test device");
    if (err) {
    vfree(addr);
    kfree(mtd_info);
    mtd_info = core::ptr::null_mut();
    return err;
    }
    memset(mtd_info.priv, 0xff, MTDRAM_TOTAL_SIZE);
    return err;
    }
    module_init(init_mtdram);
    module_exit(cleanup_mtdram);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Larsson <alexl@redhat.com>");
    MODULE_DESCRIPTION("Simulated MTD driver for testing");
