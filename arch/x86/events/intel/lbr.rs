//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/intel/lbr.c
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
// Intel LBR_SELECT bits
// Intel Vol3a, April 2011, Section 16.7 Table 16-10
//
// Hardware branch filter (not available on all CPUs)
//

//
// Following bit only exists in Linux; we mask it out before writing it to
// the actual MSR. But it helps the constraint perf code to understand
// that this is a separate configuration.
//

pub const LBR_SEL_MASK: c_uint = 0x3ff	/* valid bits in LBR_SELECT */;

    (LBR_JCC	|\
    LBR_REL_CALL	|\
    LBR_IND_CALL	|\
    LBR_RETURN	|\
    LBR_REL_JMP	|\
    LBR_IND_JMP	|\
    LBR_FAR)

//
// Intel LBR_CTL bits
//
// Hardware branch filter for Arch LBR
//

    (ARCH_LBR_JCC			|\
    ARCH_LBR_REL_JMP		|\
    ARCH_LBR_IND_JMP		|\
    ARCH_LBR_REL_CALL		|\
    ARCH_LBR_IND_CALL		|\
    ARCH_LBR_RETURN		|\
    ARCH_LBR_OTHER_BRANCH)
pub const ARCH_LBR_CTL_MASK: c_uint = 0x7f000e;
    static void intel_pmu_lbr_filter(struct cpu_hw_events *cpuc);
#[no_mangle]
unsafe extern "C" fn is_lbr_call_stack_bit_set(config: u64) -> __always_inline bool {
    static __always_inline bool is_lbr_call_stack_bit_set(u64 config)
    {
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    return !!(config & ARCH_LBR_CALL_STACK);
    return !!(config & LBR_CALL_STACK);
    }
//
// We only support LBR implementations that have FREEZE_LBRS_ON_PMI
// otherwise it becomes near impossible to get a reliable stack.
//
#[no_mangle]
unsafe extern "C" fn __intel_pmu_lbr_enable(pmi: bool) {
    static void __intel_pmu_lbr_enable(bool pmi)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    u64 debugctl, lbr_select = 0, orig_debugctl;
//
// No need to unfreeze manually, as v4 can do that as part
// of the GLOBAL_STATUS ack.
//
    if (pmi && x86_pmu.version >= 4)
    return;
//
// No need to reprogram LBR_SELECT in a PMI, as it
// did not change.
//
    if (cpuc.lbr_sel)
    lbr_select = cpuc.lbr_sel.config & x86_pmu.lbr_sel_mask;
    if (!cpu_feature_enabled(X86_FEATURE_ARCH_LBR) && !pmi && cpuc.lbr_sel)
    wrmsrq(MSR_LBR_SELECT, lbr_select);
    rdmsrq(MSR_IA32_DEBUGCTLMSR, debugctl);
    orig_debugctl = debugctl;
    if (!cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    debugctl |= DEBUGCTLMSR_LBR;
//
// LBR callstack does not work well with FREEZE_LBRS_ON_PMI.
// If FREEZE_LBRS_ON_PMI is set, PMI near call/return instructions
// may cause superfluous increase/decrease of LBR_TOS.
//
    if (is_lbr_call_stack_bit_set(lbr_select))
    debugctl &= ~DEBUGCTLMSR_FREEZE_LBRS_ON_PMI;
    else
    debugctl |= DEBUGCTLMSR_FREEZE_LBRS_ON_PMI;
    if (orig_debugctl != debugctl)
    wrmsrq(MSR_IA32_DEBUGCTLMSR, debugctl);
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    wrmsrq(MSR_ARCH_LBR_CTL, lbr_select | ARCH_LBR_CTL_LBREN);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_reset_32() {
    void intel_pmu_lbr_reset_32(void)
    {
    int i;
    for (i = 0; i < x86_pmu.lbr_nr; i++)
    wrmsrq(x86_pmu.lbr_from + i, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_reset_64() {
    void intel_pmu_lbr_reset_64(void)
    {
    int i;
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    wrmsrq(x86_pmu.lbr_from + i, 0);
    wrmsrq(x86_pmu.lbr_to   + i, 0);
    if (x86_pmu.lbr_has_info)
    wrmsrq(x86_pmu.lbr_info + i, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_reset() {
    static void intel_pmu_arch_lbr_reset(void)
    {
// Write to ARCH_LBR_DEPTH MSR, all LBR entries are reset to 0
    wrmsrq(MSR_ARCH_LBR_DEPTH, x86_pmu.lbr_nr);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_reset() {
    void intel_pmu_lbr_reset(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!x86_pmu.lbr_nr)
    return;
    x86_pmu.lbr_reset();
    cpuc.last_task_ctx = core::ptr::null_mut();
    cpuc.last_log_id = 0;
    if (!cpu_feature_enabled(X86_FEATURE_ARCH_LBR) && cpuc.lbr_select)
    wrmsrq(MSR_LBR_SELECT, 0);
    }
//
// TOS = most recently recorded branch
//
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_tos() -> u64 {
    static inline u64 intel_pmu_lbr_tos(void)
    {
    u64 tos;
    rdmsrq(x86_pmu.lbr_tos, tos);
    return tos;
    }
    enum {
    LBR_NONE,
    LBR_VALID,
    };
//
// For format LBR_FORMAT_EIP_FLAGS2, bits 61:62 in MSR_LAST_BRANCH_FROM_x
// are the TSX flags when TSX is supported, but when TSX is not supported
// they have no consistent behavior:
//
// - For wrmsr(), bits 61:62 are considered part of the sign extension.
// - For HW updates (branch captures) bits 61:62 are always OFF and are not
// part of the sign extension.
//
// Therefore, if:
//
// 1) LBR format LBR_FORMAT_EIP_FLAGS2
// 2) CPU has no TSX support enabled
//
// ... then any value passed to wrmsr() must be sign extended to 63 bits and any
// value from rdmsr() must be converted to have a 61 bits sign extension,
// ignoring the TSX flags.
//
#[no_mangle]
pub unsafe extern "C" fn lbr_from_signext_quirk_needed() -> bool {
    static inline bool lbr_from_signext_quirk_needed(void)
    {
    bool tsx_support = boot_cpu_has(X86_FEATURE_HLE) ||
    boot_cpu_has(X86_FEATURE_RTM);
    return !tsx_support;
    }
    static DEFINE_STATIC_KEY_FALSE(lbr_from_quirk_key);
// If quirk is enabled, ensure sign extension is 63 bits:
#[no_mangle]
pub unsafe extern "C" fn lbr_from_signext_quirk_wr(val: u64) -> u64 {
    inline u64 lbr_from_signext_quirk_wr(u64 val)
    {
    if (static_branch_unlikely(&lbr_from_quirk_key)) {
//
// Sign extend into bits 61:62 while preserving bit 63.
//
// Quirk is enabled when TSX is disabled. Therefore TSX bits
// in val are always OFF and must be changed to be sign
// extension bits. Since bits 59:60 are guaranteed to be
// part of the sign extension bits, we can just copy them
// to 61:62.
//
    val |= (LBR_FROM_SIGNEXT_2MSB & val) << 2;
    }
    return val;
    }
//
// If quirk is needed, ensure sign extension is 61 bits:
//
#[no_mangle]
unsafe extern "C" fn lbr_from_signext_quirk_rd(val: u64) -> u64 {
    static u64 lbr_from_signext_quirk_rd(u64 val)
    {
    if (static_branch_unlikely(&lbr_from_quirk_key)) {
//
// Quirk is on when TSX is not enabled. Therefore TSX
// flags must be read as OFF.
//
    val &= ~(LBR_FROM_FLAG_IN_TX | LBR_FROM_FLAG_ABORT);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn wrlbr_from(idx: c_uint, val: u64) -> __always_inline void {
    static __always_inline void wrlbr_from(unsigned int idx, u64 val)
    {
    val = lbr_from_signext_quirk_wr(val);
    wrmsrq(x86_pmu.lbr_from + idx, val);
    }
#[no_mangle]
unsafe extern "C" fn wrlbr_to(idx: c_uint, val: u64) -> __always_inline void {
    static __always_inline void wrlbr_to(unsigned int idx, u64 val)
    {
    wrmsrq(x86_pmu.lbr_to + idx, val);
    }
#[no_mangle]
unsafe extern "C" fn wrlbr_info(idx: c_uint, val: u64) -> __always_inline void {
    static __always_inline void wrlbr_info(unsigned int idx, u64 val)
    {
    wrmsrq(x86_pmu.lbr_info + idx, val);
    }
#[no_mangle]
unsafe extern "C" fn rdlbr_from(idx: c_uint, lbr: *mut lbr_entry) -> __always_inline u64 {
    static __always_inline u64 rdlbr_from(unsigned int idx, struct lbr_entry *lbr)
    {
    u64 val;
    if (lbr)
    return lbr.from;
    rdmsrq(x86_pmu.lbr_from + idx, val);
    return lbr_from_signext_quirk_rd(val);
    }
#[no_mangle]
unsafe extern "C" fn rdlbr_to(idx: c_uint, lbr: *mut lbr_entry) -> __always_inline u64 {
    static __always_inline u64 rdlbr_to(unsigned int idx, struct lbr_entry *lbr)
    {
    u64 val;
    if (lbr)
    return lbr.to;
    rdmsrq(x86_pmu.lbr_to + idx, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn rdlbr_info(idx: c_uint, lbr: *mut lbr_entry) -> __always_inline u64 {
    static __always_inline u64 rdlbr_info(unsigned int idx, struct lbr_entry *lbr)
    {
    u64 val;
    if (lbr)
    return lbr.info;
    rdmsrq(x86_pmu.lbr_info + idx, val);
    return val;
    }
    static inline void
    wrlbr_all(struct lbr_entry *lbr, unsigned int idx, bool need_info)
    {
    wrlbr_from(idx, lbr.from);
    wrlbr_to(idx, lbr.to);
    if (need_info)
    wrlbr_info(idx, lbr.info);
    }
    static inline bool
    rdlbr_all(struct lbr_entry *lbr, unsigned int idx, bool need_info)
    {
    let mut from: u64 = rdlbr_from(idx, core::ptr::null_mut());
// Don't read invalid entry
    if (!from)
    return false;
    lbr.from = from;
    lbr.to = rdlbr_to(idx, core::ptr::null_mut());
    if (need_info)
    lbr.info = rdlbr_info(idx, core::ptr::null_mut());
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_restore(ctx: *mut c_void) {
    void intel_pmu_lbr_restore(void *ctx)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    struct x86_perf_task_context *task_ctx = ctx;
    let mut need_info: bool = x86_pmu.lbr_has_info;
    let mut tos: u64 = task_ctx.tos;
    unsigned lbr_idx, mask;
    int i;
    mask = x86_pmu.lbr_nr - 1;
    for (i = 0; i < task_ctx.valid_lbrs; i++) {
    lbr_idx = (tos - i) & mask;
    wrlbr_all(&task_ctx.lbr[i], lbr_idx, need_info);
    }
    for (; i < x86_pmu.lbr_nr; i++) {
    lbr_idx = (tos - i) & mask;
    wrlbr_from(lbr_idx, 0);
    wrlbr_to(lbr_idx, 0);
    if (need_info)
    wrlbr_info(lbr_idx, 0);
    }
    wrmsrq(x86_pmu.lbr_tos, tos);
    if (cpuc.lbr_select)
    wrmsrq(MSR_LBR_SELECT, task_ctx.lbr_sel);
    }
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_restore(ctx: *mut c_void) {
    static void intel_pmu_arch_lbr_restore(void *ctx)
    {
    struct x86_perf_task_context_arch_lbr *task_ctx = ctx;
    struct lbr_entry *entries = task_ctx.entries;
    int i;
// Fast reset the LBRs before restore if the call stack is not full.
    if (!entries[x86_pmu.lbr_nr - 1].from)
    intel_pmu_arch_lbr_reset();
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    if (!entries[i].from)
    break;
    wrlbr_all(&entries[i], i, true);
    }
    }
//
// Restore the Architecture LBR state from the xsave area in the perf
// context data for the task via the XRSTORS instruction.
//
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_xrstors(ctx: *mut c_void) {
    static void intel_pmu_arch_lbr_xrstors(void *ctx)
    {
    struct x86_perf_task_context_arch_lbr_xsave *task_ctx = ctx;
    xrstors(&task_ctx.xsave, XFEATURE_MASK_LBR);
    }
#[no_mangle]
unsafe extern "C" fn lbr_is_reset_in_cstate(ctx: *mut c_void) -> __always_inline bool {
    static __always_inline bool lbr_is_reset_in_cstate(void *ctx)
    {
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    return x86_pmu.lbr_deep_c_reset && !rdlbr_from(0, core::ptr::null_mut());
    return !rdlbr_from(((struct x86_perf_task_context *)ctx).tos, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn has_lbr_callstack_users(ctx: *mut c_void) -> bool {
    static inline bool has_lbr_callstack_users(void *ctx)
    {
    return task_context_opt(ctx).lbr_callstack_users ||
    x86_pmu.lbr_callstack_users;
    }
#[no_mangle]
unsafe extern "C" fn __intel_pmu_lbr_restore(ctx: *mut c_void) {
    static void __intel_pmu_lbr_restore(void *ctx)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!has_lbr_callstack_users(ctx) ||
    task_context_opt(ctx).lbr_stack_state == LBR_NONE) {
    intel_pmu_lbr_reset();
    return;
    }
//
// Does not restore the LBR registers, if
// - No one else touched them, and
// - Was not cleared in Cstate
//
    if ((ctx == cpuc.last_task_ctx) &&
    (task_context_opt(ctx).log_id == cpuc.last_log_id) &&
    !lbr_is_reset_in_cstate(ctx)) {
    task_context_opt(ctx).lbr_stack_state = LBR_NONE;
    return;
    }
    x86_pmu.lbr_restore(ctx);
    task_context_opt(ctx).lbr_stack_state = LBR_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_save(ctx: *mut c_void) {
    void intel_pmu_lbr_save(void *ctx)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    struct x86_perf_task_context *task_ctx = ctx;
    let mut need_info: bool = x86_pmu.lbr_has_info;
    unsigned lbr_idx, mask;
    u64 tos;
    int i;
    mask = x86_pmu.lbr_nr - 1;
    tos = intel_pmu_lbr_tos();
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    lbr_idx = (tos - i) & mask;
    if (!rdlbr_all(&task_ctx.lbr[i], lbr_idx, need_info))
    break;
    }
    task_ctx.valid_lbrs = i;
    task_ctx.tos = tos;
    if (cpuc.lbr_select)
    rdmsrq(MSR_LBR_SELECT, task_ctx.lbr_sel);
    }
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_save(ctx: *mut c_void) {
    static void intel_pmu_arch_lbr_save(void *ctx)
    {
    struct x86_perf_task_context_arch_lbr *task_ctx = ctx;
    struct lbr_entry *entries = task_ctx.entries;
    int i;
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    if (!rdlbr_all(&entries[i], i, true))
    break;
    }
// LBR call stack is not full. Reset is required in restore.
    if (i < x86_pmu.lbr_nr)
    entries[x86_pmu.lbr_nr - 1].from = 0;
    }
//
// Save the Architecture LBR state to the xsave area in the perf
// context data for the task via the XSAVES instruction.
//
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_xsaves(ctx: *mut c_void) {
    static void intel_pmu_arch_lbr_xsaves(void *ctx)
    {
    struct x86_perf_task_context_arch_lbr_xsave *task_ctx = ctx;
    xsaves(&task_ctx.xsave, XFEATURE_MASK_LBR);
    }
#[no_mangle]
unsafe extern "C" fn __intel_pmu_lbr_save(ctx: *mut c_void) {
    static void __intel_pmu_lbr_save(void *ctx)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!has_lbr_callstack_users(ctx)) {
    task_context_opt(ctx).lbr_stack_state = LBR_NONE;
    return;
    }
    x86_pmu.lbr_save(ctx);
    task_context_opt(ctx).lbr_stack_state = LBR_VALID;
    cpuc.last_task_ctx = ctx;
    cpuc.last_log_id = ++task_context_opt(ctx).log_id;
    }
    void intel_pmu_lbr_sched_task(struct perf_event_pmu_context *pmu_ctx,
    struct task_struct *task, bool sched_in)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    struct perf_ctx_data *ctx_data;
    void *task_ctx;
    if (!cpuc.lbr_users)
    return;
//
// If LBR callstack feature is enabled and the stack was saved when
// the task was scheduled out, restore the stack. Otherwise flush
// the LBR stack.
//
    rcu_read_lock();
    ctx_data = rcu_dereference(task.perf_ctx_data);
    task_ctx = ctx_data ? ctx_data.data : core::ptr::null_mut();
    if (task_ctx) {
    if (sched_in)
    __intel_pmu_lbr_restore(task_ctx);
    else
    __intel_pmu_lbr_save(task_ctx);
    rcu_read_unlock();
    return;
    }
    rcu_read_unlock();
//
// Since a context switch can flip the address space and LBR entries
// are not tagged with an identifier, we need to wipe the LBR, even for
// per-cpu events. You simply cannot resolve the branches from the old
// address space.
//
    if (sched_in)
    intel_pmu_lbr_reset();
    }
#[no_mangle]
pub unsafe extern "C" fn branch_user_callstack(br_sel: unsigned) -> bool {
    static inline bool branch_user_callstack(unsigned br_sel)
    {
    return (br_sel & X86_BR_USER) && (br_sel & X86_BR_CALL_STACK);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_add(event: *mut perf_event) {
    void intel_pmu_lbr_add(struct perf_event *event)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!x86_pmu.lbr_nr)
    return;
    if (event.hw.flags & PERF_X86_EVENT_LBR_SELECT)
    cpuc.lbr_select = 1;
    cpuc.br_sel = event.hw.branch_reg.reg;
    if (branch_user_callstack(cpuc.br_sel)) {
    if (event.attach_state & PERF_ATTACH_TASK) {
    struct task_struct *task = event.hw.target;
    struct perf_ctx_data *ctx_data;
    rcu_read_lock();
    ctx_data = rcu_dereference(task.perf_ctx_data);
    if (ctx_data)
    task_context_opt(ctx_data.data).lbr_callstack_users++;
    rcu_read_unlock();
    } else
    x86_pmu.lbr_callstack_users++;
    }
//
// Request pmu::sched_task() callback, which will fire inside the
// regular perf event scheduling, so that call will:
//
// - restore or wipe; when LBR-callstack,
// - wipe; otherwise,
//
// when this is from __perf_event_task_sched_in().
//
// However, if this is from perf_install_in_context(), no such callback
// will follow and we'll need to reset the LBR here if this is the
// first LBR event.
//
// The problem is, we cannot tell these cases apart... but we can
// exclude the biggest chunk of cases by looking at
// event->total_time_running. An event that has accrued runtime cannot
// be 'new'. Conversely, a new event can get installed through the
// context switch path for the first time.
//
    if (x86_pmu.intel_cap.pebs_baseline && event.attr.precise_ip > 0)
    cpuc.lbr_pebs_users++;
    perf_sched_cb_inc(event.pmu);
    if (!cpuc.lbr_users++ && !event.total_time_running)
    intel_pmu_lbr_reset();
    }
#[no_mangle]
pub unsafe extern "C" fn release_lbr_buffers() {
    void release_lbr_buffers(void)
    {
    struct kmem_cache *kmem_cache;
    struct cpu_hw_events *cpuc;
    int cpu;
    if (!cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    return;
    for_each_possible_cpu(cpu) {
    cpuc = per_cpu_ptr(&cpu_hw_events, cpu);
    kmem_cache = x86_get_pmu(cpu).task_ctx_cache;
    if (kmem_cache && cpuc.lbr_xsave) {
    kmem_cache_free(kmem_cache, cpuc.lbr_xsave);
    cpuc.lbr_xsave = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn reserve_lbr_buffers() {
    void reserve_lbr_buffers(void)
    {
    struct kmem_cache *kmem_cache;
    struct cpu_hw_events *cpuc;
    int cpu;
    if (!cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    return;
    for_each_possible_cpu(cpu) {
    cpuc = per_cpu_ptr(&cpu_hw_events, cpu);
    kmem_cache = x86_get_pmu(cpu).task_ctx_cache;
    if (!kmem_cache || cpuc.lbr_xsave)
    continue;
    cpuc.lbr_xsave = kmem_cache_alloc_node(kmem_cache,
    GFP_KERNEL | __GFP_ZERO,
    cpu_to_node(cpu));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_del(event: *mut perf_event) {
    void intel_pmu_lbr_del(struct perf_event *event)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (!x86_pmu.lbr_nr)
    return;
    if (branch_user_callstack(cpuc.br_sel)) {
    if (event.attach_state & PERF_ATTACH_TASK) {
    struct task_struct *task = event.hw.target;
    struct perf_ctx_data *ctx_data;
    rcu_read_lock();
    ctx_data = rcu_dereference(task.perf_ctx_data);
    if (ctx_data)
    task_context_opt(ctx_data.data).lbr_callstack_users--;
    rcu_read_unlock();
    } else
    x86_pmu.lbr_callstack_users--;
    }
    if (event.hw.flags & PERF_X86_EVENT_LBR_SELECT)
    cpuc.lbr_select = 0;
    if (x86_pmu.intel_cap.pebs_baseline && event.attr.precise_ip > 0)
    cpuc.lbr_pebs_users--;
    cpuc.lbr_users--;
    WARN_ON_ONCE(cpuc.lbr_users < 0);
    WARN_ON_ONCE(cpuc.lbr_pebs_users < 0);
    perf_sched_cb_dec(event.pmu);
//
// The logged occurrences information is only valid for the
// current LBR group. If another LBR group is scheduled in
// later, the information from the stale LBRs will be wrongly
// interpreted. Reset the LBRs here.
//
// Only clear once for a branch counter group with the leader
// event. Because
// - Cannot simply reset the LBRs with the !cpuc->lbr_users.
// Because it's possible that the last LBR user is not in a
// branch counter group, e.g., a branch_counters group +
// several normal LBR events.
// - The LBR reset can be done with any one of the events in a
// branch counter group, since they are always scheduled together.
// It's easy to force the leader event an LBR event.
//
    if (is_branch_counters_group(event) && event == event.group_leader)
    intel_pmu_lbr_reset();
    }
#[no_mangle]
pub unsafe extern "C" fn vlbr_exclude_host() -> bool {
    static inline bool vlbr_exclude_host(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    return test_bit(INTEL_PMC_IDX_FIXED_VLBR,
    (unsigned long *)&cpuc.intel_ctrl_guest_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_enable_all(pmi: bool) {
    void intel_pmu_lbr_enable_all(bool pmi)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (cpuc.lbr_users && !vlbr_exclude_host())
    __intel_pmu_lbr_enable(pmi);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_disable_all() {
    void intel_pmu_lbr_disable_all(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
    if (cpuc.lbr_users && !vlbr_exclude_host()) {
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR))
    return __intel_pmu_arch_lbr_disable();
    __intel_pmu_lbr_disable();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_read_32(cpuc: *mut cpu_hw_events) {
    void intel_pmu_lbr_read_32(struct cpu_hw_events *cpuc)
    {
    let mut mask: c_ulong = x86_pmu.lbr_nr - 1;
    struct perf_branch_entry *br = cpuc.lbr_entries;
    let mut tos: u64 = intel_pmu_lbr_tos();
    int i;
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    let mut lbr_idx: c_ulong = (tos - i) & mask;
    union {
    struct {
    u32 from;
    u32 to;
    };
    u64     lbr;
    } msr_lastbranch;
    rdmsrq(x86_pmu.lbr_from + lbr_idx, msr_lastbranch.lbr);
    perf_clear_branch_entry_bitfields(br);
    br.from	= msr_lastbranch.from;
    br.to		= msr_lastbranch.to;
    br++;
    }
    cpuc.lbr_stack.nr = i;
    cpuc.lbr_stack.hw_idx = tos;
    }
//
// Due to lack of segmentation in Linux the effective address (offset)
// is the same as the linear address, allowing us to merge the LIP and EIP
// LBR formats.
//
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_read_64(cpuc: *mut cpu_hw_events) {
    void intel_pmu_lbr_read_64(struct cpu_hw_events *cpuc)
    {
    let mut need_info: bool = false, call_stack = false;
    let mut mask: c_ulong = x86_pmu.lbr_nr - 1;
    struct perf_branch_entry *br = cpuc.lbr_entries;
    let mut tos: u64 = intel_pmu_lbr_tos();
    int i;
    let mut out: c_int = 0;
    let mut num: c_int = x86_pmu.lbr_nr;
    if (cpuc.lbr_sel) {
    need_info = !(cpuc.lbr_sel.config & LBR_NO_INFO);
    if (cpuc.lbr_sel.config & LBR_CALL_STACK)
    call_stack = true;
    }
    for (i = 0; i < num; i++) {
    let mut lbr_idx: c_ulong = (tos - i) & mask;
    u64 from, to, mis = 0, pred = 0, in_tx = 0, abort = 0;
    let mut cycles: u16 = 0;
    from = rdlbr_from(lbr_idx, core::ptr::null_mut());
    to   = rdlbr_to(lbr_idx, core::ptr::null_mut());
//
// Read LBR call stack entries
// until invalid entry (0s) is detected.
//
    if (call_stack && !from)
    break;
    if (x86_pmu.lbr_has_info) {
    if (need_info) {
    u64 info;
    info = rdlbr_info(lbr_idx, core::ptr::null_mut());
    mis = !!(info & LBR_INFO_MISPRED);
    pred = !mis;
    cycles = (info & LBR_INFO_CYCLES);
    if (x86_pmu.lbr_has_tsx) {
    in_tx = !!(info & LBR_INFO_IN_TX);
    abort = !!(info & LBR_INFO_ABORT);
    }
    }
    } else {
    let mut skip: c_int = 0;
    if (x86_pmu.lbr_from_flags) {
    mis = !!(from & LBR_FROM_FLAG_MISPRED);
    pred = !mis;
    skip = 1;
    }
    if (x86_pmu.lbr_has_tsx) {
    in_tx = !!(from & LBR_FROM_FLAG_IN_TX);
    abort = !!(from & LBR_FROM_FLAG_ABORT);
    skip = 3;
    }
    from = (u64)((((s64)from) << skip) >> skip);
    if (x86_pmu.lbr_to_cycles) {
    cycles = ((to >> 48) & LBR_INFO_CYCLES);
    to = (u64)((((s64)to) << 16) >> 16);
    }
    }
//
// Some CPUs report duplicated abort records,
// with the second entry not having an abort bit set.
// Skip them here. This loop runs backwards,
// so we need to undo the previous record.
// If the abort just happened outside the window
// the extra entry cannot be removed.
//
    if (abort && x86_pmu.lbr_double_abort && out > 0)
    out--;
    perf_clear_branch_entry_bitfields(br+out);
    br[out].from	 = from;
    br[out].to	 = to;
    br[out].mispred	 = mis;
    br[out].predicted = pred;
    br[out].in_tx	 = in_tx;
    br[out].abort	 = abort;
    br[out].cycles	 = cycles;
    out++;
    }
    cpuc.lbr_stack.nr = out;
    cpuc.lbr_stack.hw_idx = tos;
    }
    static DEFINE_STATIC_KEY_FALSE(x86_lbr_mispred);
    static DEFINE_STATIC_KEY_FALSE(x86_lbr_cycles);
    static DEFINE_STATIC_KEY_FALSE(x86_lbr_type);
#[no_mangle]
unsafe extern "C" fn get_lbr_br_type(info: u64) -> __always_inline int {
    static __always_inline int get_lbr_br_type(u64 info)
    {
    let mut type: c_int = 0;
    if (static_branch_likely(&x86_lbr_type))
    type = (info & LBR_INFO_BR_TYPE) >> LBR_INFO_BR_TYPE_OFFSET;
    return type;
    }
#[no_mangle]
unsafe extern "C" fn get_lbr_mispred(info: u64) -> __always_inline bool {
    static __always_inline bool get_lbr_mispred(u64 info)
    {
    let mut mispred: bool = 0;
    if (static_branch_likely(&x86_lbr_mispred))
    mispred = !!(info & LBR_INFO_MISPRED);
    return mispred;
    }
#[no_mangle]
unsafe extern "C" fn get_lbr_cycles(info: u64) -> __always_inline u16 {
    static __always_inline u16 get_lbr_cycles(u64 info)
    {
    let mut cycles: u16 = info & LBR_INFO_CYCLES;
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR) &&
    (!static_branch_likely(&x86_lbr_cycles) ||
    !(info & LBR_INFO_CYC_CNT_VALID)))
    cycles = 0;
    return cycles;
    }
    static_assert((64 - PERF_BRANCH_ENTRY_INFO_BITS_MAX) > LBR_INFO_BR_CNTR_NUM * LBR_INFO_BR_CNTR_BITS);
    static void intel_pmu_store_lbr(struct cpu_hw_events *cpuc,
    struct lbr_entry *entries)
    {
    struct perf_branch_entry *e;
    struct lbr_entry *lbr;
    u64 from, to, info;
    int i;
    for (i = 0; i < x86_pmu.lbr_nr; i++) {
    lbr = entries ? &entries[i] : core::ptr::null_mut();
    e = &cpuc.lbr_entries[i];
    from = rdlbr_from(i, lbr);
//
// Read LBR entries until invalid entry (0s) is detected.
//
    if (!from)
    break;
    to = rdlbr_to(i, lbr);
    info = rdlbr_info(i, lbr);
    perf_clear_branch_entry_bitfields(e);
    e.from		= from;
    e.to		= to;
    e.mispred	= get_lbr_mispred(info);
    e.predicted	= !e.mispred;
    e.in_tx	= !!(info & LBR_INFO_IN_TX);
    e.abort	= !!(info & LBR_INFO_ABORT);
    e.cycles	= get_lbr_cycles(info);
    e.type		= get_lbr_br_type(info);
//
// Leverage the reserved field of cpuc->lbr_entries[i] to
// temporarily store the branch counters information.
// The later code will decide what content can be disclosed
// to the perf tool. Pleae see intel_pmu_lbr_counters_reorder().
//
    e.reserved	= (info >> LBR_INFO_BR_CNTR_OFFSET) & LBR_INFO_BR_CNTR_FULL_MASK;
    }
    cpuc.lbr_stack.nr = i;
    }
//
// The enabled order may be different from the counter order.
// Update the lbr_counters with the enabled order.
//
    static void intel_pmu_lbr_counters_reorder(struct cpu_hw_events *cpuc,
    struct perf_event *event)
    {
    int i, j, pos = 0, order[X86_PMC_IDX_MAX];
    struct perf_event *leader, *sibling;
    u64 src, dst, cnt;
    leader = event.group_leader;
    if (branch_sample_counters(leader))
    order[pos++] = leader.hw.idx;
    for_each_sibling_event(sibling, leader) {
    if (!branch_sample_counters(sibling))
    continue;
    order[pos++] = sibling.hw.idx;
    }
    WARN_ON_ONCE(!pos);
    for (i = 0; i < cpuc.lbr_stack.nr; i++) {
    src = cpuc.lbr_entries[i].reserved;
    dst = 0;
    for (j = 0; j < pos; j++) {
    cnt = (src >> (order[j] * LBR_INFO_BR_CNTR_BITS)) & LBR_INFO_BR_CNTR_MASK;
    dst |= cnt << j * LBR_INFO_BR_CNTR_BITS;
    }
    cpuc.lbr_counters[i] = dst;
    cpuc.lbr_entries[i].reserved = 0;
    }
    }
    void intel_pmu_lbr_save_brstack(struct perf_sample_data *data,
    struct cpu_hw_events *cpuc,
    struct perf_event *event)
    {
    if (is_branch_counters_group(event)) {
    intel_pmu_lbr_counters_reorder(cpuc, event);
    perf_sample_save_brstack(data, event, &cpuc.lbr_stack, cpuc.lbr_counters);
    return;
    }
    perf_sample_save_brstack(data, event, &cpuc.lbr_stack, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_read(cpuc: *mut cpu_hw_events) {
    static void intel_pmu_arch_lbr_read(struct cpu_hw_events *cpuc)
    {
    intel_pmu_store_lbr(cpuc, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn intel_pmu_arch_lbr_read_xsave(cpuc: *mut cpu_hw_events) {
    static void intel_pmu_arch_lbr_read_xsave(struct cpu_hw_events *cpuc)
    {
    struct x86_perf_task_context_arch_lbr_xsave *xsave = cpuc.lbr_xsave;
    if (!xsave) {
    intel_pmu_store_lbr(cpuc, core::ptr::null_mut());
    return;
    }
    xsaves(&xsave.xsave, XFEATURE_MASK_LBR);
    intel_pmu_store_lbr(cpuc, xsave.lbr.entries);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_read() {
    void intel_pmu_lbr_read(void)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
//
// Don't read when all LBRs users are using adaptive PEBS.
//
// This could be smarter and actually check the event,
// but this simple approach seems to work for now.
//
    if (!cpuc.lbr_users || vlbr_exclude_host() ||
    cpuc.lbr_users == cpuc.lbr_pebs_users)
    return;
    x86_pmu.lbr_read(cpuc);
    intel_pmu_lbr_filter(cpuc);
    }
//
// SW filter is used:
// - in case there is no HW filter
// - in case the HW filter has errata or limitations
//
#[no_mangle]
unsafe extern "C" fn intel_pmu_setup_sw_lbr_filter(event: *mut perf_event) -> c_int {
    static int intel_pmu_setup_sw_lbr_filter(struct perf_event *event)
    {
    let mut br_type: u64 = event.attr.branch_sample_type;
    let mut mask: c_int = 0;
    if (br_type & PERF_SAMPLE_BRANCH_USER)
    mask |= X86_BR_USER;
    if (br_type & PERF_SAMPLE_BRANCH_KERNEL)
    mask |= X86_BR_KERNEL;
// we ignore BRANCH_HV here
    if (br_type & PERF_SAMPLE_BRANCH_ANY)
    mask |= X86_BR_ANY;
    if (br_type & PERF_SAMPLE_BRANCH_ANY_CALL)
    mask |= X86_BR_ANY_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_ANY_RETURN)
    mask |= X86_BR_RET | X86_BR_IRET | X86_BR_SYSRET;
    if (br_type & PERF_SAMPLE_BRANCH_IND_CALL)
    mask |= X86_BR_IND_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_ABORT_TX)
    mask |= X86_BR_ABORT;
    if (br_type & PERF_SAMPLE_BRANCH_IN_TX)
    mask |= X86_BR_IN_TX;
    if (br_type & PERF_SAMPLE_BRANCH_NO_TX)
    mask |= X86_BR_NO_TX;
    if (br_type & PERF_SAMPLE_BRANCH_COND)
    mask |= X86_BR_JCC;
    if (br_type & PERF_SAMPLE_BRANCH_CALL_STACK) {
    if (!x86_pmu_has_lbr_callstack())
    return -EOPNOTSUPP;
    if (mask & ~(X86_BR_USER | X86_BR_KERNEL))
    return -EINVAL;
    mask |= X86_BR_CALL | X86_BR_IND_CALL | X86_BR_RET |
    X86_BR_CALL_STACK;
    }
    if (br_type & PERF_SAMPLE_BRANCH_IND_JUMP)
    mask |= X86_BR_IND_JMP;
    if (br_type & PERF_SAMPLE_BRANCH_CALL)
    mask |= X86_BR_CALL | X86_BR_ZERO_CALL;
    if (br_type & PERF_SAMPLE_BRANCH_TYPE_SAVE)
    mask |= X86_BR_TYPE_SAVE;
//
// stash actual user request into reg, it may
// be used by fixup code for some CPU
//
    event.hw.branch_reg.reg = mask;
    return 0;
    }
//
// setup the HW LBR filter
// Used only when available, may not be enough to disambiguate
// all branches, may need the help of the SW filter
//
#[no_mangle]
unsafe extern "C" fn intel_pmu_setup_hw_lbr_filter(event: *mut perf_event) -> c_int {
    static int intel_pmu_setup_hw_lbr_filter(struct perf_event *event)
    {
    struct hw_perf_event_extra *reg;
    let mut br_type: u64 = event.attr.branch_sample_type;
    let mut mask: u64 = 0, v;
    int i;
    for (i = 0; i < PERF_SAMPLE_BRANCH_MAX_SHIFT; i++) {
    if (!(br_type & (1ULL << i)))
    continue;
    v = x86_pmu.lbr_sel_map[i];
    if (v == LBR_NOT_SUPP)
    return -EOPNOTSUPP;
    if (v != LBR_IGN)
    mask |= v;
    }
    reg = &event.hw.branch_reg;
    reg.idx = EXTRA_REG_LBR;
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR)) {
    reg.config = mask;
//
// The Arch LBR HW can retrieve the common branch types
// from the LBR_INFO. It doesn't require the high overhead
// SW disassemble.
// Enable the branch type by default for the Arch LBR.
//
    reg.reg |= X86_BR_TYPE_SAVE;
    return 0;
    }
//
// The first 9 bits (LBR_SEL_MASK) in LBR_SELECT operate
// in suppress mode. So LBR_SELECT should be set to
// (~mask & LBR_SEL_MASK) | (mask & ~LBR_SEL_MASK)
// But the 10th bit LBR_CALL_STACK does not operate
// in suppress mode.
//
    reg.config = mask ^ (x86_pmu.lbr_sel_mask & ~LBR_CALL_STACK);
    if ((br_type & PERF_SAMPLE_BRANCH_NO_CYCLES) &&
    (br_type & PERF_SAMPLE_BRANCH_NO_FLAGS) &&
    x86_pmu.lbr_has_info)
    reg.config |= LBR_NO_INFO;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_setup_lbr_filter(event: *mut perf_event) -> c_int {
    int intel_pmu_setup_lbr_filter(struct perf_event *event)
    {
    let mut ret: c_int = 0;
//
// no LBR on this PMU
//
    if (!x86_pmu.lbr_nr)
    return -EOPNOTSUPP;
//
// setup SW LBR filter
//
    ret = intel_pmu_setup_sw_lbr_filter(event);
    if (ret)
    return ret;
//
// setup HW LBR filter, if any
//
    if (x86_pmu.lbr_sel_map)
    ret = intel_pmu_setup_hw_lbr_filter(event);
    return ret;
    }
    enum {
    ARCH_LBR_BR_TYPE_JCC			= 0,
    ARCH_LBR_BR_TYPE_NEAR_IND_JMP		= 1,
    ARCH_LBR_BR_TYPE_NEAR_REL_JMP		= 2,
    ARCH_LBR_BR_TYPE_NEAR_IND_CALL		= 3,
    ARCH_LBR_BR_TYPE_NEAR_REL_CALL		= 4,
    ARCH_LBR_BR_TYPE_NEAR_RET		= 5,
    ARCH_LBR_BR_TYPE_KNOWN_MAX		= ARCH_LBR_BR_TYPE_NEAR_RET,
    ARCH_LBR_BR_TYPE_MAP_MAX		= 16,
    };
    static const int arch_lbr_br_type_map[ARCH_LBR_BR_TYPE_MAP_MAX] = {
    [ARCH_LBR_BR_TYPE_JCC]			= X86_BR_JCC,
    [ARCH_LBR_BR_TYPE_NEAR_IND_JMP]		= X86_BR_IND_JMP,
    [ARCH_LBR_BR_TYPE_NEAR_REL_JMP]		= X86_BR_JMP,
    [ARCH_LBR_BR_TYPE_NEAR_IND_CALL]	= X86_BR_IND_CALL,
    [ARCH_LBR_BR_TYPE_NEAR_REL_CALL]	= X86_BR_CALL,
    [ARCH_LBR_BR_TYPE_NEAR_RET]		= X86_BR_RET,
    };
//
// implement actual branch filter based on user demand.
// Hardware may not exactly satisfy that request, thus
// we need to inspect opcodes. Mismatched branches are
// discarded. Therefore, the number of branches returned
// in PERF_SAMPLE_BRANCH_STACK sample may vary.
//
    static void
    intel_pmu_lbr_filter(struct cpu_hw_events *cpuc)
    {
    u64 from, to;
    let mut br_sel: c_int = cpuc.br_sel;
    int i, j, type, from_plm, to_plm;
    let mut compress: bool = false;
// if sampling all branches, then nothing to filter
    if (((br_sel & X86_BR_ALL) == X86_BR_ALL) &&
    ((br_sel & X86_BR_TYPE_SAVE) != X86_BR_TYPE_SAVE))
    return;
    for (i = 0; i < cpuc.lbr_stack.nr; i++) {
    from = cpuc.lbr_entries[i].from;
    to = cpuc.lbr_entries[i].to;
    type = cpuc.lbr_entries[i].type;
//
// Parse the branch type recorded in LBR_x_INFO MSR.
// Doesn't support OTHER_BRANCH decoding for now.
// OTHER_BRANCH branch type still rely on software decoding.
//
    if (static_branch_likely(&x86_lbr_type) &&
    type <= ARCH_LBR_BR_TYPE_KNOWN_MAX) {
    to_plm = kernel_ip(to) ? X86_BR_KERNEL : X86_BR_USER;
    type = arch_lbr_br_type_map[type] | to_plm;
    } else
    type = branch_type(from, to, cpuc.lbr_entries[i].abort);
    if (type != X86_BR_NONE && (br_sel & X86_BR_ANYTX)) {
    if (cpuc.lbr_entries[i].in_tx)
    type |= X86_BR_IN_TX;
    else
    type |= X86_BR_NO_TX;
    }
    from_plm = kernel_ip(from) ? X86_BR_KERNEL : X86_BR_USER;
//
// If type does not correspond, then discard.
// Specifically reject entries whose from address is in
// kernel space when only X86_BR_USER is requested.
//
    if (type == X86_BR_NONE || (br_sel & type) != type ||
    (!(br_sel & X86_BR_KERNEL) && (from_plm & X86_BR_KERNEL))) {
    cpuc.lbr_entries[i].from = 0;
    compress = true;
    }
    if ((br_sel & X86_BR_TYPE_SAVE) == X86_BR_TYPE_SAVE)
    cpuc.lbr_entries[i].type = common_branch_type(type);
    }
    if (!compress)
    return;
// remove all entries with from=0
    for (i = 0; i < cpuc.lbr_stack.nr; ) {
    if (!cpuc.lbr_entries[i].from) {
    j = i;
    while (++j < cpuc.lbr_stack.nr) {
    cpuc.lbr_entries[j-1] = cpuc.lbr_entries[j];
    cpuc.lbr_counters[j-1] = cpuc.lbr_counters[j];
    }
    cpuc.lbr_stack.nr--;
    if (!cpuc.lbr_entries[i].from)
    continue;
    }
    i++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_store_pebs_lbrs(lbr: *mut lbr_entry) {
    void intel_pmu_store_pebs_lbrs(struct lbr_entry *lbr)
    {
    struct cpu_hw_events *cpuc = this_cpu_ptr(&cpu_hw_events);
// Cannot get TOS for large PEBS and Arch LBR
    if (cpu_feature_enabled(X86_FEATURE_ARCH_LBR) ||
    (cpuc.n_pebs == cpuc.n_large_pebs))
    cpuc.lbr_stack.hw_idx = -1ULL;
    else
    cpuc.lbr_stack.hw_idx = intel_pmu_lbr_tos();
    intel_pmu_store_lbr(cpuc, lbr);
    intel_pmu_lbr_filter(cpuc);
    }
//
// Map interface branch filters onto LBR filters
//
    static const int nhm_lbr_sel_map[PERF_SAMPLE_BRANCH_MAX_SHIFT] = {
    [PERF_SAMPLE_BRANCH_ANY_SHIFT]		= LBR_ANY,
    [PERF_SAMPLE_BRANCH_USER_SHIFT]		= LBR_USER,
    [PERF_SAMPLE_BRANCH_KERNEL_SHIFT]	= LBR_KERNEL,
    [PERF_SAMPLE_BRANCH_HV_SHIFT]		= LBR_IGN,
    [PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT]	= LBR_RETURN | LBR_REL_JMP
    | LBR_IND_JMP | LBR_FAR,
//
// NHM/WSM erratum: must include REL_JMP+IND_JMP to get CALL branches
//
    [PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT] =
    LBR_REL_CALL | LBR_IND_CALL | LBR_REL_JMP | LBR_IND_JMP | LBR_FAR,
//
// NHM/WSM erratum: must include IND_JMP to capture IND_CALL
//
    [PERF_SAMPLE_BRANCH_IND_CALL_SHIFT] = LBR_IND_CALL | LBR_IND_JMP,
    [PERF_SAMPLE_BRANCH_COND_SHIFT]     = LBR_JCC,
    [PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT] = LBR_IND_JMP,
    };
    static const int snb_lbr_sel_map[PERF_SAMPLE_BRANCH_MAX_SHIFT] = {
    [PERF_SAMPLE_BRANCH_ANY_SHIFT]		= LBR_ANY,
    [PERF_SAMPLE_BRANCH_USER_SHIFT]		= LBR_USER,
    [PERF_SAMPLE_BRANCH_KERNEL_SHIFT]	= LBR_KERNEL,
    [PERF_SAMPLE_BRANCH_HV_SHIFT]		= LBR_IGN,
    [PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT]	= LBR_RETURN | LBR_FAR,
    [PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT]	= LBR_REL_CALL | LBR_IND_CALL
    | LBR_FAR,
    [PERF_SAMPLE_BRANCH_IND_CALL_SHIFT]	= LBR_IND_CALL,
    [PERF_SAMPLE_BRANCH_COND_SHIFT]		= LBR_JCC,
    [PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT]	= LBR_IND_JMP,
    [PERF_SAMPLE_BRANCH_CALL_SHIFT]		= LBR_REL_CALL,
    };
    static const int hsw_lbr_sel_map[PERF_SAMPLE_BRANCH_MAX_SHIFT] = {
    [PERF_SAMPLE_BRANCH_ANY_SHIFT]		= LBR_ANY,
    [PERF_SAMPLE_BRANCH_USER_SHIFT]		= LBR_USER,
    [PERF_SAMPLE_BRANCH_KERNEL_SHIFT]	= LBR_KERNEL,
    [PERF_SAMPLE_BRANCH_HV_SHIFT]		= LBR_IGN,
    [PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT]	= LBR_RETURN | LBR_FAR,
    [PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT]	= LBR_REL_CALL | LBR_IND_CALL
    | LBR_FAR,
    [PERF_SAMPLE_BRANCH_IND_CALL_SHIFT]	= LBR_IND_CALL,
    [PERF_SAMPLE_BRANCH_COND_SHIFT]		= LBR_JCC,
    [PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT]	= LBR_REL_CALL | LBR_IND_CALL
    | LBR_RETURN | LBR_CALL_STACK,
    [PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT]	= LBR_IND_JMP,
    [PERF_SAMPLE_BRANCH_CALL_SHIFT]		= LBR_REL_CALL,
    };
    static int arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_MAX_SHIFT] = {
    [PERF_SAMPLE_BRANCH_ANY_SHIFT]		= ARCH_LBR_ANY,
    [PERF_SAMPLE_BRANCH_USER_SHIFT]		= ARCH_LBR_USER,
    [PERF_SAMPLE_BRANCH_KERNEL_SHIFT]	= ARCH_LBR_KERNEL,
    [PERF_SAMPLE_BRANCH_HV_SHIFT]		= LBR_IGN,
    [PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT]	= ARCH_LBR_RETURN |
    ARCH_LBR_OTHER_BRANCH,
    [PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT]     = ARCH_LBR_REL_CALL |
    ARCH_LBR_IND_CALL |
    ARCH_LBR_OTHER_BRANCH,
    [PERF_SAMPLE_BRANCH_IND_CALL_SHIFT]     = ARCH_LBR_IND_CALL,
    [PERF_SAMPLE_BRANCH_COND_SHIFT]         = ARCH_LBR_JCC,
    [PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT]   = ARCH_LBR_REL_CALL |
    ARCH_LBR_IND_CALL |
    ARCH_LBR_RETURN |
    ARCH_LBR_CALL_STACK,
    [PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT]	= ARCH_LBR_IND_JMP,
    [PERF_SAMPLE_BRANCH_CALL_SHIFT]		= ARCH_LBR_REL_CALL,
    };
// core
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_core() -> void __init {
    void __init intel_pmu_lbr_init_core(void)
    {
    x86_pmu.lbr_nr     = 4;
    x86_pmu.lbr_tos    = MSR_LBR_TOS;
    x86_pmu.lbr_from   = MSR_LBR_CORE_FROM;
    x86_pmu.lbr_to     = MSR_LBR_CORE_TO;
//
// SW branch filter usage:
// - compensate for lack of HW filter
//
    }
// nehalem/westmere
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_nhm() -> void __init {
    void __init intel_pmu_lbr_init_nhm(void)
    {
    x86_pmu.lbr_nr     = 16;
    x86_pmu.lbr_tos    = MSR_LBR_TOS;
    x86_pmu.lbr_from   = MSR_LBR_NHM_FROM;
    x86_pmu.lbr_to     = MSR_LBR_NHM_TO;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = nhm_lbr_sel_map;
//
// SW branch filter usage:
// - workaround LBR_SEL errata (see above)
// - support syscall, sysret capture.
// That requires LBR_FAR but that means far
// jmp need to be filtered out
//
    }
// sandy bridge
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_snb() -> void __init {
    void __init intel_pmu_lbr_init_snb(void)
    {
    x86_pmu.lbr_nr	 = 16;
    x86_pmu.lbr_tos	 = MSR_LBR_TOS;
    x86_pmu.lbr_from = MSR_LBR_NHM_FROM;
    x86_pmu.lbr_to   = MSR_LBR_NHM_TO;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = snb_lbr_sel_map;
//
// SW branch filter usage:
// - support syscall, sysret capture.
// That requires LBR_FAR but that means far
// jmp need to be filtered out
//
    }
    static inline struct kmem_cache *
    create_lbr_kmem_cache(size_t size, size_t align)
    {
    return kmem_cache_create("x86_lbr", size, align, 0, core::ptr::null_mut());
    }
// haswell
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_hsw() {
    void intel_pmu_lbr_init_hsw(void)
    {
    let mut size: usize = sizeof(struct x86_perf_task_context);
    x86_pmu.lbr_nr	 = 16;
    x86_pmu.lbr_tos	 = MSR_LBR_TOS;
    x86_pmu.lbr_from = MSR_LBR_NHM_FROM;
    x86_pmu.lbr_to   = MSR_LBR_NHM_TO;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = hsw_lbr_sel_map;
    x86_get_pmu(smp_processor_id()).task_ctx_cache = create_lbr_kmem_cache(size, 0);
    }
// skylake
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_skl() -> __init void {
    __init void intel_pmu_lbr_init_skl(void)
    {
    let mut size: usize = sizeof(struct x86_perf_task_context);
    x86_pmu.lbr_nr	 = 32;
    x86_pmu.lbr_tos	 = MSR_LBR_TOS;
    x86_pmu.lbr_from = MSR_LBR_NHM_FROM;
    x86_pmu.lbr_to   = MSR_LBR_NHM_TO;
    x86_pmu.lbr_info = MSR_LBR_INFO_0;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = hsw_lbr_sel_map;
    x86_get_pmu(smp_processor_id()).task_ctx_cache = create_lbr_kmem_cache(size, 0);
//
// SW branch filter usage:
// - support syscall, sysret capture.
// That requires LBR_FAR but that means far
// jmp need to be filtered out
//
    }
// atom
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_atom() -> void __init {
    void __init intel_pmu_lbr_init_atom(void)
    {
//
// only models starting at stepping 10 seems
// to have an operational LBR which can freeze
// on PMU interrupt
//
    if (boot_cpu_data.x86_vfm == INTEL_ATOM_BONNELL
    && boot_cpu_data.x86_stepping < 10) {
    pr_cont("LBR disabled due to erratum");
    return;
    }
    x86_pmu.lbr_nr	   = 8;
    x86_pmu.lbr_tos    = MSR_LBR_TOS;
    x86_pmu.lbr_from   = MSR_LBR_CORE_FROM;
    x86_pmu.lbr_to     = MSR_LBR_CORE_TO;
//
// SW branch filter usage:
// - compensate for lack of HW filter
//
    }
// slm
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_slm() -> void __init {
    void __init intel_pmu_lbr_init_slm(void)
    {
    x86_pmu.lbr_nr	   = 8;
    x86_pmu.lbr_tos    = MSR_LBR_TOS;
    x86_pmu.lbr_from   = MSR_LBR_CORE_FROM;
    x86_pmu.lbr_to     = MSR_LBR_CORE_TO;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = nhm_lbr_sel_map;
//
// SW branch filter usage:
// - compensate for lack of HW filter
//
    pr_cont("8-deep LBR, ");
    }
// Knights Landing
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init_knl() {
    void intel_pmu_lbr_init_knl(void)
    {
    x86_pmu.lbr_nr	   = 8;
    x86_pmu.lbr_tos    = MSR_LBR_TOS;
    x86_pmu.lbr_from   = MSR_LBR_NHM_FROM;
    x86_pmu.lbr_to     = MSR_LBR_NHM_TO;
    x86_pmu.lbr_sel_mask = LBR_SEL_MASK;
    x86_pmu.lbr_sel_map  = snb_lbr_sel_map;
// Knights Landing does have MISPREDICT bit
    if (x86_pmu.intel_cap.lbr_format == LBR_FORMAT_LIP)
    x86_pmu.intel_cap.lbr_format = LBR_FORMAT_EIP_FLAGS;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_lbr_init() {
    void intel_pmu_lbr_init(void)
    {
    switch (x86_pmu.intel_cap.lbr_format) {
    case LBR_FORMAT_EIP_FLAGS2:
    x86_pmu.lbr_has_tsx = 1;
    x86_pmu.lbr_from_flags = 1;
    if (lbr_from_signext_quirk_needed())
    static_branch_enable(&lbr_from_quirk_key);
    break;
    case LBR_FORMAT_EIP_FLAGS:
    x86_pmu.lbr_from_flags = 1;
    break;
    case LBR_FORMAT_INFO:
    x86_pmu.lbr_has_tsx = 1;
    fallthrough;
    case LBR_FORMAT_INFO2:
    x86_pmu.lbr_has_info = 1;
    break;
    case LBR_FORMAT_TIME:
    x86_pmu.lbr_from_flags = 1;
    x86_pmu.lbr_to_cycles = 1;
    break;
    }
    if (x86_pmu.lbr_has_info) {
//
// Only used in combination with baseline pebs.
//
    static_branch_enable(&x86_lbr_mispred);
    static_branch_enable(&x86_lbr_cycles);
    }
    }
//
// LBR state size is variable based on the max number of registers.
// This calculates the expected state size, which should match
// what the hardware enumerates for the size of XFEATURE_LBR.
//
#[no_mangle]
pub unsafe extern "C" fn get_lbr_state_size() -> c_uint {
    static inline unsigned int get_lbr_state_size(void)
    {
    return sizeof(struct arch_lbr_state) +
    x86_pmu.lbr_nr * sizeof(struct lbr_entry);
    }
#[no_mangle]
unsafe extern "C" fn is_arch_lbr_xsave_available() -> bool {
    static bool is_arch_lbr_xsave_available(void)
    {
    if (!boot_cpu_has(X86_FEATURE_XSAVES))
    return false;
//
// Check the LBR state with the corresponding software structure.
// Disable LBR XSAVES support if the size doesn't match.
//
    if (xfeature_size(XFEATURE_LBR) == 0)
    return false;
    if (WARN_ON(xfeature_size(XFEATURE_LBR) != get_lbr_state_size()))
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_pmu_arch_lbr_init() -> void __init {
    void __init intel_pmu_arch_lbr_init(void)
    {
    struct pmu *pmu = x86_get_pmu(smp_processor_id());
    union cpuid28_eax eax;
    union cpuid28_ebx ebx;
    union cpuid28_ecx ecx;
    unsigned int unused_edx;
    bool arch_lbr_xsave;
    size_t size;
    u64 lbr_nr;
// Arch LBR Capabilities
    cpuid(28, &eax.full, &ebx.full, &ecx.full, &unused_edx);
    lbr_nr = fls(eax.split.lbr_depth_mask) * 8;
    if (!lbr_nr)
    goto clear_arch_lbr;
// Apply the max depth of Arch LBR
    if (wrmsrq_safe(MSR_ARCH_LBR_DEPTH, lbr_nr))
    goto clear_arch_lbr;
    x86_pmu.lbr_depth_mask = eax.split.lbr_depth_mask;
    x86_pmu.lbr_deep_c_reset = eax.split.lbr_deep_c_reset;
    x86_pmu.lbr_lip = eax.split.lbr_lip;
    x86_pmu.lbr_cpl = ebx.split.lbr_cpl;
    x86_pmu.lbr_filter = ebx.split.lbr_filter;
    x86_pmu.lbr_call_stack = ebx.split.lbr_call_stack;
    x86_pmu.lbr_mispred = ecx.split.lbr_mispred;
    x86_pmu.lbr_timed_lbr = ecx.split.lbr_timed_lbr;
    x86_pmu.lbr_br_type = ecx.split.lbr_br_type;
    x86_pmu.lbr_counters = ecx.split.lbr_counters;
    x86_pmu.lbr_nr = lbr_nr;
    if (!!x86_pmu.lbr_counters)
    x86_pmu.flags |= PMU_FL_BR_CNTR | PMU_FL_DYN_CONSTRAINT;
    if (x86_pmu.lbr_mispred)
    static_branch_enable(&x86_lbr_mispred);
    if (x86_pmu.lbr_timed_lbr)
    static_branch_enable(&x86_lbr_cycles);
    if (x86_pmu.lbr_br_type)
    static_branch_enable(&x86_lbr_type);
    arch_lbr_xsave = is_arch_lbr_xsave_available();
    if (arch_lbr_xsave) {
    size = sizeof(struct x86_perf_task_context_arch_lbr_xsave) +
    get_lbr_state_size();
    pmu.task_ctx_cache = create_lbr_kmem_cache(size,
    XSAVE_ALIGNMENT);
    }
    if (!pmu.task_ctx_cache) {
    arch_lbr_xsave = false;
    size = sizeof(struct x86_perf_task_context_arch_lbr) +
    lbr_nr * sizeof(struct lbr_entry);
    pmu.task_ctx_cache = create_lbr_kmem_cache(size, 0);
    }
    x86_pmu.lbr_from = MSR_ARCH_LBR_FROM_0;
    x86_pmu.lbr_to = MSR_ARCH_LBR_TO_0;
    x86_pmu.lbr_info = MSR_ARCH_LBR_INFO_0;
// LBR callstack requires both CPL and Branch Filtering support
    if (!x86_pmu.lbr_cpl ||
    !x86_pmu.lbr_filter ||
    !x86_pmu.lbr_call_stack)
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT] = LBR_NOT_SUPP;
    if (!x86_pmu.lbr_cpl) {
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_USER_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_KERNEL_SHIFT] = LBR_NOT_SUPP;
    } else if (!x86_pmu.lbr_filter) {
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_ANY_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_IND_CALL_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_COND_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT] = LBR_NOT_SUPP;
    arch_lbr_ctl_map[PERF_SAMPLE_BRANCH_CALL_SHIFT] = LBR_NOT_SUPP;
    }
    x86_pmu.lbr_ctl_mask = ARCH_LBR_CTL_MASK;
    x86_pmu.lbr_ctl_map  = arch_lbr_ctl_map;
    if (!x86_pmu.lbr_cpl && !x86_pmu.lbr_filter)
    x86_pmu.lbr_ctl_map = core::ptr::null_mut();
    x86_pmu.lbr_reset = intel_pmu_arch_lbr_reset;
    if (arch_lbr_xsave) {
    x86_pmu.lbr_save = intel_pmu_arch_lbr_xsaves;
    x86_pmu.lbr_restore = intel_pmu_arch_lbr_xrstors;
    x86_pmu.lbr_read = intel_pmu_arch_lbr_read_xsave;
    pr_cont("XSAVE ");
    } else {
    x86_pmu.lbr_save = intel_pmu_arch_lbr_save;
    x86_pmu.lbr_restore = intel_pmu_arch_lbr_restore;
    x86_pmu.lbr_read = intel_pmu_arch_lbr_read;
    }
    pr_cont("Architectural LBR, ");
    return;
    clear_arch_lbr:
    setup_clear_cpu_cap(X86_FEATURE_ARCH_LBR);
    }
//
// x86_perf_get_lbr - get the LBR records information
//
// @lbr: the caller's memory to store the LBR records information
//
#[no_mangle]
pub unsafe extern "C" fn x86_perf_get_lbr(lbr: *mut x86_pmu_lbr) {
    void x86_perf_get_lbr(struct x86_pmu_lbr *lbr)
    {
    lbr.nr = x86_pmu.lbr_nr;
    lbr.from = x86_pmu.lbr_from;
    lbr.to = x86_pmu.lbr_to;
    lbr.info = x86_pmu.lbr_info;
    lbr.has_callstack = x86_pmu_has_lbr_callstack();
    }
    EXPORT_SYMBOL_FOR_KVM(x86_perf_get_lbr);
    struct event_constraint vlbr_constraint =
    __EVENT_CONSTRAINT(INTEL_FIXED_VLBR_EVENT, (1ULL << INTEL_PMC_IDX_FIXED_VLBR),
    FIXED_EVENT_FLAGS, 1, 0, PERF_X86_EVENT_LBR_SELECT);
