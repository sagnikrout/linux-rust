//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/x_tables.h
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

// Test a struct->invflags and a boolean for inequality

//
// struct xt_action_param - parameters for matches/targets
//
// @match:	the match extension
// @target:	the target extension
// @matchinfo:	per-match data
// @targinfo:	per-target data
// @state:	pointer to hook state this packet came from
// @fragoff:	packet is a fragment, this is the data offset
// @thoff:	position of transport header relative to skb->data
//
// Fields written to by extensions:
//
// @hotdrop:	drop packet if we had inspection problems
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_action_param {
    pub match: *const xt_match,
    pub target: *const xt_target,
}

//
// struct xt_mtchk_param - parameters for match extensions'
// checkentry functions
//
// @net:	network namespace through which the check was invoked
// @table:	table the rule is tried to be inserted into
// @entryinfo:	the family-specific rule data
// (struct ipt_ip, ip6t_ip, arpt_arp or (note) ebt_entry)
// @match:	struct xt_match through which this function was invoked
// @matchinfo:	per-match data
// @hook_mask:	via which hooks the new rule is reachable
// @family:	actual NFPROTO_* through which the function is invoked
// (helpful when match->family == NFPROTO_UNSPEC)
// @nft_compat:	running from the nft compat layer if true
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_mtchk_param {
    pub net: *mut net,
    pub table: *const c_char,
    pub entryinfo: *const c_void,
    pub match: *const xt_match,
    pub matchinfo: *mut c_void,
    pub hook_mask: c_uint,
    pub family: u_int8_t,
    pub nft_compat: bool,
}

//
// struct xt_mtdtor_param - match destructor parameters
//
// @net:	network namespace through which the check was invoked
// @match:	struct xt_match through which this function was invoked
// @matchinfo:	per-match data
// @family:	actual NFPROTO_* through which the function is invoked
// (helpful when match->family == NFPROTO_UNSPEC)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_mtdtor_param {
    pub net: *mut net,
    pub match: *const xt_match,
    pub matchinfo: *mut c_void,
    pub family: u_int8_t,
}

//
// struct xt_tgchk_param - parameters for target extensions'
// checkentry functions
//
// @net:	network namespace through which the check was invoked
// @table:	table the rule is tried to be inserted into
// @entryinfo:	the family-specific rule data
// (struct ipt_entry, ip6t_entry, arpt_entry, ebt_entry)
// @target:	the target extension
// @targinfo:	per-target data
// @hook_mask:	via which hooks the new rule is reachable
// @family:	actual NFPROTO_* through which the function is invoked
// (helpful when match->family == NFPROTO_UNSPEC)
// @nft_compat:	running from the nft compat layer if true
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tgchk_param {
    pub net: *mut net,
    pub table: *const c_char,
    pub entryinfo: *const c_void,
    pub target: *const xt_target,
    pub targinfo: *mut c_void,
    pub hook_mask: c_uint,
    pub family: u_int8_t,
    pub nft_compat: bool,
}

// Target destructor parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tgdtor_param {
    pub net: *mut net,
    pub target: *const xt_target,
    pub targinfo: *mut c_void,
    pub family: u_int8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_match {
    pub list: list_head,
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u_int8_t,
// Return true or false: return FALSE and set *hotdrop = 1 to
// Arguments changed since 2.6.9, as this must now handle
    pub ): *mut xt_action_param,
// Called when user tries to insert an entry of this type.
    pub ): *const *const int (checkentry)(struct xt_mtchk_param,
// Called to validate hooks based on the match configuration.
    pub ): *const *const int (check_hooks)(struct xt_mtchk_param,
// Called when entry of this type deleted.
    pub ): *const *const void (destroy)(struct xt_mtdtor_param,

// Called when userspace align differs from kernel space one
    pub src): *const *const *const void (compat_from_user)(void dst, void,
    pub src): *const *const *const int (compat_to_user)(void __user dst, void,

// Set this to THIS_MODULE if you are a module, otherwise NULL
    pub me: *mut module,
    pub table: *const c_char,
    pub matchsize: c_uint,
    pub usersize: c_uint,

    pub compatsize: c_uint,

    pub hooks: c_uint,
    pub proto: c_ushort,
    pub family: c_ushort,
}

// Registration hooks for targets.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_target {
    pub list: list_head,
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u_int8_t,
// Returns verdict. Argument order changed since 2.6.9, as this
    pub ): *const xt_action_param,
// Called when user tries to insert an entry of this type:
// Should return 0 on success or an error code otherwise (-Exxxx).
    pub ): *const *const int (checkentry)(struct xt_tgchk_param,
// Called to validate hooks based on the target configuration.
    pub ): *const *const int (check_hooks)(struct xt_tgchk_param,
// Called when entry of this type deleted.
    pub ): *const *const void (destroy)(struct xt_tgdtor_param,

// Called when userspace align differs from kernel space one
    pub src): *const *const *const void (compat_from_user)(void dst, void,
    pub src): *const *const *const int (compat_to_user)(void __user dst, void,

// Set this to THIS_MODULE if you are a module, otherwise NULL
    pub me: *mut module,
    pub table: *const c_char,
    pub targetsize: c_uint,
    pub usersize: c_uint,

    pub compatsize: c_uint,

    pub hooks: c_uint,
    pub proto: c_ushort,
    pub family: c_ushort,
}

// Furniture shopping...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_table {
    pub list: list_head,
// What hooks you will enter on
    pub valid_hooks: c_uint,
// Man behind the curtain...
    pub private: *mut xt_table_info,
// hook ops that register the table with the netfilter core
    pub ops: *mut nf_hook_ops,
// Set this to THIS_MODULE if you are a module, otherwise NULL
    pub me: *mut module,
    pub /: *mut *mut u_int8_t af; / address/protocol family,
    pub /: *mut *mut int priority; / hook order,
// A unique name...
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
}

// The table itself
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_table_info {
// Size per table
    pub size: c_uint,
// Number of entries: FIXME. --RR
    pub number: c_uint,
// Initial number of entries. Needed for module usage count
    pub initial_entries: c_uint,
// Entry points and underflows
    pub hook_entry: [c_uint; NF_INET_NUMHOOKS],
    pub underflow: [c_uint; NF_INET_NUMHOOKS],
//
// Number of user chains. Since tables cannot have loops, at most
// @stacksize jumps (number of user chains) can possibly be made.
//
    pub stacksize: c_uint,
    pub jumpstack: *mut c_void,
    pub __aligned(8): unsigned char entries[],
}

extern "C" {
    pub fn xt_register_target(target: *mut xt_target) -> c_int;
}
extern "C" {
    pub fn xt_unregister_target(target: *mut xt_target);
}
extern "C" {
    pub fn xt_register_targets(target: *mut xt_target, n: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_unregister_targets(target: *mut xt_target, n: c_uint);
}
extern "C" {
    pub fn xt_register_match(target: *mut xt_match) -> c_int;
}
extern "C" {
    pub fn xt_unregister_match(target: *mut xt_match);
}
extern "C" {
    pub fn xt_register_matches(match: *mut xt_match, n: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_unregister_matches(match: *mut xt_match, n: c_uint);
}
extern "C" {
    pub fn xt_check_table_hooks(info: *const xt_table_info, valid_hooks: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_check_proc_name(name: *const c_char, size: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_check_hooks_match(par: *mut xt_mtchk_param) -> c_int;
}
extern "C" {
    pub fn xt_check_hooks_target(par: *mut xt_tgchk_param) -> c_int;
}
extern "C" {
    pub fn xt_unregister_table_pre_exit(net: *mut net, af: u8, name: *const c_char);
}
extern "C" {
    pub fn xt_table_unlock(t: *mut xt_table);
}
extern "C" {
    pub fn xt_proto_init(net: *mut net, af: u_int8_t) -> c_int;
}
extern "C" {
    pub fn xt_proto_fini(net: *mut net, af: u_int8_t);
}
extern "C" {
    pub fn xt_free_table_info(info: *mut xt_table_info);
}
//
// var xt_recseq - recursive seqcount for netfilter use
//
// Packet processing changes the seqcount only if no recursion happened.
// get_counters() can use read_seqcount_begin()/read_seqcount_retry(),
// because we use the normal seqcount convention :
// Low order bit set to 1 if a writer is active.
//
// xt_tee_enabled - true if x_tables needs to handle reentrancy
//
// Enabled if current ip(6)tables ruleset has at least one -j TEE rule.
//
// xt_write_recseq_begin - start of a write section
//
// Begin packet processing : all readers must wait the end
// 1) Must be called with preemption disabled
// 2) softirqs must be disabled too (or we should use this_cpu_add())
// Returns:
// 1 if no recursion on this cpu
// 0 if recursion detected
//
// Low order bit of sequence is set if we already
// called xt_write_recseq_begin().
//
// This is kind of a write_seqcount_begin(), but addend is 0 or 1
// We dont check addend value to avoid a test and conditional jump,
// since addend is most likely 1
//
// xt_write_recseq_end - end of a write section
// @addend: return value from previous xt_write_recseq_begin()
//
// End packet processing : all readers can proceed
// 1) Must be called with preemption disabled
// 2) softirqs must be disabled too (or we should use this_cpu_add())
//
// this is kind of a write_seqcount_end(), but addend is 0 or 1
//
// This helper is performance critical and must be inlined
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_percpu_counter_alloc_state {
    pub off: c_uint,
    pub mem: *const char __percpu,
}

extern "C" {
    pub fn xt_percpu_counter_free(cnt: *mut xt_counters);
}
extern "C" {
    pub fn this_cpu_ptr(cnt->pcnt: *mut *mut (void __percpu ) (unsigned long)) -> return;
}
extern "C" {
    pub fn per_cpu_ptr(cnt->pcnt: *mut *mut (void __percpu ) (unsigned long), _arg: cpu) -> return;
}
extern "C" {
    pub fn xt_register_template(t: *const xt_table, net): *mut *mut int(table_init)(struct net) -> c_int;
}
extern "C" {
    pub fn xt_unregister_template(t: *const xt_table);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xt_entry_match {
    pub match_size: u_int16_t,
    pub 1]: char name[XT_FUNCTION_MAXNAMELEN -,
    pub revision: u_int8_t,
    pub user: },
    pub match_size: u_int16_t,
    pub match: compat_uptr_t,
    pub kernel: },
    pub match_size: u_int16_t,
    pub u: },
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xt_entry_target {
    pub target_size: u_int16_t,
    pub 1]: char name[XT_FUNCTION_MAXNAMELEN -,
    pub revision: u_int8_t,
    pub user: },
    pub target_size: u_int16_t,
    pub target: compat_uptr_t,
    pub kernel: },
    pub target_size: u_int16_t,
    pub u: },
    pub data: [c_uchar; ],
}

// FIXME: this works only on 32 bit tasks
// need to change whole approach in order to calculate align as function of
// current task alignment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xt_counters {
    pub /: *mut *mut compat_u64 pcnt, bcnt; / Packet and byte counters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xt_counters_info {
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub num_counters: compat_uint_t,
    pub counters: [compat_xt_counters; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _compat_xt_align {
    pub u8: __u8,
    pub u16: __u16,
    pub u32: __u32,
    pub u64: compat_u64,
}

extern "C" {
    pub fn xt_compat_lock(af: u_int8_t);
}
extern "C" {
    pub fn xt_compat_unlock(af: u_int8_t);
}
extern "C" {
    pub fn xt_compat_add_offset(af: u_int8_t, offset: c_uint, delta: c_int) -> c_int;
}
extern "C" {
    pub fn xt_compat_flush_offsets(af: u_int8_t);
}
extern "C" {
    pub fn xt_compat_init_offsets(af: u8, number: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_compat_calc_jump(af: u_int8_t, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn xt_compat_match_offset(match: *const xt_match) -> c_int;
}
extern "C" {
    pub fn xt_compat_target_offset(target: *const xt_target) -> c_int;
}

