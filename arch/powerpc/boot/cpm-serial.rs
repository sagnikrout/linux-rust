//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cpm-serial.c
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
// CPM serial console support.
//
// Copyright 2007 Freescale Semiconductor, Inc.
// Author: Scott Wood <scottwood@freescale.com>
//
// It is assumed that the firmware (or the platform file) has already set
// up the port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_scc {
    pub gsmrl: u32,
    pub gsmrh: u32,
    pub psmr: u16,
    pub res1: [u8; 2],
    pub todr: u16,
    pub dsr: u16,
    pub scce: u16,
    pub res2: [u8; 2],
    pub sccm: u16,
    pub res3: u8,
    pub sccs: u8,
    pub res4: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_smc {
    pub res1: [u8; 2],
    pub smcmr: u16,
    pub res2: [u8; 2],
    pub smce: u8,
    pub res3: [u8; 3],
    pub smcm: u8,
    pub res4: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_param {
    pub rbase: u16,
    pub tbase: u16,
    pub rfcr: u8,
    pub tfcr: u8,
    pub mrblr: u16,
    pub rstate: u32,
    pub res1: [u8; 4],
    pub rbptr: u16,
    pub res2: [u8; 6],
    pub tstate: u32,
    pub res3: [u8; 4],
    pub tbptr: u16,
    pub res4: [u8; 6],
    pub maxidl: u16,
    pub idlc: u16,
    pub brkln: u16,
    pub brkec: u16,
    pub brkcr: u16,
    pub rmask: u16,
    pub res5: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_bd {
    pub /: *mut *mut u16 sc; / Status and Control,
    pub /: *mut *mut u16 len; / Data length in buffer,
    pub /: *mut *mut *mut u8 addr; / Buffer address in host memory,
}

    static void *cpcr;
    static struct cpm_param *param;
    static struct cpm_smc *smc;
    static struct cpm_scc *scc;
    static struct cpm_bd *tbdf, *rbdf;
    static u32 cpm_cmd;
    static void *cbd_addr;
    static u32 cbd_offset;
    static void (*do_cmd)(int op);
    static void (*enable_port)(void);
    static void (*disable_port)(void);
pub const CPM_CMD_STOP_TX: c_int = 4;
pub const CPM_CMD_RESTART_TX: c_int = 6;
pub const CPM_CMD_INIT_RX_TX: c_int = 0;
#[no_mangle]
unsafe extern "C" fn cpm1_cmd(op: c_int) {
    static void cpm1_cmd(int op)
    {
    while (in_be16(cpcr) & 1)
    ;
    out_be16(cpcr, (op << 8) | cpm_cmd | 1);
    while (in_be16(cpcr) & 1)
    ;
    }
#[no_mangle]
unsafe extern "C" fn cpm2_cmd(op: c_int) {
    static void cpm2_cmd(int op)
    {
    while (in_be32(cpcr) & 0x10000)
    ;
    out_be32(cpcr, op | cpm_cmd | 0x10000);
    while (in_be32(cpcr) & 0x10000)
    ;
    }
#[no_mangle]
unsafe extern "C" fn smc_disable_port() {
    static void smc_disable_port(void)
    {
    do_cmd(CPM_CMD_STOP_TX);
    out_be16(&smc.smcmr, in_be16(&smc.smcmr) & ~3);
    }
#[no_mangle]
unsafe extern "C" fn scc_disable_port() {
    static void scc_disable_port(void)
    {
    do_cmd(CPM_CMD_STOP_TX);
    out_be32(&scc.gsmrl, in_be32(&scc.gsmrl) & ~0x30);
    }
#[no_mangle]
unsafe extern "C" fn smc_enable_port() {
    static void smc_enable_port(void)
    {
    out_be16(&smc.smcmr, in_be16(&smc.smcmr) | 3);
    do_cmd(CPM_CMD_RESTART_TX);
    }
#[no_mangle]
unsafe extern "C" fn scc_enable_port() {
    static void scc_enable_port(void)
    {
    out_be32(&scc.gsmrl, in_be32(&scc.gsmrl) | 0x30);
    do_cmd(CPM_CMD_RESTART_TX);
    }
#[no_mangle]
unsafe extern "C" fn cpm_serial_open() -> c_int {
    static int cpm_serial_open(void)
    {
    disable_port();
    out_8(&param.rfcr, 0x10);
    out_8(&param.tfcr, 0x10);
    out_be16(&param.mrblr, 1);
    out_be16(&param.maxidl, 0);
    out_be16(&param.brkec, 0);
    out_be16(&param.brkln, 0);
    out_be16(&param.brkcr, 0);
    rbdf = cbd_addr;
    rbdf.addr = (u8 *)rbdf - 1;
    rbdf.sc = 0xa000;
    rbdf.len = 1;
    tbdf = rbdf + 1;
    tbdf.addr = (u8 *)rbdf - 2;
    tbdf.sc = 0x2000;
    tbdf.len = 1;
    sync();
    out_be16(&param.rbase, cbd_offset);
    out_be16(&param.tbase, cbd_offset + sizeof(struct cpm_bd));
    do_cmd(CPM_CMD_INIT_RX_TX);
    enable_port();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpm_serial_putc(c: c_uchar) {
    static void cpm_serial_putc(unsigned char c)
    {
    while (tbdf.sc & 0x8000)
    barrier();
    sync();
    tbdf.addr[0] = c;
    eieio();
    tbdf.sc |= 0x8000;
    }
#[no_mangle]
unsafe extern "C" fn cpm_serial_tstc() -> c_uchar {
    static unsigned char cpm_serial_tstc(void)
    {
    barrier();
    return !(rbdf.sc & 0x8000);
    }
#[no_mangle]
unsafe extern "C" fn cpm_serial_getc() -> c_uchar {
    static unsigned char cpm_serial_getc(void)
    {
    unsigned char c;
    while (!cpm_serial_tstc())
    ;
    sync();
    c = rbdf.addr[0];
    eieio();
    rbdf.sc |= 0x8000;
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn cpm_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int {
    int cpm_console_init(void *devp, struct serial_console_data *scdp)
    {
    void *vreg[2];
    u32 reg[2];
    let mut is_smc: c_int = 0, is_cpm2 = 0;
    void *parent, *muram;
    void *muram_addr;
    unsigned long muram_offset, muram_size;
    if (dt_is_compatible(devp, "fsl,cpm1-smc-uart")) {
    is_smc = 1;
    } else if (dt_is_compatible(devp, "fsl,cpm2-scc-uart")) {
    is_cpm2 = 1;
    } else if (dt_is_compatible(devp, "fsl,cpm2-smc-uart")) {
    is_cpm2 = 1;
    is_smc = 1;
    }
    if (is_smc) {
    enable_port = smc_enable_port;
    disable_port = smc_disable_port;
    } else {
    enable_port = scc_enable_port;
    disable_port = scc_disable_port;
    }
    if (is_cpm2)
    do_cmd = cpm2_cmd;
    else
    do_cmd = cpm1_cmd;
    if (getprop(devp, "fsl,cpm-command", &cpm_cmd, 4) < 4)
    return -1;
    if (dt_get_virtual_reg(devp, vreg, 2) < 2)
    return -1;
    if (is_smc)
    smc = vreg[0];
    else
    scc = vreg[0];
    param = vreg[1];
    parent = get_parent(devp);
    if (!parent)
    return -1;
    if (dt_get_virtual_reg(parent, &cpcr, 1) < 1)
    return -1;
    muram = finddevice("/soc/cpm/muram/data");
    if (!muram)
    return -1;
// For bootwrapper-compatible device trees, we assume that the first
// entry has at least 128 bytes, and that #address-cells/#data-cells
// is one for both parent and child.
//
    if (dt_get_virtual_reg(muram, &muram_addr, 1) < 1)
    return -1;
    if (getprop(muram, "reg", reg, 8) < 8)
    return -1;
    muram_offset = reg[0];
    muram_size = reg[1];
// Store the buffer descriptors at the end of the first muram chunk.
// For SMC ports on CPM2-based platforms, relocate the parameter RAM
// just before the buffer descriptors.
//
    cbd_offset = muram_offset + muram_size - 2 * sizeof(struct cpm_bd);
    if (is_cpm2 && is_smc) {
    u16 *smc_base = (u16 *)param;
    u16 pram_offset;
    pram_offset = cbd_offset - 64;
    pram_offset = _ALIGN_DOWN(pram_offset, 64);
    disable_port();
    out_be16(smc_base, pram_offset);
    param = muram_addr - muram_offset + pram_offset;
    }
    cbd_addr = muram_addr - muram_offset + cbd_offset;
    scdp.open = cpm_serial_open;
    scdp.putc = cpm_serial_putc;
    scdp.getc = cpm_serial_getc;
    scdp.tstc = cpm_serial_tstc;
    return 0;
    }
