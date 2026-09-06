//! Automatically rewritten from C to Rust
//! Source: drivers/pci/vgaarb.c
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


// SPDX-License-Identifier: MIT
//
// vgaarb.c: Implements VGA arbitration. For details refer to
// Documentation/gpu/vgaarbiter.rst
//
// (C) Copyright 2005 Benjamin Herrenschmidt <benh@kernel.crashing.org>
// (C) Copyright 2007 Paulo R. Zanoni <przanoni@gmail.com>
// (C) Copyright 2007, 2009 Tiago Vignatti <vignatti@freedesktop.org>
//

    static void vga_arbiter_notify_clients(void);
//
// We keep a list of all VGA devices in the system to speed
// up the various operations of the arbiter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_device {
    pub list: list_head,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut unsigned int decodes; / what it decodes,
    pub /: *mut *mut unsigned int owns; / what it owns,
    pub /: *mut *mut unsigned int locks; / what it locks,
    pub /: *mut *mut unsigned int io_lock_cnt; / legacy IO lock count,
    pub /: *mut *mut unsigned int mem_lock_cnt; / legacy MEM lock count,
    pub /: *mut *mut unsigned int io_norm_cnt; / normal IO count,
    pub /: *mut *mut unsigned int mem_norm_cnt; / normal MEM count,
    pub bridge_has_one_vga: bool,
    pub /: *mut *mut bool is_firmware_default; / device selected by firmware,
    pub decode): *mut *mut *mut unsigned int (set_decode)(struct pci_dev pdev, bool,
}

    static LIST_HEAD(vga_list);
    static int vga_count, vga_decode_count;
    static bool vga_arbiter_used;
    static DEFINE_SPINLOCK(vga_lock);
    static DECLARE_WAIT_QUEUE_HEAD(vga_wait_queue);
    static const char *vga_iostate_to_str(unsigned int iostate)
    {
// Ignore VGA_RSRC_IO and VGA_RSRC_MEM
    iostate &= VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM;
    switch (iostate) {
    case VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM:
    return "io+mem";
    case VGA_RSRC_LEGACY_IO:
    return "io";
    case VGA_RSRC_LEGACY_MEM:
    return "mem";
    }
    return "none";
    }
#[no_mangle]
unsafe extern "C" fn vga_str_to_iostate(buf: *mut c_char, str_size: c_int, io_state: *mut c_uint) -> c_int {
    static int vga_str_to_iostate(char *buf, int str_size, unsigned int *io_state)
    {
//
// In theory, we could hand out locks on IO and MEM separately to
// userspace, but this can cause deadlocks.
//
    if (strncmp(buf, "none", 4) == 0) {
// io_state = VGA_RSRC_NONE;
    return 1;
    }
// XXX We're not checking the str_size!
    if (strncmp(buf, "io+mem", 6) == 0)
    goto both;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(buf, _arg: "io", 0: 2) ==) -> else {
    else if (strncmp(buf, "io", 2) == 0)
    goto both;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(buf, _arg: "mem", 0: 3) ==) -> else {
    else if (strncmp(buf, "mem", 3) == 0)
    goto both;
    return 0;
    both:
// io_state = VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM;
    return 1;
    }
// This is only used as a cookie, it should not be dereferenced
    static struct pci_dev *vga_default;
// Find somebody in our list
    static struct vga_device *vgadev_find(struct pci_dev *pdev)
    {
    struct vga_device *vgadev;
    list_for_each_entry(vgadev, &vga_list, list)
    if (pdev == vgadev.pdev)
    return vgadev;
    return core::ptr::null_mut();
    }
//
// vga_default_device - return the default VGA device, for vgacon
//
// This can be defined by the platform. The default implementation is
// rather dumb and will probably only work properly on single VGA card
// setups and/or x86 platforms.
//
// If your VGA default device is not PCI, you'll have to return NULL here.
// In this case, I assume it will not conflict with any PCI card. If this
// is not true, I'll have to define two arch hooks for enabling/disabling
// the VGA default device if that is possible. This may be a problem with
// real _ISA_ VGA cards, in addition to a PCI one. I don't know at this
// point how to deal with that card. Can their IOs be disabled at all? If
// not, then I suppose it's a matter of having the proper arch hook telling
// us about it, so we basically never allow anybody to succeed a vga_get().
//
    struct pci_dev *vga_default_device(void)
    {
    return vga_default;
    }
    EXPORT_SYMBOL_GPL(vga_default_device);
#[no_mangle]
pub unsafe extern "C" fn vga_set_default_device(pdev: *mut pci_dev) {
    void vga_set_default_device(struct pci_dev *pdev)
    {
    if (vga_default == pdev)
    return;
    pci_dev_put(vga_default);
    vga_default = pci_dev_get(pdev);
    }
//
// vga_remove_vgacon - deactivate VGA console
//
// Unbind and unregister vgacon in case pdev is the default VGA device.
// Can be called by GPU drivers on initialization to make sure VGA register
// access done by vgacon will not disturb the device.
//
// @pdev: PCI device.
//

#[no_mangle]
pub unsafe extern "C" fn vga_remove_vgacon(pdev: *mut pci_dev) -> c_int {
    int vga_remove_vgacon(struct pci_dev *pdev)
    {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn vga_remove_vgacon(pdev: *mut pci_dev) -> c_int {
    int vga_remove_vgacon(struct pci_dev *pdev)
    {
    return -ENODEV;
    }

#[no_mangle]
pub unsafe extern "C" fn vga_remove_vgacon(pdev: *mut pci_dev) -> c_int {
    int vga_remove_vgacon(struct pci_dev *pdev)
    {
    let mut ret: c_int = 0;
    if (pdev != vga_default)
    return 0;
    vgaarb_info(&pdev.dev, "deactivate vga console\n");
    console_lock();
    if (con_is_bound(&vga_con))
    ret = do_take_over_console(&dummy_con, 0,
    MAX_NR_CONSOLES - 1, 1);
    if (ret == 0) {
    ret = do_unregister_con_driver(&vga_con);
// Ignore "already unregistered".
    if (ret == -ENODEV)
    ret = 0;
    }
    console_unlock();
    return ret;
    }

    EXPORT_SYMBOL(vga_remove_vgacon);
//
// If we don't ever use VGA arbitration, we should avoid turning off
// anything anywhere due to old X servers getting confused about the boot
// device not being VGA.
//
#[no_mangle]
unsafe extern "C" fn vga_check_first_use() {
    static void vga_check_first_use(void)
    {
//
// Inform all GPUs in the system that VGA arbitration has occurred
// so they can disable resources if possible.
//
    if (!vga_arbiter_used) {
    vga_arbiter_used = true;
    vga_arbiter_notify_clients();
    }
    }
    static struct vga_device *__vga_tryget(struct vga_device *vgadev,
    unsigned int rsrc)
    {
    struct device *dev = &vgadev.pdev.dev;
    unsigned int wants, legacy_wants, match;
    struct vga_device *conflict;
    unsigned int pci_bits;
    let mut flags: u32 = 0;
    int err;
//
// Account for "normal" resources to lock. If we decode the legacy,
// counterpart, we need to request it as well
//
    if ((rsrc & VGA_RSRC_NORMAL_IO) &&
    (vgadev.decodes & VGA_RSRC_LEGACY_IO))
    rsrc |= VGA_RSRC_LEGACY_IO;
    if ((rsrc & VGA_RSRC_NORMAL_MEM) &&
    (vgadev.decodes & VGA_RSRC_LEGACY_MEM))
    rsrc |= VGA_RSRC_LEGACY_MEM;
    vgaarb_dbg(dev, "%s: %d\n", __func__, rsrc);
    vgaarb_dbg(dev, "%s: owns: %d\n", __func__, vgadev.owns);
// Check what resources we need to acquire
    wants = rsrc & ~vgadev.owns;
// We already own everything, just mark locked & bye bye
    if (wants == 0)
    goto lock_them;
//
// We don't need to request a legacy resource, we just enable
// appropriate decoding and go.
//
    legacy_wants = wants & VGA_RSRC_LEGACY_MASK;
    if (legacy_wants == 0)
    goto enable_them;
// Ok, we don't, let's find out who we need to kick off
    list_for_each_entry(conflict, &vga_list, list) {
    let mut lwants: c_uint = legacy_wants;
    let mut change_bridge: c_uint = 0;
// Don't conflict with myself
    if (vgadev == conflict)
    continue;
//
// We have a possible conflict. Before we go further, we must
// check if we sit on the same bus as the conflicting device.
// If we don't, then we must tie both IO and MEM resources
// together since there is only a single bit controlling
// VGA forwarding on P2P bridges.
//
    if (vgadev.pdev.bus != conflict.pdev.bus) {
    change_bridge = 1;
    lwants = VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM;
    }
//
// Check if the guy has a lock on the resource. If he does,
// return the conflicting entry.
//
    if (conflict.locks & lwants)
    return conflict;
//
// Ok, now check if it owns the resource we want.  We can
// lock resources that are not decoded; therefore a device
// can own resources it doesn't decode.
//
    match = lwants & conflict.owns;
    if (!match)
    continue;
//
// Looks like he doesn't have a lock, we can steal them
// from him.
//
    flags = 0;
    pci_bits = 0;
//
// If we can't control legacy resources via the bridge, we
// also need to disable normal decoding.
//
    if (!conflict.bridge_has_one_vga) {
    if ((match & conflict.decodes) & VGA_RSRC_LEGACY_MEM)
    pci_bits |= PCI_COMMAND_MEMORY;
    if ((match & conflict.decodes) & VGA_RSRC_LEGACY_IO)
    pci_bits |= PCI_COMMAND_IO;
    if (pci_bits)
    flags |= PCI_VGA_STATE_CHANGE_DECODES;
    }
    if (change_bridge)
    flags |= PCI_VGA_STATE_CHANGE_BRIDGE;
    err = pci_set_vga_state(conflict.pdev, false, pci_bits, flags);
    if (err)
    return ERR_PTR(err);
    conflict.owns &= ~match;
// If we disabled normal decoding, reflect it in owns
    if (pci_bits & PCI_COMMAND_MEMORY)
    conflict.owns &= ~VGA_RSRC_NORMAL_MEM;
    if (pci_bits & PCI_COMMAND_IO)
    conflict.owns &= ~VGA_RSRC_NORMAL_IO;
    }
    enable_them:
//
// Ok, we got it, everybody conflicting has been disabled, let's
// enable us.  Mark any bits in "owns" regardless of whether we
// decoded them.  We can lock resources we don't decode, therefore
// we must track them via "owns".
//
    flags = 0;
    pci_bits = 0;
    if (!vgadev.bridge_has_one_vga) {
    flags |= PCI_VGA_STATE_CHANGE_DECODES;
    if (wants & (VGA_RSRC_LEGACY_MEM|VGA_RSRC_NORMAL_MEM))
    pci_bits |= PCI_COMMAND_MEMORY;
    if (wants & (VGA_RSRC_LEGACY_IO|VGA_RSRC_NORMAL_IO))
    pci_bits |= PCI_COMMAND_IO;
    }
    if (wants & VGA_RSRC_LEGACY_MASK)
    flags |= PCI_VGA_STATE_CHANGE_BRIDGE;
    err = pci_set_vga_state(vgadev.pdev, true, pci_bits, flags);
    if (err)
    return ERR_PTR(err);
    vgadev.owns |= wants;
    lock_them:
    vgadev.locks |= (rsrc & VGA_RSRC_LEGACY_MASK);
    if (rsrc & VGA_RSRC_LEGACY_IO)
    vgadev.io_lock_cnt++;
    if (rsrc & VGA_RSRC_LEGACY_MEM)
    vgadev.mem_lock_cnt++;
    if (rsrc & VGA_RSRC_NORMAL_IO)
    vgadev.io_norm_cnt++;
    if (rsrc & VGA_RSRC_NORMAL_MEM)
    vgadev.mem_norm_cnt++;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __vga_put(vgadev: *mut vga_device, rsrc: c_uint) {
    static void __vga_put(struct vga_device *vgadev, unsigned int rsrc)
    {
    struct device *dev = &vgadev.pdev.dev;
    let mut old_locks: c_uint = vgadev.locks;
    vgaarb_dbg(dev, "%s\n", __func__);
//
// Update our counters and account for equivalent legacy resources
// if we decode them.
//
    if ((rsrc & VGA_RSRC_NORMAL_IO) && vgadev.io_norm_cnt > 0) {
    vgadev.io_norm_cnt--;
    if (vgadev.decodes & VGA_RSRC_LEGACY_IO)
    rsrc |= VGA_RSRC_LEGACY_IO;
    }
    if ((rsrc & VGA_RSRC_NORMAL_MEM) && vgadev.mem_norm_cnt > 0) {
    vgadev.mem_norm_cnt--;
    if (vgadev.decodes & VGA_RSRC_LEGACY_MEM)
    rsrc |= VGA_RSRC_LEGACY_MEM;
    }
    if ((rsrc & VGA_RSRC_LEGACY_IO) && vgadev.io_lock_cnt > 0)
    vgadev.io_lock_cnt--;
    if ((rsrc & VGA_RSRC_LEGACY_MEM) && vgadev.mem_lock_cnt > 0)
    vgadev.mem_lock_cnt--;
//
// Just clear lock bits, we do lazy operations so we don't really
// have to bother about anything else at this point.
//
    if (vgadev.io_lock_cnt == 0)
    vgadev.locks &= ~VGA_RSRC_LEGACY_IO;
    if (vgadev.mem_lock_cnt == 0)
    vgadev.locks &= ~VGA_RSRC_LEGACY_MEM;
//
// Kick the wait queue in case somebody was waiting if we actually
// released something.
//
    if (old_locks != vgadev.locks)
    wake_up_all(&vga_wait_queue);
    }
//
// vga_get - acquire & lock VGA resources
// @pdev: PCI device of the VGA card or NULL for the system default
// @rsrc: bit mask of resources to acquire and lock
// @interruptible: blocking should be interruptible by signals ?
//
// Acquire VGA resources for the given card and mark those resources
// locked. If the resources requested are "normal" (and not legacy)
// resources, the arbiter will first check whether the card is doing legacy
// decoding for that type of resource. If yes, the lock is "converted" into
// a legacy resource lock.
//
// The arbiter will first look for all VGA cards that might conflict and disable
// their IOs and/or Memory access, including VGA forwarding on P2P bridges if
// necessary, so that the requested resources can be used. Then, the card is
// marked as locking these resources and the IO and/or Memory accesses are
// enabled on the card (including VGA forwarding on parent P2P bridges if any).
//
// This function will block if some conflicting card is already locking one of
// the required resources (or any resource on a different bus segment, since P2P
// bridges don't differentiate VGA memory and IO afaik). You can indicate
// whether this blocking should be interruptible by a signal (for userland
// interface) or not.
//
// Must not be called at interrupt time or in atomic context.  If the card
// already owns the resources, the function succeeds.  Nested calls are
// supported (a per-resource counter is maintained)
//
// On success, release the VGA resource again with vga_put().
//
// Returns:
//
// 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn vga_get(pdev: *mut pci_dev, rsrc: c_uint, interruptible: c_int) -> c_int {
    int vga_get(struct pci_dev *pdev, unsigned int rsrc, int interruptible)
    {
    struct vga_device *vgadev, *conflict;
    unsigned long flags;
    wait_queue_entry_t wait;
    let mut rc: c_int = 0;
    vga_check_first_use();
// The caller should check for this, but let's be sure
    if (pdev == core::ptr::null_mut())
    pdev = vga_default_device();
    if (pdev == core::ptr::null_mut())
    return 0;
    for (;;) {
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut()) {
    spin_unlock_irqrestore(&vga_lock, flags);
    rc = -ENODEV;
    break;
    }
    conflict = __vga_tryget(vgadev, rsrc);
    spin_unlock_irqrestore(&vga_lock, flags);
    if (IS_ERR(conflict)) {
    rc = PTR_ERR(conflict);
    break;
    }
    if (conflict == core::ptr::null_mut())
    break;
//
// We have a conflict; we wait until somebody kicks the
// work queue. Currently we have one work queue that we
// kick each time some resources are released, but it would
// be fairly easy to have a per-device one so that we only
// need to attach to the conflicting device.
//
    init_waitqueue_entry(&wait, current);
    add_wait_queue(&vga_wait_queue, &wait);
    set_current_state(interruptible ?
    TASK_INTERRUPTIBLE :
    TASK_UNINTERRUPTIBLE);
    if (interruptible && signal_pending(current)) {
    __set_current_state(TASK_RUNNING);
    remove_wait_queue(&vga_wait_queue, &wait);
    rc = -ERESTARTSYS;
    break;
    }
    schedule();
    remove_wait_queue(&vga_wait_queue, &wait);
    }
    return rc;
    }
    EXPORT_SYMBOL(vga_get);
//
// vga_tryget - try to acquire & lock legacy VGA resources
// @pdev: PCI device of VGA card or NULL for system default
// @rsrc: bit mask of resources to acquire and lock
//
// Perform the same operation as vga_get(), but return an error (-EBUSY)
// instead of blocking if the resources are already locked by another card.
// Can be called in any context.
//
// On success, release the VGA resource again with vga_put().
//
// Returns:
//
// 0 on success, negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn vga_tryget(pdev: *mut pci_dev, rsrc: c_uint) -> c_int {
    static int vga_tryget(struct pci_dev *pdev, unsigned int rsrc)
    {
    struct vga_device *vgadev;
    unsigned long flags;
    let mut rc: c_int = 0;
    vga_check_first_use();
// The caller should check for this, but let's be sure
    if (pdev == core::ptr::null_mut())
    pdev = vga_default_device();
    if (pdev == core::ptr::null_mut())
    return 0;
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut()) {
    rc = -ENODEV;
    goto bail;
    }
    if (__vga_tryget(vgadev, rsrc))
    rc = -EBUSY;
    bail:
    spin_unlock_irqrestore(&vga_lock, flags);
    return rc;
    }
//
// vga_put - release lock on legacy VGA resources
// @pdev: PCI device of VGA card or NULL for system default
// @rsrc: bit mask of resource to release
//
// Release resources previously locked by vga_get() or vga_tryget().  The
// resources aren't disabled right away, so that a subsequent vga_get() on
// the same card will succeed immediately.  Resources have a counter, so
// locks are only released if the counter reaches 0.
//
#[no_mangle]
pub unsafe extern "C" fn vga_put(pdev: *mut pci_dev, rsrc: c_uint) {
    void vga_put(struct pci_dev *pdev, unsigned int rsrc)
    {
    struct vga_device *vgadev;
    unsigned long flags;
// The caller should check for this, but let's be sure
    if (pdev == core::ptr::null_mut())
    pdev = vga_default_device();
    if (pdev == core::ptr::null_mut())
    return;
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut())
    goto bail;
    __vga_put(vgadev, rsrc);
    bail:
    spin_unlock_irqrestore(&vga_lock, flags);
    }
    EXPORT_SYMBOL(vga_put);
#[no_mangle]
unsafe extern "C" fn vga_is_firmware_default(pdev: *mut pci_dev) -> bool {
    static bool vga_is_firmware_default(struct pci_dev *pdev)
    {

    let mut pdev: return = = screen_info_pci_dev(&sysfb_primary_display.screen);

    return false;

    }
#[no_mangle]
unsafe extern "C" fn vga_arb_integrated_gpu(dev: *mut device) -> bool {
    static bool vga_arb_integrated_gpu(struct device *dev)
    {

    return acpi_dev_is_video_device(ACPI_COMPANION(dev));

    return false;

    }
//
// Return true if vgadev is a better default VGA device than the best one
// we've seen so far.
//
#[no_mangle]
unsafe extern "C" fn vga_is_boot_device(vgadev: *mut vga_device) -> bool {
    static bool vga_is_boot_device(struct vga_device *vgadev)
    {
    struct vga_device *boot_vga = vgadev_find(vga_default_device());
    struct pci_dev *pdev = vgadev.pdev;
    u16 cmd, boot_cmd;
//
// We select the default VGA device in this order:
// Firmware framebuffer (see vga_arb_select_default_device())
// Legacy VGA device (owns VGA_RSRC_LEGACY_MASK)
// Non-legacy integrated device (see vga_arb_select_default_device())
// Non-legacy discrete device (see vga_arb_select_default_device())
// Other device (see vga_arb_select_default_device())
//
// We always prefer a firmware default device, so if we've already
// found one, there's no need to consider vgadev.
//
    if (boot_vga && boot_vga.is_firmware_default)
    return false;
    if (vga_is_firmware_default(pdev)) {
    vgadev.is_firmware_default = true;
    return true;
    }
//
// A legacy VGA device has MEM and IO enabled and any bridges
// leading to it have PCI_BRIDGE_CTL_VGA enabled so the legacy
// resources ([mem 0xa0000-0xbffff], [io 0x3b0-0x3bb], etc) are
// routed to it.
//
// We use the first one we find, so if we've already found one,
// vgadev is no better.
//
    if (boot_vga &&
    (boot_vga.owns & VGA_RSRC_LEGACY_MASK) == VGA_RSRC_LEGACY_MASK)
    return false;
    if ((vgadev.owns & VGA_RSRC_LEGACY_MASK) == VGA_RSRC_LEGACY_MASK)
    return true;
//
// If we haven't found a legacy VGA device, accept a non-legacy
// device.  It may have either IO or MEM enabled, and bridges may
// not have PCI_BRIDGE_CTL_VGA enabled, so it may not be able to
// use legacy VGA resources.  Prefer an integrated GPU over others.
//
    pci_read_config_word(pdev, PCI_COMMAND, &cmd);
    if (cmd & (PCI_COMMAND_IO | PCI_COMMAND_MEMORY)) {
//
// An integrated GPU overrides a previous non-legacy
// device.  We expect only a single integrated GPU, but if
// there are more, we use the *last* because that was the
// previous behavior.
//
    if (vga_arb_integrated_gpu(&pdev.dev))
    return true;
//
// We prefer the first non-legacy discrete device we find.
// If we already found one, vgadev is no better.
//
    if (boot_vga) {
    pci_read_config_word(boot_vga.pdev, PCI_COMMAND,
    &boot_cmd);
    if (boot_cmd & (PCI_COMMAND_IO | PCI_COMMAND_MEMORY))
    return false;
    }
    return true;
    }
    return false;
    }
//
// Rules for using a bridge to control a VGA descendant decoding: if a bridge
// has only one VGA descendant then it can be used to control the VGA routing
// for that device. It should always use the bridge closest to the device to
// control it. If a bridge has a direct VGA descendant, but also have a sub-
// bridge VGA descendant then we cannot use that bridge to control the direct
// VGA descendant. So for every device we register, we need to iterate all
// its parent bridges so we can invalidate any devices using them properly.
//
#[no_mangle]
unsafe extern "C" fn vga_arbiter_check_bridge_sharing(vgadev: *mut vga_device) {
    static void vga_arbiter_check_bridge_sharing(struct vga_device *vgadev)
    {
    struct vga_device *same_bridge_vgadev;
    struct pci_bus *new_bus, *bus;
    struct pci_dev *new_bridge, *bridge;
    vgadev.bridge_has_one_vga = true;
    if (list_empty(&vga_list)) {
    vgaarb_info(&vgadev.pdev.dev, "bridge control possible\n");
    return;
    }
// Iterate the new device's bridge hierarchy
    new_bus = vgadev.pdev.bus;
    while (new_bus) {
    new_bridge = new_bus.self;
// Go through list of devices already registered
    list_for_each_entry(same_bridge_vgadev, &vga_list, list) {
    bus = same_bridge_vgadev.pdev.bus;
    bridge = bus.self;
// See if it shares a bridge with this device
    if (new_bridge == bridge) {
//
// If its direct parent bridge is the same
// as any bridge of this device then it can't
// be used for that device.
//
    same_bridge_vgadev.bridge_has_one_vga = false;
    }
//
// Now iterate the previous device's bridge hierarchy.
// If the new device's parent bridge is in the other
// device's hierarchy, we can't use it to control this
// device.
//
    while (bus) {
    bridge = bus.self;
    if (bridge && bridge == vgadev.pdev.bus.self)
    vgadev.bridge_has_one_vga = false;
    bus = bus.parent;
    }
    }
    new_bus = new_bus.parent;
    }
    if (vgadev.bridge_has_one_vga)
    vgaarb_info(&vgadev.pdev.dev, "bridge control possible\n");
    else
    vgaarb_info(&vgadev.pdev.dev, "no bridge control possible\n");
    }
//
// Currently, we assume that the "initial" setup of the system is not sane,
// that is, we come up with conflicting devices and let the arbiter's
// client decide if devices decodes legacy things or not.
//
#[no_mangle]
unsafe extern "C" fn vga_arbiter_add_pci_device(pdev: *mut pci_dev) -> bool {
    static bool vga_arbiter_add_pci_device(struct pci_dev *pdev)
    {
    struct vga_device *vgadev;
    unsigned long flags;
    struct pci_bus *bus;
    struct pci_dev *bridge;
    u16 cmd;
// Allocate structure
    vgadev = kzalloc_obj(struct vga_device);
    if (vgadev == core::ptr::null_mut()) {
    vgaarb_err(&pdev.dev, "failed to allocate VGA arbiter data\n");
//
// What to do on allocation failure? For now, let's just do
// nothing, I'm not sure there is anything saner to be done.
//
    return false;
    }
// Take lock & check for duplicates
    spin_lock_irqsave(&vga_lock, flags);
    if (vgadev_find(pdev) != core::ptr::null_mut()) {
    BUG_ON(1);
    goto fail;
    }
    vgadev.pdev = pdev;
// By default, assume we decode everything
    vgadev.decodes = VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM |
    VGA_RSRC_NORMAL_IO | VGA_RSRC_NORMAL_MEM;
// By default, mark it as decoding
    vga_decode_count++;
//
// Mark that we "own" resources based on our enables, we will
// clear that below if the bridge isn't forwarding.
//
    pci_read_config_word(pdev, PCI_COMMAND, &cmd);
    if (cmd & PCI_COMMAND_IO)
    vgadev.owns |= VGA_RSRC_LEGACY_IO;
    if (cmd & PCI_COMMAND_MEMORY)
    vgadev.owns |= VGA_RSRC_LEGACY_MEM;
// Check if VGA cycles can get down to us
    bus = pdev.bus;
    while (bus) {
    bridge = bus.self;
    if (bridge) {
    u16 l;
    pci_read_config_word(bridge, PCI_BRIDGE_CONTROL, &l);
    if (!(l & PCI_BRIDGE_CTL_VGA)) {
    vgadev.owns = 0;
    break;
    }
    }
    bus = bus.parent;
    }
    if (vga_is_boot_device(vgadev)) {
    vgaarb_info(&pdev.dev, "setting as boot VGA device%s\n",
    vga_default_device() ?
    " (overriding previous)" : "");
    vga_set_default_device(pdev);
    }
    vga_arbiter_check_bridge_sharing(vgadev);
// Add to the list
    list_add_tail(&vgadev.list, &vga_list);
    vga_count++;
    vgaarb_info(&pdev.dev, "VGA device added: decodes=%s,owns=%s,locks=%s\n",
    vga_iostate_to_str(vgadev.decodes),
    vga_iostate_to_str(vgadev.owns),
    vga_iostate_to_str(vgadev.locks));
    spin_unlock_irqrestore(&vga_lock, flags);
    return true;
    fail:
    spin_unlock_irqrestore(&vga_lock, flags);
    kfree(vgadev);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vga_arbiter_del_pci_device(pdev: *mut pci_dev) -> bool {
    static bool vga_arbiter_del_pci_device(struct pci_dev *pdev)
    {
    struct vga_device *vgadev;
    unsigned long flags;
    let mut ret: bool = true;
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut()) {
    ret = false;
    goto bail;
    }
    if (vga_default == pdev)
    vga_set_default_device(core::ptr::null_mut());
    if (vgadev.decodes & (VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM))
    vga_decode_count--;
// Remove entry from list
    list_del(&vgadev.list);
    vga_count--;
// Wake up all possible waiters
    wake_up_all(&vga_wait_queue);
    bail:
    spin_unlock_irqrestore(&vga_lock, flags);
    kfree(vgadev);
    return ret;
    }
// Called with the lock
    static void vga_update_device_decodes(struct vga_device *vgadev,
    unsigned int new_decodes)
    {
    struct device *dev = &vgadev.pdev.dev;
    let mut old_decodes: c_uint = vgadev.decodes;
    let mut decodes_removed: c_uint = ~new_decodes & old_decodes;
    let mut decodes_unlocked: c_uint = vgadev.locks & decodes_removed;
    vgadev.decodes = new_decodes;
    vgaarb_info(dev, "VGA decodes changed: olddecodes=%s,decodes=%s:owns=%s\n",
    vga_iostate_to_str(old_decodes),
    vga_iostate_to_str(vgadev.decodes),
    vga_iostate_to_str(vgadev.owns));
// If we removed locked decodes, lock count goes to zero, and release
    if (decodes_unlocked) {
    if (decodes_unlocked & VGA_RSRC_LEGACY_IO)
    vgadev.io_lock_cnt = 0;
    if (decodes_unlocked & VGA_RSRC_LEGACY_MEM)
    vgadev.mem_lock_cnt = 0;
    __vga_put(vgadev, decodes_unlocked);
    }
// Change decodes counter
    if (old_decodes & VGA_RSRC_LEGACY_MASK &&
    !(new_decodes & VGA_RSRC_LEGACY_MASK))
    vga_decode_count--;
    if (!(old_decodes & VGA_RSRC_LEGACY_MASK) &&
    new_decodes & VGA_RSRC_LEGACY_MASK)
    vga_decode_count++;
    vgaarb_dbg(dev, "decoding count now is: %d\n", vga_decode_count);
    }
    static void __vga_set_legacy_decoding(struct pci_dev *pdev,
    unsigned int decodes,
    bool userspace)
    {
    struct vga_device *vgadev;
    unsigned long flags;
    decodes &= VGA_RSRC_LEGACY_MASK;
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut())
    goto bail;
// Don't let userspace futz with kernel driver decodes
    if (userspace && vgadev.set_decode)
    goto bail;
// Update the device decodes + counter
    vga_update_device_decodes(vgadev, decodes);
//
// XXX If somebody is going from "doesn't decode" to "decodes"
// state here, additional care must be taken as we may have pending
// ownership of non-legacy region.
//
    bail:
    spin_unlock_irqrestore(&vga_lock, flags);
    }
//
// vga_set_legacy_decoding
// @pdev: PCI device of the VGA card
// @decodes: bit mask of what legacy regions the card decodes
//
// Indicate to the arbiter if the card decodes legacy VGA IOs, legacy VGA
// Memory, both, or none. All cards default to both, the card driver (fbdev for
// example) should tell the arbiter if it has disabled legacy decoding, so the
// card can be left out of the arbitration process (and can be safe to take
// interrupts at any time.
//
#[no_mangle]
pub unsafe extern "C" fn vga_set_legacy_decoding(pdev: *mut pci_dev, decodes: c_uint) {
    void vga_set_legacy_decoding(struct pci_dev *pdev, unsigned int decodes)
    {
    __vga_set_legacy_decoding(pdev, decodes, false);
    }
    EXPORT_SYMBOL(vga_set_legacy_decoding);
//
// vga_client_register - register or unregister a VGA arbitration client
// @pdev: PCI device of the VGA client
// @set_decode: VGA decode change callback
//
// Clients have two callback mechanisms they can use.
//
// @set_decode callback: If a client can disable its GPU VGA resource, it
// will get a callback from this to set the encode/decode state.
//
// Rationale: we cannot disable VGA decode resources unconditionally
// because some single GPU laptops seem to require ACPI or BIOS access to
// the VGA registers to control things like backlights etc. Hopefully newer
// multi-GPU laptops do something saner, and desktops won't have any
// special ACPI for this. The driver will get a callback when VGA
// arbitration is first used by userspace since some older X servers have
// issues.
//
// Does not check whether a client for @pdev has been registered already.
//
// To unregister, call vga_client_unregister().
//
// Returns: 0 on success, -ENODEV on failure
//
    int vga_client_register(struct pci_dev *pdev,
#[no_mangle]
pub unsafe extern "C" fn int(pdev: *mut *mut set_decode)(struct pci_dev, decode): bool) -> unsigned {
    unsigned int (*set_decode)(struct pci_dev *pdev, bool decode))
    {
    unsigned long flags;
    struct vga_device *vgadev;
    spin_lock_irqsave(&vga_lock, flags);
    vgadev = vgadev_find(pdev);
    if (vgadev)
    vgadev.set_decode = set_decode;
    spin_unlock_irqrestore(&vga_lock, flags);
    if (!vgadev)
    return -ENODEV;
    return 0;
    }
    EXPORT_SYMBOL(vga_client_register);
//
// Char driver implementation
//
// Semantics is:
//
// open       : Open user instance of the arbiter. By default, it's
// attached to the default VGA device of the system.
//
// close      : Close user instance, release locks
//
// read       : Return a string indicating the status of the target.
// An IO state string is of the form {io,mem,io+mem,none},
// mc and ic are respectively mem and io lock counts (for
// debugging/diagnostic only). "decodes" indicate what the
// card currently decodes, "owns" indicates what is currently
// enabled on it, and "locks" indicates what is locked by this
// card. If the card is unplugged, we get "invalid" then for
// card_ID and an -ENODEV error is returned for any command
// until a new card is targeted
//
// "<card_ID>,decodes=<io_state>,owns=<io_state>,locks=<io_state> (ic,mc)"
//
// write       : write a command to the arbiter. List of commands is:
//
// target <card_ID>   : switch target to card <card_ID> (see below)
// lock <io_state>    : acquire locks on target ("none" is invalid io_state)
// trylock <io_state> : non-blocking acquire locks on target
// unlock <io_state>  : release locks on target
// unlock all         : release all locks on target held by this user
// decodes <io_state> : set the legacy decoding attributes for the card
//
// poll         : event if something change on any card (not just the target)
//
// card_ID is of the form "PCI:domain:bus:dev.fn". It can be set to "default"
// to go back to the system default card (TODO: not implemented yet).
// Currently, only PCI is supported as a prefix, but the userland API may
// support other bus types in the future, even if the current kernel
// implementation doesn't.
//
// Note about locks:
//
// The driver keeps track of which user has what locks on which card. It
// supports stacking, like the kernel one. This complicates the implementation
// a bit, but makes the arbiter more tolerant to userspace problems and able
// to properly cleanup in all cases when a process dies.
// Currently, a max of 16 cards simultaneously can have locks issued from
// userspace for a given user (file descriptor instance) of the arbiter.
//
// If the device is hot-unplugged, there is a hook inside the module to notify
// it being added/removed in the system and automatically added/removed in
// the arbiter.
//

// Each user has an array of these, tracking which cards have locks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_arb_user_card {
    pub pdev: *mut pci_dev,
    pub mem_cnt: c_uint,
    pub io_cnt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_arb_private {
    pub list: list_head,
    pub target: *mut pci_dev,
    pub cards: [vga_arb_user_card; MAX_USER_CARDS],
    pub lock: spinlock_t,
}

    static LIST_HEAD(vga_user_list);
    static DEFINE_SPINLOCK(vga_user_lock);
//
// Take a string in the format: "PCI:domain:bus:dev.fn" and return the
// respective values. If the string is not in this format, return 0.
//
    static int vga_pci_str_to_vars(char *buf, int count, unsigned int *domain,
    unsigned int *bus, unsigned int *devfn)
    {
    int n;
    unsigned int slot, func;
    n = sscanf(buf, "PCI:%x:%x:%x.%x", domain, bus, &slot, &func);
    if (n != 4)
    return 0;
// devfn = PCI_DEVFN(slot, func);
    return 1;
    }
    static ssize_t vga_arb_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct vga_arb_private *priv = file.private_data;
    struct vga_device *vgadev;
    struct pci_dev *pdev;
    unsigned long flags;
    size_t len;
    int rc;
    char *lbuf;
    lbuf = kmalloc(1024, GFP_KERNEL);
    if (lbuf == core::ptr::null_mut())
    return -ENOMEM;
// Protect vga_list
    spin_lock_irqsave(&vga_lock, flags);
// If we are targeting the default, use it
    pdev = priv.target;
    if (pdev == core::ptr::null_mut() || pdev == PCI_INVALID_CARD) {
    spin_unlock_irqrestore(&vga_lock, flags);
    len = sprintf(lbuf, "invalid");
    goto done;
    }
// Find card vgadev structure
    vgadev = vgadev_find(pdev);
    if (vgadev == core::ptr::null_mut()) {
//
// Wow, it's not in the list, that shouldn't happen, let's
// fix us up and return invalid card.
//
    spin_unlock_irqrestore(&vga_lock, flags);
    len = sprintf(lbuf, "invalid");
    goto done;
    }
// Fill the buffer with info
    len = snprintf(lbuf, 1024,
    "count:%d,PCI:%s,decodes=%s,owns=%s,locks=%s(%u:%u)\n",
    vga_decode_count, pci_name(pdev),
    vga_iostate_to_str(vgadev.decodes),
    vga_iostate_to_str(vgadev.owns),
    vga_iostate_to_str(vgadev.locks),
    vgadev.io_lock_cnt, vgadev.mem_lock_cnt);
    spin_unlock_irqrestore(&vga_lock, flags);
    done:
// Copy that to user
    if (len > count)
    len = count;
    rc = copy_to_user(buf, lbuf, len);
    kfree(lbuf);
    if (rc)
    return -EFAULT;
    return len;
    }
//
// TODO: To avoid parsing inside kernel and to improve the speed we may
// consider use ioctl here
//
    static ssize_t vga_arb_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct vga_arb_private *priv = file.private_data;
    struct vga_arb_user_card *uc = core::ptr::null_mut();
    struct pci_dev *pdev;
    unsigned int io_state;
    char kbuf[64], *curr_pos;
    let mut remaining: usize = count;
    int err;
    int ret_val;
    int i;
    if (count >= sizeof(kbuf))
    return -EINVAL;
    if (copy_from_user(kbuf, buf, count))
    return -EFAULT;
    curr_pos = kbuf;
    kbuf[count] = '\0';
    if (strncmp(curr_pos, "lock ", 5) == 0) {
    curr_pos += 5;
    remaining -= 5;
    pr_debug("client 0x%p called 'lock'\n", priv);
    if (!vga_str_to_iostate(curr_pos, remaining, &io_state)) {
    ret_val = -EPROTO;
    goto done;
    }
    if (io_state == VGA_RSRC_NONE) {
    ret_val = -EPROTO;
    goto done;
    }
    pdev = priv.target;
    if (priv.target == core::ptr::null_mut()) {
    ret_val = -ENODEV;
    goto done;
    }
    err = vga_get_uninterruptible(pdev, io_state);
    if (err) {
    ret_val = err;
    goto done;
    }
// Update the client's locks lists
    for (i = 0; i < MAX_USER_CARDS; i++) {
    if (priv.cards[i].pdev == pdev) {
    if (io_state & VGA_RSRC_LEGACY_IO)
    priv.cards[i].io_cnt++;
    if (io_state & VGA_RSRC_LEGACY_MEM)
    priv.cards[i].mem_cnt++;
    break;
    }
    }
    ret_val = count;
    goto done;
    } else if (strncmp(curr_pos, "unlock ", 7) == 0) {
    curr_pos += 7;
    remaining -= 7;
    pr_debug("client 0x%p called 'unlock'\n", priv);
    if (strncmp(curr_pos, "all", 3) == 0)
    io_state = VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM;
    else {
    if (!vga_str_to_iostate
    (curr_pos, remaining, &io_state)) {
    ret_val = -EPROTO;
    goto done;
    }
// TODO: Add this?
    if (io_state == VGA_RSRC_NONE) {
    ret_val = -EPROTO;
    goto done;
    }
//
    }
    pdev = priv.target;
    if (priv.target == core::ptr::null_mut()) {
    ret_val = -ENODEV;
    goto done;
    }
    for (i = 0; i < MAX_USER_CARDS; i++) {
    if (priv.cards[i].pdev == pdev)
    uc = &priv.cards[i];
    }
    if (!uc) {
    ret_val = -EINVAL;
    goto done;
    }
    if (io_state & VGA_RSRC_LEGACY_IO && uc.io_cnt == 0) {
    ret_val = -EINVAL;
    goto done;
    }
    if (io_state & VGA_RSRC_LEGACY_MEM && uc.mem_cnt == 0) {
    ret_val = -EINVAL;
    goto done;
    }
    vga_put(pdev, io_state);
    if (io_state & VGA_RSRC_LEGACY_IO)
    uc.io_cnt--;
    if (io_state & VGA_RSRC_LEGACY_MEM)
    uc.mem_cnt--;
    ret_val = count;
    goto done;
    } else if (strncmp(curr_pos, "trylock ", 8) == 0) {
    curr_pos += 8;
    remaining -= 8;
    pr_debug("client 0x%p called 'trylock'\n", priv);
    if (!vga_str_to_iostate(curr_pos, remaining, &io_state)) {
    ret_val = -EPROTO;
    goto done;
    }
// TODO: Add this?
    if (io_state == VGA_RSRC_NONE) {
    ret_val = -EPROTO;
    goto done;
    }
//
    pdev = priv.target;
    if (priv.target == core::ptr::null_mut()) {
    ret_val = -ENODEV;
    goto done;
    }
    if (vga_tryget(pdev, io_state)) {
// Update the client's locks lists...
    for (i = 0; i < MAX_USER_CARDS; i++) {
    if (priv.cards[i].pdev == pdev) {
    if (io_state & VGA_RSRC_LEGACY_IO)
    priv.cards[i].io_cnt++;
    if (io_state & VGA_RSRC_LEGACY_MEM)
    priv.cards[i].mem_cnt++;
    break;
    }
    }
    ret_val = count;
    goto done;
    } else {
    ret_val = -EBUSY;
    goto done;
    }
    } else if (strncmp(curr_pos, "target ", 7) == 0) {
    unsigned int domain, bus, devfn;
    struct vga_device *vgadev;
    curr_pos += 7;
    remaining -= 7;
    pr_debug("client 0x%p called 'target'\n", priv);
// If target is default
    if (!strncmp(curr_pos, "default", 7))
    pdev = pci_dev_get(vga_default_device());
    else {
    if (!vga_pci_str_to_vars(curr_pos, remaining,
    &domain, &bus, &devfn)) {
    ret_val = -EPROTO;
    goto done;
    }
    pdev = pci_get_domain_bus_and_slot(domain, bus, devfn);
    if (!pdev) {
    pr_debug("invalid PCI address %04x:%02x:%02x.%x\n",
    domain, bus, PCI_SLOT(devfn),
    PCI_FUNC(devfn));
    ret_val = -ENODEV;
    goto done;
    }
    pr_debug("%s ==> %04x:%02x:%02x.%x pdev %p\n", curr_pos,
    domain, bus, PCI_SLOT(devfn), PCI_FUNC(devfn),
    pdev);
    }
    vgadev = vgadev_find(pdev);
    pr_debug("vgadev %p\n", vgadev);
    if (vgadev == core::ptr::null_mut()) {
    if (pdev) {
    vgaarb_dbg(&pdev.dev, "not a VGA device\n");
    pci_dev_put(pdev);
    }
    ret_val = -ENODEV;
    goto done;
    }
    priv.target = pdev;
    for (i = 0; i < MAX_USER_CARDS; i++) {
    if (priv.cards[i].pdev == pdev)
    break;
    if (priv.cards[i].pdev == core::ptr::null_mut()) {
    priv.cards[i].pdev = pdev;
    priv.cards[i].io_cnt = 0;
    priv.cards[i].mem_cnt = 0;
    break;
    }
    }
    if (i == MAX_USER_CARDS) {
    vgaarb_dbg(&pdev.dev, "maximum user cards (%d) number reached, ignoring this one!\n",
    MAX_USER_CARDS);
    pci_dev_put(pdev);
// XXX: Which value to return?
    ret_val =  -ENOMEM;
    goto done;
    }
    ret_val = count;
    pci_dev_put(pdev);
    goto done;
    } else if (strncmp(curr_pos, "decodes ", 8) == 0) {
    curr_pos += 8;
    remaining -= 8;
    pr_debug("client 0x%p called 'decodes'\n", priv);
    if (!vga_str_to_iostate(curr_pos, remaining, &io_state)) {
    ret_val = -EPROTO;
    goto done;
    }
    pdev = priv.target;
    if (priv.target == core::ptr::null_mut()) {
    ret_val = -ENODEV;
    goto done;
    }
    __vga_set_legacy_decoding(pdev, io_state, true);
    ret_val = count;
    goto done;
    }
// If we got here, the message written is not part of the protocol!
    return -EPROTO;
    done:
    return ret_val;
    }
#[no_mangle]
unsafe extern "C" fn vga_arb_fpoll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t vga_arb_fpoll(struct file *file, poll_table *wait)
    {
    pr_debug("%s\n", __func__);
    poll_wait(file, &vga_wait_queue, wait);
    return EPOLLIN;
    }
#[no_mangle]
unsafe extern "C" fn vga_arb_open(inode: *mut inode, file: *mut file) -> c_int {
    static int vga_arb_open(struct inode *inode, struct file *file)
    {
    struct vga_arb_private *priv;
    unsigned long flags;
    pr_debug("%s\n", __func__);
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    file.private_data = priv;
    spin_lock_irqsave(&vga_user_lock, flags);
    list_add(&priv.list, &vga_user_list);
    spin_unlock_irqrestore(&vga_user_lock, flags);
// Set the client's lists of locks
    priv.target = vga_default_device(); /* Maybe this is still null! */
    priv.cards[0].pdev = priv.target;
    priv.cards[0].io_cnt = 0;
    priv.cards[0].mem_cnt = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vga_arb_release(inode: *mut inode, file: *mut file) -> c_int {
    static int vga_arb_release(struct inode *inode, struct file *file)
    {
    struct vga_arb_private *priv = file.private_data;
    struct vga_arb_user_card *uc;
    unsigned long flags;
    int i;
    pr_debug("%s\n", __func__);
    spin_lock_irqsave(&vga_user_lock, flags);
    list_del(&priv.list);
    for (i = 0; i < MAX_USER_CARDS; i++) {
    uc = &priv.cards[i];
    if (uc.pdev == core::ptr::null_mut())
    continue;
    vgaarb_dbg(&uc.pdev.dev, "uc.io_cnt == %d, uc.mem_cnt == %d\n",
    uc.io_cnt, uc.mem_cnt);
    while (uc.io_cnt--)
    vga_put(uc.pdev, VGA_RSRC_LEGACY_IO);
    while (uc.mem_cnt--)
    vga_put(uc.pdev, VGA_RSRC_LEGACY_MEM);
    }
    spin_unlock_irqrestore(&vga_user_lock, flags);
    kfree(priv);
    return 0;
    }
//
// Callback any registered clients to let them know we have a change in VGA
// cards.
//
#[no_mangle]
unsafe extern "C" fn vga_arbiter_notify_clients() {
    static void vga_arbiter_notify_clients(void)
    {
    struct vga_device *vgadev;
    unsigned long flags;
    unsigned int new_decodes;
    bool new_state;
    if (!vga_arbiter_used)
    return;
    new_state = (vga_count > 1) ? false : true;
    spin_lock_irqsave(&vga_lock, flags);
    list_for_each_entry(vgadev, &vga_list, list) {
    if (vgadev.set_decode) {
    new_decodes = vgadev.set_decode(vgadev.pdev,
    new_state);
    vga_update_device_decodes(vgadev, new_decodes);
    }
    }
    spin_unlock_irqrestore(&vga_lock, flags);
    }
    static int pci_notify(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct device *dev = data;
    struct pci_dev *pdev = to_pci_dev(dev);
    let mut notify: bool = false;
    vgaarb_dbg(dev, "%s\n", __func__);
// Only deal with VGA class devices
    if (!pci_is_vga(pdev))
    return 0;
//
// For now, we're only interested in devices added and removed.
// I didn't test this thing here, so someone needs to double check
// for the cases of hot-pluggable VGA cards.
//
    if (action == BUS_NOTIFY_ADD_DEVICE)
    notify = vga_arbiter_add_pci_device(pdev);
#[no_mangle]
pub unsafe extern "C" fn if(BUS_NOTIFY_DEL_DEVICE: action ==) -> else {
    else if (action == BUS_NOTIFY_DEL_DEVICE)
    notify = vga_arbiter_del_pci_device(pdev);
    if (notify)
    vga_arbiter_notify_clients();
    return 0;
    }
    static struct notifier_block pci_notifier = {
    .notifier_call = pci_notify,
    };
    static const struct file_operations vga_arb_device_fops = {
    .read = vga_arb_read,
    .write = vga_arb_write,
    .poll = vga_arb_fpoll,
    .open = vga_arb_open,
    .release = vga_arb_release,
    .llseek = noop_llseek,
    };
    static struct miscdevice vga_arb_device = {
    MISC_DYNAMIC_MINOR, "vga_arbiter", &vga_arb_device_fops
    };
#[no_mangle]
unsafe extern "C" fn vga_arb_device_init() -> int __init {
    static int __init vga_arb_device_init(void)
    {
    int rc;
    struct pci_dev *pdev;
    rc = misc_register(&vga_arb_device);
    if (rc < 0)
    pr_err("error %d registering device\n", rc);
    bus_register_notifier(&pci_bus_type, &pci_notifier);
// Add all VGA class PCI devices by default
    pdev = core::ptr::null_mut();
    while ((pdev =
    pci_get_subsys(PCI_ANY_ID, PCI_ANY_ID, PCI_ANY_ID,
    PCI_ANY_ID, pdev)) != core::ptr::null_mut()) {
    if (pci_is_vga(pdev))
    vga_arbiter_add_pci_device(pdev);
    }
    pr_info("loaded\n");
    return rc;
    }
    subsys_initcall_sync(vga_arb_device_init);
