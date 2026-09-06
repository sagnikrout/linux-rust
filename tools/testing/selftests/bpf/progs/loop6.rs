//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/loop6.c
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

    char _license[] SEC("license") = "GPL";
// typically virtio scsi has max SGs of 6
pub const VIRTIO_MAX_SGS: c_int = 6;
// Verifier will fail with SG_MAX = 128. The failure can be
// workarounded with a smaller SG_MAX, e.g. 10.
//
// Macro flag: #define WORKAROUND

pub const SG_MAX: c_int = 10;

// typically virtio blk has max SEG of 128
pub const SG_MAX: c_int = 128;

pub const SG_CHAIN: c_uint = 0x01UL;
pub const SG_END: c_uint = 0x02UL;

    ((struct scatterlist *) ((sg).page_link & ~(SG_CHAIN | SG_END)))
    static inline struct scatterlist *__sg_next(struct scatterlist *sgp)
    {
    struct scatterlist sg;
    bpf_probe_read_kernel(&sg, sizeof(sg), sgp);
    if (sg_is_last(&sg))
    return core::ptr::null_mut();
    sgp++;
    bpf_probe_read_kernel(&sg, sizeof(sg), sgp);
    if (sg_is_chain(&sg))
    sgp = sg_chain_ptr(&sg);
    return sgp;
    }
    static inline struct scatterlist *get_sgp(struct scatterlist **sgs, int i)
    {
    struct scatterlist *sgp;
    bpf_probe_read_kernel(&sgp, sizeof(sgp), sgs + i);
    return sgp;
    }
    let mut run_once: c_int = 0;
    let mut result: c_int = 0;
    SEC("kprobe/virtqueue_add_sgs")
    int BPF_KPROBE(trace_virtqueue_add_sgs, void *unused, struct scatterlist **sgs,
    unsigned int out_sgs, unsigned int in_sgs)
    {
    struct scatterlist *sgp = core::ptr::null_mut();
    let mut length1: __u64 = 0, length2 = 0;
    unsigned int i, n, len;
    if (run_once != 0)
    return 0;
    for (i = 0; (i < VIRTIO_MAX_SGS) && (i < out_sgs); i++) {
    __sink(out_sgs);
    for (n = 0, sgp = get_sgp(sgs, i); sgp && (n < SG_MAX);
    sgp = __sg_next(sgp)) {
    len = BPF_CORE_READ(sgp, length);
    length1 += len;
    n++;
    }
    }
    for (i = 0; (i < VIRTIO_MAX_SGS) && (i < in_sgs); i++) {
    __sink(in_sgs);
    for (n = 0, sgp = get_sgp(sgs, i); sgp && (n < SG_MAX);
    sgp = __sg_next(sgp)) {
    len = BPF_CORE_READ(sgp, length);
    length2 += len;
    n++;
    }
    }
    run_once = 1;
    result = length2 - length1;
    return 0;
    }
