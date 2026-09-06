//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/bugs_32.c
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


//
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

// Set during early boot
    let mut host_has_cmov: static int = 1;
    static jmp_buf cmov_test_return;
#[no_mangle]
unsafe extern "C" fn cmov_sigill_test_handler(sig: c_int) {
    static void cmov_sigill_test_handler(int sig)
    {
    host_has_cmov = 0;
    longjmp(cmov_test_return, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_check_bugs() {
    void arch_check_bugs(void)
    {
    struct sigaction old, new;
    printk(UM_KERN_INFO "Checking for host processor cmov support...");
    new.sa_handler = cmov_sigill_test_handler;
// Make sure that SIGILL is enabled after the handler longjmps back
    new.sa_flags = SA_NODEFER;
    sigemptyset(&new.sa_mask);
    sigaction(SIGILL, &new, &old);
    if (setjmp(cmov_test_return) == 0) {
    let mut foo: c_ulong = 0;
    __asm__ __volatile__("cmovz %0, %1" : "=r" (foo) : "0" (foo));
    printk(UM_KERN_CONT "Yes\n");
    } else
    printk(UM_KERN_CONT "No\n");
    sigaction(SIGILL, &old, &new);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_examine_signal(sig: c_int, regs: *mut uml_pt_regs) {
    void arch_examine_signal(int sig, struct uml_pt_regs *regs)
    {
    unsigned char tmp[2];
//
// This is testing for a cmov (0x0f 0x4x) instruction causing a
// SIGILL in init.
//
    if ((sig != SIGILL) || (get_current_pid() != 1))
    return;
    if (copy_from_user_proc(tmp, (void *) UPT_IP(regs), 2)) {
    printk(UM_KERN_ERR "SIGILL in init, could not read "
    "instructions!\n");
    return;
    }
    if ((tmp[0] != 0x0f) || ((tmp[1] & 0xf0) != 0x40))
    return;
    if (host_has_cmov == 0)
    printk(UM_KERN_ERR "SIGILL caused by cmov, which this "
    "processor doesn't implement.  Boot a filesystem "
    "compiled for older processors");
#[no_mangle]
pub unsafe extern "C" fn if(1: host_has_cmov ==) -> else {
    else if (host_has_cmov == 1)
    printk(UM_KERN_ERR "SIGILL caused by cmov, which this "
    "processor claims to implement");
    else
    printk(UM_KERN_ERR "Bad value for host_has_cmov (%d)",
    host_has_cmov);
    }
