//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spu_callbacks.c
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
// System call callback functions for SPUs
//

//
// This table defines the system calls that an SPU can call.
// It is currently a subset of the 64 bit powerpc system calls,
// with the exact semantics.
//
// The reasons for disabling some of the system calls are:
// 1. They interact with the way SPU syscalls are handled
// and we can't let them execute ever:
// restart_syscall, exit, for, execve, ptrace, ...
// 2. They are deprecated and replaced by other means:
// uselib, pciconfig_*, sysfs, ...
// 3. They are somewhat interacting with the system in a way
// we don't want an SPU to:
// reboot, init_module, mount, kexec_load
// 4. They are optional and we can't rely on them being
// linked into the kernel. Unfortunately, the cond_syscall
// helper does not work here as it does not add the necessary
// opd symbols:
// mbind, mq_open, ipc, ...
//
    static const syscall_fn spu_syscall_table[] = {

    };
#[no_mangle]
pub unsafe extern "C" fn spu_sys_callback(s: *mut spu_syscall_block) -> c_long {
    long spu_sys_callback(struct spu_syscall_block *s)
    {
    syscall_fn syscall;
    if (s.nr_ret >= ARRAY_SIZE(spu_syscall_table)) {
    pr_debug("%s: invalid syscall #%lld", __func__, s.nr_ret);
    return -ENOSYS;
    }
    syscall = spu_syscall_table[s.nr_ret];
    pr_debug("SPU-syscall "
    "%pSR:syscall%lld(%llx, %llx, %llx, %llx, %llx, %llx)\n",
    syscall,
    s.nr_ret,
    s.parm[0], s.parm[1], s.parm[2],
    s.parm[3], s.parm[4], s.parm[5]);
    return syscall(s.parm[0], s.parm[1], s.parm[2],
    s.parm[3], s.parm[4], s.parm[5]);
    }
    EXPORT_SYMBOL_GPL(spu_sys_callback);
