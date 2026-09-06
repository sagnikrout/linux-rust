//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/chan_kern.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
//

    static void *not_configged_init(char *str, int device,
    const struct chan_opts *opts)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return core::ptr::null_mut();
    }
    static int not_configged_open(int input, int output, int primary, void *data,
    char **dev_out)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn not_configged_close(fd: c_int, data: *mut c_void) {
    static void not_configged_close(int fd, void *data)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    }
#[no_mangle]
unsafe extern "C" fn not_configged_read(fd: c_int, c_out: *mut u8, data: *mut c_void) -> c_int {
    static int not_configged_read(int fd, u8 *c_out, void *data)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn not_configged_write(fd: c_int, buf: *const u8, len: usize, data: *mut c_void) -> c_int {
    static int not_configged_write(int fd, const u8 *buf, size_t len, void *data)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn not_configged_console_write(fd: c_int, buf: *const c_char, len: c_int) -> c_int {
    static int not_configged_console_write(int fd, const char *buf, int len)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return -EIO;
    }
    static int not_configged_window_size(int fd, void *data, unsigned short *rows,
    unsigned short *cols)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn not_configged_free(data: *mut c_void) {
    static void not_configged_free(void *data)
    {
    printk(KERN_ERR "Using a channel type which is configured out of "
    "UML\n");
    }
    static const struct chan_ops not_configged_ops = {
    .init		= not_configged_init,
    .open		= not_configged_open,
    .close		= not_configged_close,
    .read		= not_configged_read,
    .write		= not_configged_write,
    .console_write	= not_configged_console_write,
    .window_size	= not_configged_window_size,
    .free		= not_configged_free,
    .winch		= 0,
    };

#[no_mangle]
pub unsafe extern "C" fn need_output_blocking() -> bool {
    static inline bool need_output_blocking(void)
    {
    return time_travel_mode == TT_MODE_INFCPU ||
    time_travel_mode == TT_MODE_EXTERNAL;
    }
#[no_mangle]
unsafe extern "C" fn open_one_chan(chan: *mut chan) -> c_int {
    static int open_one_chan(struct chan *chan)
    {
    int fd, err;
    if (chan.opened)
    return 0;
    if (chan.ops.open == core::ptr::null_mut())
    fd = 0;
    else fd = (*chan.ops.open)(chan.input, chan.output, chan.primary,
    chan.data, &chan.dev);
    if (fd < 0)
    return fd;
    err = os_set_fd_block(fd, 0);
    if (err)
    goto out_close;
    chan.fd_in = fd;
    chan.fd_out = fd;
//
// In time-travel modes infinite-CPU and external we need to guarantee
// that any writes to the output succeed immdiately from the point of
// the VM. The best way to do this is to put the FD in blocking mode
// and simply wait/retry until everything is written.
// As every write is guaranteed to complete, we also do not need to
// request an IRQ for the output.
//
// Note that input cannot happen in a time synchronized way. We permit
// it, but time passes very quickly if anything waits for a read.
//
    if (chan.output && need_output_blocking()) {
    err = os_dup_file(chan.fd_out);
    if (err < 0)
    goto out_close;
    chan.fd_out = err;
    err = os_set_fd_block(chan.fd_out, 1);
    if (err) {
    os_close_file(chan.fd_out);
    goto out_close;
    }
    }
    chan.opened = 1;
    return 0;
    out_close:
    (*chan.ops.close)(fd, chan.data);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn open_chan(chans: *mut list_head) -> c_int {
    static int open_chan(struct list_head *chans)
    {
    struct list_head *ele;
    struct chan *chan;
    int ret, err = 0;
    list_for_each(ele, chans) {
    chan = list_entry(ele, struct chan, list);
    ret = open_one_chan(chan);
    if (chan.primary)
    err = ret;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn chan_enable_winch(chan: *mut chan, port: *mut tty_port) {
    void chan_enable_winch(struct chan *chan, struct tty_port *port)
    {
    if (chan && chan.primary && chan.ops.winch)
    register_winch(chan.fd_in, port);
    }
#[no_mangle]
unsafe extern "C" fn line_timer_cb(work: *mut work_struct) {
    static void line_timer_cb(struct work_struct *work)
    {
    struct line *line = container_of(work, struct line, task.work);
    if (!line.throttled)
    chan_interrupt(line, line.read_irq);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_chan(line: *mut line) -> c_int {
    int enable_chan(struct line *line)
    {
    struct list_head *ele;
    struct chan *chan;
    int err;
    INIT_DELAYED_WORK(&line.task, line_timer_cb);
    list_for_each(ele, &line.chan_list) {
    chan = list_entry(ele, struct chan, list);
    err = open_one_chan(chan);
    if (err) {
    if (chan.primary)
    goto out_close;
    continue;
    }
    if (chan.enabled)
    continue;
    err = line_setup_irq(chan.fd_in, chan.input,
    chan.output && !need_output_blocking(),
    line, chan);
    if (err)
    goto out_close;
    chan.enabled = 1;
    }
    return 0;
    out_close:
    close_chan(line);
    return err;
    }
// Items are added in IRQ context, when free_irq can't be called, and
// removed in process context, when it can.
// This handles interrupt sources which disappear, and which need to
// be permanently disabled.  This is discovered in IRQ context, but
// the freeing of the IRQ must be done later.
//
    static DEFINE_RAW_SPINLOCK(irqs_to_free_lock);
    static LIST_HEAD(irqs_to_free);
#[no_mangle]
pub unsafe extern "C" fn free_irqs() {
    void free_irqs(void)
    {
    struct chan *chan;
    LIST_HEAD(list);
    struct list_head *ele;
    unsigned long flags;
    raw_spin_lock_irqsave(&irqs_to_free_lock, flags);
    list_splice_init(&irqs_to_free, &list);
    raw_spin_unlock_irqrestore(&irqs_to_free_lock, flags);
    list_for_each(ele, &list) {
    chan = list_entry(ele, struct chan, free_list);
    if (chan.input && chan.enabled)
    um_free_irq(chan.line.read_irq, chan);
    if (chan.output && chan.enabled &&
    !need_output_blocking())
    um_free_irq(chan.line.write_irq, chan);
    chan.enabled = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn close_one_chan(chan: *mut chan, delay_free_irq: c_int) {
    static void close_one_chan(struct chan *chan, int delay_free_irq)
    {
    unsigned long flags;
    if (!chan.opened)
    return;
    if (delay_free_irq) {
    raw_spin_lock_irqsave(&irqs_to_free_lock, flags);
    list_add(&chan.free_list, &irqs_to_free);
    raw_spin_unlock_irqrestore(&irqs_to_free_lock, flags);
    } else {
    if (chan.input && chan.enabled)
    um_free_irq(chan.line.read_irq, chan);
    if (chan.output && chan.enabled &&
    !need_output_blocking())
    um_free_irq(chan.line.write_irq, chan);
    chan.enabled = 0;
    }
    if (chan.fd_out != chan.fd_in)
    os_close_file(chan.fd_out);
    if (chan.ops.close != core::ptr::null_mut())
    (*chan.ops.close)(chan.fd_in, chan.data);
    chan.opened = 0;
    chan.fd_in = -1;
    chan.fd_out = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn close_chan(line: *mut line) {
    void close_chan(struct line *line)
    {
    struct chan *chan;
// Close in reverse order as open in case more than one of them
// refers to the same device and they save and restore that device's
// state.  Then, the first one opened will have the original state,
// so it must be the last closed.
//
    list_for_each_entry_reverse(chan, &line.chan_list, list) {
    close_one_chan(chan, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn deactivate_chan(chan: *mut chan, irq: c_int) {
    void deactivate_chan(struct chan *chan, int irq)
    {
    if (chan && chan.enabled)
    deactivate_fd(chan.fd_in, irq);
    }
#[no_mangle]
pub unsafe extern "C" fn write_chan(chan: *mut chan, buf: *const u8, len: usize, write_irq: c_int) -> c_int {
    int write_chan(struct chan *chan, const u8 *buf, size_t len, int write_irq)
    {
    int n, ret = 0;
    if (len == 0 || !chan || !chan.ops.write)
    return 0;
    n = chan.ops.write(chan.fd_out, buf, len, chan.data);
    if (chan.primary) {
    ret = n;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn console_write_chan(chan: *mut chan, buf: *const c_char, len: c_int) -> c_int {
    int console_write_chan(struct chan *chan, const char *buf, int len)
    {
    int n, ret = 0;
    if (!chan || !chan.ops.console_write)
    return 0;
    n = chan.ops.console_write(chan.fd_out, buf, len);
    if (chan.primary)
    ret = n;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn console_open_chan(line: *mut line, co: *mut console) -> c_int {
    int console_open_chan(struct line *line, struct console *co)
    {
    int err;
    err = open_chan(&line.chan_list);
    if (err)
    return err;
    printk(KERN_INFO "Console initialized on /dev/%s%d\n", co.name,
    co.index);
    return 0;
    }
    int chan_window_size(struct line *line, unsigned short *rows_out,
    unsigned short *cols_out)
    {
    struct chan *chan;
    chan = line.chan_in;
    if (chan && chan.primary) {
    if (chan.ops.window_size == core::ptr::null_mut())
    return 0;
    return chan.ops.window_size(chan.fd_in, chan.data,
    rows_out, cols_out);
    }
    chan = line.chan_out;
    if (chan && chan.primary) {
    if (chan.ops.window_size == core::ptr::null_mut())
    return 0;
    return chan.ops.window_size(chan.fd_in, chan.data,
    rows_out, cols_out);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_one_chan(chan: *mut chan) {
    static void free_one_chan(struct chan *chan)
    {
    list_del(&chan.list);
    close_one_chan(chan, 0);
    if (chan.ops.free != core::ptr::null_mut())
    (*chan.ops.free)(chan.data);
    if (chan.primary && chan.output)
    ignore_sigio_fd(chan.fd_in);
    kfree(chan);
    }
#[no_mangle]
unsafe extern "C" fn free_chan(chans: *mut list_head) {
    static void free_chan(struct list_head *chans)
    {
    struct list_head *ele, *next;
    struct chan *chan;
    list_for_each_safe(ele, next, chans) {
    chan = list_entry(ele, struct chan, list);
    free_one_chan(chan);
    }
    }
    static int one_chan_config_string(struct chan *chan, char *str, int size,
    char **error_out)
    {
    let mut n: c_int = 0;
    if (chan == core::ptr::null_mut()) {
    CONFIG_CHUNK(str, size, n, "none", 1);
    return n;
    }
    CONFIG_CHUNK(str, size, n, chan.ops.type, 0);
    if (chan.dev == core::ptr::null_mut()) {
    CONFIG_CHUNK(str, size, n, "", 1);
    return n;
    }
    CONFIG_CHUNK(str, size, n, ":", 0);
    CONFIG_CHUNK(str, size, n, chan.dev, 0);
    return n;
    }
    static int chan_pair_config_string(struct chan *in, struct chan *out,
    char *str, int size, char **error_out)
    {
    int n;
    n = one_chan_config_string(in, str, size, error_out);
    str += n;
    size -= n;
    if (in == out) {
    CONFIG_CHUNK(str, size, n, "", 1);
    return n;
    }
    CONFIG_CHUNK(str, size, n, ",", 1);
    n = one_chan_config_string(out, str, size, error_out);
    str += n;
    size -= n;
    CONFIG_CHUNK(str, size, n, "", 1);
    return n;
    }
    int chan_config_string(struct line *line, char *str, int size,
    char **error_out)
    {
    struct chan *in = line.chan_in, *out = line.chan_out;
    if (in && !in.primary)
    in = core::ptr::null_mut();
    if (out && !out.primary)
    out = core::ptr::null_mut();
    return chan_pair_config_string(in, out, str, size, error_out);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_type {
    pub key: *mut c_char,
    pub ops: *const chan_ops,
}

    static const struct chan_type chan_table[] = {
    { "fd", &fd_ops },

    { "null", &null_ops },

    { "null", &not_configged_ops },

    { "port", &port_ops },

    { "port", &not_configged_ops },

    { "pty", &pty_ops },
    { "pts", &pts_ops },

    { "pty", &not_configged_ops },
    { "pts", &not_configged_ops },

    { "tty", &tty_ops },

    { "tty", &not_configged_ops },

    { "xterm", &xterm_ops },

    { "xterm", &not_configged_ops },

    };
    static struct chan *parse_chan(struct line *line, char *str, int device,
    const struct chan_opts *opts, char **error_out)
    {
    const struct chan_type *entry;
    const struct chan_ops *ops;
    struct chan *chan;
    void *data;
    int i;
    ops = core::ptr::null_mut();
    data = core::ptr::null_mut();
    for(i = 0; i < ARRAY_SIZE(chan_table); i++) {
    entry = &chan_table[i];
    if (!strncmp(str, entry.key, strlen(entry.key))) {
    ops = entry.ops;
    str += strlen(entry.key);
    break;
    }
    }
    if (ops == core::ptr::null_mut()) {
// error_out = "No match for configured backends";
    return core::ptr::null_mut();
    }
    data = (*ops.init)(str, device, opts);
    if (data == core::ptr::null_mut()) {
// error_out = "Configuration failed";
    return core::ptr::null_mut();
    }
    chan = kmalloc_obj(*chan, GFP_ATOMIC);
    if (chan == core::ptr::null_mut()) {
// error_out = "Memory allocation failed";
    return core::ptr::null_mut();
    }
// chan = ((struct chan) { .list	 	= LIST_HEAD_INIT(chan->list),
    .free_list 	=
    LIST_HEAD_INIT(chan.free_list),
    .line		= line,
    .primary	= 1,
    .input		= 0,
    .output 	= 0,
    .opened  	= 0,
    .enabled  	= 0,
    .fd_in		= -1,
    .fd_out	= -1,
    .ops 		= ops,
    .data 		= data });
    return chan;
    }
    int parse_chan_pair(char *str, struct line *line, int device,
    const struct chan_opts *opts, char **error_out)
    {
    struct list_head *chans = &line.chan_list;
    struct chan *new;
    char *in, *out;
    if (!list_empty(chans)) {
    line.chan_in = line.chan_out = core::ptr::null_mut();
    free_chan(chans);
    INIT_LIST_HEAD(chans);
    }
    if (!str)
    return 0;
    out = strchr(str, ',');
    if (out != core::ptr::null_mut()) {
    in = str;
// out = '\0';
    out++;
    new = parse_chan(line, in, device, opts, error_out);
    if (new == core::ptr::null_mut())
    return -1;
    new.input = 1;
    list_add(&new.list, chans);
    line.chan_in = new;
    new = parse_chan(line, out, device, opts, error_out);
    if (new == core::ptr::null_mut())
    return -1;
    list_add(&new.list, chans);
    new.output = 1;
    line.chan_out = new;
    }
    else {
    new = parse_chan(line, str, device, opts, error_out);
    if (new == core::ptr::null_mut())
    return -1;
    list_add(&new.list, chans);
    new.input = 1;
    new.output = 1;
    line.chan_in = line.chan_out = new;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn chan_interrupt(line: *mut line, irq: c_int) {
    void chan_interrupt(struct line *line, int irq)
    {
    struct tty_port *port = &line.port;
    struct chan *chan = line.chan_in;
    int err;
    u8 c;
    if (!chan || !chan.ops.read)
    goto out;
    do {
    if (!tty_buffer_request_room(port, 1)) {
    schedule_delayed_work(&line.task, 1);
    goto out;
    }
    err = chan.ops.read(chan.fd_in, &c, chan.data);
    if (err > 0)
    tty_insert_flip_char(port, c, TTY_NORMAL);
    } while (err > 0);
    if (err == -EIO) {
    if (chan.primary) {
    tty_port_tty_hangup(&line.port, false);
    if (line.chan_out != chan)
    close_one_chan(line.chan_out, 1);
    }
    close_one_chan(chan, 1);
    if (chan.primary)
    return;
    }
    out:
    tty_flip_buffer_push(port);
    }
