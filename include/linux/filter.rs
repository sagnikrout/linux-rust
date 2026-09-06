//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/filter.h
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
// Linux Socket Filter Data Structures
//

// ArgX, context and stack frame pointer register positions. Note,
// Arg1, Arg2, Arg3, etc are used as argument mappings of function
// calls in BPF_CALL instruction.
//

// Additional register mappings for converted user programs.

// Kernel hidden auxiliary/helper register.

// unused opcode to mark special call to bpf_tail_call() helper
pub const BPF_TAIL_CALL: c_uint = 0xf0;
// unused opcode to mark special load instruction. Same as BPF_ABS
pub const BPF_PROBE_MEM: c_uint = 0x20;
// unused opcode to mark special ldsx instruction. Same as BPF_IND
pub const BPF_PROBE_MEMSX: c_uint = 0x40;
// unused opcode to mark special load instruction. Same as BPF_MSH
pub const BPF_PROBE_MEM32: c_uint = 0xa0;
// unused opcode to mark special atomic instruction
pub const BPF_PROBE_ATOMIC: c_uint = 0xe0;
// unused opcode to mark special ldsx instruction. Same as BPF_NOSPEC
pub const BPF_PROBE_MEM32SX: c_uint = 0xc0;
// unused opcode to mark call to interpreter with arguments
pub const BPF_CALL_ARGS: c_uint = 0xe0;
// unused opcode to mark speculation barrier for mitigating
// Spectre v1 and v4
//
pub const BPF_NOSPEC: c_uint = 0xc0;
// As per nm, we expose JITed images as text (code) section for
// kallsyms. That way, tools like perf can find it to match
// addresses.
//

// BPF program can access up to 512 bytes of stack space.
pub const MAX_BPF_STACK: c_int = 512;
// Helper macros for filter block array initializers.
// ALU ops on registers, bpf_add|sub|...: dst_reg += src_reg

// ALU ops on immediates, bpf_add|sub|...: dst_reg += imm32

// Endianess conversion, cpu_to_{l,b}e(), {l,b}e_to_cpu()

// Byte Swap, bswap16/32/64

// Short form of mov, dst_reg = src_reg

// Special (internal-only) form of mov, used to resolve per-CPU addrs:
// dst_reg = src_reg + <percpu_base_off>
// BPF_ADDR_PERCPU is used as a special insn->off value.
//

// Short form of mov, dst_reg = imm32

// Short form of movsx, dst_reg = (s8,s16,s32)src_reg

// Special form of mov32, used for doing explicit zero extension on dst.

// addr_space_cast from as(0) to as(1) is for converting bpf arena pointers
// to pointers in user vma.
//
// BPF_LD_IMM64 macro encodes single 'load 64-bit immediate' insn

// pseudo BPF_LD_IMM64 insn used to refer to process-local map_fd

// Short form of mov based on type, BPF_X: dst_reg = src_reg, BPF_K: dst_reg = imm32

// Direct packet access, R0 = *(uint *) (skb->data + imm32)

// Indirect packet access, R0 = *(uint *) (skb->data + src_reg + imm32)

// Memory load, dst_reg = *(uint *) (src_reg + off16)

// Memory load, dst_reg = *(signed size *) (src_reg + off16)

// Memory store, *(uint *) (dst_reg + off16) = src_reg

//
// Atomic operations:
//
// BPF_ADD                  *(uint *) (dst_reg + off16) += src_reg
// BPF_AND                  *(uint *) (dst_reg + off16) &= src_reg
// BPF_OR                   *(uint *) (dst_reg + off16) |= src_reg
// BPF_XOR                  *(uint *) (dst_reg + off16) ^= src_reg
// BPF_ADD | BPF_FETCH      src_reg = atomic_fetch_add(dst_reg + off16, src_reg);
// BPF_AND | BPF_FETCH      src_reg = atomic_fetch_and(dst_reg + off16, src_reg);
// BPF_OR | BPF_FETCH       src_reg = atomic_fetch_or(dst_reg + off16, src_reg);
// BPF_XOR | BPF_FETCH      src_reg = atomic_fetch_xor(dst_reg + off16, src_reg);
// BPF_XCHG                 src_reg = atomic_xchg(dst_reg + off16, src_reg)
// BPF_CMPXCHG              r0 = atomic_cmpxchg(dst_reg + off16, r0, src_reg)
// BPF_LOAD_ACQ             dst_reg = smp_load_acquire(src_reg + off16)
// BPF_STORE_REL            smp_store_release(dst_reg + off16, src_reg)
//

// Legacy alias

//
// Given a BPF_ATOMIC instruction @atomic_insn, return true if it is an
// atomic load or store, and false if it is a read-modify-write instruction.
//
// A load-acquire is the only BPF_STX class instruction that reads into
// dst_reg from src_reg + off16, i.e. it has the operand roles of a BPF_LDX.
// Unlike bpf_atomic_is_load_store(), @insn is not assumed to be a BPF_ATOMIC
// instruction here, so that callers which walk all instruction classes can
// use this directly.
//
// Given an instruction @insn, return the number of the BPF register that a
// BPF_ATOMIC reads the value at its memory operand into, or -1 if there is
// no such register. That is the register a BPF_PROBE_ATOMIC has to clear when
// the access faults. Like bpf_atomic_is_load_acq(), @insn is not assumed to
// be a BPF_ATOMIC here.
//
// Memory store, *(uint *) (dst_reg + off16) = imm32

// Conditional jumps against registers, if (dst_reg 'op' src_reg) goto pc + off16

// Conditional jumps against immediates, if (dst_reg 'op' imm32) goto pc + off16

// Like BPF_JMP_REG, but with 32-bit wide operands for comparison.

// Like BPF_JMP_IMM, but with 32-bit wide operands for comparison.

// Unconditional jumps, goto pc + off16

// Unconditional jumps, gotol pc + imm32

// Relative call

// Convert function address to BPF immediate

// Kfunc call

// Raw code statement block

// Program exit

// Speculation barrier

// Internal classic blocks for direct assignment

// (PTR_SIZE) = (SIZE);						\
// A struct sock_filter is architecture independent.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sock_fprog {
    pub len: u16,
    pub /: *mut *mut *mut compat_uptr_t filter; / struct sock_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_fprog_kern {
    pub len: u16,
    pub filter: *mut sock_filter,
}

// Some arches need doubleword alignment for their instructions and/or data
pub const BPF_IMAGE_ALIGNMENT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_binary_header {
    pub size: u32,
    pub __aligned(BPF_IMAGE_ALIGNMENT): u8 image[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_stats {
    pub cnt: u64_stats_t,
    pub nsecs: u64_stats_t,
    pub misses: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub sizeof(u64)): *mut *mut } __aligned(2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_timed_may_goto {
    pub count: u64,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_filter {
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
    pub prog: *mut bpf_prog,
}

extern "C" {
    pub fn __bpf_prog_run(_arg: prog, _arg: ctx, _arg: bpf_dispatcher_nop_func) -> return;
}
//
// Use in preemptible and therefore migratable context to make sure that
// the execution of the BPF program runs on one CPU.
//
// This uses migrate_disable/enable() explicitly to document that the
// invocation of a BPF program does not require reentrancy protection
// against a BPF program which is invoked from a preempting task.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_skb_data_end {
    pub qdisc_cb: qdisc_skb_cb,
    pub data_meta: *mut c_void,
    pub data_end: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_nh_params {
    pub nh_family: u32,
    pub ipv4_nh: u32,
    pub ipv6_nh: in6_addr,
}

// flags for bpf_redirect_info kern_flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_redirect_info {
    pub tgt_index: u64,
    pub tgt_value: *mut c_void,
    pub map: *mut bpf_map,
    pub flags: u32,
    pub map_id: u32,
    pub map_type: bpf_map_type,
    pub nh: bpf_nh_params,
    pub kern_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_net_context {
    pub ri: bpf_redirect_info,
    pub cpu_map_flush_list: list_head,
    pub dev_map_flush_list: list_head,
    pub xskmap_map_flush_list: list_head,
}

// lh_map = *lh_dev = *lh_xsk = NULL;
// lh_dev = lh;
// lh_map = lh;
// lh_xsk = lh;
// Compute the linear packet data range [data, data_end) which
// will be accessed by various program types (cls_bpf, act_bpf,
// lwt, ...). Subsystems allowing direct data access must (!)
// ensure that cb[] area can be written to when BPF program is
// invoked (otherwise cb[] save/restore is necessary).
//
// Similar to bpf_compute_data_pointers(), except that save orginal
// data in cb->data and cb->meta_data for restore.
//
// saved_data_end = cb->data_end;
// Restore data saved by bpf_compute_and_save_data_end().
// eBPF programs may read/write skb->cb[] area to transfer meta
// data between tail calls. Since this also needs to work with
// tc, that scratch memory is mapped to qdisc_skb_cb's data area.
//
// In some socket filter cases, the cb unfortunately needs to be
// saved/restored so that protocol specific skb->cb[] data won't
// be lost. In any case, due to unpriviledged eBPF programs
// attached to sockets, we need to clear the bpf_skb_cb() area
// to not leak previous contents to user space.
//
// Must be invoked with migration disabled
extern "C" {
    pub fn xdp_master_redirect(xdp: *mut xdp_buff) -> u32;
}
extern "C" {
    pub fn bpf_prog_change_xdp(prev_prog: *mut bpf_prog, prog: *mut bpf_prog);
}
// When classic BPF programs have been loaded and the arch
// does not have a classic BPF JIT (anymore), they have been
// converted via bpf_migrate_filter() to eBPF and thus always
// have an unspec program type.
//

extern "C" {
    pub fn set_memory_ro(long)fp: (unsigned, _arg: fp->pages) -> return;
}

extern "C" {
    pub fn set_memory_rox(long)hdr: (unsigned, PAGE_SHIFT: hdr->size >>) -> return;
}
extern "C" {
    pub fn sk_filter_trim_cap(_arg: sk, _arg: skb, _arg: 1) -> return;
}
extern "C" {
    pub fn bpf_prog_free(fp: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_opcode_in_insntable(code: u8) -> bool;
}
extern "C" {
    pub fn bpf_prog_alloc_jited_linfo(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_prog_jit_attempt_done(prog: *mut bpf_prog);
}
extern "C" {
    pub fn __bpf_prog_free(fp: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_create(pfp: *mut bpf_prog, fprog: *mut sock_fprog_kern) -> c_int;
}
extern "C" {
    pub fn bpf_prog_destroy(fp: *mut bpf_prog);
}
extern "C" {
    pub fn sk_attach_filter(fprog: *mut sock_fprog, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn sk_attach_bpf(ufd: u32, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn sk_reuseport_attach_filter(fprog: *mut sock_fprog, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn sk_reuseport_attach_bpf(ufd: u32, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn sk_reuseport_prog_free(prog: *mut bpf_prog);
}
extern "C" {
    pub fn sk_detach_filter(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn sk_get_filter(sk: *mut sock, optval: sockptr_t, len: c_uint) -> c_int;
}
extern "C" {
    pub fn sk_filter_charge(sk: *mut sock, fp: *mut sk_filter) -> bool;
}
extern "C" {
    pub fn sk_filter_uncharge(sk: *mut sock, fp: *mut sk_filter);
}
extern "C" {
    pub fn __bpf_call_base(r1: u64, r2: u64, r3: u64, r4: u64, r5: u64) -> u64;
}
extern "C" {
    pub fn bpf_jit_compile(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_jit_needs_zext() -> bool;
}
extern "C" {
    pub fn bpf_jit_inlines_helper_call(imm: i32) -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_subprog_tailcalls() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_percpu_insn() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_kfunc_call() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_stack_args() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_arena_args() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_far_kfunc_call() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_exceptions() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_ptr_xchg() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_arena() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_insn(insn: *mut bpf_insn, in_arena: bool) -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_private_stack() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_timed_may_goto() -> bool;
}
extern "C" {
    pub fn bpf_jit_supports_fsession() -> bool;
}
extern "C" {
    pub fn bpf_arch_uaddress_limit() -> u64;
}
extern "C" {
    pub fn arch_bpf_stack_walk(cookie: *mut *mut bool (consume_fn)(void, ip: u64, sp: u64, bp): u64, cookie: *mut c_void);
}
extern "C" {
    pub fn arch_bpf_timed_may_goto() -> u64;
}
extern "C" {
    pub fn bpf_check_timed_may_goto(: *mut bpf_timed_may_goto) -> u64;
}
extern "C" {
    pub fn bpf_helper_changes_pkt_data(func_id: bpf_func_id) -> bool;
}
// Reconstruction of call-sites is dependent on kallsyms,
// thus make dump the same restriction.
//
extern "C" {
    pub fn kallsyms_show_value(_arg: cred) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}

extern "C" {
    pub fn bpf_remove_insns(prog: *mut bpf_prog, off: u32, cnt: u32) -> c_int;
}
// The pair of xdp_do_redirect and xdp_do_flush MUST be called in the
// same cpu context. Further for best results no more than a single map
// for the do_redirect/do_flush pair should be used. This limitation is
// because we only track one map and force a flush when the map changes.
// This does not appear to be a real limitation for existing software.
//
extern "C" {
    pub fn xdp_do_flush();
}

extern "C" {
    pub fn void(area: *mut *mut bpf_jit_fill_hole_t)(void, size: c_uint) -> typedef;
}
//
// Flush the indirect branch predictors before reusing JIT memory, so that
// indirect jumps into a newly written program don't reuse predictions left
// behind by an old program that occupied the same space.
//
extern "C" {
    pub fn bpf_arch_pred_flush();
}
extern "C" {
    pub fn bpf_jit_fill_hole_with_zero(area: *mut c_void, size: c_uint);
}
extern "C" {
    pub fn bpf_jit_binary_free(hdr: *mut bpf_binary_header);
}
extern "C" {
    pub fn bpf_jit_alloc_exec_limit() -> u64;
}
extern "C" {
    pub fn bpf_jit_free_exec(addr: *mut c_void);
}
extern "C" {
    pub fn bpf_jit_free(fp: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_pack_free(ptr: *mut c_void, size: u32);
}
extern "C" {
    pub fn bpf_jit_prog_release_other(fp: *mut bpf_prog, fp_other: *mut bpf_prog);
}

// These are the prerequisites, should someone ever have the
// idea to call blinding outside of them, we make sure to
// bail out.
//
// There are a couple of corner cases where kallsyms should
// not be enabled f.e. on hardening.
//
extern "C" {
    pub fn is_bpf_text_address(addr: c_ulong) -> bool;
}
extern "C" {
    pub fn bpf_prog_kallsyms_add(fp: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_kallsyms_del(fp: *mut bpf_prog);
}

extern "C" {
    pub fn bpf_prog_kallsyms_del_all(fp: *mut bpf_prog);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_addr_kern {
    pub sk: *mut sock,
    pub uaddr: *mut sockaddr_unsized,
// Temporary "register" to make indirect stores to nested structures
// defined above. We need three registers to make such a store, but
// only two (src and dst) are available at convert_ctx_access time
//
    pub tmp_reg: u64,
    pub /: *mut *mut *mut void t_ctx; / Attach type specific context.,
    pub uaddrlen: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_ops_kern {
    pub sk: *mut sock,
    pub args: [u32; 4],
    pub reply: u32,
    pub replylong: [u32; 4],
}

// initialized to 0 before calling
// the BPF program. New fields that
// should be initialized to 0 should
// be inserted before temp.
// temp is scratch storage used by
// sock_ops_convert_ctx_access
// as temporary storage of a register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sysctl_kern {
    pub head: *mut ctl_table_header,
    pub table: *const ctl_table,
    pub cur_val: *mut c_void,
    pub cur_len: usize,
    pub new_val: *mut c_void,
    pub new_len: usize,
    pub new_updated: c_int,
    pub write: c_int,
    pub ppos: *mut loff_t,
// Temporary "register" for indirect stores to ppos.
    pub tmp_reg: u64,
}

pub const BPF_SOCKOPT_KERN_BUF_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sockopt_buf {
    pub data: [u8; BPF_SOCKOPT_KERN_BUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sockopt_kern {
    pub sk: *mut sock,
    pub optval: *mut u8,
    pub optval_end: *mut u8,
    pub level: i32,
    pub optname: i32,
    pub optlen: i32,
// for retval in struct bpf_cg_run_ctx
    pub current_task: *mut task_struct,
// Temporary "register" for indirect stores to ppos.
    pub tmp_reg: u64,
}

extern "C" {
    pub fn copy_bpf_fprog_from_user(dst: *mut sock_fprog, src: sockptr_t, len: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sk_lookup_kern {
    pub family: u16,
    pub protocol: u16,
    pub sport: __be16,
    pub dport: u16,
    pub saddr: __be32,
    pub daddr: __be32,
    pub v4: },
    pub saddr: *const in6_addr,
    pub daddr: *const in6_addr,
    pub v6: },
    pub selected_sk: *mut sock,
    pub ingress_ifindex: u32,
    pub no_reuseport: bool,
}

// Runners for BPF_SK_LOOKUP programs to invoke on socket lookup.
//
// Allowed return values for a BPF SK_LOOKUP program are SK_PASS and
// SK_DROP. Their meaning is as follows:
//
// SK_PASS && ctx.selected_sk != NULL: use selected_sk as lookup result
// SK_PASS && ctx.selected_sk == NULL: continue to htable-based socket lookup
// SK_DROP                           : terminate lookup with -ECONNREFUSED
//
// This macro aggregates return values and selected sockets from
// multiple BPF programs according to following rules in order:
//
// 1. If any program returned SK_PASS and a non-NULL ctx.selected_sk,
// macro result is SK_PASS and last ctx.selected_sk is used.
// 2. If any program returned SK_DROP return value,
// macro result is SK_DROP.
// 3. Otherwise result is SK_PASS and ctx.selected_sk is NULL.
//
// Caller must ensure that the prog array is non-NULL, and that the
// array as well as the programs it contains remain valid.
//

// restore most recent selection */		\
// remember last non-NULL socket */	\
// psk = selected_sk;

// psk = selected_sk;

// Lower bits of the flags are used as return code on lookup failure
// If the lookup fails we want to clear out the state in the
// redirect_info struct completely, so that if an eBPF program
// performs multiple lookups, the last one always takes
// precedence.
//

extern "C" {
    pub fn __bpf_skb_load_bytes(skb: *const sk_buff, offset: u32, to: *mut c_void, len: u32) -> c_int;
}
extern "C" {
    pub fn __bpf_xdp_load_bytes(xdp: *mut xdp_buff, offset: u32, buf: *mut c_void, len: u32) -> c_int;
}
extern "C" {
    pub fn __bpf_xdp_store_bytes(xdp: *mut xdp_buff, offset: u32, buf: *mut c_void, len: u32) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

