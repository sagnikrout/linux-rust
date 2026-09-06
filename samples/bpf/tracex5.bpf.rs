//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex5.bpf.c
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


// Copyright (c) 2015 PLUMgrid, http://plumgrid.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));

    __uint(max_entries, 6000); /* MIPS n64 syscalls start at 5000 */

    __uint(max_entries, 1024);

    } progs SEC(".maps");
    SEC("kprobe/__seccomp_filter")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
    let mut sc_nr: c_int = (int)PT_REGS_PARM1(ctx);
// dispatch into next BPF program depending on syscall number
    bpf_tail_call(ctx, &progs, sc_nr);
// fall through -> unknown syscall
    if (sc_nr >= __NR_getuid && sc_nr <= __NR_getsid) {
    char fmt[] = "syscall=%d (one of get/set uid/pid/gid)\n";
    bpf_trace_printk(fmt, sizeof(fmt), sc_nr);
    }
    return 0;
    }
// we jump here when syscall number == __NR_write
    PROG(SYS__NR_write)(struct pt_regs *ctx)
    {
    struct seccomp_data sd;
    bpf_core_read(&sd, sizeof(sd), (void *)PT_REGS_PARM2(ctx));
    if (sd.args[2] == 512) {
    char fmt[] = "write(fd=%d, buf=%p, size=%d)\n";
    bpf_trace_printk(fmt, sizeof(fmt),
    sd.args[0], sd.args[1], sd.args[2]);
    }
    return 0;
    }
    PROG(SYS__NR_read)(struct pt_regs *ctx)
    {
    struct seccomp_data sd;
    bpf_core_read(&sd, sizeof(sd), (void *)PT_REGS_PARM2(ctx));
    if (sd.args[2] > 128 && sd.args[2] <= 1024) {
    char fmt[] = "read(fd=%d, buf=%p, size=%d)\n";
    bpf_trace_printk(fmt, sizeof(fmt),
    sd.args[0], sd.args[1], sd.args[2]);
    }
    return 0;
    }

    PROG(SYS__NR_mmap2)(struct pt_regs *ctx)
    {
    char fmt[] = "mmap2\n";
    bpf_trace_printk(fmt, sizeof(fmt));
    return 0;
    }

    PROG(SYS__NR_mmap)(struct pt_regs *ctx)
    {
    char fmt[] = "mmap\n";
    bpf_trace_printk(fmt, sizeof(fmt));
    return 0;
    }

    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
