//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_printk.c
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
// trace binary printk
//
// Copyright (C) 2008 Lai Jiangshan <laijs@cn.fujitsu.com>
//

//
// modules trace_printk()'s formats are autosaved in struct trace_bprintk_fmt
// which are queued on trace_bprintk_fmt_list.
//
pub static mut trace_bprintk_fmt_list: usize = 0;
// serialize accesses to trace_bprintk_fmt_list
pub static mut btrace_mutex: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_bprintk_fmt {
    pub list: list_head,
    pub fmt: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn lookup_format(fmt: *mut c_char) -> *mut c_void {
pub static mut pos: *mut c_void = core::ptr::null_mut();
    if (!fmt) {
    return ERR_PTR(-EINVAL);
    }
    list_for_each_entry(pos, &trace_bprintk_fmt_list, list) {
    if (!strcmp(pos.fmt, fmt)) {
    return pos;
    }
    }
    return core::ptr::null_mut();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn hold_module_trace_bprintk_format(start: *const c_char, end: *const c_char) {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut fmt: *mut c_void = core::ptr::null_mut();
// allocate the trace_printk per cpu buffers
    if (start != end) {
    trace_printk_init_buffers();
    }
    mutex_lock(&btrace_mutex);
    while (iter < end) {
    let mut tb_fmt = lookup_format(*iter);
    if (tb_fmt) {
    if (!IS_ERR(tb_fmt)) {
// iter = tb_fmt->fmt;
    }
    continue;
    }
    fmt = core::ptr::null_mut();
    tb_fmt = kmalloc_obj(*tb_fmt);
    if (tb_fmt) {
    fmt = kstrdup(*iter, GFP_KERNEL);
    if (fmt) {
    list_add_tail(&tb_fmt.list, &trace_bprintk_fmt_list);
    tb_fmt.fmt = fmt;
    } else {
    kfree(tb_fmt);
    }
    }
// iter = fmt;
    }
    mutex_unlock(&btrace_mutex);
    }
    static int module_trace_bprintk_format_notify!(notifier_block *self,
    unsigned long val, void *data)
    {
    let mut mod = data;
    if (mod.num_trace_bprintk_fmt) {
    let mut start = mod.trace_bprintk_fmt_start;
    let mut end = start + mod.num_trace_bprintk_fmt;
    if (val == MODULE_STATE_COMING) {
    hold_module_trace_bprintk_format(start, end);
    }
    }
    return NOTIFY_OK;
    }
//
// The debugfs/tracing/printk_formats file maps the addresses with
// the ASCII formats that are used in the bprintk events in the
// buffer. For userspace tools to be able to decode the events from
// the buffer, they need to be able to map the address with the format.
//
// The addresses of the bprintk formats are in their own section
// __trace_printk_fmt. But for modules we copy them into a link list.
// The code to print the formats and their addresses passes around the
// address of the fmt string. If the fmt address passed into the seq
// functions is within the kernel core __trace_printk_fmt section, then
// it simply uses the next pointer in the list.
//
// When the fmt pointer is outside the kernel core __trace_printk_fmt
// section, then we need to read the link list pointers. The trick is
// we pass the address of the string to the seq function just like
// we do for the kernel core formats. To get back the structure that
// holds the format, we simply use container_of!() and then go to the
// next format in the list.
//
    static const char **
    find_next_mod_format(int start_index, void *v, const char **fmt, loff_t *pos)
    {
pub static mut mod_fmt: *mut c_void = core::ptr::null_mut();
    if (list_empty(&trace_bprintk_fmt_list)) {
    return core::ptr::null_mut();
    }
//
// v will point to the address of the fmt record from t_next
// v will be NULL from t_start.
// If this is the first pointer or called from start
// then we need to walk the list.
//
    if (!v || start_index == *pos) {
pub static mut p: *mut c_void = core::ptr::null_mut();
// search the module list
    list_for_each_entry(p, &trace_bprintk_fmt_list, list) {
    if (start_index == *pos) {
    return &p.fmt;
    }
    start_index += 1;
    }
// pos > index
    return core::ptr::null_mut();
    }
//
// v points to the address of the fmt field in the mod list
// structure that holds the module print format.
//
    mod_fmt = container_of!(v, typeof(*mod_fmt), fmt);
    if (mod_fmt.list.next == &trace_bprintk_fmt_list) {
    return core::ptr::null_mut();
    }
    mod_fmt = container_of!(mod_fmt.list.next, typeof(*mod_fmt), list);
    return &mod_fmt.fmt;
    }
#[no_mangle]
unsafe extern "C" fn format_mod_start() {
    mutex_lock(&btrace_mutex);
    }
#[no_mangle]
unsafe extern "C" fn format_mod_stop() {
    mutex_unlock(&btrace_mutex);
    }

    __init static int
    module_trace_bprintk_format_notify!(notifier_block *self,
    unsigned long val, void *data)
    {
    return NOTIFY_OK;
    }
    static inline const char **
    find_next_mod_format(int start_index, void *v, const char **fmt, loff_t *pos)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn format_mod_start() { }
#[no_mangle]
pub unsafe extern "C" fn format_mod_stop() { }

pub static mut trace_printk_enabled: bool  = true;
#[no_mangle]
pub unsafe extern "C" fn trace_printk_control(enabled: bool) {
    trace_printk_enabled = enabled;
    }
    __initdata_or_module static
pub static mut notifier_block: usize = 0;
    __printf(2, 3)
#[no_mangle]
pub unsafe extern "C" fn __trace_bprintk(ip: c_ulong, fmt: *const c_char, ...) -> c_int {
    let mut ret = 0;
    let mut ap;
    if (unlikely(!fmt)) {
    return 0;
    }
    if (!trace_printk_enabled) {
    return 0;
    }
    va_start(ap, fmt);
    ret = trace_vbprintk(ip, fmt, ap);
    va_end(ap);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__trace_bprintk);
#[no_mangle]
pub unsafe extern "C" fn __ftrace_vbprintk(ip: c_ulong, fmt: *const c_char, ap: va_list) -> c_int {
    if (unlikely(!fmt)) {
    return 0;
    }
    if (!trace_printk_enabled) {
    return 0;
    }
    return trace_vbprintk(ip, fmt, ap);
    }
    EXPORT_SYMBOL_GPL(__ftrace_vbprintk);
#[no_mangle]
pub unsafe extern "C" fn __trace_printk(ip: c_ulong, fmt: *const c_char, ...) -> c_int {
    let mut ret = 0;
    let mut ap;
    if (!trace_printk_enabled) {
    return 0;
    }
    va_start(ap, fmt);
    ret = trace_vprintk(ip, fmt, ap);
    va_end(ap);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__trace_printk);
#[no_mangle]
pub unsafe extern "C" fn __ftrace_vprintk(ip: c_ulong, fmt: *const c_char, ap: va_list) -> c_int {
    if (!trace_printk_enabled) {
    return 0;
    }
    return trace_vprintk(ip, fmt, ap);
    }
    EXPORT_SYMBOL_GPL(__ftrace_vprintk);
#[no_mangle]
pub unsafe extern "C" fn trace_is_tracepoint_string(str: *const c_char) -> bool {
    let mut ptr = __start___tracepoint_str;
    while (ptr < __stop___tracepoint_str) {
    if (str == *ptr) {
    return true;
    }
    }
    return false;
    }
    static const char **find_next(void *v, loff_t *pos)
    {
    let mut fmt = v;
    let mut start_index = 0;
    let mut last_index = 0;
    start_index = __stop___trace_bprintk_fmt - __start___trace_bprintk_fmt;
    if (*pos < start_index) {
    return __start___trace_bprintk_fmt + *pos;
    }
//
// The __tracepoint_str section is treated the same as the
// __trace_printk_fmt section. The difference is that the
// __trace_printk_fmt section should only be used by trace_printk()
// in a debugging environment, as if anything exists in that section
// the trace_prink() helper buffers are allocated, which would just
// waste space in a production environment.
//
// The __tracepoint_str sections on the other hand are used by
// tracepoints which need to map pointers to their strings to
// the ASCII text for userspace.
//
    last_index = start_index;
    start_index = __stop___tracepoint_str - __start___tracepoint_str;
    if (*pos < last_index + start_index) {
    return __start___tracepoint_str + (*pos - last_index);
    }
    start_index += last_index;
    return find_next_mod_format(start_index, v, fmt, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    format_mod_start();
    return find_next(core::ptr::null_mut(), pos);
    }
#[no_mangle]
pub unsafe extern "C" fn t_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return find_next(v, pos);
    }
#[no_mangle]
unsafe extern "C" fn t_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut fmt = v;
    let mut str = *fmt;
    let mut i = 0;
    if (!*fmt) {
    return 0;
    }
    seq_printf(m, "0x%lx : \"", *fmt);
//
// Tabs and new lines need to be converted.
//
    while (str[i]) {
    match (str[i]) {
    '\n' => {
    seq_puts(m, "\\n");
    // break;
    }
    '\t' => {
    seq_puts(m, "\\t");
    // break;
    }
    '\\' => {
    seq_putc(m, '\\');
    // break;
    }
    '"' => {
    seq_puts(m, "\\\"");
    // break;
    }
    _ => {
    seq_putc(m, str[i]);
    }
    }
    }
    seq_puts(m, "\"\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t_stop(m: *mut seq_file, p: *mut c_void) {
    format_mod_stop();
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_formats_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    return seq_open(file, &show_format_seq_ops);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn printk_binsafe(tr: *mut trace_array) -> __always_inline bool {
//
// The binary format of traceprintk can cause a crash if used
// by a buffer from another boot. Force the use of the
// non binary version of trace_printk if the trace_printk
// buffer is a boot mapped ring buffer.
//
    return !(tr.flags & TRACE_ARRAY_FL_BOOT);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_array_puts(tr: *mut trace_array, ip: c_ulong, str: *mut c_char, size: c_int) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    let mut alloc = 0;
    if (!(tr.trace_flags & TRACE_ITER(PRINTK))) {
    return 0;
    }
    if (unlikely(tracing_selftest_running &&
    (tr.flags & TRACE_ARRAY_FL_GLOBAL))) {
    return 0;
    }
    if (unlikely(tracing_disabled)) {
    return 0;
    }
    alloc = sizeof!(*entry) + size + 2; /* possible \n added */
    trace_ctx = tracing_gen_ctx();
    buffer = tr.array_buffer.buffer;
    guard(ring_buffer_nest)(buffer);
    event = __trace_buffer_lock_reserve(buffer, TRACE_PRINT, alloc,
    trace_ctx);
    if (!event) {
    return 0;
    }
    entry = ring_buffer_event_data(event);
    entry.ip = ip;
    memcpy(&entry.buf, str, size);
// Add a newline if necessary
    if (entry.buf[size - 1] != '\n') {
    entry.buf[size] = '\n';
    entry.buf[size + 1] = '\0';
    } else {
    entry.buf[size] = '\0';
    }
    __buffer_unlock_commit(buffer, event);
    ftrace_trace_stack(tr, buffer, trace_ctx, 4, core::ptr::null_mut());
    return size;
    }
    EXPORT_SYMBOL_GPL(__trace_array_puts);
//
// __trace_puts - write a constant string into the trace buffer.
// @ip:	   The address of the caller
// @str:   The constant string to write
//
#[no_mangle]
pub unsafe extern "C" fn __trace_puts(ip: c_ulong, str: *const c_char) -> c_int {
    return __trace_array_puts(printk_trace, ip, str, strlen(str));
    }
    EXPORT_SYMBOL_GPL(__trace_puts);
//
// __trace_bputs - write the pointer to a constant string into trace buffer
// @ip:	   The address of the caller
// @str:   The constant string to write to the buffer to
//
#[no_mangle]
pub unsafe extern "C" fn __trace_bputs(ip: c_ulong, str: *const c_char) -> c_int {
    let mut tr = READ_ONCE(printk_trace);
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut size: c_int = 0;
    if (!printk_binsafe(tr)) {
    return __trace_puts(ip, str);
    }
    if (!(tr.trace_flags & TRACE_ITER(PRINTK))) {
    return 0;
    }
    if (unlikely(tracing_selftest_running || tracing_disabled)) {
    return 0;
    }
    trace_ctx = tracing_gen_ctx();
    buffer = tr.array_buffer.buffer;
    guard(ring_buffer_nest)(buffer);
    event = __trace_buffer_lock_reserve(buffer, TRACE_BPUTS, size,
    trace_ctx);
    if (!event) {
    return 0;
    }
    entry = ring_buffer_event_data(event);
    entry.ip			= ip;
    entry.str			= str;
    __buffer_unlock_commit(buffer, event);
    ftrace_trace_stack(tr, buffer, trace_ctx, 4, core::ptr::null_mut());
    return 1;
    }
    EXPORT_SYMBOL_GPL(__trace_bputs);
// created for use with alloc_percpu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_buffer_struct {
    pub nesting: c_int,
    pub buffer: [c_char; 4][TRACE_BUF_SIZE],
}

    static struct trace_buffer_struct  *trace_percpu_buffer;
//
// This allows for lockless recording.  If we're nested too deeply, then
// this returns NULL.
//
#[no_mangle]
pub unsafe extern "C" fn get_trace_buf() -> *mut c_void {
    let mut buffer = this_cpu_ptr(trace_percpu_buffer);
    if (!trace_percpu_buffer || buffer.nesting >= 4) {
    return core::ptr::null_mut();
    }
    buffer.nesting += 1;
// Interrupts must see nesting incremented before we use the buffer
    barrier();
    return &buffer.buffer[buffer.nesting - 1][0];
    }
#[no_mangle]
unsafe extern "C" fn put_trace_buf() {
// Don't let the decrement of nesting leak before this
    barrier();
    this_cpu_dec(trace_percpu_buffer.nesting);
    }
#[no_mangle]
unsafe extern "C" fn alloc_percpu_trace_buffer() -> c_int {
    let mut buffers = core::ptr::null_mut();
    if (trace_percpu_buffer) {
    return 0;
    }
    buffers = alloc_percpu(trace_buffer_struct);
    if (MEM_FAIL(!buffers, "Could not allocate percpu trace_printk buffer")) {
    return -ENOMEM;
    }
    trace_percpu_buffer = buffers;
    return 0;
    }
    static int buffers_allocated;
#[no_mangle]
pub unsafe extern "C" fn trace_printk_init_buffers() {
    if (buffers_allocated) {
    return;
    }
    if (alloc_percpu_trace_buffer()) {
    return;
    }
// trace_printk() is for debug use only. Don't use it in production.
    pr_warn!("\n");
    pr_warn!("**********************************************************\n");
    pr_warn!("**   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **\n");
    pr_warn!("**                                                      **\n");
    pr_warn!("** trace_printk() being used. Allocating extra memory.  **\n");
    pr_warn!("**                                                      **\n");
    pr_warn!("** This means that this is a DEBUG kernel and it is     **\n");
    pr_warn!("** unsafe for production use.                           **\n");
    pr_warn!("**                                                      **\n");
    pr_warn!("** If you see this message and you are not debugging    **\n");
    pr_warn!("** the kernel, report this immediately to your vendor!  **\n");
    pr_warn!("**                                                      **\n");
    pr_warn!("**   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **\n");
    pr_warn!("**********************************************************\n");
// Expand the buffers to set size
    if (tracing_update_buffers(core::ptr::null_mut()) < 0) {
    pr_err!("Failed to expand tracing buffers for trace_printk() calls\n");
    }
    else {
    buffers_allocated = 1;
    }
//
// trace_printk_init_buffers() can be called by modules.
// If that happens, then we need to start cmdline recording
// directly here.
//
    if (system_state == SYSTEM_RUNNING) {
    tracing_start_cmdline_record();
    }
    }
    EXPORT_SYMBOL_GPL(trace_printk_init_buffers);
#[no_mangle]
pub unsafe extern "C" fn trace_printk_start_comm() {
// Start tracing comms if trace printk is set
    if (!buffers_allocated) {
    return;
    }
    tracing_start_cmdline_record();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_printk_start_stop_comm(enabled: c_int) {
    if (!buffers_allocated) {
    return;
    }
    if (enabled) {
    tracing_start_cmdline_record();
    }
    else {
    tracing_stop_cmdline_record();
    }
    }
//
// trace_vbprintk - write binary msg to tracing buffer
// @ip:    The address of the caller
// @fmt:   The string format to write to the buffer
// @args:  Arguments for @fmt
//
#[no_mangle]
pub unsafe extern "C" fn trace_vbprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
    let mut tr = READ_ONCE(printk_trace);
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut tbuffer: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    if (!printk_binsafe(tr)) {
    return trace_vprintk(ip, fmt, args);
    }
    if (unlikely(tracing_selftest_running || tracing_disabled)) {
    return 0;
    }
// Don't pollute graph traces with trace_vprintk internals
    pause_graph_tracing();
    trace_ctx = tracing_gen_ctx();
    guard(preempt_notrace)();
    tbuffer = get_trace_buf();
    if (!tbuffer) {
    len = 0;
// goto;
    }
    len = vbin_printf(tbuffer, TRACE_BUF_SIZE/sizeof!(int), fmt, args);
    if (len > TRACE_BUF_SIZE/sizeof!(int) || len < 0) {
// goto;
    }
    size = sizeof!(*entry) + sizeof!(u32) * len;
    buffer = tr.array_buffer.buffer;
    scoped_guard(ring_buffer_nest, buffer) {
    event = __trace_buffer_lock_reserve(buffer, TRACE_BPRINT, size,
    trace_ctx);
    if (!event) {
// goto;
    }
    entry = ring_buffer_event_data(event);
    entry.ip			= ip;
    entry.fmt			= fmt;
    memcpy(entry.buf, tbuffer, sizeof!(u32) * len);
    __buffer_unlock_commit(buffer, event);
    ftrace_trace_stack(tr, buffer, trace_ctx, 6, core::ptr::null_mut());
    }
// label;
    put_trace_buf();
// label;
    unpause_graph_tracing();
    return len;
    }
    EXPORT_SYMBOL_GPL(trace_vbprintk);
#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 3, _arg: 0) -> static {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut tbuffer: *mut c_void = core::ptr::null_mut();
    if (unlikely(tracing_disabled)) {
    return 0;
    }
// Don't pollute graph traces with trace_vprintk internals
    pause_graph_tracing();
    trace_ctx = tracing_gen_ctx();
    guard(preempt_notrace)();
    tbuffer = get_trace_buf();
    if (!tbuffer) {
    len = 0;
// goto;
    }
    len = vscnprintf(tbuffer, TRACE_BUF_SIZE, fmt, args);
    size = sizeof!(*entry) + len + 1;
    scoped_guard(ring_buffer_nest, buffer) {
    event = __trace_buffer_lock_reserve(buffer, TRACE_PRINT, size,
    trace_ctx);
    if (!event) {
// goto;
    }
    entry = ring_buffer_event_data(event);
    entry.ip = ip;
    memcpy(&entry.buf, tbuffer, len + 1);
    __buffer_unlock_commit(buffer, event);
    ftrace_trace_stack(printk_trace, buffer, trace_ctx, 6, core::ptr::null_mut());
    }
// label;
    put_trace_buf();
// label;
    unpause_graph_tracing();
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_array_vprintk(tr: *mut trace_array, ip: c_ulong, fmt: *mut c_char, args: va_list) -> c_int {
    if (tracing_selftest_running && (tr.flags & TRACE_ARRAY_FL_GLOBAL)) {
    return 0;
    }
    return __trace_array_vprintk(tr.array_buffer.buffer, ip, fmt, args);
    }
//
// trace_array_printk - Print a message to a specific instance
// @tr: The instance trace_array descriptor
// @ip: The instruction pointer that this is called from.
// @fmt: The format to print (printf format)
//
// If a subsystem sets up its own instance, they have the right to
// printk strings into their tracing instance buffer using this
// function. Note, this function will not write into the top level
// buffer (use trace_printk() for that), as writing into the top level
// buffer should only have events that can be individually disabled.
// trace_printk() is only used for debugging a kernel, and should not
// be ever incorporated in normal use.
//
// trace_array_printk() can be used, as it will not add noise to the
// top level tracing buffer.
//
// Note, trace_array_init_printk() must be called on @tr before this
// can be used.
//
#[no_mangle]
pub unsafe extern "C" fn trace_array_printk(tr: *mut trace_array, ip: c_ulong, fmt: *mut c_char) -> c_int {
    let mut ret = 0;
    let mut ap;
    if (!tr) {
    return -ENOENT;
    }
// This is only allowed for created instances
    if (tr.flags & TRACE_ARRAY_FL_GLOBAL) {
    return 0;
    }
    if (!(tr.trace_flags & TRACE_ITER(PRINTK))) {
    return 0;
    }
    va_start(ap, fmt);
    ret = trace_array_vprintk(tr, ip, fmt, ap);
    va_end(ap);
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_array_printk);
//
// trace_array_init_printk - Initialize buffers for trace_array_printk()
// @tr: The trace array to initialize the buffers for
//
// As trace_array_printk() only writes into instances, they are OK to
// have in the kernel (unlike trace_printk()). This needs to be called
// before trace_array_printk() can be used on a trace_array.
//
#[no_mangle]
pub unsafe extern "C" fn trace_array_init_printk(tr: *mut trace_array) -> c_int {
    if (!tr) {
    return -ENOENT;
    }
// This is only allowed for created instances
    if (tr.flags & TRACE_ARRAY_FL_GLOBAL) {
    return -EINVAL;
    }
    return alloc_percpu_trace_buffer();
    }
    EXPORT_SYMBOL_GPL(trace_array_init_printk);
#[no_mangle]
pub unsafe extern "C" fn trace_array_printk_buf(buffer: *mut trace_buffer, ip: c_ulong, fmt: *mut c_char) -> c_int {
    let mut ret = 0;
    let mut ap;
    if (!(printk_trace.trace_flags & TRACE_ITER(PRINTK))) {
    return 0;
    }
    va_start(ap, fmt);
    ret = __trace_array_vprintk(buffer, ip, fmt, ap);
    va_end(ap);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_vprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int {
    return trace_array_vprintk(printk_trace, ip, fmt, args);
    }
    EXPORT_SYMBOL_GPL(trace_vprintk);
#[no_mangle]
unsafe extern "C" fn init_trace_printk_function_export() -> __init int {
    let mut ret = 0;
    ret = tracing_init_dentry();
    if (ret) {
    return 0;
    }
    trace_create_file("printk_formats", TRACE_MODE_READ, core::ptr::null_mut(),
    core::ptr::null_mut(), &ftrace_formats_fops);
    return 0;
    }
    fs_initcall!(init_trace_printk_function_export);
#[no_mangle]
unsafe extern "C" fn init_trace_printk() -> __init int {
    return register_module_notifier(&module_trace_bprintk_format_nb);
    }
    early_initcall!(init_trace_printk);