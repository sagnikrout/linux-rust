//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/callthunks.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static int __initdata_or_module debug_callthunks;

    do {								\
    if (debug_callthunks)					\
    printk(KERN_DEBUG pr_fmt(fmt), ##args);		\
    } while(0)
#[no_mangle]
unsafe extern "C" fn debug_thunks(str: *mut c_char) -> int __init {
    static int __init debug_thunks(char *str)
    {
    debug_callthunks = 1;
    return 1;
    }
    __setup("debug-callthunks", debug_thunks);

    DEFINE_PER_CPU(u64, __x86_call_count);
    DEFINE_PER_CPU(u64, __x86_ret_count);
    DEFINE_PER_CPU(u64, __x86_stuffs_count);
    DEFINE_PER_CPU(u64, __x86_ctxsw_count);
    EXPORT_PER_CPU_SYMBOL_GPL(__x86_ctxsw_count);
    EXPORT_PER_CPU_SYMBOL_GPL(__x86_call_count);

    extern s32 __call_sites[], __call_sites_end[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_text {
    pub base: c_ulong,
    pub end: c_ulong,
    pub name: *const c_char,
}

    static bool thunks_initialized __ro_after_init;
    static const struct core_text builtin_coretext = {
    .base = (unsigned long)_text,
    .end  = (unsigned long)_etext,
    .name = "builtin",
    };
    asm (
    ".pushsection .rodata				\n"
    ".global skl_call_thunk_template		\n"
    "skl_call_thunk_template:			\n"
    __stringify(INCREMENT_CALL_DEPTH)"	\n"
    ".global skl_call_thunk_tail			\n"
    "skl_call_thunk_tail:				\n"
    ".popsection					\n"
    );
    extern u8 skl_call_thunk_template[];
    extern u8 skl_call_thunk_tail[];

    ((unsigned int)(skl_call_thunk_tail - skl_call_thunk_template))
    extern void error_entry(void);
    extern void xen_error_entry(void);
    extern void paranoid_entry(void);
#[no_mangle]
pub unsafe extern "C" fn within_coretext(ct: *const core_text, addr: *mut c_void) -> bool {
    static inline bool within_coretext(const struct core_text *ct, void *addr)
    {
    let mut p: c_ulong = (unsigned long)addr;
    return ct.base <= p && p < ct.end;
    }
#[no_mangle]
pub unsafe extern "C" fn within_module_coretext(addr: *mut c_void) -> bool {
    static inline bool within_module_coretext(void *addr)
    {
    let mut ret: bool = false;

    struct module *mod;
    guard(rcu)();
    mod = __module_address((unsigned long)addr);
    if (mod && within_module_core((unsigned long)addr, mod))
    ret = true;

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn is_coretext(ct: *const core_text, addr: *mut c_void) -> bool {
    static bool is_coretext(const struct core_text *ct, void *addr)
    {
    if (ct && within_coretext(ct, addr))
    return true;
    if (within_coretext(&builtin_coretext, addr))
    return true;
    return within_module_coretext(addr);
    }
#[no_mangle]
unsafe extern "C" fn skip_addr(dest: *mut c_void) -> bool {
    static bool skip_addr(void *dest)
    {
    if (dest == error_entry)
    return true;
    if (dest == paranoid_entry)
    return true;
    if (dest == xen_error_entry)
    return true;
// Does FILL_RSB...
    if (dest == __switch_to_asm)
    return true;
// Accounts directly
    if (dest == ret_from_fork)
    return true;

    if (dest == soft_restart_cpu)
    return true;

    if (dest == __fentry__)
    return true;

    if (dest >= (void *)__relocate_kernel_start &&
    dest < (void *)__relocate_kernel_end)
    return true;

    if (dest >= (void *)relocate_kernel &&
    dest < (void*)relocate_kernel + KEXEC_CONTROL_CODE_MAX_SIZE)
    return true;

    return false;
    }
    static __init_or_module void *call_get_dest(void *addr)
    {
    struct insn insn;
    void *dest;
    int ret;
    ret = insn_decode_kernel(&insn, addr);
    if (ret)
    return ERR_PTR(ret);
// Patched out call?
    if (insn.opcode.bytes[0] != CALL_INSN_OPCODE)
    return core::ptr::null_mut();
    dest = addr + insn.length + insn.immediate.value;
    if (skip_addr(dest))
    return core::ptr::null_mut();
    return dest;
    }
    static const u8 nops[] = {
    0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    };
    static void *patch_dest(void *dest, bool direct)
    {
    let mut tsize: c_uint = SKL_TMPL_SIZE;
    u8 insn_buff[MAX_PATCH_LEN];
    u8 *pad = dest - tsize;
    memcpy(insn_buff, skl_call_thunk_template, tsize);
    text_poke_apply_relocation(insn_buff, pad, tsize, skl_call_thunk_template, tsize);
// Already patched?
    if (!bcmp(pad, insn_buff, tsize))
    return pad;
// Ensure there are nops
    if (bcmp(pad, nops, tsize)) {
    pr_warn_once("Invalid padding area for %pS\n", dest);
    return core::ptr::null_mut();
    }
    if (direct)
    memcpy(pad, insn_buff, tsize);
    else
    text_poke_copy_locked(pad, insn_buff, tsize, true);
    return pad;
    }
#[no_mangle]
unsafe extern "C" fn patch_call(addr: *mut c_void, ct: *const core_text) -> __init_or_module void {
    static __init_or_module void patch_call(void *addr, const struct core_text *ct)
    {
    void *pad, *dest;
    u8 bytes[8];
    if (!within_coretext(ct, addr))
    return;
    dest = call_get_dest(addr);
    if (!dest || WARN_ON_ONCE(IS_ERR(dest)))
    return;
    if (!is_coretext(ct, dest))
    return;
    pad = patch_dest(dest, within_coretext(ct, dest));
    if (!pad)
    return;
    prdbg("Patch call at: %pS %px to %pS %px . %px \n", addr, addr,
    dest, dest, pad);
    __text_gen_insn(bytes, CALL_INSN_OPCODE, addr, pad, CALL_INSN_SIZE);
    text_poke_early(addr, bytes, CALL_INSN_SIZE);
    }
    static __init_or_module void
    patch_call_sites(s32 *start, s32 *end, const struct core_text *ct)
    {
    s32 *s;
    for (s = start; s < end; s++)
    patch_call((void *)s + *s, ct);
    }
    static __init_or_module void
    callthunks_setup(struct callthunk_sites *cs, const struct core_text *ct)
    {
    prdbg("Patching call sites %s\n", ct.name);
    patch_call_sites(cs.call_start, cs.call_end, ct);
    prdbg("Patching call sites done%s\n", ct.name);
    }
#[no_mangle]
pub unsafe extern "C" fn callthunks_patch_builtin_calls() -> void __init {
    void __init callthunks_patch_builtin_calls(void)
    {
    struct callthunk_sites cs = {
    .call_start	= __call_sites,
    .call_end	= __call_sites_end,
    };
    if (!cpu_feature_enabled(X86_FEATURE_CALL_DEPTH))
    return;
    pr_info("Setting up call depth tracking\n");
    mutex_lock(&text_mutex);
    callthunks_setup(&cs, &builtin_coretext);
    thunks_initialized = true;
    mutex_unlock(&text_mutex);
    }
    void *callthunks_translate_call_dest(void *dest)
    {
    void *target;
    lockdep_assert_held(&text_mutex);
    if (!thunks_initialized || skip_addr(dest))
    return dest;
    if (!is_coretext(core::ptr::null_mut(), dest))
    return dest;
    target = patch_dest(dest, false);
    return target ? : dest;
    }

#[no_mangle]
unsafe extern "C" fn is_callthunk(addr: *mut c_void) -> bool {
    static bool is_callthunk(void *addr)
    {
    let mut tmpl_size: c_uint = SKL_TMPL_SIZE;
    u8 insn_buff[MAX_PATCH_LEN];
    unsigned long dest;
    u8 *pad;
    dest = roundup((unsigned long)addr, CONFIG_FUNCTION_ALIGNMENT);
    if (!thunks_initialized || skip_addr((void *)dest))
    return false;
    pad = (void *)(dest - tmpl_size);
    memcpy(insn_buff, skl_call_thunk_template, tmpl_size);
    text_poke_apply_relocation(insn_buff, pad, tmpl_size, skl_call_thunk_template, tmpl_size);
    return !bcmp(pad, insn_buff, tmpl_size);
    }
#[no_mangle]
pub unsafe extern "C" fn x86_call_depth_emit_accounting(pprog: *mut u8, func: *mut c_void, ip: *mut c_void) -> c_int {
    int x86_call_depth_emit_accounting(u8 **pprog, void *func, void *ip)
    {
    let mut tmpl_size: c_uint = SKL_TMPL_SIZE;
    u8 insn_buff[MAX_PATCH_LEN];
    if (!thunks_initialized)
    return 0;
// Is function call target a thunk?
    if (func && is_callthunk(func))
    return 0;
    memcpy(insn_buff, skl_call_thunk_template, tmpl_size);
    text_poke_apply_relocation(insn_buff, ip, tmpl_size, skl_call_thunk_template, tmpl_size);
    memcpy(*pprog, insn_buff, tmpl_size);
// pprog += tmpl_size;
    return tmpl_size;
    }

    void noinline callthunks_patch_module_calls(struct callthunk_sites *cs,
    struct module *mod)
    {
    struct core_text ct = {
    .base = (unsigned long)mod.mem[MOD_TEXT].base,
    .end  = (unsigned long)mod.mem[MOD_TEXT].base + mod.mem[MOD_TEXT].size,
    .name = mod.name,
    };
    if (!thunks_initialized)
    return;
    mutex_lock(&text_mutex);
    callthunks_setup(cs, &ct);
    mutex_unlock(&text_mutex);
    }

#[no_mangle]
unsafe extern "C" fn callthunks_debug_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int callthunks_debug_show(struct seq_file *m, void *p)
    {
    let mut cpu: c_ulong = (unsigned long)m.private;
    seq_printf(m, "C: %16llu R: %16llu S: %16llu X: %16llu\n,",
    per_cpu(__x86_call_count, cpu),
    per_cpu(__x86_ret_count, cpu),
    per_cpu(__x86_stuffs_count, cpu),
    per_cpu(__x86_ctxsw_count, cpu));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn callthunks_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    static int callthunks_debug_open(struct inode *inode, struct file *file)
    {
    return single_open(file, callthunks_debug_show, inode.i_private);
    }
    static const struct file_operations dfs_ops = {
    .open		= callthunks_debug_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn callthunks_debugfs_init() -> int __init {
    static int __init callthunks_debugfs_init(void)
    {
    struct dentry *dir;
    unsigned long cpu;
    dir = debugfs_create_dir("callthunks", core::ptr::null_mut());
    for_each_possible_cpu(cpu) {
    void *arg = (void *)cpu;
    char name [10];
    sprintf(name, "cpu%lu", cpu);
    debugfs_create_file(name, 0644, dir, arg, &dfs_ops);
    }
    return 0;
    }
    __initcall(callthunks_debugfs_init);
