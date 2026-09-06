//! Automatically rewritten from C to Rust
//! Source: tools/build/fixdep.c
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
// "Optimize" a list of dependencies as spit out by gcc -MD
// for the build framework.
//
// Original author:
// Copyright    2002 by Kai Germaschewski  <kai.germaschewski@gmx.de>
//
// This code has been borrowed from kbuild's fixdep (scripts/basic/fixdep.c),
// Please check it for detailed explanation. This fixdep borow only the
// base transformation of dependecies without the CONFIG mangle.
//

    char *target;
    char *depfile;
    char *cmdline;
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    fprintf(stderr, "Usage: fixdep <depfile> <target> <cmdline>\n");
    exit(1);
    }
//
// Print out the commandline prefixed with cmd_<target filename> :=
//
#[no_mangle]
unsafe extern "C" fn print_cmdline() {
    static void print_cmdline(void)
    {
    printf("cmd_%s := %s\n\n", target, cmdline);
    }
//
// Important: The below generated source_foo.o and deps_foo.o variable
// assignments are parsed not only by make, but also by the rather simple
// parser in scripts/mod/sumversion.c.
//
#[no_mangle]
unsafe extern "C" fn parse_dep_file(map: *mut c_void, len: usize) {
    static void parse_dep_file(void *map, size_t len)
    {
    char *m = map;
    char *end = m + len;
    char *p;
    char s[PATH_MAX];
    int is_target, has_target = 0;
    let mut saw_any_target: c_int = 0;
    let mut is_first_dep: c_int = 0;
    while (m < end) {
// Skip any "white space"
    while (m < end && (*m == ' ' || *m == '\\' || *m == '\n'))
    m++;
// Find next "white space"
    p = m;
    while (p < end && *p != ' ' && *p != '\\' && *p != '\n')
    p++;
// Is the token we found a target name?
    is_target = (*(p-1) == ':');
// Don't write any target names into the dependency file
    if (is_target) {
// The /next/ file is the first dependency
    is_first_dep = 1;
    has_target = 1;
    } else if (has_target) {
// Save this token/filename
    memcpy(s, m, p-m);
    s[p - m] = 0;
//
// Do not list the source file as dependency,
// so that kbuild is not confused if a .c file
// is rewritten into .S or vice versa. Storing
// it in source_* is needed for modpost to
// compute srcversions.
//
    if (is_first_dep) {
//
// If processing the concatenation of
// multiple dependency files, only
// process the first target name, which
// will be the original source name,
// and ignore any other target names,
// which will be intermediate temporary
// files.
//
    if (!saw_any_target) {
    saw_any_target = 1;
    printf("source_%s := %s\n\n",
    target, s);
    printf("deps_%s := \\\n",
    target);
    }
    is_first_dep = 0;
    } else
    printf("  %s \\\n", s);
    }
//
// Start searching for next token immediately after the first
// "whitespace" character that follows this token.
//
    m = p + 1;
    }
    if (!saw_any_target) {
    fprintf(stderr, "fixdep: parse error; no targets found\n");
    exit(1);
    }
    printf("\n%s: $(deps_%s)\n\n", target, target);
    printf("$(deps_%s):\n", target);
    }
#[no_mangle]
unsafe extern "C" fn print_deps() {
    static void print_deps(void)
    {
    struct stat st;
    int fd;
    void *map;
    fd = open(depfile, O_RDONLY);
    if (fd < 0) {
    fprintf(stderr, "fixdep: error opening depfile: ");
    perror(depfile);
    exit(2);
    }
    if (fstat(fd, &st) < 0) {
    fprintf(stderr, "fixdep: error fstat'ing depfile: ");
    perror(depfile);
    exit(2);
    }
    if (st.st_size == 0) {
    fprintf(stderr, "fixdep: %s is empty\n", depfile);
    close(fd);
    return;
    }
    map = mmap(core::ptr::null_mut(), st.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if ((long) map == -1) {
    perror("fixdep: mmap");
    close(fd);
    return;
    }
    parse_dep_file(map, st.st_size);
    munmap(map, st.st_size);
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    if (argc != 4)
    usage();
    depfile = argv[1];
    target  = argv[2];
    cmdline = argv[3];
    print_cmdline();
    print_deps();
    return 0;
    }
