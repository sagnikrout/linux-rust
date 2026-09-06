//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kexec/core_32.c
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
// PPC32 code to handle Linux booting another kernel.
//
// Copyright (C) 2002-2003 Eric Biederman  <ebiederm@xmission.com>
// GameCube/ppc32 port Copyright (C) 2004 Albert Herranz
// Copyright (C) 2005 IBM Corporation.
//

    typedef void (*relocate_new_kernel_t)(
    unsigned long indirection_page,
    unsigned long reboot_code_buffer,
    unsigned long start_address) __noreturn;
//
// This is a generic machine_kexec function suitable at least for
// non-OpenFirmware embedded platforms.
// It merely copies the image relocation code to the control page and
// jumps to it.
// A platform specific function may just call this one.
//
#[no_mangle]
pub unsafe extern "C" fn default_machine_kexec(image: *mut kimage) {
    void default_machine_kexec(struct kimage *image)
    {
    extern const unsigned int relocate_new_kernel_size;
    unsigned long page_list;
    unsigned long reboot_code_buffer, reboot_code_buffer_phys;
    relocate_new_kernel_t rnk;
// Interrupts aren't acceptable while we reboot
    local_irq_disable();
// mask each interrupt so we are in a more sane state for the
// kexec kernel
    machine_kexec_mask_interrupts();
    page_list = image.head;
// we need both effective and real address here
    reboot_code_buffer =
    (unsigned long)page_address(image.control_code_page);
    reboot_code_buffer_phys = virt_to_phys((void *)reboot_code_buffer);
// copy our kernel relocation code to the control code page
    memcpy((void *)reboot_code_buffer, relocate_new_kernel,
    relocate_new_kernel_size);
    flush_icache_range(reboot_code_buffer,
    reboot_code_buffer + KEXEC_CONTROL_PAGE_SIZE);
    printk(KERN_INFO "Bye!\n");
    if (!IS_ENABLED(CONFIG_PPC_85xx) && !IS_ENABLED(CONFIG_44x))
    relocate_new_kernel(page_list, reboot_code_buffer_phys, image.start);
// now call it
    rnk = (relocate_new_kernel_t) reboot_code_buffer;
    (*rnk)(page_list, reboot_code_buffer_phys, image.start);
    }
#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> c_int {
    int machine_kexec_prepare(struct kimage *image)
    {
    return 0;
    }
