//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/synth.c
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

    static LIST_HEAD(synths);
    struct spk_synth *synth;
    char spk_pitch_buff[32] = "";
    static int module_status;
    bool spk_quiet_boot;
    struct speakup_info_t speakup_info = {
//
// This spinlock is used to protect the entire speakup machinery, and
// must be taken at each kernel->speakup transition and released at
// each corresponding speakup->kernel transition.
//
// The progression thread only interferes with the speakup machinery
// through the synth buffer, so only needs to take the lock
// while tinkering with the buffer.
//
// We use spin_lock/trylock_irqsave and spin_unlock_irqrestore with this
// spinlock because speakup needs to disable the keyboard IRQ.
//
    .spinlock = __SPIN_LOCK_UNLOCKED(speakup_info.spinlock),
    .flushing = 0,
    };
    EXPORT_SYMBOL_GPL(speakup_info);
    static int do_synth_init(struct spk_synth *in_synth);
//
// Main loop of the progression thread: keep eating from the buffer
// and push to the serial port, waiting as needed
//
// For devices that have a "full" notification mechanism, the driver can
// adapt the loop the way they prefer.
//
#[no_mangle]
unsafe extern "C" fn _spk_do_catch_up(synth: *mut spk_synth, unicode: c_int) {
    static void _spk_do_catch_up(struct spk_synth *synth, int unicode)
    {
    u16 ch;
    unsigned long flags;
    unsigned long jiff_max;
    struct var_t *delay_time;
    struct var_t *full_time;
    struct var_t *jiffy_delta;
    int jiffy_delta_val;
    int delay_time_val;
    int full_time_val;
    int ret;
    jiffy_delta = spk_get_var(JIFFY);
    full_time = spk_get_var(FULL);
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
    if (!unicode)
    synth_buffer_skip_nonlatin1();
    if (synth_buffer_empty()) {
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    break;
    }
    ch = synth_buffer_peek();
    set_current_state(TASK_INTERRUPTIBLE);
    full_time_val = full_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (ch == '\n')
    ch = synth.procspeech;
    if (unicode)
    ret = synth.io_ops.synth_out_unicode(synth, ch);
    else
    ret = synth.io_ops.synth_out(synth, ch);
    if (!ret) {
    schedule_timeout(msecs_to_jiffies(full_time_val));
    continue;
    }
    if (time_after_eq(jiffies, jiff_max) && (ch == SPACE)) {
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    jiffy_delta_val = jiffy_delta.u.n.value;
    delay_time_val = delay_time.u.n.value;
    full_time_val = full_time.u.n.value;
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (synth.io_ops.synth_out(synth, synth.procspeech))
    schedule_timeout(
    msecs_to_jiffies(delay_time_val));
    else
    schedule_timeout(
    msecs_to_jiffies(full_time_val));
    jiff_max = jiffies + jiffy_delta_val;
    }
    set_current_state(TASK_RUNNING);
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    synth_buffer_getc();
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    }
    synth.io_ops.synth_out(synth, synth.procspeech);
    }
#[no_mangle]
pub unsafe extern "C" fn spk_do_catch_up(synth: *mut spk_synth) {
    void spk_do_catch_up(struct spk_synth *synth)
    {
    _spk_do_catch_up(synth, 0);
    }
    EXPORT_SYMBOL_GPL(spk_do_catch_up);
#[no_mangle]
pub unsafe extern "C" fn spk_do_catch_up_unicode(synth: *mut spk_synth) {
    void spk_do_catch_up_unicode(struct spk_synth *synth)
    {
    _spk_do_catch_up(synth, 1);
    }
    EXPORT_SYMBOL_GPL(spk_do_catch_up_unicode);
#[no_mangle]
pub unsafe extern "C" fn spk_synth_flush(synth: *mut spk_synth) {
    void spk_synth_flush(struct spk_synth *synth)
    {
    synth.io_ops.flush_buffer(synth);
    synth.io_ops.synth_out(synth, synth.clear);
    }
    EXPORT_SYMBOL_GPL(spk_synth_flush);
#[no_mangle]
pub unsafe extern "C" fn spk_synth_get_index(synth: *mut spk_synth) -> c_uchar {
    unsigned char spk_synth_get_index(struct spk_synth *synth)
    {
    return synth.io_ops.synth_in_nowait(synth);
    }
    EXPORT_SYMBOL_GPL(spk_synth_get_index);
#[no_mangle]
pub unsafe extern "C" fn spk_synth_is_alive_nop(synth: *mut spk_synth) -> c_int {
    int spk_synth_is_alive_nop(struct spk_synth *synth)
    {
    synth.alive = 1;
    return 1;
    }
    EXPORT_SYMBOL_GPL(spk_synth_is_alive_nop);
#[no_mangle]
pub unsafe extern "C" fn spk_synth_is_alive_restart(synth: *mut spk_synth) -> c_int {
    int spk_synth_is_alive_restart(struct spk_synth *synth)
    {
    if (synth.alive)
    return 1;
    if (synth.io_ops.wait_for_xmitr(synth) > 0) {
// restart
    synth.alive = 1;
    synth_printf("%s", synth.init);
    return 2; /* re-enabled */
    }
    pr_warn("%s: can't restart synth\n", synth.long_name);
    return 0;
    }
    EXPORT_SYMBOL_GPL(spk_synth_is_alive_restart);
#[no_mangle]
unsafe extern "C" fn thread_wake_up(unused: *mut timer_list) {
    static void thread_wake_up(struct timer_list *unused)
    {
    wake_up_interruptible_all(&speakup_event);
    }
    static DEFINE_TIMER(thread_timer, thread_wake_up);
#[no_mangle]
pub unsafe extern "C" fn synth_start() {
    void synth_start(void)
    {
    struct var_t *trigger_time;
    if (!synth.alive) {
    synth_buffer_clear();
    return;
    }
    trigger_time = spk_get_var(TRIGGER);
    if (!timer_pending(&thread_timer))
    mod_timer(&thread_timer, jiffies +
    msecs_to_jiffies(trigger_time.u.n.value));
    }
#[no_mangle]
pub unsafe extern "C" fn spk_do_flush() {
    void spk_do_flush(void)
    {
    if (!synth)
    return;
    speakup_info.flushing = 1;
    synth_buffer_clear();
    if (synth.alive) {
    if (spk_pitch_shift) {
    synth_printf("%s", spk_pitch_buff);
    spk_pitch_shift = 0;
    }
    }
    wake_up_interruptible_all(&speakup_event);
    wake_up_process(speakup_task);
    }
#[no_mangle]
pub unsafe extern "C" fn synth_write(_buf: *const c_char, count: usize) {
    void synth_write(const char *_buf, size_t count)
    {
    const unsigned char *buf = (const unsigned char *) _buf;
    while (count--)
    synth_buffer_add(*buf++);
    synth_start();
    }
// Consume one utf-8 character from buf (that contains up to count bytes),
// returns the unicode codepoint if valid, -1 otherwise.
// In all cases, returns the number of consumed bytes in *consumed,
// and the minimum number of bytes that would be needed for the next character
// in *want.
//
#[no_mangle]
pub unsafe extern "C" fn synth_utf8_get(buf: *const c_char, count: usize, consumed: *mut usize, want: *mut usize) -> i32 {
    s32 synth_utf8_get(const char *buf, size_t count, size_t *consumed, size_t *want)
    {
    let mut c: c_uchar = buf[0];
    let mut nbytes: c_int = 8 - fls(c ^ 0xff);
    u32 value;
    size_t i;
    switch (nbytes) {
    case 8: /* 0xff */
    case 7: /* 0xfe */
    case 1: /* 0x80 */
// Invalid, drop
// consumed = 1;
// want = 1;
    return -1;
    case 0:
// ASCII, take as such
// consumed = 1;
// want = 1;
    return c;
    default:
// 2..6-byte UTF-8
    if (count < nbytes) {
// We don't have it all
// consumed = 0;
// want = nbytes;
    return -1;
    }
// First byte
    value = c & ((1u << (7 - nbytes)) - 1);
// Other bytes
    for (i = 1; i < nbytes; i++) {
    c = buf[i];
    if ((c & 0xc0) != 0x80)	{
// Invalid, drop the head
// consumed = i;
// want = 1;
    return -1;
    }
    value = (value << 6) | (c & 0x3f);
    }
// consumed = nbytes;
// want = 1;
    return value;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn synth_writeu(buf: *const c_char, count: usize) {
    void synth_writeu(const char *buf, size_t count)
    {
    size_t i, consumed, want;
// Convert to u16
    for (i = 0; i < count; i++) {
    s32 value;
    value = synth_utf8_get(buf + i, count - i, &consumed, &want);
    if (value == -1) {
// Invalid or incomplete
    if (want > count - i)
// We don't have it all, stop
    count = i;
    continue;
    }
    if (value < 0x10000)
    synth_buffer_add(value);
    }
    synth_start();
    }
#[no_mangle]
pub unsafe extern "C" fn synth_printf(fmt: *const c_char, ...) {
    void synth_printf(const char *fmt, ...)
    {
    va_list args;
    unsigned char buf[160];
    int r;
    va_start(args, fmt);
    r = vsnprintf(buf, sizeof(buf), fmt, args);
    va_end(args);
    if (r > sizeof(buf) - 1)
    r = sizeof(buf) - 1;
    synth_writeu(buf, r);
    }
    EXPORT_SYMBOL_GPL(synth_printf);
#[no_mangle]
pub unsafe extern "C" fn synth_putwc(wc: u16) {
    void synth_putwc(u16 wc)
    {
    synth_buffer_add(wc);
    }
    EXPORT_SYMBOL_GPL(synth_putwc);
#[no_mangle]
pub unsafe extern "C" fn synth_putwc_s(wc: u16) {
    void synth_putwc_s(u16 wc)
    {
    synth_buffer_add(wc);
    synth_start();
    }
    EXPORT_SYMBOL_GPL(synth_putwc_s);
#[no_mangle]
pub unsafe extern "C" fn synth_putws(buf: *const u16) {
    void synth_putws(const u16 *buf)
    {
    const u16 *p;
    for (p = buf; *p; p++)
    synth_buffer_add(*p);
    }
    EXPORT_SYMBOL_GPL(synth_putws);
#[no_mangle]
pub unsafe extern "C" fn synth_putws_s(buf: *const u16) {
    void synth_putws_s(const u16 *buf)
    {
    synth_putws(buf);
    synth_start();
    }
    EXPORT_SYMBOL_GPL(synth_putws_s);
    static int index_count;
    static int sentence_count;
#[no_mangle]
pub unsafe extern "C" fn spk_reset_index_count(sc: c_int) {
    void spk_reset_index_count(int sc)
    {
    let mut first: static int = 1;
    if (first)
    first = 0;
    else
    synth.get_index(synth);
    index_count = 0;
    sentence_count = sc;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_supports_indexing() -> c_int {
    int synth_supports_indexing(void)
    {
    if (synth.get_index)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_insert_next_index(sent_num: c_int) {
    void synth_insert_next_index(int sent_num)
    {
    int out;
    if (synth.alive) {
    if (sent_num == 0) {
    synth.indexing.currindex++;
    index_count++;
    if (synth.indexing.currindex >
    synth.indexing.highindex)
    synth.indexing.currindex =
    synth.indexing.lowindex;
    }
    out = synth.indexing.currindex * 10 + sent_num;
    synth_printf(synth.indexing.command, out, out);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn spk_get_index_count(linecount: *mut c_int, sentcount: *mut c_int) {
    void spk_get_index_count(int *linecount, int *sentcount)
    {
    let mut ind: c_int = synth.get_index(synth);
    if (ind) {
    sentence_count = ind % 10;
    if ((ind / 10) <= synth.indexing.currindex)
    index_count = synth.indexing.currindex - (ind / 10);
    else
    index_count = synth.indexing.currindex
    - synth.indexing.lowindex
    + synth.indexing.highindex - (ind / 10) + 1;
    }
// sentcount = sentence_count;
// linecount = index_count;
    }
    static struct resource synth_res;
#[no_mangle]
pub unsafe extern "C" fn synth_request_region(start: c_ulong, n: c_ulong) -> c_int {
    int synth_request_region(unsigned long start, unsigned long n)
    {
    struct resource *parent = &ioport_resource;
    memset(&synth_res, 0, sizeof(synth_res));
    synth_res.name = synth.name;
    synth_res.start = start;
    synth_res.end = start + n - 1;
    synth_res.flags = IORESOURCE_BUSY;
    return request_resource(parent, &synth_res);
    }
    EXPORT_SYMBOL_GPL(synth_request_region);
#[no_mangle]
pub unsafe extern "C" fn synth_release_region(start: c_ulong, n: c_ulong) -> c_int {
    int synth_release_region(unsigned long start, unsigned long n)
    {
    return release_resource(&synth_res);
    }
    EXPORT_SYMBOL_GPL(synth_release_region);
    struct var_t synth_time_vars[] = {
    { DELAY, .u.n = {core::ptr::null_mut(), 100, 100, 2000, 0, 0, core::ptr::null_mut() } },
    { TRIGGER, .u.n = {core::ptr::null_mut(), 20, 10, 2000, 0, 0, core::ptr::null_mut() } },
    { JIFFY, .u.n = {core::ptr::null_mut(), 50, 20, 200, 0, 0, core::ptr::null_mut() } },
    { FULL, .u.n = {core::ptr::null_mut(), 400, 200, 60000, 0, 0, core::ptr::null_mut() } },
    { FLUSH, .u.n = {core::ptr::null_mut(), 4000, 10, 4000, 0, 0, core::ptr::null_mut() } },
    V_LAST_VAR
    };
// called by: speakup_init()
#[no_mangle]
pub unsafe extern "C" fn synth_init(synth_name: *mut c_char) -> c_int {
    int synth_init(char *synth_name)
    {
    let mut ret: c_int = 0;
    struct spk_synth *tmp, *synth = core::ptr::null_mut();
    if (!synth_name)
    return 0;
    if (strcmp(synth_name, "none") == 0) {
    mutex_lock(&spk_mutex);
    synth_release();
    mutex_unlock(&spk_mutex);
    return 0;
    }
    mutex_lock(&spk_mutex);
// First, check if we already have it loaded.
    list_for_each_entry(tmp, &synths, node) {
    if (strcmp(tmp.name, synth_name) == 0)
    synth = tmp;
    }
// If we got one, initialize it now.
    if (synth)
    ret = do_synth_init(synth);
    else
    ret = -ENODEV;
    mutex_unlock(&spk_mutex);
    return ret;
    }
// called by: synth_add()
#[no_mangle]
unsafe extern "C" fn do_synth_init(in_synth: *mut spk_synth) -> c_int {
    static int do_synth_init(struct spk_synth *in_synth)
    {
    struct var_t *var;
    synth_release();
    if (in_synth.checkval != SYNTH_CHECK)
    return -EINVAL;
    synth = in_synth;
    synth.alive = 0;
    pr_warn("synth probe\n");
    if (synth.probe(synth) < 0) {
    pr_warn("%s: device probe failed\n", in_synth.name);
    synth = core::ptr::null_mut();
    return -ENODEV;
    }
    synth_time_vars[0].u.n.value =
    synth_time_vars[0].u.n.default_val = synth.delay;
    synth_time_vars[1].u.n.value =
    synth_time_vars[1].u.n.default_val = synth.trigger;
    synth_time_vars[2].u.n.value =
    synth_time_vars[2].u.n.default_val = synth.jiffies;
    synth_time_vars[3].u.n.value =
    synth_time_vars[3].u.n.default_val = synth.full;
    synth_time_vars[4].u.n.value =
    synth_time_vars[4].u.n.default_val = synth.flush_time;
    synth_printf("%s", synth.init);
    for (var = synth.vars;
    (var.var_id >= 0) && (var.var_id < MAXVARS); var++)
    speakup_register_var(var);
    if (!spk_quiet_boot)
    synth_printf("%s found\n", synth.long_name);
    if (synth.attributes.name &&
    sysfs_create_group(speakup_kobj, &synth.attributes) < 0)
    return -ENOMEM;
    synth_flags = synth.flags;
    wake_up_interruptible_all(&speakup_event);
    if (speakup_task)
    wake_up_process(speakup_task);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_release() {
    void synth_release(void)
    {
    struct var_t *var;
    unsigned long flags;
    if (!synth)
    return;
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    pr_info("releasing synth %s\n", synth.name);
    synth.alive = 0;
    timer_delete(&thread_timer);
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (synth.attributes.name)
    sysfs_remove_group(speakup_kobj, &synth.attributes);
    for (var = synth.vars; var.var_id != MAXVARS; var++)
    speakup_unregister_var(var.var_id);
    synth.release(synth);
    synth = core::ptr::null_mut();
    }
// called by: all_driver_init()
#[no_mangle]
pub unsafe extern "C" fn synth_add(in_synth: *mut spk_synth) -> c_int {
    int synth_add(struct spk_synth *in_synth)
    {
    let mut status: c_int = 0;
    struct spk_synth *tmp;
    mutex_lock(&spk_mutex);
    list_for_each_entry(tmp, &synths, node) {
    if (tmp == in_synth) {
    mutex_unlock(&spk_mutex);
    return 0;
    }
    }
    if (in_synth.startup)
    status = do_synth_init(in_synth);
    if (!status)
    list_add_tail(&in_synth.node, &synths);
    mutex_unlock(&spk_mutex);
    return status;
    }
    EXPORT_SYMBOL_GPL(synth_add);
#[no_mangle]
pub unsafe extern "C" fn synth_remove(in_synth: *mut spk_synth) {
    void synth_remove(struct spk_synth *in_synth)
    {
    mutex_lock(&spk_mutex);
    if (synth == in_synth)
    synth_release();
    list_del(&in_synth.node);
    module_status = 0;
    mutex_unlock(&spk_mutex);
    }
    EXPORT_SYMBOL_GPL(synth_remove);
    struct spk_synth *synth_current(void)
    {
    return synth;
    }
    EXPORT_SYMBOL_GPL(synth_current);
    u16 spk_punc_masks[] = { 0, SOME, MOST, PUNC, PUNC | B_SYM };
