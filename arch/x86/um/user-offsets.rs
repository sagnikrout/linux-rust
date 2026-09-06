//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/user-offsets.c
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

// Macro flag: #define __FRAME_OFFSETS

    COMMENT(#val " / sizeof(unsigned long)");	\
    DEFINE(sym, val / sizeof(unsigned long))
// workaround for a warning with -Wmissing-prototypes
    void foo(void);
#[no_mangle]
pub unsafe extern "C" fn foo() {
    void foo(void)
    {

    DEFINE(HOST_IP, EIP);
    DEFINE(HOST_SP, UESP);
    DEFINE(HOST_EFLAGS, EFL);
    DEFINE(HOST_AX, EAX);
    DEFINE(HOST_BX, EBX);
    DEFINE(HOST_CX, ECX);
    DEFINE(HOST_DX, EDX);
    DEFINE(HOST_SI, ESI);
    DEFINE(HOST_DI, EDI);
    DEFINE(HOST_BP, EBP);
    DEFINE(HOST_CS, CS);
    DEFINE(HOST_SS, SS);
    DEFINE(HOST_DS, DS);
    DEFINE(HOST_FS, FS);
    DEFINE(HOST_ES, ES);
    DEFINE(HOST_GS, GS);
    DEFINE(HOST_ORIG_AX, ORIG_EAX);

    DEFINE_LONGS(HOST_BX, RBX);
    DEFINE_LONGS(HOST_CX, RCX);
    DEFINE_LONGS(HOST_DI, RDI);
    DEFINE_LONGS(HOST_SI, RSI);
    DEFINE_LONGS(HOST_DX, RDX);
    DEFINE_LONGS(HOST_BP, RBP);
    DEFINE_LONGS(HOST_AX, RAX);
    DEFINE_LONGS(HOST_R8, R8);
    DEFINE_LONGS(HOST_R9, R9);
    DEFINE_LONGS(HOST_R10, R10);
    DEFINE_LONGS(HOST_R11, R11);
    DEFINE_LONGS(HOST_R12, R12);
    DEFINE_LONGS(HOST_R13, R13);
    DEFINE_LONGS(HOST_R14, R14);
    DEFINE_LONGS(HOST_R15, R15);
    DEFINE_LONGS(HOST_ORIG_AX, ORIG_RAX);
    DEFINE_LONGS(HOST_CS, CS);
    DEFINE_LONGS(HOST_SS, SS);
    DEFINE_LONGS(HOST_EFLAGS, EFLAGS);

    DEFINE_LONGS(HOST_FS, FS);
    DEFINE_LONGS(HOST_GS, GS);
    DEFINE_LONGS(HOST_DS, DS);
    DEFINE_LONGS(HOST_ES, ES);

    DEFINE_LONGS(HOST_IP, RIP);
    DEFINE_LONGS(HOST_SP, RSP);

    DEFINE(UM_FRAME_SIZE, sizeof(struct user_regs_struct));
    DEFINE(UM_POLLIN, POLLIN);
    DEFINE(UM_POLLPRI, POLLPRI);
    DEFINE(UM_POLLOUT, POLLOUT);
    DEFINE(UM_PROT_READ, PROT_READ);
    DEFINE(UM_PROT_WRITE, PROT_WRITE);
    DEFINE(UM_PROT_EXEC, PROT_EXEC);
    }
