//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_irq.c
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn fbnic_fw_msix_intr(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fbnic_fw_msix_intr(int __always_unused irq, void *data)
    {
    struct fbnic_dev *fbd = (struct fbnic_dev *)data;
    fbnic_mbx_poll(fbd);
    fbnic_wr32(fbd, FBNIC_INTR_MASK_CLEAR(0), 1u << FBNIC_FW_MSIX_ENTRY);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn __fbnic_fw_enable_mbx(fbd: *mut fbnic_dev, vector: c_int) -> c_int {
    static int __fbnic_fw_enable_mbx(struct fbnic_dev *fbd, int vector)
    {
    int err;
// Initialize mailbox and attempt to poll it into ready state
    fbnic_mbx_init(fbd);
    err = fbnic_mbx_poll_tx_ready(fbd);
    if (err) {
    dev_warn(fbd.dev, "FW mailbox did not enter ready state\n");
    return err;
    }
// Enable interrupt and unmask the vector
    enable_irq(vector);
    fbnic_wr32(fbd, FBNIC_INTR_MASK_CLEAR(0), 1u << FBNIC_FW_MSIX_ENTRY);
    return 0;
    }
//
// fbnic_fw_request_mbx - Configure and initialize Firmware Mailbox
// @fbd: Pointer to device to initialize
//
// This function will allocate the IRQ and then reinitialize the mailbox
// starting communication between the host and firmware.
//
// Return: non-zero on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fbnic_fw_request_mbx(fbd: *mut fbnic_dev) -> c_int {
    int fbnic_fw_request_mbx(struct fbnic_dev *fbd)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    int vector, err;
    WARN_ON(fbd.fw_msix_vector);
    vector = pci_irq_vector(pdev, FBNIC_FW_MSIX_ENTRY);
    if (vector < 0)
    return vector;
// Request the IRQ for FW Mailbox vector.
    err = request_threaded_irq(vector, core::ptr::null_mut(), &fbnic_fw_msix_intr,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    dev_name(fbd.dev), fbd);
    if (err)
    return err;
// Initialize mailbox and attempt to poll it into ready state
    err = __fbnic_fw_enable_mbx(fbd, vector);
    if (err)
    free_irq(vector, fbd);
    fbd.fw_msix_vector = vector;
    return err;
    }
//
// fbnic_fw_disable_mbx - Temporarily place mailbox in standby state
// @fbd: Pointer to device
//
// Shutdown the mailbox by notifying the firmware to stop sending us logs, mask
// and synchronize the IRQ, and then clean up the rings.
//
#[no_mangle]
unsafe extern "C" fn fbnic_fw_disable_mbx(fbd: *mut fbnic_dev) {
    static void fbnic_fw_disable_mbx(struct fbnic_dev *fbd)
    {
// Disable interrupt and synchronize the IRQ
    disable_irq(fbd.fw_msix_vector);
// Mask the vector
    fbnic_wr32(fbd, FBNIC_INTR_MASK_SET(0), 1u << FBNIC_FW_MSIX_ENTRY);
// Make sure disabling logs message is sent, must be done here to
// avoid risk of completing without a running interrupt.
//
    fbnic_mbx_flush_tx(fbd);
    fbnic_mbx_clean(fbd);
    }
//
// fbnic_fw_free_mbx - Disable mailbox and place it in standby state
// @fbd: Pointer to device to disable
//
// This function will disable the mailbox interrupt, free any messages still
// in the mailbox and place it into a disabled state. The firmware is
// expected to see the update and assume that the host is in the reset state.
//
#[no_mangle]
pub unsafe extern "C" fn fbnic_fw_free_mbx(fbd: *mut fbnic_dev) {
    void fbnic_fw_free_mbx(struct fbnic_dev *fbd)
    {
// Vector has already been freed
    if (!fbd.fw_msix_vector)
    return;
    fbnic_fw_disable_mbx(fbd);
// Free the vector
    free_irq(fbd.fw_msix_vector, fbd);
    fbd.fw_msix_vector = 0;
    }
#[no_mangle]
unsafe extern "C" fn fbnic_mac_msix_intr(irq: int __always_unused, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fbnic_mac_msix_intr(int __always_unused irq, void *data)
    {
    struct fbnic_dev *fbd = data;
    struct fbnic_net *fbn;
    if (fbd.mac.get_link_event(fbd) == FBNIC_LINK_EVENT_NONE) {
    fbnic_wr32(fbd, FBNIC_INTR_MASK_CLEAR(0),
    1u << FBNIC_PCS_MSIX_ENTRY);
    return IRQ_HANDLED;
    }
    fbn = netdev_priv(fbd.netdev);
// Record link down events
    if (!fbd.mac.get_link(fbd, fbn.aui, fbn.fec))
    phylink_pcs_change(fbn.pcs, false);
    return IRQ_HANDLED;
    }
//
// fbnic_mac_request_irq - Configure the MAC to enable it to advertise link
// @fbd: Pointer to device to initialize
//
// This function provides basic bringup for the MAC/PHY IRQ. For now the IRQ
// will remain disabled until we start the MAC/PCS/PHY logic via phylink.
//
// Return: non-zero on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fbnic_mac_request_irq(fbd: *mut fbnic_dev) -> c_int {
    int fbnic_mac_request_irq(struct fbnic_dev *fbd)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    int vector, err;
    WARN_ON(fbd.mac_msix_vector);
    vector = pci_irq_vector(pdev, FBNIC_PCS_MSIX_ENTRY);
    if (vector < 0)
    return vector;
// Request the IRQ for PCS link vector.
// Map PCS cause to it, and unmask it
//
    err = request_irq(vector, &fbnic_mac_msix_intr, 0,
    fbd.netdev.name, fbd);
    if (err)
    return err;
// Map and enable interrupt, unmask vector after link is configured
    fbnic_wr32(fbd, FBNIC_INTR_MSIX_CTRL(FBNIC_INTR_MSIX_CTRL_PCS_IDX),
    FBNIC_PCS_MSIX_ENTRY | FBNIC_INTR_MSIX_CTRL_ENABLE);
    fbnic_wr32(fbd, FBNIC_INTR_MSIX_CTRL(FBNIC_INTR_MSIX_CTRL_RXB_IDX), 0);
    fbd.mac_msix_vector = vector;
    return 0;
    }
//
// fbnic_mac_free_irq - Teardown the MAC IRQ to prepare for stopping
// @fbd: Pointer to device that is stopping
//
// This function undoes the work done in fbnic_mac_request_irq and prepares
// the device to no longer receive traffic on the host interface.
//
#[no_mangle]
pub unsafe extern "C" fn fbnic_mac_free_irq(fbd: *mut fbnic_dev) {
    void fbnic_mac_free_irq(struct fbnic_dev *fbd)
    {
// Vector has already been freed
    if (!fbd.mac_msix_vector)
    return;
// Disable interrupt
    fbnic_wr32(fbd, FBNIC_INTR_MSIX_CTRL(FBNIC_INTR_MSIX_CTRL_PCS_IDX),
    FBNIC_PCS_MSIX_ENTRY);
    fbnic_wrfl(fbd);
// Synchronize IRQ to prevent race that would unmask vector
    synchronize_irq(fbd.mac_msix_vector);
// Mask the vector
    fbnic_wr32(fbd, FBNIC_INTR_MASK_SET(0), 1u << FBNIC_PCS_MSIX_ENTRY);
// Free the vector
    free_irq(fbd.mac_msix_vector, fbd);
    fbd.mac_msix_vector = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_synchronize_irq(fbd: *mut fbnic_dev, nr: c_int) {
    void fbnic_synchronize_irq(struct fbnic_dev *fbd, int nr)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    let mut irq: c_int = pci_irq_vector(pdev, nr);
    if (irq < 0)
    return;
    synchronize_irq(irq);
    }
    int fbnic_request_irq(struct fbnic_dev *fbd, int nr, irq_handler_t handler,
    unsigned long flags, const char *name, void *data)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    let mut irq: c_int = pci_irq_vector(pdev, nr);
    if (irq < 0)
    return irq;
    return request_irq(irq, handler, flags, name, data);
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_free_irq(fbd: *mut fbnic_dev, nr: c_int, data: *mut c_void) {
    void fbnic_free_irq(struct fbnic_dev *fbd, int nr, void *data)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    let mut irq: c_int = pci_irq_vector(pdev, nr);
    if (irq < 0)
    return;
    free_irq(irq, data);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_msix_test_data {
    pub fbd: *mut fbnic_dev,
    pub test_msix_status: [c_ulong; BITS_TO_LONGS(FBNIC_MAX_MSIX_VECS)],
    pub irq_vector: [c_int; FBNIC_MAX_MSIX_VECS],
}

#[no_mangle]
unsafe extern "C" fn fbnic_irq_test(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fbnic_irq_test(int irq, void *data)
    {
    struct fbnic_msix_test_data *test_data = data;
    struct fbnic_dev *fbd = test_data.fbd;
    int i;
    for (i = fbd.num_irqs; i--;) {
    if (test_data.irq_vector[i] == irq) {
    set_bit(i, test_data.test_msix_status);
    break;
    }
    }
    return IRQ_HANDLED;
    }
//
// fbnic_msix_test - Verify behavior of NIC interrupts
// @fbd: device to test
//
// This function is meant to test the global interrupt registers and the
// PCIe IP MSI-X functionality. It essentially goes through and tests
// various combinations of the set, clear, and mask bits in order to
// verify the behavior is as we expect it to be from the driver.
//
// Return: See enum fbnic_msix_self_test_codes
//
#[no_mangle]
pub unsafe extern "C" fn fbnic_msix_test(fbd: *mut fbnic_dev) -> enum fbnic_msix_self_test_codes {
    enum fbnic_msix_self_test_codes fbnic_msix_test(struct fbnic_dev *fbd)
    {
    let mut result: enum fbnic_msix_self_test_codes = FBNIC_TEST_MSIX_SUCCESS;
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    struct fbnic_msix_test_data *test_data;
    let mut mask: u32 = 0;
    int i;
// Allocate bitmap and IRQ vector table
    test_data = kzalloc_obj(*test_data);
// memory allocation failure
    if (!test_data)
    return FBNIC_TEST_MSIX_NOMEM;
// Initialize test data
    test_data.fbd = fbd;
    for (i = FBNIC_NON_NAPI_VECTORS; i < fbd.num_irqs; i++) {
// Add IRQ to vector table so it can be found
    test_data.irq_vector[i] = pci_irq_vector(pdev, i);
// Enable the interrupt
    if (!fbnic_request_irq(fbd, i, fbnic_irq_test, 0,
    fbd.netdev.name, test_data))
    continue;
    while (i-- > FBNIC_NON_NAPI_VECTORS)
    fbnic_free_irq(fbd, i, test_data);
    kfree(test_data);
// IRQ request failure
    return FBNIC_TEST_MSIX_IRQ_REQ_FAIL;
    }
// Test each bit individually
    for (i = FBNIC_NON_NAPI_VECTORS; i < fbd.num_irqs; i++) {
    mask = 1U << (i % 32);
// Start with mask set and interrupt cleared
    fbnic_wr32(fbd, FBNIC_INTR_MASK_SET(i / 32), mask);
    fbnic_wrfl(fbd);
    fbnic_wr32(fbd, FBNIC_INTR_CLEAR(i / 32), mask);
    fbnic_wrfl(fbd);
// masking failure to prevent interrupt
    result = FBNIC_TEST_MSIX_MASK;
    fbnic_wr32(fbd, FBNIC_INTR_SET(i / 32), mask);
    fbnic_wrfl(fbd);
    usleep_range(10000, 11000);
    if (test_bit(i, test_data.test_msix_status))
    break;
// unmasking failure w/ sw status set
    result = FBNIC_TEST_MSIX_UNMASK;
    fbnic_wr32(fbd, FBNIC_INTR_MASK_CLEAR(i / 32), mask);
    fbnic_wrfl(fbd);
    usleep_range(10000, 11000);
    if (!test_bit(i, test_data.test_msix_status))
    break;
// interrupt when clearing mask
    result = FBNIC_TEST_MSIX_IRQ_CLEAR;
    clear_bit(i, test_data.test_msix_status);
    fbnic_wr32(fbd, FBNIC_INTR_MASK_CLEAR(i / 32), mask);
    fbnic_wrfl(fbd);
    usleep_range(10000, 11000);
    if (test_bit(i, test_data.test_msix_status))
    break;
// interrupt not triggering when not masked
    result = FBNIC_TEST_MSIX_NO_INTERRUPT;
    fbnic_wr32(fbd, FBNIC_INTR_SET(i / 32), mask);
    fbnic_wrfl(fbd);
    usleep_range(10000, 11000);
    if (!test_bit(i, test_data.test_msix_status))
    break;
// status not cleared, or mask not set
    result = FBNIC_TEST_MSIX_NO_CLEAR_OR_MASK;
    if (mask & fbnic_rd32(fbd, FBNIC_INTR_STATUS(i / 32)))
    break;
    if (!(mask & fbnic_rd32(fbd, FBNIC_INTR_MASK(i / 32))))
    break;
// Result = 0 - Success
    result = FBNIC_TEST_MSIX_SUCCESS;
    clear_bit(i, test_data.test_msix_status);
    }
    if (i < fbd.num_irqs) {
    fbnic_wr32(fbd, FBNIC_INTR_MASK_SET(i / 32), mask);
    fbnic_wrfl(fbd);
    fbnic_wr32(fbd, FBNIC_INTR_CLEAR(i / 32), mask);
    fbnic_wrfl(fbd);
    clear_bit(i, test_data.test_msix_status);
    }
    for (i = FBNIC_NON_NAPI_VECTORS; i < fbd.num_irqs; i++) {
// Test for bits set after testing
    if (test_bit(i, test_data.test_msix_status))
    result = FBNIC_TEST_MSIX_BITS_SET_AFTER_TEST;
// Free IRQ
    fbnic_free_irq(fbd, i, test_data);
    }
    kfree(test_data);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_napi_name_irqs(fbd: *mut fbnic_dev) {
    void fbnic_napi_name_irqs(struct fbnic_dev *fbd)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(fbd.napi_irq); i++)
    snprintf(fbd.napi_irq[i].name,
    sizeof(fbd.napi_irq[i].name),
    "%s-TxRx-%u", fbd.netdev.name, i);
    }
    int fbnic_napi_request_irq(struct fbnic_dev *fbd,
    struct fbnic_napi_vector *nv)
    {
    struct fbnic_net *fbn = netdev_priv(fbd.netdev);
    let mut i: c_int = fbnic_napi_idx(nv);
    int err;
    if (!fbd.napi_irq[i].users) {
    err = fbnic_request_irq(fbd, nv.v_idx,
    fbnic_msix_clean_rings,	0,
    fbd.napi_irq[i].name,
    &fbn.napi[i]);
    if (err)
    return err;
    }
    fbd.napi_irq[i].users++;
    return 0;
    }
    void fbnic_napi_free_irq(struct fbnic_dev *fbd,
    struct fbnic_napi_vector *nv)
    {
    struct fbnic_net *fbn = netdev_priv(fbd.netdev);
    let mut i: c_int = fbnic_napi_idx(nv);
    if (--fbd.napi_irq[i].users)
    return;
    fbnic_free_irq(fbd, nv.v_idx, &fbn.napi[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_free_irqs(fbd: *mut fbnic_dev) {
    void fbnic_free_irqs(struct fbnic_dev *fbd)
    {
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    fbd.num_irqs = 0;
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_alloc_irqs(fbd: *mut fbnic_dev) -> c_int {
    int fbnic_alloc_irqs(struct fbnic_dev *fbd)
    {
    let mut wanted_irqs: c_uint = FBNIC_NON_NAPI_VECTORS;
    struct pci_dev *pdev = to_pci_dev(fbd.dev);
    int num_irqs;
    wanted_irqs += min_t(unsigned int, num_online_cpus(), FBNIC_MAX_RXQS);
    num_irqs = pci_alloc_irq_vectors(pdev, FBNIC_NON_NAPI_VECTORS + 1,
    wanted_irqs, PCI_IRQ_MSIX);
    if (num_irqs < 0) {
    dev_err(fbd.dev, "Failed to allocate MSI-X entries\n");
    return num_irqs;
    }
    if (num_irqs < wanted_irqs)
    dev_warn(fbd.dev, "Allocated %d IRQs, expected %d\n",
    num_irqs, wanted_irqs);
    fbd.num_irqs = num_irqs;
    return 0;
    }
