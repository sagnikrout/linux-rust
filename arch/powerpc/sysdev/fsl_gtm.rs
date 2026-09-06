//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_gtm.c
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
// Freescale General-purpose Timers Module
//
// Copyright (c) Freescale Semiconductor, Inc. 2006.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtm_timers_regs {
    pub /: *mut *mut u8 gtcfr1; / Timer 1, Timer 2 global config register,
    pub res0: [u8; 0x3],
    pub /: *mut *mut u8 gtcfr2; / Timer 3, timer 4 global config register,
    pub res1: [u8; 0xB],
    pub /: *mut *mut __be16 gtmdr1; / Timer 1 mode register,
    pub /: *mut *mut __be16 gtmdr2; / Timer 2 mode register,
    pub /: *mut *mut __be16 gtrfr1; / Timer 1 reference register,
    pub /: *mut *mut __be16 gtrfr2; / Timer 2 reference register,
    pub /: *mut *mut __be16 gtcpr1; / Timer 1 capture register,
    pub /: *mut *mut __be16 gtcpr2; / Timer 2 capture register,
    pub /: *mut *mut __be16 gtcnr1; / Timer 1 counter,
    pub /: *mut *mut __be16 gtcnr2; / Timer 2 counter,
    pub /: *mut *mut __be16 gtmdr3; / Timer 3 mode register,
    pub /: *mut *mut __be16 gtmdr4; / Timer 4 mode register,
    pub /: *mut *mut __be16 gtrfr3; / Timer 3 reference register,
    pub /: *mut *mut __be16 gtrfr4; / Timer 4 reference register,
    pub /: *mut *mut __be16 gtcpr3; / Timer 3 capture register,
    pub /: *mut *mut __be16 gtcpr4; / Timer 4 capture register,
    pub /: *mut *mut __be16 gtcnr3; / Timer 3 counter,
    pub /: *mut *mut __be16 gtcnr4; / Timer 4 counter,
    pub /: *mut *mut __be16 gtevr1; / Timer 1 event register,
    pub /: *mut *mut __be16 gtevr2; / Timer 2 event register,
    pub /: *mut *mut __be16 gtevr3; / Timer 3 event register,
    pub /: *mut *mut __be16 gtevr4; / Timer 4 event register,
    pub /: *mut *mut __be16 gtpsr1; / Timer 1 prescale register,
    pub /: *mut *mut __be16 gtpsr2; / Timer 2 prescale register,
    pub /: *mut *mut __be16 gtpsr3; / Timer 3 prescale register,
    pub /: *mut *mut __be16 gtpsr4; / Timer 4 prescale register,
    pub res2: [u8; 0x40],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtm {
    pub clock: c_uint,
    pub regs: *mut gtm_timers_regs __iomem,
    pub timers: [gtm_timer; 4],
    pub lock: spinlock_t,
    pub list_node: list_head,
}

    static LIST_HEAD(gtms);
//
// gtm_get_timer16 - request GTM timer to use it with the rest of GTM API
// Context:	non-IRQ
//
// This function reserves GTM timer for later use. It returns gtm_timer
// structure to use with the rest of GTM API, you should use timer->irq
// to manage timer interrupt.
//
    struct gtm_timer *gtm_get_timer16(void)
    {
    struct gtm *gtm;
    int i;
    list_for_each_entry(gtm, &gtms, list_node) {
    spin_lock_irq(&gtm.lock);
    for (i = 0; i < ARRAY_SIZE(gtm.timers); i++) {
    if (!gtm.timers[i].requested) {
    gtm.timers[i].requested = true;
    spin_unlock_irq(&gtm.lock);
    return &gtm.timers[i];
    }
    }
    spin_unlock_irq(&gtm.lock);
    }
    if (!list_empty(&gtms))
    return ERR_PTR(-EBUSY);
    return ERR_PTR(-ENODEV);
    }
    EXPORT_SYMBOL(gtm_get_timer16);
//
// gtm_get_specific_timer16 - request specific GTM timer
// @gtm:	specific GTM, pass here GTM's device_node->data
// @timer:	specific timer number, Timer1 is 0.
// Context:	non-IRQ
//
// This function reserves GTM timer for later use. It returns gtm_timer
// structure to use with the rest of GTM API, you should use timer->irq
// to manage timer interrupt.
//
    struct gtm_timer *gtm_get_specific_timer16(struct gtm *gtm,
    unsigned int timer)
    {
    struct gtm_timer *ret = ERR_PTR(-EBUSY);
    if (timer > 3)
    return ERR_PTR(-EINVAL);
    spin_lock_irq(&gtm.lock);
    if (gtm.timers[timer].requested)
    goto out;
    ret = &gtm.timers[timer];
    ret.requested = true;
    out:
    spin_unlock_irq(&gtm.lock);
    return ret;
    }
    EXPORT_SYMBOL(gtm_get_specific_timer16);
//
// gtm_put_timer16 - release 16 bits GTM timer
// @tmr:	pointer to the gtm_timer structure obtained from gtm_get_timer
// Context:	any
//
// This function releases GTM timer so others may request it.
//
#[no_mangle]
pub unsafe extern "C" fn gtm_put_timer16(tmr: *mut gtm_timer) {
    void gtm_put_timer16(struct gtm_timer *tmr)
    {
    gtm_stop_timer16(tmr);
    spin_lock_irq(&tmr.gtm.lock);
    tmr.requested = false;
    spin_unlock_irq(&tmr.gtm.lock);
    }
    EXPORT_SYMBOL(gtm_put_timer16);
//
// This is back-end for the exported functions, it's used to reset single
// timer in reference mode.
//
    static int gtm_set_ref_timer16(struct gtm_timer *tmr, int frequency,
    int reference_value, bool free_run)
    {
    struct gtm *gtm = tmr.gtm;
    let mut num: c_int = tmr - &gtm.timers[0];
    unsigned int prescaler;
    let mut iclk: u8 = GTMDR_ICLK_ICLK;
    u8 psr;
    u8 sps;
    unsigned long flags;
    let mut max_prescaler: c_int = 256 * 256 * 16;
// CPM2 doesn't have primary prescaler
    if (!tmr.gtpsr)
    max_prescaler /= 256;
    prescaler = gtm.clock / frequency;
//
// We have two 8 bit prescalers -- primary and secondary (psr, sps),
// plus "slow go" mode (clk / 16). So, total prescale value is
// 16 * (psr + 1) * (sps + 1). Though, for CPM2 GTMs we losing psr.
//
    if (prescaler > max_prescaler)
    return -EINVAL;
    if (prescaler > max_prescaler / 16) {
    iclk = GTMDR_ICLK_SLGO;
    prescaler /= 16;
    }
    if (prescaler <= 256) {
    psr = 0;
    sps = prescaler - 1;
    } else {
    psr = 256 - 1;
    sps = prescaler / 256 - 1;
    }
    spin_lock_irqsave(&gtm.lock, flags);
//
// Properly reset timers: stop, reset, set up prescalers, reference
// value and clear event register.
//
    clrsetbits_8(tmr.gtcfr, ~(GTCFR_STP(num) | GTCFR_RST(num)),
    GTCFR_STP(num) | GTCFR_RST(num));
    setbits8(tmr.gtcfr, GTCFR_STP(num));
    if (tmr.gtpsr)
    out_be16(tmr.gtpsr, psr);
    clrsetbits_be16(tmr.gtmdr, 0xFFFF, iclk | GTMDR_SPS(sps) |
    GTMDR_ORI | (free_run ? GTMDR_FRR : 0));
    out_be16(tmr.gtcnr, 0);
    out_be16(tmr.gtrfr, reference_value);
    out_be16(tmr.gtevr, 0xFFFF);
// Let it be.
    clrbits8(tmr.gtcfr, GTCFR_STP(num));
    spin_unlock_irqrestore(&gtm.lock, flags);
    return 0;
    }
//
// gtm_set_timer16 - (re)set 16 bit timer with arbitrary precision
// @tmr:	pointer to the gtm_timer structure obtained from gtm_get_timer
// @usec:	timer interval in microseconds
// @reload:	if set, the timer will reset upon expiry rather than
// continue running free.
// Context:	any
//
// This function (re)sets the GTM timer so that it counts up to the requested
// interval value, and fires the interrupt when the value is reached. This
// function will reduce the precision of the timer as needed in order for the
// requested timeout to fit in a 16-bit register.
//
#[no_mangle]
pub unsafe extern "C" fn gtm_set_timer16(tmr: *mut gtm_timer, usec: c_ulong, reload: bool) -> c_int {
    int gtm_set_timer16(struct gtm_timer *tmr, unsigned long usec, bool reload)
    {
// quite obvious, frequency which is enough for µSec precision
    let mut freq: c_int = 1000000;
    unsigned int bit;
    bit = fls_long(usec);
    if (bit > 15) {
    freq >>= bit - 15;
    usec >>= bit - 15;
    }
    if (!freq)
    return -EINVAL;
    return gtm_set_ref_timer16(tmr, freq, usec, reload);
    }
    EXPORT_SYMBOL(gtm_set_timer16);
//
// gtm_set_exact_timer16 - (re)set 16 bits timer
// @tmr:	pointer to the gtm_timer structure obtained from gtm_get_timer
// @usec:	timer interval in microseconds
// @reload:	if set, the timer will reset upon expiry rather than
// continue running free.
// Context:	any
//
// This function (re)sets GTM timer so that it counts up to the requested
// interval value, and fires the interrupt when the value is reached. If reload
// flag was set, timer will also reset itself upon reference value, otherwise
// it continues to increment.
//
// The _exact_ bit in the function name states that this function will not
// crop precision of the "usec" argument, thus usec is limited to 16 bits
// (single timer width).
//
#[no_mangle]
pub unsafe extern "C" fn gtm_set_exact_timer16(tmr: *mut gtm_timer, usec: u16, reload: bool) -> c_int {
    int gtm_set_exact_timer16(struct gtm_timer *tmr, u16 usec, bool reload)
    {
// quite obvious, frequency which is enough for µSec precision
    let mut freq: c_int = 1000000;
//
// We can lower the frequency (and probably power consumption) by
// dividing both frequency and usec by 2 until there is no remainder.
// But we won't bother with this unless savings are measured, so just
// run the timer as is.
//
    return gtm_set_ref_timer16(tmr, freq, usec, reload);
    }
    EXPORT_SYMBOL(gtm_set_exact_timer16);
//
// gtm_stop_timer16 - stop single timer
// @tmr:	pointer to the gtm_timer structure obtained from gtm_get_timer
// Context:	any
//
// This function simply stops the GTM timer.
//
#[no_mangle]
pub unsafe extern "C" fn gtm_stop_timer16(tmr: *mut gtm_timer) {
    void gtm_stop_timer16(struct gtm_timer *tmr)
    {
    struct gtm *gtm = tmr.gtm;
    let mut num: c_int = tmr - &gtm.timers[0];
    unsigned long flags;
    spin_lock_irqsave(&gtm.lock, flags);
    setbits8(tmr.gtcfr, GTCFR_STP(num));
    out_be16(tmr.gtevr, 0xFFFF);
    spin_unlock_irqrestore(&gtm.lock, flags);
    }
    EXPORT_SYMBOL(gtm_stop_timer16);
//
// gtm_ack_timer16 - acknowledge timer event (free-run timers only)
// @tmr:	pointer to the gtm_timer structure obtained from gtm_get_timer
// @events:	events mask to ack
// Context:	any
//
// Thus function used to acknowledge timer interrupt event, use it inside the
// interrupt handler.
//
#[no_mangle]
pub unsafe extern "C" fn gtm_ack_timer16(tmr: *mut gtm_timer, events: u16) {
    void gtm_ack_timer16(struct gtm_timer *tmr, u16 events)
    {
    out_be16(tmr.gtevr, events);
    }
    EXPORT_SYMBOL(gtm_ack_timer16);
    static void __init gtm_set_shortcuts(struct device_node *np,
    struct gtm_timer *timers,
    struct gtm_timers_regs __iomem *regs)
    {
//
// Yeah, I don't like this either, but timers' registers a bit messed,
// so we have to provide shortcuts to write timer independent code.
// Alternative option is to create gt*() accessors, but that will be
// even uglier and cryptic.
//
    timers[0].gtcfr = &regs.gtcfr1;
    timers[0].gtmdr = &regs.gtmdr1;
    timers[0].gtcnr = &regs.gtcnr1;
    timers[0].gtrfr = &regs.gtrfr1;
    timers[0].gtevr = &regs.gtevr1;
    timers[1].gtcfr = &regs.gtcfr1;
    timers[1].gtmdr = &regs.gtmdr2;
    timers[1].gtcnr = &regs.gtcnr2;
    timers[1].gtrfr = &regs.gtrfr2;
    timers[1].gtevr = &regs.gtevr2;
    timers[2].gtcfr = &regs.gtcfr2;
    timers[2].gtmdr = &regs.gtmdr3;
    timers[2].gtcnr = &regs.gtcnr3;
    timers[2].gtrfr = &regs.gtrfr3;
    timers[2].gtevr = &regs.gtevr3;
    timers[3].gtcfr = &regs.gtcfr2;
    timers[3].gtmdr = &regs.gtmdr4;
    timers[3].gtcnr = &regs.gtcnr4;
    timers[3].gtrfr = &regs.gtrfr4;
    timers[3].gtevr = &regs.gtevr4;
// CPM2 doesn't have primary prescaler
    if (!of_device_is_compatible(np, "fsl,cpm2-gtm")) {
    timers[0].gtpsr = &regs.gtpsr1;
    timers[1].gtpsr = &regs.gtpsr2;
    timers[2].gtpsr = &regs.gtpsr3;
    timers[3].gtpsr = &regs.gtpsr4;
    }
    }
#[no_mangle]
unsafe extern "C" fn fsl_gtm_init() -> int __init {
    static int __init fsl_gtm_init(void)
    {
    struct device_node *np;
    for_each_compatible_node(np, core::ptr::null_mut(), "fsl,gtm") {
    int i;
    struct gtm *gtm;
    const u32 *clock;
    int size;
    gtm = kzalloc_obj(*gtm);
    if (!gtm) {
    pr_err("%pOF: unable to allocate memory\n",
    np);
    continue;
    }
    spin_lock_init(&gtm.lock);
    clock = of_get_property(np, "clock-frequency", &size);
    if (!clock || size != sizeof(*clock)) {
    pr_err("%pOF: no clock-frequency\n", np);
    goto err;
    }
    gtm.clock = *clock;
    for (i = 0; i < ARRAY_SIZE(gtm.timers); i++) {
    unsigned int irq;
    irq = irq_of_parse_and_map(np, i);
    if (!irq) {
    pr_err("%pOF: not enough interrupts specified\n",
    np);
    goto err;
    }
    gtm.timers[i].irq = irq;
    gtm.timers[i].gtm = gtm;
    }
    gtm.regs = of_iomap(np, 0);
    if (!gtm.regs) {
    pr_err("%pOF: unable to iomap registers\n",
    np);
    goto err;
    }
    gtm_set_shortcuts(np, gtm.timers, gtm.regs);
    list_add(&gtm.list_node, &gtms);
// We don't want to lose the node and its ->data
    np.data = gtm;
    of_node_get(np);
    continue;
    err:
    kfree(gtm);
    }
    return 0;
    }
    arch_initcall(fsl_gtm_init);
