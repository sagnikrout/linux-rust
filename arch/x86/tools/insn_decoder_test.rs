//! Automatically rewritten from C to Rust
//! Source: arch/x86/tools/insn_decoder_test.c
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
// Copyright (C) IBM Corporation, 2009
//

//
// Test of instruction analysis in general and insn_get_length() in
// particular.  See if insn_get_length() and the disassembler agree
// on the length of each instruction in an elf disassembly.
//
// Usage: objdump -d a.out | awk -f objdump_reformat.awk | ./insn_decoder_test
//
    const char *prog;
    static int verbose;
    static int x86_64;
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    fprintf(stderr, "Usage: objdump -d a.out | awk -f objdump_reformat.awk"
    " | %s [-y|-n] [-v]\n", prog);
    fprintf(stderr, "\t-y	64bit mode\n");
    fprintf(stderr, "\t-n	32bit mode\n");
    fprintf(stderr, "\t-v	verbose mode\n");
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn malformed_line(line: *const c_char, line_nr: c_int) {
    static void malformed_line(const char *line, int line_nr)
    {
    fprintf(stderr, "%s: error: malformed line %d:\n%s",
    prog, line_nr, line);
    exit(3);
    }
#[no_mangle]
unsafe extern "C" fn pr_warn(fmt: *const c_char, ...) {
    static void pr_warn(const char *fmt, ...)
    {
    va_list ap;
    fprintf(stderr, "%s: warning: ", prog);
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    }
    static void dump_field(FILE *fp, const char *name, const char *indent,
    struct insn_field *field)
    {
    fprintf(fp, "%s.%s = {\n", indent, name);
    fprintf(fp, "%s\t.value = %d, bytes[] = {%x, %x, %x, %x},\n",
    indent, field.value, field.bytes[0], field.bytes[1],
    field.bytes[2], field.bytes[3]);
    fprintf(fp, "%s\t.got = %d, .nbytes = %d},\n", indent,
    field.got, field.nbytes);
    }
#[no_mangle]
unsafe extern "C" fn dump_insn(fp: *mut FILE, insn: *mut insn) {
    static void dump_insn(FILE *fp, struct insn *insn)
    {
    fprintf(fp, "Instruction = {\n");
    dump_field(fp, "prefixes", "\t",	&insn.prefixes);
    dump_field(fp, "rex_prefix", "\t",	&insn.rex_prefix);
    dump_field(fp, "vex_prefix", "\t",	&insn.vex_prefix);
    dump_field(fp, "opcode", "\t",		&insn.opcode);
    dump_field(fp, "modrm", "\t",		&insn.modrm);
    dump_field(fp, "sib", "\t",		&insn.sib);
    dump_field(fp, "displacement", "\t",	&insn.displacement);
    dump_field(fp, "immediate1", "\t",	&insn.immediate1);
    dump_field(fp, "immediate2", "\t",	&insn.immediate2);
    fprintf(fp, "\t.attr = %x, .opnd_bytes = %d, .addr_bytes = %d,\n",
    insn.attr, insn.opnd_bytes, insn.addr_bytes);
    fprintf(fp, "\t.length = %d, .x86_64 = %d, .kaddr = %p}\n",
    insn.length, insn.x86_64, insn.kaddr);
    }
#[no_mangle]
unsafe extern "C" fn parse_args(argc: c_int, argv: *mut c_char) {
    static void parse_args(int argc, char **argv)
    {
    int c;
    prog = argv[0];
    while ((c = getopt(argc, argv, "ynv")) != -1) {
    switch (c) {
    case 'y':
    x86_64 = 1;
    break;
    case 'n':
    x86_64 = 0;
    break;
    case 'v':
    verbose = 1;
    break;
    default:
    usage();
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char line[BUFSIZE], sym[BUFSIZE] = "<unknown>";
    unsigned char insn_buff[16];
    struct insn insn;
    let mut insns: c_int = 0;
    let mut warnings: c_int = 0;
    parse_args(argc, argv);
    while (fgets(line, BUFSIZE, stdin)) {
    char copy[BUFSIZE], *s, *tab1, *tab2;
    let mut nb: c_int = 0, ret;
    unsigned int b;
    if (line[0] == '<') {
// Symbol line
    strcpy(sym, line);
    continue;
    }
    insns++;
    memset(insn_buff, 0, 16);
    strcpy(copy, line);
    tab1 = strchr(copy, '\t');
    if (!tab1)
    malformed_line(line, insns);
    s = tab1 + 1;
    s += strspn(s, " ");
    tab2 = strchr(s, '\t');
    if (!tab2)
    malformed_line(line, insns);
// tab2 = '\0';	/* Characters beyond tab2 aren't examined
    while (s < tab2) {
    if (sscanf(s, "%x", &b) == 1) {
    insn_buff[nb++] = (unsigned char) b;
    s += 3;
    } else
    break;
    }
// Decode an instruction
    ret = insn_decode(&insn, insn_buff, sizeof(insn_buff),
    x86_64 ? INSN_MODE_64 : INSN_MODE_32);
    if (ret < 0 || insn.length != nb) {
    warnings++;
    pr_warn("Found an x86 instruction decoder bug, "
    "please report this.\n", sym);
    pr_warn("%s", line);
    pr_warn("objdump says %d bytes, but insn_get_length() "
    "says %d\n", nb, insn.length);
    if (verbose)
    dump_insn(stderr, &insn);
    }
    }
    if (warnings)
    pr_warn("Decoded and checked %d instructions with %d "
    "failures\n", insns, warnings);
    else
    fprintf(stdout, "  %s: success: Decoded and checked %d"
    " instructions\n", prog, insns);
    return 0;
    }
