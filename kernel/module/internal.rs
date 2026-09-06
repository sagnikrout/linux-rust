//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/module/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Module internals
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
// Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
//

pub const ARCH_SHF_SMALL: c_int = 0;

//
// Use highest 4 bits of sh_entsize to store the mod_mem_type of this
// section. This leaves 28 bits for offset on 32-bit systems, which is
// about 256 MiB (WARN_ON_ONCE if we exceed that).
//
pub const SH_ENTSIZE_TYPE_BITS: c_int = 4;

// Maximum number of characters written by module_flags!()

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_symbol {

    pub value_offset: c_int,
    pub name_offset: c_int,
    pub namespace_offset: c_int,

    pub value: c_ulong,
    pub name: *const c_char,
    pub namespace: *const c_char,

}

// Provided by the linker
pub const KMOD_PATH_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct load_info {
    pub name: *const c_char,
pointer to module in temporary copy, freed at end of load_module()
    pub mod: *mut module,
    pub hdr: *mut Elf_Ehdr,
    pub len: c_ulong,
    pub sechdrs: *mut Elf_Shdr,
    pub strtab: *mut *mut char secstrings,,
    pub core_typeoffs: unsigned long symoffs, stroffs, init_typeoffs,,
    pub sig_ok: bool,

    pub mod_kallsyms_init_off: c_ulong,

    pub compressed_len: c_ulong,

    pub pages: *mut page,
    pub max_pages: c_uint,
    pub used_pages: c_uint,

    pub sym: c_uint,
    pub str: c_uint,
    pub mod: c_uint,
    pub vers: c_uint,
    pub info: c_uint,
    pub pcpu: c_uint,
    pub vers_ext_crc: c_uint,
    pub vers_ext_name: c_uint,
    pub index: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_license {
    NOT_GPL_ONLY,
    GPL_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct find_symbol_arg {
// Input
    pub name: *const c_char,
    pub gplok: bool,
    pub warn: bool,
// Output
    pub owner: *mut module,
    pub crc: *const u32,
    pub sym: *const kernel_symbol,
    pub license: mod_license,
}

// modules using other modules
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_use {
    pub source_list: list_head,
    pub target_list: list_head,
    pub target: *mut *mut module source,,
}

extern "C" {
    pub fn mod_verify_sig(mod: *const c_void, info: *mut load_info) -> c_int;
}
extern "C" {
    pub fn try_to_force_load(mod: *mut module, reason: *const c_char) -> c_int;
}
extern "C" {
    pub fn find_symbol(fsa: *mut find_symbol_arg) -> bool;
}
extern "C" {
    pub fn cmp_name(name: *const c_void, sym: *const c_void) -> c_int;
}
extern "C" {
    pub fn module_flags_taint!(taints: c_ulong, buf: *mut c_char) -> usize;
}

extern "C" {
    pub fn copy_module_elf(mod: *mut module, info: *mut load_info) -> c_int;
}
extern "C" {
    pub fn free_module_elf(mod: *mut module);
}

//
// enum fail_dup_mod_reason - state at which a duplicate module was detected
//
// @FAIL_DUP_MOD_BECOMING: the module is read properly, passes all checks but
// we've determined that another module with the same name is already loaded
// or being processed on our &modules list. This happens on early_mod_check()
// right before layout_and_allocate(). The kernel would have already
// vmalloc()'d space for the entire module through finit_module(). If
// decompression was used two vmap() spaces were used. These failures can
// happen when userspace has not seen the module present on the kernel and
// tries to load the module multiple times at same time.
// @FAIL_DUP_MOD_LOAD: the module has been read properly, passes all validation
// checks and the kernel determines that the module was unique and because
// of this allocated yet another private kernel copy of the module space in
// layout_and_allocate() but after this determined in add_unformed_module()
// that another module with the same name is already loaded or being processed.
// These failures should be mitigated as much as possible and are indicative
// of really fast races in loading modules. Without module decompression
// they waste twice as much vmap space. With module decompression three
// times the module's size vmap space is wasted.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fail_dup_mod_reason {
    FAIL_DUP_MOD_BECOMING = 0,
    FAIL_DUP_MOD_LOAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_fail_load {
    pub list: list_head,
    pub name: [c_char; MODULE_NAME_LEN],
    pub count: atomic_long_t,
    pub dup_fail_mask: c_ulong,
}

extern "C" {
    pub fn try_add_failed_module(name: *const c_char, reason: fail_dup_mod_reason) -> c_int;
}
extern "C" {
    pub fn mod_stat_bump_invalid(info: *mut load_info, flags: c_int);
}
extern "C" {
    pub fn mod_stat_bump_becoming(info: *mut load_info, flags: c_int);
}

// Macro flag: #define mod_stat_inc(name)

extern "C" {
    pub fn kmod_dup_request_exists_wait(module_name: *mut c_char, wait: bool, dup_ret: *mut c_int) -> bool;
}
extern "C" {
    pub fn kmod_dup_request_announce(module_name: *mut c_char, ret: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_unload_taint {
    pub list: list_head,
    pub name: [c_char; MODULE_NAME_LEN],
    pub taints: c_ulong,
    pub count: u64,
}

extern "C" {
    pub fn try_add_tainted_module(mod: *mut module) -> c_int;
}
extern "C" {
    pub fn print_unloaded_tainted_modules();
}

extern "C" {
    pub fn module_decompress!(info: *mut load_info, buf: *const c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn module_decompress_cleanup!(info: *mut load_info);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_tree_root {

    pub root: latch_tree_root,

    pub addr_min: c_ulong,
    pub addr_max: c_ulong,

    pub data_addr_min: c_ulong,
    pub data_addr_max: c_ulong,

}

extern "C" {
    pub fn mod_tree_insert(mod: *mut module);
}
extern "C" {
    pub fn mod_tree_remove_init(mod: *mut module);
}
extern "C" {
    pub fn mod_tree_remove(mod: *mut module);
}

extern "C" {
    pub fn module_enable_rodata_ro!(mod: *const module) -> c_int;
}
extern "C" {
    pub fn module_enable_rodata_ro_after_init!(mod: *const module) -> c_int;
}
extern "C" {
    pub fn module_enable_data_nx!(mod: *const module) -> c_int;
}
extern "C" {
    pub fn module_enable_text_rox!(mod: *const module) -> c_int;
}

extern "C" {
    pub fn module_sig_check!(info: *mut load_info, flags: c_int) -> c_int;
}

extern "C" {
    pub fn kmemleak_load_module(mod: *const module, info: *const load_info);
}

extern "C" {
    pub fn init_build_id(mod: *mut module, info: *const load_info);
}
extern "C" {
    pub fn layout_symtab(mod: *mut module, info: *mut load_info);
}
extern "C" {
    pub fn add_kallsyms(mod: *mut module, info: *const load_info);
}

extern "C" {
    pub fn mod_sysfs_teardown(mod: *mut module);
}
extern "C" {
    pub fn init_param_lock(mod: *mut module);
}

extern "C" {
    pub fn check_modstruct_version(info: *const load_info, mod: *mut module) -> c_int;
}
extern "C" {
    pub fn same_magic(amagic: *const c_char, bmagic: *const c_char, has_crcs: bool) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modversion_info_ext {
    pub remaining: usize,
    pub crc: *const u32,
    pub name: *const c_char,
}

extern "C" {
    pub fn modversion_ext_start(info: *const load_info, ver: *mut modversion_info_ext);
}
extern "C" {
    pub fn modversion_ext_advance(ver: *mut modversion_info_ext);