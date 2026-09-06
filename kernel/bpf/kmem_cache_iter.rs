//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/kmem_cache_iter.c
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
// Copyright (c) 2024 Google

// open-coded version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_kmem_cache {
    pub __opaque: [__u64; 1],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_kmem_cache_kern {
    pub pos: *mut kmem_cache,
    pub __attribute__((aligned(8))): },

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_new(it: *mut bpf_iter_kmem_cache) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_iter_kmem_cache_new(struct bpf_iter_kmem_cache *it)
    {
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub sizeof(*it)): *mut *mut BUILD_BUG_ON(sizeof(kit) >,
    pub __alignof__(*it)): *mut *mut BUILD_BUG_ON(__alignof__(kit) !=,
    pub KMEM_CACHE_POS_START: kit->pos =,
    pub 0: return,
    }
    __bpf_kfunc struct kmem_cache *bpf_iter_kmem_cache_next(struct bpf_iter_kmem_cache *it)
    {
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub kit->pos: *mut *mut kmem_cache prev =,
    pub next: *mut kmem_cache,
    pub false: bool destroy =,
    if (!prev)
    pub NULL: return,
    if (list_empty(&slab_caches)) {
    pub NULL: return,
    }
    if (prev == KMEM_CACHE_POS_START)
    pub list): next = list_first_entry(&slab_caches, struct kmem_cache,,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: list_last_entry(&slab_caches, kmem_cache: struct, prev: list) ==) -> else {
    else if (list_last_entry(&slab_caches, struct kmem_cache, list) == prev)
    pub NULL: next =,
    else
    pub list): next = list_next_entry(prev,,
// boot_caches have negative refcount, don't touch them
    if (next && next.refcount > 0)
// Skip kmem_cache_destroy() for active entries
    if (prev && prev != KMEM_CACHE_POS_START) {
    if (prev.refcount > 1)
#[no_mangle]
pub unsafe extern "C" fn if(1: prev->refcount ==) -> else {
    else if (prev.refcount == 1)
    pub true: destroy =,
    }
    if (destroy)
    pub next: kit->pos =,
    pub next: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_destroy(it: *mut bpf_iter_kmem_cache) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_kmem_cache_destroy(struct bpf_iter_kmem_cache *it)
    {
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub kit->pos: *mut *mut kmem_cache s =,
    pub false: bool destroy =,
    if (s == core::ptr::null_mut() || s == KMEM_CACHE_POS_START)
// Skip kmem_cache_destroy() for active entries
    if (s.refcount > 1)
#[no_mangle]
pub unsafe extern "C" fn if(1: s->refcount ==) -> else {
    else if (s.refcount == 1)
    pub true: destroy =,
    if (destroy)
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__kmem_cache {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub s): *mut *mut __bpf_md_ptr(struct kmem_cache ,,
}

    union kmem_cache_iter_priv {
    struct bpf_iter_kmem_cache it;
    struct bpf_iter_kmem_cache_kern kit;
    };
    static void *kmem_cache_iter_seq_start(struct seq_file *seq, loff_t *pos)
    {
    let mut cnt: loff_t = 0;
    let mut found: bool = false;
    struct kmem_cache *s;
    union kmem_cache_iter_priv *p = seq.private;
    mutex_lock(&slab_mutex);
// Find an entry at the given position in the slab_caches list instead
// of keeping a reference (of the last visited entry, if any) out of
// slab_mutex. It might miss something if one is deleted in the middle
// while it releases the lock.  But it should be rare and there's not
// much we can do about it.
//
    list_for_each_entry(s, &slab_caches, list) {
    if (cnt == *pos) {
// Make sure this entry remains in the list by getting
// a new reference count.  Note that boot_cache entries
// have a negative refcount, so don't touch them.
//
    if (s.refcount > 0)
    s.refcount++;
    found = true;
    break;
    }
    cnt++;
    }
    mutex_unlock(&slab_mutex);
    if (!found)
    s = core::ptr::null_mut();
    p.kit.pos = s;
    return s;
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_iter_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void kmem_cache_iter_seq_stop(struct seq_file *seq, void *v)
    {
    struct bpf_iter_meta meta;
    struct bpf_iter__kmem_cache ctx = {
    .meta = &meta,
    .s = v,
    };
    union kmem_cache_iter_priv *p = seq.private;
    struct bpf_prog *prog;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, true);
    if (prog && !ctx.s)
    bpf_iter_run_prog(prog, &ctx);
    bpf_iter_kmem_cache_destroy(&p.it);
    }
    static void *kmem_cache_iter_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    union kmem_cache_iter_priv *p = seq.private;
    ++*pos;
    return bpf_iter_kmem_cache_next(&p.it);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_iter_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int kmem_cache_iter_seq_show(struct seq_file *seq, void *v)
    {
    struct bpf_iter_meta meta;
    struct bpf_iter__kmem_cache ctx = {
    .meta = &meta,
    .s = v,
    };
    struct bpf_prog *prog;
    let mut ret: c_int = 0;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, false);
    if (prog)
    ret = bpf_iter_run_prog(prog, &ctx);
    return ret;
    }
    static const struct seq_operations kmem_cache_iter_seq_ops = {
    .start  = kmem_cache_iter_seq_start,
    .next   = kmem_cache_iter_seq_next,
    .stop   = kmem_cache_iter_seq_stop,
    .show   = kmem_cache_iter_seq_show,
    };
    BTF_ID_LIST_GLOBAL_SINGLE(bpf_kmem_cache_btf_id, struct, kmem_cache)
    static const struct bpf_iter_seq_info kmem_cache_iter_seq_info = {
    .seq_ops		= &kmem_cache_iter_seq_ops,
    .seq_priv_size		= sizeof(union kmem_cache_iter_priv),
    };
    static void bpf_iter_kmem_cache_show_fdinfo(const struct bpf_iter_aux_info *aux,
    struct seq_file *seq)
    {
    seq_puts(seq, "kmem_cache iter\n");
    }
    DEFINE_BPF_ITER_FUNC(kmem_cache, struct bpf_iter_meta *meta,
    struct kmem_cache *s)
    static struct bpf_iter_reg bpf_kmem_cache_reg_info = {
    .target			= "kmem_cache",
    .feature		= BPF_ITER_RESCHED,
    .show_fdinfo		= bpf_iter_kmem_cache_show_fdinfo,
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__kmem_cache, s),
    PTR_TO_BTF_ID_OR_NULL | PTR_TRUSTED },
    },
    .seq_info		= &kmem_cache_iter_seq_info,
    };
#[no_mangle]
unsafe extern "C" fn bpf_kmem_cache_iter_init() -> int __init {
    static int __init bpf_kmem_cache_iter_init(void)
    {
    bpf_kmem_cache_reg_info.ctx_arg_info[0].btf_id = bpf_kmem_cache_btf_id[0];
    return bpf_iter_reg_target(&bpf_kmem_cache_reg_info);
    }
    late_initcall(bpf_kmem_cache_iter_init);
