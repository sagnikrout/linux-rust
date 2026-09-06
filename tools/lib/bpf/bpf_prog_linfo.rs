//! Automatically rewritten from C to Rust
//! Source: tools/lib/bpf/bpf_prog_linfo.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2018 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_linfo {
    pub raw_linfo: *mut c_void,
    pub raw_jited_linfo: *mut c_void,
    pub nr_jited_linfo_per_func: *mut __u32,
    pub jited_linfo_func_idx: *mut __u32,
    pub nr_linfo: __u32,
    pub nr_jited_func: __u32,
    pub rec_size: __u32,
    pub jited_rec_size: __u32,
}

    static int dissect_jited_func(struct bpf_prog_linfo *prog_linfo,
    const __u64 *ksym_func, const __u32 *ksym_len)
    {
    __u32 nr_jited_func, nr_linfo;
    const void *raw_jited_linfo;
    const __u64 *jited_linfo;
    __u64 last_jited_linfo;
//
// Index to raw_jited_linfo:
// i: Index for searching the next ksym_func
// prev_i: Index to the last found ksym_func
//
    __u32 i, prev_i;
    __u32 f; /* Index to ksym_func */
    raw_jited_linfo = prog_linfo.raw_jited_linfo;
    jited_linfo = raw_jited_linfo;
    if (ksym_func[0] != *jited_linfo)
    goto errout;
    prog_linfo.jited_linfo_func_idx[0] = 0;
    nr_jited_func = prog_linfo.nr_jited_func;
    nr_linfo = prog_linfo.nr_linfo;
    for (prev_i = 0, i = 1, f = 1;
    i < nr_linfo && f < nr_jited_func;
    i++) {
    raw_jited_linfo += prog_linfo.jited_rec_size;
    last_jited_linfo = *jited_linfo;
    jited_linfo = raw_jited_linfo;
    if (ksym_func[f] == *jited_linfo) {
    prog_linfo.jited_linfo_func_idx[f] = i;
// Sanity check
    if (last_jited_linfo - ksym_func[f - 1] + 1 >
    ksym_len[f - 1])
    goto errout;
    prog_linfo.nr_jited_linfo_per_func[f - 1] =
    i - prev_i;
    prev_i = i;
//
// The ksym_func[f] is found in jited_linfo.
// Look for the next one.
//
    f++;
    } else if (*jited_linfo <= last_jited_linfo) {
// Ensure the addr is increasing _within_ a func
    goto errout;
    }
    }
    if (f != nr_jited_func)
    goto errout;
    prog_linfo.nr_jited_linfo_per_func[nr_jited_func - 1] =
    nr_linfo - prev_i;
    return 0;
    errout:
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_linfo__free(prog_linfo: *mut bpf_prog_linfo) {
    void bpf_prog_linfo__free(struct bpf_prog_linfo *prog_linfo)
    {
    if (!prog_linfo)
    return;
    free(prog_linfo.raw_linfo);
    free(prog_linfo.raw_jited_linfo);
    free(prog_linfo.nr_jited_linfo_per_func);
    free(prog_linfo.jited_linfo_func_idx);
    free(prog_linfo);
    }
    struct bpf_prog_linfo *bpf_prog_linfo__new(const struct bpf_prog_info *info)
    {
    struct bpf_prog_linfo *prog_linfo;
    __u32 nr_linfo, nr_jited_func;
    __u64 data_sz;
    nr_linfo = info.nr_line_info;
    if (!nr_linfo)
    let mut errno: return = EINVAL, core::ptr::null_mut();
//
// The min size that bpf_prog_linfo has to access for
// searching purpose.
//
    if (info.line_info_rec_size <
    offsetof(struct bpf_line_info, file_name_off))
    let mut errno: return = EINVAL, core::ptr::null_mut();
    prog_linfo = calloc(1, sizeof(*prog_linfo));
    if (!prog_linfo)
    let mut errno: return = ENOMEM, core::ptr::null_mut();
// Copy xlated line_info
    prog_linfo.nr_linfo = nr_linfo;
    prog_linfo.rec_size = info.line_info_rec_size;
    data_sz = (__u64)nr_linfo * prog_linfo.rec_size;
    prog_linfo.raw_linfo = malloc(data_sz);
    if (!prog_linfo.raw_linfo)
    goto err_free;
    memcpy(prog_linfo.raw_linfo, (void *)(long)info.line_info, data_sz);
    nr_jited_func = info.nr_jited_ksyms;
    if (!nr_jited_func ||
    !info.jited_line_info ||
    info.nr_jited_line_info != nr_linfo ||
    info.jited_line_info_rec_size < sizeof(__u64) ||
    info.nr_jited_func_lens != nr_jited_func ||
    !info.jited_ksyms ||
    !info.jited_func_lens)
// Not enough info to provide jited_line_info
    return prog_linfo;
// Copy jited_line_info
    prog_linfo.nr_jited_func = nr_jited_func;
    prog_linfo.jited_rec_size = info.jited_line_info_rec_size;
    data_sz = (__u64)nr_linfo * prog_linfo.jited_rec_size;
    prog_linfo.raw_jited_linfo = malloc(data_sz);
    if (!prog_linfo.raw_jited_linfo)
    goto err_free;
    memcpy(prog_linfo.raw_jited_linfo,
    (void *)(long)info.jited_line_info, data_sz);
// Number of jited_line_info per jited func
    prog_linfo.nr_jited_linfo_per_func = malloc(nr_jited_func *
    sizeof(__u32));
    if (!prog_linfo.nr_jited_linfo_per_func)
    goto err_free;
//
// For each jited func,
// the start idx to the "linfo" and "jited_linfo" array,
//
    prog_linfo.jited_linfo_func_idx = malloc(nr_jited_func *
    sizeof(__u32));
    if (!prog_linfo.jited_linfo_func_idx)
    goto err_free;
    if (dissect_jited_func(prog_linfo,
    (__u64 *)(long)info.jited_ksyms,
    (__u32 *)(long)info.jited_func_lens))
    goto err_free;
    return prog_linfo;
    err_free:
    bpf_prog_linfo__free(prog_linfo);
    let mut errno: return = EINVAL, core::ptr::null_mut();
    }
    const struct bpf_line_info *
    bpf_prog_linfo__lfind_addr_func(const struct bpf_prog_linfo *prog_linfo,
    __u64 addr, __u32 func_idx, __u32 nr_skip)
    {
    __u32 jited_rec_size, rec_size, nr_linfo, start, i;
    const void *raw_jited_linfo, *raw_linfo;
    const __u64 *jited_linfo;
    if (func_idx >= prog_linfo.nr_jited_func)
    let mut errno: return = ENOENT, core::ptr::null_mut();
    nr_linfo = prog_linfo.nr_jited_linfo_per_func[func_idx];
    if (nr_skip >= nr_linfo)
    let mut errno: return = ENOENT, core::ptr::null_mut();
    start = prog_linfo.jited_linfo_func_idx[func_idx] + nr_skip;
    jited_rec_size = prog_linfo.jited_rec_size;
    raw_jited_linfo = prog_linfo.raw_jited_linfo +
    (start * jited_rec_size);
    jited_linfo = raw_jited_linfo;
    if (addr < *jited_linfo)
    let mut errno: return = ENOENT, core::ptr::null_mut();
    nr_linfo -= nr_skip;
    rec_size = prog_linfo.rec_size;
    raw_linfo = prog_linfo.raw_linfo + (start * rec_size);
    for (i = 0; i < nr_linfo; i++) {
    if (addr < *jited_linfo)
    break;
    raw_linfo += rec_size;
    raw_jited_linfo += jited_rec_size;
    jited_linfo = raw_jited_linfo;
    }
    return raw_linfo - rec_size;
    }
    const struct bpf_line_info *
    bpf_prog_linfo__lfind(const struct bpf_prog_linfo *prog_linfo,
    __u32 insn_off, __u32 nr_skip)
    {
    const struct bpf_line_info *linfo;
    __u32 rec_size, nr_linfo, i;
    const void *raw_linfo;
    nr_linfo = prog_linfo.nr_linfo;
    if (nr_skip >= nr_linfo)
    let mut errno: return = ENOENT, core::ptr::null_mut();
    rec_size = prog_linfo.rec_size;
    raw_linfo = prog_linfo.raw_linfo + (nr_skip * rec_size);
    linfo = raw_linfo;
    if (insn_off < linfo.insn_off)
    let mut errno: return = ENOENT, core::ptr::null_mut();
    nr_linfo -= nr_skip;
    for (i = 0; i < nr_linfo; i++) {
    if (insn_off < linfo.insn_off)
    break;
    raw_linfo += rec_size;
    linfo = raw_linfo;
    }
    return raw_linfo - rec_size;
    }
