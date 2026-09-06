//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/mem2node.c
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

    static struct node {
    int		 node;
    const char 	*map;
    } test_nodes[] = {
    { .node = 0, .map = "0"     },
    { .node = 1, .map = "1-2"   },
    { .node = 3, .map = "5-7,9" },
    };

    static unsigned long *get_bitmap(const char *str, int nbits)
    {
    struct perf_cpu_map *map = perf_cpu_map__new(str);
    unsigned long *bm = core::ptr::null_mut();
    bm = bitmap_zalloc(nbits);
    if (map && bm) {
    struct perf_cpu cpu;
    unsigned int i;
    perf_cpu_map__for_each_cpu(cpu, i, map)
    __set_bit(cpu.cpu, bm);
    }
    if (map)
    perf_cpu_map__put(map);
    else
    free(bm);
    return bm && map ? bm : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test__mem2node(__maybe_unused: *mut *mut test_suite t, __maybe_unused: int subtest) -> c_int {
    static int test__mem2node(struct test_suite *t __maybe_unused, int subtest __maybe_unused)
    {
    struct mem2node map;
    struct memory_node nodes[3];
    struct perf_env env = {
    .memory_nodes    = (struct memory_node *) &nodes[0],
    .nr_memory_nodes = ARRAY_SIZE(nodes),
    .memory_bsize    = 0x100,
    };
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(nodes); i++) {
    nodes[i].node = test_nodes[i].node;
    nodes[i].size = 10;
    T("failed: alloc bitmap",
    (nodes[i].set = get_bitmap(test_nodes[i].map, 10)));
    }
    T("failed: mem2node__init", !mem2node__init(&map, &env));
    T("failed: mem2node__node",  0 == mem2node__node(&map,   0x50));
    T("failed: mem2node__node",  1 == mem2node__node(&map,  0x100));
    T("failed: mem2node__node",  1 == mem2node__node(&map,  0x250));
    T("failed: mem2node__node",  3 == mem2node__node(&map,  0x500));
    T("failed: mem2node__node",  3 == mem2node__node(&map,  0x650));
    T("failed: mem2node__node", -1 == mem2node__node(&map,  0x450));
    T("failed: mem2node__node", -1 == mem2node__node(&map, 0x1050));
    for (i = 0; i < ARRAY_SIZE(nodes); i++)
    zfree(&nodes[i].set);
    mem2node__exit(&map);
    return 0;
    }
    DEFINE_SUITE("mem2node", mem2node);
