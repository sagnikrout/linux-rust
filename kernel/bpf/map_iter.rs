//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/map_iter.c
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
// Copyright (c) 2020 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_map_info {
    pub map_id: u32,
}

    static void *bpf_map_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct bpf_iter_seq_map_info *info = seq.private;
    struct bpf_map *map;
    map = bpf_map_get_curr_or_next(&info.map_id);
    if (!map)
    return core::ptr::null_mut();
    if (*pos == 0)
    ++*pos;
    return map;
    }
    static void *bpf_map_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bpf_iter_seq_map_info *info = seq.private;
    ++*pos;
    ++info.map_id;
    bpf_map_put((struct bpf_map *)v);
    return bpf_map_get_curr_or_next(&info.map_id);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__bpf_map {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub map): *mut *mut __bpf_md_ptr(struct bpf_map ,,
}

    DEFINE_BPF_ITER_FUNC(bpf_map, struct bpf_iter_meta *meta, struct bpf_map *map)
#[no_mangle]
unsafe extern "C" fn __bpf_map_seq_show(seq: *mut seq_file, v: *mut c_void, in_stop: bool) -> c_int {
    static int __bpf_map_seq_show(struct seq_file *seq, void *v, bool in_stop)
    {
    struct bpf_iter__bpf_map ctx;
    struct bpf_iter_meta meta;
    struct bpf_prog *prog;
    let mut ret: c_int = 0;
    ctx.meta = &meta;
    ctx.map = v;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (prog)
    ret = bpf_iter_run_prog(prog, &ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_map_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int bpf_map_seq_show(struct seq_file *seq, void *v)
    {
    return __bpf_map_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn bpf_map_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void bpf_map_seq_stop(struct seq_file *seq, void *v)
    {
    if (!v)
    (void)__bpf_map_seq_show(seq, v, true);
    else
    bpf_map_put((struct bpf_map *)v);
    }
    static const struct seq_operations bpf_map_seq_ops = {
    .start	= bpf_map_seq_start,
    .next	= bpf_map_seq_next,
    .stop	= bpf_map_seq_stop,
    .show	= bpf_map_seq_show,
    };
    BTF_ID_LIST_GLOBAL_SINGLE(btf_bpf_map_id, struct, bpf_map)
    static const struct bpf_iter_seq_info bpf_map_seq_info = {
    .seq_ops		= &bpf_map_seq_ops,
    .init_seq_private	= core::ptr::null_mut(),
    .fini_seq_private	= core::ptr::null_mut(),
    .seq_priv_size		= sizeof(struct bpf_iter_seq_map_info),
    };
    static struct bpf_iter_reg bpf_map_reg_info = {
    .target			= "bpf_map",
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__bpf_map, map),
    PTR_TO_BTF_ID_OR_NULL | PTR_TRUSTED },
    },
    .seq_info		= &bpf_map_seq_info,
    };
    static int bpf_iter_attach_map(struct bpf_prog *prog,
    union bpf_iter_link_info *linfo,
    struct bpf_iter_aux_info *aux)
    {
    u32 key_acc_size, value_acc_size, key_size, value_size;
    struct bpf_map *map;
    let mut is_percpu: bool = false;
    let mut err: c_int = -EINVAL;
    if (!linfo.map.map_fd)
    return -EBADF;
    map = bpf_map_get_with_uref(linfo.map.map_fd);
    if (IS_ERR(map))
    return PTR_ERR(map);
    if (map.excl_prog_sha) {
    err = -EPERM;
    goto put_map;
    }
    if (map.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
    map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH ||
    map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY)
    is_percpu = true;
    else if (map.map_type != BPF_MAP_TYPE_HASH &&
    map.map_type != BPF_MAP_TYPE_LRU_HASH &&
    map.map_type != BPF_MAP_TYPE_ARRAY &&
    map.map_type != BPF_MAP_TYPE_RHASH)
    goto put_map;
    key_acc_size = prog.aux.max_rdonly_access;
    value_acc_size = prog.aux.max_rdwr_access;
    key_size = map.key_size;
    if (!is_percpu)
    value_size = map.value_size;
    else
    value_size = round_up(map.value_size, 8) * num_possible_cpus();
    if (key_acc_size > key_size || value_acc_size > value_size) {
    err = -EACCES;
    goto put_map;
    }
    aux.map = map;
    return 0;
    put_map:
    bpf_map_put_with_uref(map);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_detach_map(aux: *mut bpf_iter_aux_info) {
    static void bpf_iter_detach_map(struct bpf_iter_aux_info *aux)
    {
    bpf_map_put_with_uref(aux.map);
    }
    void bpf_iter_map_show_fdinfo(const struct bpf_iter_aux_info *aux,
    struct seq_file *seq)
    {
    seq_printf(seq, "map_id:\t%u\n", aux.map.id);
    }
    int bpf_iter_map_fill_link_info(const struct bpf_iter_aux_info *aux,
    struct bpf_link_info *info)
    {
    info.iter.map.map_id = aux.map.id;
    return 0;
    }
    DEFINE_BPF_ITER_FUNC(bpf_map_elem, struct bpf_iter_meta *meta,
    struct bpf_map *map, void *key, void *value)
    static const struct bpf_iter_reg bpf_map_elem_reg_info = {
    .target			= "bpf_map_elem",
    .attach_target		= bpf_iter_attach_map,
    .detach_target		= bpf_iter_detach_map,
    .show_fdinfo		= bpf_iter_map_show_fdinfo,
    .fill_link_info		= bpf_iter_map_fill_link_info,
    .ctx_arg_info_size	= 2,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__bpf_map_elem, key),
    PTR_TO_BUF | PTR_MAYBE_NULL | MEM_RDONLY },
    { offsetof(struct bpf_iter__bpf_map_elem, value),
    PTR_TO_BUF | PTR_MAYBE_NULL },
    },
    };
#[no_mangle]
unsafe extern "C" fn bpf_map_iter_init() -> int __init {
    static int __init bpf_map_iter_init(void)
    {
    int ret;
    bpf_map_reg_info.ctx_arg_info[0].btf_id = *btf_bpf_map_id;
    ret = bpf_iter_reg_target(&bpf_map_reg_info);
    if (ret)
    return ret;
    return bpf_iter_reg_target(&bpf_map_elem_reg_info);
    }
    late_initcall(bpf_map_iter_init);
    __bpf_kfunc_start_defs();
#[no_mangle]
pub unsafe extern "C" fn bpf_map_sum_elem_count(map: *const bpf_map) -> __bpf_kfunc s64 {
    __bpf_kfunc s64 bpf_map_sum_elem_count(const struct bpf_map *map)
    {
    s64 *pcount;
    let mut ret: i64 = 0;
    int cpu;
    if (!map || !map.elem_count)
    return 0;
    for_each_possible_cpu(cpu) {
    pcount = per_cpu_ptr(map.elem_count, cpu);
    ret += READ_ONCE(*pcount);
    }
    return ret;
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(bpf_map_iter_kfunc_ids)
    BTF_ID_FLAGS(func, bpf_map_sum_elem_count)
    BTF_KFUNCS_END(bpf_map_iter_kfunc_ids)
    static const struct btf_kfunc_id_set bpf_map_iter_kfunc_set = {
    .owner = THIS_MODULE,
    .set   = &bpf_map_iter_kfunc_ids,
    };
#[no_mangle]
unsafe extern "C" fn init_subsystem() -> c_int {
    static int init_subsystem(void)
    {
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC, &bpf_map_iter_kfunc_set);
    }
    late_initcall(init_subsystem);
