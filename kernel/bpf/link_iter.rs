//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/link_iter.c
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
// Copyright (c) 2022 Red Hat, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_link_info {
    pub link_id: u32,
}

    static void *bpf_link_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct bpf_iter_seq_link_info *info = seq.private;
    struct bpf_link *link;
    link = bpf_link_get_curr_or_next(&info.link_id);
    if (!link)
    return core::ptr::null_mut();
    if (*pos == 0)
    ++*pos;
    return link;
    }
    static void *bpf_link_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bpf_iter_seq_link_info *info = seq.private;
    ++*pos;
    ++info.link_id;
    bpf_link_put((struct bpf_link *)v);
    return bpf_link_get_curr_or_next(&info.link_id);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__bpf_link {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub link): *mut *mut __bpf_md_ptr(struct bpf_link ,,
}

    DEFINE_BPF_ITER_FUNC(bpf_link, struct bpf_iter_meta *meta, struct bpf_link *link)
#[no_mangle]
unsafe extern "C" fn __bpf_link_seq_show(seq: *mut seq_file, v: *mut c_void, in_stop: bool) -> c_int {
    static int __bpf_link_seq_show(struct seq_file *seq, void *v, bool in_stop)
    {
    struct bpf_iter__bpf_link ctx;
    struct bpf_iter_meta meta;
    struct bpf_prog *prog;
    let mut ret: c_int = 0;
    ctx.meta = &meta;
    ctx.link = v;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (prog)
    ret = bpf_iter_run_prog(prog, &ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_link_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int bpf_link_seq_show(struct seq_file *seq, void *v)
    {
    return __bpf_link_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn bpf_link_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void bpf_link_seq_stop(struct seq_file *seq, void *v)
    {
    if (!v)
    (void)__bpf_link_seq_show(seq, v, true);
    else
    bpf_link_put((struct bpf_link *)v);
    }
    static const struct seq_operations bpf_link_seq_ops = {
    .start	= bpf_link_seq_start,
    .next	= bpf_link_seq_next,
    .stop	= bpf_link_seq_stop,
    .show	= bpf_link_seq_show,
    };
    BTF_ID_LIST_SINGLE(btf_bpf_link_id, struct, bpf_link)
    static const struct bpf_iter_seq_info bpf_link_seq_info = {
    .seq_ops		= &bpf_link_seq_ops,
    .init_seq_private	= core::ptr::null_mut(),
    .fini_seq_private	= core::ptr::null_mut(),
    .seq_priv_size		= sizeof(struct bpf_iter_seq_link_info),
    };
    static struct bpf_iter_reg bpf_link_reg_info = {
    .target			= "bpf_link",
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__bpf_link, link),
    PTR_TO_BTF_ID_OR_NULL },
    },
    .seq_info		= &bpf_link_seq_info,
    };
#[no_mangle]
unsafe extern "C" fn bpf_link_iter_init() -> int __init {
    static int __init bpf_link_iter_init(void)
    {
    bpf_link_reg_info.ctx_arg_info[0].btf_id = *btf_bpf_link_id;
    return bpf_iter_reg_target(&bpf_link_reg_info);
    }
    late_initcall(bpf_link_iter_init);
