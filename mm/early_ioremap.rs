//! Automatically rewritten from C to Rust
//! Source: mm/early_ioremap.c
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
// Provide common bits of early_ioremap() support for architectures needing
// temporary mappings during boot before ioremap() is available.
//
// This is mostly a direct copy of the x86 early_ioremap implementation.
//
// (C) Copyright 1995 1996, 2014 Linus Torvalds
//

    static int early_ioremap_debug __initdata;
#[no_mangle]
unsafe extern "C" fn early_ioremap_debug_setup(str: *mut c_char) -> c_int {
    early_ioremap_debug = 1;
    return 0;
    }
    early_param!("early_ioremap_debug", early_ioremap_debug_setup);

    do {						
    if (unlikely(early_ioremap_debug)) {	
    pr_warn!(fmt, ##args);		
    dump_stack();			
    }					
    } while (0)
    static int after_paging_init __initdata;
    pgprot_t __init __weak early_memremap_pgprot_adjust(resource_size_t phys_addr,
    unsigned long size,
    pgprot_t prot)
    {
    return prot;
    }
//
// Only architectures whose early_ioremap() must stop using __early_set_fixmap()
// after paging_init() need to call this.
//
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_reset()  {
    after_paging_init = 1;
    }
//
// Only architectures that call early_ioremap_reset() need to define
// __late_set_fixmap() and __late_clear_fixmap(), which early_ioremap() uses
// instead of __early_set_fixmap() after the reset.
//

    static inline void __init __late_set_fixmap(enum fixed_addresses idx,
    phys_addr_t phys, pgprot_t prot)
    {
    BUG();
    }

#[no_mangle]
pub unsafe extern "C" fn __late_clear_fixmap(idx: fixed_addresses)  {
    BUG();
    }

    static void __iomem *prev_map[FIX_BTMAPS_SLOTS] __initdata;
    static unsigned long prev_size[FIX_BTMAPS_SLOTS] __initdata;
    static unsigned long slot_virt[FIX_BTMAPS_SLOTS] __initdata;
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_setup()  {
    let mut i = 0;
    while (i < FIX_BTMAPS_SLOTS) {
    WARN_ON_ONCE!(prev_map[i]);
    slot_virt[i] = __fix_to_virt(FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*i);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_early_ioremap_leak() -> c_int {
pub static mut count: c_int = 0;
    let mut i = 0;
    for (i = 0; i < FIX_BTMAPS_SLOTS; i++) {
    if (prev_map[i])
    count += 1;
    }
    if (WARN(count, "Debug warning: early ioremap leak of %d areas detected.\n"
    "please boot with early_ioremap_debug and report the dmesg.\n",
    count)) {
    return 1;
    }
    return 0;
    }
    late_initcall!(check_early_ioremap_leak);
    static void __init __iomem *
    __early_ioremap(resource_size_t phys_addr, unsigned long size, pgprot_t prot)
    {
    let mut offset = 0;
    let mut last_addr;
    let mut nrpages = 0;
    enum fixed_addresses idx;
    let mut i = 0;
    let mut slot = 0;
    WARN_ON!(system_state >= SYSTEM_RUNNING);
    slot = -1;
    while (i < FIX_BTMAPS_SLOTS) {
    if (!prev_map[i]) {
    slot = i;
    break;
    }
    }
    if (WARN(slot < 0, "%s(%pa, %08lx) not found slot\n",
    __func__, &phys_addr, size)) {
    return core::ptr::null_mut();
    }
// Don't allow wraparound or zero size
    last_addr = phys_addr + size - 1;
    if (WARN_ON!(!size || last_addr < phys_addr)) {
    return core::ptr::null_mut();
    }
    prev_size[slot] = size;
//
// Mappings have to be page-aligned
//
    offset = offset_in_page(phys_addr);
    phys_addr &= PAGE_MASK;
    size = PAGE_ALIGN(last_addr + 1) - phys_addr;
//
// Mappings have to fit in the FIX_BTMAP area.
//
    nrpages = size >> PAGE_SHIFT;
    if (WARN_ON!(nrpages > NR_FIX_BTMAPS)) {
    return core::ptr::null_mut();
    }
    early_ioremap_dbg("%s(%pa, %08lx) [%d] => %08lx + %08lx\n",
    __func__, &phys_addr, size, slot, slot_virt[slot], offset);
//
// Ok, go for it..
//
    idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*slot;
    while (nrpages > 0) {
    if (after_paging_init) {
    __late_set_fixmap(idx, phys_addr, prot);
    }
    else {
    __early_set_fixmap(idx, phys_addr, prot);
    }
    phys_addr += PAGE_SIZE;
    idx -= 1;
    nrpages -= 1;
    }
    prev_map[slot] = (offset + slot_virt[slot]);
    return prev_map[slot];
    }
#[no_mangle]
pub unsafe extern "C" fn early_iounmap(addr: *mut c_void, size: c_ulong)  {
    let mut virt_addr = 0;
    let mut offset = 0;
    let mut nrpages = 0;
    enum fixed_addresses idx;
    let mut i = 0;
    let mut slot = 0;
    slot = -1;
    while (i < FIX_BTMAPS_SLOTS) {
    if (prev_map[i] == addr) {
    slot = i;
    break;
    }
    }
    if (WARN(slot < 0, "%s(%p, %08lx) not found slot\n",
    __func__, addr, size)) {
    return;
    }
    if (WARN(prev_size[slot] != size,
    "%s(%p, %08lx) [%d] size not consistent %08lx\n",
    __func__, addr, size, slot, prev_size[slot])) {
    return;
    }
    early_ioremap_dbg("%s(%p, %08lx) [%d]\n", __func__, addr, size, slot);
    virt_addr = (unsigned long)addr;
    if (WARN_ON!(virt_addr < fix_to_virt(FIX_BTMAP_BEGIN))) {
    return;
    }
    offset = offset_in_page(virt_addr);
    nrpages = PAGE_ALIGN(offset + size) >> PAGE_SHIFT;
    idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*slot;
    while (nrpages > 0) {
    if (after_paging_init) {
    __late_clear_fixmap(idx);
    }
    else {
    __early_set_fixmap(idx, 0, FIXMAP_PAGE_CLEAR);
    }
    idx -= 1;
    nrpages -= 1;
    }
    prev_map[slot] = core::ptr::null_mut();
    }
// Remap an IO device
    void __init __iomem *
    early_ioremap(resource_size_t phys_addr, unsigned long size)
    {
    return __early_ioremap(phys_addr, size, FIXMAP_PAGE_IO);
    }
// Remap memory
    void __init *
    early_memremap(resource_size_t phys_addr, unsigned long size)
    {
    pgprot_t prot = early_memremap_pgprot_adjust(phys_addr, size,
    FIXMAP_PAGE_NORMAL);
    return __early_ioremap(phys_addr, size, prot);
    }

    void __init *
    early_memremap_ro(resource_size_t phys_addr, unsigned long size)
    {
    pgprot_t prot = early_memremap_pgprot_adjust(phys_addr, size,
    FIXMAP_PAGE_RO);
    return __early_ioremap(phys_addr, size, prot);
    }

    void __init *
    early_memremap_prot(resource_size_t phys_addr, unsigned long size,
    unsigned long prot_val)
    {
    return __early_ioremap(phys_addr, size,
    __pgprot(prot_val));
    }

//
// If no empty slot, handle that and return -ENOMEM.
//
#[no_mangle]
pub unsafe extern "C" fn copy_from_early_mem(dest: *mut c_void, src: phys_addr_t, size: c_ulong) -> c_int {
    unsigned long slop, clen;
pub static mut p: *mut c_void = core::ptr::null_mut();
    while (size) {
    slop = offset_in_page(src);
    clen = size;
    if (clen > MAX_MAP_CHUNK - slop) {
    clen = MAX_MAP_CHUNK - slop;
    }
    p = early_memremap(src & PAGE_MASK, clen + slop);
    if (!p) {
    return -ENOMEM;
    }
    memcpy(dest, p + slop, clen);
    early_memunmap(p, clen + slop);
    dest += clen;
    src += clen;
    size -= clen;
    }
    return 0;
    }

    void __init __iomem *
    early_ioremap(resource_size_t phys_addr, unsigned long size)
    {
    return phys_addr;
    }
// Remap memory
    void __init *
    early_memremap(resource_size_t phys_addr, unsigned long size)
    {
    return phys_addr;
    }
    void __init *
    early_memremap_ro(resource_size_t phys_addr, unsigned long size)
    {
    return phys_addr;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: early_iounmap
pub unsafe extern "C" fn early_iounmap_dup(addr: *mut c_void, size: c_ulong)  {
    }

#[no_mangle]
pub unsafe extern "C" fn early_memunmap(addr: *mut c_void, size: c_ulong)  {
    early_iounmap(addr, size);
    }