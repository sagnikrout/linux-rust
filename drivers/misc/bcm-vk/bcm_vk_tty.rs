//! Automatically rewritten from C to Rust
//! Source: drivers/misc/bcm-vk/bcm_vk_tty.c
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
// Copyright 2018-2020 Broadcom.
//

// TTYVK base offset is 0x30000 into BAR1
pub const BAR1_TTYVK_BASE_OFFSET: c_uint = 0x300000;
// Each TTYVK channel (TO or FROM) is 0x10000
pub const BAR1_TTYVK_CHAN_OFFSET: c_uint = 0x100000;
// Each TTYVK channel has TO and FROM, hence the * 2

    ((index) * BAR1_TTYVK_CHAN_OFFSET * 2))
// TO TTYVK channel base comes before FROM for each index

    BAR1_TTYVK_CHAN_OFFSET)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_tty_chan {
    pub reserved: u32,
    pub size: u32,
    pub wr: u32,
    pub rd: u32,
    pub data: *mut u32,
}

    + offsetof(struct bcm_vk_tty_chan, e))

pub const VK_BAR0_REGSEG_TTY_DB_OFFSET: c_uint = 0x86c;
// Poll every 1/10 of second - temp hack till we use MSI interrupt

#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_poll(t: *mut timer_list) {
    static void bcm_vk_tty_poll(struct timer_list *t)
    {
    struct bcm_vk *vk = timer_container_of(vk, t, serial_timer);
    queue_work(vk.tty_wq_thread, &vk.tty_wq_work);
    mod_timer(&vk.serial_timer, jiffies + SERIAL_TIMER_VALUE);
    }
#[no_mangle]
pub unsafe extern "C" fn bcm_vk_tty_irqhandler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    irqreturn_t bcm_vk_tty_irqhandler(int irq, void *dev_id)
    {
    struct bcm_vk *vk = dev_id;
    queue_work(vk.tty_wq_thread, &vk.tty_wq_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_wq_handler(work: *mut work_struct) {
    static void bcm_vk_tty_wq_handler(struct work_struct *work)
    {
    struct bcm_vk *vk = container_of(work, struct bcm_vk, tty_wq_work);
    struct bcm_vk_tty *vktty;
    int card_status;
    int count;
    int i;
    int wr;
    u8 c;
    card_status = vkread32(vk, BAR_0, BAR_CARD_STATUS);
    if (BCM_VK_INTF_IS_DOWN(card_status))
    return;
    for (i = 0; i < BCM_VK_NUM_TTY; i++) {
    count = 0;
// Check the card status that the tty channel is ready
    if ((card_status & BIT(i)) == 0)
    continue;
    vktty = &vk.tty[i];
// Don't increment read index if tty app is closed
    if (!vktty.is_opened)
    continue;
// Fetch the wr offset in buffer from VK
    wr = vkread32(vk, BAR_1, VK_BAR_CHAN_WR(vktty, from));
// safe to ignore until bar read gives proper size
    if (vktty.from_size == 0)
    continue;
    if (wr >= vktty.from_size) {
    dev_err(&vk.pdev.dev,
    "ERROR: wq handler ttyVK%d wr:0x%x > 0x%x\n",
    i, wr, vktty.from_size);
// Need to signal and close device in this case
    continue;
    }
//
// Simple read of circular buffer and
// insert into tty flip buffer
//
    while (vk.tty[i].rd != wr) {
    c = vkread8(vk, BAR_1,
    VK_BAR_CHAN_DATA(vktty, from, vktty.rd));
    vktty.rd++;
    if (vktty.rd >= vktty.from_size)
    vktty.rd = 0;
    tty_insert_flip_char(&vktty.port, c, TTY_NORMAL);
    count++;
    }
    if (count) {
    tty_flip_buffer_push(&vktty.port);
// Update read offset from shadow register to card
    vkwrite32(vk, vktty.rd, BAR_1,
    VK_BAR_CHAN_RD(vktty, from));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_open(tty: *mut tty_struct, file: *mut file) -> c_int {
    static int bcm_vk_tty_open(struct tty_struct *tty, struct file *file)
    {
    int card_status;
    struct bcm_vk *vk;
    struct bcm_vk_tty *vktty;
    int index;
// initialize the pointer in case something fails
    tty.driver_data = core::ptr::null_mut();
    vk = (struct bcm_vk *)dev_get_drvdata(tty.dev);
    index = tty.index;
    if (index >= BCM_VK_NUM_TTY)
    return -EINVAL;
    vktty = &vk.tty[index];
    vktty.pid = task_pid_nr(current);
    vktty.to_offset = TO_TTYK_BASE(index);
    vktty.from_offset = FROM_TTYK_BASE(index);
// Do not allow tty device to be opened if tty on card not ready
    card_status = vkread32(vk, BAR_0, BAR_CARD_STATUS);
    if (BCM_VK_INTF_IS_DOWN(card_status) || ((card_status & BIT(index)) == 0))
    return -EBUSY;
//
// Get shadow registers of the buffer sizes and the "to" write offset
// and "from" read offset
//
    vktty.to_size = vkread32(vk, BAR_1, VK_BAR_CHAN_SIZE(vktty, to));
    vktty.wr = vkread32(vk, BAR_1,  VK_BAR_CHAN_WR(vktty, to));
    vktty.from_size = vkread32(vk, BAR_1, VK_BAR_CHAN_SIZE(vktty, from));
    vktty.rd = vkread32(vk, BAR_1,  VK_BAR_CHAN_RD(vktty, from));
    vktty.is_opened = true;
    if (tty.count == 1 && !vktty.irq_enabled) {
    timer_setup(&vk.serial_timer, bcm_vk_tty_poll, 0);
    mod_timer(&vk.serial_timer, jiffies + SERIAL_TIMER_VALUE);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_close(tty: *mut tty_struct, file: *mut file) {
    static void bcm_vk_tty_close(struct tty_struct *tty, struct file *file)
    {
    struct bcm_vk *vk = dev_get_drvdata(tty.dev);
    if (tty.index >= BCM_VK_NUM_TTY)
    return;
    vk.tty[tty.index].is_opened = false;
    if (tty.count == 1)
    timer_delete_sync(&vk.serial_timer);
    }
#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_doorbell(vk: *mut bcm_vk, db_val: u32) {
    static void bcm_vk_tty_doorbell(struct bcm_vk *vk, u32 db_val)
    {
    vkwrite32(vk, db_val, BAR_0,
    VK_BAR0_REGSEG_DB_BASE + VK_BAR0_REGSEG_TTY_DB_OFFSET);
    }
    static ssize_t bcm_vk_tty_write(struct tty_struct *tty, const u8 *buffer,
    size_t count)
    {
    int index;
    struct bcm_vk *vk;
    struct bcm_vk_tty *vktty;
    size_t i;
    index = tty.index;
    vk = dev_get_drvdata(tty.dev);
    vktty = &vk.tty[index];
// Simple write each byte to circular buffer
    for (i = 0; i < count; i++) {
    vkwrite8(vk, buffer[i], BAR_1,
    VK_BAR_CHAN_DATA(vktty, to, vktty.wr));
    vktty.wr++;
    if (vktty.wr >= vktty.to_size)
    vktty.wr = 0;
    }
// Update write offset from shadow register to card
    vkwrite32(vk, vktty.wr, BAR_1, VK_BAR_CHAN_WR(vktty, to));
    bcm_vk_tty_doorbell(vk, 0);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn bcm_vk_tty_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int bcm_vk_tty_write_room(struct tty_struct *tty)
    {
    struct bcm_vk *vk = dev_get_drvdata(tty.dev);
    return vk.tty[tty.index].to_size - 1;
    }
    static const struct tty_operations serial_ops = {
    .open = bcm_vk_tty_open,
    .close = bcm_vk_tty_close,
    .write = bcm_vk_tty_write,
    .write_room = bcm_vk_tty_write_room,
    };
#[no_mangle]
pub unsafe extern "C" fn bcm_vk_tty_init(vk: *mut bcm_vk, name: *mut c_char) -> c_int {
    int bcm_vk_tty_init(struct bcm_vk *vk, char *name)
    {
    int i;
    int err;
    struct tty_driver *tty_drv;
    struct device *dev = &vk.pdev.dev;
    tty_drv = tty_alloc_driver
    (BCM_VK_NUM_TTY,
    TTY_DRIVER_REAL_RAW | TTY_DRIVER_DYNAMIC_DEV);
    if (IS_ERR(tty_drv))
    return PTR_ERR(tty_drv);
// Save struct tty_driver for uninstalling the device
    vk.tty_drv = tty_drv;
// initialize the tty driver
    tty_drv.driver_name = KBUILD_MODNAME;
    tty_drv.name = kstrdup(name, GFP_KERNEL);
    if (!tty_drv.name) {
    err = -ENOMEM;
    goto err_tty_driver_kref_put;
    }
    tty_drv.type = TTY_DRIVER_TYPE_SERIAL;
    tty_drv.subtype = SERIAL_TYPE_NORMAL;
    tty_drv.init_termios = tty_std_termios;
    tty_set_operations(tty_drv, &serial_ops);
// register the tty driver
    err = tty_register_driver(tty_drv);
    if (err) {
    dev_err(dev, "tty_register_driver failed\n");
    goto err_kfree_tty_name;
    }
    for (i = 0; i < BCM_VK_NUM_TTY; i++) {
    struct device *tty_dev;
    tty_port_init(&vk.tty[i].port);
    tty_dev = tty_port_register_device_attr(&vk.tty[i].port,
    tty_drv, i, dev, vk,
    core::ptr::null_mut());
    if (IS_ERR(tty_dev)) {
    err = PTR_ERR(tty_dev);
    goto unwind;
    }
    vk.tty[i].is_opened = false;
    }
    INIT_WORK(&vk.tty_wq_work, bcm_vk_tty_wq_handler);
    vk.tty_wq_thread = create_singlethread_workqueue("tty");
    if (!vk.tty_wq_thread) {
    dev_err(dev, "Fail to create tty workqueue thread\n");
    err = -ENOMEM;
    goto unwind;
    }
    return 0;
    unwind:
    while (--i >= 0)
    tty_port_unregister_device(&vk.tty[i].port, tty_drv, i);
    tty_unregister_driver(tty_drv);
    err_kfree_tty_name:
    kfree(tty_drv.name);
    tty_drv.name = core::ptr::null_mut();
    err_tty_driver_kref_put:
    tty_driver_kref_put(tty_drv);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bcm_vk_tty_exit(vk: *mut bcm_vk) {
    void bcm_vk_tty_exit(struct bcm_vk *vk)
    {
    int i;
    timer_delete_sync(&vk.serial_timer);
    for (i = 0; i < BCM_VK_NUM_TTY; ++i) {
    tty_port_unregister_device(&vk.tty[i].port,
    vk.tty_drv,
    i);
    tty_port_destroy(&vk.tty[i].port);
    }
    tty_unregister_driver(vk.tty_drv);
    kfree(vk.tty_drv.name);
    vk.tty_drv.name = core::ptr::null_mut();
    tty_driver_kref_put(vk.tty_drv);
    }
#[no_mangle]
pub unsafe extern "C" fn bcm_vk_tty_terminate_tty_user(vk: *mut bcm_vk) {
    void bcm_vk_tty_terminate_tty_user(struct bcm_vk *vk)
    {
    struct bcm_vk_tty *vktty;
    int i;
    for (i = 0; i < BCM_VK_NUM_TTY; ++i) {
    vktty = &vk.tty[i];
    if (vktty.pid)
    kill_pid(find_vpid(vktty.pid), SIGKILL, 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bcm_vk_tty_wq_exit(vk: *mut bcm_vk) {
    void bcm_vk_tty_wq_exit(struct bcm_vk *vk)
    {
    cancel_work_sync(&vk.tty_wq_work);
    destroy_workqueue(vk.tty_wq_thread);
    }
