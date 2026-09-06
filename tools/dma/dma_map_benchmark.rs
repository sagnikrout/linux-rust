//! Automatically rewritten from C to Rust
//! Source: tools/dma/dma_map_benchmark.c
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
// Copyright (C) 2020 HiSilicon Limited.
//

    static char *directions[] = {
    "BIDIRECTIONAL",
    "TO_DEVICE",
    "FROM_DEVICE",
    };
    static char *mode[] = {
    "SINGLE_MODE",
    "SG_MODE",
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct map_benchmark map;
    int fd, opt;
// default single thread, run 20 seconds on NUMA_NO_NODE
    let mut threads: c_int = 1, seconds = 20, node = -1;
// default single map mode
    let mut map_mode: c_int = DMA_MAP_BENCH_SINGLE_MODE;
// default dma mask 32bit, bidirectional DMA
    let mut bits: c_int = 32, xdelay = 0, dir = DMA_MAP_BIDIRECTIONAL;
// default granule 1 PAGESIZE
    let mut granule: c_int = 1;
    let mut cmd: c_int = DMA_MAP_BENCHMARK;
    while ((opt = getopt(argc, argv, "t:s:n:b:d:x:g:m:")) != -1) {
    switch (opt) {
    case 't':
    threads = atoi(optarg);
    break;
    case 's':
    seconds = atoi(optarg);
    break;
    case 'n':
    node = atoi(optarg);
    break;
    case 'b':
    bits = atoi(optarg);
    break;
    case 'd':
    dir = atoi(optarg);
    break;
    case 'x':
    xdelay = atoi(optarg);
    break;
    case 'g':
    granule = atoi(optarg);
    break;
    case 'm':
    map_mode = atoi(optarg);
    break;
    default:
    return -1;
    }
    }
    if (map_mode < 0 || map_mode >= DMA_MAP_BENCH_MODE_MAX) {
    fprintf(stderr, "invalid map mode, SINGLE_MODE:%d, SG_MODE: %d\n",
    DMA_MAP_BENCH_SINGLE_MODE, DMA_MAP_BENCH_SG_MODE);
    exit(1);
    }
    if (threads <= 0 || threads > DMA_MAP_MAX_THREADS) {
    fprintf(stderr, "invalid number of threads, must be in 1-%d\n",
    DMA_MAP_MAX_THREADS);
    exit(1);
    }
    if (seconds <= 0 || seconds > DMA_MAP_MAX_SECONDS) {
    fprintf(stderr, "invalid number of seconds, must be in 1-%d\n",
    DMA_MAP_MAX_SECONDS);
    exit(1);
    }
    if (xdelay < 0 || xdelay > DMA_MAP_MAX_TRANS_DELAY) {
    fprintf(stderr, "invalid transmit delay, must be in 0-%ld\n",
    DMA_MAP_MAX_TRANS_DELAY);
    exit(1);
    }
// suppose the mininum DMA zone is 1MB in the world
    if (bits < 20 || bits > 64) {
    fprintf(stderr, "invalid dma mask bit, must be in 20-64\n");
    exit(1);
    }
    if (dir != DMA_MAP_BIDIRECTIONAL && dir != DMA_MAP_TO_DEVICE &&
    dir != DMA_MAP_FROM_DEVICE) {
    fprintf(stderr, "invalid dma direction\n");
    exit(1);
    }
    if (granule < 1 || granule > 1024) {
    fprintf(stderr, "invalid granule size\n");
    exit(1);
    }
    fd = open("/sys/kernel/debug/dma_map_benchmark", O_RDWR);
    if (fd == -1) {
    perror("open");
    exit(1);
    }
    memset(&map, 0, sizeof(map));
    map.seconds = seconds;
    map.threads = threads;
    map.node = node;
    map.dma_bits = bits;
    map.dma_dir = dir;
    map.dma_trans_ns = xdelay;
    map.granule = granule;
    map.map_mode = map_mode;
    if (ioctl(fd, cmd, &map)) {
    perror("ioctl");
    exit(1);
    }
    printf("dma mapping benchmark(%s): threads:%d seconds:%d node:%d dir:%s granule:%d\n",
    mode[map_mode], threads, seconds, node, directions[dir], granule);
    printf("average map latency(us):%.1f standard deviation:%.1f\n",
    map.avg_map_100ns/10.0, map.map_stddev/10.0);
    printf("average unmap latency(us):%.1f standard deviation:%.1f\n",
    map.avg_unmap_100ns/10.0, map.unmap_stddev/10.0);
    return 0;
    }
