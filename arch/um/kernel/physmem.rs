//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/physmem.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    let mut physmem_fd: static int = -1;
// Changed during early boot
    unsigned long high_physmem;
    EXPORT_SYMBOL(high_physmem);
    void map_memory(unsigned long virt, unsigned long phys, unsigned long len,
    int r, int w, int x)
    {
    __u64 offset;
    int fd, err;
    fd = phys_mapping(phys, &offset);
    err = os_map_memory((void *) virt, fd, offset, len, r, w, x);
    if (err) {
    if (err == -ENOMEM)
    printk(KERN_ERR "try increasing the host's "
    "/proc/sys/vm/max_map_count to <physical "
    "memory size>/4096\n");
    panic("map_memory(0x%lx, %d, 0x%llx, %ld, %d, %d, %d) failed, "
    "err = %d\n", virt, fd, offset, len, r, w, x, err);
    }
    }
//
// setup_physmem() - Setup physical memory for UML
// @start:	Start address of the physical kernel memory,
// i.e start address of the executable image.
// @reserve_end:	end address of the physical kernel memory.
// @len:	Length of total physical memory that should be mapped/made
// available, in bytes.
//
// Creates an unlinked temporary file of size (len) and memory maps
// it on the last executable image address (uml_reserved).
//
// The offset is needed as the length of the total physical memory
// (len) includes the size of the memory used be the executable image,
// but the mapped-to address is the last address of the executable image
// (uml_reserved == end address of executable image).
//
// The memory mapped memory of the temporary file is used as backing memory
// of all user space processes/kernel tasks.
//
    void __init setup_physmem(unsigned long start, unsigned long reserve_end,
    unsigned long len)
    {
    let mut reserve: c_ulong = reserve_end - start;
    let mut map_size: c_ulong = len - reserve;
    int err;
    if (len <= reserve) {
    os_warn("Too few physical memory! Needed=%lu, given=%lu\n",
    reserve, len);
    exit(1);
    }
    physmem_fd = create_mem_file(len);
    err = os_map_memory((void *) reserve_end, physmem_fd, reserve,
    map_size, 1, 1, 1);
    if (err < 0) {
    os_warn("setup_physmem - mapping %lu bytes of memory at 0x%p "
    "failed - errno = %d\n", map_size,
    (void *) reserve_end, err);
    exit(1);
    }
//
// Special kludge - This page will be mapped in to userspace processes
// from physmem_fd, so it needs to be written out there.
//
    os_seek_file(physmem_fd, __pa(__syscall_stub_start));
    os_write_file(physmem_fd, __syscall_stub_start, PAGE_SIZE);
    memblock_add(__pa(start), len);
    memblock_reserve(__pa(start), reserve);
    min_low_pfn = PFN_UP(__pa(reserve_end));
    max_low_pfn = min_low_pfn + (map_size >> PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn phys_mapping(phys: c_ulong, offset_out: *mut c_ulonglong) -> c_int {
    int phys_mapping(unsigned long phys, unsigned long long *offset_out)
    {
    let mut fd: c_int = -1;
    if (phys < physmem_size) {
    fd = physmem_fd;
// offset_out = phys;
    }
    return fd;
    }
    EXPORT_SYMBOL(phys_mapping);
#[no_mangle]
unsafe extern "C" fn uml_mem_setup(line: *mut c_char, add: *mut c_int) -> int __init {
    static int __init uml_mem_setup(char *line, int *add)
    {
    char *retptr;
// add = 0;
    physmem_size = memparse(line,&retptr);
    return 0;
    }
    __uml_setup("mem=", uml_mem_setup,
    "mem=<Amount of desired ram>\n"
    "    This controls how much \"physical\" memory the kernel allocates\n"
    "    for the system. The size is specified as a number followed by\n"
    "    one of 'k', 'K', 'm', 'M', which have the obvious meanings.\n"
    "    This is not related to the amount of memory in the host.  It can\n"
    "    be more, and the excess, if it's ever used, will just be swapped out.\n"
    "	Example: mem=64M\n\n"
    );
