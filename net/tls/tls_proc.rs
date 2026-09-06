//! Automatically rewritten from C to Rust
//! Source: net/tls/tls_proc.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2019 Netronome Systems, Inc.

    static const struct snmp_mib tls_mib_list[] = {
    SNMP_MIB_ITEM("TlsCurrTxSw", LINUX_MIB_TLSCURRTXSW),
    SNMP_MIB_ITEM("TlsCurrRxSw", LINUX_MIB_TLSCURRRXSW),
    SNMP_MIB_ITEM("TlsCurrTxDevice", LINUX_MIB_TLSCURRTXDEVICE),
    SNMP_MIB_ITEM("TlsCurrRxDevice", LINUX_MIB_TLSCURRRXDEVICE),
    SNMP_MIB_ITEM("TlsTxSw", LINUX_MIB_TLSTXSW),
    SNMP_MIB_ITEM("TlsRxSw", LINUX_MIB_TLSRXSW),
    SNMP_MIB_ITEM("TlsTxDevice", LINUX_MIB_TLSTXDEVICE),
    SNMP_MIB_ITEM("TlsRxDevice", LINUX_MIB_TLSRXDEVICE),
    SNMP_MIB_ITEM("TlsDecryptError", LINUX_MIB_TLSDECRYPTERROR),
    SNMP_MIB_ITEM("TlsRxDeviceResync", LINUX_MIB_TLSRXDEVICERESYNC),
    SNMP_MIB_ITEM("TlsDecryptRetry", LINUX_MIB_TLSDECRYPTRETRY),
    SNMP_MIB_ITEM("TlsRxNoPadViolation", LINUX_MIB_TLSRXNOPADVIOL),
    SNMP_MIB_ITEM("TlsRxRekeyOk", LINUX_MIB_TLSRXREKEYOK),
    SNMP_MIB_ITEM("TlsRxRekeyError", LINUX_MIB_TLSRXREKEYERROR),
    SNMP_MIB_ITEM("TlsTxRekeyOk", LINUX_MIB_TLSTXREKEYOK),
    SNMP_MIB_ITEM("TlsTxRekeyError", LINUX_MIB_TLSTXREKEYERROR),
    SNMP_MIB_ITEM("TlsRxRekeyReceived", LINUX_MIB_TLSRXREKEYRECEIVED),
    };
#[no_mangle]
unsafe extern "C" fn tls_statistics_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int tls_statistics_seq_show(struct seq_file *seq, void *v)
    {
    unsigned long buf[ARRAY_SIZE(tls_mib_list)];
    let mut cnt: c_int = ARRAY_SIZE(tls_mib_list);
    struct net *net = seq.private;
    int i;
    memset(buf, 0, sizeof(buf));
    snmp_get_cpu_field_batch_cnt(buf, tls_mib_list, cnt,
    net.mib.tls_statistics);
    for (i = 0; i < cnt; i++)
    seq_printf(seq, "%-32s\t%lu\n", tls_mib_list[i].name, buf[i]);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn tls_proc_init(net: *mut net) -> int __net_init {
    int __net_init tls_proc_init(struct net *net)
    {

    if (!proc_create_net_single("tls_stat", 0444, net.proc_net,
    tls_statistics_seq_show, core::ptr::null_mut()))
    return -ENOMEM;

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tls_proc_fini(net: *mut net) -> void __net_exit {
    void __net_exit tls_proc_fini(struct net *net)
    {
    remove_proc_entry("tls_stat", net.proc_net);
    }
