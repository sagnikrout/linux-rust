//! Automatically rewritten from C to Rust
//! Source: drivers/xen/events/events_fifo.c
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
// Xen event channels (FIFO-based ABI)
//
// Copyright (C) 2013 Citrix Systems R&D ltd.
//
// This source code is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the
// License, or (at your option) any later version.
//
// Or, when distributed separately from the Linux kernel or
// incorporated into other software packages, subject to the following
// license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_fifo_queue {
    pub head: [u32; EVTCHN_FIFO_MAX_QUEUES],
}

    static DEFINE_PER_CPU(struct evtchn_fifo_control_block *, cpu_control_block);
    static DEFINE_PER_CPU(struct evtchn_fifo_queue, cpu_queue);
    static event_word_t *event_array[MAX_EVENT_ARRAY_PAGES] __read_mostly;
    static unsigned event_array_pages __read_mostly;
//
// sync_set_bit() and friends must be unsigned long aligned.
//

    (((unsigned long)w & 0x4UL) ? (EVTCHN_FIFO_ ##b + 32) : EVTCHN_FIFO_ ##b)

    static inline event_word_t *event_word_from_port(evtchn_port_t port)
    {
    let mut i: unsigned = port / EVENT_WORDS_PER_PAGE;
    return event_array[i] + port % EVENT_WORDS_PER_PAGE;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_max_channels() -> unsigned {
    static unsigned evtchn_fifo_max_channels(void)
    {
    return EVTCHN_FIFO_NR_CHANNELS;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_nr_channels() -> unsigned {
    static unsigned evtchn_fifo_nr_channels(void)
    {
    return event_array_pages * EVENT_WORDS_PER_PAGE;
    }
    static int init_control_block(int cpu,
    struct evtchn_fifo_control_block *control_block)
    {
    struct evtchn_fifo_queue *q = &per_cpu(cpu_queue, cpu);
    struct evtchn_init_control init_control;
    unsigned int i;
// Reset the control block and the local HEADs.
    clear_page(control_block);
    for (i = 0; i < EVTCHN_FIFO_MAX_QUEUES; i++)
    q.head[i] = 0;
    init_control.control_gfn = virt_to_gfn(control_block);
    init_control.offset      = 0;
    init_control.vcpu        = xen_vcpu_nr(cpu);
    return HYPERVISOR_event_channel_op(EVTCHNOP_init_control, &init_control);
    }
#[no_mangle]
unsafe extern "C" fn free_unused_array_pages() {
    static void free_unused_array_pages(void)
    {
    unsigned i;
    for (i = event_array_pages; i < MAX_EVENT_ARRAY_PAGES; i++) {
    if (!event_array[i])
    break;
    free_page((unsigned long)event_array[i]);
    event_array[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn init_array_page(array_page: *mut event_word_t) {
    static void init_array_page(event_word_t *array_page)
    {
    unsigned i;
    for (i = 0; i < EVENT_WORDS_PER_PAGE; i++)
    array_page[i] = 1 << EVTCHN_FIFO_MASKED;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_setup(port: evtchn_port_t) -> c_int {
    static int evtchn_fifo_setup(evtchn_port_t port)
    {
    unsigned new_array_pages;
    int ret;
    new_array_pages = port / EVENT_WORDS_PER_PAGE + 1;
    if (new_array_pages > MAX_EVENT_ARRAY_PAGES)
    return -EINVAL;
    while (event_array_pages < new_array_pages) {
    void *array_page;
    struct evtchn_expand_array expand_array;
// Might already have a page if we've resumed.
    array_page = event_array[event_array_pages];
    if (!array_page) {
    array_page = (void *)__get_free_page(GFP_KERNEL);
    if (array_page == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto error;
    }
    event_array[event_array_pages] = array_page;
    }
// Mask all events in this page before adding it.
    init_array_page(array_page);
    expand_array.array_gfn = virt_to_gfn(array_page);
    ret = HYPERVISOR_event_channel_op(EVTCHNOP_expand_array, &expand_array);
    if (ret < 0)
    goto error;
    event_array_pages++;
    }
    return 0;
    error:
    if (event_array_pages == 0)
    panic("xen: unable to expand event array with initial page (%d)\n", ret);
    else
    pr_err("unable to expand event array (%d)\n", ret);
    free_unused_array_pages();
    return ret;
    }
    static void evtchn_fifo_bind_to_cpu(evtchn_port_t evtchn, unsigned int cpu,
    unsigned int old_cpu)
    {
// no-op
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_clear_pending(port: evtchn_port_t) {
    static void evtchn_fifo_clear_pending(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    sync_clear_bit(EVTCHN_FIFO_BIT(PENDING, word), BM(word));
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_set_pending(port: evtchn_port_t) {
    static void evtchn_fifo_set_pending(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    sync_set_bit(EVTCHN_FIFO_BIT(PENDING, word), BM(word));
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_is_pending(port: evtchn_port_t) -> bool {
    static bool evtchn_fifo_is_pending(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    return sync_test_bit(EVTCHN_FIFO_BIT(PENDING, word), BM(word));
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_mask(port: evtchn_port_t) {
    static void evtchn_fifo_mask(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    sync_set_bit(EVTCHN_FIFO_BIT(MASKED, word), BM(word));
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_is_masked(port: evtchn_port_t) -> bool {
    static bool evtchn_fifo_is_masked(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    return sync_test_bit(EVTCHN_FIFO_BIT(MASKED, word), BM(word));
    }
//
// Clear MASKED if not PENDING, spinning if BUSY is set.
// Return true if mask was cleared.
//
#[no_mangle]
unsafe extern "C" fn clear_masked_cond(word: *mut volatile event_word_t) -> bool {
    static bool clear_masked_cond(volatile event_word_t *word)
    {
    event_word_t new, old;
    old = *word;
    do {
    if (!(old & (1 << EVTCHN_FIFO_MASKED)))
    return true;
    if (old & (1 << EVTCHN_FIFO_PENDING))
    return false;
    old = old & ~(1 << EVTCHN_FIFO_BUSY);
    new = old & ~(1 << EVTCHN_FIFO_MASKED);
    } while (!sync_try_cmpxchg(word, &old, new));
    return true;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_unmask(port: evtchn_port_t) {
    static void evtchn_fifo_unmask(evtchn_port_t port)
    {
    event_word_t *word = event_word_from_port(port);
    BUG_ON(!irqs_disabled());
    if (!clear_masked_cond(word)) {
    let mut unmask: evtchn_unmask = { .port = port };
    (void)HYPERVISOR_event_channel_op(EVTCHNOP_unmask, &unmask);
    }
    }
#[no_mangle]
unsafe extern "C" fn clear_linked(word: *mut volatile event_word_t) -> u32 {
    static uint32_t clear_linked(volatile event_word_t *word)
    {
    event_word_t new, old;
    old = *word;
    do {
    new = (old & ~((1 << EVTCHN_FIFO_LINKED)
    | EVTCHN_FIFO_LINK_MASK));
    } while (!sync_try_cmpxchg(word, &old, new));
    return old & EVTCHN_FIFO_LINK_MASK;
    }
    static void consume_one_event(unsigned cpu, struct evtchn_loop_ctrl *ctrl,
    struct evtchn_fifo_control_block *control_block,
    unsigned priority, unsigned long *ready)
    {
    struct evtchn_fifo_queue *q = &per_cpu(cpu_queue, cpu);
    uint32_t head;
    evtchn_port_t port;
    event_word_t *word;
    head = q.head[priority];
//
// Reached the tail last time?  Read the new HEAD from the
// control block.
//
    if (head == 0) {
    virt_rmb(); /* Ensure word is up-to-date before reading head. */
    head = control_block.head[priority];
    }
    port = head;
    word = event_word_from_port(port);
    head = clear_linked(word);
//
// If the link is non-zero, there are more events in the
// queue, otherwise the queue is empty.
//
// If the queue is empty, clear this priority from our local
// copy of the ready word.
//
    if (head == 0)
    clear_bit(priority, ready);
    if (evtchn_fifo_is_pending(port) && !evtchn_fifo_is_masked(port)) {
    if (unlikely(!ctrl))
    pr_warn("Dropping pending event for port %u\n", port);
    else
    handle_irq_for_port(port, ctrl);
    }
    q.head[priority] = head;
    }
    static void __evtchn_fifo_handle_events(unsigned cpu,
    struct evtchn_loop_ctrl *ctrl)
    {
    struct evtchn_fifo_control_block *control_block;
    unsigned long ready;
    unsigned q;
    control_block = per_cpu(cpu_control_block, cpu);
    ready = xchg(&control_block.ready, 0);
    while (ready) {
    q = find_first_bit(&ready, EVTCHN_FIFO_MAX_QUEUES);
    consume_one_event(cpu, ctrl, control_block, q, &ready);
    ready |= xchg(&control_block.ready, 0);
    }
    }
    static void evtchn_fifo_handle_events(unsigned cpu,
    struct evtchn_loop_ctrl *ctrl)
    {
    __evtchn_fifo_handle_events(cpu, ctrl);
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_resume() {
    static void evtchn_fifo_resume(void)
    {
    unsigned cpu;
    for_each_possible_cpu(cpu) {
    void *control_block = per_cpu(cpu_control_block, cpu);
    int ret;
    if (!control_block)
    continue;
//
// If this CPU is offline, take the opportunity to
// free the control block while it is not being
// used.
//
    if (!cpu_online(cpu)) {
    free_page((unsigned long)control_block);
    per_cpu(cpu_control_block, cpu) = core::ptr::null_mut();
    continue;
    }
    ret = init_control_block(cpu, control_block);
    BUG_ON(ret < 0);
    }
//
// The event array starts out as empty again and is extended
// as normal when events are bound.  The existing pages will
// be reused.
//
    event_array_pages = 0;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_alloc_control_block(cpu: unsigned) -> c_int {
    static int evtchn_fifo_alloc_control_block(unsigned cpu)
    {
    void *control_block = core::ptr::null_mut();
    let mut ret: c_int = -ENOMEM;
    control_block = (void *)__get_free_page(GFP_KERNEL);
    if (control_block == core::ptr::null_mut())
    goto error;
    ret = init_control_block(cpu, control_block);
    if (ret < 0)
    goto error;
    per_cpu(cpu_control_block, cpu) = control_block;
    return 0;
    error:
    free_page((unsigned long)control_block);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_percpu_init(cpu: c_uint) -> c_int {
    static int evtchn_fifo_percpu_init(unsigned int cpu)
    {
    if (!per_cpu(cpu_control_block, cpu))
    return evtchn_fifo_alloc_control_block(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn evtchn_fifo_percpu_deinit(cpu: c_uint) -> c_int {
    static int evtchn_fifo_percpu_deinit(unsigned int cpu)
    {
    __evtchn_fifo_handle_events(cpu, core::ptr::null_mut());
    return 0;
    }
    static const struct evtchn_ops evtchn_ops_fifo = {
    .max_channels      = evtchn_fifo_max_channels,
    .nr_channels       = evtchn_fifo_nr_channels,
    .setup             = evtchn_fifo_setup,
    .bind_to_cpu       = evtchn_fifo_bind_to_cpu,
    .clear_pending     = evtchn_fifo_clear_pending,
    .set_pending       = evtchn_fifo_set_pending,
    .is_pending        = evtchn_fifo_is_pending,
    .mask              = evtchn_fifo_mask,
    .unmask            = evtchn_fifo_unmask,
    .handle_events     = evtchn_fifo_handle_events,
    .resume            = evtchn_fifo_resume,
    .percpu_init       = evtchn_fifo_percpu_init,
    .percpu_deinit     = evtchn_fifo_percpu_deinit,
    };
#[no_mangle]
pub unsafe extern "C" fn xen_evtchn_fifo_init() -> int __init {
    int __init xen_evtchn_fifo_init(void)
    {
    let mut cpu: c_int = smp_processor_id();
    int ret;
    ret = evtchn_fifo_alloc_control_block(cpu);
    if (ret < 0)
    return ret;
    pr_info("Using FIFO-based ABI\n");
    evtchn_ops = &evtchn_ops_fifo;
    return ret;
    }
