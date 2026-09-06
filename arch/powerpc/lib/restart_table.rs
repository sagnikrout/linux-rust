//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/lib/restart_table.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct soft_mask_table_entry {
    pub start: c_ulong,
    pub end: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct restart_table_entry {
    pub start: c_ulong,
    pub end: c_ulong,
    pub fixup: c_ulong,
}

    extern struct soft_mask_table_entry __start___soft_mask_table[];
    extern struct soft_mask_table_entry __stop___soft_mask_table[];
    extern struct restart_table_entry __start___restart_table[];
    extern struct restart_table_entry __stop___restart_table[];
// Given an address, look for it in the soft mask table
#[no_mangle]
pub unsafe extern "C" fn search_kernel_soft_mask_table(addr: c_ulong) -> bool {
    bool search_kernel_soft_mask_table(unsigned long addr)
    {
    struct soft_mask_table_entry *smte = __start___soft_mask_table;
    while (smte < __stop___soft_mask_table) {
    let mut start: c_ulong = smte.start;
    let mut end: c_ulong = smte.end;
    if (addr >= start && addr < end)
    return true;
    smte++;
    }
    return false;
    }
    NOKPROBE_SYMBOL(search_kernel_soft_mask_table);
// Given an address, look for it in the kernel exception table
#[no_mangle]
pub unsafe extern "C" fn search_kernel_restart_table(addr: c_ulong) -> c_ulong {
    unsigned long search_kernel_restart_table(unsigned long addr)
    {
    struct restart_table_entry *rte = __start___restart_table;
    while (rte < __stop___restart_table) {
    let mut start: c_ulong = rte.start;
    let mut end: c_ulong = rte.end;
    let mut fixup: c_ulong = rte.fixup;
    if (addr >= start && addr < end)
    return fixup;
    rte++;
    }
    return 0;
    }
    NOKPROBE_SYMBOL(search_kernel_restart_table);
