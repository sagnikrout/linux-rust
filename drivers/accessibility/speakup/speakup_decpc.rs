//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/speakup_decpc.c
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
// This is the DECtalk PC speakup driver
//
// Some constants from DEC's DOS driver:
// Copyright (c) by Digital Equipment Corp.
//
// 386BSD DECtalk PC driver:
// Copyright (c) 1996 Brian Buhrow <buhrow@lothlorien.nfbcal.org>
//
// Linux DECtalk PC driver:
// Copyright (c) 1997 Nicolas Pitre <nico@cam.org>
//
// speakup DECtalk PC Internal driver:
// Copyright (c) 2003 David Borowski <david575@golden.net>
//
// All rights reserved.
//

pub const MODULE_init: c_uint = 0x0dec	/* module in boot code */;
pub const MODULE_self_test: c_uint = 0x8800	/* module in self-test */;
pub const MODULE_reset: c_uint = 0xffff	/* reinit the whole module */;
pub const MODE_mask: c_uint = 0xf000	/* mode bits in high nibble */;
pub const MODE_null: c_uint = 0x0000;
pub const MODE_test: c_uint = 0x2000	/* in testing mode */;
pub const MODE_status: c_uint = 0x8000;
pub const STAT_int: c_uint = 0x0001	/* running in interrupt mode */;
pub const STAT_tr_char: c_uint = 0x0002	/* character data to transmit */;
pub const STAT_rr_char: c_uint = 0x0004	/* ready to receive char data */;
pub const STAT_cmd_ready: c_uint = 0x0008	/* ready to accept commands */;
pub const STAT_dma_ready: c_uint = 0x0010	/* dma command ready */;
pub const STAT_digitized: c_uint = 0x0020	/* spc in digitized mode */;
pub const STAT_new_index: c_uint = 0x0040	/* new last index ready */;
pub const STAT_new_status: c_uint = 0x0080	/* new status posted */;
pub const STAT_dma_state: c_uint = 0x0100	/* dma state toggle */;
pub const STAT_index_valid: c_uint = 0x0200	/* indexes are valid */;
pub const STAT_flushing: c_uint = 0x0400	/* flush in progress */;
pub const STAT_self_test: c_uint = 0x0800	/* module in self test */;
pub const MODE_ready: c_uint = 0xc000	/* module ready for next phase */;
pub const READY_boot: c_uint = 0x0000;
pub const READY_kernel: c_uint = 0x0001;
pub const MODE_error: c_uint = 0xf000;
pub const CMD_mask: c_uint = 0xf000	/* mask for command nibble */;
pub const CMD_null: c_uint = 0x0000	/* post status */;
pub const CMD_control: c_uint = 0x1000	/* hard control command */;
pub const CTRL_mask: c_uint = 0x0F00	/* mask off control nibble */;
pub const CTRL_data: c_uint = 0x00FF	/* mask to get data byte */;
pub const CTRL_null: c_uint = 0x0000	/* null control */;
pub const CTRL_vol_up: c_uint = 0x0100	/* increase volume */;
pub const CTRL_vol_down: c_uint = 0x0200	/* decrease volume */;
pub const CTRL_vol_set: c_uint = 0x0300	/* set volume */;
pub const CTRL_pause: c_uint = 0x0400	/* pause spc */;
pub const CTRL_resume: c_uint = 0x0500	/* resume spc clock */;
pub const CTRL_resume_spc: c_uint = 0x0001	/* resume spc soft pause */;
pub const CTRL_flush: c_uint = 0x0600	/* flush all buffers */;
pub const CTRL_int_enable: c_uint = 0x0700	/* enable status change ints */;
pub const CTRL_buff_free: c_uint = 0x0800	/* buffer remain count */;
pub const CTRL_buff_used: c_uint = 0x0900	/* buffer in use */;
pub const CTRL_speech: c_uint = 0x0a00	/* immediate speech change */;
pub const CTRL_SP_voice: c_uint = 0x0001	/* voice change */;
pub const CTRL_SP_rate: c_uint = 0x0002	/* rate change */;
pub const CTRL_SP_comma: c_uint = 0x0003	/* comma pause change */;
pub const CTRL_SP_period: c_uint = 0x0004	/* period pause change */;
pub const CTRL_SP_rate_delta: c_uint = 0x0005	/* delta rate change */;
pub const CTRL_SP_get_param: c_uint = 0x0006	/* return the desired parameter */;
pub const CTRL_last_index: c_uint = 0x0b00	/* get last index spoken */;
pub const CTRL_io_priority: c_uint = 0x0c00	/* change i/o priority */;
pub const CTRL_free_mem: c_uint = 0x0d00	/* get free paragraphs on module */;
pub const CTRL_get_lang: c_uint = 0x0e00	/* return bitmask of loaded languages */;
pub const CMD_test: c_uint = 0x2000	/* self-test request */;
pub const TEST_mask: c_uint = 0x0F00	/* isolate test field */;
pub const TEST_null: c_uint = 0x0000	/* no test requested */;
pub const TEST_isa_int: c_uint = 0x0100	/* assert isa irq */;
pub const TEST_echo: c_uint = 0x0200	/* make data in == data out */;
pub const TEST_seg: c_uint = 0x0300	/* set peek/poke segment */;
pub const TEST_off: c_uint = 0x0400	/* set peek/poke offset */;
pub const TEST_peek: c_uint = 0x0500	/* data out == *peek */;
pub const TEST_poke: c_uint = 0x0600	/* *peek == data in */;
pub const TEST_sub_code: c_uint = 0x00FF	/* user defined test sub codes */;
pub const CMD_id: c_uint = 0x3000	/* return software id */;
pub const ID_null: c_uint = 0x0000	/* null id */;
pub const ID_kernel: c_uint = 0x0100	/* kernel code executing */;
pub const ID_boot: c_uint = 0x0200	/* boot code executing */;
pub const CMD_dma: c_uint = 0x4000	/* force a dma start */;
pub const CMD_reset: c_uint = 0x5000	/* reset module status */;
pub const CMD_sync: c_uint = 0x6000	/* kernel sync command */;
pub const CMD_char_in: c_uint = 0x7000	/* single character send */;
pub const CMD_char_out: c_uint = 0x8000	/* single character get */;
pub const CHAR_count_1: c_uint = 0x0100	/* one char in cmd_low */;
pub const CHAR_count_2: c_uint = 0x0200	/* the second in data_low */;
pub const CHAR_count_3: c_uint = 0x0300	/* the third in data_high */;
pub const CMD_spc_mode: c_uint = 0x9000	/* change spc mode */;
pub const CMD_spc_to_text: c_uint = 0x0100	/* set to text mode */;
pub const CMD_spc_to_digit: c_uint = 0x0200	/* set to digital mode */;
pub const CMD_spc_rate: c_uint = 0x0400	/* change spc data rate */;
pub const CMD_error: c_uint = 0xf000	/* severe error */;
    enum {	PRIMARY_DIC	= 0, USER_DIC, COMMAND_DIC, ABBREV_DIC };
pub const DMA_single_in: c_uint = 0x01;
pub const DMA_single_out: c_uint = 0x02;
pub const DMA_buff_in: c_uint = 0x03;
pub const DMA_buff_out: c_uint = 0x04;
pub const DMA_control: c_uint = 0x05;
pub const DT_MEM_ALLOC: c_uint = 0x03;
pub const DT_SET_DIC: c_uint = 0x04;
pub const DT_START_TASK: c_uint = 0x05;
pub const DT_LOAD_MEM: c_uint = 0x06;
pub const DT_READ_MEM: c_uint = 0x07;
pub const DT_DIGITAL_IN: c_uint = 0x08;
pub const DMA_sync: c_uint = 0x06;
pub const DMA_sync_char: c_uint = 0x07;

pub const PROCSPEECH: c_uint = 0x0b;
pub const SYNTH_IO_EXTENT: c_int = 8;
    static int synth_probe(struct spk_synth *synth);
    static void dtpc_release(struct spk_synth *synth);
    static const char *synth_immediate(struct spk_synth *synth, const char *buf);
    static void do_catch_up(struct spk_synth *synth);
    static void synth_flush(struct spk_synth *synth);
    static int synth_portlist[] = { 0x340, 0x350, 0x240, 0x250, 0 };
    static int in_escape, is_flushing;
    static int dt_stat, dma_state;
    enum default_vars_id {
    CAPS_START_ID = 0, CAPS_STOP_ID,
    RATE_ID, PITCH_ID, INFLECTION_ID,
    VOL_ID, PUNCT_ID, VOICE_ID,
    DIRECT_ID, V_LAST_VAR_ID,
    NB_ID,
    };
    static struct var_t vars[NB_ID] = {
    [CAPS_START_ID] = { CAPS_START, .u.s = {"[:dv ap 200]" } },
    [CAPS_STOP_ID] = { CAPS_STOP, .u.s = {"[:dv ap 100]" } },
    [RATE_ID] = { RATE, .u.n = {"[:ra %d]", 9, 0, 18, 150, 25, core::ptr::null_mut() } },
    [PITCH_ID] = { PITCH, .u.n = {"[:dv ap %d]", 80, 0, 100, 20, 0, core::ptr::null_mut() } },
    [INFLECTION_ID] = { INFLECTION, .u.n = {"[:dv pr %d] ", 100, 0, 10000, 0, 0, core::ptr::null_mut() } },
    [VOL_ID] = { VOL, .u.n = {"[:vo se %d]", 5, 0, 9, 5, 10, core::ptr::null_mut() } },
    [PUNCT_ID] = { PUNCT, .u.n = {"[:pu %c]", 0, 0, 2, 0, 0, "nsa" } },
    [VOICE_ID] = { VOICE, .u.n = {"[:n%c]", 0, 0, 9, 0, 0, "phfdburwkv" } },
    [DIRECT_ID] = { DIRECT, .u.n = {core::ptr::null_mut(), 0, 0, 1, 0, 0, core::ptr::null_mut() } },
    V_LAST_VAR
    };
//
// These attributes will appear in /sys/accessibility/speakup/decpc.
//
    static struct kobj_attribute caps_start_attribute =
    __ATTR(caps_start, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute caps_stop_attribute =
    __ATTR(caps_stop, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute pitch_attribute =
    __ATTR(pitch, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute inflection_attribute =
    __ATTR(inflection, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute punct_attribute =
    __ATTR(punct, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute rate_attribute =
    __ATTR(rate, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute voice_attribute =
    __ATTR(voice, 0644, spk_var_show, spk_var_store);
    static struct kobj_attribute vol_attribute =
    __ATTR(vol, 0644, spk_var_show, spk_var_store);
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
    &inflection_attribute.attr,
    &punct_attribute.attr,
    &rate_attribute.attr,
    &voice_attribute.attr,
    &vol_attribute.attr,
    &delay_time_attribute.attr,
    &direct_attribute.attr,
    &full_time_attribute.attr,
    &jiffy_delta_attribute.attr,
    &trigger_time_attribute.attr,
    core::ptr::null_mut(),	/* need to core::ptr::null_mut() terminate the list of attributes */
    };
    static struct spk_synth synth_dec_pc = {
    .name = "decpc",
    .version = DRV_VERSION,
    .long_name = "Dectalk PC",
    .init = "[:pe -380]",
    .procspeech = PROCSPEECH,
    .delay = 500,
    .trigger = 50,
    .jiffies = 50,
    .full = 1000,
    .flags = SF_DEC,
    .startup = SYNTH_START,
    .checkval = SYNTH_CHECK,
    .vars = vars,
    .io_ops = &spk_serial_io_ops,
    .probe = synth_probe,
    .release = dtpc_release,
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
    .name = "decpc",
    },
    };
#[no_mangle]
unsafe extern "C" fn dt_getstatus() -> c_int {
    static int dt_getstatus(void)
    {
    dt_stat = inb_p(speakup_info.port_tts) |
    (inb_p(speakup_info.port_tts + 1) << 8);
    return dt_stat;
    }
#[no_mangle]
unsafe extern "C" fn dt_sendcmd(cmd: u_int) {
    static void dt_sendcmd(u_int cmd)
    {
    outb_p(cmd & 0xFF, speakup_info.port_tts);
    outb_p((cmd >> 8) & 0xFF, speakup_info.port_tts + 1);
    }
#[no_mangle]
unsafe extern "C" fn dt_waitbit(bit: c_int) -> c_int {
    static int dt_waitbit(int bit)
    {
    let mut timeout: c_int = 100;
    while (--timeout > 0) {
    if ((dt_getstatus() & bit) == bit)
    return 1;
    udelay(50);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dt_wait_dma() -> c_int {
    static int dt_wait_dma(void)
    {
    let mut timeout: c_int = 100, state = dma_state;
    if (!dt_waitbit(STAT_dma_ready))
    return 0;
    while (--timeout > 0) {
    if ((dt_getstatus() & STAT_dma_state) == state)
    return 1;
    udelay(50);
    }
    dma_state = dt_getstatus() & STAT_dma_state;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn dt_ctrl(cmd: u_int) -> c_int {
    static int dt_ctrl(u_int cmd)
    {
    let mut timeout: c_int = 10;
    if (!dt_waitbit(STAT_cmd_ready))
    return -1;
    outb_p(0, speakup_info.port_tts + 2);
    outb_p(0, speakup_info.port_tts + 3);
    dt_getstatus();
    dt_sendcmd(CMD_control | cmd);
    outb_p(0, speakup_info.port_tts + 6);
    while (dt_getstatus() & STAT_cmd_ready) {
    udelay(20);
    if (--timeout == 0)
    break;
    }
    dt_sendcmd(CMD_null);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synth_flush(synth: *mut spk_synth) {
    static void synth_flush(struct spk_synth *synth)
    {
    let mut timeout: c_int = 10;
    if (is_flushing)
    return;
    is_flushing = 4;
    in_escape = 0;
    while (dt_ctrl(CTRL_flush)) {
    if (--timeout == 0)
    break;
    udelay(50);
    }
    for (timeout = 0; timeout < 10; timeout++) {
    if (dt_waitbit(STAT_dma_ready))
    break;
    udelay(50);
    }
    outb_p(DMA_sync, speakup_info.port_tts + 4);
    outb_p(0, speakup_info.port_tts + 4);
    udelay(100);
    for (timeout = 0; timeout < 10; timeout++) {
    if (!(dt_getstatus() & STAT_flushing))
    break;
    udelay(50);
    }
    dma_state = dt_getstatus() & STAT_dma_state;
    dma_state ^= STAT_dma_state;
    is_flushing = 0;
    }
#[no_mangle]
unsafe extern "C" fn dt_sendchar(ch: c_char) -> c_int {
    static int dt_sendchar(char ch)
    {
    if (!dt_wait_dma())
    return -1;
    if (!(dt_stat & STAT_rr_char))
    return -2;
    outb_p(DMA_single_in, speakup_info.port_tts + 4);
    outb_p(ch, speakup_info.port_tts + 4);
    dma_state ^= STAT_dma_state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn testkernel() -> c_int {
    static int testkernel(void)
    {
    let mut status: c_int = 0;
    if (dt_getstatus() == 0xffff) {
    status = -1;
    goto oops;
    }
    dt_sendcmd(CMD_sync);
    if (!dt_waitbit(STAT_cmd_ready))
    status = -2;
#[no_mangle]
pub unsafe extern "C" fn if(0x8000: dt_stat &) -> else {
    else if (dt_stat & 0x8000)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(0x0dec: dt_stat ==) -> else {
    else if (dt_stat == 0x0dec)
    pr_warn("dec_pc at 0x%x, software not loaded\n",
    speakup_info.port_tts);
    status = -3;
    oops:	synth_release_region(speakup_info.port_tts, SYNTH_IO_EXTENT);
    speakup_info.port_tts = 0;
    return status;
    }
#[no_mangle]
unsafe extern "C" fn do_catch_up(synth: *mut spk_synth) {
    static void do_catch_up(struct spk_synth *synth)
    {
    u_char ch;
    static u_char last;
    unsigned long flags;
    unsigned long jiff_max;
    struct var_t *jiffy_delta;
    struct var_t *delay_time;
    int jiffy_delta_val;
    int delay_time_val;
    jiffy_delta = spk_get_var(JIFFY);
    delay_time = spk_get_var(DELAY);
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
    ch = synth_buffer_peek();
    set_current_state(TASK_INTERRUPTIBLE);
    delay_time_val = delay_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (ch == '\n')
    ch = 0x0D;
    if (dt_sendchar(ch)) {
    schedule_timeout(msecs_to_jiffies(delay_time_val));
    continue;
    }
    set_current_state(TASK_RUNNING);
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    synth_buffer_getc();
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (ch == '[') {
    in_escape = 1;
    } else if (ch == ']') {
    in_escape = 0;
    } else if (ch <= SPACE) {
    if (!in_escape && strchr(",.!?;:", last))
    dt_sendchar(PROCSPEECH);
    if (time_after_eq(jiffies, jiff_max)) {
    if (!in_escape)
    dt_sendchar(PROCSPEECH);
    spin_lock_irqsave(&speakup_info.spinlock,
    flags);
    jiffy_delta_val = jiffy_delta.u.n.value;
    delay_time_val = delay_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock,
    flags);
    schedule_timeout(msecs_to_jiffies
    (delay_time_val));
    jiff_max = jiffies + jiffy_delta_val;
    }
    }
    last = ch;
    ch = 0;
    }
    if (!in_escape)
    dt_sendchar(PROCSPEECH);
    }
    static const char *synth_immediate(struct spk_synth *synth, const char *buf)
    {
    u_char ch;
    while ((ch = *buf)) {
    if (ch == '\n')
    ch = PROCSPEECH;
    if (dt_sendchar(ch))
    return buf;
    buf++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn synth_probe(synth: *mut spk_synth) -> c_int {
    static int synth_probe(struct spk_synth *synth)
    {
    let mut i: c_int = 0, failed = 0;
    pr_info("Probing for %s.\n", synth.long_name);
    for (i = 0; synth_portlist[i]; i++) {
    if (synth_request_region(synth_portlist[i], SYNTH_IO_EXTENT)) {
    pr_warn("request_region: failed with 0x%x, %d\n",
    synth_portlist[i], SYNTH_IO_EXTENT);
    continue;
    }
    speakup_info.port_tts = synth_portlist[i];
    failed = testkernel();
    if (failed == 0)
    break;
    }
    if (failed) {
    pr_info("%s: not found\n", synth.long_name);
    return -ENODEV;
    }
    pr_info("%s: %03x-%03x, Driver Version %s,\n", synth.long_name,
    speakup_info.port_tts, speakup_info.port_tts + 7,
    synth.version);
    synth.alive = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dtpc_release(synth: *mut spk_synth) {
    static void dtpc_release(struct spk_synth *synth)
    {
    spk_stop_serial_interrupt();
    if (speakup_info.port_tts)
    synth_release_region(speakup_info.port_tts, SYNTH_IO_EXTENT);
    speakup_info.port_tts = 0;
    }
    module_param_named(start, synth_dec_pc.startup, short, 0444);
    module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
    module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
    module_param_named(inflection, vars[INFLECTION_ID].u.n.default_val, int, 0444);
    module_param_named(vol, vars[VOL_ID].u.n.default_val, int, 0444);
    module_param_named(punct, vars[PUNCT_ID].u.n.default_val, int, 0444);
    module_param_named(voice, vars[VOICE_ID].u.n.default_val, int, 0444);
    module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);
    MODULE_PARM_DESC(start, "Start the synthesizer once it is loaded.");
    MODULE_PARM_DESC(rate, "Set the rate variable on load.");
    MODULE_PARM_DESC(pitch, "Set the pitch variable on load.");
    MODULE_PARM_DESC(inflection, "Set the inflection variable on load.");
    MODULE_PARM_DESC(vol, "Set the vol variable on load.");
    MODULE_PARM_DESC(punct, "Set the punct variable on load.");
    MODULE_PARM_DESC(voice, "Set the voice variable on load.");
    MODULE_PARM_DESC(direct, "Set the direct variable on load.");
    module_spk_synth(synth_dec_pc);
    MODULE_AUTHOR("Kirk Reiser <kirk@braille.uwo.ca>");
    MODULE_AUTHOR("David Borowski");
    MODULE_DESCRIPTION("Speakup support for DECtalk PC synthesizers");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_VERSION);
