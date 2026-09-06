//! Automatically rewritten from C to Rust
//! Source: kernel/kallsyms.c
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
// kallsyms.c: in-kernel printing of symbolic oopses and stack traces.
//
// Rewritten and vastly simplified by Rusty Russell for in-kernel
// module loader:
// Copyright 2002 Rusty Russell <rusty@rustcorp.com.au> IBM Corporation
//
// ChangeLog:
//
// (25/Aug/2004) Paulo Marques <pmarques@grupopie.com>
// Changed the compression method from stem compression to "table lookup"
// compression (see scripts/kallsyms.c for a more complete description)
//

//
// Expand a compressed symbol data into the resulting uncompressed string,
// if uncompressed string is too long (>= maxlen), it will be truncated,
// given the offset to where the symbol is in the compressed stream.
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_expand_symbol() {
    int len, skipped_first = 0;
    let mut tptr = core::ptr::null_mut();
    let mut data = core::ptr::null_mut();
// Get the compressed symbol length from the first symbol byte.
    data = &kallsyms_names[off];
    len = *data;
    data += 1;
    off += 1;
// If MSB is 1, it is a "big" symbol, so needs an additional byte.
    if ((len & 0x80) != 0) {
    len = (len & 0x7F) | (*data << 7);
    data += 1;
    off += 1;
    }
//
// Update the offset to return the offset for the next symbol on
// the compressed stream.
//
    off += len;
//
// For every byte on the compressed symbol data, copy the table
// entry for that byte.
//
    while (len) {
    tptr = &kallsyms_token_table[kallsyms_token_index[*data]];
    data += 1;
    len -= 1;
    while (*tptr) {
    if (skipped_first) {
    if (maxlen <= 1) {
// goto;
    }
// result = *tptr;
    result += 1;
    maxlen -= 1;
    } else {
    skipped_first = 1;
    }
    tptr += 1;
    }
    }
// label;
    if (maxlen) {
// result = '\0';
    }
// Return to offset to the next symbol.
    return off;
    }
//
// Get symbol type information. This is encoded as a single char at the
// beginning of the symbol name.
//
#[no_mangle]
unsafe extern "C" fn kallsyms_get_symbol_type(off: c_uint) -> c_char {
//
// Get just the first code, look it up in the token table,
// and return the first char from this token. If MSB of length
// is 1, it is a "big" symbol, so needs an additional byte.
//
    if (kallsyms_names[off] & 0x80) {
    off += 1;
    }
    return kallsyms_token_table[kallsyms_token_index[kallsyms_names[off + 1]]];
    }
//
// Find the offset on the compressed stream given and index in the
// kallsyms array.
//
#[no_mangle]
unsafe extern "C" fn get_symbol_offset(pos: c_ulong) -> c_uint {
    let mut name = core::ptr::null_mut();
    let mut i = 0;
    let mut len = 0;
//
// Use the closest marker we have. We have markers every 256 positions,
// so that should be close enough.
//
    name = &kallsyms_names[kallsyms_markers[pos >> 8]];
//
// Sequentially scan all the symbols up to the point we're searching
// for. Every symbol is stored in a [<len>][<len> bytes of data] format,
// so we just need to add the len to the current pointer for every
// symbol we wish to skip.
//
    while (i < (pos & 0xFF)) {
    len = *name;
//
// If MSB is 1, it is a "big" symbol, so we need to look into
// the next byte (and skip it, too).
//
    if ((len & 0x80) != 0) {
    len = ((len & 0x7F) | (name[1] << 7)) + 1;
    }
    name = name + len + 1;
    }
    return name - kallsyms_names;
    }
#[no_mangle]
pub unsafe extern "C" fn kallsyms_sym_address(idx: c_int) -> c_ulong {
// non-relocatable 32-bit kernels just embed the value directly
    if (!IS_ENABLED!(CONFIG_64BIT) && !IS_ENABLED!(CONFIG_RELOCATABLE)) {
    return (u32)kallsyms_offsets[idx];
    }
    return (unsigned long)offset_to_ptr(kallsyms_offsets + idx);
    }
#[no_mangle]
unsafe extern "C" fn get_symbol_seq(index: c_int) -> c_uint {
    unsigned int i, seq = 0;
    for (i = 0; i < 3; i++) {
    seq = (seq << 8) | kallsyms_seqs_of_names[3 * index + i];
    }
    return seq;
    }
#[no_mangle]
pub unsafe extern "C" fn kallsyms_lookup_names() {
    let mut ret = 0;
    let mut low = 0;
    let mut mid = 0;
    let mut high = 0;
    let mut seq = 0;
    let mut off = 0;
    char namebuf[KSYM_NAME_LEN];
    low = 0;
    high = kallsyms_num_syms - 1;
    while (low <= high) {
    mid = low + (high - low) / 2;
    seq = get_symbol_seq(mid);
    off = get_symbol_offset(seq);
    kallsyms_expand_symbol(off, namebuf, ARRAY_SIZE!(namebuf));
    ret = strcmp(name, namebuf);
    if (ret > 0) {
    low = mid + 1;
    }

    else if (ret < 0) {
    high = mid - 1;
    }
    else {
    break;
    }
    }
    if (low > high) {
    return -ESRCH;
    }
    low = mid;
    while (low) {
    seq = get_symbol_seq(low - 1);
    off = get_symbol_offset(seq);
    kallsyms_expand_symbol(off, namebuf, ARRAY_SIZE!(namebuf));
    if (strcmp(name, namebuf)) {
    break;
    }
    low -= 1;
    }
// start = low;
    if (end) {
    high = mid;
    while (high < kallsyms_num_syms - 1) {
    seq = get_symbol_seq(high + 1);
    off = get_symbol_offset(seq);
    kallsyms_expand_symbol(off, namebuf, ARRAY_SIZE!(namebuf));
    if (strcmp(name, namebuf)) {
    break;
    }
    high += 1;
    }
// end = high;
    }
    return 0;
    }
// Lookup the address for this symbol. Returns 0 if not found.
#[no_mangle]
pub unsafe extern "C" fn kallsyms_lookup_name(name: *const c_char) -> c_ulong {
    let mut ret = 0;
    let mut i = 0;
// Skip the search for empty string.
    if (!*name) {
    return 0;
    }
    ret = kallsyms_lookup_names(name, &i, core::ptr::null_mut());
    if (!ret) {
    return kallsyms_sym_address(get_symbol_seq(i));
    }
    return module_kallsyms_lookup_name!(name);
    }
//
// Iterate over all symbols in vmlinux.  For symbols from modules use
// module_kallsyms_on_each_symbol instead.
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_on_each_symbol() {
    char namebuf[KSYM_NAME_LEN];
    let mut i = 0;
    let mut off = 0;
    let mut ret = 0;
    while (i < kallsyms_num_syms) {
    off = kallsyms_expand_symbol(off, namebuf, ARRAY_SIZE!(namebuf));
    ret = fn(data, namebuf, kallsyms_sym_address(i));
    if (ret != 0) {
    return ret;
    }
    cond_resched();
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kallsyms_on_each_match_symbol() {
    let mut ret = 0;
    let mut i = 0;
    let mut start = 0;
    let mut end = 0;
    ret = kallsyms_lookup_names(name, &start, &end);
    if (ret) {
    return 0;
    }
    while (!ret && i <= end) {
    ret = fn(data, kallsyms_sym_address(get_symbol_seq(i)));
    cond_resched();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn get_symbol_pos() {
pub static mut symbol_start: c_ulong = 0;
    unsigned long i, low, high, mid;
// Do a binary search on the sorted kallsyms_offsets array.
    low = 0;
    high = kallsyms_num_syms;
    while (high - low > 1) {
    mid = low + (high - low) / 2;
    if (kallsyms_sym_address(mid) <= addr) {
    low = mid;
    }
    else {
    high = mid;
    }
    }
//
// Search for the first aliased symbol. Aliased
// symbols are symbols with the same address.
//
    while (low && kallsyms_sym_address(low-1) == kallsyms_sym_address(low)) {
    low -= 1;
    }
    symbol_start = kallsyms_sym_address(low);
// Search for next non-aliased symbol.
    while (i < kallsyms_num_syms) {
    if (kallsyms_sym_address(i) > symbol_start) {
    symbol_end = kallsyms_sym_address(i);
    break;
    }
    }
// If we found no next symbol, we use the end of the section.
    if (!symbol_end) {
    if (is_kernel_inittext(addr)) {
    symbol_end = (unsigned long)_einittext;
    }

    else if (IS_ENABLED!(CONFIG_KALLSYMS_ALL)) {
    symbol_end = (unsigned long)_end;
    }
    else {
    symbol_end = (unsigned long)_etext;
    }
    }
    if (symbolsize) {
// symbolsize = symbol_end - symbol_start;
    }
    if (offset) {
// offset = addr - symbol_start;
    }
    return low;
    }
//
// Lookup an address but don't bother to find any names.
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_lookup_size_offset() {
    char namebuf[KSYM_NAME_LEN];
    if (is_ksym_addr(addr)) {
    get_symbol_pos(addr, symbolsize, offset);
    return 1;
    }
    return !!module_address_lookup!(addr, symbolsize, offset, core::ptr::null_mut(), core::ptr::null_mut(), namebuf) ||
    !!bpf_address_lookup(addr, symbolsize, offset, namebuf);
    }
#[no_mangle]
pub unsafe extern "C" fn kallsyms_lookup_buildid() {
    let mut ret = 0;
//
// kallsyms_lookus() returns pointer to namebuf on success and
// NULL on error. But some callers ignore the return value.
// Instead they expect @namebuf filled either with valid
// or empty string.
//
    namebuf[0] = 0;
//
// Initialize the module-related return values. They are not set
// when the symbol is in vmlinux or it is a bpf address.
//
    if (modname) {
// modname = NULL;
    }
    if (modbuildid) {
// modbuildid = NULL;
    }
    if (is_ksym_addr(addr)) {
    let mut pos = 0;
    pos = get_symbol_pos(addr, symbolsize, offset);
// Grab name
    kallsyms_expand_symbol(get_symbol_offset(pos),
    namebuf, KSYM_NAME_LEN);
    return strlen(namebuf);
    }
// See if it's in a module or a BPF JITed image.
    ret = module_address_lookup!(addr, symbolsize, offset,
    modname, modbuildid, namebuf);
    if (!ret) {
    ret = bpf_address_lookup(addr, symbolsize, offset, namebuf);
    }
    if (!ret) {
    ret = ftrace_mod_address_lookup(addr, symbolsize, offset,
    modname, modbuildid, namebuf);
    }
    return ret;
    }
//
// Lookup an address
// - modname is set to NULL if it's in the kernel.
// - We guarantee that the returned name is valid until we reschedule even if.
// It resides in a module.
// - We also guarantee that modname will be valid until rescheduled.
//
    const char *kallsyms_lookup(unsigned long addr,
    unsigned long *symbolsize,
    unsigned long *offset,
    char **modname, char *namebuf)
    {
    let mut ret = kallsyms_lookup_buildid(addr, symbolsize, offset, modname,
    core::ptr::null_mut(), namebuf);
    if (!ret) {
    return core::ptr::null_mut();
    }
    return namebuf;
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_symbol_name(addr: c_ulong, symname: *mut c_char) -> c_int {
    symname[0] = '\0';
    symname[KSYM_NAME_LEN - 1] = '\0';
    if (is_ksym_addr(addr)) {
    let mut pos = 0;
    pos = get_symbol_pos(addr, core::ptr::null_mut(), core::ptr::null_mut());
// Grab name
    kallsyms_expand_symbol(get_symbol_offset(pos),
    symname, KSYM_NAME_LEN);
    return 0;
    }
// See if it's in a module.
    return lookup_module_symbol_name(addr, symname);
    }

#[no_mangle]
pub unsafe extern "C" fn append_buildid() {
    if (!modname) {
    return 0;
    }
    if (!buildid) {
    pr_warn_once("Undefined buildid for the module %s\n", modname);
    return 0;
    }
// build ID should match length of sprintf

    static_assert(sizeof!(typeof_member(module, build_id)) == 20);

    return sprintf(buffer, " %20phN", buildid);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: append_buildid
pub unsafe extern "C" fn append_buildid_dup() {
    return 0;
    }

// Look up a kernel symbol and return it in a text buffer.
#[no_mangle]
pub unsafe extern "C" fn __sprint_symbol() {
    let mut modname = core::ptr::null_mut();
pub static mut buildid: *mut c_void = core::ptr::null_mut();
    unsigned long offset, size;
    let mut len = 0;
// Prevent module removal until modname and modbuildid are printed
    guard(rcu)();
    address += symbol_offset;
    len = kallsyms_lookup_buildid(address, &size, &offset, &modname, &buildid,
    buffer);
    if (!len) {
    return sprintf(buffer, "0x%lx", address - symbol_offset);
    }
    offset -= symbol_offset;
    if (add_offset) {
    len += sprintf(buffer + len, "+%#lx/%#lx", offset, size);
    }
    if (modname) {
    len += sprintf(buffer + len, " [%s", modname);
    if (add_buildid) {
    len += append_buildid(buffer + len, modname, buildid);
    }
    len += sprintf(buffer + len, "]");
    }
    return len;
    }
//
// sprint_symbol - Look up a kernel symbol and return it in a text buffer
// @buffer: buffer to be stored
// @address: address to lookup
//
// This function looks up a kernel symbol with @address and stores its name,
// offset, size and module name to @buffer if possible. If no symbol was found,
// just saves its @address as is.
//
// This function returns the number of bytes stored in @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_symbol(buffer: *mut c_char, address: c_ulong) -> c_int {
    return __sprint_symbol(buffer, address, 0, 1, 0);
    }
// EXPORT_SYMBOL_GPL;
//
// sprint_symbol_build_id - Look up a kernel symbol and return it in a text buffer
// @buffer: buffer to be stored
// @address: address to lookup
//
// This function looks up a kernel symbol with @address and stores its name,
// offset, size, module name and module build ID to @buffer if possible. If no
// symbol was found, just saves its @address as is.
//
// This function returns the number of bytes stored in @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_symbol_build_id(buffer: *mut c_char, address: c_ulong) -> c_int {
    return __sprint_symbol(buffer, address, 0, 1, 1);
    }
// EXPORT_SYMBOL_GPL;
//
// sprint_symbol_no_offset - Look up a kernel symbol and return it in a text buffer
// @buffer: buffer to be stored
// @address: address to lookup
//
// This function looks up a kernel symbol with @address and stores its name
// and module name to @buffer if possible. If no symbol was found, just saves
// its @address as is.
//
// This function returns the number of bytes stored in @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_symbol_no_offset(buffer: *mut c_char, address: c_ulong) -> c_int {
    return __sprint_symbol(buffer, address, 0, 0, 0);
    }
// EXPORT_SYMBOL_GPL;
//
// sprint_backtrace - Look up a backtrace symbol and return it in a text buffer
// @buffer: buffer to be stored
// @address: address to lookup
//
// This function is for stack backtrace and does the same thing as
// sprint_symbol() but with modified/decreased @address. If there is a
// tail-call to the function marked "noreturn", gcc optimized out code after
// the call so that the stack-saved return address could point outside of the
// caller. This function ensures that kallsyms will find the original caller
// by decreasing @address.
//
// This function returns the number of bytes stored in @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_backtrace(buffer: *mut c_char, address: c_ulong) -> c_int {
    return __sprint_symbol(buffer, address, -1, 1, 0);
    }
//
// sprint_backtrace_build_id - Look up a backtrace symbol and return it in a text buffer
// @buffer: buffer to be stored
// @address: address to lookup
//
// This function is for stack backtrace and does the same thing as
// sprint_symbol() but with modified/decreased @address. If there is a
// tail-call to the function marked "noreturn", gcc optimized out code after
// the call so that the stack-saved return address could point outside of the
// caller. This function ensures that kallsyms will find the original caller
// by decreasing @address. This function also appends the module build ID to
// the @buffer if @address is within a kernel module.
//
// This function returns the number of bytes stored in @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_backtrace_build_id(buffer: *mut c_char, address: c_ulong) -> c_int {
    return __sprint_symbol(buffer, address, -1, 1, 1);
    }
// To avoid using get_symbol_offset for every symbol, we carry prefix along.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kallsym_iter {
    pub pos: loff_t,
    pub pos_mod_end: loff_t,
    pub pos_ftrace_mod_end: loff_t,
    pub pos_bpf_end: loff_t,
    pub value: c_ulong,
//     pub /: *mut *mut unsigned int nameoff; / If iterating in core kernel symbols.,
    pub type: c_char,
    pub name: [c_char; KSYM_NAME_LEN],
    pub module_name: [c_char; MODULE_NAME_LEN],
    pub exported: c_int,
    pub show_value: c_int,
}

#[no_mangle]
unsafe extern "C" fn get_ksymbol_mod(iter: *mut kallsym_iter) -> c_int {
    let mut ret = module_get_kallsym!(iter.pos - kallsyms_num_syms,
    &iter.value, &iter.type,
    iter.name, iter.module_name,
    &iter.exported);
    if (ret < 0) {
    iter.pos_mod_end = iter.pos;
    return 0;
    }
    return 1;
    }
//
// ftrace_mod_get_kallsym() may also get symbols for pages allocated for ftrace
// purposes. In that case "__builtin__ftrace" is used as a module name, even
// though "__builtin__ftrace" is not a module.
//
#[no_mangle]
unsafe extern "C" fn get_ksymbol_ftrace_mod(iter: *mut kallsym_iter) -> c_int {
    let mut ret = ftrace_mod_get_kallsym(iter.pos - iter.pos_mod_end,
    &iter.value, &iter.type,
    iter.name, iter.module_name,
    &iter.exported);
    if (ret < 0) {
    iter.pos_ftrace_mod_end = iter.pos;
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn get_ksymbol_bpf(iter: *mut kallsym_iter) -> c_int {
    let mut ret = 0;
    strscpy(iter.module_name, "bpf", MODULE_NAME_LEN);
    iter.exported = 0;
    ret = bpf_get_kallsym(iter.pos - iter.pos_ftrace_mod_end,
    &iter.value, &iter.type,
    iter.name);
    if (ret < 0) {
    iter.pos_bpf_end = iter.pos;
    return 0;
    }
    return 1;
    }
//
// This uses "__builtin__kprobes" as a module name for symbols for pages
// allocated for kprobes' purposes, even though "__builtin__kprobes" is not a
// module.
//
#[no_mangle]
unsafe extern "C" fn get_ksymbol_kprobe(iter: *mut kallsym_iter) -> c_int {
    strscpy(iter.module_name, "__builtin__kprobes", MODULE_NAME_LEN);
    iter.exported = 0;
    return kprobe_get_kallsym(iter.pos - iter.pos_bpf_end,
    &iter.value, &iter.type,
    iter.name) < 0 ? 0 : 1;
    }
// Returns space to next name.
#[no_mangle]
unsafe extern "C" fn get_ksymbol_core(iter: *mut kallsym_iter) -> c_ulong {
pub static mut off: unsigned = 0;
    iter.module_name[0] = '\0';
    iter.value = kallsyms_sym_address(iter.pos);
    iter.type = kallsyms_get_symbol_type(off);
    off = kallsyms_expand_symbol(off, iter.name, ARRAY_SIZE!(iter.name));
    return off - iter.nameoff;
    }
#[no_mangle]
unsafe extern "C" fn reset_iter(iter: *mut kallsym_iter, new_pos: loff_t) {
    iter.name[0] = '\0';
    iter.nameoff = get_symbol_offset(new_pos);
    iter.pos = new_pos;
    if (new_pos == 0) {
    iter.pos_mod_end = 0;
    iter.pos_ftrace_mod_end = 0;
    iter.pos_bpf_end = 0;
    }
    }
//
// The end position (last + 1) of each additional kallsyms section is recorded
// in iter->pos_..._end as each section is added, and so can be used to
// determine which get_ksymbol_...() function to call next.
//
#[no_mangle]
unsafe extern "C" fn update_iter_mod(iter: *mut kallsym_iter, pos: loff_t) -> c_int {
    iter.pos = pos;
    if ((!iter.pos_mod_end || iter.pos_mod_end > pos) &&
    get_ksymbol_mod(iter)) {
    return 1;
    }
    if ((!iter.pos_ftrace_mod_end || iter.pos_ftrace_mod_end > pos) &&
    get_ksymbol_ftrace_mod(iter)) {
    return 1;
    }
    if ((!iter.pos_bpf_end || iter.pos_bpf_end > pos) &&
    get_ksymbol_bpf(iter)) {
    return 1;
    }
    return get_ksymbol_kprobe(iter);
    }
// Returns false if pos at or past end of file.
#[no_mangle]
unsafe extern "C" fn update_iter(iter: *mut kallsym_iter, pos: loff_t) -> c_int {
// Module symbols can be accessed randomly.
    if (pos >= kallsyms_num_syms) {
    return update_iter_mod(iter, pos);
    }
// If we're not on the desired position, reset to new position.
    if (pos != iter.pos) {
    reset_iter(iter, pos);
    }
    iter.nameoff += get_ksymbol_core(iter);
    iter.pos += 1;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn s_next() {
    (*pos)++;
    if (!update_iter(m.private, *pos)) {
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn s_start() {
    if (!update_iter(m.private, *pos)) {
    return core::ptr::null_mut();
    }
    return m.private;
    }
#[no_mangle]
unsafe extern "C" fn s_stop(m: *mut seq_file, p: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn s_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    let mut value = core::ptr::null_mut();
    let mut iter = m.private;
// Some debugging symbols have no name.  Ignore them.
    if (!iter.name[0]) {
    return 0;
    }
    value = iter.show_value ? iter.value : core::ptr::null_mut();
    if (iter.module_name[0]) {
    let mut type = 0;
//
// Label it "global" if it is exported,
// "local" if not exported.
//
    type = iter.exported ? toupper(iter.type) :
    tolower(iter.type);
    seq_printf(m, "%px %c %s\t[%s]\n", value,
    type, iter.name, iter.module_name);
    } else {
    seq_printf(m, "%px %c %s\n", value,
    iter.type, iter.name);
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__ksym {
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub ksym): *mut *mut __bpf_md_ptr(kallsym_iter ,,
}

#[no_mangle]
unsafe extern "C" fn ksym_prog_seq_show(m: *mut seq_file, in_stop: bool) -> c_int {
    let mut ctx;
    let mut meta;
    let mut prog = core::ptr::null_mut();
    meta.seq = m;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.ksym = m ? m.private : core::ptr::null_mut();
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_ksym_seq_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    return ksym_prog_seq_show(m, false);
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_ksym_seq_stop(m: *mut seq_file, p: *mut c_void) {
    if (!p) {
    (void) ksym_prog_seq_show(m, true);
    }
    else {
    s_stop(m, p);
    }
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_iter_ksym_init(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut iter = priv_data;
    reset_iter(iter, 0);
// cache here as in kallsyms_open() case; use current process
// credentials to tell BPF iterators if values should be shown.
//
    iter.show_value = kallsyms_show_value(current_cred());
    return 0;
    }
    DEFINE_BPF_ITER_FUNC(ksym, bpf_iter_meta *meta, kallsym_iter *ksym)
pub static mut bpf_iter_seq_info: usize = 0;
pub static mut bpf_iter_reg: usize = 0;
    BTF_ID_LIST_SINGLE(btf_ksym_iter_id, struct, kallsym_iter)
#[no_mangle]
unsafe extern "C" fn bpf_ksym_iter_register() -> c_int {
    ksym_iter_reg_info.ctx_arg_info[0].btf_id = *btf_ksym_iter_id;
    return bpf_iter_reg_target(&ksym_iter_reg_info);
    }
// late_initcall;

#[no_mangle]
unsafe extern "C" fn kallsyms_open(inode: *mut inode, file: *mut file) -> c_int {
//
// We keep iterator in m->private, since normal case is to
// s_start from where we left off, so we avoid doing
// using get_symbol_offset for every symbol.
//
    let mut iter = core::ptr::null_mut();
    iter = __seq_open_private(file, &kallsyms_op, sizeof!(*iter));
    if (!iter) {
    return -ENOMEM;
    }
    reset_iter(iter, 0);
//
// Instead of checking this on every s_show() call, cache
// the result here at open time.
//
    iter.show_value = kallsyms_show_value(file.f_cred);
    return 0;
    }

    const char *kdb_walk_kallsyms(loff_t *pos)
    {
pub static mut kdb_walk_kallsyms_iter: usize = 0;
    if (*pos == 0) {
    memset(&kdb_walk_kallsyms_iter, 0,
    sizeof!(kdb_walk_kallsyms_iter));
    reset_iter(&kdb_walk_kallsyms_iter, 0);
    }
    while (1) {
    if (!update_iter(&kdb_walk_kallsyms_iter, *pos)) {
    return core::ptr::null_mut();
    }
    ++*pos;
// Some debugging symbols have no name.  Ignore them.
    if (kdb_walk_kallsyms_iter.name[0]) {
    return kdb_walk_kallsyms_iter.name;
    }
    }
    }

pub static mut proc_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn kallsyms_init() -> c_int {
    proc_create("kallsyms", 0444, core::ptr::null_mut(), &kallsyms_proc_ops);
    return 0;
    }
// device_initcall;