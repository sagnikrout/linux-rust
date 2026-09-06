//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/alternative.c
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
//
// alternative runtime patching
// inspired by the ARM64 and x86 version
//
// Copyright (C) 2021 Sifive.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_manufacturer_info_t {
    pub vendor_id: c_ulong,
    pub arch_id: c_ulong,
    pub imp_id: c_ulong,
    void (*patch_func)(struct alt_entry *begin, struct alt_entry *end,
    unsigned long archid, unsigned long impid,
    pub stage): c_uint,
}

#[no_mangle]
unsafe extern "C" fn riscv_fill_cpu_mfr_info(cpu_mfr_info: *mut cpu_manufacturer_info_t) {
    static void riscv_fill_cpu_mfr_info(struct cpu_manufacturer_info_t *cpu_mfr_info)
    {

    cpu_mfr_info.vendor_id = csr_read(CSR_MVENDORID);
    cpu_mfr_info.arch_id = csr_read(CSR_MARCHID);
    cpu_mfr_info.imp_id = csr_read(CSR_MIMPID);

    cpu_mfr_info.vendor_id = sbi_get_mvendorid();
    cpu_mfr_info.arch_id = sbi_get_marchid();
    cpu_mfr_info.imp_id = sbi_get_mimpid();

    switch (cpu_mfr_info.vendor_id) {

    case ANDES_VENDOR_ID:
    cpu_mfr_info.patch_func = andes_errata_patch_func;
    break;

    case MIPS_VENDOR_ID:
    cpu_mfr_info.patch_func = mips_errata_patch_func;
    break;

    case SIFIVE_VENDOR_ID:
    cpu_mfr_info.patch_func = sifive_errata_patch_func;
    break;

    case THEAD_VENDOR_ID:
    cpu_mfr_info.patch_func = thead_errata_patch_func;
    break;

    default:
    cpu_mfr_info.patch_func = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn riscv_instruction_at(p: *mut c_void) -> u32 {
    static u32 riscv_instruction_at(void *p)
    {
    u16 *parcel = p;
    return (u32)parcel[0] | (u32)parcel[1] << 16;
    }
    static void riscv_alternative_fix_auipc_jalr(void *ptr, u32 auipc_insn,
    u32 jalr_insn, int patch_offset)
    {
    u32 call[2] = { auipc_insn, jalr_insn };
    s32 imm;
// get and adjust new target address
    imm = riscv_insn_extract_utype_itype_imm(auipc_insn, jalr_insn);
    imm -= patch_offset;
// update instructions
    riscv_insn_insert_utype_itype_imm(&call[0], &call[1], imm);
// patch the call place again
    patch_text_nosync(ptr, call, sizeof(u32) * 2);
    }
#[no_mangle]
unsafe extern "C" fn riscv_alternative_fix_jal(ptr: *mut c_void, jal_insn: u32, patch_offset: c_int) {
    static void riscv_alternative_fix_jal(void *ptr, u32 jal_insn, int patch_offset)
    {
    s32 imm;
// get and adjust new target address
    imm = riscv_insn_extract_jtype_imm(jal_insn);
    imm -= patch_offset;
// update instruction
    riscv_insn_insert_jtype_imm(&jal_insn, imm);
// patch the call place again
    patch_text_nosync(ptr, &jal_insn, sizeof(u32));
    }
    void riscv_alternative_fix_offsets(void *alt_ptr, unsigned int len,
    int patch_offset)
    {
    let mut num_insn: c_int = len / sizeof(u32);
    int i;
    for (i = 0; i < num_insn; i++) {
    let mut insn: u32 = riscv_instruction_at(alt_ptr + i * sizeof(u32));
//
// May be the start of an auipc + jalr pair
// Needs to check that at least one more instruction
// is in the list.
//
    if (riscv_insn_is_auipc(insn) && i < num_insn - 1) {
    let mut insn2: u32 = riscv_instruction_at(alt_ptr + (i + 1) * sizeof(u32));
    if (!riscv_insn_is_jalr(insn2))
    continue;
// if instruction pair is a call, it will use the ra register
    if (RV_EXTRACT_RD_REG(insn) != 1)
    continue;
    riscv_alternative_fix_auipc_jalr(alt_ptr + i * sizeof(u32),
    insn, insn2, patch_offset);
    i++;
    }
    if (riscv_insn_is_jal(insn)) {
    let mut imm: i32 = riscv_insn_extract_jtype_imm(insn);
// Don't modify jumps inside the alternative block
    if ((alt_ptr + i * sizeof(u32) + imm) >= alt_ptr &&
    (alt_ptr + i * sizeof(u32) + imm) < (alt_ptr + len))
    continue;
    riscv_alternative_fix_jal(alt_ptr + i * sizeof(u32),
    insn, patch_offset);
    }
    }
    }
//
// This is called very early in the boot process (directly after we run
// a feature detect on the boot CPU). No need to worry about other CPUs
// here.
//
    static void __init_or_module _apply_alternatives(struct alt_entry *begin,
    struct alt_entry *end,
    unsigned int stage)
    {
    struct cpu_manufacturer_info_t cpu_mfr_info;
    riscv_fill_cpu_mfr_info(&cpu_mfr_info);
    riscv_cpufeature_patch_func(begin, end, stage);
    if (!cpu_mfr_info.patch_func)
    return;
    cpu_mfr_info.patch_func(begin, end,
    cpu_mfr_info.arch_id,
    cpu_mfr_info.imp_id,
    stage);
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn apply_vdso_alternatives(base: *mut c_void, alternatives_begin: usize, alternatives_end: usize) -> void __init {
    void __init apply_vdso_alternatives(void *base, size_t alternatives_begin, size_t alternatives_end)
    {
    if (alternatives_begin == alternatives_end)
    return;
    _apply_alternatives(base + alternatives_begin,
    base + alternatives_end,
    RISCV_ALTERNATIVES_BOOT);
    }
#[no_mangle]
pub unsafe extern "C" fn apply_boot_alternatives() -> void __init {
    void __init apply_boot_alternatives(void)
    {
// If called on non-boot cpu things could go wrong
    WARN_ON(smp_processor_id() != 0);
    _apply_alternatives((struct alt_entry *)__alt_start,
    (struct alt_entry *)__alt_end,
    RISCV_ALTERNATIVES_BOOT);
    if (IS_ENABLED(CONFIG_MMU))
    apply_vdso_alternatives(vdso_start,
    __vdso_alternatives_start_offset,
    __vdso_alternatives_end_offset);
    if (IS_ENABLED(CONFIG_RISCV_USER_CFI))
    apply_vdso_alternatives(vdso_cfi_start,
    __vdso_alternatives_start_cfi_offset,
    __vdso_alternatives_end_cfi_offset);
    if (IS_ENABLED(CONFIG_COMPAT))
    apply_vdso_alternatives(compat_vdso_start,
    compat__vdso_alternatives_start_offset,
    compat__vdso_alternatives_end_offset);
    }
//
// apply_early_boot_alternatives() is called from setup_vm() with MMU-off.
//
// Following requirements should be honoured for it to work correctly:
// 1) It should use PC-relative addressing for accessing kernel symbols.
// To achieve this we always use GCC cmodel=medany.
// 2) The compiler instrumentation for FTRACE will not work for setup_vm()
// so disable compiler instrumentation when FTRACE is enabled.
//
// Currently, the above requirements are honoured by using custom CFLAGS
// for alternative.o in kernel/Makefile.
//
#[no_mangle]
pub unsafe extern "C" fn apply_early_boot_alternatives() -> void __init {
    void __init apply_early_boot_alternatives(void)
    {

    _apply_alternatives((struct alt_entry *)__alt_start,
    (struct alt_entry *)__alt_end,
    RISCV_ALTERNATIVES_EARLY_BOOT);

    }

#[no_mangle]
pub unsafe extern "C" fn apply_module_alternatives(start: *mut c_void, length: usize) {
    void apply_module_alternatives(void *start, size_t length)
    {
    _apply_alternatives((struct alt_entry *)start,
    (struct alt_entry *)(start + length),
    RISCV_ALTERNATIVES_MODULE);
    }
