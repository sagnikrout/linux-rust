//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/debug/i386/powernow-k8-decode.c
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
// (C) 2004 Bruno Ducrot <ducrot@poupinou.org>
//
// Based on code found in
// linux/arch/i386/kernel/cpu/cpufreq/powernow-k8.c
// and originally developed by Paul Devriendt
//

pub const MCPU: c_int = 32;
pub const MSR_FIDVID_STATUS: c_uint = 0xc0010042;
pub const MSR_S_HI_CURRENT_VID: c_uint = 0x0000001f;
pub const MSR_S_LO_CURRENT_FID: c_uint = 0x0000003f;
#[no_mangle]
unsafe extern "C" fn get_fidvid(cpu: u32, fid: *mut u32, vid: *mut u32) -> c_int {
    static int get_fidvid(uint32_t cpu, uint32_t *fid, uint32_t *vid)
    {
    let mut err: c_int = 1;
    let mut msr: u64 = 0;
    int fd;
    char file[20];
    if (cpu > MCPU)
    goto out;
    sprintf(file, "/dev/cpu/%d/msr", cpu);
    fd = open(file, O_RDONLY);
    if (fd < 0)
    goto out;
    lseek(fd, MSR_FIDVID_STATUS, SEEK_CUR);
    if (read(fd, &msr, 8) != 8)
    goto err1;
// fid = ((uint32_t )(msr & 0xffffffffull)) & MSR_S_LO_CURRENT_FID;
// vid = ((uint32_t )(msr>>32 & 0xffffffffull)) & MSR_S_HI_CURRENT_VID;
    err = 0;
    err1:
    close(fd);
    out:
    return err;
    }
// Return a frequency in MHz, given an input fid
#[no_mangle]
unsafe extern "C" fn find_freq_from_fid(fid: u32) -> u32 {
    static uint32_t find_freq_from_fid(uint32_t fid)
    {
    return 800 + (fid * 100);
    }
// Return a voltage in miliVolts, given an input vid
#[no_mangle]
unsafe extern "C" fn find_millivolts_from_vid(vid: u32) -> u32 {
    static uint32_t find_millivolts_from_vid(uint32_t vid)
    {
    return 1550-vid*25;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main (int argc, char *argv[])
    {
    int err;
    int cpu;
    uint32_t fid, vid;
    if (argc < 2)
    cpu = 0;
    else
    cpu = strtoul(argv[1], core::ptr::null_mut(), 0);
    err = get_fidvid(cpu, &fid, &vid);
    if (err) {
    printf("can't get fid, vid from MSR\n");
    printf("Possible trouble: you don't run a powernow-k8 capable cpu\n");
    printf("or you are not root, or the msr driver is not present\n");
    exit(1);
    }
    printf("cpu %d currently at %d MHz and %d mV\n",
    cpu,
    find_freq_from_fid(fid),
    find_millivolts_from_vid(vid));
    return 0;
    }
