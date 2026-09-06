//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/cper-x86.c
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
// Copyright (C) 2018, Advanced Micro Devices, Inc.

//
// We don't need a "CPER_IA" prefix since these are all locally defined.
// This will save us a lot of line space.
//

    GUID_INIT(0xA55701F5, 0xE3EF, 0x43DE, 0xAC, 0x72, 0x24, 0x9B,	\
    0x57, 0x3F, 0xAD, 0x2C)

    GUID_INIT(0xFC06B535, 0x5E1F, 0x4562, 0x9F, 0x25, 0x0A, 0x3B,	\
    0x9A, 0xDB, 0x63, 0xC3)

    GUID_INIT(0x1CF3F8B3, 0xC5B1, 0x49a2, 0xAA, 0x59, 0x5E, 0xEF,	\
    0x92, 0xFF, 0xA6, 0x3C)

    GUID_INIT(0x48AB7F57, 0xDC34, 0x4f6c, 0xA7, 0xD3, 0xB0, 0xB5,	\
    0xB0, 0xA7, 0x43, 0x14)

pub const CTX_TYPE_MSR: c_int = 1;
pub const CTX_TYPE_MMREG: c_int = 7;
    enum err_types {
    ERR_TYPE_CACHE = 0,
    ERR_TYPE_TLB,
    ERR_TYPE_BUS,
    ERR_TYPE_MS,
    N_ERR_TYPES
    };
#[no_mangle]
unsafe extern "C" fn cper_get_err_type(err_type: *const guid_t) -> enum err_types {
    static enum err_types cper_get_err_type(const guid_t *err_type)
    {
    if (guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_CACHE))
    return ERR_TYPE_CACHE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: guid_equal(err_type, _arg: &INFO_ERR_STRUCT_TYPE_TLB)) -> else {
    else if (guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_TLB))
    return ERR_TYPE_TLB;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: guid_equal(err_type, _arg: &INFO_ERR_STRUCT_TYPE_BUS)) -> else {
    else if (guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_BUS))
    return ERR_TYPE_BUS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: guid_equal(err_type, _arg: &INFO_ERR_STRUCT_TYPE_MS)) -> else {
    else if (guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_MS))
    return ERR_TYPE_MS;
    else
    return N_ERR_TYPES;
    }
    static const char * const ia_check_trans_type_strs[] = {
    "Instruction",
    "Data Access",
    "Generic",
    };
    static const char * const ia_check_op_strs[] = {
    "generic error",
    "generic read",
    "generic write",
    "data read",
    "data write",
    "instruction fetch",
    "prefetch",
    "eviction",
    "snoop",
    };
    static const char * const ia_check_bus_part_type_strs[] = {
    "Local Processor originated request",
    "Local Processor responded to request",
    "Local Processor observed",
    "Generic",
    };
    static const char * const ia_check_bus_addr_space_strs[] = {
    "Memory Access",
    "Reserved",
    "I/O",
    "Other Transaction",
    };
    static const char * const ia_check_ms_error_type_strs[] = {
    "No Error",
    "Unclassified",
    "Microcode ROM Parity Error",
    "External Error",
    "FRC Error",
    "Internal Unclassified",
    };
    static const char * const ia_reg_ctx_strs[] = {
    "Unclassified Data",
    "MSR Registers (Machine Check and other MSRs)",
    "32-bit Mode Execution Context",
    "64-bit Mode Execution Context",
    "FXSAVE Context",
    "32-bit Mode Debug Registers (DR0-DR7)",
    "64-bit Mode Debug Registers (DR0-DR7)",
    "Memory Mapped Registers",
    };
#[no_mangle]
pub unsafe extern "C" fn print_bool(str: *mut c_char, pfx: *const c_char, check: u64, bit: u64) {
    static inline void print_bool(char *str, const char *pfx, u64 check, u64 bit)
    {
    printk("%s%s: %s\n", pfx, str, (check & bit) ? "true" : "false");
    }
#[no_mangle]
unsafe extern "C" fn print_err_info_ms(pfx: *const c_char, validation_bits: u16, check: u64) {
    static void print_err_info_ms(const char *pfx, u16 validation_bits, u64 check)
    {
    if (validation_bits & CHECK_VALID_MS_ERR_TYPE) {
    let mut err_type: u8 = CHECK_MS_ERR_TYPE(check);
    printk("%sError Type: %u, %s\n", pfx, err_type,
    err_type < ARRAY_SIZE(ia_check_ms_error_type_strs) ?
    ia_check_ms_error_type_strs[err_type] : "unknown");
    }
    if (validation_bits & CHECK_VALID_MS_PCC)
    print_bool("Processor Context Corrupt", pfx, check, CHECK_MS_PCC);
    if (validation_bits & CHECK_VALID_MS_UNCORRECTED)
    print_bool("Uncorrected", pfx, check, CHECK_MS_UNCORRECTED);
    if (validation_bits & CHECK_VALID_MS_PRECISE_IP)
    print_bool("Precise IP", pfx, check, CHECK_MS_PRECISE_IP);
    if (validation_bits & CHECK_VALID_MS_RESTARTABLE_IP)
    print_bool("Restartable IP", pfx, check, CHECK_MS_RESTARTABLE_IP);
    if (validation_bits & CHECK_VALID_MS_OVERFLOW)
    print_bool("Overflow", pfx, check, CHECK_MS_OVERFLOW);
    }
#[no_mangle]
unsafe extern "C" fn print_err_info(pfx: *const c_char, err_type: u8, check: u64) {
    static void print_err_info(const char *pfx, u8 err_type, u64 check)
    {
    let mut validation_bits: u16 = CHECK_VALID_BITS(check);
//
// The MS Check structure varies a lot from the others, so use a
// separate function for decoding.
//
    if (err_type == ERR_TYPE_MS)
    return print_err_info_ms(pfx, validation_bits, check);
    if (validation_bits & CHECK_VALID_TRANS_TYPE) {
    let mut trans_type: u8 = CHECK_TRANS_TYPE(check);
    printk("%sTransaction Type: %u, %s\n", pfx, trans_type,
    trans_type < ARRAY_SIZE(ia_check_trans_type_strs) ?
    ia_check_trans_type_strs[trans_type] : "unknown");
    }
    if (validation_bits & CHECK_VALID_OPERATION) {
    let mut op: u8 = CHECK_OPERATION(check);
//
// CACHE has more operation types than TLB or BUS, though the
// name and the order are the same.
//
    let mut max_ops: u8 = (err_type == ERR_TYPE_CACHE) ? 9 : 7;
    printk("%sOperation: %u, %s\n", pfx, op,
    op < max_ops ? ia_check_op_strs[op] : "unknown");
    }
    if (validation_bits & CHECK_VALID_LEVEL)
    printk("%sLevel: %llu\n", pfx, CHECK_LEVEL(check));
    if (validation_bits & CHECK_VALID_PCC)
    print_bool("Processor Context Corrupt", pfx, check, CHECK_PCC);
    if (validation_bits & CHECK_VALID_UNCORRECTED)
    print_bool("Uncorrected", pfx, check, CHECK_UNCORRECTED);
    if (validation_bits & CHECK_VALID_PRECISE_IP)
    print_bool("Precise IP", pfx, check, CHECK_PRECISE_IP);
    if (validation_bits & CHECK_VALID_RESTARTABLE_IP)
    print_bool("Restartable IP", pfx, check, CHECK_RESTARTABLE_IP);
    if (validation_bits & CHECK_VALID_OVERFLOW)
    print_bool("Overflow", pfx, check, CHECK_OVERFLOW);
    if (err_type != ERR_TYPE_BUS)
    return;
    if (validation_bits & CHECK_VALID_BUS_PART_TYPE) {
    let mut part_type: u8 = CHECK_BUS_PART_TYPE(check);
    printk("%sParticipation Type: %u, %s\n", pfx, part_type,
    part_type < ARRAY_SIZE(ia_check_bus_part_type_strs) ?
    ia_check_bus_part_type_strs[part_type] : "unknown");
    }
    if (validation_bits & CHECK_VALID_BUS_TIME_OUT)
    print_bool("Time Out", pfx, check, CHECK_BUS_TIME_OUT);
    if (validation_bits & CHECK_VALID_BUS_ADDR_SPACE) {
    let mut addr_space: u8 = CHECK_BUS_ADDR_SPACE(check);
    printk("%sAddress Space: %u, %s\n", pfx, addr_space,
    addr_space < ARRAY_SIZE(ia_check_bus_addr_space_strs) ?
    ia_check_bus_addr_space_strs[addr_space] : "unknown");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cper_print_proc_ia(pfx: *const c_char, proc: *const cper_sec_proc_ia) {
    void cper_print_proc_ia(const char *pfx, const struct cper_sec_proc_ia *proc)
    {
    int i;
    struct cper_ia_err_info *err_info;
    struct cper_ia_proc_ctx *ctx_info;
    char newpfx[64], infopfx[64];
    u8 err_type;
    if (proc.validation_bits & VALID_LAPIC_ID)
    printk("%sLocal APIC_ID: 0x%llx\n", pfx, proc.lapic_id);
    if (proc.validation_bits & VALID_CPUID_INFO) {
    printk("%sCPUID Info:\n", pfx);
    print_hex_dump(pfx, "", DUMP_PREFIX_OFFSET, 16, 4, proc.cpuid,
    sizeof(proc.cpuid), 0);
    }
    snprintf(newpfx, sizeof(newpfx), "%s ", pfx);
    err_info = (struct cper_ia_err_info *)(proc + 1);
    for (i = 0; i < VALID_PROC_ERR_INFO_NUM(proc.validation_bits); i++) {
    printk("%sError Information Structure %d:\n", pfx, i);
    err_type = cper_get_err_type(&err_info.err_type);
    printk("%sError Structure Type: %s\n", newpfx,
    err_type < ARRAY_SIZE(cper_proc_error_type_strs) ?
    cper_proc_error_type_strs[err_type] : "unknown");
    if (err_type >= N_ERR_TYPES) {
    printk("%sError Structure Type: %pUl\n", newpfx,
    &err_info.err_type);
    }
    if (err_info.validation_bits & INFO_VALID_CHECK_INFO) {
    printk("%sCheck Information: 0x%016llx\n", newpfx,
    err_info.check_info);
    if (err_type < N_ERR_TYPES) {
    snprintf(infopfx, sizeof(infopfx), "%s ",
    newpfx);
    print_err_info(infopfx, err_type,
    err_info.check_info);
    }
    }
    if (err_info.validation_bits & INFO_VALID_TARGET_ID) {
    printk("%sTarget Identifier: 0x%016llx\n",
    newpfx, err_info.target_id);
    }
    if (err_info.validation_bits & INFO_VALID_REQUESTOR_ID) {
    printk("%sRequestor Identifier: 0x%016llx\n",
    newpfx, err_info.requestor_id);
    }
    if (err_info.validation_bits & INFO_VALID_RESPONDER_ID) {
    printk("%sResponder Identifier: 0x%016llx\n",
    newpfx, err_info.responder_id);
    }
    if (err_info.validation_bits & INFO_VALID_IP) {
    printk("%sInstruction Pointer: 0x%016llx\n",
    newpfx, err_info.ip);
    }
    err_info++;
    }
    ctx_info = (struct cper_ia_proc_ctx *)err_info;
    for (i = 0; i < VALID_PROC_CXT_INFO_NUM(proc.validation_bits); i++) {
    let mut size: c_int = ALIGN(sizeof(*ctx_info) + ctx_info.reg_arr_size, 16);
    let mut groupsize: c_int = 4;
    printk("%sContext Information Structure %d:\n", pfx, i);
    printk("%sRegister Context Type: %s\n", newpfx,
    ctx_info.reg_ctx_type < ARRAY_SIZE(ia_reg_ctx_strs) ?
    ia_reg_ctx_strs[ctx_info.reg_ctx_type] : "unknown");
    printk("%sRegister Array Size: 0x%04x\n", newpfx,
    ctx_info.reg_arr_size);
    if (ctx_info.reg_ctx_type == CTX_TYPE_MSR) {
    groupsize = 8; /* MSRs are 8 bytes wide. */
    printk("%sMSR Address: 0x%08x\n", newpfx,
    ctx_info.msr_addr);
    }
    if (ctx_info.reg_ctx_type == CTX_TYPE_MMREG) {
    printk("%sMM Register Address: 0x%016llx\n", newpfx,
    ctx_info.mm_reg_addr);
    }
    if (ctx_info.reg_ctx_type != CTX_TYPE_MSR ||
    arch_apei_report_x86_error(ctx_info, proc.lapic_id)) {
    printk("%sRegister Array:\n", newpfx);
    print_hex_dump(newpfx, "", DUMP_PREFIX_OFFSET, 16,
    groupsize, (ctx_info + 1),
    ctx_info.reg_arr_size, 0);
    }
    ctx_info = (struct cper_ia_proc_ctx *)((long)ctx_info + size);
    }
    }
