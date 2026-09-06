//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/sys_riscv.c
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
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2014 Darius Rad <darius@bluespec.com>
// Copyright (C) 2017 SiFive
//

    static long riscv_sys_mmap(unsigned long addr, unsigned long len,
    unsigned long prot, unsigned long flags,
    unsigned long fd, unsigned long offset,
    unsigned long page_shift_offset)
    {
    if (unlikely(offset & (~PAGE_MASK >> page_shift_offset)))
    return -EINVAL;
//
// If PROT_WRITE is specified then extend that to PROT_READ
// protection_map[VM_WRITE] is now going to select shadow stack encodings.
// So specifying PROT_WRITE actually should select protection_map [VM_WRITE | VM_READ]
// If user wants to create shadow stack then they should use `map_shadow_stack` syscall.
//
    if (unlikely((prot & PROT_WRITE) && !(prot & PROT_READ)))
    prot |= PROT_READ;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd,
    offset >> (PAGE_SHIFT - page_shift_offset));
    }

    SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, offset)
    {
    return riscv_sys_mmap(addr, len, prot, flags, fd, offset, 0);
    }

    SYSCALL_DEFINE6(mmap2, unsigned long, addr, unsigned long, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, offset)
    {
//
// Note that the shift for mmap2 is constant (12),
// regardless of PAGE_SIZE
//
    return riscv_sys_mmap(addr, len, prot, flags, fd, offset, 12);
    }

//
// Allows the instruction cache to be flushed from userspace.  Despite RISC-V
// having a direct 'fence.i' instruction available to userspace (which we
// can't trap!), that's not actually viable when running on Linux because the
// kernel might schedule a process on another hart.  There is no way for
// userspace to handle this without invoking the kernel (as it doesn't know the
// thread->hart mappings), so we've defined a RISC-V specific system call to
// flush the instruction cache.
//
// sys_riscv_flush_icache() is defined to flush the instruction cache over an
// address range, with the flush applying to either all threads or just the
// caller.  We don't currently do anything with the address range, that's just
// in there for forwards compatibility.
//
    SYSCALL_DEFINE3(riscv_flush_icache, uintptr_t, start, uintptr_t, end,
    uintptr_t, flags)
    {
// Check the reserved flags.
    if (unlikely(flags & ~SYS_RISCV_FLUSH_ICACHE_ALL))
    return -EINVAL;
    flush_icache_mm(current.mm, flags & SYS_RISCV_FLUSH_ICACHE_LOCAL);
    return 0;
    }
// Not defined using SYSCALL_DEFINE0 to avoid error injection
#[no_mangle]
pub unsafe extern "C" fn __riscv_sys_ni_syscall(__unused: *const pt_regs) -> asmlinkage long {
    asmlinkage long __riscv_sys_ni_syscall(const struct pt_regs *__unused)
    {
    return -ENOSYS;
    }
