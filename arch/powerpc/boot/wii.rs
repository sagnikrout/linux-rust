//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/wii.c
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
// arch/powerpc/boot/wii.c
//
// Nintendo Wii bootwrapper support
// Copyright (C) 2008-2009 The GameCube Linux Team
// Copyright (C) 2008,2009 Albert Herranz
//

    BSS_STACK(8192);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipc_infohdr {
    pub magic: [c_char; 3],
    pub version: u8,
    pub mem2_boundary: u32,
    pub ipc_in: u32,
    pub ipc_in_size: usize,
    pub ipc_out: u32,
    pub ipc_out_size: usize,
}

#[no_mangle]
unsafe extern "C" fn mipc_check_address(pa: u32) -> c_int {
    static int mipc_check_address(u32 pa)
    {
// only MEM2 addresses
    if (pa < 0x10000000 || pa > 0x14000000)
    return -EINVAL;
    return 0;
    }
    static struct mipc_infohdr *mipc_get_infohdr(void)
    {
    struct mipc_infohdr **hdrp, *hdr;
// 'mini' header pointer is the last word of MEM2 memory
    hdrp = (struct mipc_infohdr **)0x13fffffc;
    if (mipc_check_address((u32)hdrp)) {
    printf("mini: invalid hdrp %08X\n", (u32)hdrp);
    hdr = core::ptr::null_mut();
    goto out;
    }
    hdr = *hdrp;
    if (mipc_check_address((u32)hdr)) {
    printf("mini: invalid hdr %08X\n", (u32)hdr);
    hdr = core::ptr::null_mut();
    goto out;
    }
    if (memcmp(hdr.magic, "IPC", 3)) {
    printf("mini: invalid magic\n");
    hdr = core::ptr::null_mut();
    goto out;
    }
    out:
    return hdr;
    }
#[no_mangle]
unsafe extern "C" fn mipc_get_mem2_boundary(mem2_boundary: *mut u32) -> c_int {
    static int mipc_get_mem2_boundary(u32 *mem2_boundary)
    {
    struct mipc_infohdr *hdr;
    int error;
    hdr = mipc_get_infohdr();
    if (!hdr) {
    error = -1;
    goto out;
    }
    if (mipc_check_address(hdr.mem2_boundary)) {
    printf("mini: invalid mem2_boundary %08X\n",
    hdr.mem2_boundary);
    error = -EINVAL;
    goto out;
    }
// mem2_boundary = hdr->mem2_boundary;
    error = 0;
    out:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    void *mem;
    u32 reg[4];
    u32 mem2_boundary;
    int len;
    int error;
    mem = finddevice("/memory");
    if (!mem)
    fatal("Can't find memory node\n");
// two ranges of (address, size) words
    len = getprop(mem, "reg", reg, sizeof(reg));
    if (len != sizeof(reg)) {
// nothing to do
    goto out;
    }
// retrieve MEM2 boundary from 'mini'
    error = mipc_get_mem2_boundary(&mem2_boundary);
    if (error) {
// if that fails use a sane value
    mem2_boundary = MEM2_TOP - FIRMWARE_DEFAULT_SIZE;
    }
    if (mem2_boundary > reg[2] && mem2_boundary < reg[2] + reg[3]) {
    reg[3] = mem2_boundary - reg[2];
    printf("top of MEM2 @ %08X\n", reg[2] + reg[3]);
    setprop(mem, "reg", reg, sizeof(reg));
    }
    out:
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn platform_init(r3: c_ulong, r4: c_ulong, r5: c_ulong) {
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5)
    {
    let mut heapsize: u32 = 24*1024*1024 - (u32)_end;
    simple_alloc_init(_end, heapsize, 32, 64);
    fdt_init(_dtb_start);
//
// 'mini' boots the Broadway processor with EXI disabled.
// We need it enabled before probing for the USB Gecko.
//
    out_be32(EXI_CTRL, in_be32(EXI_CTRL) | EXI_CTRL_ENABLE);
    if (ug_probe())
    console_ops.write = ug_console_write;
    platform_ops.fixups = platform_fixups;
    }
