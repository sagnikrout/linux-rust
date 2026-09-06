//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/helpers/msr.c
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

// Intel specific MSRs
pub const MSR_IA32_PERF_STATUS: c_uint = 0x198;
pub const MSR_IA32_MISC_ENABLES: c_uint = 0x1a0;
pub const MSR_NEHALEM_TURBO_RATIO_LIMIT: c_uint = 0x1ad;
//
// read_msr
//
// Will return 0 on success and -1 on failure.
// Possible errno values could be:
// EFAULT -If the read/write did not fully complete
// EIO    -If the CPU does not support MSRs
// ENXIO  -If the CPU does not exist
//
#[no_mangle]
pub unsafe extern "C" fn read_msr(cpu: c_int, idx: c_uint, val: *mut c_ulonglong) -> c_int {
    int read_msr(int cpu, unsigned int idx, unsigned long long *val)
    {
    int fd;
    char msr_file_name[64];
    sprintf(msr_file_name, "/dev/cpu/%d/msr", cpu);
    fd = open(msr_file_name, O_RDONLY);
    if (fd < 0)
    return -1;
    if (lseek(fd, idx, SEEK_CUR) == -1)
    goto err;
    if (read(fd, val, sizeof *val) != sizeof *val)
    goto err;
    close(fd);
    return 0;
    err:
    close(fd);
    return -1;
    }
//
// write_msr
//
// Will return 0 on success and -1 on failure.
// Possible errno values could be:
// EFAULT -If the read/write did not fully complete
// EIO    -If the CPU does not support MSRs
// ENXIO  -If the CPU does not exist
//
#[no_mangle]
pub unsafe extern "C" fn write_msr(cpu: c_int, idx: c_uint, val: c_ulonglong) -> c_int {
    int write_msr(int cpu, unsigned int idx, unsigned long long val)
    {
    int fd;
    char msr_file_name[64];
    sprintf(msr_file_name, "/dev/cpu/%d/msr", cpu);
    fd = open(msr_file_name, O_WRONLY);
    if (fd < 0)
    return -1;
    if (lseek(fd, idx, SEEK_CUR) == -1)
    goto err;
    if (write(fd, &val, sizeof val) != sizeof val)
    goto err;
    close(fd);
    return 0;
    err:
    close(fd);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn msr_intel_get_turbo_ratio(cpu: c_uint) -> c_ulonglong {
    unsigned long long msr_intel_get_turbo_ratio(unsigned int cpu)
    {
    unsigned long long val;
    int ret;
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_HAS_TURBO_RATIO))
    return -1;
    ret = read_msr(cpu, MSR_NEHALEM_TURBO_RATIO_LIMIT, &val);
    if (ret)
    return ret;
    return val;
    }
