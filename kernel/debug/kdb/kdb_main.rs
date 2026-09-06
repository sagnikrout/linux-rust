//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_main.c
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
// Kernel Debugger Architecture Independent Main Code
//
// Copyright (C) 1999-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (C) 2000 Stephane Eranian <eranian@hpl.hp.com>
// Xscale (R) modifications copyright (C) 2003 Intel Corporation.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

pub static mut kdb_cmd_enabled: int = 0;
    module_param_named!(cmd_enable, kdb_cmd_enabled, int, 0600);
    char kdb_grep_string[KDB_GREP_STRLEN];
    let mut kdb_grepping_flag = 0;
    EXPORT_SYMBOL(kdb_grepping_flag);
    let mut kdb_grep_leading = 0;
    let mut kdb_grep_trailing = 0;
//
// Kernel debugger state flags
//
    let mut kdb_flags = 0;
//
// kdb_lock protects updates to kdb_initial_cpu.  Used to
// single thread processors through the kernel debugger.
//
    let mut kdb_initial_cpu = -1;	/* cpu number that owns kdb */
pub static mut kdb_nextline: c_int = 1;
    let mut kdb_state = 0;			/* General KDB state */
pub static mut kdb_current_task: *mut c_void = core::ptr::null_mut();
pub static mut kdb_current_regs: *mut c_void = core::ptr::null_mut();
pub static mut kdb_diemsg: *mut c_void = core::ptr::null_mut();
    static int kdb_go_count;

    static unsigned int kdb_continue_catastrophic =
    CONFIG_KDB_CONTINUE_CATASTROPHIC;

    static unsigned int kdb_continue_catastrophic;

// kdb_cmds_head describes the available commands.
pub static mut kdb_cmds_head: usize = 0;
    typedef struct _kdbmsg {
    let mut km_diag = 0;	/* kdb diagnostic */
pub static mut km_msg: *mut c_void = core::ptr::null_mut();	/* Corresponding message text */
    } kdbmsg_t;

    { KDB_##msgnum, text }
    static kdbmsg_t kdbmsgs[] = {
    KDBMSG(NOTFOUND, "Command Not Found"),
    KDBMSG(ARGCOUNT, "Improper argument count, see usage."),
    KDBMSG(BADWIDTH, "Illegal value for BYTESPERWORD use 1, 2, 4 or 8, "
    "8 is only allowed on 64 bit systems"),
    KDBMSG(BADRADIX, "Illegal value for RADIX use 8, 10 or 16"),
    KDBMSG(NOTENV, "Cannot find environment variable"),
    KDBMSG(NOENVVALUE, "Environment variable should have value"),
    KDBMSG(NOTIMP, "Command not implemented"),
    KDBMSG(ENVFULL, "Environment full"),
    KDBMSG(KMALLOCFAILED, "Failed to allocate memory"),
    KDBMSG(TOOMANYBPT, "Too many breakpoints defined"),

    KDBMSG(TOOMANYDBREGS, "More breakpoints than ibcr registers defined"),

    KDBMSG(TOOMANYDBREGS, "More breakpoints than db registers defined"),

    KDBMSG(DUPBPT, "Duplicate breakpoint address"),
    KDBMSG(BPTNOTFOUND, "Breakpoint not found"),
    KDBMSG(BADMODE, "Invalid IDMODE"),
    KDBMSG(BADINT, "Illegal numeric value"),
    KDBMSG(INVADDRFMT, "Invalid symbolic address format"),
    KDBMSG(BADREG, "Invalid register name"),
    KDBMSG(BADCPUNUM, "Invalid cpu number"),
    KDBMSG(BADLENGTH, "Invalid length field"),
    KDBMSG(NOBP, "No Breakpoint exists"),
    KDBMSG(BADADDR, "Invalid address"),
    KDBMSG(NOPERM, "Permission denied"),
    };

pub static mut __nkdb_err: int = 0;
//
// Initial environment. This is all kept static and local to this file.
// The entire environment is limited to a fixed number of entries
// (add more to __env[] if required)
//
    static char *__env[31] = {

    "PROMPT=[%d]kdb> ",

    "PROMPT=kdb> ",

    "MOREPROMPT=more> ",
    "RADIX=16",
    "MDCOUNT=8",		/* lines of md output */
    KDB_PLATFORM_ENV,
    "DTABCOUNT=30",
    "NOSECT=1",
    };
pub static mut __nenv: int = 0;
//
// Update the permissions flags (kdb_cmd_enabled) to match the
// current lockdown state.
//
// Within this function the calls to security_locked_down() are "lazy". We
// avoid calling them if the current value of kdb_cmd_enabled already excludes
// flags that might be subject to lockdown. Additionally we deliberately check
// the lockdown flags independently (even though read lockdown implies write
// lockdown) since that results in both simpler code and clearer messages to
// the user on first-time debugger entry.
//
// The permission masks during a read+write lockdown permits the following
// flags: INSPECT, SIGNAL, REBOOT (and ALWAYS_SAFE).
//
// The INSPECT commands are not blocked during lockdown because they are
// not arbitrary memory reads. INSPECT covers the backtrace family (sometimes
// forcing them to have no arguments) and lsmod. These commands do expose
// some kernel state but do not allow the developer seated at the console to
// choose what state is reported. SIGNAL and REBOOT should not be controversial,
// given these are allowed for root during lockdown already.
//
#[no_mangle]
unsafe extern "C" fn kdb_check_for_lockdown() {
    let mut write_flags = KDB_ENABLE_MEM_WRITE |
    KDB_ENABLE_REG_WRITE |
    KDB_ENABLE_FLOW_CTRL;
    let mut read_flags = KDB_ENABLE_MEM_READ |
    KDB_ENABLE_REG_READ;
pub static mut need_to_lockdown_write: bool = false;
pub static mut need_to_lockdown_read: bool = false;
    if (kdb_cmd_enabled & (KDB_ENABLE_ALL | write_flags)) {
    need_to_lockdown_write =
    security_locked_down(LOCKDOWN_DBG_WRITE_KERNEL);
    }
    if (kdb_cmd_enabled & (KDB_ENABLE_ALL | read_flags)) {
    need_to_lockdown_read =
    security_locked_down(LOCKDOWN_DBG_READ_KERNEL);
    }
// De-compose KDB_ENABLE_ALL if required
    if (need_to_lockdown_write || need_to_lockdown_read) {
    if (kdb_cmd_enabled & KDB_ENABLE_ALL)
    kdb_cmd_enabled = KDB_ENABLE_MASK & ~KDB_ENABLE_ALL;
    }
    if (need_to_lockdown_write) {
    kdb_cmd_enabled &= ~write_flags;
    }
    if (need_to_lockdown_read) {
    kdb_cmd_enabled &= ~read_flags;
    }
    }
//
// Check whether the flags of the current command, the permissions of the kdb
// console and the lockdown state allow a command to be run.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_check_flags(flags: kdb_cmdflags_t, permissions: c_int, no_args: bool) -> bool {
// permissions comes from userspace so needs massaging slightly
    permissions &= KDB_ENABLE_MASK;
    permissions |= KDB_ENABLE_ALWAYS_SAFE;
// some commands change group when launched with no arguments
    if (no_args) {
    permissions |= permissions << KDB_ENABLE_NO_ARGS_SHIFT;
    }
    flags |= KDB_ENABLE_ALL;
    return permissions & flags;
    }
//
// kdbgetenv - This function will return the character string value of
// an environment variable.
// Parameters:
// match	A character string representing an environment variable.
// Returns:
// NULL	No environment variable matches 'match'
// char*	Pointer to string value of environment variable.
//
#[no_mangle]
pub unsafe extern "C" fn kdbgetenv(match: *mut c_char) -> *mut c_void {
    let mut ep = __env;
pub static mut matchlen: c_int = 0;
    let mut i = 0;
    while (i < __nenv) {
    let mut e = *ep += 1;
    if (!e) {
    continue;
    }
    if ((strncmp(match, e, matchlen) == 0)
    && ((e[matchlen] == '\0')
    || (e[matchlen] == '='))) {
    let mut cp = strchr(e, '=');
    return cp ? ++cp : "";
    }
    }
    return core::ptr::null_mut();
    }
//
// kdbgetulenv - This function will return the value of an unsigned
// long-valued environment variable.
// Parameters:
// match	A character string representing a numeric value
// Outputs:
// *value  the unsigned long representation of the env variable 'match'
// Returns:
// Zero on success, a kdb diagnostic on failure.
//
#[no_mangle]
unsafe extern "C" fn kdbgetulenv(match: *const c_char, value: *mut c_ulong) -> c_int {
pub static mut ep: *mut c_void = core::ptr::null_mut();
    ep = kdbgetenv(match);
    if (!ep) {
    return KDB_NOTENV;
    }
    if (strlen(ep) == 0) {
    return KDB_NOENVVALUE;
    }
    if (kstrtoul(ep, 0, value)) {
    return KDB_BADINT;
    }
    return 0;
    }
//
// kdbgetintenv - This function will return the value of an
// integer-valued environment variable.
// Parameters:
// match	A character string representing an integer-valued env variable
// Outputs:
// *value  the integer representation of the environment variable 'match'
// Returns:
// Zero on success, a kdb diagnostic on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kdbgetintenv(match: *const c_char, value: *mut c_int) -> c_int {
    let mut val = 0;
    let mut diag = 0;
    diag = kdbgetulenv(match, &val);
    if (!diag) {
// value = (int) val;
    }
    return diag;
    }
//
// kdb_setenv() - Alter an existing environment variable or create a new one.
// @var: Name of the variable
// @val: Value of the variable
//
// Return: Zero on success, a kdb diagnostic on failure.
//
#[no_mangle]
unsafe extern "C" fn kdb_setenv(var: *const c_char, val: *const c_char) -> c_int {
    let mut i = 0;
pub static mut ep: *mut c_void = core::ptr::null_mut();
    size_t varlen, vallen;
    varlen = strlen(var);
    vallen = strlen(val);
    ep = kmalloc(varlen + vallen + 2, GFP_KDB);
    if (!ep) {
    return KDB_KMALLOCFAILED;
    }
    sprintf(ep, "%s=%s", var, val);
    while (i < __nenv) {
    if (__env[i]
    && ((strncmp(__env[i], var, varlen) == 0)
    && ((__env[i][varlen] == '\0')
    || (__env[i][varlen] == '=')))) {
    kfree_const(__env[i]);
    __env[i] = ep;
    return 0;
    }
    }
//
// Wasn't existing variable.  Fit into slot.
//
    while (i < __nenv-1) {
    if (__env[i] == 0) {
    __env[i] = ep;
    return 0;
    }
    }
    return KDB_ENVFULL;
    }
//
// kdb_printenv() - Display the current environment variables.
//
#[no_mangle]
unsafe extern "C" fn kdb_printenv() {
    let mut i = 0;
    while (i < __nenv) {
    if (__env[i]) {
    kdb_printf("%s\n", __env[i]);
    }
    }
    }
//
// kdbgetularg - This function will convert a numeric string into an
// unsigned long value.
// Parameters:
// arg	A character string representing a numeric value
// Outputs:
// *value  the unsigned long representation of arg.
// Returns:
// Zero on success, a kdb diagnostic on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kdbgetularg(arg: *const c_char, value: *mut c_ulong) -> c_int {
    if (kstrtoul(arg, 0, value)) {
    return KDB_BADINT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kdbgetu64arg(arg: *const c_char, value: *mut u64) -> c_int {
    if (kstrtou64(arg, 0, value)) {
    return KDB_BADINT;
    }
    return 0;
    }
//
// kdb_set - This function implements the 'set' command.  Alter an
// existing environment variable or create a new one.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_set(argc: c_int, argv: *const c_char) -> c_int {
//
// we can be invoked two ways:
// set var=value    argv[1]="var", argv[2]="value"
// set var = value  argv[1]="var", argv[2]="=", argv[3]="value"
// - if the latter, shift 'em down.
//
    if (argc == 3) {
    argv[2] = argv[3];
    argc -= 1;
    }
    if (argc != 2) {
    return KDB_ARGCOUNT;
    }
//
// Censor sensitive variables
//
    if (strcmp(argv[1], "PROMPT") == 0 &&
    !kdb_check_flags(KDB_ENABLE_MEM_READ, kdb_cmd_enabled, false)) {
    return KDB_NOPERM;
    }
//
// Check for internal variables
//
    if (strcmp(argv[1], "KDBDEBUG") == 0) {
    let mut debugflags = 0;
    let mut ret = 0;
    ret = kstrtouint(argv[2], 0, &debugflags);
    if (ret || debugflags & ~KDB_DEBUG_FLAG_MASK) {
    kdb_printf("kdb: illegal debug flags '%s'\n",
    argv[2]);
    return 0;
    }
    kdb_flags = (kdb_flags & ~KDB_DEBUG(MASK))
    | (debugflags << KDB_DEBUG_FLAG_SHIFT);
    return 0;
    }
//
// Tokenizer squashed the '=' sign.  argv[1] is variable
// name, argv[2] = value.
//
    return kdb_setenv(argv[1], argv[2]);
    }
#[no_mangle]
unsafe extern "C" fn kdb_check_regs() -> c_int {
    if (!kdb_current_regs) {
    kdb_printf("No current kdb registers."
    "  You may need to select another task\n");
    return KDB_BADREG;
    }
    return 0;
    }
//
// kdbgetaddrarg - This function is responsible for parsing an
// address-expression and returning the value of the expression,
// symbol name, and offset to the caller.
//
// The argument may consist of a numeric value (decimal or
// hexadecimal), a symbol name, a register name (preceded by the
// percent sign), an environment variable with a numeric value
// (preceded by a dollar sign) or a simple arithmetic expression
// consisting of a symbol name, +/-, and a numeric constant value
// (offset).
// Parameters:
// argc	- count of arguments in argv
// argv	- argument vector
// *nextarg - index to next unparsed argument in argv[]
// regs	- Register state at time of KDB entry
// Outputs:
// *value	- receives the value of the address-expression
// *offset - receives the offset specified, if any
// *name   - receives the symbol name, if any
// *nextarg - index to next unparsed argument in argv[]
// Returns:
// zero is returned on success, a kdb diagnostic code is
// returned on error.
//
#[no_mangle]
pub unsafe extern "C" fn kdbgetaddrarg(argc: c_int, argv: *mut *mut c_char, nextarg: *mut c_int, value: *mut c_ulong, offset: *mut c_long, name: *mut *mut c_char) -> c_int {
    let mut addr = 0;
pub static mut off: c_ulong = 0;
    let mut positive = 0;
    let mut diag = 0;
pub static mut found: c_int = 0;
pub static mut symname: *mut c_void = core::ptr::null_mut();
pub static mut symbol: c_char = '\0';
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut symtab;
//
// If the enable flags prohibit both arbitrary memory access
// and flow control then there are no reasonable grounds to
// provide symbol lookup.
//
    if (!kdb_check_flags(KDB_ENABLE_MEM_READ | KDB_ENABLE_FLOW_CTRL,
    kdb_cmd_enabled, false)) {
    return KDB_NOPERM;
    }
//
// Process arguments which follow the following syntax:
//
// symbol | numeric-address [+/- numeric-offset]
// %register
// $environment-variable
//
    if (*nextarg > argc) {
    return KDB_ARGCOUNT;
    }
    symname = argv[*nextarg];
//
// If there is no whitespace between the symbol
// or address and the '+' or '-' symbols, we
// remember the character and replace it with a
// null so the symbol/value can be properly parsed
//
    cp = strpbrk(symname, "+-");
    if (cp != core::ptr::null_mut()) {
    symbol = *cp;
// cp++ = '\0';
    }
    if (symname[0] == '$') {
    diag = kdbgetulenv(&symname[1], &addr);
    if (diag) {
    return diag;
    }
    } else if (symname[0] == '%') {
    diag = kdb_check_regs();
    if (diag) {
    return diag;
    }
// Implement register values with % at a later time as it is
// arch optional.
//
    return KDB_NOTIMP;
    } else {
    found = kdbgetsymval(symname, &symtab);
    if (found) {
    addr = symtab.sym_start;
    } else {
    diag = kdbgetularg(argv[*nextarg], &addr);
    if (diag) {
    return diag;
    }
    }
    }
    if (!found) {
    found = kdbnearsym(addr, &symtab);
    }
    (*nextarg)++;
    if (name) {
// name = symname;
    }
    if (value) {
// value = addr;
    }
    if (offset && name && *name) {
// offset = addr - symtab.sym_start;
    }
    if ((*nextarg > argc)
    && (symbol == '\0')) {
    return 0;
    }
//
// check for +/- and offset
//
    if (symbol == '\0') {
    if ((argv[*nextarg][0] != '+')
    && (argv[*nextarg][0] != '-')) {
//
// Not our argument.  Return.
//
    return 0;
    } else {
    positive = (argv[*nextarg][0] == '+');
    (*nextarg)++;
    }
    } else {
    positive = (symbol == '+');
    }
//
// Now there must be an offset!
//
    if ((*nextarg > argc)
    && (symbol == '\0')) {
    return KDB_INVADDRFMT;
    }
    if (!symbol) {
    cp = argv[*nextarg];
    (*nextarg)++;
    }
    diag = kdbgetularg(cp, &off);
    if (diag) {
    return diag;
    }
    if (!positive) {
    off = -off;
    }
    if (offset) {
// offset += off;
    }
    if (value) {
// value += off;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_cmderror(diag: c_int) {
    let mut i = 0;
    if (diag >= 0) {
    kdb_printf("no error detected (diagnostic is %d)\n", diag);
    return;
    }
    while (i < __nkdb_err) {
    if (kdbmsgs[i].km_diag == diag) {
    kdb_printf("diag: %d: %s\n", diag, kdbmsgs[i].km_msg);
    return;
    }
    }
    kdb_printf("Unknown diag %d\n", -diag);
    }
//
// kdb_defcmd, kdb_defcmd2 - This function implements the 'defcmd'
// command which defines one command as a set of other commands,
// terminated by endefcmd.  kdb_defcmd processes the initial
// 'defcmd' command, kdb_defcmd2 is invoked from kdb_parse for
// the following commands until 'endefcmd'.
// Inputs:
// argc	argument count
// argv	argument vector
// Returns:
// zero for success, a kdb diagnostic if error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kdb_macro {
//     pub /: *mut *mut kdbtab_t cmd; / Macro command,
//     pub /: *mut *mut list_head statements; / Associated statement list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kdb_macro_statement {
//     pub /: *mut *mut *mut char statement; / Statement text,
//     pub /: *mut *mut list_head list_node; / Statement list node,
}

pub static mut kdb_macro: *mut c_void = core::ptr::null_mut();
    static bool defcmd_in_progress;
// Forward references
// forward_decl: kdb_exec_defcmd;
#[no_mangle]
unsafe extern "C" fn kdb_defcmd2(cmdstr: *const c_char, argv0: *const c_char) -> c_int {
pub static mut kms: *mut c_void = core::ptr::null_mut();
    if (!kdb_macro) {
    return KDB_NOTIMP;
    }
    if (strcmp(argv0, "endefcmd") == 0) {
    defcmd_in_progress = false;
    if (!list_empty(&kdb_macro.statements)) {
    kdb_register(&kdb_macro.cmd);
    }
    return 0;
    }
    kms = kmalloc_obj(*kms, GFP_KDB);
    if (!kms) {
    kdb_printf("Could not allocate new kdb macro command: %s\n",
    cmdstr);
    return KDB_NOTIMP;
    }
    kms.statement = kdb_strdup(cmdstr, GFP_KDB);
    list_add_tail(&kms.list_node, &kdb_macro.statements);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_defcmd(argc: c_int, argv: *const c_char) -> c_int {
pub static mut mp: *mut c_void = core::ptr::null_mut();
    if (defcmd_in_progress) {
    kdb_printf("kdb: nested defcmd detected, assuming missing "
    "endefcmd\n");
    kdb_defcmd2("endefcmd", "endefcmd");
    }
    if (argc == 0) {
pub static mut kp: *mut c_void = core::ptr::null_mut();
pub static mut kmp: *mut c_void = core::ptr::null_mut();
pub static mut kms: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(kp, &kdb_cmds_head, list_node) {
    if (kp.func == kdb_exec_defcmd) {
    kdb_printf("defcmd %s \"%s\" \"%s\"\n",
    kp.name, kp.usage, kp.help);
    kmp = container_of!(kp, kdb_macro, cmd);
    list_for_each_entry(kms, &kmp.statements,
    list_node) {
    kdb_printf("%s", kms.statement);
    }
    kdb_printf("endefcmd\n");
    }
    }
    return 0;
    }
    if (argc != 3) {
    return KDB_ARGCOUNT;
    }
    if (in_dbg_master()) {
    kdb_printf("Command only available during kdb_init()\n");
    return KDB_NOTIMP;
    }
    kdb_macro = kzalloc_obj(*kdb_macro, GFP_KDB);
    if (!kdb_macro) {
// goto;
    }
    mp = &kdb_macro.cmd;
    mp.func = kdb_exec_defcmd;
    mp.minlen = 0;
    mp.flags = KDB_ENABLE_ALWAYS_SAFE;
    mp.name = kdb_strdup(argv[1], GFP_KDB);
    if (!mp.name) {
// goto;
    }
    mp.usage = kdb_strdup_dequote(argv[2], GFP_KDB);
    if (!mp.usage) {
// goto;
    }
    mp.help = kdb_strdup_dequote(argv[3], GFP_KDB);
    if (!mp.help) {
// goto;
    }
    INIT_LIST_HEAD(&kdb_macro.statements);
    defcmd_in_progress = true;
    return 0;
// label;
    kfree(mp.usage);
// label;
    kfree(mp.name);
// label;
    kfree(kdb_macro);
// label;
    kdb_printf("Could not allocate new kdb_macro entry for %s\n", argv[1]);
    return KDB_NOTIMP;
    }
//
// kdb_exec_defcmd - Execute the set of commands associated with this
// defcmd name.
// Inputs:
// argc	argument count
// argv	argument vector
// Returns:
// zero for success, a kdb diagnostic if error
//
#[no_mangle]
unsafe extern "C" fn kdb_exec_defcmd(argc: c_int, argv: *const c_char) -> c_int {
    let mut ret = 0;
pub static mut kp: *mut c_void = core::ptr::null_mut();
pub static mut kmp: *mut c_void = core::ptr::null_mut();
pub static mut kms: *mut c_void = core::ptr::null_mut();
    if (argc != 0) {
    return KDB_ARGCOUNT;
    }
    list_for_each_entry(kp, &kdb_cmds_head, list_node) {
    if (strcmp(kp.name, argv[0]) == 0) {
    break;
    }
    }
    if (list_entry_is_head(kp, &kdb_cmds_head, list_node)) {
    kdb_printf("kdb_exec_defcmd: could not find commands for %s\n",
    argv[0]);
    return KDB_NOTIMP;
    }
    kmp = container_of!(kp, kdb_macro, cmd);
    list_for_each_entry(kms, &kmp.statements, list_node) {
//
// Recursive use of kdb_parse, do not use argv after this point.
//
    argv = core::ptr::null_mut();
    kdb_printf("[%s]kdb> %s\n", kmp.cmd.name, kms.statement);
    ret = kdb_parse(kms.statement);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
// Command history
pub const KDB_CMD_HISTORY_COUNT: c_int = 32;

// size == 256
    static unsigned int cmd_head, cmd_tail;
    static unsigned int cmdptr;
    static char cmd_hist[KDB_CMD_HISTORY_COUNT][CMD_BUFLEN];
    static char cmd_cur[CMD_BUFLEN];
//
// The "str" argument may point to something like  | grep xyz
//
#[no_mangle]
unsafe extern "C" fn parse_grep(str: *const c_char) {
    let mut len = 0;
    let mut cp = str, *cp2;
// sanity check: we should have been called with the \ first
    if (*cp != '|') {
    return;
    }
    cp += 1;
    while (isspace(*cp)) {
    cp += 1;
    }
    if (!str_has_prefix(cp, "grep ")) {
    kdb_printf("invalid 'pipe', see grephelp\n");
    return;
    }
    cp += 5;
    while (isspace(*cp)) {
    cp += 1;
    }
    cp2 = strchr(cp, '\n');
    if (cp2) {
// cp2 = '\0'; // remove the trailing newline
    }
    len = strlen(cp);
    if (len == 0) {
    kdb_printf("invalid 'pipe', see grephelp\n");
    return;
    }
// now cp points to a nonzero length search string
    if (*cp == '"') {
// allow it be "x y z" by removing the "'s - there must
    be two of them */
    cp += 1;
    cp2 = strchr(cp, '"');
    if (!cp2) {
    kdb_printf("invalid quoted string, see grephelp\n");
    return;
    }
// cp2 = '\0'; // end the string where the 2nd " was
    }
    kdb_grep_leading = 0;
    if (*cp == '^') {
    kdb_grep_leading = 1;
    cp += 1;
    }
    len = strlen(cp);
    kdb_grep_trailing = 0;
    if (*(cp+len-1) == '$') {
    kdb_grep_trailing = 1;
// (cp+len-1) = '\0';
    }
    len = strlen(cp);
    if (!len) {
    return;
    }
    if (len >= KDB_GREP_STRLEN) {
    kdb_printf("search string too long\n");
    return;
    }
    memcpy(kdb_grep_string, cp, len + 1);
    kdb_grepping_flag += 1;
    return;
    }
//
// kdb_parse - Parse the command line, search the command table for a
// matching command and invoke the command function.  This
// function may be called recursively, if it is, the second call
// will overwrite argv and cbuf.  It is the caller's
// responsibility to save their argv if they recursively call
// kdb_parse().
// Parameters:
// cmdstr	The input command line to be parsed.
// regs	The registers at the time kdb was entered.
// Returns:
// Zero for success, a kdb diagnostic if failure.
// Remarks:
// Limited to 20 tokens.
//
// Real rudimentary tokenization. Basically only whitespace
// is considered a token delimiter (but special consideration
// is taken of the '=' sign as used by the 'set' command).
//
// The algorithm used to tokenize the input string relies on
// there being at least one whitespace (or otherwise useless)
// character between tokens as the character immediately following
// the token is altered in-place to a null-byte to terminate the
// token string.
//
pub const MAXARGC: c_int = 20;
#[no_mangle]
pub unsafe extern "C" fn kdb_parse(cmdstr: *const c_char) -> c_int {
    static char *argv[MAXARGC];
    static int argc;
    static char cbuf[CMD_BUFLEN+2];
pub static mut cp: *mut c_void = core::ptr::null_mut();
    char *cpp, quoted;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    int escaped, ignore_errors = 0, check_grep = 0;
//
// First tokenize the command string.
//
    cp = cmdstr;
    if (KDB_FLAG(CMD_INTERRUPT)) {
// Previous command was interrupted, newline must not
// repeat the command
    KDB_FLAG_CLEAR(CMD_INTERRUPT);
    KDB_STATE_SET(PAGER);
    argc = 0;	/* no repeat */
    }
    if (*cp != '\n' && *cp != '\0') {
    argc = 0;
    cpp = cbuf;
    while (*cp) {
// skip whitespace
    while (isspace(*cp)) {
    cp += 1;
    }
    if ((*cp == '\0') || (*cp == '\n') ||
    (*cp == '#' && !defcmd_in_progress)) {
    break;
    }
// special case: check for | grep pattern
    if (*cp == '|') {
    check_grep += 1;
    break;
    }
    if (cpp >= cbuf + CMD_BUFLEN) {
    kdb_printf("kdb_parse: command buffer "
    "overflow, command ignored\n%s\n",
    cmdstr);
    return KDB_NOTFOUND;
    }
    if (argc >= MAXARGC - 1) {
    kdb_printf("kdb_parse: too many arguments, "
    "command ignored\n%s\n", cmdstr);
    return KDB_NOTFOUND;
    }
    argv[argc++] = cpp;
    escaped = 0;
    quoted = '\0';
// Copy to next unquoted and unescaped
// whitespace or '='
    while (*cp && *cp != '\n' &&
    (escaped || quoted || !isspace(*cp))) {
    if (cpp >= cbuf + CMD_BUFLEN) {
    break;
    }
    if (escaped) {
    escaped = 0;
// cpp++ = *cp += 1;
    continue;
    }
    if (*cp == '\\') {
    escaped = 1;
    cp += 1;
    continue;
    }
    if (*cp == quoted) {
    quoted = '\0';
    }

    else if (*cp == '\'' || *cp == '"') {
    quoted = *cp;
    }
// cpp = *cp += 1;
    if (*cpp == '=' && !quoted) {
    break;
    }
    cpp += 1;
    }
// cpp++ = '\0';	// Squash a ws or '=' character
    }
    }
    if (!argc) {
    return 0;
    }
    if (check_grep) {
    parse_grep(cp);
    }
    if (defcmd_in_progress) {
pub static mut result: c_int = 0;
    if (!defcmd_in_progress) {
    argc = 0;	/* avoid repeat on endefcmd */
// (argv[0]) = '\0';
    }
    return result;
    }
    if (argv[0][0] == '-' && argv[0][1] &&
    (argv[0][1] < '0' || argv[0][1] > '9')) {
    ignore_errors = 1;
    ++argv[0];
    }
    list_for_each_entry(tp, &kdb_cmds_head, list_node) {
//
// If this command is allowed to be abbreviated,
// check to see if this is it.
//
    if (tp.minlen && (strlen(argv[0]) <= tp.minlen) &&
    (strncmp(argv[0], tp.name, tp.minlen) == 0)) {
    break;
    }
    if (strcmp(argv[0], tp.name) == 0) {
    break;
    }
    }
//
// If we don't find a command by this name, see if the first
// few characters of this match any of the known commands.
// e.g., md1c20 should match md.
//
    if (list_entry_is_head(tp, &kdb_cmds_head, list_node)) {
    list_for_each_entry(tp, &kdb_cmds_head, list_node) {
    if (strncmp(argv[0], tp.name, strlen(tp.name)) == 0) {
    break;
    }
    }
    }
    if (!list_entry_is_head(tp, &kdb_cmds_head, list_node)) {
    let mut result = 0;
    if (!kdb_check_flags(tp.flags, kdb_cmd_enabled, argc <= 1)) {
    return KDB_NOPERM;
    }
    KDB_STATE_SET(CMD);
    result = (*tp.func)(argc-1, argv);
    if (result && ignore_errors && result > KDB_CMD_GO) {
    result = 0;
    }
    KDB_STATE_CLEAR(CMD);
    if (tp.flags & KDB_REPEAT_WITH_ARGS) {
    return result;
    }
    argc = tp.flags & KDB_REPEAT_NO_ARGS ? 1 : 0;
    if (argv[argc]) {
// (argv[argc]) = '\0';
    }
    return result;
    }
//
// If the input with which we were presented does not
// map to an existing command, attempt to parse it as an
// address argument and display the result.   Useful for
// obtaining the address of a variable, or the nearest symbol
// to an address contained in a register.
//
    {
    let mut value = 0;
    let mut name = core::ptr::null_mut();
    let mut offset = 0;
pub static mut nextarg: c_int = 0;
    if (kdbgetaddrarg(0, argv, &nextarg,
    &value, &offset, &name)) {
    return KDB_NOTFOUND;
    }
    kdb_printf("%s = ", argv[0]);
    kdb_symbol_print(value, core::ptr::null_mut(), KDB_SP_DEFAULT);
    kdb_printf("\n");
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_ctrl_cmd(cmd: *mut c_char) -> c_int {
pub const CTRL_P: c_int = 16;
pub const CTRL_N: c_int = 14;
// initial situation
    if (cmd_head == cmd_tail) {
    return 0;
    }
    match (*cmd) {
    CTRL_P => {
    if (cmdptr != cmd_tail) {
    cmdptr = (cmdptr + KDB_CMD_HISTORY_COUNT - 1) %
    KDB_CMD_HISTORY_COUNT;
    }
    strscpy(cmd_cur, cmd_hist[cmdptr], CMD_BUFLEN);
    return 1;
    }
    CTRL_N => {
    if (cmdptr != cmd_head) {
    cmdptr = (cmdptr+1) % KDB_CMD_HISTORY_COUNT;
    }
    strscpy(cmd_cur, cmd_hist[cmdptr], CMD_BUFLEN);
    return 1;
    }
    }
    return 0;
    }
//
// kdb_reboot - This function implements the 'reboot' command.  Reboot
// the system immediately, or loop for ever on failure.
//
#[no_mangle]
unsafe extern "C" fn kdb_reboot(argc: c_int, argv: *const c_char) -> c_int {
    emergency_restart();
    kdb_printf("Hmm, kdb_reboot did not reboot, spinning here\n");
    while (1) {
    cpu_relax();
    }
// NOTREACHED
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_dumpregs(regs: *mut pt_regs) {
pub static mut old_lvl: c_int = 0;
    console_loglevel = CONSOLE_LOGLEVEL_MOTORMOUTH;
    kdb_trap_printk += 1;
    show_regs(regs);
    kdb_trap_printk -= 1;
    kdb_printf("\n");
    console_loglevel = old_lvl;
    }
#[no_mangle]
unsafe extern "C" fn kdb_set_current_task(p: *mut task_struct) {
    kdb_current_task = p;
    if (kdb_task_has_cpu(p)) {
    kdb_current_regs = KDB_TSKREGS(kdb_process_cpu(p));
    return;
    }
    kdb_current_regs = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn drop_newline(buf: *mut c_char) {
pub static mut len: usize = 0;
    if (len == 0) {
    return;
    }
    if (*(buf + len - 1) == '\n') {
// (buf + len - 1) = '\0';
    }
    }
//
// kdb_local - The main code for kdb.  This routine is invoked on a
// specific processor, it is not global.  The main kdb() routine
// ensures that only one processor at a time is in this routine.
// This code is called with the real reason code on the first
// entry to a kdb session, thereafter it is called with reason
// SWITCH, even if the user goes back to the original cpu.
// Inputs:
// reason		The reason KDB was invoked
// error		The hardware-defined error code
// regs		The exception frame at time of fault/breakpoint.
// db_result	Result code from the break or debug point.
// Returns:
// 0	KDB was invoked for an event which it wasn't responsible
// 1	KDB handled the event for which it was invoked.
// KDB_CMD_GO	User typed 'go'.
// KDB_CMD_CPU	User switched to another cpu.
// KDB_CMD_SS	Single step.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_local(reason: kdb_reason_t, error: c_int, regs: *mut pt_regs, db_result: kdb_dbtrap_t) -> c_int {
pub static mut cmdbuf: *mut c_void = core::ptr::null_mut();
    let mut diag = 0;
    let mut kdb_current = curr_task(raw_smp_processor_id());
    KDB_DEBUG_STATE("kdb_local 1", reason);
    kdb_check_for_lockdown();
    kdb_go_count = 0;
    if (reason == KDB_REASON_DEBUG) {
// special case below
    } else {
    kdb_printf("\nEntering kdb (current=0x%px, pid %d) ",
    kdb_current, kdb_current ? kdb_current.pid : 0);

    kdb_printf("on processor %d ", raw_smp_processor_id());

    }
    match (reason) {
    KDB_REASON_DEBUG => {
    {
//
// If re-entering kdb after a single step
// command, don't print the message.
//
    match (db_result) {
    KDB_DB_BPT => {
    kdb_printf("\nEntering kdb (0x%px, pid %d) ",
    kdb_current, kdb_current.pid);

    kdb_printf("on processor %d ", raw_smp_processor_id());

    kdb_printf("due to Debug @ " kdb_machreg_fmt "\n",
    instruction_pointer(regs));
    // break;
    }
    KDB_DB_SS => {
    // break;
    }
    KDB_DB_SSBPT => {
    KDB_DEBUG_STATE("kdb_local 4", reason);
    return 1;	/* kdba_db_trap did the work */
    }
    _ => {
    kdb_printf("kdb: Bad result from kdba_db_trap: %d\n",
    db_result);
    // break;
    }
    }
    }
    break;
    case KDB_REASON_ENTER:
    if (KDB_STATE(KEYBOARD)) {
    kdb_printf("due to Keyboard Entry\n");
    }
    else {
    kdb_printf("due to KDB_ENTER()\n");
    }
    break;
    case KDB_REASON_KEYBOARD:
    KDB_STATE_SET(KEYBOARD);
    kdb_printf("due to Keyboard Entry\n");
    break;
    case KDB_REASON_ENTER_SLAVE:
// drop through, slaves only get released via cpu switch
    case KDB_REASON_SWITCH:
    kdb_printf("due to cpu switch\n");
    break;
    case KDB_REASON_OOPS:
    kdb_printf("Oops: %s\n", kdb_diemsg);
    kdb_printf("due to oops @ " kdb_machreg_fmt "\n",
    instruction_pointer(regs));
    kdb_dumpregs(regs);
    break;
    case KDB_REASON_SYSTEM_NMI:
    kdb_printf("due to System NonMaskable Interrupt\n");
    break;
    case KDB_REASON_NMI:
    kdb_printf("due to NonMaskable Interrupt @ "
    kdb_machreg_fmt "\n",
    instruction_pointer(regs));
    break;
    case KDB_REASON_SSTEP:
    case KDB_REASON_BREAK:
    kdb_printf("due to %s @ " kdb_machreg_fmt "\n",
    reason == KDB_REASON_BREAK ?
    "Breakpoint" : "SS trap", instruction_pointer(regs));
//
// Determine if this breakpoint is one that we
// are interested in.
//
    if (db_result != KDB_DB_BPT) {
    kdb_printf("kdb: error return from kdba_bp_trap: %d\n",
    db_result);
    KDB_DEBUG_STATE("kdb_local 6", reason);
    return 0;	/* Not for us, dismiss it */
    }
    break;
    case KDB_REASON_RECURSE:
    kdb_printf("due to Recursion @ " kdb_machreg_fmt "\n",
    instruction_pointer(regs));
    break;
// label;
    kdb_printf("kdb: unexpected reason code: %d\n", reason);
    KDB_DEBUG_STATE("kdb_local 8", reason);
    return 0;	/* Not for us, dismiss it */
    }
    while (1) {
//
// Initialize pager context.
//
    kdb_nextline = 1;
    KDB_STATE_CLEAR(SUPPRESS);
    kdb_grepping_flag = 0;
// ensure the old search does not leak into '/' commands
    kdb_grep_string[0] = '\0';
    cmdbuf = cmd_cur;
// cmdbuf = '\0';
// (cmd_hist[cmd_head]) = '\0';
// label;
// PROMPT can only be set if we have MEM_READ permission.
    snprintf(kdb_prompt_str, CMD_BUFLEN, kdbgetenv("PROMPT"),
    raw_smp_processor_id());
//
// Fetch command from keyboard
//
    cmdbuf = kdb_getstr(cmdbuf, CMD_BUFLEN, kdb_prompt_str);
    if (*cmdbuf != '\n') {
    if (*cmdbuf < 32) {
    if (cmdptr == cmd_head) {
    strscpy(cmd_hist[cmd_head], cmd_cur,
    CMD_BUFLEN);
// (cmd_hist[cmd_head] +
    strlen(cmd_hist[cmd_head])-1) = '\0';
    }
    if (!handle_ctrl_cmd(cmdbuf)) {
// (cmd_cur+strlen(cmd_cur)-1) = '\0';
    }
    cmdbuf = cmd_cur;
// goto;
    } else {
    strscpy(cmd_hist[cmd_head], cmd_cur,
    CMD_BUFLEN);
    }
    cmd_head = (cmd_head+1) % KDB_CMD_HISTORY_COUNT;
    if (cmd_head == cmd_tail) {
    cmd_tail = (cmd_tail+1) % KDB_CMD_HISTORY_COUNT;
    }
    }
    cmdptr = cmd_head;
    diag = kdb_parse(cmdbuf);
    if (diag == KDB_NOTFOUND) {
    drop_newline(cmdbuf);
    kdb_printf("Unknown kdb command: '%s'\n", cmdbuf);
    diag = 0;
    }
    if (diag == KDB_CMD_GO
    || diag == KDB_CMD_CPU
    || diag == KDB_CMD_SS
    || diag == KDB_CMD_KGDB) {
    break;
    }
    if (diag) {
    kdb_cmderror(diag);
    }
    }
    KDB_DEBUG_STATE("kdb_local 9", diag);
    return diag;
    }
//
// kdb_print_state - Print the state data for the current processor
// for debugging.
// Inputs:
// text		Identifies the debug point
// value		Any integer value to be printed, e.g. reason code.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_print_state(text: *const c_char, value: c_int) {
    kdb_printf("state: %s cpu %d value %d initial %d state %x\n",
    text, raw_smp_processor_id(), value, kdb_initial_cpu,
    kdb_state);
    }
//
// kdb_main_loop - After initial setup and assignment of the
// controlling cpu, all cpus are in this loop.  One cpu is in
// control and will issue the kdb prompt, the others will spin
// until 'go' or cpu switch.
//
// To get a consistent view of the kernel stacks for all
// processes, this routine is invoked from the main kdb code via
// an architecture specific routine.  kdba_main_loop is
// responsible for making the kernel stacks consistent for all
// processes, there should be no difference between a blocked
// process and a running process as far as kdb is concerned.
// Inputs:
// reason		The reason KDB was invoked
// error		The hardware-defined error code
// reason2		kdb's current reason code.
// Initially error but can change
// according to kdb state.
// db_result	Result code from break or debug point.
// regs		The exception frame at time of fault/breakpoint.
// should always be valid.
// Returns:
// 0	KDB was invoked for an event which it wasn't responsible
// 1	KDB handled the event for which it was invoked.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_main_loop(reason: kdb_reason_t, reason2: kdb_reason_t, error: c_int, db_result: kdb_dbtrap_t, regs: *mut pt_regs) -> c_int {
pub static mut result: c_int = 1;
// Stay in kdb() until 'go', 'ss[b]' or an error
    while (1) {
//
// All processors except the one that is in control
// will spin here.
//
    KDB_DEBUG_STATE("kdb_main_loop 1", reason);
    while (KDB_STATE(HOLD_CPU)) {
// state KDB is turned off by kdb_cpu to see if the
// other cpus are still live, each cpu in this loop
// turns it back on.
//
    if (!KDB_STATE(KDB)) {
    KDB_STATE_SET(KDB);
    }
    }
    KDB_STATE_CLEAR(SUPPRESS);
    KDB_DEBUG_STATE("kdb_main_loop 2", reason);
    if (KDB_STATE(LEAVING)) {
    break;	/* Another cpu said 'go' */
    }
// Still using kdb, this processor is in control
    result = kdb_local(reason2, error, regs, db_result);
    KDB_DEBUG_STATE("kdb_main_loop 3", result);
    if (result == KDB_CMD_CPU) {
    break;
    }
    if (result == KDB_CMD_SS) {
    KDB_STATE_SET(DOING_SS);
    break;
    }
    if (result == KDB_CMD_KGDB) {
    if (!KDB_STATE(DOING_KGDB)) {
    kdb_printf("Entering please attach debugger "
    "or use $D#44+ or $3#33\n");
    }
    break;
    }
    if (result && result != 1 && result != KDB_CMD_GO) {
    kdb_printf("\nUnexpected kdb_local return code %d\n",
    result);
    }
    KDB_DEBUG_STATE("kdb_main_loop 4", reason);
    break;
    }
    if (KDB_STATE(DOING_SS)) {
    KDB_STATE_CLEAR(SSBPT);
    }
// Clean up any keyboard devices before leaving
    kdb_kbd_cleanup_state();
    return result;
    }
//
// kdb_mdr - This function implements the guts of the 'mdr', memory
// read command.
// mdr  <addr arg>,<byte count>
// Inputs:
// addr	Start address
// count	Number of bytes
// Returns:
// Always 0.  Any errors are detected and printed by kdb_getarea.
//
#[no_mangle]
unsafe extern "C" fn kdb_mdr(addr: c_ulong, count: c_uint) -> c_int {
    let mut c = 0;
    while (count--) {
    if (kdb_getarea(c, addr)) {
    return 0;
    }
    kdb_printf("%02x", c);
    addr += 1;
    }
    kdb_printf("\n");
    return 0;
    }
//
// kdb_md - This function implements the 'md', 'md1', 'md2', 'md4',
// 'md8' 'mdr' and 'mds' commands.
//
// md|mds  [<addr arg> [<line count> [<radix>]]]
// mdWcN	[<addr arg> [<line count> [<radix>]]]
// where W = is the width (1, 2, 4 or 8) and N is the count.
// for eg., md1c20 reads 20 bytes, 1 at a time.
// mdr  <addr arg>,<byte count>
//
#[no_mangle]
pub unsafe extern "C" fn kdb_md_line(fmtstr: *mut c_char, addr: c_ulong, symbolic: c_int, nosect: c_int, bytesperword: c_int, num: c_int, repeat: c_int, phys: c_int) {
// print just one line of data
    let mut symtab;
    char cbuf[32];
    let mut c = cbuf;
    let mut i = 0;
    let mut j = 0;
    let mut word = 0;
    memset(cbuf, '\0', sizeof!(cbuf));
    if (phys) {
    kdb_printf("phys " kdb_machreg_fmt0 " ", addr);
    }
    else {
    kdb_printf(kdb_machreg_fmt0 " ", addr);
    }
    while (i < num && repeat--) {
    if (phys) {
    if (kdb_getphysword(&word, addr, bytesperword)) {
    break;
    }
    } else if (kdb_getword(&word, addr, bytesperword)) {
    break;
    }
    kdb_printf(fmtstr, word);
    if (symbolic) {
    kdbnearsym(word, &symtab);
    }
    else {
    memset(&symtab, 0, sizeof!(symtab));
    }
    if (symtab.sym_name) {
    kdb_symbol_print(word, &symtab, 0);
    if (!nosect) {
    kdb_printf("\n");
    kdb_printf("                       %s %s "
    kdb_machreg_fmt " "
    kdb_machreg_fmt " "
    kdb_machreg_fmt, symtab.mod_name,
    symtab.sec_name, symtab.sec_start,
    symtab.sym_start, symtab.sym_end);
    }
    addr += bytesperword;
    } else {
    union {
    let mut word = 0;
    unsigned char c[8];
    } wc;
pub static mut cp: *mut c_void = core::ptr::null_mut();

    cp = wc.c + 8 - bytesperword;

    cp = wc.c;

    wc.word = word;

    ({unsigned char __c = c; isascii(__c) && isprint(__c) ? __c : '.'; })
    for (j = 0; j < bytesperword; j++) {
// c++ = printable_char(*cp++);
    }
    addr += bytesperword;

    }
    }
    kdb_printf("%*s %s\n", (int)((num-i)*(2*bytesperword + 1)+1),
    " ", cbuf);
    }
#[no_mangle]
unsafe extern "C" fn kdb_md(argc: c_int, argv: *const c_char) -> c_int {
    static unsigned long last_addr;
    static int last_radix, last_bytesperword, last_repeat;
pub static mut radix: c_int = 0;
pub static mut nosect: c_int = 0;
    char fmtchar, fmtstr[64];
    let mut addr = 0;
    let mut word = 0;
pub static mut offset: c_long = 0;
pub static mut symbolic: c_int = 0;
pub static mut valid: c_int = 0;
pub static mut phys: c_int = 0;
pub static mut raw: c_int = 0;
    kdbgetintenv("MDCOUNT", &mdcount);
    kdbgetintenv("RADIX", &radix);
    kdbgetintenv("BYTESPERWORD", &bytesperword);
// Assume 'md <addr>' and start with environment values
    repeat = mdcount * 16 / bytesperword;
    if (strcmp(argv[0], "mdr") == 0) {
    if (argc == 2 || (argc == 0 && last_addr != 0)) {
    valid = raw = 1;
    }
    else {
    return KDB_ARGCOUNT;
    }
    } else if (isdigit(argv[0][2])) {
    bytesperword = (int)(argv[0][2] - '0');
    if (bytesperword == 0) {
    bytesperword = last_bytesperword;
    if (bytesperword == 0) {
    bytesperword = 4;
    }
    }
    last_bytesperword = bytesperword;
    repeat = mdcount * 16 / bytesperword;
    if (!argv[0][3]) {
    valid = 1;
    }
if true {
    if (kstrtouint(argv[0] + 4, 10, &repeat)) {
    return KDB_BADINT;
    }
    mdcount = ((repeat * bytesperword) + 15) / 16;
    valid = 1;
    }
    last_repeat = repeat;
    } else if (strcmp(argv[0], "md") == 0) {
    valid = 1;
    }

    else if (strcmp(argv[0], "mds") == 0) {
    valid = 1;
    }
if true {
    phys = valid = 1;
    }
    if (!valid) {
    return KDB_NOTFOUND;
    }
    if (argc == 0) {
    if (last_addr == 0) {
    return KDB_ARGCOUNT;
    }
    addr = last_addr;
    radix = last_radix;
    bytesperword = last_bytesperword;
    repeat = last_repeat;
    if (raw) {
    mdcount = repeat;
    }
    else {
    mdcount = ((repeat * bytesperword) + 15) / 16;
    }
    }
    if (argc) {
    let mut val = 0;
    int diag, nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg, &addr,
    &offset, core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    if (argc > nextarg+2) {
    return KDB_ARGCOUNT;
    }
    if (argc >= nextarg) {
    diag = kdbgetularg(argv[nextarg], &val);
    if (!diag) {
    mdcount = (int) val;
    if (raw) {
    repeat = mdcount;
    }
    else {
    repeat = mdcount * 16 / bytesperword;
    }
    }
    }
    if (argc >= nextarg+1) {
    diag = kdbgetularg(argv[nextarg+1], &val);
    if (!diag) {
    radix = (int) val;
    }
    }
    }
    if (strcmp(argv[0], "mdr") == 0) {
    let mut ret = 0;
    last_addr = addr;
    ret = kdb_mdr(addr, mdcount);
    last_addr += mdcount;
    last_repeat = mdcount;
    last_bytesperword = bytesperword; // to make REPEAT happy
    return ret;
    }
    match (radix) {
    10 => {
    fmtchar = 'd';
    // break;
    }
    16 => {
    fmtchar = 'x';
    // break;
    }
    8 => {
    fmtchar = 'o';
    // break;
    }
    _ => {
    return KDB_BADRADIX;
    }
    }
    last_radix = radix;
    if (bytesperword > KDB_WORD_SIZE) {
    return KDB_BADWIDTH;
    }
    match (bytesperword) {
    8 => {
    sprintf(fmtstr, "%%16.16l%c ", fmtchar);
    // break;
    }
    4 => {
    sprintf(fmtstr, "%%8.8l%c ", fmtchar);
    // break;
    }
    2 => {
    sprintf(fmtstr, "%%4.4l%c ", fmtchar);
    // break;
    }
    1 => {
    sprintf(fmtstr, "%%2.2l%c ", fmtchar);
    // break;
    }
    _ => {
    return KDB_BADWIDTH;
    }
    }
    last_repeat = repeat;
    last_bytesperword = bytesperword;
    if (strcmp(argv[0], "mds") == 0) {
    symbolic = 1;
// Do not save these changes as last_*, they are temporary mds
// overrides.
//
    bytesperword = KDB_WORD_SIZE;
    repeat = mdcount;
    kdbgetintenv("NOSECT", &nosect);
    }
// Round address down modulo BYTESPERWORD
    addr &= ~(bytesperword-1);
    while (repeat > 0) {
    let mut a = 0;
    int n, z, num = (symbolic ? 1 : (16 / bytesperword));
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    while (z < repeat) {
    if (phys) {
    if (kdb_getphysword(&word, a, bytesperword)
    || word) {
    break;
    }
    } else if (kdb_getword(&word, a, bytesperword) || word) {
    break;
    }
    }
    n = min(num, repeat);
    kdb_md_line(fmtstr, addr, symbolic, nosect, bytesperword,
    num, repeat, phys);
    addr += bytesperword * n;
    repeat -= n;
    z = (z + num - 1) / num;
    if (z > 2) {
pub static mut s: c_int = 0;
    kdb_printf(kdb_machreg_fmt0 "-" kdb_machreg_fmt0
    " zero suppressed\n",
    addr, addr + bytesperword * s - 1);
    addr += bytesperword * s;
    repeat -= s;
    }
    }
    last_addr = addr;
    return 0;
    }
//
// kdb_mm - This function implements the 'mm' command.
// mm address-expression new-value
// Remarks:
// mm works on machine words, mmW works on bytes.
//
#[no_mangle]
unsafe extern "C" fn kdb_mm(argc: c_int, argv: *const c_char) -> c_int {
    let mut diag = 0;
    let mut addr = 0;
pub static mut offset: c_long = 0;
    let mut contents = 0;
    let mut nextarg = 0;
    let mut width = 0;
    if (argv[0][2] && !isdigit(argv[0][2])) {
    return KDB_NOTFOUND;
    }
    if (argc < 2) {
    return KDB_ARGCOUNT;
    }
    nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg, &addr, &offset, core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    if (nextarg > argc) {
    return KDB_ARGCOUNT;
    }
    diag = kdbgetaddrarg(argc, argv, &nextarg, &contents, core::ptr::null_mut(), core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    if (nextarg != argc + 1) {
    return KDB_ARGCOUNT;
    }
    width = argv[0][2] ? (argv[0][2] - '0') : (KDB_WORD_SIZE);
    diag = kdb_putword(addr, contents, width);
    if (diag) {
    return diag;
    }
    kdb_printf(kdb_machreg_fmt " = " kdb_machreg_fmt "\n", addr, contents);
    return 0;
    }
//
// kdb_go - This function implements the 'go' command.
// go [address-expression]
//
#[no_mangle]
unsafe extern "C" fn kdb_go(argc: c_int, argv: *const c_char) -> c_int {
    let mut addr = 0;
    let mut diag = 0;
    let mut nextarg = 0;
    let mut offset = 0;
    if (raw_smp_processor_id() != kdb_initial_cpu) {
    kdb_printf("go must execute on the entry cpu, "
    "please use \"cpu %d\" and then execute go\n",
    kdb_initial_cpu);
    return KDB_BADCPUNUM;
    }
    if (argc == 1) {
    nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg,
    &addr, &offset, core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    } else if (argc) {
    return KDB_ARGCOUNT;
    }
    diag = KDB_CMD_GO;
    if (KDB_FLAG(CATASTROPHIC)) {
    kdb_printf("Catastrophic error detected\n");
    kdb_printf("kdb_continue_catastrophic=%d, ",
    kdb_continue_catastrophic);
    if (kdb_continue_catastrophic == 0 && kdb_go_count++ == 0) {
    kdb_printf("type go a second time if you really want "
    "to continue\n");
    return 0;
    }
    if (kdb_continue_catastrophic == 2) {
    kdb_printf("forcing reboot\n");
    kdb_reboot(0, core::ptr::null_mut());
    }
    kdb_printf("attempting to continue\n");
    }
    return diag;
    }
//
// kdb_rd - This function implements the 'rd' command.
//
#[no_mangle]
unsafe extern "C" fn kdb_rd(argc: c_int, argv: *const c_char) -> c_int {
pub static mut len: c_int = 0;

    let mut i = 0;
pub static mut rname: *mut c_void = core::ptr::null_mut();
    let mut rsize = 0;
    let mut reg64 = 0;
    let mut reg32 = 0;
    let mut reg16 = 0;
    let mut reg8 = 0;
    if (len) {
    return len;
    }
    while (i < DBG_MAX_REG_NUM) {
    rsize = dbg_reg_def[i].size * 2;
    if (rsize > 16) {
    rsize = 2;
    }
    if (len + strlen(dbg_reg_def[i].name) + 4 + rsize > 80) {
    len = 0;
    kdb_printf("\n");
    }
    if (len) {
    len += kdb_printf("  ");
    }
    match (dbg_reg_def[i].size * 8) {
    8 => {
    rname = dbg_get_reg(i, &reg8, kdb_current_regs);
    if (!rname) {
    // break;
    }
    len += kdb_printf("%s: %02x", rname, reg8);
    // break;
    }
    16 => {
    rname = dbg_get_reg(i, &reg16, kdb_current_regs);
    if (!rname) {
    // break;
    }
    len += kdb_printf("%s: %04x", rname, reg16);
    // break;
    }
    32 => {
    rname = dbg_get_reg(i, &reg32, kdb_current_regs);
    if (!rname) {
    // break;
    }
    len += kdb_printf("%s: %08x", rname, reg32);
    // break;
    }
    64 => {
    rname = dbg_get_reg(i, &reg64, kdb_current_regs);
    if (!rname) {
    // break;
    }
    len += kdb_printf("%s: %016llx", rname, reg64);
    // break;
    }
    _ => {
    len += kdb_printf("%s: ??", dbg_reg_def[i].name);
    }
    }
    }
    kdb_printf("\n");

    if (len) {
    return len;
    }
    kdb_dumpregs(kdb_current_regs);

    return 0;
    }
//
// kdb_rm - This function implements the 'rm' (register modify)  command.
// rm register-name new-contents
// Remarks:
// Allows register modification with the same restrictions as gdb
//
#[no_mangle]
unsafe extern "C" fn kdb_rm(argc: c_int, argv: *const c_char) -> c_int {

    let mut diag = 0;
pub static mut rname: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut reg64 = 0;
    let mut reg32 = 0;
    let mut reg16 = 0;
    let mut reg8 = 0;
    if (argc != 2) {
    return KDB_ARGCOUNT;
    }
//
// Allow presence or absence of leading '%' symbol.
//
    rname = argv[1];
    if (*rname == '%') {
    rname += 1;
    }
    diag = kdbgetu64arg(argv[2], &reg64);
    if (diag) {
    return diag;
    }
    diag = kdb_check_regs();
    if (diag) {
    return diag;
    }
    diag = KDB_BADREG;
    while (i < DBG_MAX_REG_NUM) {
    if (strcmp(rname, dbg_reg_def[i].name) == 0) {
    diag = 0;
    break;
    }
    }
    if (!diag) {
    match (dbg_reg_def[i].size * 8) {
    8 => {
    reg8 = reg64;
    dbg_set_reg(i, &reg8, kdb_current_regs);
    // break;
    }
    16 => {
    reg16 = reg64;
    dbg_set_reg(i, &reg16, kdb_current_regs);
    // break;
    }
    32 => {
    reg32 = reg64;
    dbg_set_reg(i, &reg32, kdb_current_regs);
    // break;
    }
    64 => {
    dbg_set_reg(i, &reg64, kdb_current_regs);
    // break;
    }
    }
    }
    return diag;

    kdb_printf("ERROR: Register set currently not implemented\n");
    return 0;

    }

//
// kdb_sr - This function implements the 'sr' (SYSRQ key) command
// which interfaces to the soi-disant MAGIC SYSRQ functionality.
// sr <magic-sysrq-code>
//
#[no_mangle]
unsafe extern "C" fn kdb_sr(argc: c_int, argv: *const c_char) -> c_int {
    let mut check_mask = !kdb_check_flags(KDB_ENABLE_ALL, kdb_cmd_enabled, false);
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    kdb_trap_printk += 1;
    __handle_sysrq(*argv[1], check_mask);
    kdb_trap_printk -= 1;
    return 0;
    }

//
// kdb_ef - This function implements the 'regs' (display exception
// frame) command.  This command takes an address and expects to
// find an exception frame at that address, formats and prints
// it.
// regs address-expression
// Remarks:
// Not done yet.
//
#[no_mangle]
unsafe extern "C" fn kdb_ef(argc: c_int, argv: *const c_char) -> c_int {
    let mut diag = 0;
    let mut addr = 0;
    let mut offset = 0;
    let mut nextarg = 0;
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg, &addr, &offset, core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    show_regs(addr);
    return 0;
    }
//
// kdb_env - This function implements the 'env' command.  Display the
// current environment variables.
//
#[no_mangle]
unsafe extern "C" fn kdb_env(argc: c_int, argv: *const c_char) -> c_int {
    kdb_printenv();
    if (KDB_DEBUG(MASK)) {
    kdb_printf("KDBDEBUG=0x%x\n",
    (kdb_flags & KDB_DEBUG(MASK)) >> KDB_DEBUG_FLAG_SHIFT);
    }
    return 0;
    }

//
// kdb_dmesg - This function implements the 'dmesg' command to display
// the contents of the syslog buffer.
// dmesg [lines] [adjust]
//
#[no_mangle]
unsafe extern "C" fn kdb_dmesg(argc: c_int, argv: *const c_char) -> c_int {
    let mut diag = 0;
    let mut logging = 0;
pub static mut lines: c_int = 0;
pub static mut adjust: c_int = 0;
pub static mut n: c_int = 0;
pub static mut skip: c_int = 0;
pub static mut iter: usize = 0;
    let mut len = 0;
    char buf[201];
    if (argc > 2) {
    return KDB_ARGCOUNT;
    }
    if (argc) {
    if (kstrtoint(argv[1], 0, &lines)) {
    lines = 0;
    }
    if (argc > 1 && (kstrtoint(argv[2], 0, &adjust) || adjust < 0)) {
    adjust = 0;
    }
    }
// disable LOGGING if set
    diag = kdbgetintenv("LOGGING", &logging);
    if (!diag && logging) {
    const char *setargs[] = { "set", "LOGGING", "0" };
    kdb_set(2, setargs);
    }
    kmsg_dump_rewind(&iter);
    while (kmsg_dump_get_line(&iter, 1, core::ptr::null_mut(), 0, core::ptr::null_mut())) {
    n += 1;
    }
    if (lines < 0) {
    if (adjust >= n) {
    kdb_printf("buffer only contains %d lines, nothing "
    "printed\n", n);
    }

    else if (adjust - lines >= n) {
    kdb_printf("buffer only contains %d lines, last %d "
    "lines printed\n", n, n - adjust);
    }
    skip = adjust;
    lines = abs(lines);
    } else if (lines > 0) {
    skip = n - lines - adjust;
    lines = abs(lines);
    if (adjust >= n) {
    kdb_printf("buffer only contains %d lines, "
    "nothing printed\n", n);
    skip = n;
    } else if (skip < 0) {
    lines += skip;
    skip = 0;
    kdb_printf("buffer only contains %d lines, first "
    "%d lines printed\n", n, lines);
    }
    } else {
    lines = n;
    }
    if (skip >= n || skip < 0) {
    return 0;
    }
    kmsg_dump_rewind(&iter);
    while (kmsg_dump_get_line(&iter, 1, buf, sizeof!(buf), &len)) {
    if (skip) {
    skip -= 1;
    continue;
    }
    if (!lines--) {
    break;
    }
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    kdb_printf("%.*s\n", (int)len - 1, buf);
    }
    return 0;
    }

//
// kdb_cpu - This function implements the 'cpu' command.
// cpu	[<cpunum>]
// Returns:
// KDB_CMD_CPU for success, a kdb diagnostic if error
//
#[no_mangle]
unsafe extern "C" fn kdb_cpu_status() {
    int i, start_cpu, first_print = 1;
    char state, prev_state = '?';
    kdb_printf("Currently on cpu %d\n", raw_smp_processor_id());
    kdb_printf("Available cpus: ");
    while (i < NR_CPUS) {
    if (!cpu_online(i)) {
    state = 'F';	/* cpu is offline */
    } else if (!kgdb_info[i].enter_kgdb) {
    state = 'D';	/* cpu is online but unresponsive */
    } else {
    state = ' ';	/* cpu is responding to kdb */
    if (kdb_task_state_char(KDB_TSK(i)) == '-') {
    state = '-';	/* idle task */
    }
    }
    if (state != prev_state) {
    if (prev_state != '?') {
    if (!first_print) {
    kdb_printf(", ");
    }
    first_print = 0;
    kdb_printf("%d", start_cpu);
    if (start_cpu < i-1) {
    kdb_printf("-%d", i-1);
    }
    if (prev_state != ' ') {
    kdb_printf("(%c)", prev_state);
    }
    }
    prev_state = state;
    start_cpu = i;
    }
    }
// print the trailing cpus, ignoring them if they are all offline
    if (prev_state != 'F') {
    if (!first_print) {
    kdb_printf(", ");
    }
    kdb_printf("%d", start_cpu);
    if (start_cpu < i-1) {
    kdb_printf("-%d", i-1);
    }
    if (prev_state != ' ') {
    kdb_printf("(%c)", prev_state);
    }
    }
    kdb_printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn kdb_cpu(argc: c_int, argv: *const c_char) -> c_int {
    let mut cpunum = 0;
    let mut diag = 0;
    if (argc == 0) {
    kdb_cpu_status();
    return 0;
    }
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    diag = kdbgetularg(argv[1], &cpunum);
    if (diag) {
    return diag;
    }
//
// Validate cpunum
//
    if ((cpunum >= CONFIG_NR_CPUS) || !kgdb_info[cpunum].enter_kgdb) {
    return KDB_BADCPUNUM;
    }
    dbg_switch_cpu = cpunum;
//
// Switch to other cpu
//
    return KDB_CMD_CPU;
    }
// The user may not realize that ps/bta with no parameters does not print idle
// or sleeping system daemon processes, so tell them how many were suppressed.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_ps_suppressed() {
pub static mut idle: c_int = 0;
    let mut cpu = 0;
    let mut p = core::ptr::null_mut();
    let mut g = core::ptr::null_mut();
    for_each_online_cpu(cpu) {
    p = curr_task(cpu);
    if (kdb_task_state(p, "-")) {
    idle += 1;
    }
    }
    for_each_process_thread(g, p) {
    if (kdb_task_state(p, "ims")) {
    daemon += 1;
    }
    }
    if (idle || daemon) {
    if (idle) {
    kdb_printf("%d idle process%s (state -)%s\n",
    idle, idle == 1 ? "" : "es",
    daemon ? " and " : "");
    }
    if (daemon) {
    kdb_printf("%d sleeping system daemon (state [ims]) "
    "process%s", daemon,
    daemon == 1 ? "" : "es");
    }
    kdb_printf(" suppressed,\nuse 'ps A' to see all.\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_ps1(p: *const task_struct) {
    let mut cpu = 0;
    let mut tmp = 0;
    if (!p ||
    copy_from_kernel_nofault(&tmp, p, sizeof!(unsigned long))) {
    return;
    }
    cpu = kdb_process_cpu(p);
    kdb_printf("0x%px %8d %8d  %d %4d   %c  0x%px %c%s\n",
    p, p.pid, p.parent.pid,
    kdb_task_has_cpu(p), kdb_process_cpu(p),
    kdb_task_state_char(p),
    (&p.thread),
    p == curr_task(raw_smp_processor_id()) ? '*' : ' ',
    p.comm);
    if (kdb_task_has_cpu(p)) {
    if (!KDB_TSK(cpu)) {
    kdb_printf("  Error: no saved data for this cpu\n");
    } else {
    if (KDB_TSK(cpu) != p) {
    kdb_printf("  Error: does not match running "
    "process table (0x%px)\n", KDB_TSK(cpu));
    }
    }
    }
    }
//
// kdb_ps - This function implements the 'ps' command which shows a
// list of the active processes.
//
// ps [<state_chars>]   Show processes, optionally selecting only those whose
// state character is found in <state_chars>.
//
#[no_mangle]
unsafe extern "C" fn kdb_ps(argc: c_int, argv: *const c_char) -> c_int {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut mask: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    if (argc == 0) {
    kdb_ps_suppressed();
    }
    kdb_printf("%-*s      Pid   Parent [*] cpu State %-*s Command\n",
    (int)(2*sizeof!)+2, "Task Addr",
    (int)(2*sizeof!)+2, "Thread");
    mask = argc ? argv[1] : kdbgetenv("PS");
// Run the active tasks first
    for_each_online_cpu(cpu) {
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    p = curr_task(cpu);
    if (kdb_task_state(p, mask)) {
    kdb_ps1(p);
    }
    }
    kdb_printf("\n");
// Now the real tasks
    for_each_process_thread(g, p) {
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    if (kdb_task_state(p, mask)) {
    kdb_ps1(p);
    }
    }
    return 0;
    }
//
// kdb_pid - This function implements the 'pid' command which switches
// the currently active process.
// pid [<pid> | R]
//
#[no_mangle]
unsafe extern "C" fn kdb_pid(argc: c_int, argv: *const c_char) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
    let mut diag = 0;
    if (argc > 1) {
    return KDB_ARGCOUNT;
    }
    if (argc) {
    if (strcmp(argv[1], "R") == 0) {
    p = KDB_TSK(kdb_initial_cpu);
    } else {
    diag = kdbgetularg(argv[1], &val);
    if (diag) {
    return KDB_BADINT;
    }
    p = find_task_by_pid_ns((pid_t)val,	&init_pid_ns);
    if (!p) {
    kdb_printf("No task with pid=%d\n", (pid_t)val);
    return 0;
    }
    }
    kdb_set_current_task(p);
    }
    kdb_printf("KDB current process is %s(pid=%d)\n",
    kdb_current_task.comm,
    kdb_current_task.pid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_kgdb(argc: c_int, argv: *const c_char) -> c_int {
    return KDB_CMD_KGDB;
    }
//
// kdb_help - This function implements the 'help' and '?' commands.
//
#[no_mangle]
unsafe extern "C" fn kdb_help(argc: c_int, argv: *const c_char) -> c_int {
pub static mut kt: *mut c_void = core::ptr::null_mut();
    kdb_printf("%-15.15s %-20.20s %s\n", "Command", "Usage", "Description");
    kdb_printf("-----------------------------"
    "-----------------------------\n");
    list_for_each_entry(kt, &kdb_cmds_head, list_node) {
    let mut space = "";
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    if (!kdb_check_flags(kt.flags, kdb_cmd_enabled, true)) {
    continue;
    }
    if (strlen(kt.usage) > 20) {
    space = "\n                                    ";
    }
    kdb_printf("%-15.15s %-20s%s%s\n", kt.name,
    kt.usage, space, kt.help);
    }
    return 0;
    }
//
// kdb_kill - This function implements the 'kill' commands.
//
#[no_mangle]
unsafe extern "C" fn kdb_kill(argc: c_int, argv: *const c_char) -> c_int {
    let mut sig = 0;
    let mut pid = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (argc != 2) {
    return KDB_ARGCOUNT;
    }
    if (kstrtol(argv[1], 0, &sig)) {
    return KDB_BADINT;
    }
    if ((sig >= 0) || !valid_signal(-sig)) {
    kdb_printf("Invalid signal parameter.<-signal>\n");
    return 0;
    }
    sig = -sig;
    if (kstrtol(argv[2], 0, &pid)) {
    return KDB_BADINT;
    }
    if (pid <= 0) {
    kdb_printf("Process ID must be large than 0.\n");
    return 0;
    }
// Find the process.
    p = find_task_by_pid_ns(pid, &init_pid_ns);
    if (!p) {
    kdb_printf("The specified process isn't found.\n");
    return 0;
    }
    p = p.group_leader;
    kdb_send_sig(p, sig);
    return 0;
    }
//
// Most of this code has been lifted from kernel/timer.c::sys_sysinfo().
// I cannot call that code directly from kdb, it has an unconditional
// cli()/sti() and calls routines that take locks which can stop the debugger.
//
#[no_mangle]
unsafe extern "C" fn kdb_sysinfo(val: *mut sysinfo) {
pub static mut uptime: u64 = 0;
    memset(val, 0, sizeof!(*val));
    val.uptime = div_u64(uptime, NSEC_PER_SEC);
    val.loads[0] = avenrun[0];
    val.loads[1] = avenrun[1];
    val.loads[2] = avenrun[2];
    val.procs = nr_threads-1;
    si_meminfo(val);
    return;
    }
//
// kdb_summary - This function implements the 'summary' command.
//
#[no_mangle]
unsafe extern "C" fn kdb_summary(argc: c_int, argv: *const c_char) -> c_int {
    let mut now;
pub static mut val: usize = 0;
    if (argc) {
    return KDB_ARGCOUNT;
    }
    kdb_printf("sysname    %s\n", init_uts_ns.name.sysname);
    kdb_printf("release    %s\n", init_uts_ns.name.release);
    kdb_printf("version    %s\n", init_uts_ns.name.version);
    kdb_printf("machine    %s\n", init_uts_ns.name.machine);
    kdb_printf("nodename   %s\n", init_uts_ns.name.nodename);
    kdb_printf("domainname %s\n", init_uts_ns.name.domainname);
    now = __ktime_get_real_seconds();
    kdb_printf("date       %ptTs tz_minuteswest %d\n", &now, sys_tz.tz_minuteswest);
    kdb_sysinfo(&val);
    kdb_printf("uptime     ");
    if (val.uptime > (24*60*60)) {
pub static mut days: c_int = 0;
    val.uptime %= (24*60*60);
    kdb_printf("%d day%s ", days, str_plural(days));
    }
    kdb_printf("%02ld:%02ld\n", val.uptime/(60*60), (val.uptime/60)%60);
    kdb_printf("load avg   %ld.%02ld %ld.%02ld %ld.%02ld\n",
    LOAD_INT(val.loads[0]), LOAD_FRAC(val.loads[0]),
    LOAD_INT(val.loads[1]), LOAD_FRAC(val.loads[1]),
    LOAD_INT(val.loads[2]), LOAD_FRAC(val.loads[2]));
// Display in kilobytes

    kdb_printf("\nMemTotal:       %8lu kB\nMemFree:        %8lu kB\n"
    "Buffers:        %8lu kB\n",
    K(val.totalram), K(val.freeram), K(val.bufferram));
    return 0;
    }
//
// kdb_per_cpu - This function implements the 'per_cpu' command.
//
#[no_mangle]
unsafe extern "C" fn kdb_per_cpu(argc: c_int, argv: *const c_char) -> c_int {
    char fmtstr[64];
    int cpu, diag, nextarg = 1;
    unsigned long addr, symaddr, val, bytesperword = 0, whichcpu = ~0UL;
    if (argc < 1 || argc > 3) {
    return KDB_ARGCOUNT;
    }
    diag = kdbgetaddrarg(argc, argv, &nextarg, &symaddr, core::ptr::null_mut(), core::ptr::null_mut());
    if (diag) {
    return diag;
    }
    if (argc >= 2) {
    diag = kdbgetularg(argv[2], &bytesperword);
    if (diag) {
    return diag;
    }
    }
    if (!bytesperword) {
    bytesperword = KDB_WORD_SIZE;
    }

    else if (bytesperword > KDB_WORD_SIZE) {
    return KDB_BADWIDTH;
    }
    sprintf(fmtstr, "%%0%dlx ", (int)(2*bytesperword));
    if (argc >= 3) {
    diag = kdbgetularg(argv[3], &whichcpu);
    if (diag) {
    return diag;
    }
    if (whichcpu >= nr_cpu_ids || !cpu_online(whichcpu)) {
    kdb_printf("cpu %ld is not online\n", whichcpu);
    return KDB_BADCPUNUM;
    }
    }
// Most architectures use __per_cpu_offset[cpu], some use
// __per_cpu_offset(cpu), smp has no __per_cpu_offset.
//

pub const KDB_PCU(cpu): c_int = 0;

    for_each_online_cpu(cpu) {
    if (KDB_FLAG(CMD_INTERRUPT)) {
    return 0;
    }
    if (whichcpu != ~0UL && whichcpu != cpu) {
    continue;
    }
    addr = symaddr + KDB_PCU(cpu);
    diag = kdb_getword(&val, addr, bytesperword);
    if (diag) {
    kdb_printf("%5d " kdb_bfd_vma_fmt0 " - unable to "
    "read, diag=%d\n", cpu, addr, diag);
    continue;
    }
    kdb_printf("%5d ", cpu);
    kdb_md_line(fmtstr, addr,
    bytesperword == KDB_WORD_SIZE,
    1, bytesperword, 1, 1, 0);
    }

    return 0;
    }
//
// display help for the use of cmd | grep pattern
//
#[no_mangle]
unsafe extern "C" fn kdb_grep_help(argc: c_int, argv: *const c_char) -> c_int {
    kdb_printf("Usage of  cmd args | grep pattern:\n");
    kdb_printf("  Any command's output may be filtered through an ");
    kdb_printf("emulated 'pipe'.\n");
    kdb_printf("  'grep' is just a key word.\n");
    kdb_printf("  The pattern may include a very limited set of "
    "metacharacters:\n");
    kdb_printf("   pattern or ^pattern or pattern$ or ^pattern$\n");
    kdb_printf("  And if there are spaces in the pattern, you may "
    "quote it:\n");
    kdb_printf("   \"pat tern\" or \"^pat tern\" or \"pat tern$\""
    " or \"^pat tern$\"\n");
    return 0;
    }
//
// kdb_register() - This function is used to register a kernel debugger
// command.
// @cmd: pointer to kdb command
//
// Note that it's the job of the caller to keep the memory for the cmd
// allocated until unregister is called.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_register(cmd: *mut kdbtab_t) -> c_int {
pub static mut kp: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(kp, &kdb_cmds_head, list_node) {
    if (strcmp(kp.name, cmd.name) == 0) {
    kdb_printf("Duplicate kdb cmd: %s, func %p help %s\n",
    cmd.name, cmd.func, cmd.help);
    return 1;
    }
    }
    list_add_tail(&cmd.list_node, &kdb_cmds_head);
    return 0;
    }
    EXPORT_SYMBOL_GPL(kdb_register);
//
// kdb_register_table() - This function is used to register a kdb command
// table.
// @kp: pointer to kdb command table
// @len: length of kdb command table
//
#[no_mangle]
pub unsafe extern "C" fn kdb_register_table(kp: *mut kdbtab_t, len: usize) {
    while (len--) {
    list_add_tail(&kp.list_node, &kdb_cmds_head);
    kp += 1;
    }
    }
//
// kdb_unregister() - This function is used to unregister a kernel debugger
// command. It is generally called when a module which
// implements kdb command is unloaded.
// @cmd: pointer to kdb command
//
#[no_mangle]
pub unsafe extern "C" fn kdb_unregister(cmd: *mut kdbtab_t) {
    list_del(&cmd.list_node);
    }
    EXPORT_SYMBOL_GPL(kdb_unregister);
    static kdbtab_t maintab[] = {
    {	.name = "md",
    .func = kdb_md,
    .usage = "<vaddr>",
    .help = "Display Memory Contents, also mdWcN, e.g. md8c1",
    .minlen = 1,
    .flags = KDB_ENABLE_MEM_READ | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "mdr",
    .func = kdb_md,
    .usage = "<vaddr> <bytes>",
    .help = "Display Raw Memory",
    .flags = KDB_ENABLE_MEM_READ | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "mdp",
    .func = kdb_md,
    .usage = "<paddr> <bytes>",
    .help = "Display Physical Memory",
    .flags = KDB_ENABLE_MEM_READ | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "mds",
    .func = kdb_md,
    .usage = "<vaddr>",
    .help = "Display Memory Symbolically",
    .flags = KDB_ENABLE_MEM_READ | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "mm",
    .func = kdb_mm,
    .usage = "<vaddr> <contents>",
    .help = "Modify Memory Contents",
    .flags = KDB_ENABLE_MEM_WRITE | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "go",
    .func = kdb_go,
    .usage = "[<vaddr>]",
    .help = "Continue Execution",
    .minlen = 1,
    .flags = KDB_ENABLE_REG_WRITE |
    KDB_ENABLE_ALWAYS_SAFE_NO_ARGS,
    },
    {	.name = "rd",
    .func = kdb_rd,
    .usage = "",
    .help = "Display Registers",
    .flags = KDB_ENABLE_REG_READ,
    },
    {	.name = "rm",
    .func = kdb_rm,
    .usage = "<reg> <contents>",
    .help = "Modify Registers",
    .flags = KDB_ENABLE_REG_WRITE,
    },
    {	.name = "ef",
    .func = kdb_ef,
    .usage = "<vaddr>",
    .help = "Display exception frame",
    .flags = KDB_ENABLE_MEM_READ,
    },
    {	.name = "bt",
    .func = kdb_bt,
    .usage = "[<vaddr>]",
    .help = "Stack traceback",
    .minlen = 1,
    .flags = KDB_ENABLE_MEM_READ | KDB_ENABLE_INSPECT_NO_ARGS,
    },
    {	.name = "btp",
    .func = kdb_bt,
    .usage = "<pid>",
    .help = "Display stack for process <pid>",
    .flags = KDB_ENABLE_INSPECT,
    },
    {	.name = "bta",
    .func = kdb_bt,
    .usage = "[<state_chars>|A]",
    .help = "Backtrace all processes whose state matches",
    .flags = KDB_ENABLE_INSPECT,
    },
    {	.name = "btc",
    .func = kdb_bt,
    .usage = "",
    .help = "Backtrace current process on each cpu",
    .flags = KDB_ENABLE_INSPECT,
    },
    {	.name = "btt",
    .func = kdb_bt,
    .usage = "<vaddr>",
    .help = "Backtrace process given its struct task address",
    .flags = KDB_ENABLE_MEM_READ | KDB_ENABLE_INSPECT_NO_ARGS,
    },
    {	.name = "env",
    .func = kdb_env,
    .usage = "",
    .help = "Show environment variables",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "set",
    .func = kdb_set,
    .usage = "",
    .help = "Set environment variables",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "help",
    .func = kdb_help,
    .usage = "",
    .help = "Display Help Message",
    .minlen = 1,
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "?",
    .func = kdb_help,
    .usage = "",
    .help = "Display Help Message",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "cpu",
    .func = kdb_cpu,
    .usage = "<cpunum>",
    .help = "Switch to new cpu",
    .flags = KDB_ENABLE_ALWAYS_SAFE_NO_ARGS,
    },
    {	.name = "kgdb",
    .func = kdb_kgdb,
    .usage = "",
    .help = "Enter kgdb mode",
    .flags = 0,
    },
    {	.name = "ps",
    .func = kdb_ps,
    .usage = "[<state_chars>|A]",
    .help = "Display active task list",
    .flags = KDB_ENABLE_INSPECT,
    },
    {	.name = "pid",
    .func = kdb_pid,
    .usage = "<pidnum>",
    .help = "Switch to another task",
    .flags = KDB_ENABLE_INSPECT,
    },
    {	.name = "reboot",
    .func = kdb_reboot,
    .usage = "",
    .help = "Reboot the machine immediately",
    .flags = KDB_ENABLE_REBOOT,
    },

    {	.name = "lsmod",
    .func = kdb_lsmod,
    .usage = "",
    .help = "List loaded kernel modules",
    .flags = KDB_ENABLE_INSPECT,
    },

    {	.name = "sr",
    .func = kdb_sr,
    .usage = "<key>",
    .help = "Magic SysRq key",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },

    {	.name = "dmesg",
    .func = kdb_dmesg,
    .usage = "[lines]",
    .help = "Display syslog buffer",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },

    {	.name = "defcmd",
    .func = kdb_defcmd,
    .usage = "name \"usage\" \"help\"",
    .help = "Define a set of commands, down to endefcmd",
//
// Macros are always safe because when executed each
// internal command re-enters kdb_parse() and is safety
// checked individually.
//
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "kill",
    .func = kdb_kill,
    .usage = "<-signal> <pid>",
    .help = "Send a signal to a process",
    .flags = KDB_ENABLE_SIGNAL,
    },
    {	.name = "summary",
    .func = kdb_summary,
    .usage = "",
    .help = "Summarize the system",
    .minlen = 4,
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    {	.name = "per_cpu",
    .func = kdb_per_cpu,
    .usage = "<sym> [<bytes>] [<cpu>]",
    .help = "Display per_cpu variables",
    .minlen = 3,
    .flags = KDB_ENABLE_MEM_READ,
    },
    {	.name = "grephelp",
    .func = kdb_grep_help,
    .usage = "",
    .help = "Display help on | grep",
    .flags = KDB_ENABLE_ALWAYS_SAFE,
    },
    };
// Initialize the kdb command table.
#[no_mangle]
unsafe extern "C" fn kdb_inittab()  {
    kdb_register_table(maintab, ARRAY_SIZE!(maintab));
    }
// Execute any commands defined in kdb_cmds.
#[no_mangle]
unsafe extern "C" fn kdb_cmd_init()  {
    let mut i = 0;
    let mut diag = 0;
    while (kdb_cmds[i]) {
    diag = kdb_parse(kdb_cmds[i]);
    if (diag) {
    kdb_printf("kdb command %s failed, kdb diag %d\n",
    kdb_cmds[i], diag);
    }
    }
    if (defcmd_in_progress) {
    kdb_printf("Incomplete 'defcmd' set, forcing endefcmd\n");
    kdb_parse("endefcmd");
    }
    }
// Initialize kdb_printf, breakpoint tables and kdb state
#[no_mangle]
pub unsafe extern "C" fn kdb_init(lvl: c_int)  {
pub static mut kdb_init_lvl: int = 0;
    let mut i = 0;
    if (kdb_init_lvl == KDB_INIT_FULL || lvl <= kdb_init_lvl) {
    return;
    }
    while (i < lvl) {
    match (i) {
    KDB_NOT_INITIALIZED => {
    kdb_inittab();		/* Initialize Command Table */
    kdb_initbptab();	/* Initialize Breakpoints */
    // break;
    }
    KDB_INIT_EARLY => {
    kdb_cmd_init();		/* Build kdb_cmds tables */
    // break;
    }
    }
    }
    kdb_init_lvl = lvl;
    }
}
