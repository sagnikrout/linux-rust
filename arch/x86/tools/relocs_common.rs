//! Automatically rewritten from C to Rust
//! Source: arch/x86/tools/relocs_common.c
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

#[no_mangle]
pub unsafe extern "C" fn die(fmt: *mut c_char, ...) {
    void die(char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    die("relocs [--abs-syms|--abs-relocs|--reloc-info|--text|--realmode]" \
    " vmlinux\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int show_absolute_syms, show_absolute_relocs, show_reloc_info;
    int as_text, use_real_mode;
    const char *fname;
    FILE *fp;
    int i;
    unsigned char e_ident[EI_NIDENT];
    show_absolute_syms = 0;
    show_absolute_relocs = 0;
    show_reloc_info = 0;
    as_text = 0;
    use_real_mode = 0;
    fname = core::ptr::null_mut();
    for (i = 1; i < argc; i++) {
    char *arg = argv[i];
    if (*arg == '-') {
    if (strcmp(arg, "--abs-syms") == 0) {
    show_absolute_syms = 1;
    continue;
    }
    if (strcmp(arg, "--abs-relocs") == 0) {
    show_absolute_relocs = 1;
    continue;
    }
    if (strcmp(arg, "--reloc-info") == 0) {
    show_reloc_info = 1;
    continue;
    }
    if (strcmp(arg, "--text") == 0) {
    as_text = 1;
    continue;
    }
    if (strcmp(arg, "--realmode") == 0) {
    use_real_mode = 1;
    continue;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !fname) -> else {
    fname = arg;
    continue;
    }
    usage();
    }
    if (!fname) {
    usage();
    }
    fp = fopen(fname, "r");
    if (!fp) {
    die("Cannot open %s: %s\n", fname, strerror(errno));
    }
    if (fread(&e_ident, 1, EI_NIDENT, fp) != EI_NIDENT) {
    die("Cannot read %s: %s", fname, strerror(errno));
    }
    rewind(fp);
    if (e_ident[EI_CLASS] == ELFCLASS64)
    process_64(fp, use_real_mode, as_text,
    show_absolute_syms, show_absolute_relocs,
    show_reloc_info);
    else
    process_32(fp, use_real_mode, as_text,
    show_absolute_syms, show_absolute_relocs,
    show_reloc_info);
    fclose(fp);
    return 0;
    }
