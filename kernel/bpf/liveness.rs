//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/liveness.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_frame_masks {
    pub /: *mut *mut spis_t may_read; / stack slots that may be read by this instruction,
    pub /: *mut *mut spis_t must_write; / stack slots written by this instruction,
    pub /: *mut *mut spis_t live_before; / stack slots that may be read by this insn and its successors,
}

//
// A function instance keyed by (callsite, depth).
// Encapsulates read and write marks for each instruction in the function.
// Marks are tracked for each frame up to @depth.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct func_instance {
    pub hl_node: hlist_node,
    pub /: *mut *mut u32 callsite; / call insn that invoked this subprog (subprog_start for depth 0),
    pub /: *mut *mut u32 depth; / call depth (0 = entry subprog),
    pub /: *mut *mut u32 subprog; / subprog index,
    pub /: *mut *mut u32 subprog_start; / cached env->subprog_info[subprog].start,
    pub /: *mut *mut u32 insn_cnt; / cached number of insns in the function,
// Per frame, per instruction masks, frames allocated lazily.
    pub frames: [*mut per_frame_masks; MAX_CALL_FRAMES],
    pub must_write_initialized: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct live_stack_query {
    pub /: *mut *mut *mut func_instance instances[MAX_CALL_FRAMES]; / valid in range [0..curframe],
    pub /: *mut *mut u32 callsites[MAX_CALL_FRAMES]; / callsite[i] = insn calling frame i+1,
    pub curframe: u32,
    pub insn_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_liveness {
    pub /: *mut *mut DECLARE_HASHTABLE(func_instances, 8); / maps (depth, callsite) to func_instance,
    pub /: *mut *mut live_stack_query live_stack_query; / cache to avoid repetitive ht lookups,
    pub /: *mut *mut u32 subprog_calls; / analyze_subprog() invocations,
}

//
// Hash/compare key for func_instance: (depth, callsite).
// For depth == 0 (entry subprog), @callsite is the subprog start insn.
// For depth > 0, @callsite is the call instruction index that invoked the subprog.
//
#[no_mangle]
unsafe extern "C" fn instance_hash(callsite: u32, depth: u32) -> u32 {
    static u32 instance_hash(u32 callsite, u32 depth)
    {
    u32 key[2] = { depth, callsite };
    return jhash2(key, 2, 0);
    }
    static struct func_instance *find_instance(struct bpf_verifier_env *env,
    u32 callsite, u32 depth)
    {
    struct bpf_liveness *liveness = env.liveness;
    struct func_instance *f;
    let mut key: u32 = instance_hash(callsite, depth);
    hash_for_each_possible(liveness.func_instances, f, hl_node, key)
    if (f.depth == depth && f.callsite == callsite)
    return f;
    return core::ptr::null_mut();
    }
    static struct func_instance *call_instance(struct bpf_verifier_env *env,
    struct func_instance *caller,
    u32 callsite, int subprog)
    {
    let mut depth: u32 = caller ? caller.depth + 1 : 0;
    let mut subprog_start: u32 = env.subprog_info[subprog].start;
    let mut lookup_key: u32 = depth > 0 ? callsite : subprog_start;
    struct func_instance *f;
    u32 hash;
    f = find_instance(env, lookup_key, depth);
    if (f)
    return f;
    f = kvzalloc_obj(*f, GFP_KERNEL_ACCOUNT);
    if (!f)
    return ERR_PTR(-ENOMEM);
    f.callsite = lookup_key;
    f.depth = depth;
    f.subprog = subprog;
    f.subprog_start = subprog_start;
    f.insn_cnt = (env.subprog_info + subprog + 1).start - subprog_start;
    hash = instance_hash(lookup_key, depth);
    hash_add(env.liveness.func_instances, &f.hl_node, hash);
    return f;
    }
    static struct func_instance *lookup_instance(struct bpf_verifier_env *env,
    struct bpf_verifier_state *st,
    u32 frameno)
    {
    u32 callsite, subprog_start;
    struct func_instance *f;
    u32 key, depth;
    subprog_start = env.subprog_info[st.frame[frameno].subprogno].start;
    callsite = frameno > 0 ? st.frame[frameno].callsite : subprog_start;
    for (depth = frameno; ; depth--) {
    key = depth > 0 ? callsite : subprog_start;
    f = find_instance(env, key, depth);
    if (f || depth == 0)
    return f;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stack_liveness_init(env: *mut bpf_verifier_env) -> c_int {
    int bpf_stack_liveness_init(struct bpf_verifier_env *env)
    {
    env.liveness = kvzalloc_obj(*env.liveness, GFP_KERNEL_ACCOUNT);
    if (!env.liveness)
    return -ENOMEM;
    hash_init(env.liveness.func_instances);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stack_liveness_free(env: *mut bpf_verifier_env) {
    void bpf_stack_liveness_free(struct bpf_verifier_env *env)
    {
    struct func_instance *instance;
    struct hlist_node *tmp;
    int bkt, i;
    if (!env.liveness)
    return;
    hash_for_each_safe(env.liveness.func_instances, bkt, tmp, instance, hl_node) {
    for (i = 0; i <= instance.depth; i++)
    kvfree(instance.frames[i]);
    kvfree(instance);
    }
    kvfree(env.liveness);
    }
//
// Convert absolute instruction index @insn_idx to an index relative
// to start of the function corresponding to @instance.
//
#[no_mangle]
unsafe extern "C" fn relative_idx(instance: *mut func_instance, insn_idx: u32) -> c_int {
    static int relative_idx(struct func_instance *instance, u32 insn_idx)
    {
    return insn_idx - instance.subprog_start;
    }
    static struct per_frame_masks *get_frame_masks(struct func_instance *instance,
    u32 frame, u32 insn_idx)
    {
    if (!instance.frames[frame])
    return core::ptr::null_mut();
    return &instance.frames[frame][relative_idx(instance, insn_idx)];
    }
    static struct per_frame_masks *alloc_frame_masks(struct func_instance *instance,
    u32 frame, u32 insn_idx)
    {
    struct per_frame_masks *arr;
    if (!instance.frames[frame]) {
    arr = kvzalloc_objs(*arr, instance.insn_cnt,
    GFP_KERNEL_ACCOUNT);
    instance.frames[frame] = arr;
    if (!arr)
    return ERR_PTR(-ENOMEM);
    }
    return get_frame_masks(instance, frame, insn_idx);
    }
// Accumulate may_read masks for @frame at @insn_idx
#[no_mangle]
unsafe extern "C" fn mark_stack_read(instance: *mut func_instance, frame: u32, insn_idx: u32, mask: spis_t) -> c_int {
    static int mark_stack_read(struct func_instance *instance, u32 frame, u32 insn_idx, spis_t mask)
    {
    struct per_frame_masks *masks;
    masks = alloc_frame_masks(instance, frame, insn_idx);
    if (IS_ERR(masks))
    return PTR_ERR(masks);
    masks.may_read = spis_or(masks.may_read, mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mark_stack_write(instance: *mut func_instance, frame: u32, insn_idx: u32, mask: spis_t) -> c_int {
    static int mark_stack_write(struct func_instance *instance, u32 frame, u32 insn_idx, spis_t mask)
    {
    struct per_frame_masks *masks;
    masks = alloc_frame_masks(instance, frame, insn_idx);
    if (IS_ERR(masks))
    return PTR_ERR(masks);
    masks.must_write = spis_or(masks.must_write, mask);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_jmp_offset(insn: *mut bpf_insn) -> c_int {
    int bpf_jmp_offset(struct bpf_insn *insn)
    {
    let mut code: u8 = insn.code;
    if (code == (BPF_JMP32 | BPF_JA))
    return insn.imm;
    return insn.off;
    }
    __diag_push();
    __diag_ignore_all("-Woverride-init", "Allow field initialization overrides for opcode_info_tbl");
//
// Returns an array of instructions succ, with succ->items[0], ...,
// succ->items[n-1] with successor instructions, where n=succ->cnt
//
    inline struct bpf_iarray *
    bpf_insn_successors(struct bpf_verifier_env *env, u32 idx)
    {
    static const struct opcode_info {
    bool can_jump;
    bool can_fallthrough;
    } opcode_info_tbl[256] = {
    [0 ... 255] = {.can_jump = false, .can_fallthrough = true},

    [BPF_JMP   | code] = __VA_ARGS__, \
    [BPF_JMP32 | code] = __VA_ARGS__
    _J(BPF_EXIT,  {.can_jump = false, .can_fallthrough = false}),
    _J(BPF_JA,    {.can_jump = true,  .can_fallthrough = false}),
    _J(BPF_JEQ,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JNE,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JLT,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JLE,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JGT,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JGE,   {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JSGT,  {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JSGE,  {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JSLT,  {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JSLE,  {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JCOND, {.can_jump = true,  .can_fallthrough = true}),
    _J(BPF_JSET,  {.can_jump = true,  .can_fallthrough = true}),

    };
    struct bpf_prog *prog = env.prog;
    struct bpf_insn *insn = &prog.insnsi[idx];
    const struct opcode_info *opcode_info;
    struct bpf_iarray *succ, *jt;
    int insn_sz;
    jt = env.insn_aux_data[idx].jt;
    if (unlikely(jt))
    return jt;
// pre-allocated array of size up to 2; reset cnt, as it may have been used already
    succ = env.succ;
    succ.cnt = 0;
    opcode_info = &opcode_info_tbl[BPF_CLASS(insn.code) | BPF_OP(insn.code)];
    insn_sz = bpf_is_ldimm64(insn) ? 2 : 1;
    if (opcode_info.can_fallthrough)
    succ.items[succ.cnt++] = idx + insn_sz;
    if (opcode_info.can_jump)
    succ.items[succ.cnt++] = idx + bpf_jmp_offset(insn) + 1;
    return succ;
    }
    __diag_pop();
    static inline bool update_insn(struct bpf_verifier_env *env,
    struct func_instance *instance, u32 frame, u32 insn_idx)
    {
    spis_t new_before, new_after;
    struct per_frame_masks *insn, *succ_insn;
    struct bpf_iarray *succ;
    u32 s;
    bool changed;
    succ = bpf_insn_successors(env, insn_idx);
    if (succ.cnt == 0)
    return false;
    changed = false;
    insn = get_frame_masks(instance, frame, insn_idx);
    new_before = SPIS_ZERO;
    new_after = SPIS_ZERO;
    for (s = 0; s < succ.cnt; ++s) {
    succ_insn = get_frame_masks(instance, frame, succ.items[s]);
    new_after = spis_or(new_after, succ_insn.live_before);
    }
//
// New "live_before" is a union of all "live_before" of successors
// minus slots written by instruction plus slots read by instruction.
// new_before = (new_after & ~insn->must_write) | insn->may_read
//
    new_before = spis_or(spis_and(new_after, spis_not(insn.must_write)),
    insn.may_read);
    changed |= !spis_equal(new_before, insn.live_before);
    insn.live_before = new_before;
    return changed;
    }
// Fixed-point computation of @live_before marks
#[no_mangle]
unsafe extern "C" fn update_instance(env: *mut bpf_verifier_env, instance: *mut func_instance) {
    static void update_instance(struct bpf_verifier_env *env, struct func_instance *instance)
    {
    u32 i, frame, po_start, po_end;
    int *insn_postorder = env.cfg.insn_postorder;
    struct bpf_subprog_info *subprog;
    bool changed;
    instance.must_write_initialized = true;
    subprog = &env.subprog_info[instance.subprog];
    po_start = subprog.postorder_start;
    po_end = (subprog + 1).postorder_start;
// repeat until fixed point is reached
    do {
    changed = false;
    for (frame = 0; frame <= instance.depth; frame++) {
    if (!instance.frames[frame])
    continue;
    for (i = po_start; i < po_end; i++)
    changed |= update_insn(env, instance, frame, insn_postorder[i]);
    }
    } while (changed);
    }
#[no_mangle]
unsafe extern "C" fn is_live_before(instance: *mut func_instance, insn_idx: u32, frameno: u32, half_spi: u32) -> bool {
    static bool is_live_before(struct func_instance *instance, u32 insn_idx, u32 frameno, u32 half_spi)
    {
    struct per_frame_masks *masks;
    masks = get_frame_masks(instance, frameno, insn_idx);
    return masks && spis_test_bit(masks.live_before, half_spi);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_live_stack_query_init(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int {
    int bpf_live_stack_query_init(struct bpf_verifier_env *env, struct bpf_verifier_state *st)
    {
    struct live_stack_query *q = &env.liveness.live_stack_query;
    struct func_instance *instance;
    u32 frame;
    memset(q, 0, sizeof(*q));
    for (frame = 0; frame <= st.curframe; frame++) {
    instance = lookup_instance(env, st, frame);
    if (IS_ERR_OR_NULL(instance))
    q.instances[frame] = core::ptr::null_mut();
    else
    q.instances[frame] = instance;
    if (frame < st.curframe)
    q.callsites[frame] = st.frame[frame + 1].callsite;
    }
    q.curframe = st.curframe;
    q.insn_idx = st.insn_idx;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stack_slot_alive(env: *mut bpf_verifier_env, frameno: u32, half_spi: u32) -> bool {
    bool bpf_stack_slot_alive(struct bpf_verifier_env *env, u32 frameno, u32 half_spi)
    {
//
// Slot is alive if it is read before q->insn_idx in current func instance,
// or if for some outer func instance:
// - alive before callsite if callsite calls callback, otherwise
// - alive after callsite
//
    struct live_stack_query *q = &env.liveness.live_stack_query;
    struct func_instance *instance, *curframe_instance;
    u32 i, callsite, rel;
    int cur_delta, delta;
    let mut alive: bool = false;
    curframe_instance = q.instances[q.curframe];
    if (!curframe_instance)
    return true;
    cur_delta = (int)curframe_instance.depth - (int)q.curframe;
    rel = frameno + cur_delta;
    if (rel <= curframe_instance.depth)
    alive = is_live_before(curframe_instance, q.insn_idx, rel, half_spi);
    if (alive)
    return true;
    for (i = frameno; i < q.curframe; i++) {
    instance = q.instances[i];
    if (!instance)
    return true;
// Map actual frameno to frame index within this instance
    delta = (int)instance.depth - (int)i;
    rel = frameno + delta;
    if (rel > instance.depth)
    return true;
// Get callsite from verifier state, not from instance callchain
    callsite = q.callsites[i];
    alive = bpf_calls_callback(env, callsite)
    ? is_live_before(instance, callsite, rel, half_spi)
    : is_live_before(instance, callsite + 1, rel, half_spi);
    if (alive)
    return true;
    }
    return false;
    }
    static char *fmt_subprog(struct bpf_verifier_env *env, int subprog)
    {
    const char *name = env.subprog_info[subprog].name;
    snprintf(env.tmp_str_buf, sizeof(env.tmp_str_buf),
    "subprog#%d%s%s", subprog, name ? " " : "", name ? name : "");
    return env.tmp_str_buf;
    }
    static char *fmt_instance(struct bpf_verifier_env *env, struct func_instance *instance)
    {
    snprintf(env.tmp_str_buf, sizeof(env.tmp_str_buf),
    "(d%d,cs%d)", instance.depth, instance.callsite);
    return env.tmp_str_buf;
    }
#[no_mangle]
unsafe extern "C" fn spi_off(spi: c_int) -> c_int {
    static int spi_off(int spi)
    {
    return -(spi + 1) * BPF_REG_SIZE;
    }
//
// When both halves of an 8-byte SPI are set, print as "-8","-16",...
// When only one half is set, print as "-4h","-8h",...
// Runs of 3+ consecutive fully-set SPIs are collapsed: "fp0-8..-24"
//
    static char *fmt_spis_mask(struct bpf_verifier_env *env, int frame, bool first, spis_t spis)
    {
    let mut buf_sz: c_int = sizeof(env.tmp_str_buf);
    char *buf = env.tmp_str_buf;
    int spi, n, run_start;
    buf[0] = '\0';
    for (spi = 0; spi < STACK_SLOTS / 2 && buf_sz > 0; spi++) {
    let mut lo: bool = spis_test_bit(spis, spi * 2);
    let mut hi: bool = spis_test_bit(spis, spi * 2 + 1);
    const char *space = first ? "" : " ";
    if (!lo && !hi)
    continue;
    if (!lo || !hi) {
// half-spi
    n = scnprintf(buf, buf_sz, "%sfp%d%d%s",
    space, frame, spi_off(spi) + (lo ? STACK_SLOT_SZ : 0), "h");
    } else if (spi + 2 < STACK_SLOTS / 2 &&
    spis_test_bit(spis, spi * 2 + 2) &&
    spis_test_bit(spis, spi * 2 + 3) &&
    spis_test_bit(spis, spi * 2 + 4) &&
    spis_test_bit(spis, spi * 2 + 5)) {
// 3+ consecutive full spis
    run_start = spi;
    while (spi + 1 < STACK_SLOTS / 2 &&
    spis_test_bit(spis, (spi + 1) * 2) &&
    spis_test_bit(spis, (spi + 1) * 2 + 1))
    spi++;
    n = scnprintf(buf, buf_sz, "%sfp%d%d..%d",
    space, frame, spi_off(run_start), spi_off(spi));
    } else {
// just a full spi
    n = scnprintf(buf, buf_sz, "%sfp%d%d", space, frame, spi_off(spi));
    }
    first = false;
    buf += n;
    buf_sz -= n;
    }
    return env.tmp_str_buf;
    }
#[no_mangle]
unsafe extern "C" fn print_instance(env: *mut bpf_verifier_env, instance: *mut func_instance) {
    static void print_instance(struct bpf_verifier_env *env, struct func_instance *instance)
    {
    let mut start: c_int = env.subprog_info[instance.subprog].start;
    struct bpf_insn *insns = env.prog.insnsi;
    struct per_frame_masks *masks;
    let mut len: c_int = instance.insn_cnt;
    int insn_idx, frame, i;
    bool has_use, has_def;
    u64 pos, insn_pos;
    if (!(env.log.level & BPF_LOG_LEVEL2))
    return;
    verbose(env, "stack use/def %s ", fmt_subprog(env, instance.subprog));
    verbose(env, "%s:\n", fmt_instance(env, instance));
    for (i = 0; i < len; i++) {
    insn_idx = start + i;
    has_use = false;
    has_def = false;
    pos = env.log.end_pos;
    verbose(env, "%3d: ", insn_idx);
    bpf_verbose_insn(env, &insns[insn_idx]);
    insn_pos = env.log.end_pos;
    verbose(env, "%*c;", bpf_vlog_alignment(insn_pos - pos), ' ');
    pos = env.log.end_pos;
    verbose(env, " use: ");
    for (frame = instance.depth; frame >= 0; --frame) {
    masks = get_frame_masks(instance, frame, insn_idx);
    if (!masks || spis_is_zero(masks.may_read))
    continue;
    verbose(env, "%s", fmt_spis_mask(env, frame, !has_use, masks.may_read));
    has_use = true;
    }
    if (!has_use)
    bpf_vlog_reset(&env.log, pos);
    pos = env.log.end_pos;
    verbose(env, " def: ");
    for (frame = instance.depth; frame >= 0; --frame) {
    masks = get_frame_masks(instance, frame, insn_idx);
    if (!masks || spis_is_zero(masks.must_write))
    continue;
    verbose(env, "%s", fmt_spis_mask(env, frame, !has_def, masks.must_write));
    has_def = true;
    }
    if (!has_def)
    bpf_vlog_reset(&env.log, has_use ? pos : insn_pos);
    verbose(env, "\n");
    if (bpf_is_ldimm64(&insns[insn_idx]))
    i++;
    }
    }
#[no_mangle]
unsafe extern "C" fn cmp_instances(pa: *const c_void, pb: *const c_void) -> c_int {
    static int cmp_instances(const void *pa, const void *pb)
    {
    struct func_instance *a = *(struct func_instance **)pa;
    struct func_instance *b = *(struct func_instance **)pb;
    let mut dcallsite: c_int = (int)a.callsite - b.callsite;
    let mut ddepth: c_int = (int)a.depth - b.depth;
    if (dcallsite)
    return dcallsite;
    if (ddepth)
    return ddepth;
    return 0;
    }
// print use/def slots for all instances ordered by callsite first, then by depth
#[no_mangle]
unsafe extern "C" fn print_instances(env: *mut bpf_verifier_env) -> c_int {
    static int print_instances(struct bpf_verifier_env *env)
    {
    struct func_instance *instance, **sorted_instances;
    struct bpf_liveness *liveness = env.liveness;
    int i, bkt, cnt;
    cnt = 0;
    hash_for_each(liveness.func_instances, bkt, instance, hl_node)
    cnt++;
    sorted_instances = kvmalloc_objs(*sorted_instances, cnt, GFP_KERNEL_ACCOUNT);
    if (!sorted_instances)
    return -ENOMEM;
    cnt = 0;
    hash_for_each(liveness.func_instances, bkt, instance, hl_node)
    sorted_instances[cnt++] = instance;
    sort(sorted_instances, cnt, sizeof(*sorted_instances), cmp_instances, core::ptr::null_mut());
    for (i = 0; i < cnt; i++)
    print_instance(env, sorted_instances[i]);
    kvfree(sorted_instances);
    return 0;
    }
//
// Per-register tracking state for compute_subprog_args().
// Tracks which frame's FP a value is derived from
// and the byte offset from that frame's FP.
//
// The .frame field forms a lattice with three levels of precision:
//
// precise {frame=N, off=V}      -- known absolute frame index and byte offset
// |
// offset-imprecise {frame=N, cnt=0}
// |                        -- known frame identity, unknown offset
// fully-imprecise {frame=ARG_IMPRECISE, mask=bitmask}
// -- unknown frame identity; .mask is a
// bitmask of which frame indices might be
// involved
//
// At CFG merge points, arg_track_join() moves down the lattice:
// - same frame + same offset  -> precise
// - same frame + different offset -> offset-imprecise
// - different frames          -> fully-imprecise (bitmask OR)
//
// At memory access sites (LDX/STX/ST), offset-imprecise marks only
// the known frame's access mask as SPIS_ALL, while fully-imprecise
// iterates bits in the bitmask and routes each frame to its target.
//
pub const MAX_ARG_OFFSETS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arg_track {
    union {
    pub /: *mut *mut s16 off[MAX_ARG_OFFSETS]; / byte offsets; off_cnt says how many,
    pub /: *mut *mut u16 mask; / arg bitmask when arg == ARG_IMPRECISE,
}

    s8 frame;	/* absolute frame index, or enum arg_track_state */
    s8 off_cnt;	/* 0 = offset-imprecise, 1-4 = # of precise offsets */
    };
    enum arg_track_state {
    ARG_NONE	= -1,	/* not derived from any argument */
    ARG_UNVISITED	= -2,	/* not yet reached by dataflow */
    ARG_IMPRECISE	= -3,	/* lost identity; .mask is arg bitmask */
    };
// Track callee stack slots fp-8 through fp-512 (64 slots of 8 bytes each)
pub const MAX_ARG_SPILL_SLOTS: c_int = 64;
//
// Combined register + stack arg tracking: R0-R10 at indices 0-10,
// outgoing stack arg slots at indices MAX_BPF_REG..MAX_BPF_REG+6.
//

#[no_mangle]
unsafe extern "C" fn stack_arg_off_to_slot(off: i16) -> c_int {
    static int stack_arg_off_to_slot(s16 off)
    {
    let mut aoff: c_int = off < 0 ? -off : off;
    if (aoff / 8 > MAX_STACK_ARG_SLOTS)
    return -1;
    return aoff / 8 - 1;
    }
#[no_mangle]
unsafe extern "C" fn arg_is_visited(at: *const arg_track) -> bool {
    static bool arg_is_visited(const struct arg_track *at)
    {
    return at.frame != ARG_UNVISITED;
    }
#[no_mangle]
unsafe extern "C" fn arg_is_fp(at: *const arg_track) -> bool {
    static bool arg_is_fp(const struct arg_track *at)
    {
    return at.frame >= 0 || at.frame == ARG_IMPRECISE;
    }
#[no_mangle]
unsafe extern "C" fn verbose_arg_track(env: *mut bpf_verifier_env, at: *mut arg_track) {
    static void verbose_arg_track(struct bpf_verifier_env *env, struct arg_track *at)
    {
    int i;
    switch (at.frame) {
    case ARG_NONE:      verbose(env, "_");                          break;
    case ARG_UNVISITED: verbose(env, "?");                          break;
    case ARG_IMPRECISE: verbose(env, "IMP%x", at.mask);            break;
    default:
// frame >= 0: absolute frame index
    if (at.off_cnt == 0) {
    verbose(env, "fp%d ?", at.frame);
    } else {
    for (i = 0; i < at.off_cnt; i++) {
    if (i)
    verbose(env, "|");
    verbose(env, "fp%d%+d", at.frame, at.off[i]);
    }
    }
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn arg_track_eq(a: *const arg_track, b: *const arg_track) -> bool {
    static bool arg_track_eq(const struct arg_track *a, const struct arg_track *b)
    {
    int i;
    if (a.frame != b.frame)
    return false;
    if (a.frame == ARG_IMPRECISE)
    return a.mask == b.mask;
    if (a.frame < 0)
    return true;
    if (a.off_cnt != b.off_cnt)
    return false;
    for (i = 0; i < a.off_cnt; i++)
    if (a.off[i] != b.off[i])
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn arg_single(arg: i8, off: i16) -> arg_track {
    static struct arg_track arg_single(s8 arg, s16 off)
    {
    let mut at: arg_track = {};
    at.frame = arg;
    at.off[0] = off;
    at.off_cnt = 1;
    return at;
    }
//
// Merge two sorted offset arrays, deduplicate.
// Returns off_cnt=0 if the result exceeds MAX_ARG_OFFSETS.
// Both args must have the same frame and off_cnt > 0.
//
#[no_mangle]
unsafe extern "C" fn arg_merge_offsets(a: arg_track, b: arg_track) -> arg_track {
    static struct arg_track arg_merge_offsets(struct arg_track a, struct arg_track b)
    {
    let mut result: arg_track = { .frame = a.frame };
    let mut imp: arg_track = { .frame = a.frame };
    let mut i: c_int = 0, j = 0, k = 0;
    while (i < a.off_cnt && j < b.off_cnt) {
    s16 v;
    if (a.off[i] <= b.off[j]) {
    v = a.off[i++];
    if (v == b.off[j])
    j++;
    } else {
    v = b.off[j++];
    }
    if (k > 0 && result.off[k - 1] == v)
    continue;
    if (k >= MAX_ARG_OFFSETS)
    return imp;
    result.off[k++] = v;
    }
    while (i < a.off_cnt) {
    if (k >= MAX_ARG_OFFSETS)
    return imp;
    result.off[k++] = a.off[i++];
    }
    while (j < b.off_cnt) {
    if (k >= MAX_ARG_OFFSETS)
    return imp;
    result.off[k++] = b.off[j++];
    }
    result.off_cnt = k;
    return result;
    }
//
// Merge two arg_tracks into ARG_IMPRECISE, collecting the frame
// bits from both operands. Precise frame indices (frame >= 0)
// contribute a single bit; existing ARG_IMPRECISE values
// contribute their full bitmask.
//
#[no_mangle]
unsafe extern "C" fn arg_join_imprecise(a: arg_track, b: arg_track) -> arg_track {
    static struct arg_track arg_join_imprecise(struct arg_track a, struct arg_track b)
    {
    let mut m: u32 = 0;
    if (a.frame >= 0)
    m |= BIT(a.frame);
#[no_mangle]
pub unsafe extern "C" fn if(ARG_IMPRECISE: a.frame ==) -> else {
    else if (a.frame == ARG_IMPRECISE)
    m |= a.mask;
    if (b.frame >= 0)
    m |= BIT(b.frame);
#[no_mangle]
pub unsafe extern "C" fn if(ARG_IMPRECISE: b.frame ==) -> else {
    else if (b.frame == ARG_IMPRECISE)
    m |= b.mask;
    return (struct arg_track){ .mask = m, .frame = ARG_IMPRECISE };
    }
// Join two arg_track values at merge points
#[no_mangle]
unsafe extern "C" fn __arg_track_join(a: arg_track, b: arg_track) -> arg_track {
    static struct arg_track __arg_track_join(struct arg_track a, struct arg_track b)
    {
    if (!arg_is_visited(&b))
    return a;
    if (!arg_is_visited(&a))
    return b;
    if (a.frame == b.frame && a.frame >= 0) {
// Both offset-imprecise: stay imprecise
    if (a.off_cnt == 0 || b.off_cnt == 0)
    return (struct arg_track){ .frame = a.frame };
// Merge offset sets; falls back to off_cnt=0 if >4
    return arg_merge_offsets(a, b);
    }
//
// args are different, but one of them is known
// arg + none -> arg
// none + arg -> arg
//
// none + none -> none
//
    if (a.frame == ARG_NONE && b.frame == ARG_NONE)
    return a;
    if (a.frame >= 0 && b.frame == ARG_NONE) {
//
// When joining single fp-N add fake fp+0 to
// keep stack_use and prevent stack_def
//
    if (a.off_cnt == 1)
    return arg_merge_offsets(a, arg_single(a.frame, 0));
    return a;
    }
    if (b.frame >= 0 && a.frame == ARG_NONE) {
    if (b.off_cnt == 1)
    return arg_merge_offsets(b, arg_single(b.frame, 0));
    return b;
    }
    return arg_join_imprecise(a, b);
    }
    static bool arg_track_join(struct bpf_verifier_env *env, int idx, int target, int r,
    struct arg_track *in, struct arg_track out)
    {
    let mut old: arg_track = *in;
    let mut new_val: arg_track = __arg_track_join(old, out);
    if (arg_track_eq(&new_val, &old))
    return false;
// in = new_val;
    if (!(env.log.level & BPF_LOG_LEVEL2) || !arg_is_visited(&old))
    return true;
    verbose(env, "arg JOIN insn %d . %d ", idx, target);
    if (r >= MAX_BPF_REG)
    verbose(env, "sa%d: ", r - MAX_BPF_REG);
#[no_mangle]
pub unsafe extern "C" fn if(0: r >=) -> else {
    else if (r >= 0)
    verbose(env, "r%d: ", r);
    else
    verbose(env, "fp%+d: ", r * 8);
    verbose_arg_track(env, &old);
    verbose(env, " + ");
    verbose_arg_track(env, &out);
    verbose(env, " => ");
    verbose_arg_track(env, &new_val);
    verbose(env, "\n");
    return true;
    }
//
// Compute the result when an ALU op destroys offset precision.
// If a single arg is identifiable, preserve it with OFF_IMPRECISE.
// If two different args are involved or one is already ARG_IMPRECISE,
// the result is fully ARG_IMPRECISE.
//
#[no_mangle]
unsafe extern "C" fn arg_track_alu64(dst: *mut arg_track, src: *const arg_track) {
    static void arg_track_alu64(struct arg_track *dst, const struct arg_track *src)
    {
    WARN_ON_ONCE(!arg_is_visited(dst));
    WARN_ON_ONCE(!arg_is_visited(src));
    if (dst.frame >= 0 && (src.frame == ARG_NONE || src.frame == dst.frame)) {
//
// rX += rY where rY is not arg derived
// rX += rX
//
    dst.off_cnt = 0;
    return;
    }
    if (src.frame >= 0 && dst.frame == ARG_NONE) {
//
// rX += rY where rX is not arg derived
// rY identity leaks into rX
//
    dst.off_cnt = 0;
    dst.frame = src.frame;
    return;
    }
    if (dst.frame == ARG_NONE && src.frame == ARG_NONE)
    return;
// dst = arg_join_imprecise(*dst, *src);
    }
#[no_mangle]
unsafe extern "C" fn arg_add(off: i16, delta: i64, out: *mut i16) -> bool {
    static bool arg_add(s16 off, s64 delta, s16 *out)
    {
    let mut d: i16 = delta;
    if (d != delta)
    return true;
    return check_add_overflow(off, d, out);
    }
#[no_mangle]
unsafe extern "C" fn arg_padd(at: *mut arg_track, delta: i64) {
    static void arg_padd(struct arg_track *at, s64 delta)
    {
    int i;
    if (at.off_cnt == 0)
    return;
    for (i = 0; i < at.off_cnt; i++) {
    s16 new_off;
    if (arg_add(at.off[i], delta, &new_off)) {
    at.off_cnt = 0;
    return;
    }
    at.off[i] = new_off;
    }
    }
//
// Convert a byte offset from FP to a callee stack slot index.
// Returns -1 if out of range or not 8-byte aligned.
// Slot 0 = fp-8, slot 1 = fp-16, ..., slot 7 = fp-64, ....
//
#[no_mangle]
unsafe extern "C" fn fp_off_to_slot(off: i16) -> c_int {
    static int fp_off_to_slot(s16 off)
    {
    if (off >= 0 || off < -(int)(MAX_ARG_SPILL_SLOTS * 8))
    return -1;
    if (off % 8)
    return -1;
    return (-off) / 8 - 1;
    }
    static struct arg_track fill_from_stack(struct bpf_insn *insn,
    struct arg_track *at_out, int reg,
    struct arg_track *at_stack_out,
    int depth)
    {
    struct arg_track imp = {
    .mask = (1u << (depth + 1)) - 1,
    .frame = ARG_IMPRECISE
    };
    let mut result: arg_track = { .frame = ARG_NONE };
    int cnt, i;
    if (reg == BPF_REG_FP) {
    let mut slot: c_int = fp_off_to_slot(insn.off);
    return slot >= 0 ? at_stack_out[slot] : imp;
    }
    cnt = at_out[reg].off_cnt;
    if (cnt == 0)
    return imp;
    for (i = 0; i < cnt; i++) {
    s16 fp_off, slot;
    if (arg_add(at_out[reg].off[i], insn.off, &fp_off))
    return imp;
    slot = fp_off_to_slot(fp_off);
    if (slot < 0)
    return imp;
    result = __arg_track_join(result, at_stack_out[slot]);
    }
    return result;
    }
//
// Spill @val to all possible stack slots indicated by the FP offsets in @reg.
// For an 8-byte store, single candidate slot gets @val. multi-slots are joined.
// sub-8-byte store joins with ARG_NONE.
// When exact offset is unknown conservatively add reg values to all slots in at_stack_out.
//
    static void spill_to_stack(struct bpf_insn *insn, struct arg_track *at_out,
    int reg, struct arg_track *at_stack_out,
    struct arg_track *val, u32 sz)
    {
    let mut none: arg_track = { .frame = ARG_NONE };
    let mut new_val: arg_track = sz == 8 ? *val : none;
    int cnt, i;
    if (reg == BPF_REG_FP) {
    let mut slot: c_int = fp_off_to_slot(insn.off);
    if (slot >= 0)
    at_stack_out[slot] = new_val;
    return;
    }
    cnt = at_out[reg].off_cnt;
    if (cnt == 0) {
    for (int slot = 0; slot < MAX_ARG_SPILL_SLOTS; slot++)
    at_stack_out[slot] = __arg_track_join(at_stack_out[slot], new_val);
    return;
    }
    for (i = 0; i < cnt; i++) {
    s16 fp_off;
    int slot;
    if (arg_add(at_out[reg].off[i], insn.off, &fp_off))
    continue;
    slot = fp_off_to_slot(fp_off);
    if (slot < 0)
    continue;
    if (cnt == 1)
    at_stack_out[slot] = new_val;
    else
    at_stack_out[slot] = __arg_track_join(at_stack_out[slot], new_val);
    }
    }
//
// Clear all tracked callee stack slots overlapping the byte range
// [off, off+sz-1] where off is a negative FP-relative offset.
//
#[no_mangle]
unsafe extern "C" fn clear_overlapping_stack_slots(at_stack: *mut arg_track, off: i16, sz: u32, cnt: c_int) {
    static void clear_overlapping_stack_slots(struct arg_track *at_stack, s16 off, u32 sz, int cnt)
    {
    let mut none: arg_track = { .frame = ARG_NONE };
    if (cnt == 0) {
    for (int i = 0; i < MAX_ARG_SPILL_SLOTS; i++)
    at_stack[i] = __arg_track_join(at_stack[i], none);
    return;
    }
    for (int i = 0; i < MAX_ARG_SPILL_SLOTS; i++) {
    let mut slot_start: c_int = -((i + 1) * 8);
    let mut slot_end: c_int = slot_start + 8;
    if (slot_start < off + (int)sz && slot_end > off) {
    if (cnt == 1)
    at_stack[i] = none;
    else
    at_stack[i] = __arg_track_join(at_stack[i], none);
    }
    }
    }
//
// Clear stack slots overlapping all possible FP offsets in @reg.
//
    static void clear_stack_for_all_offs(struct bpf_insn *insn,
    struct arg_track *at_out, int reg,
    struct arg_track *at_stack_out, u32 sz)
    {
    int cnt, i;
    if (reg == BPF_REG_FP) {
    clear_overlapping_stack_slots(at_stack_out, insn.off, sz, 1);
    return;
    }
    cnt = at_out[reg].off_cnt;
    if (cnt == 0) {
    clear_overlapping_stack_slots(at_stack_out, 0, sz, cnt);
    return;
    }
    for (i = 0; i < cnt; i++) {
    s16 fp_off;
    if (arg_add(at_out[reg].off[i], insn.off, &fp_off)) {
    clear_overlapping_stack_slots(at_stack_out, 0, sz, 0);
    break;
    }
    clear_overlapping_stack_slots(at_stack_out, fp_off, sz, cnt);
    }
    }
    static void arg_track_log(struct bpf_verifier_env *env, struct bpf_insn *insn, int idx,
    struct arg_track *at_in, struct arg_track *at_stack_in,
    struct arg_track *at_out, struct arg_track *at_stack_out)
    {
    let mut printed: bool = false;
    int i;
    if (!(env.log.level & BPF_LOG_LEVEL2))
    return;
    for (i = 0; i < MAX_BPF_REG; i++) {
    if (arg_track_eq(&at_out[i], &at_in[i]))
    continue;
    if (!printed) {
    verbose(env, "%3d: ", idx);
    bpf_verbose_insn(env, insn);
    printed = true;
    }
    verbose(env, "\tr%d: ", i); verbose_arg_track(env, &at_in[i]);
    verbose(env, " . "); verbose_arg_track(env, &at_out[i]);
    }
// Log outgoing stack arg slot transitions at indices MAX_BPF_REG..MAX_AT_TRACK_REGS-1
    for (i = 0; i < MAX_STACK_ARG_SLOTS; i++) {
    let mut ai: c_int = MAX_BPF_REG + i;
    if (arg_track_eq(&at_out[ai], &at_in[ai]))
    continue;
    if (!printed) {
    verbose(env, "%3d: ", idx);
    bpf_verbose_insn(env, insn);
    printed = true;
    }
    verbose(env, "\tsa%d: ", i); verbose_arg_track(env, &at_in[ai]);
    verbose(env, " . "); verbose_arg_track(env, &at_out[ai]);
    }
    for (i = 0; i < MAX_ARG_SPILL_SLOTS; i++) {
    if (arg_track_eq(&at_stack_out[i], &at_stack_in[i]))
    continue;
    if (!printed) {
    verbose(env, "%3d: ", idx);
    bpf_verbose_insn(env, insn);
    printed = true;
    }
    verbose(env, "\tfp%+d: ", -(i + 1) * 8); verbose_arg_track(env, &at_stack_in[i]);
    verbose(env, " . "); verbose_arg_track(env, &at_stack_out[i]);
    }
    if (printed)
    verbose(env, "\n");
    }
#[no_mangle]
unsafe extern "C" fn can_be_local_fp(depth: c_int, regno: c_int, at: *mut arg_track) -> bool {
    static bool can_be_local_fp(int depth, int regno, struct arg_track *at)
    {
    return regno == BPF_REG_FP || at.frame == depth ||
    (at.frame == ARG_IMPRECISE && (at.mask & BIT(depth)));
    }
//
// Pure dataflow transfer function for arg_track state.
// Updates at_out[] based on how the instruction modifies registers.
// Tracks spill/fill, but not other memory accesses.
//
    static void arg_track_xfer(struct bpf_verifier_env *env, struct bpf_insn *insn,
    int insn_idx,
    struct arg_track *at_out, struct arg_track *at_stack_out,
    const struct arg_track *at_stack_arg_entry,
    struct func_instance *instance,
    u32 *callsites)
    {
    let mut depth: c_int = instance.depth;
    let mut class: u8 = BPF_CLASS(insn.code);
    let mut code: u8 = BPF_OP(insn.code);
    struct arg_track *dst = &at_out[insn.dst_reg];
    struct arg_track *src = &at_out[insn.src_reg];
    let mut none: arg_track = { .frame = ARG_NONE };
    int r, slot;
// Handle stack arg stores and loads.
    if (is_stack_arg_st(insn) || is_stack_arg_stx(insn)) {
    slot = stack_arg_off_to_slot(insn.off);
    if (slot >= 0) {
    if (is_stack_arg_stx(insn))
    at_out[MAX_BPF_REG + slot] = at_out[insn.src_reg];
    else
    at_out[MAX_BPF_REG + slot] = none;
    }
    } else if (is_stack_arg_ldx(insn)) {
    slot = stack_arg_off_to_slot(insn.off);
    at_out[insn.dst_reg] = (slot >= 0) ? at_stack_arg_entry[slot] : none;
    } else if (class == BPF_ALU64 && BPF_SRC(insn.code) == BPF_K) {
    if (code == BPF_MOV) {
// dst = none;
    } else if (dst.frame >= 0) {
    if (code == BPF_ADD)
    arg_padd(dst, insn.imm);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_SUB: code ==) -> else {
    else if (code == BPF_SUB)
    arg_padd(dst, -(s64)insn.imm);
    else
// Any other 64-bit alu on the pointer makes it imprecise
    dst.off_cnt = 0;
    } /* else if dst.frame is imprecise it stays so */
    } else if (class == BPF_ALU64 && BPF_SRC(insn.code) == BPF_X) {
    if (code == BPF_MOV) {
    if (insn.off == 0) {
// dst = *src;
    } else {
// addr_space_cast destroys a pointer
// dst = none;
    }
    } else {
    arg_track_alu64(dst, src);
    }
    } else if (class == BPF_ALU) {
//
// 32-bit alu destroys the pointer.
// If src was a pointer it cannot leak into dst
//
// dst = none;
    } else if (class == BPF_JMP && code == BPF_CALL) {
//
// at_stack_out[slot] is not cleared by the helper and subprog calls.
// The fill_from_stack() may return the stale spill — which is an FP-derived arg_track
// (the value that was originally spilled there). The loaded register then carries
// a phantom FP-derived identity that doesn't correspond to what's actually in the slot.
// This phantom FP pointer propagates forward, and wherever it's subsequently used
// (as a helper argument, another store, etc.), it sets stack liveness bits.
// Those bits correspond to stack accesses that don't actually happen.
// So the effect is over-reporting stack liveness — marking slots as live that aren't
// actually accessed. The verifier preserves more state than necessary across calls,
// which is conservative.
//
// helpers can scratch stack slots, but they won't make a valid pointer out of it.
// subprogs are allowed to write into parent slots, but they cannot write
// _any_ FP-derived pointer into it (either their own or parent's FP).
//
    for (r = BPF_REG_0; r <= BPF_REG_5; r++)
    at_out[r] = none;
    } else if (class == BPF_LDX) {
    let mut sz: u32 = bpf_size_to_bytes(BPF_SIZE(insn.code));
    let mut src_is_local_fp: bool = can_be_local_fp(depth, insn.src_reg, src);
//
// Reload from callee stack: if src is current-frame FP-derived
// and the load is an 8-byte BPF_MEM, try to restore the spill
// identity.  For imprecise sources fill_from_stack() returns
// ARG_IMPRECISE (off_cnt == 0).
//
    if (src_is_local_fp && BPF_MODE(insn.code) == BPF_MEM && sz == 8) {
// dst = fill_from_stack(insn, at_out, insn->src_reg, at_stack_out, depth);
    } else if (src.frame >= 0 && src.frame < depth &&
    BPF_MODE(insn.code) == BPF_MEM && sz == 8) {
    struct arg_track *parent_stack =
    env.callsite_at_stack[callsites[src.frame]];
// dst = fill_from_stack(insn, at_out, insn->src_reg,
    parent_stack, src.frame);
    } else if (src.frame == ARG_IMPRECISE &&
    !(src.mask & BIT(depth)) && src.mask &&
    BPF_MODE(insn.code) == BPF_MEM && sz == 8) {
//
// Imprecise src with only parent-frame bits:
// conservative fallback.
//
// dst = *src;
    } else {
// dst = none;
    }
    } else if (class == BPF_LD && BPF_MODE(insn.code) == BPF_IMM) {
// dst = none;
    } else if (class == BPF_STX) {
    let mut sz: u32 = bpf_size_to_bytes(BPF_SIZE(insn.code));
    bool dst_is_local_fp;
// Track spills to current-frame FP-derived callee stack
    dst_is_local_fp = can_be_local_fp(depth, insn.dst_reg, dst);
    if (dst_is_local_fp && BPF_MODE(insn.code) == BPF_MEM)
    spill_to_stack(insn, at_out, insn.dst_reg,
    at_stack_out, src, sz);
    if (BPF_MODE(insn.code) == BPF_ATOMIC) {
    if (dst_is_local_fp && insn.imm != BPF_LOAD_ACQ)
    clear_stack_for_all_offs(insn, at_out, insn.dst_reg,
    at_stack_out, sz);
    r = bpf_atomic_load_reg(insn);
    if (r >= 0)
    at_out[r] = none;
    }
    } else if (class == BPF_ST && BPF_MODE(insn.code) == BPF_MEM) {
    let mut sz: u32 = bpf_size_to_bytes(BPF_SIZE(insn.code));
    let mut dst_is_local_fp: bool = can_be_local_fp(depth, insn.dst_reg, dst);
// BPF_ST to FP-derived dst: clear overlapping stack slots
    if (dst_is_local_fp)
    clear_stack_for_all_offs(insn, at_out, insn.dst_reg,
    at_stack_out, sz);
    }
    }
//
// Record access_bytes from helper/kfunc or load/store insn.
// access_bytes > 0:      stack read
// access_bytes < 0:      stack write
// access_bytes == S64_MIN: unknown   — conservative, mark [0..slot] as read
// access_bytes == 0:      no access
//
    static int record_stack_access_off(struct func_instance *instance, s64 fp_off,
    s64 access_bytes, u32 frame, u32 insn_idx)
    {
    s32 slot_hi, slot_lo;
    spis_t mask;
    if (fp_off >= 0)
//
// out of bounds stack access doesn't contribute
// into actual stack liveness. It will be rejected
// by the main verifier pass later.
//
    return 0;
    if (access_bytes == S64_MIN) {
// helper/kfunc read unknown amount of bytes from fp_off until fp+0
    slot_hi = (-fp_off - 1) / STACK_SLOT_SZ;
    mask = SPIS_ZERO;
    spis_or_range(&mask, 0, slot_hi);
    return mark_stack_read(instance, frame, insn_idx, mask);
    }
    if (access_bytes > 0) {
// Mark any touched slot as use
    slot_hi = (-fp_off - 1) / STACK_SLOT_SZ;
    slot_lo = max_t(s32, (-fp_off - access_bytes) / STACK_SLOT_SZ, 0);
    mask = SPIS_ZERO;
    spis_or_range(&mask, slot_lo, slot_hi);
    return mark_stack_read(instance, frame, insn_idx, mask);
    } else if (access_bytes < 0) {
// Mark only fully covered slots as def
    access_bytes = -access_bytes;
    slot_hi = (-fp_off) / STACK_SLOT_SZ - 1;
    slot_lo = max_t(s32, (-fp_off - access_bytes + STACK_SLOT_SZ - 1) / STACK_SLOT_SZ, 0);
    if (slot_lo <= slot_hi) {
    mask = SPIS_ZERO;
    spis_or_range(&mask, slot_lo, slot_hi);
    return mark_stack_write(instance, frame, insn_idx, mask);
    }
    }
    return 0;
    }
//
// 'arg' is FP-derived argument to helper/kfunc or load/store that
// reads (positive) or writes (negative) 'access_bytes' into 'use' or 'def'.
//
    static int record_stack_access(struct func_instance *instance,
    const struct arg_track *arg,
    s64 access_bytes, u32 frame, u32 insn_idx)
    {
    int i, err;
    if (access_bytes == 0)
    return 0;
    if (arg.off_cnt == 0) {
    if (access_bytes > 0 || access_bytes == S64_MIN)
    return mark_stack_read(instance, frame, insn_idx, SPIS_ALL);
    return 0;
    }
    if (access_bytes != S64_MIN && access_bytes < 0 && arg.off_cnt != 1)
// multi-offset write cannot set stack_def
    return 0;
    for (i = 0; i < arg.off_cnt; i++) {
    err = record_stack_access_off(instance, arg.off[i], access_bytes, frame, insn_idx);
    if (err)
    return err;
    }
    return 0;
    }
//
// When a pointer is ARG_IMPRECISE, conservatively mark every frame in
// the bitmask as fully used.
//
#[no_mangle]
unsafe extern "C" fn record_imprecise(instance: *mut func_instance, mask: u32, insn_idx: u32) -> c_int {
    static int record_imprecise(struct func_instance *instance, u32 mask, u32 insn_idx)
    {
    let mut depth: c_int = instance.depth;
    int f, err;
    for (f = 0; mask; f++, mask >>= 1) {
    if (!(mask & 1))
    continue;
    if (f <= depth) {
    err = mark_stack_read(instance, f, insn_idx, SPIS_ALL);
    if (err)
    return err;
    }
    }
    return 0;
    }
// Record load/store access for a given 'at' state of 'insn'.
    static int record_load_store_access(struct bpf_verifier_env *env,
    struct func_instance *instance,
    struct arg_track *at, int insn_idx)
    {
    struct bpf_insn *insn = &env.prog.insnsi[insn_idx];
    let mut depth: c_int = instance.depth;
    let mut sz: i32 = bpf_size_to_bytes(BPF_SIZE(insn.code));
    let mut class: u8 = BPF_CLASS(insn.code);
    struct arg_track resolved, *ptr;
    int oi;
//
// Stack arg insns use dst_reg/src_reg=BPF_REG_PARAMS(11). Since at[]
// is extended to MAX_AT_TRACK_REGS, at[11] holds the arg_track for
// outgoing stack arg slot 0 — not the pointer used for the memory
// access. Skip so the slot's tracked value isn't confused with the
// base register that record_stack_access() expects.
//
    if (is_stack_arg_stx(insn) || is_stack_arg_st(insn) || is_stack_arg_ldx(insn))
    return 0;
    switch (class) {
    case BPF_LDX:
    ptr = &at[insn.src_reg];
    break;
    case BPF_STX:
    if (BPF_MODE(insn.code) == BPF_ATOMIC) {
    if (insn.imm == BPF_STORE_REL)
    sz = -sz;
    if (insn.imm == BPF_LOAD_ACQ)
    ptr = &at[insn.src_reg];
    else
    ptr = &at[insn.dst_reg];
    } else {
    ptr = &at[insn.dst_reg];
    sz = -sz;
    }
    break;
    case BPF_ST:
    ptr = &at[insn.dst_reg];
    sz = -sz;
    break;
    default:
    return 0;
    }
// Resolve offsets: fold insn->off into arg_track
    if (ptr.off_cnt > 0) {
    resolved.off_cnt = ptr.off_cnt;
    resolved.frame = ptr.frame;
    for (oi = 0; oi < ptr.off_cnt; oi++) {
    if (arg_add(ptr.off[oi], insn.off, &resolved.off[oi])) {
    resolved.off_cnt = 0;
    break;
    }
    }
    ptr = &resolved;
    }
    if (ptr.frame >= 0 && ptr.frame <= depth)
    return record_stack_access(instance, ptr, sz, ptr.frame, insn_idx);
    if (ptr.frame == ARG_IMPRECISE)
    return record_imprecise(instance, ptr.mask, insn_idx);
// ARG_NONE: not derived from any frame pointer, skip
    return 0;
    }
    static int record_arg_access(struct bpf_verifier_env *env,
    struct func_instance *instance,
    struct bpf_insn *insn,
    struct arg_track *at, int arg_idx,
    int insn_idx)
    {
    let mut depth: c_int = instance.depth;
    let mut frame: c_int = at.frame;
    let mut err: c_int = 0;
    s64 bytes;
    if (!arg_is_fp(at))
    return 0;
    if (bpf_helper_call(insn)) {
    bytes = bpf_helper_stack_access_bytes(env, insn, arg_idx, insn_idx);
    } else if (bpf_pseudo_kfunc_call(insn)) {
    bytes = bpf_kfunc_stack_access_bytes(env, insn, arg_idx, insn_idx);
    } else {
    for (int f = 0; f <= depth; f++) {
    err = mark_stack_read(instance, f, insn_idx, SPIS_ALL);
    if (err)
    return err;
    }
    return 0;
    }
    if (bytes == 0)
    return 0;
    if (frame >= 0 && frame <= depth)
    err = record_stack_access(instance, at, bytes, frame, insn_idx);
#[no_mangle]
pub unsafe extern "C" fn if(ARG_IMPRECISE: frame ==) -> else {
    else if (frame == ARG_IMPRECISE)
    err = record_imprecise(instance, at.mask, insn_idx);
    return err;
    }
// Record stack access for a given 'at' state of helper/kfunc 'insn'
    static int record_call_access(struct bpf_verifier_env *env,
    struct func_instance *instance,
    struct arg_track *at,
    int insn_idx)
    {
    struct bpf_insn *insn = &env.prog.insnsi[insn_idx];
    struct bpf_call_summary cs;
    int r, err, num_params = 5;
    if (bpf_pseudo_call(insn))
    return 0;
    if (bpf_get_call_summary(env, insn, &cs))
    num_params = cs.num_params;
    for (r = BPF_REG_1; r < BPF_REG_1 + min(num_params, MAX_BPF_FUNC_REG_ARGS); r++) {
    err = record_arg_access(env, instance, insn, &at[r], r - 1, insn_idx);
    if (err)
    return err;
    }
    for (r = 0; r < MAX_STACK_ARG_SLOTS && r < num_params - MAX_BPF_FUNC_REG_ARGS; r++) {
    err = record_arg_access(env, instance, insn, &at[MAX_BPF_REG + r],
    r + MAX_BPF_FUNC_REG_ARGS, insn_idx);
    if (err)
    return err;
    }
    return 0;
    }
//
// For a calls_callback helper, find the callback subprog and determine
// which caller register maps to which callback register for FP passthrough.
//
    static int find_callback_subprog(struct bpf_verifier_env *env,
    struct bpf_insn *insn, int insn_idx,
    int *caller_reg, int *callee_reg)
    {
    struct bpf_insn_aux_data *aux = &env.insn_aux_data[insn_idx];
    let mut cb_reg: c_int = -1;
// caller_reg = -1;
// callee_reg = -1;
    if (!bpf_helper_call(insn))
    return -1;
    switch (insn.imm) {
    case BPF_FUNC_loop:
// bpf_loop(nr, cb, ctx, flags): cb=R2, R3->cb R2
    cb_reg = BPF_REG_2;
// caller_reg = BPF_REG_3;
// callee_reg = BPF_REG_2;
    break;
    case BPF_FUNC_for_each_map_elem:
// for_each_map_elem(map, cb, ctx, flags): cb=R2, R3->cb R4
    cb_reg = BPF_REG_2;
// caller_reg = BPF_REG_3;
// callee_reg = BPF_REG_4;
    break;
    case BPF_FUNC_find_vma:
// find_vma(task, addr, cb, ctx, flags): cb=R3, R4->cb R3
    cb_reg = BPF_REG_3;
// caller_reg = BPF_REG_4;
// callee_reg = BPF_REG_3;
    break;
    case BPF_FUNC_user_ringbuf_drain:
// user_ringbuf_drain(map, cb, ctx, flags): cb=R2, R3->cb R2
    cb_reg = BPF_REG_2;
// caller_reg = BPF_REG_3;
// callee_reg = BPF_REG_2;
    break;
    default:
    return -1;
    }
    if (!(aux.const_reg_subprog_mask & BIT(cb_reg)))
    return -2;
    return aux.const_reg_vals[cb_reg];
    }
// Per-subprog intermediate state kept alive across analysis phases
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subprog_at_info {
    pub (*at_in)[MAX_AT_TRACK_REGS]: *mut arg_track,
    pub len: c_int,
}

    static void print_subprog_arg_access(struct bpf_verifier_env *env,
    int subprog,
    struct subprog_at_info *info,
#[no_mangle]
pub unsafe extern "C" fn arg_track(_arg: *mut at_stack_in)[MAX_ARG_SPILL_SLOTS]) -> struct {
    struct arg_track (*at_stack_in)[MAX_ARG_SPILL_SLOTS])
    {
    struct bpf_insn *insns = env.prog.insnsi;
    let mut start: c_int = env.subprog_info[subprog].start;
    let mut len: c_int = info.len;
    int i, r;
    if (!(env.log.level & BPF_LOG_LEVEL2))
    return;
    verbose(env, "%s:\n", fmt_subprog(env, subprog));
    for (i = 0; i < len; i++) {
    let mut idx: c_int = start + i;
    let mut has_extra: bool = false;
    let mut cls: u8 = BPF_CLASS(insns[idx].code);
    bool is_ldx_stx_call = cls == BPF_LDX || cls == BPF_STX ||
    insns[idx].code == (BPF_JMP | BPF_CALL);
    verbose(env, "%3d: ", idx);
    bpf_verbose_insn(env, &insns[idx]);
    verbose(env, "\n");
// Collect what needs printing
    if (is_ldx_stx_call &&
    arg_is_visited(&info.at_in[i][0])) {
    for (r = 0; r < MAX_BPF_REG - 1; r++)
    if (arg_is_fp(&info.at_in[i][r]))
    has_extra = true;
    for (r = 0; r < MAX_STACK_ARG_SLOTS; r++)
    if (arg_is_fp(&info.at_in[i][MAX_BPF_REG + r]))
    has_extra = true;
    }
    if (is_ldx_stx_call) {
    for (r = 0; r < MAX_ARG_SPILL_SLOTS; r++)
    if (arg_is_fp(&at_stack_in[i][r]))
    has_extra = true;
    }
    if (!has_extra) {
    if (bpf_is_ldimm64(&insns[idx]))
    i++;
    continue;
    }
    bpf_vlog_reset(&env.log, env.log.end_pos - 1);
    verbose(env, " //");
    if (is_ldx_stx_call && info.at_in &&
    arg_is_visited(&info.at_in[i][0])) {
    for (r = 0; r < MAX_BPF_REG - 1; r++) {
    if (!arg_is_fp(&info.at_in[i][r]))
    continue;
    verbose(env, " r%d=", r);
    verbose_arg_track(env, &info.at_in[i][r]);
    }
    for (r = 0; r < MAX_STACK_ARG_SLOTS; r++) {
    if (!arg_is_fp(&info.at_in[i][MAX_BPF_REG + r]))
    continue;
    verbose(env, " sa%d=", r);
    verbose_arg_track(env, &info.at_in[i][MAX_BPF_REG + r]);
    }
    }
    if (is_ldx_stx_call) {
    for (r = 0; r < MAX_ARG_SPILL_SLOTS; r++) {
    if (!arg_is_fp(&at_stack_in[i][r]))
    continue;
    verbose(env, " fp%+d=", -(r + 1) * 8);
    verbose_arg_track(env, &at_stack_in[i][r]);
    }
    }
    verbose(env, "\n");
    if (bpf_is_ldimm64(&insns[idx]))
    i++;
    }
    }
//
// Compute arg tracking dataflow for a single subprog.
// Runs forward fixed-point with arg_track_xfer(), then records
// memory accesses in a single linear pass over converged state.
//
// @callee_entry: pre-populated entry state for R1-R5 and stack args
// NULL for main (subprog 0).
// @info:         stores at_in, len for debug printing.
//
    static int compute_subprog_args(struct bpf_verifier_env *env,
    struct subprog_at_info *info,
    struct arg_track *callee_entry,
    struct func_instance *instance,
    u32 *callsites)
    {
    let mut subprog: c_int = instance.subprog;
    struct bpf_insn *insns = env.prog.insnsi;
    let mut depth: c_int = instance.depth;
    let mut start: c_int = env.subprog_info[subprog].start;
    let mut po_start: c_int = env.subprog_info[subprog].postorder_start;
    let mut end: c_int = env.subprog_info[subprog + 1].start;
    let mut po_end: c_int = env.subprog_info[subprog + 1].postorder_start;
    let mut len: c_int = end - start;
    struct arg_track (*at_in)[MAX_AT_TRACK_REGS] = core::ptr::null_mut();
    struct arg_track at_out[MAX_AT_TRACK_REGS];
    struct arg_track (*at_stack_in)[MAX_ARG_SPILL_SLOTS] = core::ptr::null_mut();
    struct arg_track *at_stack_out = core::ptr::null_mut();
    struct arg_track at_stack_arg_entry[MAX_STACK_ARG_SLOTS];
    let mut unvisited: arg_track = { .frame = ARG_UNVISITED };
    let mut none: arg_track = { .frame = ARG_NONE };
    bool changed;
    int i, p, r, err = -ENOMEM;
    at_in = kvmalloc_objs(*at_in, len, GFP_KERNEL_ACCOUNT);
    if (!at_in)
    goto err_free;
    at_stack_in = kvmalloc_objs(*at_stack_in, len, GFP_KERNEL_ACCOUNT);
    if (!at_stack_in)
    goto err_free;
    at_stack_out = kvmalloc_objs(*at_stack_out, MAX_ARG_SPILL_SLOTS, GFP_KERNEL_ACCOUNT);
    if (!at_stack_out)
    goto err_free;
    for (i = 0; i < len; i++) {
    for (r = 0; r < MAX_AT_TRACK_REGS; r++)
    at_in[i][r] = unvisited;
    for (r = 0; r < MAX_ARG_SPILL_SLOTS; r++)
    at_stack_in[i][r] = unvisited;
    }
    for (r = 0; r < MAX_AT_TRACK_REGS; r++)
    at_in[0][r] = none;
// Entry: R10 is always precisely the current frame's FP
    at_in[0][BPF_REG_FP] = arg_single(depth, 0);
// R1-R5: from caller or ARG_NONE for main
    if (callee_entry) {
    for (r = BPF_REG_1; r <= BPF_REG_5; r++)
    at_in[0][r] = callee_entry[r];
    }
// Entry: all stack slots are ARG_NONE
    for (r = 0; r < MAX_ARG_SPILL_SLOTS; r++)
    at_stack_in[0][r] = none;
// Entry: incoming stack args from caller, or ARG_NONE for main
    for (r = 0; r < MAX_STACK_ARG_SLOTS; r++)
    at_stack_arg_entry[r] = callee_entry ? callee_entry[MAX_BPF_REG + r] : none;
    if (env.log.level & BPF_LOG_LEVEL2)
    verbose(env, "subprog#%d: analyzing (depth %d)...\n", subprog, depth);
// Forward fixed-point iteration in reverse post order
    redo:
    changed = false;
    for (p = po_end - 1; p >= po_start; p--) {
    let mut idx: c_int = env.cfg.insn_postorder[p];
    let mut i: c_int = idx - start;
    struct bpf_insn *insn = &insns[idx];
    struct bpf_iarray *succ;
    if (!arg_is_visited(&at_in[i][0]) && !arg_is_visited(&at_in[i][1]))
    continue;
    memcpy(at_out, at_in[i], sizeof(at_out));
    memcpy(at_stack_out, at_stack_in[i], MAX_ARG_SPILL_SLOTS * sizeof(*at_stack_out));
    arg_track_xfer(env, insn, idx, at_out, at_stack_out,
    at_stack_arg_entry, instance, callsites);
    arg_track_log(env, insn, idx, at_in[i], at_stack_in[i], at_out, at_stack_out);
// Propagate to successors within this subprogram
    succ = bpf_insn_successors(env, idx);
    for (int s = 0; s < succ.cnt; s++) {
    let mut target: c_int = succ.items[s];
    int ti;
// Filter: stay within the subprogram's range
    if (target < start || target >= end)
    continue;
    ti = target - start;
    for (r = 0; r < MAX_AT_TRACK_REGS; r++)
    changed |= arg_track_join(env, idx, target, r,
    &at_in[ti][r], at_out[r]);
    for (r = 0; r < MAX_ARG_SPILL_SLOTS; r++)
    changed |= arg_track_join(env, idx, target, -r - 1,
    &at_stack_in[ti][r], at_stack_out[r]);
    }
    }
    if (changed)
    goto redo;
// Record memory accesses using converged at_in (RPO skips dead code)
    for (p = po_end - 1; p >= po_start; p--) {
    let mut idx: c_int = env.cfg.insn_postorder[p];
    let mut i: c_int = idx - start;
    struct bpf_insn *insn = &insns[idx];
    err = record_load_store_access(env, instance, at_in[i], idx);
    if (err)
    goto err_free;
    if (insn.code == (BPF_JMP | BPF_CALL)) {
    err = record_call_access(env, instance, at_in[i], idx);
    if (err)
    goto err_free;
    }
    if (bpf_pseudo_call(insn) || bpf_calls_callback(env, idx)) {
    kvfree(env.callsite_at_stack[idx]);
    env.callsite_at_stack[idx] =
    kvmalloc_objs(*env.callsite_at_stack[idx],
    MAX_ARG_SPILL_SLOTS, GFP_KERNEL_ACCOUNT);
    if (!env.callsite_at_stack[idx]) {
    err = -ENOMEM;
    goto err_free;
    }
    memcpy(env.callsite_at_stack[idx],
    at_stack_in[i], sizeof(struct arg_track) * MAX_ARG_SPILL_SLOTS);
    }
    }
    info.at_in = at_in;
    at_in = core::ptr::null_mut();
    info.len = len;
    print_subprog_arg_access(env, subprog, info, at_stack_in);
    err = 0;
    err_free:
    kvfree(at_stack_out);
    kvfree(at_stack_in);
    kvfree(at_in);
    return err;
    }
// Return true if any of R1-R5 or stack args is derived from a frame pointer.
#[no_mangle]
unsafe extern "C" fn has_fp_args(args: *mut arg_track) -> bool {
    static bool has_fp_args(struct arg_track *args)
    {
    for (int r = BPF_REG_1; r <= BPF_REG_5; r++)
    if (arg_is_fp(&args[r]))
    return true;
    for (int r = 0; r < MAX_STACK_ARG_SLOTS; r++)
    if (arg_is_fp(&args[MAX_BPF_REG + r]))
    return true;
    return false;
    }
//
// Merge a freshly analyzed instance into the original.
// may_read: union (any pass might read the slot).
// must_write: intersection (only slots written on ALL passes are guaranteed).
// live_before is recomputed by a subsequent update_instance() on @dst.
//
#[no_mangle]
unsafe extern "C" fn merge_instances(dst: *mut func_instance, src: *mut func_instance) {
    static void merge_instances(struct func_instance *dst, struct func_instance *src)
    {
    int f, i;
    for (f = 0; f <= dst.depth; f++) {
    if (!src.frames[f]) {
// This pass didn't touch frame f — must_write intersects with empty.
    if (dst.frames[f])
    for (i = 0; i < dst.insn_cnt; i++)
    dst.frames[f][i].must_write = SPIS_ZERO;
    continue;
    }
    if (!dst.frames[f]) {
// Previous pass didn't touch frame f — take src, zero must_write.
    dst.frames[f] = src.frames[f];
    src.frames[f] = core::ptr::null_mut();
    for (i = 0; i < dst.insn_cnt; i++)
    dst.frames[f][i].must_write = SPIS_ZERO;
    continue;
    }
    for (i = 0; i < dst.insn_cnt; i++) {
    dst.frames[f][i].may_read =
    spis_or(dst.frames[f][i].may_read,
    src.frames[f][i].may_read);
    dst.frames[f][i].must_write =
    spis_and(dst.frames[f][i].must_write,
    src.frames[f][i].must_write);
    }
    }
    }
    static struct func_instance *fresh_instance(struct func_instance *src)
    {
    struct func_instance *f;
    f = kvzalloc_obj(*f, GFP_KERNEL_ACCOUNT);
    if (!f)
    return ERR_PTR(-ENOMEM);
    f.callsite = src.callsite;
    f.depth = src.depth;
    f.subprog = src.subprog;
    f.subprog_start = src.subprog_start;
    f.insn_cnt = src.insn_cnt;
    return f;
    }
#[no_mangle]
unsafe extern "C" fn free_instance(instance: *mut func_instance) {
    static void free_instance(struct func_instance *instance)
    {
    int i;
    for (i = 0; i <= instance.depth; i++)
    kvfree(instance.frames[i]);
    kvfree(instance);
    }
//
// Recursively analyze a subprog with specific 'entry_args'.
// Each callee is analyzed with the exact args from its call site.
//
// Args are recomputed for each call because the dataflow result at_in[]
// depends on the entry args and frame depth. Consider: A->C->D and B->C->D
// Callsites in A and B pass different args into C, so C is recomputed.
// Then within C the same callsite passes different args into D.
//
    static int analyze_subprog(struct bpf_verifier_env *env,
    struct arg_track *entry_args,
    struct subprog_at_info *info,
    struct func_instance *instance,
    u32 *callsites)
    {
    let mut subprog: c_int = instance.subprog;
    let mut depth: c_int = instance.depth;
    struct bpf_insn *insns = env.prog.insnsi;
    let mut start: c_int = env.subprog_info[subprog].start;
    let mut po_start: c_int = env.subprog_info[subprog].postorder_start;
    let mut po_end: c_int = env.subprog_info[subprog + 1].postorder_start;
    struct func_instance *prev_instance = core::ptr::null_mut();
    int j, err;
    if (++env.liveness.subprog_calls > 10000) {
    verbose(env, "liveness analysis exceeded complexity limit (%d calls)\n",
    env.liveness.subprog_calls);
    return -E2BIG;
    }
    if (need_resched())
    cond_resched();
//
// When an instance is reused (must_write_initialized == true),
// record into a fresh instance and merge afterward.  This avoids
// stale must_write marks for instructions not reached in this pass.
//
    if (instance.must_write_initialized) {
    struct func_instance *fresh = fresh_instance(instance);
    if (IS_ERR(fresh))
    return PTR_ERR(fresh);
    prev_instance = instance;
    instance = fresh;
    }
// Free prior analysis if this subprog was already visited
    kvfree(info[subprog].at_in);
    info[subprog].at_in = core::ptr::null_mut();
    err = compute_subprog_args(env, &info[subprog], entry_args, instance, callsites);
    if (err)
    goto out_free;
// For each reachable call site in the subprog, recurse into callees
    for (int p = po_start; p < po_end; p++) {
    let mut idx: c_int = env.cfg.insn_postorder[p];
    struct arg_track callee_args[MAX_AT_TRACK_REGS] = {};
    let mut none: arg_track = { .frame = ARG_NONE };
    struct bpf_insn *insn = &insns[idx];
    struct func_instance *callee_instance;
    int callee, target;
    int caller_reg, cb_callee_reg;
    j = idx - start; /* relative index within this subprog */
    if (bpf_pseudo_call(insn)) {
    target = idx + insn.imm + 1;
    callee = bpf_find_subprog(env, target);
    if (callee < 0)
    continue;
// Build entry args: R1-R5 and stack args from at_in at call site
    for (int r = BPF_REG_1; r <= BPF_REG_5; r++)
    callee_args[r] = info[subprog].at_in[j][r];
    for (int r = 0; r < MAX_STACK_ARG_SLOTS; r++)
    callee_args[MAX_BPF_REG + r] = info[subprog].at_in[j][MAX_BPF_REG + r];
    } else if (bpf_calls_callback(env, idx)) {
    callee = find_callback_subprog(env, insn, idx, &caller_reg, &cb_callee_reg);
    if (callee == -2) {
//
// same bpf_loop() calls two different callbacks and passes
// stack pointer to them
//
    if (info[subprog].at_in[j][caller_reg].frame == ARG_NONE)
    continue;
    for (int f = 0; f <= depth; f++) {
    err = mark_stack_read(instance, f, idx, SPIS_ALL);
    if (err)
    goto out_free;
    }
    continue;
    }
    if (callee < 0)
    continue;
    for (int r = BPF_REG_1; r <= BPF_REG_5; r++)
    callee_args[r] = none;
    for (int r = 0; r < MAX_STACK_ARG_SLOTS; r++)
    callee_args[MAX_BPF_REG + r] = none;
    callee_args[cb_callee_reg] = info[subprog].at_in[j][caller_reg];
    } else {
    continue;
    }
    if (!has_fp_args(callee_args))
    continue;
    if (depth == MAX_CALL_FRAMES - 1) {
    err = -EINVAL;
    goto out_free;
    }
    callee_instance = call_instance(env, instance, idx, callee);
    if (IS_ERR(callee_instance)) {
    err = PTR_ERR(callee_instance);
    goto out_free;
    }
    callsites[depth] = idx;
    err = analyze_subprog(env, callee_args, info, callee_instance, callsites);
    if (err)
    goto out_free;
// Pull callee's entry liveness back to caller's callsite
    {
    let mut callee_start: u32 = callee_instance.subprog_start;
    struct per_frame_masks *entry;
    for (int f = 0; f < callee_instance.depth; f++) {
    entry = get_frame_masks(callee_instance, f, callee_start);
    if (!entry)
    continue;
    err = mark_stack_read(instance, f, idx, entry.live_before);
    if (err)
    goto out_free;
    }
    }
    }
    if (prev_instance) {
    merge_instances(prev_instance, instance);
    free_instance(instance);
    instance = prev_instance;
    }
    update_instance(env, instance);
    return 0;
    out_free:
    if (prev_instance)
    free_instance(instance);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_compute_subprog_arg_access(env: *mut bpf_verifier_env) -> c_int {
    int bpf_compute_subprog_arg_access(struct bpf_verifier_env *env)
    {
    u32 callsites[MAX_CALL_FRAMES] = {};
    let mut insn_cnt: c_int = env.prog.len;
    struct func_instance *instance;
    struct subprog_at_info *info;
    int k, err = 0;
    info = kvzalloc_objs(*info, env.subprog_cnt, GFP_KERNEL_ACCOUNT);
    if (!info)
    return -ENOMEM;
    env.callsite_at_stack = kvzalloc_objs(*env.callsite_at_stack, insn_cnt,
    GFP_KERNEL_ACCOUNT);
    if (!env.callsite_at_stack) {
    kvfree(info);
    return -ENOMEM;
    }
//
// Analyze every subprog in reverse topological order (callers
// before callees) so that each subprog is analyzed before its
// callees, allowing the recursive walk inside analyze_subprog()
// to naturally reach callees that receive FP-derived args.
//
// Subprogs and callbacks that don't receive FP-derived arguments
// cannot access ancestor stack frames are analyzed independently.
// Async callbacks (timer, workqueue) are handled the same way.
//
    for (k = env.subprog_cnt - 1; k >= 0; k--) {
    let mut sub: c_int = env.subprog_topo_order[k];
    if (info[sub].at_in && !bpf_subprog_is_global(env, sub))
    continue;
    instance = call_instance(env, core::ptr::null_mut(), 0, sub);
    if (IS_ERR(instance)) {
    err = PTR_ERR(instance);
    goto out;
    }
    err = analyze_subprog(env, core::ptr::null_mut(), info, instance, callsites);
    if (err)
    goto out;
    }
    if (env.log.level & BPF_LOG_LEVEL2)
    err = print_instances(env);
    out:
    for (k = 0; k < insn_cnt; k++)
    kvfree(env.callsite_at_stack[k]);
    kvfree(env.callsite_at_stack);
    env.callsite_at_stack = core::ptr::null_mut();
    for (k = 0; k < env.subprog_cnt; k++)
    kvfree(info[k].at_in);
    kvfree(info);
    return err;
    }
// Each field is a register bitmask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_live_regs {
    pub /: *mut *mut u32 use; / registers read by instruction,
    pub /: *mut *mut u32 def; / registers written by instruction,
    pub /: *mut *mut u32 in; / registers that may be alive before instruction,
    pub /: *mut *mut u32 out; / registers that may be alive after instruction,
}

// Bitmask with 1s for all caller saved registers

    static inline u32 reg32_mask(u32 n) { return BIT(n); }
    static inline u32 reg64_mask(u32 n) { return BIT(n) | BIT(n + 16); }
    static inline u32 mask_widen(u32 m) { return m | (m << 16); }
    static inline u16 mask_lo(u32 m) { return (u16)m; }
    static inline u16 mask_hi(u32 m) { return (u16)(m >> 16); }
// Compute info->{use,def} fields for the instruction
    static void compute_insn_live_regs(struct bpf_verifier_env *env,
    struct bpf_insn *insn,
    struct insn_live_regs *info)
    {
    struct bpf_call_summary cs;
    let mut class: u8 = BPF_CLASS(insn.code);
    let mut code: u8 = BPF_OP(insn.code);
    let mut mode: u8 = BPF_MODE(insn.code);
    let mut size: u8 = BPF_SIZE(insn.code);
    let mut src: u32 = reg64_mask(insn.src_reg);
    let mut dst: u32 = reg64_mask(insn.dst_reg);
    let mut src32: u32 = mask_lo(src);
    let mut dst32: u32 = mask_lo(dst);
    let mut r0: u32 = reg64_mask(0);
    let mut def: u32 = 0;
    let mut use: u32 = U32_MAX;
    switch (class) {
    case BPF_LD:
    switch (mode) {
    case BPF_IMM:
    if (BPF_SIZE(insn.code) == BPF_DW) {
    def = dst;
    use = 0;
    }
    break;
    case BPF_ABS:
    case BPF_IND:
// stick with defaults
    break;
    }
    break;
    case BPF_LDX:
    switch (mode) {
    case BPF_MEM:
// a narrow load still redefines the whole register
    def = dst;
    use = src;
    break;
    case BPF_MEMSX:
//
// sign extension defines the whole register;
// src holds a pointer, hence is used as 64-bit.
//
    def = dst;
    use = src;
    break;
    }
    break;
    case BPF_ST:
    switch (mode) {
    case BPF_MEM:
    def = 0;
    use = dst;
    break;
    }
    break;
    case BPF_STX:
    switch (mode) {
    case BPF_MEM:
    def = 0;
    use = dst | (size == BPF_DW ? src : src32);
    break;
    case BPF_ATOMIC: {
//
// dst holds a pointer and is always used as 64-bit;
// the value operand and r0 are read as 32-bit for BPF_W atomics.
//
    let mut srcv: u32 = size == BPF_DW ? src : src32;
    let mut r0v: u32 = size == BPF_DW ? r0 : mask_lo(r0);
    switch (insn.imm) {
    case BPF_CMPXCHG:
    use = r0v | dst | srcv;
    def = r0;
    break;
    case BPF_LOAD_ACQ:
    def = dst;
    use = src;
    break;
    case BPF_STORE_REL:
    def = 0;
    use = dst | srcv;
    break;
    default:
    use = dst | srcv;
    if (insn.imm & BPF_FETCH)
    def = src;
    else
    def = 0;
    }
    break;
    }
    }
    break;
    case BPF_ALU:
    case BPF_ALU64:
    switch (code) {
    case BPF_END:
    use = dst;
    def = dst;
    break;
    case BPF_MOV:
    def = dst;
    if (BPF_SRC(insn.code) == BPF_K)
    use = 0;
    else
    use = class == BPF_ALU64 ? src : src32;
    break;
    default:
    def = dst;
    if (BPF_SRC(insn.code) == BPF_K)
    use = class == BPF_ALU64 ? dst : dst32;
    else
    use = class == BPF_ALU64 ? (dst | src) : (dst32 | src32);
    }
    break;
    case BPF_JMP:
    case BPF_JMP32:
    switch (code) {
    case BPF_JA:
    def = 0;
    if (BPF_SRC(insn.code) == BPF_X)
    use = dst;
    else
    use = 0;
    break;
    case BPF_JCOND:
    def = 0;
    use = 0;
    break;
    case BPF_EXIT:
    def = 0;
    use = r0;
    break;
    case BPF_CALL:
    def = ALL_CALLER_SAVED_REGS;
    use = def & ~BIT(BPF_REG_0);
    if (bpf_get_call_summary(env, insn, &cs))
    use = GENMASK(min_t(u8, cs.num_params, MAX_BPF_FUNC_REG_ARGS), 1);
    def = mask_widen(def);
    use = mask_widen(use);
    break;
    default:
    def = 0;
    use = class == BPF_JMP ? dst : dst32;
    if (BPF_SRC(insn.code) == BPF_X)
    use |= class == BPF_JMP ? src : src32;
    }
    break;
    }
    info.def = def;
    info.use = use;
    }
// Compute may-live registers after each instruction in the program.
// The register is live after the instruction I if it is read by some
// instruction S following I during program execution and is not
// overwritten between I and S.
//
// Store result in env->insn_aux_data[i].live_regs.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_compute_live_registers(env: *mut bpf_verifier_env) -> c_int {
    int bpf_compute_live_registers(struct bpf_verifier_env *env)
    {
    struct bpf_insn_aux_data *insn_aux = env.insn_aux_data;
    struct bpf_insn *insns = env.prog.insnsi;
    struct insn_live_regs *state;
    let mut insn_cnt: c_int = env.prog.len;
    u64 pos, insn_pos;
    let mut err: c_int = 0, i, j;
    bool changed;
// Use the following algorithm:
// - define the following:
// - I.use : a set of all registers read by instruction I;
// - I.def : a set of all registers written by instruction I;
// - I.in  : a set of all registers that may be alive before I execution;
// - I.out : a set of all registers that may be alive after I execution;
// - insn_successors(I): a set of instructions S that might immediately
// follow I for some program execution;
// - associate separate empty sets 'I.in' and 'I.out' with each instruction;
// - visit each instruction in a postorder and update
// state[i].in, state[i].out as follows:
//
// state[i].out = U [state[s].in for S in insn_successors(i)]
// state[i].in  = (state[i].out / state[i].def) U state[i].use
//
// (where U stands for set union, / stands for set difference)
// - repeat the computation while {in,out} fields changes for
// any instruction.
//
    state = kvzalloc_objs(*state, insn_cnt, GFP_KERNEL_ACCOUNT);
    if (!state) {
    err = -ENOMEM;
    goto out;
    }
    for (i = 0; i < insn_cnt; ++i)
    compute_insn_live_regs(env, &insns[i], &state[i]);
// Forward pass: resolve stack access through FP-derived pointers
    err = bpf_compute_subprog_arg_access(env);
    if (err)
    goto out;
    changed = true;
    while (changed) {
    changed = false;
    for (i = 0; i < env.cfg.cur_postorder; ++i) {
    let mut insn_idx: c_int = env.cfg.insn_postorder[i];
    struct insn_live_regs *live = &state[insn_idx];
    struct bpf_iarray *succ;
    let mut new_out: u32 = 0;
    let mut new_in: u32 = 0;
    succ = bpf_insn_successors(env, insn_idx);
    for (int s = 0; s < succ.cnt; ++s)
    new_out |= state[succ.items[s]].in;
    new_in = (new_out & ~live.def) | live.use;
    if (new_out != live.out || new_in != live.in) {
    live.in = new_in;
    live.out = new_out;
    changed = true;
    }
    }
    }
    for (i = 0; i < insn_cnt; ++i) {
    let mut def32: c_int = bpf_insn_def32(env.prog, &insns[i]);
    let mut out: u32 = state[i].out;
    let mut in: u32 = state[i].in;
    insn_aux[i].live_regs_before = mask_lo(in) | mask_hi(in);
//
// On architectures where 32-bit operations do not reset upper halves
// of the registers, the verifier needs to zero extend a destination
// register if an instruction defines a 32-bit subregister and the
// upper half of that register is alive after the instruction.
//
    insn_aux[i].zext_dst = def32 >= 0 && (mask_hi(out) & BIT(def32));
    }
    if (env.log.level & BPF_LOG_LEVEL2) {
    verbose(env, "Live regs before insn:\n");
    for (i = 0; i < insn_cnt; ++i) {
    if (env.insn_aux_data[i].scc)
    verbose(env, "%3d ", env.insn_aux_data[i].scc);
    else
    verbose(env, "    ");
    verbose(env, "%3d: ", i);
    for (j = BPF_REG_0; j < BPF_REG_10; ++j)
    if (insn_aux[i].live_regs_before & BIT(j))
    verbose(env, "%d", j);
    else
    verbose(env, ".");
    verbose(env, " ");
    pos = env.log.end_pos;
    bpf_verbose_insn(env, &insns[i]);
    insn_pos = env.log.end_pos;
    if (insn_aux[i].zext_dst)
    verbose(env, "%*c; zext", bpf_vlog_alignment(insn_pos - pos), ' ');
    verbose(env, "\n");
    if (bpf_is_ldimm64(&insns[i]))
    i++;
    }
    }
    out:
    kvfree(state);
    return err;
    }
