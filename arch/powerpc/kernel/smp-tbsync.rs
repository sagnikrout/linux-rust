//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/smp-tbsync.c
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
// Smp timebase synchronization for ppc.
//
// Copyright (C) 2003 Samuel Rydh (samuel@ibrium.se)
//

pub const NUM_ITER: c_int = 300;
    enum {
    kExit=0, kSetAndTest, kTest
    };
    static struct {
    volatile u64		tb;
    volatile u64		mark;
    volatile int		cmd;
    volatile int		handshake;
    int			filler[2];
    volatile int		ack;
    int			filler2[7];
    volatile int		race_result;
    } *tbsync;
    static volatile int		running;
#[no_mangle]
unsafe extern "C" fn enter_contest(mark: u64, add: c_long) {
    static void enter_contest(u64 mark, long add)
    {
    while (get_tb() < mark)
    tbsync.race_result = add;
    }
#[no_mangle]
pub unsafe extern "C" fn smp_generic_take_timebase() {
    void smp_generic_take_timebase(void)
    {
    int cmd;
    u64 tb;
    unsigned long flags;
    local_irq_save(flags);
    while (!running)
    barrier();
    rmb();
    for (;;) {
    tbsync.ack = 1;
    while (!tbsync.handshake)
    barrier();
    rmb();
    cmd = tbsync.cmd;
    tb = tbsync.tb;
    mb();
    tbsync.ack = 0;
    if (cmd == kExit)
    break;
    while (tbsync.handshake)
    barrier();
    if (cmd == kSetAndTest)
    set_tb(tb >> 32, tb & 0xfffffffful);
    enter_contest(tbsync.mark, -1);
    }
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn start_contest(cmd: c_int, offset: c_long, num: c_int) -> c_int {
    static int start_contest(int cmd, long offset, int num)
    {
    int i, score=0;
    u64 tb;
    u64 mark;
    tbsync.cmd = cmd;
    local_irq_disable();
    for (i = -3; i < num; ) {
    tb = get_tb() + 400;
    tbsync.tb = tb + offset;
    tbsync.mark = mark = tb + 400;
    wmb();
    tbsync.handshake = 1;
    while (tbsync.ack)
    barrier();
    while (get_tb() <= tb)
    barrier();
    tbsync.handshake = 0;
    enter_contest(mark, 1);
    while (!tbsync.ack)
    barrier();
    if (i++ > 0)
    score += tbsync.race_result;
    }
    local_irq_enable();
    return score;
    }
#[no_mangle]
pub unsafe extern "C" fn smp_generic_give_timebase() {
    void smp_generic_give_timebase(void)
    {
    int i, score, score2, old, min=0, max=5000, offset=1000;
    pr_debug("Software timebase sync\n");
// if this fails then this kernel won't work anyway...
    tbsync = kzalloc_obj(*tbsync);
    mb();
    running = 1;
    while (!tbsync.ack)
    barrier();
    pr_debug("Got ack\n");
// binary search
    for (old = -1; old != offset ; offset = (min+max) / 2) {
    score = start_contest(kSetAndTest, offset, NUM_ITER);
    pr_debug("score %d, offset %d\n", score, offset );
    if( score > 0 )
    max = offset;
    else
    min = offset;
    old = offset;
    }
    score = start_contest(kSetAndTest, min, NUM_ITER);
    score2 = start_contest(kSetAndTest, max, NUM_ITER);
    pr_debug("Min %d (score %d), Max %d (score %d)\n",
    min, score, max, score2);
    score = abs(score);
    score2 = abs(score2);
    offset = (score < score2) ? min : max;
// guard against inaccurate mttb
    for (i = 0; i < 10; i++) {
    start_contest(kSetAndTest, offset, NUM_ITER/10);
    if ((score2 = start_contest(kTest, offset, NUM_ITER)) < 0)
    score2 = -score2;
    if (score2 <= score || score2 < 20)
    break;
    }
    pr_debug("Final offset: %d (%d/%d)\n", offset, score2, NUM_ITER );
// exiting
    tbsync.cmd = kExit;
    wmb();
    tbsync.handshake = 1;
    while (tbsync.ack)
    barrier();
    tbsync.handshake = 0;
    kfree(tbsync);
    tbsync = core::ptr::null_mut();
    running = 0;
    }
