//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/vdso/vgetcpu.c
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
// Fast user context implementation of getcpu()
//

#[no_mangle]
unsafe extern "C" fn read_cpu_id() -> __always_inline int {
    static __always_inline int read_cpu_id(void)
    {
    int cpu_id;

    __asm__ __volatile__(
    "	rdtime.d $zero, %0\n"
    : "=r" (cpu_id)
    :
    : "memory");

    __asm__ __volatile__(
    "	rdtimel.w $zero, %0\n"
    : "=r" (cpu_id)
    :
    : "memory");

    return cpu_id;
    }
    extern
    int __vdso_getcpu(unsigned int *cpu, unsigned int *node, void *unused);
#[no_mangle]
pub unsafe extern "C" fn __vdso_getcpu(cpu: *mut c_uint, node: *mut c_uint, unused: *mut c_void) -> c_int {
    int __vdso_getcpu(unsigned int *cpu, unsigned int *node, void *unused)
    {
    int cpu_id;
    cpu_id = read_cpu_id();
    if (cpu)
// cpu = cpu_id;
    if (node)
// node = vdso_u_arch_data.pdata[cpu_id].node;
    return 0;
    }
