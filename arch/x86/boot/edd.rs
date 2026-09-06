//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/edd.c
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
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Get EDD BIOS disk information
//

//
// Read the MBR (first sector) from a specific device.
//
#[no_mangle]
unsafe extern "C" fn read_mbr(devno: u8, buf: *mut c_void) -> c_int {
    static int read_mbr(u8 devno, void *buf)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ax = 0x0201;		/* Legacy Read, one sector */
    ireg.cx = 0x0001;		/* Sector 0-0-1 */
    ireg.dl = devno;
    ireg.bx = (size_t)buf;
    intcall(0x13, &ireg, &oreg);
    return -(oreg.eflags & X86_EFLAGS_CF); /* 0 or -1 */
    }
#[no_mangle]
unsafe extern "C" fn read_mbr_sig(devno: u8, ei: *mut edd_info, mbrsig: *mut u32) -> u32 {
    static u32 read_mbr_sig(u8 devno, struct edd_info *ei, u32 *mbrsig)
    {
    int sector_size;
    char *mbrbuf_ptr, *mbrbuf_end;
    u32 buf_base, mbr_base;
    extern char _end[];
    u16 mbr_magic;
    sector_size = ei.params.bytes_per_sector;
    if (!sector_size)
    sector_size = 512; /* Best available guess */
// Produce a naturally aligned buffer on the heap
    buf_base = (ds() << 4) + (u32)&_end;
    mbr_base = (buf_base+sector_size-1) & ~(sector_size-1);
    mbrbuf_ptr = _end + (mbr_base-buf_base);
    mbrbuf_end = mbrbuf_ptr + sector_size;
// Make sure we actually have space on the heap...
    if (!(boot_params.hdr.loadflags & CAN_USE_HEAP))
    return -1;
    if (mbrbuf_end > (char *)(size_t)boot_params.hdr.heap_end_ptr)
    return -1;
    memset(mbrbuf_ptr, 0, sector_size);
    if (read_mbr(devno, mbrbuf_ptr))
    return -1;
// mbrsig = *(u32 *)&mbrbuf_ptr[EDD_MBR_SIG_OFFSET];
    mbr_magic = *(u16 *)&mbrbuf_ptr[510];
// check for valid MBR magic
    let mut mbr_magic: return = = 0xAA55 ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn get_edd_info(devno: u8, ei: *mut edd_info) -> c_int {
    static int get_edd_info(u8 devno, struct edd_info *ei)
    {
    struct biosregs ireg, oreg;
    memset(ei, 0, sizeof(*ei));
// Check Extensions Present
    initregs(&ireg);
    ireg.ah = 0x41;
    ireg.bx = EDDMAGIC1;
    ireg.dl = devno;
    intcall(0x13, &ireg, &oreg);
    if (oreg.eflags & X86_EFLAGS_CF)
    return -1;	/* No extended information */
    if (oreg.bx != EDDMAGIC2)
    return -1;
    ei.device  = devno;
    ei.version = oreg.ah;		 /* EDD version number */
    ei.interface_support = oreg.cx; /* EDD functionality subsets */
// Extended Get Device Parameters
    ei.params.length = sizeof(ei.params);
    ireg.ah = 0x48;
    ireg.si = (size_t)&ei.params;
    intcall(0x13, &ireg, &oreg);
// Get legacy CHS parameters
// Ralf Brown recommends setting ES:DI to 0:0
    ireg.ah = 0x08;
    ireg.es = 0;
    intcall(0x13, &ireg, &oreg);
    if (!(oreg.eflags & X86_EFLAGS_CF)) {
    ei.legacy_max_cylinder = oreg.ch + ((oreg.cl & 0xc0) << 2);
    ei.legacy_max_head = oreg.dh;
    ei.legacy_sectors_per_track = oreg.cl & 0x3f;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn query_edd() {
    void query_edd(void)
    {
    char eddarg[8];
    let mut do_mbr: bool = true;
    let mut do_edd: bool = !IS_ENABLED(CONFIG_EDD_OFF);
    bool be_quiet;
    int devno;
    struct edd_info ei, *edp;
    u32 *mbrptr;
    if (cmdline_find_option("edd", eddarg, sizeof(eddarg)) > 0) {
    if (!strcmp(eddarg, "skipmbr") || !strcmp(eddarg, "skip")) {
    do_edd = true;
    do_mbr = false;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(eddarg, _arg: "off")) -> else {
    else if (!strcmp(eddarg, "off"))
    do_edd = false;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(eddarg, _arg: "on")) -> else {
    else if (!strcmp(eddarg, "on"))
    do_edd = true;
    }
    be_quiet = cmdline_find_option_bool("quiet");
    edp    = boot_params.eddbuf;
    mbrptr = boot_params.edd_mbr_sig_buffer;
    if (!do_edd)
    return;
// Bugs in OnBoard or AddOnCards Bios may hang the EDD probe,
// so give a hint if this happens.
//
    if (!be_quiet)
    printf("Probing EDD (edd=off to disable)... ");
    for (devno = 0x80; devno < 0x80+EDD_MBR_SIG_MAX; devno++) {
//
// Scan the BIOS-supported hard disks and query EDD
// information...
//
    if (!get_edd_info(devno, &ei)
    && boot_params.eddbuf_entries < EDDMAXNR) {
    memcpy(edp, &ei, sizeof(ei));
    edp++;
    boot_params.eddbuf_entries++;
    }
    if (do_mbr && !read_mbr_sig(devno, &ei, mbrptr++))
    boot_params.edd_mbr_sig_buf_entries = devno-0x80+1;
    }
    if (!be_quiet)
    printf("ok\n");
    }
