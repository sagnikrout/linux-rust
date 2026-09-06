//! Automatically rewritten from C to Rust
//! Source: kernel/module/kallsyms.c
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
//
// Module kallsyms support
//
// Copyright (C) 2010 Rusty Russell
//

// Lookup exported symbol in given range of kernel_symbols
    static const struct kernel_symbol *lookup_exported_symbol(const char *name,
    const struct kernel_symbol *start,
    const struct kernel_symbol *stop)
    {
    return bsearch(name, start, stop - start,
    sizeof!(kernel_symbol), cmp_name);
    }
#[no_mangle]
pub unsafe extern "C" fn is_exported(name: *mut c_char, value: c_ulong, mod: *mut module) -> c_int {
pub static mut ks: *mut c_void = core::ptr::null_mut();
    if (!mod) {
    ks = lookup_exported_symbol(name, __start___ksymtab, __stop___ksymtab);
    }
    else {
    ks = lookup_exported_symbol(name, mod.syms, mod.syms + mod.num_syms);
    }
    return ks && kernel_symbol_value(ks) == value;
    }
// As per nm
#[no_mangle]
unsafe extern "C" fn elf_type(sym: *const Elf_Sym, info: *const load_info) -> c_char {
    let mut sechdrs = info.sechdrs;
    if (ELF_ST_BIND(sym.st_info) == STB_WEAK) {
    if (ELF_ST_TYPE(sym.st_info) == STT_OBJECT) {
    return 'v';
    }
    else {
    return 'w';
    }
    }
    if (sym.st_shndx == SHN_UNDEF) {
    return 'U';
    }
    if (sym.st_shndx == SHN_ABS || sym.st_shndx == info.index.pcpu) {
    return 'a';
    }
    if (sym.st_shndx >= SHN_LORESERVE) {
    return '?';
    }
    if (sechdrs[sym.st_shndx].sh_flags & SHF_EXECINSTR) {
    return 't';
    }
    if (sechdrs[sym.st_shndx].sh_flags & SHF_ALLOC &&
    sechdrs[sym.st_shndx].sh_type != SHT_NOBITS) {
    if (!(sechdrs[sym.st_shndx].sh_flags & SHF_WRITE)) {
    return 'r';
    }

    else if (sechdrs[sym.st_shndx].sh_flags & ARCH_SHF_SMALL) {
    return 'g';
    }
    else {
    return 'd';
    }
    }
    if (sechdrs[sym.st_shndx].sh_type == SHT_NOBITS) {
    if (sechdrs[sym.st_shndx].sh_flags & ARCH_SHF_SMALL) {
    return 's';
    }
    else {
    return 'b';
    }
    }
    if (strstarts(info.secstrings + sechdrs[sym.st_shndx].sh_name,
    ".debug")) {
    return 'n';
    }
    return '?';
    }
#[no_mangle]
pub unsafe extern "C" fn is_core_symbol(src: *mut Elf_Sym, sechdrs: *mut Elf_Shdr, shnum: c_uint, pcpundx: c_uint) -> bool {
pub static mut sec: *mut c_void = core::ptr::null_mut();
    enum mod_mem_type type;
    if (src.st_shndx == SHN_UNDEF ||
    src.st_shndx >= shnum ||
    !src.st_name) {
    return false;
    }

    if (src.st_shndx == pcpundx) {
    return true;
    }

    sec = sechdrs + src.st_shndx;
    type = sec.sh_entsize >> SH_ENTSIZE_TYPE_SHIFT;
    if (!(sec.sh_flags & SHF_ALLOC)

    || !(sec.sh_flags & SHF_EXECINSTR)

    || mod_mem_type_is_init(type)) {
    return false;
    }
    return true;
    }
//
// We only allocate and copy the strings needed by the parts of symtab
// we keep.  This is simple, but has the effect of making multiple
// copies of duplicates.  We could be more sophisticated, see
// linux-kernel thread starting with
// <73defb5e4bca04a6431392cc341112b1@localhost>.
//
#[no_mangle]
pub unsafe extern "C" fn layout_symtab(mod: *mut module, info: *mut load_info) {
    let mut symsect = info.sechdrs + info.index.sym;
    let mut strsect = info.sechdrs + info.index.str;
pub static mut src: *mut c_void = core::ptr::null_mut();
    unsigned int i, nsrc, ndst, strtab_size = 0;
    let mut mod_mem_data = &mod.mem[MOD_DATA];
    let mut mod_mem_init_data = &mod.mem[MOD_INIT_DATA];
// Put symbol section at end of init part of module.
    symsect.sh_flags |= SHF_ALLOC;
    symsect.sh_entsize = module_get_offset_and_type!(mod, MOD_INIT_DATA,
    symsect, info.index.sym);
    pr_debug!("\t%s\n", info.secstrings + symsect.sh_name);
    src = info.hdr + symsect.sh_offset;
    nsrc = symsect.sh_size / sizeof!(*src);
// Compute total space required for the core symbols' strtab.
    while (i < nsrc) {
    if (i == 0 || is_livepatch_module(mod) ||
    is_core_symbol(src + i, info.sechdrs, info.hdr.e_shnum,
    info.index.pcpu)) {
    strtab_size += strlen(&info.strtab[src[i].st_name]) + 1;
    ndst += 1;
    }
    }
// Append room for core symbols at end of core part.
    info.symoffs = ALIGN(mod_mem_data.size, symsect.sh_addralign ?: 1);
    info.stroffs = mod_mem_data.size = info.symoffs + ndst * sizeof!(Elf_Sym);
    mod_mem_data.size += strtab_size;
// Note add_kallsyms() computes strtab_size as core_typeoffs - stroffs
    info.core_typeoffs = mod_mem_data.size;
    mod_mem_data.size += ndst * sizeof!(char);
// Put string table section at end of init part of module.
    strsect.sh_flags |= SHF_ALLOC;
    strsect.sh_entsize = module_get_offset_and_type!(mod, MOD_INIT_DATA,
    strsect, info.index.str);
    pr_debug!("\t%s\n", info.secstrings + strsect.sh_name);
// We'll tack temporary mod_kallsyms on the end.
    mod_mem_init_data.size = ALIGN(mod_mem_init_data.size,
    __alignof__(mod_kallsyms));
    info.mod_kallsyms_init_off = mod_mem_init_data.size;
    mod_mem_init_data.size += sizeof!(mod_kallsyms);
    info.init_typeoffs = mod_mem_init_data.size;
    mod_mem_init_data.size += nsrc * sizeof!(char);
    }
//
// We use the full symtab and strtab which layout_symtab arranged to
// be appended to the init section.  Later we switch to the cut-down
// core-only ones.
//
#[no_mangle]
pub unsafe extern "C" fn add_kallsyms(mod: *mut module, info: *const load_info) {
    let mut i = 0;
    let mut ndst = 0;
pub static mut src: *mut c_void = core::ptr::null_mut();
pub static mut dst: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut symsec = &info.sechdrs[info.index.sym];
    let mut strtab_size = 0;
    let mut data_base = mod.mem[MOD_DATA].base;
    let mut init_data_base = mod.mem[MOD_INIT_DATA].base;
pub static mut kallsyms: *mut c_void = core::ptr::null_mut();
    kallsyms = init_data_base + info.mod_kallsyms_init_off;
    kallsyms.symtab = symsec.sh_addr;
    kallsyms.num_symtab = symsec.sh_size / sizeof!(Elf_Sym);
// Make sure we get permanent strtab: don't use info->strtab.
    kallsyms.strtab = info.sechdrs[info.index.str].sh_addr;
    kallsyms.typetab = init_data_base + info.init_typeoffs;
//
// Now populate the cut down core kallsyms for after init
// and set types up while we still have access to sections.
//
    mod.core_kallsyms.symtab = dst = data_base + info.symoffs;
    mod.core_kallsyms.strtab = s = data_base + info.stroffs;
    mod.core_kallsyms.typetab = data_base + info.core_typeoffs;
    strtab_size = info.core_typeoffs - info.stroffs;
    src = kallsyms.symtab;
    while (i < kallsyms.num_symtab) {
    kallsyms.typetab[i] = elf_type(src + i, info);
    if (i == 0 || is_livepatch_module(mod) ||
    is_core_symbol(src + i, info.sechdrs, info.hdr.e_shnum,
    info.index.pcpu)) {
    let mut ret = 0;
    mod.core_kallsyms.typetab[ndst] =
    kallsyms.typetab[i];
    dst[ndst] = src[i];
    dst[ndst++].st_name = s - mod.core_kallsyms.strtab;
    ret = strscpy(s, &kallsyms.strtab[src[i].st_name],
    strtab_size);
    if (ret < 0) {
    break;
    }
    s += ret + 1;
    strtab_size -= ret + 1;
    }
    }
// Set up to point into init section.
    rcu_assign_pointer(mod.kallsyms, kallsyms);
    mod.core_kallsyms.num_symtab = ndst;
    }

#[no_mangle]
pub unsafe extern "C" fn init_build_id(mod: *mut module, info: *const load_info) {
pub static mut sechdr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < info.hdr.e_shnum) {
    sechdr = &info.sechdrs[i];
    if (!sect_empty(sechdr) && sechdr.sh_type == SHT_NOTE &&
    !build_id_parse_buf(sechdr.sh_addr, mod.build_id,
    sechdr.sh_size)) {
    break;
    }
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: init_build_id
pub unsafe extern "C" fn init_build_id_dup(mod: *mut module, info: *const load_info) {
    }

    static const char *kallsyms_symbol_name(mod_kallsyms *kallsyms, unsigned int symnum)
    {
    return kallsyms.strtab + kallsyms.symtab[symnum].st_name;
    }
//
// Given a module and address, find the corresponding symbol and return its name
// while providing its size and offset if needed.
//
    static const char *find_kallsyms_symbol(module *mod,
    unsigned long addr,
    unsigned long *size,
    unsigned long *offset)
    {
    unsigned int i, best = 0;
    unsigned long nextval, bestval;
    let mut kallsyms = rcu_dereference(mod.kallsyms);
    let mut mod_mem = core::ptr::null_mut();
    for_each_mod_mem_type(type) {

    if (!mod_mem_type_is_text(type)) {
    continue;
    }

    if (within_module_mem_type(addr, mod, type)) {
    mod_mem = &mod.mem[type];
    break;
    }
    }
    if (!mod_mem) {
    return core::ptr::null_mut();
    }
// Initialize bounds within memory region the address belongs to.
    nextval = (unsigned long)mod_mem.base + mod_mem.size;
    bestval = (unsigned long)mod_mem.base - 1;
//
// Scan for closest preceding symbol, and next symbol. (ELF
// starts real symbols at 1).
//
    while (i < kallsyms.num_symtab) {
    let mut sym = &kallsyms.symtab[i];
pub static mut thisval: c_ulong = 0;
    if (sym.st_shndx == SHN_UNDEF) {
    continue;
    }
//
// We ignore unnamed symbols: they're uninformative
// and inserted at a whim.
//
    if (*kallsyms_symbol_name(kallsyms, i) == '\0' ||
    is_mapping_symbol(kallsyms_symbol_name(kallsyms, i))) {
    continue;
    }
    if (thisval <= addr && thisval > bestval) {
    best = i;
    bestval = thisval;
    }
    if (thisval > addr && thisval < nextval) {
    nextval = thisval;
    }
    }
    if (!best) {
    return core::ptr::null_mut();
    }
    if (size) {
// size = nextval - bestval;
    }
    if (offset) {
// offset = addr - bestval;
    }
    return kallsyms_symbol_name(kallsyms, best);
    }
    void * __weak dereference_module_function_descriptor(module *mod,
    void *ptr)
    {
    return ptr;
    }
//
// For kallsyms to ask for address resolution.  NULL means not found.  Careful
// not to lock to avoid deadlock on oopses, RCU is enough.
//
    int module_address_lookup!(unsigned long addr,
    unsigned long *size,
    unsigned long *offset,
    char **modname,
    const unsigned char **modbuildid,
    char *namebuf)
    {
pub static mut sym: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    mod = __module_address(addr);
    if (mod) {
    if (modname) {
// modname = mod->name;
    }
    if (modbuildid) {
// modbuildid = module_buildid!(mod);
    }
    sym = find_kallsyms_symbol(mod, addr, size, offset);
    if (sym) {
    ret = strscpy(namebuf, sym, KSYM_NAME_LEN);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_module_symbol_name(addr: c_ulong, symname: *mut c_char) -> c_int {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    list_for_each_entry_rcu(mod, &modules, list) {
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (within_module(addr, mod)) {
pub static mut sym: *mut c_void = core::ptr::null_mut();
    sym = find_kallsyms_symbol(mod, addr, core::ptr::null_mut(), core::ptr::null_mut());
    if (!sym) {
// goto;
    }
    strscpy(symname, sym, KSYM_NAME_LEN);
    return 0;
    }
    }
// label;
    return -ERANGE;
    }
    int module_get_kallsym!(unsigned int symnum, unsigned long *value, char *type,
    char *name, char *module_name, int *exported)
    {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    list_for_each_entry_rcu(mod, &modules, list) {
pub static mut kallsyms: *mut c_void = core::ptr::null_mut();
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    kallsyms = rcu_dereference(mod.kallsyms);
    if (symnum < kallsyms.num_symtab) {
    let mut sym = &kallsyms.symtab[symnum];
// value = kallsyms_symbol_value(sym);
// type = kallsyms->typetab[symnum];
    strscpy(name, kallsyms_symbol_name(kallsyms, symnum), KSYM_NAME_LEN);
    strscpy(module_name, mod.name, MODULE_NAME_LEN);
// exported = is_exported(name, *value, mod);
    return 0;
    }
    symnum -= kallsyms.num_symtab;
    }
    return -ERANGE;
    }
// Given a module and name of symbol, find and return the symbol's value
#[no_mangle]
unsafe extern "C" fn __find_kallsyms_symbol_value(mod: *mut module, name: *const c_char) -> c_ulong {
    let mut i = 0;
    let mut kallsyms = rcu_dereference(mod.kallsyms);
    while (i < kallsyms.num_symtab) {
    let mut sym = &kallsyms.symtab[i];
    if (strcmp(name, kallsyms_symbol_name(kallsyms, i)) == 0 &&
    sym.st_shndx != SHN_UNDEF) {
    return kallsyms_symbol_value(sym);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __module_kallsyms_lookup_name(name: *const c_char) -> c_ulong {
pub static mut mod: *mut c_void = core::ptr::null_mut();
pub static mut colon: *mut c_void = core::ptr::null_mut();
    colon = strnchr(name, MODULE_NAME_LEN, ':');
    if (colon) {
    mod = find_module_all(name, colon - name, false);
    if (mod) {
    return __find_kallsyms_symbol_value(mod, colon + 1);
    }
    return 0;
    }
    list_for_each_entry_rcu(mod, &modules, list) {
    let mut ret = 0;
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    ret = __find_kallsyms_symbol_value(mod, name);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
// Look for this name: can be of form module:name.
#[no_mangle]
pub unsafe extern "C" fn module_kallsyms_lookup_name!(name: *const c_char) -> c_ulong {
// Don't lock: we're in enough trouble already.
    guard(rcu)();
    return __module_kallsyms_lookup_name(name);
    }
#[no_mangle]
pub unsafe extern "C" fn find_kallsyms_symbol_value(mod: *mut module, name: *const c_char) -> c_ulong {
    guard(rcu)();
    return __find_kallsyms_symbol_value(mod, name);
    }
    int module_kallsyms_on_each_symbol!(const char *modname,
    int (*fn)(void *, const char *, unsigned long),
    void *data)
    {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut ret: c_int = 0;
    mutex_lock(&module_mutex);
    list_for_each_entry(mod, &modules, list) {
pub static mut kallsyms: *mut c_void = core::ptr::null_mut();
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (modname && strcmp(modname, mod.name)) {
    continue;
    }
    kallsyms = rcu_dereference_check(mod.kallsyms,
    lockdep_is_held(&module_mutex));
    while (i < kallsyms.num_symtab) {
    let mut sym = &kallsyms.symtab[i];
    if (sym.st_shndx == SHN_UNDEF) {
    continue;
    }
    ret = fn(data, kallsyms_symbol_name(kallsyms, i),
    kallsyms_symbol_value(sym));
    if (ret != 0) {
// goto;
    }
    }
//
// The given module is found, the subsequent modules do not
// need to be compared.
//
    if (modname) {
    break;
    }
    }
// label;
    mutex_unlock(&module_mutex);
    return ret;
    }