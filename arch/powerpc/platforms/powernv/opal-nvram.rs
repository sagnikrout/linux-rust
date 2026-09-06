//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-nvram.c
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
// PowerNV nvram code.
//
// Copyright 2011 IBM Corp.
//
// Macro flag: #define DEBUG

    static unsigned int nvram_size;
#[no_mangle]
unsafe extern "C" fn opal_nvram_size() -> isize {
    static ssize_t opal_nvram_size(void)
    {
    return nvram_size;
    }
#[no_mangle]
unsafe extern "C" fn opal_nvram_read(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t opal_nvram_read(char *buf, size_t count, loff_t *index)
    {
    s64 rc;
    int off;
    if (*index >= nvram_size)
    return 0;
    off = *index;
    if ((off + count) > nvram_size)
    count = nvram_size - off;
    rc = opal_read_nvram(__pa(buf), count, off);
    if (rc != OPAL_SUCCESS)
    return -EIO;
// index += count;
    return count;
    }
//
// This can be called in the panic path with interrupts off, so use
// mdelay in that case.
//
#[no_mangle]
unsafe extern "C" fn opal_nvram_write(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t opal_nvram_write(char *buf, size_t count, loff_t *index)
    {
    let mut rc: i64 = OPAL_BUSY;
    int off;
    if (*index >= nvram_size)
    return 0;
    off = *index;
    if ((off + count) > nvram_size)
    count = nvram_size - off;
    while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
    rc = opal_write_nvram(__pa(buf), count, off);
    if (rc == OPAL_BUSY_EVENT) {
    if (in_interrupt() || irqs_disabled())
    mdelay(OPAL_BUSY_DELAY_MS);
    else
    msleep(OPAL_BUSY_DELAY_MS);
    opal_poll_events(core::ptr::null_mut());
    } else if (rc == OPAL_BUSY) {
    if (in_interrupt() || irqs_disabled())
    mdelay(OPAL_BUSY_DELAY_MS);
    else
    msleep(OPAL_BUSY_DELAY_MS);
    }
    }
    if (rc)
    return -EIO;
// index += count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn opal_nvram_init_log_partitions() -> int __init {
    static int __init opal_nvram_init_log_partitions(void)
    {
// Scan nvram for partitions
    nvram_scan_partitions();
    nvram_init_oops_partition(0);
    return 0;
    }
    machine_arch_initcall(powernv, opal_nvram_init_log_partitions);
#[no_mangle]
pub unsafe extern "C" fn opal_nvram_init() -> void __init {
    void __init opal_nvram_init(void)
    {
    struct device_node *np;
    const __be32 *nbytes_p;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opal-nvram");
    if (np == core::ptr::null_mut())
    return;
    nbytes_p = of_get_property(np, "#bytes", core::ptr::null_mut());
    if (!nbytes_p) {
    of_node_put(np);
    return;
    }
    nvram_size = be32_to_cpup(nbytes_p);
    pr_info("OPAL nvram setup, %u bytes\n", nvram_size);
    of_node_put(np);
    ppc_md.nvram_read = opal_nvram_read;
    ppc_md.nvram_write = opal_nvram_write;
    ppc_md.nvram_size = opal_nvram_size;
    }
