//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_lsm.c
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
//
// Copyright (C) 2020 Google LLC.
//

// For every LSM hook that allows attachment of BPF programs, declare a nop
// function where a BPF program can be attached. Notably, we qualify each with
// weak linkage such that strong overrides can be implemented if need be.
//

    __weak noinline RET bpf_lsm_##NAME(__VA_ARGS__)	\
    {						\
    return DEFAULT;				\
    }

    BTF_SET_START(bpf_lsm_hooks)

    BTF_SET_END(bpf_lsm_hooks)
    BTF_SET_START(bpf_lsm_disabled_hooks)
    BTF_ID(func, bpf_lsm_vm_enough_memory)
    BTF_ID(func, bpf_lsm_inode_need_killpriv)
    BTF_ID(func, bpf_lsm_inode_getsecurity)
    BTF_ID(func, bpf_lsm_inode_listsecurity)
    BTF_ID(func, bpf_lsm_inode_copy_up_xattr)
    BTF_ID(func, bpf_lsm_getselfattr)
    BTF_ID(func, bpf_lsm_getprocattr)
    BTF_ID(func, bpf_lsm_setprocattr)

    BTF_ID(func, bpf_lsm_key_getsecurity)

    BTF_ID(func, bpf_lsm_audit_rule_match)

    BTF_ID(func, bpf_lsm_xfrm_decode_session)

    BTF_ID(func, bpf_lsm_ismaclabel)
    BTF_ID(func, bpf_lsm_file_alloc_security)
    BTF_SET_END(bpf_lsm_disabled_hooks)
// List of LSM hooks that should operate on 'current' cgroup regardless
// of function signature.
//
    BTF_SET_START(bpf_lsm_current_hooks)
// operate on freshly allocated sk without any cgroup association

    BTF_ID(func, bpf_lsm_sk_alloc_security)
    BTF_ID(func, bpf_lsm_sk_free_security)

    BTF_SET_END(bpf_lsm_current_hooks)
// List of LSM hooks that trigger while the socket is properly locked.
//
    BTF_SET_START(bpf_lsm_locked_sockopt_hooks)

    BTF_ID(func, bpf_lsm_sock_graft)
    BTF_ID(func, bpf_lsm_inet_csk_clone)
    BTF_ID(func, bpf_lsm_inet_conn_established)

    BTF_SET_END(bpf_lsm_locked_sockopt_hooks)
// List of LSM hooks that trigger while the socket is _not_ locked,
// but it's ok to call bpf_{g,s}etsockopt because the socket is still
// in the early init phase.
//
    BTF_SET_START(bpf_lsm_unlocked_sockopt_hooks)

    BTF_ID(func, bpf_lsm_socket_post_create)
    BTF_ID(func, bpf_lsm_socket_socketpair)

    BTF_SET_END(bpf_lsm_unlocked_sockopt_hooks)

    void bpf_lsm_find_cgroup_shim(const struct bpf_prog *prog,
    bpf_func_t *bpf_func)
    {
    const struct btf_param *args __maybe_unused;
    if (btf_type_vlen(prog.aux.attach_func_proto) < 1 ||
    btf_id_set_contains(&bpf_lsm_current_hooks,
    prog.aux.attach_btf_id)) {
// bpf_func = __cgroup_bpf_run_lsm_current;
    return;
    }

    args = btf_params(prog.aux.attach_func_proto);
    if (args[0].type == btf_sock_ids[BTF_SOCK_TYPE_SOCKET])
// bpf_func = __cgroup_bpf_run_lsm_socket;
#[no_mangle]
pub unsafe extern "C" fn if(btf_sock_ids[BTF_SOCK_TYPE_SOCK]: args[0].type ==) -> else {
    else if (args[0].type == btf_sock_ids[BTF_SOCK_TYPE_SOCK])
// bpf_func = __cgroup_bpf_run_lsm_sock;
    else

// bpf_func = __cgroup_bpf_run_lsm_current;
    }

    int bpf_lsm_verify_prog(struct bpf_verifier_log *vlog,
    const struct bpf_prog *prog)
    {
    let mut btf_id: u32 = prog.aux.attach_btf_id;
    const char *func_name = prog.aux.attach_func_name;
    if (!prog.gpl_compatible) {
    bpf_log(vlog,
    "LSM programs must have a GPL compatible license\n");
    return -EINVAL;
    }
    if (btf_id_set_contains(&bpf_lsm_disabled_hooks, btf_id)) {
    bpf_log(vlog, "attach_btf_id %u points to disabled hook %s\n",
    btf_id, func_name);
    return -EINVAL;
    }
    if (!btf_id_set_contains(&bpf_lsm_hooks, btf_id)) {
    bpf_log(vlog, "attach_btf_id %u points to wrong type name %s\n",
    btf_id, func_name);
    return -EINVAL;
    }
    return 0;
    }
// Mask for all the currently supported BPRM option flags

    BPF_CALL_2(bpf_bprm_opts_set, struct linux_binprm *, bprm, u64, flags)
    {
    if (flags & ~BPF_F_BRPM_OPTS_MASK)
    return -EINVAL;
    bprm.secureexec = (flags & BPF_F_BPRM_SECUREEXEC);
    return 0;
    }
    BTF_ID_LIST_SINGLE(bpf_bprm_opts_set_btf_ids, struct, linux_binprm)
    static const struct bpf_func_proto bpf_bprm_opts_set_proto = {
    .func		= bpf_bprm_opts_set,
    .gpl_only	= false,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &bpf_bprm_opts_set_btf_ids[0],
    .arg2_type	= ARG_ANYTHING,
    };
    BPF_CALL_3(bpf_ima_inode_hash, struct inode *, inode, void *, dst, u32, size)
    {
    return ima_inode_hash(inode, dst, size);
    }
#[no_mangle]
unsafe extern "C" fn bpf_ima_inode_hash_allowed(prog: *const bpf_prog) -> bool {
    static bool bpf_ima_inode_hash_allowed(const struct bpf_prog *prog)
    {
    return bpf_lsm_is_sleepable_hook(prog.aux.attach_btf_id);
    }
    BTF_ID_LIST_SINGLE(bpf_ima_inode_hash_btf_ids, struct, inode)
    static const struct bpf_func_proto bpf_ima_inode_hash_proto = {
    .func		= bpf_ima_inode_hash,
    .gpl_only	= false,
    .might_sleep	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &bpf_ima_inode_hash_btf_ids[0],
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE,
    .allowed	= bpf_ima_inode_hash_allowed,
    };
    BPF_CALL_3(bpf_ima_file_hash, struct file *, file, void *, dst, u32, size)
    {
    return ima_file_hash(file, dst, size);
    }
    BTF_ID_LIST_SINGLE(bpf_ima_file_hash_btf_ids, struct, file)
    static const struct bpf_func_proto bpf_ima_file_hash_proto = {
    .func		= bpf_ima_file_hash,
    .gpl_only	= false,
    .might_sleep	= true,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &bpf_ima_file_hash_btf_ids[0],
    .arg2_type	= ARG_PTR_TO_UNINIT_MEM,
    .arg3_type	= ARG_MEM_SIZE,
    .allowed	= bpf_ima_inode_hash_allowed,
    };
    BPF_CALL_1(bpf_get_attach_cookie, void *, ctx)
    {
    struct bpf_trace_run_ctx *run_ctx;
    run_ctx = container_of(current.bpf_ctx, struct bpf_trace_run_ctx, run_ctx);
    return run_ctx.bpf_cookie;
    }
    static const struct bpf_func_proto bpf_get_attach_cookie_proto = {
    .func		= bpf_get_attach_cookie,
    .gpl_only	= false,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_CTX,
    };
    static const struct bpf_func_proto *
    bpf_lsm_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
    const struct bpf_func_proto *func_proto;
    if (prog.expected_attach_type == BPF_LSM_CGROUP) {
    func_proto = cgroup_common_func_proto(func_id, prog);
    if (func_proto)
    return func_proto;
    }
    switch (func_id) {
    case BPF_FUNC_inode_storage_get:
    return &bpf_inode_storage_get_proto;
    case BPF_FUNC_inode_storage_delete:
    return &bpf_inode_storage_delete_proto;

    case BPF_FUNC_sk_storage_get:
    return &bpf_sk_storage_get_proto;
    case BPF_FUNC_sk_storage_delete:
    return &bpf_sk_storage_delete_proto;

    case BPF_FUNC_spin_lock:
    return &bpf_spin_lock_proto;
    case BPF_FUNC_spin_unlock:
    return &bpf_spin_unlock_proto;
    case BPF_FUNC_bprm_opts_set:
    return &bpf_bprm_opts_set_proto;
    case BPF_FUNC_ima_inode_hash:
    return &bpf_ima_inode_hash_proto;
    case BPF_FUNC_ima_file_hash:
    return &bpf_ima_file_hash_proto;
    case BPF_FUNC_get_attach_cookie:
    return bpf_prog_has_trampoline(prog) ? &bpf_get_attach_cookie_proto : core::ptr::null_mut();

    case BPF_FUNC_setsockopt:
    if (prog.expected_attach_type != BPF_LSM_CGROUP)
    return core::ptr::null_mut();
    if (btf_id_set_contains(&bpf_lsm_locked_sockopt_hooks,
    prog.aux.attach_btf_id))
    return &bpf_sk_setsockopt_proto;
    if (btf_id_set_contains(&bpf_lsm_unlocked_sockopt_hooks,
    prog.aux.attach_btf_id))
    return &bpf_unlocked_sk_setsockopt_proto;
    return core::ptr::null_mut();
    case BPF_FUNC_getsockopt:
    if (prog.expected_attach_type != BPF_LSM_CGROUP)
    return core::ptr::null_mut();
    if (btf_id_set_contains(&bpf_lsm_locked_sockopt_hooks,
    prog.aux.attach_btf_id))
    return &bpf_sk_getsockopt_proto;
    if (btf_id_set_contains(&bpf_lsm_unlocked_sockopt_hooks,
    prog.aux.attach_btf_id))
    return &bpf_unlocked_sk_getsockopt_proto;
    return core::ptr::null_mut();

    default:
    return tracing_prog_func_proto(func_id, prog);
    }
    }
// The set of hooks which are called without pagefaults disabled and are allowed
// to "sleep" and thus can be used for sleepable BPF programs.
//
    BTF_SET_START(sleepable_lsm_hooks)
    BTF_ID(func, bpf_lsm_bpf)
    BTF_ID(func, bpf_lsm_bpf_map)
    BTF_ID(func, bpf_lsm_bpf_map_create)
    BTF_ID(func, bpf_lsm_bpf_map_free)
    BTF_ID(func, bpf_lsm_bpf_prog)
    BTF_ID(func, bpf_lsm_bpf_prog_load)
    BTF_ID(func, bpf_lsm_bpf_token_create)
    BTF_ID(func, bpf_lsm_bpf_token_free)
    BTF_ID(func, bpf_lsm_bpf_token_cmd)
    BTF_ID(func, bpf_lsm_bpf_token_capable)
    BTF_ID(func, bpf_lsm_bprm_check_security)
    BTF_ID(func, bpf_lsm_bprm_committed_creds)
    BTF_ID(func, bpf_lsm_bprm_committing_creds)
    BTF_ID(func, bpf_lsm_bprm_creds_for_exec)
    BTF_ID(func, bpf_lsm_bprm_creds_from_file)
    BTF_ID(func, bpf_lsm_capget)
    BTF_ID(func, bpf_lsm_capset)
    BTF_ID(func, bpf_lsm_cred_prepare)
    BTF_ID(func, bpf_lsm_file_ioctl)
    BTF_ID(func, bpf_lsm_file_lock)
    BTF_ID(func, bpf_lsm_file_open)
    BTF_ID(func, bpf_lsm_file_post_open)
    BTF_ID(func, bpf_lsm_file_receive)
    BTF_ID(func, bpf_lsm_inode_create)
    BTF_ID(func, bpf_lsm_inode_free_security)
    BTF_ID(func, bpf_lsm_inode_getattr)
    BTF_ID(func, bpf_lsm_inode_getxattr)
    BTF_ID(func, bpf_lsm_inode_mknod)
    BTF_ID(func, bpf_lsm_inode_need_killpriv)
    BTF_ID(func, bpf_lsm_inode_post_setxattr)
    BTF_ID(func, bpf_lsm_inode_post_removexattr)
    BTF_ID(func, bpf_lsm_inode_readlink)
    BTF_ID(func, bpf_lsm_inode_removexattr)
    BTF_ID(func, bpf_lsm_inode_rename)
    BTF_ID(func, bpf_lsm_inode_rmdir)
    BTF_ID(func, bpf_lsm_inode_setattr)
    BTF_ID(func, bpf_lsm_inode_setxattr)
    BTF_ID(func, bpf_lsm_inode_symlink)
    BTF_ID(func, bpf_lsm_inode_unlink)
    BTF_ID(func, bpf_lsm_kernel_module_request)
    BTF_ID(func, bpf_lsm_kernel_read_file)
    BTF_ID(func, bpf_lsm_kernfs_init_security)

    BTF_ID(func, bpf_lsm_path_unlink)
    BTF_ID(func, bpf_lsm_path_mkdir)
    BTF_ID(func, bpf_lsm_path_rmdir)
    BTF_ID(func, bpf_lsm_path_truncate)
    BTF_ID(func, bpf_lsm_path_symlink)
    BTF_ID(func, bpf_lsm_path_link)
    BTF_ID(func, bpf_lsm_path_rename)
    BTF_ID(func, bpf_lsm_path_chmod)
    BTF_ID(func, bpf_lsm_path_chown)

    BTF_ID(func, bpf_lsm_mmap_file)
    BTF_ID(func, bpf_lsm_netlink_send)
    BTF_ID(func, bpf_lsm_path_notify)
    BTF_ID(func, bpf_lsm_release_secctx)
    BTF_ID(func, bpf_lsm_sb_alloc_security)
    BTF_ID(func, bpf_lsm_sb_eat_lsm_opts)
    BTF_ID(func, bpf_lsm_sb_kern_mount)
    BTF_ID(func, bpf_lsm_sb_mount)
    BTF_ID(func, bpf_lsm_sb_remount)
    BTF_ID(func, bpf_lsm_sb_set_mnt_opts)
    BTF_ID(func, bpf_lsm_sb_show_options)
    BTF_ID(func, bpf_lsm_sb_statfs)
    BTF_ID(func, bpf_lsm_sb_umount)
    BTF_ID(func, bpf_lsm_settime)

    BTF_ID(func, bpf_lsm_socket_accept)
    BTF_ID(func, bpf_lsm_socket_bind)
    BTF_ID(func, bpf_lsm_socket_connect)
    BTF_ID(func, bpf_lsm_socket_create)
    BTF_ID(func, bpf_lsm_socket_getpeername)
    BTF_ID(func, bpf_lsm_socket_getpeersec_dgram)
    BTF_ID(func, bpf_lsm_socket_getsockname)
    BTF_ID(func, bpf_lsm_socket_getsockopt)
    BTF_ID(func, bpf_lsm_socket_listen)
    BTF_ID(func, bpf_lsm_socket_post_create)
    BTF_ID(func, bpf_lsm_socket_recvmsg)
    BTF_ID(func, bpf_lsm_socket_sendmsg)
    BTF_ID(func, bpf_lsm_socket_shutdown)
    BTF_ID(func, bpf_lsm_socket_socketpair)

    BTF_ID(func, bpf_lsm_syslog)
    BTF_ID(func, bpf_lsm_task_alloc)
    BTF_ID(func, bpf_lsm_task_prctl)
    BTF_ID(func, bpf_lsm_task_setscheduler)
    BTF_ID(func, bpf_lsm_userns_create)
    BTF_ID(func, bpf_lsm_bdev_alloc_security)
    BTF_ID(func, bpf_lsm_bdev_setintegrity)
    BTF_SET_END(sleepable_lsm_hooks)
    BTF_SET_START(untrusted_lsm_hooks)
    BTF_ID(func, bpf_lsm_bpf_map_free)
    BTF_ID(func, bpf_lsm_bpf_prog_free)
    BTF_ID(func, bpf_lsm_file_alloc_security)
    BTF_ID(func, bpf_lsm_file_free_security)

    BTF_ID(func, bpf_lsm_sk_alloc_security)
    BTF_ID(func, bpf_lsm_sk_free_security)

    BTF_ID(func, bpf_lsm_task_free)
    BTF_ID(func, bpf_lsm_bdev_alloc_security)
    BTF_ID(func, bpf_lsm_bdev_free_security)
    BTF_SET_END(untrusted_lsm_hooks)
#[no_mangle]
pub unsafe extern "C" fn bpf_lsm_is_sleepable_hook(btf_id: u32) -> bool {
    bool bpf_lsm_is_sleepable_hook(u32 btf_id)
    {
    return btf_id_set_contains(&sleepable_lsm_hooks, btf_id);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_lsm_is_trusted(prog: *const bpf_prog) -> bool {
    bool bpf_lsm_is_trusted(const struct bpf_prog *prog)
    {
    return !btf_id_set_contains(&untrusted_lsm_hooks, prog.aux.attach_btf_id);
    }
    const struct bpf_prog_ops lsm_prog_ops = {
    };
    const struct bpf_verifier_ops lsm_verifier_ops = {
    .get_func_proto = bpf_lsm_func_proto,
    .is_valid_access = btf_ctx_access,
    };
// hooks return 0 or 1
    BTF_SET_START(bool_lsm_hooks)

    BTF_ID(func, bpf_lsm_xfrm_state_pol_flow_match)

    BTF_ID(func, bpf_lsm_audit_rule_known)

    BTF_ID(func, bpf_lsm_inode_xattr_skipcap)
    BTF_SET_END(bool_lsm_hooks)
// hooks returning void

    BTF_SET_START(void_lsm_hooks)

    BTF_SET_END(void_lsm_hooks)
#[no_mangle]
pub unsafe extern "C" fn bpf_lsm_hook_returns_errno(btf_id: u32) -> bool {
    bool bpf_lsm_hook_returns_errno(u32 btf_id)
    {
    if (btf_id_set_contains(&bool_lsm_hooks, btf_id))
    return false;
    if (btf_id_set_contains(&void_lsm_hooks, btf_id))
    return false;
    return true;
    }
    int bpf_lsm_get_retval_range(const struct bpf_prog *prog,
    struct bpf_retval_range *retval_range)
    {
// no return value range for void hooks
    if (!prog.aux.attach_func_proto.type)
    return -EINVAL;
    if (btf_id_set_contains(&bool_lsm_hooks, prog.aux.attach_btf_id)) {
    retval_range.minval = 0;
    retval_range.maxval = 1;
    } else {
// All other available LSM hooks, except task_prctl, return 0
// on success and negative error code on failure.
// To keep things simple, we only allow bpf progs to return 0
// or negative errno for task_prctl too.
//
    retval_range.minval = -MAX_ERRNO;
    retval_range.maxval = 0;
    }
    return 0;
    }
