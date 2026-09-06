//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/jit_disasm.c
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
//
// Based on:
//
// Minimal BPF JIT image disassembler
//
// Disassembles BPF JIT compiler emitted opcodes back to asm insn's for
// debugging or verification purposes.
//
// Copyright 2013 Daniel Borkmann <daniel@iogearbox.net>
// Licensed under the GNU General Public License, version 2.0 (GPLv2)
//

// Macro flag: #define _GNU_SOURCE

    static int oper_count;

// Macro flag: #define DISASM_SPACER
    typedef LLVMDisasmContextRef disasm_ctx_t;
#[no_mangle]
unsafe extern "C" fn printf_json(s: *mut c_char) -> c_int {
    static int printf_json(char *s)
    {
    s = strtok(s, " \t");
    jsonw_string_field(json_wtr, "operation", s);
    jsonw_name(json_wtr, "operands");
    jsonw_start_array(json_wtr);
    oper_count = 1;
    while ((s = strtok(core::ptr::null_mut(), " \t,()")) != 0) {
    jsonw_string(json_wtr, s);
    oper_count++;
    }
    return 0;
    }
// This callback to set the ref_type is necessary to have the LLVM disassembler
// print PC-relative addresses instead of byte offsets for branch instruction
// targets.
//
    static const char *
    symbol_lookup_callback(__maybe_unused void *disasm_info,
    __maybe_unused uint64_t ref_value,
    uint64_t *ref_type, __maybe_unused uint64_t ref_PC,
    __maybe_unused const char **ref_name)
    {
// ref_type = LLVMDisassembler_ReferenceType_InOut_None;
    return core::ptr::null_mut();
    }
    static int
    init_context(disasm_ctx_t *ctx, const char *arch,
    __maybe_unused const char *disassembler_options,
    __maybe_unused unsigned char *image, __maybe_unused ssize_t len,
    __maybe_unused __u64 func_ksym)
    {
    char *triple;
    if (arch)
    triple = LLVMNormalizeTargetTriple(arch);
    else
    triple = LLVMGetDefaultTargetTriple();
    if (!triple) {
    p_err("Failed to retrieve triple");
    return -1;
    }
//
// Enable all aarch64 ISA extensions so the disassembler can handle any
// instruction the kernel JIT might emit (e.g. ARM64 LSE atomics).
//
    if (!strncmp(triple, "aarch64", 7))
// ctx = LLVMCreateDisasmCPUFeatures(triple, "", "+all", NULL, 0, NULL,
    symbol_lookup_callback);
    else
// ctx = LLVMCreateDisasm(triple, NULL, 0, NULL, symbol_lookup_callback);
    LLVMDisposeMessage(triple);
    if (!*ctx) {
    p_err("Failed to create disassembler");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn destroy_context(ctx: *mut disasm_ctx_t) {
    static void destroy_context(disasm_ctx_t *ctx)
    {
    LLVMDisposeMessage(*ctx);
    }
    static int
    disassemble_insn(disasm_ctx_t *ctx, unsigned char *image, ssize_t len, int pc,
    __u64 func_ksym)
    {
    char buf[256];
    int count;
    count = LLVMDisasmInstruction(*ctx, image + pc, len - pc, func_ksym + pc,
    buf, sizeof(buf));
    if (json_output)
    printf_json(buf);
    else
    printf("%s", buf);
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn disasm_init() -> c_int {
    int disasm_init(void)
    {
    LLVMInitializeAllTargetInfos();
    LLVMInitializeAllTargetMCs();
    LLVMInitializeAllDisassemblers();
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disasm_info {
    pub info: disassemble_info,
    pub func_ksym: __u64,
}

#[no_mangle]
unsafe extern "C" fn disasm_print_addr(addr: bfd_vma, info: *mut disassemble_info) {
    static void disasm_print_addr(bfd_vma addr, struct disassemble_info *info)
    {
    struct disasm_info *dinfo = container_of(info, struct disasm_info, info);
    addr += dinfo.func_ksym;
    generic_print_address(addr, info);
    }
    typedef struct {
    struct disasm_info *info;
    disassembler_ftype disassemble;
    bfd *bfdf;
    } disasm_ctx_t;
#[no_mangle]
unsafe extern "C" fn get_exec_path(tpath: *mut c_char, size: usize) -> c_int {
    static int get_exec_path(char *tpath, size_t size)
    {
    const char *path = "/proc/self/exe";
    ssize_t len;
    len = readlink(path, tpath, size - 1);
    if (len <= 0)
    return -1;
    tpath[len] = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn printf_json(out: *mut c_void, fmt: *const c_char, ap: va_list) -> c_int {
    static int printf_json(void *out, const char *fmt, va_list ap)
    {
    char *s;
    int err;
    err = vasprintf(&s, fmt, ap);
    if (err < 0)
    return -1;
    if (!oper_count) {
    int i;
// Strip trailing spaces
    i = strlen(s) - 1;
    while (s[i] == ' ')
    s[i--] = '\0';
    jsonw_string_field(json_wtr, "operation", s);
    jsonw_name(json_wtr, "operands");
    jsonw_start_array(json_wtr);
    oper_count++;
    } else if (!strcmp(fmt, ",")) {
// Skip
    } else {
    jsonw_string(json_wtr, s);
    oper_count++;
    }
    free(s);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fprintf_json(out: *mut c_void, fmt: *const c_char, ...) -> c_int {
    static int fprintf_json(void *out, const char *fmt, ...)
    {
    va_list ap;
    int r;
    va_start(ap, fmt);
    r = printf_json(out, fmt, ap);
    va_end(ap);
    return r;
    }
    static int fprintf_json_styled(void *out,
    enum disassembler_style style __maybe_unused,
    const char *fmt, ...)
    {
    va_list ap;
    int r;
    va_start(ap, fmt);
    r = printf_json(out, fmt, ap);
    va_end(ap);
    return r;
    }
    static int init_context(disasm_ctx_t *ctx, const char *arch,
    const char *disassembler_options,
    unsigned char *image, ssize_t len, __u64 func_ksym)
    {
    struct disassemble_info *info;
    char tpath[PATH_MAX];
    bfd *bfdf;
    memset(tpath, 0, sizeof(tpath));
    if (get_exec_path(tpath, sizeof(tpath))) {
    p_err("failed to create disassembler (get_exec_path)");
    return -1;
    }
    ctx.bfdf = bfd_openr(tpath, core::ptr::null_mut());
    if (!ctx.bfdf) {
    p_err("failed to create disassembler (bfd_openr)");
    return -1;
    }
    if (!bfd_check_format(ctx.bfdf, bfd_object)) {
    p_err("failed to create disassembler (bfd_check_format)");
    goto err_close;
    }
    bfdf = ctx.bfdf;
    ctx.info = malloc(sizeof(struct disasm_info));
    if (!ctx.info) {
    p_err("mem alloc failed");
    goto err_close;
    }
    ctx.info.func_ksym = func_ksym;
    info = &ctx.info.info;
    if (json_output)
    init_disassemble_info_compat(info, stdout,
    (fprintf_ftype) fprintf_json,
    fprintf_json_styled);
    else
    init_disassemble_info_compat(info, stdout,
    (fprintf_ftype) fprintf,
    fprintf_styled);
// Update architecture info for offload.
    if (arch) {
    const bfd_arch_info_type *inf = bfd_scan_arch(arch);
    if (inf) {
    bfdf.arch_info = inf;
    } else {
    p_err("No libbfd support for %s", arch);
    goto err_free;
    }
    }
    info.arch = bfd_get_arch(bfdf);
    info.mach = bfd_get_mach(bfdf);
    if (disassembler_options)
    info.disassembler_options = disassembler_options;
    info.buffer = image;
    info.buffer_length = len;
    info.print_address_func = disasm_print_addr;
    disassemble_init_for_target(info);

    ctx.disassemble = disassembler(info.arch,
    bfd_big_endian(bfdf),
    info.mach,
    bfdf);

    ctx.disassemble = disassembler(bfdf);

    if (!ctx.disassemble) {
    p_err("failed to create disassembler");
    goto err_free;
    }
    return 0;
    err_free:
    free(info);
    err_close:
    bfd_close(ctx.bfdf);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn destroy_context(ctx: *mut disasm_ctx_t) {
    static void destroy_context(disasm_ctx_t *ctx)
    {
    free(ctx.info);
    bfd_close(ctx.bfdf);
    }
    static int
    disassemble_insn(disasm_ctx_t *ctx, __maybe_unused unsigned char *image,
    __maybe_unused ssize_t len, int pc,
    __maybe_unused __u64 func_ksym)
    {
    return ctx.disassemble(pc, &ctx.info.info);
    }
#[no_mangle]
pub unsafe extern "C" fn disasm_init() -> c_int {
    int disasm_init(void)
    {
    bfd_init();
    return 0;
    }

    int disasm_print_insn(unsigned char *image, ssize_t len, int opcodes,
    const char *arch, const char *disassembler_options,
    const struct btf *btf,
    const struct bpf_prog_linfo *prog_linfo,
    __u64 func_ksym, unsigned int func_idx,
    bool linum)
    {
    const struct bpf_line_info *linfo = core::ptr::null_mut();
    let mut nr_skip: c_uint = 0;
    int count, i;
    let mut pc: c_uint = 0;
    disasm_ctx_t ctx;
    if (!len)
    return -1;
    if (init_context(&ctx, arch, disassembler_options, image, len, func_ksym))
    return -1;
    if (json_output)
    jsonw_start_array(json_wtr);
    do {
    if (prog_linfo) {
    linfo = bpf_prog_linfo__lfind_addr_func(prog_linfo,
    func_ksym + pc,
    func_idx,
    nr_skip);
    if (linfo)
    nr_skip++;
    }
    if (json_output) {
    jsonw_start_object(json_wtr);
    oper_count = 0;
    if (linfo)
    btf_dump_linfo_json(btf, linfo, linum);
    jsonw_name(json_wtr, "pc");
    jsonw_printf(json_wtr, "\"0x%x\"", pc);
    } else {
    if (linfo)
    btf_dump_linfo_plain(btf, linfo, "; ",
    linum);
    printf("%4x:" DISASM_SPACER, pc);
    }
    count = disassemble_insn(&ctx, image, len, pc, func_ksym);
    if (json_output) {
// Operand array, was started in fprintf_json. Before
// that, make sure we have a _null_ value if no operand
// other than operation code was present.
//
    if (oper_count == 1)
    jsonw_null(json_wtr);
    jsonw_end_array(json_wtr);
    }
    if (opcodes) {
    if (json_output) {
    jsonw_name(json_wtr, "opcodes");
    jsonw_start_array(json_wtr);
    for (i = 0; i < count; ++i)
    jsonw_printf(json_wtr, "\"0x%02hhx\"",
    (uint8_t)image[pc + i]);
    jsonw_end_array(json_wtr);
    } else {
    printf("\n\t");
    for (i = 0; i < count; ++i)
    printf("%02x ",
    (uint8_t)image[pc + i]);
    }
    }
    if (json_output)
    jsonw_end_object(json_wtr);
    else
    printf("\n");
    pc += count;
    } while (count > 0 && pc < len);
    if (json_output)
    jsonw_end_array(json_wtr);
    destroy_context(&ctx);
    return 0;
    }
