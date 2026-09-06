//! Automatically rewritten from C to Rust
//! Source: security/min_addr.c
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

// amount of vm to protect from userspace access by both DAC and the LSM
    unsigned long mmap_min_addr;
// amount of vm to protect from userspace using CAP_SYS_RAWIO (DAC)
    let mut dac_mmap_min_addr: c_ulong = CONFIG_DEFAULT_MMAP_MIN_ADDR;
// amount of vm to protect from userspace using the LSM = CONFIG_LSM_MMAP_MIN_ADDR
//
// Update mmap_min_addr = max(dac_mmap_min_addr, CONFIG_LSM_MMAP_MIN_ADDR)
//
#[no_mangle]
unsafe extern "C" fn update_mmap_min_addr() {
    static void update_mmap_min_addr(void)
    {

    mmap_min_addr = umax(dac_mmap_min_addr, CONFIG_LSM_MMAP_MIN_ADDR);

    mmap_min_addr = dac_mmap_min_addr;

    }
//
// sysctl handler which just sets dac_mmap_min_addr = the new value and then
// calls update_mmap_min_addr() so non MAP_FIXED hints get rounded properly
//
    int mmap_min_addr_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    int ret;
    if (write && !capable(CAP_SYS_RAWIO))
    return -EPERM;
    ret = proc_doulongvec_minmax(table, write, buffer, lenp, ppos);
    update_mmap_min_addr();
    return ret;
    }
    static const struct ctl_table min_addr_sysctl_table[] = {
    {
    .procname	= "mmap_min_addr",
    .data		= &dac_mmap_min_addr,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= mmap_min_addr_handler,
    },
    };
#[no_mangle]
unsafe extern "C" fn mmap_min_addr_init() -> int __init {
    static int __init mmap_min_addr_init(void)
    {
    register_sysctl_init("vm", min_addr_sysctl_table);
    update_mmap_min_addr();
    return 0;
    }
    pure_initcall(mmap_min_addr_init);
