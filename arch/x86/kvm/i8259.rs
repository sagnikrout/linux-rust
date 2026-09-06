//! Automatically rewritten from C to Rust
//! Source: arch/x86/kvm/i8259.c
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
// 8259 interrupt controller emulation
//
// Copyright (c) 2003-2004 Fabrice Bellard
// Copyright (c) 2007 Intel Corporation
// Copyright 2009 Red Hat, Inc. and/or its affiliates.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
// Authors:
// Yaozu (Eddie) Dong <Eddie.dong@intel.com>
// Port from Qemu.
//

    pr_err_ratelimited("pic: " fmt, ## __VA_ARGS__)
    static void pic_irq_request(struct kvm *kvm, int level);
#[no_mangle]
unsafe extern "C" fn pic_lock(s: *mut kvm_pic) {
    static void pic_lock(struct kvm_pic *s)
    __acquires(&s.lock)
    {
    spin_lock(&s.lock);
    }
#[no_mangle]
unsafe extern "C" fn pic_unlock(s: *mut kvm_pic) {
    static void pic_unlock(struct kvm_pic *s)
    __releases(&s.lock)
    {
    let mut wakeup: bool = s.wakeup_needed;
    struct kvm_vcpu *vcpu;
    unsigned long i;
    s.wakeup_needed = false;
    spin_unlock(&s.lock);
    if (wakeup) {
    kvm_for_each_vcpu(i, vcpu, s.kvm) {
    if (kvm_apic_accept_pic_intr(vcpu)) {
    kvm_make_request(KVM_REQ_EVENT, vcpu);
    kvm_vcpu_kick(vcpu);
    return;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pic_clear_isr(s: *mut kvm_kpic_state, irq: c_int) {
    static void pic_clear_isr(struct kvm_kpic_state *s, int irq)
    {
    s.isr &= ~(1 << irq);
    if (s != &s.pics_state.pics[0])
    irq += 8;
//
// We are dropping lock while calling ack notifiers since ack
// notifier callbacks for assigned devices call into PIC recursively.
// Other interrupt may be delivered to PIC while lock is dropped but
// it should be safe since PIC state is already updated at this stage.
//
    pic_unlock(s.pics_state);
    kvm_notify_acked_irq(s.pics_state.kvm, SELECT_PIC(irq), irq);
    pic_lock(s.pics_state);
    }
//
// set irq level. If an edge is detected, then the IRR is set to 1
//
#[no_mangle]
pub unsafe extern "C" fn pic_set_irq1(s: *mut kvm_kpic_state, irq: c_int, level: c_int) -> c_int {
    static inline int pic_set_irq1(struct kvm_kpic_state *s, int irq, int level)
    {
    int mask, ret = 1;
    mask = 1 << irq;
    if (s.elcr & mask)	/* level triggered */
    if (level) {
    ret = !(s.irr & mask);
    s.irr |= mask;
    s.last_irr |= mask;
    } else {
    s.irr &= ~mask;
    s.last_irr &= ~mask;
    }
    else	/* edge triggered */
    if (level) {
    if ((s.last_irr & mask) == 0) {
    ret = !(s.irr & mask);
    s.irr |= mask;
    }
    s.last_irr |= mask;
    } else
    s.last_irr &= ~mask;
    return (s.imr & mask) ? -1 : ret;
    }
//
// return the highest priority found in mask (highest = smallest
// number). Return 8 if no irq
//
#[no_mangle]
pub unsafe extern "C" fn get_priority(s: *mut kvm_kpic_state, mask: c_int) -> c_int {
    static inline int get_priority(struct kvm_kpic_state *s, int mask)
    {
    int priority;
    if (mask == 0)
    return 8;
    priority = 0;
    while ((mask & (1 << ((priority + s.priority_add) & 7))) == 0)
    priority++;
    return priority;
    }
//
// return the pic wanted interrupt. return -1 if none
//
#[no_mangle]
unsafe extern "C" fn pic_get_irq(s: *mut kvm_kpic_state) -> c_int {
    static int pic_get_irq(struct kvm_kpic_state *s)
    {
    int mask, cur_priority, priority;
    mask = s.irr & ~s.imr;
    priority = get_priority(s, mask);
    if (priority == 8)
    return -1;
//
// compute current priority. If special fully nested mode on the
// master, the IRQ coming from the slave is not taken into account
// for the priority computation.
//
    mask = s.isr;
    if (s.special_fully_nested_mode && s == &s.pics_state.pics[0])
    mask &= ~(1 << 2);
    cur_priority = get_priority(s, mask);
    if (priority < cur_priority)
//
// higher priority found: an irq should be generated
//
    return (priority + s.priority_add) & 7;
    else
    return -1;
    }
//
// raise irq to CPU if necessary. must be called every time the active
// irq may change
//
#[no_mangle]
unsafe extern "C" fn pic_update_irq(s: *mut kvm_pic) {
    static void pic_update_irq(struct kvm_pic *s)
    {
    int irq2, irq;
    irq2 = pic_get_irq(&s.pics[1]);
    if (irq2 >= 0) {
//
// if irq request by slave pic, signal master PIC
//
    pic_set_irq1(&s.pics[0], 2, 1);
    pic_set_irq1(&s.pics[0], 2, 0);
    }
    irq = pic_get_irq(&s.pics[0]);
    pic_irq_request(s.kvm, irq >= 0);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_pic_update_irq(s: *mut kvm_pic) {
    void kvm_pic_update_irq(struct kvm_pic *s)
    {
    pic_lock(s);
    pic_update_irq(s);
    pic_unlock(s);
    }
    int kvm_pic_set_irq(struct kvm_kernel_irq_routing_entry *e, struct kvm *kvm,
    int irq_source_id, int level, bool line_status)
    {
    struct kvm_pic *s = kvm.arch.vpic;
    let mut irq: c_int = e.irqchip.pin;
    int ret, irq_level;
    if (WARN_ON_ONCE(irq < 0 || irq >= PIC_NUM_PINS))
    return -1;
    pic_lock(s);
    irq_level = __kvm_irq_line_state(&s.irq_states[irq],
    irq_source_id, level);
    ret = pic_set_irq1(&s.pics[irq >> 3], irq & 7, irq_level);
    pic_update_irq(s);
    trace_kvm_pic_set_irq(irq >> 3, irq & 7, s.pics[irq >> 3].elcr,
    s.pics[irq >> 3].imr, ret == 0);
    pic_unlock(s);
    return ret;
    }
//
// acknowledge interrupt 'irq'
//
#[no_mangle]
pub unsafe extern "C" fn pic_intack(s: *mut kvm_kpic_state, irq: c_int) {
    static inline void pic_intack(struct kvm_kpic_state *s, int irq)
    {
    s.isr |= 1 << irq;
//
// We don't clear a level sensitive interrupt here
//
    if (!(s.elcr & (1 << irq)))
    s.irr &= ~(1 << irq);
    if (s.auto_eoi) {
    if (s.rotate_on_auto_eoi)
    s.priority_add = (irq + 1) & 7;
    pic_clear_isr(s, irq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_pic_read_irq(kvm: *mut kvm) -> c_int {
    int kvm_pic_read_irq(struct kvm *kvm)
    {
    int irq, irq2, intno;
    struct kvm_pic *s = kvm.arch.vpic;
    s.output = 0;
    pic_lock(s);
    irq = pic_get_irq(&s.pics[0]);
    if (irq >= 0) {
    pic_intack(&s.pics[0], irq);
    if (irq == 2) {
    irq2 = pic_get_irq(&s.pics[1]);
    if (irq2 >= 0)
    pic_intack(&s.pics[1], irq2);
    else
//
// spurious IRQ on slave controller
//
    irq2 = 7;
    intno = s.pics[1].irq_base + irq2;
    } else
    intno = s.pics[0].irq_base + irq;
    } else {
//
// spurious IRQ on host controller
//
    irq = 7;
    intno = s.pics[0].irq_base + irq;
    }
    pic_update_irq(s);
    pic_unlock(s);
    return intno;
    }
#[no_mangle]
unsafe extern "C" fn kvm_pic_reset(s: *mut kvm_kpic_state) {
    static void kvm_pic_reset(struct kvm_kpic_state *s)
    {
    int irq;
    unsigned long i;
    struct kvm_vcpu *vcpu;
    let mut edge_irr: u8 = s.irr & ~s.elcr;
    let mut found: bool = false;
    s.last_irr = 0;
    s.irr &= s.elcr;
    s.imr = 0;
    s.priority_add = 0;
    s.special_mask = 0;
    s.read_reg_select = 0;
    if (!s.init4) {
    s.special_fully_nested_mode = 0;
    s.auto_eoi = 0;
    }
    s.init_state = 1;
    kvm_for_each_vcpu(i, vcpu, s.pics_state.kvm)
    if (kvm_apic_accept_pic_intr(vcpu)) {
    found = true;
    break;
    }
    if (!found)
    return;
    for (irq = 0; irq < PIC_NUM_PINS/2; irq++)
    if (edge_irr & (1 << irq))
    pic_clear_isr(s, irq);
    }
#[no_mangle]
unsafe extern "C" fn pic_ioport_write(opaque: *mut c_void, addr: u32, val: u32) {
    static void pic_ioport_write(void *opaque, u32 addr, u32 val)
    {
    struct kvm_kpic_state *s = opaque;
    int priority, cmd, irq;
    addr &= 1;
    if (addr == 0) {
    if (val & 0x10) {
    s.init4 = val & 1;
    if (val & 0x02)
    pr_pic_unimpl("single mode not supported");
    if (val & 0x08)
    pr_pic_unimpl(
    "level sensitive irq not supported");
    kvm_pic_reset(s);
    } else if (val & 0x08) {
    if (val & 0x04)
    s.poll = 1;
    if (val & 0x02)
    s.read_reg_select = val & 1;
    if (val & 0x40)
    s.special_mask = (val >> 5) & 1;
    } else {
    cmd = val >> 5;
    switch (cmd) {
    case 0:
    case 4:
    s.rotate_on_auto_eoi = cmd >> 2;
    break;
    case 1:	/* end of interrupt */
    case 5:
    priority = get_priority(s, s.isr);
    if (priority != 8) {
    irq = (priority + s.priority_add) & 7;
    if (cmd == 5)
    s.priority_add = (irq + 1) & 7;
    pic_clear_isr(s, irq);
    pic_update_irq(s.pics_state);
    }
    break;
    case 3:
    irq = val & 7;
    pic_clear_isr(s, irq);
    pic_update_irq(s.pics_state);
    break;
    case 6:
    s.priority_add = (val + 1) & 7;
    pic_update_irq(s.pics_state);
    break;
    case 7:
    irq = val & 7;
    s.priority_add = (irq + 1) & 7;
    pic_clear_isr(s, irq);
    pic_update_irq(s.pics_state);
    break;
    default:
    break;	/* no operation */
    }
    }
    } else
    switch (s.init_state) {
    case 0: { /* normal mode */
    u8 imr_diff = s.imr ^ val,
    off = (s == &s.pics_state.pics[0]) ? 0 : 8;
    s.imr = val;
    for (irq = 0; irq < PIC_NUM_PINS/2; irq++)
    if (imr_diff & (1 << irq))
    kvm_fire_mask_notifiers(
    s.pics_state.kvm,
    SELECT_PIC(irq + off),
    irq + off,
    !!(s.imr & (1 << irq)));
    pic_update_irq(s.pics_state);
    break;
    }
    case 1:
    s.irq_base = val & 0xf8;
    s.init_state = 2;
    break;
    case 2:
    if (s.init4)
    s.init_state = 3;
    else
    s.init_state = 0;
    break;
    case 3:
    s.special_fully_nested_mode = (val >> 4) & 1;
    s.auto_eoi = (val >> 1) & 1;
    s.init_state = 0;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn pic_poll_read(s: *mut kvm_kpic_state, addr1: u32) -> u32 {
    static u32 pic_poll_read(struct kvm_kpic_state *s, u32 addr1)
    {
    int ret;
    ret = pic_get_irq(s);
    if (ret >= 0) {
    if (addr1 >> 7) {
    s.pics_state.pics[0].isr &= ~(1 << 2);
    s.pics_state.pics[0].irr &= ~(1 << 2);
    }
    s.irr &= ~(1 << ret);
    pic_clear_isr(s, ret);
    if (addr1 >> 7 || ret != 2)
    pic_update_irq(s.pics_state);
// Bit 7 is 1, means there's an interrupt
    ret |= 0x80;
    } else {
// Bit 7 is 0, means there's no interrupt
    ret = 0x07;
    pic_update_irq(s.pics_state);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic_ioport_read(opaque: *mut c_void, addr: u32) -> u32 {
    static u32 pic_ioport_read(void *opaque, u32 addr)
    {
    struct kvm_kpic_state *s = opaque;
    int ret;
    if (s.poll) {
    ret = pic_poll_read(s, addr);
    s.poll = 0;
    } else
    if ((addr & 1) == 0)
    if (s.read_reg_select)
    ret = s.isr;
    else
    ret = s.irr;
    else
    ret = s.imr;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn elcr_ioport_write(opaque: *mut c_void, val: u32) {
    static void elcr_ioport_write(void *opaque, u32 val)
    {
    struct kvm_kpic_state *s = opaque;
    s.elcr = val & s.elcr_mask;
    }
#[no_mangle]
unsafe extern "C" fn elcr_ioport_read(opaque: *mut c_void) -> u32 {
    static u32 elcr_ioport_read(void *opaque)
    {
    struct kvm_kpic_state *s = opaque;
    return s.elcr;
    }
    static int picdev_write(struct kvm_pic *s,
    gpa_t addr, int len, const void *val)
    {
    let mut data: c_uchar = *(unsigned char *)val;
    if (len != 1) {
    pr_pic_unimpl("non byte write\n");
    return 0;
    }
    switch (addr) {
    case 0x20:
    case 0x21:
    pic_lock(s);
    pic_ioport_write(&s.pics[0], addr, data);
    pic_unlock(s);
    break;
    case 0xa0:
    case 0xa1:
    pic_lock(s);
    pic_ioport_write(&s.pics[1], addr, data);
    pic_unlock(s);
    break;
    case 0x4d0:
    case 0x4d1:
    pic_lock(s);
    elcr_ioport_write(&s.pics[addr & 1], data);
    pic_unlock(s);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int picdev_read(struct kvm_pic *s,
    gpa_t addr, int len, void *val)
    {
    unsigned char *data = (unsigned char *)val;
    if (len != 1) {
    memset(val, 0, len);
    pr_pic_unimpl("non byte read\n");
    return 0;
    }
    switch (addr) {
    case 0x20:
    case 0x21:
    case 0xa0:
    case 0xa1:
    pic_lock(s);
// data = pic_ioport_read(&s->pics[addr >> 7], addr);
    pic_unlock(s);
    break;
    case 0x4d0:
    case 0x4d1:
    pic_lock(s);
// data = elcr_ioport_read(&s->pics[addr & 1]);
    pic_unlock(s);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int picdev_master_write(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, const void *val)
    {
    return picdev_write(container_of(dev, struct kvm_pic, dev_master),
    addr, len, val);
    }
    static int picdev_master_read(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, void *val)
    {
    return picdev_read(container_of(dev, struct kvm_pic, dev_master),
    addr, len, val);
    }
    static int picdev_slave_write(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, const void *val)
    {
    return picdev_write(container_of(dev, struct kvm_pic, dev_slave),
    addr, len, val);
    }
    static int picdev_slave_read(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, void *val)
    {
    return picdev_read(container_of(dev, struct kvm_pic, dev_slave),
    addr, len, val);
    }
    static int picdev_elcr_write(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, const void *val)
    {
    return picdev_write(container_of(dev, struct kvm_pic, dev_elcr),
    addr, len, val);
    }
    static int picdev_elcr_read(struct kvm_vcpu *vcpu, struct kvm_io_device *dev,
    gpa_t addr, int len, void *val)
    {
    return picdev_read(container_of(dev, struct kvm_pic, dev_elcr),
    addr, len, val);
    }
//
// callback when PIC0 irq status changed
//
#[no_mangle]
unsafe extern "C" fn pic_irq_request(kvm: *mut kvm, level: c_int) {
    static void pic_irq_request(struct kvm *kvm, int level)
    {
    struct kvm_pic *s = kvm.arch.vpic;
    if (!s.output && level)
    s.wakeup_needed = true;
    s.output = level;
    }
    static const struct kvm_io_device_ops picdev_master_ops = {
    .read     = picdev_master_read,
    .write    = picdev_master_write,
    };
    static const struct kvm_io_device_ops picdev_slave_ops = {
    .read     = picdev_slave_read,
    .write    = picdev_slave_write,
    };
    static const struct kvm_io_device_ops picdev_elcr_ops = {
    .read     = picdev_elcr_read,
    .write    = picdev_elcr_write,
    };
#[no_mangle]
pub unsafe extern "C" fn kvm_pic_init(kvm: *mut kvm) -> c_int {
    int kvm_pic_init(struct kvm *kvm)
    {
    struct kvm_pic *s;
    int ret;
    s = kzalloc_obj(struct kvm_pic, GFP_KERNEL_ACCOUNT);
    if (!s)
    return -ENOMEM;
    spin_lock_init(&s.lock);
    s.kvm = kvm;
    s.pics[0].elcr_mask = 0xf8;
    s.pics[1].elcr_mask = 0xde;
    s.pics[0].pics_state = s;
    s.pics[1].pics_state = s;
//
// Initialize PIO device
//
    kvm_iodevice_init(&s.dev_master, &picdev_master_ops);
    kvm_iodevice_init(&s.dev_slave, &picdev_slave_ops);
    kvm_iodevice_init(&s.dev_elcr, &picdev_elcr_ops);
    mutex_lock(&kvm.slots_lock);
    ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0x20, 2,
    &s.dev_master);
    if (ret < 0)
    goto fail_unlock;
    ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0xa0, 2, &s.dev_slave);
    if (ret < 0)
    goto fail_unreg_2;
    ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0x4d0, 2, &s.dev_elcr);
    if (ret < 0)
    goto fail_unreg_1;
    mutex_unlock(&kvm.slots_lock);
    kvm.arch.vpic = s;
    return 0;
    fail_unreg_1:
    kvm_io_bus_unregister_dev(kvm, KVM_PIO_BUS, &s.dev_slave);
    fail_unreg_2:
    kvm_io_bus_unregister_dev(kvm, KVM_PIO_BUS, &s.dev_master);
    fail_unlock:
    mutex_unlock(&kvm.slots_lock);
    kfree(s);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_pic_destroy(kvm: *mut kvm) {
    void kvm_pic_destroy(struct kvm *kvm)
    {
    struct kvm_pic *vpic = kvm.arch.vpic;
    if (!vpic)
    return;
    mutex_lock(&kvm.slots_lock);
    kvm_io_bus_unregister_dev(vpic.kvm, KVM_PIO_BUS, &vpic.dev_master);
    kvm_io_bus_unregister_dev(vpic.kvm, KVM_PIO_BUS, &vpic.dev_slave);
    kvm_io_bus_unregister_dev(vpic.kvm, KVM_PIO_BUS, &vpic.dev_elcr);
    mutex_unlock(&kvm.slots_lock);
    kvm.arch.vpic = core::ptr::null_mut();
    kfree(vpic);
    }
