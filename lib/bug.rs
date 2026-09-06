//! Automatically rewritten from C to Rust
//! Source: lib/bug.c
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
// Generic support for BUG()
//
// This respects the following config options:
//
// CONFIG_BUG - emit BUG traps.  Nothing happens without this.
// CONFIG_GENERIC_BUG - enable this code.
// CONFIG_GENERIC_BUG_RELATIVE_POINTERS - use 32-bit relative pointers for bug_addr and file
// CONFIG_DEBUG_BUGVERBOSE - emit full file+line information for each BUG
//
// CONFIG_BUG and CONFIG_DEBUG_BUGVERBOSE are potentially user-settable
// (though they're generally always on).
//
// CONFIG_GENERIC_BUG is set by each architecture using this code.
//
// To use this, your architecture must:
//
// 1. Set up the config options:
// - Enable CONFIG_GENERIC_BUG if CONFIG_BUG
//
// 2. Implement BUG (and optionally BUG_ON, WARN, WARN_ON)
// - Define HAVE_ARCH_BUG
// - Implement BUG() to generate a faulting instruction
// - NOTE: struct bug_entry does not have "file" or "line" entries
// when CONFIG_DEBUG_BUGVERBOSE is not enabled, so you must generate
// the values accordingly.
//
// 3. Implement the trap
// - In the illegal instruction trap handler (typically), verify
// that the fault was in kernel mode, and call report_bug()
// - report_bug() will return whether it was a false alarm, a warning,
// or an actual bug.
// - You must implement the is_valid_bugaddr(bugaddr) callback which
// returns true if the eip is a real kernel address, and it points
// to the expected BUG trap instruction.
//
// Jeremy Fitzhardinge <jeremy@goop.org> 2006
//

    extern struct bug_entry __start___bug_table[], __stop___bug_table[];
#[no_mangle]
pub unsafe extern "C" fn bug_addr(bug: *const bug_entry) -> c_ulong {
    static inline unsigned long bug_addr(const struct bug_entry *bug)
    {

    return (unsigned long)&bug.bug_addr_disp + bug.bug_addr_disp;

    return bug.bug_addr;

    }

// Updates are protected by module mutex
    static LIST_HEAD(module_bug_list);
    static struct bug_entry *module_find_bug(unsigned long bugaddr)
    {
    struct bug_entry *bug;
    struct module *mod;
    guard(rcu)();
    list_for_each_entry_rcu(mod, &module_bug_list, bug_list) {
    unsigned int i;
    bug = mod.bug_table;
    for (i = 0; i < mod.num_bugs; ++i, ++bug)
    if (bugaddr == bug_addr(bug))
    return bug;
    }
    return core::ptr::null_mut();
    }
    void module_bug_finalize(const Elf_Ehdr *hdr, const Elf_Shdr *sechdrs,
    struct module *mod)
    {
    char *secstrings;
    unsigned int i;
    mod.bug_table = core::ptr::null_mut();
    mod.num_bugs = 0;
// Find the __bug_table section, if present
    secstrings = (char *)hdr + sechdrs[hdr.e_shstrndx].sh_offset;
    for (i = 1; i < hdr.e_shnum; i++) {
    if (strcmp(secstrings+sechdrs[i].sh_name, "__bug_table"))
    continue;
    mod.bug_table = (void *) sechdrs[i].sh_addr;
    mod.num_bugs = sechdrs[i].sh_size / sizeof(struct bug_entry);
    break;
    }
//
// Strictly speaking this should have a spinlock to protect against
// traversals, but since we only traverse on BUG()s, a spinlock
// could potentially lead to deadlock and thus be counter-productive.
// Thus, this uses RCU to safely manipulate the bug list, since BUG
// must run in non-interruptive state.
//
    list_add_rcu(&mod.bug_list, &module_bug_list);
    }
#[no_mangle]
pub unsafe extern "C" fn module_bug_cleanup(mod: *mut module) {
    void module_bug_cleanup(struct module *mod)
    {
    list_del_rcu(&mod.bug_list);
    }

    static inline struct bug_entry *module_find_bug(unsigned long bugaddr)
    {
    return core::ptr::null_mut();
    }

    void bug_get_file_line(struct bug_entry *bug, const char **file,
    unsigned int *line)
    {

// file = (const char *)&bug->file_disp + bug->file_disp;

// file = bug->file;

// line = bug->line;

// file = NULL;
// line = 0;

    }
    static const char *bug_get_format(struct bug_entry *bug)
    {
    const char *format = core::ptr::null_mut();

//
// Allow an architecture to:
// - relative encode NULL (difficult vs KASLR);
// - use a literal 0 (there are no valid objects inside
// the __bug_table itself to refer to after all);
// - use an empty string.
//
    if (bug.format_disp)
    format = (const char *)&bug.format_disp + bug.format_disp;
    if (format && format[0] == '\0')
    format = core::ptr::null_mut();

    format = bug.format;

    return format;
    }
    struct bug_entry *find_bug(unsigned long bugaddr)
    {
    struct bug_entry *bug;
    for (bug = __start___bug_table; bug < __stop___bug_table; ++bug)
    if (bugaddr == bug_addr(bug))
    return bug;
    return module_find_bug(bugaddr);
    }
#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 1, _arg: 0) -> static {
    static __printf(1, 0)
#[no_mangle]
pub unsafe extern "C" fn __warn_printf(fmt: *const c_char, regs: *mut pt_regs) {
    void __warn_printf(const char *fmt, struct pt_regs *regs)
    {
    if (!fmt)
    return;

    if (regs) {
    struct arch_va_list _args;
    va_list *args = __warn_args(&_args, regs);
    if (args) {
    vprintk(fmt, *args);
    return;
    }
    }

    pr_warn("%s", fmt);
    }
#[no_mangle]
unsafe extern "C" fn __report_bug(bug: *mut bug_entry, bugaddr: c_ulong, regs: *mut pt_regs) -> enum bug_trap_type {
    static enum bug_trap_type __report_bug(struct bug_entry *bug, unsigned long bugaddr, struct pt_regs *regs)
    {
    bool warning, once, done, no_cut, has_args;
    const char *file, *fmt;
    unsigned int line;
    if (!bug) {
    if (!is_valid_bugaddr(bugaddr))
    return BUG_TRAP_TYPE_NONE;
    bug = find_bug(bugaddr);
    if (!bug)
    return BUG_TRAP_TYPE_NONE;
    }
    bug_get_file_line(bug, &file, &line);
    fmt = bug_get_format(bug);
    warning  = bug.flags & BUGFLAG_WARNING;
    once     = bug.flags & BUGFLAG_ONCE;
    done     = bug.flags & BUGFLAG_DONE;
    no_cut   = bug.flags & BUGFLAG_NO_CUT_HERE;
    has_args = bug.flags & BUGFLAG_ARGS;
//
// Before the once logic so suppressed warnings do not consume
// the single-fire budget of WARN_ON_ONCE().
//
    if (warning && kunit_is_suppressed_warning(true))
    return BUG_TRAP_TYPE_WARN;
    disable_trace_on_warning();
    if (warning && once) {
    if (done)
    return BUG_TRAP_TYPE_WARN;
//
// Since this is the only store, concurrency is not an issue.
//
    bug.flags |= BUGFLAG_DONE;
    }
//
// BUG() and WARN_ON() families don't print a custom debug message
// before triggering the exception handler, so we must add the
// "cut here" line now. WARN() issues its own "cut here" before the
// extra debugging message it writes before triggering the handler.
//
    if (!no_cut) {
    pr_info(CUT_HERE);
    __warn_printf(fmt, has_args ? regs : core::ptr::null_mut());
    }
    if (warning) {
// this is a WARN_ON rather than BUG/BUG_ON
    __warn(file, line, (void *)bugaddr, BUG_GET_TAINT(bug), regs,
    core::ptr::null_mut());
    return BUG_TRAP_TYPE_WARN;
    }
    if (file)
    pr_crit("kernel BUG at %s:%u!\n", file, line);
    else
    pr_crit("kernel BUG at %pB [verbose debug info unavailable]\n",
    (void *)bugaddr);
    return BUG_TRAP_TYPE_BUG;
    }
#[no_mangle]
pub unsafe extern "C" fn report_bug_entry(bug: *mut bug_entry, regs: *mut pt_regs) -> enum bug_trap_type {
    enum bug_trap_type report_bug_entry(struct bug_entry *bug, struct pt_regs *regs)
    {
    enum bug_trap_type ret;
    bool rcu;
    rcu = warn_rcu_enter();
    ret = __report_bug(bug, bug_addr(bug), regs);
    warn_rcu_exit(rcu);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn report_bug(bugaddr: c_ulong, regs: *mut pt_regs) -> enum bug_trap_type {
    enum bug_trap_type report_bug(unsigned long bugaddr, struct pt_regs *regs)
    {
    enum bug_trap_type ret;
    bool rcu;
    rcu = warn_rcu_enter();
    ret = __report_bug(core::ptr::null_mut(), bugaddr, regs);
    warn_rcu_exit(rcu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clear_once_table(start: *mut bug_entry, end: *mut bug_entry) {
    static void clear_once_table(struct bug_entry *start, struct bug_entry *end)
    {
    struct bug_entry *bug;
    for (bug = start; bug < end; bug++)
    bug.flags &= ~BUGFLAG_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn generic_bug_clear_once() {
    void generic_bug_clear_once(void)
    {

    struct module *mod;
    scoped_guard(rcu) {
    list_for_each_entry_rcu(mod, &module_bug_list, bug_list)
    clear_once_table(mod.bug_table,
    mod.bug_table + mod.num_bugs);
    }

    clear_once_table(__start___bug_table, __stop___bug_table);
    }
