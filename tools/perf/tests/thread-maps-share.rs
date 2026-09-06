//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/thread-maps-share.c
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
unsafe extern "C" fn test__thread_maps_share(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__thread_maps_share(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    struct machines machines;
    struct machine *machine;
// thread group
    struct thread *leader;
    struct thread *t1, *t2, *t3;
    struct maps *maps;
// other process
    struct thread *other, *other_leader;
    struct maps *other_maps;
//
// This test create 2 processes abstractions (struct thread)
// with several threads and checks they properly share and
// maintain maps info (struct maps).
//
// thread group (pid: 0, tids: 0, 1, 2, 3)
// other  group (pid: 4, tids: 4, 5)
//
    TEST_ASSERT_VAL("failed to init machines", machines__init(&machines) == 0);
    machine = &machines.host;
// create process with 4 threads
    leader = machine__findnew_thread(machine, 0, 0);
    t1     = machine__findnew_thread(machine, 0, 1);
    t2     = machine__findnew_thread(machine, 0, 2);
    t3     = machine__findnew_thread(machine, 0, 3);
// and create 1 separated process, without thread leader
    other  = machine__findnew_thread(machine, 4, 5);
    TEST_ASSERT_VAL("failed to create threads",
    leader && t1 && t2 && t3 && other);
    maps = thread__maps(leader);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(maps)), 4);
// test the maps pointer is shared
    TEST_ASSERT_VAL("maps don't match", maps__equal(maps, thread__maps(t1)));
    TEST_ASSERT_VAL("maps don't match", maps__equal(maps, thread__maps(t2)));
    TEST_ASSERT_VAL("maps don't match", maps__equal(maps, thread__maps(t3)));
//
// Verify the other leader was created by previous call.
// It should have shared maps with no change in
// refcnt.
//
    other_leader = machine__find_thread(machine, 4, 4);
    TEST_ASSERT_VAL("failed to find other leader", other_leader);
//
// Ok, now that all the rbtree related operations were done,
// lets remove all of them from there so that we can do the
// refcounting tests.
//
    machine__remove_thread(machine, leader);
    machine__remove_thread(machine, t1);
    machine__remove_thread(machine, t2);
    machine__remove_thread(machine, t3);
    machine__remove_thread(machine, other);
    machine__remove_thread(machine, other_leader);
    other_maps = thread__maps(other);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(other_maps)), 2);
    TEST_ASSERT_VAL("maps don't match", maps__equal(other_maps, thread__maps(other_leader)));
// release thread group
    thread__put(t3);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(maps)), 3);
    thread__put(t2);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(maps)), 2);
    thread__put(t1);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(maps)), 1);
    thread__put(leader);
// release other group
    thread__put(other_leader);
    TEST_ASSERT_EQUAL("wrong refcnt", refcount_read(maps__refcnt(other_maps)), 1);
    thread__put(other);
    machines__exit(&machines);
    return 0;
    }
    DEFINE_SUITE("Share thread maps", thread_maps_share);
