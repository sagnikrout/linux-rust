//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/smp.c
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
// Copyright (C) 2025 Ant Group
// Author: Tiwei Bie <tiwei.btw@antgroup.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_thread_data {
    pub cpu: c_int,
    pub sigset: sigset_t,
}

    static __thread int __curr_cpu;
#[no_mangle]
pub unsafe extern "C" fn uml_curr_cpu() -> c_int {
    int uml_curr_cpu(void)
    {
    return __curr_cpu;
    }
    static pthread_t cpu_threads[CONFIG_NR_CPUS];
    static void *cpu_thread(void *arg)
    {
    struct cpu_thread_data *data = arg;
    __curr_cpu = data.cpu;
    uml_start_secondary(data);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn os_start_cpu_thread(cpu: c_int) -> c_int {
    int os_start_cpu_thread(int cpu)
    {
    struct cpu_thread_data *data;
    sigset_t sigset, oset;
    int err;
    data = uml_kmalloc(sizeof(*data), UM_GFP_ATOMIC);
    if (!data)
    return -ENOMEM;
    sigfillset(&sigset);
    if (sigprocmask(SIG_SETMASK, &sigset, &oset) < 0) {
    err = errno;
    goto err;
    }
    data.cpu = cpu;
    data.sigset = oset;
    err = pthread_create(&cpu_threads[cpu], core::ptr::null_mut(), cpu_thread, data);
    if (sigprocmask(SIG_SETMASK, &oset, core::ptr::null_mut()) < 0)
    panic("Failed to restore the signal mask, errno = %d", errno);
    if (err != 0)
    goto err;
    return 0;
    err:
    kfree(data);
    return -err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_start_secondary(arg: *mut c_void, switch_buf: *mut jmp_buf) {
    void os_start_secondary(void *arg, jmp_buf *switch_buf)
    {
    struct cpu_thread_data *data = arg;
    sigaddset(&data.sigset, IPI_SIGNAL);
    sigaddset(&data.sigset, SIGIO);
    if (sigprocmask(SIG_SETMASK, &data.sigset, core::ptr::null_mut()) < 0)
    panic("Failed to restore the signal mask, errno = %d", errno);
    kfree(data);
    longjmp(*switch_buf, 1);
// unreachable
    printk(UM_KERN_ERR "impossible long jump!");
    fatal_sigsegv();
    }
#[no_mangle]
pub unsafe extern "C" fn os_send_ipi(cpu: c_int, vector: c_int) -> c_int {
    int os_send_ipi(int cpu, int vector)
    {
    let mut value: union sigval = { .sival_int = vector };
    return pthread_sigqueue(cpu_threads[cpu], IPI_SIGNAL, value);
    }
#[no_mangle]
unsafe extern "C" fn __local_ipi_set(enable: c_int) {
    static void __local_ipi_set(int enable)
    {
    sigset_t sigset;
    sigemptyset(&sigset);
    sigaddset(&sigset, IPI_SIGNAL);
    if (sigprocmask(enable ? SIG_UNBLOCK : SIG_BLOCK, &sigset, core::ptr::null_mut()) < 0)
    panic("%s: sigprocmask failed, errno = %d", __func__, errno);
    }
#[no_mangle]
pub unsafe extern "C" fn os_local_ipi_enable() {
    void os_local_ipi_enable(void)
    {
    __local_ipi_set(1);
    }
#[no_mangle]
pub unsafe extern "C" fn os_local_ipi_disable() {
    void os_local_ipi_disable(void)
    {
    __local_ipi_set(0);
    }
#[no_mangle]
unsafe extern "C" fn ipi_sig_handler(sig: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    static void ipi_sig_handler(int sig, siginfo_t *si, void *uc)
    {
    let mut save_errno: c_int = errno;
    signals_enabled = 0;
    um_trace_signals_off();
    uml_ipi_handler(si.si_value.sival_int);
    um_trace_signals_on();
    signals_enabled = 1;
    errno = save_errno;
    }
#[no_mangle]
pub unsafe extern "C" fn os_init_smp() -> void __init {
    void __init os_init_smp(void)
    {
    struct sigaction action = {
    .sa_sigaction = ipi_sig_handler,
    .sa_flags = SA_SIGINFO | SA_ONSTACK | SA_RESTART,
    };
    sigfillset(&action.sa_mask);
    if (sigaction(IPI_SIGNAL, &action, core::ptr::null_mut()) < 0)
    panic("%s: sigaction failed, errno = %d", __func__, errno);
    cpu_threads[0] = pthread_self();
    }
