//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/speakup_keypc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// written by David Borowski
//
// Copyright (C) 2003 David Borowski.
//
// specifically written as a driver for the speakup screenreview
// package it's not a general device driver.
// This driver is for the Keynote Gold internal synthesizer.
//

pub const SYNTH_IO_EXTENT: c_uint = 0x04;

pub const PROCSPEECH: c_uint = 0x1f;
pub const SYNTH_CLEAR: c_uint = 0x03;
    static int synth_probe(struct spk_synth *synth);
    static void keynote_release(struct spk_synth *synth);
    static const char *synth_immediate(struct spk_synth *synth, const char *buf);
    static void do_catch_up(struct spk_synth *synth);
    static void synth_flush(struct spk_synth *synth);
    static int synth_port;
    static int port_forced;
    static unsigned int synth_portlist[] = { 0x2a8, 0 };
    enum default_vars_id {
    CAPS_START_ID = 0, CAPS_STOP_ID,
    RATE_ID, PITCH_ID,
    DIRECT_ID, V_LAST_VAR_ID,
    NB_ID
    };
    static struct var_t vars[NB_ID] = {
    [CAPS_START_ID] = { CAPS_START, .u.s = {"[f130]" } },
    [CAPS_STOP_ID] = { CAPS_STOP, .u.s = {"[f90]" } },
    [RATE_ID] = { RATE, .u.n = {"\04%c ", 8, 0, 10, 81, -8, core::ptr::null_mut() } },
    [PITCH_ID] = { PITCH, .u.n = {"[f%d]", 5, 0, 9, 40, 10, core::ptr::null_mut() } },
    [DIRECT_ID] = { DIRECT, .u.n = {core::ptr::null_mut(), 0, 0, 1, 0, 0, core::ptr::null_mut() } },
    V_LAST_VAR
    };
//
// These attributes will appear in /sys/accessibility/speakup/keypc.
//
    static struct kobj_attribute caps_start_attribute =
    __ATTR(caps_start, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute caps_stop_attribute =
    __ATTR(caps_stop, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute pitch_attribute =
    __ATTR(pitch, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute rate_attribute =
    __ATTR(rate, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute delay_time_attribute =
    __ATTR(delay_time, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute direct_attribute =
    __ATTR(direct, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute full_time_attribute =
    __ATTR(full_time, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute jiffy_delta_attribute =
    __ATTR(jiffy_delta, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute trigger_time_attribute =
    __ATTR(trigger_time, 0644, spk_var_show, spk_var_store);
//
// Create a group of attributes so that we can create and destroy them all
// at once.
//
    static struct attribute *synth_attrs[] = {
    &caps_start_attribute.attr,
    &caps_stop_attribute.attr,
    &pitch_attribute.attr,
    &rate_attribute.attr,
    &delay_time_attribute.attr,
    &direct_attribute.attr,
    &full_time_attribute.attr,
    &jiffy_delta_attribute.attr,
    &trigger_time_attribute.attr,
    core::ptr::null_mut(),	/* need to core::ptr::null_mut() terminate the list of attributes */
    };
    static struct spk_synth synth_keypc = {
    .name = "keypc",
    .version = DRV_VERSION,
    .long_name = "Keynote PC",
    .init = "[t][n7,1][n8,0]",
    .procspeech = PROCSPEECH,
    .clear = SYNTH_CLEAR,
    .delay = 500,
    .trigger = 50,
    .jiffies = 50,
    .full = 1000,
    .startup = SYNTH_START,
    .checkval = SYNTH_CHECK,
    .vars = vars,
    .io_ops = &spk_serial_io_ops,
    .probe = synth_probe,
    .release = keynote_release,
    .synth_immediate = synth_immediate,
    .catch_up = do_catch_up,
    .flush = synth_flush,
    .is_alive = spk_synth_is_alive_nop,
    .synth_adjust = core::ptr::null_mut(),
    .read_buff_add = core::ptr::null_mut(),
    .get_index = core::ptr::null_mut(),
    .indexing = {
    .command = core::ptr::null_mut(),
    .lowindex = 0,
    .highindex = 0,
    .currindex = 0,
    },
    .attributes = {
    .attrs = synth_attrs,
    .name = "keypc",
    },
    };
#[no_mangle]
pub unsafe extern "C" fn synth_writable() -> bool {
    static inline bool synth_writable(void)
    {
    return (inb_p(synth_port + UART_RX) & 0x10) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_full() -> bool {
    static inline bool synth_full(void)
    {
    return (inb_p(synth_port + UART_RX) & 0x80) == 0;
    }
    static char *oops(void)
    {
    int s1, s2, s3, s4;
    s1 = inb_p(synth_port);
    s2 = inb_p(synth_port + 1);
    s3 = inb_p(synth_port + 2);
    s4 = inb_p(synth_port + 3);
    pr_warn("synth timeout %d %d %d %d\n", s1, s2, s3, s4);
    return core::ptr::null_mut();
    }
    static const char *synth_immediate(struct spk_synth *synth, const char *buf)
    {
    u_char ch;
    int timeout;
    while ((ch = *buf)) {
    if (ch == '\n')
    ch = PROCSPEECH;
    if (synth_full())
    return buf;
    timeout = 1000;
    while (synth_writable())
    if (--timeout <= 0)
    return oops();
    outb_p(ch, synth_port);
    udelay(70);
    buf++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn do_catch_up(synth: *mut spk_synth) {
    static void do_catch_up(struct spk_synth *synth)
    {
    u_char ch;
    int timeout;
    unsigned long flags;
    unsigned long jiff_max;
    struct var_t *jiffy_delta;
    struct var_t *delay_time;
    struct var_t *full_time;
    int delay_time_val;
    int full_time_val;
    int jiffy_delta_val;
    jiffy_delta = spk_get_var(JIFFY);
    delay_time = spk_get_var(DELAY);
    full_time = spk_get_var(FULL);
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    jiffy_delta_val = jiffy_delta.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    jiff_max = jiffies + jiffy_delta_val;
    while (!kthread_should_stop()) {
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    if (speakup_info.flushing) {
    speakup_info.flushing = 0;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    synth.flush(synth);
    continue;
    }
    synth_buffer_skip_nonlatin1();
    if (synth_buffer_empty()) {
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    break;
    }
    set_current_state(TASK_INTERRUPTIBLE);
    full_time_val = full_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (synth_full()) {
    schedule_timeout(msecs_to_jiffies(full_time_val));
    continue;
    }
    set_current_state(TASK_RUNNING);
    timeout = 1000;
    while (synth_writable())
    if (--timeout <= 0)
    break;
    if (timeout <= 0) {
    oops();
    break;
    }
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    ch = synth_buffer_getc();
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (ch == '\n')
    ch = PROCSPEECH;
    outb_p(ch, synth_port);
    SWAIT;
    if (time_after_eq(jiffies, jiff_max) && (ch == SPACE)) {
    timeout = 1000;
    while (synth_writable())
    if (--timeout <= 0)
    break;
    if (timeout <= 0) {
    oops();
    break;
    }
    outb_p(PROCSPEECH, synth_port);
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    jiffy_delta_val = jiffy_delta.u.n.value;
    delay_time_val = delay_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    schedule_timeout(msecs_to_jiffies(delay_time_val));
    jiff_max = jiffies + jiffy_delta_val;
    }
    }
    timeout = 1000;
    while (synth_writable())
    if (--timeout <= 0)
    break;
    if (timeout <= 0)
    oops();
    else
    outb_p(PROCSPEECH, synth_port);
    }
#[no_mangle]
unsafe extern "C" fn synth_flush(synth: *mut spk_synth) {
    static void synth_flush(struct spk_synth *synth)
    {
    outb_p(SYNTH_CLEAR, synth_port);
    }
#[no_mangle]
unsafe extern "C" fn synth_probe(synth: *mut spk_synth) -> c_int {
    static int synth_probe(struct spk_synth *synth)
    {
    let mut port_val: c_uint = 0;
    int i;
    pr_info("Probing for %s.\n", synth.long_name);
    if (port_forced) {
    synth_port = port_forced;
    pr_info("probe forced to %x by kernel command line\n",
    synth_port);
    if (synth_request_region(synth_port - 1, SYNTH_IO_EXTENT)) {
    pr_warn("sorry, port already reserved\n");
    return -EBUSY;
    }
    port_val = inb(synth_port);
    } else {
    for (i = 0; synth_portlist[i]; i++) {
    if (synth_request_region(synth_portlist[i],
    SYNTH_IO_EXTENT)) {
    pr_warn
    ("request_region: failed with 0x%x, %d\n",
    synth_portlist[i], SYNTH_IO_EXTENT);
    continue;
    }
    port_val = inb(synth_portlist[i]);
    if (port_val == 0x80) {
    synth_port = synth_portlist[i];
    break;
    }
    }
    }
    if (port_val != 0x80) {
    pr_info("%s: not found\n", synth.long_name);
    synth_release_region(synth_port, SYNTH_IO_EXTENT);
    synth_port = 0;
    return -ENODEV;
    }
    pr_info("%s: %03x-%03x, driver version %s,\n", synth.long_name,
    synth_port, synth_port + SYNTH_IO_EXTENT - 1,
    synth.version);
    synth.alive = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keynote_release(synth: *mut spk_synth) {
    static void keynote_release(struct spk_synth *synth)
    {
    spk_stop_serial_interrupt();
    if (synth_port)
    synth_release_region(synth_port, SYNTH_IO_EXTENT);
    synth_port = 0;
    }
    module_param_hw_named(port, port_forced, int, ioport, 0444);
    module_param_named(start, synth_keypc.startup, short, 0444);
    module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
    module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
    module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);
    MODULE_PARM_DESC(port, "Set the port for the synthesizer (override probing).");
    MODULE_PARM_DESC(start, "Start the synthesizer once it is loaded.");
    MODULE_PARM_DESC(rate, "Set the rate variable on load.");
    MODULE_PARM_DESC(pitch, "Set the pitch variable on load.");
    MODULE_PARM_DESC(direct, "Set the direct variable on load.");
    module_spk_synth(synth_keypc);
    MODULE_AUTHOR("David Borowski");
    MODULE_DESCRIPTION("Speakup support for Keynote Gold PC synthesizers");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_VERSION);
