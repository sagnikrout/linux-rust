//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/mtk-cmdq-mailbox.c
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
// Copyright (c) 2018 MediaTek Inc.

pub const CMDQ_MBOX_AUTOSUSPEND_DELAY_MS: c_int = 100;

pub const CMDQ_CURR_IRQ_STATUS: c_uint = 0x10;
pub const CMDQ_SYNC_TOKEN_UPDATE: c_uint = 0x68;
pub const CMDQ_THR_SLOT_CYCLES: c_uint = 0x30;
pub const CMDQ_THR_BASE: c_uint = 0x100;
pub const CMDQ_THR_SIZE: c_uint = 0x80;
pub const CMDQ_THR_WARM_RESET: c_uint = 0x00;
pub const CMDQ_THR_ENABLE_TASK: c_uint = 0x04;
pub const CMDQ_THR_SUSPEND_TASK: c_uint = 0x08;
pub const CMDQ_THR_CURR_STATUS: c_uint = 0x0c;
pub const CMDQ_THR_IRQ_STATUS: c_uint = 0x10;
pub const CMDQ_THR_IRQ_ENABLE: c_uint = 0x14;
pub const CMDQ_THR_CURR_ADDR: c_uint = 0x20;
pub const CMDQ_THR_END_ADDR: c_uint = 0x24;
pub const CMDQ_THR_WAIT_TOKEN: c_uint = 0x30;
pub const CMDQ_THR_PRIORITY: c_uint = 0x40;
pub const GCE_GCTL_VALUE: c_uint = 0x48;

pub const GCE_VM_CPR_GSIZE: c_uint = 0x50c4;

pub const CMDQ_THR_ACTIVE_SLOT_CYCLES: c_uint = 0x3200;
pub const CMDQ_THR_ENABLED: c_uint = 0x1;
pub const CMDQ_THR_DISABLED: c_uint = 0x0;
pub const CMDQ_THR_SUSPEND: c_uint = 0x1;
pub const CMDQ_THR_RESUME: c_uint = 0x0;

pub const CMDQ_THR_IRQ_DONE: c_uint = 0x1;
pub const CMDQ_THR_IRQ_ERROR: c_uint = 0x12;

pub const CMDQ_JUMP_BY_OFFSET: c_uint = 0x10000000;
pub const CMDQ_JUMP_BY_PA: c_uint = 0x10000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_thread {
    pub chan: *mut mbox_chan,
    pub base: *mut void __iomem,
    pub task_busy_list: list_head,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_task {
    pub cmdq: *mut cmdq,
    pub list_entry: list_head,
    pub pa_base: dma_addr_t,
    pub thread: *mut cmdq_thread,
    pub /: *mut *mut *mut cmdq_pkt pkt; / the packet sent from mailbox client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq {
    pub mbox: mbox_controller,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub irq_mask: u32,
    pub pdata: *const gce_plat,
    pub thread: *mut cmdq_thread,
    pub clocks: *mut clk_bulk_data,
    pub suspended: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gce_plat {
    pub thread_nr: u32,
    pub shift: u8,
    pub mminfra_offset: dma_addr_t,
    pub control_by_sw: bool,
    pub sw_ddr_en: bool,
    pub gce_vm: bool,
    pub gce_num: u32,
}

#[no_mangle]
pub unsafe extern "C" fn cmdq_convert_gce_addr(addr: dma_addr_t, pdata: *const gce_plat) -> u32 {
    static inline u32 cmdq_convert_gce_addr(dma_addr_t addr, const struct gce_plat *pdata)
    {
// Convert DMA addr (PA or IOVA) to GCE readable addr
    return (addr + pdata.mminfra_offset) >> pdata.shift;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdq_revert_gce_addr(addr: u32, pdata: *const gce_plat) -> dma_addr_t {
    static inline dma_addr_t cmdq_revert_gce_addr(u32 addr, const struct gce_plat *pdata)
    {
// Revert GCE readable addr to DMA addr (PA or IOVA)
    return ((dma_addr_t)addr << pdata.shift) - pdata.mminfra_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_mbox_priv(chan: *mut mbox_chan, priv: *mut cmdq_mbox_priv) {
    void cmdq_get_mbox_priv(struct mbox_chan *chan, struct cmdq_mbox_priv *priv)
    {
    struct cmdq *cmdq = container_of(chan.mbox, struct cmdq, mbox);
    priv.shift_pa = cmdq.pdata.shift;
    priv.mminfra_offset = cmdq.pdata.mminfra_offset;
    }
    EXPORT_SYMBOL(cmdq_get_mbox_priv);
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_shift_pa(chan: *mut mbox_chan) -> u8 {
    u8 cmdq_get_shift_pa(struct mbox_chan *chan)
    {
    struct cmdq *cmdq = container_of(chan.mbox, struct cmdq, mbox);
    return cmdq.pdata.shift;
    }
    EXPORT_SYMBOL(cmdq_get_shift_pa);
#[no_mangle]
unsafe extern "C" fn cmdq_vm_init(cmdq: *mut cmdq) {
    static void cmdq_vm_init(struct cmdq *cmdq)
    {
    int i;
    let mut vm_cpr_gsize: u32 = 0, vm_id_map = 0;
    u32 *vm_map = core::ptr::null_mut();
    if (!cmdq.pdata.gce_vm)
    return;
    vm_map = kcalloc(cmdq.pdata.thread_nr, sizeof(*vm_map), GFP_KERNEL);
    if (!vm_map)
    return;
// only configure the max CPR SRAM size to host vm (vm_id = 0) currently
    vm_cpr_gsize = GCE_VM_CPR_GSIZE_MAX << GCE_VM_CPR_GSIZE_FLD_SHIFT(0);
// set all thread mapping to host vm currently
    for (i = 0; i < cmdq.pdata.thread_nr; i++)
    vm_map[i] = GCE_VM_ID_MAP_HOST_VM << GCE_VM_ID_MAP_THR_FLD_SHIFT(i);
// set the amount of CPR SRAM to allocate to each VM
    writel(vm_cpr_gsize, cmdq.base + GCE_VM_CPR_GSIZE);
// config CPR_GSIZE before setting VM_ID_MAP to avoid data leakage
    for (i = 0; i < cmdq.pdata.thread_nr; i++) {
    vm_id_map |= vm_map[i];
// config every 10 threads, e.g., thread id=0~9, 10~19, ..., into one register
    if ((i + 1) % 10 == 0) {
    writel(vm_id_map, cmdq.base + GCE_VM_ID_MAP(i));
    vm_id_map = 0;
    }
    }
// config remaining threads settings
    if (cmdq.pdata.thread_nr % 10 != 0)
    writel(vm_id_map, cmdq.base + GCE_VM_ID_MAP(cmdq.pdata.thread_nr - 1));
    kfree(vm_map);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_gctl_value_toggle(cmdq: *mut cmdq, ddr_enable: bool) {
    static void cmdq_gctl_value_toggle(struct cmdq *cmdq, bool ddr_enable)
    {
    let mut val: u32 = cmdq.pdata.control_by_sw ? GCE_CTRL_BY_SW : 0;
    if (!cmdq.pdata.control_by_sw && !cmdq.pdata.sw_ddr_en)
    return;
    if (cmdq.pdata.sw_ddr_en && ddr_enable)
    val |= GCE_DDR_EN;
    writel(val, cmdq.base + GCE_GCTL_VALUE);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_thread_suspend(cmdq: *mut cmdq, thread: *mut cmdq_thread) -> c_int {
    static int cmdq_thread_suspend(struct cmdq *cmdq, struct cmdq_thread *thread)
    {
    u32 status;
    writel(CMDQ_THR_SUSPEND, thread.base + CMDQ_THR_SUSPEND_TASK);
// If already disabled, treat as suspended successful.
    if (!(readl(thread.base + CMDQ_THR_ENABLE_TASK) & CMDQ_THR_ENABLED))
    return 0;
    if (readl_poll_timeout_atomic(thread.base + CMDQ_THR_CURR_STATUS,
    status, status & CMDQ_THR_STATUS_SUSPENDED, 0, 10)) {
    dev_err(cmdq.mbox.dev, "suspend GCE thread 0x%x failed\n",
    (u32)(thread.base - cmdq.base));
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_thread_resume(thread: *mut cmdq_thread) {
    static void cmdq_thread_resume(struct cmdq_thread *thread)
    {
    writel(CMDQ_THR_RESUME, thread.base + CMDQ_THR_SUSPEND_TASK);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_init(cmdq: *mut cmdq) {
    static void cmdq_init(struct cmdq *cmdq)
    {
    int i;
    WARN_ON(clk_bulk_enable(cmdq.pdata.gce_num, cmdq.clocks));
    cmdq_vm_init(cmdq);
    cmdq_gctl_value_toggle(cmdq, true);
    writel(CMDQ_THR_ACTIVE_SLOT_CYCLES, cmdq.base + CMDQ_THR_SLOT_CYCLES);
    for (i = 0; i <= CMDQ_MAX_EVENT; i++)
    writel(i, cmdq.base + CMDQ_SYNC_TOKEN_UPDATE);
    clk_bulk_disable(cmdq.pdata.gce_num, cmdq.clocks);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_thread_reset(cmdq: *mut cmdq, thread: *mut cmdq_thread) -> c_int {
    static int cmdq_thread_reset(struct cmdq *cmdq, struct cmdq_thread *thread)
    {
    u32 warm_reset;
    writel(CMDQ_THR_DO_WARM_RESET, thread.base + CMDQ_THR_WARM_RESET);
    if (readl_poll_timeout_atomic(thread.base + CMDQ_THR_WARM_RESET,
    warm_reset, !(warm_reset & CMDQ_THR_DO_WARM_RESET),
    0, 10)) {
    dev_err(cmdq.mbox.dev, "reset GCE thread 0x%x failed\n",
    (u32)(thread.base - cmdq.base));
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_thread_disable(cmdq: *mut cmdq, thread: *mut cmdq_thread) {
    static void cmdq_thread_disable(struct cmdq *cmdq, struct cmdq_thread *thread)
    {
    cmdq_thread_reset(cmdq, thread);
    writel(CMDQ_THR_DISABLED, thread.base + CMDQ_THR_ENABLE_TASK);
    }
// notify GCE to re-fetch commands by setting GCE thread PC
#[no_mangle]
unsafe extern "C" fn cmdq_thread_invalidate_fetched_data(thread: *mut cmdq_thread) {
    static void cmdq_thread_invalidate_fetched_data(struct cmdq_thread *thread)
    {
    writel(readl(thread.base + CMDQ_THR_CURR_ADDR),
    thread.base + CMDQ_THR_CURR_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_task_insert_into_thread(task: *mut cmdq_task) {
    static void cmdq_task_insert_into_thread(struct cmdq_task *task)
    {
    struct device *dev = task.cmdq.mbox.dev;
    struct cmdq_thread *thread = task.thread;
    struct cmdq_task *prev_task = list_last_entry(
    &thread.task_busy_list, typeof(*task), list_entry);
    u64 *prev_task_base = prev_task.pkt.va_base;
    let mut gce_addr: u32 = cmdq_convert_gce_addr(task.pa_base, task.cmdq.pdata);
// let previous task jump to this task
    dma_sync_single_for_cpu(dev, prev_task.pa_base,
    prev_task.pkt.cmd_buf_size, DMA_TO_DEVICE);
    prev_task_base[CMDQ_NUM_CMD(prev_task.pkt) - 1] = (u64)CMDQ_JUMP_BY_PA << 32 | gce_addr;
    dma_sync_single_for_device(dev, prev_task.pa_base,
    prev_task.pkt.cmd_buf_size, DMA_TO_DEVICE);
    cmdq_thread_invalidate_fetched_data(thread);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_thread_is_in_wfe(thread: *mut cmdq_thread) -> bool {
    static bool cmdq_thread_is_in_wfe(struct cmdq_thread *thread)
    {
    return readl(thread.base + CMDQ_THR_WAIT_TOKEN) & CMDQ_THR_IS_WAITING;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_task_exec_done(task: *mut cmdq_task, sta: c_int) {
    static void cmdq_task_exec_done(struct cmdq_task *task, int sta)
    {
    struct cmdq_cb_data data;
    data.sta = sta;
    data.pkt = task.pkt;
    mbox_chan_received_data(task.thread.chan, &data);
    list_del(&task.list_entry);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_task_handle_error(task: *mut cmdq_task) {
    static void cmdq_task_handle_error(struct cmdq_task *task)
    {
    struct cmdq_thread *thread = task.thread;
    struct cmdq_task *next_task;
    struct cmdq *cmdq = task.cmdq;
    dev_err(cmdq.mbox.dev, "task 0x%p error\n", task);
    WARN_ON(cmdq_thread_suspend(cmdq, thread) < 0);
    next_task = list_first_entry_or_null(&thread.task_busy_list,
    struct cmdq_task, list_entry);
    if (next_task)
    writel(next_task.pa_base >> cmdq.pdata.shift,
    thread.base + CMDQ_THR_CURR_ADDR);
    cmdq_thread_resume(thread);
    }
    static void cmdq_thread_irq_handler(struct cmdq *cmdq,
    struct cmdq_thread *thread)
    {
    struct cmdq_task *task, *tmp, *curr_task = core::ptr::null_mut();
    u32 irq_flag, gce_addr;
    dma_addr_t curr_pa, task_end_pa;
    bool err;
    irq_flag = readl(thread.base + CMDQ_THR_IRQ_STATUS);
    writel(~irq_flag, thread.base + CMDQ_THR_IRQ_STATUS);
//
// When ISR call this function, another CPU core could run
// "release task" right before we acquire the spin lock, and thus
// reset / disable this GCE thread, so we need to check the enable
// bit of this GCE thread.
//
    if (!(readl(thread.base + CMDQ_THR_ENABLE_TASK) & CMDQ_THR_ENABLED))
    return;
    if (irq_flag & CMDQ_THR_IRQ_ERROR)
    err = true;
#[no_mangle]
pub unsafe extern "C" fn if(CMDQ_THR_IRQ_DONE: irq_flag &) -> else {
    else if (irq_flag & CMDQ_THR_IRQ_DONE)
    err = false;
    else
    return;
    gce_addr = readl(thread.base + CMDQ_THR_CURR_ADDR);
    curr_pa = cmdq_revert_gce_addr(gce_addr, cmdq.pdata);
    list_for_each_entry_safe(task, tmp, &thread.task_busy_list,
    list_entry) {
    task_end_pa = task.pa_base + task.pkt.cmd_buf_size;
    if (curr_pa >= task.pa_base && curr_pa < task_end_pa)
    curr_task = task;
    if (!curr_task || curr_pa == task_end_pa - CMDQ_INST_SIZE) {
    cmdq_task_exec_done(task, 0);
    kfree(task);
    } else if (err) {
    cmdq_task_exec_done(task, -ENOEXEC);
    cmdq_task_handle_error(curr_task);
    kfree(task);
    }
    if (curr_task)
    break;
    }
    if (list_empty(&thread.task_busy_list))
    cmdq_thread_disable(cmdq, thread);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t cmdq_irq_handler(int irq, void *dev)
    {
    struct cmdq *cmdq = dev;
    unsigned long irq_status, flags = 0L;
    int bit;
    irq_status = readl(cmdq.base + CMDQ_CURR_IRQ_STATUS) & cmdq.irq_mask;
    if (!(irq_status ^ cmdq.irq_mask))
    return IRQ_NONE;
    for_each_clear_bit(bit, &irq_status, cmdq.pdata.thread_nr) {
    struct cmdq_thread *thread = &cmdq.thread[bit];
    spin_lock_irqsave(&thread.chan.lock, flags);
    cmdq_thread_irq_handler(cmdq, thread);
    spin_unlock_irqrestore(&thread.chan.lock, flags);
    }
    pm_runtime_mark_last_busy(cmdq.mbox.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_runtime_resume(dev: *mut device) -> c_int {
    static int cmdq_runtime_resume(struct device *dev)
    {
    struct cmdq *cmdq = dev_get_drvdata(dev);
    int ret;
    ret = clk_bulk_enable(cmdq.pdata.gce_num, cmdq.clocks);
    if (ret)
    return ret;
    cmdq_gctl_value_toggle(cmdq, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_runtime_suspend(dev: *mut device) -> c_int {
    static int cmdq_runtime_suspend(struct device *dev)
    {
    struct cmdq *cmdq = dev_get_drvdata(dev);
    cmdq_gctl_value_toggle(cmdq, false);
    clk_bulk_disable(cmdq.pdata.gce_num, cmdq.clocks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_suspend(dev: *mut device) -> c_int {
    static int cmdq_suspend(struct device *dev)
    {
    struct cmdq *cmdq = dev_get_drvdata(dev);
    struct cmdq_thread *thread;
    int i;
    let mut task_running: bool = false;
    cmdq.suspended = true;
    for (i = 0; i < cmdq.pdata.thread_nr; i++) {
    thread = &cmdq.thread[i];
    if (!list_empty(&thread.task_busy_list)) {
    task_running = true;
    break;
    }
    }
    if (task_running)
    dev_warn(dev, "exist running task(s) in suspend\n");
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_resume(dev: *mut device) -> c_int {
    static int cmdq_resume(struct device *dev)
    {
    struct cmdq *cmdq = dev_get_drvdata(dev);
    WARN_ON(pm_runtime_force_resume(dev));
    cmdq.suspended = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_remove(pdev: *mut platform_device) {
    static void cmdq_remove(struct platform_device *pdev)
    {
    struct cmdq *cmdq = platform_get_drvdata(pdev);
    if (!IS_ENABLED(CONFIG_PM))
    cmdq_runtime_suspend(&pdev.dev);
    clk_bulk_unprepare(cmdq.pdata.gce_num, cmdq.clocks);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int cmdq_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct cmdq_pkt *pkt = (struct cmdq_pkt *)data;
    struct cmdq_thread *thread = (struct cmdq_thread *)chan.con_priv;
    struct cmdq *cmdq = dev_get_drvdata(chan.mbox.dev);
    struct cmdq_task *task;
    u32 gce_addr;
    dma_addr_t curr_pa, end_pa;
// Client should not flush new tasks if suspended.
    WARN_ON(cmdq.suspended);
    task = kzalloc_obj(*task, GFP_ATOMIC);
    if (!task)
    return -ENOMEM;
    task.cmdq = cmdq;
    INIT_LIST_HEAD(&task.list_entry);
    task.pa_base = pkt.pa_base;
    task.thread = thread;
    task.pkt = pkt;
    if (list_empty(&thread.task_busy_list)) {
//
// The thread reset will clear thread related register to 0,
// including pc, end, priority, irq, suspend and enable. Thus
// set CMDQ_THR_ENABLED to CMDQ_THR_ENABLE_TASK will enable
// thread and make it running.
//
    WARN_ON(cmdq_thread_reset(cmdq, thread) < 0);
    gce_addr = cmdq_convert_gce_addr(task.pa_base, cmdq.pdata);
    writel(gce_addr, thread.base + CMDQ_THR_CURR_ADDR);
    gce_addr = cmdq_convert_gce_addr(task.pa_base + pkt.cmd_buf_size, cmdq.pdata);
    writel(gce_addr, thread.base + CMDQ_THR_END_ADDR);
    writel(thread.priority, thread.base + CMDQ_THR_PRIORITY);
    writel(CMDQ_THR_IRQ_EN, thread.base + CMDQ_THR_IRQ_ENABLE);
    writel(CMDQ_THR_ENABLED, thread.base + CMDQ_THR_ENABLE_TASK);
    } else {
    WARN_ON(cmdq_thread_suspend(cmdq, thread) < 0);
    gce_addr = readl(thread.base + CMDQ_THR_CURR_ADDR);
    curr_pa = cmdq_revert_gce_addr(gce_addr, cmdq.pdata);
    gce_addr = readl(thread.base + CMDQ_THR_END_ADDR);
    end_pa = cmdq_revert_gce_addr(gce_addr, cmdq.pdata);
// check boundary
    if (curr_pa == end_pa - CMDQ_INST_SIZE ||
    curr_pa == end_pa) {
// set to this task directly
    gce_addr = cmdq_convert_gce_addr(task.pa_base, cmdq.pdata);
    writel(gce_addr, thread.base + CMDQ_THR_CURR_ADDR);
    } else {
    cmdq_task_insert_into_thread(task);
    smp_mb(); /* modify jump before enable thread */
    }
    gce_addr = cmdq_convert_gce_addr(task.pa_base + pkt.cmd_buf_size, cmdq.pdata);
    writel(gce_addr, thread.base + CMDQ_THR_END_ADDR);
    cmdq_thread_resume(thread);
    }
    list_move_tail(&task.list_entry, &thread.task_busy_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int cmdq_mbox_startup(struct mbox_chan *chan)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_mbox_shutdown(chan: *mut mbox_chan) {
    static void cmdq_mbox_shutdown(struct mbox_chan *chan)
    {
    struct cmdq_thread *thread = (struct cmdq_thread *)chan.con_priv;
    struct cmdq *cmdq = dev_get_drvdata(chan.mbox.dev);
    struct cmdq_task *task, *tmp;
    unsigned long flags;
    WARN_ON(pm_runtime_get_sync(cmdq.mbox.dev) < 0);
    spin_lock_irqsave(&thread.chan.lock, flags);
    if (list_empty(&thread.task_busy_list))
    goto done;
    WARN_ON(cmdq_thread_suspend(cmdq, thread) < 0);
// make sure executed tasks have success callback
    cmdq_thread_irq_handler(cmdq, thread);
    if (list_empty(&thread.task_busy_list))
    goto done;
    list_for_each_entry_safe(task, tmp, &thread.task_busy_list,
    list_entry) {
    cmdq_task_exec_done(task, -ECONNABORTED);
    kfree(task);
    }
    cmdq_thread_disable(cmdq, thread);
    done:
//
// The thread->task_busy_list empty means thread already disable. The
// cmdq_mbox_send_data() always reset thread which clear disable and
// suspend statue when first pkt send to channel, so there is no need
// to do any operation here, only unlock and leave.
//
    spin_unlock_irqrestore(&thread.chan.lock, flags);
    pm_runtime_mark_last_busy(cmdq.mbox.dev);
    pm_runtime_put_autosuspend(cmdq.mbox.dev);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_mbox_flush(chan: *mut mbox_chan, timeout: c_ulong) -> c_int {
    static int cmdq_mbox_flush(struct mbox_chan *chan, unsigned long timeout)
    {
    struct cmdq_thread *thread = (struct cmdq_thread *)chan.con_priv;
    struct cmdq_cb_data data;
    struct cmdq *cmdq = dev_get_drvdata(chan.mbox.dev);
    struct cmdq_task *task, *tmp;
    unsigned long flags;
    u32 enable;
    int ret;
    ret = pm_runtime_get_sync(cmdq.mbox.dev);
    if (ret < 0)
    return ret;
    spin_lock_irqsave(&thread.chan.lock, flags);
    if (list_empty(&thread.task_busy_list))
    goto out;
    WARN_ON(cmdq_thread_suspend(cmdq, thread) < 0);
    if (!cmdq_thread_is_in_wfe(thread))
    goto wait;
    list_for_each_entry_safe(task, tmp, &thread.task_busy_list,
    list_entry) {
    data.sta = -ECONNABORTED;
    data.pkt = task.pkt;
    mbox_chan_received_data(task.thread.chan, &data);
    list_del(&task.list_entry);
    kfree(task);
    }
    cmdq_thread_resume(thread);
    cmdq_thread_disable(cmdq, thread);
    out:
    spin_unlock_irqrestore(&thread.chan.lock, flags);
    pm_runtime_mark_last_busy(cmdq.mbox.dev);
    pm_runtime_put_autosuspend(cmdq.mbox.dev);
    return 0;
    wait:
    cmdq_thread_resume(thread);
    spin_unlock_irqrestore(&thread.chan.lock, flags);
    if (readl_poll_timeout_atomic(thread.base + CMDQ_THR_ENABLE_TASK,
    enable, enable == 0, 1, timeout)) {
    dev_err(cmdq.mbox.dev, "Fail to wait GCE thread 0x%x done\n",
    (u32)(thread.base - cmdq.base));
    return -EFAULT;
    }
    pm_runtime_mark_last_busy(cmdq.mbox.dev);
    pm_runtime_put_autosuspend(cmdq.mbox.dev);
    return 0;
    }
    static const struct mbox_chan_ops cmdq_mbox_chan_ops = {
    .send_data = cmdq_mbox_send_data,
    .startup = cmdq_mbox_startup,
    .shutdown = cmdq_mbox_shutdown,
    .flush = cmdq_mbox_flush,
    };
    static struct mbox_chan *cmdq_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *sp)
    {
    let mut ind: c_int = sp.args[0];
    struct cmdq_thread *thread;
    if (ind >= mbox.num_chans)
    return ERR_PTR(-EINVAL);
    thread = (struct cmdq_thread *)mbox.chans[ind].con_priv;
    thread.priority = sp.args[1];
    thread.chan = &mbox.chans[ind];
    return &mbox.chans[ind];
    }
#[no_mangle]
unsafe extern "C" fn cmdq_get_clocks(dev: *mut device, cmdq: *mut cmdq) -> c_int {
    static int cmdq_get_clocks(struct device *dev, struct cmdq *cmdq)
    {
    let mut gce_name: *const static char  const = "gce";
    struct device_node *parent = dev.of_node.parent;
    struct clk_bulk_data *clks;
    cmdq.clocks = devm_kcalloc(dev, cmdq.pdata.gce_num,
    sizeof(*cmdq.clocks), GFP_KERNEL);
    if (!cmdq.clocks)
    return -ENOMEM;
    if (cmdq.pdata.gce_num == 1) {
    clks = &cmdq.clocks[0];
    clks.id = gce_name;
    clks.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clks.clk))
    return dev_err_probe(dev, PTR_ERR(clks.clk),
    "failed to get gce clock\n");
    return 0;
    }
//
// If there is more than one GCE, get the clocks for the others too,
// as the clock of the main GCE must be enabled for additional IPs
// to be reachable.
//
    for_each_child_of_node_scoped(parent, node) {
    let mut alias_id: c_int = of_alias_get_id(node, gce_name);
    if (alias_id < 0 || alias_id >= cmdq.pdata.gce_num)
    continue;
    clks = &cmdq.clocks[alias_id];
    clks.id = devm_kasprintf(dev, GFP_KERNEL, "gce%d", alias_id);
    if (!clks.id)
    return -ENOMEM;
    clks.clk = of_clk_get(node, 0);
    if (IS_ERR(clks.clk))
    return dev_err_probe(dev, PTR_ERR(clks.clk),
    "failed to get gce%d clock\n", alias_id);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_probe(pdev: *mut platform_device) -> c_int {
    static int cmdq_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cmdq *cmdq;
    int err, i;
    cmdq = devm_kzalloc(dev, sizeof(*cmdq), GFP_KERNEL);
    if (!cmdq)
    return -ENOMEM;
    cmdq.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cmdq.base))
    return PTR_ERR(cmdq.base);
    cmdq.irq = platform_get_irq(pdev, 0);
    if (cmdq.irq < 0)
    return cmdq.irq;
    cmdq.pdata = device_get_match_data(dev);
    if (!cmdq.pdata) {
    dev_err(dev, "failed to get match data\n");
    return -EINVAL;
    }
    cmdq.irq_mask = GENMASK(cmdq.pdata.thread_nr - 1, 0);
    dev_dbg(dev, "cmdq device: addr:0x%p, va:0x%p, irq:%d\n",
    dev, cmdq.base, cmdq.irq);
    err = cmdq_get_clocks(dev, cmdq);
    if (err)
    return err;
    dma_set_coherent_mask(dev,
    DMA_BIT_MASK(sizeof(u32) * BITS_PER_BYTE + cmdq.pdata.shift));
    cmdq.mbox.dev = dev;
    cmdq.mbox.chans = devm_kcalloc(dev, cmdq.pdata.thread_nr,
    sizeof(*cmdq.mbox.chans), GFP_KERNEL);
    if (!cmdq.mbox.chans)
    return -ENOMEM;
    cmdq.mbox.num_chans = cmdq.pdata.thread_nr;
    cmdq.mbox.ops = &cmdq_mbox_chan_ops;
    cmdq.mbox.of_xlate = cmdq_xlate;
// make use of MBOX_TXDONE_BY_ACK
    cmdq.mbox.txdone_irq = false;
    cmdq.mbox.txdone_poll = false;
    cmdq.thread = devm_kcalloc(dev, cmdq.pdata.thread_nr,
    sizeof(*cmdq.thread), GFP_KERNEL);
    if (!cmdq.thread)
    return -ENOMEM;
    for (i = 0; i < cmdq.pdata.thread_nr; i++) {
    cmdq.thread[i].base = cmdq.base + CMDQ_THR_BASE +
    CMDQ_THR_SIZE * i;
    INIT_LIST_HEAD(&cmdq.thread[i].task_busy_list);
    cmdq.mbox.chans[i].con_priv = (void *)&cmdq.thread[i];
    }
    platform_set_drvdata(pdev, cmdq);
    WARN_ON(clk_bulk_prepare(cmdq.pdata.gce_num, cmdq.clocks));
    cmdq_init(cmdq);
    err = devm_request_irq(dev, cmdq.irq, cmdq_irq_handler, IRQF_SHARED,
    "mtk_cmdq", cmdq);
    if (err < 0) {
    dev_err(dev, "failed to register ISR (%d)\n", err);
    return err;
    }
// If Runtime PM is not available enable the clocks now.
    if (!IS_ENABLED(CONFIG_PM)) {
    err = cmdq_runtime_resume(dev);
    if (err)
    return err;
    }
    err = devm_pm_runtime_enable(dev);
    if (err)
    return err;
    pm_runtime_set_autosuspend_delay(dev, CMDQ_MBOX_AUTOSUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);
    err = devm_mbox_controller_register(dev, &cmdq.mbox);
    if (err < 0) {
    dev_err(dev, "failed to register mailbox: %d\n", err);
    return err;
    }
    return 0;
    }
    static const struct dev_pm_ops cmdq_pm_ops = {
    .suspend = cmdq_suspend,
    .resume = cmdq_resume,
    SET_RUNTIME_PM_OPS(cmdq_runtime_suspend,
    cmdq_runtime_resume, core::ptr::null_mut())
    };
    static const struct gce_plat gce_plat_mt6779 = {
    .thread_nr = 24,
    .shift = 3,
    .control_by_sw = false,
    .gce_num = 1
    };
    static const struct gce_plat gce_plat_mt8173 = {
    .thread_nr = 16,
    .shift = 0,
    .control_by_sw = false,
    .gce_num = 1
    };
    static const struct gce_plat gce_plat_mt8183 = {
    .thread_nr = 24,
    .shift = 0,
    .control_by_sw = false,
    .gce_num = 1
    };
    static const struct gce_plat gce_plat_mt8186 = {
    .thread_nr = 24,
    .shift = 3,
    .control_by_sw = true,
    .sw_ddr_en = true,
    .gce_num = 1
    };
    static const struct gce_plat gce_plat_mt8188 = {
    .thread_nr = 32,
    .shift = 3,
    .control_by_sw = true,
    .gce_num = 2
    };
    static const struct gce_plat gce_plat_mt8192 = {
    .thread_nr = 24,
    .shift = 3,
    .control_by_sw = true,
    .gce_num = 1
    };
    static const struct gce_plat gce_plat_mt8195 = {
    .thread_nr = 24,
    .shift = 3,
    .control_by_sw = true,
    .gce_num = 2
    };
    static const struct gce_plat gce_plat_mt8196 = {
    .thread_nr = 32,
    .shift = 3,
    .mminfra_offset = SZ_2G,
    .control_by_sw = true,
    .sw_ddr_en = true,
    .gce_vm = true,
    .gce_num = 2
    };
    static const struct of_device_id cmdq_of_ids[] = {
    {.compatible = "mediatek,mt6779-gce", .data = (void *)&gce_plat_mt6779},
    {.compatible = "mediatek,mt8173-gce", .data = (void *)&gce_plat_mt8173},
    {.compatible = "mediatek,mt8183-gce", .data = (void *)&gce_plat_mt8183},
    {.compatible = "mediatek,mt8186-gce", .data = (void *)&gce_plat_mt8186},
    {.compatible = "mediatek,mt8188-gce", .data = (void *)&gce_plat_mt8188},
    {.compatible = "mediatek,mt8192-gce", .data = (void *)&gce_plat_mt8192},
    {.compatible = "mediatek,mt8195-gce", .data = (void *)&gce_plat_mt8195},
    {.compatible = "mediatek,mt8196-gce", .data = (void *)&gce_plat_mt8196},
    {}
    };
    MODULE_DEVICE_TABLE(of, cmdq_of_ids);
    static struct platform_driver cmdq_drv = {
    .probe = cmdq_probe,
    .remove = cmdq_remove,
    .driver = {
    .name = "mtk_cmdq",
    .pm = &cmdq_pm_ops,
    .of_match_table = cmdq_of_ids,
    }
    };
#[no_mangle]
unsafe extern "C" fn cmdq_drv_init() -> int __init {
    static int __init cmdq_drv_init(void)
    {
    return platform_driver_register(&cmdq_drv);
    }
#[no_mangle]
unsafe extern "C" fn cmdq_drv_exit() -> void __exit {
    static void __exit cmdq_drv_exit(void)
    {
    platform_driver_unregister(&cmdq_drv);
    }
    subsys_initcall(cmdq_drv_init);
    module_exit(cmdq_drv_exit);
    MODULE_DESCRIPTION("Mediatek Command Queue(CMDQ) Mailbox driver");
    MODULE_LICENSE("GPL v2");
