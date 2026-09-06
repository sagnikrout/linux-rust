//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/main.c
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
// Main module for the real-mode kernel code
//

    struct boot_params boot_params __attribute__((aligned(16)));
    struct port_io_ops pio_ops;
    char *HEAP = _end;
    char *heap_end = _end;		/* Default end of heap = no heap */
//
// Copy the header into the boot parameter block.  Since this
// screws up the old-style command line protocol, adjust by
// filling in the new-style command line pointer instead.
//
#[no_mangle]
unsafe extern "C" fn copy_boot_params() {
    static void copy_boot_params(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_cmdline {
    pub cl_magic: u16,
    pub cl_offset: u16,
}

    let mut oldcmd: *const old_cmdline  const = absolute_pointer(OLD_CL_ADDRESS);
    BUILD_BUG_ON(sizeof(boot_params) != 4096);
    memcpy(&boot_params.hdr, &hdr, sizeof(hdr));
    if (!boot_params.hdr.cmd_line_ptr && oldcmd.cl_magic == OLD_CL_MAGIC) {
// Old-style command line protocol
    u16 cmdline_seg;
//
// Figure out if the command line falls in the region
// of memory that an old kernel would have copied up
// to 0x90000...
//
    if (oldcmd.cl_offset < boot_params.hdr.setup_move_size)
    cmdline_seg = ds();
    else
    cmdline_seg = 0x9000;
    boot_params.hdr.cmd_line_ptr = (cmdline_seg << 4) + oldcmd.cl_offset;
    }
    }
//
// Query the keyboard lock status as given by the BIOS, and
// set the keyboard repeat rate to maximum.  Unclear why the latter
// is done here; this might be possible to kill off as stale code.
//
#[no_mangle]
unsafe extern "C" fn keyboard_init() {
    static void keyboard_init(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ah = 0x02;		/* Get keyboard status */
    intcall(0x16, &ireg, &oreg);
    boot_params.kbd_status = oreg.al;
    ireg.ax = 0x0305;	/* Set keyboard repeat rate */
    intcall(0x16, &ireg, core::ptr::null_mut());
    }
//
// Get Intel SpeedStep (IST) information.
//
#[no_mangle]
unsafe extern "C" fn query_ist() {
    static void query_ist(void)
    {
    struct biosregs ireg, oreg;
//
// Some older BIOSes apparently crash on this call, so filter
// it from machines too old to have SpeedStep at all.
//
    if (cpu.level < 6)
    return;
    initregs(&ireg);
    ireg.ax  = 0xe980;	 /* IST Support */
    ireg.edx = 0x47534943;	 /* Request value */
    intcall(0x15, &ireg, &oreg);
    boot_params.ist_info.signature  = oreg.eax;
    boot_params.ist_info.command    = oreg.ebx;
    boot_params.ist_info.event      = oreg.ecx;
    boot_params.ist_info.perf_level = oreg.edx;
    }
//
// Tell the BIOS what CPU mode we intend to run in.
//
#[no_mangle]
unsafe extern "C" fn set_bios_mode() {
    static void set_bios_mode(void)
    {

    struct biosregs ireg;
    initregs(&ireg);
    ireg.ax = 0xec00;
    ireg.bx = 2;
    intcall(0x15, &ireg, core::ptr::null_mut());

    }
#[no_mangle]
unsafe extern "C" fn init_heap() {
    static void init_heap(void)
    {
    char *stack_end;
    if (boot_params.hdr.loadflags & CAN_USE_HEAP) {
    stack_end = (char *) (current_stack_pointer - STACK_SIZE);
    heap_end = (char *) ((size_t)boot_params.hdr.heap_end_ptr + 0x200);
    if (heap_end > stack_end)
    heap_end = stack_end;
    } else {
// Boot protocol 2.00 only, no heap available
    puts("WARNING: Ancient bootloader, some functionality may be limited!\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() {
    void main(void)
    {
    init_default_io_ops();
// First, copy the boot header into the "zeropage"
    copy_boot_params();
// Initialize the early-boot console
    console_init();
    if (cmdline_find_option_bool("debug"))
    puts("early console in setup code\n");
// End of heap check
    init_heap();
// Make sure we have all the proper CPU support
    if (validate_cpu()) {
    puts("Unable to boot - please use a kernel appropriate for your CPU.\n");
    die();
    }
// Tell the BIOS what CPU mode we intend to run in
    set_bios_mode();
// Detect memory layout
    detect_memory();
// Set keyboard repeat rate (why?) and query the lock flags
    keyboard_init();
// Query Intel SpeedStep (IST) information
    query_ist();
// Query APM information
    if (IS_ENABLED(CONFIG_APM))
    query_apm_bios();
// Query EDD information
    if (IS_ENABLED(CONFIG_EDD))
    query_edd();
// Set the video mode
    set_video();
// Do the last things and invoke protected mode
    go_to_protected_mode();
    }
