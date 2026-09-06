//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/verifier.c
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

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    __maybe_unused
    static void run_tests_aux(const char *skel_name,
    skel_elf_bytes_fn elf_bytes_factory,
    pre_execution_cb pre_execution_cb)
    {
    let mut tester: test_loader = {};
    __u64 old_caps;
    int err;
// test_verifier tests are executed w/o CAP_SYS_ADMIN, do the same here
    err = cap_disable_effective(1ULL << CAP_SYS_ADMIN, &old_caps);
    if (err) {
    PRINT_FAIL("failed to drop CAP_SYS_ADMIN: %i, %s\n", err, strerror(-err));
    return;
    }
    test_loader__set_pre_execution_cb(&tester, pre_execution_cb);
    test_loader__run_subtests(&tester, skel_name, elf_bytes_factory);
    test_loader_fini(&tester);
    err = cap_enable_effective(old_caps, core::ptr::null_mut());
    if (err)
    PRINT_FAIL("failed to restore CAP_SYS_ADMIN: %i, %s\n", err, strerror(-err));
    }

    void test_arena_kfunc(void)                   { RUN_TESTS(arena_kfunc); }
    void test_arena_kfunc_jit(void)               { RUN_TESTS(arena_kfunc_jit); }
    void test_verifier_align(void)                { RUN(verifier_align); }
    void test_verifier_and(void)                  { RUN(verifier_and); }
    void test_verifier_arena(void)                { RUN(verifier_arena); }
    void test_verifier_arena_large(void)          { RUN(verifier_arena_large); }
    void test_verifier_arena_globals1(void)       { RUN(verifier_arena_globals1); }
    void test_verifier_arena_globals2(void)       { RUN(verifier_arena_globals2); }
    void test_verifier_basic_stack(void)          { RUN(verifier_basic_stack); }
    void test_verifier_bitfield_write(void)       { RUN(verifier_bitfield_write); }
    void test_verifier_bounds(void)               { RUN(verifier_bounds); }
    void test_verifier_bounds_deduction(void)     { RUN(verifier_bounds_deduction); }
    void test_verifier_bounds_deduction_non_const(void)     { RUN(verifier_bounds_deduction_non_const); }
    void test_verifier_bounds_mix_sign_unsign(void) { RUN(verifier_bounds_mix_sign_unsign); }
    void test_verifier_bpf_get_stack(void)        { RUN(verifier_bpf_get_stack); }
    void test_verifier_bpf_trap(void)             { RUN(verifier_bpf_trap); }
    void test_verifier_bswap(void)                { RUN(verifier_bswap); }
    void test_verifier_btf_ctx_access(void)       { RUN(verifier_btf_ctx_access); }
    void test_verifier_btf_unreliable_prog(void)  { RUN(verifier_btf_unreliable_prog); }
    void test_verifier_call_large_imm(void)       { RUN(verifier_call_large_imm); }
    void test_verifier_cfg(void)                  { RUN(verifier_cfg); }
    void test_verifier_cgroup_inv_retcode(void)   { RUN(verifier_cgroup_inv_retcode); }
    void test_verifier_cgroup_skb(void)           { RUN(verifier_cgroup_skb); }
    void test_verifier_cgroup_storage(void)       { RUN(verifier_cgroup_storage); }
    void test_verifier_const(void)                { RUN(verifier_const); }
    void test_verifier_const_or(void)             { RUN(verifier_const_or); }
    void test_verifier_ctx(void)                  { RUN_TESTS(verifier_ctx); }
    void test_verifier_ctx_sk_msg(void)           { RUN(verifier_ctx_sk_msg); }
    void test_verifier_d_path(void)               { RUN(verifier_d_path); }
    void test_verifier_default_trusted_ptr(void)  { RUN_TESTS(verifier_default_trusted_ptr); }
    void test_verifier_direct_packet_access(void) { RUN(verifier_direct_packet_access); }
    void test_verifier_direct_stack_access_wraparound(void) { RUN(verifier_direct_stack_access_wraparound); }
    void test_verifier_div0(void)                 { RUN(verifier_div0); }
    void test_verifier_div_mod_bounds(void)       { RUN(verifier_div_mod_bounds); }
    void test_verifier_div_overflow(void)         { RUN(verifier_div_overflow); }
    void test_verifier_flow_keys(void)            { RUN(verifier_flow_keys); }
    void test_verifier_global_subprogs(void)      { RUN(verifier_global_subprogs); }
    void test_verifier_global_ptr_args(void)      { RUN(verifier_global_ptr_args); }
    void test_verifier_gotol(void)                { RUN(verifier_gotol); }
    void test_verifier_gotox(void)                { RUN(verifier_gotox); }
    void test_verifier_helper_access_var_len(void) { RUN(verifier_helper_access_var_len); }
    void test_verifier_helper_packet_access(void) { RUN(verifier_helper_packet_access); }
    void test_verifier_helper_restricted(void)    { RUN(verifier_helper_restricted); }
    void test_verifier_helper_value_access(void)  { RUN(verifier_helper_value_access); }
    void test_verifier_int_ptr(void)              { RUN(verifier_int_ptr); }
    void test_verifier_iterating_callbacks(void)  { RUN(verifier_iterating_callbacks); }
    void test_verifier_jeq_infer_not_null(void)   { RUN(verifier_jeq_infer_not_null); }
    void test_verifier_jit_convergence(void)      { RUN(verifier_jit_convergence); }
    void test_verifier_load_acquire(void)         { RUN(verifier_load_acquire); }
    void test_verifier_ld_ind(void)               { RUN(verifier_ld_ind); }
    void test_verifier_ldsx(void)                  { RUN(verifier_ldsx); }
    void test_verifier_leak_ptr(void)             { RUN(verifier_leak_ptr); }
    void test_verifier_linked_scalars(void)       { RUN(verifier_linked_scalars); }
    void test_verifier_live_stack(void)           { RUN(verifier_live_stack); }
    void test_verifier_liveness_exp(void)         { RUN(verifier_liveness_exp); }
    void test_verifier_loops1(void)               { RUN(verifier_loops1); }
    void test_verifier_lwt(void)                  { RUN(verifier_lwt); }
    void test_verifier_map_in_map(void)           { RUN(verifier_map_in_map); }
    void test_verifier_map_lookup_refine(void)    { RUN(verifier_map_lookup_refine); }
    void test_verifier_map_ptr(void)              { RUN(verifier_map_ptr); }
    void test_verifier_map_ptr_mixing(void)       { RUN(verifier_map_ptr_mixing); }
    void test_verifier_map_ret_val(void)          { RUN(verifier_map_ret_val); }
    void test_verifier_masking(void)              { RUN(verifier_masking); }
    void test_verifier_may_goto_1(void)           { RUN(verifier_may_goto_1); }
    void test_verifier_may_goto_2(void)           { RUN(verifier_may_goto_2); }
    void test_verifier_mem_size_reg(void)         { RUN(verifier_mem_size_reg); }
    void test_verifier_meta_access(void)          { RUN(verifier_meta_access); }
    void test_verifier_movsx(void)                 { RUN(verifier_movsx); }
    void test_verifier_mul(void)                  { RUN(verifier_mul); }
    void test_verifier_netfilter_ctx(void)        { RUN(verifier_netfilter_ctx); }
    void test_verifier_netfilter_retcode(void)    { RUN(verifier_netfilter_retcode); }
    void test_verifier_bpf_fastcall(void)         { RUN(verifier_bpf_fastcall); }
    void test_verifier_or_jmp32_k(void)           { RUN(verifier_or_jmp32_k); }
    void test_verifier_percpu_addr(void)          { RUN(verifier_percpu_addr); }
    void test_verifier_precision(void)            { RUN(verifier_precision); }
    void test_verifier_prevent_map_lookup(void)   { RUN(verifier_prevent_map_lookup); }
    void test_verifier_private_stack(void)        { RUN(verifier_private_stack); }
    void test_verifier_ptr_to_buf(void)           { RUN(verifier_ptr_to_buf); }
    void test_verifier_raw_stack(void)            { RUN(verifier_raw_stack); }
    void test_verifier_raw_tp_writable(void)      { RUN(verifier_raw_tp_writable); }
    void test_verifier_reg_equal(void)            { RUN(verifier_reg_equal); }
    void test_verifier_ref_tracking(void)         { RUN(verifier_ref_tracking); }
    void test_verifier_regalloc(void)             { RUN(verifier_regalloc); }
    void test_verifier_ringbuf(void)              { RUN(verifier_ringbuf); }
    void test_verifier_runtime_jit(void)          { RUN(verifier_runtime_jit); }
    void test_verifier_scalar_ids(void)           { RUN(verifier_scalar_ids); }
    void test_verifier_sdiv(void)                 { RUN(verifier_sdiv); }
    void test_verifier_search_pruning(void)       { RUN(verifier_search_pruning); }
    void test_verifier_sock(void)                 { RUN(verifier_sock); }
    void test_verifier_sock_addr(void)            { RUN(verifier_sock_addr); }
    void test_verifier_sockmap_mutate(void)       { RUN(verifier_sockmap_mutate); }
    void test_verifier_spill_fill(void)           { RUN(verifier_spill_fill); }
    void test_verifier_spin_lock(void)            { RUN(verifier_spin_lock); }
    void test_verifier_stack_arg(void)            { RUN(verifier_stack_arg); }
    void test_verifier_stack_arg_order(void)      { RUN(verifier_stack_arg_order); }
    void test_verifier_stack_ptr(void)            { RUN(verifier_stack_ptr); }
    void test_verifier_store_release(void)        { RUN(verifier_store_release); }
    void test_verifier_subprog_insn_stats(void)   { RUN(verifier_subprog_insn_stats); }
    void test_verifier_subprog_precision(void)    { RUN(verifier_subprog_precision); }
    void test_verifier_subprog_topo(void)        { RUN(verifier_subprog_topo); }
    void test_verifier_subreg(void)               { RUN(verifier_subreg); }
    void test_verifier_tailcall(void)             { RUN(verifier_tailcall); }
    void test_verifier_tailcall_jit(void)         { RUN(verifier_tailcall_jit); }
    void test_verifier_typedef(void)              { RUN(verifier_typedef); }
    void test_verifier_uninit(void)               { RUN(verifier_uninit); }
    void test_verifier_unpriv(void)               { RUN(verifier_unpriv); }
    void test_verifier_unpriv_perf(void)          { RUN(verifier_unpriv_perf); }
    void test_verifier_value_adj_spill(void)      { RUN(verifier_value_adj_spill); }
    void test_verifier_value(void)                { RUN(verifier_value); }
    void test_verifier_value_illegal_alu(void)    { RUN(verifier_value_illegal_alu); }
    void test_verifier_value_or_null(void)        { RUN(verifier_value_or_null); }
    void test_verifier_var_off(void)              { RUN(verifier_var_off); }
    void test_verifier_vfs_accept(void)	      { RUN(verifier_vfs_accept); }
    void test_verifier_vfs_reject(void)	      { RUN(verifier_vfs_reject); }
    void test_verifier_xadd(void)                 { RUN(verifier_xadd); }
    void test_verifier_xdp(void)                  { RUN(verifier_xdp); }
    void test_verifier_xdp_direct_packet_access(void) { RUN(verifier_xdp_direct_packet_access); }
    void test_verifier_bits_iter(void) { RUN(verifier_bits_iter); }
    void test_verifier_set_retval(void)            { RUN(verifier_set_retval); }
    void test_verifier_lsm(void)                  { RUN(verifier_lsm); }
    void test_irq(void)			      { RUN(irq); }
    void test_verifier_mtu(void)		      { RUN(verifier_mtu); }
    void test_verifier_jit_inline(void)               { RUN(verifier_jit_inline); }
    void test_verifier_ctx_ptr_param(void)       { RUN(verifier_ctx_ptr_param); }
    void test_verifier_zext(void)                 { RUN_TESTS(verifier_zext); }
#[no_mangle]
unsafe extern "C" fn init_test_val_map(obj: *mut bpf_object, map_name: *mut c_char) -> c_int {
    static int init_test_val_map(struct bpf_object *obj, char *map_name)
    {
    struct test_val value = {
    .index = (6 + 1) * sizeof(int),
    .foo[6] = 0xabcdef12,
    };
    struct bpf_map *map;
    int err, key = 0;
    map = bpf_object__find_map_by_name(obj, map_name);
    if (!map) {
    PRINT_FAIL("Can't find map '%s'\n", map_name);
    return -EINVAL;
    }
    err = bpf_map_update_elem(bpf_map__fd(map), &key, &value, 0);
    if (err) {
    PRINT_FAIL("Error while updating map '%s': %d\n", map_name, err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_array_access_maps(obj: *mut bpf_object) -> c_int {
    static int init_array_access_maps(struct bpf_object *obj)
    {
    return init_test_val_map(obj, "map_array_ro");
    }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_array_access() {
    void test_verifier_array_access(void)
    {
    run_tests_aux("verifier_array_access",
    verifier_array_access__elf_bytes,
    init_array_access_maps);
    }
    void test_verifier_async_cb_context(void)    { RUN(verifier_async_cb_context); }
#[no_mangle]
unsafe extern "C" fn init_value_ptr_arith_maps(obj: *mut bpf_object) -> c_int {
    static int init_value_ptr_arith_maps(struct bpf_object *obj)
    {
    return init_test_val_map(obj, "map_array_48b");
    }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_value_ptr_arith() {
    void test_verifier_value_ptr_arith(void)
    {
    run_tests_aux("verifier_value_ptr_arith",
    verifier_value_ptr_arith__elf_bytes,
    init_value_ptr_arith_maps);
    }
