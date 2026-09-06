//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_bpf.c
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
// Xtables module to match packets using a BPF filter.
// Copyright 2013 Google Inc.
// Written by Willem de Bruijn <willemb@google.com>
//

    MODULE_AUTHOR("Willem de Bruijn <willemb@google.com>");
    MODULE_DESCRIPTION("Xtables: BPF filter match");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_bpf");
    MODULE_ALIAS("ip6t_bpf");
    static int __bpf_mt_check_bytecode(struct sock_filter *insns, __u16 len,
    struct bpf_prog **ret)
    {
    struct sock_fprog_kern program;
    if (len > XT_BPF_MAX_NUM_INSTR)
    return -EINVAL;
    program.len = len;
    program.filter = insns;
    if (bpf_prog_create(ret, &program)) {
    pr_info_ratelimited("check failed: parse error\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_mt_check_fd(fd: c_int, ret: *mut bpf_prog) -> c_int {
    static int __bpf_mt_check_fd(int fd, struct bpf_prog **ret)
    {
    struct bpf_prog *prog;
    prog = bpf_prog_get_type(fd, BPF_PROG_TYPE_SOCKET_FILTER);
    if (IS_ERR(prog))
    return PTR_ERR(prog);
// ret = prog;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_mt_check_path(path: *const c_char, ret: *mut bpf_prog) -> c_int {
    static int __bpf_mt_check_path(const char *path, struct bpf_prog **ret)
    {
    if (strnlen(path, XT_BPF_PATH_MAX) == XT_BPF_PATH_MAX)
    return -EINVAL;
// ret = bpf_prog_get_type_path(path, BPF_PROG_TYPE_SOCKET_FILTER);
    return PTR_ERR_OR_ZERO(*ret);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int bpf_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_bpf_info *info = par.matchinfo;
    return __bpf_mt_check_bytecode(info.bpf_program,
    info.bpf_program_num_elem,
    &info.filter);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_check_v1(par: *const xt_mtchk_param) -> c_int {
    static int bpf_mt_check_v1(const struct xt_mtchk_param *par)
    {
    struct xt_bpf_info_v1 *info = par.matchinfo;
    if (info.mode == XT_BPF_MODE_BYTECODE)
    return __bpf_mt_check_bytecode(info.bpf_program,
    info.bpf_program_num_elem,
    &info.filter);
#[no_mangle]
pub unsafe extern "C" fn if(XT_BPF_MODE_FD_ELF: info->mode ==) -> else {
    else if (info.mode == XT_BPF_MODE_FD_ELF)
    return __bpf_mt_check_fd(info.fd, &info.filter);
#[no_mangle]
pub unsafe extern "C" fn if(XT_BPF_MODE_PATH_PINNED: info->mode ==) -> else {
    else if (info.mode == XT_BPF_MODE_PATH_PINNED)
    return __bpf_mt_check_path(info.path, &info.filter);
    else
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool bpf_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_bpf_info *info = par.matchinfo;
    return bpf_prog_run(info.filter, skb);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool bpf_mt_v1(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_bpf_info_v1 *info = par.matchinfo;
    return !!bpf_prog_run_save_cb(info.filter, (struct sk_buff *) skb);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_destroy(par: *const xt_mtdtor_param) {
    static void bpf_mt_destroy(const struct xt_mtdtor_param *par)
    {
    const struct xt_bpf_info *info = par.matchinfo;
    bpf_prog_destroy(info.filter);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_destroy_v1(par: *const xt_mtdtor_param) {
    static void bpf_mt_destroy_v1(const struct xt_mtdtor_param *par)
    {
    const struct xt_bpf_info_v1 *info = par.matchinfo;
    bpf_prog_destroy(info.filter);
    }
    static struct xt_match bpf_mt_reg[] __read_mostly = {
    {
    .name		= "bpf",
    .revision	= 0,
    .family		= NFPROTO_UNSPEC,
    .checkentry	= bpf_mt_check,
    .match		= bpf_mt,
    .destroy	= bpf_mt_destroy,
    .matchsize	= sizeof(struct xt_bpf_info),
    .usersize	= offsetof(struct xt_bpf_info, filter),
    .me		= THIS_MODULE,
    },
    {
    .name		= "bpf",
    .revision	= 1,
    .family		= NFPROTO_UNSPEC,
    .checkentry	= bpf_mt_check_v1,
    .match		= bpf_mt_v1,
    .destroy	= bpf_mt_destroy_v1,
    .matchsize	= sizeof(struct xt_bpf_info_v1),
    .usersize	= offsetof(struct xt_bpf_info_v1, filter),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn bpf_mt_init() -> int __init {
    static int __init bpf_mt_init(void)
    {
    return xt_register_matches(bpf_mt_reg, ARRAY_SIZE(bpf_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn bpf_mt_exit() -> void __exit {
    static void __exit bpf_mt_exit(void)
    {
    xt_unregister_matches(bpf_mt_reg, ARRAY_SIZE(bpf_mt_reg));
    }
    module_init(bpf_mt_init);
    module_exit(bpf_mt_exit);
