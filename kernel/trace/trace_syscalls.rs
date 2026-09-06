//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_syscalls.c
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

pub static mut syscall_trace_lock: usize = 0;
// forward_decl: syscall_enter_register;
// forward_decl: syscall_exit_register;
#[no_mangle]
pub unsafe extern "C" fn syscall_get_enter_fields(call: *mut trace_event_call) -> *mut c_void {
    let mut entry = call.data;
    return &entry.enter_fields;
    }
    extern struct syscall_metadata *__start_syscalls_metadata[];
    extern struct syscall_metadata *__stop_syscalls_metadata[];
pub static mut syscalls_metadata_sparse: usize = 0;
pub static mut syscalls_metadata: *mut c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn arch_syscall_match_sym_name(sym: *const c_char, name: *const c_char) -> bool {
//
// Only compare after the "sys" prefix. Archs that use
// syscall wrappers may have syscalls symbols aliases prefixed
// with ".SyS" or ".sys" instead of "sys", leading to an unwanted
// mismatch.
//
    return !strcmp(sym + 3, name + 3);
    }

//
// Some architectures that allow for 32bit applications
// to run on a 64bit kernel, do not map the syscalls for
// the 32bit tasks the same as they do for 64bit tasks.
//
// *cough*x86*cough
//
// In such a case, instead of reporting the wrong syscalls,
// simply ignore them.
//
// For an arch to ignore the compat syscalls it needs to
// define ARCH_TRACE_IGNORE_COMPAT_SYSCALLS as well as
// define the function arch_trace_is_compat_syscall() to let
// the tracing system know that it should ignore it.
//
#[no_mangle]
pub unsafe extern "C" fn trace_get_syscall_nr(task: *mut task_struct, regs: *mut pt_regs) -> c_int {
    if (unlikely(arch_trace_is_compat_syscall(regs))) {
    return -1;
    }
    return syscall_get_nr(task, regs);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: trace_get_syscall_nr
pub unsafe extern "C" fn trace_get_syscall_nr_dup(task: *mut task_struct, regs: *mut pt_regs) -> c_int {
    return syscall_get_nr(task, regs);
    }

#[no_mangle]
pub unsafe extern "C" fn find_syscall_meta(syscall: c_ulong) -> *mut c_void {
pub static mut start: *mut c_void = core::ptr::null_mut();
pub static mut stop: *mut c_void = core::ptr::null_mut();
    char str[KSYM_SYMBOL_LEN];
    start = __start_syscalls_metadata;
    stop = __stop_syscalls_metadata;
    kallsyms_lookup(syscall, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), str);
    if (arch_syscall_match_sym_name(str, "sys_ni_syscall")) {
    return core::ptr::null_mut();
    }
    while (start < stop) {
    if ((*start).name && arch_syscall_match_sym_name(str, (*start).name)) {
pub static mut start: *mut c_void = core::ptr::null_mut();
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_nr_to_meta(nr: c_int) -> *mut c_void {
    if (IS_ENABLED!(CONFIG_HAVE_SPARSE_SYSCALL_NR)) {
    return xa_load(&syscalls_metadata_sparse, (unsigned long)nr);
    }
    if (!syscalls_metadata || nr >= NR_syscalls || nr < 0) {
    return core::ptr::null_mut();
    }
    return syscalls_metadata[nr];
    }
    const char *get_syscall_name(int syscall)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    entry = syscall_nr_to_meta(syscall);
    if (!entry) {
    return core::ptr::null_mut();
    }
    return entry.name;
    }
// Added to user strings or arrays when max limit is reached

#[no_mangle]
pub unsafe extern "C" fn get_dynamic_len_ptr(trace: *mut syscall_trace_enter, entry: *mut syscall_metadata, offset_p: *mut c_int, len_p: *mut c_int, ptr_p: *mut *mut c_uchar) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    let mut val = 0;
// This arg points to a user space string
    ptr = trace.args + sizeof!(long) * entry.nb_args + offset;
    val = *ptr;
// The value is a dynamic string (len << 16 | offset)
    ptr = trace + (val & 0xffff);
// len_p = val >> 16;
    offset += 4;
// ptr_p = ptr;
// offset_p = offset;
    }
    static enum print_line_t
    sys_enter_openat_print(syscall_trace_enter *trace, syscall_metadata *entry, trace_seq *s, trace_event *event)
    {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    let mut bits = 0;
    let mut len = 0;
pub static mut done: bool = false;
pub static mut trace_print_flags: usize = 0;
    trace_seq_printf(s, "%s(", entry.name);
    while (!done && i < entry.nb_args) {
    if (trace_seq_has_overflowed(s)) {
// goto;
    }
    if (i) {
    trace_seq_puts(s, ", ");
    }
    match (i) {
    2 => {
    bits = trace.args[2];
    trace_seq_puts(s, "flags: ");
// No need to show mode when not creating the file
    if (!(bits & (O_CREAT|O_TMPFILE))) {
    done = true;
    }
    if (!(bits & O_ACCMODE)) {
    if (!bits) {
    trace_seq_puts(s, "O_RDONLY");
    continue;
    }
    trace_seq_puts(s, "O_RDONLY|");
    }
    trace_print_flags_seq(s, "|", bits, __flags, ARRAY_SIZE!(__flags));
//
// trace_print_flags_seq() adds a '\0' to the
// buffer, but this needs to append more to the seq.
//
    if (!trace_seq_has_overflowed(s)) {
    trace_seq_pop(s);
    }
    continue;
    }
    3 => {
    trace_seq_printf(s, "%s: 0%03o", entry.args[i],
    (unsigned int)trace.args[i]);
    continue;
    }
    }
    trace_seq_printf(s, "%s: %lu", entry.args[i],
    trace.args[i]);
    if (!(BIT(i) & entry.user_mask)) {
    continue;
    }
    get_dynamic_len_ptr(trace, entry, &offset, &len, &ptr);
    trace_seq_printf(s, " \"%.*s\"", len, ptr);
    }
    trace_seq_putc(s, ')');
// label;
    trace_seq_putc(s, '\n');
    return trace_handle_return(s);
    }
    static enum print_line_t
    print_syscall_enter(trace_iterator *iter, int flags, trace_event *event)
    {
    let mut tr = iter.tr;
    let mut s = &iter.seq;
    let mut ent = iter.ent;
pub static mut trace: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut syscall = 0;
    let mut val = 0;
    let mut len = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    trace = (typeof(trace))ent;
    syscall = trace.nr;
    entry = syscall_nr_to_meta(syscall);
    if (!entry) {
// goto;
    }
    if (entry.enter_event.event.type != ent.type) {
    WARN_ON_ONCE!(1);
// goto;
    }
    match (entry.syscall_nr) {
    __NR_openat => {
    if (!tr || !(tr.trace_flags & TRACE_ITER(VERBOSE))) {
    return sys_enter_openat_print(trace, entry, s, event);
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    trace_seq_printf(s, "%s(", entry.name);
    while (i < entry.nb_args) {
pub static mut printable: bool = false;
pub static mut str: *mut c_void = core::ptr::null_mut();
    if (trace_seq_has_overflowed(s)) {
// goto;
    }
    if (i) {
    trace_seq_puts(s, ", ");
    }
// parameter types
    if (tr && tr.trace_flags & TRACE_ITER(VERBOSE)) {
    trace_seq_printf(s, "%s ", entry.types[i]);
    }
// parameter values
    if (trace.args[i] < 10) {
    trace_seq_printf(s, "%s: %lu", entry.args[i],
    trace.args[i]);
    }
    else {
    trace_seq_printf(s, "%s: 0x%lx", entry.args[i],
    trace.args[i]);
    }
    if (!(BIT(i) & entry.user_mask)) {
    continue;
    }
    get_dynamic_len_ptr(trace, entry, &offset, &len, &ptr);
    if (entry.user_arg_size < 0 || entry.user_arg_is_str) {
    trace_seq_printf(s, " \"%.*s\"", len, ptr);
    continue;
    }
    val = trace.args[entry.user_arg_size];
    str = ptr;
    trace_seq_puts(s, " (");
    while (x < len) {
    if (isascii(*ptr) && isprint(*ptr)) {
    printable = true;
    }
    if (x) {
    trace_seq_putc(s, ':');
    }
    trace_seq_printf(s, "%02x", *ptr);
    }
    if (len < val) {
    trace_seq_printf(s, ", %s", EXTRA);
    }
    trace_seq_putc(s, ')');
// If nothing is printable, don't bother printing anything
    if (!printable) {
    continue;
    }
    trace_seq_puts(s, " \"");
    while (x < len) {
    if (isascii(str[x]) && isprint(str[x])) {
    trace_seq_putc(s, str[x]);
    }
    else {
    trace_seq_putc(s, '.');
    }
    }
    if (len < val) {
    trace_seq_printf(s, "\"%s", EXTRA);
    }
    else {
    trace_seq_putc(s, '"');
    }
    }
    trace_seq_putc(s, ')');
// label;
    trace_seq_putc(s, '\n');
    return trace_handle_return(s);
    }
    static enum print_line_t
    print_syscall_exit(trace_iterator *iter, int flags, trace_event *event)
    {
    let mut s = &iter.seq;
    let mut ent = iter.ent;
pub static mut trace: *mut c_void = core::ptr::null_mut();
    let mut syscall = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    trace = (typeof(trace))ent;
    syscall = trace.nr;
    entry = syscall_nr_to_meta(syscall);
    if (!entry) {
    trace_seq_putc(s, '\n');
// goto;
    }
    if (entry.exit_event.event.type != ent.type) {
    WARN_ON_ONCE!(1);
    return TRACE_TYPE_UNHANDLED;
    }
    trace_seq_printf(s, "%s . 0x%lx\n", entry.name,
    trace.ret);
// label;
    return trace_handle_return(s);
    }

    .type = #_type, .name = #_name,					
    .size = sizeof!(_type), .align = __alignof__(_type),		
    .is_signed = is_signed_type(_type), .filter_type = FILTER_OTHER }
// When len=0, we just calculate the needed length

    static int __init
    sys_enter_openat_print_fmt(syscall_metadata *entry, char *buf, int len)
    {
pub static mut pos: c_int = 0;
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "\"dfd: 0x%%08lx, filename: 0x%%08lx \\\"%%s\\\", flags: %%s%%s, mode: 0%%03o\",");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " ((unsigned long)(REC.dfd)),");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " ((unsigned long)(REC.filename)),");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " __get_str(__filename_val),");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " (REC.flags & ~3) && !(REC.flags & 3) ? \"O_RDONLY|\" : \"\", ");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " REC.flags ? __print_flags(REC.flags, \"|\", ");
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_WRONLY\" }, ", O_WRONLY);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_RDWR\" }, ", O_RDWR);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_CREAT\" }, ", O_CREAT);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_EXCL\" }, ", O_EXCL);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_NOCTTY\" }, ", O_NOCTTY);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_TRUNC\" }, ", O_TRUNC);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_APPEND\" }, ", O_APPEND);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_NONBLOCK\" }, ", O_NONBLOCK);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_DSYNC\" }, ", O_DSYNC);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_DIRECT\" }, ", O_DIRECT);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_LARGEFILE\" }, ", O_LARGEFILE);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_DIRECTORY\" }, ", O_DIRECTORY);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_NOFOLLOW\" }, ", O_NOFOLLOW);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_NOATIME\" }, ", O_NOATIME);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    "{ 0x%x, \"O_CLOEXEC\" }) : \"O_RDONLY\", ", O_CLOEXEC);
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    " ((unsigned long)(REC.mode))");
    return pos;
    }
    static int __init
    __set_enter_print_fmt(syscall_metadata *entry, char *buf, int len)
    {
pub static mut is_string: bool = false;
    let mut i = 0;
pub static mut pos: c_int = 0;
    match (entry.syscall_nr) {
    __NR_openat => {
    return sys_enter_openat_print_fmt(entry, buf, len);
    }
    _ => {
    // break;
    }
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    while (i < entry.nb_args) {
    if (i) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, ", ");
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s: 0x%%0%zulx",
    entry.args[i], sizeof!(unsigned long));
    if (!(BIT(i) & entry.user_mask)) {
    continue;
    }
// Add the format for the user space string or array
    if (entry.user_arg_size < 0 || is_string) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, " \\\"%%s\\\"");
    }
    else {
    pos += snprintf(buf + pos, LEN_OR_ZERO, " (%%s)");
    }
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    while (i < entry.nb_args) {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", ((unsigned long)(REC.%s))", entry.args[i]);
    if (!(BIT(i) & entry.user_mask)) {
    continue;
    }
// The user space data for arg has name __<arg>_val
    if (entry.user_arg_size < 0 || is_string) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, ", __get_str(__%s_val)",
    entry.args[i]);
    } else {
    pos += snprintf(buf + pos, LEN_OR_ZERO, ", __print_dynamic_array(__%s_val, 1)",
    entry.args[i]);
    }
    }

// return the length of print_fmt
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn set_syscall_print_fmt(call: *mut trace_event_call) -> c_int {
pub static mut print_fmt: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut entry = call.data;
    if (entry.enter_event != call) {
    call.print_fmt = "\"0x%lx\", REC.ret";
    return 0;
    }
// First: called with 0 length to calculate the needed length
    len = __set_enter_print_fmt(entry, core::ptr::null_mut(), 0);
    print_fmt = kmalloc(len + 1, GFP_KERNEL);
    if (!print_fmt) {
    return -ENOMEM;
    }
// Second: actually write the @print_fmt
    __set_enter_print_fmt(entry, print_fmt, len + 1);
    call.print_fmt = print_fmt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_syscall_print_fmt(call: *mut trace_event_call)  {
    let mut entry = call.data;
    if (entry.enter_event == call) {
    kfree(call.print_fmt);
    }
    }
#[no_mangle]
unsafe extern "C" fn syscall_enter_define_fields(call: *mut trace_event_call) -> c_int {
pub static mut trace: usize = 0;
    let mut meta = call.data;
    let mut mask = 0;
pub static mut arg: *mut c_void = core::ptr::null_mut();
pub static mut offset: c_int = 0;
pub static mut ret: c_int = 0;
    let mut len = 0;
    let mut i = 0;
    while (i < meta.nb_args) {
    ret = trace_define_field(call, meta.types[i],
    meta.args[i], offset,
    sizeof!(unsigned long), 0,
    FILTER_OTHER);
    if (ret) {
    break;
    }
    offset += sizeof!(unsigned long);
    }
    if (ret || !meta.user_mask) {
    return ret;
    }
    mask = meta.user_mask;
    while (mask) {
pub static mut idx: c_int = 0;
    mask &= ~BIT(idx);
//
// User space data is faulted into a temporary buffer and then
// added as a dynamic string or array to the end of the event.
// The user space data name for the arg pointer is
// "__<arg>_val".
//
    len = strlen(meta.args[idx]) + sizeof!("___val");
    arg = kmalloc(len, GFP_KERNEL);
    if (WARN_ON_ONCE!(!arg)) {
    meta.user_mask = 0;
    return -ENOMEM;
    }
    snprintf(arg, len, "__%s_val", meta.args[idx]);
    ret = trace_define_field(call, "__data_loc char[]",
    arg, offset, sizeof!(int), 0,
    FILTER_OTHER);
    if (ret) {
    kfree(arg);
    break;
    }
    offset += 4;
    }
    return ret;
    }
//
// Create a per CPU temporary buffer to copy user space pointers into.
//
// SYSCALL_FAULT_USER_MAX is the amount to copy from user space.
// (defined in kernel/trace/trace.h)
// SYSCALL_FAULT_ARG_SZ is the amount to copy from user space plus the
// nul terminating byte and possibly appended EXTRA (4 bytes).
//
// SYSCALL_FAULT_BUF_SZ holds the size of the per CPU buffer to use
// to copy memory from user space addresses into that will hold
// 3 args as only 3 args are allowed to be copied from system calls.
//

pub const SYSCALL_FAULT_MAX_CNT: c_int = 3;

// Use the tracing per CPU buffer infrastructure to copy from user space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_user_buffer {
    pub buf: trace_user_buf_info,
    pub rcu: rcu_head,
}

pub static mut syscall_buffer: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn syscall_fault_buffer_enable() -> c_int {
pub static mut sbuf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    lockdep_assert_held(&syscall_trace_lock);
    if (syscall_buffer) {
    trace_user_fault_get(&syscall_buffer.buf);
    return 0;
    }
    sbuf = kmalloc_obj(*sbuf);
    if (!sbuf) {
    return -ENOMEM;
    }
    ret = trace_user_fault_init(&sbuf.buf, SYSCALL_FAULT_BUF_SZ);
    if (ret < 0) {
    kfree(sbuf);
    return ret;
    }
    WRITE_ONCE(syscall_buffer, sbuf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcu_free_syscall_buffer(rcu: *mut rcu_head) {
    let mut sbuf = container_of!(rcu, syscall_user_buffer, rcu);
    trace_user_fault_destroy(&sbuf.buf);
    kfree(sbuf);
    }
#[no_mangle]
unsafe extern "C" fn syscall_fault_buffer_disable() {
    let mut sbuf = syscall_buffer;
    lockdep_assert_held(&syscall_trace_lock);
    if (trace_user_fault_put(&sbuf.buf)) {
    return;
    }
    WRITE_ONCE(syscall_buffer, core::ptr::null_mut());
    call_rcu_tasks_trace(&sbuf.rcu, rcu_free_syscall_buffer);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_args {
    pub ptr_array: [*mut c_char; SYSCALL_FAULT_MAX_CNT],
    pub read: [c_int; SYSCALL_FAULT_MAX_CNT],
    pub uargs: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn syscall_copy_user(buf: *mut c_char, ptr: *mut c_char, size: size_t, data: *mut c_void) -> c_int {
    let mut args = data;
    let mut ret = 0;
    while (i < args.uargs) {
    ptr = args.ptr_array[i];
    ret = strncpy_from_user(buf, ptr, size);
    args.read[i] = ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_copy_user_array(buf: *mut c_char, ptr: *mut c_char, size: size_t, data: *mut c_void) -> c_int {
    let mut args = data;
    let mut ret = 0;
    while (i < args.uargs) {
    ptr = args.ptr_array[i];
    ret = __copy_from_user(buf, ptr, size);
    args.read[i] = ret ? -1 : size;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_fault_user(buf_size: c_uint, sys_data: *mut syscall_metadata, sbuf: *mut syscall_user_buffer, args: *mut c_ulong) -> *mut c_void {
pub static mut syscall_copy: trace_user_buf_copy = 0;
pub static mut mask: c_ulong = 0;
pub static mut size: c_ulong = 0;
pub static mut sargs: usize = 0;
pub static mut array: bool = false;
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
pub static mut i: c_int = 0;
// The extra is appended to the user data in the buffer
    BUILD_BUG_ON!(SYSCALL_FAULT_USER_MAX + sizeof!(EXTRA) >=
    SYSCALL_FAULT_ARG_SZ);
//
// If this system call event has a size argument, use
// it to define how much of user space memory to read,
// and read it as an array and not a string.
//
    if (sys_data.user_arg_size >= 0) {
    array = true;
    size = args[sys_data.user_arg_size];
    if (size > SYSCALL_FAULT_ARG_SZ - 1) {
    size = SYSCALL_FAULT_ARG_SZ - 1;
    }
    syscall_copy = syscall_copy_user_array;
    }
    while (mask) {
pub static mut idx: c_int = 0;
    mask &= ~BIT(idx);
    if (WARN_ON_ONCE!(i == SYSCALL_FAULT_MAX_CNT)) {
    break;
    }
// Get the pointer to user space memory to read
    sargs.ptr_array[i++] = args[idx];
    }
    sargs.uargs = i;
// Clear the values that are not used
    while (i < SYSCALL_FAULT_MAX_CNT) {
    data_size[i] = -1; /* Denotes no pointer */
    }
// A zero size means do not even try
    if (!buf_size) {
    return core::ptr::null_mut();
    }
    buffer = trace_user_fault_read(&sbuf.buf, core::ptr::null_mut(), size,
    syscall_copy, &sargs);
    if (!buffer) {
    return core::ptr::null_mut();
    }
    buf = buffer;
    while (i < sargs.uargs) {
    ret = sargs.read[i];
    if (ret < 0) {
    continue;
    }
    buf[ret] = '\0';
// For strings, replace any non-printable characters with '.'
    if (!array) {
    while (x < ret) {
    if (!isprint(buf[x])) {
    buf[x] = '.';
    }
    }
    size = min(buf_size, SYSCALL_FAULT_USER_MAX);
//
// If the text was truncated due to our max limit,
// add "..." to the string.
//
    if (ret > size) {
    strscpy(buf + size, EXTRA, sizeof!(EXTRA));
    ret = size + sizeof!(EXTRA);
    } else {
    buf[ret++] = '\0';
    }
    } else {
    ret = min((unsigned int)ret, buf_size);
    }
    data_size[i] = ret;
    }
    return buffer;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_get_data(sys_data: *mut syscall_metadata, args: *mut c_ulong, buffer: *mut *mut c_char, size: *mut c_int, user_sizes: *mut c_int, uargs: *mut c_int, buf_size: c_int) -> c_int {
pub static mut sbuf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// If the syscall_buffer is NULL, tracing is being shutdown
    sbuf = READ_ONCE(syscall_buffer);
    if (!sbuf) {
    return -1;
    }
// buffer = sys_fault_user(buf_size, sys_data, sbuf, args, user_sizes);
//
// user_size is the amount of data to append.
// Need to add 4 for the meta field that points to
// the user memory at the end of the event and also
// stores its size.
//
    while (i < SYSCALL_FAULT_MAX_CNT) {
    if (user_sizes[i] < 0) {
    break;
    }
// size += user_sizes[i] + 4;
    }
// Save the number of user read arguments of this syscall
// uargs = i;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_put_data(sys_data: *mut syscall_metadata, entry: *mut syscall_trace_enter, buffer: *mut c_char, size: c_int, user_sizes: *mut c_int, uargs: c_int) {
    let mut buf = buffer;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
//
// Set the pointer to point to the meta data of the event
// that has information about the stored user space memory.
//
    ptr = entry.args + sizeof!(unsigned long) * sys_data.nb_args;
//
// The meta data will store the offset of the user data from
// the beginning of the event. That is after the static arguments
// and the meta data fields.
//
    val = (ptr - entry) + 4 * uargs;
    while (i < uargs) {
    if (i) {
    val += user_sizes[i - 1];
    }
// Store the offset and the size into the meta data
// ptr = val | (user_sizes[i] << 16);
// Skip the meta data
    ptr += 4;
    }
    while (i < uargs) {
// Nothing to do if the user space was empty or faulted
    if (!user_sizes[i]) {
    continue;
    }
    memcpy(ptr, buf, user_sizes[i]);
    ptr += user_sizes[i];
    }
    }
#[no_mangle]
unsafe extern "C" fn ftrace_syscall_enter(data: *mut c_void, regs: *mut pt_regs, id: c_long) {
    let mut tr = data;
pub static mut trace_file: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut sys_data: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
    unsigned long args[6];
pub static mut user_ptr: *mut c_void = core::ptr::null_mut();
    int user_sizes[SYSCALL_FAULT_MAX_CNT] = {};
    let mut syscall_nr = 0;
pub static mut size: c_int = 0;
pub static mut uargs: c_int = 0;
    let mut mayfault = 0;
//
// Syscall probe called with preemption enabled, but the ring
// buffer and per-cpu data require preemption to be disabled.
//
    might_fault();
    syscall_nr = trace_get_syscall_nr(current, regs);
    if (syscall_nr < 0 || syscall_nr >= NR_syscalls) {
    return;
    }
    trace_file = READ_ONCE(tr.enter_syscall_files[syscall_nr]);
    if (!trace_file) {
    return;
    }
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    sys_data = syscall_nr_to_meta(syscall_nr);
    if (!sys_data) {
    return;
    }
// Check if this syscall event faults in user space memory
    mayfault = sys_data.user_mask != 0;
    guard(preempt_notrace)();
    syscall_get_arguments(current, regs, args);
    if (mayfault) {
    if (syscall_get_data(sys_data, args, &user_ptr,
    &size, user_sizes, &uargs, tr.syscall_buf_sz) < 0) {
    return;
    }
    }
    size += sizeof!(*entry) + sizeof!(unsigned long) * sys_data.nb_args;
    entry = trace_event_buffer_reserve(&fbuffer, trace_file, size);
    if (!entry) {
    return;
    }
    entry = ring_buffer_event_data(fbuffer.event);
    entry.nr = syscall_nr;
    memcpy(entry.args, args, sizeof!(unsigned long) * sys_data.nb_args);
    if (mayfault) {
    syscall_put_data(sys_data, entry, user_ptr, size, user_sizes, uargs);
    }
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_syscall_exit(data: *mut c_void, regs: *mut pt_regs, ret: c_long) {
    let mut tr = data;
pub static mut trace_file: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut sys_data: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
    let mut syscall_nr = 0;
//
// Syscall probe called with preemption enabled, but the ring
// buffer and per-cpu data require preemption to be disabled.
//
    might_fault();
    guard(preempt_notrace)();
    syscall_nr = trace_get_syscall_nr(current, regs);
    if (syscall_nr < 0 || syscall_nr >= NR_syscalls) {
    return;
    }
    trace_file = READ_ONCE(tr.exit_syscall_files[syscall_nr]);
    if (!trace_file) {
    return;
    }
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    sys_data = syscall_nr_to_meta(syscall_nr);
    if (!sys_data) {
    return;
    }
    entry = trace_event_buffer_reserve(&fbuffer, trace_file, sizeof!(*entry));
    if (!entry) {
    return;
    }
    entry = ring_buffer_event_data(fbuffer.event);
    entry.nr = syscall_nr;
    entry.ret = syscall_get_return_value(current, regs);
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_event_syscall_enter(file: *mut trace_event_file, call: *mut trace_event_call) -> c_int {
    let mut sys_data = call.data;
    let mut tr = file.tr;
pub static mut ret: c_int = 0;
    let mut num = 0;
    num = sys_data.syscall_nr;
    if (WARN_ON_ONCE!(num < 0 || num >= NR_syscalls)) {
    return -ENOSYS;
    }
    guard(mutex)(&syscall_trace_lock);
    if (sys_data.user_mask) {
    ret = syscall_fault_buffer_enable();
    if (ret < 0) {
    return ret;
    }
    }
    if (!tr.sys_refcount_enter) {
    ret = register_trace_sys_enter(ftrace_syscall_enter, tr);
    if (ret < 0) {
    if (sys_data.user_mask) {
    syscall_fault_buffer_disable();
    }
    return ret;
    }
    }
    WRITE_ONCE(tr.enter_syscall_files[num], file);
    tr.sys_refcount_enter += 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unreg_event_syscall_enter(file: *mut trace_event_file, call: *mut trace_event_call) {
    let mut sys_data = call.data;
    let mut tr = file.tr;
    let mut num = 0;
    num = sys_data.syscall_nr;
    if (WARN_ON_ONCE!(num < 0 || num >= NR_syscalls)) {
    return;
    }
    guard(mutex)(&syscall_trace_lock);
    tr.sys_refcount_enter -= 1;
    WRITE_ONCE(tr.enter_syscall_files[num], core::ptr::null_mut());
    if (!tr.sys_refcount_enter) {
    unregister_trace_sys_enter(ftrace_syscall_enter, tr);
    }
    if (sys_data.user_mask) {
    syscall_fault_buffer_disable();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn reg_event_syscall_exit(file: *mut trace_event_file, call: *mut trace_event_call) -> c_int {
    let mut tr = file.tr;
pub static mut ret: c_int = 0;
    let mut num = 0;
    num = (call.data).syscall_nr;
    if (WARN_ON_ONCE!(num < 0 || num >= NR_syscalls)) {
    return -ENOSYS;
    }
    mutex_lock(&syscall_trace_lock);
    if (!tr.sys_refcount_exit) {
    ret = register_trace_sys_exit(ftrace_syscall_exit, tr);
    }
    if (!ret) {
    WRITE_ONCE(tr.exit_syscall_files[num], file);
    tr.sys_refcount_exit += 1;
    }
    mutex_unlock(&syscall_trace_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unreg_event_syscall_exit(file: *mut trace_event_file, call: *mut trace_event_call) {
    let mut tr = file.tr;
    let mut num = 0;
    num = (call.data).syscall_nr;
    if (WARN_ON_ONCE!(num < 0 || num >= NR_syscalls)) {
    return;
    }
    mutex_lock(&syscall_trace_lock);
    tr.sys_refcount_exit -= 1;
    WRITE_ONCE(tr.exit_syscall_files[num], core::ptr::null_mut());
    if (!tr.sys_refcount_exit) {
    unregister_trace_sys_exit(ftrace_syscall_exit, tr);
    }
    mutex_unlock(&syscall_trace_lock);
    }
//
// For system calls that reference user space memory that can
// be recorded into the event, set the system call meta data's user_mask
// to the "args" index that points to the user space memory to retrieve.
//
#[no_mangle]
unsafe extern "C" fn check_faultable_syscall(call: *mut trace_event_call, nr: c_int) {
    let mut sys_data = call.data;
    let mut mask = 0;
// Only work on entry
    if (sys_data.enter_event != call) {
    return;
    }
    sys_data.user_arg_size = -1;
    match (nr) {
// user arg 1 with size arg at 2
    __NR_write => {

    }
    __NR_mq_timedsend => {

    }
    __NR_pwrite64 => {
    sys_data.user_mask = BIT(1);
    sys_data.user_arg_size = 2;
    // break;
// user arg 0 with size arg at 1 as string
    }
    __NR_setdomainname => {
    }
    __NR_sethostname => {
    sys_data.user_mask = BIT(0);
    sys_data.user_arg_size = 1;
    sys_data.user_arg_is_str = 1;
    // break;

// user arg 4 with size arg at 3 as string
    }
    __NR_kexec_file_load => {
    sys_data.user_mask = BIT(4);
    sys_data.user_arg_size = 3;
    sys_data.user_arg_is_str = 1;
    // break;

// user arg at position 0

    }
    __NR_access => {

    }
    __NR_acct => {
    }
    __NR_chdir => {

    }
    __NR_chown => {

    }
    __NR_chmod => {

    }
    __NR_chroot => {

    }
    __NR_creat => {

    }
    __NR_delete_module => {
    }
    __NR_execve => {
    }
    __NR_fsopen => {

    }
    __NR_lchown => {

    }
    __NR_open => {

    }
    __NR_memfd_create => {

    }
    __NR_mkdir => {

    }
    __NR_mknod => {

    }
    __NR_mq_open => {
    }
    __NR_mq_unlink => {

    }
    __NR_readlink => {

    }
    __NR_rmdir => {

    }
    __NR_shmdt => {

    }
    __NR_statfs => {

    }
    __NR_swapon => {
    }
    __NR_swapoff => {

    }
    __NR_truncate => {

    }
    __NR_unlink => {

    }
    __NR_umount2 => {

    }
    __NR_utime => {

    }
    __NR_utimes => {

    sys_data.user_mask = BIT(0);
    // break;
// user arg at position 1
    }
    __NR_execveat => {
    }
    __NR_faccessat => {
    }
    __NR_faccessat2 => {
    }
    __NR_finit_module => {
    }
    __NR_fchmodat => {
    }
    __NR_fchmodat2 => {
    }
    __NR_fchownat => {
    }
    __NR_fgetxattr => {
    }
    __NR_flistxattr => {
    }
    __NR_fsetxattr => {
    }
    __NR_fspick => {
    }
    __NR_fremovexattr => {

    }
    __NR_futimesat => {

    }
    __NR_inotify_add_watch => {
    }
    __NR_mkdirat => {
    }
    __NR_mknodat => {
    }
    __NR_mount_setattr => {
    }
    __NR_name_to_handle_at => {

    }
    __NR_newfstatat => {

    }
    __NR_openat => {
    }
    __NR_openat2 => {
    }
    __NR_open_tree => {
    }
    __NR_open_tree_attr => {
    }
    __NR_readlinkat => {
    }
    __NR_quotactl => {
    }
    __NR_syslog => {
    }
    __NR_statx => {
    }
    __NR_unlinkat => {

    }
    __NR_utimensat => {

    sys_data.user_mask = BIT(1);
    // break;
// user arg at position 2
    }
    __NR_init_module => {
    }
    __NR_fsconfig => {
    sys_data.user_mask = BIT(2);
    // break;
// user arg at position 4
    }
    __NR_fanotify_mark => {
    sys_data.user_mask = BIT(4);
    // break;
// 2 user args, 0 and 1
    }
    __NR_add_key => {
    }
    __NR_getxattr => {
    }
    __NR_lgetxattr => {
    }
    __NR_lremovexattr => {

    }
    __NR_link => {

    }
    __NR_listxattr => {
    }
    __NR_llistxattr => {
    }
    __NR_lsetxattr => {
    }
    __NR_pivot_root => {
    }
    __NR_removexattr => {

    }
    __NR_rename => {

    }
    __NR_request_key => {
    }
    __NR_setxattr => {

    }
    __NR_symlink => {

    sys_data.user_mask = BIT(0) | BIT(1);
    // break;
// 2 user args, 0 and 2
    }
    __NR_symlinkat => {
    sys_data.user_mask = BIT(0) | BIT(2);
    // break;
// 2 user args, 1 and 3
    }
    __NR_getxattrat => {
    }
    __NR_linkat => {
    }
    __NR_listxattrat => {
    }
    __NR_move_mount => {

    }
    __NR_renameat => {

    }
    __NR_renameat2 => {
    }
    __NR_removexattrat => {
    }
    __NR_setxattrat => {
    sys_data.user_mask = BIT(1) | BIT(3);
    // break;
    }
    __NR_mount => {
    sys_data.user_mask = BIT(0) | BIT(1) | BIT(2);
    // break;
    }
    _ => {
    sys_data.user_mask = 0;
    return;
    }
    }
    if (sys_data.user_arg_size < 0) {
    return;
    }
//
// The user_arg_size can only be used when the system call
// is reading only a single address from user space.
//
    mask = sys_data.user_mask;
    if (WARN_ON!(mask & (mask - 1))) {
    sys_data.user_arg_size = -1;
    }
    }
#[no_mangle]
unsafe extern "C" fn init_syscall_trace(call: *mut trace_event_call) -> c_int {
    let mut id = 0;
    let mut num = 0;
    num = (call.data).syscall_nr;
    if (num < 0 || num >= NR_syscalls) {
    pr_debug!("syscall %s metadata not mapped, disabling ftrace event\n",
    (call.data).name);
    return -ENOSYS;
    }
    check_faultable_syscall(call, num);
    if (set_syscall_print_fmt(call) < 0) {
    return -ENOMEM;
    }
    id = trace_event_raw_init(call);
    if (id < 0) {
    free_syscall_print_fmt(call);
    return id;
    }
    return id;
    }
    static struct trace_event_fields __refdata syscall_enter_fields_array[] = {
    SYSCALL_FIELD(int, __syscall_nr),
    { .type = TRACE_FUNCTION_TYPE,
    .define_fields = syscall_enter_define_fields },
    {}
    };
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_functions: usize = 0;

// BTF id lists for the shared sys_enter/sys_exit dispatcher tracepoints.
    BTF_ID_LIST(syscall_enter_btf_ids)
    BTF_ID(func,   __bpf_trace_sys_enter)
    BTF_ID(struct, trace_event_raw_sys_enter)
    BTF_ID_LIST(syscall_exit_btf_ids)
    BTF_ID(func,   __bpf_trace_sys_exit)
    BTF_ID(struct, trace_event_raw_sys_exit)

    struct trace_event_class __refdata event_class_syscall_enter = {
    .system		= "syscalls",
    .reg		= syscall_enter_register,
    .fields_array	= syscall_enter_fields_array,
    .get_fields	= syscall_get_enter_fields,
    .raw_init	= init_syscall_trace,

    .btf_ids	= syscall_enter_btf_ids,

    };
    struct trace_event_class __refdata event_class_syscall_exit = {
    .system		= "syscalls",
    .reg		= syscall_exit_register,
    .fields_array	= (trace_event_fields[]){
    SYSCALL_FIELD(int, __syscall_nr),
    SYSCALL_FIELD(long, ret),
    {}
    },
    .fields		= LIST_HEAD_INIT(event_class_syscall_exit.fields),
    .raw_init	= init_syscall_trace,

    .btf_ids	= syscall_exit_btf_ids,

    };
#[no_mangle]
pub unsafe extern "C" fn arch_syscall_addr(nr: c_int) -> unsigned long __init __weak {
    return (unsigned long)sys_call_table[nr];
    }
#[no_mangle]
pub unsafe extern "C" fn init_ftrace_syscalls()  {
pub static mut meta: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    let mut i = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (!IS_ENABLED!(CONFIG_HAVE_SPARSE_SYSCALL_NR)) {
    syscalls_metadata = kzalloc_objs(*syscalls_metadata,
    NR_syscalls);
    if (!syscalls_metadata) {
    WARN_ON!(1);
    return;
    }
    }
    while (i < NR_syscalls) {
    addr = arch_syscall_addr(i);
    meta = find_syscall_meta(addr);
    if (!meta) {
    continue;
    }
    meta.syscall_nr = i;
    if (!IS_ENABLED!(CONFIG_HAVE_SPARSE_SYSCALL_NR)) {
    syscalls_metadata[i] = meta;
    } else {
    ret = xa_store(&syscalls_metadata_sparse, i, meta,
    GFP_KERNEL);
    WARN(xa_is_err(ret),
    "Syscall memory allocation failed\n");
    }
    }
    }

pub static mut enabled_perf_enter_syscalls: usize = 0;
pub static mut enabled_perf_exit_syscalls: usize = 0;
    static int sys_perf_refcount_enter;
    static int sys_perf_refcount_exit;
#[no_mangle]
pub unsafe extern "C" fn perf_call_bpf_enter(call: *mut trace_event_call, sys_data: *mut syscall_metadata, syscall_nr: c_int, args: *mut c_ulong) -> c_int {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_tp_t {
    pub ent: trace_entry,
    pub syscall_nr: c_int,
    pub args: [c_ulong; SYSCALL_DEFINE_MAXARGS],
    pub param: } __aligned(8),
    pub {}: pt_regs regs =,
    pub i: c_int,
    pub )): *mut BUILD_BUG_ON!(sizeof!(param.ent) < sizeof!(void,
// bpf prog requires 'regs' to be the first member in the ctx
// &param = &regs;
    pub syscall_nr: param.syscall_nr =,
    pub i++): for (i = 0; i < sys_data->nb_args;,
    pub args: [param.args[i] =; i],
    pub &param): return trace_call_bpf_faultable(call,,
    }
#[no_mangle]
unsafe extern "C" fn perf_syscall_enter(ignore: *mut c_void, regs: *mut pt_regs, id: c_long) {
    pub sys_data: *mut syscall_metadata,
    pub rec: *mut syscall_trace_enter,
    pub head: *mut hlist_head,
    pub args: [c_ulong; 6],
    pub valid_prog_array: bool,
    pub mayfault: bool,
    pub user_ptr: *mut c_char,
    pub {}: int user_sizes[SYSCALL_FAULT_MAX_CNT] =,
    pub CONFIG_TRACE_SYSCALL_BUF_SIZE_DEFAULT: int buf_size =,
    pub syscall_nr: c_int,
    pub rctx: c_int,
    pub 0: int size =,
    pub 0: int uargs =,
    pub regs): syscall_nr = trace_get_syscall_nr(current,,
    if (syscall_nr < 0 || syscall_nr >= NR_syscalls) {
    if (!test_bit(syscall_nr, enabled_perf_enter_syscalls))
    pub syscall_nr_to_meta(syscall_nr): sys_data =,
    if (!sys_data)
    pub args): syscall_get_arguments(current, regs,,
//
// Run BPF program in faultable context before per-cpu buffer
// allocation, allowing sleepable BPF programs to execute.
//
    pub bpf_prog_array_valid(sys_data->enter_event): valid_prog_array =,
    if (valid_prog_array &&
    !perf_call_bpf_enter(sys_data.enter_event, sys_data,
    syscall_nr, args))
//
// Per-cpu ring buffer and perf event list operations require
// preemption to be disabled.
//
    pub this_cpu_ptr(sys_data->enter_event->perf_events): head =,
    if (hlist_empty(head))
// Check if this syscall event faults in user space memory
    pub 0: mayfault = sys_data->user_mask !=,
    if (mayfault) {
    }
    if (syscall_get_data(sys_data, args, &user_ptr,
    &size, user_sizes, &uargs, buf_size) < 0) {
// The above may have caused a migration
    pub this_cpu_ptr(sys_data->enter_event->perf_events): head =,
    if (hlist_empty(head))
    }
// get the size after alignment with the u32 buffer size field
    pub sizeof!(*rec): *mut *mut size += sizeof!(unsigned long)  sys_data->nb_args +,
    pub sizeof!(u64)): size = ALIGN(size + sizeof!(u32),,
    pub sizeof!(u32): size -=,
    pub &rctx): rec = perf_trace_buf_alloc(size, NULL,,
    if (!rec)
    pub syscall_nr: rec->nr =,
    pub sys_data->nb_args): *mut *mut memcpy(&rec->args, args, sizeof!(unsigned long),
    if (mayfault)
    pub uargs): syscall_put_data(sys_data, rec, user_ptr, size, user_sizes,,
    perf_trace_buf_submit(rec, size, rctx,
    sys_data.enter_event.event.type, 1, regs,
    pub NULL): head,,
    }
#[no_mangle]
unsafe extern "C" fn perf_sysenter_enable(call: *mut trace_event_call) -> c_int {
    }
    pub call->data: *mut *mut syscall_metadata sys_data =,
    pub num: c_int,
    pub ret: c_int,
    pub sys_data->syscall_nr: num =,
    if (sys_data.user_mask) {
    pub syscall_fault_buffer_enable(): ret =,
    if (ret < 0) {
    pub ret: return,
    }
    if (!sys_perf_refcount_enter) {
    }
    pub NULL): ret = register_trace_sys_enter(perf_syscall_enter,,
    if (ret) {
    pub point"): pr_info!("event trace: Could not activate syscall entry trace,
    if (sys_data.user_mask) {
    pub ret: return,
    }
    }
    pub enabled_perf_enter_syscalls): set_bit(num,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn perf_sysenter_disable(call: *mut trace_event_call) {
    }
    pub call->data: *mut *mut syscall_metadata sys_data =,
    pub num: c_int,
    pub sys_data->syscall_nr: num =,
    pub enabled_perf_enter_syscalls): clear_bit(num,,
    if (!sys_perf_refcount_enter) {
    pub NULL): unregister_trace_sys_enter(perf_syscall_enter,,
    if (sys_data.user_mask)
    }
#[no_mangle]
pub unsafe extern "C" fn perf_call_bpf_exit(call: *mut trace_event_call, syscall_nr: c_int, ret_val: c_long) -> c_int {
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_tp_t {
    pub ent: trace_entry,
    pub syscall_nr: c_int,
    pub ret: c_ulong,
    pub param: } __aligned(8),
    pub {}: pt_regs regs =,
// bpf prog requires 'regs' to be the first member in the ctx
// &param = &regs;
    pub syscall_nr: param.syscall_nr =,
    pub ret_val: param.ret =,
    pub &param): return trace_call_bpf_faultable(call,,
    }
#[no_mangle]
unsafe extern "C" fn perf_syscall_exit(ignore: *mut c_void, regs: *mut pt_regs, ret: c_long) {
    pub sys_data: *mut syscall_metadata,
    pub rec: *mut syscall_trace_exit,
    pub head: *mut hlist_head,
    pub valid_prog_array: bool,
    pub syscall_nr: c_int,
    pub rctx: c_int,
    pub size: c_int,
    pub regs): syscall_nr = trace_get_syscall_nr(current,,
    if (syscall_nr < 0 || syscall_nr >= NR_syscalls) {
    if (!test_bit(syscall_nr, enabled_perf_exit_syscalls))
    pub syscall_nr_to_meta(syscall_nr): sys_data =,
    if (!sys_data)
//
// Run BPF program in faultable context before per-cpu buffer
// allocation, allowing sleepable BPF programs to execute.
//
    pub bpf_prog_array_valid(sys_data->exit_event): valid_prog_array =,
    if (valid_prog_array &&
    !perf_call_bpf_exit(sys_data.exit_event, syscall_nr,
    syscall_get_return_value(current, regs)))
//
// Per-cpu ring buffer and perf event list operations require
// preemption to be disabled.
//
    pub this_cpu_ptr(sys_data->exit_event->perf_events): head =,
    if (hlist_empty(head))
// We can probably do that at build time
    pub sizeof!(u64)): *mut *mut size = ALIGN(sizeof!(rec) + sizeof!(u32),,
    pub sizeof!(u32): size -=,
    pub &rctx): rec = perf_trace_buf_alloc(size, NULL,,
    if (!rec)
    pub syscall_nr: rec->nr =,
    pub regs): rec->ret = syscall_get_return_value(current,,
    perf_trace_buf_submit(rec, size, rctx, sys_data.exit_event.event.type,
    pub NULL): 1, regs, head,,
    }
#[no_mangle]
unsafe extern "C" fn perf_sysexit_enable(call: *mut trace_event_call) -> c_int {
    }
    pub num: c_int,
    pub )call->data)->syscall_nr: *mut num = ((syscall_metadata,
    if (!sys_perf_refcount_exit) {
    pub NULL): int ret = register_trace_sys_exit(perf_syscall_exit,,
    if (ret) {
    pub point"): pr_info!("event trace: Could not activate syscall exit trace,
    pub ret: return,
    }
    }
    pub enabled_perf_exit_syscalls): set_bit(num,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn perf_sysexit_disable(call: *mut trace_event_call) {
    pub num: c_int,
    pub )call->data)->syscall_nr: *mut num = ((syscall_metadata,
    pub enabled_perf_exit_syscalls): clear_bit(num,,
    if (!sys_perf_refcount_exit) {
    pub NULL): unregister_trace_sys_exit(perf_syscall_exit,,
    }

#[no_mangle]
pub unsafe extern "C" fn syscall_enter_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    }
    pub data: *mut *mut trace_event_file file =,
    match (type) {
    TRACE_REG_REGISTER => {
    pub event): return reg_event_syscall_enter(file,,
    }
    TRACE_REG_UNREGISTER => {
    pub event): unreg_event_syscall_enter(file,,
    pub 0: return,

    }
    TRACE_REG_PERF_REGISTER => {
    pub perf_sysenter_enable(event): return,
    }
    TRACE_REG_PERF_UNREGISTER => {
    pub 0: return,
    }
    TRACE_REG_PERF_OPEN => {
    }
    TRACE_REG_PERF_CLOSE => {
    }
    TRACE_REG_PERF_ADD => {
    }
    TRACE_REG_PERF_DEL => {
    pub 0: return,

    }
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_exit_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    pub data: *mut *mut trace_event_file file =,
    match (type) {
    TRACE_REG_REGISTER => {
    pub event): return reg_event_syscall_exit(file,,
    }
    TRACE_REG_UNREGISTER => {
    pub event): unreg_event_syscall_exit(file,,
    pub 0: return,

    }
    TRACE_REG_PERF_REGISTER => {
    pub perf_sysexit_enable(event): return,
    }
    TRACE_REG_PERF_UNREGISTER => {
    pub 0: return,
    }
    TRACE_REG_PERF_OPEN => {
    }
    TRACE_REG_PERF_CLOSE => {
    }
    TRACE_REG_PERF_ADD => {
    }
    TRACE_REG_PERF_DEL => {
    pub 0: return,

    }
    }
    pub 0: return,