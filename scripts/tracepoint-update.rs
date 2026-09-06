//! Automatically rewritten from C to Rust
//! Source: scripts/tracepoint-update.c
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

    static Elf_Shdr *check_data_sec;
    static Elf_Shdr *tracepoint_data_sec;
    static inline void *get_index(void *start, int entsize, int index)
    {
    return start + (entsize * index);
    }
#[no_mangle]
unsafe extern "C" fn compare_strings(a: *const c_void, b: *const c_void) -> c_int {
    static int compare_strings(const void *a, const void *b)
    {
    const char *av = *(const char **)a;
    const char *bv = *(const char **)b;
    return strcmp(av, bv);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_tracepoint {
    pub ehdr: *mut Elf_Ehdr,
    pub array: *const c_char,
    pub count: c_int,
}

#[no_mangle]
unsafe extern "C" fn add_string(str: *const c_char, vals: *const c_char, count: *mut c_int) -> c_int {
    static int add_string(const char *str, const char ***vals, int *count)
    {
    const char **array = *vals;
    if (!(*count & REALLOC_MASK)) {
    let mut size: c_int = (*count) + REALLOC_SIZE;
    array = realloc(array, sizeof(char *) * size);
    if (!array) {
    fprintf(stderr, "Failed memory allocation\n");
    free(*vals);
// vals = NULL;
    return -1;
    }
// vals = array;
    }
    array[(*count)++] = str;
    return 0;
    }
//
// for_each_shdr_str - iterator that reads strings that are in an ELF section.
// @len: "int" to hold the length of the current string
// @ehdr: A pointer to the ehdr of the ELF file
// @sec: The section that has the strings to iterate on
//
// This is a for loop that iterates over all the nul terminated strings
// that are in a given ELF section. The variable "str" will hold
// the current string for each iteration and the passed in @len will
// contain the strlen() of that string.
//

    for (const char *str = (void *)(ehdr) + shdr_offset(sec),	\
// end = str + shdr_size(sec);			\
    len = strlen(str), str < end;				\
    str += (len) + 1)
#[no_mangle]
unsafe extern "C" fn make_trace_array(etrace: *mut elf_tracepoint) {
    static void make_trace_array(struct elf_tracepoint *etrace)
    {
    Elf_Ehdr *ehdr = etrace.ehdr;
    const char **vals = core::ptr::null_mut();
    let mut count: c_int = 0;
    int len;
    etrace.array = core::ptr::null_mut();
//
// The __tracepoint_check section is filled with strings of the
// names of tracepoints (in tracepoint_strings). Create an array
// that points to each string and then sort the array.
//
    for_each_shdr_str(len, ehdr, check_data_sec) {
    if (!len)
    continue;
    if (add_string(str, &vals, &count) < 0)
    return;
    }
// If CONFIG_TRACEPOINT_VERIFY_USED is not set, there's nothing to do
    if (!count)
    return;
    qsort(vals, count, sizeof(char *), compare_strings);
    etrace.array = vals;
    etrace.count = count;
    }
#[no_mangle]
unsafe extern "C" fn find_event(str: *const c_char, array: *mut c_void, size: usize) -> c_int {
    static int find_event(const char *str, void *array, size_t size)
    {
    return bsearch(&str, array, size, sizeof(char *), compare_strings) != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn check_tracepoints(etrace: *mut elf_tracepoint, fname: *const c_char) {
    static void check_tracepoints(struct elf_tracepoint *etrace, const char *fname)
    {
    Elf_Ehdr *ehdr = etrace.ehdr;
    int len;
    if (!etrace.array)
    return;
//
// The __tracepoints_strings section holds all the names of the
// defined tracepoints. If any of them are not in the
// __tracepoint_check_section it means they are not used.
//
    for_each_shdr_str(len, ehdr, tracepoint_data_sec) {
    if (!len)
    continue;
    if (!find_event(str, etrace.array, etrace.count)) {
    fprintf(stderr, "warning: tracepoint '%s' is unused", str);
    if (fname)
    fprintf(stderr, " in module %s\n", fname);
    else
    fprintf(stderr, "\n");
    }
    }
    free(etrace.array);
    }
    static void *tracepoint_check(struct elf_tracepoint *etrace, const char *fname)
    {
    make_trace_array(etrace);
    check_tracepoints(etrace, fname);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn process_tracepoints(mod: bool, addr: *mut c_void, fname: *const c_char) -> c_int {
    static int process_tracepoints(bool mod, void *addr, const char *fname)
    {
    let mut etrace: elf_tracepoint = {0};
    Elf_Ehdr *ehdr = addr;
    Elf_Shdr *shdr_start;
    Elf_Shdr *string_sec;
    const char *secstrings;
    unsigned int shnum;
    unsigned int shstrndx;
    int shentsize;
    int idx;
    let mut done: c_int = 2;
    shdr_start = (Elf_Shdr *)((char *)ehdr + ehdr_shoff(ehdr));
    shentsize = ehdr_shentsize(ehdr);
    shstrndx = ehdr_shstrndx(ehdr);
    if (shstrndx == SHN_XINDEX)
    shstrndx = shdr_link(shdr_start);
    string_sec = get_index(shdr_start, shentsize, shstrndx);
    secstrings = (const char *)ehdr + shdr_offset(string_sec);
    shnum = ehdr_shnum(ehdr);
    if (shnum == SHN_UNDEF)
    shnum = shdr_size(shdr_start);
    for (int i = 0; done && i < shnum; i++) {
    Elf_Shdr *shdr = get_index(shdr_start, shentsize, i);
    idx = shdr_name(shdr);
// locate the __tracepoint_check in vmlinux
    if (!strcmp(secstrings + idx, "__tracepoint_check")) {
    check_data_sec = shdr;
    done--;
    }
// locate the __tracepoints_ptrs section in vmlinux
    if (!strcmp(secstrings + idx, "__tracepoints_strings")) {
    tracepoint_data_sec = shdr;
    done--;
    }
    }
//
// Modules may not have either section. But if it has one section,
// it should have both of them.
//
    if (mod && !check_data_sec && !tracepoint_data_sec)
    return 0;
    if (!check_data_sec) {
    if (mod) {
    fprintf(stderr, "warning: Module %s has only unused tracepoints\n", fname);
// Do not fail build
    return 0;
    }
    fprintf(stderr,	"no __tracepoint_check in file: %s\n", fname);
    return -1;
    }
    if (!tracepoint_data_sec) {
// A module may reference only exported tracepoints
    if (mod)
    return 0;
    fprintf(stderr,	"no __tracepoint_strings in file: %s\n", fname);
    return -1;
    }
    if (!mod)
    fname = core::ptr::null_mut();
    etrace.ehdr = ehdr;
    tracepoint_check(&etrace, fname);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut n_error: c_int = 0;
    let mut size: usize = 0;
    void *addr = core::ptr::null_mut();
    let mut mod: bool = false;
    if (argc > 1 && strcmp(argv[1], "--module") == 0) {
    mod = true;
    argc--;
    argv++;
    }
    if (argc < 2) {
    if (mod)
    fprintf(stderr, "usage: tracepoint-update --module module...\n");
    else
    fprintf(stderr, "usage: tracepoint-update vmlinux...\n");
    return 0;
    }
// Process each file in turn, allowing deep failure.
    for (int i = 1; i < argc; i++) {
    addr = elf_map(argv[i], &size, 1 << ET_REL);
    if (!addr) {
    ++n_error;
    continue;
    }
    if (process_tracepoints(mod, addr, argv[i]))
    ++n_error;
    elf_unmap(addr, size);
    }
    return !!n_error;
    }
