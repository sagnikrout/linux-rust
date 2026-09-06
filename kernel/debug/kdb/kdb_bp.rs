//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_bp.c
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
// Kernel Debugger Architecture Independent Breakpoint Handler
//
// Copyright (c) 1999-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

//
// Table of kdb_breakpoints
//
    kdb_bp_t kdb_breakpoints[KDB_MAXBPT];
#[no_mangle]
unsafe extern "C" fn kdb_setsinglestep(regs: *mut pt_regs) {
    KDB_STATE_SET(DOING_SS);
    }
    static char *kdb_rwtypes[] = {
    "Instruction(i)",
    "Instruction(Register)",
    "Data Write",
    "I/O",
    "Data Access"
    };
#[no_mangle]
pub unsafe extern "C" fn kdb_bptype(bp: *mut kdb_bp_t) -> *mut c_void {
    if (bp.bp_type < 0 || bp.bp_type > 4) {
    return "";
    }
    return kdb_rwtypes[bp.bp_type];
    }
#[no_mangle]
unsafe extern "C" fn kdb_parsebp(argc: c_int, argv: *const c_char, nextargp: *mut c_int, bp: *mut kdb_bp_t) -> c_int {
pub static mut nextarg: c_int = 0;
    let mut diag = 0;
    bp.bph_length = 1;
    if ((argc + 1) != nextarg) {
    if (strncasecmp(argv[nextarg], "datar", sizeof!("datar")) == 0) {
    bp.bp_type = BP_ACCESS_WATCHPOINT;
    }

    else if (strncasecmp(argv[nextarg], "dataw", sizeof!("dataw")) == 0) {
    bp.bp_type = BP_WRITE_WATCHPOINT;
    }

    else if (strncasecmp(argv[nextarg], "inst", sizeof!("inst")) == 0) {
    bp.bp_type = BP_HARDWARE_BREAKPOINT;
    }
    else {
    return KDB_ARGCOUNT;
    }
    bp.bph_length = 1;
    nextarg += 1;
    if ((argc + 1) != nextarg) {
    let mut len = 0;
    diag = kdbgetularg(argv[nextarg],
    &len);
    if (diag) {
    return diag;
    }
    if (len > 8) {
    return KDB_BADLENGTH;
    }
    bp.bph_length = len;
    nextarg += 1;
    }
    if ((argc + 1) != nextarg) {
    return KDB_ARGCOUNT;
    }
    }
// nextargp = nextarg;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _kdb_bp_remove(bp: *mut kdb_bp_t) -> c_int {
pub static mut ret: c_int = 1;
    if (!bp.bp_installed) {
    return ret;
    }
    if (!bp.bp_type) {
    ret = dbg_remove_sw_break(bp.bp_addr);
    }
    else {
    ret = arch_kgdb_ops.remove_hw_breakpoint(bp.bp_addr,
    bp.bph_length,
    bp.bp_type);
    }
    if (ret == 0) {
    bp.bp_installed = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kdb_handle_bp(regs: *mut pt_regs, bp: *mut kdb_bp_t) {
    if (KDB_DEBUG(BP)) {
    kdb_printf("regs.ip = 0x%lx\n", instruction_pointer(regs));
    }
//
// Setup single step
//
    kdb_setsinglestep(regs);
//
// Reset delay attribute
//
    bp.bp_delay = 0;
    bp.bp_delayed = 1;
    }
#[no_mangle]
unsafe extern "C" fn _kdb_bp_install(regs: *mut pt_regs, bp: *mut kdb_bp_t) -> c_int {
    let mut ret = 0;
//
// Install the breakpoint, if it is not already installed.
//
    if (KDB_DEBUG(BP)) {
    kdb_printf("%s: bp_installed %d\n",
    __func__, bp.bp_installed);
    }
    if (!KDB_STATE(SSBPT)) {
    bp.bp_delay = 0;
    }
    if (bp.bp_installed) {
    return 1;
    }
    if (bp.bp_delay || (bp.bp_delayed && KDB_STATE(DOING_SS))) {
    if (KDB_DEBUG(BP)) {
    kdb_printf("%s: delayed bp\n", __func__);
    }
    kdb_handle_bp(regs, bp);
    return 0;
    }
    if (!bp.bp_type) {
    ret = dbg_set_sw_break(bp.bp_addr);
    }
    else {
    ret = arch_kgdb_ops.set_hw_breakpoint(bp.bp_addr,
    bp.bph_length,
    bp.bp_type);
    }
    if (ret == 0) {
    bp.bp_installed = 1;
    } else {
    kdb_printf("%s: failed to set breakpoint at 0x%lx\n",
    __func__, bp.bp_addr);
    if (!bp.bp_type) {
    kdb_printf("Software breakpoints are unavailable.\n"
    "  Boot the kernel with rodata=off\n"
    "  OR use hw breaks: help bph\n");
    }
    return 1;
    }
    return 0;
    }
//
// kdb_bp_install
//
// Install kdb_breakpoints prior to returning from the
// kernel debugger.  This allows the kdb_breakpoints to be set
// upon functions that are used internally by kdb, such as
// printk().  This function is only called once per kdb session.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_bp_install(regs: *mut pt_regs) {
    let mut i = 0;
    while (i < KDB_MAXBPT) {
    let mut bp = &kdb_breakpoints[i];
    if (KDB_DEBUG(BP)) {
    kdb_printf("%s: bp %d bp_enabled %d\n",
    __func__, i, bp.bp_enabled);
    }
    if (bp.bp_enabled) {
    _kdb_bp_install(regs, bp);
    }
    }
    }
//
// kdb_bp_remove
//
// Remove kdb_breakpoints upon entry to the kernel debugger.
//
// Parameters:
// None.
// Outputs:
// None.
// Returns:
// None.
// Locking:
// None.
// Remarks:
//
#[no_mangle]
pub unsafe extern "C" fn kdb_bp_remove() {
    let mut i = 0;
    while (i >= 0) {
    let mut bp = &kdb_breakpoints[i];
    if (KDB_DEBUG(BP)) {
    kdb_printf("%s: bp %d bp_enabled %d\n",
    __func__, i, bp.bp_enabled);
    }
    if (bp.bp_enabled) {
    _kdb_bp_remove(bp);
    }
    }
    }
//
// kdb_printbp
//
// Internal function to format and print a breakpoint entry.
//
// Parameters:
// None.
// Outputs:
// None.
// Returns:
// None.
// Locking:
// None.
// Remarks:
//
#[no_mangle]
unsafe extern "C" fn kdb_printbp(bp: *mut kdb_bp_t, i: c_int) {
    kdb_printf("%s ", kdb_bptype(bp));
    kdb_printf("BP #%d at ", i);
    kdb_symbol_print(bp.bp_addr, core::ptr::null_mut(), KDB_SP_DEFAULT);
    if (bp.bp_enabled) {
    kdb_printf("\n    is enabled ");
    }
    else {
    kdb_printf("\n    is disabled");
    }
    kdb_printf("  addr at %016lx, hardtype=%d installed=%d\n",
    bp.bp_addr, bp.bp_type, bp.bp_installed);
    kdb_printf("\n");
    }
//
// kdb_bp
//
// Handle the bp commands.
//
// [bp|bph] <addr-expression> [DATAR|DATAW]
//
// Parameters:
// argc	Count of arguments in argv
// argv	Space delimited command line arguments
// Outputs:
// None.
// Returns:
// Zero for success, a kdb diagnostic if failure.
// Locking:
// None.
// Remarks:
//
// bp	Set breakpoint on all cpus.  Only use hardware assist if need.
// bph	Set breakpoint on all cpus.  Force hardware register
//
#[no_mangle]
unsafe extern "C" fn kdb_bp(argc: c_int, argv: *const c_char) -> c_int {
    let mut i = 0;
    let mut bpno = 0;
    let mut bp = core::ptr::null_mut();
    let mut bp_check = core::ptr::null_mut();
    let mut diag = 0;
    let mut symname = core::ptr::null_mut();
pub static mut offset: c_long = 0;
    let mut nextarg = 0;
pub static mut template: kdb_bp_t = 0;
    if (argc == 0) {
//
// Display breakpoint table
//
    while (bpno < KDB_MAXBPT) {
    if (bp.bp_free) {
    continue;
    }
    kdb_printbp(bp, bpno);
    }
    return 0;
    }
    nextarg = 1;
    diag = kdbgetaddrarg(argc, argv, &nextarg, &template.bp_addr,
    &offset, &symname);
    if (diag) {
    return diag;
    }
    if (!template.bp_addr) {
    return KDB_BADINT;
    }
//
// This check is redundant (since the breakpoint machinery should
// be doing the same check during kdb_bp_install) but gives the
// user immediate feedback.
//
    diag = kgdb_validate_break_address(template.bp_addr);
    if (diag) {
    return diag;
    }
//
// Find an empty bp structure to allocate
//
    while (bpno < KDB_MAXBPT) {
    if (bp.bp_free) {
    break;
    }
    }
    if (bpno == KDB_MAXBPT) {
    return KDB_TOOMANYBPT;
    }
    if (strcmp(argv[0], "bph") == 0) {
    template.bp_type = BP_HARDWARE_BREAKPOINT;
    diag = kdb_parsebp(argc, argv, &nextarg, &template);
    if (diag) {
    return diag;
    }
    } else {
    template.bp_type = BP_BREAKPOINT;
    }
//
// Check for clashing breakpoints.
//
// Note, in this design we can't have hardware breakpoints
// enabled for both read and write on the same address.
//
    while (i < KDB_MAXBPT) {
    if (!bp_check.bp_free &&
    bp_check.bp_addr == template.bp_addr) {
    kdb_printf("You already have a breakpoint at "
    kdb_bfd_vma_fmt0 "\n", template.bp_addr);
    return KDB_DUPBPT;
    }
    }
    template.bp_enabled = 1;
//
// Actually allocate the breakpoint found earlier
//
// bp = template;
    bp.bp_free = 0;
    kdb_printbp(bp, bpno);
    return 0;
    }
//
// kdb_bc
//
// Handles the 'bc', 'be', and 'bd' commands
//
// [bd|bc|be] <breakpoint-number>
// [bd|bc|be]
//
// Parameters:
// argc	Count of arguments in argv
// argv	Space delimited command line arguments
// Outputs:
// None.
// Returns:
// Zero for success, a kdb diagnostic for failure
// Locking:
// None.
// Remarks:
//
#[no_mangle]
unsafe extern "C" fn kdb_bc(argc: c_int, argv: *const c_char) -> c_int {
    let mut addr = 0;
    let mut bp = core::ptr::null_mut();
pub static mut lowbp: c_int = 0;
pub static mut highbp: c_int = 0;
pub static mut done: c_int = 0;
    let mut i = 0;
pub static mut diag: c_int = 0;
    let mut cmd = 0;			/* KDBCMD_B? */
pub const KDBCMD_BC: c_int = 0;
pub const KDBCMD_BE: c_int = 1;
pub const KDBCMD_BD: c_int = 2;
    if (strcmp(argv[0], "be") == 0) {
    cmd = KDBCMD_BE;
    }

    else if (strcmp(argv[0], "bd") == 0) {
    cmd = KDBCMD_BD;
    }
    else {
    cmd = KDBCMD_BC;
    }
    if (argc != 1) {
    return KDB_ARGCOUNT;
    }
    if (strcmp(argv[1], "*") == 0) {
    lowbp = 0;
    highbp = KDB_MAXBPT;
    } else {
    diag = kdbgetularg(argv[1], &addr);
    if (diag) {
    return diag;
    }
//
// For addresses less than the maximum breakpoint number,
// assume that the breakpoint number is desired.
//
    if (addr < KDB_MAXBPT) {
    lowbp = highbp = addr;
    highbp += 1;
    } else {
    while (i < KDB_MAXBPT) {
    if (bp.bp_addr == addr) {
    lowbp = highbp = i;
    highbp += 1;
    break;
    }
    }
    }
    }
//
// Now operate on the set of breakpoints matching the input
// criteria (either '*' for all, or an individual breakpoint).
//
    while (i < highbp) {
    if (bp.bp_free) {
    continue;
    }
    done += 1;
    match (cmd) {
    KDBCMD_BC => {
    bp.bp_enabled = 0;
    kdb_printf("Breakpoint %d at "
    kdb_bfd_vma_fmt " cleared\n",
    i, bp.bp_addr);
    bp.bp_addr = 0;
    bp.bp_free = 1;
    // break;
    }
    KDBCMD_BE => {
    if (bp.bp_enabled) {
    // break;
    }
    bp.bp_enabled = 1;
    kdb_printf("Breakpoint %d at "
    kdb_bfd_vma_fmt " enabled\n",
    i, bp.bp_addr);
    // break;
    }
    KDBCMD_BD => {
    if (!bp.bp_enabled) {
    // break;
    }
    bp.bp_enabled = 0;
    kdb_printf("Breakpoint %d at "
    kdb_bfd_vma_fmt " disabled\n",
    i, bp.bp_addr);
    // break;
    }
    }
    if (bp.bp_delay && (cmd == KDBCMD_BC || cmd == KDBCMD_BD)) {
    bp.bp_delay = 0;
    KDB_STATE_CLEAR(SSBPT);
    }
    }
    return (!done) ? KDB_BPTNOTFOUND : 0;
    }
//
// kdb_ss
//
// Process the 'ss' (Single Step) command.
//
// ss
//
// Parameters:
// argc	Argument count
// argv	Argument vector
// Outputs:
// None.
// Returns:
// KDB_CMD_SS for success, a kdb error if failure.
// Locking:
// None.
// Remarks:
//
// Set the arch specific option to trigger a debug trap after the next
// instruction.
//
#[no_mangle]
unsafe extern "C" fn kdb_ss(argc: c_int, argv: *const c_char) -> c_int {
    if (argc != 0) {
    return KDB_ARGCOUNT;
    }
//
// Set trace flag and go.
//
    KDB_STATE_SET(DOING_SS);
    return KDB_CMD_SS;
    }
    static kdbtab_t bptab[] = {
    {	.name = "bp",
    .func = kdb_bp,
    .usage = "[<vaddr>]",
    .help = "Set/Display breakpoints",
    .flags = KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "bl",
    .func = kdb_bp,
    .usage = "[<vaddr>]",
    .help = "Display breakpoints",
    .flags = KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS,
    },
    {	.name = "bc",
    .func = kdb_bc,
    .usage = "<bpnum>",
    .help = "Clear Breakpoint",
    .flags = KDB_ENABLE_FLOW_CTRL,
    },
    {	.name = "be",
    .func = kdb_bc,
    .usage = "<bpnum>",
    .help = "Enable Breakpoint",
    .flags = KDB_ENABLE_FLOW_CTRL,
    },
    {	.name = "bd",
    .func = kdb_bc,
    .usage = "<bpnum>",
    .help = "Disable Breakpoint",
    .flags = KDB_ENABLE_FLOW_CTRL,
    },
    {	.name = "ss",
    .func = kdb_ss,
    .usage = "",
    .help = "Single Step",
    .minlen = 1,
    .flags = KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS,
    },
    };
    static kdbtab_t bphcmd = {
    .name = "bph",
    .func = kdb_bp,
    .usage = "[<vaddr>]",
    .help = "[datar [length]|dataw [length]]   Set hw brk",
    .flags = KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS,
    };
// Initialize the breakpoint table and register	breakpoint commands.
#[no_mangle]
pub unsafe extern "C" fn kdb_initbptab()  {
    let mut i = 0;
pub static mut bp: *mut c_void = core::ptr::null_mut();
//
// First time initialization.
//
    memset(&kdb_breakpoints, '\0', sizeof!(kdb_breakpoints));
    for (i = 0, bp = kdb_breakpoints; i < KDB_MAXBPT; i++, bp++) {
    bp.bp_free = 1;
    }
    kdb_register_table(bptab, ARRAY_SIZE!(bptab));
    if (arch_kgdb_ops.flags & KGDB_HW_BREAKPOINT) {
    kdb_register_table(&bphcmd, 1);
    }
    }