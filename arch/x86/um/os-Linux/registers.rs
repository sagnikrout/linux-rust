//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/os-Linux/registers.c
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
// Copyright (C) 2004 PathScale, Inc
// Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

    static unsigned long ptrace_regset;
    unsigned long host_fp_size;
#[no_mangle]
pub unsafe extern "C" fn get_fp_registers(pid: c_int, regs: *mut c_ulong) -> c_int {
    int get_fp_registers(int pid, unsigned long *regs)
    {
    struct iovec iov = {
    .iov_base = regs,
    .iov_len = host_fp_size,
    };
    if (ptrace(PTRACE_GETREGSET, pid, ptrace_regset, &iov) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn put_fp_registers(pid: c_int, regs: *mut c_ulong) -> c_int {
    int put_fp_registers(int pid, unsigned long *regs)
    {
    struct iovec iov = {
    .iov_base = regs,
    .iov_len = host_fp_size,
    };
    if (ptrace(PTRACE_SETREGSET, pid, ptrace_regset, &iov) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_init_registers(pid: c_int) -> c_int {
    int arch_init_registers(int pid)
    {
    struct iovec iov = {
// Just use plenty of space, it does not cost us anything
    .iov_len = 2 * 1024 * 1024,
    };
    int ret;
    iov.iov_base = mmap(core::ptr::null_mut(), iov.iov_len, PROT_WRITE | PROT_READ,
    MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (iov.iov_base == MAP_FAILED)
    return -ENOMEM;
// GDB has x86_xsave_length, which uses x86_cpuid_count
    ptrace_regset = NT_X86_XSTATE;
    ret = ptrace(PTRACE_GETREGSET, pid, ptrace_regset, &iov);
    if (ret)
    ret = -errno;
    if (ret == -ENODEV) {

    ptrace_regset = NT_PRXFPREG;

    ptrace_regset = NT_PRFPREG;

    iov.iov_len = 2 * 1024 * 1024;
    ret = ptrace(PTRACE_GETREGSET, pid, ptrace_regset, &iov);
    if (ret)
    ret = -errno;
    }
    munmap(iov.iov_base, 2 * 1024 * 1024);
    host_fp_size = iov.iov_len;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn get_thread_reg(reg: c_int, buf: *mut jmp_buf) -> c_ulong {
    unsigned long get_thread_reg(int reg, jmp_buf *buf)
    {
    switch (reg) {

    case HOST_IP:
    return buf[0].__eip;
    case HOST_SP:
    return buf[0].__esp;
    case HOST_BP:
    return buf[0].__ebp;

    case HOST_IP:
    return buf[0].__rip;
    case HOST_SP:
    return buf[0].__rsp;
    case HOST_BP:
    return buf[0].__rbp;

    default:
    printk(UM_KERN_ERR "get_thread_regs - unknown register %d\n",
    reg);
    return 0;
    }
    }
