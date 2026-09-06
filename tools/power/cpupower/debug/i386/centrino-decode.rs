//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/debug/i386/centrino-decode.c
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
// (C) 2003 - 2004  Dominik Brodowski <linux@dominikbrodowski.de>
//
// Based on code found in
// linux/arch/i386/kernel/cpu/cpufreq/speedstep-centrino.c
// and originally developed by Jeremy Fitzhardinge.
//
// USAGE: simply run it to decode the current settings on CPU 0,
// or pass the CPU number as argument, or pass the MSR content
// as argument.
//

pub const MCPU: c_int = 32;
pub const MSR_IA32_PERF_STATUS: c_uint = 0x198;
    static int rdmsr(unsigned int cpu, unsigned int msr,
    unsigned int *lo, unsigned int *hi)
    {
    int fd;
    char file[20];
    unsigned long long val;
    let mut retval: c_int = -1;
// lo = *hi = 0;
    if (cpu > MCPU)
    goto err1;
    sprintf(file, "/dev/cpu/%d/msr", cpu);
    fd = open(file, O_RDONLY);
    if (fd < 0)
    goto err1;
    if (lseek(fd, msr, SEEK_CUR) == -1)
    goto err2;
    if (read(fd, &val, 8) != 8)
    goto err2;
// lo = (uint32_t )(val & 0xffffffffull);
// hi = (uint32_t )(val>>32 & 0xffffffffull);
    retval = 0;
    err2:
    close(fd);
    err1:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn decode(msr: c_uint) {
    static void decode (unsigned int msr)
    {
    unsigned int multiplier;
    unsigned int mv;
    multiplier = ((msr >> 8) & 0xFF);
    mv = (((msr & 0xFF) * 16) + 700);
    printf("0x%x means multiplier %d @ %d mV\n", msr, multiplier, mv);
    }
#[no_mangle]
unsafe extern "C" fn decode_live(cpu: c_uint) -> c_int {
    static int decode_live(unsigned int cpu)
    {
    unsigned int lo, hi;
    int err;
    err = rdmsr(cpu, MSR_IA32_PERF_STATUS, &lo, &hi);
    if (err) {
    printf("can't get MSR_IA32_PERF_STATUS for cpu %d\n", cpu);
    printf("Possible trouble: you don't run an Enhanced SpeedStep capable cpu\n");
    printf("or you are not root, or the msr driver is not present\n");
    return 1;
    }
    decode(lo);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main (int argc, char **argv)
    {
    unsigned int cpu, mode = 0;
    if (argc < 2)
    cpu = 0;
    else {
    cpu = strtoul(argv[1], core::ptr::null_mut(), 0);
    if (cpu >= MCPU)
    mode = 1;
    }
    if (mode)
    decode(cpu);
    else
    decode_live(cpu);
    return 0;
    }
