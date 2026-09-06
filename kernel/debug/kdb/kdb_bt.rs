//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_bt.c
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
// Kernel Debugger Architecture Independent Stack Traceback
//
// Copyright (c) 1999-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

#[no_mangle]
unsafe extern "C" fn kdb_show_stack(p: *mut task_struct, addr: *mut c_void) {
    kdb_trap_printk += 1;
    if (!addr && kdb_task_has_cpu(p)) {
pub static mut old_lvl: c_int = 0;
    console_loglevel = CONSOLE_LOGLEVEL_MOTORMOUTH;
    kdb_dump_stack_on_cpu(kdb_process_cpu(p));
    console_loglevel = old_lvl;
    } else {
    show_stack(p, addr, KERN_EMERG);
    }
    kdb_trap_printk -= 1;
    }
//
// kdb_bt
//
// This function implements the 'bt' command.  Print a stack
// traceback.
//
// bt [<address-expression>]	(addr-exp is for alternate stacks)
// btp <pid>			Kernel stack for <pid>
// btt <address-expression>	Kernel stack for task structure at
// <address-expression>
// bta [state_chars>|A]		All useful processes, optionally
// filtered by state
// btc [<cpu>]			The current process on one cpu,
// default is all cpus
//
// bt <address-expression> refers to a address on the stack, that location
// is assumed to contain a return address.
//
// btt <address-expression> refers to the address of a struct task.
//
// Inputs:
// argc	argument count
// argv	argument vector
// Outputs:
// None.
// Returns:
// zero for success, a kdb diagnostic if error
// Locking:
// none.
// Remarks:
// Backtrack works best when the code uses frame pointers.  But even
// without frame pointers we should get a reasonable trace.
//
// mds comes in handy when examining the stack to do a manual traceback or
// to get a starting point for bt <address-expression>.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_bt1(p: *mut task_struct, mask: *mut c_char, btaprompt: bool) -> c_int {
    let mut ch = 0;
    if (kdb_getarea(ch, (unsigned long)p) ||
    kdb_getarea(ch, (unsigned long)(p+1)-1)) {
    return KDB_BADADDR;
    }
    if (!kdb_task_state(p, mask)) {
    return 0;
    }
    kdb_printf("Stack traceback for pid %d\n", p.pid);
    kdb_ps1(p);
    kdb_show_stack(p, core::ptr::null_mut());
    if (btaprompt) {
    kdb_printf("Enter <q> to end, <cr> or <space> to continue:");
    do {
    ch = kdb_getchar();
    } while (!strchr("\r\n q", ch));
    kdb_printf("\n");
// reset the pager
    kdb_nextline = 1;
    if (ch == 'q') {
    return 1;
    }
    }
    touch_nmi_watchdog();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_bt_cpu(cpu: c_ulong) {
pub static mut kdb_tsk: *mut c_void = core::ptr::null_mut();
    if (cpu >= num_possible_cpus() || !cpu_online(cpu)) {
    kdb_printf("WARNING: no process for cpu %ld\n", cpu);
    return;
    }
// If a CPU failed to round up we could be here
    kdb_tsk = KDB_TSK(cpu);
    if (!kdb_tsk) {
    kdb_printf("WARNING: no task for cpu %ld\n", cpu);
    return;
    }
    kdb_bt1(kdb_tsk, "A", false);
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_bt(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut diag = 0;
pub static mut btaprompt: c_int = 1;
    let mut nextarg = 0;
    let mut addr = 0;
    let mut offset = 0;
// Prompt after each proc in bta
    kdbgetintenv("BTAPROMPT", &btaprompt);
    if (strcmp(argv[0], "bta") == 0) {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut cpu = 0;
    let mut mask = argc ? argv[1] : kdbgetenv("PS");
    if (argc == 0) {
    kdb_ps_suppressed();
    }
// Run the active tasks first
    for_each_online_cpu(cpu) {
    p = curr_task(cpu);
    if (kdb_bt1(p, mask, btaprompt)) {
    return 0;
    }
    }
// Now the inactive tasks
    for_each_process_thread(g, p) {
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    if (task_curr(p)) {
    continue;
    }
    if (kdb_bt1(p, mask, btaprompt)) {
    return 0;
    }
    }
    } else if (strcmp(argv[0], "btp") == 0) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut pid = 0;
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    diag = kdbgetularg(argv[1], &pid);
    if (diag) {
    return diag;
    }
    p = find_task_by_pid_ns(pid, &init_pid_ns);
    if (p) {
    return kdb_bt1(p, "A", false);
    }
    kdb_printf("No process with pid == %ld found\n", pid);
    return 0;
    } else if (strcmp(argv[0], "btt") == 0) {
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    diag = kdbgetularg(argv[1], &addr);
    if (diag) {
    return diag;
    }
    return kdb_bt1(addr, "A", false);
    } else if (strcmp(argv[0], "btc") == 0) {
pub static mut cpu: c_ulong = 0;
    if (argc > 1) {
    return KDB_ARGCOUNT;
    }
    if (argc == 1) {
    diag = kdbgetularg(argv[1], &cpu);
    if (diag) {
    return diag;
    }
    }
    if (cpu != ~0) {
    kdb_bt_cpu(cpu);
    } else {
//
// Recursive use of kdb_parse, do not use argv after
// this point.
//
    argv = core::ptr::null_mut();
    kdb_printf("btc: cpu status: ");
    kdb_parse("cpu\n");
    for_each_online_cpu(cpu) {
    kdb_bt_cpu(cpu);
    touch_nmi_watchdog();
    }
    }
    return 0;
    } else {
    if (argc) {
    nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg, &addr,
    &offset, core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    kdb_show_stack(kdb_current_task, addr);
    return 0;
    } else {
    return kdb_bt1(kdb_current_task, "A", false);
    }
    }
// NOTREACHED
    return 0;
    }