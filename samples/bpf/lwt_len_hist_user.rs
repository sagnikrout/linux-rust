//! Automatically rewritten from C to Rust
//! Source: samples/bpf/lwt_len_hist_user.c
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

pub const MAX_INDEX: c_int = 64;
pub const MAX_STARS: c_int = 38;
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
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut nr_cpus: c_uint = bpf_num_possible_cpus();
    const char *map_filename = "/sys/fs/bpf/tc/globals/lwt_len_hist_map";
    uint64_t values[nr_cpus], sum, max_value = 0, data[MAX_INDEX] = {};
    let mut key: u64 = 0, next_key, max_key = 0;
    char starstr[MAX_STARS];
    int i, map_fd;
    map_fd = bpf_obj_get(map_filename);
    if (map_fd < 0) {
    fprintf(stderr, "bpf_obj_get(%s): %s(%d)\n",
    map_filename, strerror(errno), errno);
    return -1;
    }
    while (bpf_map_get_next_key(map_fd, &key, &next_key) == 0) {
    if (next_key >= MAX_INDEX) {
    fprintf(stderr, "Key %lu out of bounds\n", next_key);
    continue;
    }
    bpf_map_lookup_elem(map_fd, &next_key, values);
    sum = 0;
    for (i = 0; i < nr_cpus; i++)
    sum += values[i];
    data[next_key] = sum;
    if (sum && next_key > max_key)
    max_key = next_key;
    if (sum > max_value)
    max_value = sum;
    key = next_key;
    }
    for (i = 1; i <= max_key + 1; i++) {
    stars(starstr, data[i - 1], max_value, MAX_STARS);
    printf("%8ld . %-8ld : %-8ld |%-*s|\n",
    (1l << i) >> 1, (1l << i) - 1, data[i - 1],
    MAX_STARS, starstr);
    }
    close(map_fd);
    return 0;
    }
