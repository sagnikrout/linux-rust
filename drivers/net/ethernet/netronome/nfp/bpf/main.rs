//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/bpf/main.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2016-2018 Netronome Systems, Inc.
pub const __NFP_BPF_H__: c_int = 1;

// For relocation logic use up-most byte of branch instruction as scratch
// area.  Remember to clear this before sending instructions to HW!
//
pub const OP_RELO_TYPE: c_uint = 0xff00000000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_relo_type {
    RELO_NONE = 0,
// standard internal jumps
    RELO_BR_REL,
// internal jumps to parts of the outro
    RELO_BR_GO_OUT,
    RELO_BR_GO_ABORT,
    RELO_BR_GO_CALL_PUSH_REGS,
    RELO_BR_GO_CALL_POP_REGS,
// external jumps to fixed addresses
    RELO_BR_NEXT_PKT,
    RELO_BR_HELPER,
// immediate relocation against load address
    RELO_IMMED_REL,
}

// To make absolute relocated branches (branches other than RELO_BR_REL)
// distinguishable in user space dumps from normal jumps, add a large offset
// to them.
//
pub const BR_OFF_RELO: c_int = 15000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum static_regs {
    STATIC_REG_IMMA		= 20, /* Bank AB */
    STATIC_REG_IMM		= 21, /* Bank AB */
    STATIC_REG_STACK	= 22, /* Bank A */
    STATIC_REG_PKT_LEN	= 22, /* Bank B */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pkt_vec {
    PKT_VEC_PKT_LEN		= 0,
    PKT_VEC_PKT_PTR		= 2,
    PKT_VEC_QSEL_SET	= 4,
    PKT_VEC_QSEL_VAL	= 6,
}

pub const PKT_VEL_QSEL_SET_BIT: c_int = 4;

pub const NFP_BPF_ABI_FLAG_MARK: c_int = 1;
//
// struct nfp_app_bpf - bpf app priv structure
// @app:		backpointer to the app
// @ccm:		common control message handler data
//
// @bpf_dev:		BPF offload device handle
//
// @cmsg_key_sz:	size of key in cmsg element array
// @cmsg_val_sz:	size of value in cmsg element array
//
// @map_list:		list of offloaded maps
// @maps_in_use:	number of currently offloaded maps
// @map_elems_in_use:	number of elements allocated to offloaded maps
//
// @maps_neutral:	hash table of offload-neutral maps (on pointer)
//
// @abi_version:	global BPF ABI version
// @cmsg_cache_cnt:	number of entries to read for caching
//
// @adjust_head:	adjust head capability
// @adjust_head.flags:		extra flags for adjust head
// @adjust_head.off_min:	minimal packet offset within buffer required
// @adjust_head.off_max:	maximum packet offset within buffer required
// @adjust_head.guaranteed_sub:	negative adjustment guaranteed possible
// @adjust_head.guaranteed_add:	positive adjustment guaranteed possible
//
// @maps:		map capability
// @maps.types:			supported map types
// @maps.max_maps:		max number of maps supported
// @maps.max_elems:		max number of entries in each map
// @maps.max_key_sz:		max size of map key
// @maps.max_val_sz:		max size of map value
// @maps.max_elem_sz:		max size of map entry (key + value)
//
// @helpers:		helper addressess for various calls
// @helpers.map_lookup:		map lookup helper address
// @helpers.map_update:		map update helper address
// @helpers.map_delete:		map delete helper address
// @helpers.perf_event_output:	output perf event to a ring buffer
//
// @pseudo_random:	FW initialized the pseudo-random machinery (CSRs)
// @queue_select:	BPF can set the RX queue ID in packet vector
// @adjust_tail:	BPF can simply trunc packet size for adjust tail
// @cmsg_multi_ent:	FW can pack multiple map entries in a single cmsg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_app_bpf {
    pub app: *mut nfp_app,
    pub ccm: nfp_ccm,
    pub bpf_dev: *mut bpf_offload_dev,
    pub cmsg_key_sz: c_uint,
    pub cmsg_val_sz: c_uint,
    pub cmsg_cache_cnt: c_uint,
    pub map_list: list_head,
    pub maps_in_use: c_uint,
    pub map_elems_in_use: c_uint,
    pub maps_neutral: rhashtable,
    pub abi_version: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_cap_adjust_head {
    pub flags: u32,
    pub off_min: c_int,
    pub off_max: c_int,
    pub guaranteed_sub: c_int,
    pub guaranteed_add: c_int,
    pub adjust_head: },
    pub types: u32,
    pub max_maps: u32,
    pub max_elems: u32,
    pub max_key_sz: u32,
    pub max_val_sz: u32,
    pub max_elem_sz: u32,
    pub maps: },
    pub map_lookup: u32,
    pub map_update: u32,
    pub map_delete: u32,
    pub perf_event_output: u32,
    pub helpers: },
    pub pseudo_random: bool,
    pub queue_select: bool,
    pub adjust_tail: bool,
    pub cmsg_multi_ent: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_bpf_map_use {
    NFP_MAP_UNUSED = 0,
    NFP_MAP_USE_READ,
    NFP_MAP_USE_WRITE,
    NFP_MAP_USE_ATOMIC_CNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_map_word {
    pub :4: unsigned char type,
    pub :1: unsigned char non_zero_update,
}

//
// struct nfp_bpf_map - private per-map data attached to BPF maps for offload
// @offmap:	pointer to the offloaded BPF map
// @bpf:	back pointer to bpf app private structure
// @tid:	table id identifying map on datapath
//
// @cache_lock:	protects @cache_blockers, @cache_to, @cache
// @cache_blockers:	number of ops in flight which block caching
// @cache_gen:	counter incremented by every blocker on exit
// @cache_to:	time when cache will no longer be valid (ns)
// @cache:	skb with cached response
//
// @l:		link on the nfp_app_bpf->map_list list
// @use_map:	map of how the value is used (in 4B chunks)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_map {
    pub offmap: *mut bpf_offloaded_map,
    pub bpf: *mut nfp_app_bpf,
    pub tid: u32,
    pub cache_lock: spinlock_t,
    pub cache_blockers: u32,
    pub cache_gen: u32,
    pub cache_to: u64,
    pub cache: *mut sk_buff,
    pub l: list_head,
    pub use_map: [nfp_bpf_map_word; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_neutral_map {
    pub l: rhash_head,
    pub ptr: *mut bpf_map,
    pub map_id: u32,
    pub count: u32,
}

extern "C" {
    pub fn int(: *mut *mut instr_cb_t)(struct nfp_prog, : *mut nfp_insn_meta) -> typedef;
}

//
// struct nfp_bpf_reg_state - register state for calls
// @reg: BPF register state from latest path
// @var_off: for stack arg - changes stack offset on different paths
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_reg_state {
    pub reg: bpf_reg_state,
    pub var_off: bool,
}

// Instruction is pointless, noop even on its own

// Instruction is optimized out based on preceding instructions

// Instruction is optimized by the verifier

// Instruction needs to zero extend to high 32-bit

//
// struct nfp_insn_meta - BPF instruction wrapper
// @insn: BPF instruction
// @ptr: pointer type for memory operations
// @ldst_gather_len: memcpy length gathered from load/store sequence
// @paired_st: the paired store insn at the head of the sequence
// @ptr_not_const: pointer is not always constant
// @pkt_cache: packet data cache information
// @pkt_cache.range_start: start offset for associated packet data cache
// @pkt_cache.range_end: end offset for associated packet data cache
// @pkt_cache.do_init: this read needs to initialize packet data cache
// @xadd_over_16bit: 16bit immediate is not guaranteed
// @xadd_maybe_16bit: 16bit immediate is possible
// @jmp_dst: destination info for jump instructions
// @jump_neg_op: jump instruction has inverted immediate, use ADD instead of SUB
// @num_insns_after_br: number of insns following a branch jump, used for fixup
// @func_id: function id for call instructions
// @arg1: arg1 for call instructions
// @arg2: arg2 for call instructions
// @umin_src: copy of core verifier umin_value for src opearnd.
// @umax_src: copy of core verifier umax_value for src operand.
// @umin_dst: copy of core verifier umin_value for dst opearnd.
// @umax_dst: copy of core verifier umax_value for dst operand.
// @off: index of first generated machine instruction (in nfp_prog.prog)
// @n: eBPF instruction number
// @flags: eBPF instruction extra optimization flags
// @subprog_idx: index of subprogram to which the instruction belongs
// @double_cb: callback for second part of the instruction
// @l: link on nfp_prog->insns list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_insn_meta {
    pub insn: bpf_insn,
// pointer ops (ld/st/xadd)
    pub ptr: bpf_reg_state,
    pub paired_st: *mut bpf_insn,
    pub ldst_gather_len: i16,
    pub ptr_not_const: bool,
    pub range_start: i16,
    pub range_end: i16,
    pub do_init: bool,
    pub pkt_cache: },
    pub xadd_over_16bit: bool,
    pub xadd_maybe_16bit: bool,
}

// jump
// function calls
// We are interested in range info for operands of ALU
// operations. For example, shift amount, multiplicand and
// multiplier etc.
//
pub const BPF_SIZE_MASK: c_uint = 0x18;
extern "C" {
    pub fn BPF_CLASS(_arg: meta->insn.code) -> return;
}
extern "C" {
    pub fn BPF_SRC(_arg: meta->insn.code) -> return;
}
extern "C" {
    pub fn BPF_OP(_arg: meta->insn.code) -> return;
}
extern "C" {
    pub fn BPF_MODE(_arg: meta->insn.code) -> return;
}
extern "C" {
    pub fn is_mbpf_jmp32(is_mbpf_jmp64(meta: meta) ||) -> return;
}
pub const STACK_FRAME_ALIGN: c_int = 64;
//
// struct nfp_bpf_subprog_info - nfp BPF sub-program (a.k.a. function) info
// @stack_depth:	maximum stack depth used by this sub-program
// @needs_reg_push:	whether sub-program uses callee-saved registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_subprog_info {
    pub stack_depth: u16,
    pub 1: u8 needs_reg_push :,
}

//
// struct nfp_prog - nfp BPF program
// @bpf: backpointer to the bpf app priv structure
// @prog: machine code
// @prog_len: number of valid instructions in @prog array
// @__prog_alloc_len: alloc size of @prog array
// @stack_size: total amount of stack used
// @verifier_meta: temporary storage for verifier's insn meta
// @type: BPF program type
// @last_bpf_off: address of the last instruction translated from BPF
// @tgt_out: jump target for normal exit
// @tgt_abort: jump target for abort (e.g. access outside of packet buffer)
// @tgt_call_push_regs: jump target for subroutine for saving R6~R9 to stack
// @tgt_call_pop_regs: jump target for subroutine used for restoring R6~R9
// @n_translated: number of successfully translated instructions (for errors)
// @error: error code if something went wrong
// @stack_frame_depth: max stack depth for current frame
// @adjust_head_location: if program has single adjust head call - the insn no.
// @map_records_cnt: the number of map pointers recorded for this prog
// @subprog_cnt: number of sub-programs, including main function
// @map_records: the map record pointers from bpf->maps_neutral
// @subprog: pointer to an array of objects holding info about sub-programs
// @n_insns: number of instructions on @insns list
// @insns: list of BPF instruction wrappers (struct nfp_insn_meta)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_prog {
    pub bpf: *mut nfp_app_bpf,
    pub prog: *mut u64,
    pub prog_len: c_uint,
    pub __prog_alloc_len: c_uint,
    pub stack_size: c_uint,
    pub verifier_meta: *mut nfp_insn_meta,
    pub type: bpf_prog_type,
    pub last_bpf_off: c_uint,
    pub tgt_out: c_uint,
    pub tgt_abort: c_uint,
    pub tgt_call_push_regs: c_uint,
    pub tgt_call_pop_regs: c_uint,
    pub n_translated: c_uint,
    pub error: c_int,
    pub stack_frame_depth: c_uint,
    pub adjust_head_location: c_uint,
    pub map_records_cnt: c_uint,
    pub subprog_cnt: c_uint,
    pub map_records: *mut nfp_bpf_neutral_map,
    pub subprog: *mut nfp_bpf_subprog_info,
    pub n_insns: c_uint,
    pub insns: list_head,
}

//
// struct nfp_bpf_vnic - per-vNIC BPF priv structure
// @tc_prog:	currently loaded cls_bpf program
// @start_off:	address of the first instruction in the memory
// @tgt_done:	jump target to get the next packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_vnic {
    pub tc_prog: *mut bpf_prog,
    pub start_off: c_uint,
    pub tgt_done: c_uint,
}

extern "C" {
    pub fn nfp_is_subprog_start(meta: *mut nfp_insn_meta) -> bool;
}
extern "C" {
    pub fn nfp_bpf_jit_prepare(nfp_prog: *mut nfp_prog);
}
extern "C" {
    pub fn nfp_bpf_jit(prog: *mut nfp_prog) -> c_int;
}
extern "C" {
    pub fn nfp_bpf_supported_opcode(code: u8) -> bool;
}
extern "C" {
    pub fn nfp_bpf_finalize(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn nfp_bpf_opt_remove_insns(env: *mut bpf_verifier_env, off: u32, cnt: u32) -> c_int;
}
extern "C" {
    pub fn nfp_bpf_ctrl_cmsg_min_mtu(bpf: *mut nfp_app_bpf) -> c_uint;
}
extern "C" {
    pub fn nfp_bpf_ctrl_cmsg_mtu(bpf: *mut nfp_app_bpf) -> c_uint;
}
extern "C" {
    pub fn nfp_bpf_ctrl_cmsg_cache_cnt(bpf: *mut nfp_app_bpf) -> c_uint;
}
extern "C" {
    pub fn nfp_bpf_ctrl_del_entry(offmap: *mut bpf_offloaded_map, key: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nfp_bpf_ctrl_msg_rx(app: *mut nfp_app, skb: *mut sk_buff);
}
