//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/sh_cmt.c
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
// SuperH Timer Support - CMT
//
// Copyright (C) 2008 Magnus Damm
//

    struct sh_cmt_device;
//
// The CMT comes in 5 different identified flavours, depending not only on the
// SoC but also on the particular instance. The following table lists the main
// characteristics of those flavours.
//
// 16B	32B	32B-F	48B	R-Car Gen2
// -----------------------------------------------------------------------------
// Channels		2	1/4	1	6	2/8
// Control Width	16	16	16	16	32
// Counter Width	16	32	32	32/48	32/48
// Shared Start/Stop	Y	Y	Y	Y	N
//
// The r8a73a4 / R-Car Gen2 version has a per-channel start/stop register
// located in the channel registers block. All other versions have a shared
// start/stop register located in the global space.
//
// Channels are indexed from 0 to N-1 in the documentation. The channel index
// infers the start/stop bit position in the control register and the channel
// registers block address. Some CMT instances have a subset of channels
// available, in which case the index in the documentation doesn't match the
// "real" index as implemented in hardware. This is for instance the case with
// CMT0 on r8a7740, which is a 32-bit variant with a single channel numbered 0
// in the documentation but using start/stop bit 5 and having its registers
// block at 0x60.
//
// Similarly CMT0 on r8a73a4, r8a7790 and r8a7791, while implementing 32-bit
// channels only, is a 48-bit gen2 CMT with the 48-bit channels unavailable.
//
    enum sh_cmt_model {
    SH_CMT_16BIT,
    SH_CMT_32BIT,
    SH_CMT_48BIT,
    SH_CMT0_RCAR_GEN2,
    SH_CMT1_RCAR_GEN2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_cmt_info {
    pub model: enum sh_cmt_model,
    pub channels_mask: c_uint,
    pub /: *mut *mut unsigned long width; / 16 or 32 bit version of hardware block,
    pub overflow_bit: u32,
    pub clear_bits: u32,
// callbacks for CMSTR and CMCSR access
    pub offs): *mut *mut *mut u32 (read_control)(void __iomem base, unsigned long,
    void (*write_control)(void __iomem *base, unsigned long offs,
    pub value): u32,
// callbacks for CMCNT and CMCOR access
    pub offs): *mut *mut *mut u32 (read_count)(void __iomem base, unsigned long,
    pub value): *mut *mut *mut void (write_count)(void __iomem base, unsigned long offs, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_cmt_channel {
    pub cmt: *mut sh_cmt_device,
    pub /: *mut *mut unsigned int index; / Index in the documentation,
    pub /: *mut *mut unsigned int hwidx; / Real hardware index,
    pub iostart: *mut void __iomem,
    pub ioctrl: *mut void __iomem,
    pub timer_bit: c_uint,
    pub flags: c_ulong,
    pub match_value: u32,
    pub next_match_value: u32,
    pub max_match_value: u32,
    pub lock: raw_spinlock_t,
    pub ced: clock_event_device,
    pub cs: clocksource,
    pub total_cycles: u64,
    pub cs_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_cmt_device {
    pub pdev: *mut platform_device,
    pub info: *const sh_cmt_info,
    pub mapbase: *mut void __iomem,
    pub clk: *mut clk,
    pub rate: c_ulong,
    pub reg_delay: c_uint,
    pub /: *mut *mut raw_spinlock_t lock; / Protect the shared start/stop register,
    pub channels: *mut sh_cmt_channel,
    pub num_channels: c_uint,
    pub hw_channels: c_uint,
    pub has_clockevent: bool,
    pub has_clocksource: bool,
}

#[no_mangle]
unsafe extern "C" fn sh_cmt_read16(base: *mut void __iomem, offs: c_ulong) -> u32 {
    static u32 sh_cmt_read16(void __iomem *base, unsigned long offs)
    {
    return ioread16(base + (offs << 1));
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_read32(base: *mut void __iomem, offs: c_ulong) -> u32 {
    static u32 sh_cmt_read32(void __iomem *base, unsigned long offs)
    {
    return ioread32(base + (offs << 2));
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_write16(base: *mut void __iomem, offs: c_ulong, value: u32) {
    static void sh_cmt_write16(void __iomem *base, unsigned long offs, u32 value)
    {
    iowrite16(value, base + (offs << 1));
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_write32(base: *mut void __iomem, offs: c_ulong, value: u32) {
    static void sh_cmt_write32(void __iomem *base, unsigned long offs, u32 value)
    {
    iowrite32(value, base + (offs << 2));
    }
    static const struct sh_cmt_info sh_cmt_info[] = {
    [SH_CMT_16BIT] = {
    .model = SH_CMT_16BIT,
    .width = 16,
    .overflow_bit = SH_CMT16_CMCSR_CMF,
    .clear_bits = ~SH_CMT16_CMCSR_CMF,
    .read_control = sh_cmt_read16,
    .write_control = sh_cmt_write16,
    .read_count = sh_cmt_read16,
    .write_count = sh_cmt_write16,
    },
    [SH_CMT_32BIT] = {
    .model = SH_CMT_32BIT,
    .width = 32,
    .overflow_bit = SH_CMT32_CMCSR_CMF,
    .clear_bits = ~(SH_CMT32_CMCSR_CMF | SH_CMT32_CMCSR_OVF),
    .read_control = sh_cmt_read16,
    .write_control = sh_cmt_write16,
    .read_count = sh_cmt_read32,
    .write_count = sh_cmt_write32,
    },
    [SH_CMT_48BIT] = {
    .model = SH_CMT_48BIT,
    .channels_mask = 0x3f,
    .width = 32,
    .overflow_bit = SH_CMT32_CMCSR_CMF,
    .clear_bits = ~(SH_CMT32_CMCSR_CMF | SH_CMT32_CMCSR_OVF),
    .read_control = sh_cmt_read32,
    .write_control = sh_cmt_write32,
    .read_count = sh_cmt_read32,
    .write_count = sh_cmt_write32,
    },
    [SH_CMT0_RCAR_GEN2] = {
    .model = SH_CMT0_RCAR_GEN2,
    .channels_mask = 0x60,
    .width = 32,
    .overflow_bit = SH_CMT32_CMCSR_CMF,
    .clear_bits = ~(SH_CMT32_CMCSR_CMF | SH_CMT32_CMCSR_OVF),
    .read_control = sh_cmt_read32,
    .write_control = sh_cmt_write32,
    .read_count = sh_cmt_read32,
    .write_count = sh_cmt_write32,
    },
    [SH_CMT1_RCAR_GEN2] = {
    .model = SH_CMT1_RCAR_GEN2,
    .channels_mask = 0xff,
    .width = 32,
    .overflow_bit = SH_CMT32_CMCSR_CMF,
    .clear_bits = ~(SH_CMT32_CMCSR_CMF | SH_CMT32_CMCSR_OVF),
    .read_control = sh_cmt_read32,
    .write_control = sh_cmt_write32,
    .read_count = sh_cmt_read32,
    .write_count = sh_cmt_write32,
    },
    };

pub const CMCLKE: c_uint = 0x1000	/* CLK Enable Register (R-Car Gen2) */;
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_read_cmstr(ch: *mut sh_cmt_channel) -> u32 {
    static inline u32 sh_cmt_read_cmstr(struct sh_cmt_channel *ch)
    {
    if (ch.iostart)
    return ch.cmt.info.read_control(ch.iostart, 0);
    else
    return ch.cmt.info.read_control(ch.cmt.mapbase, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_write_cmstr(ch: *mut sh_cmt_channel, value: u32) {
    static inline void sh_cmt_write_cmstr(struct sh_cmt_channel *ch, u32 value)
    {
    let mut old_value: u32 = sh_cmt_read_cmstr(ch);
    if (value != old_value) {
    if (ch.iostart) {
    ch.cmt.info.write_control(ch.iostart, 0, value);
    udelay(ch.cmt.reg_delay);
    } else {
    ch.cmt.info.write_control(ch.cmt.mapbase, 0, value);
    udelay(ch.cmt.reg_delay);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_read_cmcsr(ch: *mut sh_cmt_channel) -> u32 {
    static inline u32 sh_cmt_read_cmcsr(struct sh_cmt_channel *ch)
    {
    return ch.cmt.info.read_control(ch.ioctrl, CMCSR);
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_write_cmcsr(ch: *mut sh_cmt_channel, value: u32) {
    static inline void sh_cmt_write_cmcsr(struct sh_cmt_channel *ch, u32 value)
    {
    let mut old_value: u32 = sh_cmt_read_cmcsr(ch);
    if (value != old_value) {
    ch.cmt.info.write_control(ch.ioctrl, CMCSR, value);
    udelay(ch.cmt.reg_delay);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_read_cmcnt(ch: *mut sh_cmt_channel) -> u32 {
    static inline u32 sh_cmt_read_cmcnt(struct sh_cmt_channel *ch)
    {
    return ch.cmt.info.read_count(ch.ioctrl, CMCNT);
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_write_cmcnt(ch: *mut sh_cmt_channel, value: u32) -> c_int {
    static inline int sh_cmt_write_cmcnt(struct sh_cmt_channel *ch, u32 value)
    {
// Tests showed that we need to wait 3 clocks here
    let mut cmcnt_delay: c_uint = DIV_ROUND_UP(3 * ch.cmt.reg_delay, 2);
    u32 reg;
    if (ch.cmt.info.model > SH_CMT_16BIT) {
    int ret = read_poll_timeout_atomic(sh_cmt_read_cmcsr, reg,
    !(reg & SH_CMT32_CMCSR_WRFLG),
    1, cmcnt_delay, false, ch);
    if (ret < 0)
    return ret;
    }
    ch.cmt.info.write_count(ch.ioctrl, CMCNT, value);
    udelay(cmcnt_delay);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sh_cmt_write_cmcor(ch: *mut sh_cmt_channel, value: u32) {
    static inline void sh_cmt_write_cmcor(struct sh_cmt_channel *ch, u32 value)
    {
    let mut old_value: u32 = ch.cmt.info.read_count(ch.ioctrl, CMCOR);
    if (value != old_value) {
    ch.cmt.info.write_count(ch.ioctrl, CMCOR, value);
    udelay(ch.cmt.reg_delay);
    }
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_get_counter(ch: *mut sh_cmt_channel, has_wrapped: *mut u32) -> u32 {
    static u32 sh_cmt_get_counter(struct sh_cmt_channel *ch, u32 *has_wrapped)
    {
    u32 v1, v2, v3;
    u32 o1, o2;
    o1 = sh_cmt_read_cmcsr(ch) & ch.cmt.info.overflow_bit;
// Make sure the timer value is stable. Stolen from acpi_pm.c
    do {
    o2 = o1;
    v1 = sh_cmt_read_cmcnt(ch);
    v2 = sh_cmt_read_cmcnt(ch);
    v3 = sh_cmt_read_cmcnt(ch);
    o1 = sh_cmt_read_cmcsr(ch) & ch.cmt.info.overflow_bit;
    } while (unlikely((o1 != o2) || (v1 > v2 && v1 < v3)
    || (v2 > v3 && v2 < v1) || (v3 > v1 && v3 < v2)));
// has_wrapped = o1;
    return v2;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_start_stop_ch(ch: *mut sh_cmt_channel, start: c_int) {
    static void sh_cmt_start_stop_ch(struct sh_cmt_channel *ch, int start)
    {
    unsigned long flags;
    u32 value;
// start stop register shared by multiple timer channels
    raw_spin_lock_irqsave(&ch.cmt.lock, flags);
    value = sh_cmt_read_cmstr(ch);
    if (start)
    value |= 1 << ch.timer_bit;
    else
    value &= ~(1 << ch.timer_bit);
    sh_cmt_write_cmstr(ch, value);
    raw_spin_unlock_irqrestore(&ch.cmt.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_enable(ch: *mut sh_cmt_channel) -> c_int {
    static int sh_cmt_enable(struct sh_cmt_channel *ch)
    {
    int ret;
    dev_pm_syscore_device(&ch.cmt.pdev.dev, true);
// make sure channel is disabled
    sh_cmt_start_stop_ch(ch, 0);
// configure channel, periodic mode and maximum timeout
    if (ch.cmt.info.width == 16) {
    sh_cmt_write_cmcsr(ch, SH_CMT16_CMCSR_CMIE |
    SH_CMT16_CMCSR_CKS512);
    } else {
    u32 cmtout = ch.cmt.info.model <= SH_CMT_48BIT ?
    SH_CMT32_CMCSR_CMTOUT_IE : 0;
    sh_cmt_write_cmcsr(ch, cmtout | SH_CMT32_CMCSR_CMM |
    SH_CMT32_CMCSR_CMR_IRQ |
    SH_CMT32_CMCSR_CKS_RCLK8);
    }
    sh_cmt_write_cmcor(ch, 0xffffffff);
    ret = sh_cmt_write_cmcnt(ch, 0);
    if (ret || sh_cmt_read_cmcnt(ch)) {
    dev_err(&ch.cmt.pdev.dev, "ch%u: cannot clear CMCNT\n",
    ch.index);
    return -ETIMEDOUT;
    }
// enable channel
    sh_cmt_start_stop_ch(ch, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_disable(ch: *mut sh_cmt_channel) {
    static void sh_cmt_disable(struct sh_cmt_channel *ch)
    {
// disable channel
    sh_cmt_start_stop_ch(ch, 0);
// disable interrupts in CMT block
    sh_cmt_write_cmcsr(ch, 0);
    dev_pm_syscore_device(&ch.cmt.pdev.dev, false);
    }
// private flags

    static void sh_cmt_clock_event_program_verify(struct sh_cmt_channel *ch,
    int absolute)
    {
    let mut value: u32 = ch.next_match_value;
    u32 new_match;
    let mut delay: u32 = 0;
    let mut now: u32 = 0;
    u32 has_wrapped;
    now = sh_cmt_get_counter(ch, &has_wrapped);
    ch.flags |= FLAG_REPROGRAM; /* force reprogram */
    if (has_wrapped) {
// we're competing with the interrupt handler.
// -> let the interrupt handler reprogram the timer.
// -> interrupt number two handles the event.
//
    ch.flags |= FLAG_SKIPEVENT;
    return;
    }
    if (absolute)
    now = 0;
    do {
// reprogram the timer hardware,
// but don't save the new match value yet.
//
    new_match = now + value + delay;
    if (new_match > ch.max_match_value)
    new_match = ch.max_match_value;
    sh_cmt_write_cmcor(ch, new_match);
    now = sh_cmt_get_counter(ch, &has_wrapped);
    if (has_wrapped && (new_match > ch.match_value)) {
// we are changing to a greater match value,
// so this wrap must be caused by the counter
// matching the old value.
// -> first interrupt reprograms the timer.
// -> interrupt number two handles the event.
//
    ch.flags |= FLAG_SKIPEVENT;
    break;
    }
    if (has_wrapped) {
// we are changing to a smaller match value,
// so the wrap must be caused by the counter
// matching the new value.
// -> save programmed match value.
// -> let isr handle the event.
//
    ch.match_value = new_match;
    break;
    }
// be safe: verify hardware settings
    if (now < new_match) {
// timer value is below match value, all good.
// this makes sure we won't miss any match events.
// -> save programmed match value.
// -> let isr handle the event.
//
    ch.match_value = new_match;
    break;
    }
// the counter has reached a value greater
// than our new match value. and since the
// has_wrapped flag isn't set we must have
// programmed a too close event.
// -> increase delay and retry.
//
    if (delay)
    delay <<= 1;
    else
    delay = 1;
    if (!delay)
    dev_warn(&ch.cmt.pdev.dev, "ch%u: too long delay\n",
    ch.index);
    } while (delay);
    }
#[no_mangle]
unsafe extern "C" fn __sh_cmt_set_next(ch: *mut sh_cmt_channel, delta: c_ulong) {
    static void __sh_cmt_set_next(struct sh_cmt_channel *ch, unsigned long delta)
    {
    if (delta > ch.max_match_value)
    dev_warn(&ch.cmt.pdev.dev, "ch%u: delta out of range\n",
    ch.index);
    ch.next_match_value = delta;
    sh_cmt_clock_event_program_verify(ch, 0);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_set_next(ch: *mut sh_cmt_channel, delta: c_ulong) {
    static void sh_cmt_set_next(struct sh_cmt_channel *ch, unsigned long delta)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&ch.lock, flags);
    __sh_cmt_set_next(ch, delta);
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_cmt_interrupt(int irq, void *dev_id)
    {
    struct sh_cmt_channel *ch = dev_id;
    unsigned long flags;
// clear flags
    sh_cmt_write_cmcsr(ch, sh_cmt_read_cmcsr(ch) &
    ch.cmt.info.clear_bits);
// update clock source counter to begin with if enabled
// the wrap flag should be cleared by the timer specific
// isr before we end up here.
//
    if (ch.flags & FLAG_CLOCKSOURCE)
    ch.total_cycles += ch.match_value + 1;
    if (!(ch.flags & FLAG_REPROGRAM))
    ch.next_match_value = ch.max_match_value;
    ch.flags |= FLAG_IRQCONTEXT;
    if (ch.flags & FLAG_CLOCKEVENT) {
    if (!(ch.flags & FLAG_SKIPEVENT)) {
    if (clockevent_state_oneshot(&ch.ced)) {
    ch.next_match_value = ch.max_match_value;
    ch.flags |= FLAG_REPROGRAM;
    }
    ch.ced.event_handler(&ch.ced);
    }
    }
    ch.flags &= ~FLAG_SKIPEVENT;
    raw_spin_lock_irqsave(&ch.lock, flags);
    if (ch.flags & FLAG_REPROGRAM) {
    ch.flags &= ~FLAG_REPROGRAM;
    sh_cmt_clock_event_program_verify(ch, 1);
    if (ch.flags & FLAG_CLOCKEVENT)
    if ((clockevent_state_shutdown(&ch.ced))
    || (ch.match_value == ch.next_match_value))
    ch.flags &= ~FLAG_REPROGRAM;
    }
    ch.flags &= ~FLAG_IRQCONTEXT;
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_start_clocksource(ch: *mut sh_cmt_channel) -> c_int {
    static int sh_cmt_start_clocksource(struct sh_cmt_channel *ch)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    raw_spin_lock_irqsave(&ch.lock, flags);
    if (!(ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE)))
    ret = sh_cmt_enable(ch);
    if (ret)
    goto out;
    ch.flags |= FLAG_CLOCKSOURCE;
// setup timeout if no clockevent
    if (ch.cmt.num_channels == 1 && !(ch.flags & FLAG_CLOCKEVENT))
    __sh_cmt_set_next(ch, ch.max_match_value);
    out:
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_stop_clocksource(ch: *mut sh_cmt_channel) {
    static void sh_cmt_stop_clocksource(struct sh_cmt_channel *ch)
    {
    unsigned long flags;
    unsigned long f;
    raw_spin_lock_irqsave(&ch.lock, flags);
    f = ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE);
    ch.flags &= ~FLAG_CLOCKSOURCE;
    if (f && !(ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE)))
    sh_cmt_disable(ch);
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_start_clockevent(ch: *mut sh_cmt_channel) -> c_int {
    static int sh_cmt_start_clockevent(struct sh_cmt_channel *ch)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    raw_spin_lock_irqsave(&ch.lock, flags);
    if (!(ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE)))
    ret = sh_cmt_enable(ch);
    if (ret)
    goto out;
    ch.flags |= FLAG_CLOCKEVENT;
    out:
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_stop_clockevent(ch: *mut sh_cmt_channel) {
    static void sh_cmt_stop_clockevent(struct sh_cmt_channel *ch)
    {
    unsigned long flags;
    unsigned long f;
    raw_spin_lock_irqsave(&ch.lock, flags);
    f = ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE);
    ch.flags &= ~FLAG_CLOCKEVENT;
    if (f && !(ch.flags & (FLAG_CLOCKEVENT | FLAG_CLOCKSOURCE)))
    sh_cmt_disable(ch);
// adjust the timeout to maximum if only clocksource left
    if (ch.flags & FLAG_CLOCKSOURCE)
    __sh_cmt_set_next(ch, ch.max_match_value);
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    }
    static struct sh_cmt_channel *cs_to_sh_cmt(struct clocksource *cs)
    {
    return container_of(cs, struct sh_cmt_channel, cs);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 sh_cmt_clocksource_read(struct clocksource *cs)
    {
    struct sh_cmt_channel *ch = cs_to_sh_cmt(cs);
    u32 has_wrapped;
    if (ch.cmt.num_channels == 1) {
    unsigned long flags;
    u64 value;
    u32 raw;
    raw_spin_lock_irqsave(&ch.lock, flags);
    value = ch.total_cycles;
    raw = sh_cmt_get_counter(ch, &has_wrapped);
    if (unlikely(has_wrapped))
    raw += ch.match_value + 1;
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    return value + raw;
    }
    return sh_cmt_get_counter(ch, &has_wrapped);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clocksource_enable(cs: *mut clocksource) -> c_int {
    static int sh_cmt_clocksource_enable(struct clocksource *cs)
    {
    int ret;
    struct sh_cmt_channel *ch = cs_to_sh_cmt(cs);
    WARN_ON(ch.cs_enabled);
    ch.total_cycles = 0;
    ret = sh_cmt_start_clocksource(ch);
    if (!ret)
    ch.cs_enabled = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clocksource_disable(cs: *mut clocksource) {
    static void sh_cmt_clocksource_disable(struct clocksource *cs)
    {
    struct sh_cmt_channel *ch = cs_to_sh_cmt(cs);
    WARN_ON(!ch.cs_enabled);
    sh_cmt_stop_clocksource(ch);
    ch.cs_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clocksource_suspend(cs: *mut clocksource) {
    static void sh_cmt_clocksource_suspend(struct clocksource *cs)
    {
    struct sh_cmt_channel *ch = cs_to_sh_cmt(cs);
    if (!ch.cs_enabled)
    return;
    sh_cmt_stop_clocksource(ch);
    dev_pm_genpd_suspend(&ch.cmt.pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clocksource_resume(cs: *mut clocksource) {
    static void sh_cmt_clocksource_resume(struct clocksource *cs)
    {
    struct sh_cmt_channel *ch = cs_to_sh_cmt(cs);
    if (!ch.cs_enabled)
    return;
    dev_pm_genpd_resume(&ch.cmt.pdev.dev);
    sh_cmt_start_clocksource(ch);
    }
    static int sh_cmt_register_clocksource(struct sh_cmt_channel *ch,
    const char *name)
    {
    struct clocksource *cs = &ch.cs;
    cs.name = name;
    cs.rating = 125;
    cs.read = sh_cmt_clocksource_read;
    cs.enable = sh_cmt_clocksource_enable;
    cs.disable = sh_cmt_clocksource_disable;
    cs.suspend = sh_cmt_clocksource_suspend;
    cs.resume = sh_cmt_clocksource_resume;
    cs.mask = CLOCKSOURCE_MASK(ch.cmt.info.width);
    cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    dev_info(&ch.cmt.pdev.dev, "ch%u: used as clock source\n",
    ch.index);
    clocksource_register_hz(cs, ch.cmt.rate);
    return 0;
    }
    static struct sh_cmt_channel *ced_to_sh_cmt(struct clock_event_device *ced)
    {
    return container_of(ced, struct sh_cmt_channel, ced);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_start(ch: *mut sh_cmt_channel, periodic: c_int) {
    static void sh_cmt_clock_event_start(struct sh_cmt_channel *ch, int periodic)
    {
    sh_cmt_start_clockevent(ch);
    if (periodic)
    sh_cmt_set_next(ch, ((ch.cmt.rate + HZ/2) / HZ) - 1);
    else
    sh_cmt_set_next(ch, ch.max_match_value);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_shutdown(ced: *mut clock_event_device) -> c_int {
    static int sh_cmt_clock_event_shutdown(struct clock_event_device *ced)
    {
    struct sh_cmt_channel *ch = ced_to_sh_cmt(ced);
    sh_cmt_stop_clockevent(ch);
    return 0;
    }
    static int sh_cmt_clock_event_set_state(struct clock_event_device *ced,
    int periodic)
    {
    struct sh_cmt_channel *ch = ced_to_sh_cmt(ced);
// deal with old setting first
    if (clockevent_state_oneshot(ced) || clockevent_state_periodic(ced))
    sh_cmt_stop_clockevent(ch);
    dev_info(&ch.cmt.pdev.dev, "ch%u: used for %s clock events\n",
    ch.index, periodic ? "periodic" : "oneshot");
    sh_cmt_clock_event_start(ch, periodic);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_set_oneshot(ced: *mut clock_event_device) -> c_int {
    static int sh_cmt_clock_event_set_oneshot(struct clock_event_device *ced)
    {
    return sh_cmt_clock_event_set_state(ced, 0);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_set_periodic(ced: *mut clock_event_device) -> c_int {
    static int sh_cmt_clock_event_set_periodic(struct clock_event_device *ced)
    {
    return sh_cmt_clock_event_set_state(ced, 1);
    }
    static int sh_cmt_clock_event_next(unsigned long delta,
    struct clock_event_device *ced)
    {
    struct sh_cmt_channel *ch = ced_to_sh_cmt(ced);
    unsigned long flags;
    BUG_ON(!clockevent_state_oneshot(ced));
    raw_spin_lock_irqsave(&ch.lock, flags);
    if (likely(ch.flags & FLAG_IRQCONTEXT))
    ch.next_match_value = delta - 1;
    else
    __sh_cmt_set_next(ch, delta - 1);
    raw_spin_unlock_irqrestore(&ch.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_suspend(ced: *mut clock_event_device) {
    static void sh_cmt_clock_event_suspend(struct clock_event_device *ced)
    {
    struct sh_cmt_channel *ch = ced_to_sh_cmt(ced);
    dev_pm_genpd_suspend(&ch.cmt.pdev.dev);
    clk_unprepare(ch.cmt.clk);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_clock_event_resume(ced: *mut clock_event_device) {
    static void sh_cmt_clock_event_resume(struct clock_event_device *ced)
    {
    struct sh_cmt_channel *ch = ced_to_sh_cmt(ced);
    clk_prepare(ch.cmt.clk);
    dev_pm_genpd_resume(&ch.cmt.pdev.dev);
    }
    static int sh_cmt_register_clockevent(struct sh_cmt_channel *ch,
    const char *name)
    {
    struct clock_event_device *ced = &ch.ced;
    int irq;
    int ret;
    irq = platform_get_irq(ch.cmt.pdev, ch.index);
    if (irq < 0)
    return irq;
    ret = request_irq(irq, sh_cmt_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL | IRQF_NOBALANCING,
    dev_name(&ch.cmt.pdev.dev), ch);
    if (ret) {
    dev_err(&ch.cmt.pdev.dev, "ch%u: failed to request irq %d\n",
    ch.index, irq);
    return ret;
    }
    ced.name = name;
    ced.features = CLOCK_EVT_FEAT_PERIODIC;
    ced.features |= CLOCK_EVT_FEAT_ONESHOT;
    ced.rating = 125;
    ced.cpumask = cpu_possible_mask;
    ced.set_next_event = sh_cmt_clock_event_next;
    ced.set_state_shutdown = sh_cmt_clock_event_shutdown;
    ced.set_state_periodic = sh_cmt_clock_event_set_periodic;
    ced.set_state_oneshot = sh_cmt_clock_event_set_oneshot;
    ced.suspend = sh_cmt_clock_event_suspend;
    ced.resume = sh_cmt_clock_event_resume;
// TODO: calculate good shift from rate and counter bit width
    ced.shift = 32;
    ced.mult = div_sc(ch.cmt.rate, NSEC_PER_SEC, ced.shift);
    ced.max_delta_ns = clockevent_delta2ns(ch.max_match_value, ced);
    ced.max_delta_ticks = ch.max_match_value;
    ced.min_delta_ns = clockevent_delta2ns(0x1f, ced);
    ced.min_delta_ticks = 0x1f;
    dev_info(&ch.cmt.pdev.dev, "ch%u: used for clock events\n",
    ch.index);
    clockevents_register_device(ced);
    return 0;
    }
    static int sh_cmt_register(struct sh_cmt_channel *ch, const char *name,
    bool clockevent, bool clocksource)
    {
    int ret;
    if (clockevent) {
    ch.cmt.has_clockevent = true;
    ret = sh_cmt_register_clockevent(ch, name);
    if (ret < 0)
    return ret;
    }
    if (clocksource) {
    ch.cmt.has_clocksource = true;
    sh_cmt_register_clocksource(ch, name);
    }
    return 0;
    }
    static int sh_cmt_setup_channel(struct sh_cmt_channel *ch, unsigned int index,
    unsigned int hwidx, bool clockevent,
    bool clocksource, struct sh_cmt_device *cmt)
    {
    u32 value;
    int ret;
// Skip unused channels.
    if (!clockevent && !clocksource)
    return 0;
    ch.cmt = cmt;
    ch.index = index;
    ch.hwidx = hwidx;
    ch.timer_bit = hwidx;
//
// Compute the address of the channel control register block. For the
// timers with a per-channel start/stop register, compute its address
// as well.
//
    switch (cmt.info.model) {
    case SH_CMT_16BIT:
    ch.ioctrl = cmt.mapbase + 2 + ch.hwidx * 6;
    break;
    case SH_CMT_32BIT:
    case SH_CMT_48BIT:
    ch.ioctrl = cmt.mapbase + 0x10 + ch.hwidx * 0x10;
    break;
    case SH_CMT0_RCAR_GEN2:
    case SH_CMT1_RCAR_GEN2:
    ch.iostart = cmt.mapbase + ch.hwidx * 0x100;
    ch.ioctrl = ch.iostart + 0x10;
    ch.timer_bit = 0;
// Enable the clock supply to the channel
    value = ioread32(cmt.mapbase + CMCLKE);
    value |= BIT(hwidx);
    iowrite32(value, cmt.mapbase + CMCLKE);
    break;
    }
    if (cmt.info.width == (sizeof(ch.max_match_value) * 8))
    ch.max_match_value = ~0;
    else
    ch.max_match_value = (1 << cmt.info.width) - 1;
    ch.match_value = ch.max_match_value;
    raw_spin_lock_init(&ch.lock);
    ret = sh_cmt_register(ch, dev_name(&cmt.pdev.dev),
    clockevent, clocksource);
    if (ret) {
    dev_err(&cmt.pdev.dev, "ch%u: registration failed\n",
    ch.index);
    return ret;
    }
    ch.cs_enabled = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_map_memory(cmt: *mut sh_cmt_device) -> c_int {
    static int sh_cmt_map_memory(struct sh_cmt_device *cmt)
    {
    struct resource *mem;
    mem = platform_get_resource(cmt.pdev, IORESOURCE_MEM, 0);
    if (!mem) {
    dev_err(&cmt.pdev.dev, "failed to get I/O memory\n");
    return -ENXIO;
    }
    cmt.mapbase = ioremap(mem.start, resource_size(mem));
    if (cmt.mapbase == core::ptr::null_mut()) {
    dev_err(&cmt.pdev.dev, "failed to remap I/O memory\n");
    return -ENXIO;
    }
    return 0;
    }
    static const struct platform_device_id sh_cmt_id_table[] = {
    { .name = "sh-cmt-16", .driver_data = (kernel_ulong_t)&sh_cmt_info[SH_CMT_16BIT] },
    { .name = "sh-cmt-32", .driver_data = (kernel_ulong_t)&sh_cmt_info[SH_CMT_32BIT] },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sh_cmt_id_table);
    static const struct of_device_id sh_cmt_of_table[] __maybe_unused = {
    {
// deprecated, preserved for backward compatibility
    .compatible = "renesas,cmt-48",
    .data = &sh_cmt_info[SH_CMT_48BIT]
    },
    {
// deprecated, preserved for backward compatibility
    .compatible = "renesas,cmt-48-gen2",
    .data = &sh_cmt_info[SH_CMT0_RCAR_GEN2]
    },
    {
    .compatible = "renesas,r8a7740-cmt1",
    .data = &sh_cmt_info[SH_CMT_48BIT]
    },
    {
    .compatible = "renesas,sh73a0-cmt1",
    .data = &sh_cmt_info[SH_CMT_48BIT]
    },
    {
    .compatible = "renesas,rcar-gen2-cmt0",
    .data = &sh_cmt_info[SH_CMT0_RCAR_GEN2]
    },
    {
    .compatible = "renesas,rcar-gen2-cmt1",
    .data = &sh_cmt_info[SH_CMT1_RCAR_GEN2]
    },
    {
    .compatible = "renesas,rcar-gen3-cmt0",
    .data = &sh_cmt_info[SH_CMT0_RCAR_GEN2]
    },
    {
    .compatible = "renesas,rcar-gen3-cmt1",
    .data = &sh_cmt_info[SH_CMT1_RCAR_GEN2]
    },
    {
    .compatible = "renesas,rcar-gen4-cmt0",
    .data = &sh_cmt_info[SH_CMT0_RCAR_GEN2]
    },
    {
    .compatible = "renesas,rcar-gen4-cmt1",
    .data = &sh_cmt_info[SH_CMT1_RCAR_GEN2]
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, sh_cmt_of_table);
#[no_mangle]
unsafe extern "C" fn sh_cmt_setup(cmt: *mut sh_cmt_device, pdev: *mut platform_device) -> c_int {
    static int sh_cmt_setup(struct sh_cmt_device *cmt, struct platform_device *pdev)
    {
    unsigned int mask, i;
    unsigned long rate;
    int ret;
    cmt.pdev = pdev;
    raw_spin_lock_init(&cmt.lock);
    if (IS_ENABLED(CONFIG_OF) && pdev.dev.of_node) {
    cmt.info = of_device_get_match_data(&pdev.dev);
    cmt.hw_channels = cmt.info.channels_mask;
    } else if (pdev.dev.platform_data) {
    struct sh_timer_config *cfg = pdev.dev.platform_data;
    const struct platform_device_id *id = pdev.id_entry;
    cmt.info = (const struct sh_cmt_info *)id.driver_data;
    cmt.hw_channels = cfg.channels_mask;
    } else {
    dev_err(&cmt.pdev.dev, "missing platform data\n");
    return -ENXIO;
    }
// Get hold of clock.
    cmt.clk = clk_get(&cmt.pdev.dev, "fck");
    if (IS_ERR(cmt.clk)) {
    dev_err(&cmt.pdev.dev, "cannot get clock\n");
    return PTR_ERR(cmt.clk);
    }
    ret = clk_prepare(cmt.clk);
    if (ret < 0)
    goto err_clk_put;
// Determine clock rate.
    ret = clk_enable(cmt.clk);
    if (ret < 0)
    goto err_clk_unprepare;
    rate = clk_get_rate(cmt.clk);
    if (!rate) {
    ret = -EINVAL;
    goto err_clk_disable;
    }
// We shall wait 2 input clks after register writes
    if (cmt.info.model >= SH_CMT_48BIT)
    cmt.reg_delay = DIV_ROUND_UP(2UL * USEC_PER_SEC, rate);
    cmt.rate = rate / (cmt.info.width == 16 ? 512 : 8);
// Map the memory resource(s).
    ret = sh_cmt_map_memory(cmt);
    if (ret < 0)
    goto err_clk_disable;
// Allocate and setup the channels.
    cmt.num_channels = hweight8(cmt.hw_channels);
    cmt.channels = kzalloc_objs(*cmt.channels, cmt.num_channels);
    if (cmt.channels == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err_unmap;
    }
//
// Use the first channel as a clock event device and the second channel
// as a clock source. If only one channel is available use it for both.
//
    for (i = 0, mask = cmt.hw_channels; i < cmt.num_channels; ++i) {
    let mut hwidx: c_uint = ffs(mask) - 1;
    let mut clocksource: bool = i == 1 || cmt.num_channels == 1;
    let mut clockevent: bool = i == 0;
    ret = sh_cmt_setup_channel(&cmt.channels[i], i, hwidx,
    clockevent, clocksource, cmt);
    if (ret < 0)
    goto err_unmap;
    mask &= ~(1 << hwidx);
    }
    platform_set_drvdata(pdev, cmt);
    return 0;
    err_unmap:
    kfree(cmt.channels);
    iounmap(cmt.mapbase);
    err_clk_disable:
    clk_disable(cmt.clk);
    err_clk_unprepare:
    clk_unprepare(cmt.clk);
    err_clk_put:
    clk_put(cmt.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_probe(pdev: *mut platform_device) -> c_int {
    static int sh_cmt_probe(struct platform_device *pdev)
    {
    struct sh_cmt_device *cmt = platform_get_drvdata(pdev);
    int ret;
    if (!is_sh_early_platform_device(pdev)) {
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    }
    if (cmt) {
    dev_info(&pdev.dev, "kept as earlytimer\n");
    goto out;
    }
    cmt = kzalloc_obj(*cmt);
    if (cmt == core::ptr::null_mut())
    return -ENOMEM;
    ret = sh_cmt_setup(cmt, pdev);
    if (ret) {
    kfree(cmt);
    pm_runtime_idle(&pdev.dev);
    return ret;
    }
    if (is_sh_early_platform_device(pdev))
    return 0;
    out:
    if (cmt.has_clockevent || cmt.has_clocksource)
    pm_runtime_irq_safe(&pdev.dev);
    return 0;
    }
    static struct platform_driver sh_cmt_device_driver = {
    .probe		= sh_cmt_probe,
    .driver		= {
    .name	= "sh_cmt",
    .of_match_table = of_match_ptr(sh_cmt_of_table),
    .suppress_bind_attrs = true,
    },
    .id_table	= sh_cmt_id_table,
    };
#[no_mangle]
unsafe extern "C" fn sh_cmt_init() -> int __init {
    static int __init sh_cmt_init(void)
    {
    return platform_driver_register(&sh_cmt_device_driver);
    }
#[no_mangle]
unsafe extern "C" fn sh_cmt_exit() -> void __exit {
    static void __exit sh_cmt_exit(void)
    {
    platform_driver_unregister(&sh_cmt_device_driver);
    }

    sh_early_platform_init("earlytimer", &sh_cmt_device_driver);

    subsys_initcall(sh_cmt_init);
    module_exit(sh_cmt_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("SuperH CMT Timer Driver");
