//! Automatically rewritten from C to Rust
//! Source: kernel/crash_core.c
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
// crash.c - kernel crash support code.
// Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
//

// Per cpu memory for storing cpu states in case of system crash.
    let mut crash_notes = core::ptr::null_mut();
// time to wait for possible DMA to finish before starting the kdump kernel
// when a CMA reservation is used
//
pub const CMA_DMA_TIMEOUT_SEC: c_int = 10;

#[no_mangle]
pub unsafe extern "C" fn kimage_crash_copy_vmcoreinfo(image: *mut kimage) -> c_int {
    let mut vmcoreinfo_base = core::ptr::null_mut();
    struct page *vmcoreinfo_pages[DIV_ROUND_UP(VMCOREINFO_BYTES, PAGE_SIZE)];
    let mut order = 0;
    let mut nr_pages = 0;
    let mut i = 0;
    let mut safecopy = core::ptr::null_mut();
    nr_pages = DIV_ROUND_UP(VMCOREINFO_BYTES, PAGE_SIZE);
    order = get_order(VMCOREINFO_BYTES);
    if (!IS_ENABLED!(CONFIG_CRASH_DUMP)) {
    return 0;
    }
    if (image.type != KEXEC_TYPE_CRASH) {
    return 0;
    }
//
// For kdump, allocate one vmcoreinfo safe copy from the
// crash memory. as we have arch_kexec_protect_crashkres()
// after kexec syscall, we naturally protect it from write
// (even read) access under kernel direct mapping. But on
// the other hand, we still need to operate it when crash
// happens to generate vmcoreinfo note, hereby we rely on
// vmap for this purpose.
//
    vmcoreinfo_base = kimage_alloc_control_pages(image, order);
    if (!vmcoreinfo_base) {
    pr_warn!("Could not allocate vmcoreinfo buffer\n");
    return -ENOMEM;
    }
    for (i = 0; i < nr_pages; i++) {
    vmcoreinfo_pages[i] = vmcoreinfo_base + i;
    }
    safecopy = vmap(vmcoreinfo_pages, nr_pages, VM_MAP, PAGE_KERNEL);
    if (!safecopy) {
    pr_warn!("Could not vmap vmcoreinfo buffer\n");
    return -ENOMEM;
    }
    image.vmcoreinfo_data_copy = safecopy;
    crash_update_vmcoreinfo_safecopy(safecopy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_should_crash(p: *mut task_struct) -> c_int {
//
// If crash_kexec_post_notifiers is enabled, don't run
// crash_kexec() here yet, which must be run after panic
// notifiers in panic().
//
    if (crash_kexec_post_notifiers) {
    return 0;
    }
//
// There are 4 panic() calls in make_task_dead() path, each of which
// corresponds to each of these 4 conditions.
//
    if (in_interrupt() || !p.pid || is_global_init(p) || panic_on_oops) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_crash_loaded() -> c_int {
    return !!kexec_crash_image;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn crash_cma_clear_pending_dma() {
    if (!crashk_cma_cnt) {
    return;
    }
    mdelay(CMA_DMA_TIMEOUT_SEC * 1000);
    }
//
// No panic_cpu check version of crash_kexec().  This function is called
// only when panic_cpu holds the current CPU number; this is the only CPU
// which processes crash_kexec routines.
//
#[no_mangle]
pub unsafe extern "C" fn __crash_kexec(regs: *mut pt_regs) -> void __noclone {
// Take the kexec_lock here to prevent sys_kexec_load
// running on one cpu from replacing the crash kernel
// we are using after a panic on a different cpu.
//
// If the crash kernel was not located in a fixed area
// of memory the xchg(&kexec_crash_image) would be
// sufficient.  But since I reuse the memory...
//
    if (kexec_trylock()) {
    if (kexec_crash_image) {
    let mut fixed_regs;
    crash_setup_regs(&fixed_regs, regs);
    crash_save_vmcoreinfo();
    machine_crash_shutdown(&fixed_regs);
    crash_cma_clear_pending_dma();
    machine_kexec(kexec_crash_image);
    }
    kexec_unlock();
    }
    }
// STACK_FRAME_NON_STANDARD;
#[no_mangle]
pub unsafe extern "C" fn crash_kexec(regs: *mut pt_regs) -> __bpf_kfunc void {
    if (panic_try_start()) {
// This is the 1st CPU which comes here, so go ahead.
    __crash_kexec(regs);
//
// Reset panic_cpu to allow another panic()/crash_kexec()
// call.
//
    panic_reset();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crash_resource_size(res: *const resource) -> resource_size_t {
    return !res.end ? 0 : resource_size(res);
    }
#[no_mangle]
pub unsafe extern "C" fn crash_prepare_elf64_headers() {
    let mut ehdr = core::ptr::null_mut();
    let mut phdr = core::ptr::null_mut();
pub static mut nr_cpus: c_ulong = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut i = 0;
    unsigned long long notes_addr;
    unsigned long mstart, mend;
// extra phdr for vmcoreinfo ELF note
    nr_phdr = nr_cpus + 1;
    nr_phdr += mem.nr_ranges;
//
// kexec-tools creates an extra PT_LOAD phdr for kernel text mapping
// area (for example, ffffffff80000000 - ffffffffa0000000 on x86_64).
// I think this is required by tools like gdb. So same physical
// memory will be mapped in two ELF headers. One will contain kernel
// text virtual addresses and other will have __va(physical) addresses.
//
    nr_phdr += 1;
    elf_sz = sizeof!(Elf64_Ehdr) + nr_phdr * sizeof!(Elf64_Phdr);
    elf_sz = ALIGN(elf_sz, ELF_CORE_HEADER_ALIGN);
    buf = vzalloc(elf_sz);
    if (!buf) {
    return -ENOMEM;
    }
    ehdr = buf;
    phdr = (ehdr + 1);
    memcpy(ehdr.e_ident, ELFMAG, SELFMAG);
    ehdr.e_ident[EI_CLASS] = ELFCLASS64;
    ehdr.e_ident[EI_DATA] = ELFDATA2LSB;
    ehdr.e_ident[EI_VERSION] = EV_CURRENT;
    ehdr.e_ident[EI_OSABI] = ELF_OSABI;
    memset(ehdr.e_ident + EI_PAD, 0, EI_NIDENT - EI_PAD);
    ehdr.e_type = ET_CORE;
    ehdr.e_machine = ELF_ARCH;
    ehdr.e_version = EV_CURRENT;
    ehdr.e_phoff = sizeof!(Elf64_Ehdr);
    ehdr.e_ehsize = sizeof!(Elf64_Ehdr);
    ehdr.e_phentsize = sizeof!(Elf64_Phdr);
// Prepare one phdr of type PT_NOTE for each possible CPU
    for_each_possible_cpu(cpu) {
    phdr.p_type = PT_NOTE;
    notes_addr = per_cpu_ptr_to_phys(per_cpu_ptr(crash_notes, cpu));
    phdr.p_offset = phdr.p_paddr = notes_addr;
    phdr.p_filesz = phdr.p_memsz = sizeof!(note_buf_t);
    (ehdr.e_phnum)++;
    phdr += 1;
    }
// Prepare one PT_NOTE header for vmcoreinfo
    phdr.p_type = PT_NOTE;
    phdr.p_offset = phdr.p_paddr = paddr_vmcoreinfo_note();
    phdr.p_filesz = phdr.p_memsz = VMCOREINFO_NOTE_SIZE;
    (ehdr.e_phnum)++;
    phdr += 1;
// Prepare PT_LOAD type program header for kernel text region
    if (need_kernel_map) {
    phdr.p_type = PT_LOAD;
    phdr.p_flags = PF_R|PF_W|PF_X;
    phdr.p_vaddr = (unsigned long) _text;
    phdr.p_filesz = phdr.p_memsz = _end - _text;
    phdr.p_offset = phdr.p_paddr = __pa_symbol(_text);
    ehdr.e_phnum += 1;
    phdr += 1;
    }
// Go through all the ranges in mem->ranges[] and prepare phdr
    while (i < mem.nr_ranges) {
    mstart = mem.ranges[i].start;
    mend = mem.ranges[i].end;
    phdr.p_type = PT_LOAD;
    phdr.p_flags = PF_R|PF_W|PF_X;
    phdr.p_offset  = mstart;
    phdr.p_paddr = mstart;
    phdr.p_vaddr = (unsigned long) __va(mstart);
    phdr.p_filesz = phdr.p_memsz = mend - mstart + 1;
    phdr.p_align = 0;
    ehdr.e_phnum += 1;

    kexec_dprintk("Crash PT_LOAD ELF header. phdr=%p vaddr=0x%llx, paddr=0x%llx, sz=0x%llx e_phnum=%d p_offset=0x%llx\n",
    phdr, phdr.p_vaddr, phdr.p_paddr, phdr.p_filesz,
    ehdr.e_phnum, phdr.p_offset);

    phdr += 1;
    }
// addr = buf;
// sz = elf_sz;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_cmem() {
    let mut cmem = core::ptr::null_mut();
    cmem = kvzalloc_flex(*cmem, ranges, nr_ranges);
    if (!cmem) {
    return core::ptr::null_mut();
    }
    cmem.max_nr_ranges = nr_ranges;
    return cmem;
    }
    unsigned int __weak arch_get_system_nr_ranges(void) { return 0; }
    int __weak arch_crash_populate_cmem(crash_mem *cmem) { return -1; }
    int __weak arch_crash_exclude_ranges(crash_mem *cmem) { return 0; }
    int __weak arch_crash_exclude_mem_range(crash_mem **mem,
    unsigned long long mstart,
    unsigned long long mend)
    {
    return crash_exclude_mem_range(*mem, mstart, mend);
    }
#[no_mangle]
pub unsafe extern "C" fn crash_exclude_core_ranges(cmem: *mut crash_mem) -> c_int {
    let mut ret = 0;
    let mut i = 0;
// Exclude crashkernel region
    ret = arch_crash_exclude_mem_range(cmem, crashk_res.start, crashk_res.end);
    if (ret) {
    return ret;
    }
    if (crashk_low_res.end) {
    ret = arch_crash_exclude_mem_range(cmem, crashk_low_res.start, crashk_low_res.end);
    if (ret) {
    return ret;
    }
    }
    while (i < crashk_cma_cnt) {
    ret = arch_crash_exclude_mem_range(cmem, crashk_cma_ranges[i].start,
    crashk_cma_ranges[i].end);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_prepare_headers() {
    let mut max_nr_ranges = 0;
    let mut cmem = core::ptr::null_mut();
    let mut ret = 0;
    max_nr_ranges = arch_get_system_nr_ranges();
    if (!max_nr_ranges) {
    return -ENOMEM;
    }
    cmem = alloc_cmem(max_nr_ranges);
    if (!cmem) {
    return -ENOMEM;
    }
    ret = arch_crash_populate_cmem(cmem);
    if (ret) {
// goto;
    }
    ret = crash_exclude_core_ranges(&cmem);
    if (ret) {
// goto;
    }
    ret = arch_crash_exclude_ranges(cmem);
    if (ret) {
// goto;
    }
// Return the computed number of memory ranges, for hotplug usage
    if (nr_mem_ranges) {
// nr_mem_ranges = cmem->nr_ranges;
    }
    ret = crash_prepare_elf64_headers(cmem, need_kernel_map, addr, sz);
// label;
    kvfree(cmem);
    return ret;
    }
//
// crash_exclude_mem_range - exclude a mem range for existing ranges
// @mem: mem->range contains an array of ranges sorted in ascending order
// @mstart: the start of to-be-excluded range
// @mend: the start of to-be-excluded range
//
// If you are unsure if a range split will happen, to avoid function call
// failure because of -ENOMEM, always make sure
// mem->max_nr_ranges == mem->nr_ranges + 1
// before calling the function each time.
//
// returns 0 if a memory range is excluded successfully
// return -ENOMEM if mem->ranges doesn't have space to hold split ranges
//
#[no_mangle]
pub unsafe extern "C" fn crash_exclude_mem_range() {
    let mut i = 0;
    unsigned long long start, end, p_start, p_end;
    while (i < mem.nr_ranges) {
    start = mem.ranges[i].start;
    end = mem.ranges[i].end;
    p_start = mstart;
    p_end = mend;
    if (p_start > end) {
    continue;
    }
//
// Because the memory ranges in mem->ranges are stored in
// ascending order, when we detect `p_end < start`, we can
// immediately exit the for loop, as the subsequent memory
// ranges will definitely be outside the range we are looking
// for.
//
    if (p_end < start) {
    break;
    }
// Truncate any area outside of range
    if (p_start < start) {
    p_start = start;
    }
    if (p_end > end) {
    p_end = end;
    }
// Found completely overlapping range
    if (p_start == start && p_end == end) {
    memmove(&mem.ranges[i], &mem.ranges[i + 1],
    (mem.nr_ranges - (i + 1)) * sizeof!(mem.ranges[i]));
    i -= 1;
    mem.nr_ranges -= 1;
    } else if (p_start > start && p_end < end) {
// Split original range
    if (mem.nr_ranges >= mem.max_nr_ranges) {
    return -ENOMEM;
    }
    memmove(&mem.ranges[i + 2], &mem.ranges[i + 1],
    (mem.nr_ranges - (i + 1)) * sizeof!(mem.ranges[i]));
    mem.ranges[i].end = p_start - 1;
    mem.ranges[i + 1].start = p_end + 1;
    mem.ranges[i + 1].end = end;
    i += 1;
    mem.nr_ranges += 1;
    } else if (p_start != start) {
    mem.ranges[i].end = p_start - 1;
    }
    else {
    mem.ranges[i].start = p_end + 1;
    }
    }
    return 0;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn crash_get_memory_size() -> isize {
pub static mut size: isize = 0;
    if (!kexec_trylock()) {
    return -EBUSY;
    }
    size += crash_resource_size(&crashk_res);
    size += crash_resource_size(&crashk_low_res);
    kexec_unlock();
    return size;
    }
#[no_mangle]
pub unsafe extern "C" fn __crash_shrink_memory() {
    let mut ram_res = core::ptr::null_mut();
    ram_res = kzalloc_obj(*ram_res);
    if (!ram_res) {
    return -ENOMEM;
    }
    ram_res.start = old_res.start + new_size;
    ram_res.end   = old_res.end;
    ram_res.flags = IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM;
    ram_res.name  = "System RAM";
    if (!new_size) {
    release_resource(old_res);
    old_res.start = 0;
    old_res.end   = 0;
    } else {
    old_res.end = ram_res.start - 1;
    }
    crash_free_reserved_phys_range(ram_res.start, ram_res.end);
    insert_resource(&iomem_resource, ram_res);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_shrink_memory(new_size: c_ulong) -> c_int {
pub static mut ret: c_int = 0;
    unsigned long old_size, low_size;
    if (!kexec_trylock()) {
    return -EBUSY;
    }
    if (kexec_crash_image) {
    ret = -ENOENT;
// goto;
    }
    low_size = crash_resource_size(&crashk_low_res);
    old_size = crash_resource_size(&crashk_res) + low_size;
    new_size = roundup(new_size, KEXEC_CRASH_MEM_ALIGN);
    if (new_size >= old_size) {
    ret = (new_size == old_size) ? 0 : -EINVAL;
// goto;
    }
//
// (low_size > new_size) implies that low_size is greater than zero.
// This also means that if low_size is zero, the else branch is taken.
//
// If low_size is greater than 0, (low_size > new_size) indicates that
// crashk_low_res also needs to be shrunken. Otherwise, only crashk_res
// needs to be shrunken.
//
    if (low_size > new_size) {
    ret = __crash_shrink_memory(&crashk_res, 0);
    if (ret) {
// goto;
    }
    ret = __crash_shrink_memory(&crashk_low_res, new_size);
    } else {
    ret = __crash_shrink_memory(&crashk_res, new_size - low_size);
    }
// Swap crashk_res and crashk_low_res if needed
    if (!crashk_res.end && crashk_low_res.end) {
    crashk_res.start = crashk_low_res.start;
    crashk_res.end   = crashk_low_res.end;
    release_resource(&crashk_low_res);
    crashk_low_res.start = 0;
    crashk_low_res.end   = 0;
    insert_resource(&iomem_resource, &crashk_res);
    }
// label;
    kexec_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_save_cpu(regs: *mut pt_regs, cpu: c_int) {
    let mut prstatus;
    let mut buf = core::ptr::null_mut();
    if ((cpu < 0) || (cpu >= nr_cpu_ids)) {
    return;
    }
// Using ELF notes here is opportunistic.
// I need a well defined structure format
// for the data I pass, and I need tags
// on the data to indicate what information I have
// squirrelled away.  ELF notes happen to provide
// all of that, so there is no need to invent something new.
//
    buf = per_cpu_ptr(crash_notes, cpu);
    if (!buf) {
    return;
    }
    memset(&prstatus, 0, sizeof!(prstatus));
    prstatus.common.pr_pid = current.pid;
    elf_core_copy_regs(&prstatus.pr_reg, regs);
    buf = append_elf_note(buf, NN_PRSTATUS, NT_PRSTATUS,
    &prstatus, sizeof!(prstatus));
    final_note(buf);
    }
#[no_mangle]
unsafe extern "C" fn crash_notes_memory_init() -> c_int {
// Allocate memory for saving cpu registers.
    size_t size, align;
//
// crash_notes could be allocated across 2 vmalloc pages when percpu
// is vmalloc based . vmalloc doesn't guarantee 2 continuous vmalloc
// pages are also on 2 continuous physical pages. In this case the
// 2nd part of crash_notes in 2nd page could be lost since only the
// starting address and size of crash_notes are exported through sysfs.
// Here round up the size of crash_notes to the nearest power of two
// and pass it to __alloc_percpu as align value. This can make sure
// crash_notes is allocated inside one physical page.
//
    size = sizeof!(note_buf_t);
    align = min(roundup_pow_of_two(sizeof!(note_buf_t)), PAGE_SIZE);
//
// Break compile if size is bigger than PAGE_SIZE since crash_notes
// definitely will be in 2 pages with that.
//
// BUILD_BUG_ON;
    crash_notes = __alloc_percpu(size, align);
    if (!crash_notes) {
    pr_warn!("Memory allocation for saving cpu register states failed\n");
    return -ENOMEM;
    }
    return 0;
    }
// subsys_initcall;

//
// Different than kexec/kdump loading/unloading/jumping/shrinking which
// usually rarely happen, there will be many crash hotplug events notified
// during one short period, e.g one memory board is hot added and memory
// regions are online. So mutex lock  __crash_hotplug_lock is used to
// serialize the crash hotplug handling specifically.
//
// static DEFINE_MUTEX(__crash_hotplug_lock);

//
// This routine utilized when the crash_hotplug sysfs node is read.
// It reflects the kernel's ability/permission to update the kdump
// image directly.
//
#[no_mangle]
pub unsafe extern "C" fn crash_check_hotplug_support() -> c_int {
pub static mut rc: c_int = 0;
    crash_hotplug_lock();
// Obtain lock while reading crash information
    if (!kexec_trylock()) {
    if (!kexec_in_progress) {
    pr_info!("kexec_trylock() failed, kdump image may be inaccurate\n");
    }
    crash_hotplug_unlock();
    return 0;
    }
    if (kexec_crash_image) {
    rc = kexec_crash_image.hotplug_support;
    }
// Release lock now that update complete
    kexec_unlock();
    crash_hotplug_unlock();
    return rc;
    }
//
// To accurately reflect hot un/plug changes of CPU and Memory resources
// (including onling and offlining of those resources), the relevant
// kexec segments must be updated with latest CPU and Memory resources.
//
// Architectures must ensure two things for all segments that need
// updating during hotplug events:
//
// 1. Segments must be large enough to accommodate a growing number of
// resources.
// 2. Exclude the segments from SHA verification.
//
// For example, on most architectures, the elfcorehdr (which is passed
// to the crash kernel via the elfcorehdr= parameter) must include the
// new list of CPUs and memory. To make changes to the elfcorehdr, it
// should be large enough to permit a growing number of CPU and Memory
// resources. One can estimate the elfcorehdr memory size based on
// NR_CPUS_DEFAULT and CRASH_MAX_MEMORY_RANGES. The elfcorehdr is
// excluded from SHA verification by default if the architecture
// supports crash hotplug.
//
#[no_mangle]
unsafe extern "C" fn crash_handle_hotplug_event(hp_action: c_uint, cpu: c_uint, arg: *mut c_void) {
    let mut image = core::ptr::null_mut();
    crash_hotplug_lock();
// Obtain lock while changing crash information
    if (!kexec_trylock()) {
    if (!kexec_in_progress) {
    pr_info!("kexec_trylock() failed, kdump image may be inaccurate\n");
    }
    crash_hotplug_unlock();
    return;
    }
// Check kdump is not loaded
    if (!kexec_crash_image) {
// goto;
    }
    image = kexec_crash_image;
// Check that kexec segments update is permitted
    if (!image.hotplug_support) {
// goto;
    }
    if (hp_action == KEXEC_CRASH_HP_ADD_CPU ||
    hp_action == KEXEC_CRASH_HP_REMOVE_CPU) {
    pr_debug!("hp_action %u, cpu %u\n", hp_action, cpu);
    }
    else {
    pr_debug!("hp_action %u\n", hp_action);
    }
//
// The elfcorehdr_index is set to -1 when the struct kimage
// is allocated. Find the segment containing the elfcorehdr,
// if not already found.
//
    if (image.elfcorehdr_index < 0) {
    let mut mem = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
    while (n < image.nr_segments) {
    mem = image.segment[n].mem;
    ptr = kmap_local_page(pfn_to_page(mem >> PAGE_SHIFT));
    if (ptr) {
// The segment containing elfcorehdr
    if (memcmp(ptr, ELFMAG, SELFMAG) == 0) {
    image.elfcorehdr_index = (int)n;
    }
    kunmap_local(ptr);
    }
    }
    }
    if (image.elfcorehdr_index < 0) {
    pr_err!("unable to locate elfcorehdr segment");
// goto;
    }
// Needed in order for the segments to be updated
    arch_kexec_unprotect_crashkres();
// Differentiate between normal load and hotplug update
    image.hp_action = hp_action;
// Now invoke arch-specific update handler
    arch_crash_handle_hotplug_event(image, arg);
// No longer handling a hotplug event
    image.hp_action = KEXEC_CRASH_HP_NONE;
    image.elfcorehdr_updated = true;
// Change back to read-only
    arch_kexec_protect_crashkres();
// Errors in the callback is not a reason to rollback state
// label;
// Release lock now that update complete
    kexec_unlock();
    crash_hotplug_unlock();
    }
#[no_mangle]
unsafe extern "C" fn crash_memhp_notifier(nb: *mut notifier_block, val: c_ulong, arg: *mut c_void) -> c_int {
    match (val) {
    MEM_ONLINE => {
    crash_handle_hotplug_event(KEXEC_CRASH_HP_ADD_MEMORY,
    KEXEC_CRASH_HP_INVALID_CPU, arg);
    // break;
    }
    MEM_OFFLINE => {
    crash_handle_hotplug_event(KEXEC_CRASH_HP_REMOVE_MEMORY,
    KEXEC_CRASH_HP_INVALID_CPU, arg);
    // break;
    }
    }
    return NOTIFY_OK;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn crash_cpuhp_online(cpu: c_uint) -> c_int {
    crash_handle_hotplug_event(KEXEC_CRASH_HP_ADD_CPU, cpu, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crash_cpuhp_offline(cpu: c_uint) -> c_int {
    crash_handle_hotplug_event(KEXEC_CRASH_HP_REMOVE_CPU, cpu, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crash_hotplug_init() -> c_int {
pub static mut result: c_int = 0;
    if (IS_ENABLED!(CONFIG_MEMORY_HOTPLUG)) {
    register_memory_notifier(&crash_memhp_nb);
    }
    if (IS_ENABLED!(CONFIG_HOTPLUG_CPU)) {
    result = cpuhp_setup_state_nocalls(CPUHP_BP_PREPARE_DYN,
    "crash/cpuhp", crash_cpuhp_online, crash_cpuhp_offline);
    }
    return result;
    }
// subsys_initcall;