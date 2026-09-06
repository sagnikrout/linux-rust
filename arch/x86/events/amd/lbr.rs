//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/amd/lbr.c
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

// LBR Branch Select valid bits
pub const LBR_SELECT_MASK: c_uint = 0x1ff;
//
// LBR Branch Select filter bits which when set, ensures that the
// corresponding type of branches are not recorded
//

pub const LBR_IGNORE: c_int = 0;

    (LBR_JCC | LBR_REL_CALL | LBR_IND_CALL | LBR_RETURN |	\
    LBR_REL_JMP | LBR_IND_JMP | LBR_FAR)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_entry {
    union {
    struct {
    pub ip:58: u64,
    pub ip_sign_ext:5: u64,
    pub mispredict:1: u64,
    pub split: },
    pub full: u64,
    pub from: },
    union {
    struct {
    pub ip:58: u64,
    pub ip_sign_ext:3: u64,
    pub reserved:1: u64,
    pub spec:1: u64,
    pub valid:1: u64,
    pub split: },
    pub full: u64,
    pub to: },
}

#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_set_from(idx: c_uint, val: u64) -> __always_inline void {
    static __always_inline void amd_pmu_lbr_set_from(unsigned int idx, u64 val)
    {
    wrmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2, val);
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_set_to(idx: c_uint, val: u64) -> __always_inline void {
    static __always_inline void amd_pmu_lbr_set_to(unsigned int idx, u64 val)
    {
    wrmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2 + 1, val);
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_get_from(idx: c_uint) -> __always_inline u64 {
    static __always_inline u64 amd_pmu_lbr_get_from(unsigned int idx)
    {
    u64 val;
    rdmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_get_to(idx: c_uint) -> __always_inline u64 {
    static __always_inline u64 amd_pmu_lbr_get_to(unsigned int idx)
    {
    u64 val;
    rdmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2 + 1, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn sign_ext_branch_ip(ip: u64) -> __always_inline u64 {
    static __always_inline u64 sign_ext_branch_ip(u64 ip)
    {
    let mut shift: u32 = 64 - boot_cpu_data.x86_virt_bits;
    return (u64)(((s64)ip << shift) >> shift);
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_filter() {
    static void amd_pmu_lbr_filter(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    let mut br_sel: c_int = cpuc.br_sel, offset, type, i, j;
    let mut compress: bool = false;
    let mut fused_only: bool = false;
    u64 from, to;
// If sampling all branches, there is nothing to filter
    if (((br_sel & X86_BR_ALL) == X86_BR_ALL) &&
    ((br_sel & X86_BR_TYPE_SAVE) != X86_BR_TYPE_SAVE))
    fused_only = true;
    for (i = 0; i < cpuc.lbr_stack.nr; i++) {
    from = cpuc.lbr_entries[i].from;
    to = cpuc.lbr_entries[i].to;
    type = branch_type_fused(from, to, 0, &offset);
//
// Adjust the branch from address in case of instruction
// fusion where it points to an instruction preceding the
// actual branch
//
    if (offset) {
    cpuc.lbr_entries[i].from += offset;
    if (fused_only)
    continue;
    }
// If type does not correspond, then discard
    if (type == X86_BR_NONE || (br_sel & type) != type ||
    (!(br_sel & X86_BR_KERNEL) && kernel_ip(cpuc.lbr_entries[i].from))) {
    cpuc.lbr_entries[i].from = 0;	/* mark invalid */
    compress = true;
    }
    if ((br_sel & X86_BR_TYPE_SAVE) == X86_BR_TYPE_SAVE)
    cpuc.lbr_entries[i].type = common_branch_type(type);
    }
    if (!compress)
    return;
// Remove all invalid entries
    for (i = 0; i < cpuc.lbr_stack.nr; ) {
    if (!cpuc.lbr_entries[i].from) {
    j = i;
    while (++j < cpuc.lbr_stack.nr)
    cpuc.lbr_entries[j - 1] = cpuc.lbr_entries[j];
    cpuc.lbr_stack.nr--;
    if (!cpuc.lbr_entries[i].from)
    continue;
    }
    i++;
    }
    }
    static const int lbr_spec_map[PERF_BR_SPEC_MAX] = {
    PERF_BR_SPEC_NA,
    PERF_BR_SPEC_WRONG_PATH,
    PERF_BR_NON_SPEC_CORRECT_PATH,
    PERF_BR_SPEC_CORRECT_PATH,
    };
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_read() {
    void amd_pmu_lbr_read(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    struct perf_branch_entry *br = cpuc.lbr_entries;
    struct branch_entry entry;
    let mut out: c_int = 0, idx, i;
    if (!cpuc.lbr_users)
    return;
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    entry.from.full	= amd_pmu_lbr_get_from(i);
    entry.to.full	= amd_pmu_lbr_get_to(i);
//
// Check if a branch has been logged; if valid = 0, spec = 0
// then no branch was recorded; if reserved = 1 then an
// erroneous branch was recorded (see Erratum 1452)
//
    if ((!entry.to.split.valid && !entry.to.split.spec) ||
    entry.to.split.reserved)
    continue;
    perf_clear_branch_entry_bitfields(br + out);
    br[out].from	= sign_ext_branch_ip(entry.from.split.ip);
    br[out].to	= sign_ext_branch_ip(entry.to.split.ip);
    br[out].mispred	= entry.from.split.mispredict;
    br[out].predicted = !br[out].mispred;
//
// Set branch speculation information using the status of
// the valid and spec bits.
//
// When valid = 0, spec = 0, no branch was recorded and the
// entry is discarded as seen above.
//
// When valid = 0, spec = 1, the recorded branch was
// speculative but took the wrong path.
//
// When valid = 1, spec = 0, the recorded branch was
// non-speculative but took the correct path.
//
// When valid = 1, spec = 1, the recorded branch was
// speculative and took the correct path
//
    idx = (entry.to.split.valid << 1) | entry.to.split.spec;
    br[out].spec = lbr_spec_map[idx];
    out++;
    }
    cpuc.lbr_stack.nr = out;
//
// Internal register renaming always ensures that LBR From[0] and
// LBR To[0] always represent the TOS
//
    cpuc.lbr_stack.hw_idx = 0;
// Perform further software filtering
    amd_pmu_lbr_filter();
    }
    static const int lbr_select_map[PERF_SAMPLE_BRANCH_MAX_SHIFT] = {
    [PERF_SAMPLE_BRANCH_USER_SHIFT]		= LBR_USER,
    [PERF_SAMPLE_BRANCH_KERNEL_SHIFT]	= LBR_KERNEL,
    [PERF_SAMPLE_BRANCH_HV_SHIFT]		= LBR_IGNORE,
    [PERF_SAMPLE_BRANCH_ANY_SHIFT]		= LBR_ANY,
    [PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT]	= LBR_REL_CALL | LBR_IND_CALL | LBR_FAR,
    [PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT]	= LBR_RETURN | LBR_FAR,
    [PERF_SAMPLE_BRANCH_IND_CALL_SHIFT]	= LBR_IND_CALL,
    [PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT]	= LBR_NOT_SUPP,
    [PERF_SAMPLE_BRANCH_IN_TX_SHIFT]	= LBR_NOT_SUPP,
    [PERF_SAMPLE_BRANCH_NO_TX_SHIFT]	= LBR_NOT_SUPP,
    [PERF_SAMPLE_BRANCH_COND_SHIFT]		= LBR_JCC,
    [PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT]	= LBR_NOT_SUPP,
    [PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT]	= LBR_IND_JMP,
    [PERF_SAMPLE_BRANCH_CALL_SHIFT]		= LBR_REL_CALL,
    [PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT]	= LBR_NOT_SUPP,
    [PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT]	= LBR_NOT_SUPP,
    };
#[no_mangle]
unsafe extern "C" fn amd_pmu_lbr_setup_filter(event: *mut perf_event) -> c_int {
    static int amd_pmu_lbr_setup_filter(struct perf_event *event)
    {
    struct hw_perf_event_extra *reg = &event.hw.branch_reg;
    let mut br_type: u64 = event.attr.branch_sample_type;
    let mut mask: u64 = 0, v;
    int i;
// No LBR support
    if (!x86_pmu.lbr_nr)
    return -EOPNOTSUPP;
    if (br_type & PERF_SAMPLE_BRANCH_USER)
    mask |= X86_BR_USER;
    if (br_type & PERF_SAMPLE_BRANCH_KERNEL)
    mask |= X86_BR_KERNEL;
// Ignore BRANCH_HV here
    if (br_type & PERF_SAMPLE_BRANCH_ANY)
    mask |= X86_BR_ANY;
    if (br_type & PERF_SAMPLE_BRANCH_ANY_CALL)
    mask |= X86_BR_ANY_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_ANY_RETURN)
    mask |= X86_BR_RET | X86_BR_IRET | X86_BR_SYSRET;
    if (br_type & PERF_SAMPLE_BRANCH_IND_CALL)
    mask |= X86_BR_IND_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_COND)
    mask |= X86_BR_JCC;
    if (br_type & PERF_SAMPLE_BRANCH_IND_JUMP)
    mask |= X86_BR_IND_JMP;
    if (br_type & PERF_SAMPLE_BRANCH_CALL)
    mask |= X86_BR_CALL | X86_BR_ZERO_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_TYPE_SAVE)
    mask |= X86_BR_TYPE_SAVE;
    reg.reg = mask;
    mask = 0;
    for (i = 0; i < PERF_SAMPLE_BRANCH_MAX_SHIFT; i++) {
    if (!(br_type & BIT_ULL(i)))
    continue;
    v = lbr_select_map[i];
    if (v == LBR_NOT_SUPP)
    return -EOPNOTSUPP;
    if (v != LBR_IGNORE)
    mask |= v;
    }
// Filter bits operate in suppress mode
    reg.config = mask ^ LBR_SELECT_MASK;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_hw_config(event: *mut perf_event) -> c_int {
    int amd_pmu_lbr_hw_config(struct perf_event *event)
    {
    let mut ret: c_int = 0;
    ret = amd_pmu_lbr_setup_filter(event);
    if (!ret)
    event.attach_state |= PERF_ATTACH_SCHED_CB;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_reset() {
    void amd_pmu_lbr_reset(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    int i;
    if (!x86_pmu.lbr_nr)
    return;
// Reset all branch records individually
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    amd_pmu_lbr_set_from(i, 0);
    amd_pmu_lbr_set_to(i, 0);
    }
    cpuc.last_task_ctx = core::ptr::null_mut();
    cpuc.last_log_id = 0;
    wrmsrq(MSR_AMD64_LBR_SELECT, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_add(event: *mut perf_event) {
    void amd_pmu_lbr_add(struct perf_event *event)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    struct hw_perf_event_extra *reg = &event.hw.branch_reg;
    if (!x86_pmu.lbr_nr)
    return;
    if (has_branch_stack(event)) {
    cpuc.lbr_select = 1;
    cpuc.lbr_sel.config = reg.config;
    cpuc.br_sel = reg.reg;
    }
    perf_sched_cb_inc(event.pmu);
    if (!cpuc.lbr_users++ && !event.total_time_running)
    amd_pmu_lbr_reset();
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_del(event: *mut perf_event) {
    void amd_pmu_lbr_del(struct perf_event *event)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!x86_pmu.lbr_nr)
    return;
    if (has_branch_stack(event))
    cpuc.lbr_select = 0;
    cpuc.lbr_users--;
    WARN_ON_ONCE(cpuc.lbr_users < 0);
    perf_sched_cb_dec(event.pmu);
    }
    void amd_pmu_lbr_sched_task(struct perf_event_pmu_context *pmu_ctx,
    struct task_struct *task, bool sched_in)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
//
// A context switch can flip the address space and LBR entries are
// not tagged with an identifier. Hence, branches cannot be resolved
// from the old address space and the LBR records should be wiped.
//
    if (cpuc.lbr_users && sched_in)
    amd_pmu_lbr_reset();
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_enable_all() {
    void amd_pmu_lbr_enable_all(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    u64 lbr_select, dbg_ctl, dbg_extn_cfg;
    if (!cpuc.lbr_users || !x86_pmu.lbr_nr)
    return;
// Set hardware branch filter
    if (cpuc.lbr_select) {
    lbr_select = cpuc.lbr_sel.config & LBR_SELECT_MASK;
    wrmsrq(MSR_AMD64_LBR_SELECT, lbr_select);
    }
    if (cpu_feature_enabled(X86_FEATURE_AMD_LBR_PMC_FREEZE)) {
    rdmsrq(MSR_IA32_DEBUGCTLMSR, dbg_ctl);
    wrmsrq(MSR_IA32_DEBUGCTLMSR, dbg_ctl | DEBUGCTLMSR_FREEZE_LBRS_ON_PMI);
    }
    rdmsrq(MSR_AMD_DBG_EXTN_CFG, dbg_extn_cfg);
    wrmsrq(MSR_AMD_DBG_EXTN_CFG, dbg_extn_cfg | DBG_EXTN_CFG_LBRV2EN);
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_disable_all() {
    void amd_pmu_lbr_disable_all(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!cpuc.lbr_users || !x86_pmu.lbr_nr)
    return;
    __amd_pmu_lbr_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn amd_pmu_lbr_init() -> __init int {
    __init int amd_pmu_lbr_init(void)
    {
    union cpuid_0x80000022_ebx ebx;
    if (x86_pmu.version < 2 || !boot_cpu_has(X86_FEATURE_AMD_LBR_V2))
    return -EOPNOTSUPP;
// Set number of entries
    ebx.full = cpuid_ebx(EXT_PERFMON_DEBUG_FEATURES);
    x86_pmu.lbr_nr = ebx.split.lbr_v2_stack_sz;
    pr_cont("%d-deep LBR, ", x86_pmu.lbr_nr);
    return 0;
    }
