//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/static_call.c
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

#[no_mangle]
pub unsafe extern "C" fn arch_static_call_transform(site: *mut c_void, tramp: *mut c_void, func: *mut c_void, tail: bool) {
    void arch_static_call_transform(void *site, void *tramp, void *func, bool tail)
    {
    u64 literal;
    int ret;
    if (!func)
    func = __static_call_return0;
// decode the instructions to discover the literal address
    literal = ALIGN_DOWN((u64)tramp + 4, SZ_4K) +
    aarch64_insn_adrp_get_offset(le32_to_cpup(tramp + 4)) +
    8 * aarch64_insn_decode_immediate(AARCH64_INSN_IMM_12,
    le32_to_cpup(tramp + 8));
    ret = aarch64_insn_write_literal_u64((void *)literal, (u64)func);
    WARN_ON_ONCE(ret);
    }
    EXPORT_SYMBOL_GPL(arch_static_call_transform);
