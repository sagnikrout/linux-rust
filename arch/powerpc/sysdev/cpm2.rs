//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/cpm2.c
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


//
// General Purpose functions for the global management of the
// 8260 Communication Processor Module.
// Copyright (c) 1999-2001 Dan Malek <dan@embeddedalley.com>
// Copyright (c) 2000 MontaVista Software, Inc (source@mvista.com)
// 2.3.99 Updates
//
// 2006 (c) MontaVista Software, Inc.
// Vitaly Bordug <vbordug@ru.mvista.com>
// Merged to arch/powerpc from arch/ppc/syslib/cpm2_common.c
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// In addition to the individual control of the communication
// channels, there are a few functions that globally affect the
// communication processor.
//
// Buffer descriptors must be allocated from the dual ported memory
// space.  The allocator for that is here.  When the communication
// process is reset, we reclaim the memory available.  There is
// currently no deallocator for this memory.
//

    cpm_cpm2_t __iomem *cpmp; /* Pointer to comm processor space */
// We allocate this here because it is used almost exclusively for
// the communication processor devices.
//
    cpm2_map_t __iomem *cpm2_immr;
    EXPORT_SYMBOL(cpm2_immr);

    of space for CPM as it is larger
    than on PQ2 */
#[no_mangle]
pub unsafe extern "C" fn cpm2_reset() -> void __init {
    void __init cpm2_reset(void)
    {

    cpm2_immr = ioremap(get_immrbase() + 0x80000, CPM_MAP_SIZE);

    cpm2_immr = ioremap(get_immrbase(), CPM_MAP_SIZE);

// Tell everyone where the comm processor resides.
//
    cpmp = &cpm2_immr.im_cpm;

// Reset the CPM.
//
    cpm_command(CPM_CR_RST, 0);

    }
    static DEFINE_SPINLOCK(cmd_lock);
pub const MAX_CR_CMD_LOOPS: c_int = 10000;
#[no_mangle]
pub unsafe extern "C" fn cpm_command(command: u32, opcode: u8) -> c_int {
    int cpm_command(u32 command, u8 opcode)
    {
    int i, ret;
    unsigned long flags;
    spin_lock_irqsave(&cmd_lock, flags);
    ret = 0;
    out_be32(&cpmp.cp_cpcr, command | opcode | CPM_CR_FLG);
    for (i = 0; i < MAX_CR_CMD_LOOPS; i++)
    if ((in_be32(&cpmp.cp_cpcr) & CPM_CR_FLG) == 0)
    goto out;
    printk(KERN_ERR "%s(): Not able to issue CPM command\n", __func__);
    ret = -EIO;
    out:
    spin_unlock_irqrestore(&cmd_lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(cpm_command);
// Set a baud rate generator.  This needs lots of work.  There are
// eight BRGs, which can be connected to the CPM channels or output
// as clocks.  The BRGs are in two different block of internal
// memory mapped space.
// The baud rate clock is the system clock divided by something.
// It was set up long ago during the initial boot phase and is
// given to us.
// Baud rate clocks are zero-based in the driver code (as that maps
// to port numbers).  Documentation uses 1-based numbering.
//
#[no_mangle]
pub unsafe extern "C" fn __cpm2_setbrg(brg: c_uint, rate: c_uint, clk: c_uint, div16: c_int, src: c_int) {
    void __cpm2_setbrg(uint brg, uint rate, uint clk, int div16, int src)
    {
    u32 __iomem *bp;
    u32 val;
// This is good enough to get SMCs running.....
//
    if (brg < 4) {
    bp = &cpm2_immr.im_brgc1;
    } else {
    bp = &cpm2_immr.im_brgc5;
    brg -= 4;
    }
    bp += brg;
// Round the clock divider to the nearest integer.
    val = (((clk * 2 / rate) - 1) & ~1) | CPM_BRG_EN | src;
    if (div16)
    val |= CPM_BRG_DIV16;
    out_be32(bp, val);
    }
    EXPORT_SYMBOL(__cpm2_setbrg);
#[no_mangle]
pub unsafe extern "C" fn cpm2_clk_setup(target: enum cpm_clk_target, clock: c_int, mode: c_int) -> int __init {
    int __init cpm2_clk_setup(enum cpm_clk_target target, int clock, int mode)
    {
    let mut ret: c_int = 0;
    int shift;
    int i, bits = 0;
    u32 __iomem *reg;
    let mut mask: u32 = 7;
    u8 clk_map[][3] = {
    {CPM_CLK_FCC1, CPM_BRG5, 0},
    {CPM_CLK_FCC1, CPM_BRG6, 1},
    {CPM_CLK_FCC1, CPM_BRG7, 2},
    {CPM_CLK_FCC1, CPM_BRG8, 3},
    {CPM_CLK_FCC1, CPM_CLK9, 4},
    {CPM_CLK_FCC1, CPM_CLK10, 5},
    {CPM_CLK_FCC1, CPM_CLK11, 6},
    {CPM_CLK_FCC1, CPM_CLK12, 7},
    {CPM_CLK_FCC2, CPM_BRG5, 0},
    {CPM_CLK_FCC2, CPM_BRG6, 1},
    {CPM_CLK_FCC2, CPM_BRG7, 2},
    {CPM_CLK_FCC2, CPM_BRG8, 3},
    {CPM_CLK_FCC2, CPM_CLK13, 4},
    {CPM_CLK_FCC2, CPM_CLK14, 5},
    {CPM_CLK_FCC2, CPM_CLK15, 6},
    {CPM_CLK_FCC2, CPM_CLK16, 7},
    {CPM_CLK_FCC3, CPM_BRG5, 0},
    {CPM_CLK_FCC3, CPM_BRG6, 1},
    {CPM_CLK_FCC3, CPM_BRG7, 2},
    {CPM_CLK_FCC3, CPM_BRG8, 3},
    {CPM_CLK_FCC3, CPM_CLK13, 4},
    {CPM_CLK_FCC3, CPM_CLK14, 5},
    {CPM_CLK_FCC3, CPM_CLK15, 6},
    {CPM_CLK_FCC3, CPM_CLK16, 7},
    {CPM_CLK_SCC1, CPM_BRG1, 0},
    {CPM_CLK_SCC1, CPM_BRG2, 1},
    {CPM_CLK_SCC1, CPM_BRG3, 2},
    {CPM_CLK_SCC1, CPM_BRG4, 3},
    {CPM_CLK_SCC1, CPM_CLK11, 4},
    {CPM_CLK_SCC1, CPM_CLK12, 5},
    {CPM_CLK_SCC1, CPM_CLK3, 6},
    {CPM_CLK_SCC1, CPM_CLK4, 7},
    {CPM_CLK_SCC2, CPM_BRG1, 0},
    {CPM_CLK_SCC2, CPM_BRG2, 1},
    {CPM_CLK_SCC2, CPM_BRG3, 2},
    {CPM_CLK_SCC2, CPM_BRG4, 3},
    {CPM_CLK_SCC2, CPM_CLK11, 4},
    {CPM_CLK_SCC2, CPM_CLK12, 5},
    {CPM_CLK_SCC2, CPM_CLK3, 6},
    {CPM_CLK_SCC2, CPM_CLK4, 7},
    {CPM_CLK_SCC3, CPM_BRG1, 0},
    {CPM_CLK_SCC3, CPM_BRG2, 1},
    {CPM_CLK_SCC3, CPM_BRG3, 2},
    {CPM_CLK_SCC3, CPM_BRG4, 3},
    {CPM_CLK_SCC3, CPM_CLK5, 4},
    {CPM_CLK_SCC3, CPM_CLK6, 5},
    {CPM_CLK_SCC3, CPM_CLK7, 6},
    {CPM_CLK_SCC3, CPM_CLK8, 7},
    {CPM_CLK_SCC4, CPM_BRG1, 0},
    {CPM_CLK_SCC4, CPM_BRG2, 1},
    {CPM_CLK_SCC4, CPM_BRG3, 2},
    {CPM_CLK_SCC4, CPM_BRG4, 3},
    {CPM_CLK_SCC4, CPM_CLK5, 4},
    {CPM_CLK_SCC4, CPM_CLK6, 5},
    {CPM_CLK_SCC4, CPM_CLK7, 6},
    {CPM_CLK_SCC4, CPM_CLK8, 7},
    };
    switch (target) {
    case CPM_CLK_SCC1:
    reg = &cpm2_immr.im_cpmux.cmx_scr;
    shift = 24;
    break;
    case CPM_CLK_SCC2:
    reg = &cpm2_immr.im_cpmux.cmx_scr;
    shift = 16;
    break;
    case CPM_CLK_SCC3:
    reg = &cpm2_immr.im_cpmux.cmx_scr;
    shift = 8;
    break;
    case CPM_CLK_SCC4:
    reg = &cpm2_immr.im_cpmux.cmx_scr;
    shift = 0;
    break;
    case CPM_CLK_FCC1:
    reg = &cpm2_immr.im_cpmux.cmx_fcr;
    shift = 24;
    break;
    case CPM_CLK_FCC2:
    reg = &cpm2_immr.im_cpmux.cmx_fcr;
    shift = 16;
    break;
    case CPM_CLK_FCC3:
    reg = &cpm2_immr.im_cpmux.cmx_fcr;
    shift = 8;
    break;
    default:
    printk(KERN_ERR "cpm2_clock_setup: invalid clock target\n");
    return -EINVAL;
    }
    for (i = 0; i < ARRAY_SIZE(clk_map); i++) {
    if (clk_map[i][0] == target && clk_map[i][1] == clock) {
    bits = clk_map[i][2];
    break;
    }
    }
    if (i == ARRAY_SIZE(clk_map))
    ret = -EINVAL;
    bits <<= shift;
    mask <<= shift;
    if (mode == CPM_CLK_RTX) {
    bits |= bits << 3;
    mask |= mask << 3;
    } else if (mode == CPM_CLK_RX) {
    bits <<= 3;
    mask <<= 3;
    }
    out_be32(reg, (in_be32(reg) & ~mask) | bits);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cpm2_smc_clk_setup(target: enum cpm_clk_target, clock: c_int) -> int __init {
    int __init cpm2_smc_clk_setup(enum cpm_clk_target target, int clock)
    {
    let mut ret: c_int = 0;
    int shift;
    int i, bits = 0;
    u8 __iomem *reg;
    let mut mask: u8 = 3;
    u8 clk_map[][3] = {
    {CPM_CLK_SMC1, CPM_BRG1, 0},
    {CPM_CLK_SMC1, CPM_BRG7, 1},
    {CPM_CLK_SMC1, CPM_CLK7, 2},
    {CPM_CLK_SMC1, CPM_CLK9, 3},
    {CPM_CLK_SMC2, CPM_BRG2, 0},
    {CPM_CLK_SMC2, CPM_BRG8, 1},
    {CPM_CLK_SMC2, CPM_CLK4, 2},
    {CPM_CLK_SMC2, CPM_CLK15, 3},
    };
    switch (target) {
    case CPM_CLK_SMC1:
    reg = &cpm2_immr.im_cpmux.cmx_smr;
    mask = 3;
    shift = 4;
    break;
    case CPM_CLK_SMC2:
    reg = &cpm2_immr.im_cpmux.cmx_smr;
    mask = 3;
    shift = 0;
    break;
    default:
    printk(KERN_ERR "cpm2_smc_clock_setup: invalid clock target\n");
    return -EINVAL;
    }
    for (i = 0; i < ARRAY_SIZE(clk_map); i++) {
    if (clk_map[i][0] == target && clk_map[i][1] == clock) {
    bits = clk_map[i][2];
    break;
    }
    }
    if (i == ARRAY_SIZE(clk_map))
    ret = -EINVAL;
    bits <<= shift;
    mask <<= shift;
    out_8(reg, (in_8(reg) & ~mask) | bits);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm2_ioports {
    pub dat: u32 dir, par, sor, odr,,
    pub res: [u32; 3],
}

#[no_mangle]
pub unsafe extern "C" fn cpm2_set_pin(port: c_int, pin: c_int, flags: c_int) -> void __init {
    void __init cpm2_set_pin(int port, int pin, int flags)
    {
    struct cpm2_ioports __iomem *iop =
    (struct cpm2_ioports __iomem *)&cpm2_immr.im_ioport;
    pin = 1 << (31 - pin);
    if (flags & CPM_PIN_OUTPUT)
    setbits32(&iop[port].dir, pin);
    else
    clrbits32(&iop[port].dir, pin);
    if (!(flags & CPM_PIN_GPIO))
    setbits32(&iop[port].par, pin);
    else
    clrbits32(&iop[port].par, pin);
    if (flags & CPM_PIN_SECONDARY)
    setbits32(&iop[port].sor, pin);
    else
    clrbits32(&iop[port].sor, pin);
    if (flags & CPM_PIN_OPENDRAIN)
    setbits32(&iop[port].odr, pin);
    else
    clrbits32(&iop[port].odr, pin);
    }
