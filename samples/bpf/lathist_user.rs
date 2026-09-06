//! Automatically rewritten from C to Rust
//! Source: samples/bpf/lathist_user.c
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
// Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
// Copyright (c) 2015 BMW Car IT GmbH
//

pub const MAX_ENTRIES: c_int = 20;
pub const MAX_CPU: c_int = 4;
pub const MAX_STARS: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_hist {
    pub data: [c_long; MAX_ENTRIES],
    pub max: c_long,
}

    static struct cpu_hist cpu_hist[MAX_CPU];
#[no_mangle]
unsafe extern "C" fn stars(str: *mut c_char, val: c_long, max: c_long, width: c_int) {
    static void stars(char *str, long val, long max, int width)
    {
    int i;
    for (i = 0; i < (width * val / max) - 1 && i < width - 1; i++)
    str[i] = '*';
    if (val > max)
    str[i - 1] = '+';
    str[i] = '\0';
    }
#[no_mangle]
unsafe extern "C" fn print_hist() {
    static void print_hist(void)
    {
    char starstr[MAX_STARS];
    struct cpu_hist *hist;
    int i, j;
// clear screen
    printf("\033[2J");
    for (j = 0; j < MAX_CPU; j++) {
    hist = &cpu_hist[j];
// ignore CPUs without data (maybe offline?)
    if (hist.max == 0)
    continue;
    printf("CPU %d\n", j);
    printf("      latency        : count     distribution\n");
    for (i = 1; i <= MAX_ENTRIES; i++) {
    stars(starstr, hist.data[i - 1], hist.max, MAX_STARS);
    printf("%8ld . %-8ld : %-8ld |%-*s|\n",
    (1l << i) >> 1, (1l << i) - 1,
    hist.data[i - 1], MAX_STARS, starstr);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn get_data(fd: c_int) {
    static void get_data(int fd)
    {
    long key, value;
    int c, i;
    for (i = 0; i < MAX_CPU; i++)
    cpu_hist[i].max = 0;
    for (c = 0; c < MAX_CPU; c++) {
    for (i = 0; i < MAX_ENTRIES; i++) {
    key = c * MAX_ENTRIES + i;
    bpf_map_lookup_elem(fd, &key, &value);
    cpu_hist[c].data[i] = value;
    if (value > cpu_hist[c].max)
    cpu_hist[c].max = value;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct bpf_link *links[2];
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    int map_fd, i = 0;
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    map_fd = bpf_object__find_map_fd_by_name(obj, "my_lat");
    if (map_fd < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    bpf_object__for_each_program(prog, obj) {
    links[i] = bpf_program__attach(prog);
    if (libbpf_get_error(links[i])) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    links[i] = core::ptr::null_mut();
    goto cleanup;
    }
    i++;
    }
    while (1) {
    get_data(map_fd);
    print_hist();
    sleep(5);
    }
    cleanup:
    for (i--; i >= 0; i--)
    bpf_link__destroy(links[i]);
    bpf_object__close(obj);
    return 0;
    }
