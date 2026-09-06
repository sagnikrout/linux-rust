//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/cfag12864b.c
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
// Filename: cfag12864b.c
// Version: 0.1.0
// Description: cfag12864b LCD driver
// Depends: ks0108
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-31
//

//
// Module Parameters
//
    let mut cfag12864b_rate: static unsigned int = CONFIG_CFAG12864B_RATE;
    module_param(cfag12864b_rate, uint, 0444);
    MODULE_PARM_DESC(cfag12864b_rate,
    "Refresh rate (hertz)");
//
// cfag12864b Commands
//
// E = Enable signal
// Every time E switch from low to high,
// cfag12864b/ks0108 reads the command/data.
//
// CS1 = First ks0108controller.
// If high, the first ks0108 controller receives commands/data.
//
// CS2 = Second ks0108 controller
// If high, the second ks0108 controller receives commands/data.
//
// DI = Data/Instruction
// If low, cfag12864b will expect commands.
// If high, cfag12864b will expect data.
//

    static unsigned char cfag12864b_state;
#[no_mangle]
unsafe extern "C" fn cfag12864b_set() {
    static void cfag12864b_set(void)
    {
    ks0108_writecontrol(cfag12864b_state);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_setbit(state: c_uchar, n: c_uchar) {
    static void cfag12864b_setbit(unsigned char state, unsigned char n)
    {
    if (state)
    cfag12864b_state |= bit(n);
    else
    cfag12864b_state &= ~bit(n);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_e(state: c_uchar) {
    static void cfag12864b_e(unsigned char state)
    {
    cfag12864b_setbit(state, CFAG12864B_BIT_E);
    cfag12864b_set();
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_cs1(state: c_uchar) {
    static void cfag12864b_cs1(unsigned char state)
    {
    cfag12864b_setbit(state, CFAG12864B_BIT_CS1);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_cs2(state: c_uchar) {
    static void cfag12864b_cs2(unsigned char state)
    {
    cfag12864b_setbit(state, CFAG12864B_BIT_CS2);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_di(state: c_uchar) {
    static void cfag12864b_di(unsigned char state)
    {
    cfag12864b_setbit(state, CFAG12864B_BIT_DI);
    }
    static void cfag12864b_setcontrollers(unsigned char first,
    unsigned char second)
    {
    if (first)
    cfag12864b_cs1(0);
    else
    cfag12864b_cs1(1);
    if (second)
    cfag12864b_cs2(0);
    else
    cfag12864b_cs2(1);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_controller(which: c_uchar) {
    static void cfag12864b_controller(unsigned char which)
    {
    if (which == 0)
    cfag12864b_setcontrollers(1, 0);
#[no_mangle]
pub unsafe extern "C" fn if(1: which ==) -> else {
    else if (which == 1)
    cfag12864b_setcontrollers(0, 1);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_displaystate(state: c_uchar) {
    static void cfag12864b_displaystate(unsigned char state)
    {
    cfag12864b_di(0);
    cfag12864b_e(1);
    ks0108_displaystate(state);
    cfag12864b_e(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_address(address: c_uchar) {
    static void cfag12864b_address(unsigned char address)
    {
    cfag12864b_di(0);
    cfag12864b_e(1);
    ks0108_address(address);
    cfag12864b_e(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_page(page: c_uchar) {
    static void cfag12864b_page(unsigned char page)
    {
    cfag12864b_di(0);
    cfag12864b_e(1);
    ks0108_page(page);
    cfag12864b_e(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_startline(startline: c_uchar) {
    static void cfag12864b_startline(unsigned char startline)
    {
    cfag12864b_di(0);
    cfag12864b_e(1);
    ks0108_startline(startline);
    cfag12864b_e(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_writebyte(byte: c_uchar) {
    static void cfag12864b_writebyte(unsigned char byte)
    {
    cfag12864b_di(1);
    cfag12864b_e(1);
    ks0108_writedata(byte);
    cfag12864b_e(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_nop() {
    static void cfag12864b_nop(void)
    {
    cfag12864b_startline(0);
    }
//
// cfag12864b Internal Commands
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_on() {
    static void cfag12864b_on(void)
    {
    cfag12864b_setcontrollers(1, 1);
    cfag12864b_displaystate(1);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_off() {
    static void cfag12864b_off(void)
    {
    cfag12864b_setcontrollers(1, 1);
    cfag12864b_displaystate(0);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_clear() {
    static void cfag12864b_clear(void)
    {
    unsigned char i, j;
    cfag12864b_setcontrollers(1, 1);
    for (i = 0; i < CFAG12864B_PAGES; i++) {
    cfag12864b_page(i);
    cfag12864b_address(0);
    for (j = 0; j < CFAG12864B_ADDRESSES; j++)
    cfag12864b_writebyte(0);
    }
    }
//
// Update work
//
    unsigned char *cfag12864b_buffer;
    static unsigned char *cfag12864b_cache;
    static DEFINE_MUTEX(cfag12864b_mutex);
    static unsigned char cfag12864b_updating;
    static void cfag12864b_update(struct work_struct *delayed_work);
    static struct workqueue_struct *cfag12864b_workqueue;
    static DECLARE_DELAYED_WORK(cfag12864b_work, cfag12864b_update);
#[no_mangle]
unsafe extern "C" fn cfag12864b_queue() {
    static void cfag12864b_queue(void)
    {
    queue_delayed_work(cfag12864b_workqueue, &cfag12864b_work,
    HZ / cfag12864b_rate);
    }
#[no_mangle]
pub unsafe extern "C" fn cfag12864b_enable() -> c_uchar {
    unsigned char cfag12864b_enable(void)
    {
    unsigned char ret;
    mutex_lock(&cfag12864b_mutex);
    if (!cfag12864b_updating) {
    cfag12864b_updating = 1;
    cfag12864b_queue();
    ret = 0;
    } else
    ret = 1;
    mutex_unlock(&cfag12864b_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cfag12864b_disable() {
    void cfag12864b_disable(void)
    {
    mutex_lock(&cfag12864b_mutex);
    if (cfag12864b_updating) {
    cfag12864b_updating = 0;
    cancel_delayed_work(&cfag12864b_work);
    flush_workqueue(cfag12864b_workqueue);
    }
    mutex_unlock(&cfag12864b_mutex);
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_update(work: *mut work_struct) {
    static void cfag12864b_update(struct work_struct *work)
    {
    unsigned char c;
    unsigned short i, j, k, b;
    if (memcmp(cfag12864b_cache, cfag12864b_buffer, CFAG12864B_SIZE)) {
    for (i = 0; i < CFAG12864B_CONTROLLERS; i++) {
    cfag12864b_controller(i);
    cfag12864b_nop();
    for (j = 0; j < CFAG12864B_PAGES; j++) {
    cfag12864b_page(j);
    cfag12864b_nop();
    cfag12864b_address(0);
    cfag12864b_nop();
    for (k = 0; k < CFAG12864B_ADDRESSES; k++) {
    for (c = 0, b = 0; b < 8; b++)
    if (cfag12864b_buffer
    [i * CFAG12864B_ADDRESSES / 8
    + k / 8 + (j * 8 + b) *
    CFAG12864B_WIDTH / 8]
    & bit(k % 8))
    c |= bit(b);
    cfag12864b_writebyte(c);
    }
    }
    }
    memcpy(cfag12864b_cache, cfag12864b_buffer, CFAG12864B_SIZE);
    }
    if (cfag12864b_updating)
    cfag12864b_queue();
    }
//
// cfag12864b Exported Symbols
//
    EXPORT_SYMBOL_GPL(cfag12864b_buffer);
    EXPORT_SYMBOL_GPL(cfag12864b_enable);
    EXPORT_SYMBOL_GPL(cfag12864b_disable);
//
// Is the module inited?
//
    static unsigned char cfag12864b_inited;
#[no_mangle]
pub unsafe extern "C" fn cfag12864b_isinited() -> c_uchar {
    unsigned char cfag12864b_isinited(void)
    {
    return cfag12864b_inited;
    }
    EXPORT_SYMBOL_GPL(cfag12864b_isinited);
//
// Module Init & Exit
//
#[no_mangle]
unsafe extern "C" fn cfag12864b_init() -> int __init {
    static int __init cfag12864b_init(void)
    {
    let mut ret: c_int = -EINVAL;
// ks0108_init() must be called first
    if (!ks0108_isinited()) {
    printk(KERN_ERR CFAG12864B_NAME ": ERROR: "
    "ks0108 is not initialized\n");
    goto none;
    }
    BUILD_BUG_ON(PAGE_SIZE < CFAG12864B_SIZE);
    cfag12864b_buffer = (unsigned char *) get_zeroed_page(GFP_KERNEL);
    if (cfag12864b_buffer == core::ptr::null_mut()) {
    printk(KERN_ERR CFAG12864B_NAME ": ERROR: "
    "can't get a free page\n");
    ret = -ENOMEM;
    goto none;
    }
    cfag12864b_cache = kmalloc(CFAG12864B_SIZE,
    GFP_KERNEL);
    if (cfag12864b_cache == core::ptr::null_mut()) {
    printk(KERN_ERR CFAG12864B_NAME ": ERROR: "
    "can't alloc cache buffer (%i bytes)\n",
    CFAG12864B_SIZE);
    ret = -ENOMEM;
    goto bufferalloced;
    }
    cfag12864b_workqueue = create_singlethread_workqueue(CFAG12864B_NAME);
    if (cfag12864b_workqueue == core::ptr::null_mut())
    goto cachealloced;
    cfag12864b_clear();
    cfag12864b_on();
    cfag12864b_inited = 1;
    return 0;
    cachealloced:
    kfree(cfag12864b_cache);
    bufferalloced:
    free_page((unsigned long) cfag12864b_buffer);
    none:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cfag12864b_exit() -> void __exit {
    static void __exit cfag12864b_exit(void)
    {
    cfag12864b_disable();
    cfag12864b_off();
    destroy_workqueue(cfag12864b_workqueue);
    kfree(cfag12864b_cache);
    free_page((unsigned long) cfag12864b_buffer);
    }
    module_init(cfag12864b_init);
    module_exit(cfag12864b_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Miguel Ojeda <ojeda@kernel.org>");
    MODULE_DESCRIPTION("cfag12864b LCD driver");
