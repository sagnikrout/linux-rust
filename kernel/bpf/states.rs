//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/states.c
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

pub const BPF_COMPLEXITY_LIMIT_STATES: c_int = 64;
#[no_mangle]
unsafe extern "C" fn is_may_goto_insn_at(env: *mut bpf_verifier_env, insn_idx: c_int) -> bool {
    return bpf_is_may_goto_insn(&env.prog.insnsi[insn_idx]);
    }
#[no_mangle]
unsafe extern "C" fn is_iter_next_insn(env: *mut bpf_verifier_env, insn_idx: c_int) -> bool {
    return env.insn_aux_data[insn_idx].is_iter_next;
    }
#[no_mangle]
unsafe extern "C" fn update_peak_states(env: *mut bpf_verifier_env) {
    let mut cur_states = 0;
    cur_states = env.explored_states_size + env.free_list_size + env.num_backedges;
    env.peak_states = max(env.peak_states, cur_states);
    }
// struct bpf_verifier_state->parent refers to states
// that are in either of env->{expored_states,free_list}.
// In both cases the state is contained in struct bpf_verifier_state_list.
//
#[no_mangle]
pub unsafe extern "C" fn state_parent_as_list(st: *mut bpf_verifier_state) -> *mut c_void {
    if (st.parent) {
    return container_of!(st.parent, bpf_verifier_state_list, state);
    }
    return core::ptr::null_mut();
    }
// forward_decl: incomplete_read_marks;
// A state can be freed if it is no longer referenced:
// - is in the env->free_list;
// - has no children states;
//
#[no_mangle]
pub unsafe extern "C" fn maybe_free_verifier_state(env: *mut bpf_verifier_env, sl: *mut bpf_verifier_state_list) {
    if (!sl.in_free_list
    || sl.state.branches != 0
    || incomplete_read_marks(env, &sl.state)) {
    return;
    }
    list_del(&sl.node);
    bpf_free_verifier_state(&sl.state, false);
    kfree(sl);
    env.free_list_size -= 1;
    }
// For state @st look for a topmost frame with frame_insn_idx() in some SCC,
// if such frame exists form a corresponding @callchain as an array of
// call sites leading to this frame and SCC id.
// E.g.:
//
// void foo()  { A: loop {... SCC#1 ...}; }
// void bar()  { B: loop { C: foo(); ... SCC#2 ... }
// D: loop { E: foo(); ... SCC#3 ... } }
// void main() { F: bar(); }
//
// @callchain at (A) would be either (F,SCC#2) or (F,SCC#3) depending
// on @st frame call sites being (F,C,A) or (F,E,A).
//
#[no_mangle]
pub unsafe extern "C" fn compute_scc_callchain(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state, callchain: *mut bpf_scc_callchain) -> bool {
    u32 i, scc, insn_idx;
    memset(callchain, 0, sizeof!(*callchain));
    while (i <= st.curframe) {
    insn_idx = bpf_frame_insn_idx(st, i);
    scc = env.insn_aux_data[insn_idx].scc;
    if (scc) {
    callchain.scc = scc;
    break;
    } else if (i < st.curframe) {
    callchain.callsites[i] = insn_idx;
    } else {
    return false;
    }
    }
    return true;
    }
// Check if bpf_scc_visit instance for @callchain exists.
#[no_mangle]
pub unsafe extern "C" fn scc_visit_lookup(env: *mut bpf_verifier_env, callchain: *mut bpf_scc_callchain) -> *mut c_void {
    let mut info = env.scc_info[callchain.scc];
    let mut visits = info.visits;
    let mut i = 0;
    if (!info) {
    return core::ptr::null_mut();
    }
    for (i = 0; i < info.num_visits; i++) {
    if (memcmp(callchain, &visits[i].callchain, sizeof!(*callchain)) == 0)
    return &visits[i];
    }
    return core::ptr::null_mut();
    }
// Allocate a new bpf_scc_visit instance corresponding to @callchain.
// Allocated instances are alive for a duration of the do_check_common()
// call and are freed by free_states().
//
#[no_mangle]
pub unsafe extern "C" fn scc_visit_alloc(env: *mut bpf_verifier_env, callchain: *mut bpf_scc_callchain) -> *mut c_void {
pub static mut visit: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
    u32 scc, num_visits;
    let mut new_sz = 0;
    scc = callchain.scc;
    info = env.scc_info[scc];
    num_visits = info ? info.num_visits : 0;
    new_sz = sizeof!(*info) + sizeof!(bpf_scc_visit) * (num_visits + 1);
    info = kvrealloc(env.scc_info[scc], new_sz, GFP_KERNEL_ACCOUNT);
    if (!info) {
    return core::ptr::null_mut();
    }
    env.scc_info[scc] = info;
    info.num_visits = num_visits + 1;
    visit = &info.visits[num_visits];
    memset(visit, 0, sizeof!(*visit));
    memcpy(&visit.callchain, callchain, sizeof!(*callchain));
    return visit;
    }
// Form a string '(callsite#1,callsite#2,...,scc)' in env->tmp_str_buf
#[no_mangle]
pub unsafe extern "C" fn format_callchain(env: *mut bpf_verifier_env, callchain: *mut bpf_scc_callchain) -> *mut c_void {
    let mut buf = env.tmp_str_buf;
    int i, delta = 0;
    delta += snprintf(buf + delta, TMP_STR_BUF_LEN - delta, "(");
    while (i < ARRAY_SIZE!(callchain.callsites)) {
    if (!callchain.callsites[i]) {
    break;
    }
    delta += snprintf(buf + delta, TMP_STR_BUF_LEN - delta, "%u,",
    callchain.callsites[i]);
    }
    delta += snprintf(buf + delta, TMP_STR_BUF_LEN - delta, "%u)", callchain.scc);
    return env.tmp_str_buf;
    }
// If callchain for @st exists (@st is in some SCC), ensure that
// bpf_scc_visit instance for this callchain exists.
// If instance does not exist or is empty, assign visit->entry_state to @st.
//
#[no_mangle]
unsafe extern "C" fn maybe_enter_scc(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int {
    let mut callchain = &env.callchain_buf;
pub static mut visit: *mut c_void = core::ptr::null_mut();
    if (!compute_scc_callchain(env, st, callchain)) {
    return 0;
    }
    visit = scc_visit_lookup(env, callchain);
    visit = visit ?: scc_visit_alloc(env, callchain);
    if (!visit) {
    return -ENOMEM;
    }
    if (!visit.entry_state) {
    visit.entry_state = st;
    if (env.log.level & BPF_LOG_LEVEL2) {
    verbose(env, "SCC enter %s\n", format_callchain(env, callchain));
    }
    }
    return 0;
    }
// forward_decl: propagate_backedges;
// If callchain for @st exists (@st is in some SCC), make it empty:
// - set visit->entry_state to NULL;
// - flush accumulated backedges.
//
#[no_mangle]
unsafe extern "C" fn maybe_exit_scc(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int {
    let mut callchain = &env.callchain_buf;
pub static mut visit: *mut c_void = core::ptr::null_mut();
    if (!compute_scc_callchain(env, st, callchain)) {
    return 0;
    }
    visit = scc_visit_lookup(env, callchain);
    if (!visit) {
//
// If path traversal stops inside an SCC, corresponding bpf_scc_visit
// must exist for non-speculative paths. For non-speculative paths
// traversal stops when:
// a. Verification error is found, maybe_exit_scc() is not called.
// b. Top level BPF_EXIT is reached. Top level BPF_EXIT is not a member
// of any SCC.
// c. A checkpoint is reached and matched. Checkpoints are created by
// is_state_visited(), which calls maybe_enter_scc(), which allocates
// bpf_scc_visit instances for checkpoints within SCCs.
// (c) is the only case that can reach this point.
//
    if (!st.speculative) {
    verifier_bug(env, "scc exit: no visit info for call chain %s",
    format_callchain(env, callchain));
    return -EFAULT;
    }
    return 0;
    }
    if (visit.entry_state != st) {
    return 0;
    }
    if (env.log.level & BPF_LOG_LEVEL2) {
    verbose(env, "SCC exit %s\n", format_callchain(env, callchain));
    }
    visit.entry_state = core::ptr::null_mut();
    env.num_backedges -= visit.num_backedges;
    visit.num_backedges = 0;
    update_peak_states(env);
    return propagate_backedges(env, visit);
    }
// Lookup an bpf_scc_visit instance corresponding to @st callchain
// and add @backedge to visit->backedges. @st callchain must exist.
//
#[no_mangle]
pub unsafe extern "C" fn add_scc_backedge(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state, backedge: *mut bpf_scc_backedge) -> c_int {
    let mut callchain = &env.callchain_buf;
pub static mut visit: *mut c_void = core::ptr::null_mut();
    if (!compute_scc_callchain(env, st, callchain)) {
    verifier_bug(env, "add backedge: no SCC in verification path, insn_idx %d",
    st.insn_idx);
    return -EFAULT;
    }
    visit = scc_visit_lookup(env, callchain);
    if (!visit) {
    verifier_bug(env, "add backedge: no visit info for call chain %s",
    format_callchain(env, callchain));
    return -EFAULT;
    }
    if (env.log.level & BPF_LOG_LEVEL2) {
    verbose(env, "SCC backedge %s\n", format_callchain(env, callchain));
    }
    backedge.next = visit.backedges;
    visit.backedges = backedge;
    visit.num_backedges += 1;
    env.num_backedges += 1;
    update_peak_states(env);
    return 0;
    }
// bpf_reg_state->live marks for registers in a state @st are incomplete,
// if state @st is in some SCC and not all execution paths starting at this
// SCC are fully explored.
//
#[no_mangle]
pub unsafe extern "C" fn incomplete_read_marks(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> bool {
    let mut callchain = &env.callchain_buf;
pub static mut visit: *mut c_void = core::ptr::null_mut();
    if (!compute_scc_callchain(env, st, callchain)) {
    return false;
    }
    visit = scc_visit_lookup(env, callchain);
    if (!visit) {
    return false;
    }
    return !!visit.backedges;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_update_branch_counts(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int {
    let mut sl = core::ptr::null_mut(), *parent_sl;
pub static mut parent: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    while (st) {
pub static mut br: u32 = 0;
// verifier_bug_if(br > 1, ...) technically makes sense here,
// but see comment in push_stack(), hence:
//
    verifier_bug_if((int)br < 0, env, "%s:branches_to_explore=%d", __func__, br);
    if (br) {
    break;
    }
    err = maybe_exit_scc(env, st);
    if (err) {
    return err;
    }
    parent = st.parent;
    parent_sl = state_parent_as_list(st);
    if (sl) {
    maybe_free_verifier_state(env, sl);
    }
    st = parent;
    sl = parent_sl;
    }
    return 0;
    }
// check %cur's range satisfies %old's
#[no_mangle]
pub unsafe extern "C" fn range_within(old: *mut bpf_reg_state, cur: *mut bpf_reg_state) -> bool {
    return cnum64_is_subset(old.r64, cur.r64) &&
    cnum32_is_subset(old.r32, cur.r32);
    }
// If in the old state two registers had the same id, then they need to have
// the same id in the new state as well.  But that id could be different from
// the old state, so we need to track the mapping from old to new ids.
// Once we have seen that, say, a reg with old id 5 had new id 9, any subsequent
// regs with old id 5 must also have new id 9 for the new state to be safe.  But
// regs with a different old id could still have new id 9, we don't care about
// that.
// So we look through our idmap to see if this old id has been seen before.  If
// so, we require the new id to match; otherwise, we add the id pair to the map.
//
#[no_mangle]
unsafe extern "C" fn check_ids(old_id: u32, cur_id: u32, idmap: *mut bpf_idmap) -> bool {
    let mut map = idmap.map;
    let mut i = 0;
// either both IDs should be set or both should be zero
    if (!!old_id != !!cur_id) {
    return false;
    }
    if (old_id == 0) /* cur_id == 0 as well */ {
    return true;
    }
    while (i < idmap.cnt) {
    if (map[i].old == old_id) {
    return map[i].cur == cur_id;
    }
    if (map[i].cur == cur_id) {
    return false;
    }
    }
// Reached the end of known mappings; haven't seen this id before
    if (idmap.cnt < BPF_ID_MAP_SIZE) {
    map[idmap.cnt].old = old_id;
    map[idmap.cnt].cur = cur_id;
    idmap.cnt += 1;
    return true;
    }
//
// idmap slots are bounded by the number of registers and stack slots.
// Since referenced dynptrs acquire intermediate references that do
// not live in either, so the map can be exhausted. Since it is unlikely,
// fail the verification by treating the states as not equivalent.
//
    return false;
    }
//
// Compare scalar register IDs for state equivalence.
//
// When old_id == 0, the old register is independent - not linked to any
// other register. Any linking in the current state only adds constraints,
// making it more restrictive. Since the old state didn't rely on any ID
// relationships for this register, it's always safe to accept cur regardless
// of its ID. Hence, return true immediately.
//
// When old_id != 0 but cur_id == 0, we need to ensure that different
// independent registers in cur don't incorrectly satisfy the ID matching
// requirements of linked registers in old.
//
// Example: if old has r6.id=X and r7.id=X (linked), but cur has r6.id=0
// and r7.id=0 (both independent), without temp IDs both would map old_id=X
// to cur_id=0 and pass. With temp IDs: r6 maps X->temp1, r7 tries to map
// X->temp2, but X is already mapped to temp1, so the check fails correctly.
//
// When old_id has BPF_ADD_CONST set, the compound id (base | flag) and the
// base id (flag stripped) must both map consistently. Example: old has
// r2.id=A, r3.id=A|flag (r3 = r2 + delta), cur has r2.id=B, r3.id=C|flag
// (r3 derived from unrelated r4). Without the base check, idmap gets two
// independent entries A->B and A|flag->C|flag, missing that A->C conflicts
// with A->B. The base ID cross-check catches this.
//
#[no_mangle]
unsafe extern "C" fn check_scalar_ids(old_id: u32, cur_id: u32, idmap: *mut bpf_idmap) -> bool {
    if (!old_id) {
    return true;
    }
    cur_id = cur_id ? cur_id : ++idmap.tmp_id_gen;
    if (!check_ids(old_id, cur_id, idmap)) {
    return false;
    }
    if (old_id & BPF_ADD_CONST) {
    old_id &= ~BPF_ADD_CONST;
    cur_id &= ~BPF_ADD_CONST;
    if (!check_ids(old_id, cur_id, idmap)) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __clean_func_state(env: *mut bpf_verifier_env, st: *mut bpf_func_state, live_regs: u16, frame: c_int) {
    let mut i = 0;
    let mut j = 0;
    while (i < BPF_REG_FP) {
// liveness must not touch this register anymore
    if (!(live_regs & BIT(i))) {
// since the register is unused, clear its state
// to make further comparison simpler
//
    bpf_mark_reg_not_init(env, &st.regs[i]);
    }
    }
//
// Clean dead 4-byte halves within each SPI independently.
// half_spi 2*i   → lower half: slot_type[0..3] (closer to FP)
// half_spi 2*i+1 → upper half: slot_type[4..7] (farther from FP)
//
    while (i < st.allocated_stack / BPF_REG_SIZE) {
pub static mut lo_live: bool = false;
pub static mut hi_live: bool = false;
    if (!hi_live || !lo_live) {
pub static mut start: c_int = 0;
pub static mut end: c_int = 0;
pub static mut stype: u8 = 0;
//
// Don't clear special slots.
// destroy_if_dynptr_stack_slot() needs STACK_DYNPTR to
// detect overwrites and invalidate associated data slices.
// is_iter_reg_valid_uninit() and is_irq_flag_reg_valid_uninit()
// check for their respective slot types to detect double-create.
//
    if (stype == STACK_DYNPTR || stype == STACK_ITER ||
    stype == STACK_IRQ_FLAG) {
    continue;
    }
//
// Only scalar spills can be degraded to raw stack bytes
// when their high half is dead. Pointer spills need the
// saved spilled_ptr metadata so partial fills keep
// rejecting as non-scalar register fills.
//
    if (!hi_live) {
    let mut spill = &st.stack[i].spilled_ptr;
    if (lo_live && stype == STACK_SPILL) {
pub static mut val: u8 = 0;
    if (spill.type != SCALAR_VALUE) {
    continue;
    }
//
// 8 byte spill of scalar 0 where half slot is dead
// should become STACK_ZERO in lo 4 bytes.
//
    if (bpf_register_is_null(spill)) {
    val = STACK_ZERO;
    }
    while (j < 4) {
    let mut t = &st.stack[i].slot_type[j];
    if (*t == STACK_SPILL) {
// t = val;
    }
    }
    }
    bpf_mark_reg_not_init(env, spill);
    }
    for (j = start; j < end; j++) {
    st.stack[i].slot_type[j] = STACK_POISON;
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn clean_verifier_state(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int {
    let mut i = 0;
    let mut err = 0;
    err = bpf_live_stack_query_init(env, st);
    if (err) {
    return err;
    }
    while (i <= st.curframe) {
pub static mut ip: u32 = 0;
pub static mut live_regs: u16 = 0;
    __clean_func_state(env, st.frame[i], live_regs, i);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn regs_exact(rold: *mut bpf_reg_state, rcur: *mut bpf_reg_state, idmap: *mut bpf_idmap) -> bool {
    return memcmp(rold, rcur, offsetof(bpf_reg_state, id)) == 0 &&
    check_ids(rold.id, rcur.id, idmap) &&
    check_ids(rold.parent_id, rcur.parent_id, idmap);
    }
    enum exact_level {
    NOT_EXACT,
    EXACT,
    RANGE_WITHIN
    };
// Returns true if (rold safe implies rcur safe)
#[no_mangle]
pub unsafe extern "C" fn regsafe(env: *mut bpf_verifier_env, rold: *mut bpf_reg_state, rcur: *mut bpf_reg_state, idmap: *mut bpf_idmap, exact: exact_level) -> bool {
    if (exact == EXACT) {
    return regs_exact(rold, rcur, idmap);
    }
    if (rold.type == NOT_INIT) {
// explored state can't have used this
    return true;
    }
// Enforce that register types have to match exactly, including their
// modifiers (like PTR_MAYBE_NULL, MEM_RDONLY, etc), as a general
// rule.
//
// One can make a point that using a pointer register as unbounded
// SCALAR would be technically acceptable, but this could lead to
// pointer leaks because scalars are allowed to leak while pointers
// are not. We could make this safe in special cases if root is
// calling us, but it's probably not worth the hassle.
//
// Also, register types that are *not* MAYBE_NULL could technically be
// safe to use as their MAYBE_NULL variants (e.g., PTR_TO_MAP_VALUE
// is safe to be used as PTR_TO_MAP_VALUE_OR_NULL, provided both point
// to the same map).
// However, if the old MAYBE_NULL register then got NULL checked,
// doing so could have affected others with the same id, and we can't
// check for that because we lost the id when we converted to
// a non-MAYBE_NULL variant.
// So, as a general rule we don't allow mixing MAYBE_NULL and
// non-MAYBE_NULL registers as well.
//
    if (rold.type != rcur.type) {
    return false;
    }
    switch (base_type(rold.type)) {
    case SCALAR_VALUE:
    if (env.explore_alu_limits) {
// explore_alu_limits disables tnum_in() and range_within()
// logic and requires everything to be strict
//
    return memcmp(rold, rcur, offsetof(bpf_reg_state, id)) == 0 &&
    check_scalar_ids(rold.id, rcur.id, idmap);
    }
    if (!rold.precise && exact == NOT_EXACT) {
    return true;
    }
//
// Linked register tracking uses rold->id to detect relationships.
// When rold->id == 0, the register is independent and any linking
// in rcur only adds constraints. When rold->id != 0, we must verify
// id mapping and (for BPF_ADD_CONST) offset consistency.
//
// +------------------+-----------+------------------+---------------+
// |                  | rold->id  | rold + ADD_CONST | rold->id == 0 |
// |------------------+-----------+------------------+---------------|
// | rcur->id         | range,ids | false            | range         |
// | rcur + ADD_CONST | false     | range,ids,off    | range         |
// | rcur->id == 0    | range,ids | false            | range         |
// +------------------+-----------+------------------+---------------+
//
// Why check_ids() for scalar registers?
//
// Consider the following BPF code:
// 1: r6 = ... unbound scalar, ID=a ...
// 2: r7 = ... unbound scalar, ID=b ...
// 3: if (r6 > r7) goto +1
// 4: r6 = r7
// 5: if (r6 > X) goto ...
// 6: ... memory operation using r7 ...
//
// First verification path is [1-6]:
// - at (4) same bpf_reg_state::id (b) would be assigned to r6 and r7;
// - at (5) r6 would be marked <= X, sync_linked_regs() would also mark
// r7 <= X, because r6 and r7 share same id.
// Next verification path is [1-4, 6].
//
// Instruction (6) would be reached in two states:
// I.  r6{.id=b}, r7{.id=b} via path 1-6;
// II. r6{.id=a}, r7{.id=b} via path 1-4, 6.
//
// Use check_ids() to distinguish these states.
// ---
// Also verify that new value satisfies old value range knowledge.
//
// ADD_CONST flags must match exactly: BPF_ADD_CONST32 and
// BPF_ADD_CONST64 have different linking semantics in
// sync_linked_regs() (alu32 zero-extends, alu64 does not),
// so pruning across different flag types is unsafe.
//
    if (rold.id &&
    (rold.id & BPF_ADD_CONST) != (rcur.id & BPF_ADD_CONST)) {
    return false;
    }
// Both have offset linkage: offsets must match
    if ((rold.id & BPF_ADD_CONST) && rold.delta != rcur.delta) {
    return false;
    }
    if (!check_scalar_ids(rold.id, rcur.id, idmap)) {
    return false;
    }
    return range_within(rold, rcur) && tnum_in(rold.var_off, rcur.var_off);
    case PTR_TO_MAP_KEY:
    case PTR_TO_MAP_VALUE:
    case PTR_TO_MEM:
    case PTR_TO_BUF:
    case PTR_TO_TP_BUFFER:
// If the new min/max/var_off satisfy the old ones and
// everything else matches, we are OK.
//
    return memcmp(rold, rcur, offsetof(bpf_reg_state, var_off)) == 0 &&
    range_within(rold, rcur) &&
    tnum_in(rold.var_off, rcur.var_off) &&
    check_ids(rold.id, rcur.id, idmap) &&
    check_ids(rold.parent_id, rcur.parent_id, idmap);
    case PTR_TO_PACKET_META:
    case PTR_TO_PACKET:
// We must have at least as much range as the old ptr
// did, so that any accesses which were safe before are
// still safe.  This is true even if old range < old off,
// since someone could have accessed through (ptr - k), or
// even done ptr -= k in a register, to get a safe access.
//
    if (rold.range < 0 || rcur.range < 0) {
// special case for [BEYOND|AT]_PKT_END
    if (rold.range != rcur.range) {
    return false;
    }
    } else if (rold.range > rcur.range) {
    return false;
    }
// id relations must be preserved
    if (!check_ids(rold.id, rcur.id, idmap)) {
    return false;
    }
// new val must satisfy old val knowledge
    return range_within(rold, rcur) &&
    tnum_in(rold.var_off, rcur.var_off);
    case PTR_TO_STACK:
// two stack pointers are equal only if they're pointing to
// the same stack frame, since fp-8 in foo != fp-8 in bar
//
    return regs_exact(rold, rcur, idmap) && rold.frameno == rcur.frameno;
    case PTR_TO_ARENA:
    return true;
    case PTR_TO_INSN:
    return memcmp(rold, rcur, offsetof(bpf_reg_state, var_off)) == 0 &&
    range_within(rold, rcur) && tnum_in(rold.var_off, rcur.var_off);
// label;
    return regs_exact(rold, rcur, idmap);
    }
    }
pub static mut unbound_reg: usize = 0;
#[no_mangle]
unsafe extern "C" fn unbound_reg_init() -> __init int {
    bpf_mark_reg_unknown_imprecise(&unbound_reg);
    return 0;
    }
    late_initcall!(unbound_reg_init);
#[no_mangle]
unsafe extern "C" fn is_spilled_scalar_after(stack: *const bpf_stack_state, im: c_int) -> bool {
    return stack.slot_type[im] == STACK_SPILL &&
    stack.spilled_ptr.type == SCALAR_VALUE;
    }
#[no_mangle]
pub unsafe extern "C" fn is_stack_misc_after(env: *mut bpf_verifier_env, stack: *mut bpf_stack_state, im: c_int) -> bool {
    let mut i = 0;
    while (i < ARRAY_SIZE!(stack.slot_type)) {
    if ((stack.slot_type[i] == STACK_MISC) ||
    ((stack.slot_type[i] == STACK_INVALID || stack.slot_type[i] == STACK_POISON) &&
    env.allow_uninit_stack)) {
    continue;
    }
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn scalar_reg_for_stack(env: *mut bpf_verifier_env, stack: *mut bpf_stack_state, im: c_int) -> *mut c_void {
    if (is_spilled_scalar_after(stack, im)) {
    return &stack.spilled_ptr;
    }
    if (is_stack_misc_after(env, stack, im)) {
    return &unbound_reg;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn stacksafe(env: *mut bpf_verifier_env, old: *mut bpf_func_state, cur: *mut bpf_func_state, idmap: *mut bpf_idmap, exact: exact_level) -> bool {
    let mut i = 0;
    let mut spi = 0;
// walk slots of the explored stack and ignore any additional
// slots in the current stack, since explored(safe) state
// didn't use them
//
    while (i < old.allocated_stack) {
    let mut old_reg = core::ptr::null_mut();
    let mut cur_reg = core::ptr::null_mut();
pub static mut im: c_int = 0;
    spi = i / BPF_REG_SIZE;
    if (exact == EXACT) {
pub static mut old_type: u8 = 0;
    u8 cur_type = i < cur.allocated_stack ?
    cur.stack[spi].slot_type[i % BPF_REG_SIZE] : STACK_INVALID;
// STACK_INVALID and STACK_POISON are equivalent for pruning
    if (old_type == STACK_POISON) {
    old_type = STACK_INVALID;
    }
    if (cur_type == STACK_POISON) {
    cur_type = STACK_INVALID;
    }
    if (i >= cur.allocated_stack || old_type != cur_type) {
    return false;
    }
    }
    if (old.stack[spi].slot_type[i % BPF_REG_SIZE] == STACK_INVALID ||
    old.stack[spi].slot_type[i % BPF_REG_SIZE] == STACK_POISON) {
    continue;
    }
    if (env.allow_uninit_stack &&
    old.stack[spi].slot_type[i % BPF_REG_SIZE] == STACK_MISC) {
    continue;
    }
// explored stack has more populated slots than current stack
// and these slots were used
//
    if (i >= cur.allocated_stack) {
    return false;
    }
//
// 64 and 32-bit scalar spills vs MISC/INVALID slots and vice versa.
// Load from MISC/INVALID slots produces unbound scalar.
// Construct a fake register for such stack and call
// regsafe() to ensure scalar ids are compared.
//
    if (im == 0 || im == 4) {
    old_reg = scalar_reg_for_stack(env, &old.stack[spi], im);
    cur_reg = scalar_reg_for_stack(env, &cur.stack[spi], im);
    if (old_reg && cur_reg) {
    if (!regsafe(env, old_reg, cur_reg, idmap, exact)) {
    return false;
    }
    i += (im == 0 ? BPF_REG_SIZE - 1 : 3);
    continue;
    }
    }
// if old state was safe with misc data in the stack
// it will be safe with zero-initialized stack.
// The opposite is not true
//
    if (old.stack[spi].slot_type[i % BPF_REG_SIZE] == STACK_MISC &&
    cur.stack[spi].slot_type[i % BPF_REG_SIZE] == STACK_ZERO) {
    continue;
    }
    if (old.stack[spi].slot_type[i % BPF_REG_SIZE] !=
    cur.stack[spi].slot_type[i % BPF_REG_SIZE]) {
// Ex: old explored (safe) state has STACK_SPILL in
// this stack slot, but current has STACK_MISC ->
// this verifier states are not equivalent,
// return false to continue verification of this path
//
    return false;
    }
    if (i % BPF_REG_SIZE != BPF_REG_SIZE - 1) {
    continue;
    }
// Both old and cur are having same slot_type
    match (old.stack[spi].slot_type[BPF_REG_SIZE - 1]) {
    STACK_SPILL => {
// when explored and current stack slot are both storing
// spilled registers, check that stored pointers types
// are the same as well.
// Ex: explored safe path could have stored
// (bpf_reg_state) {.type = PTR_TO_STACK, .off = -8}
// but current path has stored:
// (bpf_reg_state) {.type = PTR_TO_STACK, .off = -16}
// such verifier states are not equivalent.
// return false to continue verification of this path
//
    if (!regsafe(env, &old.stack[spi].spilled_ptr,
    &cur.stack[spi].spilled_ptr, idmap, exact)) {
    return false;
    }
    // break;
    }
    STACK_DYNPTR => {
    old_reg = &old.stack[spi].spilled_ptr;
    cur_reg = &cur.stack[spi].spilled_ptr;
    if (old_reg.dynptr.type != cur_reg.dynptr.type ||
    old_reg.dynptr.first_slot != cur_reg.dynptr.first_slot ||
    !check_ids(old_reg.id, cur_reg.id, idmap) ||
    !check_ids(old_reg.parent_id, cur_reg.parent_id, idmap)) {
    return false;
    }
    // break;
    }
    STACK_ITER => {
    old_reg = &old.stack[spi].spilled_ptr;
    cur_reg = &cur.stack[spi].spilled_ptr;
// iter.depth is not compared between states as it
// doesn't matter for correctness and would otherwise
// prevent convergence; we maintain it only to prevent
// infinite loop check triggering, see
// iter_active_depths_differ()
//
    if (old_reg.type != cur_reg.type ||
    old_reg.iter.btf != cur_reg.iter.btf ||
    old_reg.iter.btf_id != cur_reg.iter.btf_id ||
    old_reg.iter.state != cur_reg.iter.state ||
// ignore {old_reg,cur_reg}->iter.depth, see above
    !check_ids(old_reg.id, cur_reg.id, idmap)) {
    return false;
    }
    // break;
    }
    STACK_IRQ_FLAG => {
    old_reg = &old.stack[spi].spilled_ptr;
    cur_reg = &cur.stack[spi].spilled_ptr;
    if (!check_ids(old_reg.id, cur_reg.id, idmap) ||
    old_reg.irq.kfunc_class != cur_reg.irq.kfunc_class) {
    return false;
    }
    // break;
    }
    STACK_MISC => {
    }
    STACK_ZERO => {
    }
    STACK_INVALID => {
    }
    STACK_POISON => {
    continue;
// Ensure that new unhandled slot types return false by default
    }
    _ => {
    return false;
    }
    }
    }
    return true;
    }
//
// Compare stack arg slots between old and current states.
// Outgoing stack args are path-local state and must agree for pruning.
//
#[no_mangle]
pub unsafe extern "C" fn stack_arg_safe(env: *mut bpf_verifier_env, old: *mut bpf_func_state, cur: *mut bpf_func_state, idmap: *mut bpf_idmap, exact: exact_level) -> bool {
    let mut i = 0;
    let mut nslots = 0;
    nslots = max(old.out_stack_arg_cnt, cur.out_stack_arg_cnt);
    while (i < nslots) {
    let mut old_arg = core::ptr::null_mut();
    let mut cur_arg = core::ptr::null_mut();
pub static mut not_init: bpf_reg_state = 0;
    old_arg = i < old.out_stack_arg_cnt ?
    &old.stack_arg_regs[i] : &not_init;
    cur_arg = i < cur.out_stack_arg_cnt ?
    &cur.stack_arg_regs[i] : &not_init;
    if (!regsafe(env, old_arg, cur_arg, idmap, exact)) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn refsafe(old: *mut bpf_verifier_state, cur: *mut bpf_verifier_state, idmap: *mut bpf_idmap) -> bool {
    let mut i = 0;
    if (old.acquired_refs != cur.acquired_refs) {
    return false;
    }
    if (old.active_locks != cur.active_locks) {
    return false;
    }
    if (old.active_preempt_locks != cur.active_preempt_locks) {
    return false;
    }
    if (old.active_rcu_locks != cur.active_rcu_locks) {
    return false;
    }
    if (!check_ids(old.active_irq_id, cur.active_irq_id, idmap)) {
    return false;
    }
    if (!check_ids(old.active_lock_id, cur.active_lock_id, idmap) ||
    old.active_lock_ptr != cur.active_lock_ptr) {
    return false;
    }
    while (i < old.acquired_refs) {
    if (!check_ids(old.refs[i].id, cur.refs[i].id, idmap) ||
    old.refs[i].type != cur.refs[i].type) {
    return false;
    }
    match (old.refs[i].type) {
    REF_TYPE_PTR => {
    if (!check_ids(old.refs[i].parent_id, cur.refs[i].parent_id, idmap)) {
    return false;
    }
    // break;
    }
    REF_TYPE_IRQ => {
    // break;
    }
    REF_TYPE_LOCK => {
    }
    REF_TYPE_RES_LOCK => {
    }
    REF_TYPE_RES_LOCK_IRQ => {
    if (old.refs[i].ptr != cur.refs[i].ptr) {
    return false;
    }
    // break;
    }
    _ => {
    WARN_ONCE(1, "Unhandled enum type for reference state: %d\n", old.refs[i].type);
    return false;
    }
    }
    }
    return true;
    }
// compare two verifier states
//
// all states stored in state_list are known to be valid, since
// verifier reached 'bpf_exit' instruction through them
//
// this function is called when verifier exploring different branches of
// execution popped from the state stack. If it sees an old state that has
// more strict register state and more strict stack state then this execution
// branch doesn't need to be explored further, since verifier already
// concluded that more strict state leads to valid finish.
//
// Therefore two states are equivalent if register state is more conservative
// and explored stack state is more conservative than the current one.
// Example:
// explored                   current
// (slot1=INV slot2=MISC) == (slot1=MISC slot2=MISC)
// (slot1=MISC slot2=MISC) != (slot1=INV slot2=MISC)
//
// In other words if current stack state (one being explored) has more
// valid slots than old one that already passed validation, it means
// the verifier can stop exploring and conclude that current state is valid too
//
// Similarly with registers. If explored state has register type as invalid
// whereas register type in current state is meaningful, it means that
// the current state will reach 'bpf_exit' instruction safely
//
#[no_mangle]
pub unsafe extern "C" fn func_states_equal(env: *mut bpf_verifier_env, old: *mut bpf_func_state, cur: *mut bpf_func_state, insn_idx: u32, exact: exact_level) -> bool {
pub static mut live_regs: u16 = 0;
    let mut i = 0;
    if (old.callback_depth > cur.callback_depth) {
    return false;
    }
    if (!old.no_stack_arg_load && cur.no_stack_arg_load) {
    return false;
    }
    for (i = 0; i < MAX_BPF_REG; i++) {
    if (((1 << i) & live_regs) &&
    !regsafe(env, &old.regs[i], &cur.regs[i],
    &env.idmap_scratch, exact))
    return false;
    }
    if (!stacksafe(env, old, cur, &env.idmap_scratch, exact)) {
    return false;
    }
    if (!stack_arg_safe(env, old, cur, &env.idmap_scratch, exact)) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn reset_idmap_scratch(env: *mut bpf_verifier_env) {
    let mut idmap = &env.idmap_scratch;
    idmap.tmp_id_gen = env.id_gen;
    idmap.cnt = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn states_equal(env: *mut bpf_verifier_env, old: *mut bpf_verifier_state, cur: *mut bpf_verifier_state, exact: exact_level) -> bool {
    let mut insn_idx = 0;
    let mut i = 0;
    if (old.curframe != cur.curframe) {
    return false;
    }
    reset_idmap_scratch(env);
// Verification state from speculative execution simulation
// must never prune a non-speculative execution one.
//
    if (old.speculative && !cur.speculative) {
    return false;
    }
    if (old.in_sleepable != cur.in_sleepable) {
    return false;
    }
    if (!refsafe(old, cur, &env.idmap_scratch)) {
    return false;
    }
// for states to be equal callsites have to be the same
// and all frame states need to be equivalent
//
    while (i <= old.curframe) {
    insn_idx = bpf_frame_insn_idx(old, i);
    if (old.frame[i].callsite != cur.frame[i].callsite) {
    return false;
    }
    if (!func_states_equal(env, old.frame[i], cur.frame[i], insn_idx, exact)) {
    return false;
    }
    }
    return true;
    }
// find precise scalars in the previous equivalent state and
// propagate them into the current state
//
#[no_mangle]
pub unsafe extern "C" fn propagate_precision(env: *mut bpf_verifier_env, old: *mut bpf_verifier_state, cur: *mut bpf_verifier_state, changed: *mut bool) -> c_int {
pub static mut state_reg: *mut c_void = core::ptr::null_mut();
pub static mut state: *mut c_void = core::ptr::null_mut();
    int i, err = 0, fr;
    let mut first = 0;
    while (fr >= 0) {
    state = old.frame[fr];
    state_reg = state.regs;
    first = true;
    while (i < BPF_REG_FP) {
    if (state_reg.type != SCALAR_VALUE ||
    !state_reg.precise) {
    continue;
    }
    if (env.log.level & BPF_LOG_LEVEL2) {
    if (first) {
    verbose(env, "frame %d: propagating r%d", fr, i);
    }
    else {
    verbose(env, ",r%d", i);
    }
    }
    bpf_bt_set_frame_reg(&env.bt, fr, i);
    first = false;
    }
    while (i < state.allocated_stack / BPF_REG_SIZE) {
    if (!bpf_is_spilled_reg(&state.stack[i])) {
    continue;
    }
    state_reg = &state.stack[i].spilled_ptr;
    if (state_reg.type != SCALAR_VALUE ||
    !state_reg.precise) {
    continue;
    }
    if (env.log.level & BPF_LOG_LEVEL2) {
    if (first) {
    verbose(env, "frame %d: propagating fp%d",
    fr, (-i - 1) * BPF_REG_SIZE);
    }
    else {
    verbose(env, ",fp%d", (-i - 1) * BPF_REG_SIZE);
    }
    }
    bpf_bt_set_frame_slot(&env.bt, fr, i);
    first = false;
    }
    if (!first && (env.log.level & BPF_LOG_LEVEL2)) {
    verbose(env, "\n");
    }
    }
    err = bpf_mark_chain_precision(env, cur, -1, changed);
    if (err < 0) {
    return err;
    }
    return 0;
    }
pub const MAX_BACKEDGE_ITERS: c_int = 64;
// Propagate read and precision marks from visit->backedges[*].state->equal_state
// to corresponding parent states of visit->backedges[*].state until fixed point is reached,
// then free visit->backedges.
// After execution of this function incomplete_read_marks() will return false
// for all states corresponding to @visit->callchain.
//
#[no_mangle]
unsafe extern "C" fn propagate_backedges(env: *mut bpf_verifier_env, visit: *mut bpf_scc_visit) -> c_int {
pub static mut backedge: *mut c_void = core::ptr::null_mut();
pub static mut st: *mut c_void = core::ptr::null_mut();
    let mut changed = 0;
    let mut i = 0;
    let mut err = 0;
    i = 0;
    do {
    if (i++ > MAX_BACKEDGE_ITERS) {
    if (env.log.level & BPF_LOG_LEVEL2) {
    verbose(env, "%s: too many iterations\n", __func__);
    }
    for (backedge = visit.backedges; backedge; backedge = backedge.next) {
    bpf_mark_all_scalars_precise(env, &backedge.state);
    }
    break;
    }
    changed = false;
    while (backedge) {
    st = &backedge.state;
    err = propagate_precision(env, st.equal_state, st, &changed);
    if (err) {
    return err;
    }
    }
    } while (changed);
    bpf_free_backedges(visit);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn states_maybe_looping(old: *mut bpf_verifier_state, cur: *mut bpf_verifier_state) -> bool {
    let mut fold = core::ptr::null_mut();
    let mut fcur = core::ptr::null_mut();
    int i, fr = cur.curframe;
    if (old.curframe != fr) {
    return false;
    }
    fold = old.frame[fr];
    fcur = cur.frame[fr];
    for (i = 0; i < MAX_BPF_REG; i++) {
    if (memcmp(&fold.regs[i], &fcur.regs[i],
    offsetof(bpf_reg_state, frameno)))
    return false;
    }
    return true;
    }
// is_state_visited() handles iter_next() (see process_iter_next_call() for
// terminology) calls specially: as opposed to bounded BPF loops, it *expects
// states to match, which otherwise would look like an infinite loop. So while
// iter_next() calls are taken care of, we still need to be careful and
// prevent erroneous and too eager declaration of "infinite loop", when
// iterators are involved.
//
// Here's a situation in pseudo-BPF assembly form:
//
// 0: again:                          ; set up iter_next() call args
// 1:   r1 = &it                      ; <CHECKPOINT HERE>
// 2:   call bpf_iter_num_next        ; this is iter_next() call
// 3:   if r0 == 0 goto done
// 4:   ... something useful here ...
// 5:   goto again                    ; another iteration
// 6: done:
// 7:   r1 = &it
// 8:   call bpf_iter_num_destroy     ; clean up iter state
// 9:   exit
//
// This is a typical loop. Let's assume that we have a prune point at 1:,
// before we get to `call bpf_iter_num_next` (e.g., because of that `goto
// again`, assuming other heuristics don't get in a way).
//
// When we first time come to 1:, let's say we have some state X. We proceed
// to 2:, fork states, enqueue ACTIVE, validate NULL case successfully, exit.
// Now we come back to validate that forked ACTIVE state. We proceed through
// 3-5, come to goto, jump to 1:. Let's assume our state didn't change, so we
// are converging. But the problem is that we don't know that yet, as this
// convergence has to happen at iter_next() call site only. So if nothing is
// done, at 1: verifier will use bounded loop logic and declare infinite
// looping (and would be *technically* correct, if not for iterator's
// "eventual sticky NULL" contract, see process_iter_next_call()). But we
// don't want that. So what we do in process_iter_next_call() when we go on
// another ACTIVE iteration, we bump slot->iter.depth, to mark that it's
// a different iteration. So when we suspect an infinite loop, we additionally
// check if any of the *ACTIVE* iterator states depths differ. If yes, we
// pretend we are not looping and wait for next iter_next() call.
//
// This only applies to ACTIVE state. In DRAINED state we don't expect to
// loop, because that would actually mean infinite loop, as DRAINED state is
// "sticky", and so we'll keep returning into the same instruction with the
// same state (at least in one of possible code paths).
//
// This approach allows to keep infinite loop heuristic even in the face of
// active iterator. E.g., C snippet below is and will be detected as
// infinitely looping:
//
// struct bpf_iter_num it;
// int *p, x;
//
// bpf_iter_num_new(&it, 0, 10);
// while ((p = bpf_iter_num_next(&t))) {
// x = p;
// while (x--) {} // <<-- infinite loop here
// }
//
#[no_mangle]
unsafe extern "C" fn iter_active_depths_differ(old: *mut bpf_verifier_state, cur: *mut bpf_verifier_state) -> bool {
    let mut slot = core::ptr::null_mut();
    let mut cur_slot = core::ptr::null_mut();
pub static mut state: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut fr = 0;
    while (fr >= 0) {
    state = old.frame[fr];
    while (i < state.allocated_stack / BPF_REG_SIZE) {
    if (state.stack[i].slot_type[0] != STACK_ITER) {
    continue;
    }
    slot = &state.stack[i].spilled_ptr;
    if (slot.iter.state != BPF_ITER_STATE_ACTIVE) {
    continue;
    }
    cur_slot = &cur.frame[fr].stack[i].spilled_ptr;
    if (cur_slot.iter.depth != slot.iter.depth) {
    return true;
    }
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mark_all_scalars_imprecise(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) {
pub static mut func: *mut c_void = core::ptr::null_mut();
pub static mut reg: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    while (i <= st.curframe) {
    func = st.frame[i];
    while (j < BPF_REG_FP) {
    reg = &func.regs[j];
    if (reg.type != SCALAR_VALUE) {
    continue;
    }
    reg.precise = false;
    }
    while (j < func.allocated_stack / BPF_REG_SIZE) {
    if (!bpf_is_spilled_reg(&func.stack[j])) {
    continue;
    }
    reg = &func.stack[j].spilled_ptr;
    if (reg.type != SCALAR_VALUE) {
    continue;
    }
    reg.precise = false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_is_state_visited(env: *mut bpf_verifier_env, insn_idx: c_int) -> c_int {
pub static mut new_sl: *mut c_void = core::ptr::null_mut();
pub static mut sl: *mut c_void = core::ptr::null_mut();
    let mut cur = env.cur_state, *new;
    let mut force_new_state = 0;
    let mut add_new_state = 0;
    let mut loop = 0;
    int n, err, states_cnt = 0;
    let mut pos = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    force_new_state = env.test_state_freq || bpf_is_force_checkpoint(env, insn_idx) ||
// Avoid accumulating infinitely long jmp history
    cur.jmp_history_cnt > 40;
// bpf progs typically have pruning point every 4 instructions
// http://vger.kernel.org/bpfconf2019.html#session-1
// Do not add new state for future pruning if the verifier hasn't seen
// at least 2 jumps and at least 8 instructions.
// This heuristics helps decrease 'total_states' and 'peak_states' metric.
// In tests that amounts to up to 50% reduction into total verifier
// memory consumption and 20% verifier time speedup.
//
    add_new_state = force_new_state;
    if (env.jmps_processed - env.prev_jmps_processed >= 2 &&
    env.insn_processed - env.prev_insn_processed >= 8) {
    add_new_state = true;
    }
// keep cleaning the current state as registers/stack become dead
    err = clean_verifier_state(env, cur);
    if (err) {
    return err;
    }
    loop = false;
    head = bpf_explored_state(env, insn_idx);
    list_for_each_safe(pos, tmp, head) {
    sl = container_of!(pos, bpf_verifier_state_list, node);
    states_cnt += 1;
    if (sl.state.insn_idx != insn_idx) {
    continue;
    }
    if (sl.state.branches) {
    let mut frame = sl.state.frame[sl.state.curframe];
    if (frame.in_async_callback_fn &&
    frame.async_entry_cnt != cur.frame[cur.curframe].async_entry_cnt) {
// Different async_entry_cnt means that the verifier is
// processing another entry into async callback.
// Seeing the same state is not an indication of infinite
// loop or infinite recursion.
// But finding the same state doesn't mean that it's safe
// to stop processing the current state. The previous state
// hasn't yet reached bpf_exit, since state.branches > 0.
// Checking in_async_callback_fn alone is not enough either.
// Since the verifier still needs to catch infinite loops
// inside async callbacks.
//
// goto;
    }
// BPF open-coded iterators loop detection is special.
// states_maybe_looping() logic is too simplistic in detecting
// states that *might* be equivalent, because it doesn't know
// about ID remapping, so don't even perform it.
// See process_iter_next_call() and iter_active_depths_differ()
// for overview of the logic. When current and one of parent
// states are detected as equivalent, it's a good thing: we prove
// convergence and can stop simulating further iterations.
// It's safe to assume that iterator loop will finish, taking into
// account iter_next() contract of eventually returning
// sticky NULL result.
//
// Note, that states have to be compared exactly in this case because
// read and precision marks might not be finalized inside the loop.
// E.g. as in the program below:
//
// 1. r7 = -16
// 2. r6 = bpf_get_prandom_u32()
// 3. while (bpf_iter_num_next(&fp[-8])) {
// 4.   if (r6 != 42) {
// 5.     r7 = -32
// 6.     r6 = bpf_get_prandom_u32()
// 7.     continue
// 8.   }
// 9.   r0 = r10
// 10.   r0 += r7
// 11.   r8 = *(r0 + 0)
// 12.   r6 = bpf_get_prandom_u32()
// 13. }
//
// Here verifier would first visit path 1-3, create a checkpoint at 3
// with r7=-16, continue to 4-7,3. Existing checkpoint at 3 does
// not have read or precision mark for r7 yet, thus inexact states
// comparison would discard current state with r7=-32
// => unsafe memory access at 11 would not be caught.
//
    if (is_iter_next_insn(env, insn_idx)) {
    if (states_equal(env, &sl.state, cur, RANGE_WITHIN)) {
pub static mut cur_frame: *mut c_void = core::ptr::null_mut();
    let mut iter_state = core::ptr::null_mut();
    let mut iter_reg = core::ptr::null_mut();
    let mut spi = 0;
    cur_frame = cur.frame[cur.curframe];
// btf_check_iter_kfuncs() enforces that
// iter state pointer is always the first arg
//
    iter_reg = &cur_frame.regs[BPF_REG_1];
// current state is valid due to states_equal(),
// so we can assume valid iter and reg state,
// no need for extra (re-)validations
//
    spi = bpf_get_spi(iter_reg.var_off.value);
    iter_state = &bpf_func(env, iter_reg).stack[spi].spilled_ptr;
    if (iter_state.iter.state == BPF_ITER_STATE_ACTIVE) {
    loop = true;
// goto;
    }
    }
// goto;
    }
    if (is_may_goto_insn_at(env, insn_idx)) {
    if (sl.state.may_goto_depth != cur.may_goto_depth &&
    states_equal(env, &sl.state, cur, RANGE_WITHIN)) {
    loop = true;
// goto;
    }
    }
    if (bpf_calls_callback(env, insn_idx)) {
    if (states_equal(env, &sl.state, cur, RANGE_WITHIN)) {
    loop = true;
// goto;
    }
// goto;
    }
// attempt to detect infinite loop to avoid unnecessary doomed work
    if (states_maybe_looping(&sl.state, cur) &&
    states_equal(env, &sl.state, cur, EXACT) &&
    !iter_active_depths_differ(&sl.state, cur) &&
    sl.state.may_goto_depth == cur.may_goto_depth &&
    sl.state.callback_unroll_depth == cur.callback_unroll_depth) {
    verbose_linfo(env, insn_idx, "; ");
    verbose(env, "infinite loop detected at insn %d\n", insn_idx);
    verbose(env, "cur state:");
    print_verifier_state(env, cur, cur.curframe, true);
    verbose(env, "old state:");
    print_verifier_state(env, &sl.state, cur.curframe, true);
    return -EINVAL;
    }
// if the verifier is processing a loop, avoid adding new state
// too often, since different loop iterations have distinct
// states and may not help future pruning.
// This threshold shouldn't be too low to make sure that
// a loop with large bound will be rejected quickly.
// The most abusive loop will be:
// r1 += 1
// if r1 < 1000000 goto pc-2
// 1M insn_procssed limit / 100 == 10k peak states.
// This threshold shouldn't be too high either, since states
// at the end of the loop are likely to be useful in pruning.
//
// label;
    if (!force_new_state &&
    env.jmps_processed - env.prev_jmps_processed < 20 &&
    env.insn_processed - env.prev_insn_processed < 100) {
    add_new_state = false;
    }
// goto;
    }
// See comments for mark_all_regs_read_and_precise()
    loop = incomplete_read_marks(env, &sl.state);
    if (states_equal(env, &sl.state, cur, loop ? RANGE_WITHIN : NOT_EXACT)) {
// label;
    sl.hit_cnt += 1;
// if previous state reached the exit with precision and
// current state is equivalent to it (except precision marks)
// the precision needs to be propagated back in
// the current state.
//
    err = 0;
    if (bpf_is_jmp_point(env, env.insn_idx)) {
    err = bpf_push_jmp_history(env, cur, 0, 0, 0, 0);
    }
    err = err ? : propagate_precision(env, &sl.state, cur, core::ptr::null_mut());
    if (err) {
    return err;
    }
// When processing iterator based loops above propagate_liveness and
// propagate_precision calls are not sufficient to transfer all relevant
// read and precision marks. E.g. consider the following case:
//
// .-> A --.  Assume the states are visited in the order A, B, C.
// |   |   |  Assume that state B reaches a state equivalent to state A.
// |   v   v  At this point, state C is not processed yet, so state A
// '-- B   C  has not received any read or precision marks from C.
// Thus, marks propagated from A to B are incomplete.
//
// The verifier mitigates this by performing the following steps:
//
// - Prior to the main verification pass, strongly connected components
// (SCCs) are computed over the program's control flow graph,
// intraprocedurally.
//
// - During the main verification pass, `maybe_enter_scc()` checks
// whether the current verifier state is entering an SCC. If so, an
// instance of a `bpf_scc_visit` object is created, and the state
// entering the SCC is recorded as the entry state.
//
// - This instance is associated not with the SCC itself, but with a
// `bpf_scc_callchain`: a tuple consisting of the call sites leading to
// the SCC and the SCC id. See `compute_scc_callchain()`.
//
// - When a verification path encounters a `states_equal(...,
// RANGE_WITHIN)` condition, there exists a call chain describing the
// current state and a corresponding `bpf_scc_visit` instance. A copy
// of the current state is created and added to
// `bpf_scc_visit->backedges`.
//
// - When a verification path terminates, `maybe_exit_scc()` is called
// from `bpf_update_branch_counts()`. For states with `branches == 0`, it
// checks whether the state is the entry state of any `bpf_scc_visit`
// instance. If it is, this indicates that all paths originating from
// this SCC visit have been explored. `propagate_backedges()` is then
// called, which propagates read and precision marks through the
// backedges until a fixed point is reached.
// (In the earlier example, this would propagate marks from A to B,
from C to A, and then again from A to B.)
//
// A note on callchains
// --------------------
//
// Consider the following example:
//
// void foo() { loop { ... SCC#1 ... } }
// void main() {
// A: foo();
// B: ...
// C: foo();
// }
//
// Here, there are two distinct callchains leading to SCC#1:
// - (A, SCC#1)
// - (C, SCC#1)
//
// Each callchain identifies a separate `bpf_scc_visit` instance that
// accumulates backedge states. The `propagate_{liveness,precision}()`
// functions traverse the parent state of each backedge state, which
// means these parent states must remain valid (i.e., not freed) while
// the corresponding `bpf_scc_visit` instance exists.
//
// Associating `bpf_scc_visit` instances directly with SCCs instead of
// callchains would break this invariant:
// - States explored during `C: foo()` would contribute backedges to
// SCC#1, but SCC#1 would only be exited once the exploration of
// `A: foo()` completes.
// - By that time, the states explored between `A: foo()` and `C: foo()`
// (i.e., `B: ...`) may have already been freed, causing the parent
// links for states from `C: foo()` to become invalid.
//
    if (loop) {
pub static mut backedge: *mut c_void = core::ptr::null_mut();
    backedge = kzalloc_obj(*backedge,
    GFP_KERNEL_ACCOUNT);
    if (!backedge) {
    return -ENOMEM;
    }
    err = bpf_copy_verifier_state(&backedge.state, cur);
    backedge.state.equal_state = &sl.state;
    backedge.state.insn_idx = insn_idx;
    err = err ?: add_scc_backedge(env, &sl.state, backedge);
    if (err) {
    bpf_free_verifier_state(&backedge.state, false);
    kfree(backedge);
    return err;
    }
    }
    return 1;
    }
// label;
// when new state is not going to be added do not increase miss count.
// Otherwise several loop iterations will remove the state
// recorded earlier. The goal of these heuristics is to have
// states from some iterations of the loop (some in the beginning
// and some at the end) to help pruning.
//
    if (add_new_state) {
    sl.miss_cnt += 1;
    }
// heuristic to determine whether this state is beneficial
// to keep checking from state equivalence point of view.
// Higher numbers increase max_states_per_insn and verification time,
// but do not meaningfully decrease insn_processed.
// 'n' controls how many times state could miss before eviction.
// Use bigger 'n' for checkpoints because evicting checkpoint states
// too early would hinder iterator convergence.
//
    n = bpf_is_force_checkpoint(env, insn_idx) && sl.state.branches > 0 ? 64 : 3;
    if (sl.miss_cnt > sl.hit_cnt * n + n) {
// the state is unlikely to be useful. Remove it to
// speed up verification
//
    sl.in_free_list = true;
    list_del(&sl.node);
    list_add(&sl.node, &env.free_list);
    env.free_list_size += 1;
    env.explored_states_size -= 1;
    maybe_free_verifier_state(env, sl);
    }
    }
    if (env.max_states_per_insn < states_cnt) {
    env.max_states_per_insn = states_cnt;
    }
    if (!env.bpf_capable && states_cnt > BPF_COMPLEXITY_LIMIT_STATES) {
    return 0;
    }
    if (!add_new_state) {
    return 0;
    }
// There were no equivalent states, remember the current one.
// Technically the current state is not proven to be safe yet,
but it will either reach outer most bpf_exit (which means it's safe)
// or it will be rejected. When there are no loops the verifier won't be
// seeing this tuple (frame[0].callsite, frame[1].callsite, .. insn_idx)
// again on the way to bpf_exit.
// When looping the sl->state.branches will be > 0 and this state
// will not be considered for equivalence until branches == 0.
//
    new_sl = kzalloc_obj(bpf_verifier_state_list, GFP_KERNEL_ACCOUNT);
    if (!new_sl) {
    return -ENOMEM;
    }
    env.total_states += 1;
    env.explored_states_size += 1;
    update_peak_states(env);
    env.prev_jmps_processed = env.jmps_processed;
    env.prev_insn_processed = env.insn_processed;
// forget precise markings we inherited, see __mark_chain_precision
    if (env.bpf_capable) {
    mark_all_scalars_imprecise(env, cur);
    }
    bpf_clear_singular_ids(env, cur);
// add new state to the head of linked list
    new = &new_sl.state;
    err = bpf_copy_verifier_state(new, cur);
    if (err) {
    bpf_free_verifier_state(new, false);
    kfree(new_sl);
    return err;
    }
    new.insn_idx = insn_idx;
    verifier_bug_if(new.branches != 1, env,
    "%s:branches_to_explore=%d insn %d",
    __func__, new.branches, insn_idx);
    err = maybe_enter_scc(env, new);
    if (err) {
    bpf_free_verifier_state(new, false);
    kfree(new_sl);
    return err;
    }
    cur.parent = new;
    cur.first_insn_idx = insn_idx;
    cur.dfs_depth = new.dfs_depth + 1;
    bpf_clear_jmp_history(cur);
    list_add(&new_sl.node, head);
    return 0;
    }