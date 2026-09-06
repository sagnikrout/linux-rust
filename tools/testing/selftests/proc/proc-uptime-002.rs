//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-uptime-002.c
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


//
// Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Test that boottime value in /proc/uptime and CLOCK_BOOTTIME increment
// monotonically while shifting across CPUs. We don't test idle time
// monotonicity due to broken iowait task counting, cf: comment above
// get_cpu_idle_time_us()

#[no_mangle]
pub unsafe extern "C" fn sys_sched_getaffinity(pid: pid_t, len: c_uint, m: *mut c_ulong) -> c_int {
    static inline int sys_sched_getaffinity(pid_t pid, unsigned int len, unsigned long *m)
    {
    return syscall(SYS_sched_getaffinity, pid, len, m);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setaffinity(pid: pid_t, len: c_uint, m: *mut c_ulong) -> c_int {
    static inline int sys_sched_setaffinity(pid_t pid, unsigned int len, unsigned long *m)
    {
    return syscall(SYS_sched_setaffinity, pid, len, m);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    uint64_t u0, u1, c0, c1;
    unsigned int len;
    unsigned long *m;
    unsigned int cpu;
    int fd;
// find out "nr_cpu_ids"
    m = core::ptr::null_mut();
    len = 0;
    do {
    len += sizeof(unsigned long);
    free(m);
    m = malloc(len);
    } while (sys_sched_getaffinity(0, len, m) == -1 && errno == EINVAL);
    fd = open("/proc/uptime", O_RDONLY);
    assert(fd >= 0);
    u0 = proc_uptime(fd);
    c0 = clock_boottime();
    for (cpu = 0; cpu < len * 8; cpu++) {
    memset(m, 0, len);
    m[cpu / (8 * sizeof(unsigned long))] |= 1UL << (cpu % (8 * sizeof(unsigned long)));
// CPU might not exist, ignore error
    sys_sched_setaffinity(0, len, m);
    u1 = proc_uptime(fd);
    c1 = clock_boottime();
// Is /proc/uptime monotonic ?
    assert(u1 >= u0);
// Is CLOCK_BOOTTIME monotonic ?
    assert(c1 >= c0);
// Is CLOCK_BOOTTIME VS /proc/uptime monotonic ?
    assert(c0 >= u0);
    u0 = u1;
    c0 = c1;
    }
    return 0;
    }
