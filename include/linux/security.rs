//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/security.h
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


//
// Linux Security plug
//
// Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
// Copyright (C) 2001 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
// Copyright (C) 2001 James Morris <jmorris@intercode.com.au>
// Copyright (C) 2001 Silicon Graphics, Inc. (Trust Technology Group)
// Copyright (C) 2016 Mellanox Techonologies
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// Due to this file being licensed under the GPL there is controversy over
// whether this permits you to write a module that #includes this file
// without placing your module under the GPL.  Please consult a lawyer for
// advice before doing this.
//

// Default (no) options for the capable function
pub const CAP_OPT_NONE: c_uint = 0x0;
// If capable should audit the security request

// If capable is being called by a setid function

// LSM Agnostic defines for security_sb_set_mnt_opts() flags
pub const SECURITY_LSM_NATIVE_LABELS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsm_event {
    LSM_POLICY_CHANGE,
    LSM_STARTED_ALL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity_digest {
    pub alg: *const c_char,
    pub digest: *const u8,
    pub digest_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsm_integrity_type {
    LSM_INT_DMVERITY_SIG_VALID,
    LSM_INT_DMVERITY_ROOTHASH,
    LSM_INT_FSVERITY_BUILTINSIG_VALID,
}

//
// These are reasons that can be passed to the security_locked_down()
// LSM hook. Lockdown reasons that protect kernel integrity (ie, the
// ability for userland to modify kernel code) are placed before
// LOCKDOWN_INTEGRITY_MAX.  Lockdown reasons that protect kernel
// confidentiality (ie, the ability for userland to extract
// information from the running kernel that would otherwise be
// restricted) are placed before LOCKDOWN_CONFIDENTIALITY_MAX.
//
// LSM authors should note that the semantics of any given lockdown
// reason are not guaranteed to be stable - the same reason may block
// one set of features in one kernel release, and a slightly different
// set of features in a later kernel release. LSMs that seek to expose
// lockdown policy at any level of granularity other than "none",
// "integrity" or "confidentiality" are responsible for either
// ensuring that they expose a consistent level of functionality to
// userland, or ensuring that userland is aware that this is
// potentially a moving target. It is easy to misuse this information
// in a way that could break userspace. Please be careful not to do
// so.
//
// If you add to this, remember to extend lockdown_reasons in
// security/lockdown/lockdown.c.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdown_reason {
    LOCKDOWN_NONE,
    LOCKDOWN_MODULE_SIGNATURE,
    LOCKDOWN_DEV_MEM,
    LOCKDOWN_EFI_TEST,
    LOCKDOWN_KEXEC,
    LOCKDOWN_HIBERNATION,
    LOCKDOWN_PCI_ACCESS,
    LOCKDOWN_IOPORT,
    LOCKDOWN_MSR,
    LOCKDOWN_ACPI_TABLES,
    LOCKDOWN_DEVICE_TREE,
    LOCKDOWN_PCMCIA_CIS,
    LOCKDOWN_TIOCSSERIAL,
    LOCKDOWN_MODULE_PARAMETERS,
    LOCKDOWN_MMIOTRACE,
    LOCKDOWN_DEBUGFS,
    LOCKDOWN_XMON_WR,
    LOCKDOWN_BPF_WRITE_USER,
    LOCKDOWN_DBG_WRITE_KERNEL,
    LOCKDOWN_RTAS_ERROR_INJECTION,
    LOCKDOWN_XEN_USER_ACTIONS,
    LOCKDOWN_INTEGRITY_MAX,
    LOCKDOWN_KCORE,
    LOCKDOWN_KPROBES,
    LOCKDOWN_BPF_READ_KERNEL,
    LOCKDOWN_DBG_READ_KERNEL,
    LOCKDOWN_PERF,
    LOCKDOWN_TRACEFS,
    LOCKDOWN_XMON_RW,
    LOCKDOWN_XFRM_SECRET,
    LOCKDOWN_CONFIDENTIALITY_MAX,
}

//
// Data exported by the security modules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_prop {
    pub selinux: lsm_prop_selinux,
    pub smack: lsm_prop_smack,
    pub apparmor: lsm_prop_apparmor,
    pub bpf: lsm_prop_bpf,
}

// These functions are in security/commoncap.c
extern "C" {
    pub fn cap_settime(ts: *const timespec64, tz: *const timezone) -> c_int;
}
extern "C" {
    pub fn cap_ptrace_access_check(child: *mut task_struct, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn cap_ptrace_traceme(parent: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn cap_bprm_creds_from_file(bprm: *mut linux_binprm, file: *const file) -> c_int;
}
extern "C" {
    pub fn cap_inode_need_killpriv(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cap_inode_killpriv(idmap: *mut mnt_idmap, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cap_mmap_addr(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn cap_task_fix_setuid(new: *mut cred, old: *const cred, flags: c_int) -> c_int;
}
extern "C" {
    pub fn cap_task_setscheduler(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn cap_task_setioprio(p: *mut task_struct, ioprio: c_int) -> c_int;
}
extern "C" {
    pub fn cap_task_setnice(p: *mut task_struct, nice: c_int) -> c_int;
}
extern "C" {
    pub fn cap_vm_enough_memory(mm: *mut mm_struct, pages: c_long) -> c_int;
}

//
// A "security context" is the text representation of
// the information used by LSMs.
// This structure contains the string, its length, and which LSM
// it is useful for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_context {
    pub /: *mut *mut *mut char context; / Provided by the module,
    pub len: u32,
    pub /: *mut *mut int id; / Identifies the module,
}

//
// Values used in the task_security_ops calls
//
// setuid or setgid, id0 == uid or gid
pub const LSM_SETID_ID: c_int = 1;
// setreuid or setregid, id0 == real, id1 == eff
pub const LSM_SETID_RE: c_int = 2;
// setresuid or setresgid, id0 == real, id1 == eff, uid2 == saved
pub const LSM_SETID_RES: c_int = 4;
// setfsuid or setfsgid, id0 == fsuid or fsgid
pub const LSM_SETID_FS: c_int = 8;
// Flags for security_task_prlimit().
pub const LSM_PRLIMIT_READ: c_int = 1;
pub const LSM_PRLIMIT_WRITE: c_int = 2;
// forward declares to avoid warnings
// bprm->unsafe reasons
pub const LSM_UNSAFE_SHARE: c_int = 1;
pub const LSM_UNSAFE_PTRACE: c_int = 2;
pub const LSM_UNSAFE_NO_NEW_PRIVS: c_int = 4;

// security_inode_init_security callback function to write xattrs
// Keep the kernel_load_data_id enum in sync with kernel_read_file_id

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kernel_load_data_id {
    __kernel_read_file_id(__data_id_enumify)
}

//
// lsmprop_init - initialize a lsm_prop structure
// @prop: Pointer to the data to initialize
//
// Set all secid for all modules to the specified value.
//

//
// lsmprop_is_set - report if there is a value in the lsm_prop
// @prop: Pointer to the exported LSM data
//
// Returns true if there is a value set, false otherwise
//
extern "C" {
    pub fn call_blocking_lsm_notifier(event: lsm_event, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn register_blocking_lsm_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_blocking_lsm_notifier(nb: *mut notifier_block) -> c_int;
}
// prototypes
extern "C" {
    pub fn security_init() -> c_int;
}
extern "C" {
    pub fn early_security_init() -> c_int;
}
extern "C" {
    pub fn lsm_name_to_attr(name: *const c_char) -> u64;
}
// Security operations
extern "C" {
    pub fn security_binder_set_context_mgr(mgr: *const cred) -> c_int;
}
extern "C" {
    pub fn security_ptrace_access_check(child: *mut task_struct, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn security_ptrace_traceme(parent: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_quotactl(cmds: c_int, type: c_int, id: c_int, sb: *const super_block) -> c_int;
}
extern "C" {
    pub fn security_quota_on(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_syslog(type: c_int) -> c_int;
}
extern "C" {
    pub fn security_settime64(ts: *const timespec64, tz: *const timezone) -> c_int;
}
extern "C" {
    pub fn security_vm_enough_memory_mm(mm: *mut mm_struct, pages: c_long) -> c_int;
}
extern "C" {
    pub fn security_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int;
}
extern "C" {
    pub fn security_bprm_creds_from_file(bprm: *mut linux_binprm, file: *const file) -> c_int;
}
extern "C" {
    pub fn security_bprm_check(bprm: *mut linux_binprm) -> c_int;
}
extern "C" {
    pub fn security_bprm_committing_creds(bprm: *const linux_binprm);
}
extern "C" {
    pub fn security_bprm_committed_creds(bprm: *const linux_binprm);
}
extern "C" {
    pub fn security_fs_context_submount(fc: *mut fs_context, reference: *mut super_block) -> c_int;
}
extern "C" {
    pub fn security_fs_context_dup(fc: *mut fs_context, src_fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn security_fs_context_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
}
extern "C" {
    pub fn security_sb_alloc(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn security_sb_delete(sb: *mut super_block);
}
extern "C" {
    pub fn security_sb_free(sb: *mut super_block);
}
extern "C" {
    pub fn security_free_mnt_opts(mnt_opts: *mut c_void);
}
extern "C" {
    pub fn security_sb_eat_lsm_opts(options: *mut c_char, mnt_opts: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_sb_mnt_opts_compat(sb: *mut super_block, mnt_opts: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_sb_remount(sb: *mut super_block, mnt_opts: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_sb_kern_mount(sb: *const super_block) -> c_int;
}
extern "C" {
    pub fn security_sb_show_options(m: *mut seq_file, sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn security_sb_statfs(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_sb_umount(mnt: *mut vfsmount, flags: c_int) -> c_int;
}
extern "C" {
    pub fn security_sb_pivotroot(old_path: *const path, new_path: *const path) -> c_int;
}
extern "C" {
    pub fn security_move_mount(from_path: *const path, to_path: *const path) -> c_int;
}
extern "C" {
    pub fn security_inode_alloc(inode: *mut inode, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn security_inode_free(inode: *mut inode);
}
extern "C" {
    pub fn security_inode_create(dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn security_inode_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_mkdir(dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn security_inode_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_mknod(dir: *mut inode, dentry: *mut dentry, mode: umode_t, dev: dev_t) -> c_int;
}
extern "C" {
    pub fn security_inode_readlink(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_permission(inode: *mut inode, mask: c_int) -> c_int;
}
extern "C" {
    pub fn security_inode_getattr(path: *const path) -> c_int;
}
extern "C" {
    pub fn security_inode_getxattr(dentry: *mut dentry, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn security_inode_listxattr(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_post_removexattr(dentry: *mut dentry, name: *const c_char);
}
extern "C" {
    pub fn security_inode_need_killpriv(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_killpriv(idmap: *mut mnt_idmap, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_inode_setsecurity(inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: c_int) -> c_int;
}
extern "C" {
    pub fn security_inode_listsecurity(inode: *mut inode, buffer: *mut c_char, remaining_size: *mut isize) -> c_int;
}
extern "C" {
    pub fn security_inode_getlsmprop(inode: *mut inode, prop: *mut lsm_prop);
}
extern "C" {
    pub fn security_inode_copy_up(src: *mut dentry, new: *mut cred) -> c_int;
}
extern "C" {
    pub fn security_inode_copy_up_xattr(src: *mut dentry, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn security_file_permission(file: *mut file, mask: c_int) -> c_int;
}
extern "C" {
    pub fn security_file_alloc(file: *mut file) -> c_int;
}
extern "C" {
    pub fn security_file_release(file: *mut file);
}
extern "C" {
    pub fn security_file_free(file: *mut file);
}
extern "C" {
    pub fn security_backing_file_free(backing_file: *mut file);
}
extern "C" {
    pub fn security_file_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn security_mmap_addr(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn security_file_lock(file: *mut file, cmd: c_uint) -> c_int;
}
extern "C" {
    pub fn security_file_fcntl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn security_file_set_fowner(file: *mut file);
}
extern "C" {
    pub fn security_file_receive(file: *mut file) -> c_int;
}
extern "C" {
    pub fn security_file_open(file: *mut file) -> c_int;
}
extern "C" {
    pub fn security_file_post_open(file: *mut file, mask: c_int) -> c_int;
}
extern "C" {
    pub fn security_file_truncate(file: *mut file) -> c_int;
}
extern "C" {
    pub fn security_task_alloc(task: *mut task_struct, clone_flags: u64) -> c_int;
}
extern "C" {
    pub fn security_task_free(task: *mut task_struct);
}
extern "C" {
    pub fn security_cred_alloc_blank(cred: *mut cred, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn security_cred_free(cred: *mut cred);
}
extern "C" {
    pub fn security_prepare_creds(new: *mut cred, old: *const cred, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn security_transfer_creds(new: *mut cred, old: *const cred);
}
extern "C" {
    pub fn security_cred_getsecid(c: *const cred, secid: *mut u32);
}
extern "C" {
    pub fn security_cred_getlsmprop(c: *const cred, prop: *mut lsm_prop);
}
extern "C" {
    pub fn security_kernel_act_as(new: *mut cred, secid: u32) -> c_int;
}
extern "C" {
    pub fn security_kernel_create_files_as(new: *mut cred, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn security_kernel_module_request(kmod_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn security_kernel_load_data(id: kernel_load_data_id, contents: bool) -> c_int;
}
extern "C" {
    pub fn security_task_fix_setgroups(new: *mut cred, old: *const cred) -> c_int;
}
extern "C" {
    pub fn security_task_setpgid(p: *mut task_struct, pgid: pid_t) -> c_int;
}
extern "C" {
    pub fn security_task_getpgid(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_task_getsid(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_current_getlsmprop_subj(prop: *mut lsm_prop);
}
extern "C" {
    pub fn security_task_getlsmprop_obj(p: *mut task_struct, prop: *mut lsm_prop);
}
extern "C" {
    pub fn security_task_setnice(p: *mut task_struct, nice: c_int) -> c_int;
}
extern "C" {
    pub fn security_task_setioprio(p: *mut task_struct, ioprio: c_int) -> c_int;
}
extern "C" {
    pub fn security_task_getioprio(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_task_setscheduler(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_task_getscheduler(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_task_movememory(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn security_task_to_inode(p: *mut task_struct, inode: *mut inode);
}
extern "C" {
    pub fn security_create_user_ns(cred: *const cred) -> c_int;
}
extern "C" {
    pub fn security_ipc_permission(ipcp: *mut kern_ipc_perm, flag: c_short) -> c_int;
}
extern "C" {
    pub fn security_ipc_getlsmprop(ipcp: *mut kern_ipc_perm, prop: *mut lsm_prop);
}
extern "C" {
    pub fn security_msg_msg_alloc(msg: *mut msg_msg) -> c_int;
}
extern "C" {
    pub fn security_msg_msg_free(msg: *mut msg_msg);
}
extern "C" {
    pub fn security_msg_queue_alloc(msq: *mut kern_ipc_perm) -> c_int;
}
extern "C" {
    pub fn security_msg_queue_free(msq: *mut kern_ipc_perm);
}
extern "C" {
    pub fn security_msg_queue_associate(msq: *mut kern_ipc_perm, msqflg: c_int) -> c_int;
}
extern "C" {
    pub fn security_msg_queue_msgctl(msq: *mut kern_ipc_perm, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn security_shm_alloc(shp: *mut kern_ipc_perm) -> c_int;
}
extern "C" {
    pub fn security_shm_free(shp: *mut kern_ipc_perm);
}
extern "C" {
    pub fn security_shm_associate(shp: *mut kern_ipc_perm, shmflg: c_int) -> c_int;
}
extern "C" {
    pub fn security_shm_shmctl(shp: *mut kern_ipc_perm, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn security_shm_shmat(shp: *mut kern_ipc_perm, shmaddr: *mut char __user, shmflg: c_int) -> c_int;
}
extern "C" {
    pub fn security_sem_alloc(sma: *mut kern_ipc_perm) -> c_int;
}
extern "C" {
    pub fn security_sem_free(sma: *mut kern_ipc_perm);
}
extern "C" {
    pub fn security_sem_associate(sma: *mut kern_ipc_perm, semflg: c_int) -> c_int;
}
extern "C" {
    pub fn security_sem_semctl(sma: *mut kern_ipc_perm, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn security_d_instantiate(dentry: *mut dentry, inode: *mut inode);
}
extern "C" {
    pub fn security_setprocattr(lsmid: c_int, name: *const c_char, value: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn security_ismaclabel(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn security_secid_to_secctx(secid: u32, cp: *mut lsm_context) -> c_int;
}
extern "C" {
    pub fn security_secctx_to_secid(secdata: *const c_char, seclen: u32, secid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_release_secctx(cp: *mut lsm_context);
}
extern "C" {
    pub fn security_inode_invalidate_secctx(inode: *mut inode);
}
extern "C" {
    pub fn security_inode_notifysecctx(inode: *mut inode, ctx: *mut c_void, ctxlen: u32) -> c_int;
}
extern "C" {
    pub fn security_inode_setsecctx(dentry: *mut dentry, ctx: *mut c_void, ctxlen: u32) -> c_int;
}
extern "C" {
    pub fn security_inode_getsecctx(inode: *mut inode, cp: *mut lsm_context) -> c_int;
}
extern "C" {
    pub fn security_locked_down(what: lockdown_reason) -> c_int;
}
extern "C" {
    pub fn security_bdev_alloc(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn security_bdev_free(bdev: *mut block_device);
}

//
// lsmprop_is_set - report if there is a value in the lsm_prop
// @prop: Pointer to the exported LSM data
//
// Returns true if there is a value set, false otherwise
//
// This is the default capabilities functionality.  Most of these functions
// are just stubbed out, but a few must call the proper capable code.
//
extern "C" {
    pub fn cap_ptrace_access_check(_arg: child, _arg: mode) -> return;
}
extern "C" {
    pub fn cap_ptrace_traceme(_arg: parent) -> return;
}
extern "C" {
    pub fn cap_capget(_arg: target, _arg: effective, _arg: inheritable, _arg: permitted) -> return;
}
extern "C" {
    pub fn cap_capset(_arg: new, _arg: old, _arg: effective, _arg: inheritable, _arg: permitted) -> return;
}
extern "C" {
    pub fn cap_capable(_arg: cred, _arg: ns, _arg: cap, _arg: opts) -> return;
}
extern "C" {
    pub fn cap_settime(_arg: ts, _arg: tz) -> return;
}
extern "C" {
    pub fn __vm_enough_memory(_arg: mm, _arg: pages, _arg: !cap_vm_enough_memory(mm, _arg: pages)) -> return;
}
extern "C" {
    pub fn cap_bprm_creds_from_file(_arg: bprm, _arg: file) -> return;
}
extern "C" {
    pub fn cap_inode_setxattr(_arg: dentry, _arg: name, _arg: value, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn cap_inode_removexattr(_arg: idmap, _arg: dentry, _arg: name) -> return;
}
extern "C" {
    pub fn cap_inode_need_killpriv(_arg: dentry) -> return;
}
extern "C" {
    pub fn cap_inode_killpriv(_arg: idmap, _arg: dentry) -> return;
}
extern "C" {
    pub fn cap_inode_getsecurity(_arg: idmap, _arg: inode, _arg: name, _arg: buffer, _arg: alloc) -> return;
}
extern "C" {
    pub fn cap_mmap_addr(_arg: addr) -> return;
}
// secid = 0;
extern "C" {
    pub fn cap_task_fix_setuid(_arg: new, _arg: old, _arg: flags) -> return;
}
extern "C" {
    pub fn cap_task_setnice(_arg: p, _arg: nice) -> return;
}
extern "C" {
    pub fn cap_task_setioprio(_arg: p, _arg: ioprio) -> return;
}
extern "C" {
    pub fn cap_task_setscheduler(_arg: p) -> return;
}
extern "C" {
    pub fn cap_task_prctl(_arg: option, _arg: arg2, _arg: arg3, _arg: arg4, _arg: arg5) -> return;
}

extern "C" {
    pub fn security_watch_key(key: *mut key) -> c_int;
}

extern "C" {
    pub fn security_netlink_send(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn security_unix_stream_connect(sock: *mut sock, other: *mut sock, newsk: *mut sock) -> c_int;
}
extern "C" {
    pub fn security_unix_may_send(sock: *mut socket, other: *mut socket) -> c_int;
}
extern "C" {
    pub fn security_socket_create(family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_socketpair(socka: *mut socket, sockb: *mut socket) -> c_int;
}
extern "C" {
    pub fn security_socket_bind(sock: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_connect(sock: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_listen(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_accept(sock: *mut socket, newsock: *mut socket) -> c_int;
}
extern "C" {
    pub fn security_socket_sendmsg(sock: *mut socket, msg: *mut msghdr, size: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_getsockname(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn security_socket_getpeername(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn security_socket_getsockopt(sock: *mut socket, level: c_int, optname: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_setsockopt(sock: *mut socket, level: c_int, optname: c_int) -> c_int;
}
extern "C" {
    pub fn security_socket_shutdown(sock: *mut socket, how: c_int) -> c_int;
}
extern "C" {
    pub fn security_sock_rcv_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn security_socket_getpeersec_dgram(sock: *mut socket, skb: *mut sk_buff, secid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_sk_alloc(sk: *mut sock, family: c_int, priority: gfp_t) -> c_int;
}
extern "C" {
    pub fn security_sk_free(sk: *mut sock);
}
extern "C" {
    pub fn security_sk_clone(sk: *const sock, newsk: *mut sock);
}
extern "C" {
    pub fn security_sock_graft(sock*sk: *mut struct, parent: *mut socket);
}
extern "C" {
    pub fn security_secmark_relabel_packet(secid: u32) -> c_int;
}
extern "C" {
    pub fn security_secmark_refcount_inc();
}
extern "C" {
    pub fn security_secmark_refcount_dec();
}
extern "C" {
    pub fn security_tun_dev_alloc_security(security: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_tun_dev_free_security(security: *mut c_void);
}
extern "C" {
    pub fn security_tun_dev_create() -> c_int;
}
extern "C" {
    pub fn security_tun_dev_attach_queue(security: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_tun_dev_attach(sk: *mut sock, security: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_tun_dev_open(security: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_sctp_assoc_request(asoc: *mut sctp_association, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn security_mptcp_add_subflow(sk: *mut sock, ssk: *mut sock) -> c_int;
}

extern "C" {
    pub fn security_unix_find(path: *const path, other: *mut sock, flags: c_int) -> c_int;
}

extern "C" {
    pub fn security_ib_pkey_access(sec: *mut c_void, subnet_prefix: u64, pkey: u16) -> c_int;
}
extern "C" {
    pub fn security_ib_endport_manage_subnet(sec: *mut c_void, name: *const c_char, port_num: u8) -> c_int;
}
extern "C" {
    pub fn security_ib_alloc_security(sec: *mut c_void) -> c_int;
}
extern "C" {
    pub fn security_ib_free_security(sec: *mut c_void);
}

extern "C" {
    pub fn security_xfrm_policy_clone(old_ctx: *mut xfrm_sec_ctx, new_ctxp: *mut xfrm_sec_ctx) -> c_int;
}
extern "C" {
    pub fn security_xfrm_policy_free(ctx: *mut xfrm_sec_ctx);
}
extern "C" {
    pub fn security_xfrm_policy_delete(ctx: *mut xfrm_sec_ctx) -> c_int;
}
extern "C" {
    pub fn security_xfrm_state_alloc(x: *mut xfrm_state, sec_ctx: *mut xfrm_user_sec_ctx) -> c_int;
}
extern "C" {
    pub fn security_xfrm_state_delete(x: *mut xfrm_state) -> c_int;
}
extern "C" {
    pub fn security_xfrm_state_free(x: *mut xfrm_state);
}
extern "C" {
    pub fn security_xfrm_policy_lookup(ctx: *mut xfrm_sec_ctx, fl_secid: u32) -> c_int;
}
extern "C" {
    pub fn security_xfrm_decode_session(skb: *mut sk_buff, secid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_skb_classify_flow(skb: *mut sk_buff, flic: *mut flowi_common);
}

extern "C" {
    pub fn security_path_unlink(dir: *const path, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_path_mkdir(dir: *const path, dentry: *mut dentry, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn security_path_rmdir(dir: *const path, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn security_path_post_mknod(idmap: *mut mnt_idmap, dentry: *mut dentry);
}
extern "C" {
    pub fn security_path_truncate(path: *const path) -> c_int;
}
extern "C" {
    pub fn security_path_chmod(path: *const path, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn security_path_chown(path: *const path, uid: kuid_t, gid: kgid_t) -> c_int;
}
extern "C" {
    pub fn security_path_chroot(path: *const path) -> c_int;
}

extern "C" {
    pub fn security_key_alloc(key: *mut key, cred: *const cred, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn security_key_free(key: *mut key);
}
extern "C" {
    pub fn security_key_getsecurity(key: *mut key, _buffer: *mut c_char) -> c_int;
}

// _buffer = NULL;

extern "C" {
    pub fn security_audit_rule_known(krule: *mut audit_krule) -> c_int;
}
extern "C" {
    pub fn security_audit_rule_free(lsmrule: *mut c_void);
}

extern "C" {
    pub fn securityfs_remove(dentry: *mut dentry);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn security_bpf(cmd: c_int, attr: *mut bpf_attr, size: c_uint, kernel: bool) -> c_int;
}
extern "C" {
    pub fn security_bpf_map(map: *mut bpf_map, fmode: fmode_t) -> c_int;
}
extern "C" {
    pub fn security_bpf_prog(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn security_bpf_map_free(map: *mut bpf_map);
}
extern "C" {
    pub fn security_bpf_prog_free(prog: *mut bpf_prog);
}
extern "C" {
    pub fn security_bpf_token_free(token: *mut bpf_token);
}
extern "C" {
    pub fn security_bpf_token_cmd(token: *const bpf_token, cmd: bpf_cmd) -> c_int;
}
extern "C" {
    pub fn security_bpf_token_capable(token: *const bpf_token, cap: c_int) -> c_int;
}

extern "C" {
    pub fn security_perf_event_open(type: c_int) -> c_int;
}
extern "C" {
    pub fn security_perf_event_alloc(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn security_perf_event_free(event: *mut perf_event);
}
extern "C" {
    pub fn security_perf_event_read(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn security_perf_event_write(event: *mut perf_event) -> c_int;
}

extern "C" {
    pub fn security_uring_override_creds(new: *const cred) -> c_int;
}
extern "C" {
    pub fn security_uring_sqpoll() -> c_int;
}
extern "C" {
    pub fn security_uring_cmd(ioucmd: *mut io_uring_cmd) -> c_int;
}
extern "C" {
    pub fn security_uring_allowed() -> c_int;
}

extern "C" {
    pub fn security_initramfs_populated();
}

