//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/srso.c
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
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct perf_event_attr ret_attr, mret_attr;
    long long count_rets, count_rets_mispred;
    int rrets_fd, mrrets_fd;
    unsigned int cpuid1_eax, b, c, d;
    __cpuid(1, cpuid1_eax, b, c, d);
    if (cpuid1_eax < 0x00800f00 ||
    cpuid1_eax > 0x00afffff) {
    fprintf(stderr, "This needs to run on a Zen[1-4] machine (CPUID(1).EAX: 0x%x). Exiting...\n", cpuid1_eax);
    exit(EXIT_FAILURE);
    }
    memset(&ret_attr, 0, sizeof(struct perf_event_attr));
    memset(&mret_attr, 0, sizeof(struct perf_event_attr));
    ret_attr.type = mret_attr.type = PERF_TYPE_RAW;
    ret_attr.size = mret_attr.size = sizeof(struct perf_event_attr);
    ret_attr.config = 0xc8;
    mret_attr.config = 0xc9;
    ret_attr.disabled = mret_attr.disabled = 1;
    ret_attr.exclude_user = mret_attr.exclude_user = 1;
    ret_attr.exclude_hv = mret_attr.exclude_hv = 1;
    rrets_fd = syscall(SYS_perf_event_open, &ret_attr, 0, -1, -1, 0);
    if (rrets_fd == -1) {
    perror("opening retired RETs fd");
    exit(EXIT_FAILURE);
    }
    mrrets_fd = syscall(SYS_perf_event_open, &mret_attr, 0, -1, -1, 0);
    if (mrrets_fd == -1) {
    perror("opening retired mispredicted RETs fd");
    exit(EXIT_FAILURE);
    }
    ioctl(rrets_fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(mrrets_fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(rrets_fd, PERF_EVENT_IOC_ENABLE, 0);
    ioctl(mrrets_fd, PERF_EVENT_IOC_ENABLE, 0);
    printf("Sleeping for 10 seconds\n");
    sleep(10);
    ioctl(rrets_fd, PERF_EVENT_IOC_DISABLE, 0);
    ioctl(mrrets_fd, PERF_EVENT_IOC_DISABLE, 0);
    read(rrets_fd, &count_rets, sizeof(long long));
    read(mrrets_fd, &count_rets_mispred, sizeof(long long));
    printf("RETs: (%lld retired <. %lld mispredicted)\n",
    count_rets, count_rets_mispred);
    printf("SRSO Safe-RET mitigation works correctly if both counts are almost equal.\n");
    return 0;
    }
