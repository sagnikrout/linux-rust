//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/kprobes/opt.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Kernel Probes Jump Optimization (Optprobes)
//
// Copyright (C) IBM Corporation, 2002, 2004
// Copyright (C) Hitachi Ltd., 2012
//

#[no_mangle]
pub unsafe extern "C" fn __recover_optprobed_insn(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong {
    unsigned long __recover_optprobed_insn(kprobe_opcode_t *buf, unsigned long addr)
    {
    struct optimized_kprobe *op;
    struct kprobe *kp;
    long offs;
    int i;
    for (i = 0; i < JMP32_INSN_SIZE; i++) {
    kp = get_kprobe((void *)addr - i);
// This function only handles jump-optimized kprobe
    if (kp && kprobe_optimized(kp)) {
    op = container_of(kp, struct optimized_kprobe, kp);
// If op is optimized or under unoptimizing
    if (list_empty(&op.list) || optprobe_queued_unopt(op))
    goto found;
    }
    }
    return addr;
    found:
//
// If the kprobe can be optimized, original bytes which can be
// overwritten by jump destination address. In this case, original
// bytes must be recovered from op->optinsn.copied_insn buffer.
//
    if (copy_from_kernel_nofault(buf, (void *)addr,
#[no_mangle]
pub unsafe extern "C" fn sizeof(_arg: kprobe_opcode_t))) -> *mut MAX_INSN_SIZE {
    MAX_INSN_SIZE * sizeof(kprobe_opcode_t)))
    return 0UL;
    if (addr == (unsigned long)kp.addr) {
    buf[0] = kp.opcode;
    memcpy(buf + 1, op.optinsn.copied_insn, DISP32_SIZE);
    } else {
    offs = addr - (unsigned long)kp.addr - 1;
    memcpy(buf, op.optinsn.copied_insn + offs, DISP32_SIZE - offs);
    }
    return (unsigned long)buf;
    }
#[no_mangle]
unsafe extern "C" fn synthesize_clac(addr: *mut kprobe_opcode_t) {
    static void synthesize_clac(kprobe_opcode_t *addr)
    {
//
// Can't be cpu_feature_enabled() due to how objtool treats this feature bit.
// This isn't a fast path anyway.
//
    if (!boot_cpu_has(X86_FEATURE_SMAP))
    return;
// Replace the NOP3 with CLAC
    addr[0] = 0x0f;
    addr[1] = 0x01;
    addr[2] = 0xca;
    }
// Insert a move instruction which sets a pointer to eax/rdi (1st arg).
#[no_mangle]
unsafe extern "C" fn synthesize_set_arg1(addr: *mut kprobe_opcode_t, val: c_ulong) {
    static void synthesize_set_arg1(kprobe_opcode_t *addr, unsigned long val)
    {

// addr++ = 0x48;
// addr++ = 0xbf;

// addr++ = 0xb8;

// (unsigned long *)addr = val;
    }
    asm (
    ".pushsection .rodata\n"
    ".global optprobe_template_entry\n"
    "optprobe_template_entry:\n"

    "       pushq $" __stringify(__KERNEL_DS) "\n"
// Save the 'sp - 8', this will be fixed later.
    "	pushq %rsp\n"
    "	pushfq\n"
    ".global optprobe_template_clac\n"
    "optprobe_template_clac:\n"
    ASM_NOP3
    SAVE_REGS_STRING
    "	movq %rsp, %rsi\n"
    ".global optprobe_template_val\n"
    "optprobe_template_val:\n"
    ASM_NOP5
    ASM_NOP5
    ".global optprobe_template_call\n"
    "optprobe_template_call:\n"
    ASM_NOP5
// Copy 'regs->flags' into 'regs->ss'.
    "	movq 18*8(%rsp), %rdx\n"
    "	movq %rdx, 20*8(%rsp)\n"
    RESTORE_REGS_STRING
// Skip 'regs->flags' and 'regs->sp'.
    "	addq $16, %rsp\n"
// And pop flags register from 'regs->ss'.
    "	popfq\n"

    "	pushl %ss\n"
// Save the 'sp - 4', this will be fixed later.
    "	pushl %esp\n"
    "	pushfl\n"
    ".global optprobe_template_clac\n"
    "optprobe_template_clac:\n"
    ASM_NOP3
    SAVE_REGS_STRING
    "	movl %esp, %edx\n"
    ".global optprobe_template_val\n"
    "optprobe_template_val:\n"
    ASM_NOP5
    ".global optprobe_template_call\n"
    "optprobe_template_call:\n"
    ASM_NOP5
// Copy 'regs->flags' into 'regs->ss'.
    "	movl 14*4(%esp), %edx\n"
    "	movl %edx, 16*4(%esp)\n"
    RESTORE_REGS_STRING
// Skip 'regs->flags' and 'regs->sp'.
    "	addl $8, %esp\n"
// And pop flags register from 'regs->ss'.
    "	popfl\n"

    ".global optprobe_template_end\n"
    "optprobe_template_end:\n"
    ".popsection\n");

    ((long)optprobe_template_clac - (long)optprobe_template_entry)

    ((long)optprobe_template_val - (long)optprobe_template_entry)

    ((long)optprobe_template_call - (long)optprobe_template_entry)

    ((long)optprobe_template_end - (long)optprobe_template_entry)
// Optimized kprobe call back function: called from optinsn
    static void
    optimized_callback(struct optimized_kprobe *op, struct pt_regs *regs)
    {
// This is possible if op is under delayed unoptimizing
    if (kprobe_disabled(&op.kp))
    return;
    preempt_disable();
    if (kprobe_running()) {
    kprobes_inc_nmissed_count(&op.kp);
    } else {
    struct kprobe_ctlblk *kcb = get_kprobe_ctlblk();
// Adjust stack pointer
    regs.sp += sizeof(long);
// Save skipped registers
    regs.cs = __KERNEL_CS;

    regs.gs = 0;

    regs.ip = (unsigned long)op.kp.addr + INT3_INSN_SIZE;
    regs.orig_ax = ~0UL;
    __this_cpu_write(current_kprobe, &op.kp);
    kcb.kprobe_status = KPROBE_HIT_ACTIVE;
    opt_pre_handler(&op.kp, regs);
    __this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    preempt_enable();
    }
    NOKPROBE_SYMBOL(optimized_callback);
#[no_mangle]
unsafe extern "C" fn copy_optimized_instructions(dest: *mut u8, src: *mut u8, real: *mut u8) -> c_int {
    static int copy_optimized_instructions(u8 *dest, u8 *src, u8 *real)
    {
    struct insn insn;
    let mut len: c_int = 0, ret;
    while (len < JMP32_INSN_SIZE) {
    ret = __copy_instruction(dest + len, src + len, real + len, &insn);
    if (!ret || !can_boost(&insn, src + len))
    return -EINVAL;
    len += ret;
    }
// Check whether the address range is reserved
    if (ftrace_text_reserved(src, src + len - 1) ||
    jump_label_text_reserved(src, src + len - 1) ||
    static_call_text_reserved(src, src + len - 1))
    return -EBUSY;
    return len;
    }
// Check whether insn is indirect jump
#[no_mangle]
unsafe extern "C" fn insn_is_indirect_jump(insn: *mut insn) -> c_int {
    static int insn_is_indirect_jump(struct insn *insn)
    {
    return ((insn.opcode.bytes[0] == 0xff &&
    (X86_MODRM_REG(insn.modrm.value) & 6) == 4) || /* Jump */
    insn.opcode.bytes[0] == 0xea);	/* Segment based jump */
    }
// Check whether insn jumps into specified address range
#[no_mangle]
unsafe extern "C" fn insn_jump_into_range(insn: *mut insn, start: c_ulong, len: c_int) -> c_int {
    static int insn_jump_into_range(struct insn *insn, unsigned long start, int len)
    {
    let mut target: c_ulong = 0;
    switch (insn.opcode.bytes[0]) {
    case 0xe0:	/* loopne */
    case 0xe1:	/* loope */
    case 0xe2:	/* loop */
    case 0xe3:	/* jcxz */
    case 0xe9:	/* near relative jump */
    case 0xeb:	/* short relative jump */
    break;
    case 0x0f:
    if ((insn.opcode.bytes[1] & 0xf0) == 0x80) /* jcc near */
    break;
    return 0;
    default:
    if ((insn.opcode.bytes[0] & 0xf0) == 0x70) /* jcc short */
    break;
    return 0;
    }
    target = (unsigned long)insn.next_byte + insn.immediate.value;
    return (start <= target && target <= start + len);
    }
// Decode whole function to ensure any instructions don't jump into target
#[no_mangle]
unsafe extern "C" fn can_optimize(paddr: c_ulong) -> c_int {
    static int can_optimize(unsigned long paddr)
    {
    unsigned long addr, size = 0, offset = 0;
    struct insn insn;
    kprobe_opcode_t buf[MAX_INSN_SIZE];
// Lookup symbol including addr
    if (!kallsyms_lookup_size_offset(paddr, &size, &offset))
    return 0;
//
// Do not optimize in the entry code due to the unstable
// stack handling and registers setup.
//
    if (((paddr >= (unsigned long)__entry_text_start) &&
    (paddr <  (unsigned long)__entry_text_end)))
    return 0;
// Check there is enough space for a relative jump.
    if (size - offset < JMP32_INSN_SIZE)
    return 0;
// Decode instructions
    addr = paddr - offset;
    while (addr < paddr - offset + size) { /* Decode until function end */
    unsigned long recovered_insn;
    int ret;
    if (search_exception_tables(addr))
//
// Since some fixup code will jumps into this function,
// we can't optimize kprobe in this function.
//
    return 0;
    recovered_insn = recover_probed_instruction(buf, addr);
    if (!recovered_insn)
    return 0;
    ret = insn_decode_kernel(&insn, (void *)recovered_insn);
    if (ret < 0)
    return 0;

//
// If there is a dynamically installed kgdb sw breakpoint,
// this function should not be probed.
//
    if (insn.opcode.bytes[0] == INT3_INSN_OPCODE &&
    kgdb_has_hit_break(addr))
    return 0;

// Recover address
    insn.kaddr = (void *)addr;
    insn.next_byte = (void *)(addr + insn.length);
//
// Check any instructions don't jump into target, indirectly or
// directly.
//
// The indirect case is present to handle a code with jump
// tables. When the kernel uses retpolines, the check should in
// theory additionally look for jumps to indirect thunks.
// However, the kernel built with retpolines or IBT has jump
// tables disabled so the check can be skipped altogether.
//
    if (!IS_ENABLED(CONFIG_MITIGATION_RETPOLINE) &&
    !IS_ENABLED(CONFIG_X86_KERNEL_IBT) &&
    insn_is_indirect_jump(&insn))
    return 0;
    if (insn_jump_into_range(&insn, paddr + INT3_INSN_SIZE,
    DISP32_SIZE))
    return 0;
    addr += insn.length;
    }
    return 1;
    }
// Check optimized_kprobe can actually be optimized.
#[no_mangle]
pub unsafe extern "C" fn arch_check_optimized_kprobe(op: *mut optimized_kprobe) -> c_int {
    int arch_check_optimized_kprobe(struct optimized_kprobe *op)
    {
    int i;
    struct kprobe *p;
    for (i = 1; i < op.optinsn.size; i++) {
    p = get_kprobe(op.kp.addr + i);
    if (p && !kprobe_disarmed(p))
    return -EEXIST;
    }
    return 0;
    }
// Check the addr is within the optimized instructions.
    int arch_within_optimized_kprobe(struct optimized_kprobe *op,
    kprobe_opcode_t *addr)
    {
    return (op.kp.addr <= addr &&
    op.kp.addr + op.optinsn.size > addr);
    }
// Free optimized instruction slot
    static
#[no_mangle]
pub unsafe extern "C" fn __arch_remove_optimized_kprobe(op: *mut optimized_kprobe, dirty: c_int) {
    void __arch_remove_optimized_kprobe(struct optimized_kprobe *op, int dirty)
    {
    u8 *slot = op.optinsn.insn;
    if (slot) {
    let mut len: c_int = TMPL_END_IDX + op.optinsn.size + JMP32_INSN_SIZE;
// Record the perf event before freeing the slot
    if (dirty)
    perf_event_text_poke(slot, slot, len, core::ptr::null_mut(), 0);
    free_optinsn_slot(slot, dirty);
    op.optinsn.insn = core::ptr::null_mut();
    op.optinsn.size = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe) {
    void arch_remove_optimized_kprobe(struct optimized_kprobe *op)
    {
    __arch_remove_optimized_kprobe(op, 1);
    }
//
// Copy replacing target instructions
// Target instructions MUST be relocatable (checked inside)
// This is called when new aggr(opt)probe is allocated or reused.
//
    int arch_prepare_optimized_kprobe(struct optimized_kprobe *op,
    struct kprobe *__unused)
    {
    u8 *buf = core::ptr::null_mut(), *slot;
    int ret, len;
    long rel;
    if (!can_optimize((unsigned long)op.kp.addr))
    return -EILSEQ;
    buf = kzalloc(MAX_OPTINSN_SIZE, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    op.optinsn.insn = slot = get_optinsn_slot();
    if (!slot) {
    ret = -ENOMEM;
    goto out;
    }
//
// Verify if the address gap is in 2GB range, because this uses
// a relative jump.
//
    rel = (long)slot - (long)op.kp.addr + JMP32_INSN_SIZE;
    if (abs(rel) > 0x7fffffff) {
    ret = -ERANGE;
    goto err;
    }
// Copy arch-dep-instance from template
    memcpy(buf, optprobe_template_entry, TMPL_END_IDX);
// Copy instructions into the out-of-line buffer
    ret = copy_optimized_instructions(buf + TMPL_END_IDX, op.kp.addr,
    slot + TMPL_END_IDX);
    if (ret < 0)
    goto err;
    op.optinsn.size = ret;
    len = TMPL_END_IDX + op.optinsn.size;
    synthesize_clac(buf + TMPL_CLAC_IDX);
// Set probe information
    synthesize_set_arg1(buf + TMPL_MOVE_IDX, (unsigned long)op);
// Set probe function call
    synthesize_relcall(buf + TMPL_CALL_IDX,
    slot + TMPL_CALL_IDX, optimized_callback);
// Set returning jmp instruction at the tail of out-of-line buffer
    synthesize_reljump(buf + len, slot + len,
    (u8 *)op.kp.addr + op.optinsn.size);
    len += JMP32_INSN_SIZE;
//
// Note	len = TMPL_END_IDX + op->optinsn.size + JMP32_INSN_SIZE is also
// used in __arch_remove_optimized_kprobe().
//
// We have to use text_poke() for instruction buffer because it is RO
    perf_event_text_poke(slot, core::ptr::null_mut(), 0, buf, len);
    text_poke(slot, buf, len);
    ret = 0;
    out:
    kfree(buf);
    return ret;
    err:
    __arch_remove_optimized_kprobe(op, 0);
    goto out;
    }
//
// Replace breakpoints (INT3) with relative jumps (JMP.d32).
// Caller must call with locking kprobe_mutex and text_mutex.
//
// The caller will have installed a regular kprobe and after that issued
// syncrhonize_rcu_tasks(), this ensures that the instruction(s) that live in
// the 4 bytes after the INT3 are unused and can now be overwritten.
//
#[no_mangle]
pub unsafe extern "C" fn arch_optimize_kprobes(oplist: *mut list_head) {
    void arch_optimize_kprobes(struct list_head *oplist)
    {
    struct optimized_kprobe *op, *tmp;
    u8 insn_buff[JMP32_INSN_SIZE];
    list_for_each_entry_safe(op, tmp, oplist, list) {
    s32 rel = (s32)((long)op.optinsn.insn -
    ((long)op.kp.addr + JMP32_INSN_SIZE));
    WARN_ON(kprobe_disabled(&op.kp));
// Backup instructions which will be replaced by jump address
    memcpy(op.optinsn.copied_insn, op.kp.addr + INT3_INSN_SIZE,
    DISP32_SIZE);
    insn_buff[0] = JMP32_INSN_OPCODE;
// (s32 *)(&insn_buff[1]) = rel;
    smp_text_poke_single(op.kp.addr, insn_buff, JMP32_INSN_SIZE, core::ptr::null_mut());
    list_del_init(&op.list);
    }
    }
//
// Replace a relative jump (JMP.d32) with a breakpoint (INT3).
//
// After that, we can restore the 4 bytes after the INT3 to undo what
// arch_optimize_kprobes() scribbled. This is safe since those bytes will be
// unused once the INT3 lands.
//
#[no_mangle]
pub unsafe extern "C" fn arch_unoptimize_kprobe(op: *mut optimized_kprobe) {
    void arch_unoptimize_kprobe(struct optimized_kprobe *op)
    {
    u8 new[JMP32_INSN_SIZE] = { INT3_INSN_OPCODE, };
    u8 old[JMP32_INSN_SIZE];
    u8 *addr = op.kp.addr;
    memcpy(old, op.kp.addr, JMP32_INSN_SIZE);
    memcpy(new + INT3_INSN_SIZE,
    op.optinsn.copied_insn,
    JMP32_INSN_SIZE - INT3_INSN_SIZE);
    text_poke(addr, new, INT3_INSN_SIZE);
    smp_text_poke_sync_each_cpu();
    text_poke(addr + INT3_INSN_SIZE,
    new + INT3_INSN_SIZE,
    JMP32_INSN_SIZE - INT3_INSN_SIZE);
    smp_text_poke_sync_each_cpu();
    perf_event_text_poke(op.kp.addr, old, JMP32_INSN_SIZE, new, JMP32_INSN_SIZE);
    }
//
// Recover original instructions and breakpoints from relative jumps.
// Caller must call with locking kprobe_mutex.
//
    extern void arch_unoptimize_kprobes(struct list_head *oplist,
    struct list_head *done_list)
    {
    struct optimized_kprobe *op, *tmp;
    list_for_each_entry_safe(op, tmp, oplist, list) {
    arch_unoptimize_kprobe(op);
    list_move(&op.list, done_list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn setup_detour_execution(p: *mut kprobe, regs: *mut pt_regs, reenter: c_int) -> c_int {
    int setup_detour_execution(struct kprobe *p, struct pt_regs *regs, int reenter)
    {
    struct optimized_kprobe *op;
    if (p.flags & KPROBE_FLAG_OPTIMIZED) {
// This kprobe is really able to run optimized path.
    op = container_of(p, struct optimized_kprobe, kp);
// Detour through copied instructions
    regs.ip = (unsigned long)op.optinsn.insn + TMPL_END_IDX;
    if (!reenter)
    reset_current_kprobe();
    return 1;
    }
    return 0;
    }
    NOKPROBE_SYMBOL(setup_detour_execution);
