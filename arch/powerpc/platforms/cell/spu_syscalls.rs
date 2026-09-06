//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spu_syscalls.c
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
// SPU file system -- system call stubs
//
// (C) Copyright IBM Deutschland Entwicklung GmbH 2005
// (C) Copyright 2006-2007, IBM Corporation
//
// Author: Arnd Bergmann <arndb@de.ibm.com>
//

// protected by rcu
    static struct spufs_calls *spufs_calls;

    static inline struct spufs_calls *spufs_calls_get(void)
    {
    struct spufs_calls *calls = core::ptr::null_mut();
    rcu_read_lock();
    calls = rcu_dereference(spufs_calls);
    if (calls && !try_module_get(calls.owner))
    calls = core::ptr::null_mut();
    rcu_read_unlock();
    return calls;
    }
#[no_mangle]
pub unsafe extern "C" fn spufs_calls_put(calls: *mut spufs_calls) {
    static inline void spufs_calls_put(struct spufs_calls *calls)
    {
    if (!calls)
    return;
    BUG_ON(calls != spufs_calls);
// we don't need to rcu this, as we hold a reference to the module
    module_put(spufs_calls.owner);
    }

    static inline struct spufs_calls *spufs_calls_get(void)
    {
    return spufs_calls;
    }
    static inline void spufs_calls_put(struct spufs_calls *calls) { }

    DEFINE_CLASS(spufs_calls, struct spufs_calls *, spufs_calls_put(_T), spufs_calls_get(), void)
    SYSCALL_DEFINE4(spu_create, const char __user *, name, unsigned int, flags,
    umode_t, mode, int, neighbor_fd)
    {
    CLASS(spufs_calls, calls)();
    if (!calls)
    return -ENOSYS;
    if (flags & SPU_CREATE_AFFINITY_SPU) {
    CLASS(fd, neighbor)(neighbor_fd);
    if (fd_empty(neighbor))
    return -EBADF;
    return calls.create_thread(name, flags, mode, fd_file(neighbor));
    } else {
    return calls.create_thread(name, flags, mode, core::ptr::null_mut());
    }
    }
    SYSCALL_DEFINE3(spu_run,int, fd, __u32 __user *, unpc, __u32 __user *, ustatus)
    {
    CLASS(spufs_calls, calls)();
    if (!calls)
    return -ENOSYS;
    CLASS(fd, arg)(fd);
    if (fd_empty(arg))
    return -EBADF;
    return calls.spu_run(fd_file(arg), unpc, ustatus);
    }

#[no_mangle]
pub unsafe extern "C" fn elf_coredump_extra_notes_size() -> c_int {
    int elf_coredump_extra_notes_size(void)
    {
    CLASS(spufs_calls, calls)();
    if (!calls)
    return 0;
    return calls.coredump_extra_notes_size();
    }
#[no_mangle]
pub unsafe extern "C" fn elf_coredump_extra_notes_write(cprm: *mut coredump_params) -> c_int {
    int elf_coredump_extra_notes_write(struct coredump_params *cprm)
    {
    CLASS(spufs_calls, calls)();
    if (!calls)
    return 0;
    return calls.coredump_extra_notes_write(cprm);
    }

#[no_mangle]
pub unsafe extern "C" fn notify_spus_active() {
    void notify_spus_active(void)
    {
    struct spufs_calls *calls;
    calls = spufs_calls_get();
    if (!calls)
    return;
    calls.notify_spus_active();
    spufs_calls_put(calls);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn register_spu_syscalls(calls: *mut spufs_calls) -> c_int {
    int register_spu_syscalls(struct spufs_calls *calls)
    {
    if (spufs_calls)
    return -EBUSY;
    rcu_assign_pointer(spufs_calls, calls);
    return 0;
    }
    EXPORT_SYMBOL_GPL(register_spu_syscalls);
#[no_mangle]
pub unsafe extern "C" fn unregister_spu_syscalls(calls: *mut spufs_calls) {
    void unregister_spu_syscalls(struct spufs_calls *calls)
    {
    BUG_ON(spufs_calls.owner != calls.owner);
    RCU_INIT_POINTER(spufs_calls, core::ptr::null_mut());
    synchronize_rcu();
    }
    EXPORT_SYMBOL_GPL(unregister_spu_syscalls);
