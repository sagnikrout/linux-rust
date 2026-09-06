//! Automatically rewritten from C to Rust
//! Source: sound/hda/core/bus.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// HD-audio core bus driver
//

    static void snd_hdac_bus_process_unsol_events(struct work_struct *work);
    static const struct hdac_bus_ops default_ops = {
    .command = snd_hdac_bus_send_cmd,
    .get_response = snd_hdac_bus_get_response,
    .link_power = snd_hdac_bus_link_power,
    };
//
// snd_hdac_bus_init - initialize a HD-audio bas bus
// @bus: the pointer to bus object
// @dev: device pointer
// @ops: bus verb operators
//
// Returns 0 if successful, or a negative error code.
//
    int snd_hdac_bus_init(struct hdac_bus *bus, struct device *dev,
    const struct hdac_bus_ops *ops)
    {
    memset(bus, 0, sizeof(*bus));
    bus.dev = dev;
    if (ops)
    bus.ops = ops;
    else
    bus.ops = &default_ops;
    bus.dma_type = SNDRV_DMA_TYPE_DEV;
    INIT_LIST_HEAD(&bus.stream_list);
    INIT_LIST_HEAD(&bus.codec_list);
    INIT_WORK(&bus.unsol_work, snd_hdac_bus_process_unsol_events);
    spin_lock_init(&bus.reg_lock);
    mutex_init(&bus.cmd_mutex);
    mutex_init(&bus.lock);
    INIT_LIST_HEAD(&bus.hlink_list);
    init_waitqueue_head(&bus.rirb_wq);
    bus.irq = -1;
    bus.addr_offset = 0;
//
// Default value of '8' is as per the HD audio specification (Rev 1.0a).
// Following relation is used to derive STRIPE control value.
// For sample rate <= 48K:
// { ((num_channels * bits_per_sample) / number of SDOs) >= 8 }
// For sample rate > 48K:
// { ((num_channels * bits_per_sample * rate/48000)
// number of SDOs) >= 8 }
//
    bus.sdo_limit = 8;
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_bus_init);
//
// snd_hdac_bus_exit - clean up a HD-audio bas bus
// @bus: the pointer to bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_exit(bus: *mut hdac_bus) {
    void snd_hdac_bus_exit(struct hdac_bus *bus)
    {
    WARN_ON(!list_empty(&bus.stream_list));
    WARN_ON(!list_empty(&bus.codec_list));
    cancel_work_sync(&bus.unsol_work);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_bus_exit);
//
// snd_hdac_bus_exec_verb - execute a HD-audio verb on the given bus
// @bus: bus object
// @addr: the HDAC device address
// @cmd: HD-audio encoded verb
// @res: pointer to store the response, NULL if performing asynchronously
//
// Returns 0 if successful, or a negative error code.
//
    int snd_hdac_bus_exec_verb(struct hdac_bus *bus, unsigned int addr,
    unsigned int cmd, unsigned int *res)
    {
    guard(mutex)(&bus.cmd_mutex);
    return snd_hdac_bus_exec_verb_unlocked(bus, addr, cmd, res);
    }
//
// snd_hdac_bus_exec_verb_unlocked - unlocked version
// @bus: bus object
// @addr: the HDAC device address
// @cmd: HD-audio encoded verb
// @res: pointer to store the response, NULL if performing asynchronously
//
// Returns 0 if successful, or a negative error code.
//
    int snd_hdac_bus_exec_verb_unlocked(struct hdac_bus *bus, unsigned int addr,
    unsigned int cmd, unsigned int *res)
    {
    unsigned int tmp;
    int err;
    if (cmd == ~0)
    return -EINVAL;
    if (res)
// res = -1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: bus->sync_write) -> else {
    else if (bus.sync_write)
    res = &tmp;
    for (;;) {
    trace_hda_send_cmd(bus, cmd);
    err = bus.ops.command(bus, cmd);
    if (err != -EAGAIN)
    break;
// process pending verbs
    err = bus.ops.get_response(bus, addr, &tmp);
    if (err)
    break;
    }
    if (!err && res) {
    err = bus.ops.get_response(bus, addr, res);
    trace_hda_get_response(bus, addr, *res);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_bus_exec_verb_unlocked);
//
// snd_hdac_bus_queue_event - add an unsolicited event to queue
// @bus: the BUS
// @res: unsolicited event (lower 32bit of RIRB entry)
// @res_ex: codec addr and flags (upper 32bit or RIRB entry)
//
// Adds the given event to the queue.  The events are processed in
// the workqueue asynchronously.  Call this function in the interrupt
// hanlder when RIRB receives an unsolicited event.
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_queue_event(bus: *mut hdac_bus, res: u32, res_ex: u32) {
    void snd_hdac_bus_queue_event(struct hdac_bus *bus, u32 res, u32 res_ex)
    {
    unsigned int wp;
    if (!bus)
    return;
    trace_hda_unsol_event(bus, res, res_ex);
    wp = (bus.unsol_wp + 1) % HDA_UNSOL_QUEUE_SIZE;
    bus.unsol_wp = wp;
    wp <<= 1;
    bus.unsol_queue[wp] = res;
    bus.unsol_queue[wp + 1] = res_ex;
    schedule_work(&bus.unsol_work);
    }
//
// process queued unsolicited events
//
#[no_mangle]
unsafe extern "C" fn snd_hdac_bus_process_unsol_events(work: *mut work_struct) {
    static void snd_hdac_bus_process_unsol_events(struct work_struct *work)
    {
    struct hdac_bus *bus = container_of(work, struct hdac_bus, unsol_work);
    struct hdac_device *codec;
    struct hdac_driver *drv;
    unsigned int rp, caddr, res;
    spin_lock_irq(&bus.reg_lock);
    while (bus.unsol_rp != bus.unsol_wp) {
    rp = (bus.unsol_rp + 1) % HDA_UNSOL_QUEUE_SIZE;
    bus.unsol_rp = rp;
    rp <<= 1;
    res = bus.unsol_queue[rp];
    caddr = bus.unsol_queue[rp + 1];
    if (!(caddr & (1 << 4))) /* no unsolicited event? */
    continue;
    codec = bus.caddr_tbl[caddr & 0x0f];
    if (!codec || !codec.registered)
    continue;
    spin_unlock_irq(&bus.reg_lock);
    drv = drv_to_hdac_driver(codec.dev.driver);
    if (drv.unsol_event)
    drv.unsol_event(codec, res);
    spin_lock_irq(&bus.reg_lock);
    }
    spin_unlock_irq(&bus.reg_lock);
    }
//
// snd_hdac_bus_add_device - Add a codec to bus
// @bus: HDA core bus
// @codec: HDA core device to add
//
// Adds the given codec to the list in the bus.  The caddr_tbl array
// and codec_powered bits are updated, as well.
// Returns zero if success, or a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_add_device(bus: *mut hdac_bus, codec: *mut hdac_device) -> c_int {
    int snd_hdac_bus_add_device(struct hdac_bus *bus, struct hdac_device *codec)
    {
    if (bus.caddr_tbl[codec.addr]) {
    dev_err(bus.dev, "address 0x%x is already occupied\n",
    codec.addr);
    return -EBUSY;
    }
    list_add_tail(&codec.list, &bus.codec_list);
    bus.caddr_tbl[codec.addr] = codec;
    set_bit(codec.addr, &bus.codec_powered);
    bus.num_codecs++;
    return 0;
    }
//
// snd_hdac_bus_remove_device - Remove a codec from bus
// @bus: HDA core bus
// @codec: HDA core device to remove
//
    void snd_hdac_bus_remove_device(struct hdac_bus *bus,
    struct hdac_device *codec)
    {
    WARN_ON(bus != codec.bus);
    if (list_empty(&codec.list))
    return;
    list_del_init(&codec.list);
    bus.caddr_tbl[codec.addr] = core::ptr::null_mut();
    clear_bit(codec.addr, &bus.codec_powered);
    bus.num_codecs--;
    flush_work(&bus.unsol_work);
    }

// Helpers for aligned read/write of mmio space, for Tegra
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_aligned_read(addr: *mut void __iomem, mask: c_uint) -> c_uint {
    unsigned int snd_hdac_aligned_read(void __iomem *addr, unsigned int mask)
    {
    void __iomem *aligned_addr =
    (void __iomem *)((unsigned long)(addr) & ~0x3);
    let mut shift: c_uint = ((unsigned long)(addr) & 0x3) << 3;
    unsigned int v;
    v = readl(aligned_addr);
    return (v >> shift) & mask;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_aligned_read);
    void snd_hdac_aligned_write(unsigned int val, void __iomem *addr,
    unsigned int mask)
    {
    void __iomem *aligned_addr =
    (void __iomem *)((unsigned long)(addr) & ~0x3);
    let mut shift: c_uint = ((unsigned long)(addr) & 0x3) << 3;
    unsigned int v;
    v = readl(aligned_addr);
    v &= ~(mask << shift);
    v |= val << shift;
    writel(v, aligned_addr);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_aligned_write);

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_codec_link_up(codec: *mut hdac_device) {
    void snd_hdac_codec_link_up(struct hdac_device *codec)
    {
    struct hdac_bus *bus = codec.bus;
    if (bus.ops.link_power)
    bus.ops.link_power(codec, true);
    else
    snd_hdac_bus_link_power(codec, true);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_codec_link_up);
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_codec_link_down(codec: *mut hdac_device) {
    void snd_hdac_codec_link_down(struct hdac_device *codec)
    {
    struct hdac_bus *bus = codec.bus;
    if (bus.ops.link_power)
    bus.ops.link_power(codec, false);
    else
    snd_hdac_bus_link_power(codec, false);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_codec_link_down);
