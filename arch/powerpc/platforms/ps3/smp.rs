//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/smp.c
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
// PS3 SMP routines.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
//

//
// ps3_ipi_virqs - a per cpu array of virqs for ipi use
//
pub const MSG_COUNT: c_int = 4;
    static DEFINE_PER_CPU(unsigned int [MSG_COUNT], ps3_ipi_virqs);
#[no_mangle]
unsafe extern "C" fn ps3_smp_message_pass(cpu: c_int, msg: c_int) {
    static void ps3_smp_message_pass(int cpu, int msg)
    {
    int result;
    unsigned int virq;
    if (msg >= MSG_COUNT) {
    DBG("%s:%d: bad msg: %d\n", __func__, __LINE__, msg);
    return;
    }
    virq = per_cpu(ps3_ipi_virqs, cpu)[msg];
    result = ps3_send_event_locally(virq);
    if (result)
    DBG("%s:%d: ps3_send_event_locally(%d, %d) failed"
    " (%d)\n", __func__, __LINE__, cpu, msg, result);
    }
#[no_mangle]
unsafe extern "C" fn ps3_smp_probe() -> void __init {
    static void __init ps3_smp_probe(void)
    {
    int cpu;
    for (cpu = 0; cpu < 2; cpu++) {
    int result;
    unsigned int *virqs = per_cpu(ps3_ipi_virqs, cpu);
    int i;
    DBG(" . %s:%d: (%d)\n", __func__, __LINE__, cpu);
//
// Check assumptions on ps3_ipi_virqs[] indexing. If this
// check fails, then a different mapping of PPC_MSG_
// to index needs to be setup.
//
    BUILD_BUG_ON(PPC_MSG_CALL_FUNCTION    != 0);
    BUILD_BUG_ON(PPC_MSG_RESCHEDULE       != 1);
    BUILD_BUG_ON(PPC_MSG_TICK_BROADCAST   != 2);
    BUILD_BUG_ON(PPC_MSG_NMI_IPI          != 3);
    for (i = 0; i < MSG_COUNT; i++) {
    result = ps3_event_receive_port_setup(cpu, &virqs[i]);
    if (result)
    continue;
    DBG("%s:%d: (%d, %d) => virq %u\n",
    __func__, __LINE__, cpu, i, virqs[i]);
    result = smp_request_message_ipi(virqs[i], i);
    if (result)
    virqs[i] = 0;
    else
    ps3_register_ipi_irq(cpu, virqs[i]);
    }
    ps3_register_ipi_debug_brk(cpu, virqs[PPC_MSG_NMI_IPI]);
    DBG(" <- %s:%d: (%d)\n", __func__, __LINE__, cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_smp_cleanup_cpu(cpu: c_int) {
    void ps3_smp_cleanup_cpu(int cpu)
    {
    unsigned int *virqs = per_cpu(ps3_ipi_virqs, cpu);
    int i;
    DBG(" . %s:%d: (%d)\n", __func__, __LINE__, cpu);
    for (i = 0; i < MSG_COUNT; i++) {
// Can't call free_irq from interrupt context.
    ps3_event_receive_port_destroy(virqs[i]);
    virqs[i] = 0;
    }
    DBG(" <- %s:%d: (%d)\n", __func__, __LINE__, cpu);
    }
    static struct smp_ops_t ps3_smp_ops = {
    .probe		= ps3_smp_probe,
    .message_pass	= ps3_smp_message_pass,
    .kick_cpu	= smp_generic_kick_cpu,
    };
#[no_mangle]
pub unsafe extern "C" fn smp_init_ps3() -> void __init {
    void __init smp_init_ps3(void)
    {
    DBG(" . %s\n", __func__);
    smp_ops = &ps3_smp_ops;
    DBG(" <- %s\n", __func__);
    }
