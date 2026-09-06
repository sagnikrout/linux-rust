//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/os_info.c
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
// OS info memory interface
//
// Copyright IBM Corp. 2012
// Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

//
// OS info structure has to be page aligned
//
    static struct os_info os_info __page_aligned_data;
//
// Compute checksum over OS info structure
//
#[no_mangle]
pub unsafe extern "C" fn os_info_csum(os_info: *mut os_info) -> u32 {
    u32 os_info_csum(struct os_info *os_info)
    {
    let mut size: c_int = sizeof(*os_info) - offsetof(struct os_info, version_major);
    return ( u32)cksm(&os_info.version_major, size, 0);
    }
//
// Add crashkernel info to OS info and update checksum
//
#[no_mangle]
pub unsafe extern "C" fn os_info_crashkernel_add(base: c_ulong, size: c_ulong) {
    void os_info_crashkernel_add(unsigned long base, unsigned long size)
    {
    os_info.crashkernel_addr = (u64)(unsigned long)base;
    os_info.crashkernel_size = (u64)(unsigned long)size;
    os_info.csum = os_info_csum(&os_info);
    }
//
// Add OS info data entry and update checksum
//
#[no_mangle]
pub unsafe extern "C" fn os_info_entry_add_data(nr: c_int, ptr: *mut c_void, size: u64) {
    void os_info_entry_add_data(int nr, void *ptr, u64 size)
    {
    os_info.entry[nr].addr = __pa(ptr);
    os_info.entry[nr].size = size;
    os_info.entry[nr].csum = ( u32)cksm(ptr, size, 0);
    os_info.csum = os_info_csum(&os_info);
    }
//
// Add OS info value entry and update checksum
//
#[no_mangle]
pub unsafe extern "C" fn os_info_entry_add_val(nr: c_int, value: u64) {
    void os_info_entry_add_val(int nr, u64 value)
    {
    os_info.entry[nr].val = value;
    os_info.entry[nr].size = 0;
    os_info.entry[nr].csum = 0;
    os_info.csum = os_info_csum(&os_info);
    }
//
// Initialize OS info structure and set lowcore pointer
//
#[no_mangle]
pub unsafe extern "C" fn os_info_init() -> void __init {
    void __init os_info_init(void)
    {
    struct lowcore *abs_lc;
    BUILD_BUG_ON(sizeof(struct os_info) != PAGE_SIZE);
    os_info.version_major = OS_INFO_VERSION_MAJOR;
    os_info.version_minor = OS_INFO_VERSION_MINOR;
    os_info.magic = OS_INFO_MAGIC;
    os_info_entry_add_val(OS_INFO_IDENTITY_BASE, __identity_base);
    os_info_entry_add_val(OS_INFO_KASLR_OFFSET, kaslr_offset());
    os_info_entry_add_val(OS_INFO_KASLR_OFF_PHYS, __kaslr_offset_phys);
    os_info_entry_add_val(OS_INFO_VMEMMAP, (unsigned long)vmemmap);
    os_info_entry_add_val(OS_INFO_AMODE31_START, AMODE31_START);
    os_info_entry_add_val(OS_INFO_AMODE31_END, AMODE31_END);
    os_info_entry_add_val(OS_INFO_IMAGE_START, (unsigned long)_stext);
    os_info_entry_add_val(OS_INFO_IMAGE_END, (unsigned long)_end);
    os_info_entry_add_val(OS_INFO_IMAGE_PHYS, __pa_symbol(_stext));
    os_info.csum = os_info_csum(&os_info);
    abs_lc = get_abs_lowcore();
    abs_lc.os_info = __pa(&os_info);
    put_abs_lowcore(abs_lc);
    }

    static struct os_info *os_info_old;
//
// Allocate and copy OS info entry from oldmem
//
#[no_mangle]
unsafe extern "C" fn os_info_old_alloc(nr: c_int, align: c_int) {
    static void os_info_old_alloc(int nr, int align)
    {
    unsigned long addr, size = 0;
    char *buf, *buf_align, *msg;
    u32 csum;
    addr = os_info_old.entry[nr].addr;
    if (!addr) {
    msg = "not available";
    goto fail;
    }
    size = os_info_old.entry[nr].size;
    buf = kmalloc(size + align - 1, GFP_KERNEL);
    if (!buf) {
    msg = "alloc failed";
    goto fail;
    }
    buf_align = PTR_ALIGN(buf, align);
    if (copy_oldmem_kernel(buf_align, addr, size)) {
    msg = "copy failed";
    goto fail_free;
    }
    csum = ( u32)cksm(buf_align, size, 0);
    if (csum != os_info_old.entry[nr].csum) {
    msg = "checksum failed";
    goto fail_free;
    }
    os_info_old.entry[nr].addr = (u64)(unsigned long)buf_align;
    msg = "copied";
    goto out;
    fail_free:
    kfree(buf);
    fail:
    os_info_old.entry[nr].addr = 0;
    out:
    pr_info("entry %i: %s (addr=0x%lx size=%lu)\n",
    nr, msg, addr, size);
    }
//
// Initialize os info and os info entries from oldmem
//
#[no_mangle]
unsafe extern "C" fn os_info_old_init() {
    static void os_info_old_init(void)
    {
    static int os_info_init;
    unsigned long addr;
    if (os_info_init)
    return;
    if (!oldmem_data.start && !is_ipl_type_dump())
    goto fail;
    if (copy_oldmem_kernel(&addr, __LC_OS_INFO, sizeof(addr)))
    goto fail;
    if (addr == 0 || addr % PAGE_SIZE)
    goto fail;
    os_info_old = kzalloc_obj(*os_info_old);
    if (!os_info_old)
    goto fail;
    if (copy_oldmem_kernel(os_info_old, addr, sizeof(*os_info_old)))
    goto fail_free;
    if (os_info_old.magic != OS_INFO_MAGIC)
    goto fail_free;
    if (os_info_old.csum != os_info_csum(os_info_old))
    goto fail_free;
    if (os_info_old.version_major > OS_INFO_VERSION_MAJOR)
    goto fail_free;
    os_info_old_alloc(OS_INFO_VMCOREINFO, 1);
    os_info_old_alloc(OS_INFO_REIPL_BLOCK, 1);
    pr_info("crashkernel: addr=0x%lx size=%lu\n",
    (unsigned long) os_info_old.crashkernel_addr,
    (unsigned long) os_info_old.crashkernel_size);
    os_info_init = 1;
    return;
    fail_free:
    kfree(os_info_old);
    fail:
    os_info_init = 1;
    os_info_old = core::ptr::null_mut();
    }
//
// Return pointer to os info entry and its size
//
    void *os_info_old_entry(int nr, unsigned long *size)
    {
    os_info_old_init();
    if (!os_info_old)
    return core::ptr::null_mut();
    if (!os_info_old.entry[nr].addr)
    return core::ptr::null_mut();
// size = (unsigned long) os_info_old->entry[nr].size;
    return (void *)(unsigned long)os_info_old.entry[nr].addr;
    }
