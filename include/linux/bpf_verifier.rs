//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_verifier.h
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
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
//
pub const _LINUX_BPF_VERIFIER_H: c_int = 1;

// Maximum variable offset umax_value permitted when resolving memory accesses.
// In practice this is far bigger than any realistic pointer offset; this limit
// ensures that umax_value + (int)off + (int)size cannot overflow a u64.
//

// Maximum variable size permitted for ARG_MEM_SIZE[_OR_ZERO].  This ensures
// that converting umax_value to int cannot overflow.
//

// size of tmp_str_buf in bpf_verifier.
// we need at least 306 bytes to fit full stack mask representation
// (in the "-8,-16,...,-512" form)
//
pub const TMP_STR_BUF_LEN: c_int = 320;
// Patch buffer size
pub const INSN_BUF_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_iter_state {
    BPF_ITER_STATE_INVALID, /* for non-first slot */
    BPF_ITER_STATE_ACTIVE,
    BPF_ITER_STATE_DRAINED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_reg_state {
// Ordering of fields matters.  See states_equal()
    pub type: bpf_reg_type,
//
// Constant delta between "linked" scalars with the same ID.
//
    pub delta: i32,
// valid when type == PTR_TO_PACKET
    pub range: c_int,
// valid when type == CONST_PTR_TO_MAP | PTR_TO_MAP_VALUE |
// PTR_TO_MAP_VALUE_OR_NULL
//
    pub map_ptr: *mut bpf_map,
// To distinguish map lookups from outer map
// the map_uid is non-zero for registers
// pointing to inner maps.
//
    pub map_uid: u32,
}

// for PTR_TO_BTF_ID
// For dynptr stack slots
// A dynptr is 16 bytes so it takes up 2 stack slots.
// We need to track which slot is the first slot
// to protect against cases where the user may try to
// pass in an address starting at the second slot of the
// dynptr.
//
// For bpf_iter stack slots
// BTF container and BTF type ID describing
// struct bpf_iter_<type> of an iterator state
//
// packing following two fields to fit iter state into 16 bytes
// For irq stack slots
// Max size from any of the above.
// For scalar types (SCALAR_VALUE), this represents our knowledge of
// the actual value.
// For pointer types, this represents the variable part of the offset
// from the pointed-to object, and is shared with all bpf_reg_states
// with the same id as us.
//
// Used to determine if any memory access using this register will
// result in a bad access.
// These refer to the same value as var_off, not necessarily the actual
// contents of the register.
//
// For PTR_TO_PACKET, used to find other pointers with the same variable
// offset, so they can share range knowledge.
// For PTR_TO_MAP_VALUE_OR_NULL this is used to share which map value we
// came from, when one is tested for != NULL.
// For PTR_TO_MEM_OR_NULL this is used to identify memory allocation
// for the purpose of tracking that it's freed.
// For PTR_TO_SOCKET this is used to share which pointers retain the
// same reference to the socket, to determine proper reference freeing.
// For stack slots that are dynptrs, this is used to track references to
// the dynptr to determine proper reference freeing.
// Similarly to dynptrs, we use ID to track "belonging" of a reference
// to a specific instance of bpf_iter.
//
// Upper bit of ID is used to remember relationship between "linked"
// registers. Example:
// r1 = r2;    both will have r1->id == r2->id == N
// r1 += 10;   r1->id == N | BPF_ADD_CONST and r1->delta == 10
// r3 = r2;    both will have r3->id == r2->id == N
// w3 += 10;   r3->id == N | BPF_ADD_CONST32 and r3->delta == 10
//

//
// Tracks the parent object this register was derived from.
// Used for cascading invalidation: when the parent object is
// released or invalidated, all registers with matching parent_id
// are also invalidated. For example, a slice from bpf_dynptr_data()
// gets parent_id set to the dynptr's id.
//
// Inside the callee two registers can be both PTR_TO_STACK like
// R1=fp-8 and R2=fp-8, but one of them points to this function stack
// while another to the caller's stack. To differentiate them 'frameno'
// is used which is an index in bpf_verifier_state->frame[] array
// pointing to bpf_func_state.
//
// if (!precise && SCALAR_VALUE) min/max/tnum don't affect safety
extern "C" {
    pub fn cnum64_smin(_arg: reg->r64) -> return;
}
extern "C" {
    pub fn cnum64_smax(_arg: reg->r64) -> return;
}
extern "C" {
    pub fn cnum64_umin(_arg: reg->r64) -> return;
}
extern "C" {
    pub fn cnum64_umax(_arg: reg->r64) -> return;
}
extern "C" {
    pub fn cnum32_smin(_arg: reg->r32) -> return;
}
extern "C" {
    pub fn cnum32_smax(_arg: reg->r32) -> return;
}
extern "C" {
    pub fn cnum32_umin(_arg: reg->r32) -> return;
}
extern "C" {
    pub fn cnum32_umax(_arg: reg->r32) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_stack_slot_type {
    STACK_INVALID,    /* nothing was stored in this stack slot */
    STACK_SPILL,      /* register spilled into stack */
    STACK_MISC,	  /* BPF program wrote some data into this slot */
    STACK_ZERO,	  /* BPF program wrote constant zero */
// A dynptr is stored in this stack slot. The type of dynptr
// is stored in bpf_stack_state->spilled_ptr.dynptr.type
//
    STACK_DYNPTR,
    STACK_ITER,
    STACK_IRQ_FLAG,
    STACK_POISON,
}

// 4-byte stack slot granularity for liveness analysis
pub const BPF_HALF_REG_SIZE: c_int = 4;
pub const STACK_SLOT_SZ: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_state {
    pub spilled_ptr: bpf_reg_state,
    pub slot_type: [u8; BPF_REG_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_reference_state {
// Each reference object has a type. Ensure REF_TYPE_PTR is zero to
// default to pointer reference on zero initialization of a state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ref_state_type {
    REF_TYPE_PTR		= (1 << 1),
    REF_TYPE_IRQ		= (1 << 2),
    REF_TYPE_LOCK		= (1 << 3),
    REF_TYPE_RES_LOCK 	= (1 << 4),
    REF_TYPE_RES_LOCK_IRQ	= (1 << 5),
    REF_TYPE_LOCK_MASK	= REF_TYPE_LOCK | REF_TYPE_RES_LOCK | REF_TYPE_RES_LOCK_IRQ,
    } type;
// Track each reference created with a unique id, even if the same
// instruction creates the reference multiple times (eg, via CALL).
//
    int id;
// Instruction where the allocation of this reference occurred. This
// is used purely to inform the user of a reference leak.
//
    int insn_idx;
    union {
// For REF_TYPE_PTR
    int parent_id;
// Use to keep track of the source object of a lock, to ensure
// it matches on unlock.
//
    void *ptr;
}

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_retval_range {
    pub minval: i32,
    pub maxval: i32,
    pub return_32bit: bool,
}

// state of the program:
// type of all registers and stack info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_state {
    pub regs: [bpf_reg_state; MAX_BPF_REG],
// index of call instruction that called into this func
    pub callsite: c_int,
// stack frame number of this function state from pov of
// enclosing bpf_verifier_state.
// 0 = main function, 1 = first callee.
//
    pub frameno: u32,
//
// Unique diagnostic identity for this function invocation. Frame depth is
// reused after returns, while this ID is preserved across state clones.
//
    pub diag_frame_id: u32,
// subprog number == index within subprog_info
// zero == main subprog
//
    pub subprogno: u32,
// Every bpf_timer_start will increment async_entry_cnt.
// It's used to distinguish:
// void foo(void) { for(;;); }
// void foo(void) { bpf_timer_set_callback(,foo); }
//
    pub async_entry_cnt: u32,
    pub callback_ret_range: bpf_retval_range,
    pub in_callback_fn: bool,
    pub in_async_callback_fn: bool,
    pub in_exception_callback_fn: bool,
    pub no_stack_arg_load: bool,
// For callback calling functions that limit number of possible
// callback executions (e.g. bpf_loop) keeps track of current
// simulated iteration number.
// Value in frame N refers to number of times callback with frame
// N+1 was simulated, e.g. for the following call:
//
// bpf_loop(..., fn, ...); | suppose current frame is N
// | fn would be simulated in frame N+1
// | number of simulations is tracked in frame N
//
    pub callback_depth: u32,
// Instructions processed in this frame and callees on the current path.
    pub insns_subtotal: u32,
// The following fields should be last. See copy_func_state()
// The state of the stack. Each element of the array describes BPF_REG_SIZE
// (i.e. 8) bytes worth of stack memory.
// stack[0] represents bytes [*(r10-8)..*(r10-1)]
// stack[1] represents bytes [*(r10-16)..*(r10-9)]
// ...
// stack[allocated_stack/8 - 1] represents [*(r10-allocated_stack)..*(r10-allocated_stack+7)]
//
    pub stack: *mut bpf_stack_state,
// Size of the current stack, in bytes. The stack state is tracked below, in
// `stack`. allocated_stack is always a multiple of BPF_REG_SIZE.
//
    pub allocated_stack: c_int,
    pub /: *mut *mut u16 out_stack_arg_cnt; / Number of outgoing on-stack argument slots,
    pub /: *mut *mut *mut bpf_reg_state stack_arg_regs; / Outgoing on-stack arguments,
}

pub const MAX_CALL_FRAMES: c_int = 16;
// instruction history flags, used in bpf_jmp_history_entry.flags field.
// Frame number and SPI are stored in dedicated fields of bpf_jmp_history_entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_jmp_history_entry {
// insn idx can't be bigger than 1 million
    pub 20: u32 idx :,
    pub /: *mut *mut u32 frame : 4; / stack access frame number,
    pub /: *mut *mut u32 spi : 6; / stack slot index (0..63),
    pub 2: u32 :,
    pub 20: u32 prev_idx :,
// special INSN_F_xxx flags
    pub 4: u32 flags :,
    pub 8: u32 :,
//
// additional registers that need precision tracking when this
// jump is backtracked, vector of five 11-bit records
//
    pub linked_regs: u64,
}

// Maximum number of bpf_reg_state objects that can exist at once

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_verifier_state {
// call stack tracking
    pub frame: [*mut bpf_func_state; MAX_CALL_FRAMES],
    pub parent: *mut bpf_verifier_state,
// Acquired reference states
    pub refs: *mut bpf_reference_state,
//
// 'branches' field is the number of branches left to explore:
// 0 - all possible paths from this state reached bpf_exit or
// were safely pruned
// 1 - at least one path is being explored.
// This state hasn't reached bpf_exit
// 2 - at least two paths are being explored.
// This state is an immediate parent of two children.
// One is fallthrough branch with branches==1 and another
// state is pushed into stack (to be explored later) also with
// branches==1. The parent of this state has branches==1.
// The verifier state tree connected via 'parent' pointer looks like:
// 1
// 2 -> 1 (first 'if' pushed into stack)
// 1
// 2 -> 1 (second 'if' pushed into stack)
// 1
// 1 bpf_exit.
//
// Once do_check() reaches bpf_exit, it calls update_branch_counts()
// and the verifier state tree will look:
// 1
// 2 -> 1 (first 'if' pushed into stack)
// 1
// 1 -> 1 (second 'if' pushed into stack)
// 0
// 0 bpf_exit.
// After pop_stack() the do_check() will resume at second 'if'.
//
// If is_state_visited() sees a state with branches > 0 it means
// there is a loop. If such state is exactly equal to the current state
// it's an infinite loop. Note states_equal() checks for states
// equivalency, so two states being 'states_equal' does not mean
// infinite loop. The exact comparison is provided by
// states_maybe_looping() function. It's a stronger pre-check and
// much faster than states_equal().
//
// This algorithm may not find all possible infinite loops or
// loop iteration count may be too high.
// In such cases BPF_COMPLEXITY_LIMIT_INSNS limit kicks in.
//
    pub branches: u32,
    pub insn_idx: u32,
    pub curframe: u32,
    pub acquired_refs: u32,
    pub active_locks: u32,
    pub active_preempt_locks: u32,
    pub active_irq_id: u32,
    pub active_lock_id: u32,
    pub active_lock_ptr: *mut c_void,
    pub active_rcu_locks: u32,
    pub speculative: bool,
    pub in_sleepable: bool,
// first and last insn idx of this verifier state
    pub first_insn_idx: u32,
    pub last_insn_idx: u32,
// if this state is a backedge state then equal_state
// records cached state to which this state is equal.
//
    pub equal_state: *mut bpf_verifier_state,
// jmp history recorded from first to last.
// backtracking is using it to go from last to first.
// For most states jmp_history_cnt is [0-3].
// For loops can go up to ~40.
//
    pub jmp_history: *mut bpf_jmp_history_entry,
    pub jmp_history_cnt: u32,
    pub dfs_depth: u32,
    pub callback_unroll_depth: u32,
    pub may_goto_depth: u32,
}

// Iterate over 'frame', setting 'reg' to either NULL or a spilled register.

// Iterate over 'frame', setting 'reg' to either NULL or a spilled stack arg.

// Invoke __expr over regsiters in __vst, setting __state and __reg

// linked list of verifier states used to prune search
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_verifier_state_list {
    pub state: bpf_verifier_state,
    pub node: list_head,
    pub miss_cnt: u32,
    pub hit_cnt:31: u32,
    pub in_free_list:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_loop_inline_state {
    pub /: *mut *mut unsigned int initialized:1; / set to true upon first entry,
    pub same: *mut *mut unsigned int fit_for_inline:1; / true if callback function is the,
// at each call and flags are always zero
//
    pub /: *mut *mut u32 callback_subprogno; / valid when fit_for_inline is true,
}

// pointer and state for maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_ptr_state {
    pub map_ptr: *mut bpf_map,
    pub poison: bool,
    pub unpriv: bool,
}

// Possible states for alu_state member.

//
// An array of BPF instructions.
// Primary usage: return value of bpf_insn_successors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iarray {
    pub cnt: c_int,
    pub items: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn_aux_data {
    pub /: *mut *mut bpf_reg_type ptr_type; / pointer type for load/store insns,
    pub map_ptr_state: bpf_map_ptr_state,
    pub /: *mut *mut s32 call_imm; / saved imm field of call insn,
    pub /: *mut *mut u32 alu_limit; / limit for add/sub register with pointer,
    pub /: *mut *mut u32 map_index; / index into used_maps[],
    pub /: *mut *mut u32 map_off; / offset from value base address,
}

// if instruction is a call to bpf_loop this field tracks
// the state of the relevant registers to make decision about inlining
//
// remember the size of type passed to bpf_obj_new to rewrite R1
// remember the offset of node field within type to rewrite
// true if STX or LDX instruction is a part of a spill/fill
// pattern for a bpf_fastcall call.
//
// for CALL instructions, a number of spill/fill pairs in the
// bpf_fastcall pattern.
//
// below fields are initialized once
// ensure we check state equivalence and save state checkpoint and
// this instruction, regardless of any heuristics
//
// true if instruction is a call to a helper function that
// accepts callback function as a parameter.
//
// CFG strongly connected component this instruction belongs to,
// zero if it is a singleton SCC.
//
// registers alive before this instruction.
//
// Bitmask of R0-R9 that hold known values at this instruction.
// const_reg_mask: scalar constants that fit in 32 bits.
// const_reg_map_mask: map pointers, val is map_index into used_maps[].
// const_reg_subprog_mask: subprog pointers, val is subprog number.
// const_reg_vals[i] holds the 32-bit value for register i.
// Populated by compute_const_regs() pre-pass.
//

pub const BPF_VERIFIER_TMP_LOG_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_verifier_log {
// Logical start and end positions of a "log window" of the verifier log.
// start_pos == 0 means we haven't truncated anything.
// Once truncation starts to happen, start_pos + len_total == end_pos,
// except during log reset situations, in which (end_pos - start_pos)
// might get smaller than len_total (see bpf_vlog_reset()).
// Generally, (end_pos - start_pos) gives number of useful data in
// user log buffer.
//
    pub start_pos: u64,
    pub end_pos: u64,
    pub ubuf: *mut char __user,
    pub level: u32,
    pub len_total: u32,
    pub len_max: u32,
    pub kbuf: [c_char; BPF_VERIFIER_TMP_LOG_SIZE],
}

pub const BPF_LOG_LEVEL1: c_int = 1;
pub const BPF_LOG_LEVEL2: c_int = 2;
pub const BPF_LOG_STATS: c_int = 4;
pub const BPF_LOG_FIXED: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_log_attr {
    pub ubuf: *mut char __user,
    pub size: u32,
    pub level: u32,
    pub offsetof_true_size: u32,
    pub uattr: bpfptr_t,
}

extern "C" {
    pub fn bpf_log_attr_finalize(attr: *mut bpf_log_attr, log: *mut bpf_verifier_log) -> c_int;
}
pub const BPF_MAX_SUBPROGS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_subprog_arg_info {
    pub arg_type: bpf_arg_type,
    pub mem_size: u32,
    pub btf_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum priv_stack_mode {
    PRIV_STACK_UNKNOWN,
    NO_PRIV_STACK,
    PRIV_STACK_ADAPTIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_subprog_info {
    pub /: *const *const *const char name; / name extracted from BTF,
    pub /: *mut *mut u32 start; / insn idx of function entry point,
    pub /: *mut *mut u32 linfo_idx; / The idx to the main_prog->aux->linfo,
    pub /: *mut *mut u32 postorder_start; / The idx to the env->cfg.insn_postorder,
    pub /: *mut *mut u32 exit_idx; / Index of one of the BPF_EXIT instructions in this subprogram,
    pub /: *mut *mut u16 stack_depth; / max. stack depth used by this function,
    pub stack_extra: u16,
    pub insns_total: u32,
    pub insns_self: u32,
// offsets in range [stack_depth .. fastcall_stack_off)
// are used for bpf_fastcall spills and fills.
//
    pub fastcall_stack_off: i16,
    pub 1: bool has_tail_call:,
    pub 1: bool might_throw:,
    pub 1: bool tail_call_reachable:,
    pub 1: bool has_ld_abs:,
    pub 1: bool is_cb:,
    pub 1: bool is_async_cb:,
    pub 1: bool is_exception_cb:,
    pub 1: bool args_cached:,
// true if bpf_fastcall stack region is used by functions that can't be inlined
    pub 1: bool keep_fastcall_stack:,
    pub 1: bool changes_pkt_data:,
    pub 1: bool might_sleep:,
    pub arg_cnt:4: u8,
    pub priv_stack_mode: priv_stack_mode,
    pub args: [bpf_subprog_arg_info; MAX_BPF_FUNC_ARGS],
    pub /: *mut *mut u16 stack_arg_cnt; / incoming + max outgoing,
    pub max_out_stack_arg_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct backtrack_state {
    pub env: *mut bpf_verifier_env,
    pub frame: u32,
    pub reg_masks: [u32; MAX_CALL_FRAMES],
    pub stack_masks: [u64; MAX_CALL_FRAMES],
    pub stack_arg_masks: [u8; MAX_CALL_FRAMES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_id_pair {
    pub old: u32,
    pub cur: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_idmap {
    pub tmp_id_gen: u32,
    pub cnt: u32,
    pub map: [bpf_id_pair; BPF_ID_MAP_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_idset {
    pub num_ids: u32,
    pub id: u32,
    pub cnt: u32,
    pub entries: [}; BPF_ID_MAP_SIZE],
}

// see verifier.c:compute_scc_callchain()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_scc_callchain {
// call sites from bpf_verifier_state->frame[*]->callsite leading to this SCC
    pub 1]: u32 callsites[MAX_CALL_FRAMES -,
// last frame in a chain is identified by SCC id
    pub scc: u32,
}

// verifier state waiting for propagate_backedges()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_scc_backedge {
    pub next: *mut bpf_scc_backedge,
    pub state: bpf_verifier_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_scc_visit {
    pub callchain: bpf_scc_callchain,
// first state in current verification path that entered SCC
// identified by the callchain
//
    pub entry_state: *mut bpf_verifier_state,
    pub /: *mut *mut *mut bpf_scc_backedge backedges; / list of backedges,
    pub num_backedges: u32,
}

// An array of bpf_scc_visit structs sharing tht same bpf_scc_callchain->scc
// but having different bpf_scc_callchain->callsites.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_scc_info {
    pub num_visits: u32,
    pub visits: [bpf_scc_visit; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fd_array {
    pub map: *mut bpf_map,
    pub btf: *mut btf,
    pub val: c_ulong,
}

// single container for all structs
// one verifier_env per bpf_check() call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_verifier_env {
    pub insn_idx: u32,
    pub prev_insn_idx: u32,
    pub /: *mut *mut *mut bpf_prog prog; / eBPF program being verified,
    pub ops: *const bpf_verifier_ops,
    pub /: *mut *mut *mut module attach_btf_mod; / The owner module of prog->aux->attach_btf,
    pub /: *mut *mut *mut bpf_verifier_stack_elem head; / stack of verifier states to be processed,
    pub /: *mut *mut int stack_size; / number of states to be processed,
    pub /: *mut *mut bool strict_alignment; / perform strict pointer alignment checks,
    pub /: *mut *mut bool test_state_freq; / test verifier with different pruning frequency,
    pub /: *mut *mut bool test_reg_invariants; / fail verification on register invariants violations,
    pub /: *mut *mut *mut bpf_verifier_state cur_state; / current verifier state,
// Search pruning optimization, array of list_heads for
// lists of struct bpf_verifier_state_list.
//
    pub explored_states: *mut list_head,
    pub /: *mut *mut list_head free_list; / list of bpf_verifier_state_list,
    pub /: *mut *mut *mut bpf_map used_maps[MAX_USED_MAPS]; / array of map's used by eBPF program,
    pub /: *mut *mut btf_mod_pair used_btfs[MAX_USED_BTFS]; / array of BTF's used by BPF program,
    pub /: *mut *mut *mut bpf_map insn_array_maps[MAX_USED_MAPS]; / array of INSN_ARRAY map's to be relocated,
    pub /: *mut *mut u32 used_map_cnt; / number of used maps,
    pub /: *mut *mut u32 used_btf_cnt; / number of used BTF objects,
    pub /: *mut *mut u32 insn_array_map_cnt; / number of used maps of type BPF_MAP_TYPE_INSN_ARRAY,
    pub /: *mut *mut u32 id_gen; / used to generate unique reg IDs,
    pub /: *mut *mut u32 hidden_subprog_cnt; / number of hidden subprogs,
    pub exception_callback_subprog: c_int,
    pub explore_alu_limits: bool,
    pub allow_ptr_leaks: bool,
// Allow access to uninitialized stack memory. Writes with fixed offset are
// always allowed, so this refers to reads (with fixed or variable offset),
// to writes with variable offset and to indirect (helper) accesses.
//
    pub allow_uninit_stack: bool,
    pub bpf_capable: bool,
    pub bypass_spec_v1: bool,
    pub bypass_spec_v4: bool,
    pub seen_direct_write: bool,
    pub seen_exception: bool,
    pub signature: bool,
    pub insn_aux_data_len: u32,
    pub /: *mut *mut *mut bpf_insn_aux_data insn_aux_data; / array of per-insn state,
    pub prev_linfo: *const bpf_line_info,
    pub log: bpf_verifier_log,
    pub diag: *mut bpf_diag,
    pub /: *mut *mut bpf_subprog_info subprog_info[BPF_MAX_SUBPROGS + 2]; / max + 2 for the fake and exception subprogs,
// subprog indices sorted in topological order: leaves first, callers last
    pub 2]: int subprog_topo_order[BPF_MAX_SUBPROGS +,
    pub idmap_scratch: bpf_idmap,
    pub idset_scratch: bpf_idset,
}

//
// vector of instruction indexes sorted in post-order, grouped by subprogram,
// see bpf_subprog_info->postorder_start.
//
// current position in the insn_postorder vector
// Per-callsite copy of parent's converged at_stack_in for cross-frame fills.
// number of instructions analyzed by the verifier
// number of jmps, calls, exits analyzed so far
// maximum combined stack depth
// total verification time
// maximum number of verifier states kept in 'branching' instructions
// total number of allocated verifier states
// some states are freed during program analysis.
// this is peak number of states. this number dominates kernel
// memory consumption during verification
//
// longest register parentage chain walked for liveness marking
//
// The program's fd_array comes in two shapes, told apart by whether
// the caller passed fd_array_cnt. They are mutually exclusive:
// - continuous (fd_array_cnt given): ->fd_array holds every entry
// resolved to its object up front, indexed by fd_array position,
// with ->fd_array_cnt slots; ->fd_array_raw is unused.
// - sparse (no fd_array_cnt): ->fd_array is NULL, and entries are
// read from ->fd_array_raw (the caller's fd_array) and resolved
// on the spot at each reference.
//
// bit mask to keep track of whether a register has been accessed
// since the last time the function state was printed
//
// Same as scratched_regs but for stack slots
// buffer used to temporary hold constants as scalar registers
// buffers used to save updated reg states while simulating branches
// buffer used to generate temporary string representations,
// e.g., in reg_type_str() to generate reg_type string
//
// array of pointers to bpf_scc_info indexed by SCC id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_call_summary {
    pub num_params: u8,
    pub is_void: bool,
    pub fastcall: bool,
}

extern "C" {
    pub fn bpf_vlog_reset(log: *mut bpf_verifier_log, new_pos: u64);
}
extern "C" {
    pub fn bpf_vlog_finalize(log: *mut bpf_verifier_log, log_size_actual: *mut u32) -> c_int;
}

extern "C" {
    pub fn bpf_prog_offload_verifier_prep(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_prog_offload_finalize(env: *mut bpf_verifier_env) -> c_int;
}
// this lives here instead of in bpf.h because it needs to dereference tgt_prog
// unpack the IDs from the key as constructed above
// obj_id = key >> 32;
// btf_id = key & 0x7FFFFFFF;
extern "C" {
    pub fn bpf_free_kfunc_btf_tab(tab: *mut bpf_kfunc_btf_tab);
}
extern "C" {
    pub fn mark_chain_precision(env: *mut bpf_verifier_env, regno: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_is_state_visited(env: *mut bpf_verifier_env, insn_idx: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_update_branch_counts(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int;
}
extern "C" {
    pub fn bpf_clear_jmp_history(state: *mut bpf_verifier_state);
}
extern "C" {
    pub fn bpf_free_verifier_state(state: *mut bpf_verifier_state, free_self: bool);
}
extern "C" {
    pub fn bpf_free_backedges(visit: *mut bpf_scc_visit);
}
extern "C" {
    pub fn bpf_bt_sync_linked_regs(bt: *mut backtrack_state, hist: *mut bpf_jmp_history_entry);
}
extern "C" {
    pub fn bpf_mark_reg_unknown_imprecise(reg: *mut bpf_reg_state);
}
extern "C" {
    pub fn bpf_clear_singular_ids(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state);
}
// Return IP for a given frame in a call stack
extern "C" {
    pub fn bpf_map_is_rdonly(map: *const bpf_map) -> bool;
}

// extract base type from bpf_{arg, return, reg}_type.
// extract flags from an extended type. See bpf_type_flag in bpf.h.
//
// The pointer types which must not be dereferenced without fault
// protection, that is, the ones bpf_convert_ctx_accesses() has to
// turn a BPF_LDX into a BPF_PROBE_MEM one for.
//

// Used for printing the entire verifier state.

extern "C" {
    pub fn bpf_vlog_alignment(pos: u32) -> u32;
}
extern "C" {
    pub fn bpf_jmp_offset(insn: *mut bpf_insn) -> c_int;
}
extern "C" {
    pub fn bpf_fmt_stack_mask(buf: *mut c_char, buf_sz: isize, stack_mask: u64);
}
extern "C" {
    pub fn bpf_subprog_is_global(env: *const bpf_verifier_env, subprog: c_int) -> bool;
}
extern "C" {
    pub fn bpf_find_subprog(env: *mut bpf_verifier_env, off: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_is_throw_kfunc(insn: *mut bpf_insn) -> bool;
}
extern "C" {
    pub fn bpf_compute_const_regs(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_prune_dead_branches(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_check_cfg(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_compute_postorder(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_compute_scc(env: *mut bpf_verifier_env) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_desc {
    pub ptr: *mut bpf_map,
    pub uid: c_int,
}

// The last initialized dynptr; Populated by process_dynptr_func()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dynptr_desc {
    pub type: bpf_dynptr_type,
    pub id: u32,
    pub parent_id: u32,
}

//
// The last seen rereferenced object; Updated by update_ref_obj() when a register refers to a
// referenced object. Used when the helper or kfunc is casting a referenced object, returning
// allocated memory derived from referenced object or creating a dynptr with a referenced
// object as parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_obj_desc {
    pub id: u32,
    pub parent_id: u32,
    pub cnt: u8,
}

//
// A memory argument a call fills in. The verifier allows the stack to be uninitialized if
// the range is a known constant. Stack slots are marked as STACK_MISC by check_mem_access().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arg_raw_mem_desc {
    pub regno: u8,
    pub size: c_int,
}

// Size of PTR_TO_MEM returned, taken from a constant allocation-size argument
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ret_mem_desc {
    pub size: u32,
    pub found: bool,
}

// A constant scalar argument; Populated by process_const_arg()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arg_constant_desc {
    pub value: u64,
    pub found: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_call_arg_meta {
// Common
    pub btf: *mut btf,
    pub func_id: u32,
    pub fn: *const bpf_func_proto,
    pub release_regno: u8,
    pub ret_btf_id: u32,
    pub subprogno: u32,
    pub map: bpf_map_desc,
    pub dynptr: bpf_dynptr_desc,
    pub ref_obj: ref_obj_desc,
    pub ret_mem: ret_mem_desc,
// Only set by kfunc
    pub r0_rdonly: bool,
    pub kfunc_flags: u32,
    pub func_proto: *const btf_type,
    pub func_name: *const c_char,
    pub arg_constant: arg_constant_desc,
// arg_{btf,btf_id,owning_ref} are used by kfunc-specific handling,
// generally to pass info about user-defined local kptr types to later
// verification logic
// bpf_obj_drop/bpf_percpu_obj_drop
// Record the local kptr type to be drop'd
// bpf_refcount_acquire (via KF_ARG_PTR_TO_REFCOUNTED_KPTR arg type)
// Record the local kptr type to be refcount_incr'd and use
// arg_owning_ref to determine whether refcount_acquire should be
// fallible
//
    pub arg_btf: *mut btf,
    pub arg_btf_id: u32,
    pub arg_owning_ref: bool,
    pub arg_prog: bool,
    pub field: *mut btf_field,
    pub arg_list_head: },
    pub field: *mut btf_field,
    pub arg_rbtree_root: },
    pub spi: u8,
    pub frameno: u8,
    pub iter: },
// Only set by helper
    pub msize_max_value: u64,
    pub const_map_key: i64,
    pub ret_btf: *mut btf,
    pub kptr_field: *mut btf_field,
    pub arg_raw_mem: arg_raw_mem_desc,
}

extern "C" {
    pub fn bpf_is_async_callback_calling_insn(insn: *mut bpf_insn) -> bool;
}
extern "C" {
    pub fn bpf_is_sync_callback_calling_insn(insn: *mut bpf_insn) -> bool;
}
extern "C" {
    pub fn bpf_is_kfunc_pkt_changing(meta: *mut bpf_call_arg_meta) -> bool;
}
extern "C" {
    pub fn bpf_copy_insn_array_uniq(map: *mut bpf_map, start: u32, end: u32, off: *mut u32) -> c_int;
}
extern "C" {
    pub fn bpf_insn_is_cond_jump(code: u8) -> bool;
}
extern "C" {
    pub fn bpf_is_may_goto_insn(insn: *mut bpf_insn) -> bool;
}
extern "C" {
    pub fn bpf_verbose_insn(env: *mut bpf_verifier_env, insn: *mut bpf_insn);
}
extern "C" {
    pub fn bpf_compute_subprog_arg_access(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_stack_liveness_init(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_stack_liveness_free(env: *mut bpf_verifier_env);
}
extern "C" {
    pub fn bpf_live_stack_query_init(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> c_int;
}
extern "C" {
    pub fn bpf_stack_slot_alive(env: *mut bpf_verifier_env, frameno: u32, spi: u32) -> bool;
}
extern "C" {
    pub fn bpf_compute_live_registers(env: *mut bpf_verifier_env) -> c_int;
}

pub const MAX_PACKET_OFF: c_uint = 0xffff;
pub const CALLER_SAVED_REGS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_reg_arg_type {
    SRC_OP,		/* register is used as source operand */
    DST_OP,		/* register is used as destination operand */
    DST_OP_NO_MARK	/* same as above, check only, don't mark */
}

pub const MAX_KFUNC_DESCS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_kfunc_desc {
    pub func_model: btf_func_model,
    pub proto: bpf_func_proto,
    pub func_id: u32,
    pub imm: i32,
    pub offset: u16,
    pub addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_kfunc_desc_tab {
    pub nr_descs: u32,
// Sorted by func_id (BTF ID) and offset (fd_array offset) during
// verification. JITs do lookups by bpf_insn, where func_id may not be
// available, therefore at the end of verification do_misc_fixups()
// sorts this by imm and offset.
//
// Grown one entry at a time by bpf_add_kfunc_call().
//
    pub descs: [bpf_kfunc_desc; ],
}

// Functions exported from verifier.c, used by fixups.c
extern "C" {
    pub fn bpf_clear_insn_aux_data(env: *mut bpf_verifier_env, start: c_int, len: c_int);
}
extern "C" {
    pub fn bpf_mark_subprog_exc_cb(env: *mut bpf_verifier_env, subprog: c_int);
}
extern "C" {
    pub fn bpf_allow_tail_call_in_subprogs(env: *mut bpf_verifier_env) -> bool;
}
extern "C" {
    pub fn bpf_verifier_inlines_helper_call(env: *mut bpf_verifier_env, imm: i32) -> bool;
}
extern "C" {
    pub fn bpf_add_kfunc_call(env: *mut bpf_verifier_env, func_id: u32, offset: u16) -> c_int;
}
// Functions exported from verifier.c, used by trampoline.c
// Functions in fixups.c, called from bpf_check()
extern "C" {
    pub fn bpf_remove_fastcall_spills_fills(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_optimize_bpf_loop(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_opt_hard_wire_dead_code_branches(env: *mut bpf_verifier_env);
}
extern "C" {
    pub fn bpf_opt_remove_dead_code(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_opt_remove_nops(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_opt_subreg_zext_lo32_rnd_hi32(env: *mut bpf_verifier_env, attr: *const bpf_attr) -> c_int;
}
extern "C" {
    pub fn bpf_convert_ctx_accesses(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_jit_subprogs(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_fixup_call_args(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_do_misc_fixups(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_insn_def32(prog: *mut bpf_prog, insn: *mut bpf_insn) -> c_int;
}
