//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/testmmiotrace.c
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
// Written by Pekka Paalanen, 2008-2009 <pq@iki.fi>
//

    static unsigned long mmio_address;
    module_param_hw(mmio_address, ulong, iomem, 0);
    MODULE_PARM_DESC(mmio_address, " Start address of the mapping of 16 kB "
    "(or 8 MB if read_far is non-zero).");
    let mut read_far: static unsigned long = 0x400100;
    module_param(read_far, ulong, 0);
    MODULE_PARM_DESC(read_far, " Offset of a 32-bit read within 8 MB "
    "(default: 0x400100).");
#[no_mangle]
unsafe extern "C" fn v16(i: unsigned) -> unsigned {
    static unsigned v16(unsigned i)
    {
    return i * 12 + 7;
    }
#[no_mangle]
unsafe extern "C" fn v32(i: unsigned) -> unsigned {
    static unsigned v32(unsigned i)
    {
    return i * 212371 + 13;
    }
#[no_mangle]
unsafe extern "C" fn do_write_test(p: *mut void __iomem) {
    static void do_write_test(void __iomem *p)
    {
    unsigned int i;
    pr_info("write test.\n");
    mmiotrace_printk("Write test.\n");
    for (i = 0; i < 256; i++)
    iowrite8(i, p + i);
    for (i = 1024; i < (5 * 1024); i += 2)
    iowrite16(v16(i), p + i);
    for (i = (5 * 1024); i < (16 * 1024); i += 4)
    iowrite32(v32(i), p + i);
    }
#[no_mangle]
unsafe extern "C" fn do_read_test(p: *mut void __iomem) {
    static void do_read_test(void __iomem *p)
    {
    unsigned int i;
    unsigned errs[3] = { 0 };
    pr_info("read test.\n");
    mmiotrace_printk("Read test.\n");
    for (i = 0; i < 256; i++)
    if (ioread8(p + i) != i)
    ++errs[0];
    for (i = 1024; i < (5 * 1024); i += 2)
    if (ioread16(p + i) != v16(i))
    ++errs[1];
    for (i = (5 * 1024); i < (16 * 1024); i += 4)
    if (ioread32(p + i) != v32(i))
    ++errs[2];
    mmiotrace_printk("Read errors: 8-bit %d, 16-bit %d, 32-bit %d.\n",
    errs[0], errs[1], errs[2]);
    }
#[no_mangle]
unsafe extern "C" fn do_read_far_test(p: *mut void __iomem) {
    static void do_read_far_test(void __iomem *p)
    {
    pr_info("read far test.\n");
    mmiotrace_printk("Read far test.\n");
    ioread32(p + read_far);
    }
#[no_mangle]
unsafe extern "C" fn do_test(size: c_ulong) {
    static void do_test(unsigned long size)
    {
    void __iomem *p = ioremap(mmio_address, size);
    if (!p) {
    pr_err("could not ioremap, aborting.\n");
    return;
    }
    mmiotrace_printk("ioremap returned %p.\n", p);
    do_write_test(p);
    do_read_test(p);
    if (read_far && read_far < size - 4)
    do_read_far_test(p);
    iounmap(p);
    }
//
// Tests how mmiotrace behaves in face of multiple ioremap / iounmaps in
// a short time. We had a bug in deferred freeing procedure which tried
// to free this region multiple times (ioremap can reuse the same address
// for many mappings).
//
#[no_mangle]
unsafe extern "C" fn do_test_bulk_ioremapping() {
    static void do_test_bulk_ioremapping(void)
    {
    void __iomem *p;
    int i;
    for (i = 0; i < 10; ++i) {
    p = ioremap(mmio_address, PAGE_SIZE);
    if (p)
    iounmap(p);
    }
// Force freeing. If it will crash we will know why.
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn init() -> int __init {
    static int __init init(void)
    {
    let mut size: c_ulong = (read_far) ? (8 << 20) : (16 << 10);
    let mut ret: c_int = security_locked_down(LOCKDOWN_MMIOTRACE);
    if (ret)
    return ret;
    if (mmio_address == 0) {
    pr_err("you have to use the module argument mmio_address.\n");
    pr_err("DO NOT LOAD THIS MODULE UNLESS YOU REALLY KNOW WHAT YOU ARE DOING!\n");
    return -ENXIO;
    }
    pr_warn("WARNING: mapping %lu kB @ 0x%08lx in PCI address space, "
    "and writing 16 kB of rubbish in there.\n",
    size >> 10, mmio_address);
    do_test(size);
    do_test_bulk_ioremapping();
    pr_info("All done.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup() -> void __exit {
    static void __exit cleanup(void)
    {
    pr_debug("unloaded.\n");
    }
    module_init(init);
    module_exit(cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Test module for mmiotrace");
