//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vector.c
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
// Copyright (C) 2023 SiFive
// Author: Andy Chiu <andy.chiu@sifive.com>
//

    let mut riscv_v_implicit_uacc: static bool = IS_ENABLED(CONFIG_RISCV_ISA_V_DEFAULT_ENABLE);
    static struct kmem_cache *riscv_v_user_cachep;

    static struct kmem_cache *riscv_v_kernel_cachep;

    unsigned long riscv_v_vsize __read_mostly;
    EXPORT_SYMBOL_GPL(riscv_v_vsize);
#[no_mangle]
pub unsafe extern "C" fn riscv_v_setup_vsize() -> c_int {
    int riscv_v_setup_vsize(void)
    {
    unsigned long this_vsize;
//
// There are 32 vector registers with vlenb length.
//
// If the thead,vlenb property was provided by the firmware, use that
// instead of probing the CSRs.
//
    if (thead_vlenb_of) {
    riscv_v_vsize = thead_vlenb_of * 32;
    return 0;
    }
    riscv_v_enable();
    this_vsize = csr_read(CSR_VLENB) * 32;
    riscv_v_disable();
    if (!riscv_v_vsize) {
    riscv_v_vsize = this_vsize;
    return 0;
    }
    if (riscv_v_vsize != this_vsize) {
    WARN(1, "RISCV_ISA_V only supports one vlenb on SMP systems");
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_setup_ctx_cache() -> void __init {
    void __init riscv_v_setup_ctx_cache(void)
    {
    if (!(has_vector() || has_xtheadvector()))
    return;
    update_regset_vector_info(riscv_v_vsize);
    riscv_v_user_cachep = kmem_cache_create_usercopy("riscv_vector_ctx",
    riscv_v_vsize, 16, SLAB_PANIC,
    0, riscv_v_vsize, core::ptr::null_mut());

    riscv_v_kernel_cachep = kmem_cache_create("riscv_vector_kctx",
    riscv_v_vsize, 16,
    SLAB_PANIC, core::ptr::null_mut());

    }
#[no_mangle]
pub unsafe extern "C" fn insn_is_vector(insn_buf: u32) -> bool {
    bool insn_is_vector(u32 insn_buf)
    {
    let mut opcode: u32 = insn_buf & __INSN_OPCODE_MASK;
    u32 width, csr;
//
// All V-related instructions, including CSR operations are 4-Byte. So,
// do not handle if the instruction length is not 4-Byte.
//
    if (unlikely(GET_INSN_LENGTH(insn_buf) != 4))
    return false;
    switch (opcode) {
    case RVV_OPCODE_VECTOR:
    return true;
    case RVV_OPCODE_VL:
    case RVV_OPCODE_VS:
    width = RVV_EXTRACT_VL_VS_WIDTH(insn_buf);
    if (width == RVV_VL_VS_WIDTH_8 || width == RVV_VL_VS_WIDTH_16 ||
    width == RVV_VL_VS_WIDTH_32 || width == RVV_VL_VS_WIDTH_64)
    return true;
    break;
    case RVG_OPCODE_SYSTEM:
    csr = RVG_EXTRACT_SYSTEM_CSR(insn_buf);
    if ((csr >= CSR_VSTART && csr <= CSR_VCSR) ||
    (csr >= CSR_VL && csr <= CSR_VLENB))
    return true;
    }
    return false;
    }
    static int riscv_v_thread_ctx_alloc(struct kmem_cache *cache,
    struct __riscv_v_ext_state *ctx)
    {
    void *datap;
    datap = kmem_cache_zalloc(cache, GFP_KERNEL);
    if (!datap)
    return -ENOMEM;
    ctx.datap = datap;
    memset(ctx, 0, offsetof(struct __riscv_v_ext_state, datap));
    ctx.vlenb = riscv_v_vsize / 32;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_thread_alloc(tsk: *mut task_struct) {
    void riscv_v_thread_alloc(struct task_struct *tsk)
    {

    riscv_v_thread_ctx_alloc(riscv_v_kernel_cachep, &tsk.thread.kernel_vstate);

    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_thread_free(tsk: *mut task_struct) {
    void riscv_v_thread_free(struct task_struct *tsk)
    {
    if (tsk.thread.vstate.datap)
    kmem_cache_free(riscv_v_user_cachep, tsk.thread.vstate.datap);

    if (tsk.thread.kernel_vstate.datap)
    kmem_cache_free(riscv_v_kernel_cachep, tsk.thread.kernel_vstate.datap);

    }

#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctrl_get_cur(tsk: *mut task_struct) -> c_int {
    static inline int riscv_v_ctrl_get_cur(struct task_struct *tsk)
    {
    return VSTATE_CTRL_GET_CUR(tsk.thread.vstate_ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctrl_get_next(tsk: *mut task_struct) -> c_int {
    static inline int riscv_v_ctrl_get_next(struct task_struct *tsk)
    {
    return VSTATE_CTRL_GET_NEXT(tsk.thread.vstate_ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctrl_test_inherit(tsk: *mut task_struct) -> bool {
    static inline bool riscv_v_ctrl_test_inherit(struct task_struct *tsk)
    {
    return VSTATE_CTRL_GET_INHERIT(tsk.thread.vstate_ctrl);
    }
    static inline void riscv_v_ctrl_set(struct task_struct *tsk, int cur, int nxt,
    bool inherit)
    {
    unsigned long ctrl;
    ctrl = cur & PR_RISCV_V_VSTATE_CTRL_CUR_MASK;
    ctrl |= VSTATE_CTRL_MAKE_NEXT(nxt);
    if (inherit)
    ctrl |= PR_RISCV_V_VSTATE_CTRL_INHERIT;
    tsk.thread.vstate_ctrl &= ~PR_RISCV_V_VSTATE_CTRL_MASK;
    tsk.thread.vstate_ctrl |= ctrl;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_vstate_ctrl_user_allowed() -> bool {
    bool riscv_v_vstate_ctrl_user_allowed(void)
    {
    return riscv_v_ctrl_get_cur(current) == PR_RISCV_V_VSTATE_CTRL_ON;
    }
    EXPORT_SYMBOL_GPL(riscv_v_vstate_ctrl_user_allowed);
#[no_mangle]
pub unsafe extern "C" fn riscv_v_first_use_handler(regs: *mut pt_regs) -> bool {
    bool riscv_v_first_use_handler(struct pt_regs *regs)
    {
    u32 __user *epc = (u32 __user *)regs.epc;
    let mut insn: u32 = (u32)regs.badaddr;
    if (!(has_vector() || has_xtheadvector()))
    return false;
// Do not handle if V is not supported, or disabled
    if (!riscv_v_vstate_ctrl_user_allowed())
    return false;
// If V has been enabled then it is not the first-use trap
    if (riscv_v_vstate_query(regs))
    return false;
// Get the instruction
    if (!insn) {
    if (__get_user(insn, epc))
    return false;
    }
// Filter out non-V instructions
    if (!insn_is_vector(insn))
    return false;
// Sanity check. datap should be null by the time of the first-use trap
    WARN_ON(current.thread.vstate.datap);
//
// Now we sure that this is a V instruction. And it executes in the
// context where VS has been off. So, try to allocate the user's V
// context and resume execution.
//
    if (riscv_v_thread_ctx_alloc(riscv_v_user_cachep, &current.thread.vstate)) {
    force_sig(SIGBUS);
    return true;
    }
    riscv_v_vstate_on(regs);
    riscv_v_vstate_set_restore(current, regs);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_vstate_ctrl_init(tsk: *mut task_struct) {
    void riscv_v_vstate_ctrl_init(struct task_struct *tsk)
    {
    bool inherit;
    int cur, next;
    if (!(has_vector() || has_xtheadvector()))
    return;
    next = riscv_v_ctrl_get_next(tsk);
    if (!next) {
    if (READ_ONCE(riscv_v_implicit_uacc))
    cur = PR_RISCV_V_VSTATE_CTRL_ON;
    else
    cur = PR_RISCV_V_VSTATE_CTRL_OFF;
    } else {
    cur = next;
    }
// Clear next mask if inherit-bit is not set
    inherit = riscv_v_ctrl_test_inherit(tsk);
    if (!inherit)
    next = PR_RISCV_V_VSTATE_CTRL_DEFAULT;
    riscv_v_ctrl_set(tsk, cur, next, inherit);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_vstate_ctrl_get_current() -> c_long {
    long riscv_v_vstate_ctrl_get_current(void)
    {
    if (!(has_vector() || has_xtheadvector()))
    return -EINVAL;
    return current.thread.vstate_ctrl & PR_RISCV_V_VSTATE_CTRL_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_vstate_ctrl_set_current(arg: c_ulong) -> c_long {
    long riscv_v_vstate_ctrl_set_current(unsigned long arg)
    {
    bool inherit;
    int cur, next;
    if (!(has_vector() || has_xtheadvector()))
    return -EINVAL;
    if (arg & ~PR_RISCV_V_VSTATE_CTRL_MASK)
    return -EINVAL;
    cur = VSTATE_CTRL_GET_CUR(arg);
    switch (cur) {
    case PR_RISCV_V_VSTATE_CTRL_OFF:
// Do not allow user to turn off V if current is not off
    if (riscv_v_ctrl_get_cur(current) != PR_RISCV_V_VSTATE_CTRL_OFF)
    return -EPERM;
    break;
    case PR_RISCV_V_VSTATE_CTRL_ON:
    break;
    case PR_RISCV_V_VSTATE_CTRL_DEFAULT:
    cur = riscv_v_ctrl_get_cur(current);
    break;
    default:
    return -EINVAL;
    }
    next = VSTATE_CTRL_GET_NEXT(arg);
    inherit = VSTATE_CTRL_GET_INHERIT(arg);
    switch (next) {
    case PR_RISCV_V_VSTATE_CTRL_DEFAULT:
    case PR_RISCV_V_VSTATE_CTRL_OFF:
    case PR_RISCV_V_VSTATE_CTRL_ON:
    riscv_v_ctrl_set(current, cur, next, inherit);
    return 0;
    }
    return -EINVAL;
    }

    static const struct ctl_table riscv_v_default_vstate_table[] = {
    {
    .procname	= "riscv_v_default_allow",
    .data		= &riscv_v_implicit_uacc,
    .maxlen		= sizeof(riscv_v_implicit_uacc),
    .mode		= 0644,
    .proc_handler	= proc_dobool,
    },
    };
#[no_mangle]
unsafe extern "C" fn riscv_v_sysctl_init() -> int __init {
    static int __init riscv_v_sysctl_init(void)
    {
    if (has_vector() || has_xtheadvector())
    if (!register_sysctl("abi", riscv_v_default_vstate_table))
    return -EINVAL;
    return 0;
    }

    static int __init riscv_v_sysctl_init(void) { return 0; }

#[no_mangle]
unsafe extern "C" fn riscv_v_init() -> int __init {
    static int __init riscv_v_init(void)
    {
    return riscv_v_sysctl_init();
    }
    core_initcall(riscv_v_init);
