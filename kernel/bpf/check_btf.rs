//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/check_btf.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn check_abnormal_return(env: *mut bpf_verifier_env) -> c_int {
    let mut i = 0;
    while (i < env.subprog_cnt) {
    if (env.subprog_info[i].has_ld_abs) {
    verbose(env, "LD_ABS is not allowed in subprogs without BTF\n");
    return -EINVAL;
    }
    if (env.subprog_info[i].has_tail_call) {
    verbose(env, "tail_call is not allowed in subprogs without BTF\n");
    return -EINVAL;
    }
    }
    return 0;
    }
// The minimum supported BTF func info size
pub const MIN_BPF_FUNCINFO_SIZE: c_int = 8;
pub const MAX_FUNCINFO_REC_SIZE: c_int = 252;
#[no_mangle]
pub unsafe extern "C" fn prepare_btf_func(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
pub static mut krec_size: u32 = 0;
    let mut type = core::ptr::null_mut();
    let mut func_proto = core::ptr::null_mut();
    u32 i, nfuncs, urec_size, min_size;
pub static mut krecord: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
pub static mut prev_offset: u32 = 0;
    let mut urecord;
pub static mut ret: c_int = 0;
    nfuncs = attr.func_info_cnt;
    if (!nfuncs) {
    if (check_abnormal_return(env)) {
    return -EINVAL;
    }
    return 0;
    }
    urec_size = attr.func_info_rec_size;
    if (urec_size < MIN_BPF_FUNCINFO_SIZE ||
    urec_size > MAX_FUNCINFO_REC_SIZE ||
    urec_size % sizeof!(u32)) {
    verbose(env, "invalid func info rec size %u\n", urec_size);
    return -EINVAL;
    }
    prog = env.prog;
    btf = prog.aux.btf;
    urecord = make_bpfptr(attr.func_info, uattr.is_kernel);
    min_size = min_t(u32, krec_size, urec_size);
    krecord = kvcalloc(nfuncs, krec_size, GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!krecord) {
    return -ENOMEM;
    }
    while (i < nfuncs) {
    ret = bpf_check_uarg_tail_zero(urecord, krec_size, urec_size);
    if (ret) {
    if (ret == -E2BIG) {
    verbose(env, "nonzero tailing record in func info");
// set the size kernel expects so loader can zero
// out the rest of the record.
//
    if (copy_to_bpfptr_offset(uattr,
    offsetof(union bpf_attr, func_info_rec_size),
    &min_size, sizeof!(min_size))) {
    ret = -EFAULT;
    }
    }
// goto;
    }
    if (copy_from_bpfptr(&krecord[i], urecord, min_size)) {
    ret = -EFAULT;
// goto;
    }
// check insn_off
    ret = -EINVAL;
    if (i == 0) {
    if (krecord[i].insn_off) {
    verbose(env,
    "nonzero insn_off %u for the first func info record",
    krecord[i].insn_off);
// goto;
    }
    } else if (krecord[i].insn_off <= prev_offset) {
    verbose(env,
    "same or smaller insn offset (%u) than previous func info record (%u)",
    krecord[i].insn_off, prev_offset);
// goto;
    }
// check type_id
    type = btf_type_by_id(btf, krecord[i].type_id);
    if (!type || !btf_type_is_func(type)) {
    verbose(env, "invalid type id %d in func info",
    krecord[i].type_id);
// goto;
    }
    func_proto = btf_type_by_id(btf, type.type);
    if (unlikely(!func_proto || !btf_type_is_func_proto(func_proto))) {
// btf_func_check() already verified it during BTF load
// goto;
    }
    prev_offset = krecord[i].insn_off;
    bpfptr_add(&urecord, urec_size);
    }
    prog.aux.func_info = krecord;
    prog.aux.func_info_cnt = nfuncs;
    return 0;
// label;
    kvfree(krecord);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn check_btf_func(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
    let mut type = core::ptr::null_mut();
    let mut func_proto = core::ptr::null_mut();
    let mut ret_type = core::ptr::null_mut();
    u32 i, nfuncs, urec_size;
pub static mut krecord: *mut c_void = core::ptr::null_mut();
    let mut info_aux = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut urecord;
    let mut scalar_return = 0;
pub static mut ret: c_int = 0;
    nfuncs = attr.func_info_cnt;
    if (!nfuncs) {
    if (check_abnormal_return(env)) {
    return -EINVAL;
    }
    return 0;
    }
    if (nfuncs != env.subprog_cnt) {
    verbose(env, "number of funcs in func_info doesn't match number of subprogs\n");
    return -EINVAL;
    }
    urec_size = attr.func_info_rec_size;
    prog = env.prog;
    btf = prog.aux.btf;
    urecord = make_bpfptr(attr.func_info, uattr.is_kernel);
    krecord = prog.aux.func_info;
    info_aux = kzalloc_objs(*info_aux, nfuncs,
    GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!info_aux) {
    return -ENOMEM;
    }
    while (i < nfuncs) {
// check insn_off
    ret = -EINVAL;
    if (env.subprog_info[i].start != krecord[i].insn_off) {
    verbose(env, "func_info BTF section doesn't match subprog layout in BPF program\n");
// goto;
    }
// Already checked type_id
    type = btf_type_by_id(btf, krecord[i].type_id);
    info_aux[i].linkage = BTF_INFO_VLEN(type.info);
// Already checked func_proto
    func_proto = btf_type_by_id(btf, type.type);
    ret_type = btf_type_skip_modifiers(btf, func_proto.type, core::ptr::null_mut());
    scalar_return =
    btf_type_is_small_int(ret_type) || btf_is_any_enum(ret_type);
    if (i && !scalar_return && env.subprog_info[i].has_ld_abs) {
    verbose(env, "LD_ABS is only allowed in functions that return 'int'.\n");
// goto;
    }
    if (i && !scalar_return && env.subprog_info[i].has_tail_call) {
    verbose(env, "tail_call is only allowed in functions that return 'int'.\n");
// goto;
    }
    env.subprog_info[i].name = btf_name_by_offset(btf, type.name_off);
    bpfptr_add(&urecord, urec_size);
    }
    prog.aux.func_info_aux = info_aux;
    return 0;
// label;
    kfree(info_aux);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn check_btf_line(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
    u32 i, s, nr_linfo, ncopy, expected_size, rec_size, prev_offset = 0;
pub static mut sub: *mut c_void = core::ptr::null_mut();
pub static mut linfo: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut ulinfo;
    let mut err = 0;
    nr_linfo = attr.line_info_cnt;
    if (!nr_linfo) {
    return 0;
    }
    if (nr_linfo > INT_MAX / sizeof!(bpf_line_info)) {
    return -EINVAL;
    }
    rec_size = attr.line_info_rec_size;
    if (rec_size < MIN_BPF_LINEINFO_SIZE ||
    rec_size > MAX_LINEINFO_REC_SIZE ||
    rec_size & (sizeof!(u32) - 1)) {
    return -EINVAL;
    }
// Need to zero it in case the userspace may
// pass in a smaller bpf_line_info object.
//
    linfo = kvzalloc_objs(bpf_line_info, nr_linfo,
    GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!linfo) {
    return -ENOMEM;
    }
    prog = env.prog;
    btf = prog.aux.btf;
    s = 0;
    sub = env.subprog_info;
    ulinfo = make_bpfptr(attr.line_info, uattr.is_kernel);
    expected_size = sizeof!(bpf_line_info);
    ncopy = min_t(u32, expected_size, rec_size);
    while (i < nr_linfo) {
    err = bpf_check_uarg_tail_zero(ulinfo, expected_size, rec_size);
    if (err) {
    if (err == -E2BIG) {
    verbose(env, "nonzero tailing record in line_info");
    if (copy_to_bpfptr_offset(uattr,
    offsetof(union bpf_attr, line_info_rec_size),
    &expected_size, sizeof!(expected_size))) {
    err = -EFAULT;
    }
    }
// goto;
    }
    if (copy_from_bpfptr(&linfo[i], ulinfo, ncopy)) {
    err = -EFAULT;
// goto;
    }
//
// Check insn_off to ensure
// 1) strictly increasing AND
// 2) bounded by prog->len
//
// The linfo[0].insn_off == 0 check logically falls into
// the later "missing bpf_line_info for func..." case
// because the first linfo[0].insn_off must be the
// first sub also and the first sub must have
// subprog_info[0].start == 0.
//
    if ((i && linfo[i].insn_off <= prev_offset) ||
    linfo[i].insn_off >= prog.len) {
    verbose(env, "Invalid line_info[%u].insn_off:%u (prev_offset:%u prog.len:%u)\n",
    i, linfo[i].insn_off, prev_offset,
    prog.len);
    err = -EINVAL;
// goto;
    }
    if (!prog.insnsi[linfo[i].insn_off].code) {
    verbose(env,
    "Invalid insn code at line_info[%u].insn_off\n",
    i);
    err = -EINVAL;
// goto;
    }
    if (!btf_name_by_offset(btf, linfo[i].line_off) ||
    !btf_name_by_offset(btf, linfo[i].file_name_off)) {
    verbose(env, "Invalid line_info[%u].line_off or .file_name_off\n", i);
    err = -EINVAL;
// goto;
    }
    if (s != env.subprog_cnt) {
    if (linfo[i].insn_off == sub[s].start) {
    sub[s].linfo_idx = i;
    s += 1;
    } else if (sub[s].start < linfo[i].insn_off) {
    verbose(env, "missing bpf_line_info for func#%u\n", s);
    err = -EINVAL;
// goto;
    }
    }
    prev_offset = linfo[i].insn_off;
    bpfptr_add(&ulinfo, rec_size);
    }
    if (s != env.subprog_cnt) {
    verbose(env, "missing bpf_line_info for %u funcs starting from func#%u\n",
    env.subprog_cnt - s, s);
    err = -EINVAL;
// goto;
    }
    prog.aux.linfo = linfo;
    prog.aux.nr_linfo = nr_linfo;
    return 0;
// label;
    kvfree(linfo);
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn check_core_relo(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
    u32 i, nr_core_relo, ncopy, expected_size, rec_size;
pub static mut core_relo: bpf_core_relo = 0;
    let mut prog = env.prog;
    let mut btf = prog.aux.btf;
pub static mut bpf_core_ctx: usize = 0;
    let mut u_core_relo;
    let mut err = 0;
    nr_core_relo = attr.core_relo_cnt;
    if (!nr_core_relo) {
    return 0;
    }
    if (nr_core_relo > INT_MAX / sizeof!(bpf_core_relo)) {
    return -EINVAL;
    }
    rec_size = attr.core_relo_rec_size;
    if (rec_size < MIN_CORE_RELO_SIZE ||
    rec_size > MAX_CORE_RELO_SIZE ||
    rec_size % sizeof!(u32)) {
    return -EINVAL;
    }
    u_core_relo = make_bpfptr(attr.core_relos, uattr.is_kernel);
    expected_size = sizeof!(bpf_core_relo);
    ncopy = min_t(u32, expected_size, rec_size);
// Unlike func_info and line_info, copy and apply each CO-RE
// relocation record one at a time.
//
    while (i < nr_core_relo) {
// future proofing when sizeof!(bpf_core_relo) changes
    err = bpf_check_uarg_tail_zero(u_core_relo, expected_size, rec_size);
    if (err) {
    if (err == -E2BIG) {
    verbose(env, "nonzero tailing record in core_relo");
    if (copy_to_bpfptr_offset(uattr,
    offsetof(union bpf_attr, core_relo_rec_size),
    &expected_size, sizeof!(expected_size))) {
    err = -EFAULT;
    }
    }
    break;
    }
    if (copy_from_bpfptr(&core_relo, u_core_relo, ncopy)) {
    err = -EFAULT;
    break;
    }
    if (core_relo.insn_off % 8 || core_relo.insn_off / 8 >= prog.len) {
    verbose(env, "Invalid core_relo[%u].insn_off:%u prog.len:%u\n",
    i, core_relo.insn_off, prog.len);
    err = -EINVAL;
    break;
    }
    err = bpf_core_apply(&ctx, &core_relo, i,
    &prog.insnsi[core_relo.insn_off / 8]);
    if (err) {
    break;
    }
    bpfptr_add(&u_core_relo, rec_size);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prepare_btf_info(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!attr.func_info_cnt && !attr.line_info_cnt) {
    if (check_abnormal_return(env)) {
    return -EINVAL;
    }
    return 0;
    }
    btf = btf_get_by_fd(attr.prog_btf_fd);
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    if (btf_is_kernel(btf)) {
    btf_put(btf);
    return -EACCES;
    }
    env.prog.aux.btf = btf;
    err = prepare_btf_func(env, attr, uattr);
    if (err) {
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_check_btf_info(env: *mut bpf_verifier_env, attr: *mut union bpf_attr, uattr: bpfptr_t) -> c_int {
    let mut err = 0;
    if (!attr.func_info_cnt && !attr.line_info_cnt) {
    if (check_abnormal_return(env)) {
    return -EINVAL;
    }
    return 0;
    }
    err = check_btf_func(env, attr, uattr);
    if (err) {
    return err;
    }
    err = check_btf_line(env, attr, uattr);
    if (err) {
    return err;
    }
    err = check_core_relo(env, attr, uattr);
    if (err) {
    return err;
    }
    return 0;
    }