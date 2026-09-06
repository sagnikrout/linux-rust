//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/pm.c
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
//
// -----------------------------------------------------------------------
//
// Prepare the machine for transition to protected mode.
//

//
// Invoke the realmode switch hook if present; otherwise
// disable all interrupts.
//
#[no_mangle]
unsafe extern "C" fn realmode_switch_hook() {
    static void realmode_switch_hook(void)
    {
    if (boot_params.hdr.realmode_swtch) {
    asm volatile("lcallw *%0"
    : : "m" (boot_params.hdr.realmode_swtch)
    : "eax", "ebx", "ecx", "edx");
    } else {
    asm volatile("cli");
    outb(0x80, 0x70); /* Disable NMI */
    io_delay();
    }
    }
//
// Disable all interrupts at the legacy PIC.
//
#[no_mangle]
unsafe extern "C" fn mask_all_interrupts() {
    static void mask_all_interrupts(void)
    {
    outb(0xff, 0xa1);	/* Mask all interrupts on the secondary PIC */
    io_delay();
    outb(0xfb, 0x21);	/* Mask all but cascade on the primary PIC */
    io_delay();
    }
//
// Reset IGNNE# if asserted in the FPU.
//
#[no_mangle]
unsafe extern "C" fn reset_coprocessor() {
    static void reset_coprocessor(void)
    {
    outb(0, 0xf0);
    io_delay();
    outb(0, 0xf1);
    io_delay();
    }
//
// Set up the GDT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdt_ptr {
    pub len: u16,
    pub ptr: u32,
    pub __attribute__((packed)): },
#[no_mangle]
unsafe extern "C" fn setup_gdt() {
    static void setup_gdt(void)
    {
// There are machines which are known to not boot with the GDT
    being 8-byte unaligned.  Intel recommends 16 byte alignment. */
    static const u64 boot_gdt[] __attribute__((aligned(16))) = {
// CS: code, read/execute, 4 GB, base 0
    [GDT_ENTRY_BOOT_CS] = GDT_ENTRY(DESC_CODE32, 0, 0xfffff),
// DS: data, read/write, 4 GB, base 0
    [GDT_ENTRY_BOOT_DS] = GDT_ENTRY(DESC_DATA32, 0, 0xfffff),
// TSS: 32-bit tss, 104 bytes, base 4096
// We only have a TSS here to keep Intel VT happy;
    we don't actually use it for anything. */
    [GDT_ENTRY_BOOT_TSS] = GDT_ENTRY(DESC_TSS32, 4096, 103),
}

// Xen HVM incorrectly stores a pointer to the gdt_ptr, instead
    of the gdt_ptr contents.  Thus, make it static so it will
    stay in memory, at least long enough that we switch to the
    proper kernel GDT. */
    static struct gdt_ptr gdt;
    gdt.len = sizeof(boot_gdt)-1;
    gdt.ptr = (u32)&boot_gdt + (ds() << 4);
    asm volatile("lgdtl %0" : : "m" (gdt));
    }
//
// Set up the IDT
//
#[no_mangle]
unsafe extern "C" fn setup_idt() {
    static void setup_idt(void)
    {
    let mut null_idt: static struct gdt_ptr = {0, 0};
    asm volatile("lidtl %0" : : "m" (null_idt));
    }
//
// Actual invocation sequence
//
#[no_mangle]
pub unsafe extern "C" fn go_to_protected_mode() {
    void go_to_protected_mode(void)
    {
// Hook before leaving real mode, also disables interrupts
    realmode_switch_hook();
// Enable the A20 gate
    if (enable_a20()) {
    puts("A20 gate not responding, unable to boot...\n");
    die();
    }
// Reset coprocessor (IGNNE#)
    reset_coprocessor();
// Mask all interrupts in the PIC
    mask_all_interrupts();
// Actual transition to protected mode...
    setup_idt();
    setup_gdt();
    protected_mode_jump(boot_params.hdr.code32_start,
    (u32)&boot_params + (ds() << 4));
    }
