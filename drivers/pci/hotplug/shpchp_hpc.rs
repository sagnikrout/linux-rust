//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/shpchp_hpc.c
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
// Standard PCI Hot Plug Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
// Copyright (C) 2003-2004 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>,<kristen.c.accardi@intel.com>
//

// Slot Available Register I field definition
pub const SLOT_33MHZ: c_uint = 0x0000001f;
pub const SLOT_66MHZ_PCIX: c_uint = 0x00001f00;
pub const SLOT_100MHZ_PCIX: c_uint = 0x001f0000;
pub const SLOT_133MHZ_PCIX: c_uint = 0x1f000000;
// Slot Available Register II field definition
pub const SLOT_66MHZ: c_uint = 0x0000001f;
pub const SLOT_66MHZ_PCIX_266: c_uint = 0x00000f00;
pub const SLOT_100MHZ_PCIX_266: c_uint = 0x0000f000;
pub const SLOT_133MHZ_PCIX_266: c_uint = 0x000f0000;
pub const SLOT_66MHZ_PCIX_533: c_uint = 0x00f00000;
pub const SLOT_100MHZ_PCIX_533: c_uint = 0x0f000000;
pub const SLOT_133MHZ_PCIX_533: c_uint = 0xf0000000;
// Slot Configuration
pub const SLOT_NUM: c_uint = 0x0000001F;
pub const FIRST_DEV_NUM: c_uint = 0x00001F00;
pub const PSN: c_uint = 0x07FF0000;
pub const UPDOWN: c_uint = 0x20000000;
pub const MRLSENSOR: c_uint = 0x40000000;
pub const ATTN_BUTTON: c_uint = 0x80000000;
//
// Interrupt Locator Register definitions
//

//
// Controller SERR-INT Register
//

pub const SERR_INTR_RSVDZ_MASK: c_uint = 0xfffc0000;
//
// Logical Slot Register definitions
//

//
// SHPC Command Code definitions
//
// Slot Operation				00h - 3Fh
// Set Bus Segment Speed/Mode A		40h - 47h
// Power-Only All Slots			48h
// Enable All Slots				49h
// Set Bus Segment Speed/Mode B (PI=2)	50h - 5Fh
// Reserved Command Codes			60h - BFh
// Vendor Specific Commands			C0h - FFh
//
pub const SET_SLOT_PWR: c_uint = 0x01	/* Slot Operation */;
pub const SET_SLOT_ENABLE: c_uint = 0x02;
pub const SET_SLOT_DISABLE: c_uint = 0x03;
pub const SET_PWR_ON: c_uint = 0x04;
pub const SET_PWR_BLINK: c_uint = 0x08;
pub const SET_PWR_OFF: c_uint = 0x0c;
pub const SET_ATTN_ON: c_uint = 0x10;
pub const SET_ATTN_BLINK: c_uint = 0x20;
pub const SET_ATTN_OFF: c_uint = 0x30;
pub const SETA_PCI_33MHZ: c_uint = 0x40	/* Set Bus Segment Speed/Mode A */;
pub const SETA_PCI_66MHZ: c_uint = 0x41;
pub const SETA_PCIX_66MHZ: c_uint = 0x42;
pub const SETA_PCIX_100MHZ: c_uint = 0x43;
pub const SETA_PCIX_133MHZ: c_uint = 0x44;
pub const SETA_RESERVED1: c_uint = 0x45;
pub const SETA_RESERVED2: c_uint = 0x46;
pub const SETA_RESERVED3: c_uint = 0x47;
pub const SET_PWR_ONLY_ALL: c_uint = 0x48	/* Power-Only All Slots */;
pub const SET_ENABLE_ALL: c_uint = 0x49	/* Enable All Slots */;
pub const SETB_PCI_33MHZ: c_uint = 0x50	/* Set Bus Segment Speed/Mode B */;
pub const SETB_PCI_66MHZ: c_uint = 0x51;
pub const SETB_PCIX_66MHZ_PM: c_uint = 0x52;
pub const SETB_PCIX_100MHZ_PM: c_uint = 0x53;
pub const SETB_PCIX_133MHZ_PM: c_uint = 0x54;
pub const SETB_PCIX_66MHZ_EM: c_uint = 0x55;
pub const SETB_PCIX_100MHZ_EM: c_uint = 0x56;
pub const SETB_PCIX_133MHZ_EM: c_uint = 0x57;
pub const SETB_PCIX_66MHZ_266: c_uint = 0x58;
pub const SETB_PCIX_100MHZ_266: c_uint = 0x59;
pub const SETB_PCIX_133MHZ_266: c_uint = 0x5a;
pub const SETB_PCIX_66MHZ_533: c_uint = 0x5b;
pub const SETB_PCIX_100MHZ_533: c_uint = 0x5c;
pub const SETB_PCIX_133MHZ_533: c_uint = 0x5d;
pub const SETB_RESERVED1: c_uint = 0x5e;
pub const SETB_RESERVED2: c_uint = 0x5f;
//
// SHPC controller command error code
//
pub const SWITCH_OPEN: c_uint = 0x1;
pub const INVALID_CMD: c_uint = 0x2;
pub const INVALID_SPEED_MODE: c_uint = 0x4;
//
// For accessing SHPC Working Register Set via PCI Configuration Space
//
pub const DWORD_SELECT: c_uint = 0x2;
pub const DWORD_DATA: c_uint = 0x4;
// Field Offset in Logical Slot Register - byte boundary
pub const SLOT_EVENT_LATCH: c_uint = 0x2;
pub const SLOT_SERR_INT_MASK: c_uint = 0x3;
    static irqreturn_t shpc_isr(int irq, void *dev_id);
    static void start_int_poll_timer(struct controller *ctrl, int sec);
#[no_mangle]
pub unsafe extern "C" fn shpc_readb(ctrl: *mut controller, reg: c_int) -> u8 {
    static inline u8 shpc_readb(struct controller *ctrl, int reg)
    {
    return readb(ctrl.creg + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_readw(ctrl: *mut controller, reg: c_int) -> u16 {
    static inline u16 shpc_readw(struct controller *ctrl, int reg)
    {
    return readw(ctrl.creg + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_writew(ctrl: *mut controller, reg: c_int, val: u16) {
    static inline void shpc_writew(struct controller *ctrl, int reg, u16 val)
    {
    writew(val, ctrl.creg + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_readl(ctrl: *mut controller, reg: c_int) -> u32 {
    static inline u32 shpc_readl(struct controller *ctrl, int reg)
    {
    return readl(ctrl.creg + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_writel(ctrl: *mut controller, reg: c_int, val: u32) {
    static inline void shpc_writel(struct controller *ctrl, int reg, u32 val)
    {
    writel(val, ctrl.creg + reg);
    }
    static inline int shpc_indirect_read(struct controller *ctrl, int index,
    u32 *value)
    {
    int rc;
    let mut cap_offset: u32 = ctrl.cap_offset;
    struct pci_dev *pdev = ctrl.pci_dev;
    rc = pci_write_config_byte(pdev, cap_offset + DWORD_SELECT, index);
    if (rc)
    return rc;
    return pci_read_config_dword(pdev, cap_offset + DWORD_DATA, value);
    }
//
// This is the interrupt polling timeout function.
//
#[no_mangle]
unsafe extern "C" fn int_poll_timeout(t: *mut timer_list) {
    static void int_poll_timeout(struct timer_list *t)
    {
    struct controller *ctrl = timer_container_of(ctrl, t, poll_timer);
// Poll for interrupt events.  regs == NULL => polling
    shpc_isr(0, ctrl);
    if (!shpchp_poll_time)
    shpchp_poll_time = 2; /* default polling interval is 2 sec */
    start_int_poll_timer(ctrl, shpchp_poll_time);
    }
//
// This function starts the interrupt polling timer.
//
#[no_mangle]
unsafe extern "C" fn start_int_poll_timer(ctrl: *mut controller, sec: c_int) {
    static void start_int_poll_timer(struct controller *ctrl, int sec)
    {
// Clamp to sane value
    if ((sec <= 0) || (sec > 60))
    sec = 2;
    ctrl.poll_timer.expires = jiffies + sec * HZ;
    add_timer(&ctrl.poll_timer);
    }
#[no_mangle]
pub unsafe extern "C" fn is_ctrl_busy(ctrl: *mut controller) -> c_int {
    static inline int is_ctrl_busy(struct controller *ctrl)
    {
    let mut cmd_status: u16 = shpc_readw(ctrl, CMD_STATUS);
    return cmd_status & 0x1;
    }
//
// Returns 1 if SHPC finishes executing a command within 1 sec,
// otherwise returns 0.
//
#[no_mangle]
pub unsafe extern "C" fn shpc_poll_ctrl_busy(ctrl: *mut controller) -> c_int {
    static inline int shpc_poll_ctrl_busy(struct controller *ctrl)
    {
    int i;
    if (!is_ctrl_busy(ctrl))
    return 1;
// Check every 0.1 sec for a total of 1 sec
    for (i = 0; i < 10; i++) {
    msleep(100);
    if (!is_ctrl_busy(ctrl))
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_wait_cmd(ctrl: *mut controller) -> c_int {
    static inline int shpc_wait_cmd(struct controller *ctrl)
    {
    let mut retval: c_int = 0;
    let mut timeout: c_ulong = msecs_to_jiffies(1000);
    int rc;
    if (shpchp_poll_mode)
    rc = shpc_poll_ctrl_busy(ctrl);
    else
    rc = wait_event_interruptible_timeout(ctrl.queue,
    !is_ctrl_busy(ctrl), timeout);
    if (!rc && is_ctrl_busy(ctrl)) {
    retval = -EIO;
    ctrl_err(ctrl, "Command not completed in 1000 msec\n");
    } else if (rc < 0) {
    retval = -EINTR;
    ctrl_info(ctrl, "Command was interrupted by a signal\n");
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn shpc_write_cmd(slot: *mut slot, t_slot: u8, cmd: u8) -> c_int {
    static int shpc_write_cmd(struct slot *slot, u8 t_slot, u8 cmd)
    {
    struct controller *ctrl = slot.ctrl;
    u16 cmd_status;
    let mut retval: c_int = 0;
    u16 temp_word;
    mutex_lock(&slot.ctrl.cmd_lock);
    if (!shpc_poll_ctrl_busy(ctrl)) {
// After 1 sec and the controller is still busy
    ctrl_err(ctrl, "Controller is still busy after 1 sec\n");
    retval = -EBUSY;
    goto out;
    }
    ++t_slot;
    temp_word =  (t_slot << 8) | (cmd & 0xFF);
    ctrl_dbg(ctrl, "%s: t_slot %x cmd %x\n", __func__, t_slot, cmd);
// To make sure the Controller Busy bit is 0 before we send out the
// command.
//
    shpc_writew(ctrl, CMD, temp_word);
//
// Wait for command completion.
//
    retval = shpc_wait_cmd(slot.ctrl);
    if (retval)
    goto out;
    cmd_status = shpchp_check_cmd_status(slot.ctrl);
    if (cmd_status) {
    ctrl_err(ctrl, "Failed to issued command 0x%x (error code = %d)\n",
    cmd, cmd_status);
    retval = -EIO;
    }
    out:
    mutex_unlock(&slot.ctrl.cmd_lock);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_check_cmd_status(ctrl: *mut controller) -> c_int {
    int shpchp_check_cmd_status(struct controller *ctrl)
    {
    let mut retval: c_int = 0;
    let mut cmd_status: u16 = shpc_readw(ctrl, CMD_STATUS) & 0x000F;
    switch (cmd_status >> 1) {
    case 0:
    retval = 0;
    break;
    case 1:
    retval = SWITCH_OPEN;
    ctrl_err(ctrl, "Switch opened!\n");
    break;
    case 2:
    retval = INVALID_CMD;
    ctrl_err(ctrl, "Invalid HPC command!\n");
    break;
    case 4:
    retval = INVALID_SPEED_MODE;
    ctrl_err(ctrl, "Invalid bus speed/mode!\n");
    break;
    default:
    retval = cmd_status;
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_attention_status(slot: *mut slot, status: *mut u8) -> c_int {
    int shpchp_get_attention_status(struct slot *slot, u8 *status)
    {
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
    let mut state: u8 = (slot_reg & ATN_LED_STATE_MASK) >> ATN_LED_STATE_SHIFT;
    switch (state) {
    case ATN_LED_STATE_ON:
// status = 1;	/* On
    break;
    case ATN_LED_STATE_BLINK:
// status = 2;	/* Blink
    break;
    case ATN_LED_STATE_OFF:
// status = 0;	/* Off
    break;
    default:
// status = 0xFF;	/* Reserved
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_power_status(slot: *mut slot, status: *mut u8) -> c_int {
    int shpchp_get_power_status(struct slot *slot, u8 *status)
    {
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
    let mut state: u8 = (slot_reg & SLOT_STATE_MASK) >> SLOT_STATE_SHIFT;
    switch (state) {
    case SLOT_STATE_PWRONLY:
// status = 2;	/* Powered only
    break;
    case SLOT_STATE_ENABLED:
// status = 1;	/* Enabled
    break;
    case SLOT_STATE_DISABLED:
// status = 0;	/* Disabled
    break;
    default:
// status = 0xFF;	/* Reserved
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_latch_status(slot: *mut slot, status: *mut u8) -> c_int {
    int shpchp_get_latch_status(struct slot *slot, u8 *status)
    {
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
// status = !!(slot_reg & MRL_SENSOR);	/* 0 -> close; 1 -> open
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_adapter_status(slot: *mut slot, status: *mut u8) -> c_int {
    int shpchp_get_adapter_status(struct slot *slot, u8 *status)
    {
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
    let mut state: u8 = (slot_reg & PRSNT_MASK) >> PRSNT_SHIFT;
// status = (state != 0x3) ? 1 : 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_prog_int(slot: *mut slot, prog_int: *mut u8) -> c_int {
    int shpchp_get_prog_int(struct slot *slot, u8 *prog_int)
    {
    struct controller *ctrl = slot.ctrl;
// prog_int = shpc_readb(ctrl, PROG_INTERFACE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_get_adapter_speed(slot: *mut slot, value: *mut enum pci_bus_speed) -> c_int {
    int shpchp_get_adapter_speed(struct slot *slot, enum pci_bus_speed *value)
    {
    let mut retval: c_int = 0;
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
    let mut m66_cap: u8 = !!(slot_reg & MHZ66_CAP);
    u8 pi, pcix_cap;
    retval = shpchp_get_prog_int(slot, &pi);
    if (retval)
    return retval;
    switch (pi) {
    case 1:
    pcix_cap = (slot_reg & PCIX_CAP_MASK_PI1) >> PCIX_CAP_SHIFT;
    break;
    case 2:
    pcix_cap = (slot_reg & PCIX_CAP_MASK_PI2) >> PCIX_CAP_SHIFT;
    break;
    default:
    return -ENODEV;
    }
    ctrl_dbg(ctrl, "%s: slot_reg = %x, pcix_cap = %x, m66_cap = %x\n",
    __func__, slot_reg, pcix_cap, m66_cap);
    switch (pcix_cap) {
    case 0x0:
// value = m66_cap ? PCI_SPEED_66MHz : PCI_SPEED_33MHz;
    break;
    case 0x1:
// value = PCI_SPEED_66MHz_PCIX;
    break;
    case 0x3:
// value = PCI_SPEED_133MHz_PCIX;
    break;
    case 0x4:
// value = PCI_SPEED_133MHz_PCIX_266;
    break;
    case 0x5:
// value = PCI_SPEED_133MHz_PCIX_533;
    break;
    case 0x2:
    default:
// value = PCI_SPEED_UNKNOWN;
    retval = -ENODEV;
    break;
    }
    ctrl_dbg(ctrl, "Adapter speed = %d\n", *value);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_query_power_fault(slot: *mut slot) -> c_int {
    int shpchp_query_power_fault(struct slot *slot)
    {
    struct controller *ctrl = slot.ctrl;
    let mut slot_reg: u32 = shpc_readl(ctrl, SLOT_REG(slot.hp_slot));
// Note: Logic 0 => fault
    return !(slot_reg & POWER_FAULT);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_set_attention_status(slot: *mut slot, value: u8) -> c_int {
    int shpchp_set_attention_status(struct slot *slot, u8 value)
    {
    let mut slot_cmd: u8 = 0;
    switch (value) {
    case 0:
    slot_cmd = SET_ATTN_OFF;	/* OFF */
    break;
    case 1:
    slot_cmd = SET_ATTN_ON;		/* ON */
    break;
    case 2:
    slot_cmd = SET_ATTN_BLINK;	/* BLINK */
    break;
    default:
    return -1;
    }
    return shpc_write_cmd(slot, slot.hp_slot, slot_cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_green_led_on(slot: *mut slot) {
    void shpchp_green_led_on(struct slot *slot)
    {
    shpc_write_cmd(slot, slot.hp_slot, SET_PWR_ON);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_green_led_off(slot: *mut slot) {
    void shpchp_green_led_off(struct slot *slot)
    {
    shpc_write_cmd(slot, slot.hp_slot, SET_PWR_OFF);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_green_led_blink(slot: *mut slot) {
    void shpchp_green_led_blink(struct slot *slot)
    {
    shpc_write_cmd(slot, slot.hp_slot, SET_PWR_BLINK);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_release_ctlr(ctrl: *mut controller) {
    void shpchp_release_ctlr(struct controller *ctrl)
    {
    int i;
    u32 slot_reg, serr_int;
//
// Mask event interrupts and SERRs of all slots
//
    for (i = 0; i < ctrl.num_slots; i++) {
    slot_reg = shpc_readl(ctrl, SLOT_REG(i));
    slot_reg |= (PRSNT_CHANGE_INTR_MASK | ISO_PFAULT_INTR_MASK |
    BUTTON_PRESS_INTR_MASK | MRL_CHANGE_INTR_MASK |
    CON_PFAULT_INTR_MASK   | MRL_CHANGE_SERR_MASK |
    CON_PFAULT_SERR_MASK);
    slot_reg &= ~SLOT_REG_RSVDZ_MASK;
    shpc_writel(ctrl, SLOT_REG(i), slot_reg);
    }
    cleanup_slots(ctrl);
//
// Mask SERR and System Interrupt generation
//
    serr_int = shpc_readl(ctrl, SERR_INTR_ENABLE);
    serr_int |= (GLOBAL_INTR_MASK  | GLOBAL_SERR_MASK |
    COMMAND_INTR_MASK | ARBITER_SERR_MASK);
    serr_int &= ~SERR_INTR_RSVDZ_MASK;
    shpc_writel(ctrl, SERR_INTR_ENABLE, serr_int);
    if (shpchp_poll_mode)
    timer_delete(&ctrl.poll_timer);
    else {
    free_irq(ctrl.pci_dev.irq, ctrl);
    pci_disable_msi(ctrl.pci_dev);
    }
    iounmap(ctrl.creg);
    release_mem_region(ctrl.mmio_base, ctrl.mmio_size);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_power_on_slot(slot: *mut slot) -> c_int {
    int shpchp_power_on_slot(struct slot *slot)
    {
    int retval;
    retval = shpc_write_cmd(slot, slot.hp_slot, SET_SLOT_PWR);
    if (retval)
    ctrl_err(slot.ctrl, "%s: Write command failed!\n", __func__);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_slot_enable(slot: *mut slot) -> c_int {
    int shpchp_slot_enable(struct slot *slot)
    {
    int retval;
// Slot - Enable, Power Indicator - Blink, Attention Indicator - Off
    retval = shpc_write_cmd(slot, slot.hp_slot,
    SET_SLOT_ENABLE | SET_PWR_BLINK | SET_ATTN_OFF);
    if (retval)
    ctrl_err(slot.ctrl, "%s: Write command failed!\n", __func__);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_slot_disable(slot: *mut slot) -> c_int {
    int shpchp_slot_disable(struct slot *slot)
    {
    int retval;
// Slot - Disable, Power Indicator - Off, Attention Indicator - On
    retval = shpc_write_cmd(slot, slot.hp_slot,
    SET_SLOT_DISABLE | SET_PWR_OFF | SET_ATTN_ON);
    if (retval)
    ctrl_err(slot.ctrl, "%s: Write command failed!\n", __func__);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn shpc_get_cur_bus_speed(ctrl: *mut controller) -> c_int {
    static int shpc_get_cur_bus_speed(struct controller *ctrl)
    {
    let mut retval: c_int = 0;
    struct pci_bus *bus = ctrl.pci_dev.subordinate;
    let mut bus_speed: enum pci_bus_speed = PCI_SPEED_UNKNOWN;
    let mut sec_bus_reg: u16 = shpc_readw(ctrl, SEC_BUS_CONFIG);
    let mut pi: u8 = shpc_readb(ctrl, PROG_INTERFACE);
    let mut speed_mode: u8 = (pi == 2) ? (sec_bus_reg & 0xF) : (sec_bus_reg & 0x7);
    if ((pi == 1) && (speed_mode > 4)) {
    retval = -ENODEV;
    goto out;
    }
    switch (speed_mode) {
    case 0x0:
    bus_speed = PCI_SPEED_33MHz;
    break;
    case 0x1:
    bus_speed = PCI_SPEED_66MHz;
    break;
    case 0x2:
    bus_speed = PCI_SPEED_66MHz_PCIX;
    break;
    case 0x3:
    bus_speed = PCI_SPEED_100MHz_PCIX;
    break;
    case 0x4:
    bus_speed = PCI_SPEED_133MHz_PCIX;
    break;
    case 0x5:
    bus_speed = PCI_SPEED_66MHz_PCIX_ECC;
    break;
    case 0x6:
    bus_speed = PCI_SPEED_100MHz_PCIX_ECC;
    break;
    case 0x7:
    bus_speed = PCI_SPEED_133MHz_PCIX_ECC;
    break;
    case 0x8:
    bus_speed = PCI_SPEED_66MHz_PCIX_266;
    break;
    case 0x9:
    bus_speed = PCI_SPEED_100MHz_PCIX_266;
    break;
    case 0xa:
    bus_speed = PCI_SPEED_133MHz_PCIX_266;
    break;
    case 0xb:
    bus_speed = PCI_SPEED_66MHz_PCIX_533;
    break;
    case 0xc:
    bus_speed = PCI_SPEED_100MHz_PCIX_533;
    break;
    case 0xd:
    bus_speed = PCI_SPEED_133MHz_PCIX_533;
    break;
    default:
    retval = -ENODEV;
    break;
    }
    out:
    bus.cur_bus_speed = bus_speed;
    ctrl_dbg(ctrl, "Current bus speed = %d\n", bus_speed);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_set_bus_speed_mode(slot: *mut slot, value: enum pci_bus_speed) -> c_int {
    int shpchp_set_bus_speed_mode(struct slot *slot, enum pci_bus_speed value)
    {
    int retval;
    struct controller *ctrl = slot.ctrl;
    u8 pi, cmd;
    pi = shpc_readb(ctrl, PROG_INTERFACE);
    if ((pi == 1) && (value > PCI_SPEED_133MHz_PCIX))
    return -EINVAL;
    switch (value) {
    case PCI_SPEED_33MHz:
    cmd = SETA_PCI_33MHZ;
    break;
    case PCI_SPEED_66MHz:
    cmd = SETA_PCI_66MHZ;
    break;
    case PCI_SPEED_66MHz_PCIX:
    cmd = SETA_PCIX_66MHZ;
    break;
    case PCI_SPEED_100MHz_PCIX:
    cmd = SETA_PCIX_100MHZ;
    break;
    case PCI_SPEED_133MHz_PCIX:
    cmd = SETA_PCIX_133MHZ;
    break;
    case PCI_SPEED_66MHz_PCIX_ECC:
    cmd = SETB_PCIX_66MHZ_EM;
    break;
    case PCI_SPEED_100MHz_PCIX_ECC:
    cmd = SETB_PCIX_100MHZ_EM;
    break;
    case PCI_SPEED_133MHz_PCIX_ECC:
    cmd = SETB_PCIX_133MHZ_EM;
    break;
    case PCI_SPEED_66MHz_PCIX_266:
    cmd = SETB_PCIX_66MHZ_266;
    break;
    case PCI_SPEED_100MHz_PCIX_266:
    cmd = SETB_PCIX_100MHZ_266;
    break;
    case PCI_SPEED_133MHz_PCIX_266:
    cmd = SETB_PCIX_133MHZ_266;
    break;
    case PCI_SPEED_66MHz_PCIX_533:
    cmd = SETB_PCIX_66MHZ_533;
    break;
    case PCI_SPEED_100MHz_PCIX_533:
    cmd = SETB_PCIX_100MHZ_533;
    break;
    case PCI_SPEED_133MHz_PCIX_533:
    cmd = SETB_PCIX_133MHZ_533;
    break;
    default:
    return -EINVAL;
    }
    retval = shpc_write_cmd(slot, 0, cmd);
    if (retval)
    ctrl_err(ctrl, "%s: Write command failed!\n", __func__);
    else
    shpc_get_cur_bus_speed(ctrl);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn shpc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t shpc_isr(int irq, void *dev_id)
    {
    struct controller *ctrl = (struct controller *)dev_id;
    u32 serr_int, slot_reg, intr_loc, intr_loc2;
    int hp_slot;
// Check to see if it was our interrupt
    intr_loc = shpc_readl(ctrl, INTR_LOC);
    if (!intr_loc)
    return IRQ_NONE;
    ctrl_dbg(ctrl, "%s: intr_loc = %x\n", __func__, intr_loc);
    if (!shpchp_poll_mode) {
//
// Mask Global Interrupt Mask - see implementation
// note on p. 139 of SHPC spec rev 1.0
//
    serr_int = shpc_readl(ctrl, SERR_INTR_ENABLE);
    serr_int |= GLOBAL_INTR_MASK;
    serr_int &= ~SERR_INTR_RSVDZ_MASK;
    shpc_writel(ctrl, SERR_INTR_ENABLE, serr_int);
    intr_loc2 = shpc_readl(ctrl, INTR_LOC);
    ctrl_dbg(ctrl, "%s: intr_loc2 = %x\n", __func__, intr_loc2);
    }
    if (intr_loc & CMD_INTR_PENDING) {
//
// Command Complete Interrupt Pending
// RO only - clear by writing 1 to the Command Completion
// Detect bit in Controller SERR-INT register
//
    serr_int = shpc_readl(ctrl, SERR_INTR_ENABLE);
    serr_int &= ~SERR_INTR_RSVDZ_MASK;
    shpc_writel(ctrl, SERR_INTR_ENABLE, serr_int);
    wake_up_interruptible(&ctrl.queue);
    }
    if (!(intr_loc & ~CMD_INTR_PENDING))
    goto out;
    for (hp_slot = 0; hp_slot < ctrl.num_slots; hp_slot++) {
// To find out which slot has interrupt pending
    if (!(intr_loc & SLOT_INTR_PENDING(hp_slot)))
    continue;
    slot_reg = shpc_readl(ctrl, SLOT_REG(hp_slot));
    ctrl_dbg(ctrl, "Slot %x with intr, slot register = %x\n",
    hp_slot, slot_reg);
    if (slot_reg & MRL_CHANGE_DETECTED)
    shpchp_handle_switch_change(hp_slot, ctrl);
    if (slot_reg & BUTTON_PRESS_DETECTED)
    shpchp_handle_attention_button(hp_slot, ctrl);
    if (slot_reg & PRSNT_CHANGE_DETECTED)
    shpchp_handle_presence_change(hp_slot, ctrl);
    if (slot_reg & (ISO_PFAULT_DETECTED | CON_PFAULT_DETECTED))
    shpchp_handle_power_fault(hp_slot, ctrl);
// Clear all slot events
    slot_reg &= ~SLOT_REG_RSVDZ_MASK;
    shpc_writel(ctrl, SLOT_REG(hp_slot), slot_reg);
    }
    out:
    if (!shpchp_poll_mode) {
// Unmask Global Interrupt Mask
    serr_int = shpc_readl(ctrl, SERR_INTR_ENABLE);
    serr_int &= ~(GLOBAL_INTR_MASK | SERR_INTR_RSVDZ_MASK);
    shpc_writel(ctrl, SERR_INTR_ENABLE, serr_int);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn shpc_get_max_bus_speed(ctrl: *mut controller) -> c_int {
    static int shpc_get_max_bus_speed(struct controller *ctrl)
    {
    let mut retval: c_int = 0;
    struct pci_bus *bus = ctrl.pci_dev.subordinate;
    let mut bus_speed: enum pci_bus_speed = PCI_SPEED_UNKNOWN;
    let mut pi: u8 = shpc_readb(ctrl, PROG_INTERFACE);
    let mut slot_avail1: u32 = shpc_readl(ctrl, SLOT_AVAIL1);
    let mut slot_avail2: u32 = shpc_readl(ctrl, SLOT_AVAIL2);
    if (pi == 2) {
    if (slot_avail2 & SLOT_133MHZ_PCIX_533)
    bus_speed = PCI_SPEED_133MHz_PCIX_533;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_100MHZ_PCIX_533: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_100MHZ_PCIX_533)
    bus_speed = PCI_SPEED_100MHz_PCIX_533;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_66MHZ_PCIX_533: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_66MHZ_PCIX_533)
    bus_speed = PCI_SPEED_66MHz_PCIX_533;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_133MHZ_PCIX_266: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_133MHZ_PCIX_266)
    bus_speed = PCI_SPEED_133MHz_PCIX_266;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_100MHZ_PCIX_266: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_100MHZ_PCIX_266)
    bus_speed = PCI_SPEED_100MHz_PCIX_266;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_66MHZ_PCIX_266: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_66MHZ_PCIX_266)
    bus_speed = PCI_SPEED_66MHz_PCIX_266;
    }
    if (bus_speed == PCI_SPEED_UNKNOWN) {
    if (slot_avail1 & SLOT_133MHZ_PCIX)
    bus_speed = PCI_SPEED_133MHz_PCIX;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_100MHZ_PCIX: slot_avail1 &) -> else {
    else if (slot_avail1 & SLOT_100MHZ_PCIX)
    bus_speed = PCI_SPEED_100MHz_PCIX;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_66MHZ_PCIX: slot_avail1 &) -> else {
    else if (slot_avail1 & SLOT_66MHZ_PCIX)
    bus_speed = PCI_SPEED_66MHz_PCIX;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_66MHZ: slot_avail2 &) -> else {
    else if (slot_avail2 & SLOT_66MHZ)
    bus_speed = PCI_SPEED_66MHz;
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_33MHZ: slot_avail1 &) -> else {
    else if (slot_avail1 & SLOT_33MHZ)
    bus_speed = PCI_SPEED_33MHz;
    else
    retval = -ENODEV;
    }
    bus.max_bus_speed = bus_speed;
    ctrl_dbg(ctrl, "Max bus speed = %d\n", bus_speed);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn shpc_init(ctrl: *mut controller, pdev: *mut pci_dev) -> c_int {
    int shpc_init(struct controller *ctrl, struct pci_dev *pdev)
    {
    let mut rc: c_int = -1, num_slots = 0;
    u8 hp_slot;
    u32 shpc_base_offset;
    u32 tempdword, slot_reg, slot_config;
    u8 i;
    ctrl.pci_dev = pdev;  /* pci_dev of the P2P bridge */
    ctrl_dbg(ctrl, "Hotplug Controller:\n");
    if (pdev.vendor == PCI_VENDOR_ID_AMD &&
    pdev.device == PCI_DEVICE_ID_AMD_GOLAM_7450) {
// amd shpc driver doesn't use Base Offset; assume 0
    ctrl.mmio_base = pci_resource_start(pdev, 0);
    ctrl.mmio_size = pci_resource_len(pdev, 0);
    } else {
    ctrl.cap_offset = pci_find_capability(pdev, PCI_CAP_ID_SHPC);
    if (!ctrl.cap_offset) {
    ctrl_err(ctrl, "Cannot find PCI capability\n");
    goto abort;
    }
    ctrl_dbg(ctrl, " cap_offset = %x\n", ctrl.cap_offset);
    rc = shpc_indirect_read(ctrl, 0, &shpc_base_offset);
    if (rc) {
    ctrl_err(ctrl, "Cannot read base_offset\n");
    goto abort;
    }
    rc = shpc_indirect_read(ctrl, 3, &tempdword);
    if (rc) {
    ctrl_err(ctrl, "Cannot read slot config\n");
    goto abort;
    }
    num_slots = tempdword & SLOT_NUM;
    ctrl_dbg(ctrl, " num_slots (indirect) %x\n", num_slots);
    for (i = 0; i < 9 + num_slots; i++) {
    rc = shpc_indirect_read(ctrl, i, &tempdword);
    if (rc) {
    ctrl_err(ctrl, "Cannot read creg (index = %d)\n",
    i);
    goto abort;
    }
    ctrl_dbg(ctrl, " offset %d: value %x\n", i, tempdword);
    }
    ctrl.mmio_base =
    pci_resource_start(pdev, 0) + shpc_base_offset;
    ctrl.mmio_size = 0x24 + 0x4 * num_slots;
    }
    ctrl_info(ctrl, "HPC vendor_id %x device_id %x ss_vid %x ss_did %x\n",
    pdev.vendor, pdev.device, pdev.subsystem_vendor,
    pdev.subsystem_device);
    rc = pci_enable_device(pdev);
    if (rc) {
    ctrl_err(ctrl, "pci_enable_device failed\n");
    goto abort;
    }
    if (!request_mem_region(ctrl.mmio_base, ctrl.mmio_size, MY_NAME)) {
    ctrl_err(ctrl, "Cannot reserve MMIO region\n");
    rc = -1;
    goto abort;
    }
    ctrl.creg = ioremap(ctrl.mmio_base, ctrl.mmio_size);
    if (!ctrl.creg) {
    ctrl_err(ctrl, "Cannot remap MMIO region %lx @ %lx\n",
    ctrl.mmio_size, ctrl.mmio_base);
    release_mem_region(ctrl.mmio_base, ctrl.mmio_size);
    rc = -1;
    goto abort;
    }
    ctrl_dbg(ctrl, "ctrl.creg %p\n", ctrl.creg);
    mutex_init(&ctrl.crit_sect);
    mutex_init(&ctrl.cmd_lock);
// Setup wait queue
    init_waitqueue_head(&ctrl.queue);
// Return PCI Controller Info
    slot_config = shpc_readl(ctrl, SLOT_CONFIG);
    ctrl.slot_device_offset = (slot_config & FIRST_DEV_NUM) >> 8;
    ctrl.num_slots = slot_config & SLOT_NUM;
    ctrl.first_slot = (slot_config & PSN) >> 16;
    ctrl.slot_num_inc = ((slot_config & UPDOWN) >> 29) ? 1 : -1;
// Mask Global Interrupt Mask & Command Complete Interrupt Mask
    tempdword = shpc_readl(ctrl, SERR_INTR_ENABLE);
    ctrl_dbg(ctrl, "SERR_INTR_ENABLE = %x\n", tempdword);
    tempdword |= (GLOBAL_INTR_MASK  | GLOBAL_SERR_MASK |
    COMMAND_INTR_MASK | ARBITER_SERR_MASK);
    tempdword &= ~SERR_INTR_RSVDZ_MASK;
    shpc_writel(ctrl, SERR_INTR_ENABLE, tempdword);
    tempdword = shpc_readl(ctrl, SERR_INTR_ENABLE);
    ctrl_dbg(ctrl, "SERR_INTR_ENABLE = %x\n", tempdword);
// Mask the MRL sensor SERR Mask of individual slot in
// Slot SERR-INT Mask & clear all the existing event if any
//
    for (hp_slot = 0; hp_slot < ctrl.num_slots; hp_slot++) {
    slot_reg = shpc_readl(ctrl, SLOT_REG(hp_slot));
    ctrl_dbg(ctrl, "Default Logical Slot Register %d value %x\n",
    hp_slot, slot_reg);
    slot_reg |= (PRSNT_CHANGE_INTR_MASK | ISO_PFAULT_INTR_MASK |
    BUTTON_PRESS_INTR_MASK | MRL_CHANGE_INTR_MASK |
    CON_PFAULT_INTR_MASK   | MRL_CHANGE_SERR_MASK |
    CON_PFAULT_SERR_MASK);
    slot_reg &= ~SLOT_REG_RSVDZ_MASK;
    shpc_writel(ctrl, SLOT_REG(hp_slot), slot_reg);
    }
    if (shpchp_poll_mode) {
// Install interrupt polling timer. Start with 10 sec delay
    timer_setup(&ctrl.poll_timer, int_poll_timeout, 0);
    start_int_poll_timer(ctrl, 10);
    } else {
// Installs the interrupt handler
    rc = pci_enable_msi(pdev);
    if (rc) {
    ctrl_info(ctrl, "Can't get msi for the hotplug controller\n");
    ctrl_info(ctrl, "Use INTx for the hotplug controller\n");
    } else {
    pci_set_master(pdev);
    }
    rc = request_irq(ctrl.pci_dev.irq, shpc_isr, IRQF_SHARED,
    MY_NAME, (void *)ctrl);
    ctrl_dbg(ctrl, "request_irq %d (returns %d)\n",
    ctrl.pci_dev.irq, rc);
    if (rc) {
    ctrl_err(ctrl, "Can't get irq %d for the hotplug controller\n",
    ctrl.pci_dev.irq);
    goto abort_iounmap;
    }
    }
    ctrl_dbg(ctrl, "HPC at %s irq=%x\n", pci_name(pdev), pdev.irq);
    shpc_get_max_bus_speed(ctrl);
    shpc_get_cur_bus_speed(ctrl);
//
// Unmask all event interrupts of all slots
//
    for (hp_slot = 0; hp_slot < ctrl.num_slots; hp_slot++) {
    slot_reg = shpc_readl(ctrl, SLOT_REG(hp_slot));
    ctrl_dbg(ctrl, "Default Logical Slot Register %d value %x\n",
    hp_slot, slot_reg);
    slot_reg &= ~(PRSNT_CHANGE_INTR_MASK | ISO_PFAULT_INTR_MASK |
    BUTTON_PRESS_INTR_MASK | MRL_CHANGE_INTR_MASK |
    CON_PFAULT_INTR_MASK | SLOT_REG_RSVDZ_MASK);
    shpc_writel(ctrl, SLOT_REG(hp_slot), slot_reg);
    }
    if (!shpchp_poll_mode) {
// Unmask all general input interrupts and SERR
    tempdword = shpc_readl(ctrl, SERR_INTR_ENABLE);
    tempdword &= ~(GLOBAL_INTR_MASK | COMMAND_INTR_MASK |
    SERR_INTR_RSVDZ_MASK);
    shpc_writel(ctrl, SERR_INTR_ENABLE, tempdword);
    tempdword = shpc_readl(ctrl, SERR_INTR_ENABLE);
    ctrl_dbg(ctrl, "SERR_INTR_ENABLE = %x\n", tempdword);
    }
    return 0;
// We end up here for the many possible ways to fail this API.
    abort_iounmap:
    iounmap(ctrl.creg);
    abort:
    return rc;
    }
