//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/compressed/sev-handle-vc.c
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

// Macro flag: #define __BOOT_COMPRESSED

// Macro flag: #define __init
// Basic instruction decoding support needed

//
// Copy a version of this function here - insn-eval.c can't be used in
// pre-decompression code.
//
#[no_mangle]
pub unsafe extern "C" fn insn_has_rep_prefix(insn: *mut insn) -> bool {
    bool insn_has_rep_prefix(struct insn *insn)
    {
    insn_byte_t p;
    insn_get_prefixes(insn);
    for_each_insn_prefix(insn, p) {
    if (p == 0xf2 || p == 0xf3)
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn vc_decode_insn(ctxt: *mut es_em_ctxt) -> enum es_result {
    enum es_result vc_decode_insn(struct es_em_ctxt *ctxt)
    {
    char buffer[MAX_INSN_SIZE];
    int ret;
    memcpy(buffer, (unsigned char *)ctxt.regs.ip, MAX_INSN_SIZE);
    ret = insn_decode(&ctxt.insn, buffer, MAX_INSN_SIZE, INSN_MODE_64);
    if (ret < 0)
    return ES_DECODE_FAILED;
    return ES_OK;
    }
    extern void sev_insn_decode_init(void) __alias(inat_init_tables);
//
// Only a dummy for insn_get_seg_base() - Early boot-code is 64bit only and
// doesn't use segments.
//
#[no_mangle]
unsafe extern "C" fn insn_get_seg_base(regs: *mut pt_regs, seg_reg_idx: c_int) -> c_ulong {
    static unsigned long insn_get_seg_base(struct pt_regs *regs, int seg_reg_idx)
    {
    return 0UL;
    }
    static enum es_result vc_write_mem(struct es_em_ctxt *ctxt,
    void *dst, char *buf, size_t size)
    {
    memcpy(dst, buf, size);
    return ES_OK;
    }
    static enum es_result vc_read_mem(struct es_em_ctxt *ctxt,
    void *src, char *buf, size_t size)
    {
    memcpy(buf, src, size);
    return ES_OK;
    }
#[no_mangle]
unsafe extern "C" fn vc_ioio_check(ctxt: *mut es_em_ctxt, port: u16, size: usize) -> enum es_result {
    static enum es_result vc_ioio_check(struct es_em_ctxt *ctxt, u16 port, size_t size)
    {
    return ES_OK;
    }
#[no_mangle]
unsafe extern "C" fn fault_in_kernel_space(address: c_ulong) -> bool {
    static bool fault_in_kernel_space(unsigned long address)
    {
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn do_boot_stage2_vc(regs: *mut pt_regs, exit_code: c_ulong) {
    void do_boot_stage2_vc(struct pt_regs *regs, unsigned long exit_code)
    {
    struct es_em_ctxt ctxt;
    enum es_result result;
    if (!boot_ghcb && !early_setup_ghcb())
    sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ);
    vc_ghcb_invalidate(boot_ghcb);
    result = vc_init_em_ctxt(&ctxt, regs, exit_code);
    if (result != ES_OK)
    goto finish;
    result = vc_check_opcode_bytes(&ctxt, exit_code);
    if (result != ES_OK)
    goto finish;
    switch (exit_code) {
    case SVM_EXIT_RDTSC:
    case SVM_EXIT_RDTSCP:
    result = vc_handle_rdtsc(boot_ghcb, &ctxt, exit_code);
    break;
    case SVM_EXIT_IOIO:
    result = vc_handle_ioio(boot_ghcb, &ctxt);
    break;
    case SVM_EXIT_CPUID:
    result = vc_handle_cpuid(boot_ghcb, &ctxt);
    break;
    default:
    result = ES_UNSUPPORTED;
    break;
    }
    finish:
    if (result == ES_OK)
    vc_finish_insn(&ctxt);
#[no_mangle]
pub unsafe extern "C" fn if(ES_RETRY: result !=) -> else {
    else if (result != ES_RETRY)
    sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ);
    }
