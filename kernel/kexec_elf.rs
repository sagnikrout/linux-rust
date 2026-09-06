//! Automatically rewritten from C to Rust
//! Source: kernel/kexec_elf.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Load ELF vmlinux file for the kexec_file_load syscall.
//
// Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
// Copyright (C) 2004  IBM Corp.
// Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
// Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
// Copyright (C) 2016  IBM Corporation
//
// Based on kexec-tools' kexec-elf-exec.c and kexec-elf-ppc64.c.
// Heavily modified for the kernel by
// Thiago Jung Bauermann <bauerman@linux.vnet.ibm.com>.
//

#[no_mangle]
pub unsafe extern "C" fn elf_is_elf_file(ehdr: *const elfhdr) -> bool {
    return memcmp(ehdr.e_ident, ELFMAG, SELFMAG) == 0;
    }
#[no_mangle]
unsafe extern "C" fn elf64_to_cpu(ehdr: *const elfhdr, value: u64) -> u64 {
    if (ehdr.e_ident[EI_DATA] == ELFDATA2LSB) {
    value = le64_to_cpu(value);
    }

    else if (ehdr.e_ident[EI_DATA] == ELFDATA2MSB) {
    value = be64_to_cpu(value);
    }
    return value;
    }
#[no_mangle]
unsafe extern "C" fn elf32_to_cpu(ehdr: *const elfhdr, value: u32) -> u32 {
    if (ehdr.e_ident[EI_DATA] == ELFDATA2LSB) {
    value = le32_to_cpu(value);
    }

    else if (ehdr.e_ident[EI_DATA] == ELFDATA2MSB) {
    value = be32_to_cpu(value);
    }
    return value;
    }
#[no_mangle]
unsafe extern "C" fn elf16_to_cpu(ehdr: *const elfhdr, value: u16) -> u16 {
    if (ehdr.e_ident[EI_DATA] == ELFDATA2LSB) {
    value = le16_to_cpu(value);
    }

    else if (ehdr.e_ident[EI_DATA] == ELFDATA2MSB) {
    value = be16_to_cpu(value);
    }
    return value;
    }
//
// elf_is_ehdr_sane - check that it is safe to use the ELF header
// @buf_len:	size of the buffer in which the ELF file is loaded.
//
#[no_mangle]
unsafe extern "C" fn elf_is_ehdr_sane(ehdr: *const elfhdr, buf_len: usize) -> bool {
    if (ehdr.e_phnum > 0 && ehdr.e_phentsize != sizeof!(elf_phdr)) {
    pr_debug!("Bad program header size.\n");
    return false;
    } else if (ehdr.e_shnum > 0 &&
    ehdr.e_shentsize != sizeof!(elf_shdr)) {
    pr_debug!("Bad section header size.\n");
    return false;
    } else if (ehdr.e_ident[EI_VERSION] != EV_CURRENT ||
    ehdr.e_version != EV_CURRENT) {
    pr_debug!("Unknown ELF version.\n");
    return false;
    }
    if (ehdr.e_phoff > 0 && ehdr.e_phnum > 0) {
    let mut phdr_size = 0;
//
// e_phnum is at most 65535 so calculating the size of the
// program header cannot overflow.
//
    phdr_size = sizeof!(elf_phdr) * ehdr.e_phnum;
// Sanity check the program header table location.
    if (ehdr.e_phoff + phdr_size < ehdr.e_phoff) {
    pr_debug!("Program headers at invalid location.\n");
    return false;
    } else if (ehdr.e_phoff + phdr_size > buf_len) {
    pr_debug!("Program headers truncated.\n");
    return false;
    }
    }
    if (ehdr.e_shoff > 0 && ehdr.e_shnum > 0) {
    let mut shdr_size = 0;
//
// e_shnum is at most 65536 so calculating
// the size of the section header cannot overflow.
//
    shdr_size = sizeof!(elf_shdr) * ehdr.e_shnum;
// Sanity check the section header table location.
    if (ehdr.e_shoff + shdr_size < ehdr.e_shoff) {
    pr_debug!("Section headers at invalid location.\n");
    return false;
    } else if (ehdr.e_shoff + shdr_size > buf_len) {
    pr_debug!("Section headers truncated.\n");
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn elf_read_ehdr(buf: *const c_char, len: usize, ehdr: *mut elfhdr) -> c_int {
    let mut buf_ehdr = core::ptr::null_mut();
    if (len < sizeof!(*buf_ehdr)) {
    pr_debug!("Buffer is too small to hold ELF header.\n");
    return -ENOEXEC;
    }
    memset(ehdr, 0, sizeof!(*ehdr));
    memcpy(ehdr.e_ident, buf, sizeof!(ehdr.e_ident));
    if (!elf_is_elf_file(ehdr)) {
    pr_debug!("No ELF header magic.\n");
    return -ENOEXEC;
    }
    if (ehdr.e_ident[EI_CLASS] != ELF_CLASS) {
    pr_debug!("Not a supported ELF class.\n");
    return -ENOEXEC;
    } else  if (ehdr.e_ident[EI_DATA] != ELFDATA2LSB &&
    ehdr.e_ident[EI_DATA] != ELFDATA2MSB) {
    pr_debug!("Not a supported ELF data format.\n");
    return -ENOEXEC;
    }
    buf_ehdr =  buf;
    if (elf16_to_cpu(ehdr, buf_ehdr.e_ehsize) != sizeof!(*buf_ehdr)) {
    pr_debug!("Bad ELF header size.\n");
    return -ENOEXEC;
    }
    ehdr.e_type      = elf16_to_cpu(ehdr, buf_ehdr.e_type);
    ehdr.e_machine   = elf16_to_cpu(ehdr, buf_ehdr.e_machine);
    ehdr.e_version   = elf32_to_cpu(ehdr, buf_ehdr.e_version);
    ehdr.e_flags     = elf32_to_cpu(ehdr, buf_ehdr.e_flags);
    ehdr.e_phentsize = elf16_to_cpu(ehdr, buf_ehdr.e_phentsize);
    ehdr.e_phnum     = elf16_to_cpu(ehdr, buf_ehdr.e_phnum);
    ehdr.e_shentsize = elf16_to_cpu(ehdr, buf_ehdr.e_shentsize);
    ehdr.e_shnum     = elf16_to_cpu(ehdr, buf_ehdr.e_shnum);
    ehdr.e_shstrndx  = elf16_to_cpu(ehdr, buf_ehdr.e_shstrndx);
    match (ehdr.e_ident[EI_CLASS]) {
    ELFCLASS64 => {
    ehdr.e_entry = elf64_to_cpu(ehdr, buf_ehdr.e_entry);
    ehdr.e_phoff = elf64_to_cpu(ehdr, buf_ehdr.e_phoff);
    ehdr.e_shoff = elf64_to_cpu(ehdr, buf_ehdr.e_shoff);
    // break;
    }
    ELFCLASS32 => {
    ehdr.e_entry = elf32_to_cpu(ehdr, buf_ehdr.e_entry);
    ehdr.e_phoff = elf32_to_cpu(ehdr, buf_ehdr.e_phoff);
    ehdr.e_shoff = elf32_to_cpu(ehdr, buf_ehdr.e_shoff);
    // break;
    }
    _ => {
    pr_debug!("Unknown ELF class.\n");
    return -EINVAL;
    }
    }
    return elf_is_ehdr_sane(ehdr, len) ? 0 : -ENOEXEC;
    }
//
// elf_is_phdr_sane - check that it is safe to use the program header
// @buf_len:	size of the buffer in which the ELF file is loaded.
//
#[no_mangle]
unsafe extern "C" fn elf_is_phdr_sane(phdr: *const elf_phdr, buf_len: usize) -> bool {
    if (phdr.p_offset + phdr.p_filesz < phdr.p_offset) {
    pr_debug!("ELF segment location wraps around.\n");
    return false;
    } else if (phdr.p_offset + phdr.p_filesz > buf_len) {
    pr_debug!("ELF segment not in file.\n");
    return false;
    } else if (phdr.p_paddr + phdr.p_memsz < phdr.p_paddr) {
    pr_debug!("ELF segment address wraps around.\n");
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn elf_read_phdr() {
// Override the const in proghdrs, we are the ones doing the loading.
    let mut phdr = &elf_info.proghdrs[idx];
    let mut ehdr = elf_info.ehdr;
    let mut pbuf = core::ptr::null_mut();
    let mut buf_phdr = core::ptr::null_mut();
    pbuf = buf + elf_info.ehdr.e_phoff + (idx * sizeof!(*buf_phdr));
    buf_phdr =  pbuf;
    phdr.p_type   = elf32_to_cpu(elf_info.ehdr, buf_phdr.p_type);
    phdr.p_flags  = elf32_to_cpu(elf_info.ehdr, buf_phdr.p_flags);
    match (ehdr.e_ident[EI_CLASS]) {
    ELFCLASS64 => {
    phdr.p_offset = elf64_to_cpu(ehdr, buf_phdr.p_offset);
    phdr.p_paddr  = elf64_to_cpu(ehdr, buf_phdr.p_paddr);
    phdr.p_vaddr  = elf64_to_cpu(ehdr, buf_phdr.p_vaddr);
    phdr.p_filesz = elf64_to_cpu(ehdr, buf_phdr.p_filesz);
    phdr.p_memsz  = elf64_to_cpu(ehdr, buf_phdr.p_memsz);
    phdr.p_align  = elf64_to_cpu(ehdr, buf_phdr.p_align);
    // break;
    }
    ELFCLASS32 => {
    phdr.p_offset = elf32_to_cpu(ehdr, buf_phdr.p_offset);
    phdr.p_paddr  = elf32_to_cpu(ehdr, buf_phdr.p_paddr);
    phdr.p_vaddr  = elf32_to_cpu(ehdr, buf_phdr.p_vaddr);
    phdr.p_filesz = elf32_to_cpu(ehdr, buf_phdr.p_filesz);
    phdr.p_memsz  = elf32_to_cpu(ehdr, buf_phdr.p_memsz);
    phdr.p_align  = elf32_to_cpu(ehdr, buf_phdr.p_align);
    // break;
    }
    _ => {
    pr_debug!("Unknown ELF class.\n");
    return -EINVAL;
    }
    }
    return elf_is_phdr_sane(phdr, len) ? 0 : -ENOEXEC;
    }
//
// elf_read_phdrs - read the program headers from the buffer
//
// This function assumes that the program header table was checked for sanity.
// Use elf_is_ehdr_sane() if it wasn't.
//
#[no_mangle]
pub unsafe extern "C" fn elf_read_phdrs() {
    size_t phdr_size, i;
    let mut ehdr = elf_info.ehdr;
//
// e_phnum is at most 65535 so calculating the size of the
// program header cannot overflow.
//
    phdr_size = sizeof!(elf_phdr) * ehdr.e_phnum;
    elf_info.proghdrs = kzalloc(phdr_size, GFP_KERNEL);
    if (!elf_info.proghdrs) {
    return -ENOMEM;
    }
    while (i < ehdr.e_phnum) {
    let mut ret = 0;
    ret = elf_read_phdr(buf, len, elf_info, i);
    if (ret) {
    kfree(elf_info.proghdrs);
    elf_info.proghdrs = core::ptr::null_mut();
    return ret;
    }
    }
    return 0;
    }
//
// elf_read_from_buffer - read ELF file and sets up ELF header and ELF info
// @buf:	Buffer to read ELF file from.
// @len:	Size of @buf.
// @ehdr:	Pointer to existing struct which will be populated.
// @elf_info:	Pointer to existing struct which will be populated.
//
// This function allows reading ELF files with different byte order than
// the kernel, byte-swapping the fields as needed.
//
// Return:
// On success returns 0, and the caller should call
// kexec_free_elf_info(elf_info) to free the memory allocated for the section
// and program headers.
//
#[no_mangle]
pub unsafe extern "C" fn elf_read_from_buffer() {
    let mut ret = 0;
    ret = elf_read_ehdr(buf, len, ehdr);
    if (ret) {
    return ret;
    }
    elf_info.buffer = buf;
    elf_info.ehdr = ehdr;
    if (ehdr.e_phoff > 0 && ehdr.e_phnum > 0) {
    ret = elf_read_phdrs(buf, len, elf_info);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
//
// kexec_free_elf_info - free memory allocated by elf_read_from_buffer
//
#[no_mangle]
pub unsafe extern "C" fn kexec_free_elf_info(elf_info: *mut kexec_elf_info) {
    kfree(elf_info.proghdrs);
    memset(elf_info, 0, sizeof!(*elf_info));
    }
//
// kexec_build_elf_info - read ELF executable and check that we can use it
//
#[no_mangle]
pub unsafe extern "C" fn kexec_build_elf_info() {
    let mut i = 0;
    let mut ret = 0;
    ret = elf_read_from_buffer(buf, len, ehdr, elf_info);
    if (ret) {
    return ret;
    }
// Big endian vmlinux has type ET_DYN.
    if (ehdr.e_type != ET_EXEC && ehdr.e_type != ET_DYN) {
    pr_err!("Not an ELF executable.\n");
// goto;
    } else if (!elf_info.proghdrs) {
    pr_err!("No ELF program header.\n");
// goto;
    }
    while (i < ehdr.e_phnum) {
//
// Kexec does not support loading interpreters.
// In addition this check keeps us from attempting
// to kexec ordinay executables.
//
    if (elf_info.proghdrs[i].p_type == PT_INTERP) {
    pr_err!("Requires an ELF interpreter.\n");
// goto;
    }
    }
    return 0;
// label;
    kexec_free_elf_info(elf_info);
    return -ENOEXEC;
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_elf_probe(buf: *const c_char, len: c_ulong) -> c_int {
    let mut ehdr;
    let mut elf_info;
    let mut ret = 0;
    ret = kexec_build_elf_info(buf, len, &ehdr, &elf_info);
    if (ret) {
    return ret;
    }
    kexec_free_elf_info(&elf_info);
    return elf_check_arch(&ehdr) ? 0 : -ENOEXEC;
    }
//
// kexec_elf_load - load ELF executable image
// @lowest_load_addr:	On return, will be the address where the first PT_LOAD
// section will be loaded in memory.
//
// Return:
// 0 on success, negative value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_elf_load() {
pub static mut lowest_addr: c_ulong = 0;
    let mut ret = 0;
    let mut i = 0;
// Read in the PT_LOAD segments.
    while (i < ehdr.e_phnum) {
    let mut load_addr = 0;
    let mut size = 0;
    let mut phdr = core::ptr::null_mut();
    phdr = &elf_info.proghdrs[i];
    if (phdr.p_type != PT_LOAD) {
    continue;
    }
    size = phdr.p_filesz;
    if (size > phdr.p_memsz) {
    size = phdr.p_memsz;
    }
    kbuf.buffer =  elf_info.buffer + phdr.p_offset;
    kbuf.bufsz = size;
    kbuf.memsz = phdr.p_memsz;
    kbuf.buf_align = phdr.p_align;
    kbuf.buf_min = phdr.p_paddr;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_add_buffer(kbuf);
    if (ret) {
// goto;
    }
    load_addr = kbuf.mem;
    if (load_addr < lowest_addr) {
    lowest_addr = load_addr;
    }
    }
// lowest_load_addr = lowest_addr;
    ret = 0;
// label;
    return ret;
    }