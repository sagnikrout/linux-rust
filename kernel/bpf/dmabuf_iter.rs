//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/dmabuf_iter.c
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
// Copyright (c) 2025 Google LLC

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmabuf_iter_priv {
//
// If this pointer is non-NULL, the buffer's refcount is elevated to
// prevent destruction between stop/start. If reading is not resumed and
// start is never called again, then dmabuf_iter_seq_fini drops the
// reference when the iterator is released.
//
    pub dmabuf: *mut dma_buf,
}

    static void *dmabuf_iter_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct dmabuf_iter_priv *p = seq.private;
    if (*pos) {
    struct dma_buf *dmabuf = p.dmabuf;
    if (!dmabuf)
    return core::ptr::null_mut();
//
// Always resume from where we stopped, regardless of the value
// of pos.
//
    p.dmabuf = core::ptr::null_mut();
    return dmabuf;
    }
    return dma_buf_iter_begin();
    }
    static void *dmabuf_iter_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct dma_buf *dmabuf = v;
    ++*pos;
    return dma_buf_iter_next(dmabuf);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__dmabuf {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub dmabuf): *mut *mut __bpf_md_ptr(struct dma_buf ,,
}

#[no_mangle]
unsafe extern "C" fn __dmabuf_seq_show(seq: *mut seq_file, v: *mut c_void, in_stop: bool) -> c_int {
    static int __dmabuf_seq_show(struct seq_file *seq, void *v, bool in_stop)
    {
    struct bpf_iter_meta meta = {
    .seq = seq,
    };
    struct bpf_iter__dmabuf ctx = {
    .meta = &meta,
    .dmabuf = v,
    };
    struct bpf_prog *prog = bpf_iter_get_info(&meta, in_stop);
    if (prog)
    return bpf_iter_run_prog(prog, &ctx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmabuf_iter_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int dmabuf_iter_seq_show(struct seq_file *seq, void *v)
    {
    return __dmabuf_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn dmabuf_iter_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void dmabuf_iter_seq_stop(struct seq_file *seq, void *v)
    {
    struct dma_buf *dmabuf = v;
    if (dmabuf) {
    struct dmabuf_iter_priv *p = seq.private;
    p.dmabuf = dmabuf;
    }
    }
    static const struct seq_operations dmabuf_iter_seq_ops = {
    .start	= dmabuf_iter_seq_start,
    .next	= dmabuf_iter_seq_next,
    .stop	= dmabuf_iter_seq_stop,
    .show	= dmabuf_iter_seq_show,
    };
    static void bpf_iter_dmabuf_show_fdinfo(const struct bpf_iter_aux_info *aux,
    struct seq_file *seq)
    {
    seq_puts(seq, "dmabuf iter\n");
    }
#[no_mangle]
unsafe extern "C" fn dmabuf_iter_seq_init(priv: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    static int dmabuf_iter_seq_init(void *priv, struct bpf_iter_aux_info *aux)
    {
    struct dmabuf_iter_priv *p = (struct dmabuf_iter_priv *)priv;
    p.dmabuf = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmabuf_iter_seq_fini(priv: *mut c_void) {
    static void dmabuf_iter_seq_fini(void *priv)
    {
    struct dmabuf_iter_priv *p = (struct dmabuf_iter_priv *)priv;
    if (p.dmabuf)
    dma_buf_put(p.dmabuf);
    }
    static const struct bpf_iter_seq_info dmabuf_iter_seq_info = {
    .seq_ops		= &dmabuf_iter_seq_ops,
    .init_seq_private	= dmabuf_iter_seq_init,
    .fini_seq_private	= dmabuf_iter_seq_fini,
    .seq_priv_size		= sizeof(struct dmabuf_iter_priv),
    };
    static struct bpf_iter_reg bpf_dmabuf_reg_info = {
    .target			= "dmabuf",
    .feature                = BPF_ITER_RESCHED,
    .show_fdinfo		= bpf_iter_dmabuf_show_fdinfo,
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__dmabuf, dmabuf),
    PTR_TO_BTF_ID_OR_NULL },
    },
    .seq_info		= &dmabuf_iter_seq_info,
    };
    DEFINE_BPF_ITER_FUNC(dmabuf, struct bpf_iter_meta *meta, struct dma_buf *dmabuf)
    BTF_ID_LIST_SINGLE(bpf_dmabuf_btf_id, struct, dma_buf)
#[no_mangle]
unsafe extern "C" fn dmabuf_iter_init() -> int __init {
    static int __init dmabuf_iter_init(void)
    {
    bpf_dmabuf_reg_info.ctx_arg_info[0].btf_id = bpf_dmabuf_btf_id[0];
    return bpf_iter_reg_target(&bpf_dmabuf_reg_info);
    }
    late_initcall(dmabuf_iter_init);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_dmabuf {
//
// opaque iterator state; having __u64 here allows to preserve correct
// alignment requirements in vmlinux.h, generated from BTF
//
    pub __opaque: [__u64; 1],
    pub __aligned(8): },
// Non-opaque version of bpf_iter_dmabuf
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_dmabuf_kern {
    pub dmabuf: *mut dma_buf,
    pub __aligned(8): },
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_dmabuf_new(it: *mut bpf_iter_dmabuf) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_iter_dmabuf_new(struct bpf_iter_dmabuf *it)
    {
    pub )it: *mut *mut bpf_iter_dmabuf_kern kit = (void,
    pub sizeof(*it)): *mut *mut BUILD_BUG_ON(sizeof(kit) >,
    pub __alignof__(*it)): *mut *mut BUILD_BUG_ON(__alignof__(kit) !=,
    pub NULL: kit->dmabuf =,
    pub 0: return,
    }
    __bpf_kfunc struct dma_buf *bpf_iter_dmabuf_next(struct bpf_iter_dmabuf *it)
    {
    pub )it: *mut *mut bpf_iter_dmabuf_kern kit = (void,
    if (kit.dmabuf)
    pub dma_buf_iter_next(kit->dmabuf): kit->dmabuf =,
    else
    pub dma_buf_iter_begin(): kit->dmabuf =,
    pub kit->dmabuf: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_dmabuf_destroy(it: *mut bpf_iter_dmabuf) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_dmabuf_destroy(struct bpf_iter_dmabuf *it)
    {
    pub )it: *mut *mut bpf_iter_dmabuf_kern kit = (void,
    if (kit.dmabuf)
    }
