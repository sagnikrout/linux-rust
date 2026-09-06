//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/static_call.c
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

// Declared locally to avoid pulling asm/paravirt-spinlock.h header.

    struct qspinlock;
    void __raw_callee_save___native_queued_spin_unlock(struct qspinlock *lock);

    enum insn_type {
    CALL = 0, /* site call */
    NOP = 1,  /* site cond-call */
    JMP = 2,  /* tramp / site tail-call */
    RET = 3,  /* tramp / site cond-tail-call */
    JCC = 4,
    };
//
// ud1 %esp, %ecx - a 3 byte #UD that is unique to trampolines, chosen such
// that there is no false-positive trampoline identification while also being a
// speculation stop.
//
    static const u8 tramp_ud[] = { 0x0f, 0xb9, 0xcc };
//
// cs cs cs xorl %eax, %eax - a single 5 byte instruction that clears %[er]ax
//
    static const u8 xor5rax[] = { 0x2e, 0x2e, 0x2e, 0x31, 0xc0 };
    static const u8 retinsn[] = { RET_INSN_OPCODE, 0xcc, 0xcc, 0xcc, 0xcc };
//
// ud1    (%edx),%rdi -- see __WARN_trap() / decode_bug()
//
    static const u8 warninsn[] = { 0x67, 0x48, 0x0f, 0xb9, 0x3a };

//
// ds ds movb $0, (_ASM_ARG1)
//

    static const u8 unlockinsn[] = { 0x3e, 0x3e, 0xc6, 0x07, 0x00 };

    static const u8 unlockinsn[] = { 0x3e, 0x3e, 0xc6, 0x00, 0x00 };

    static u8 __is_Jcc(u8 *insn) /* Jcc.d32 */
    {
    let mut ret: u8 = 0;
    if (insn[0] == 0x0f) {
    let mut tmp: u8 = insn[1];
    if ((tmp & 0xf0) == 0x80)
    ret = tmp;
    }
    return ret;
    }
    extern void __static_call_return(void);
    asm (".global __static_call_return\n\t"
    ".type __static_call_return, @function\n\t"
    ASM_FUNC_ALIGN "\n\t"
    "__static_call_return:\n\t"
    ANNOTATE_NOENDBR "\n\t"
    ANNOTATE_RETPOLINE_SAFE "\n\t"
    "ret; int3\n\t"
    ".size __static_call_return, . - __static_call_return \n\t");
    static void __ref __static_call_transform(void *insn, enum insn_type type,
    void *func, bool modinit)
    {
    const void *emulate = core::ptr::null_mut();
    let mut size: c_int = CALL_INSN_SIZE;
    const void *code;
    u8 op, buf[6];
    if ((type == JMP || type == RET) && (op = __is_Jcc(insn)))
    type = JCC;
    switch (type) {
    case CALL:
    func = callthunks_translate_call_dest(func);
    code = text_gen_insn(CALL_INSN_OPCODE, insn, func);
    if (func == &__static_call_return0) {
    emulate = code;
    code = &xor5rax;
    }
    if (func == &__WARN_trap) {
    emulate = code;
    code = &warninsn;
    }

    if (func == &__raw_callee_save___native_queued_spin_unlock) {
    emulate = code;
    code = &unlockinsn;
    }

    break;
    case NOP:
    code = x86_nops[5];
    break;
    case JMP:
    code = text_gen_insn(JMP32_INSN_OPCODE, insn, func);
    break;
    case RET:
    if (cpu_wants_rethunk_at(insn))
    code = text_gen_insn(JMP32_INSN_OPCODE, insn, x86_return_thunk);
    else
    code = &retinsn;
    break;
    case JCC:
    if (!func) {
    func = __static_call_return;
    if (cpu_wants_rethunk())
    func = x86_return_thunk;
    }
    buf[0] = 0x0f;
    __text_gen_insn(buf+1, op, insn+1, func, 5);
    code = buf;
    size = 6;
    break;
    }
    if (memcmp(insn, code, size) == 0)
    return;
    if (system_state == SYSTEM_BOOTING || modinit)
    return text_poke_early(insn, code, size);
    smp_text_poke_single(insn, code, size, emulate);
    }
#[no_mangle]
unsafe extern "C" fn __static_call_validate(insn: *mut u8, tail: bool, tramp: bool) {
    static void __static_call_validate(u8 *insn, bool tail, bool tramp)
    {
    let mut opcode: u8 = insn[0];
    if (tramp && memcmp(insn+5, tramp_ud, 3)) {
    pr_err("trampoline signature fail");
    BUG();
    }
    if (tail) {
    if (opcode == JMP32_INSN_OPCODE ||
    opcode == RET_INSN_OPCODE ||
    __is_Jcc(insn))
    return;
    } else {
    if (opcode == CALL_INSN_OPCODE ||
    !memcmp(insn, x86_nops[5], 5) ||
    !memcmp(insn, xor5rax, 5) ||
    !memcmp(insn, warninsn, 5))
    return;

    if (!memcmp(insn, unlockinsn, 5))
    return;

    }
//
// If we ever trigger this, our text is corrupt, we'll probably not live long.
//
    pr_err("unexpected static_call insn opcode 0x%x at %pS\n", opcode, insn);
    BUG();
    }
#[no_mangle]
pub unsafe extern "C" fn __sc_insn(null: bool, tail: bool) -> enum insn_type {
    static inline enum insn_type __sc_insn(bool null, bool tail)
    {
//
// Encode the following table without branches:
//
// tail	null	insn
// -----+-------+------
// 0  |   0   |  CALL
// 0  |   1   |  NOP
// 1  |   0   |  JMP
// 1  |   1   |  RET
//
    return 2*tail + null;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_static_call_transform(site: *mut c_void, tramp: *mut c_void, func: *mut c_void, tail: bool) {
    void arch_static_call_transform(void *site, void *tramp, void *func, bool tail)
    {
    mutex_lock(&text_mutex);
    if (tramp && !site) {
    __static_call_validate(tramp, true, true);
    __static_call_transform(tramp, __sc_insn(!func, true), func, false);
    }
    if (IS_ENABLED(CONFIG_HAVE_STATIC_CALL_INLINE) && site) {
    __static_call_validate(site, tail, false);
    __static_call_transform(site, __sc_insn(!func, tail), func, false);
    }
    mutex_unlock(&text_mutex);
    }
    EXPORT_SYMBOL_GPL(arch_static_call_transform);
#[no_mangle]
pub unsafe extern "C" fn __static_call_update_early(tramp: *mut c_void, func: *mut c_void) -> noinstr void {
    noinstr void __static_call_update_early(void *tramp, void *func)
    {
    BUG_ON(system_state != SYSTEM_BOOTING);
    BUG_ON(static_call_initialized);
    __text_gen_insn(tramp, JMP32_INSN_OPCODE, tramp, func, JMP32_INSN_SIZE);
    sync_core();
    }

//
// This is called by apply_returns() to fix up static call trampolines,
// specifically ARCH_DEFINE_STATIC_CALL_NULL_TRAMP which is recorded as
// having a return trampoline.
//
// The problem is that static_call() is available before determining
// X86_FEATURE_RETHUNK and, by implication, running alternatives.
//
// This means that __static_call_transform() above can have overwritten the
// return trampoline and we now need to fix things up to be consistent.
//
#[no_mangle]
pub unsafe extern "C" fn __static_call_fixup(tramp: *mut c_void, op: u8, dest: *mut c_void) -> bool {
    bool __static_call_fixup(void *tramp, u8 op, void *dest)
    {
    let mut addr: c_ulong = (unsigned long)tramp;
//
// Not all .return_sites are a static_call trampoline (most are not).
// Check if the 3 bytes after the return are still kernel text, if not,
// then this definitely is not a trampoline and we need not worry
// further.
//
// This avoids the memcmp() below tripping over pagefaults etc..
//
    if (((addr >> PAGE_SHIFT) != ((addr + 7) >> PAGE_SHIFT)) &&
    !kernel_text_address(addr + 7))
    return false;
    if (memcmp(tramp+5, tramp_ud, 3)) {
// Not a trampoline site, not our problem.
    return false;
    }
    mutex_lock(&text_mutex);
    if (op == RET_INSN_OPCODE || dest == &__x86_return_thunk)
    __static_call_transform(tramp, RET, core::ptr::null_mut(), true);
    mutex_unlock(&text_mutex);
    return true;
    }
