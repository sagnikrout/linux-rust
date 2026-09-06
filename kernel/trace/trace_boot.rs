//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_boot.c
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
// trace_boot.c
// Tracing kernel boot-time
//

pub const MAX_BUF_LEN: c_int = 256;
    static void __init
    trace_boot_set_instance_options(trace_array *tr, xbc_node *node)
    {
pub static mut anode: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
pub static mut v: c_ulong = 0;
// Common ftrace options
    xbc_node_for_each_array_value(node, "options", anode, p) {
    if (strscpy(buf, p, ARRAY_SIZE!(buf)) < 0) {
    pr_err!("String is too long: %s\n", p);
    continue;
    }
    if (trace_set_options(tr, buf) < 0) {
    pr_err!("Failed to set option: %s\n", buf);
    }
    }
    p = xbc_node_find_value(node, "tracing_on", core::ptr::null_mut());
    if (p && *p != '\0') {
    if (kstrtoul(p, 10, &v)) {
    pr_err!("Failed to set tracing on: %s\n", p);
    }
    if (v) {
    tracer_tracing_on(tr);
    }
    else {
    tracer_tracing_off(tr);
    }
    }
    p = xbc_node_find_value(node, "trace_clock", core::ptr::null_mut());
    if (p && *p != '\0') {
    if (tracing_set_clock(tr, p) < 0) {
    pr_err!("Failed to set trace clock: %s\n", p);
    }
    }
    p = xbc_node_find_value(node, "buffer_size", core::ptr::null_mut());
    if (p && *p != '\0') {
    v = memparse(p, core::ptr::null_mut());
    if (v < PAGE_SIZE) {
    pr_err!("Buffer size is too small: %s\n", p);
    }
    if (trace_array_is_readonly(tr) ||
    tracing_resize_ring_buffer(tr, v, RING_BUFFER_ALL_CPUS) < 0) {
    pr_err!("Failed to resize trace buffer to %s\n", p);
    }
    }
    p = xbc_node_find_value(node, "cpumask", core::ptr::null_mut());
    if (p && *p != '\0') {
    let mut new_mask;
    if (alloc_cpumask_var(&new_mask, GFP_KERNEL)) {
    if (cpumask_parse(p, new_mask) < 0 ||
    tracing_set_cpumask(tr, new_mask) < 0) {
    pr_err!("Failed to set new CPU mask %s\n", p);
    }
    free_cpumask_var(new_mask);
    }
    }
    }

    static void __init
    trace_boot_enable_events(trace_array *tr, xbc_node *node)
    {
pub static mut anode: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
pub static mut p: *mut c_void = core::ptr::null_mut();
    xbc_node_for_each_array_value(node, "events", anode, p) {
    if (strscpy(buf, p, ARRAY_SIZE!(buf)) < 0) {
    pr_err!("String is too long: %s\n", p);
    continue;
    }
    if (ftrace_set_clr_event(tr, buf, 1) < 0) {
    pr_err!("Failed to enable event: %s\n", p);
    }
    }
    }

    static int __init
    trace_boot_add_kprobe_event(xbc_node *node, const char *event)
    {
pub static mut cmd: usize = 0;
pub static mut anode: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
pub static mut val: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    xbc_node_for_each_array_value(node, "probes", anode, val) {
    kprobe_event_cmd_init(&cmd, buf, MAX_BUF_LEN);
    ret = kprobe_event_gen_cmd_start(&cmd, event, val);
    if (ret) {
    pr_err!("Failed to generate probe: %s\n", buf);
    break;
    }
    ret = kprobe_event_gen_cmd_end(&cmd);
    if (ret) {
    pr_err!("Failed to add probe: %s\n", buf);
    break;
    }
    }
    return ret;
    }

    static inline int __init
    trace_boot_add_kprobe_event(xbc_node *node, const char *event)
    {
    pr_err!("Kprobe event is not supported.\n");
    return -ENOTSUPP;
    }

    static int __init
    trace_boot_add_synth_event(xbc_node *node, const char *event)
    {
pub static mut cmd: usize = 0;
pub static mut anode: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    synth_event_cmd_init(&cmd, buf, MAX_BUF_LEN);
    ret = synth_event_gen_cmd_start(&cmd, event, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    xbc_node_for_each_array_value(node, "fields", anode, p) {
    ret = synth_event_add_field_str(&cmd, p);
    if (ret) {
    return ret;
    }
    }
    ret = synth_event_gen_cmd_end(&cmd);
    if (ret < 0) {
    pr_err!("Failed to add synthetic event: %s\n", buf);
    }
    return ret;
    }

    static inline int __init
    trace_boot_add_synth_event(xbc_node *node, const char *event)
    {
    pr_err!("Synthetic event is not supported.\n");
    return -ENOTSUPP;
    }

    static int __init
    trace_boot_add_probe_event(xbc_node *node, const char *group,
    const char *event, char type, const char *type_name)
    {
pub static mut anode: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
pub static mut val: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    xbc_node_for_each_array_value(node, "probes", anode, val) {
    if (val[0] == type && (val[1] == ':' || isspace(val[1]))) {
    ret = strscpy(buf, val, MAX_BUF_LEN);
    if (ret < 0) {
    pr_err!("%s command is too long: %s\n", type_name, val);
    break;
    }
    } else {
    ret = snprintf(buf, MAX_BUF_LEN, "%c:%s/%s %s", type, group, event, val);
    if (ret >= MAX_BUF_LEN || ret < 0) {
    pr_err!("%s command is too long: %c:%s/%s %s\n",
    type_name, type, group, event, val);
    ret = -E2BIG;
    break;
    }
    }
    ret = dyn_event_create(buf, core::ptr::null_mut());
    if (ret) {
    pr_err!("Failed to add %s: %s\n", type_name, buf);
    break;
    }
    }
    return ret;
    }

    static inline int __init
    trace_boot_add_eprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    return trace_boot_add_probe_event(node, group, event, 'e', "eprobe");
    }

    static inline int __init
    trace_boot_add_eprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    pr_err!("Event probe is not supported.\n");
    return -EOPNOTSUPP;
    }

    static inline int __init
    trace_boot_add_fprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    return trace_boot_add_probe_event(node, group, event, 'f', "fprobe");
    }
    static inline int __init
    trace_boot_add_tprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    return trace_boot_add_probe_event(node, group, event, 't', "tprobe");
    }

    static inline int __init
    trace_boot_add_fprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    pr_err!("Fprobe event is not supported.\n");
    return -EOPNOTSUPP;
    }
    static inline int __init
    trace_boot_add_tprobe_event(xbc_node *node, const char *group,
    const char *event)
    {
    pr_err!("Tracepoint probe is not supported.\n");
    return -EOPNOTSUPP;
    }

#[no_mangle]
unsafe extern "C" fn __printf(_arg: 3, _arg: 4) -> c_int {
    let mut args;
    let mut ret = 0;
    if (*bufp == end) {
    return -ENOSPC;
    }
    va_start(args, fmt);
    ret = vsnprintf(*bufp, end - *bufp, fmt, args);
    if (ret < end - *bufp) {
// bufp += ret;
    } else {
// bufp = end;
    ret = -ERANGE;
    }
    va_end(args);
    return ret;
    }
    static int __init
    append_str_nospace(char **bufp, char *end, const char *str)
    {
    let mut p = *bufp;
    let mut len = 0;
    while (p < end - 1 && *str != '\0') {
    if (!isspace(*str)) {
// (p++) = *str;
    }
    str += 1;
    }
// p = '\0';
    if (p == end - 1) {
// bufp = end;
    return -ENOSPC;
    }
    len = p - *bufp;
// bufp = p;
    return (int)len;
    }
    static int __init
    trace_boot_hist_add_array(xbc_node *hnode, char **bufp,
    char *end, const char *key)
    {
pub static mut anode: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut sep = 0;
    p = xbc_node_find_value(hnode, key, &anode);
    if (p) {
    if (!anode) {
    pr_err!("hist.%s requires value(s).\n", key);
    return -EINVAL;
    }
    append_printf(bufp, end, ":%s", key);
    sep = '=';
    xbc_array_for_each_value(anode, p) {
    append_printf(bufp, end, "%c%s", sep, p);
    if (sep == '=') {
    sep = ',';
    }
    }
    } else {
    return -ENOENT;
    }
    return 0;
    }
    static int __init
    trace_boot_hist_add_one_handler(xbc_node *hnode, char **bufp,
    char *end, const char *handler,
    const char *param)
    {
    let mut knode = core::ptr::null_mut();
    let mut anode = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut sep = 0;
// Compose 'handler' parameter
    p = xbc_node_find_value(hnode, param, core::ptr::null_mut());
    if (!p) {
    pr_err!("hist.%s requires '%s' option.\n",
    xbc_node_get_data(hnode), param);
    return -EINVAL;
    }
    append_printf(bufp, end, ":%s(%s)", handler, p);
// Compose 'action' parameter
    knode = xbc_node_find_subkey(hnode, "trace");
    if (!knode) {
    knode = xbc_node_find_subkey(hnode, "save");
    }
    if (knode) {
    anode = xbc_node_get_child(knode);
    if (!anode || !xbc_node_is_value(anode)) {
    pr_err!("hist.%s.%s requires value(s).\n",
    xbc_node_get_data(hnode),
    xbc_node_get_data(knode));
    return -EINVAL;
    }
    append_printf(bufp, end, ".%s", xbc_node_get_data(knode));
    sep = '(';
    xbc_array_for_each_value(anode, p) {
    append_printf(bufp, end, "%c%s", sep, p);
    if (sep == '(')
    sep = ',';
    }
    append_printf(bufp, end, ")"); {
    } else if (xbc_node_find_subkey(hnode, "snapshot")) {
    }
    append_printf(bufp, end, ".snapshot()");
    } else {
    pr_err!("hist.%s requires an action.\n",
    xbc_node_get_data(hnode));
    return -EINVAL;
    }
    return 0;
    }
    static int __init
    trace_boot_hist_add_handlers(xbc_node *hnode, char **bufp,
    char *end, const char *param)
    {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut handler = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    handler = xbc_node_get_data(hnode);
    xbc_node_for_each_subkey(hnode, node) {
    p = xbc_node_get_data(node);
    if (!isdigit(p[0])) {
    continue;
    }
// All digit started node should be instances.
    ret = trace_boot_hist_add_one_handler(node, bufp, end, handler, param);
    if (ret < 0) {
    break;
    }
    }
    if (xbc_node_find_subkey(hnode, param)) {
    ret = trace_boot_hist_add_one_handler(hnode, bufp, end, handler, param);
    }
    return ret;
    }
//
// Histogram boottime tracing syntax.
//
// ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist[.N] {
// keys = <KEY>[,...]
// values = <VAL>[,...]
// sort = <SORT-KEY>[,...]
// size = <ENTRIES>
// name = <HISTNAME>
// var { <VAR> = <EXPR> ... }
// pause|continue|clear
// onmax|onchange[.N] { var = <VAR>; <ACTION> [= <PARAM>] }
// onmatch[.N] { event = <EVENT>; <ACTION> [= <PARAM>] }
// filter = <FILTER>
// }
//
// Where <ACTION> are;
//
// trace = <EVENT>, <ARG1>[, ...]
// save = <ARG1>[, ...]
// snapshot
//
    static int __init
    trace_boot_compose_hist_cmd(xbc_node *hnode, char *buf, size_t size)
    {
    let mut node = core::ptr::null_mut();
    let mut knode = core::ptr::null_mut();
    let mut end = buf + size;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    append_printf(&buf, end, "hist");
    ret = trace_boot_hist_add_array(hnode, &buf, end, "keys");
    if (ret < 0) {
    if (ret == -ENOENT) {
    pr_err!("hist requires keys.\n");
    }
    return -EINVAL;
    }
    ret = trace_boot_hist_add_array(hnode, &buf, end, "values");
    if (ret == -EINVAL) {
    return ret;
    }
    ret = trace_boot_hist_add_array(hnode, &buf, end, "sort");
    if (ret == -EINVAL) {
    return ret;
    }
    p = xbc_node_find_value(hnode, "size", core::ptr::null_mut());
    if (p) {
    append_printf(&buf, end, ":size=%s", p);
    }
    p = xbc_node_find_value(hnode, "name", core::ptr::null_mut());
    if (p) {
    append_printf(&buf, end, ":name=%s", p);
    }
    node = xbc_node_find_subkey(hnode, "var");
    if (node) {
    xbc_node_for_each_key_value(node, knode, p) {
// Expression must not include spaces.
    append_printf(&buf, end, ":%s=",
    xbc_node_get_data(knode));
    append_str_nospace(&buf, end, p);
    }
    }
// Histogram control attributes (mutual exclusive)
    if (xbc_node_find_value(hnode, "pause", core::ptr::null_mut())) {
    append_printf(&buf, end, ":pause");
    }

    else if (xbc_node_find_value(hnode, "continue", core::ptr::null_mut())) {
    append_printf(&buf, end, ":continue");
    }

    else if (xbc_node_find_value(hnode, "clear", core::ptr::null_mut())) {
    append_printf(&buf, end, ":clear");
    }
// Histogram handler and actions
    node = xbc_node_find_subkey(hnode, "onmax");
    if (node && trace_boot_hist_add_handlers(node, &buf, end, "var") < 0) {
    return -EINVAL;
    }
    node = xbc_node_find_subkey(hnode, "onchange");
    if (node && trace_boot_hist_add_handlers(node, &buf, end, "var") < 0) {
    return -EINVAL;
    }
    node = xbc_node_find_subkey(hnode, "onmatch");
    if (node && trace_boot_hist_add_handlers(node, &buf, end, "event") < 0) {
    return -EINVAL;
    }
    p = xbc_node_find_value(hnode, "filter", core::ptr::null_mut());
    if (p) {
    append_printf(&buf, end, " if %s", p);
    }
    if (buf == end) {
    pr_err!("hist exceeds the max command length.\n");
    return -E2BIG;
    }
    return 0;
    }
    static void __init
    trace_boot_init_histograms(trace_event_file *file, xbc_node *hnode, char *buf, size_t size)
    {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    xbc_node_for_each_subkey(hnode, node) {
    p = xbc_node_get_data(node);
    if (!isdigit(p[0])) {
    continue;
    }
// All digit started node should be instances.
    if (trace_boot_compose_hist_cmd(node, buf, size) == 0) {
    tmp = kstrdup(buf, GFP_KERNEL);
    if (!tmp) {
    return;
    }
    if (trigger_process_regex(file, buf) < 0) {
    pr_err!("Failed to apply hist trigger: %s\n", tmp);
    }
    kfree(tmp);
    }
    }
    if (xbc_node_find_subkey(hnode, "keys")) {
    if (trace_boot_compose_hist_cmd(hnode, buf, size) == 0) {
    tmp = kstrdup(buf, GFP_KERNEL);
    if (!tmp) {
    return;
    }
    if (trigger_process_regex(file, buf) < 0) {
    pr_err!("Failed to apply hist trigger: %s\n", tmp);
    }
    kfree(tmp);
    }
    }
    }

    static void __init
    trace_boot_init_histograms(trace_event_file *file, xbc_node *hnode, char *buf, size_t size)
    {
// do nothing
    }

    static void __init
    trace_boot_init_one_event(trace_array *tr, xbc_node *gnode, xbc_node *enode)
    {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut anode: *mut c_void = core::ptr::null_mut();
    char buf[MAX_BUF_LEN];
    let mut p = core::ptr::null_mut();
    let mut group = core::ptr::null_mut();
    let mut event = core::ptr::null_mut();
    group = xbc_node_get_data(gnode);
    event = xbc_node_get_data(enode);
    if (!strcmp(group, "kprobes")) {
    if (trace_boot_add_kprobe_event(enode, event) < 0)
    return;
    }
    if (!strcmp(group, "synthetic")) {
    if (trace_boot_add_synth_event(enode, event) < 0)
    return;
    }
    if (!strcmp(group, "eprobes")) {
    if (trace_boot_add_eprobe_event(enode, group, event) < 0)
    return;
    }
    if (!strcmp(group, "fprobes")) {
    if (trace_boot_add_fprobe_event(enode, group, event) < 0)
    return;
    }
    if (!strcmp(group, "tracepoints") || !strcmp(group, "tprobes")) {
    if (trace_boot_add_tprobe_event(enode, group, event) < 0)
    return;
    }
    mutex_lock(&event_mutex);
    file = find_event_file(tr, group, event);
    if (!file) {
    pr_err!("Failed to find event: %s:%s\n", group, event);
// goto;
    }
    p = xbc_node_find_value(enode, "filter", core::ptr::null_mut());
    if (p && *p != '\0') {
    if (strscpy(buf, p, ARRAY_SIZE!(buf)) < 0) {
    pr_err!("filter string is too long: %s\n", p);
    }

    else if (apply_event_filter(file, buf) < 0) {
    pr_err!("Failed to apply filter: %s\n", buf);
    }
    }
    if (IS_ENABLED!(CONFIG_HIST_TRIGGERS)) {
    xbc_node_for_each_array_value(enode, "actions", anode, p) {
    if (strscpy(buf, p, ARRAY_SIZE!(buf)) < 0) {
    pr_err!("action string is too long: %s\n", p);
    }

    else if (trigger_process_regex(file, buf) < 0) {
    pr_err!("Failed to apply an action: %s\n", p);
    }
    }
    anode = xbc_node_find_subkey(enode, "hist");
    if (anode) {
    trace_boot_init_histograms(file, anode, buf, ARRAY_SIZE!(buf));
    }
    } else if (xbc_node_find_value(enode, "actions", core::ptr::null_mut())) {
    pr_err!("Failed to apply event actions because CONFIG_HIST_TRIGGERS is not set.\n");
    }
    if (xbc_node_find_value(enode, "enable", core::ptr::null_mut())) {
    if (trace_event_enable_disable(file, 1, 0) < 0) {
    pr_err!("Failed to enable event node: %s:%s\n",
    group, event);
    }
    }
// label;
    mutex_unlock(&event_mutex);
    }
    static void __init
    trace_boot_init_events(trace_array *tr, xbc_node *node)
    {
    let mut gnode = core::ptr::null_mut();
    let mut enode = core::ptr::null_mut();
    bool enable, enable_all = false;
pub static mut data: *mut c_void = core::ptr::null_mut();
    node = xbc_node_find_subkey(node, "event");
    if (!node) {
    return;
    }
// per-event key starts with "event.GROUP.EVENT"
    xbc_node_for_each_subkey(node, gnode) {
    data = xbc_node_get_data(gnode);
    if (!strcmp(data, "enable")) {
    enable_all = true;
    continue;
    }
    enable = false;
    xbc_node_for_each_subkey(gnode, enode) {
    data = xbc_node_get_data(enode);
    if (!strcmp(data, "enable")) {
    enable = true;
    continue;
    }
    trace_boot_init_one_event(tr, gnode, enode);
    }
// Event enablement must be done after event settings
    if (enable) {
    data = xbc_node_get_data(gnode);
    trace_array_set_clr_event(tr, data, core::ptr::null_mut(), true);
    }
    }
// Ditto
    if (enable_all) {
    trace_array_set_clr_event(tr, core::ptr::null_mut(), core::ptr::null_mut(), true);
    }
    }

    static void __init
    trace_boot_set_ftrace_filter(trace_array *tr, xbc_node *node)
    {
pub static mut anode: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
    xbc_node_for_each_array_value(node, "ftrace.filters", anode, p) {
    q = kstrdup(p, GFP_KERNEL);
    if (!q) {
    return;
    }
    if (ftrace_set_filter(tr.ops, q, strlen(q), 0) < 0) {
    pr_err!("Failed to add %s to ftrace filter\n", p);
    }
    else {
    ftrace_filter_param = true;
    }
    kfree(q);
    }
    xbc_node_for_each_array_value(node, "ftrace.notraces", anode, p) {
    q = kstrdup(p, GFP_KERNEL);
    if (!q) {
    return;
    }
    if (ftrace_set_notrace(tr.ops, q, strlen(q), 0) < 0) {
    pr_err!("Failed to add %s to ftrace filter\n", p);
    }
    else {
    ftrace_filter_param = true;
    }
    kfree(q);
    }
    }

    static void __init
    trace_boot_enable_tracer(trace_array *tr, xbc_node *node)
    {
pub static mut p: *mut c_void = core::ptr::null_mut();
    trace_boot_set_ftrace_filter(tr, node);
    p = xbc_node_find_value(node, "tracer", core::ptr::null_mut());
    if (p && *p != '\0') {
    if (trace_array_is_readonly(tr) || tracing_set_tracer(tr, p) < 0) {
    pr_err!("Failed to set given tracer: %s\n", p);
    }
    }
// Since tracer can free snapshot buffer, allocate snapshot here.
    if (xbc_node_find_value(node, "alloc_snapshot", core::ptr::null_mut())) {
    if (tracing_alloc_snapshot_instance(tr) < 0) {
    pr_err!("Failed to allocate snapshot buffer\n");
    }
    }
    }
    static void __init
    trace_boot_init_one_instance(trace_array *tr, xbc_node *node)
    {
    trace_boot_set_instance_options(tr, node);
    trace_boot_init_events(tr, node);
    trace_boot_enable_events(tr, node);
    trace_boot_enable_tracer(tr, node);
    }
    static void __init
    trace_boot_init_instances(xbc_node *node)
    {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    node = xbc_node_find_subkey(node, "instance");
    if (!node) {
    return;
    }
    xbc_node_for_each_subkey(node, inode) {
    p = xbc_node_get_data(inode);
    if (!p || *p == '\0') {
    continue;
    }
    tr = trace_array_get_by_name(p, core::ptr::null_mut());
    if (!tr) {
    pr_err!("Failed to get trace instance %s\n", p);
    continue;
    }
    trace_boot_init_one_instance(tr, inode);
    trace_array_put(tr);
    }
    }
#[no_mangle]
unsafe extern "C" fn trace_boot_init() -> c_int {
pub static mut trace_node: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    trace_node = xbc_find_node("ftrace");
    if (!trace_node) {
    return 0;
    }
    tr = top_trace_array();
    if (!tr) {
    return 0;
    }
// Global trace array is also one instance
    trace_boot_init_one_instance(tr, trace_node);
    trace_boot_init_instances(trace_node);
    disable_tracing_selftest("running boot-time tracing");
    return 0;
    }
//
// Start tracing at the end of core-initcall, so that it starts tracing
// from the beginning of postcore_initcall.
//
    core_initcall_sync!(trace_boot_init);