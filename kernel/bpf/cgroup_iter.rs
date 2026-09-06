//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cgroup_iter.c
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
// Copyright (c) 2022 Google

// cgroup_iter provides five modes of traversal to the cgroup hierarchy.
//
// 1. Walk the descendants of a cgroup in pre-order.
// 2. Walk the descendants of a cgroup in post-order.
// 3. Walk the ancestors of a cgroup.
// 4. Show the given cgroup only.
// 5. Walk the children of a given parent cgroup.
//
// For walking descendants, cgroup_iter can walk in either pre-order or
// post-order. For walking ancestors, the iter walks up from a cgroup to
// the root.
//
// The iter program can terminate the walk early by returning 1. Walk
// continues if prog returns 0.
//
// The prog can check (seq->num == 0) to determine whether this is
// the first element. The prog may also be passed a NULL cgroup,
// which means the walk has completed and the prog has a chance to
// do post-processing, such as outputting an epilogue.
//
// Note: the iter_prog is called with cgroup_mutex held.
//
// Currently only one session is supported, which means, depending on the
// volume of data bpf program intends to send to user space, the number
// of cgroups that can be walked is limited. For example, given the current
// buffer size is 8 * PAGE_SIZE, if the program sends 64B data for each
// cgroup, assuming PAGE_SIZE is 4kb, the total number of cgroups that can
// be walked is 512. This is a limitation of cgroup_iter. If the output data
// is larger than the kernel buffer size, after all data in the kernel buffer
// is consumed by user space, the subsequent read() syscall will signal
// EOPNOTSUPP. In order to work around, the user may have to update their
// program to reduce the volume of data sent to output. For example, skip
// some uninteresting cgroups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__cgroup {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub cgroup): *mut *mut __bpf_md_ptr(struct cgroup ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_iter_priv {
    pub start_css: *mut cgroup_subsys_state,
    pub visited_all: bool,
    pub terminate: bool,
    pub order: c_int,
}

    static void *cgroup_iter_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct cgroup_iter_priv *p = seq.private;
    cgroup_lock();
// cgroup_iter doesn't support read across multiple sessions.
    if (*pos > 0) {
    if (p.visited_all)
    return core::ptr::null_mut();
// Haven't visited all, but because cgroup_mutex has dropped,
// return -EOPNOTSUPP to indicate incomplete iteration.
//
    return ERR_PTR(-EOPNOTSUPP);
    }
    ++*pos;
    p.terminate = false;
    p.visited_all = false;
    if (p.order == BPF_CGROUP_ITER_DESCENDANTS_PRE)
    return css_next_descendant_pre(core::ptr::null_mut(), p.start_css);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_DESCENDANTS_POST: p->order ==) -> else {
    else if (p.order == BPF_CGROUP_ITER_DESCENDANTS_POST)
    return css_next_descendant_post(core::ptr::null_mut(), p.start_css);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_CHILDREN: p->order ==) -> else {
    else if (p.order == BPF_CGROUP_ITER_CHILDREN)
    return css_next_child(core::ptr::null_mut(), p.start_css);
    else /* BPF_CGROUP_ITER_SELF_ONLY and BPF_CGROUP_ITER_ANCESTORS_UP */
    return p.start_css;
    }
    static int __cgroup_iter_seq_show(struct seq_file *seq,
    struct cgroup_subsys_state *css, int in_stop);
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void cgroup_iter_seq_stop(struct seq_file *seq, void *v)
    {
    struct cgroup_iter_priv *p = seq.private;
    cgroup_unlock();
// pass NULL to the prog for post-processing
    if (!v) {
    __cgroup_iter_seq_show(seq, core::ptr::null_mut(), true);
    p.visited_all = true;
    }
    }
    static void *cgroup_iter_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct cgroup_subsys_state *curr = (struct cgroup_subsys_state *)v;
    struct cgroup_iter_priv *p = seq.private;
    ++*pos;
    if (p.terminate)
    return core::ptr::null_mut();
    if (p.order == BPF_CGROUP_ITER_DESCENDANTS_PRE)
    return css_next_descendant_pre(curr, p.start_css);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_DESCENDANTS_POST: p->order ==) -> else {
    else if (p.order == BPF_CGROUP_ITER_DESCENDANTS_POST)
    return css_next_descendant_post(curr, p.start_css);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_ANCESTORS_UP: p->order ==) -> else {
    else if (p.order == BPF_CGROUP_ITER_ANCESTORS_UP)
    return curr.parent;
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_CHILDREN: p->order ==) -> else {
    else if (p.order == BPF_CGROUP_ITER_CHILDREN)
    return css_next_child(curr, p.start_css);
    else  /* BPF_CGROUP_ITER_SELF_ONLY */
    return core::ptr::null_mut();
    }
    static int __cgroup_iter_seq_show(struct seq_file *seq,
    struct cgroup_subsys_state *css, int in_stop)
    {
    struct cgroup_iter_priv *p = seq.private;
    struct bpf_iter__cgroup ctx;
    struct bpf_iter_meta meta;
    struct bpf_prog *prog;
    let mut ret: c_int = 0;
// cgroup is dead, skip this element
    if (css && cgroup_is_dead(css.cgroup))
    return 0;
    ctx.meta = &meta;
    ctx.cgroup = css ? css.cgroup : core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (prog)
    ret = bpf_iter_run_prog(prog, &ctx);
// if prog returns > 0, terminate after this element.
    if (ret != 0)
    p.terminate = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int cgroup_iter_seq_show(struct seq_file *seq, void *v)
    {
    return __cgroup_iter_seq_show(seq, (struct cgroup_subsys_state *)v,
    false);
    }
    static const struct seq_operations cgroup_iter_seq_ops = {
    .start  = cgroup_iter_seq_start,
    .next   = cgroup_iter_seq_next,
    .stop   = cgroup_iter_seq_stop,
    .show   = cgroup_iter_seq_show,
    };
    BTF_ID_LIST_GLOBAL_SINGLE(bpf_cgroup_btf_id, struct, cgroup)
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_init(priv: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    static int cgroup_iter_seq_init(void *priv, struct bpf_iter_aux_info *aux)
    {
    struct cgroup_iter_priv *p = (struct cgroup_iter_priv *)priv;
    struct cgroup *cgrp = aux.cgroup.start;
// bpf_iter_attach_cgroup() has already acquired an extra reference
// for the start cgroup, but the reference may be released after
// cgroup_iter_seq_init(), so acquire another reference for the
// start cgroup.
//
    p.start_css = &cgrp.self;
    css_get(p.start_css);
    p.terminate = false;
    p.visited_all = false;
    p.order = aux.cgroup.order;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_fini(priv: *mut c_void) {
    static void cgroup_iter_seq_fini(void *priv)
    {
    struct cgroup_iter_priv *p = (struct cgroup_iter_priv *)priv;
    css_put(p.start_css);
    }
    static const struct bpf_iter_seq_info cgroup_iter_seq_info = {
    .seq_ops		= &cgroup_iter_seq_ops,
    .init_seq_private	= cgroup_iter_seq_init,
    .fini_seq_private	= cgroup_iter_seq_fini,
    .seq_priv_size		= sizeof(struct cgroup_iter_priv),
    };
    static int bpf_iter_attach_cgroup(struct bpf_prog *prog,
    union bpf_iter_link_info *linfo,
    struct bpf_iter_aux_info *aux)
    {
    let mut fd: c_int = linfo.cgroup.cgroup_fd;
    let mut id: u64 = linfo.cgroup.cgroup_id;
    let mut order: c_int = linfo.cgroup.order;
    struct cgroup *cgrp;
    switch (order) {
    case BPF_CGROUP_ITER_DESCENDANTS_PRE:
    case BPF_CGROUP_ITER_DESCENDANTS_POST:
    case BPF_CGROUP_ITER_ANCESTORS_UP:
    case BPF_CGROUP_ITER_SELF_ONLY:
    case BPF_CGROUP_ITER_CHILDREN:
    break;
    default:
    return -EINVAL;
    }
    if (fd && id)
    return -EINVAL;
    if (fd)
    cgrp = cgroup_v1v2_get_from_fd(fd);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: id) -> else {
    else if (id)
    cgrp = cgroup_get_from_id(id);
    else /* walk the entire hierarchy by default. */
    cgrp = cgroup_get_from_path("/");
    if (IS_ERR(cgrp))
    return PTR_ERR(cgrp);
    aux.cgroup.start = cgrp;
    aux.cgroup.order = order;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_detach_cgroup(aux: *mut bpf_iter_aux_info) {
    static void bpf_iter_detach_cgroup(struct bpf_iter_aux_info *aux)
    {
    cgroup_put(aux.cgroup.start);
    }
    static void bpf_iter_cgroup_show_fdinfo(const struct bpf_iter_aux_info *aux,
    struct seq_file *seq)
    {
    char *buf;
    buf = kzalloc(PATH_MAX, GFP_KERNEL);
    if (!buf) {
    seq_puts(seq, "cgroup_path:\t<unknown>\n");
    goto show_order;
    }
// If cgroup_path_ns() fails, buf will be an empty string, cgroup_path
// will print nothing.
//
// Path is in the calling process's cgroup namespace.
//
    cgroup_path_ns(aux.cgroup.start, buf, PATH_MAX,
    current.nsproxy.cgroup_ns);
    seq_printf(seq, "cgroup_path:\t%s\n", buf);
    kfree(buf);
    show_order:
    if (aux.cgroup.order == BPF_CGROUP_ITER_DESCENDANTS_PRE)
    seq_puts(seq, "order: descendants_pre\n");
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_DESCENDANTS_POST: aux->cgroup.order ==) -> else {
    else if (aux.cgroup.order == BPF_CGROUP_ITER_DESCENDANTS_POST)
    seq_puts(seq, "order: descendants_post\n");
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_ANCESTORS_UP: aux->cgroup.order ==) -> else {
    else if (aux.cgroup.order == BPF_CGROUP_ITER_ANCESTORS_UP)
    seq_puts(seq, "order: ancestors_up\n");
#[no_mangle]
pub unsafe extern "C" fn if(BPF_CGROUP_ITER_CHILDREN: aux->cgroup.order ==) -> else {
    else if (aux.cgroup.order == BPF_CGROUP_ITER_CHILDREN)
    seq_puts(seq, "order: children\n");
    else /* BPF_CGROUP_ITER_SELF_ONLY */
    seq_puts(seq, "order: self_only\n");
    }
    static int bpf_iter_cgroup_fill_link_info(const struct bpf_iter_aux_info *aux,
    struct bpf_link_info *info)
    {
    info.iter.cgroup.order = aux.cgroup.order;
    info.iter.cgroup.cgroup_id = cgroup_id(aux.cgroup.start);
    return 0;
    }
    DEFINE_BPF_ITER_FUNC(cgroup, struct bpf_iter_meta *meta,
    struct cgroup *cgroup)
    static struct bpf_iter_reg bpf_cgroup_reg_info = {
    .target			= "cgroup",
    .feature		= BPF_ITER_RESCHED,
    .attach_target		= bpf_iter_attach_cgroup,
    .detach_target		= bpf_iter_detach_cgroup,
    .show_fdinfo		= bpf_iter_cgroup_show_fdinfo,
    .fill_link_info		= bpf_iter_cgroup_fill_link_info,
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__cgroup, cgroup),
    PTR_TO_BTF_ID_OR_NULL | PTR_TRUSTED },
    },
    .seq_info		= &cgroup_iter_seq_info,
    };
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_iter_init() -> int __init {
    static int __init bpf_cgroup_iter_init(void)
    {
    bpf_cgroup_reg_info.ctx_arg_info[0].btf_id = bpf_cgroup_btf_id[0];
    return bpf_iter_reg_target(&bpf_cgroup_reg_info);
    }
    late_initcall(bpf_cgroup_iter_init);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css {
    pub __opaque: [__u64; 3],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css_kern {
    pub start: *mut cgroup_subsys_state,
    pub pos: *mut cgroup_subsys_state,
    pub flags: c_uint,
    pub __attribute__((aligned(8))): },
    __bpf_kfunc int bpf_iter_css_new(struct bpf_iter_css *it,
    struct cgroup_subsys_state *start, unsigned int flags)
    {
    pub )it: *mut *mut bpf_iter_css_kern kit = (void,
    pub bpf_iter_css)): BUILD_BUG_ON(sizeof(struct bpf_iter_css_kern) > sizeof(struct,
    pub bpf_iter_css)): BUILD_BUG_ON(__alignof__(struct bpf_iter_css_kern) != __alignof__(struct,
    pub NULL: kit->start =,
    switch (flags) {
    case BPF_CGROUP_ITER_DESCENDANTS_PRE:
    case BPF_CGROUP_ITER_DESCENDANTS_POST:
    case BPF_CGROUP_ITER_ANCESTORS_UP:
    case BPF_CGROUP_ITER_CHILDREN:
    default:
    pub -EINVAL: return,
    }
    pub start: kit->start =,
    pub NULL: kit->pos =,
    pub flags: kit->flags =,
    pub 0: return,
    }
    __bpf_kfunc struct cgroup_subsys_state *bpf_iter_css_next(struct bpf_iter_css *it)
    {
    pub )it: *mut *mut bpf_iter_css_kern kit = (void,
    if (!kit.start)
    pub NULL: return,
    switch (kit.flags) {
    case BPF_CGROUP_ITER_DESCENDANTS_PRE:
    pub kit->start): kit->pos = css_next_descendant_pre(kit->pos,,
    case BPF_CGROUP_ITER_DESCENDANTS_POST:
    pub kit->start): kit->pos = css_next_descendant_post(kit->pos,,
    case BPF_CGROUP_ITER_CHILDREN:
    pub kit->start): kit->pos = css_next_child(kit->pos,,
    case BPF_CGROUP_ITER_ANCESTORS_UP:
    pub kit->start: kit->pos = kit->pos ? kit->pos->parent :,
    }
    pub kit->pos: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_css_destroy(it: *mut bpf_iter_css) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_css_destroy(struct bpf_iter_css *it)
    {
    }
