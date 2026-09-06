//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/vm86.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// I'm guessing at the VIF/VIP flag usage, but hope that this is how
// the Pentium uses them. Linux will return from vm86 mode when both
// VIF and VIP is set.
//
// On a Pentium, we could probably optimize the virtual flags directly
// in the eflags register instead of doing it "by hand" in vflags...
//
// Linus
//

pub const BIOSSEG: c_uint = 0x0f000;
pub const CPU_086: c_int = 0;
pub const CPU_186: c_int = 1;
pub const CPU_286: c_int = 2;
pub const CPU_386: c_int = 3;
pub const CPU_486: c_int = 4;
pub const CPU_586: c_int = 5;
//
// Return values for the 'vm86()' system call
//

//
// Additional return values when invoking new vm86()
//

//
// function codes when invoking new vm86()
//
pub const VM86_PLUS_INSTALL_CHECK: c_int = 0;
pub const VM86_ENTER: c_int = 1;
pub const VM86_ENTER_NO_BYPASS: c_int = 2;
pub const VM86_REQUEST_IRQ: c_int = 3;
pub const VM86_FREE_IRQ: c_int = 4;
pub const VM86_GET_IRQ_BITS: c_int = 5;
pub const VM86_GET_AND_RESET_IRQ: c_int = 6;
//
// This is the stack-layout seen by the user space program when we have
// done a translation of "SAVE_ALL" from vm86 mode. The real kernel layout
// is 'kernel_vm86_regs' (see below).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86_regs {
//
// normal regs, with special meaning for the segment descriptors..
//
    pub ebx: c_long,
    pub ecx: c_long,
    pub edx: c_long,
    pub esi: c_long,
    pub edi: c_long,
    pub ebp: c_long,
    pub eax: c_long,
    pub __null_ds: c_long,
    pub __null_es: c_long,
    pub __null_fs: c_long,
    pub __null_gs: c_long,
    pub orig_eax: c_long,
    pub eip: c_long,
    pub __csh: unsigned short cs,,
    pub eflags: c_long,
    pub esp: c_long,
    pub __ssh: unsigned short ss,,
//
// these are specific to v86 mode:
//
    pub __esh: unsigned short es,,
    pub __dsh: unsigned short ds,,
    pub __fsh: unsigned short fs,,
    pub __gsh: unsigned short gs,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct revectored_struct {
    pub /: *mut *mut unsigned long __map[8]; / 256 bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86_struct {
    pub regs: vm86_regs,
    pub flags: c_ulong,
    pub /: *mut *mut unsigned long screen_bitmap; / unused, preserved by vm86(),
    pub cpu_type: c_ulong,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
}

//
// flags masks
//
pub const VM86_SCREEN_BITMAP: c_uint = 0x0001        /* no longer supported */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86plus_info_struct {
    pub force_return_for_pic:1: c_ulong,
    pub /: *mut *mut unsigned long vm86dbg_active:1; / for debugger,
    pub /: *mut *mut unsigned long vm86dbg_TFpendig:1; / for debugger,
    pub unused:28: c_ulong,
    pub /: *mut *mut unsigned long is_vm86pus:1; / for vm86 internal use,
    pub /: *mut *mut unsigned char vm86dbg_intxxtab[32]; / for debugger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86plus_struct {
    pub regs: vm86_regs,
    pub flags: c_ulong,
    pub screen_bitmap: c_ulong,
    pub cpu_type: c_ulong,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
    pub vm86plus: vm86plus_info_struct,
}
