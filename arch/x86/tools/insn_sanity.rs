//! Automatically rewritten from C to Rust
//! Source: arch/x86/tools/insn_sanity.c
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
// x86 decoder sanity test - based on test_get_insn.c
//
// Copyright (C) IBM Corporation, 2009
// Copyright (C) Hitachi, Ltd., 2011
//

//
// Test of instruction analysis against tampering.
// Feed random binary to instruction decoder and ensure not to
// access out-of-instruction-buffer.
//
pub const DEFAULT_MAX_ITER: c_int = 10000;
pub const INSN_NOP: c_uint = 0x90;
    static const char	*prog;		/* Program name */
    static int		verbose;	/* Verbosity */
    static int		x86_64;		/* x86-64 bit mode flag */
    static unsigned int	seed;		/* Random seed */
    static unsigned long	iter_start;	/* Start of iteration number */
    static unsigned long	iter_end = DEFAULT_MAX_ITER;	/* End of iteration number */
    static FILE		*input_file;	/* Input file name */
#[no_mangle]
unsafe extern "C" fn usage(err: *const c_char) {
    static void usage(const char *err)
    {
    if (err)
    fprintf(stderr, "%s: Error: %s\n\n", prog, err);
    fprintf(stderr, "Usage: %s [-y|-n|-v] [-s seed[,no]] [-m max] [-i input]\n", prog);
    fprintf(stderr, "\t-y	64bit mode\n");
    fprintf(stderr, "\t-n	32bit mode\n");
    fprintf(stderr, "\t-v	Verbosity(-vv dumps any decoded result)\n");
    fprintf(stderr, "\t-s	Give a random seed (and iteration number)\n");
    fprintf(stderr, "\t-m	Give a maximum iteration number\n");
    fprintf(stderr, "\t-i	Give an input file with decoded binary\n");
    exit(1);
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
    static void dump_stream(FILE *fp, const char *msg, unsigned long nr_iter,
    unsigned char *insn_buff, struct insn *insn)
    {
    int i;
    fprintf(fp, "%s:\n", msg);
    dump_insn(fp, insn);
    fprintf(fp, "You can reproduce this with below command(s);\n");
// Input a decoded instruction sequence directly
    fprintf(fp, " $ echo ");
    for (i = 0; i < MAX_INSN_SIZE; i++)
    fprintf(fp, " %02x", insn_buff[i]);
    fprintf(fp, " | %s -i -\n", prog);
    if (!input_file) {
    fprintf(fp, "Or \n");
// Give a seed and iteration number
    fprintf(fp, " $ %s -s 0x%x,%lu\n", prog, seed, nr_iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_random_seed() {
    static void init_random_seed(void)
    {
    int fd;
    fd = open("/dev/urandom", O_RDONLY);
    if (fd < 0)
    goto fail;
    if (read(fd, &seed, sizeof(seed)) != sizeof(seed))
    goto fail;
    close(fd);
    return;
    fail:
    usage("Failed to open /dev/urandom");
    }
// Read given instruction sequence from the input file
#[no_mangle]
unsafe extern "C" fn read_next_insn(insn_buff: *mut c_uchar) -> c_int {
    static int read_next_insn(unsigned char *insn_buff)
    {
    char buf[256]  = "", *tmp;
    int i;
    tmp = fgets(buf, ARRAY_SIZE(buf), input_file);
    if (tmp == core::ptr::null_mut() || feof(input_file))
    return 0;
    for (i = 0; i < MAX_INSN_SIZE; i++) {
    insn_buff[i] = (unsigned char)strtoul(tmp, &tmp, 16);
    if (*tmp != ' ')
    break;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn generate_insn(insn_buff: *mut c_uchar) -> c_int {
    static int generate_insn(unsigned char *insn_buff)
    {
    int i;
    if (input_file)
    return read_next_insn(insn_buff);
// Fills buffer with random binary up to MAX_INSN_SIZE
    for (i = 0; i < MAX_INSN_SIZE - 1; i += 2)
// (unsigned short *)(&insn_buff[i]) = random() & 0xffff;
    while (i < MAX_INSN_SIZE)
    insn_buff[i++] = random() & 0xff;
    return i;
    }
#[no_mangle]
unsafe extern "C" fn parse_args(argc: c_int, argv: *mut c_char) {
    static void parse_args(int argc, char **argv)
    {
    int c;
    char *tmp = core::ptr::null_mut();
    let mut set_seed: c_int = 0;
    prog = argv[0];
    while ((c = getopt(argc, argv, "ynvs:m:i:")) != -1) {
    switch (c) {
    case 'y':
    x86_64 = 1;
    break;
    case 'n':
    x86_64 = 0;
    break;
    case 'v':
    verbose++;
    break;
    case 'i':
    if (strcmp("-", optarg) == 0)
    input_file = stdin;
    else
    input_file = fopen(optarg, "r");
    if (!input_file)
    usage("Failed to open input file");
    break;
    case 's':
    seed = (unsigned int)strtoul(optarg, &tmp, 0);
    if (*tmp == ',') {
    optarg = tmp + 1;
    iter_start = strtoul(optarg, &tmp, 0);
    }
    if (*tmp != '\0' || tmp == optarg)
    usage("Failed to parse seed");
    set_seed = 1;
    break;
    case 'm':
    iter_end = strtoul(optarg, &tmp, 0);
    if (*tmp != '\0' || tmp == optarg)
    usage("Failed to parse max_iter");
    break;
    default:
    usage(core::ptr::null_mut());
    }
    }
// Check errors
    if (iter_end < iter_start)
    usage("Max iteration number must be bigger than iter-num");
    if (set_seed && input_file)
    usage("Don't use input file (-i) with random seed (-s)");
// Initialize random seed
    if (!input_file) {
    if (!set_seed)	/* No seed is given */
    init_random_seed();
    srand(seed);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut insns: c_int = 0, ret;
    struct insn insn;
    let mut errors: c_int = 0;
    unsigned long i;
    unsigned char insn_buff[MAX_INSN_SIZE * 2];
    parse_args(argc, argv);
// Prepare stop bytes with NOPs
    memset(insn_buff + MAX_INSN_SIZE, INSN_NOP, MAX_INSN_SIZE);
    for (i = 0; i < iter_end; i++) {
    if (generate_insn(insn_buff) <= 0)
    break;
    if (i < iter_start)	/* Skip to given iteration number */
    continue;
// Decode an instruction
    ret = insn_decode(&insn, insn_buff, sizeof(insn_buff),
    x86_64 ? INSN_MODE_64 : INSN_MODE_32);
    if (insn.next_byte <= insn.kaddr ||
    insn.kaddr + MAX_INSN_SIZE < insn.next_byte) {
// Access out-of-range memory
    dump_stream(stderr, "Error: Found an access violation", i, insn_buff, &insn);
    errors++;
    } else if (verbose && ret < 0)
    dump_stream(stdout, "Info: Found an undecodable input", i, insn_buff, &insn);
#[no_mangle]
pub unsafe extern "C" fn if(2: verbose >=) -> else {
    else if (verbose >= 2)
    dump_insn(stdout, &insn);
    insns++;
    }
    fprintf((errors) ? stderr : stdout,
    "  %s: %s: Decoded and checked %d %s instructions with %d errors (seed:0x%x)\n",
    prog,
    (errors) ? "failure" : "success",
    insns,
    (input_file) ? "given" : "random",
    errors,
    seed);
    return errors ? 1 : 0;
    }
