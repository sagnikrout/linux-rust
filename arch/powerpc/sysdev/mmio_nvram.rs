//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/mmio_nvram.c
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
//
// memory mapped NVRAM
//
// (C) Copyright IBM Corp. 2005
//
// Authors : Utz Bacher <utz.bacher@de.ibm.com>
//

    static void __iomem *mmio_nvram_start;
    static long mmio_nvram_len;
    static DEFINE_SPINLOCK(mmio_nvram_lock);
#[no_mangle]
unsafe extern "C" fn mmio_nvram_read(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t mmio_nvram_read(char *buf, size_t count, loff_t *index)
    {
    unsigned long flags;
    if (*index >= mmio_nvram_len)
    return 0;
    if (*index + count > mmio_nvram_len)
    count = mmio_nvram_len - *index;
    spin_lock_irqsave(&mmio_nvram_lock, flags);
    memcpy_fromio(buf, mmio_nvram_start + *index, count);
    spin_unlock_irqrestore(&mmio_nvram_lock, flags);
// index += count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mmio_nvram_read_val(addr: c_int) -> c_uchar {
    static unsigned char mmio_nvram_read_val(int addr)
    {
    unsigned long flags;
    unsigned char val;
    if (addr >= mmio_nvram_len)
    return 0xff;
    spin_lock_irqsave(&mmio_nvram_lock, flags);
    val = ioread8(mmio_nvram_start + addr);
    spin_unlock_irqrestore(&mmio_nvram_lock, flags);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn mmio_nvram_write(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t mmio_nvram_write(char *buf, size_t count, loff_t *index)
    {
    unsigned long flags;
    if (*index >= mmio_nvram_len)
    return 0;
    if (*index + count > mmio_nvram_len)
    count = mmio_nvram_len - *index;
    spin_lock_irqsave(&mmio_nvram_lock, flags);
    memcpy_toio(mmio_nvram_start + *index, buf, count);
    spin_unlock_irqrestore(&mmio_nvram_lock, flags);
// index += count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mmio_nvram_write_val(addr: c_int, val: c_uchar) {
    static void mmio_nvram_write_val(int addr, unsigned char val)
    {
    unsigned long flags;
    if (addr < mmio_nvram_len) {
    spin_lock_irqsave(&mmio_nvram_lock, flags);
    iowrite8(val, mmio_nvram_start + addr);
    spin_unlock_irqrestore(&mmio_nvram_lock, flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmio_nvram_get_size() -> isize {
    static ssize_t mmio_nvram_get_size(void)
    {
    return mmio_nvram_len;
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_nvram_init() -> int __init {
    int __init mmio_nvram_init(void)
    {
    struct device_node *nvram_node;
    unsigned long nvram_addr;
    struct resource r;
    int ret;
    nvram_node = of_find_node_by_type(core::ptr::null_mut(), "nvram");
    if (!nvram_node)
    nvram_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "nvram");
    if (!nvram_node) {
    printk(KERN_WARNING "nvram: no node found in device-tree\n");
    return -ENODEV;
    }
    ret = of_address_to_resource(nvram_node, 0, &r);
    if (ret) {
    printk(KERN_WARNING "nvram: failed to get address (err %d)\n",
    ret);
    goto out;
    }
    nvram_addr = r.start;
    mmio_nvram_len = resource_size(&r);
    if ( (!mmio_nvram_len) || (!nvram_addr) ) {
    printk(KERN_WARNING "nvram: address or length is 0\n");
    ret = -EIO;
    goto out;
    }
    mmio_nvram_start = ioremap(nvram_addr, mmio_nvram_len);
    if (!mmio_nvram_start) {
    printk(KERN_WARNING "nvram: failed to ioremap\n");
    ret = -ENOMEM;
    goto out;
    }
    printk(KERN_INFO "mmio NVRAM, %luk at 0x%lx mapped to %p\n",
    mmio_nvram_len >> 10, nvram_addr, mmio_nvram_start);
    ppc_md.nvram_read_val	= mmio_nvram_read_val;
    ppc_md.nvram_write_val	= mmio_nvram_write_val;
    ppc_md.nvram_read	= mmio_nvram_read;
    ppc_md.nvram_write	= mmio_nvram_write;
    ppc_md.nvram_size	= mmio_nvram_get_size;
    out:
    of_node_put(nvram_node);
    return ret;
    }
