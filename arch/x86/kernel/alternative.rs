//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/alternative.c
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

    int __read_mostly alternatives_patched;
    EXPORT_SYMBOL_GPL(alternatives_patched);

pub const DA_ALT: c_uint = 0x01;
pub const DA_RET: c_uint = 0x02;
pub const DA_RETPOLINE: c_uint = 0x04;
pub const DA_ENDBR: c_uint = 0x08;
pub const DA_SMP: c_uint = 0x10;
    static unsigned int debug_alternative;
#[no_mangle]
unsafe extern "C" fn debug_alt(str: *mut c_char) -> int __init {
    static int __init debug_alt(char *str)
    {
    if (str && *str == '=')
    str++;
    if (!str || kstrtouint(str, 0, &debug_alternative))
    debug_alternative = DA_ALL;
    return 1;
    }
    __setup("debug-alternative", debug_alt);

    do {									\
    if (debug_alternative & DA_##type)				\
    printk(KERN_DEBUG pr_fmt(fmt) "\n", ##args);		\
    } while (0)

    do {									\
    if (unlikely(debug_alternative & DA_##type)) {			\
    int j;							\
    \
    if (!(len))						\
    break;						\
    \
    printk(KERN_DEBUG pr_fmt(fmt), ##args);			\
    for (j = 0; j < (len) - 1; j++)				\
    printk(KERN_CONT "%02hhx ", buf[j]);		\
    printk(KERN_CONT "%02hhx\n", buf[j]);			\
    }								\
    } while (0)
    static const unsigned char x86nops[] =
    {
    BYTES_NOP1,
    BYTES_NOP2,
    BYTES_NOP3,
    BYTES_NOP4,
    BYTES_NOP5,
    BYTES_NOP6,
    BYTES_NOP7,
    BYTES_NOP8,

    BYTES_NOP9,
    BYTES_NOP10,
    BYTES_NOP11,

    };
    const unsigned char * const x86_nops[ASM_NOP_MAX+1] =
    {
    core::ptr::null_mut(),
    x86nops,
    x86nops + 1,
    x86nops + 1 + 2,
    x86nops + 1 + 2 + 3,
    x86nops + 1 + 2 + 3 + 4,
    x86nops + 1 + 2 + 3 + 4 + 5,
    x86nops + 1 + 2 + 3 + 4 + 5 + 6,
    x86nops + 1 + 2 + 3 + 4 + 5 + 6 + 7,

    x86nops + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8,
    x86nops + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9,
    x86nops + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10,

    };

    static bool cfi_paranoid __ro_after_init;

    static struct module *its_mod;

    static void *its_page;
    static unsigned int its_offset;
    struct its_array its_pages;
    static void *__its_alloc(struct its_array *pages)
    {
    void *page __free(execmem) = execmem_alloc_rw(EXECMEM_MODULE_TEXT, PAGE_SIZE);
    if (!page)
    return core::ptr::null_mut();
    void *tmp = krealloc(pages.pages, (pages.num+1) * sizeof(void *),
    GFP_KERNEL);
    if (!tmp)
    return core::ptr::null_mut();
    pages.pages = tmp;
    pages.pages[pages.num++] = page;
    return no_free_ptr(page);
    }
// Initialize a thunk with the "jmp *reg; int3" instructions.
    static void *its_init_thunk(void *thunk, int reg)
    {
    u8 *bytes = thunk;
    let mut offset: c_int = 0;
    let mut i: c_int = 0;

    if (cfi_paranoid) {
//
// When ITS uses indirect branch thunk the fineibt_paranoid
// caller sequence doesn't fit in the caller site. So put the
// remaining part of the sequence (UDB + JNE) into the ITS
// thunk.
//
    bytes[i++] = 0xd6; /* UDB */
    bytes[i++] = 0x75; /* JNE */
    bytes[i++] = 0xfd;
    offset = 1;
    }

    if (reg >= 8) {
    bytes[i++] = 0x41; /* REX.B prefix */
    reg -= 8;
    }
    bytes[i++] = 0xff;
    bytes[i++] = 0xe0 + reg; /* JMP *reg */
    bytes[i++] = 0xcc;
    return thunk + offset;
    }
#[no_mangle]
unsafe extern "C" fn its_pages_protect(pages: *mut its_array) {
    static void its_pages_protect(struct its_array *pages)
    {
    for (int i = 0; i < pages.num; i++) {
    void *page = pages.pages[i];
    execmem_restore_rox(page, PAGE_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn its_fini_core() {
    static void its_fini_core(void)
    {
    if (IS_ENABLED(CONFIG_STRICT_KERNEL_RWX))
    its_pages_protect(&its_pages);
    kfree(its_pages.pages);
    }

#[no_mangle]
pub unsafe extern "C" fn its_init_mod(mod: *mut module) {
    void its_init_mod(struct module *mod)
    {
    if (!cpu_feature_enabled(X86_FEATURE_INDIRECT_THUNK_ITS))
    return;
    mutex_lock(&text_mutex);
    its_mod = mod;
    its_page = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn its_fini_mod(mod: *mut module) {
    void its_fini_mod(struct module *mod)
    {
    if (!cpu_feature_enabled(X86_FEATURE_INDIRECT_THUNK_ITS))
    return;
    WARN_ON_ONCE(its_mod != mod);
    its_mod = core::ptr::null_mut();
    its_page = core::ptr::null_mut();
    mutex_unlock(&text_mutex);
    if (IS_ENABLED(CONFIG_STRICT_MODULE_RWX))
    its_pages_protect(&mod.arch.its_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn its_free_mod(mod: *mut module) {
    void its_free_mod(struct module *mod)
    {
    if (!cpu_feature_enabled(X86_FEATURE_INDIRECT_THUNK_ITS))
    return;
    for (int i = 0; i < mod.arch.its_pages.num; i++) {
    void *page = mod.arch.its_pages.pages[i];
    execmem_free(page);
    }
    kfree(mod.arch.its_pages.pages);
    }

    static void *its_alloc(void)
    {
    struct its_array *pages = &its_pages;
    void *page;

    if (its_mod)
    pages = &its_mod.arch.its_pages;

    page = __its_alloc(pages);
    if (!page)
    return core::ptr::null_mut();
    if (pages == &its_pages)
    set_memory_x((unsigned long)page, 1);
    return page;
    }
    static void *its_allocate_thunk(int reg)
    {
    let mut size: c_int = 3 + (reg / 8);
    void *thunk;

//
// The ITS thunk contains an indirect jump and an int3 instruction so
// its size is 3 or 4 bytes depending on the register used. If CFI
// paranoid is used then 3 extra bytes are added in the ITS thunk to
// complete the fineibt_paranoid caller sequence.
//
    if (cfi_paranoid)
    size += 3;

    if (!its_page || (its_offset + size - 1) >= PAGE_SIZE) {
    its_page = its_alloc();
    if (!its_page) {
    pr_err("ITS page allocation failed\n");
    return core::ptr::null_mut();
    }
    memset(its_page, INT3_INSN_OPCODE, PAGE_SIZE);
    its_offset = 32;
    }
//
// If the indirect branch instruction will be in the lower half
// of a cacheline, then update the offset to reach the upper half.
//
    if ((its_offset + size - 1) % 64 < 32)
    its_offset = ((its_offset - 1) | 0x3F) + 33;
    thunk = its_page + its_offset;
    its_offset += size;
    return its_init_thunk(thunk, reg);
    }
    u8 *its_static_thunk(int reg)
    {
    u8 *thunk = __x86_indirect_its_thunk_array[reg];

// Paranoid thunk starts 2 bytes before
    if (cfi_paranoid)
    return thunk - 2;

    return thunk;
    }

    static inline void its_fini_core(void) {}

//
// Nomenclature for variable names to simplify and clarify this code and ease
// any potential staring at it:
//
// @instr: source address of the original instructions in the kernel text as
// generated by the compiler.
//
// @buf: temporary buffer on which the patching operates. This buffer is
// eventually text-poked into the kernel image.
//
// @replacement/@repl: pointer to the opcodes which are replacing @instr, located
// in the .altinstr_replacement section.
//
// Fill the buffer with a single effective instruction of size @len.
//
// In order not to issue an ORC stack depth tracking CFI entry (Call Frame Info)
// for every single-byte NOP, try to generate the maximally available NOP of
// size <= ASM_NOP_MAX such that only a single CFI entry is generated (vs one for
// each single-byte NOPs). If @len to fill out is > ASM_NOP_MAX, pad with INT3 and
// *jump* over instead of executing long and daft NOPs.
//
#[no_mangle]
unsafe extern "C" fn add_nop(buf: *mut u8, len: c_uint) {
    static void add_nop(u8 *buf, unsigned int len)
    {
    u8 *target = buf + len;
    if (!len)
    return;
    if (len <= ASM_NOP_MAX) {
    memcpy(buf, x86_nops[len], len);
    return;
    }
    if (len < 128) {
    __text_gen_insn(buf, JMP8_INSN_OPCODE, buf, target, JMP8_INSN_SIZE);
    buf += JMP8_INSN_SIZE;
    } else {
    __text_gen_insn(buf, JMP32_INSN_OPCODE, buf, target, JMP32_INSN_SIZE);
    buf += JMP32_INSN_SIZE;
    }
    for (;buf < target; buf++)
// buf = INT3_INSN_OPCODE;
    }
//
// Find the offset of the first non-NOP instruction starting at @offset
// but no further than @len.
//
#[no_mangle]
unsafe extern "C" fn skip_nops(buf: *mut u8, offset: c_int, len: c_int) -> c_int {
    static int skip_nops(u8 *buf, int offset, int len)
    {
    struct insn insn;
    for (; offset < len; offset += insn.length) {
    if (insn_decode_kernel(&insn, &buf[offset]))
    break;
    if (!insn_is_nop(&insn))
    break;
    }
    return offset;
    }
//
// "noinline" to cause control flow change and thus invalidate I$ and
// cause refetch after modification.
//
#[no_mangle]
unsafe extern "C" fn optimize_nops(instr: *const *const u8, buf: *mut u8, len: usize) -> void noinline {
    static void noinline optimize_nops(const u8 * const instr, u8 *buf, size_t len)
    {
    for (int next, i = 0; i < len; i = next) {
    struct insn insn;
    if (insn_decode_kernel(&insn, &buf[i]))
    return;
    next = i + insn.length;
    if (insn_is_nop(&insn)) {
    let mut nop: c_int = i;
// Has the NOP already been optimized?
    if (i + insn.length == len)
    return;
    next = skip_nops(buf, next, len);
    add_nop(buf + nop, next - nop);
    DUMP_BYTES(ALT, buf, len, "%px: [%d:%d) optimized NOPs: ", instr, nop, next);
    }
    }
    }
//
// In this context, "source" is where the instructions are placed in the
// section .altinstr_replacement, for example during kernel build by the
// toolchain.
// "Destination" is where the instructions are being patched in by this
// machinery.
//
// The source offset is:
//
// src_imm = target - src_next_ip                  (1)
//
// and the target offset is:
//
// dst_imm = target - dst_next_ip                  (2)
//
// so rework (1) as an expression for target like:
//
// target = src_imm + src_next_ip                  (1a)
//
// and substitute in (2) to get:
//
// dst_imm = (src_imm + src_next_ip) - dst_next_ip (3)
//
// Now, since the instruction stream is 'identical' at src and dst (it
// is being copied after all) it can be stated that:
//
// src_next_ip = src + ip_offset
// dst_next_ip = dst + ip_offset                   (4)
//
// Substitute (4) in (3) and observe ip_offset being cancelled out to
// obtain:
//
// dst_imm = src_imm + (src + ip_offset) - (dst + ip_offset)
// = src_imm + src - dst + ip_offset - ip_offset
// = src_imm + src - dst                   (5)
//
// IOW, only the relative displacement of the code block matters.
//

    do {							\
    s32 v = *(s##n_ *)(p_);				\
    v += (d_);					\
    BUG_ON((v >> 31) != (v >> (n_-1)));		\
// (s##n_ *)(p_) = (s##n_)v;			\
    } while (0)
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn apply_reloc(n: c_int, ptr: *mut c_void, diff: uintptr_t) {
    void apply_reloc(int n, void *ptr, uintptr_t diff)
    {
    switch (n) {
    case 1: apply_reloc_n(8, ptr, diff); break;
    case 2: apply_reloc_n(16, ptr, diff); break;
    case 4: apply_reloc_n(32, ptr, diff); break;
    default: BUG();
    }
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn need_reloc(offset: c_ulong, src: *mut u8, src_len: usize) -> bool {
    bool need_reloc(unsigned long offset, u8 *src, size_t src_len)
    {
    u8 *target = src + offset;
//
// If the target is inside the patched block, it's relative to the
// block itself and does not need relocation.
//
    return (target < src || target > src + src_len);
    }
#[no_mangle]
unsafe extern "C" fn __apply_relocation(buf: *mut u8, instr: *const *const u8, instrlen: usize, repl: *mut u8, repl_len: usize) {
    static void __apply_relocation(u8 *buf, const u8 * const instr, size_t instrlen, u8 *repl, size_t repl_len)
    {
    for (int next, i = 0; i < instrlen; i = next) {
    struct insn insn;
    if (WARN_ON_ONCE(insn_decode_kernel(&insn, &buf[i])))
    return;
    next = i + insn.length;
    switch (insn.opcode.bytes[0]) {
    case 0x0f:
    if (insn.opcode.bytes[1] < 0x80 ||
    insn.opcode.bytes[1] > 0x8f)
    break;
    fallthrough;	/* Jcc.d32 */
    case 0x70 ... 0x7f:	/* Jcc.d8 */
    case JMP8_INSN_OPCODE:
    case JMP32_INSN_OPCODE:
    case CALL_INSN_OPCODE:
    if (need_reloc(next + insn.immediate.value, repl, repl_len)) {
    apply_reloc(insn.immediate.nbytes,
    buf + i + insn_offset_immediate(&insn),
    repl - instr);
    }
//
// Where possible, convert JMP.d32 into JMP.d8.
//
    if (insn.opcode.bytes[0] == JMP32_INSN_OPCODE) {
    let mut imm: i32 = insn.immediate.value;
    imm += repl - instr;
    imm += JMP32_INSN_SIZE - JMP8_INSN_SIZE;
    if ((imm >> 31) == (imm >> 7)) {
    buf[i+0] = JMP8_INSN_OPCODE;
    buf[i+1] = (s8)imm;
    memset(&buf[i+2], INT3_INSN_OPCODE, insn.length - 2);
    }
    }
    break;
    }
    if (insn_rip_relative(&insn)) {
    if (need_reloc(next + insn.displacement.value, repl, repl_len)) {
    apply_reloc(insn.displacement.nbytes,
    buf + i + insn_offset_displacement(&insn),
    repl - instr);
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn text_poke_apply_relocation(buf: *mut u8, instr: *const *const u8, instrlen: usize, repl: *mut u8, repl_len: usize) {
    void text_poke_apply_relocation(u8 *buf, const u8 * const instr, size_t instrlen, u8 *repl, size_t repl_len)
    {
    __apply_relocation(buf, instr, instrlen, repl, repl_len);
    optimize_nops(instr, buf, instrlen);
    }
// Low-level backend functions usable from alternative code replacements.
    DEFINE_ASM_FUNC(nop_func, "", .entry.text);
    EXPORT_SYMBOL_GPL(nop_func);
#[no_mangle]
pub unsafe extern "C" fn BUG_func() -> noinstr void {
    noinstr void BUG_func(void)
    {
    BUG();
    }
    EXPORT_SYMBOL(BUG_func);
pub const CALL_RIP_REL_OPCODE: c_uint = 0xff;
pub const CALL_RIP_REL_MODRM: c_uint = 0x15;
//
// Rewrite the "call BUG_func" replacement to point to the target of the
// indirect pv_ops call "call *disp(%ip)".
//
#[no_mangle]
unsafe extern "C" fn alt_replace_call(instr: *mut u8, insn_buff: *mut u8, a: *mut alt_instr) -> c_uint {
    static unsigned int alt_replace_call(u8 *instr, u8 *insn_buff, struct alt_instr *a)
    {
    void *target, *bug = &BUG_func;
    s32 disp;
    if (a.replacementlen != 5 || insn_buff[0] != CALL_INSN_OPCODE) {
    pr_err("ALT_FLAG_DIRECT_CALL set for a non-call replacement instruction\n");
    BUG();
    }
    if (a.instrlen != 6 ||
    instr[0] != CALL_RIP_REL_OPCODE ||
    instr[1] != CALL_RIP_REL_MODRM) {
    pr_err("ALT_FLAG_DIRECT_CALL set for unrecognized indirect call\n");
    BUG();
    }
// Skip CALL_RIP_REL_OPCODE and CALL_RIP_REL_MODRM
    disp = *(s32 *)(instr + 2);

// ff 15 00 00 00 00   call   *0x0(%rip)
// target address is stored at "next instruction + disp".
    target = *(void **)(instr + a.instrlen + disp);

// ff 15 00 00 00 00   call   *0x0
// target address is stored at disp.
    target = *(void **)disp;

    if (!target)
    target = bug;
// (BUG_func - .) + (target - BUG_func) := target - .
// (s32 *)(insn_buff + 1) += target - bug;
    if (target == &nop_func)
    return 0;
    return 5;
    }
#[no_mangle]
pub unsafe extern "C" fn instr_va(i: *mut alt_instr) -> *mut u8 {
    static inline u8 * instr_va(struct alt_instr *i)
    {
    return (u8 *)&i.instr_offset + i.instr_offset;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct patch_site {
    pub instr: *mut u8,
    pub alt: *mut alt_instr,
    pub buff: [u8; MAX_PATCH_LEN],
    pub len: u8,
}

    static struct alt_instr * __init_or_module analyze_patch_site(struct patch_site *ps,
    struct alt_instr *start,
    struct alt_instr *end)
    {
    struct alt_instr *alt = start;
    ps.instr = instr_va(start);
//
// In case of nested ALTERNATIVE()s the outer alternative might add
// more padding. To ensure consistent patching find the max padding for
// all alt_instr entries for this site (nested alternatives result in
// consecutive entries).
// Find the last alt_instr eligible for patching at the site.
//
    for (; alt < end && instr_va(alt) == ps.instr; alt++) {
    ps.len = max(ps.len, alt.instrlen);
    BUG_ON(alt.cpuid >= (NCAPINTS + NBUGINTS) * 32);
//
// Patch if either:
// - feature is present
// - feature not present but ALT_FLAG_NOT is set to mean,
// patch if feature is *NOT* present.
//
    if (!boot_cpu_has(alt.cpuid) != !(alt.flags & ALT_FLAG_NOT))
    ps.alt = alt;
    }
    BUG_ON(ps.len > sizeof(ps.buff));
    return alt;
    }
#[no_mangle]
unsafe extern "C" fn prep_patch_site(ps: *mut patch_site) -> void __init_or_module {
    static void __init_or_module prep_patch_site(struct patch_site *ps)
    {
    struct alt_instr *alt = ps.alt;
    u8 buff_sz;
    u8 *repl;
    if (!alt) {
// Nothing to patch, use original instruction.
    memcpy(ps.buff, ps.instr, ps.len);
    return;
    }
    repl = (u8 *)&alt.repl_offset + alt.repl_offset;
    DPRINTK(ALT, "feat: %d*32+%d, old: (%pS (%px) len: %d), repl: (%px, len: %d) flags: 0x%x",
    alt.cpuid >> 5, alt.cpuid & 0x1f,
    ps.instr, ps.instr, ps.len,
    repl, alt.replacementlen, alt.flags);
    memcpy(ps.buff, repl, alt.replacementlen);
    buff_sz = alt.replacementlen;
    if (alt.flags & ALT_FLAG_DIRECT_CALL)
    buff_sz = alt_replace_call(ps.instr, ps.buff, alt);
    for (; buff_sz < ps.len; buff_sz++)
    ps.buff[buff_sz] = 0x90;
    __apply_relocation(ps.buff, ps.instr, ps.len, repl, alt.replacementlen);
    DUMP_BYTES(ALT, ps.instr, ps.len, "%px:   old_insn: ", ps.instr);
    DUMP_BYTES(ALT, repl, alt.replacementlen, "%px:   rpl_insn: ", repl);
    DUMP_BYTES(ALT, ps.buff, ps.len, "%px: final_insn: ", ps.instr);
    }
#[no_mangle]
unsafe extern "C" fn patch_site(ps: *mut patch_site) -> void __init_or_module {
    static void __init_or_module patch_site(struct patch_site *ps)
    {
    optimize_nops(ps.instr, ps.buff, ps.len);
    text_poke_early(ps.instr, ps.buff, ps.len);
    }
//
// Replace instructions with better alternatives for this CPU type. This runs
// before SMP is initialized to avoid SMP problems with self modifying code.
// This implies that asymmetric systems where APs have less capabilities than
// the boot processor are not handled. Tough. Make sure you disable such
// features by hand.
//
// Marked "noinline" to cause control flow change and thus insn cache
// to refetch changed I$ lines.
//
    void __init_or_module noinline apply_alternatives(struct alt_instr *start,
    struct alt_instr *end)
    {
    struct alt_instr *a;
    DPRINTK(ALT, "alt table %px, . %px", start, end);
//
// KASAN_SHADOW_START is defined using
// cpu_feature_enabled(X86_FEATURE_LA57) and is therefore patched here.
// During the process, KASAN becomes confused seeing partial LA57
// conversion and triggers a false-positive out-of-bound report.
//
// Disable KASAN until the patching is complete.
//
    kasan_disable_current();
//
// The scan order should be from start to end. A later scanned
// alternative code can overwrite previously scanned alternative code.
// Some kernel functions (e.g. memcpy, memset, etc) use this order to
// patch code.
//
// So be careful if you want to change the scan order to any other
// order.
//
    a = start;
    while (a < end) {
    struct patch_site ps = {
    .alt = core::ptr::null_mut(),
    .len = 0
    };
    a = analyze_patch_site(&ps, a, end);
    prep_patch_site(&ps);
    patch_site(&ps);
    }
    kasan_enable_current();
    }
#[no_mangle]
pub unsafe extern "C" fn is_jcc32(insn: *mut insn) -> bool {
    static inline bool is_jcc32(struct insn *insn)
    {
// Jcc.d32 second opcode byte is in the range: 0x80-0x8f
    return insn.opcode.bytes[0] == 0x0f && (insn.opcode.bytes[1] & 0xf0) == 0x80;
    }

//
// [CS]{,3} CALL/JMP *%\reg [INT3]
//
#[no_mangle]
unsafe extern "C" fn emit_indirect(op: c_int, reg: c_int, bytes: *mut u8, len: c_int) -> c_int {
    static int emit_indirect(int op, int reg, u8 *bytes, int len)
    {
    let mut cs: c_int = 0, bp = 0;
    let mut i: c_int = 0;
    u8 modrm;
//
// Set @len to the excess bytes after writing the instruction.
//
    len -= 2 + (reg >= 8);
    WARN_ON_ONCE(len < 0);
    switch (op) {
    case CALL_INSN_OPCODE:
    modrm = 0x10; /* Reg = 2; CALL r/m */
//
// Additional NOP is better than prefix decode penalty.
//
    if (len <= 3)
    cs = len;
    break;
    case JMP32_INSN_OPCODE:
    modrm = 0x20; /* Reg = 4; JMP r/m */
    bp = len;
    break;
    default:
    WARN_ON_ONCE(1);
    return -1;
    }
    while (cs--)
    bytes[i++] = 0x2e; /* CS-prefix */
    if (reg >= 8) {
    bytes[i++] = 0x41; /* REX.B prefix */
    reg -= 8;
    }
    modrm |= 0xc0; /* Mod = 3 */
    modrm += reg;
    bytes[i++] = 0xff; /* opcode */
    bytes[i++] = modrm;
    while (bp--)
    bytes[i++] = 0xcc; /* INT3 */
    return i;
    }
    static int __emit_trampoline(void *addr, struct insn *insn, u8 *bytes,
    void *call_dest, void *jmp_dest)
    {
    let mut op: u8 = insn.opcode.bytes[0];
    let mut i: c_int = 0;
//
// Clang does 'weird' Jcc __x86_indirect_thunk_r11 conditional
// tail-calls. Deal with them.
//
    if (is_jcc32(insn)) {
    bytes[i++] = op;
    op = insn.opcode.bytes[1];
    goto clang_jcc;
    }
    if (insn.length == 6)
    bytes[i++] = 0x2e; /* CS-prefix */
    switch (op) {
    case CALL_INSN_OPCODE:
    __text_gen_insn(bytes+i, op, addr+i,
    call_dest,
    CALL_INSN_SIZE);
    i += CALL_INSN_SIZE;
    break;
    case JMP32_INSN_OPCODE:
    clang_jcc:
    __text_gen_insn(bytes+i, op, addr+i,
    jmp_dest,
    JMP32_INSN_SIZE);
    i += JMP32_INSN_SIZE;
    break;
    default:
    WARN(1, "%pS %px %*ph\n", addr, addr, 6, addr);
    return -1;
    }
    WARN_ON_ONCE(i != insn.length);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn emit_call_track_retpoline(addr: *mut c_void, insn: *mut insn, reg: c_int, bytes: *mut u8) -> c_int {
    static int emit_call_track_retpoline(void *addr, struct insn *insn, int reg, u8 *bytes)
    {
    return __emit_trampoline(addr, insn, bytes,
    __x86_indirect_call_thunk_array[reg],
    __x86_indirect_jump_thunk_array[reg]);
    }

#[no_mangle]
unsafe extern "C" fn emit_its_trampoline(addr: *mut c_void, insn: *mut insn, reg: c_int, bytes: *mut u8) -> c_int {
    static int emit_its_trampoline(void *addr, struct insn *insn, int reg, u8 *bytes)
    {
    u8 *thunk = __x86_indirect_its_thunk_array[reg];
    u8 *tmp = its_allocate_thunk(reg);
    if (tmp)
    thunk = tmp;
    return __emit_trampoline(addr, insn, bytes, thunk, thunk);
    }
// Check if an indirect branch is at ITS-unsafe address
#[no_mangle]
unsafe extern "C" fn cpu_wants_indirect_its_thunk_at(addr: c_ulong, reg: c_int) -> bool {
    static bool cpu_wants_indirect_its_thunk_at(unsigned long addr, int reg)
    {
    if (!cpu_feature_enabled(X86_FEATURE_INDIRECT_THUNK_ITS))
    return false;
// Indirect branch opcode is 2 or 3 bytes depending on reg
    addr += 1 + reg / 8;
// Lower-half of the cacheline?
    return !(addr & 0x20);
    }

#[no_mangle]
unsafe extern "C" fn cpu_wants_indirect_its_thunk_at(addr: c_ulong, reg: c_int) -> bool {
    static bool cpu_wants_indirect_its_thunk_at(unsigned long addr, int reg)
    {
    return false;
    }

//
// Rewrite the compiler generated retpoline thunk calls.
//
// For spectre_v2=off (!X86_FEATURE_RETPOLINE), rewrite them into immediate
// indirect instructions, avoiding the extra indirection.
//
// For example, convert:
//
// CALL __x86_indirect_thunk_\reg
//
// into:
//
// CALL *%\reg
//
// It also tries to inline spectre_v2=retpoline,lfence when size permits.
//
#[no_mangle]
unsafe extern "C" fn patch_retpoline(addr: *mut c_void, insn: *mut insn, bytes: *mut u8) -> c_int {
    static int patch_retpoline(void *addr, struct insn *insn, u8 *bytes)
    {
    retpoline_thunk_t *target;
    int reg, ret, i = 0;
    u8 op, cc;
    target = addr + insn.length + insn.immediate.value;
    reg = target - __x86_indirect_thunk_array;
    if (WARN_ON_ONCE(reg & ~0xf))
    return -1;
// If anyone ever does: CALL/JMP *%rsp, we're in deep trouble.
    BUG_ON(reg == 4);
    if (cpu_feature_enabled(X86_FEATURE_RETPOLINE) &&
    !cpu_feature_enabled(X86_FEATURE_RETPOLINE_LFENCE)) {
    if (cpu_feature_enabled(X86_FEATURE_CALL_DEPTH))
    return emit_call_track_retpoline(addr, insn, reg, bytes);
    return -1;
    }
    op = insn.opcode.bytes[0];
//
// Convert:
//
// Jcc.d32 __x86_indirect_thunk_\reg
//
// into:
//
// Jncc.d8 1f
// [ LFENCE ]
// JMP *%\reg
// [ NOP ]
// 1:
//
    if (is_jcc32(insn)) {
    cc = insn.opcode.bytes[1] & 0xf;
    cc ^= 1; /* invert condition */
    bytes[i++] = 0x70 + cc;        /* Jcc.d8 */
    bytes[i++] = insn.length - 2; /* sizeof(Jcc.d8) == 2 */
// Continue as if: JMP.d32 __x86_indirect_thunk_\reg
    op = JMP32_INSN_OPCODE;
    }
//
// For RETPOLINE_LFENCE: prepend the indirect CALL/JMP with an LFENCE.
//
    if (cpu_feature_enabled(X86_FEATURE_RETPOLINE_LFENCE)) {
    bytes[i++] = 0x0f;
    bytes[i++] = 0xae;
    bytes[i++] = 0xe8; /* LFENCE */
    }

//
// Check if the address of last byte of emitted-indirect is in
// lower-half of the cacheline. Such branches need ITS mitigation.
//
    if (cpu_wants_indirect_its_thunk_at((unsigned long)addr + i, reg))
    return emit_its_trampoline(addr, insn, reg, bytes);

    ret = emit_indirect(op, reg, bytes + i, insn.length - i);
    if (ret < 0)
    return ret;
    i += ret;
    for (; i < insn.length;)
    bytes[i++] = BYTES_NOP1;
    return i;
    }
//
// Generated by 'objtool --retpoline'.
//
#[no_mangle]
pub unsafe extern "C" fn apply_retpolines(start: *mut i32, end: *mut i32) -> void __init_or_module noinline {
    void __init_or_module noinline apply_retpolines(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    struct insn insn;
    int len, ret;
    u8 bytes[16];
    u8 op1, op2;
    u8 *dest;
    ret = insn_decode_kernel(&insn, addr);
    if (WARN_ON_ONCE(ret < 0))
    continue;
    op1 = insn.opcode.bytes[0];
    op2 = insn.opcode.bytes[1];
    switch (op1) {
    case 0x70 ... 0x7f:	/* Jcc.d8 */
// See cfi_paranoid.
    WARN_ON_ONCE(cfi_mode != CFI_FINEIBT);
    continue;
    case CALL_INSN_OPCODE:
    case JMP32_INSN_OPCODE:
// Check for cfi_paranoid + ITS
    dest = addr + insn.length + insn.immediate.value;
    if (dest[-1] == 0xd6 && (dest[0] & 0xf0) == 0x70) {
    WARN_ON_ONCE(cfi_mode != CFI_FINEIBT);
    continue;
    }
    break;
    case 0x0f: /* escape */
    if (op2 >= 0x80 && op2 <= 0x8f)
    break;
    fallthrough;
    default:
    WARN_ON_ONCE(1);
    continue;
    }
    DPRINTK(RETPOLINE, "retpoline at: %pS (%px) len: %d to: %pS",
    addr, addr, insn.length,
    addr + insn.length + insn.immediate.value);
    len = patch_retpoline(addr, &insn, bytes);
    if (len == insn.length) {
    optimize_nops(addr, bytes, len);
    DUMP_BYTES(RETPOLINE, ((u8*)addr),  len, "%px: orig: ", addr);
    DUMP_BYTES(RETPOLINE, ((u8*)bytes), len, "%px: repl: ", addr);
    text_poke_early(addr, bytes, len);
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_wants_rethunk() -> bool {
    bool cpu_wants_rethunk(void)
    {
    return cpu_feature_enabled(X86_FEATURE_RETHUNK);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_wants_rethunk_at(addr: *mut c_void) -> bool {
    bool cpu_wants_rethunk_at(void *addr)
    {
    if (!cpu_feature_enabled(X86_FEATURE_RETHUNK))
    return false;
    if (x86_return_thunk != its_return_thunk)
    return true;
    return !((unsigned long)addr & 0x20);
    }
//
// Rewrite the compiler generated return thunk tail-calls.
//
// For example, convert:
//
// JMP __x86_return_thunk
//
// into:
//
// RET
//
#[no_mangle]
unsafe extern "C" fn patch_return(addr: *mut c_void, insn: *mut insn, bytes: *mut u8) -> c_int {
    static int patch_return(void *addr, struct insn *insn, u8 *bytes)
    {
    let mut i: c_int = 0;
// Patch the custom return thunks...
    if (cpu_wants_rethunk_at(addr)) {
    i = JMP32_INSN_SIZE;
    __text_gen_insn(bytes, JMP32_INSN_OPCODE, addr, x86_return_thunk, i);
    } else {
// ... or patch them out if not needed.
    bytes[i++] = RET_INSN_OPCODE;
    }
    for (; i < insn.length;)
    bytes[i++] = INT3_INSN_OPCODE;
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn apply_returns(start: *mut i32, end: *mut i32) -> void __init_or_module noinline {
    void __init_or_module noinline apply_returns(s32 *start, s32 *end)
    {
    s32 *s;
    if (cpu_wants_rethunk())
    static_call_force_reinit();
    for (s = start; s < end; s++) {
    void *dest = core::ptr::null_mut(), *addr = (void *)s + *s;
    struct insn insn;
    int len, ret;
    u8 bytes[16];
    u8 op;
    ret = insn_decode_kernel(&insn, addr);
    if (WARN_ON_ONCE(ret < 0))
    continue;
    op = insn.opcode.bytes[0];
    if (op == JMP32_INSN_OPCODE)
    dest = addr + insn.length + insn.immediate.value;
    if (__static_call_fixup(addr, op, dest) ||
    WARN_ONCE(dest != &__x86_return_thunk,
    "missing return thunk: %pS-%pS: %*ph",
    addr, dest, 5, addr))
    continue;
    DPRINTK(RET, "return thunk at: %pS (%px) len: %d to: %pS",
    addr, addr, insn.length,
    addr + insn.length + insn.immediate.value);
    len = patch_return(addr, &insn, bytes);
    if (len == insn.length) {
    DUMP_BYTES(RET, ((u8*)addr),  len, "%px: orig: ", addr);
    DUMP_BYTES(RET, ((u8*)bytes), len, "%px: repl: ", addr);
    text_poke_early(addr, bytes, len);
    }
    }
    }

    void __init_or_module noinline apply_returns(s32 *start, s32 *end) { }

    void __init_or_module noinline apply_retpolines(s32 *start, s32 *end) { }
    void __init_or_module noinline apply_returns(s32 *start, s32 *end) { }

#[no_mangle]
pub unsafe extern "C" fn is_endbr(val: *mut u32) -> __noendbr bool {
    __noendbr bool is_endbr(u32 *val)
    {
    u32 endbr;
    __get_kernel_nofault(&endbr, val, u32, Efault);
    return __is_endbr(endbr);
    Efault:
    return false;
    }

#[no_mangle]
unsafe extern "C" fn exact_endbr(val: *mut u32) -> __noendbr bool {
    static __noendbr bool exact_endbr(u32 *val)
    {
    u32 endbr;
    __get_kernel_nofault(&endbr, val, u32, Efault);
    let mut endbr: return = = gen_endbr();
    Efault:
    return false;
    }

    static void poison_cfi(void *addr);
#[no_mangle]
unsafe extern "C" fn poison_endbr(addr: *mut c_void) -> void __init_or_module {
    static void __init_or_module poison_endbr(void *addr)
    {
    let mut poison: u32 = gen_endbr_poison();
    if (WARN_ON_ONCE(!is_endbr(addr)))
    return;
    DPRINTK(ENDBR, "ENDBR at: %pS (%px)", addr, addr);
//
// When we have IBT, the lack of ENDBR will trigger #CP
//
    DUMP_BYTES(ENDBR, ((u8*)addr), 4, "%px: orig: ", addr);
    DUMP_BYTES(ENDBR, ((u8*)&poison), 4, "%px: repl: ", addr);
    text_poke_early(addr, &poison, 4);
    }
//
// Generated by: objtool --ibt
//
// Seal the functions for indirect calls by clobbering the ENDBR instructions
// and the kCFI hash value.
//
#[no_mangle]
pub unsafe extern "C" fn apply_seal_endbr(start: *mut i32, end: *mut i32) -> void __init_or_module noinline {
    void __init_or_module noinline apply_seal_endbr(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    poison_endbr(addr);
    if (IS_ENABLED(CONFIG_FINEIBT))
    poison_cfi(addr - CFI_OFFSET);
    }
    }

    void __init_or_module apply_seal_endbr(s32 *start, s32 *end) { }

    let mut __ro_after_init: enum cfi_mode cfi_mode = __CFI_DEFAULT;
    static bool cfi_debug __ro_after_init;

    let mut __ro_after_init: bool cfi_bhi = false;

#[no_mangle]
pub unsafe extern "C" fn cfi_get_func_hash(func: *mut c_void) -> u32 {
    u32 cfi_get_func_hash(void *func)
    {
    u32 hash;
    func -= cfi_get_offset();
    switch (cfi_mode) {
    case CFI_FINEIBT:
    func += 7;
    break;
    case CFI_KCFI:
    func += 1;
    break;
    default:
    return 0;
    }
    if (get_kernel_nofault(hash, func))
    return 0;
    return hash;
    }
#[no_mangle]
pub unsafe extern "C" fn cfi_get_func_arity(func: *mut c_void) -> c_int {
    int cfi_get_func_arity(void *func)
    {
    bhi_thunk *target;
    s32 disp;
    if (cfi_mode != CFI_FINEIBT && !cfi_bhi)
    return 0;
    if (get_kernel_nofault(disp, func - 4))
    return 0;
    target = func + disp;
    return target - __bhi_args;
    }

    let mut __ro_after_init: static bool cfi_rand = true;
    static u32  cfi_seed __ro_after_init;
//
// Re-hash the CFI hash with a boot-time seed while making sure the result is
// not a valid ENDBR instruction.
//
#[no_mangle]
unsafe extern "C" fn cfi_rehash(hash: u32) -> u32 {
    static u32 cfi_rehash(u32 hash)
    {
    hash ^= cfi_seed;
    while (unlikely(__is_endbr(hash) || __is_endbr(-hash))) {
    let mut lsb: bool = hash & 1;
    hash >>= 1;
    if (lsb)
    hash ^= 0x80200003;
    }
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn cfi_parse_cmdline(str: *mut c_char) -> __init int {
    static __init int cfi_parse_cmdline(char *str)
    {
    if (!str)
    return -EINVAL;
    while (str) {
    char *next = strchr(str, ',');
    if (next) {
// next = 0;
    next++;
    }
    if (!strcmp(str, "auto")) {
    cfi_mode = CFI_AUTO;
    } else if (!strcmp(str, "off")) {
    cfi_mode = CFI_OFF;
    cfi_rand = false;
    } else if (!strcmp(str, "debug")) {
    cfi_debug = true;
    } else if (!strcmp(str, "kcfi")) {
    cfi_mode = CFI_KCFI;
    } else if (!strcmp(str, "fineibt")) {
    cfi_mode = CFI_FINEIBT;
    } else if (!strcmp(str, "norand")) {
    cfi_rand = false;
    } else if (!strcmp(str, "warn")) {
    pr_alert("CFI: mismatch non-fatal!\n");
    cfi_warn = true;
    } else if (!strcmp(str, "paranoid")) {
    if (cfi_mode == CFI_FINEIBT) {
    cfi_paranoid = true;
    } else {
    pr_err("CFI: ignoring paranoid; depends on fineibt.\n");
    }
    } else if (!strcmp(str, "bhi")) {

    if (cfi_mode == CFI_FINEIBT) {
    cfi_bhi = true;
    } else {
    pr_err("CFI: ignoring bhi; depends on fineibt.\n");
    }

    pr_err("CFI: ignoring bhi; depends on FINEIBT_BHI=y.\n");

    } else {
    pr_err("CFI: Ignoring unknown option (%s).", str);
    }
    str = next;
    }
    return 0;
    }
    early_param("cfi", cfi_parse_cmdline);
//
// kCFI						FineIBT
//
// __cfi_\func:					__cfi_\func:
// movl   $0x12345678,%eax		// 5	     endbr64			// 4
// nop					     subl   $0x12345678,%eax    // 5
// nop					     jne.d32,pn \func+3		// 7
// nop
// \func:					\func:
// endbr64					     nopl -42(%rax)
//
// caller:					caller:
// movl	$(-0x12345678),%r10d	 // 6	     movl   $0x12345678,%eax	// 5
// addl	$-15(%r11),%r10d	 // 4	     lea    -0x10(%r11),%r11	// 4
// je	1f			 // 2	     nop5			// 5
// ud2				 // 2
// 1:	cs call	__x86_indirect_thunk_r11 // 6	     call   *%r11; nop3;	// 6
//
// Notably, the FineIBT sequences are crafted such that branches are presumed
// non-taken. This is based on Agner Fog's optimization manual, which states:
//
// "Make conditional jumps most often not taken: The efficiency and throughput
// for not-taken branches is better than for taken branches on most
// processors. Therefore, it is good to place the most frequent branch first"
//
// NOTE: Update the kCFI caller sequence to make use of this observation:
//
// kCFI						kCFI-OPT
//
// caller:					caller:
// movl	$(-0x12345678),%r10d	 // 6	     movl	$(-0x12345678),%r10d	 // 6
// addl	$-15(%r11),%r10d	 // 4	     addl	$-15(%r11),%r10d	 // 4
// je	1f			 // 2	     jne	. + 3                    // 2
// ud2				 // 2        test	$0xd6, %al		 // 2
// 1:	cs call	__x86_indirect_thunk_r11 // 6	1:   cs call	__x86_indirect_thunk_r11 // 6
//
// This new test clobbers eflags, but those are clobbered by the hash test
// anyway.
//
// <fineibt_preamble_start>:
// 0:   f3 0f 1e fa             endbr64
// 4:   2d 78 56 34 12          sub    $0x12345678, %eax
// 9:   2e 0f 85 03 00 00 00    jne,pn 13 <fineibt_preamble_start+0x13>
// 10:   0f 1f 40 d6             nopl   -0x2a(%rax)
//
// Note that the JNE target is the 0xD6 byte inside the NOPL, this decodes as
// UDB on x86_64 and raises #UD.
//
    asm(	".pushsection .rodata				\n"
    "fineibt_preamble_start:			\n"
    "	endbr64					\n"
    "	subl	$0x12345678, %eax		\n"
    "fineibt_preamble_bhi:				\n"
    "	cs jne.d32 fineibt_preamble_start+0x13	\n"
    "#fineibt_func:					\n"
    "	nopl	-42(%rax)			\n"
    "fineibt_preamble_end:				\n"
    ".popsection\n"
    );
    extern u8 fineibt_preamble_start[];
    extern u8 fineibt_preamble_bhi[];
    extern u8 fineibt_preamble_end[];

pub const fineibt_preamble_ud: c_uint = 0x13;
pub const fineibt_preamble_hash: c_int = 5;

//
// <fineibt_caller_start>:
// 0:   b8 78 56 34 12          mov    $0x12345678, %eax
// 5:   4d 8d 5b f0             lea    -0x10(%r11), %r11
// 9:   0f 1f 44 00 00          nopl   0x0(%rax,%rax,1)
//
    asm(	".pushsection .rodata			\n"
    "fineibt_caller_start:			\n"
    "	movl	$0x12345678, %eax	\n"
    "	lea	-0x10(%r11), %r11	\n"
    ASM_NOP5
    "fineibt_caller_end:			\n"
    ".popsection				\n"
    );
    extern u8 fineibt_caller_start[];
    extern u8 fineibt_caller_end[];

pub const fineibt_caller_hash: c_int = 1;

//
// Since FineIBT does hash validation on the callee side it is prone to
// circumvention attacks where a 'naked' ENDBR instruction exists that
// is not part of the fineibt_preamble sequence.
//
// Notably the x86 entry points must be ENDBR and equally cannot be
// fineibt_preamble.
//
// The fineibt_paranoid caller sequence adds additional caller side
// hash validation. This stops such circumvention attacks dead, but at the cost
// of adding a load.
//
// <fineibt_paranoid_start>:
// 0:   b8 78 56 34 12          mov    $0x12345678, %eax
// 5:   41 3b 43 f5             cmp    -0x11(%r11), %eax
// 9:   2e 4d 8d 5b <f0>        cs lea -0x10(%r11), %r11
// e:   75 fd                   jne    d <fineibt_paranoid_start+0xd>
// 10:   41 ff d3                call   *%r11
// 13:   90                      nop
//
// Notably LEA does not modify flags and can be reordered with the CMP,
// avoiding a dependency. Again, using a non-taken (backwards) branch
// for the failure case, abusing LEA's immediate 0xf0 as LOCK prefix for the
// Jcc.d8, causing #UD.
//
    asm(	".pushsection .rodata				\n"
    "fineibt_paranoid_start:			\n"
    "	mov	$0x12345678, %eax		\n"
    "	cmpl	-11(%r11), %eax			\n"
    "	cs lea	-0x10(%r11), %r11		\n"
    "#fineibt_caller_size:                          \n"
    "	jne	fineibt_paranoid_start+0xd	\n"
    "fineibt_paranoid_ind:				\n"
    "	cs call	*%r11				\n"
    "fineibt_paranoid_end:				\n"
    ".popsection					\n"
    );
    extern u8 fineibt_paranoid_start[];
    extern u8 fineibt_paranoid_ind[];
    extern u8 fineibt_paranoid_end[];

pub const fineibt_paranoid_ud: c_uint = 0xd;
#[no_mangle]
unsafe extern "C" fn decode_preamble_hash(addr: *mut c_void, reg: *mut c_int) -> u32 {
    static u32 decode_preamble_hash(void *addr, int *reg)
    {
    u8 *p = addr;
// b8+reg 78 56 34 12          movl    $0x12345678,\reg
    if (p[0] >= 0xb8 && p[0] < 0xc0) {
    if (reg)
// reg = p[0] - 0xb8;
    return *(u32 *)(addr + 1);
    }
    return 0; /* invalid hash value */
    }
#[no_mangle]
unsafe extern "C" fn decode_caller_hash(addr: *mut c_void) -> u32 {
    static u32 decode_caller_hash(void *addr)
    {
    u8 *p = addr;
// 41 ba 88 a9 cb ed       mov    $(-0x12345678),%r10d
    if (p[0] == 0x41 && p[1] == 0xba)
    return -*(u32 *)(addr + 2);
// e8 0c 88 a9 cb ed	   jmp.d8  +12
    if (p[0] == JMP8_INSN_OPCODE && p[1] == fineibt_caller_jmp)
    return -*(u32 *)(addr + 2);
    return 0; /* invalid hash value */
    }
// .retpoline_sites
#[no_mangle]
unsafe extern "C" fn cfi_disable_callers(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_disable_callers(s32 *start, s32 *end)
    {
//
// Disable kCFI by patching in a JMP.d8, this leaves the hash immediate
// in tact for later usage. Also see decode_caller_hash() and
// cfi_rewrite_callers().
//
    const u8 jmp[] = { JMP8_INSN_OPCODE, fineibt_caller_jmp };
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    u32 hash;
    addr -= fineibt_caller_size;
    hash = decode_caller_hash(addr);
    if (!hash) /* nocfi callers */
    continue;
    text_poke_early(addr, jmp, 2);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cfi_enable_callers(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_enable_callers(s32 *start, s32 *end)
    {
//
// Re-enable (and update) kCFI, undo what cfi_disable_callers() did.
//
    const u8 udne[] = { 0x75, 0x01, 0xa8, 0xd6 };
    const u8 mov[] = { 0x41, 0xba };
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    u32 hash;
    addr -= fineibt_caller_size;
    hash = decode_caller_hash(addr);
    if (!hash) /* nocfi callers */
    continue;
//
// See the kCFI/FineIBT comment above -- update note.
//
    text_poke_early(addr + 10, udne, 4);
    text_poke_early(addr, mov, 2);
    }
    return 0;
    }
// .cfi_sites
#[no_mangle]
unsafe extern "C" fn cfi_rand_preamble(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_rand_preamble(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    u32 hash;
    hash = decode_preamble_hash(addr, core::ptr::null_mut());
    if (WARN(!hash, "no CFI hash found at: %pS %px %*ph\n",
    addr, addr, 5, addr))
    return -EINVAL;
    hash = cfi_rehash(hash);
    text_poke_early(addr + 1, &hash, 4);
    }
    return 0;
    }
//
// Inline the bhi-arity 1 case:
//
// __cfi_foo:
// 0: f3 0f 1e fa             endbr64
// 4: 2d 78 56 34 12          sub    $0x12345678, %eax
// 9: 49 0f 45 fa             cmovne %rax, %rdi
// d: 2e 75 03                jne,pn    foo+0x3
//
// foo:
// 10: 0f 1f 40 <d6>           nopl -42(%rax)
//
// Notably, this scheme is incompatible with permissive CFI
// because the CMOVcc is unconditional and RDI will have been
// clobbered.
//
    asm(	".pushsection .rodata				\n"
    "fineibt_bhi1_start:				\n"
    "	cmovne %rax, %rdi			\n"
    "	cs jne fineibt_bhi1_func + 0x3		\n"
    "fineibt_bhi1_func:				\n"
    "	nopl -42(%rax)				\n"
    "fineibt_bhi1_end:				\n"
    ".popsection					\n"
    );
    extern u8 fineibt_bhi1_start[];
    extern u8 fineibt_bhi1_end[];

#[no_mangle]
unsafe extern "C" fn cfi_fineibt_bhi_preamble(addr: *mut c_void, arity: c_int) {
    static void cfi_fineibt_bhi_preamble(void *addr, int arity)
    {
    u8 bytes[MAX_INSN_SIZE];
    if (!arity)
    return;
    if (!cfi_warn && arity == 1) {
    text_poke_early(addr + fineibt_preamble_bhi,
    fineibt_bhi1_start, fineibt_bhi1_size);
    return;
    }
//
// Replace the bytes at fineibt_preamble_bhi with a CALL instruction
// that lines up exactly with the end of the preamble, such that the
// return address will be foo+0.
//
// __cfi_foo:
// 0: f3 0f 1e fa             endbr64
// 4: 2d 78 56 34 12          sub    $0x12345678, %eax
// 9: 2e 2e e8 DD DD DD DD    cs cs call __bhi_args[arity]
//
    bytes[0] = 0x2e;
    bytes[1] = 0x2e;
    __text_gen_insn(bytes + 2, CALL_INSN_OPCODE,
    addr + fineibt_preamble_bhi + 2,
    __bhi_args[arity], CALL_INSN_SIZE);
    text_poke_early(addr + fineibt_preamble_bhi, bytes, 7);
    }
#[no_mangle]
unsafe extern "C" fn cfi_rewrite_preamble(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_rewrite_preamble(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    int arity;
    u32 hash;
//
// When the function doesn't start with ENDBR the compiler will
// have determined there are no indirect calls to it and we
// don't need no CFI either.
//
    if (!is_endbr(addr + CFI_OFFSET))
    continue;
    hash = decode_preamble_hash(addr, &arity);
    if (WARN(!hash, "no CFI hash found at: %pS %px %*ph\n",
    addr, addr, 5, addr))
    return -EINVAL;
//
// FineIBT relies on being at func-16, so if the preamble is
// actually larger than that, place it the tail end.
//
// NOTE: this is possible with things like DEBUG_CALL_THUNKS
// and DEBUG_FORCE_FUNCTION_ALIGN_64B.
//
    addr += CFI_OFFSET - fineibt_prefix_size;
    text_poke_early(addr, fineibt_preamble_start, fineibt_preamble_size);
    WARN_ON(*(u32 *)(addr + fineibt_preamble_hash) != 0x12345678);
    text_poke_early(addr + fineibt_preamble_hash, &hash, 4);
    WARN_ONCE(!IS_ENABLED(CONFIG_FINEIBT_BHI) && arity,
    "kCFI preamble has wrong register at: %pS %*ph\n",
    addr, 5, addr);
    if (cfi_bhi)
    cfi_fineibt_bhi_preamble(addr, arity);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cfi_rewrite_endbr(start: *mut i32, end: *mut i32) {
    static void cfi_rewrite_endbr(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    if (!exact_endbr(addr + CFI_OFFSET))
    continue;
    poison_endbr(addr + CFI_OFFSET);
    }
    }
// .retpoline_sites
#[no_mangle]
unsafe extern "C" fn cfi_rand_callers(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_rand_callers(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    u32 hash;
    addr -= fineibt_caller_size;
    hash = decode_caller_hash(addr);
    if (hash) {
    hash = -cfi_rehash(hash);
    text_poke_early(addr + 2, &hash, 4);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn emit_paranoid_trampoline(addr: *mut c_void, insn: *mut insn, reg: c_int, bytes: *mut u8) -> c_int {
    static int emit_paranoid_trampoline(void *addr, struct insn *insn, int reg, u8 *bytes)
    {
    u8 *thunk = (void *)__x86_indirect_its_thunk_array[reg] - 2;

    u8 *tmp = its_allocate_thunk(reg);
    if (tmp)
    thunk = tmp;

    return __emit_trampoline(addr, insn, bytes, thunk, thunk);
    }
#[no_mangle]
unsafe extern "C" fn cfi_rewrite_callers(start: *mut i32, end: *mut i32) -> c_int {
    static int cfi_rewrite_callers(s32 *start, s32 *end)
    {
    s32 *s;
    for (s = start; s < end; s++) {
    void *addr = (void *)s + *s;
    struct insn insn;
    u8 bytes[20];
    u32 hash;
    int ret;
    u8 op;
    addr -= fineibt_caller_size;
    hash = decode_caller_hash(addr);
    if (!hash)
    continue;
    if (!cfi_paranoid) {
    text_poke_early(addr, fineibt_caller_start, fineibt_caller_size);
    WARN_ON(*(u32 *)(addr + fineibt_caller_hash) != 0x12345678);
    text_poke_early(addr + fineibt_caller_hash, &hash, 4);
// rely on apply_retpolines()
    continue;
    }
// cfi_paranoid
    ret = insn_decode_kernel(&insn, addr + fineibt_caller_size);
    if (WARN_ON_ONCE(ret < 0))
    continue;
    op = insn.opcode.bytes[0];
    if (op != CALL_INSN_OPCODE && op != JMP32_INSN_OPCODE) {
    WARN_ON_ONCE(1);
    continue;
    }
    memcpy(bytes, fineibt_paranoid_start, fineibt_paranoid_size);
    memcpy(bytes + fineibt_caller_hash, &hash, 4);
    if (cpu_wants_indirect_its_thunk_at((unsigned long)addr + fineibt_paranoid_ind, 11)) {
    emit_paranoid_trampoline(addr + fineibt_caller_size,
    &insn, 11, bytes + fineibt_caller_size);
    } else {
    let mut len: c_int = fineibt_paranoid_size - fineibt_paranoid_ind;
    ret = emit_indirect(op, 11, bytes + fineibt_paranoid_ind, len);
    if (WARN_ON_ONCE(ret != len))
    continue;
    }
    text_poke_early(addr, bytes, fineibt_paranoid_size);
    }
    return 0;
    }

    WARN_ONCE((_f) != (_v), "FineIBT: " #_f " %ld != %d\n", _f, _v)
    static void __init_or_module __apply_fineibt(s32 *start_retpoline, s32 *end_retpoline,
    s32 *start_cfi, s32 *end_cfi, bool builtin)
    {
    int ret;
    if (FINEIBT_WARN(fineibt_preamble_size, 20)			||
    FINEIBT_WARN(fineibt_preamble_bhi + fineibt_bhi1_size, 20)	||
    FINEIBT_WARN(fineibt_caller_size, 14)			||
    FINEIBT_WARN(fineibt_paranoid_size, 20)			||
    WARN_ON_ONCE(CFI_OFFSET < fineibt_prefix_size))
    return;
    if (cfi_mode == CFI_AUTO) {
    cfi_mode = CFI_KCFI;
    if (HAS_KERNEL_IBT && cpu_feature_enabled(X86_FEATURE_IBT)) {
//
// FRED has much saner context on exception entry and
// is less easy to take advantage of.
//
    if (!cpu_feature_enabled(X86_FEATURE_FRED))
    cfi_paranoid = true;
    cfi_mode = CFI_FINEIBT;
    }
    }
//
// Rewrite the callers to not use the __cfi_ stubs, such that we might
// rewrite them. This disables all CFI. If this succeeds but any of the
// later stages fails, we're without CFI.
//
    pr_cfi_debug("CFI: disabling all indirect call checking\n");
    ret = cfi_disable_callers(start_retpoline, end_retpoline);
    if (ret)
    goto err;
    if (cfi_rand) {
    if (builtin) {
    cfi_seed = get_random_u32();
    cfi_bpf_hash = cfi_rehash(cfi_bpf_hash);
    cfi_bpf_subprog_hash = cfi_rehash(cfi_bpf_subprog_hash);
    }
    pr_cfi_debug("CFI: cfi_seed: 0x%08x\n", cfi_seed);
    pr_cfi_debug("CFI: rehashing all preambles\n");
    ret = cfi_rand_preamble(start_cfi, end_cfi);
    if (ret)
    goto err;
    pr_cfi_debug("CFI: rehashing all indirect calls\n");
    ret = cfi_rand_callers(start_retpoline, end_retpoline);
    if (ret)
    goto err;
    } else {
    pr_cfi_debug("CFI: rehashing disabled\n");
    }
    switch (cfi_mode) {
    case CFI_OFF:
    if (builtin)
    pr_info("CFI: disabled\n");
    return;
    case CFI_KCFI:
    pr_cfi_debug("CFI: re-enabling all indirect call checking\n");
    ret = cfi_enable_callers(start_retpoline, end_retpoline);
    if (ret)
    goto err;
    if (builtin)
    pr_info("CFI: Using %sretpoline kCFI\n",
    cfi_rand ? "rehashed " : "");
    return;
    case CFI_FINEIBT:
    pr_cfi_debug("CFI: adding FineIBT to all preambles\n");
// place the FineIBT preamble at func()-16
    ret = cfi_rewrite_preamble(start_cfi, end_cfi);
    if (ret)
    goto err;
// rewrite the callers to target func()-16
    pr_cfi_debug("CFI: rewriting indirect call sites to use FineIBT\n");
    ret = cfi_rewrite_callers(start_retpoline, end_retpoline);
    if (ret)
    goto err;
// now that nobody targets func()+0, remove ENDBR there
    pr_cfi_debug("CFI: removing old endbr insns\n");
    cfi_rewrite_endbr(start_cfi, end_cfi);
    if (builtin) {
    pr_info("Using %sFineIBT%s CFI\n",
    cfi_paranoid ? "paranoid " : "",
    cfi_bhi ? "+BHI" : "");
    }
    return;
    default:
    break;
    }
    err:
    pr_err("Something went horribly wrong trying to rewrite the CFI implementation.\n");
    }
#[no_mangle]
pub unsafe extern "C" fn poison_hash(addr: *mut c_void) {
    static inline void poison_hash(void *addr)
    {
// (u32 *)addr = 0;
    }
#[no_mangle]
unsafe extern "C" fn poison_cfi(addr: *mut c_void) {
    static void poison_cfi(void *addr)
    {
//
// Compilers manage to be inconsistent with ENDBR vs __cfi prefixes,
// some (static) functions for which they can determine the address
// is never taken do not get a __cfi prefix, but *DO* get an ENDBR.
//
// As such, these functions will get sealed, but we need to be careful
// to not unconditionally scribble the previous function.
//
    switch (cfi_mode) {
    case CFI_FINEIBT:
//
// FineIBT preamble is at func-16.
//
    addr += CFI_OFFSET - fineibt_prefix_size;
//
// FineIBT prefix should start with an ENDBR.
//
    if (!is_endbr(addr))
    break;
//
// __cfi_\func:
// nopl	-42(%rax)
// sub	$0, %eax
// jne	\func+3
// \func:
// nopl	-42(%rax)
//
    poison_endbr(addr);
    poison_hash(addr + fineibt_preamble_hash);
    break;
    case CFI_KCFI:
//
// kCFI prefix should start with a valid hash.
//
    if (!decode_preamble_hash(addr, core::ptr::null_mut()))
    break;
//
// __cfi_\func:
// movl	$0, %eax
// .skip	11, 0x90
//
    poison_hash(addr + 1);
    break;
    default:
    break;
    }
    }
//
// When regs->ip points to a 0xD6 byte in the FineIBT preamble,
// return true and fill out target and type.
//
// We check the preamble by checking for the ENDBR instruction relative to the
// UDB instruction.
//
#[no_mangle]
unsafe extern "C" fn decode_fineibt_preamble(regs: *mut pt_regs, target: *mut c_ulong, type: *mut u32) -> bool {
    static bool decode_fineibt_preamble(struct pt_regs *regs, unsigned long *target, u32 *type)
    {
    let mut addr: c_ulong = regs.ip - fineibt_preamble_ud;
    u32 hash;
    if (!exact_endbr((void *)addr))
    return false;
// target = addr + fineibt_prefix_size;
    __get_kernel_nofault(&hash, addr + fineibt_preamble_hash, u32, Efault);
// type = (u32)regs->ax + hash;
//
// Since regs->ip points to the middle of an instruction; it cannot
// continue with the normal fixup.
//
    regs.ip = *target;
    return true;
    Efault:
    return false;
    }
//
// regs->ip points to one of the UD2 in __bhi_args[].
//
#[no_mangle]
unsafe extern "C" fn decode_fineibt_bhi(regs: *mut pt_regs, target: *mut c_ulong, type: *mut u32) -> bool {
    static bool decode_fineibt_bhi(struct pt_regs *regs, unsigned long *target, u32 *type)
    {
    unsigned long addr;
    u32 hash;
    if (!cfi_bhi)
    return false;
    if (regs.ip < (unsigned long)__bhi_args ||
    regs.ip >= (unsigned long)__bhi_args_end)
    return false;
//
// Fetch the return address from the stack, this points to the
// FineIBT preamble. Since the CALL instruction is in the 5 last
// bytes of the preamble, the return address is in fact the target
// address.
//
    __get_kernel_nofault(&addr, regs.sp, unsigned long, Efault);
// target = addr;
    addr -= fineibt_prefix_size;
    if (!exact_endbr((void *)addr))
    return false;
    __get_kernel_nofault(&hash, addr + fineibt_preamble_hash, u32, Efault);
// type = (u32)regs->ax + hash;
//
// The UD2 sites are constructed with a RET immediately following,
// as such the non-fatal case can use the regular fixup.
//
    return true;
    Efault:
    return false;
    }
#[no_mangle]
unsafe extern "C" fn is_paranoid_thunk(addr: c_ulong) -> bool {
    static bool is_paranoid_thunk(unsigned long addr)
    {
    u32 thunk;
    __get_kernel_nofault(&thunk, (u32 *)addr, u32, Efault);
    return (thunk & 0x00FFFFFF) == 0xfd75d6;
    Efault:
    return false;
    }
//
// regs->ip points to a LOCK Jcc.d8 instruction from the fineibt_paranoid_start[]
// sequence, or to UDB + Jcc.d8 for cfi_paranoid + ITS thunk.
//
#[no_mangle]
unsafe extern "C" fn decode_fineibt_paranoid(regs: *mut pt_regs, target: *mut c_ulong, type: *mut u32) -> bool {
    static bool decode_fineibt_paranoid(struct pt_regs *regs, unsigned long *target, u32 *type)
    {
    let mut addr: c_ulong = regs.ip - fineibt_paranoid_ud;
    if (!cfi_paranoid)
    return false;
    if (is_cfi_trap(addr + fineibt_caller_size - LEN_UD2)) {
// target = regs->r11 + fineibt_prefix_size;
// type = regs->ax;
//
// Since the trapping instruction is the exact, but LOCK prefixed,
// Jcc.d8 that got us here, the normal fixup will work.
//
    return true;
    }
//
// The cfi_paranoid + ITS thunk combination results in:
//
// 0:   b8 78 56 34 12          mov    $0x12345678, %eax
// 5:   41 3b 43 f7             cmp    -11(%r11), %eax
// a:   2e 3d 8d 5b f0          cs lea -0x10(%r11), %r11
// e:   2e e8 XX XX XX XX	 cs call __x86_indirect_paranoid_thunk_r11
//
// Where the paranoid_thunk looks like:
//
// 1d:  <d6>                    udb
// __x86_indirect_paranoid_thunk_r11:
// 1e:  75 fd                   jne 1d
// __x86_indirect_its_thunk_r11:
// 20:  41 ff eb                jmp *%r11
// 23:  cc                      int3
//
    if (is_paranoid_thunk(regs.ip)) {
// target = regs->r11 + fineibt_prefix_size;
// type = regs->ax;
    regs.ip = *target;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn decode_fineibt_insn(regs: *mut pt_regs, target: *mut c_ulong, type: *mut u32) -> bool {
    bool decode_fineibt_insn(struct pt_regs *regs, unsigned long *target, u32 *type)
    {
    if (decode_fineibt_paranoid(regs, target, type))
    return true;
    if (decode_fineibt_bhi(regs, target, type))
    return true;
    return decode_fineibt_preamble(regs, target, type);
    }

    static void __init_or_module __apply_fineibt(s32 *start_retpoline, s32 *end_retpoline,
    s32 *start_cfi, s32 *end_cfi, bool builtin)
    {
    if (IS_ENABLED(CONFIG_CFI) && builtin)
    pr_info("CFI: Using standard kCFI\n");
    }

    static void poison_cfi(void *addr) { }

    void __init_or_module apply_fineibt(s32 *start_retpoline, s32 *end_retpoline,
    s32 *start_cfi, s32 *end_cfi)
    {
    return __apply_fineibt(start_retpoline, end_retpoline,
    start_cfi, end_cfi,
// .builtin = */ false);
    }
//
// Self-test for the INT3 based CALL emulation code.
//
// This exercises int3_emulate_call() to make sure INT3 pt_regs are set up
// properly and that there is a stack gap between the INT3 frame and the
// previous context. Without this gap doing a virtual PUSH on the interrupted
// stack would corrupt the INT3 IRET frame.
//
// See entry_{32,64}.S for more details.
//
    extern void int3_selftest_asm(unsigned int *ptr);
    asm (
    "	.pushsection	.init.text, \"ax\", @progbits\n"
    "	.type		int3_selftest_asm, @function\n"
    "int3_selftest_asm:\n"
    ANNOTATE_NOENDBR "\n"
//
// INT3 padded with NOP to CALL_INSN_SIZE. The INT3 triggers an
// exception, then the int3_exception_nb notifier emulates a call to
// int3_selftest_callee().
//
    "	int3; nop; nop; nop; nop\n"
    ASM_RET
    "	.size		int3_selftest_asm, . - int3_selftest_asm\n"
    "	.popsection\n"
    );
    extern void int3_selftest_callee(unsigned int *ptr);
    asm (
    "	.pushsection	.init.text, \"ax\", @progbits\n"
    "	.type		int3_selftest_callee, @function\n"
    "int3_selftest_callee:\n"
    ANNOTATE_NOENDBR "\n"
    "	movl	$0x1234, (%" _ASM_ARG1 ")\n"
    ASM_RET
    "	.size		int3_selftest_callee, . - int3_selftest_callee\n"
    "	.popsection\n"
    );
    extern void int3_selftest_ip(void); /* defined in asm below */
    static int __init
    int3_exception_notify(struct notifier_block *self, unsigned long val, void *data)
    {
    let mut selftest: c_ulong = (unsigned long)&int3_selftest_asm;
    struct die_args *args = data;
    struct pt_regs *regs = args.regs;
    OPTIMIZER_HIDE_VAR(selftest);
    if (!regs || user_mode(regs))
    return NOTIFY_DONE;
    if (val != DIE_INT3)
    return NOTIFY_DONE;
    if (regs.ip - INT3_INSN_SIZE != selftest)
    return NOTIFY_DONE;
    int3_emulate_call(regs, (unsigned long)&int3_selftest_callee);
    return NOTIFY_STOP;
    }
// Must be noinline to ensure uniqueness of int3_selftest_ip.
#[no_mangle]
unsafe extern "C" fn int3_selftest() -> noinline void __init {
    static noinline void __init int3_selftest(void)
    {
    static __initdata struct notifier_block int3_exception_nb = {
    .notifier_call	= int3_exception_notify,
    .priority	= INT_MAX-1, /* last */
    };
    let mut val: c_uint = 0;
    BUG_ON(register_die_notifier(&int3_exception_nb));
//
// Basically: int3_selftest_callee(&val); but really complicated :-)
//
    int3_selftest_asm(&val);
    BUG_ON(val != 0x1234);
    unregister_die_notifier(&int3_exception_nb);
    }
    static __initdata int __alt_reloc_selftest_addr;
    extern void __init __alt_reloc_selftest(void *arg);
#[no_mangle]
pub unsafe extern "C" fn __alt_reloc_selftest(arg: *mut c_void) -> __visible noinline void __init {
    __visible noinline void __init __alt_reloc_selftest(void *arg)
    {
    WARN_ON(arg != &__alt_reloc_selftest_addr);
    }
#[no_mangle]
unsafe extern "C" fn alt_reloc_selftest() -> noinline void __init {
    static noinline void __init alt_reloc_selftest(void)
    {
//
// Tests text_poke_apply_relocation().
//
// This has a relative immediate (CALL) in a place other than the first
// instruction and additionally on x86_64 we get a RIP-relative LEA:
//
// lea    0x0(%rip),%rdi  # 5d0: R_X86_64_PC32    .init.data+0x5566c
// call   +0              # 5d5: R_X86_64_PLT32   __alt_reloc_selftest-0x4
//
// Getting this wrong will either crash and burn or tickle the WARN
// above.
//
    asm_inline volatile (
    ALTERNATIVE("", "lea %[mem], %%" _ASM_ARG1 "; call __alt_reloc_selftest;", X86_FEATURE_ALWAYS)
    : ASM_CALL_CONSTRAINT
    : [mem] "m" (__alt_reloc_selftest_addr)
    : _ASM_ARG1
    );
    }
#[no_mangle]
pub unsafe extern "C" fn alternative_instructions() -> void __init {
    void __init alternative_instructions(void)
    {
    u64 ibt;
    int3_selftest();
//
// The patching is not fully atomic, so try to avoid local
// interruptions that might execute the to be patched code.
// Other CPUs are not running.
//
    stop_nmi();
//
// Don't stop machine check exceptions while patching.
// MCEs only happen when something got corrupted and in this
// case we must do something about the corruption.
// Ignoring it is worse than an unlikely patching race.
// Also machine checks tend to be broadcast and if one CPU
// goes into machine check the others follow quickly, so we don't
// expect a machine check to cause undue problems during to code
// patching.
//
// Make sure to set (artificial) features depending on used paravirt
// functions which can later influence alternative patching.
//
    paravirt_set_cap();
// Keep CET-IBT disabled until caller/callee are patched
    ibt = ibt_save(/*disable*/ true);
    __apply_fineibt(__retpoline_sites, __retpoline_sites_end,
    __cfi_sites, __cfi_sites_end, true);
    cfi_debug = false;
//
// Rewrite the retpolines, must be done before alternatives since
// those can rewrite the retpoline thunks.
//
    apply_retpolines(__retpoline_sites, __retpoline_sites_end);
    apply_returns(__return_sites, __return_sites_end);
    its_fini_core();
//
// Adjust all CALL instructions to point to func()-10, including
// those in .altinstr_replacement.
//
    callthunks_patch_builtin_calls();
    apply_alternatives(__alt_instructions, __alt_instructions_end);
//
// Seal all functions that do not have their address taken.
//
    apply_seal_endbr(__ibt_endbr_seal, __ibt_endbr_seal_end);
    ibt_restore(ibt);
    restart_nmi();
    alternatives_patched = 1;
    alt_reloc_selftest();
    }
//
// text_poke_early - Update instructions on a live kernel at boot time
// @addr: address to modify
// @opcode: source of the copy
// @len: length to copy
//
// When you use this code to patch more than one byte of an instruction
// you need to make sure that other CPUs cannot execute this code in parallel.
// Also no thread must be currently preempted in the middle of these
// instructions. And on the local CPU you need to be protected against NMI or
// MCE handlers seeing an inconsistent instruction while you patch.
//
    void __init_or_module text_poke_early(void *addr, const void *opcode,
    size_t len)
    {
    unsigned long flags;
    if (boot_cpu_has(X86_FEATURE_NX) &&
    is_module_text_address((unsigned long)addr)) {
//
// Modules text is marked initially as non-executable, so the
// code cannot be running and speculative code-fetches are
// prevented. Just change the code.
//
    memcpy(addr, opcode, len);
    } else {
    local_irq_save(flags);
    memcpy(addr, opcode, len);
    sync_core();
    local_irq_restore(flags);
//
// Could also do a CLFLUSH here to speed up CPU recovery; but
// that causes hangs on some VIA CPUs.
//
    }
    }
    __ro_after_init struct mm_struct *text_poke_mm;
    __ro_after_init unsigned long text_poke_mm_addr;
//
// Text poking creates and uses a mapping in the lower half of the
// address space. Relax LASS enforcement when accessing the poking
// address.
//
// objtool enforces a strict policy of "no function calls within AC=1
// regions". Adhere to the policy by using inline versions of
// memcpy()/memset() that will never result in a function call.
//
#[no_mangle]
unsafe extern "C" fn text_poke_memcpy(dst: *mut c_void, src: *const c_void, len: usize) {
    static void text_poke_memcpy(void *dst, const void *src, size_t len)
    {
    lass_stac();
    __inline_memcpy(dst, src, len);
    lass_clac();
    }
#[no_mangle]
unsafe extern "C" fn text_poke_memset(dst: *mut c_void, src: *const c_void, len: usize) {
    static void text_poke_memset(void *dst, const void *src, size_t len)
    {
    let mut c: c_int = *(const int *)src;
    lass_stac();
    __inline_memset(dst, c, len);
    lass_clac();
    }
    typedef void text_poke_f(void *dst, const void *src, size_t len);
    static void *__text_poke(text_poke_f func, void *addr, const void *src, size_t len)
    {
    let mut cross_page_boundary: bool = offset_in_page(addr) + len > PAGE_SIZE;
    struct page *pages[2] = {core::ptr::null_mut()};
    struct mm_struct *prev_mm;
    unsigned long flags;
    pte_t pte, *ptep;
    spinlock_t *ptl;
    pgprot_t pgprot;
//
// While boot memory allocator is running we cannot use struct pages as
// they are not yet initialized. There is no way to recover.
//
    BUG_ON(!after_bootmem);
    if (!core_kernel_text((unsigned long)addr)) {
    pages[0] = vmalloc_to_page(addr);
    if (cross_page_boundary)
    pages[1] = vmalloc_to_page(addr + PAGE_SIZE);
    } else {
    pages[0] = virt_to_page(addr);
    WARN_ON(!PageReserved(pages[0]));
    if (cross_page_boundary)
    pages[1] = virt_to_page(addr + PAGE_SIZE);
    }
//
// If something went wrong, crash and burn since recovery paths are not
// implemented.
//
    BUG_ON(!pages[0] || (cross_page_boundary && !pages[1]));
//
// Map the page without the global bit, as TLB flushing is done with
// flush_tlb_mm_range(), which is intended for non-global PTEs.
//
    pgprot = __pgprot(pgprot_val(PAGE_KERNEL) & ~_PAGE_GLOBAL);
//
// The lock is not really needed, but this allows to avoid open-coding.
//
    ptep = get_locked_pte(text_poke_mm, text_poke_mm_addr, &ptl);
//
// This must not fail; preallocated in poking_init().
//
    VM_BUG_ON(!ptep);
    local_irq_save(flags);
    pte = mk_pte(pages[0], pgprot);
    set_pte_at(text_poke_mm, text_poke_mm_addr, ptep, pte);
    if (cross_page_boundary) {
    pte = mk_pte(pages[1], pgprot);
    set_pte_at(text_poke_mm, text_poke_mm_addr + PAGE_SIZE, ptep + 1, pte);
    }
//
// Loading the temporary mm behaves as a compiler barrier, which
// guarantees that the PTE will be set at the time memcpy() is done.
//
    prev_mm = use_temporary_mm(text_poke_mm);
    kasan_disable_current();
    func((u8 *)text_poke_mm_addr + offset_in_page(addr), src, len);
    kasan_enable_current();
//
// Ensure that the PTE is only cleared after the instructions of memcpy
// were issued by using a compiler barrier.
//
    barrier();
    pte_clear(text_poke_mm, text_poke_mm_addr, ptep);
    if (cross_page_boundary)
    pte_clear(text_poke_mm, text_poke_mm_addr + PAGE_SIZE, ptep + 1);
//
// Loading the previous page-table hierarchy requires a serializing
// instruction that already allows the core to see the updated version.
// Xen-PV is assumed to serialize execution in a similar manner.
//
    unuse_temporary_mm(prev_mm);
//
// Flushing the TLB might involve IPIs, which would require enabled
// IRQs, but not if the mm is not used, as it is in this point.
//
    flush_tlb_mm_range(text_poke_mm, text_poke_mm_addr, text_poke_mm_addr +
    (cross_page_boundary ? 2 : 1) * PAGE_SIZE,
    PAGE_SHIFT, false);
    if (func == text_poke_memcpy) {
//
// If the text does not match what we just wrote then something is
// fundamentally screwy; there's nothing we can really do about that.
//
    BUG_ON(memcmp(addr, src, len));
    }
    local_irq_restore(flags);
    pte_unmap_unlock(ptep, ptl);
    return addr;
    }
//
// text_poke - Update instructions on a live kernel
// @addr: address to modify
// @opcode: source of the copy
// @len: length to copy
//
// Only atomic text poke/set should be allowed when not doing early patching.
// It means the size must be writable atomically and the address must be aligned
// in a way that permits an atomic write. It also makes sure we fit on a single
// page.
//
// Note that the caller must ensure that if the modified code is part of a
// module, the module would not be removed during poking. This can be achieved
// by registering a module notifier, and ordering module removal and patching
// through a mutex.
//
    void *text_poke(void *addr, const void *opcode, size_t len)
    {
    lockdep_assert_held(&text_mutex);
    return __text_poke(text_poke_memcpy, addr, opcode, len);
    }
//
// text_poke_kgdb - Update instructions on a live kernel by kgdb
// @addr: address to modify
// @opcode: source of the copy
// @len: length to copy
//
// Only atomic text poke/set should be allowed when not doing early patching.
// It means the size must be writable atomically and the address must be aligned
// in a way that permits an atomic write. It also makes sure we fit on a single
// page.
//
// Context: should only be used by kgdb, which ensures no other core is running,
// despite the fact it does not hold the text_mutex.
//
    void *text_poke_kgdb(void *addr, const void *opcode, size_t len)
    {
    return __text_poke(text_poke_memcpy, addr, opcode, len);
    }
    void *text_poke_copy_locked(void *addr, const void *opcode, size_t len,
    bool core_ok)
    {
    let mut start: c_ulong = (unsigned long)addr;
    let mut patched: usize = 0;
    if (WARN_ON_ONCE(!core_ok && core_kernel_text(start)))
    return core::ptr::null_mut();
    while (patched < len) {
    let mut ptr: c_ulong = start + patched;
    size_t s;
    s = min_t(size_t, PAGE_SIZE * 2 - offset_in_page(ptr), len - patched);
    __text_poke(text_poke_memcpy, (void *)ptr, opcode + patched, s);
    patched += s;
    }
    return addr;
    }
//
// text_poke_copy - Copy instructions into (an unused part of) RX memory
// @addr: address to modify
// @opcode: source of the copy
// @len: length to copy, could be more than 2x PAGE_SIZE
//
// Not safe against concurrent execution; useful for JITs to dump
// new code blocks into unused regions of RX memory. Can be used in
// conjunction with synchronize_rcu_tasks() to wait for existing
// execution to quiesce after having made sure no existing functions
// pointers are live.
//
    void *text_poke_copy(void *addr, const void *opcode, size_t len)
    {
    mutex_lock(&text_mutex);
    addr = text_poke_copy_locked(addr, opcode, len, false);
    mutex_unlock(&text_mutex);
    return addr;
    }
//
// text_poke_set - memset into (an unused part of) RX memory
// @addr: address to modify
// @c: the byte to fill the area with
// @len: length to copy, could be more than 2x PAGE_SIZE
//
// This is useful to overwrite unused regions of RX memory with illegal
// instructions.
//
    void *text_poke_set(void *addr, int c, size_t len)
    {
    let mut start: c_ulong = (unsigned long)addr;
    let mut patched: usize = 0;
    if (WARN_ON_ONCE(core_kernel_text(start)))
    return core::ptr::null_mut();
    mutex_lock(&text_mutex);
    while (patched < len) {
    let mut ptr: c_ulong = start + patched;
    size_t s;
    s = min_t(size_t, PAGE_SIZE * 2 - offset_in_page(ptr), len - patched);
    __text_poke(text_poke_memset, (void *)ptr, (void *)&c, s);
    patched += s;
    }
    mutex_unlock(&text_mutex);
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn do_sync_core(info: *mut c_void) {
    static void do_sync_core(void *info)
    {
    sync_core();
    }
#[no_mangle]
pub unsafe extern "C" fn smp_text_poke_sync_each_cpu() {
    void smp_text_poke_sync_each_cpu(void)
    {
    on_each_cpu(do_sync_core, core::ptr::null_mut(), 1);
    }
//
// NOTE: crazy scheme to allow patching Jcc.d32 but not increase the size of
// this thing. When len == 6 everything is prefixed with 0x0f and we map
// opcode to Jcc.d8, using len to distinguish.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_text_poke_loc {
// addr := _stext + rel_addr
    pub rel_addr: i32,
    pub disp: i32,
    pub len: u8,
    pub opcode: u8,
    pub text: [u8; TEXT_POKE_MAX_OPCODE_SIZE],
// see smp_text_poke_batch_finish()
    pub old: u8,
}

    static struct smp_text_poke_array {
    struct smp_text_poke_loc vec[TEXT_POKE_ARRAY_MAX];
    int nr_entries;
    } text_poke_array;
    static DEFINE_PER_CPU(atomic_t, text_poke_array_refs);
//
// These four __always_inline annotations imply noinstr, necessary
// due to smp_text_poke_int3_handler() being noinstr:
//
#[no_mangle]
unsafe extern "C" fn try_get_text_poke_array() -> __always_inline bool {
    static __always_inline bool try_get_text_poke_array(void)
    {
    atomic_t *refs = this_cpu_ptr(&text_poke_array_refs);
    if (!raw_atomic_inc_not_zero(refs))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn put_text_poke_array() -> __always_inline void {
    static __always_inline void put_text_poke_array(void)
    {
    atomic_t *refs = this_cpu_ptr(&text_poke_array_refs);
    smp_mb__before_atomic();
    raw_atomic_dec(refs);
    }
    static __always_inline void *text_poke_addr(const struct smp_text_poke_loc *tpl)
    {
    return _stext + tpl.rel_addr;
    }
#[no_mangle]
unsafe extern "C" fn patch_cmp(tpl_a: *const c_void, tpl_b: *const c_void) -> __always_inline int {
    static __always_inline int patch_cmp(const void *tpl_a, const void *tpl_b)
    {
    if (tpl_a < text_poke_addr(tpl_b))
    return -1;
    if (tpl_a > text_poke_addr(tpl_b))
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smp_text_poke_int3_handler(regs: *mut pt_regs) -> noinstr int {
    noinstr int smp_text_poke_int3_handler(struct pt_regs *regs)
    {
    struct smp_text_poke_loc *tpl;
    let mut ret: c_int = 0;
    void *ip;
    if (user_mode(regs))
    return 0;
//
// Having observed our INT3 instruction, we now must observe
// text_poke_array with non-zero refcount:
//
// text_poke_array_refs = 1		INT3
// WMB			RMB
// write INT3		if (text_poke_array_refs != 0)
//
    smp_rmb();
    if (!try_get_text_poke_array())
    return 0;
//
// Discount the INT3. See smp_text_poke_batch_finish().
//
    ip = (void *) regs.ip - INT3_INSN_SIZE;
//
// Skip the binary search if there is a single member in the vector.
//
    if (unlikely(text_poke_array.nr_entries > 1)) {
    tpl = __inline_bsearch(ip, text_poke_array.vec, text_poke_array.nr_entries,
    sizeof(struct smp_text_poke_loc),
    patch_cmp);
    if (!tpl)
    goto out_put;
    } else {
    tpl = text_poke_array.vec;
    if (text_poke_addr(tpl) != ip)
    goto out_put;
    }
    ip += tpl.len;
    switch (tpl.opcode) {
    case INT3_INSN_OPCODE:
//
// Someone poked an explicit INT3, they'll want to handle it,
// do not consume.
//
    goto out_put;
    case RET_INSN_OPCODE:
    int3_emulate_ret(regs);
    break;
    case CALL_INSN_OPCODE:
    int3_emulate_call(regs, (long)ip + tpl.disp);
    break;
    case JMP32_INSN_OPCODE:
    case JMP8_INSN_OPCODE:
    int3_emulate_jmp(regs, (long)ip + tpl.disp);
    break;
    case 0x70 ... 0x7f: /* Jcc */
    int3_emulate_jcc(regs, tpl.opcode & 0xf, (long)ip, tpl.disp);
    break;
    default:
    BUG();
    }
    ret = 1;
    out_put:
    put_text_poke_array();
    return ret;
    }
//
// smp_text_poke_batch_finish() -- update instructions on live kernel on SMP
//
// Input state:
// text_poke_array.vec: vector of instructions to patch
// text_poke_array.nr_entries: number of entries in the vector
//
// Modify multi-byte instructions by using INT3 breakpoints on SMP.
// We completely avoid using stop_machine() here, and achieve the
// synchronization using INT3 breakpoints and SMP cross-calls.
//
// The way it is done:
// - For each entry in the vector:
// - add an INT3 trap to the address that will be patched
// - SMP sync all CPUs
// - For each entry in the vector:
// - update all but the first byte of the patched range
// - SMP sync all CPUs
// - For each entry in the vector:
// - replace the first byte (INT3) by the first byte of the
// replacing opcode
// - SMP sync all CPUs
//
#[no_mangle]
pub unsafe extern "C" fn smp_text_poke_batch_finish() {
    void smp_text_poke_batch_finish(void)
    {
    let mut int3: c_uchar = INT3_INSN_OPCODE;
    unsigned int i;
    int do_sync;
    if (!text_poke_array.nr_entries)
    return;
    lockdep_assert_held(&text_mutex);
//
// Corresponds to the implicit memory barrier in try_get_text_poke_array() to
// ensure reading a non-zero refcount provides up to date text_poke_array data.
//
    for_each_possible_cpu(i)
    atomic_set_release(per_cpu_ptr(&text_poke_array_refs, i), 1);
//
// Function tracing can enable thousands of places that need to be
// updated. This can take quite some time, and with full kernel debugging
// enabled, this could cause the softlockup watchdog to trigger.
// This function gets called every 256 entries added to be patched.
// Call cond_resched() here to make sure that other tasks can get scheduled
// while processing all the functions being patched.
//
    cond_resched();
//
// Corresponding read barrier in INT3 notifier for making sure the
// text_poke_array.nr_entries and handler are correctly ordered wrt. patching.
//
    smp_wmb();
//
// First step: add a INT3 trap to the address that will be patched.
//
    for (i = 0; i < text_poke_array.nr_entries; i++) {
    text_poke_array.vec[i].old = *(u8 *)text_poke_addr(&text_poke_array.vec[i]);
    text_poke(text_poke_addr(&text_poke_array.vec[i]), &int3, INT3_INSN_SIZE);
    }
    smp_text_poke_sync_each_cpu();
//
// Second step: update all but the first byte of the patched range.
//
    for (do_sync = 0, i = 0; i < text_poke_array.nr_entries; i++) {
    u8 old[TEXT_POKE_MAX_OPCODE_SIZE+1] = { text_poke_array.vec[i].old, };
    u8 _new[TEXT_POKE_MAX_OPCODE_SIZE+1];
    const u8 *new = text_poke_array.vec[i].text;
    let mut len: c_int = text_poke_array.vec[i].len;
    if (len - INT3_INSN_SIZE > 0) {
    memcpy(old + INT3_INSN_SIZE,
    text_poke_addr(&text_poke_array.vec[i]) + INT3_INSN_SIZE,
    len - INT3_INSN_SIZE);
    if (len == 6) {
    _new[0] = 0x0f;
    memcpy(_new + 1, new, 5);
    new = _new;
    }
    text_poke(text_poke_addr(&text_poke_array.vec[i]) + INT3_INSN_SIZE,
    new + INT3_INSN_SIZE,
    len - INT3_INSN_SIZE);
    do_sync++;
    }
//
// Emit a perf event to record the text poke, primarily to
// support Intel PT decoding which must walk the executable code
// to reconstruct the trace. The flow up to here is:
// - write INT3 byte
// - IPI-SYNC
// - write instruction tail
// At this point the actual control flow will be through the
// INT3 and handler and not hit the old or new instruction.
// Intel PT outputs FUP/TIP packets for the INT3, so the flow
// can still be decoded. Subsequently:
// - emit RECORD_TEXT_POKE with the new instruction
// - IPI-SYNC
// - write first byte
// - IPI-SYNC
// So before the text poke event timestamp, the decoder will see
// either the old instruction flow or FUP/TIP of INT3. After the
// text poke event timestamp, the decoder will see either the
// new instruction flow or FUP/TIP of INT3. Thus decoders can
// use the timestamp as the point at which to modify the
// executable code.
// The old instruction is recorded so that the event can be
// processed forwards or backwards.
//
    perf_event_text_poke(text_poke_addr(&text_poke_array.vec[i]), old, len, new, len);
    }
    if (do_sync) {
//
// According to Intel, this core syncing is very likely
// not necessary and we'd be safe even without it. But
// better safe than sorry (plus there's not only Intel).
//
    smp_text_poke_sync_each_cpu();
    }
//
// Third step: replace the first byte (INT3) by the first byte of the
// replacing opcode.
//
    for (do_sync = 0, i = 0; i < text_poke_array.nr_entries; i++) {
    let mut byte: u8 = text_poke_array.vec[i].text[0];
    if (text_poke_array.vec[i].len == 6)
    byte = 0x0f;
    if (byte == INT3_INSN_OPCODE)
    continue;
    text_poke(text_poke_addr(&text_poke_array.vec[i]), &byte, INT3_INSN_SIZE);
    do_sync++;
    }
    if (do_sync)
    smp_text_poke_sync_each_cpu();
//
// Remove and wait for refs to be zero.
//
// Notably, if after step-3 above the INT3 got removed, then the
// smp_text_poke_sync_each_cpu() will have serialized against any running INT3
// handlers and the below spin-wait will not happen.
//
// IOW. unless the replacement instruction is INT3, this case goes
// unused.
//
    for_each_possible_cpu(i) {
    atomic_t *refs = per_cpu_ptr(&text_poke_array_refs, i);
    if (unlikely(!atomic_dec_and_test(refs)))
    atomic_cond_read_acquire(refs, !VAL);
    }
// They are all completed:
    text_poke_array.nr_entries = 0;
    }
#[no_mangle]
unsafe extern "C" fn __smp_text_poke_batch_add(addr: *mut c_void, opcode: *const c_void, len: usize, emulate: *const c_void) {
    static void __smp_text_poke_batch_add(void *addr, const void *opcode, size_t len, const void *emulate)
    {
    struct smp_text_poke_loc *tpl;
    struct insn insn;
    int ret, i = 0;
    tpl = &text_poke_array.vec[text_poke_array.nr_entries++];
    if (len == 6)
    i = 1;
    memcpy((void *)tpl.text, opcode+i, len-i);
    if (!emulate)
    emulate = opcode;
    ret = insn_decode_kernel(&insn, emulate);
    BUG_ON(ret < 0);
    tpl.rel_addr = addr - (void *)_stext;
    tpl.len = len;
    tpl.opcode = insn.opcode.bytes[0];
    if (is_jcc32(&insn)) {
//
// Map Jcc.d32 onto Jcc.d8 and use len to distinguish.
//
    tpl.opcode = insn.opcode.bytes[1] - 0x10;
    }
    switch (tpl.opcode) {
    case RET_INSN_OPCODE:
    case JMP32_INSN_OPCODE:
    case JMP8_INSN_OPCODE:
//
// Control flow instructions without implied execution of the
// next instruction can be padded with INT3.
//
    for (i = insn.length; i < len; i++)
    BUG_ON(tpl.text[i] != INT3_INSN_OPCODE);
    break;
    default:
    BUG_ON(len != insn.length);
    }
    switch (tpl.opcode) {
    case INT3_INSN_OPCODE:
    case RET_INSN_OPCODE:
    break;
    case CALL_INSN_OPCODE:
    case JMP32_INSN_OPCODE:
    case JMP8_INSN_OPCODE:
    case 0x70 ... 0x7f: /* Jcc */
    tpl.disp = insn.immediate.value;
    break;
    default: /* assume NOP */
    switch (len) {
    case 2: /* NOP2 -- emulate as JMP8+0 */
    BUG_ON(memcmp(emulate, x86_nops[len], len));
    tpl.opcode = JMP8_INSN_OPCODE;
    tpl.disp = 0;
    break;
    case 5: /* NOP5 -- emulate as JMP32+0 */
    BUG_ON(memcmp(emulate, x86_nops[len], len));
    tpl.opcode = JMP32_INSN_OPCODE;
    tpl.disp = 0;
    break;
    default: /* unknown instruction */
    BUG();
    }
    break;
    }
    }
//
// We hard rely on the text_poke_array.vec being ordered; ensure this is so by flushing
// early if needed.
//
#[no_mangle]
unsafe extern "C" fn text_poke_addr_ordered(addr: *mut c_void) -> bool {
    static bool text_poke_addr_ordered(void *addr)
    {
    WARN_ON_ONCE(!addr);
    if (!text_poke_array.nr_entries)
    return true;
//
// If the last current entry's address is higher than the
// new entry's address we'd like to add, then ordering
// is violated and we must first flush all pending patching
// requests:
//
    if (text_poke_addr(text_poke_array.vec + text_poke_array.nr_entries-1) > addr)
    return false;
    return true;
    }
//
// smp_text_poke_batch_add() -- update instruction on live kernel on SMP, batched
// @addr:	address to patch
// @opcode:	opcode of new instruction
// @len:	length to copy
// @emulate:	instruction to be emulated
//
// Add a new instruction to the current queue of to-be-patched instructions
// the kernel maintains. The patching request will not be executed immediately,
// but becomes part of an array of patching requests, optimized for batched
// execution. All pending patching requests will be executed on the next
// smp_text_poke_batch_finish() call.
//
#[no_mangle]
pub unsafe extern "C" fn smp_text_poke_batch_add(addr: *mut c_void, opcode: *const c_void, len: usize, emulate: *const c_void) -> void __ref {
    void __ref smp_text_poke_batch_add(void *addr, const void *opcode, size_t len, const void *emulate)
    {
    if (text_poke_array.nr_entries == TEXT_POKE_ARRAY_MAX || !text_poke_addr_ordered(addr))
    smp_text_poke_batch_finish();
    __smp_text_poke_batch_add(addr, opcode, len, emulate);
    }
//
// smp_text_poke_single() -- update instruction on live kernel on SMP immediately
// @addr:	address to patch
// @opcode:	opcode of new instruction
// @len:	length to copy
// @emulate:	instruction to be emulated
//
// Update a single instruction with the vector in the stack, avoiding
// dynamically allocated memory. This function should be used when it is
// not possible to allocate memory for a vector. The single instruction
// is patched in immediately.
//
#[no_mangle]
pub unsafe extern "C" fn smp_text_poke_single(addr: *mut c_void, opcode: *const c_void, len: usize, emulate: *const c_void) -> void __ref {
    void __ref smp_text_poke_single(void *addr, const void *opcode, size_t len, const void *emulate)
    {
    smp_text_poke_batch_add(addr, opcode, len, emulate);
    smp_text_poke_batch_finish();
    }
