//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_uprobe.c
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


// SPDX-License-Identifier: GPL-2.0
//
// uprobes-based tracing events
//
// Copyright (C) IBM Corporation, 2010-2012
// Author:	Srikar Dronamraju <srikar@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_trace_entry_head {
    pub ent: trace_entry,
    pub vaddr: [c_ulong; 0],
}

    (sizeof!(uprobe_trace_entry_head) +	
    sizeof!(unsigned long) * (is_return ? 2 : 1))

    ((void*)(entry) + SIZEOF_TRACE_ENTRY(is_return))
// forward_decl: trace_uprobe_create;
// forward_decl: trace_uprobe_show;
// forward_decl: trace_uprobe_release;
// forward_decl: trace_uprobe_is_busy;
// forward_decl: trace_uprobe_match;
pub static mut dyn_event_operations: usize = 0;
//
// uprobe event core functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_uprobe {
    pub devent: dyn_event,
    pub consumer: uprobe_consumer,
    pub path: path,
    pub filename: *mut c_char,
    pub uprobe: *mut uprobe,
    pub offset: c_ulong,
    pub ref_ctr_offset: c_ulong,
    pub nhits: *mut unsigned long ,
    pub tp: trace_probe,
}

#[no_mangle]
unsafe extern "C" fn is_trace_uprobe(ev: *mut dyn_event) -> bool {
    return ev.ops == &trace_uprobe_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn to_trace_uprobe(ev: *mut dyn_event) -> *mut c_void {
    return container_of!(ev, trace_uprobe, devent);
    }
//
// for_each_trace_uprobe - iterate over the trace_uprobe list
// @pos:	the struct trace_uprobe * for each entry
// @dpos:	the struct dyn_event * to use as a loop cursor
//

    for_each_dyn_event(dpos)		 {
    if (is_trace_uprobe(dpos) && (pos = to_trace_uprobe(dpos)))
// forward_decl: register_uprobe_event;
    }
// forward_decl: unregister_uprobe_event;
// forward_decl: uprobe_dispatcher;
// forward_decl: uretprobe_dispatcher;

#[no_mangle]
unsafe extern "C" fn adjust_stack_addr(addr: c_ulong, n: c_uint) -> c_ulong {
    return addr - (n * sizeof!(long));
    }

#[no_mangle]
unsafe extern "C" fn adjust_stack_addr(addr: c_ulong, n: c_uint) -> c_ulong {
    return addr + (n * sizeof!(long));
    }

#[no_mangle]
unsafe extern "C" fn get_user_stack_nth(regs: *mut pt_regs, n: c_uint) -> c_ulong {
    let mut ret = 0;
pub static mut addr: c_ulong = 0;
    addr = adjust_stack_addr(addr, n);
    if (copy_from_user(&ret,  addr, sizeof!(ret))) {
    return 0;
    }
    return ret;
    }
//
// Uprobes-specific fetch functions
//
    static nokprobe_inline int
    probe_mem_read(void *dest, void *src, size_t size)
    {
    let mut vaddr = src;
    return copy_from_user(dest, vaddr, size) ? -EFAULT : 0;
    }
    static nokprobe_inline int
    probe_mem_read_user(void *dest, void *src, size_t size)
    {
    return probe_mem_read(dest, src, size);
    }
//
// Fetch a null-terminated string. Caller MUST set *dest with max
// length and relative data location.
//
    static nokprobe_inline int
    fetch_store_string(unsigned long addr, void *dest, void *base)
    {
    let mut ret = 0;
pub static mut loc: u32 = 0;
pub static mut maxlen: c_int = 0;
    let mut dst = get_loc_data(dest, base);
    let mut src =  addr;
    if (unlikely(!maxlen)) {
    return -ENOMEM;
    }
    if (addr == FETCH_TOKEN_COMM) {
    ret = strscpy(dst, current.comm, maxlen);
    }
    else {
    ret = strncpy_from_user(dst, src, maxlen);
    }
    if (ret >= 0) {
    if (ret == maxlen) {
    dst[ret - 1] = '\0';
    }
    else {
//
// Include the terminating null byte. In this case it
// was copied by strncpy_from_user but not accounted
// for in ret.
//
    ret += 1;
    }
// dest = make_data_loc(ret, dst - base);
    } else {
// dest = make_data_loc(0, dst - base);
    }
    return ret;
    }
    static nokprobe_inline int
    fetch_store_string_user(unsigned long addr, void *dest, void *base)
    {
    return fetch_store_string(addr, dest, base);
    }
// Return the length of string -- including null terminal byte
    static nokprobe_inline int
    fetch_store_strlen(unsigned long addr)
    {
    let mut len = 0;
    let mut vaddr =  addr;
    if (addr == FETCH_TOKEN_COMM) {
    len = strlen(current.comm) + 1;
    }
    else {
    len = strnlen_user(vaddr, MAX_STRING_SIZE);
    }
    return (len > MAX_STRING_SIZE) ? 0 : len;
    }
    static nokprobe_inline int
    fetch_store_strlen_user(unsigned long addr)
    {
    return fetch_store_strlen(addr);
    }
#[no_mangle]
unsafe extern "C" fn translate_user_vaddr(file_offset: c_ulong) -> c_ulong {
    let mut base_addr = 0;
pub static mut udd: *mut c_void = core::ptr::null_mut();
    udd =  current.utask.vaddr;
    base_addr = udd.bp_addr - udd.tu.offset;
    return base_addr + file_offset;
    }
// Note that we don't verify it, since the code does not come from user space
#[no_mangle]
pub unsafe extern "C" fn process_fetch_insn(code: *mut fetch_insn, rec: *mut c_void, edata: *mut c_void, dest: *mut c_void, base: *mut c_void) -> c_int {
    let mut regs = rec;
    let mut val = 0;
    let mut ret = 0;
// 1st stage: get value from context
    match (code.op) {
    FETCH_OP_REG => {
    val = regs_get_register(regs, code.param);
    // break;
    }
    FETCH_OP_STACK => {
    val = get_user_stack_nth(regs, code.param);
    // break;
    }
    FETCH_OP_STACKP => {
    val = user_stack_pointer(regs);
    // break;
    }
    FETCH_OP_RETVAL => {
    val = regs_return_value(regs);
    // break;
    }
    FETCH_OP_COMM => {
    val = FETCH_TOKEN_COMM;
    // break;
    }
    FETCH_OP_FOFFS => {
    val = translate_user_vaddr(code.immediate);
    // break;
    }
    _ => {
    ret = process_common_fetch_insn(code, &val);
    if (ret < 0) {
    return ret;
    }
    }
    }
    code += 1;
    return process_fetch_insn_bottom(code, val, dest, base);
    }
    NOKPROBE_SYMBOL(process_fetch_insn)
#[no_mangle]
pub unsafe extern "C" fn init_trace_uprobe_filter(filter: *mut trace_uprobe_filter) {
    rwlock_init(&filter.rwlock);
    filter.nr_systemwide = 0;
    INIT_LIST_HEAD(&filter.perf_events);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_filter_is_empty(filter: *mut trace_uprobe_filter) -> bool {
    return !filter.nr_systemwide && list_empty(&filter.perf_events);
    }
#[no_mangle]
pub unsafe extern "C" fn is_ret_probe(tu: *mut trace_uprobe) -> bool {
    return tu.consumer.ret_handler != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace_uprobe_is_busy(ev: *mut dyn_event) -> bool {
    let mut tu = to_trace_uprobe(ev);
    return trace_probe_is_enabled(&tu.tp);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_match_command_head(tu: *mut trace_uprobe, argc: c_int, argv: *mut *mut c_char) -> bool {
    char buf[64];
    let mut len = 0;
    if (!argc) {
    return true;
    }
    len = strlen(tu.filename);
    if (strncmp(tu.filename, argv[0], len) || argv[0][len] != ':') {
    return false;
    }
    if (tu.ref_ctr_offset == 0) {
    snprintf(buf, sizeof!(buf), "0x%0*lx",
    (int)(sizeof! * 2), tu.offset);
    }
    else {
    snprintf(buf, sizeof!(buf), "0x%0*lx(0x%lx)",
    (int)(sizeof! * 2), tu.offset,
    tu.ref_ctr_offset);
    }
    if (strcmp(buf, &argv[0][len + 1])) {
    return false;
    }
    argc -= 1; argv += 1;
    return trace_probe_match_command_args(&tu.tp, argc, argv);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut tu = to_trace_uprobe(ev);
    return (event[0] == '\0' ||
    strcmp(trace_probe_name(&tu.tp), event) == 0) &&
    (!system || strcmp(trace_probe_group_name(&tu.tp), system) == 0) &&
    trace_uprobe_match_command_head(tu, argc, argv);
    }
    static nokprobe_inline struct trace_uprobe *
    trace_uprobe_primary_from_call(trace_event_call *call)
    {
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return core::ptr::null_mut();
    }
    return container_of!(tp, trace_uprobe, tp);
    }
//
// Allocate new trace_uprobe and initialize it (including uprobes).
//
#[no_mangle]
pub unsafe extern "C" fn alloc_trace_uprobe(group: *mut c_char, event: *mut c_char, nargs: c_int, is_ret: bool) -> *mut c_void {
pub static mut tu: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    tu = kzalloc_flex(*tu, tp.args, nargs);
    if (!tu) {
    return ERR_PTR(-ENOMEM);
    }
    tu.nhits = alloc_percpu(unsigned long);
    if (!tu.nhits) {
    ret = -ENOMEM;
// goto;
    }
    ret = trace_probe_init(&tu.tp, event, group, true, nargs);
    if (ret < 0) {
// goto;
    }
    dyn_event_init(&tu.devent, &trace_uprobe_ops);
    tu.consumer.handler = uprobe_dispatcher;
    if (is_ret) {
    tu.consumer.ret_handler = uretprobe_dispatcher;
    }
    init_trace_uprobe_filter(tu.tp.event.filter);
    return tu;
// label;
    free_percpu(tu.nhits);
    kfree(tu);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn free_trace_uprobe(tu: *mut trace_uprobe) {
    if (IS_ERR_OR_NULL(tu)) {
    return;
    }
    path_put(&tu.path);
    trace_probe_cleanup(&tu.tp);
    kfree(tu.filename);
    free_percpu(tu.nhits);
    kfree(tu);
    }
#[no_mangle]
pub unsafe extern "C" fn find_probe_event(event: *mut c_char, group: *mut c_char) -> *mut c_void {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut tu: *mut c_void = core::ptr::null_mut();
    for_each_trace_uprobe(tu, pos) {
    if (strcmp(trace_probe_name(&tu.tp), event) == 0 &&
    strcmp(trace_probe_group_name(&tu.tp), group) == 0)
    return tu;
    }
    return core::ptr::null_mut();
    }
// Unregister a trace_uprobe and probe_event
#[no_mangle]
unsafe extern "C" fn unregister_trace_uprobe(tu: *mut trace_uprobe) -> c_int {
    let mut ret = 0;
    if (trace_probe_has_sibling(&tu.tp)) {
// goto;
    }
// If there's a reference to the dynamic event
    if (trace_event_dyn_busy(trace_probe_event_call(&tu.tp))) {
    return -EBUSY;
    }
    ret = unregister_uprobe_event(tu);
    if (ret) {
    return ret;
    }
// label;
    dyn_event_remove(&tu.devent);
    trace_probe_unlink(&tu.tp);
    free_trace_uprobe(tu);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_has_same_uprobe(orig: *mut trace_uprobe, comp: *mut trace_uprobe) -> bool {
    let mut tpe = orig.tp.event;
    let mut comp_inode = d_real_inode(comp.path.dentry);
    let mut i = 0;
    list_for_each_entry(orig, &tpe.probes, tp.list) {
    if (comp_inode != d_real_inode(orig.path.dentry) ||
    comp.offset != orig.offset) {
    continue;
    }
//
// trace_probe_compare_arg_type() ensured that nr_args and
// each argument name and type are same. Let's compare comm.
//
    while (i < orig.tp.nr_args) {
    if (strcmp(orig.tp.args[i].comm,
    comp.tp.args[i].comm)) {
    break;
    }
    }
    if (i == orig.tp.nr_args) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn append_trace_uprobe(tu: *mut trace_uprobe, to: *mut trace_uprobe) -> c_int {
    let mut ret = 0;
    ret = trace_probe_compare_arg_type(&tu.tp, &to.tp);
    if (ret) {
// Note that argument starts index = 2
    trace_probe_log_set_index(ret + 1);
    trace_probe_log_err(0, DIFF_ARG_TYPE);
    return -EEXIST;
    }
    if (trace_uprobe_has_same_uprobe(to, tu)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, SAME_PROBE);
    return -EEXIST;
    }
// Append to existing event
    ret = trace_probe_append(&tu.tp, &to.tp);
    if (!ret) {
    dyn_event_add(&tu.devent, trace_probe_event_call(&tu.tp));
    }
    return ret;
    }
//
// Uprobe with multiple reference counter is not allowed. i.e.
// If inode and offset matches, reference counter offset *must
// match as well. Though, there is one exception: If user is
// replacing old trace_uprobe with new one(same group/event),
// then we allow same uprobe with new reference counter as far
// as the new one does not conflict with any other existing
// ones.
//
#[no_mangle]
unsafe extern "C" fn validate_ref_ctr_offset(new: *mut trace_uprobe) -> c_int {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut new_inode = d_real_inode(new.path.dentry);
    for_each_trace_uprobe(tmp, pos) {
    if (new_inode == d_real_inode(tmp.path.dentry) &&
    new.offset == tmp.offset &&
    new.ref_ctr_offset != tmp.ref_ctr_offset) {
    pr_warn!("Reference counter offset mismatch.");
    return -EINVAL;
    }
    }
    return 0;
    }
// Register a trace_uprobe and probe_event
#[no_mangle]
unsafe extern "C" fn register_trace_uprobe(tu: *mut trace_uprobe) -> c_int {
pub static mut old_tu: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    guard(mutex)(&event_mutex);
    ret = validate_ref_ctr_offset(tu);
    if (ret) {
    return ret;
    }
// register as an event
    old_tu = find_probe_event(trace_probe_name(&tu.tp),
    trace_probe_group_name(&tu.tp));
    if (old_tu) {
    if (is_ret_probe(tu) != is_ret_probe(old_tu)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, DIFF_PROBE_TYPE);
    return -EEXIST;
    }
    return append_trace_uprobe(tu, old_tu);
    }
    ret = register_uprobe_event(tu);
    if (ret) {
    if (ret == -EEXIST) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, EVENT_EXIST);
    } else {
    pr_warn!("Failed to register probe event(%d)\n", ret);
    }
    return ret;
    }
    dyn_event_add(&tu.devent, trace_probe_event_call(&tu.tp));
    return ret;
    }
    DEFINE_FREE(free_trace_uprobe, trace_uprobe *, free_trace_uprobe(_T))
//
// Argument syntax:
// - Add uprobe: p|r[:[GRP/][EVENT]] PATH:OFFSET[%return][(REF)] [FETCHARGS]
//
#[no_mangle]
unsafe extern "C" fn __trace_uprobe_create(argc: c_int, argv: *const c_char) -> c_int {
    struct traceprobe_parse_context *ctx __free(traceprobe_parse_context) = core::ptr::null_mut();
    struct trace_uprobe *tu __free(free_trace_uprobe) = core::ptr::null_mut();
    const char *trlog __free(trace_probe_log_clear) = core::ptr::null_mut();
    let mut event = core::ptr::null_mut(), *group = UPROBE_EVENT_SYSTEM;
    struct path path __free(path_put) = {};
    unsigned long offset, ref_ctr_offset;
    char *filename __free(kfree) = core::ptr::null_mut();
    let mut arg = core::ptr::null_mut();
    let mut rctr = core::ptr::null_mut();
    let mut rctr_end = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    char *gbuf __free(kfree) = core::ptr::null_mut();
    char *buf __free(kfree) = core::ptr::null_mut();
    enum probe_print_type ptype;
pub static mut is_return: bool = false;
    let mut i = 0;
    let mut ret = 0;
    ref_ctr_offset = 0;
    match (argv[0][0]) {
    'r' => {
    is_return = true;
    // break;
    }
    'p' => {
    // break;
    }
    _ => {
    return -ECANCELED;
    }
    }
    if (argc < 2) {
    return -ECANCELED;
    }
    trlog = trace_probe_log_init("trace_uprobe", argc, argv);
    if (argc - 2 > MAX_TRACE_ARGS) {
    trace_probe_log_set_index(2);
    trace_probe_log_err(0, TOO_MANY_ARGS);
    return -E2BIG;
    }
    if (argv[0][1] == ':') {
    event = &argv[0][2];
    }
    if (!strchr(argv[1], '/')) {
    return -ECANCELED;
    }
    filename = kstrdup(argv[1], GFP_KERNEL);
    if (!filename) {
    return -ENOMEM;
    }
// Find the last occurrence, in case the path contains ':' too.
    arg = strrchr(filename, ':');
    if (!arg || !isdigit(arg[1])) {
    return -ECANCELED;
    }
    trace_probe_log_set_index(1);	/* filename is the 2nd argument */
// arg++ = '\0';
    ret = kern_path(filename, LOOKUP_FOLLOW, &path);
    if (ret) {
    trace_probe_log_err(0, FILE_NOT_FOUND);
    return ret;
    }
    if (!d_is_reg(path.dentry)) {
    trace_probe_log_err(0, NO_REGULAR_FILE);
    return -EINVAL;
    }
// Parse reference counter offset if specified.
    rctr = strchr(arg, '(');
    if (rctr) {
    rctr_end = strchr(rctr, ')');
    if (!rctr_end) {
    rctr_end = rctr + strlen(rctr);
    trace_probe_log_err(rctr_end - filename,
    REFCNT_OPEN_BRACE);
    return -EINVAL;
    } else if (rctr_end[1] != '\0') {
    trace_probe_log_err(rctr_end + 1 - filename,
    BAD_REFCNT_SUFFIX);
    return -EINVAL;
    }
// rctr++ = '\0';
// rctr_end = '\0';
    ret = kstrtoul(rctr, 0, &ref_ctr_offset);
    if (ret) {
    trace_probe_log_err(rctr - filename, BAD_REFCNT);
    return ret;
    }
    }
// Check if there is %return suffix
    tmp = strchr(arg, '%');
    if (tmp) {
    if (!strcmp(tmp, "%return")) {
// tmp = '\0';
    is_return = true;
    } else {
    trace_probe_log_err(tmp - filename, BAD_ADDR_SUFFIX);
    return -EINVAL;
    }
    }
// Parse uprobe offset.
    ret = kstrtoul(arg, 0, &offset);
    if (ret) {
    trace_probe_log_err(arg - filename, BAD_UPROBE_OFFS);
    return ret;
    }
// setup a probe
    trace_probe_log_set_index(0);
    if (event) {
    gbuf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!gbuf) {
    return -ENOMEM;
    }
    ret = traceprobe_parse_event_name(&event, &group, gbuf,
    event - argv[0]);
    if (ret) {
    return ret;
    }
    }
    if (!event) {
pub static mut tail: *mut c_void = core::ptr::null_mut();
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    tail = kstrdup(kbasename(filename), GFP_KERNEL);
    if (!tail) {
    return -ENOMEM;
    }
    ptr = strpbrk(tail, ".-_");
    if (ptr) {
// ptr = '\0';
    }
    buf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    snprintf(buf, MAX_EVENT_NAME_LEN, "%c_%s_0x%lx", 'p', tail, offset);
    event = buf;
    kfree(tail);
    }
    argc -= 2;
    argv += 2;
    tu = alloc_trace_uprobe(group, event, argc, is_return);
    if (IS_ERR(tu)) {
    ret = PTR_ERR(tu);
// This must return -ENOMEM otherwise there is a bug
    WARN_ON_ONCE!(ret != -ENOMEM);
    return ret;
    }
    tu.offset = offset;
    tu.ref_ctr_offset = ref_ctr_offset;
    tu.path = path;
// Clear @path so that it will not freed by path_put()
    memset(&path, 0, sizeof!(path));
    tu.filename = no_free_ptr(filename);
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
    return -ENOMEM;
    }
    ctx.flags = (is_return ? TPARG_FL_RETURN : 0) | TPARG_FL_USER;
// parse arguments
    while (i < argc) {
    trace_probe_log_set_index(i + 2);
    ret = traceprobe_parse_probe_arg(&tu.tp, i, argv[i], ctx);
    if (ret) {
    return ret;
    }
    }
    ptype = is_ret_probe(tu) ? PROBE_PRINT_RETURN : PROBE_PRINT_NORMAL;
    ret = traceprobe_set_print_fmt(&tu.tp, ptype);
    if (ret < 0) {
    return ret;
    }
    ret = register_trace_uprobe(tu);
    if (!ret) {
    tu = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_create(raw_command: *const c_char) -> c_int {
    return trace_probe_create(raw_command, __trace_uprobe_create);
    }
#[no_mangle]
unsafe extern "C" fn create_or_delete_trace_uprobe(raw_command: *const c_char) -> c_int {
    let mut ret = 0;
    if (raw_command[0] == '-') {
    return dyn_event_release(raw_command, &trace_uprobe_ops);
    }
    ret = dyn_event_create(raw_command, &trace_uprobe_ops);
pub static mut ret: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_uprobe_release(ev: *mut dyn_event) -> c_int {
    let mut tu = to_trace_uprobe(ev);
    return unregister_trace_uprobe(tu);
    }
// Probes listing interfaces
#[no_mangle]
unsafe extern "C" fn trace_uprobe_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut tu = to_trace_uprobe(ev);
pub static mut c: c_char = 0;
    let mut i = 0;
    seq_printf(m, "%c:%s/%s %s:0x%0*lx", c, trace_probe_group_name(&tu.tp),
    trace_probe_name(&tu.tp), tu.filename,
    (int)(sizeof! * 2), tu.offset);
    if (tu.ref_ctr_offset) {
    seq_printf(m, "(0x%lx)", tu.ref_ctr_offset);
    }
    for (i = 0; i < tu.tp.nr_args; i++) {
    seq_printf(m, " %s=%s", tu.tp.args[i].name, tu.tp.args[i].comm);
    }
    seq_putc(m, '\n');
    trace_probe_dump_args(m, &tu.tp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn probes_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ev = v;
    if (!is_trace_uprobe(ev)) {
    return 0;
    }
    return trace_uprobe_show(m, ev);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn probes_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) && (file.f_flags & O_TRUNC)) {
    ret = dyn_events_release_all(&trace_uprobe_ops);
    if (ret) {
    return ret;
    }
    }
    return seq_open(file, &probes_seq_op);
    }
#[no_mangle]
pub unsafe extern "C" fn probes_write(file: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    return trace_parse_run_command(file, buffer, count, ppos,
    create_or_delete_trace_uprobe);
    }
pub static mut file_operations: usize = 0;
// Probes profiling interfaces
#[no_mangle]
unsafe extern "C" fn probes_profile_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ev = v;
pub static mut tu: *mut c_void = core::ptr::null_mut();
    let mut nhits = 0;
    let mut cpu = 0;
    if (!is_trace_uprobe(ev)) {
    return 0;
    }
    tu = to_trace_uprobe(ev);
    nhits = 0;
    for_each_possible_cpu(cpu) {
    nhits += per_cpu(*tu.nhits, cpu);
    }
    seq_printf(m, "  %s %-44s %15lu\n", tu.filename,
    trace_probe_name(&tu.tp), nhits);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn profile_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    return seq_open(file, &profile_seq_op);
    }
pub static mut file_operations: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_cpu_buffer {
    pub mutex: mutex,
    pub buf: *mut c_void,
    pub dsize: c_int,
}

    static struct uprobe_cpu_buffer  *uprobe_cpu_buffer;
    static int uprobe_buffer_refcnt;

#[no_mangle]
unsafe extern "C" fn uprobe_buffer_init() -> c_int {
    let mut cpu = 0;
    let mut err_cpu = 0;
    uprobe_cpu_buffer = alloc_percpu(uprobe_cpu_buffer);
    if (uprobe_cpu_buffer == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    for_each_possible_cpu(cpu) {
    let mut p = alloc_pages_node(cpu_to_node(cpu),
    GFP_KERNEL, 0);
    if (p == core::ptr::null_mut()) {
    err_cpu = cpu;
// goto;
    }
    per_cpu_ptr(uprobe_cpu_buffer, cpu).buf = page_address(p);
    mutex_init(&per_cpu_ptr(uprobe_cpu_buffer, cpu).mutex);
    }
    return 0;
// label;
    for_each_possible_cpu(cpu) {
    if (cpu == err_cpu) {
    break;
    }
    free_page((unsigned long)per_cpu_ptr(uprobe_cpu_buffer, cpu).buf);
    }
    free_percpu(uprobe_cpu_buffer);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_buffer_enable() -> c_int {
pub static mut ret: c_int = 0;
    lockdep_assert_held(&event_mutex);
    if (uprobe_buffer_refcnt++ == 0) {
    ret = uprobe_buffer_init();
    if (ret < 0) {
    uprobe_buffer_refcnt -= 1;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_buffer_disable() {
    let mut cpu = 0;
    lockdep_assert_held(&event_mutex);
    if (--uprobe_buffer_refcnt == 0) {
    for_each_possible_cpu(cpu) {
    free_page((unsigned long)per_cpu_ptr(uprobe_cpu_buffer,
    cpu).buf);
    }
    free_percpu(uprobe_cpu_buffer);
    uprobe_cpu_buffer = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_buffer_get() -> *mut c_void {
pub static mut ucb: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    cpu = raw_smp_processor_id();
    ucb = per_cpu_ptr(uprobe_cpu_buffer, cpu);
//
// Use per-cpu buffers for fastest access, but we might migrate
// so the mutex makes sure we have sole access to it.
//
    mutex_lock(&ucb.mutex);
    return ucb;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_buffer_put(ucb: *mut uprobe_cpu_buffer) {
    if (!ucb) {
    return;
    }
    mutex_unlock(&ucb.mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_uprobe_buffer(tu: *mut trace_uprobe, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) -> *mut c_void {
pub static mut ucb: *mut c_void = core::ptr::null_mut();
    let mut dsize = 0;
    let mut esize = 0;
    if (*ucbp) {
pub static mut ucbp: *mut c_void = core::ptr::null_mut();
    }
    esize = SIZEOF_TRACE_ENTRY(is_ret_probe(tu));
    dsize = __get_data_size(&tu.tp, regs, core::ptr::null_mut());
    ucb = uprobe_buffer_get();
    ucb.dsize = tu.tp.size + dsize;
    BUILD_BUG_ON!(MAX_UCB_BUFFER_SIZE < MAX_PROBE_EVENT_SIZE);
    if (WARN_ON_ONCE!(ucb.dsize > MAX_UCB_BUFFER_SIZE)) {
    ucb.dsize = MAX_UCB_BUFFER_SIZE;
    dsize = MAX_UCB_BUFFER_SIZE - tu.tp.size;
    }
    store_trace_args(ucb.buf, &tu.tp, regs, core::ptr::null_mut(), esize, dsize);
// ucbp = ucb;
    return ucb;
    }
#[no_mangle]
pub unsafe extern "C" fn __uprobe_trace_func(tu: *mut trace_uprobe, func: c_ulong, regs: *mut pt_regs, ucb: *mut uprobe_cpu_buffer, trace_file: *mut trace_event_file) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut esize = 0;
    let mut call = trace_probe_event_call(&tu.tp);
    WARN_ON!(call != trace_file.event_call);
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    esize = SIZEOF_TRACE_ENTRY(is_ret_probe(tu));
    size = esize + ucb.dsize;
    entry = trace_event_buffer_reserve(&fbuffer, trace_file, size);
    if (!entry) {
    return;
    }
    if (is_ret_probe(tu)) {
    entry.vaddr[0] = func;
    entry.vaddr[1] = instruction_pointer(regs);
    data = DATAOF_TRACE_ENTRY(entry, true);
    } else {
    entry.vaddr[0] = instruction_pointer(regs);
    data = DATAOF_TRACE_ENTRY(entry, false);
    }
    memcpy(data, ucb.buf, ucb.dsize);
    trace_event_buffer_commit(&fbuffer);
    }
// uprobe handler
#[no_mangle]
pub unsafe extern "C" fn uprobe_trace_func(tu: *mut trace_uprobe, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) -> c_int {
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut ucb: *mut c_void = core::ptr::null_mut();
    if (is_ret_probe(tu)) {
    return 0;
    }
    ucb = prepare_uprobe_buffer(tu, regs, ucbp);
    rcu_read_lock();
    trace_probe_for_each_link_rcu(link, &tu.tp)
    __uprobe_trace_func(tu, 0, regs, ucb, link.file);
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uretprobe_trace_func(tu: *mut trace_uprobe, func: c_ulong, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) {
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut ucb: *mut c_void = core::ptr::null_mut();
    ucb = prepare_uprobe_buffer(tu, regs, ucbp);
    rcu_read_lock();
    trace_probe_for_each_link_rcu(link, &tu.tp)
    __uprobe_trace_func(tu, func, regs, ucb, link.file);
    rcu_read_unlock();
    }
// Event entry printers
    static enum print_line_t
    print_uprobe_event(trace_iterator *iter, int flags, trace_event *event)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    entry = iter.ent;
    tu = trace_uprobe_primary_from_call(
    container_of!(event, trace_event_call, event));
    if (unlikely(!tu)) {
// goto;
    }
    if (is_ret_probe(tu)) {
    trace_seq_printf(s, "%s: (0x%lx <- 0x%lx)",
    trace_probe_name(&tu.tp),
    entry.vaddr[1], entry.vaddr[0]);
    data = DATAOF_TRACE_ENTRY(entry, true);
    } else {
    trace_seq_printf(s, "%s: (0x%lx)",
    trace_probe_name(&tu.tp),
    entry.vaddr[0]);
    data = DATAOF_TRACE_ENTRY(entry, false);
    }
    if (trace_probe_print_args(s, tu.tp.args, tu.tp.nr_args, data, entry) < 0) {
// goto;
    }
    trace_seq_putc(s, '\n');
// label;
    return trace_handle_return(s);
    }
    typedef bool (*filter_func_t)(uprobe_consumer *self, mm_struct *mm);
#[no_mangle]
unsafe extern "C" fn trace_uprobe_enable(tu: *mut trace_uprobe, filter: filter_func_t) -> c_int {
    let mut inode = d_real_inode(tu.path.dentry);
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    tu.consumer.filter = filter;
    uprobe = uprobe_register(inode, tu.offset, tu.ref_ctr_offset, &tu.consumer);
    if (IS_ERR(uprobe)) {
    return PTR_ERR(uprobe);
    }
    tu.uprobe = uprobe;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __probe_event_disable(tp: *mut trace_probe) {
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut sync: bool = false;
    tu = container_of!(tp, trace_uprobe, tp);
    WARN_ON!(!uprobe_filter_is_empty(tu.tp.event.filter));
    list_for_each_entry(tu, trace_probe_probe_list(tp), tp.list) {
    if (!tu.uprobe) {
    continue;
    }
    uprobe_unregister_nosync(tu.uprobe, &tu.consumer);
    sync = true;
    tu.uprobe = core::ptr::null_mut();
    }
    if (sync) {
    uprobe_unregister_sync();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn probe_event_enable(call: *mut trace_event_call, file: *mut trace_event_file, filter: filter_func_t) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut tu: *mut c_void = core::ptr::null_mut();
    let mut enabled = 0;
    let mut ret = 0;
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENODEV;
    }
    enabled = trace_probe_is_enabled(tp);
// This may also change "enabled" state
    if (file) {
    if (trace_probe_test_flag(tp, TP_FLAG_PROFILE)) {
    return -EINTR;
    }
    ret = trace_probe_add_file(tp, file);
    if (ret < 0) {
    return ret;
    }
    } else {
    if (trace_probe_test_flag(tp, TP_FLAG_TRACE)) {
    return -EINTR;
    }
    trace_probe_set_flag(tp, TP_FLAG_PROFILE);
    }
    tu = container_of!(tp, trace_uprobe, tp);
    WARN_ON!(!uprobe_filter_is_empty(tu.tp.event.filter));
    if (enabled) {
    return 0;
    }
    ret = uprobe_buffer_enable();
    if (ret) {
// goto;
    }
    list_for_each_entry(tu, trace_probe_probe_list(tp), tp.list) {
    ret = trace_uprobe_enable(tu, filter);
    if (ret) {
    __probe_event_disable(tp);
// goto;
    }
    }
    return 0;
// label;
    uprobe_buffer_disable();
// label;
    if (file) {
    trace_probe_remove_file(tp, file);
    }
    else {
    trace_probe_clear_flag(tp, TP_FLAG_PROFILE);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_event_disable(call: *mut trace_event_call, file: *mut trace_event_file) {
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return;
    }
    if (!trace_probe_is_enabled(tp)) {
    return;
    }
    if (file) {
    if (trace_probe_remove_file(tp, file) < 0) {
    return;
    }
    if (trace_probe_is_enabled(tp)) {
    return;
    }
    } else {
    trace_probe_clear_flag(tp, TP_FLAG_PROFILE);
    }
    __probe_event_disable(tp);
    uprobe_buffer_disable();
    }
#[no_mangle]
unsafe extern "C" fn uprobe_event_define_fields(event_call: *mut trace_event_call) -> c_int {
    let mut ret = 0;
    let mut size = 0;
pub static mut field: usize = 0;
pub static mut tu: *mut c_void = core::ptr::null_mut();
    tu = trace_uprobe_primary_from_call(event_call);
    if (unlikely(!tu)) {
    return -ENODEV;
    }
    if (is_ret_probe(tu)) {
pub static mut unsigned long: usize = 0;
pub static mut unsigned long: usize = 0;
    size = SIZEOF_TRACE_ENTRY(true);
    } else {
pub static mut unsigned long: usize = 0;
    size = SIZEOF_TRACE_ENTRY(false);
    }
    return traceprobe_define_arg_fields(event_call, size, &tu.tp);
    }

#[no_mangle]
pub unsafe extern "C" fn __uprobe_perf_filter(filter: *mut trace_uprobe_filter, mm: *mut mm_struct) -> bool {
pub static mut event: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(event, &filter.perf_events, hw.tp_list) {
    if (event.hw.target.mm == mm) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_filter_event(filter: *mut trace_uprobe_filter, event: *mut perf_event) -> bool {
    return __uprobe_perf_filter(filter, event.hw.target.mm);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_filter_remove(filter: *mut trace_uprobe_filter, event: *mut perf_event) -> bool {
    let mut done = 0;
    write_lock(&filter.rwlock);
    if (event.hw.target) {
    list_del(&event.hw.tp_list);
    done = filter.nr_systemwide ||
    (event.hw.target.flags & PF_EXITING) ||
    trace_uprobe_filter_event(filter, event);
    } else {
    filter.nr_systemwide -= 1;
    done = filter.nr_systemwide;
    }
    write_unlock(&filter.rwlock);
    return done;
    }
// This returns true if the filter always covers target mm
#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_filter_add(filter: *mut trace_uprobe_filter, event: *mut perf_event) -> bool {
    let mut done = 0;
    write_lock(&filter.rwlock);
    if (event.hw.target) {
//
// event->parent != NULL means copy_process(), we can avoid
// uprobe_apply(). current->mm must be probed and we can rely
// on dup_mmap() which preserves the already installed bp's.
//
// attr.enable_on_exec means that exec/mmap will install the
// breakpoints we need.
//
    done = filter.nr_systemwide ||
    event.parent || event.attr.enable_on_exec ||
    trace_uprobe_filter_event(filter, event);
    list_add(&event.hw.tp_list, &filter.perf_events);
    } else {
    done = filter.nr_systemwide;
    filter.nr_systemwide += 1;
    }
    write_unlock(&filter.rwlock);
    return done;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_perf_close(call: *mut trace_event_call, event: *mut perf_event) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENODEV;
    }
    tu = container_of!(tp, trace_uprobe, tp);
    if (trace_uprobe_filter_remove(tu.tp.event.filter, event)) {
    return 0;
    }
    list_for_each_entry(tu, trace_probe_probe_list(tp), tp.list) {
    ret = uprobe_apply(tu.uprobe, &tu.consumer, false);
    if (ret) {
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_perf_open(call: *mut trace_event_call, event: *mut perf_event) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENODEV;
    }
    tu = container_of!(tp, trace_uprobe, tp);
    if (trace_uprobe_filter_add(tu.tp.event.filter, event)) {
    return 0;
    }
    list_for_each_entry(tu, trace_probe_probe_list(tp), tp.list) {
    err = uprobe_apply(tu.uprobe, &tu.consumer, true);
    if (err) {
    uprobe_perf_close(call, event);
    break;
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_perf_filter(uc: *mut uprobe_consumer, mm: *mut mm_struct) -> bool {
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut tu: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    tu = container_of!(uc, trace_uprobe, consumer);
    filter = tu.tp.event.filter;
//
// speculative short-circuiting check to avoid unnecessarily taking
// filter->rwlock below, if the uprobe has system-wide consumer
//
    if (READ_ONCE(filter.nr_systemwide)) {
    return true;
    }
    read_lock(&filter.rwlock);
    ret = __uprobe_perf_filter(filter, mm);
    read_unlock(&filter.rwlock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __uprobe_perf_func(tu: *mut trace_uprobe, func: c_ulong, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) {
    let mut call = trace_probe_event_call(&tu.tp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut ucb: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut esize = 0;
    let mut rctx = 0;

    if (bpf_prog_array_valid(call)) {
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rcu_read_lock_trace();
    array = rcu_dereference_check(call.prog_array, rcu_read_lock_trace_held());
    ret = bpf_prog_run_array_uprobe(array, regs, bpf_prog_run);
    rcu_read_unlock_trace();
    if (!ret) {
    return;
    }
    }

    esize = SIZEOF_TRACE_ENTRY(is_ret_probe(tu));
    ucb = prepare_uprobe_buffer(tu, regs, ucbp);
    size = esize + ucb.dsize;
    size = ALIGN(size + sizeof!(u32), sizeof!(u64)) - sizeof!(u32);
    if (WARN_ONCE(size > PERF_MAX_TRACE_SIZE, "profile buffer not large enough")) {
    return;
    }
    preempt_disable();
    head = this_cpu_ptr(call.perf_events);
    if (hlist_empty(head)) {
// goto;
    }
    entry = perf_trace_buf_alloc(size, core::ptr::null_mut(), &rctx);
    if (!entry) {
// goto;
    }
    if (is_ret_probe(tu)) {
    entry.vaddr[0] = func;
    entry.vaddr[1] = instruction_pointer(regs);
    data = DATAOF_TRACE_ENTRY(entry, true);
    } else {
    entry.vaddr[0] = instruction_pointer(regs);
    data = DATAOF_TRACE_ENTRY(entry, false);
    }
    memcpy(data, ucb.buf, ucb.dsize);
    if (size - esize > ucb.dsize) {
    memset(data + ucb.dsize, 0, size - esize - ucb.dsize);
    }
    perf_trace_buf_submit(entry, size, rctx, call.event.type, 1, regs,
    head, core::ptr::null_mut());
// label;
    preempt_enable();
    }
// uprobe profile handler
#[no_mangle]
pub unsafe extern "C" fn uprobe_perf_func(tu: *mut trace_uprobe, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) -> c_int {
    if (!uprobe_perf_filter(&tu.consumer, current.mm)) {
    return UPROBE_HANDLER_REMOVE;
    }
    if (!is_ret_probe(tu)) {
    __uprobe_perf_func(tu, 0, regs, ucbp);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uretprobe_perf_func(tu: *mut trace_uprobe, func: c_ulong, regs: *mut pt_regs, ucbp: *mut *mut uprobe_cpu_buffer) {
    __uprobe_perf_func(tu, func, regs, ucbp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_uprobe_info(event: *mut perf_event, fd_type: *mut u32, filename: *mut *mut c_char, probe_offset: *mut u64, probe_addr: *mut u64, perf_type_tracepoint: bool) -> c_int {
    let mut pevent = trace_event_name(event.tp_event);
    let mut group = event.tp_event.class.system;
pub static mut tu: *mut c_void = core::ptr::null_mut();
    if (perf_type_tracepoint) {
    tu = find_probe_event(pevent, group);
    }
    else {
    tu = trace_uprobe_primary_from_call(event.tp_event);
    }
    if (!tu) {
    return -EINVAL;
    }
// fd_type = is_ret_probe(tu) ? BPF_FD_TYPE_URETPROBE
    : BPF_FD_TYPE_UPROBE;
// filename = tu->filename;
// probe_offset = tu->offset;
// probe_addr = tu->ref_ctr_offset;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_uprobe_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut file = data;
    match (type) {
    TRACE_REG_REGISTER => {
    return probe_event_enable(event, file, core::ptr::null_mut());
    }
    TRACE_REG_UNREGISTER => {
    probe_event_disable(event, file);
    return 0;

    }
    TRACE_REG_PERF_REGISTER => {
    return probe_event_enable(event, core::ptr::null_mut(), uprobe_perf_filter);
    }
    TRACE_REG_PERF_UNREGISTER => {
    probe_event_disable(event, core::ptr::null_mut());
    return 0;
    }
    TRACE_REG_PERF_OPEN => {
    return uprobe_perf_open(event, data);
    }
    TRACE_REG_PERF_CLOSE => {
    return uprobe_perf_close(event, data);

    }
    _ => {
    return 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_dispatcher(con: *mut uprobe_consumer, regs: *mut pt_regs, data: *mut __u64) -> c_int {
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut udd: usize = 0;
    let mut ucb = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    tu = container_of!(con, trace_uprobe, consumer);
    this_cpu_inc(*tu.nhits);
    udd.tu = tu;
    udd.bp_addr = instruction_pointer(regs);
    current.utask.vaddr = (unsigned long) &udd;
    if (WARN_ON_ONCE!(!uprobe_cpu_buffer)) {
    return 0;
    }
    flags = trace_probe_load_flag(&tu.tp);
    if (flags & TP_FLAG_TRACE) {
    ret |= uprobe_trace_func(tu, regs, &ucb);
    }

    if (flags & TP_FLAG_PROFILE) {
    ret |= uprobe_perf_func(tu, regs, &ucb);
    }

    uprobe_buffer_put(ucb);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn uretprobe_dispatcher(con: *mut uprobe_consumer, func: c_ulong, regs: *mut pt_regs, data: *mut __u64) -> c_int {
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut udd: usize = 0;
    let mut ucb = core::ptr::null_mut();
    let mut flags = 0;
    tu = container_of!(con, trace_uprobe, consumer);
    udd.tu = tu;
    udd.bp_addr = func;
    current.utask.vaddr = (unsigned long) &udd;
    if (WARN_ON_ONCE!(!uprobe_cpu_buffer)) {
    return 0;
    }
    flags = trace_probe_load_flag(&tu.tp);
    if (flags & TP_FLAG_TRACE) {
    uretprobe_trace_func(tu, func, regs, &ucb);
    }

    if (flags & TP_FLAG_PROFILE) {
    uretprobe_perf_func(tu, func, regs, &ucb);
    }

    uprobe_buffer_put(ucb);
    return 0;
    }
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_fields: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_trace_event_call(tu: *mut trace_uprobe) {
    let mut call = trace_probe_event_call(&tu.tp);
    call.event.funcs = &uprobe_funcs;
    call.class.fields_array = uprobe_fields_array;
    call.flags = TRACE_EVENT_FL_UPROBE | TRACE_EVENT_FL_CAP_ANY;
    call.class.reg = trace_uprobe_register;
    }
#[no_mangle]
unsafe extern "C" fn register_uprobe_event(tu: *mut trace_uprobe) -> c_int {
    init_trace_event_call(tu);
    return trace_probe_register_event_call(&tu.tp);
    }
#[no_mangle]
unsafe extern "C" fn unregister_uprobe_event(tu: *mut trace_uprobe) -> c_int {
    return trace_probe_unregister_event_call(&tu.tp);
    }

#[no_mangle]
pub unsafe extern "C" fn create_local_trace_uprobe(name: *mut c_char, offs: c_ulong, ref_ctr_offset: c_ulong, is_return: bool) -> *mut c_void {
    enum probe_print_type ptype;
pub static mut tu: *mut c_void = core::ptr::null_mut();
pub static mut path: usize = 0;
    let mut ret = 0;
    ret = kern_path(name, LOOKUP_FOLLOW, &path);
    if (ret) {
    return ERR_PTR(ret);
    }
    if (!d_is_reg(path.dentry)) {
    path_put(&path);
    return ERR_PTR(-EINVAL);
    }
//
// local trace_kprobes are not added to dyn_event, so they are never
// searched in find_trace_kprobe(). Therefore, there is no concern of
// duplicated name "DUMMY_EVENT" here.
//
    tu = alloc_trace_uprobe(UPROBE_EVENT_SYSTEM, "DUMMY_EVENT", 0,
    is_return);
    if (IS_ERR(tu)) {
    pr_info!("Failed to allocate trace_uprobe.(%d)\n",
    (int)PTR_ERR(tu));
    path_put(&path);
    return ERR_CAST(tu);
    }
    tu.offset = offs;
    tu.path = path;
    tu.ref_ctr_offset = ref_ctr_offset;
    tu.filename = kstrdup(name, GFP_KERNEL);
    if (!tu.filename) {
    ret = -ENOMEM;
// goto;
    }
    init_trace_event_call(tu);
    ptype = is_ret_probe(tu) ? PROBE_PRINT_RETURN : PROBE_PRINT_NORMAL;
    if (traceprobe_set_print_fmt(&tu.tp, ptype) < 0) {
    ret = -ENOMEM;
// goto;
    }
    return trace_probe_event_call(&tu.tp);
// label;
    free_trace_uprobe(tu);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_local_trace_uprobe(event_call: *mut trace_event_call) {
pub static mut tu: *mut c_void = core::ptr::null_mut();
    tu = trace_uprobe_primary_from_call(event_call);
    free_trace_uprobe(tu);
    }

// Make a trace interface for controlling probe points
#[no_mangle]
unsafe extern "C" fn init_uprobe_trace() -> __init int {
    let mut ret = 0;
    ret = dyn_event_register(&trace_uprobe_ops);
    if (ret) {
    return ret;
    }
    ret = tracing_init_dentry();
    if (ret) {
    return 0;
    }
    trace_create_file("uprobe_events", TRACE_MODE_WRITE, core::ptr::null_mut(),
    core::ptr::null_mut(), &uprobe_events_ops);
// Profile interface
    trace_create_file("uprobe_profile", TRACE_MODE_READ, core::ptr::null_mut(),
    core::ptr::null_mut(), &uprobe_profile_ops);
    return 0;
    }
    fs_initcall!(init_uprobe_trace);