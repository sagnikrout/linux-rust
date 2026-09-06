//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/core.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux Socket Filter - Kernel level socket filtering
//
// Based on the design of the Berkeley Packet Filter. The new
// internal format has been designed by PLUMgrid:
//
// Copyright (c) 2011 - 2014 PLUMgrid, http://plumgrid.com
//
// Authors:
//
// Jay Schulist <jschlst@samba.org>
// Alexei Starovoitov <ast@plumgrid.com>
// Daniel Borkmann <dborkman@redhat.com>
//
// Andi Kleen - Fix a few bad bugs and races.
// Kris Katterjohn - Added many additional checks in bpf_check_classic()
//

// Registers

// Named registers

pub static mut bpf_global_ma: usize = 0;
    let mut bpf_global_ma_set = 0;
// No hurry in this branch
//
// Exported for the bpf jit load helper.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_internal_load_pointer_neg_helper(skb: *mut sk_buff, k: c_int, size: c_uint) -> *mut c_void {
    let mut ptr = core::ptr::null_mut();
    if (k >= SKF_NET_OFF) {
    ptr = skb_network_header(skb) + k - SKF_NET_OFF;
    } else if (k >= SKF_LL_OFF) {
    if (unlikely(!skb_mac_header_was_set(skb))) {
    return core::ptr::null_mut();
    }
    ptr = skb_mac_header(skb) + k - SKF_LL_OFF;
    }
    if (ptr >= skb.head && ptr + size <= skb_tail_pointer(skb)) {
    return ptr;
    }
    return core::ptr::null_mut();
    }
// tell bpf programs that include vmlinux.h kernel's PAGE_SIZE
    enum page_size_enum {
    __PAGE_SIZE = PAGE_SIZE
    };
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_alloc_no_stats(size: c_uint, gfp_extra_flags: gfp_t) -> *mut c_void {
pub static mut gfp_flags: gfp_t = 0;
pub static mut aux: *mut c_void = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
    size = round_up(size, __PAGE_SIZE);
    fp = __vmalloc(size, gfp_flags);
    if (fp == core::ptr::null_mut()) {
    return core::ptr::null_mut();
    }
    aux = kzalloc_obj(*aux, bpf_memcg_flags(GFP_KERNEL | gfp_extra_flags));
    if (aux == core::ptr::null_mut()) {
    vfree(fp);
    return core::ptr::null_mut();
    }
    fp.active = __alloc_percpu_gfp(sizeof!(u8[BPF_NR_CONTEXTS]), 4,
    bpf_memcg_flags(GFP_KERNEL | gfp_extra_flags));
    if (!fp.active) {
    vfree(fp);
    kfree(aux);
    return core::ptr::null_mut();
    }
    fp.pages = size / PAGE_SIZE;
    fp.aux = aux;
    fp.aux.main_prog_aux = aux;
    fp.aux.prog = fp;
    fp.jit_requested = ebpf_jit_enabled();
    fp.jit_required = IS_ENABLED!(CONFIG_BPF_JIT_ALWAYS_ON);
    fp.blinding_requested = bpf_jit_blinding_enabled(fp);

    aux.cgroup_atype = CGROUP_BPF_ATTACH_TYPE_INVALID;

    INIT_LIST_HEAD_RCU(&fp.aux.ksym.lnode);

    INIT_LIST_HEAD_RCU(&fp.aux.ksym_prefix.lnode);

    mutex_init(&fp.aux.used_maps_mutex);
    mutex_init(&fp.aux.ext_mutex);
    mutex_init(&fp.aux.dst_mutex);
    mutex_init(&fp.aux.st_ops_assoc_mutex);

    bpf_prog_stream_init(fp);

    return fp;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_alloc(size: c_uint, gfp_extra_flags: gfp_t) -> *mut c_void {
pub static mut gfp_flags: gfp_t = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    prog = bpf_prog_alloc_no_stats(size, gfp_extra_flags);
    if (!prog) {
    return core::ptr::null_mut();
    }
    prog.stats = alloc_percpu_gfp(bpf_prog_stats, gfp_flags);
    if (!prog.stats) {
    free_percpu(prog.active);
    kfree(prog.aux);
    vfree(prog);
    return core::ptr::null_mut();
    }
    for_each_possible_cpu(cpu) {
pub static mut pstats: *mut c_void = core::ptr::null_mut();
    pstats = per_cpu_ptr(prog.stats, cpu);
// forward_decl: _stats_init;
    }
    return prog;
    }
    EXPORT_SYMBOL_GPL(bpf_prog_alloc);
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_alloc_jited_linfo(prog: *mut bpf_prog) -> c_int {
    if (!prog.aux.nr_linfo || !prog.jit_requested) {
    return 0;
    }
    prog.aux.jited_linfo = kvzalloc_objs(*prog.aux.jited_linfo,
    prog.aux.nr_linfo,
    bpf_memcg_flags(GFP_KERNEL | __GFP_NOWARN));
    if (!prog.aux.jited_linfo) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_jit_attempt_done(prog: *mut bpf_prog) {
    if (prog.aux.jited_linfo &&
    (!prog.jited || !prog.aux.jited_linfo[0])) {
    kvfree(prog.aux.jited_linfo);
    prog.aux.jited_linfo = core::ptr::null_mut();
    }
    kfree(prog.aux.kfunc_tab);
    prog.aux.kfunc_tab = core::ptr::null_mut();
    }
// The jit engine is responsible to provide an array
// for insn_off to the jited_off mapping (insn_to_jit_off).
//
// The idx to this array is the insn_off.  Hence, the insn_off
// here is relative to the prog itself instead of the main prog.
// This array has one entry for each xlated bpf insn.
//
// jited_off is the byte off to the end of the jited insn.
//
// Hence, with
// insn_start:
// The first bpf insn off of the prog.  The insn off
// here is relative to the main prog.
// e.g. if prog is a subprog, insn_start > 0
// linfo_idx:
// The prog's idx to prog->aux->linfo and jited_linfo
//
// jited_linfo[linfo_idx] = prog->bpf_func
//
// For i > linfo_idx,
//
// jited_linfo[i] = prog->bpf_func +
// insn_to_jit_off[linfo[i].insn_off - insn_start - 1]
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_fill_jited_linfo(prog: *mut bpf_prog, insn_to_jit_off: *mut u32) {
    u32 linfo_idx, insn_start, insn_end, nr_linfo, i;
pub static mut linfo: *mut c_void = core::ptr::null_mut();
pub static mut jited_linfo: *mut c_void = core::ptr::null_mut();
    if (!prog.aux.jited_linfo || prog.aux.func_idx > prog.aux.func_cnt) {
// Userspace did not provide linfo
    return;
    }
    linfo_idx = prog.aux.linfo_idx;
    linfo = &prog.aux.linfo[linfo_idx];
    insn_start = linfo[0].insn_off;
    insn_end = insn_start + prog.len;
    jited_linfo = &prog.aux.jited_linfo[linfo_idx];
    jited_linfo[0] = prog.bpf_func;
    nr_linfo = prog.aux.nr_linfo - linfo_idx;
    for (i = 1; i < nr_linfo && linfo[i].insn_off < insn_end; i++) {
// The verifier ensures that linfo[i].insn_off is
// strictly increasing
//
    jited_linfo[i] = prog.bpf_func +
    insn_to_jit_off[linfo[i].insn_off - insn_start - 1];
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_realloc(fp_old: *mut bpf_prog, size: c_uint, gfp_extra_flags: gfp_t) -> *mut c_void {
pub static mut gfp_flags: gfp_t = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
    let mut pages = 0;
    size = round_up(size, PAGE_SIZE);
    pages = size / PAGE_SIZE;
    if (pages <= fp_old.pages) {
    return fp_old;
    }
    fp = __vmalloc(size, gfp_flags);
    if (fp) {
    memcpy(fp, fp_old, fp_old.pages * PAGE_SIZE);
    fp.pages = pages;
    fp.aux.prog = fp;
// We keep fp->aux from fp_old around in the new
// reallocated structure.
//
    fp_old.aux = core::ptr::null_mut();
    fp_old.stats = core::ptr::null_mut();
    fp_old.active = core::ptr::null_mut();
    __bpf_prog_free(fp_old);
    }
    return fp;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_free(fp: *mut bpf_prog) {
    if (fp.aux) {
    mutex_destroy(&fp.aux.used_maps_mutex);
    mutex_destroy(&fp.aux.dst_mutex);
    mutex_destroy(&fp.aux.st_ops_assoc_mutex);
    kfree(fp.aux.poke_tab);
    kfree(fp.aux);
    }
    free_percpu(fp.stats);
    free_percpu(fp.active);
    vfree(fp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_calc_tag(fp: *mut bpf_prog) -> c_int {
pub static mut size: usize = 0;
pub static mut dst: *mut c_void = core::ptr::null_mut();
    let mut was_ld_map = 0;
    let mut i = 0;
    dst = __vmalloc(size, GFP_KERNEL_ACCOUNT);
    if (!dst) {
    return -ENOMEM;
    }
// We need to take out the map fd for the digest calculation
// since they are unstable from user space side.
//
    while (i < fp.len) {
    dst[i] = fp.insnsi[i];
    if (!was_ld_map &&
    dst[i].code == (BPF_LD | BPF_IMM | BPF_DW) &&
    (dst[i].src_reg == BPF_PSEUDO_MAP_FD ||
    dst[i].src_reg == BPF_PSEUDO_MAP_VALUE)) {
    was_ld_map = true;
    dst[i].imm = 0;
    } else if (was_ld_map &&
    dst[i].code == 0 &&
    dst[i].dst_reg == 0 &&
    dst[i].src_reg == 0 &&
    dst[i].off == 0) {
    was_ld_map = false;
    dst[i].imm = 0;
    } else {
    was_ld_map = false;
    }
    }
    sha256(dst, size, fp.digest);
    vfree(dst);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_adj_delta_to_imm(insn: *mut bpf_insn, pos: u32, end_old: s32, end_new: s32, curr: s32, probe_pass: bool) -> c_int {
pub static mut imm_min: i64 = 0;
pub static mut delta: i32 = 0;
pub static mut imm: i64 = 0;
    if (curr < pos && curr + imm + 1 >= end_old) {
    imm += delta;
    }

    else if (curr >= end_new && curr + imm + 1 < end_new) {
    imm -= delta;
    }
    if (imm < imm_min || imm > imm_max) {
    return -ERANGE;
    }
    if (!probe_pass) {
    insn.imm = imm;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_adj_delta_to_off(insn: *mut bpf_insn, pos: u32, end_old: s32, end_new: s32, curr: s32, probe_pass: bool) -> c_int {
    s64 off_min, off_max, off;
pub static mut delta: i32 = 0;
    if (insn.code == (BPF_JMP32 | BPF_JA)) {
    off = insn.imm;
    off_min = S32_MIN;
    off_max = S32_MAX;
    } else {
    off = insn.off;
    off_min = S16_MIN;
    off_max = S16_MAX;
    }
    if (curr < pos && curr + off + 1 >= end_old) {
    off += delta;
    }

    else if (curr >= end_new && curr + off + 1 < end_new) {
    off -= delta;
    }
    if (off < off_min || off > off_max) {
    return -ERANGE;
    }
    if (!probe_pass) {
    if (insn.code == (BPF_JMP32 | BPF_JA)) {
    insn.imm = off;
    }
    else {
    insn.off = off;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_adj_branches(prog: *mut bpf_prog, pos: u32, end_old: s32, end_new: s32, probe_pass: bool) -> c_int {
    u32 i, insn_cnt = prog.len + (probe_pass ? end_new - end_old : 0);
    let mut insn = prog.insnsi;
pub static mut ret: c_int = 0;
    while (i < insn_cnt) {
    let mut code = 0;
// In the probing pass we still operate on the original,
// unpatched image in order to check overflows before we
// do any other adjustments. Therefore skip the patchlet.
//
    if (probe_pass && i == pos) {
    i = end_new;
    insn = prog.insnsi + end_old;
    }
    if (bpf_pseudo_func(insn)) {
    ret = bpf_adj_delta_to_imm(insn, pos, end_old,
    end_new, i, probe_pass);
    if (ret) {
    return ret;
    }
    continue;
    }
    code = insn.code;
    if ((BPF_CLASS(code) != BPF_JMP &&
    BPF_CLASS(code) != BPF_JMP32) ||
    BPF_OP(code) == BPF_EXIT) {
    continue;
    }
// Adjust offset of jmps if we cross patch boundaries.
    if (BPF_OP(code) == BPF_CALL) {
    if (insn.src_reg != BPF_PSEUDO_CALL) {
    continue;
    }
    ret = bpf_adj_delta_to_imm(insn, pos, end_old,
    end_new, i, probe_pass);
    } else {
    ret = bpf_adj_delta_to_off(insn, pos, end_old,
    end_new, i, probe_pass);
    }
    if (ret) {
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_adj_linfo(prog: *mut bpf_prog, off: u32, delta: u32) {
pub static mut linfo: *mut c_void = core::ptr::null_mut();
    u32 i, nr_linfo;
    nr_linfo = prog.aux.nr_linfo;
    if (!nr_linfo || !delta) {
    return;
    }
    linfo = prog.aux.linfo;
    for (i = 0; i < nr_linfo; i++) {
    if (off < linfo[i].insn_off)
    break;
    }
// Push all off < linfo[i].insn_off by delta
    for (; i < nr_linfo; i++) {
    linfo[i].insn_off += delta;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_patch_insn_single(prog: *mut bpf_prog, off: u32, patch: *mut bpf_insn, len: u32) -> *mut c_void {
    u32 insn_adj_cnt, insn_rest, insn_delta = len - 1;
pub static mut cnt_max: u32 = 0;
pub static mut prog_adj: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// Since our patchlet doesn't expand the image, we're done.
    if (insn_delta == 0) {
    memcpy(prog.insnsi + off, patch, sizeof!(*patch));
    return prog;
    }
    insn_adj_cnt = prog.len + insn_delta;
// Reject anything that would potentially let the insn->off
// target overflow when we have excessive program expansions.
// We need to probe here before we do any reallocation where
// we afterwards may not fail anymore.
//
    if (insn_adj_cnt > cnt_max &&
    (err = bpf_adj_branches(prog, off, off + 1, off + len, true))) {
    return ERR_PTR(err);
    }
// Several new instructions need to be inserted. Make room
// for them. Likely, there's no need for a new allocation as
// last page could have large enough tailroom.
//
    prog_adj = bpf_prog_realloc(prog, bpf_prog_size(insn_adj_cnt),
    GFP_USER);
    if (!prog_adj) {
    return ERR_PTR(-ENOMEM);
    }
    prog_adj.len = insn_adj_cnt;
// Patching happens in 3 steps:
//
// 1) Move over tail of insnsi from next instruction onwards,
// so we can patch the single target insn with one or more
// new ones (patching is always from 1 to n insns, n > 0).
// 2) Inject new instructions at the target location.
// 3) Adjust branch offsets if necessary.
//
    insn_rest = insn_adj_cnt - off - len;
    memmove(prog_adj.insnsi + off + len, prog_adj.insnsi + off + 1,
    sizeof!(*patch) * insn_rest);
    memcpy(prog_adj.insnsi + off, patch, sizeof!(*patch) * len);
// We are guaranteed to not fail at this point, otherwise
// the ship has sailed to reverse to the original state. An
// overflow cannot happen at this point.
//
    BUG_ON!(bpf_adj_branches(prog_adj, off, off + 1, off + len, false));
    bpf_adj_linfo(prog_adj, off, insn_delta);
    return prog_adj;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_remove_insns(prog: *mut bpf_prog, off: u32, cnt: u32) -> c_int {
    let mut err = 0;
// Branch offsets can't overflow when program is shrinking, no need
// to call bpf_adj_branches(..., true) here
//
    memmove(prog.insnsi + off, prog.insnsi + off + cnt,
    sizeof!(bpf_insn) * (prog.len - off - cnt));
    prog.len -= cnt;
    err = bpf_adj_branches(prog, off, off + cnt, off, false);
    WARN_ON_ONCE!(err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_kallsyms_del_subprogs(fp: *mut bpf_prog) {
    let mut i = 0;
    for (i = 0; i < fp.aux.real_func_cnt; i++) {
    bpf_prog_kallsyms_del(fp.aux.func[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_kallsyms_del_all(fp: *mut bpf_prog) {
    bpf_prog_kallsyms_del_subprogs(fp);
    bpf_prog_kallsyms_del(fp);
    }

// All BPF JIT sysctl knobs here.
pub static mut : int bpf_jit_enable = 0;
pub static mut : int bpf_jit_kallsyms = 0;
    let mut bpf_jit_harden = 0;
    let mut bpf_jit_limit = 0;
    let mut bpf_jit_limit_max = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_ksym_set_addr(prog: *mut bpf_prog) {
    WARN_ON_ONCE!(!bpf_prog_ebpf_jited(prog));
    prog.aux.ksym.start = (unsigned long) prog.bpf_func;
    prog.aux.ksym.end   = prog.aux.ksym.start + prog.jited_len;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_ksym_set_name(prog: *mut bpf_prog) {
    let mut sym = prog.aux.ksym.name;
    let mut end = sym + KSYM_NAME_LEN;
pub static mut type: *mut c_void = core::ptr::null_mut();
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(sizeof!("bpf_prog_") +
    sizeof!(prog.tag) * 2 +
// name has been null terminated.
// We should need +1 for the '_' preceding
// the name.  However, the null character
// is double counted between the name and the
// sizeof!("bpf_prog_") above, so we omit
// the +1 here.
//
    sizeof!(prog.aux.name) > KSYM_NAME_LEN);
    sym += snprintf(sym, KSYM_NAME_LEN, "bpf_prog_");
    sym  = bin2hex(sym, prog.tag, sizeof!(prog.tag));
// prog->aux->name will be ignored if full btf name is available
    if (prog.aux.func_info_cnt && prog.aux.func_idx < prog.aux.func_info_cnt) {
    type = btf_type_by_id(prog.aux.btf,
    prog.aux.func_info[prog.aux.func_idx].type_id);
    func_name = btf_name_by_offset(prog.aux.btf, type.name_off);
    snprintf(sym, (size_t)(end - sym), "_%s", func_name);
    return;
    }
    if (prog.aux.name[0]) {
    snprintf(sym, (size_t)(end - sym), "_%s", prog.aux.name);
    }
    else {
// sym = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_get_ksym_start(n: *mut latch_tree_node) -> c_ulong {
    return container_of!(n, bpf_ksym, tnode).start;
    }
    static __always_inline bool bpf_tree_less(latch_tree_node *a, latch_tree_node *b)
    {
    return bpf_get_ksym_start(a) < bpf_get_ksym_start(b);
    }
#[no_mangle]
unsafe extern "C" fn bpf_tree_comp(key: *mut c_void, n: *mut latch_tree_node) -> __always_inline int {
pub static mut val: c_ulong = 0;
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    ksym = container_of!(n, bpf_ksym, tnode);
    if (val < ksym.start) {
    return -1;
    }
// Ensure that we detect return addresses as part of the program, when
// the final instruction is a call for a program part of the stack
// trace. Therefore, do val > ksym->end instead of val >= ksym->end.
//
    if (val > ksym.end) {
    return  1;
    }
    return 0;
    }
pub static mut latch_tree_ops: usize = 0;
pub static mut bpf_lock: usize = 0;
pub static mut bpf_kallsyms: usize = 0;
    static struct latch_tree_root bpf_tree __cacheline_aligned;
#[no_mangle]
pub unsafe extern "C" fn bpf_ksym_add(ksym: *mut bpf_ksym) {
    spin_lock_bh(&bpf_lock);
    WARN_ON_ONCE!(!list_empty(&ksym.lnode));
    list_add_tail_rcu(&ksym.lnode, &bpf_kallsyms);
    latch_tree_insert(&ksym.tnode, &bpf_tree, &bpf_tree_ops);
    spin_unlock_bh(&bpf_lock);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_ksym_del(ksym: *mut bpf_ksym) {
    if (list_empty(&ksym.lnode)) {
    return;
    }
    latch_tree_erase(&ksym.tnode, &bpf_tree, &bpf_tree_ops);
    list_del_rcu(&ksym.lnode);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_ksym_del(ksym: *mut bpf_ksym) {
    spin_lock_bh(&bpf_lock);
    __bpf_ksym_del(ksym);
    spin_unlock_bh(&bpf_lock);
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_kallsyms_candidate(fp: *const bpf_prog) -> bool {
    return fp.jited && !bpf_prog_was_classic(fp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_kallsyms_add(fp: *mut bpf_prog) {
    if (!bpf_prog_kallsyms_candidate(fp) ||
    !bpf_token_capable(fp.aux.token, CAP_BPF)) {
    return;
    }
    bpf_prog_ksym_set_addr(fp);
    bpf_prog_ksym_set_name(fp);
    fp.aux.ksym.prog = true;
    bpf_ksym_add(&fp.aux.ksym);

//
// When FineIBT, code in the __cfi_foo() symbols can get executed
// and hence unwinder needs help.
//
    if (cfi_mode != CFI_FINEIBT) {
    return;
    }
    snprintf(fp.aux.ksym_prefix.name, KSYM_NAME_LEN,
    "__cfi_%s", fp.aux.ksym.name);
    fp.aux.ksym_prefix.start = (unsigned long) fp.bpf_func - 16;
    fp.aux.ksym_prefix.end   = (unsigned long) fp.bpf_func;
    bpf_ksym_add(&fp.aux.ksym_prefix);

    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_kallsyms_del(fp: *mut bpf_prog) {
    if (!bpf_prog_kallsyms_candidate(fp)) {
    return;
    }
    bpf_ksym_del(&fp.aux.ksym);

    if (cfi_mode != CFI_FINEIBT) {
    return;
    }
    bpf_ksym_del(&fp.aux.ksym_prefix);

    }
#[no_mangle]
pub unsafe extern "C" fn bpf_ksym_find(addr: c_ulong) -> *mut c_void {
pub static mut n: *mut c_void = core::ptr::null_mut();
    n = latch_tree_find(addr, &bpf_tree, &bpf_tree_ops);
    return n ? container_of!(n, bpf_ksym, tnode) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_address_lookup(addr: c_ulong, size: *mut c_ulong, off: *mut c_ulong, sym: *mut c_char) -> c_int {
pub static mut ksym: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    rcu_read_lock();
    ksym = bpf_ksym_find(addr);
    if (ksym) {
pub static mut symbol_start: c_ulong = 0;
pub static mut symbol_end: c_ulong = 0;
    ret = strscpy(sym, ksym.name, KSYM_NAME_LEN);
    if (size) {
// size = symbol_end - symbol_start;
    }
    if (off) {
// off  = addr - symbol_start;
    }
    }
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn is_bpf_text_address(addr: c_ulong) -> bool {
    let mut ret = 0;
    rcu_read_lock();
    ret = bpf_ksym_find(addr) != core::ptr::null_mut();
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_ksym_find(addr: c_ulong) -> *mut c_void {
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(!rcu_read_lock_held());
    ksym = bpf_ksym_find(addr);
    return ksym && ksym.prog ?
    container_of!(ksym, bpf_prog_aux, ksym).prog :
    core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_has_frame_pointer(ip: c_ulong) -> bool {
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    guard(rcu)();
    ksym = bpf_ksym_find(ip);
    if (!ksym || !ksym.fp_start || !ksym.fp_end) {
    return false;
    }
    offset = ip - ksym.start;
    return offset >= ksym.fp_start && offset < ksym.fp_end;
    }
    const struct exception_table_entry *search_bpf_extables(unsigned long addr)
    {
    let mut e = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    prog = bpf_prog_ksym_find(addr);
    if (!prog) {
// goto;
    }
    if (!prog.aux.num_exentries) {
// goto;
    }
    e = search_extable(prog.aux.extable, prog.aux.num_exentries, addr);
// label;
    rcu_read_unlock();
    return e;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_kallsym(symnum: c_uint, value: *mut c_ulong, type: *mut c_char, sym: *mut c_char) -> c_int {
pub static mut ksym: *mut c_void = core::ptr::null_mut();
pub static mut it: c_uint = 0;
pub static mut ret: c_int = 0;
    if (!bpf_jit_kallsyms_enabled()) {
    return ret;
    }
    rcu_read_lock();
    list_for_each_entry_rcu(ksym, &bpf_kallsyms, lnode) {
    if (it++ != symnum) {
    continue;
    }
    strscpy(sym, ksym.name, KSYM_NAME_LEN);
// value = ksym->start;
// type  = BPF_SYM_ELF_TYPE;
    ret = 0;
    break;
    }
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_add_poke_descriptor(prog: *mut bpf_prog, poke: *mut bpf_jit_poke_descriptor) -> c_int {
    let mut tab = prog.aux.poke_tab;
pub static mut poke_tab_max: u32 = 1024;
pub static mut slot: u32 = 0;
pub static mut size: u32 = 0;
    if (size > poke_tab_max) {
    return -ENOSPC;
    }
    if (poke.tailcall_target || poke.tailcall_target_stable ||
    poke.tailcall_bypass || poke.adj_off || poke.bypass_addr) {
    return -EINVAL;
    }
    match (poke.reason) {
    BPF_POKE_REASON_TAIL_CALL => {
    if (!poke.tail_call.map) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    tab = krealloc_array(tab, size, sizeof!(*poke), GFP_KERNEL);
    if (!tab) {
    return -ENOMEM;
    }
    memcpy(&tab[slot], poke, sizeof!(*poke));
    prog.aux.size_poke_tab = size;
    prog.aux.poke_tab = tab;
    return slot;
    }
//
// BPF program pack allocator.
//
// Most BPF programs are pretty small. Allocating a hole page for each
// program is sometime a waste. Many small bpf program also adds pressure
// to instruction TLB. To solve this issue, we introduce a BPF program pack
// allocator. The prog_pack allocator uses HPAGE_PMD_SIZE page (2MB on x86)
// to host BPF programs.
//
pub const BPF_PROG_CHUNK_SHIFT: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_pack {
    pub list: list_head,
    pub ptr: *mut c_void,
    pub arch_flush_needed: bool,
    pub bitmap: [c_ulong; 0],
}

#[no_mangle]
pub unsafe extern "C" fn bpf_jit_fill_hole_with_zero(area: *mut c_void, size: c_uint) {
    memset(area, 0, size);
    }
pub static mut bpf_arch_pred_flush: usize = 0;
//
// Enabled once bpf_arch_pred_flush points at a real flush routine. Lets the
// pack allocator test "is a predictor flush wired up at all" with a cheap
// static branch instead of repeatedly querying the static call target.
//
pub static mut bpf_pred_flush_enabled: usize = 0;

pub static mut pack_mutex: usize = 0;
pub static mut pack_list: usize = 0;
// PMD_SIZE is not available in some special config, e.g. ARCH=arm with
// CONFIG_MMU=n. Use PAGE_SIZE in these cases.
//

// PMD_SIZE is really big for some archs. It doesn't make sense to
// reserve too much memory in one allocation. Hardcode BPF_PROG_PACK_SIZE to
// 2MiB * num_possible_nodes(). On most architectures PMD_SIZE will be
// greater than or equal to 2MB.
//

#[no_mangle]
unsafe extern "C" fn bpf_jit_mem_is_rox() -> bool {
    return execmem_is_rox(EXECMEM_BPF);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_new_pack(bpf_fill_ill_insns: bpf_jit_fill_hole_t) -> *mut c_void {
pub static mut pack: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    pack = kzalloc_flex(*pack, bitmap, BITS_TO_LONGS(BPF_PROG_CHUNK_COUNT));
    if (!pack) {
    return core::ptr::null_mut();
    }
    pack.ptr = bpf_jit_alloc_exec(BPF_PROG_PACK_SIZE);
    if (!pack.ptr) {
// goto;
    }
    bitmap_zero(pack.bitmap, BPF_PROG_PACK_SIZE / BPF_PROG_CHUNK_SIZE);
    if (static_branch_unlikely(&bpf_pred_flush_enabled)) {
    pack.arch_flush_needed = true;
    }
    if (!bpf_jit_mem_is_rox()) {
    bpf_fill_ill_insns(pack.ptr, BPF_PROG_PACK_SIZE);
    set_vm_flush_reset_perms(pack.ptr);
    err = set_memory_rox((unsigned long)pack.ptr,
    BPF_PROG_PACK_SIZE / PAGE_SIZE);
    if (err) {
// goto;
    }
    }
    list_add_tail(&pack.list, &pack_list);
    return pack;
// label;
    bpf_jit_free_exec(pack.ptr);
    kfree(pack);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_pack_alloc(size: u32, bpf_fill_ill_insns: bpf_jit_fill_hole_t, was_classic: bool) -> *mut c_void {
pub static mut nbits: c_uint = 0;
    struct bpf_prog_pack *pack, *fallback_pack = core::ptr::null_mut();
    unsigned long pos, fallback_pos = 0;
    let mut ptr = core::ptr::null_mut();
    mutex_lock(&pack_mutex);
    if (size > BPF_PROG_PACK_SIZE) {
//
// Allocations larger than a pack get their own pages, and
// predictors are not flushed for such allocation. This is only
// safe because cBPF programs (the unprivileged attack surface)
// are bounded well below a pack size.
//
    if (was_classic && static_branch_unlikely(&bpf_pred_flush_enabled)) {
    pr_warn_once("BPF: Predictors not flushed for allocations greater than BPF_PROG_PACK_SIZE\n");
    }
    size = round_up(size, PAGE_SIZE);
    ptr = bpf_jit_alloc_exec(size);
    if (ptr && !bpf_jit_mem_is_rox()) {
    let mut err = 0;
    bpf_fill_ill_insns(ptr, size);
    set_vm_flush_reset_perms(ptr);
    err = set_memory_rox((unsigned long)ptr,
    size / PAGE_SIZE);
    if (err) {
    bpf_jit_free_exec(ptr);
    ptr = core::ptr::null_mut();
    }
    }
// goto;
    }
    list_for_each_entry(pack, &pack_list, list) {
    pos = bitmap_find_next_zero_area(pack.bitmap, BPF_PROG_CHUNK_COUNT, 0,
    nbits, 0);
    if (pos >= BPF_PROG_CHUNK_COUNT) {
    continue;
    }
// Flush not enabled, use any pack
    if (!static_branch_unlikely(&bpf_pred_flush_enabled)) {
// goto;
    }
//
// cBPF reuse of a dirty pack triggers a flush, so prefer a
// clean pack for cBPF. eBPF never flushes, so steer it to a
// dirty pack and keep clean packs free for cBPF.
//
    if (was_classic ^ pack.arch_flush_needed) {
// goto;
    }
    if (!fallback_pack) {
    fallback_pack = pack;
    fallback_pos = pos;
    }
    }
// No preferred pack found
    if (fallback_pack) {
    pack = fallback_pack;
    pos = fallback_pos;
// goto;
    }
    pack = alloc_new_pack(bpf_fill_ill_insns);
    if (!pack) {
// goto;
    }
    pos = 0;
// label;
// Flush only for cBPF as it may contain a crafted gadget
    if (static_branch_unlikely(&bpf_pred_flush_enabled) &&
    pack.arch_flush_needed &&
    was_classic) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    static_call_cond(bpf_arch_pred_flush)();
    list_for_each_entry(p, &pack_list, list) {
    p.arch_flush_needed = false;
    }
    }
    bitmap_set(pack.bitmap, pos, nbits);
    ptr = (pack.ptr) + (pos << BPF_PROG_CHUNK_SHIFT);
// label;
    mutex_unlock(&pack_mutex);
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_pack_free(ptr: *mut c_void, size: u32) {
    let mut pack = core::ptr::null_mut(), *tmp;
    let mut nbits = 0;
    let mut pos = 0;
    mutex_lock(&pack_mutex);
    if (size > BPF_PROG_PACK_SIZE) {
    bpf_jit_free_exec(ptr);
// goto;
    }
    list_for_each_entry(tmp, &pack_list, list) {
    if (ptr >= tmp.ptr && (tmp.ptr + BPF_PROG_PACK_SIZE) > ptr) {
    pack = tmp;
    break;
    }
    }
    if (WARN_ONCE(!pack, "bpf_prog_pack bug\n")) {
// goto;
    }
    nbits = BPF_PROG_SIZE_TO_NBITS(size);
    pos = ((unsigned long)ptr - (unsigned long)pack.ptr) >> BPF_PROG_CHUNK_SHIFT;
    WARN_ONCE(bpf_arch_text_invalidate(ptr, size),
    "bpf_prog_pack bug: missing bpf_arch_text_invalidate?\n");
    bitmap_clear(pack.bitmap, pos, nbits);
    if (static_branch_unlikely(&bpf_pred_flush_enabled)) {
    pack.arch_flush_needed = true;
    }
    if (bitmap_find_next_zero_area(pack.bitmap, BPF_PROG_CHUNK_COUNT, 0,
    BPF_PROG_CHUNK_COUNT, 0) == 0) {
    list_del(&pack.list);
    bpf_jit_free_exec(pack.ptr);
    kfree(pack);
    }
// label;
    mutex_unlock(&pack_mutex);
    }
    static atomic_long_t bpf_jit_current;
// Can be overridden by an arch's JIT compiler if it has a custom,
// dedicated BPF backend memory area, or if neither of the two
// below apply.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_alloc_exec_limit() -> u64 __weak {

    return MODULES_END - MODULES_VADDR;

    return VMALLOC_END - VMALLOC_START;

    }
#[no_mangle]
unsafe extern "C" fn bpf_jit_charge_init() -> c_int {
// Only used as heuristic here to derive limit.
    bpf_jit_limit_max = bpf_jit_alloc_exec_limit();
    bpf_jit_limit = min_t(u64, round_up(bpf_jit_limit_max >> 1,
    PAGE_SIZE), LONG_MAX);
    return 0;
    }
    pure_initcall!(bpf_jit_charge_init);
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_charge_modmem(size: u32) -> c_int {
    if (atomic_long_add_return(size, &bpf_jit_current) > READ_ONCE(bpf_jit_limit)) {
    if (!bpf_capable()) {
    atomic_long_sub(size, &bpf_jit_current);
    return -EPERM;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_uncharge_modmem(size: u32) {
    atomic_long_sub(size, &bpf_jit_current);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_alloc_exec(size: c_ulong) -> *mut c_void {
    return execmem_alloc(EXECMEM_BPF, size);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_alloc_exec_rw(size: c_ulong) -> *mut c_void {
    return execmem_alloc_rw(EXECMEM_BPF, size);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_free_exec(addr: *mut c_void) {
    execmem_free(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_alloc(proglen: c_uint, image_ptr: *mut *mut u8, alignment: c_uint, bpf_fill_ill_insns: bpf_jit_fill_hole_t) -> *mut c_void {
pub static mut hdr: *mut c_void = core::ptr::null_mut();
    u32 size, hole, start;
    WARN_ON_ONCE!(!is_power_of_2(alignment) ||
    alignment > BPF_IMAGE_ALIGNMENT);
// Most of BPF filters are really small, but if some of them
// fill a page, allow at least 128 extra bytes to insert a
// random section of illegal instructions.
//
    size = round_up(proglen + sizeof!(*hdr) + 128, PAGE_SIZE);
    if (bpf_jit_charge_modmem(size)) {
    return core::ptr::null_mut();
    }
    hdr = bpf_jit_alloc_exec(size);
    if (!hdr) {
    bpf_jit_uncharge_modmem(size);
    return core::ptr::null_mut();
    }
// Fill space with illegal/arch-dep instructions.
    bpf_fill_ill_insns(hdr, size);
    hdr.size = size;
    hole = min_t(unsigned int, size - (proglen + sizeof!(*hdr)),
    PAGE_SIZE - sizeof!(*hdr));
    start = get_random_u32_below(hole) & ~(alignment - 1);
// Leave a random number of instructions before BPF code.
// image_ptr = &hdr->image[start];
    return hdr;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_free(hdr: *mut bpf_binary_header) {
pub static mut size: u32 = 0;
    bpf_jit_free_exec(hdr);
    bpf_jit_uncharge_modmem(size);
    }
// Allocate jit binary from bpf_prog_pack allocator.
// Since the allocated memory is RO+X, the JIT engine cannot write directly
// to the memory. To solve this problem, a RW buffer is also allocated at
// as the same time. The JIT engine should calculate offsets based on the
// RO memory address, but write JITed program to the RW buffer. Once the
// JIT engine finishes, it calls bpf_jit_binary_pack_finalize, which copies
// the JITed program to the RO memory.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_pack_alloc(proglen: c_uint, image_ptr: *mut *mut u8, alignment: c_uint, rw_header: *mut *mut bpf_binary_header, rw_image: *mut *mut u8, bpf_fill_ill_insns: bpf_jit_fill_hole_t, was_classic: bool) -> *mut c_void {
pub static mut ro_header: *mut c_void = core::ptr::null_mut();
    u32 size, hole, start;
    WARN_ON_ONCE!(!is_power_of_2(alignment) ||
    alignment > BPF_IMAGE_ALIGNMENT);
// add 16 bytes for a random section of illegal instructions
    size = round_up(proglen + sizeof!(*ro_header) + 16, BPF_PROG_CHUNK_SIZE);
    if (bpf_jit_charge_modmem(size)) {
    return core::ptr::null_mut();
    }
    ro_header = bpf_prog_pack_alloc(size, bpf_fill_ill_insns, was_classic);
    if (!ro_header) {
    bpf_jit_uncharge_modmem(size);
    return core::ptr::null_mut();
    }
// rw_header = kvmalloc(size, GFP_KERNEL);
    if (!*rw_header) {
    bpf_prog_pack_free(ro_header, size);
    bpf_jit_uncharge_modmem(size);
    return core::ptr::null_mut();
    }
// Fill space with illegal/arch-dep instructions.
    bpf_fill_ill_insns(*rw_header, size);
    (*rw_header).size = size;
    hole = min_t(unsigned int, size - (proglen + sizeof!(*ro_header)),
    BPF_PROG_CHUNK_SIZE - sizeof!(*ro_header));
    start = get_random_u32_below(hole) & ~(alignment - 1);
// image_ptr = &ro_header->image[start];
// rw_image = &(*rw_header)->image[start];
    return ro_header;
    }
// Copy JITed text from rw_header to its final location, the ro_header.
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_pack_finalize(ro_header: *mut bpf_binary_header, rw_header: *mut bpf_binary_header) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    ptr = bpf_arch_text_copy(ro_header, rw_header, rw_header.size);
    kvfree(rw_header);
    if (IS_ERR(ptr)) {
    bpf_prog_pack_free(ro_header, ro_header.size);
    return PTR_ERR(ptr);
    }
    return 0;
    }
// bpf_jit_binary_pack_free is called in two different scenarios:
// 1) when the program is freed after;
// 2) when the JIT engine fails (before bpf_jit_binary_pack_finalize).
// For case 2), we need to free both the RO memory and the RW buffer.
//
// bpf_jit_binary_pack_free requires proper ro_header->size. However,
// bpf_jit_binary_pack_alloc does not set it. Therefore, ro_header->size
// must be set with either bpf_jit_binary_pack_finalize (normal path) or
// bpf_arch_text_copy (when jit fails).
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_pack_free(ro_header: *mut bpf_binary_header, rw_header: *mut bpf_binary_header) {
pub static mut size: u32 = 0;
    bpf_prog_pack_free(ro_header, size);
    kvfree(rw_header);
    bpf_jit_uncharge_modmem(size);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_pack_hdr(fp: *mut bpf_prog) -> *mut c_void {
pub static mut real_start: c_ulong = 0;
    let mut addr = 0;
    addr = real_start & BPF_PROG_CHUNK_MASK;
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_binary_hdr(fp: *mut bpf_prog) -> *mut c_void {
pub static mut real_start: c_ulong = 0;
    let mut addr = 0;
    addr = real_start & PAGE_MASK;
    return addr;
    }
// This symbol is only overridden by archs that have different
// requirements than the usual eBPF JITs, f.e. when they only
// implement cBPF JIT, do not set images read-only, etc.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_free(fp: *mut bpf_prog) -> void __weak {
    if (fp.jited) {
    let mut hdr = bpf_jit_binary_hdr(fp);
    bpf_jit_binary_free(hdr);
    WARN_ON_ONCE!(!bpf_prog_kallsyms_verify_off(fp));
    }
    bpf_prog_unlock_free(fp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_get_func_addr(prog: *mut bpf_prog, insn: *mut bpf_insn, extra_pass: bool, func_addr: *mut u64, func_addr_fixed: *mut bool) -> c_int {
pub static mut off: i16 = 0;
pub static mut imm: i32 = 0;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// func_addr_fixed = insn->src_reg != BPF_PSEUDO_CALL;
    if (!*func_addr_fixed) {
// Place-holder address till the last pass has collected
// all addresses for JITed subprograms in which case we
// can pick them up from prog->aux.
//
    if (!extra_pass) {
    addr = core::ptr::null_mut();
    }
    else if (prog.aux.func &&
    off >= 0 && off < prog.aux.real_func_cnt) {
    addr = prog.aux.func[off].bpf_func;
    }
    else {
    return -EINVAL;
    }
    } else if (insn.src_reg == BPF_PSEUDO_KFUNC_CALL &&
    bpf_jit_supports_far_kfunc_call()) {
    err = bpf_get_kfunc_addr(prog, insn.imm, insn.off, &addr);
    if (err) {
    return err;
    }
    } else {
// Address of a BPF helper call. Since part of the core
// kernel, it's always at a fixed location. __bpf_call_base
// and the helper with imm relative to it are both in core
// kernel.
//
    addr = __bpf_call_base + imm;
    }
// func_addr = (unsigned long)addr;
    return 0;
    }
    const char *bpf_jit_get_prog_name(bpf_prog *prog)
    {
    if (prog.aux.ksym.prog) {
    return prog.aux.ksym.name;
    }
    return prog.aux.name;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_blind_insn(from: *mut bpf_insn, aux: *mut bpf_insn, to_buff: *mut bpf_insn, emit_zext: bool) -> c_int {
    let mut to = to_buff;
pub static mut imm_rnd: u32 = 0;
    let mut off = 0;
    BUILD_BUG_ON!(BPF_REG_PARAMS + 2 != MAX_BPF_JIT_REG);
    BUILD_BUG_ON!(BPF_REG_AX + 1 != MAX_BPF_JIT_REG);
// Constraints on AX register:
//
// AX register is inaccessible from user space. It is mapped in
// all JITs, and used here for constant blinding rewrites. It is
// typically "stateless" meaning its contents are only valid within
// the executed instruction, but not across several instructions.
// There are a few exceptions however which are further detailed
// below.
//
// Constant blinding is only used by JITs, not in the interpreter.
// The interpreter uses AX in some occasions as a local temporary
// register e.g. in DIV or MOD instructions.
//
// In restricted circumstances, the verifier can also use the AX
// register for rewrites as long as they do not interfere with
// the above cases!
//
    if (from.dst_reg == BPF_REG_AX || from.src_reg == BPF_REG_AX) {
// goto;
    }
    if (from.imm == 0 &&
    (from.code == (BPF_ALU   | BPF_MOV | BPF_K) ||
    from.code == (BPF_ALU64 | BPF_MOV | BPF_K))) {
// to++ = BPF_ALU64_REG(BPF_XOR, from->dst_reg, from->dst_reg);
// goto;
    }
    match (from.code) {
    BPF_ALU | BPF_ADD | BPF_K => {
    }
    BPF_ALU | BPF_SUB | BPF_K => {
    }
    BPF_ALU | BPF_AND | BPF_K => {
    }
    BPF_ALU | BPF_OR  | BPF_K => {
    }
    BPF_ALU | BPF_XOR | BPF_K => {
    }
    BPF_ALU | BPF_MUL | BPF_K => {
    }
    BPF_ALU | BPF_MOV | BPF_K => {
    }
    BPF_ALU | BPF_DIV | BPF_K => {
    }
    BPF_ALU | BPF_MOD | BPF_K => {
// to++ = BPF_ALU32_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ from->imm);
// to++ = BPF_ALU32_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_ALU32_REG_OFF(from->code, from->dst_reg, BPF_REG_AX, from->off);
    // break;
    }
    BPF_ALU64 | BPF_ADD | BPF_K => {
    }
    BPF_ALU64 | BPF_SUB | BPF_K => {
    }
    BPF_ALU64 | BPF_AND | BPF_K => {
    }
    BPF_ALU64 | BPF_OR  | BPF_K => {
    }
    BPF_ALU64 | BPF_XOR | BPF_K => {
    }
    BPF_ALU64 | BPF_MUL | BPF_K => {
    }
    BPF_ALU64 | BPF_MOV | BPF_K => {
    }
    BPF_ALU64 | BPF_DIV | BPF_K => {
    }
    BPF_ALU64 | BPF_MOD | BPF_K => {
// to++ = BPF_ALU64_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ from->imm);
// to++ = BPF_ALU64_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_ALU64_REG_OFF(from->code, from->dst_reg, BPF_REG_AX, from->off);
    // break;
    }
    BPF_JMP | BPF_JEQ  | BPF_K => {
    }
    BPF_JMP | BPF_JNE  | BPF_K => {
    }
    BPF_JMP | BPF_JGT  | BPF_K => {
    }
    BPF_JMP | BPF_JLT  | BPF_K => {
    }
    BPF_JMP | BPF_JGE  | BPF_K => {
    }
    BPF_JMP | BPF_JLE  | BPF_K => {
    }
    BPF_JMP | BPF_JSGT | BPF_K => {
    }
    BPF_JMP | BPF_JSLT | BPF_K => {
    }
    BPF_JMP | BPF_JSGE | BPF_K => {
    }
    BPF_JMP | BPF_JSLE | BPF_K => {
    }
    BPF_JMP | BPF_JSET | BPF_K => {
// Accommodate for extra offset in case of a backjump.
    off = from.off;
    if (off < 0) {
    off -= 2;
    }
// to++ = BPF_ALU64_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ from->imm);
// to++ = BPF_ALU64_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_JMP_REG(from->code, from->dst_reg, BPF_REG_AX, off);
    // break;
    }
    BPF_JMP32 | BPF_JEQ  | BPF_K => {
    }
    BPF_JMP32 | BPF_JNE  | BPF_K => {
    }
    BPF_JMP32 | BPF_JGT  | BPF_K => {
    }
    BPF_JMP32 | BPF_JLT  | BPF_K => {
    }
    BPF_JMP32 | BPF_JGE  | BPF_K => {
    }
    BPF_JMP32 | BPF_JLE  | BPF_K => {
    }
    BPF_JMP32 | BPF_JSGT | BPF_K => {
    }
    BPF_JMP32 | BPF_JSLT | BPF_K => {
    }
    BPF_JMP32 | BPF_JSGE | BPF_K => {
    }
    BPF_JMP32 | BPF_JSLE | BPF_K => {
    }
    BPF_JMP32 | BPF_JSET | BPF_K => {
// Accommodate for extra offset in case of a backjump.
    off = from.off;
    if (off < 0) {
    off -= 2;
    }
// to++ = BPF_ALU32_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ from->imm);
// to++ = BPF_ALU32_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_JMP32_REG(from->code, from->dst_reg, BPF_REG_AX,
    off);
    // break;
    }
    BPF_LD | BPF_IMM | BPF_DW => {
// to++ = BPF_ALU64_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ aux[1].imm);
// to++ = BPF_ALU64_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_ALU64_IMM(BPF_LSH, BPF_REG_AX, 32);
// to++ = BPF_ALU64_REG(BPF_MOV, aux[0].dst_reg, BPF_REG_AX);
    // break;
    }
    0 => {
// to++ = BPF_ALU32_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ aux[0].imm);
// to++ = BPF_ALU32_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
    if (emit_zext) {
// to++ = BPF_ZEXT_REG(BPF_REG_AX);
    }
// to++ = BPF_ALU64_REG(BPF_OR,  aux[0].dst_reg, BPF_REG_AX);
    // break;
    }
    BPF_ST | BPF_MEM | BPF_DW => {
    }
    BPF_ST | BPF_MEM | BPF_W => {
    }
    BPF_ST | BPF_MEM | BPF_H => {
    }
    BPF_ST | BPF_MEM | BPF_B => {
// to++ = BPF_ALU64_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^ from->imm);
// to++ = BPF_ALU64_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
// to++ = BPF_STX_MEM(from->code, from->dst_reg, BPF_REG_AX, from->off);
    // break;
    }
    BPF_ST | BPF_PROBE_MEM32 | BPF_DW => {
    }
    BPF_ST | BPF_PROBE_MEM32 | BPF_W => {
    }
    BPF_ST | BPF_PROBE_MEM32 | BPF_H => {
    }
    BPF_ST | BPF_PROBE_MEM32 | BPF_B => {
// to++ = BPF_ALU64_IMM(BPF_MOV, BPF_REG_AX, imm_rnd ^
    from.imm);
// to++ = BPF_ALU64_IMM(BPF_XOR, BPF_REG_AX, imm_rnd);
//
// Cannot use BPF_STX_MEM() macro here as it
// hardcodes BPF_MEM mode, losing PROBE_MEM32
// and breaking arena addressing in the JIT.
//
// to++ = (bpf_insn) {
    .code  = BPF_STX | BPF_PROBE_MEM32 |
    BPF_SIZE(from.code),
    .dst_reg = from.dst_reg,
    .src_reg = BPF_REG_AX,
    .off   = from.off,
    };
    // break;
    }
    }
// label;
    return to - to_buff;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_clone_create(fp_other: *mut bpf_prog, gfp_extra_flags: gfp_t) -> *mut c_void {
pub static mut gfp_flags: gfp_t = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
    fp = __vmalloc(fp_other.pages * PAGE_SIZE, gfp_flags);
    if (fp != core::ptr::null_mut()) {
// aux->prog still points to the fp_other one, so
// when promoting the clone to the real program,
// this still needs to be adapted.
//
    memcpy(fp, fp_other, fp_other.pages * PAGE_SIZE);
    }
    return fp;
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_clone_free(fp: *mut bpf_prog) {
// aux was stolen by the other clone, so we cannot free
// it from this path! It will be freed eventually by the
// other program on release.
//
// At this point, we don't need a deferred release since
// clone is guaranteed to not be locked.
//
    fp.aux = core::ptr::null_mut();
    fp.stats = core::ptr::null_mut();
    fp.active = core::ptr::null_mut();
    __bpf_prog_free(fp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_prog_release_other(fp: *mut bpf_prog, fp_other: *mut bpf_prog) {
// We have to repoint aux->prog to self, as we don't
// know whether fp here is the clone or the original.
//
    fp.aux.prog = fp;
    if (fp.aux.offload) {
    fp.aux.offload.prog = fp;
    }
    bpf_prog_clone_free(fp_other);
    }
//
// Now this function is used only to blind the main prog and must be invoked only when
// bpf_prog_need_blind() returns true.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_blind_constants(env: *mut bpf_verifier_env, prog: *mut bpf_prog) -> *mut c_void {
    struct bpf_insn insn_buff[16], aux[2];
    let mut clone = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut insn_delta = 0;
    let mut insn_cnt = 0;
pub static mut insn: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut rewritten = 0;
    if (WARN_ON_ONCE!(env && env.prog != prog)) {
    return ERR_PTR(-EINVAL);
    }
    clone = bpf_prog_clone_create(prog, GFP_USER);
    if (!clone) {
    return ERR_PTR(-ENOMEM);
    }
// make sure bpf_patch_insn_data() patches the correct prog
    if (env) {
    env.prog = clone;
    }
    insn_cnt = clone.len;
    insn = clone.insnsi;
    while (i < insn_cnt) {
    if (bpf_pseudo_func(insn)) {
// ld_imm64 with an address of bpf subprog is not
// a user controlled constant. Don't randomize it,
// since it will conflict with jit_subprogs() logic.
//
    insn += 1;
    i += 1;
    continue;
    }
// We temporarily need to hold the original ld64 insn
// so that we can still access the first part in the
// second blinding run.
//
    if (insn[0].code == (BPF_LD | BPF_IMM | BPF_DW) &&
    insn[1].code == 0) {
    memcpy(aux, insn, sizeof!(aux));
    }
    rewritten = bpf_jit_blind_insn(insn, aux, insn_buff,
    clone.aux.verifier_zext);
    if (!rewritten) {
    continue;
    }
    if (env) {
    tmp = bpf_patch_insn_data(env, i, insn_buff, rewritten);
    }
    else {
    tmp = bpf_patch_insn_single(clone, i, insn_buff, rewritten);
    }
    if (IS_ERR_OR_NULL(tmp)) {
    if (env) {
// restore the original prog
    env.prog = prog;
    }
// Patching may have repointed aux->prog during
// realloc from the original one, so we need to
// fix it up here on error.
//
    bpf_jit_prog_release_other(prog, clone);
    return IS_ERR(tmp) ? tmp : ERR_PTR(-ENOMEM);
    }
    clone = tmp;
    insn_delta = rewritten - 1;
    if (env) {
    env.prog = clone;
    }
// Walk new program and skip insns we just inserted.
    insn = clone.insnsi + i + insn_delta;
    insn_cnt += insn_delta;
    i        += insn_delta;
    }
    clone.blinded = 1;
    return clone;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_is_indirect_target(env: *mut bpf_verifier_env, prog: *mut bpf_prog, insn_idx: c_int) -> bool {
    if (!env) {
    return false;
    }
    insn_idx += prog.aux.subprog_start;
    return env.insn_aux_data[insn_idx].indirect_target;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_out_stack_arg_cnt(env: *const bpf_verifier_env, prog: *const bpf_prog) -> u16 {
pub static mut sub: *mut c_void = core::ptr::null_mut();
    if (!env) {
    return 0;
    }
    sub = &env.subprog_info[prog.aux.func_idx];
    return sub.stack_arg_cnt - bpf_in_stack_arg_cnt(sub);
    }

// Base function for offset calculation. Needs to go into .text section,
// therefore keeping it non-static as well; will also be used by JITs
// anyway later on, so do not let the compiler omit it. This also needs
// to go into kallsyms for correlation from e.g. bpftool, so naming
// must not change.
//
#[no_mangle]
pub unsafe extern "C" fn __bpf_call_base(r1: u64, r2: u64, r3: u64, r4: u64, r5: u64) -> noinline u64 {
    return 0;
    }
    EXPORT_SYMBOL_GPL(__bpf_call_base);
// All UAPI available opcodes.

// 32 bit ALU operations. */		
// Register based. */			
    INSN_3(ALU, ADD,  X),			
    INSN_3(ALU, SUB,  X),			
    INSN_3(ALU, AND,  X),			
    INSN_3(ALU, OR,   X),			
    INSN_3(ALU, LSH,  X),			
    INSN_3(ALU, RSH,  X),			
    INSN_3(ALU, XOR,  X),			
    INSN_3(ALU, MUL,  X),			
    INSN_3(ALU, MOV,  X),			
    INSN_3(ALU, ARSH, X),			
    INSN_3(ALU, DIV,  X),			
    INSN_3(ALU, MOD,  X),			
    INSN_2(ALU, NEG),			
    INSN_3(ALU, END, TO_BE),		
    INSN_3(ALU, END, TO_LE),		
// Immediate based. */		
    INSN_3(ALU, ADD,  K),			
    INSN_3(ALU, SUB,  K),			
    INSN_3(ALU, AND,  K),			
    INSN_3(ALU, OR,   K),			
    INSN_3(ALU, LSH,  K),			
    INSN_3(ALU, RSH,  K),			
    INSN_3(ALU, XOR,  K),			
    INSN_3(ALU, MUL,  K),			
    INSN_3(ALU, MOV,  K),			
    INSN_3(ALU, ARSH, K),			
    INSN_3(ALU, DIV,  K),			
    INSN_3(ALU, MOD,  K),			
// 64 bit ALU operations. */		
// Register based. */			
    INSN_3(ALU64, ADD,  X),			
    INSN_3(ALU64, SUB,  X),			
    INSN_3(ALU64, AND,  X),			
    INSN_3(ALU64, OR,   X),			
    INSN_3(ALU64, LSH,  X),			
    INSN_3(ALU64, RSH,  X),			
    INSN_3(ALU64, XOR,  X),			
    INSN_3(ALU64, MUL,  X),			
    INSN_3(ALU64, MOV,  X),			
    INSN_3(ALU64, ARSH, X),			
    INSN_3(ALU64, DIV,  X),			
    INSN_3(ALU64, MOD,  X),			
    INSN_2(ALU64, NEG),			
    INSN_3(ALU64, END, TO_LE),		
// Immediate based. */		
    INSN_3(ALU64, ADD,  K),			
    INSN_3(ALU64, SUB,  K),			
    INSN_3(ALU64, AND,  K),			
    INSN_3(ALU64, OR,   K),			
    INSN_3(ALU64, LSH,  K),			
    INSN_3(ALU64, RSH,  K),			
    INSN_3(ALU64, XOR,  K),			
    INSN_3(ALU64, MUL,  K),			
    INSN_3(ALU64, MOV,  K),			
    INSN_3(ALU64, ARSH, K),			
    INSN_3(ALU64, DIV,  K),			
    INSN_3(ALU64, MOD,  K),			
// Call instruction. */			
    INSN_2(JMP, CALL),			
// Exit instruction. */			
    INSN_2(JMP, EXIT),			
// 32-bit Jump instructions. */		
// Register based. */			
    INSN_3(JMP32, JEQ,  X),			
    INSN_3(JMP32, JNE,  X),			
    INSN_3(JMP32, JGT,  X),			
    INSN_3(JMP32, JLT,  X),			
    INSN_3(JMP32, JGE,  X),			
    INSN_3(JMP32, JLE,  X),			
    INSN_3(JMP32, JSGT, X),			
    INSN_3(JMP32, JSLT, X),			
    INSN_3(JMP32, JSGE, X),			
    INSN_3(JMP32, JSLE, X),			
    INSN_3(JMP32, JSET, X),			
// Immediate based. */		
    INSN_3(JMP32, JEQ,  K),			
    INSN_3(JMP32, JNE,  K),			
    INSN_3(JMP32, JGT,  K),			
    INSN_3(JMP32, JLT,  K),			
    INSN_3(JMP32, JGE,  K),			
    INSN_3(JMP32, JLE,  K),			
    INSN_3(JMP32, JSGT, K),			
    INSN_3(JMP32, JSLT, K),			
    INSN_3(JMP32, JSGE, K),			
    INSN_3(JMP32, JSLE, K),			
    INSN_3(JMP32, JSET, K),			
// Jump instructions. */		
// Register based. */			
    INSN_3(JMP, JEQ,  X),			
    INSN_3(JMP, JNE,  X),			
    INSN_3(JMP, JGT,  X),			
    INSN_3(JMP, JLT,  X),			
    INSN_3(JMP, JGE,  X),			
    INSN_3(JMP, JLE,  X),			
    INSN_3(JMP, JSGT, X),			
    INSN_3(JMP, JSLT, X),			
    INSN_3(JMP, JSGE, X),			
    INSN_3(JMP, JSLE, X),			
    INSN_3(JMP, JSET, X),			
// Immediate based. */		
    INSN_3(JMP, JEQ,  K),			
    INSN_3(JMP, JNE,  K),			
    INSN_3(JMP, JGT,  K),			
    INSN_3(JMP, JLT,  K),			
    INSN_3(JMP, JGE,  K),			
    INSN_3(JMP, JLE,  K),			
    INSN_3(JMP, JSGT, K),			
    INSN_3(JMP, JSLT, K),			
    INSN_3(JMP, JSGE, K),			
    INSN_3(JMP, JSLE, K),			
    INSN_3(JMP, JSET, K),			
    INSN_2(JMP, JA),			
    INSN_2(JMP32, JA),			
// Atomic operations. */		
    INSN_3(STX, ATOMIC, B),			
    INSN_3(STX, ATOMIC, H),			
    INSN_3(STX, ATOMIC, W),			
    INSN_3(STX, ATOMIC, DW),		
// Store instructions. */		
// Register based. */			
    INSN_3(STX, MEM,  B),			
    INSN_3(STX, MEM,  H),			
    INSN_3(STX, MEM,  W),			
    INSN_3(STX, MEM,  DW),			
// Immediate based. */		
    INSN_3(ST, MEM, B),			
    INSN_3(ST, MEM, H),			
    INSN_3(ST, MEM, W),			
    INSN_3(ST, MEM, DW),			
// Load instructions. */		
// Register based. */			
    INSN_3(LDX, MEM, B),			
    INSN_3(LDX, MEM, H),			
    INSN_3(LDX, MEM, W),			
    INSN_3(LDX, MEM, DW),			
    INSN_3(LDX, MEMSX, B),			
    INSN_3(LDX, MEMSX, H),			
    INSN_3(LDX, MEMSX, W),			
// Immediate based. */		
    INSN_3(LD, IMM, DW)
#[no_mangle]
pub unsafe extern "C" fn bpf_opcode_in_insntable(code: u8) -> bool {

pub static mut public_insntable: [usize; 256] = [0; 256];

    return public_insntable[code];
    }

// Absolute value of s32 without undefined behavior for S32_MIN
#[no_mangle]
unsafe extern "C" fn abs_s32(x: i32) -> u32 {
    return x >= 0 ? (u32)x : -(u32)x;
    }
    static u64 (*interpreters_args[])(u64 r1, u64 r2, u64 r3, u64 r4, u64 r5,
    const struct bpf_insn *insn);
//
// ___bpf_prog_run - run eBPF program on a given context
// @regs: is the array of MAX_BPF_EXT_REG eBPF pseudo-registers
// @insn: is the array of eBPF instructions
//
// Decode and execute eBPF instructions.
//
// Return: whatever value is in %BPF_R0 at program exit
//
#[no_mangle]
unsafe extern "C" fn ___bpf_prog_run(regs: *mut u64, insn: *const bpf_insn) -> u64 {

    static const void * const jumptable[256] __annotate_jump_table = {
    [0 ... 255] = &&default_label,
// Now overwrite non-defaults ...
    BPF_INSN_MAP(BPF_INSN_2_LBL, BPF_INSN_3_LBL),
// Non-UAPI available opcodes.
    [BPF_JMP | BPF_CALL_ARGS] = &&JMP_CALL_ARGS,
    [BPF_JMP | BPF_TAIL_CALL] = &&JMP_TAIL_CALL,
    [BPF_ST  | BPF_NOSPEC] = &&ST_NOSPEC,
    [BPF_LDX | BPF_PROBE_MEM | BPF_B] = &&LDX_PROBE_MEM_B,
    [BPF_LDX | BPF_PROBE_MEM | BPF_H] = &&LDX_PROBE_MEM_H,
    [BPF_LDX | BPF_PROBE_MEM | BPF_W] = &&LDX_PROBE_MEM_W,
    [BPF_LDX | BPF_PROBE_MEM | BPF_DW] = &&LDX_PROBE_MEM_DW,
    [BPF_LDX | BPF_PROBE_MEMSX | BPF_B] = &&LDX_PROBE_MEMSX_B,
    [BPF_LDX | BPF_PROBE_MEMSX | BPF_H] = &&LDX_PROBE_MEMSX_H,
    [BPF_LDX | BPF_PROBE_MEMSX | BPF_W] = &&LDX_PROBE_MEMSX_W,
    };

pub static mut tail_call_cnt: u32 = 0;

// label;
    goto *jumptable[insn.code];
// Explicitly mask the register-based shift amounts with 63 or 31
// to avoid undefined behavior. Normally this won't affect the
// generated code, for example, in case of native 64 bit archs such
// as x86-64 or arm64, the compiler is optimizing the AND away for
// the interpreter. In case of JITs, each of the JIT backends compiles
// the BPF shift operations to machine instructions which produce
// implementation-defined results in such a case; the resulting
// contents of the register may be arbitrary, but program behaviour
// as a whole remains defined. In other words, in case of JIT backends,
// the AND must /not/ be added to the emitted LSH/RSH/ARSH translation.
//
// ALU (shifts)

    ALU64_##OPCODE##_X:				
    DST = DST OP (SRC & 63);		
    CONT;					
    ALU_##OPCODE##_X:				
    DST = (u32) DST OP ((u32) SRC & 31);	
    CONT;					
    ALU64_##OPCODE##_K:				
    DST = DST OP IMM;			
    CONT;					
    ALU_##OPCODE##_K:				
    DST = (u32) DST OP (u32) IMM;		
    CONT;
// ALU (rest)

    ALU64_##OPCODE##_X:				
    DST = DST OP SRC;			
    CONT;					
    ALU_##OPCODE##_X:				
    DST = (u32) DST OP (u32) SRC;		
    CONT;					
    ALU64_##OPCODE##_K:				
    DST = DST OP IMM;			
    CONT;					
    ALU_##OPCODE##_K:				
    DST = (u32) DST OP (u32) IMM;		
    CONT;
    ALU(ADD,  +)
    ALU(SUB,  -)
    ALU(AND,  &)
    ALU(OR,   |)
    ALU(XOR,  ^)
    ALU(MUL,  *)
    SHT(LSH, <<)
    SHT(RSH, >>)

// label;
    DST = (u32) -DST;
    CONT;
// label;
    DST = -DST;
    CONT;
// label;
    match (OFF) {
    0 => {
    DST = (u32) SRC;
    // break;
    }
    8 => {
    DST = (u32)(s8) SRC;
    // break;
    }
    16 => {
    DST = (u32)(s16) SRC;
    // break;
    }
    }
    CONT;
// label;
    DST = (u32) IMM;
    CONT;
// label;
    match (OFF) {
    0 => {
    DST = SRC;
    // break;
    }
    8 => {
    DST = (s8) SRC;
    // break;
    }
    16 => {
    DST = (s16) SRC;
    // break;
    }
    32 => {
    DST = (s32) SRC;
    // break;
    }
    }
    CONT;
// label;
    DST = IMM;
    CONT;
// label;
    DST = (u64) (u32) insn[0].imm | ((u64) (u32) insn[1].imm) << 32;
    insn += 1;
    CONT;
// label;
    DST = (u64) (u32) (((s32) DST) >> (SRC & 31));
    CONT;
// label;
    DST = (u64) (u32) (((s32) DST) >> IMM);
    CONT;
// label;
    (* &DST) >>= (SRC & 63);
    CONT;
// label;
    (* &DST) >>= IMM;
    CONT;
// label;
    match (OFF) {
    0 => {
    div64_u64_rem(DST, SRC, &AX);
    DST = AX;
    // break;
    }
    1 => {
    AX = div64_s64(DST, SRC);
    DST = DST - AX * SRC;
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    AX = (u32) DST;
    DST = do_div(AX, (u32) SRC);
    // break;
    }
    1 => {
    AX = abs_s32((s32)DST);
    AX = do_div(AX, abs_s32((s32)SRC));
    if ((s32)DST < 0) {
    DST = (u32)-AX;
    }
    else {
    DST = (u32)AX;
    }
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    div64_u64_rem(DST, IMM, &AX);
    DST = AX;
    // break;
    }
    1 => {
    AX = div64_s64(DST, IMM);
    DST = DST - AX * IMM;
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    AX = (u32) DST;
    DST = do_div(AX, (u32) IMM);
    // break;
    }
    1 => {
    AX = abs_s32((s32)DST);
    AX = do_div(AX, abs_s32((s32)IMM));
    if ((s32)DST < 0) {
    DST = (u32)-AX;
    }
    else {
    DST = (u32)AX;
    }
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    DST = div64_u64(DST, SRC);
    // break;
    }
    1 => {
    DST = div64_s64(DST, SRC);
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    AX = (u32) DST;
    do_div(AX, (u32) SRC);
    DST = (u32) AX;
    // break;
    }
    1 => {
    AX = abs_s32((s32)DST);
    do_div(AX, abs_s32((s32)SRC));
    if (((s32)DST < 0) == ((s32)SRC < 0)) {
    DST = (u32)AX;
    }
    else {
    DST = (u32)-AX;
    }
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    DST = div64_u64(DST, IMM);
    // break;
    }
    1 => {
    DST = div64_s64(DST, IMM);
    // break;
    }
    }
    CONT;
// label;
    match (OFF) {
    0 => {
    AX = (u32) DST;
    do_div(AX, (u32) IMM);
    DST = (u32) AX;
    // break;
    }
    1 => {
    AX = abs_s32((s32)DST);
    do_div(AX, abs_s32((s32)IMM));
    if (((s32)DST < 0) == ((s32)IMM < 0)) {
    DST = (u32)AX;
    }
    else {
    DST = (u32)-AX;
    }
    // break;
    }
    }
    CONT;
// label;
    match (IMM) {
    16 => {
    DST = ( u16) cpu_to_be16(DST);
    // break;
    }
    32 => {
    DST = ( u32) cpu_to_be32(DST);
    // break;
    }
    64 => {
    DST = ( u64) cpu_to_be64(DST);
    // break;
    }
    }
    CONT;
// label;
    match (IMM) {
    16 => {
    DST = ( u16) cpu_to_le16(DST);
    // break;
    }
    32 => {
    DST = ( u32) cpu_to_le32(DST);
    // break;
    }
    64 => {
    DST = ( u64) cpu_to_le64(DST);
    // break;
    }
    }
    CONT;
// label;
    match (IMM) {
    16 => {
    DST = ( u16) __swab16(DST);
    // break;
    }
    32 => {
    DST = ( u32) __swab32(DST);
    // break;
    }
    64 => {
    DST = ( u64) __swab64(DST);
    // break;
    }
    }
    CONT;
// CALL
// label;
// Function call scratches BPF_R1-BPF_R5 registers,
// preserves BPF_R6-BPF_R9, and stores return value
// into BPF_R0.
//
    BPF_R0 = (__bpf_call_base + insn.imm)(BPF_R1, BPF_R2, BPF_R3,
    BPF_R4, BPF_R5);
    CONT;
// label;
    BPF_R0 = interpreters_args[insn.off](BPF_R1, BPF_R2, BPF_R3,
    BPF_R4, BPF_R5,
    insn + insn.imm + 1);
    CONT;
    JMP_TAIL_CALL: {
    let mut map =  (unsigned long) BPF_R2;
    let mut array = container_of!(map, bpf_array, map);
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut index: u32 = 0;
    if (unlikely(index >= array.map.max_entries)) {
// goto;
    }
    if (unlikely(tail_call_cnt >= MAX_TAIL_CALL_CNT)) {
// goto;
    }
    prog = READ_ONCE(array.ptrs[index]);
    if (!prog) {
// goto;
    }
    tail_call_cnt += 1;
// ARG1 at this point is guaranteed to point to CTX from
// the verifier side due to the fact that the tail call is
// handled like a helper, that is, bpf_tail_call_proto,
// where arg1_type is ARG_PTR_TO_CTX.
//
    insn = prog.insnsi;
// goto;
// label;
    CONT;
    }
// label;
    insn += insn.off;
    CONT;
// label;
    insn += insn.imm;
    CONT;
// label;
    return BPF_R0;
// JMP

    JMP_##OPCODE##_X:					
    if ((SIGN##64) DST CMP_OP (SIGN##64) SRC) {	
    insn += insn.off;			
    CONT_JMP;				
    }						
    CONT;						
    JMP32_##OPCODE##_X:					
    if ((SIGN##32) DST CMP_OP (SIGN##32) SRC) {	
    insn += insn.off;			
    CONT_JMP;				
    }						
    CONT;						
    JMP_##OPCODE##_K:					
    if ((SIGN##64) DST CMP_OP (SIGN##64) IMM) {	
    insn += insn.off;			
    CONT_JMP;				
    }						
    CONT;						
    JMP32_##OPCODE##_K:					
    if ((SIGN##32) DST CMP_OP (SIGN##32) IMM) {	
    insn += insn.off;			
    CONT_JMP;				
    }						
    CONT;
    COND_JMP(u, JEQ, ==)
    COND_JMP(u, JNE, !=)
    COND_JMP(u, JGT, >)
    COND_JMP(u, JLT, <)
    COND_JMP(u, JGE, >=)
    COND_JMP(u, JLE, <=)
    COND_JMP(u, JSET, &)
    COND_JMP(s, JSGT, >)
    COND_JMP(s, JSLT, <)
    COND_JMP(s, JSGE, >=)
    COND_JMP(s, JSLE, <=)

// ST, STX and LDX
// label;
// Speculation barrier for mitigating Speculative Store Bypass,
// Bounds-Check Bypass and Type Confusion. In case of arm64, we
// rely on the firmware mitigation as controlled via the ssbd
// kernel parameter. Whenever the mitigation is enabled, it
// works for all of the kernel code with no need to provide any
// additional instructions here. In case of x86, we use 'lfence'
// insn for mitigation. We reuse preexisting logic from Spectre
// v1 mitigation that happens to produce the required code on
// x86 for v4 as well.
//
    barrier_nospec();
    CONT;

    STX_MEM_##SIZEOP:						
// (unsigned long) (DST + insn->off) = SRC;	
    CONT;							
    ST_MEM_##SIZEOP:						
// (unsigned long) (DST + insn->off) = IMM;	
    CONT;							
    LDX_MEM_##SIZEOP:						
    DST = *(unsigned long) (SRC + insn.off);	
    CONT;							
    LDX_PROBE_MEM_##SIZEOP:						
    bpf_probe_read_kernel_common(&DST, sizeof!(SIZE),	
    (long) (SRC + insn.off));	
    DST = *(&DST);					
    CONT;
    LDST(B,   u8)
    LDST(H,  u16)
    LDST(W,  u32)
    LDST(DW, u64)

    LDX_MEMSX_##SIZEOP:						
    DST = *(unsigned long) (SRC + insn.off);	
    CONT;							
    LDX_PROBE_MEMSX_##SIZEOP:					
    bpf_probe_read_kernel_common(&DST, sizeof!(SIZE),		
    (long) (SRC + insn.off));	
    DST = *(&DST);					
    CONT;
    LDSX(B,   s8)
    LDSX(H,  s16)
    LDSX(W,  s32)

    case BOP:						
    if (BPF_SIZE(insn.code) == BPF_W)		 {
    atomic_##KOP((u32) SRC, (unsigned long) 
    (DST + insn.off));	
    }
    else if (BPF_SIZE(insn.code) == BPF_DW)	 {
    atomic64_##KOP((u64) SRC, (unsigned long) 
    (DST + insn.off));	
    }
    else {
// goto;			
    }
    break;						
    case BOP | BPF_FETCH:					
    if (BPF_SIZE(insn.code) == BPF_W)		 {
    SRC = (u32) atomic_fetch_##KOP(		
    (u32) SRC,			
    (unsigned long) (DST + insn.off)); 
    }
    else if (BPF_SIZE(insn.code) == BPF_DW)	 {
    SRC = (u64) atomic64_fetch_##KOP(	
    (u64) SRC,			
    (unsigned long) (DST + insn.off)); 
    }
    else {
// goto;			
    }
    break;
// label;
// label;
// label;
// label;
    match (IMM) {
// Atomic read-modify-write instructions support only W and DW
// size modifiers.
//
    ATOMIC_ALU_OP(BPF_ADD, add)
    ATOMIC_ALU_OP(BPF_AND, and)
    ATOMIC_ALU_OP(BPF_OR, or)
    ATOMIC_ALU_OP(BPF_XOR, xor)

    BPF_XCHG => {
    if (BPF_SIZE(insn.code) == BPF_W) {
    SRC = (u32) atomic_xchg(
    (unsigned long) (DST + insn.off),
    (u32) SRC);
    }

    else if (BPF_SIZE(insn.code) == BPF_DW) {
    SRC = (u64) atomic64_xchg(
    (unsigned long) (DST + insn.off),
    (u64) SRC);
    }
    else {
// goto;
    }
    // break;
    case BPF_CMPXCHG:
    if (BPF_SIZE(insn.code) == BPF_W) {
    BPF_R0 = (u32) atomic_cmpxchg(
    (unsigned long) (DST + insn.off),
    (u32) BPF_R0, (u32) SRC);
    }

    else if (BPF_SIZE(insn.code) == BPF_DW) {
    BPF_R0 = (u64) atomic64_cmpxchg(
    (unsigned long) (DST + insn.off),
    (u64) BPF_R0, (u64) SRC);
    }
    else {
// goto;
    }
    // break;
// Atomic load and store instructions support all size
// modifiers.
//
    case BPF_LOAD_ACQ:
    switch (BPF_SIZE(insn.code)) {

    case BPF_##SIZEOP:			
    DST = (SIZE)smp_load_acquire(	
    (unsigned long)(SRC + insn.off));	
    // break;
    LOAD_ACQUIRE(B,   u8)
    LOAD_ACQUIRE(H,  u16)
    LOAD_ACQUIRE(W,  u32)

    LOAD_ACQUIRE(DW, u64)

// label;
// goto;
    }
    // break;
    case BPF_STORE_REL:
    switch (BPF_SIZE(insn.code)) {

    case BPF_##SIZEOP:		
    smp_store_release(	
    (unsigned long)(DST + insn.off), (SIZE)SRC);	
    // break;
    STORE_RELEASE(B,   u8)
    STORE_RELEASE(H,  u16)
    STORE_RELEASE(W,  u32)

    STORE_RELEASE(DW, u64)

// label;
// goto;
    }
    // break;
// label;
// goto;
    }
    CONT;
// label;
// If we ever reach this, we have a bug somewhere. Die hard here
// instead of just returning 0; we could be somewhere in a subprog,
// so execution could continue otherwise which we do /not/ want.
//
// Note, verifier whitelists all opcodes in bpf_opcode_in_insntable().
//
    pr_warn!("BPF interpreter: unknown opcode %02x (imm: 0x%x)\n",
    insn.code, insn.imm);
    BUG_ON!(1);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn PROG_NAME(ctx: *mut stack_size)( void, insn: *mut bpf_insn) -> c_uint { 
    u64 stack[stack_size / sizeof!(u64)]; 
    u64 regs[MAX_BPF_EXT_REG] = {}; 
    
    kmsan_unpoison_memory(stack, sizeof!(stack)); 
    FP = (u64) (unsigned long) &stack[ARRAY_SIZE!(stack)]; 
    ARG1 = (u64) (unsigned long) ctx; 
    return ___bpf_prog_run(regs, insn); 
    }

#[no_mangle]
pub unsafe extern "C" fn PROG_NAME_ARGS(r1: stack_size)(u64, r2: u64, r3: u64, r4: u64, r5: u64, insn: *mut bpf_insn) -> u64 { 
    u64 stack[stack_size / sizeof!(u64)]; 
    u64 regs[MAX_BPF_EXT_REG]; 
    
    kmsan_unpoison_memory(stack, sizeof!(stack)); 
    FP = (u64) (unsigned long) &stack[ARRAY_SIZE!(stack)]; 
    BPF_R1 = r1; 
    BPF_R2 = r2; 
    BPF_R3 = r3; 
    BPF_R4 = r4; 
    BPF_R5 = r5; 
    return ___bpf_prog_run(regs, insn); 
    }

    EVAL6(DEFINE_BPF_PROG_RUN, 32, 64, 96, 128, 160, 192);
    EVAL6(DEFINE_BPF_PROG_RUN, 224, 256, 288, 320, 352, 384);
    EVAL4(DEFINE_BPF_PROG_RUN, 416, 448, 480, 512);
    EVAL6(DEFINE_BPF_PROG_RUN_ARGS, 32, 64, 96, 128, 160, 192);
    EVAL6(DEFINE_BPF_PROG_RUN_ARGS, 224, 256, 288, 320, 352, 384);
    EVAL4(DEFINE_BPF_PROG_RUN_ARGS, 416, 448, 480, 512);

    static unsigned int (*interpreters[])(const void *ctx,
    const struct bpf_insn *insn) = {
    EVAL6(PROG_NAME_LIST, 32, 64, 96, 128, 160, 192)
    EVAL6(PROG_NAME_LIST, 224, 256, 288, 320, 352, 384)
    EVAL4(PROG_NAME_LIST, 416, 448, 480, 512)
    };

    static __maybe_unused
    u64 (*interpreters_args[])(u64 r1, u64 r2, u64 r3, u64 r4, u64 r5,
    const struct bpf_insn *insn) = {
    EVAL6(PROG_NAME_LIST, 32, 64, 96, 128, 160, 192)
    EVAL6(PROG_NAME_LIST, 224, 256, 288, 320, 352, 384)
    EVAL4(PROG_NAME_LIST, 416, 448, 480, 512)
    };

#[no_mangle]
pub unsafe extern "C" fn bpf_patch_call_args(insn: *mut bpf_insn, stack_depth: u32) -> c_int {
    stack_depth = max_t(u32, stack_depth, 1);
// Prevent out-of-bounds read to interpreters_args
    if (stack_depth > MAX_BPF_STACK) {
    return -EINVAL;
    }
    insn.off = (round_up(stack_depth, 32) / 32) - 1;
    insn.code = BPF_JMP | BPF_CALL_ARGS;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_call_args_imm(idx: i16) -> i32 {
    if (WARN_ON_ONCE!(idx < 0 || idx >= ARRAY_SIZE!(interpreters_args))) {
    return 0;
    }
    return BPF_CALL_IMM(interpreters_args[idx]);
    }

#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_ret0_warn(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
// If this handler ever gets executed, then BPF_JIT_ALWAYS_ON
// is not working properly, so warn about it!
//
    WARN_ON_ONCE!(1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_map_compatible(map: *mut bpf_map, fp: *mut bpf_prog) -> bool {
pub static mut prog_type: bpf_prog_type = 0;
    let mut aux = fp.aux;
    enum bpf_cgroup_storage_type i;
pub static mut ret: bool = false;
    let mut cookie = 0;
    if (fp.kprobe_override) {
    return ret;
    }
    spin_lock(&map.owner_lock);
// There's no owner yet where we could check for compatibility.
    if (!map.owner) {
    map.owner = bpf_map_owner_alloc(map);
    if (!map.owner) {
// goto;
    }
    map.owner.type  = prog_type;
    map.owner.jited = fp.jited;
    map.owner.xdp_has_frags = aux.xdp_has_frags;
    map.owner.sleepable = fp.sleepable;
    map.owner.expected_attach_type = fp.expected_attach_type;
    map.owner.attach_func_proto = aux.attach_func_proto;
    for_each_cgroup_storage_type(i) {
    map.owner.storage_cookie[i] =
    aux.cgroup_storage[i] ?
    aux.cgroup_storage[i].cookie : 0;
    }
    ret = true;
    } else {
    ret = map.owner.type  == prog_type &&
    map.owner.jited == fp.jited &&
    map.owner.xdp_has_frags == aux.xdp_has_frags &&
    map.owner.sleepable == fp.sleepable;
    if (ret &&
    map.map_type == BPF_MAP_TYPE_PROG_ARRAY &&
    map.owner.expected_attach_type != fp.expected_attach_type) {
    ret = false;
    }
    for_each_cgroup_storage_type(i) {
    if (!ret) {
    // break;
    }
    cookie = aux.cgroup_storage[i] ?
    aux.cgroup_storage[i].cookie : 0;
    ret = map.owner.storage_cookie[i] == cookie ||
    (!cookie && !aux.tail_call_reachable);
    }
    if (ret &&
    map.owner.attach_func_proto != aux.attach_func_proto) {
    match (prog_type) {
    BPF_PROG_TYPE_TRACING => {
    }
    BPF_PROG_TYPE_LSM => {
    }
    BPF_PROG_TYPE_EXT => {
    }
    BPF_PROG_TYPE_STRUCT_OPS => {
    ret = false;
    // break;
    }
    _ => {
    // break;
    }
    }
    }
    }
// label;
    spin_unlock(&map.owner_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_map_compatible(map: *mut bpf_map, fp: *const bpf_prog) -> bool {
// XDP programs inserted into maps are not guaranteed to run on
// a particular netdev (and can run outside driver context entirely
// in the case of devmap and cpumap). Until device checks
// are implemented, prohibit adding dev-bound programs to program maps.
//
    if (bpf_prog_is_dev_bound(fp.aux)) {
    return false;
    }
    return __bpf_prog_map_compatible(map, fp);
    }
#[no_mangle]
unsafe extern "C" fn bpf_check_tail_call(fp: *const bpf_prog) -> c_int {
    let mut aux = fp.aux;
    int i, ret = 0;
    mutex_lock(&aux.used_maps_mutex);
    while (i < aux.used_map_cnt) {
    let mut map = aux.used_maps[i];
    if (!map_type_contains_progs(map)) {
    continue;
    }
    if (!__bpf_prog_map_compatible(map, fp)) {
    ret = -EINVAL;
// goto;
    }
    }
// label;
    mutex_unlock(&aux.used_maps_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_select_interpreter(fp: *mut bpf_prog) -> bool {
pub static mut select_interpreter: bool = false;

pub static mut stack_depth: u32 = 0;
pub static mut idx: u32 = 0;
// may_goto may cause stack size > 512, leading to idx out-of-bounds.
// But for non-JITed programs, we don't need bpf_func, so no bounds
// check needed.
//
    if (idx < ARRAY_SIZE!(interpreters)) {
    fp.bpf_func = interpreters[idx];
    select_interpreter = true;
    } else {
    fp.bpf_func = __bpf_prog_ret0_warn;
    }

    fp.bpf_func = __bpf_prog_ret0_warn;

    return select_interpreter;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_jit_compile(env: *mut bpf_verifier_env, prog: *mut bpf_prog) -> *mut c_void {

pub static mut orig_prog: *mut c_void = core::ptr::null_mut();
    if (!bpf_prog_need_blind(prog)) {
    return bpf_int_jit_compile(env, prog);
    }
    orig_prog = prog;
    prog = bpf_jit_blind_constants(env, prog);
//
// If blinding was requested and we failed during blinding, we must fall
// back to the interpreter.
//
    if (IS_ERR(prog)) {
// goto;
    }
    prog = bpf_int_jit_compile(env, prog);
    if (prog.jited) {
    bpf_jit_prog_release_other(prog, orig_prog);
    return prog;
    }
    bpf_jit_prog_release_other(orig_prog, prog);
// label;
    prog = orig_prog;

    return prog;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_select_runtime(env: *mut bpf_verifier_env, fp: *mut bpf_prog, err: *mut c_int) -> *mut c_void {
// In case of BPF to BPF calls, verifier did all the prep
// work with regards to JITing, etc.
//
pub static mut jit_needed: bool = false;
    if (fp.bpf_func) {
// goto;
    }
    if (!bpf_prog_select_interpreter(fp)) {
    jit_needed = true;
    }
// eBPF JITs can rewrite the program in case constant
// blinding is active. However, in case of error during
// blinding, bpf_int_jit_compile() must always return a
// valid program, which in this case would simply not
// be JITed, but falls back to the interpreter.
//
    if (!bpf_prog_is_offloaded(fp.aux)) {
// err = bpf_prog_alloc_jited_linfo(fp);
    if (*err) {
    return fp;
    }
    fp = bpf_prog_jit_compile(env, fp);
    bpf_prog_jit_attempt_done(fp);
    if (!fp.jited && jit_needed) {
// err = -ENOTSUPP;
    return fp;
    }
    } else {
// err = bpf_prog_offload_compile(fp);
    if (*err) {
    return fp;
    }
    }
// label;
// err = bpf_prog_lock_ro(fp);
    if (*err) {
    return fp;
    }
// The tail call compatibility check can only be done at
// this late stage as we need to determine, if we deal
// with JITed or non JITed program concatenations and not
// all eBPF JITs might immediately support all features.
//
// err = bpf_check_tail_call(fp);
    return fp;
    }
//
// bpf_prog_select_runtime - select exec runtime for BPF program
// @fp: bpf_prog populated with BPF program
// @err: pointer to error variable
//
// Try to JIT eBPF program, if JIT is not available, use interpreter.
// The BPF program will be executed via bpf_prog_run() function.
//
// Return: the &fp argument along with &err set to 0 for success or
// a negative errno code on failure
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_select_runtime(fp: *mut bpf_prog, err: *mut c_int) -> *mut c_void {
    return __bpf_prog_select_runtime(core::ptr::null_mut(), fp, err);
    }
    EXPORT_SYMBOL_GPL(bpf_prog_select_runtime);
#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_ret1(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
    return 1;
    }
    static struct bpf_prog_dummy {
pub static mut prog: usize = 0;
    } dummy_bpf_prog = {
    .prog = {
    .bpf_func = __bpf_prog_ret1,
    },
    };
pub static mut bpf_prog_array: usize = 0;
    EXPORT_SYMBOL(bpf_empty_prog_array);
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_alloc(prog_cnt: u32, flags: gfp_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (prog_cnt) {
    p = kzalloc_flex(*p, items, prog_cnt + 1, flags);
    }
    else {
    p = &bpf_empty_prog_array;
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_free(progs: *mut bpf_prog_array) {
    if (!progs || progs == &bpf_empty_prog_array) {
    return;
    }
    kfree_rcu(progs, rcu);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_prog_array_free_sleepable_cb(rcu: *mut rcu_head) {
pub static mut progs: *mut c_void = core::ptr::null_mut();
//
// RCU Tasks Trace grace period implies RCU grace period, there is no
// need to call kfree_rcu(), just call kfree() directly.
//
    progs = container_of!(rcu, bpf_prog_array, rcu);
    kfree(progs);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_free_sleepable(progs: *mut bpf_prog_array) {
    if (!progs || progs == &bpf_empty_prog_array) {
    return;
    }
    call_rcu_tasks_trace(&progs.rcu, __bpf_prog_array_free_sleepable_cb);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_length(array: *mut bpf_prog_array) -> c_int {
pub static mut item: *mut c_void = core::ptr::null_mut();
pub static mut cnt: u32 = 0;
    for (item = array.items; item.prog; item++) {
    if (item.prog != &dummy_bpf_prog.prog)
    cnt += 1;
    }
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_is_empty(array: *mut bpf_prog_array) -> bool {
pub static mut item: *mut c_void = core::ptr::null_mut();
    for (item = array.items; item.prog; item++) {
    if (item.prog != &dummy_bpf_prog.prog)
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_copy_core(array: *mut bpf_prog_array, prog_ids: *mut u32, request_cnt: u32) -> bool {
pub static mut item: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    while (item.prog) {
    if (item.prog == &dummy_bpf_prog.prog) {
    continue;
    }
    prog_ids[i] = item.prog.aux.id;
    if (++i == request_cnt) {
    item += 1;
    break;
    }
    }
    return !!(item.prog);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_copy_to_user(array: *mut bpf_prog_array, prog_ids: *mut __u32, cnt: u32) -> c_int {
pub static mut err: c_ulong = 0;
    let mut nospc = 0;
pub static mut ids: *mut c_void = core::ptr::null_mut();
// users of this function are doing:
// cnt = bpf_prog_array_length();
// if (cnt > 0)
// bpf_prog_array_copy_to_user(..., cnt);
// so below kcalloc doesn't need extra cnt > 0 check.
//
    ids = kcalloc(cnt, sizeof!(u32), GFP_USER | __GFP_NOWARN);
    if (!ids) {
    return -ENOMEM;
    }
    nospc = bpf_prog_array_copy_core(array, ids, cnt);
    err = copy_to_user(prog_ids, ids, cnt * sizeof!(u32));
    kfree(ids);
    if (err) {
    return -EFAULT;
    }
    if (nospc) {
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_delete_safe(array: *mut bpf_prog_array, old_prog: *mut bpf_prog) {
pub static mut item: *mut c_void = core::ptr::null_mut();
    for (item = array.items; item.prog; item++) {
    if (item.prog == old_prog) {
    }
    WRITE_ONCE(item.prog, &dummy_bpf_prog.prog);
    break;
    }
    }
//
// bpf_prog_array_delete_safe_at() - Replaces the program at the given
// index into the program array with
// a dummy no-op program.
// @array: a bpf_prog_array
// @index: the index of the program to replace
//
// Skips over dummy programs, by not counting them, when calculating
// the position of the program to replace.
//
// Return:
// * 0		- Success
// * -EINVAL	- Invalid index value. Must be a non-negative integer.
// * -ENOENT	- Index out of range
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_delete_safe_at(array: *mut bpf_prog_array, index: c_int) -> c_int {
    return bpf_prog_array_update_at(array, index, &dummy_bpf_prog.prog);
    }
//
// bpf_prog_array_update_at() - Updates the program at the given index
// into the program array.
// @array: a bpf_prog_array
// @index: the index of the program to update
// @prog: the program to insert into the array
//
// Skips over dummy programs, by not counting them, when calculating
// the position of the program to update.
//
// Return:
// * 0		- Success
// * -EINVAL	- Invalid index value. Must be a non-negative integer.
// * -ENOENT	- Index out of range
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_update_at(array: *mut bpf_prog_array, index: c_int, prog: *mut bpf_prog) -> c_int {
pub static mut item: *mut c_void = core::ptr::null_mut();
    if (unlikely(index < 0)) {
    return -EINVAL;
    }
    while (item.prog) {
    if (item.prog == &dummy_bpf_prog.prog) {
    continue;
    }
    if (!index) {
    WRITE_ONCE(item.prog, prog);
    return 0;
    }
    index -= 1;
    }
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_copy(old_array: *mut bpf_prog_array, exclude_prog: *mut bpf_prog, include_prog: *mut bpf_prog, bpf_cookie: u64, new_array: *mut *mut bpf_prog_array) -> c_int {
    int new_prog_cnt, carry_prog_cnt = 0;
    let mut existing = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
pub static mut array: *mut c_void = core::ptr::null_mut();
pub static mut found_exclude: bool = false;
// Figure out how many existing progs we need to carry over to
// the new array.
//
    if (old_array) {
    existing = old_array.items;
    while (existing.prog) {
    if (existing.prog == exclude_prog) {
    found_exclude = true;
    continue;
    }
    if (existing.prog != &dummy_bpf_prog.prog) {
    carry_prog_cnt += 1;
    }
    if (existing.prog == include_prog) {
    return -EEXIST;
    }
    }
    }
    if (exclude_prog && !found_exclude) {
    return -ENOENT;
    }
// How many progs (not NULL) will be in the new array?
    new_prog_cnt = carry_prog_cnt;
    if (include_prog) {
    new_prog_cnt += 1;
    }
// Do we have any prog (not NULL) in the new array?
    if (!new_prog_cnt) {
// new_array = NULL;
    return 0;
    }
// +1 as the end of prog_array is marked with NULL
    array = bpf_prog_array_alloc(new_prog_cnt + 1, GFP_KERNEL);
    if (!array) {
    return -ENOMEM;
    }
    new = array.items;
// Fill in the new prog array
    if (carry_prog_cnt) {
    existing = old_array.items;
    while (existing.prog) {
    if (existing.prog == exclude_prog ||
    existing.prog == &dummy_bpf_prog.prog) {
    continue;
    }
    new.prog = existing.prog;
    new.bpf_cookie = existing.bpf_cookie;
    new += 1;
    }
    }
    if (include_prog) {
    new.prog = include_prog;
    new.bpf_cookie = bpf_cookie;
    new += 1;
    }
    new.prog = core::ptr::null_mut();
// new_array = array;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_array_copy_info(array: *mut bpf_prog_array, prog_ids: *mut u32, request_cnt: u32, prog_cnt: *mut u32) -> c_int {
pub static mut cnt: u32 = 0;
    if (array) {
    cnt = bpf_prog_array_length(array);
    }
// prog_cnt = cnt;
// return early if user requested only program count or nothing to copy
    if (!request_cnt || !cnt) {
    return 0;
    }
// this function is called under trace/bpf_trace.c: bpf_event_mutex
    return bpf_prog_array_copy_core(array, prog_ids, request_cnt) ? -ENOSPC
    : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_free_used_maps(aux: *mut bpf_prog_aux, used_maps: *mut *mut bpf_map, len: u32) {
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut sleepable = 0;
    let mut i = 0;
    sleepable = aux.prog.sleepable;
    while (i < len) {
    map = used_maps[i];
    if (map.ops.map_poke_untrack) {
    map.ops.map_poke_untrack(map, aux);
    }
    if (sleepable) {
    atomic64_dec(&map.sleepable_refcnt);
    }
    bpf_map_put(map);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_free_used_maps(aux: *mut bpf_prog_aux) {
    __bpf_free_used_maps(aux, aux.used_maps, aux.used_map_cnt);
    kfree(aux.used_maps);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_free_used_btfs(used_btfs: *mut btf_mod_pair, len: u32) {

pub static mut btf_mod: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < len) {
    btf_mod = &used_btfs[i];
    if (btf_mod.module) {
    module_put!(btf_mod.module);
    }
    btf_put(btf_mod.btf);
    }

    }
#[no_mangle]
unsafe extern "C" fn bpf_free_used_btfs(aux: *mut bpf_prog_aux) {
    __bpf_free_used_btfs(aux.used_btfs, aux.used_btf_cnt);
    kfree(aux.used_btfs);
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_free_deferred(work: *mut work_struct) {
pub static mut aux: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    aux = container_of!(work, bpf_prog_aux, work);

    bpf_free_kfunc_btf_tab(aux.kfunc_btf_tab);
    bpf_prog_stream_free(aux.prog);

    if (aux.cgroup_atype != CGROUP_BPF_ATTACH_TYPE_INVALID) {
    bpf_cgroup_atype_put(aux.cgroup_atype);
    }

    bpf_free_used_maps(aux);
    bpf_free_used_btfs(aux);
    bpf_prog_disassoc_struct_ops(aux.prog);
    if (bpf_prog_is_dev_bound(aux)) {
    bpf_prog_dev_bound_destroy(aux.prog);
    }

    if (aux.prog.has_callchain_buf) {
    put_callchain_buffers();
    }

    if (aux.dst_trampoline) {
    bpf_trampoline_put(aux.dst_trampoline);
    }
    while (i < aux.real_func_cnt) {
// We can just unlink the subprog poke descriptor table as
// it was originally linked to the main program and is also
// released along with it.
//
    aux.func[i].aux.poke_tab = core::ptr::null_mut();
    bpf_jit_free(aux.func[i]);
    }
    if (aux.real_func_cnt) {
    kfree(aux.func);
    bpf_prog_unlock_free(aux.prog);
    } else {
    bpf_jit_free(aux.prog);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_free(fp: *mut bpf_prog) {
    let mut aux = fp.aux;
    if (aux.dst_prog) {
    bpf_prog_put(aux.dst_prog);
    }
    bpf_token_put(aux.token);
    INIT_WORK(&aux.work, bpf_prog_free_deferred);
    schedule_work(&aux.work);
    }
    EXPORT_SYMBOL_GPL(bpf_prog_free);
// RNG for unprivileged user space with separated state from prandom_u32().
pub static mut struct rnd_state: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_user_rnd_init_once() {
    prandom_init_once(&bpf_user_rnd_state);
    }
    BPF_CALL_0(bpf_user_rnd_u32)
    {
// Should someone ever have the rather unwise idea to use some
// of the registers passed into this function, then note that
// this function is called from native eBPF and classic-to-eBPF
// transformations. Register assignments from both sides are
// different, f.e. classic always sets fn(ctx, A, X) here.
//
pub static mut state: *mut c_void = core::ptr::null_mut();
    let mut res = 0;
    state = &get_cpu_var(bpf_user_rnd_state);
    res = prandom_u32_state(state);
    put_cpu_var(bpf_user_rnd_state);
    return res;
    }
    BPF_CALL_0(bpf_get_raw_cpu_id)
    {
    return raw_smp_processor_id();
    }
// Weak definitions of helper functions in case we don't have bpf syscall.
    const struct bpf_func_proto bpf_map_lookup_elem_proto __weak;
    const struct bpf_func_proto bpf_map_update_elem_proto __weak;
    const struct bpf_func_proto bpf_map_delete_elem_proto __weak;
    const struct bpf_func_proto bpf_map_push_elem_proto __weak;
    const struct bpf_func_proto bpf_map_pop_elem_proto __weak;
    const struct bpf_func_proto bpf_map_peek_elem_proto __weak;
    const struct bpf_func_proto bpf_map_lookup_percpu_elem_proto __weak;
    const struct bpf_func_proto bpf_spin_lock_proto __weak;
    const struct bpf_func_proto bpf_spin_unlock_proto __weak;
    const struct bpf_func_proto bpf_jiffies64_proto __weak;
    const struct bpf_func_proto bpf_get_prandom_u32_proto __weak;
    const struct bpf_func_proto bpf_get_smp_processor_id_proto __weak;
    const struct bpf_func_proto bpf_get_numa_node_id_proto __weak;
    const struct bpf_func_proto bpf_ktime_get_ns_proto __weak;
    const struct bpf_func_proto bpf_ktime_get_boot_ns_proto __weak;
    const struct bpf_func_proto bpf_ktime_get_coarse_ns_proto __weak;
    const struct bpf_func_proto bpf_ktime_get_tai_ns_proto __weak;
    const struct bpf_func_proto bpf_get_current_pid_tgid_proto __weak;
    const struct bpf_func_proto bpf_get_current_uid_gid_proto __weak;
    const struct bpf_func_proto bpf_get_current_comm_proto __weak;
    const struct bpf_func_proto bpf_get_current_cgroup_id_proto __weak;
    const struct bpf_func_proto bpf_get_current_ancestor_cgroup_id_proto __weak;
    const struct bpf_func_proto bpf_get_local_storage_proto __weak;
    const struct bpf_func_proto bpf_get_ns_current_pid_tgid_proto __weak;
    const struct bpf_func_proto bpf_snprintf_btf_proto __weak;
    const struct bpf_func_proto bpf_seq_printf_btf_proto __weak;
    const struct bpf_func_proto bpf_set_retval_proto __weak;
    const struct bpf_func_proto bpf_get_retval_proto __weak;
#[no_mangle]
pub unsafe extern "C" fn bpf_get_trace_printk_proto() -> *const bpf_func_proto  __weak {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_trace_vprintk_proto() -> *const bpf_func_proto  __weak {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_perf_event_read_value_proto() -> *const bpf_func_proto  __weak {
    return core::ptr::null_mut();
    }
    u64 __weak
    bpf_event_output(bpf_map *map, u64 flags, void *meta, u64 meta_size,
    void *ctx, u64 ctx_size, bpf_ctx_copy_t ctx_copy)
    {
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL_GPL(bpf_event_output);
// Always built-in helper functions.
pub static mut bpf_func_proto: usize = 0;
// Stub for JITs that only support cBPF. eBPF programs are interpreted.
// It is encouraged to implement bpf_int_jit_compile() instead, so that
// eBPF and implicitly also cBPF can get JITed!
//
#[no_mangle]
pub unsafe extern "C" fn bpf_int_jit_compile(env: *mut bpf_verifier_env, prog: *mut bpf_prog) -> *mut bpf_prog  __weak {
    return prog;
    }
// Stub for JITs that support eBPF. All cBPF code gets transformed into
// eBPF by the kernel and is later compiled by bpf_int_jit_compile().
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_compile(prog: *mut bpf_prog) -> void __weak {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_helper_changes_pkt_data(func_id: bpf_func_id) -> bool __weak {
    return false;
    }
// Return TRUE if the JIT backend wants verifier to enable sub-register usage
// analysis code and wants explicit zero extension inserted by verifier.
// Otherwise, return FALSE.
//
// The verifier inserts an explicit zero extension after BPF_CMPXCHGs even if
// you don't override this. JITs that don't want these extra insns can detect
// them using insn_is_zext.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_needs_zext() -> bool __weak {
    return false;
    }
// By default, enable the verifier's mitigations against Spectre v1 and v4 for
// all archs. The value returned must not change at runtime as there is
// currently no support for reloading programs that were loaded without
// mitigations.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_bypass_spec_v1() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_bypass_spec_v4() -> bool __weak {
    return false;
    }
// Return true if the JIT inlines the call to the helper corresponding to
// the imm.
//
// The verifier will not patch the insn->imm for the call to the helper if
// this returns true.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_inlines_helper_call(imm: i32) -> bool __weak {
    return false;
    }
// Return TRUE if the JIT backend supports mixing bpf2bpf and tailcalls.
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_subprog_tailcalls() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_percpu_insn() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_kfunc_call() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_stack_args() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_arena_args() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_far_kfunc_call() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_arena() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_insn(insn: *mut bpf_insn, in_arena: bool) -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_fsession() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arch_uaddress_limit() -> u64 __weak {

    return TASK_SIZE;

    return 0;

    }
// Return TRUE if the JIT backend satisfies the following two conditions:
// 1) JIT backend supports atomic_xchg() on pointer-sized words.
// 2) Under the specific arch, the implementation of xchg() is the same
// as atomic_xchg() on pointer-sized words.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_ptr_xchg() -> bool __weak {
    return false;
    }
// To execute LD_ABS/LD_IND instructions __bpf_prog_run() may call
// skb_copy_bits(), so provide a weak definition of it for NET-less config.
//
    int __weak skb_copy_bits(const struct sk_buff *skb, int offset, void *to,
    int len)
    {
    return -EFAULT;
    }
    int __weak bpf_arch_text_poke(void *ip, enum bpf_text_poke_type old_t,
    enum bpf_text_poke_type new_t, void *old_addr,
    void *new_addr)
    {
    return -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arch_text_copy(dst: *mut c_void, src: *mut c_void, len: usize) -> *mut c_void {
    return ERR_PTR(-ENOTSUPP);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arch_text_invalidate(dst: *mut c_void, len: usize) -> int __weak {
    return -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_exceptions() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_private_stack() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_bpf_stack_walk(cookie: *mut *mut bool (consume_fn)(void, ip: u64, sp: u64, bp): u64, cookie: *mut c_void) -> void __weak {
    void __weak arch_bpf_stack_walk(bool (*consume_fn)(void *cookie, u64 ip, u64 sp, u64 bp), void *cookie)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jit_supports_timed_may_goto() -> bool __weak {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_bpf_timed_may_goto() -> u64 __weak {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_report_may_goto_violation() -> noinline void {

pub static mut ss: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    prog = bpf_prog_find_from_stack();
    if (!prog) {
    return;
    }
    bpf_stream_stage(ss, prog, BPF_STDERR, ({
    bpf_stream_printk(ss, "ERROR: Timeout detected for may_goto instruction\n");
    bpf_stream_dump_stack(ss);
    }));

    }
#[no_mangle]
pub unsafe extern "C" fn bpf_check_timed_may_goto(p: *mut bpf_timed_may_goto) -> u64 {
pub static mut time: u64 = 0;
// Populate the timestamp for this stack frame, and refresh count.
    if (!p.timestamp) {
    p.timestamp = time;
    return BPF_MAX_TIMED_LOOPS;
    }
// Check if we've exhausted our time slice, and zero count.
    if (unlikely(time - p.timestamp >= (NSEC_PER_SEC / 4))) {
    bpf_prog_report_may_goto_violation();
    return 0;
    }
// Refresh the count for the stack frame.
    return BPF_MAX_TIMED_LOOPS;
    }
// for configs without MMU or 32-bit
    __weak const struct bpf_map_ops arena_map_ops;
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_get_user_vm_start(arena: *mut bpf_arena) -> __weak u64 {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_get_kern_vm_start(arena: *mut bpf_arena) -> __weak u64 {
    return 0;
    }

    __weak bool bpf_arena_handle_page_fault(unsigned long addr, bool is_write,
    unsigned long fault_ip)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn bpf_global_ma_init() -> c_int {
    let mut ret = 0;
    ret = bpf_mem_alloc_init(&bpf_global_ma, 0, false);
    bpf_global_ma_set = !ret;
    return ret;
    }
    late_initcall!(bpf_global_ma_init);

pub static mut bpf_stats_enabled_key: usize = 0;
    EXPORT_SYMBOL(bpf_stats_enabled_key);
// All definitions of tracepoints related to BPF.
// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL_GPL(xdp_exception);
    EXPORT_TRACEPOINT_SYMBOL_GPL(xdp_bulk_tx);

#[no_mangle]
pub unsafe extern "C" fn bpf_get_linfo_source(btf: *mut btf, linfo: *mut bpf_line_info, src: *mut bpf_linfo_source) {
    src.file = kbasename(btf_name_by_offset(btf, linfo.file_name_off));
    src.line = btf_name_by_offset(btf, linfo.line_off);
    src.file_name_off = linfo.file_name_off;
    src.line_num = BPF_LINE_INFO_LINE_NUM(linfo.line_col);
    src.line_col = BPF_LINE_INFO_LINE_COL(linfo.line_col);
    }
    const struct bpf_line_info *bpf_find_linfo(const struct bpf_prog *prog, u32 insn_off)
    {
pub static mut linfo: *mut c_void = core::ptr::null_mut();
    let mut nr_linfo = 0;
    let mut l = 0;
    let mut r = 0;
    let mut m = 0;
    nr_linfo = prog.aux.nr_linfo;
    if (!nr_linfo || insn_off >= prog.len) {
    return core::ptr::null_mut();
    }
    linfo = prog.aux.linfo;
// Loop invariant: linfo[l].insn_off <= insns_off.
// linfo[0].insn_off == 0 which always satisfies above condition.
// Binary search is searching for rightmost linfo entry that satisfies
// the above invariant, giving us the desired record that covers given
// instruction offset.
//
    l = 0;
    r = nr_linfo - 1;
    while (l < r) {
// (r - l + 1) / 2 means we break a tie to the right, so if:
// l=1, r=2, linfo[l].insn_off <= insn_off, linfo[r].insn_off > insn_off,
// then m=2, we see that linfo[m].insn_off > insn_off, and so
// r becomes 1 and we exit the loop with correct l==1.
// If the tie was broken to the left, m=1 would end us up in
// an endless loop where l and m stay at 1 and r stays at 2.
//
    m = l + (r - l + 1) / 2;
    if (linfo[m].insn_off <= insn_off) {
    l = m;
    }
    else {
    r = m - 1;
    }
    }
    return &linfo[l];
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_get_file_line(prog: *mut bpf_prog, ip: c_ulong, filep: *mut *mut c_char, linep: *mut *mut c_char, nump: *mut c_int) -> c_int {
pub static mut src: usize = 0;
pub static mut idx: c_int = 0;
pub static mut linfo: *mut c_void = core::ptr::null_mut();
pub static mut jited_linfo: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut nr_linfo = 0;
    btf = prog.aux.btf;
    linfo = prog.aux.linfo;
    jited_linfo = prog.aux.jited_linfo;
    if (!btf || !linfo || !jited_linfo) {
    return -EINVAL;
    }
    len = prog.aux.func ? prog.aux.func[prog.aux.func_idx].len : prog.len;
    linfo = &prog.aux.linfo[prog.aux.linfo_idx];
    jited_linfo = &prog.aux.jited_linfo[prog.aux.linfo_idx];
    insn_start = linfo[0].insn_off;
    insn_end = insn_start + len;
    nr_linfo = prog.aux.nr_linfo - prog.aux.linfo_idx;
    while (i < nr_linfo &&
    linfo[i].insn_off >= insn_start && linfo[i].insn_off < insn_end) {
    if (jited_linfo[i] >= ip) {
    break;
    }
    idx = i;
    }
    if (idx == -1) {
    return -ENOENT;
    }
    bpf_get_linfo_source(btf, &linfo[idx], &src);
    while (isspace(*src.line)) {
    src.line += 1;
    }
    if (filep) {
// filep = src.file;
    }
    if (linep) {
// linep = src.line;
    }
    if (nump) {
// nump = src.line_num;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct walk_stack_ctx {
    pub prog: *mut bpf_prog,
}

#[no_mangle]
unsafe extern "C" fn find_from_stack_cb(cookie: *mut c_void, ip: u64, sp: u64, bp: u64) -> bool {
    let mut ctxp = cookie;
pub static mut prog: *mut c_void = core::ptr::null_mut();
//
// The RCU read lock is held to safely traverse the latch tree, but we
// don't need its protection when accessing the prog, since it has an
// active stack frame on the current stack trace, and won't disappear.
//
    rcu_read_lock();
    prog = bpf_prog_ksym_find(ip);
    rcu_read_unlock();
    if (!prog) {
    return true;
    }
// Make sure we return the main prog if we found a subprog
    ctxp.prog = prog.aux.main_prog_aux.prog;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_find_from_stack() -> *mut c_void {
pub static mut ctx: walk_stack_ctx = 0;
    arch_bpf_stack_walk(find_from_stack_cb, &ctx);
    return ctx.prog;
    }
}
}
