//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/kaslr.c
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
// Entropy functions used on early boot for KASLR base and memory
// randomization. The base randomization is done in the compressed
// kernel and memory randomization is done early when the regular
// kernel starts. This file is included in the compressed kernel and
// normally linked in the regular.
//

//
// When built for the regular kernel, several functions need to be stubbed out
// or changed to their regular kernel equivalent.
//

pub const I8254_PORT_CONTROL: c_uint = 0x43;
pub const I8254_PORT_COUNTER0: c_uint = 0x40;
pub const I8254_CMD_READBACK: c_uint = 0xC0;
pub const I8254_SELECT_COUNTER0: c_uint = 0x02;
pub const I8254_STATUS_NOTREADY: c_uint = 0x40;
#[no_mangle]
pub unsafe extern "C" fn i8254() -> u16 {
    static inline u16 i8254(void)
    {
    u16 status, timer;
    do {
    outb(I8254_CMD_READBACK | I8254_SELECT_COUNTER0,
    I8254_PORT_CONTROL);
    status = inb(I8254_PORT_COUNTER0);
    timer  = inb(I8254_PORT_COUNTER0);
    timer |= inb(I8254_PORT_COUNTER0) << 8;
    } while (status & I8254_STATUS_NOTREADY);
    return timer;
    }
#[no_mangle]
pub unsafe extern "C" fn kaslr_get_random_long(purpose: *const c_char) -> c_ulong {
    unsigned long kaslr_get_random_long(const char *purpose)
    {

    let mut mix_const: c_ulong = 0x5d6008cbf3848dd3UL;

    let mut mix_const: c_ulong = 0x3f39e593UL;

    unsigned long raw, random = get_boot_seed();
    let mut use_i8254: bool = true;
    if (purpose) {
    debug_putstr(purpose);
    debug_putstr(" KASLR using");
    }
    if (has_cpuflag(X86_FEATURE_RDRAND)) {
    if (purpose)
    debug_putstr(" RDRAND");
    if (rdrand_long(&raw)) {
    random ^= raw;
    use_i8254 = false;
    }
    }
    if (has_cpuflag(X86_FEATURE_TSC)) {
    if (purpose)
    debug_putstr(" RDTSC");
    raw = rdtsc();
    random ^= raw;
    use_i8254 = false;
    }
    if (use_i8254) {
    if (purpose)
    debug_putstr(" i8254");
    random ^= i8254();
    }
// Circular multiply for better bit diffusion
    asm(_ASM_MUL "%3"
    : "=a" (random), "=d" (raw)
    : "a" (random), "rm" (mix_const));
    random += raw;
    if (purpose)
    debug_putstr("...\n");
    return random;
    }
