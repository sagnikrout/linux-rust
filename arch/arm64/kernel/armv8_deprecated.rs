//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/armv8_deprecated.c
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
//
// Copyright (C) 2014 ARM Limited
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// The runtime support for deprecated instruction support can be in one of
// following three states -
//
// 0 = undef
// 1 = emulate (software emulation)
// 2 = hw (supported in hardware)
//
    enum insn_emulation_mode {
    INSN_UNDEF,
    INSN_EMULATE,
    INSN_HW,
    };
    enum legacy_insn_status {
    INSN_DEPRECATED,
    INSN_OBSOLETE,
    INSN_UNAVAILABLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_emulation {
    pub name: *const c_char,
    pub status: enum legacy_insn_status,
    bool				(*try_emulate)(struct pt_regs *regs,
    pub insn): u32,
    pub enable): *mut *mut int (set_hw_mode)(bool,
    pub current_mode: c_int,
    pub min: c_int,
    pub max: c_int,
// sysctl for this emulation
    pub sysctl: ctl_table,
}

pub const ARM_OPCODE_CONDTEST_FAIL: c_int = 0;
pub const ARM_OPCODE_CONDTEST_PASS: c_int = 1;
pub const ARM_OPCODE_CONDTEST_UNCOND: c_int = 2;
pub const ARM_OPCODE_CONDITION_UNCOND: c_uint = 0xf;
#[no_mangle]
unsafe extern "C" fn aarch32_check_condition(opcode: u32, psr: u32) -> unsigned int __maybe_unused {
    static unsigned int __maybe_unused aarch32_check_condition(u32 opcode, u32 psr)
    {
    let mut cc_bits: u32 = opcode >> 28;
    if (cc_bits != ARM_OPCODE_CONDITION_UNCOND) {
    if ((*aarch32_opcode_cond_checks[cc_bits])(psr))
    return ARM_OPCODE_CONDTEST_PASS;
    else
    return ARM_OPCODE_CONDTEST_FAIL;
    }
    return ARM_OPCODE_CONDTEST_UNCOND;
    }

//
// Implement emulation of the SWP/SWPB instructions using load-exclusive and
// store-exclusive.
//
// Syntax of SWP{B} instruction: SWP{B}<c> <Rt>, <Rt2>, [<Rn>]
// Where: Rt  = destination
// Rt2 = source
// Rn  = address
//
// Error-checking SWP macros implemented using ldxr{b}/stxr{b}
//
// Arbitrary constant to ensure forward-progress of the LL/SC loop
pub const __SWP_LL_SC_LOOPS: c_int = 4;

    do {								\
    uaccess_enable_privileged();				\
    __asm__ __volatile__(					\
    "	mov		%w3, %w6\n"			\
    "0:	ldxr"B"		%w2, [%4]\n"			\
    "1:	stxr"B"		%w0, %w1, [%4]\n"		\
    "	cbz		%w0, 2f\n"			\
    "	sub		%w3, %w3, #1\n"			\
    "	cbnz		%w3, 0b\n"			\
    "	mov		%w0, %w5\n"			\
    "	b		3f\n"				\
    "2:\n"							\
    "	mov		%w1, %w2\n"			\
    "3:\n"							\
    _ASM_EXTABLE_UACCESS_ERR(0b, 3b, %w0)			\
    _ASM_EXTABLE_UACCESS_ERR(1b, 3b, %w0)			\
    : "=&r" (res), "+r" (data), "=&r" (temp), "=&r" (temp2)	\
    : "r" ((unsigned long)addr), "i" (-EAGAIN),		\
    "i" (__SWP_LL_SC_LOOPS)				\
    : "memory");						\
    uaccess_disable_privileged();				\
    } while (0)

    __user_swpX_asm(data, addr, res, temp, temp2, "")

    __user_swpX_asm(data, addr, res, temp, temp2, "b")
//
// Bit 22 of the instruction encoding distinguishes between
// the SWP and SWPB variants (bit set means SWPB).
//

    static int emulate_swpX(unsigned int address, unsigned int *data,
    unsigned int type)
    {
    let mut res: c_uint = 0;
    if ((type != TYPE_SWPB) && (address & 0x3)) {
// SWP to unaligned address not permitted
    pr_debug("SWP instruction on unaligned pointer!\n");
    return -EFAULT;
    }
    while (1) {
    unsigned long temp, temp2;
    if (type == TYPE_SWPB)
    __user_swpb_asm(*data, address, res, temp, temp2);
    else
    __user_swp_asm(*data, address, res, temp, temp2);
    if (likely(res != -EAGAIN) || signal_pending(current))
    break;
    cond_resched();
    }
    return res;
    }
//
// swp_handler logs the id of calling process, dissects the instruction, sanity
// checks the memory location, calls emulate_swpX for the actual operation and
// deals with fixup/error handling before returning
//
#[no_mangle]
unsafe extern "C" fn swp_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    static int swp_handler(struct pt_regs *regs, u32 instr)
    {
    u32 destreg, data, type, address = 0;
    const void __user *user_ptr;
    int rn, rt2, res = 0;
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, regs.pc);
    type = instr & TYPE_SWPB;
    switch (aarch32_check_condition(instr, regs.pstate)) {
    case ARM_OPCODE_CONDTEST_PASS:
    break;
    case ARM_OPCODE_CONDTEST_FAIL:
// Condition failed - return to next instruction
    goto ret;
    case ARM_OPCODE_CONDTEST_UNCOND:
// If unconditional encoding - not a SWP, undef
    return -EFAULT;
    default:
    return -EINVAL;
    }
    rn = aarch32_insn_extract_reg_num(instr, A32_RN_OFFSET);
    rt2 = aarch32_insn_extract_reg_num(instr, A32_RT2_OFFSET);
    address = (u32)regs.user_regs.regs[rn];
    data	= (u32)regs.user_regs.regs[rt2];
    destreg = aarch32_insn_extract_reg_num(instr, A32_RT_OFFSET);
    pr_debug("addr in r%d.0x%08x, dest is r%d, source in r%d.0x%08x)\n",
    rn, address, destreg,
    aarch32_insn_extract_reg_num(instr, A32_RT2_OFFSET), data);
// Check access in reasonable access range for both SWP and SWPB
    user_ptr = (const void __user *)(unsigned long)(address & ~3);
    if (!access_ok(user_ptr, 4)) {
    pr_debug("SWP{B} emulation: access to 0x%08x not allowed!\n",
    address);
    goto fault;
    }
    res = emulate_swpX(address, &data, type);
    if (res == -EFAULT)
    goto fault;
#[no_mangle]
pub unsafe extern "C" fn if(0: res ==) -> else {
    else if (res == 0)
    regs.user_regs.regs[destreg] = data;
    ret:
    if (type == TYPE_SWPB)
    trace_instruction_emulation("swpb", regs.pc);
    else
    trace_instruction_emulation("swp", regs.pc);
    pr_warn_ratelimited("\"%s\" (%ld) uses obsolete SWP{B} instruction at 0x%llx\n",
    current.comm, (unsigned long)current.pid, regs.pc);
    arm64_skip_faulting_instruction(regs, 4);
    return 0;
    fault:
    pr_debug("SWP{B} emulation: access caused memory abort!\n");
    arm64_notify_segfault(address);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn try_emulate_swp(regs: *mut pt_regs, insn: u32) -> bool {
    static bool try_emulate_swp(struct pt_regs *regs, u32 insn)
    {
// SWP{B} only exists in ARM state and does not exist in Thumb
    if (!compat_user_mode(regs) || compat_thumb_mode(regs))
    return false;
    if ((insn & 0x0fb00ff0) != 0x01000090)
    return false;
    return swp_handler(regs, insn) == 0;
    }
    static struct insn_emulation insn_swp = {
    .name = "swp",
    .status = INSN_OBSOLETE,
    .try_emulate = try_emulate_swp,
    .set_hw_mode = core::ptr::null_mut(),
    };

#[no_mangle]
unsafe extern "C" fn cp15barrier_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    static int cp15barrier_handler(struct pt_regs *regs, u32 instr)
    {
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, regs.pc);
    switch (aarch32_check_condition(instr, regs.pstate)) {
    case ARM_OPCODE_CONDTEST_PASS:
    break;
    case ARM_OPCODE_CONDTEST_FAIL:
// Condition failed - return to next instruction
    goto ret;
    case ARM_OPCODE_CONDTEST_UNCOND:
// If unconditional encoding - not a barrier instruction
    return -EFAULT;
    default:
    return -EINVAL;
    }
    switch (aarch32_insn_mcr_extract_crm(instr)) {
    case 10:
//
// dmb - mcr p15, 0, Rt, c7, c10, 5
// dsb - mcr p15, 0, Rt, c7, c10, 4
//
    if (aarch32_insn_mcr_extract_opc2(instr) == 5) {
    dmb(sy);
    trace_instruction_emulation(
    "mcr p15, 0, Rt, c7, c10, 5 ; dmb", regs.pc);
    } else {
    dsb(sy);
    trace_instruction_emulation(
    "mcr p15, 0, Rt, c7, c10, 4 ; dsb", regs.pc);
    }
    break;
    case 5:
//
// isb - mcr p15, 0, Rt, c7, c5, 4
//
// Taking an exception or returning from one acts as an
// instruction barrier. So no explicit barrier needed here.
//
    trace_instruction_emulation(
    "mcr p15, 0, Rt, c7, c5, 4 ; isb", regs.pc);
    break;
    }
    ret:
    pr_warn_ratelimited("\"%s\" (%ld) uses deprecated CP15 Barrier instruction at 0x%llx\n",
    current.comm, (unsigned long)current.pid, regs.pc);
    arm64_skip_faulting_instruction(regs, 4);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp15_barrier_set_hw_mode(enable: bool) -> c_int {
    static int cp15_barrier_set_hw_mode(bool enable)
    {
    if (enable)
    sysreg_clear_set(sctlr_el1, 0, SCTLR_EL1_CP15BEN);
    else
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_CP15BEN, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn try_emulate_cp15_barrier(regs: *mut pt_regs, insn: u32) -> bool {
    static bool try_emulate_cp15_barrier(struct pt_regs *regs, u32 insn)
    {
    if (!compat_user_mode(regs) || compat_thumb_mode(regs))
    return false;
    if ((insn & 0x0fff0fdf) == 0x0e070f9a)
    return cp15barrier_handler(regs, insn) == 0;
    if ((insn & 0x0fff0fff) == 0x0e070f95)
    return cp15barrier_handler(regs, insn) == 0;
    return false;
    }
    static struct insn_emulation insn_cp15_barrier = {
    .name = "cp15_barrier",
    .status = INSN_DEPRECATED,
    .try_emulate = try_emulate_cp15_barrier,
    .set_hw_mode = cp15_barrier_set_hw_mode,
    };

#[no_mangle]
unsafe extern "C" fn setend_set_hw_mode(enable: bool) -> c_int {
    static int setend_set_hw_mode(bool enable)
    {
    if (!cpu_supports_mixed_endian_el0())
    return -EINVAL;
    if (enable)
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_SED, 0);
    else
    sysreg_clear_set(sctlr_el1, 0, SCTLR_EL1_SED);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn compat_setend_handler(regs: *mut pt_regs, big_endian: u32) -> c_int {
    static int compat_setend_handler(struct pt_regs *regs, u32 big_endian)
    {
    char *insn;
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, regs.pc);
    if (big_endian) {
    insn = "setend be";
    regs.pstate |= PSR_AA32_E_BIT;
    } else {
    insn = "setend le";
    regs.pstate &= ~PSR_AA32_E_BIT;
    }
    trace_instruction_emulation(insn, regs.pc);
    pr_warn_ratelimited("\"%s\" (%ld) uses deprecated setend instruction at 0x%llx\n",
    current.comm, (unsigned long)current.pid, regs.pc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a32_setend_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    static int a32_setend_handler(struct pt_regs *regs, u32 instr)
    {
    let mut rc: c_int = compat_setend_handler(regs, (instr >> 9) & 1);
    arm64_skip_faulting_instruction(regs, 4);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn t16_setend_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    static int t16_setend_handler(struct pt_regs *regs, u32 instr)
    {
    let mut rc: c_int = compat_setend_handler(regs, (instr >> 3) & 1);
    arm64_skip_faulting_instruction(regs, 2);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn try_emulate_setend(regs: *mut pt_regs, insn: u32) -> bool {
    static bool try_emulate_setend(struct pt_regs *regs, u32 insn)
    {
    if (compat_thumb_mode(regs) &&
    (insn & 0xfffffff7) == 0x0000b650)
    return t16_setend_handler(regs, insn) == 0;
    if (compat_user_mode(regs) &&
    (insn & 0xfffffdff) == 0xf1010000)
    return a32_setend_handler(regs, insn) == 0;
    return false;
    }
    static struct insn_emulation insn_setend = {
    .name = "setend",
    .status = INSN_DEPRECATED,
    .try_emulate = try_emulate_setend,
    .set_hw_mode = setend_set_hw_mode,
    };

    static struct insn_emulation *insn_emulations[] = {

    &insn_swp,

    &insn_cp15_barrier,

    &insn_setend,

    };
    static DEFINE_MUTEX(insn_emulation_mutex);
#[no_mangle]
unsafe extern "C" fn enable_insn_hw_mode(data: *mut c_void) {
    static void enable_insn_hw_mode(void *data)
    {
    struct insn_emulation *insn = data;
    if (insn.set_hw_mode)
    insn.set_hw_mode(true);
    }
#[no_mangle]
unsafe extern "C" fn disable_insn_hw_mode(data: *mut c_void) {
    static void disable_insn_hw_mode(void *data)
    {
    struct insn_emulation *insn = data;
    if (insn.set_hw_mode)
    insn.set_hw_mode(false);
    }
// Run set_hw_mode(mode) on all active CPUs
#[no_mangle]
unsafe extern "C" fn run_all_cpu_set_hw_mode(insn: *mut insn_emulation, enable: bool) -> c_int {
    static int run_all_cpu_set_hw_mode(struct insn_emulation *insn, bool enable)
    {
    if (!insn.set_hw_mode)
    return -EINVAL;
    if (enable)
    on_each_cpu(enable_insn_hw_mode, (void *)insn, true);
    else
    on_each_cpu(disable_insn_hw_mode, (void *)insn, true);
    return 0;
    }
//
// Run set_hw_mode for all insns on a starting CPU.
// Returns:
// 0 		- If all the hooks ran successfully.
// -EINVAL	- At least one hook is not supported by the CPU.
//
#[no_mangle]
unsafe extern "C" fn run_all_insn_set_hw_mode(cpu: c_uint) -> c_int {
    static int run_all_insn_set_hw_mode(unsigned int cpu)
    {
    let mut rc: c_int = 0;
    unsigned long flags;
//
// Disable IRQs to serialize against an IPI from
// run_all_cpu_set_hw_mode(), ensuring the HW is programmed to the most
// recent enablement state if the two race with one another.
//
    local_irq_save(flags);
    for (int i = 0; i < ARRAY_SIZE(insn_emulations); i++) {
    struct insn_emulation *insn = insn_emulations[i];
    let mut enable: bool = READ_ONCE(insn.current_mode) == INSN_HW;
    if (insn.status == INSN_UNAVAILABLE)
    continue;
    if (insn.set_hw_mode && insn.set_hw_mode(enable)) {
    pr_warn("CPU[%u] cannot support the emulation of %s",
    cpu, insn.name);
    rc = -EINVAL;
    }
    }
    local_irq_restore(flags);
    return rc;
    }
    static int update_insn_emulation_mode(struct insn_emulation *insn,
    enum insn_emulation_mode prev)
    {
    let mut ret: c_int = 0;
    switch (prev) {
    case INSN_UNDEF: /* Nothing to be done */
    break;
    case INSN_EMULATE:
    break;
    case INSN_HW:
    if (!run_all_cpu_set_hw_mode(insn, false))
    pr_notice("Disabled %s support\n", insn.name);
    break;
    }
    switch (insn.current_mode) {
    case INSN_UNDEF:
    break;
    case INSN_EMULATE:
    break;
    case INSN_HW:
    ret = run_all_cpu_set_hw_mode(insn, true);
    if (!ret)
    pr_notice("Enabled %s support\n", insn.name);
    break;
    }
    return ret;
    }
    static int emulation_proc_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp,
    loff_t *ppos)
    {
    let mut ret: c_int = 0;
    struct insn_emulation *insn = container_of(table.data, struct insn_emulation, current_mode);
    let mut prev_mode: enum insn_emulation_mode = insn.current_mode;
    mutex_lock(&insn_emulation_mutex);
    ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (ret || !write || prev_mode == insn.current_mode)
    goto ret;
    ret = update_insn_emulation_mode(insn, prev_mode);
    if (ret) {
// Mode change failed, revert to previous mode.
    WRITE_ONCE(insn.current_mode, prev_mode);
    update_insn_emulation_mode(insn, INSN_UNDEF);
    }
    ret:
    mutex_unlock(&insn_emulation_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn register_insn_emulation(insn: *mut insn_emulation) -> void __init {
    static void __init register_insn_emulation(struct insn_emulation *insn)
    {
    struct ctl_table *sysctl;
    insn.min = INSN_UNDEF;
    switch (insn.status) {
    case INSN_DEPRECATED:
    insn.current_mode = INSN_EMULATE;
// Disable the HW mode if it was turned on at early boot time
    run_all_cpu_set_hw_mode(insn, false);
    insn.max = INSN_HW;
    break;
    case INSN_OBSOLETE:
    insn.current_mode = INSN_UNDEF;
    insn.max = INSN_EMULATE;
    break;
    case INSN_UNAVAILABLE:
    insn.current_mode = INSN_UNDEF;
    insn.max = INSN_UNDEF;
    break;
    }
// Program the HW if required
    update_insn_emulation_mode(insn, INSN_UNDEF);
    if (insn.status != INSN_UNAVAILABLE) {
    sysctl = &insn.sysctl;
    sysctl.mode = 0644;
    sysctl.maxlen = sizeof(int);
    sysctl.procname = insn.name;
    sysctl.data = &insn.current_mode;
    sysctl.extra1 = &insn.min;
    sysctl.extra2 = &insn.max;
    sysctl.proc_handler = emulation_proc_handler;
    register_sysctl_sz("abi", sysctl, 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn try_emulate_armv8_deprecated(regs: *mut pt_regs, insn: u32) -> bool {
    bool try_emulate_armv8_deprecated(struct pt_regs *regs, u32 insn)
    {
    for (int i = 0; i < ARRAY_SIZE(insn_emulations); i++) {
    struct insn_emulation *ie = insn_emulations[i];
    if (ie.status == INSN_UNAVAILABLE)
    continue;
//
// A trap may race with the mode being changed
// INSN_EMULATE<->INSN_HW. Try to emulate the instruction to
// avoid a spurious UNDEF.
//
    if (READ_ONCE(ie.current_mode) == INSN_UNDEF)
    continue;
    if (ie.try_emulate(regs, insn))
    return true;
    }
    return false;
    }
//
// Invoked as core_initcall, which guarantees that the instruction
// emulation is ready for userspace.
//
#[no_mangle]
unsafe extern "C" fn armv8_deprecated_init() -> int __init {
    static int __init armv8_deprecated_init(void)
    {

    if (!system_supports_mixed_endian_el0()) {
    insn_setend.status = INSN_UNAVAILABLE;
    pr_info("setend instruction emulation is not supported on this system\n");
    }

//
// The purpose of supporting LSUI is to eliminate PAN toggling. CPUs
// that support LSUI are unlikely to support a 32-bit runtime. Rather
// than emulating the SWP instruction using LSUI instructions, simply
// disable SWP emulation.
//
    if (cpus_have_final_cap(ARM64_HAS_LSUI)) {
    insn_swp.status = INSN_UNAVAILABLE;
    pr_info("swp/swpb instruction emulation is not supported on this system\n");
    }

    for (int i = 0; i < ARRAY_SIZE(insn_emulations); i++) {
    struct insn_emulation *ie = insn_emulations[i];
    if (ie.status == INSN_UNAVAILABLE)
    continue;
    register_insn_emulation(ie);
    }
    cpuhp_setup_state_nocalls(CPUHP_AP_ARM64_ISNDEP_STARTING,
    "arm64/isndep:starting",
    run_all_insn_set_hw_mode, core::ptr::null_mut());
    return 0;
    }
    core_initcall(armv8_deprecated_init);
