//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/const_fold.c
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

//
// Forward dataflow analysis to determine constant register values at every
// instruction. Tracks 64-bit constant values in R0-R9 through the program,
// using a fixed-point iteration in reverse postorder. Records which registers
// hold known constants and their values in
// env->insn_aux_data[].{const_reg_mask, const_reg_vals}.
//
    enum const_arg_state {
    CONST_ARG_UNVISITED,	/* instruction not yet reached */
    CONST_ARG_UNKNOWN,	/* register value not a known constant */
    CONST_ARG_CONST,	/* register holds a known 64-bit constant */
    CONST_ARG_MAP_PTR,	/* register holds a map pointer, map_index is set */
    CONST_ARG_MAP_VALUE,	/* register points to map value data, val is offset */
    CONST_ARG_SUBPROG,	/* register holds a subprog pointer, val is subprog number */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct const_arg_info {
    pub state: const_arg_state,
    pub map_index: u32,
    pub val: u64,
}

#[no_mangle]
unsafe extern "C" fn ci_is_unvisited(ci: *const const_arg_info) -> bool {
    return ci.state == CONST_ARG_UNVISITED;
    }
#[no_mangle]
unsafe extern "C" fn ci_is_unknown(ci: *const const_arg_info) -> bool {
    return ci.state == CONST_ARG_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn ci_is_const(ci: *const const_arg_info) -> bool {
    return ci.state == CONST_ARG_CONST;
    }
#[no_mangle]
unsafe extern "C" fn ci_is_map_value(ci: *const const_arg_info) -> bool {
    return ci.state == CONST_ARG_MAP_VALUE;
    }
// Transfer function: compute output register state from instruction.
#[no_mangle]
pub unsafe extern "C" fn const_reg_xfer(env: *mut bpf_verifier_env, ci_out: *mut const_arg_info, insn: *mut bpf_insn, insns: *mut bpf_insn, idx: c_int) {
pub static mut unknown: const_arg_info = 0;
    let mut dst = &ci_out[insn.dst_reg];
    let mut src = &ci_out[insn.src_reg];
pub static mut class: u8 = 0;
pub static mut mode: u8 = 0;
pub static mut opcode: u8 = 0;
    let mut r = 0;
// Stack arg stores (r11-based) are outside the tracked register set.
    if (is_stack_arg_st(insn) || is_stack_arg_stx(insn)) {
    return;
    }
    if (is_stack_arg_ldx(insn)) {
    ci_out[insn.dst_reg] = unknown;
    return;
    }
    match (class) {
    BPF_ALU => {
    }
    BPF_ALU64 => {
    match (opcode) {
    BPF_MOV | BPF_K => {
    dst.state = CONST_ARG_CONST;
    dst.val = (s64)insn.imm;
    // break;
    }
    BPF_MOV | BPF_X => {
// dst = *src;
    if (!insn.off) {
    // break;
    }
    if (!ci_is_const(dst)) {
// dst = unknown;
    // break;
    }
    match (insn.off) {
    8 => {
    }
    16 => {
    }
    32 => {
    }
    _ => {
    }
    }
    break;
    case BPF_ADD | BPF_K:
    if (!ci_is_const(dst) && !ci_is_map_value(dst)) {
// dst = unknown;
    break;
    }
    dst.val += insn.imm;
    break;
    case BPF_SUB | BPF_K:
    if (!ci_is_const(dst) && !ci_is_map_value(dst)) {
// dst = unknown;
    break;
    }
    dst.val -= insn.imm;
    break;
    case BPF_AND | BPF_K:
    if (!ci_is_const(dst)) {
    if (!insn.imm) {
    dst.state = CONST_ARG_CONST;
    dst.val = 0;
    } else {
// dst = unknown;
    }
    break;
    }
    dst.val &= (s64)insn.imm;
    break;
    case BPF_AND | BPF_X:
    if (ci_is_const(dst) && dst.val == 0) {
    break; /* 0 & x == 0 */
    }
    if (ci_is_const(src) && src.val == 0) {
    dst.state = CONST_ARG_CONST;
    dst.val = 0;
    break;
    }
    if (!ci_is_const(dst) || !ci_is_const(src)) {
// dst = unknown;
    break;
    }
    dst.val &= src.val;
    break;
// label;
// dst = unknown;
    break;
    }
    if (class == BPF_ALU) {
    if (ci_is_const(dst)) {
    dst.val = (u32)dst.val;
    }

    else if (!ci_is_unknown(dst)) {
// dst = unknown;
    }
    }
    break;
    case BPF_LD:
    if (mode == BPF_ABS || mode == BPF_IND) {
// goto;
    }
    if (mode != BPF_IMM || BPF_SIZE(insn.code) != BPF_DW) {
    break;
    }
    if (insn.src_reg == BPF_PSEUDO_FUNC) {
pub static mut subprog: c_int = 0;
    if (subprog >= 0) {
    dst.state = CONST_ARG_SUBPROG;
    dst.val = subprog;
    } else {
// dst = unknown;
    }
    } else if (insn.src_reg == BPF_PSEUDO_MAP_VALUE ||
    insn.src_reg == BPF_PSEUDO_MAP_IDX_VALUE) {
    dst.state = CONST_ARG_MAP_VALUE;
    dst.map_index = env.insn_aux_data[idx].map_index;
    dst.val = env.insn_aux_data[idx].map_off;
    } else if (insn.src_reg == BPF_PSEUDO_MAP_FD ||
    insn.src_reg == BPF_PSEUDO_MAP_IDX) {
    dst.state = CONST_ARG_MAP_PTR;
    dst.map_index = env.insn_aux_data[idx].map_index;
    } else if (insn.src_reg == 0) {
    dst.state = CONST_ARG_CONST;
    dst.val = (u64)(u32)insn.imm | ((u64)(u32)insns[idx + 1].imm << 32);
    } else {
// dst = unknown;
    }
    break;
    case BPF_LDX:
    if (!ci_is_map_value(src)) {
// dst = unknown;
    break;
    }
    let mut map = env.used_maps[src.map_index];
pub static mut size: c_int = 0;
pub static mut is_ldsx: bool = false;
pub static mut off: c_int = 0;
pub static mut val: u64 = 0;
    if (!bpf_map_is_rdonly(map) || !map.ops.map_direct_value_addr ||
    off < 0 || off + size > map.value_size ||
    bpf_map_direct_read(map, off, size, &val, is_ldsx)) {
// dst = unknown;
    break;
    }
    dst.state = CONST_ARG_CONST;
    dst.val = val;
    break;
    case BPF_JMP:
    if (opcode != BPF_CALL) {
    break;
    }
// label;
    for (r = BPF_REG_0; r <= BPF_REG_5; r++) {
    ci_out[r] = unknown;
    }
    break;
    case BPF_STX:
    r = bpf_atomic_load_reg(insn);
    if (r >= 0) {
    ci_out[r] = unknown;
    }
    break;
    }
    }
// Join function: merge output state into a successor's input state.
#[no_mangle]
pub unsafe extern "C" fn const_reg_join(ci_target: *mut const_arg_info, ci_out: *mut const_arg_info) -> bool {
pub static mut changed: bool = false;
    let mut r = 0;
    while (r < MAX_BPF_REG) {
    let mut old = &ci_target[r];
    let mut new = &ci_out[r];
    if (ci_is_unvisited(old) && !ci_is_unvisited(new)) {
    ci_target[r] = *new;
    changed = true;
    } else if (!ci_is_unknown(old) && !ci_is_unvisited(old) &&
    (new.state != old.state || new.val != old.val ||
    new.map_index != old.map_index)) {
    old.state = CONST_ARG_UNKNOWN;
    changed = true;
    }
    }
    return changed;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_compute_const_regs(env: *mut bpf_verifier_env) -> c_int {
pub static mut unknown: const_arg_info = 0;
    let mut insn_aux = env.insn_aux_data;
    let mut insns = env.prog.insnsi;
pub static mut insn_cnt: c_int = 0;
    struct const_arg_info (*ci_in)[MAX_BPF_REG];
    struct const_arg_info ci_out[MAX_BPF_REG];
pub static mut succ: *mut c_void = core::ptr::null_mut();
    let mut changed = 0;
    let mut i = 0;
    let mut r = 0;
// kvzalloc zeroes memory, so all entries start as CONST_ARG_UNVISITED (0)
    ci_in = kvzalloc_objs(*ci_in, insn_cnt, GFP_KERNEL_ACCOUNT);
    if (!ci_in) {
    return -ENOMEM;
    }
// Subprogram entries (including main at subprog 0): all registers unknown
    while (i < env.subprog_cnt) {
pub static mut start: c_int = 0;
    for (r = 0; r < MAX_BPF_REG; r++) {
    ci_in[start][r] = unknown;
    }
    }
// label;
    changed = false;
    while (i >= 0) {
pub static mut idx: c_int = 0;
    let mut insn = &insns[idx];
    let mut ci = ci_in[idx];
    memcpy(ci_out, ci, sizeof!(ci_out));
    const_reg_xfer(env, ci_out, insn, insns, idx);
    succ = bpf_insn_successors(env, idx);
    for (int s = 0; s < succ.cnt; s++) {
    changed |= const_reg_join(ci_in[succ.items[s]], ci_out);
    }
    }
    if (changed) {
// goto;
    }
// Save computed constants into insn_aux[] if they fit into 32-bit
    while (i < insn_cnt) {
pub static mut mask: u16 = 0;
    let mut aux = &insn_aux[i];
    let mut ci = ci_in[i];
    while (r < ARRAY_SIZE!(aux.const_reg_vals)) {
    let mut c = &ci[r];
    match (c.state) {
    CONST_ARG_CONST => {
pub static mut val: u64 = 0;
    if (val != (u32)val) {
    // break;
    }
    mask |= BIT(r);
    aux.const_reg_vals[r] = val;
    // break;
    }
    }
    case CONST_ARG_MAP_PTR:
    map_mask |= BIT(r);
    aux.const_reg_vals[r] = c.map_index;
    break;
    case CONST_ARG_SUBPROG:
    subprog_mask |= BIT(r);
    aux.const_reg_vals[r] = c.val;
    break;
// label;
    break;
    }
    }
    aux.const_reg_mask = mask;
    aux.const_reg_map_mask = map_mask;
    aux.const_reg_subprog_mask = subprog_mask;
    }
    kvfree(ci_in);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eval_const_branch(opcode: u8, dst_val: u64, src_val: u64) -> c_int {
    switch (BPF_OP(opcode)) {
    case BPF_JEQ:	return dst_val == src_val;
    case BPF_JNE:	return dst_val != src_val;
    case BPF_JGT:	return dst_val > src_val;
    case BPF_JGE:	return dst_val >= src_val;
    case BPF_JLT:	return dst_val < src_val;
    case BPF_JLE:	return dst_val <= src_val;
    case BPF_JSGT:	return (s64)dst_val > (s64)src_val;
    case BPF_JSGE:	return (s64)dst_val >= (s64)src_val;
    case BPF_JSLT:	return (s64)dst_val < (s64)src_val;
    case BPF_JSLE:	return (s64)dst_val <= (s64)src_val;
    case BPF_JSET:	return (bool)(dst_val & src_val);
    default:	return -1;
    }
    }
//
// Rewrite conditional branches with constant outcomes into unconditional
// jumps using register values resolved by bpf_compute_const_regs() pass.
// This eliminates dead edges from the CFG so that compute_live_registers()
// doesn't propagate liveness through dead code.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prune_dead_branches(env: *mut bpf_verifier_env) -> c_int {
    let mut insn_aux = env.insn_aux_data;
    let mut insns = env.prog.insnsi;
pub static mut insn_cnt: c_int = 0;
pub static mut changed: bool = false;
    let mut i = 0;
    while (i < insn_cnt) {
    let mut aux = &insn_aux[i];
    let mut insn = &insns[i];
pub static mut class: u8 = 0;
    u64 dst_val, src_val;
    let mut taken = 0;
    if (!bpf_insn_is_cond_jump(insn.code)) {
    continue;
    }
    if (bpf_is_may_goto_insn(insn)) {
    continue;
    }
    if (!(aux.const_reg_mask & BIT(insn.dst_reg))) {
    continue;
    }
    dst_val = aux.const_reg_vals[insn.dst_reg];
    if (BPF_SRC(insn.code) == BPF_K) {
    src_val = insn.imm;
    } else {
    if (!(aux.const_reg_mask & BIT(insn.src_reg))) {
    continue;
    }
    src_val = aux.const_reg_vals[insn.src_reg];
    }
    if (class == BPF_JMP32) {
//
// The (s32) cast maps the 32-bit range into two u64 sub-ranges:
// [0x00000000, 0x7FFFFFFF] -> [0x0000000000000000, 0x000000007FFFFFFF]
// [0x80000000, 0xFFFFFFFF] -> [0xFFFFFFFF80000000, 0xFFFFFFFFFFFFFFFF]
// The ordering is preserved within each sub-range, and
// the second sub-range is above the first as u64.
//
    dst_val = (s32)dst_val;
    src_val = (s32)src_val;
    }
    taken = eval_const_branch(insn.code, dst_val, src_val);
    if (taken < 0) {
    bpf_log(&env.log, "Unknown conditional jump %x\n", insn.code);
    return -EFAULT;
    }
// insn = BPF_JMP_A(taken ? insn->off : 0);
    changed = true;
    }
    if (!changed) {
    return 0;
    }
// recompute postorder, since CFG has changed
    kvfree(env.cfg.insn_postorder);
    env.cfg.insn_postorder = core::ptr::null_mut();
    return bpf_compute_postorder(env);
    }
}
