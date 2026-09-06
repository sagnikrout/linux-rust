//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/queue_stack_map.c
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

    enum {
    QUEUE,
    STACK,
    };
#[no_mangle]
unsafe extern "C" fn test_queue_stack_map_by_type(type: c_int) {
    static void test_queue_stack_map_by_type(int type)
    {
    let mut MAP_SIZE: c_int = 32;
    __u32 vals[MAP_SIZE], val;
    int i, err, prog_fd, map_in_fd, map_out_fd;
    char file[32], buf[128];
    struct bpf_object *obj;
    let mut iph: iphdr = {};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 1,
    );
// Fill test values to be used
    for (i = 0; i < MAP_SIZE; i++)
    vals[i] = rand();
    if (type == QUEUE)
    strscpy(file, "./test_queue_map.bpf.o");
#[no_mangle]
pub unsafe extern "C" fn if(STACK: type ==) -> else {
    else if (type == STACK)
    strscpy(file, "./test_stack_map.bpf.o");
    else
    return;
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, &obj, &prog_fd);
    if (CHECK_FAIL(err))
    return;
    map_in_fd = bpf_find_map(__func__, obj, "map_in");
    if (map_in_fd < 0)
    goto out;
    map_out_fd = bpf_find_map(__func__, obj, "map_out");
    if (map_out_fd < 0)
    goto out;
// Push 32 elements to the input map
    for (i = 0; i < MAP_SIZE; i++) {
    err = bpf_map_update_elem(map_in_fd, core::ptr::null_mut(), &vals[i], 0);
    if (CHECK_FAIL(err))
    goto out;
    }
// The eBPF program pushes iph.saddr in the output map,
// pops the input map and saves this value in iph.daddr
//
    for (i = 0; i < MAP_SIZE; i++) {
    if (type == QUEUE) {
    val = vals[i];
    pkt_v4.iph.saddr = vals[i] * 5;
    } else if (type == STACK) {
    val = vals[MAP_SIZE - 1 - i];
    pkt_v4.iph.saddr = vals[MAP_SIZE - 1 - i] * 5;
    }
    topts.data_size_out = sizeof(buf);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (err || topts.retval ||
    topts.data_size_out != sizeof(pkt_v4))
    break;
    memcpy(&iph, buf + sizeof(struct ethhdr), sizeof(iph));
    if (iph.daddr != val)
    break;
    }
    ASSERT_OK(err, "bpf_map_pop_elem");
    ASSERT_OK(topts.retval, "bpf_map_pop_elem test retval");
    ASSERT_EQ(topts.data_size_out, sizeof(pkt_v4),
    "bpf_map_pop_elem data_size_out");
    ASSERT_EQ(iph.daddr, val, "bpf_map_pop_elem iph.daddr");
// Queue is empty, program should return TC_ACT_SHOT
    topts.data_size_out = sizeof(buf);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "check-queue-stack-map-empty");
    ASSERT_EQ(topts.retval, 2  /* TC_ACT_SHOT */,
    "check-queue-stack-map-empty test retval");
    ASSERT_EQ(topts.data_size_out, sizeof(pkt_v4),
    "check-queue-stack-map-empty data_size_out");
// Check that the program pushed elements correctly
    for (i = 0; i < MAP_SIZE; i++) {
    err = bpf_map_lookup_and_delete_elem(map_out_fd, core::ptr::null_mut(), &val);
    ASSERT_OK(err, "bpf_map_lookup_and_delete_elem");
    ASSERT_EQ(val, vals[i] * 5, "bpf_map_push_elem val");
    }
    out:
    pkt_v4.iph.saddr = 0;
    bpf_object__close(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_queue_stack_map() {
    void test_queue_stack_map(void)
    {
    test_queue_stack_map_by_type(QUEUE);
    test_queue_stack_map_by_type(STACK);
    }
