//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_support.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Kernel Debugger Architecture Independent Support Functions
//
// Copyright (c) 1999-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
// 03/02/13    added new 2.5 kallsyms <xavier.bru@bull.net>
//

//
// kdbgetsymval - Return the address of the given symbol.
//
// Parameters:
// symname	Character string containing symbol name
// symtab  Structure to receive results
// Returns:
// 0	Symbol not found, symtab zero filled
// 1	Symbol mapped to module/symbol/section, data in symtab
//
#[no_mangle]
pub unsafe extern "C" fn kdbgetsymval(symname: *const c_char, symtab: *mut kdb_symtab_t) -> c_int {
    int kdbgetsymval(const char *symname, kdb_symtab_t *symtab)
    {
    kdb_dbg_printf(AR, "symname=%s, symtab=%px\n", symname, symtab);
    memset(symtab, 0, sizeof(*symtab));
    symtab.sym_start = kallsyms_lookup_name(symname);
    if (symtab.sym_start) {
    kdb_dbg_printf(AR, "returns 1, symtab.sym_start=0x%lx\n",
    symtab.sym_start);
    return 1;
    }
    kdb_dbg_printf(AR, "returns 0\n");
    return 0;
    }
    EXPORT_SYMBOL(kdbgetsymval);
//
// kdbnearsym() - Return the name of the symbol with the nearest address
// less than @addr.
// @addr: Address to check for near symbol
// @symtab: Structure to receive results
//
// WARNING: This function may return a pointer to a single statically
// allocated buffer (namebuf). kdb's unusual calling context (single
// threaded, all other CPUs halted) provides us sufficient locking for
// this to be safe. The only constraint imposed by the static buffer is
// that the caller must consume any previous reply prior to another call
// to lookup a new symbol.
//
// Note that, strictly speaking, some architectures may re-enter the kdb
// trap if the system turns out to be very badly damaged and this breaks
// the single-threaded assumption above. In these circumstances successful
// continuation and exit from the inner trap is unlikely to work and any
// user attempting this receives a prominent warning before being allowed
// to progress. In these circumstances we remain memory safe because
// namebuf[KSYM_NAME_LEN-1] will never change from '\0' although we do
// tolerate the possibility of garbled symbol display from the outer kdb
// trap.
//
// Return:
// * 0 - No sections contain this address, symtab zero filled
// * 1 - Address mapped to module/symbol/section, data in symtab
//
#[no_mangle]
pub unsafe extern "C" fn kdbnearsym(addr: c_ulong, symtab: *mut kdb_symtab_t) -> c_int {
    int kdbnearsym(unsigned long addr, kdb_symtab_t *symtab)
    {
    let mut ret: c_int = 0;
    let mut symbolsize: c_ulong = 0;
    let mut offset: c_ulong = 0;
    static char namebuf[KSYM_NAME_LEN];
    kdb_dbg_printf(AR, "addr=0x%lx, symtab=%px\n", addr, symtab);
    memset(symtab, 0, sizeof(*symtab));
    if (addr < 4096)
    goto out;
    symtab.sym_name = kallsyms_lookup(addr, &symbolsize , &offset,
    (char **)(&symtab.mod_name), namebuf);
    if (offset > 8*1024*1024) {
    symtab.sym_name = core::ptr::null_mut();
    addr = offset = symbolsize = 0;
    }
    symtab.sym_start = addr - offset;
    symtab.sym_end = symtab.sym_start + symbolsize;
    ret = symtab.sym_name != core::ptr::null_mut() && *(symtab.sym_name) != '\0';
    if (symtab.mod_name == core::ptr::null_mut())
    symtab.mod_name = "kernel";
    kdb_dbg_printf(AR, "returns %d symtab.sym_start=0x%lx, symtab.mod_name=%px, symtab.sym_name=%px (%s)\n",
    ret, symtab.sym_start, symtab.mod_name, symtab.sym_name, symtab.sym_name);
    out:
    return ret;
    }
    static char ks_namebuf[KSYM_NAME_LEN+1], ks_namebuf_prev[KSYM_NAME_LEN+1];
//
// kallsyms_symbol_complete
//
// Parameters:
// prefix_name	prefix of a symbol name to lookup
// max_len		maximum length that can be returned
// Returns:
// Number of symbols which match the given prefix.
// Notes:
// prefix_name is changed to contain the longest unique prefix that
// starts with this prefix (tab completion).
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_symbol_complete(prefix_name: *mut c_char, max_len: c_int) -> c_int {
    int kallsyms_symbol_complete(char *prefix_name, int max_len)
    {
    let mut pos: loff_t = 0;
    let mut prefix_len: c_int = strlen(prefix_name), prev_len = 0;
    int i, number = 0;
    const char *name;
    while ((name = kdb_walk_kallsyms(&pos))) {
    if (strncmp(name, prefix_name, prefix_len) == 0) {
    strscpy(ks_namebuf, name, sizeof(ks_namebuf));
// Work out the longest name that matches the prefix
    if (++number == 1) {
    prev_len = min_t(int, max_len-1,
    strlen(ks_namebuf));
    memcpy(ks_namebuf_prev, ks_namebuf, prev_len);
    ks_namebuf_prev[prev_len] = '\0';
    continue;
    }
    for (i = 0; i < prev_len; i++) {
    if (ks_namebuf[i] != ks_namebuf_prev[i]) {
    prev_len = i;
    ks_namebuf_prev[i] = '\0';
    break;
    }
    }
    }
    }
    if (prev_len > prefix_len)
    memcpy(prefix_name, ks_namebuf_prev, prev_len+1);
    return number;
    }
//
// kallsyms_symbol_next
//
// Parameters:
// prefix_name	prefix of a symbol name to lookup
// flag	0 means search from the head, 1 means continue search.
// buf_size	maximum length that can be written to prefix_name
// buffer
// Returns:
// 1 if a symbol matches the given prefix.
// 0 if no string found
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_symbol_next(prefix_name: *mut c_char, flag: c_int, buf_size: c_int) -> c_int {
    int kallsyms_symbol_next(char *prefix_name, int flag, int buf_size)
    {
    let mut prefix_len: c_int = strlen(prefix_name);
    static loff_t pos;
    const char *name;
    if (!flag)
    pos = 0;
    while ((name = kdb_walk_kallsyms(&pos))) {
    if (!strncmp(name, prefix_name, prefix_len))
    return strscpy(prefix_name, name, buf_size);
    }
    return 0;
    }
//
// kdb_symbol_print - Standard method for printing a symbol name and offset.
// Inputs:
// addr	Address to be printed.
// symtab	Address of symbol data, if NULL this routine does its
// own lookup.
// punc	Punctuation for string, bit field.
// Remarks:
// The string and its punctuation is only printed if the address
// is inside the kernel, except that the value is always printed
// when requested.
//
    void kdb_symbol_print(unsigned long addr, const kdb_symtab_t *symtab_p,
    unsigned int punc)
    {
    kdb_symtab_t symtab, *symtab_p2;
    if (symtab_p) {
    symtab_p2 = (kdb_symtab_t *)symtab_p;
    } else {
    symtab_p2 = &symtab;
    kdbnearsym(addr, symtab_p2);
    }
    if (!(symtab_p2.sym_name || (punc & KDB_SP_VALUE)))
    return;
    if (punc & KDB_SP_SPACEB)
    kdb_printf(" ");
    if (punc & KDB_SP_VALUE)
    kdb_printf(kdb_machreg_fmt0, addr);
    if (symtab_p2.sym_name) {
    if (punc & KDB_SP_VALUE)
    kdb_printf(" ");
    if (punc & KDB_SP_PAREN)
    kdb_printf("(");
    if (strcmp(symtab_p2.mod_name, "kernel"))
    kdb_printf("[%s]", symtab_p2.mod_name);
    kdb_printf("%s", symtab_p2.sym_name);
    if (addr != symtab_p2.sym_start)
    kdb_printf("+0x%lx", addr - symtab_p2.sym_start);
    if (punc & KDB_SP_SYMSIZE)
    kdb_printf("/0x%lx",
    symtab_p2.sym_end - symtab_p2.sym_start);
    if (punc & KDB_SP_PAREN)
    kdb_printf(")");
    }
    if (punc & KDB_SP_SPACEA)
    kdb_printf(" ");
    if (punc & KDB_SP_NEWLINE)
    kdb_printf("\n");
    }
//
// kdb_strdup - kdb equivalent of strdup, for disasm code.
// Inputs:
// str	The string to duplicate.
// type	Flags to kmalloc for the new string.
// Returns:
// Address of the new string, NULL if storage could not be allocated.
// Remarks:
// This is not in lib/string.c because it uses kmalloc which is not
// available when string.o is used in boot loaders.
//
    char *kdb_strdup(const char *str, gfp_t type)
    {
    let mut n: usize = strlen(str) + 1;
    char *s = kmalloc(n, type);
    if (!s)
    return core::ptr::null_mut();
    memcpy(s, str, n);
    return s;
    }
//
// kdb_strdup_dequote - same as kdb_strdup(), but trims surrounding quotes from
// the input string if present.
// Remarks:
// Quotes are only removed if there is both a leading and a trailing quote.
//
    char *kdb_strdup_dequote(const char *str, gfp_t type)
    {
    let mut len: usize = strlen(str);
    char *s;
    if (str[0] == '"' && len > 1 && str[len - 1] == '"') {
// trim both leading and trailing quotes
    str++;
    len -= 2;
    }
    len++; /* add space for NUL terminator */
    s = kmalloc(len, type);
    if (!s)
    return core::ptr::null_mut();
    memcpy(s, str, len - 1);
    s[len - 1] = '\0';
    return s;
    }
//
// kdb_getarea_size - Read an area of data.  The kdb equivalent of
// copy_from_user, with kdb messages for invalid addresses.
// Inputs:
// res	Pointer to the area to receive the result.
// addr	Address of the area to copy.
// size	Size of the area.
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_getarea_size(res: *mut c_void, addr: c_ulong, size: usize) -> c_int {
    int kdb_getarea_size(void *res, unsigned long addr, size_t size)
    {
    let mut ret: c_int = copy_from_kernel_nofault((char *)res, (char *)addr, size);
    if (ret) {
    if (!KDB_STATE(SUPPRESS)) {
    kdb_func_printf("Bad address 0x%lx\n", addr);
    KDB_STATE_SET(SUPPRESS);
    }
    ret = KDB_BADADDR;
    } else {
    KDB_STATE_CLEAR(SUPPRESS);
    }
    return ret;
    }
//
// kdb_putarea_size - Write an area of data.  The kdb equivalent of
// copy_to_user, with kdb messages for invalid addresses.
// Inputs:
// addr	Address of the area to write to.
// res	Pointer to the area holding the data.
// size	Size of the area.
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_putarea_size(addr: c_ulong, res: *mut c_void, size: usize) -> c_int {
    int kdb_putarea_size(unsigned long addr, void *res, size_t size)
    {
    let mut ret: c_int = copy_to_kernel_nofault((char *)addr, (char *)res, size);
    if (ret) {
    if (!KDB_STATE(SUPPRESS)) {
    kdb_func_printf("Bad address 0x%lx\n", addr);
    KDB_STATE_SET(SUPPRESS);
    }
    ret = KDB_BADADDR;
    } else {
    KDB_STATE_CLEAR(SUPPRESS);
    }
    return ret;
    }
//
// kdb_getphys - Read data from a physical address. Validate the
// address is in range, use kmap_local_page() to get data
// similar to kdb_getarea() - but for phys addresses
// Inputs:
// res	Pointer to the word to receive the result
// addr	Physical address of the area to copy
// size	Size of the area
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
unsafe extern "C" fn kdb_getphys(res: *mut c_void, addr: c_ulong, size: usize) -> c_int {
    static int kdb_getphys(void *res, unsigned long addr, size_t size)
    {
    unsigned long pfn;
    void *vaddr;
    struct page *page;
    pfn = (addr >> PAGE_SHIFT);
    if (!pfn_valid(pfn))
    return 1;
    page = pfn_to_page(pfn);
    vaddr = kmap_local_page(page);
    memcpy(res, vaddr + (addr & (PAGE_SIZE - 1)), size);
    kunmap_local(vaddr);
    return 0;
    }
//
// kdb_getphysword
// Inputs:
// word	Pointer to the word to receive the result.
// addr	Address of the area to copy.
// size	Size of the area.
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_getphysword(word: *mut c_ulong, addr: c_ulong, size: usize) -> c_int {
    int kdb_getphysword(unsigned long *word, unsigned long addr, size_t size)
    {
    int diag;
    __u8  w1;
    __u16 w2;
    __u32 w4;
    __u64 w8;
// word = 0;	/* Default value if addr or size is invalid
    switch (size) {
    case 1:
    diag = kdb_getphys(&w1, addr, sizeof(w1));
    if (!diag)
// word = w1;
    break;
    case 2:
    diag = kdb_getphys(&w2, addr, sizeof(w2));
    if (!diag)
// word = w2;
    break;
    case 4:
    diag = kdb_getphys(&w4, addr, sizeof(w4));
    if (!diag)
// word = w4;
    break;
    case 8:
    if (size <= sizeof(*word)) {
    diag = kdb_getphys(&w8, addr, sizeof(w8));
    if (!diag)
// word = w8;
    break;
    }
    fallthrough;
    default:
    diag = KDB_BADWIDTH;
    kdb_func_printf("bad width %zu\n", size);
    }
    return diag;
    }
//
// kdb_getword - Read a binary value.  Unlike kdb_getarea, this treats
// data as numbers.
// Inputs:
// word	Pointer to the word to receive the result.
// addr	Address of the area to copy.
// size	Size of the area.
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_getword(word: *mut c_ulong, addr: c_ulong, size: usize) -> c_int {
    int kdb_getword(unsigned long *word, unsigned long addr, size_t size)
    {
    int diag;
    __u8  w1;
    __u16 w2;
    __u32 w4;
    __u64 w8;
// word = 0;	/* Default value if addr or size is invalid
    switch (size) {
    case 1:
    diag = kdb_getarea(w1, addr);
    if (!diag)
// word = w1;
    break;
    case 2:
    diag = kdb_getarea(w2, addr);
    if (!diag)
// word = w2;
    break;
    case 4:
    diag = kdb_getarea(w4, addr);
    if (!diag)
// word = w4;
    break;
    case 8:
    if (size <= sizeof(*word)) {
    diag = kdb_getarea(w8, addr);
    if (!diag)
// word = w8;
    break;
    }
    fallthrough;
    default:
    diag = KDB_BADWIDTH;
    kdb_func_printf("bad width %zu\n", size);
    }
    return diag;
    }
//
// kdb_putword - Write a binary value.  Unlike kdb_putarea, this
// treats data as numbers.
// Inputs:
// addr	Address of the area to write to..
// word	The value to set.
// size	Size of the area.
// Returns:
// 0 for success, < 0 for error.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_putword(addr: c_ulong, word: c_ulong, size: usize) -> c_int {
    int kdb_putword(unsigned long addr, unsigned long word, size_t size)
    {
    int diag;
    __u8  w1;
    __u16 w2;
    __u32 w4;
    __u64 w8;
    switch (size) {
    case 1:
    w1 = word;
    diag = kdb_putarea(addr, w1);
    break;
    case 2:
    w2 = word;
    diag = kdb_putarea(addr, w2);
    break;
    case 4:
    w4 = word;
    diag = kdb_putarea(addr, w4);
    break;
    case 8:
    if (size <= sizeof(word)) {
    w8 = word;
    diag = kdb_putarea(addr, w8);
    break;
    }
    fallthrough;
    default:
    diag = KDB_BADWIDTH;
    kdb_func_printf("bad width %zu\n", size);
    }
    return diag;
    }
//
// kdb_task_state_char - Return the character that represents the task state.
// Inputs:
// p	struct task for the process
// Returns:
// One character to represent the task state.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_task_state_char(p: *const task_struct) -> c_char {
    char kdb_task_state_char (const struct task_struct *p)
    {
    unsigned long tmp;
    char state;
    int cpu;
    if (!p ||
    copy_from_kernel_nofault(&tmp, (char *)p, sizeof(unsigned long)))
    return 'E';
    state = task_state_to_char((struct task_struct *) p);
    if (is_idle_task(p)) {
// Idle task.  Is it really idle, apart from the kdb
// interrupt?
    cpu = kdb_process_cpu(p);
    if (!kdb_task_has_cpu(p) || kgdb_info[cpu].irq_depth == 1) {
    if (cpu != kdb_initial_cpu)
    state = '-';	/* idle task */
    }
    } else if (!p.mm && strchr("IMS", state)) {
    state = tolower(state);		/* sleeping system daemon */
    }
    return state;
    }
//
// kdb_task_state - Return true if a process has the desired state
// given by the mask.
// Inputs:
// p	struct task for the process
// mask	set of characters used to select processes; both NULL
// and the empty string mean adopt a default filter, which
// is to suppress sleeping system daemons and the idle tasks
// Returns:
// True if the process matches at least one criteria defined by the mask.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_task_state(p: *const task_struct, mask: *const c_char) -> bool {
    bool kdb_task_state(const struct task_struct *p, const char *mask)
    {
    let mut state: c_char = kdb_task_state_char(p);
// If there is no mask, then we will filter code that runs when the
// scheduler is idling and any system daemons that are currently
// sleeping.
//
    if (!mask || mask[0] == '\0')
    return !strchr("-ims", state);
// A is a special case that matches all states
    if (strchr(mask, 'A'))
    return true;
    return strchr(mask, state);
    }
