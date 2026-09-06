//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_parport/ktti.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// (c) 1998  Grant R. Guenther <grant@torque.net>
//
// ktti.c is a low-level protocol driver for the KT Technology
// parallel port adapter.  This adapter is used in the "PHd"
// portable hard-drives.  As far as I can tell, this device
// supports 4-bit mode _only_.
//

//
// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set
//
    static int cont_map[2] = { 0x10, 0x08 };
#[no_mangle]
unsafe extern "C" fn ktti_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    static void ktti_write_regr(struct pi_adapter *pi, int cont, int regr, int val)
    {
    let mut r: c_int = regr + cont_map[cont];
    w0(r); w2(0xb); w2(0xa); w2(3); w2(6);
    w0(val); w2(3); w0(0); w2(6); w2(0xb);
    }
#[no_mangle]
unsafe extern "C" fn ktti_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    static int ktti_read_regr(struct pi_adapter *pi, int cont, int regr)
    {
    int a, b, r;
    r = regr + cont_map[cont];
    w0(r); w2(0xb); w2(0xa); w2(9); w2(0xc); w2(9);
    a = r1(); w2(0xc);  b = r1(); w2(9); w2(0xc); w2(9);
    return j44(a, b);
    }
#[no_mangle]
unsafe extern "C" fn ktti_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void ktti_read_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k, a, b;
    for (k = 0; k < count / 2; k++) {
    w0(0x10); w2(0xb); w2(0xa); w2(9); w2(0xc); w2(9);
    a = r1(); w2(0xc); b = r1(); w2(9);
    buf[2*k] = j44(a, b);
    a = r1(); w2(0xc); b = r1(); w2(9);
    buf[2*k+1] = j44(a, b);
    }
    }
#[no_mangle]
unsafe extern "C" fn ktti_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void ktti_write_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k;
    for (k = 0; k < count / 2; k++) {
    w0(0x10); w2(0xb); w2(0xa); w2(3); w2(6);
    w0(buf[2 * k]); w2(3);
    w0(buf[2 * k + 1]); w2(6);
    w2(0xb);
    }
    }
#[no_mangle]
unsafe extern "C" fn ktti_connect(pi: *mut pi_adapter) {
    static void ktti_connect(struct pi_adapter *pi)
    {
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    w2(0xb); w2(0xa); w0(0); w2(3); w2(6);
    }
#[no_mangle]
unsafe extern "C" fn ktti_disconnect(pi: *mut pi_adapter) {
    static void ktti_disconnect(struct pi_adapter *pi)
    {
    w2(0xb); w2(0xa); w0(0xa0); w2(3); w2(4);
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    }
#[no_mangle]
unsafe extern "C" fn ktti_log_adapter(pi: *mut pi_adapter) {
    static void ktti_log_adapter(struct pi_adapter *pi)
    {
    dev_info(&pi.dev, "KT adapter at 0x%x, delay %d\n",
    pi.port, pi.delay);
    }
    static struct pi_protocol ktti = {
    .owner		= THIS_MODULE,
    .name		= "ktti",
    .max_mode	= 1,
    .epp_first	= 2,
    .default_delay	= 1,
    .max_units	= 1,
    .write_regr	= ktti_write_regr,
    .read_regr	= ktti_read_regr,
    .write_block	= ktti_write_block,
    .read_block	= ktti_read_block,
    .connect	= ktti_connect,
    .disconnect	= ktti_disconnect,
    .log_adapter	= ktti_log_adapter,
    };
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
    MODULE_DESCRIPTION("KT Technology parallel port IDE adapter protocol driver");
    module_pata_parport_driver(ktti);
