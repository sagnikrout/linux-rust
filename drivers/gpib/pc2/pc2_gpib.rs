//! Automatically rewritten from C to Rust
//! Source: drivers/gpib/pc2/pc2_gpib.c
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
// copyright            : (C) 2001, 2002 by Frank Mori Hess
//

// struct which defines private_data for pc2 driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc2_priv {
    pub nec7210_priv: nec7210_priv,
    pub irq: c_uint,
// io address that clears interrupt for pc2a (0x2f0 + irq)
    pub clear_intr_addr: c_uint,
}

// pc2 uses 8 consecutive io addresses
    let mut pc2_iosize: static int = 8;
    let mut pc2a_iosize: static int = 8;
    let mut pc2_2a_iosize: static int = 16;
// offset between io addresses of successive nec7210 registers
    let mut pc2a_reg_offset: static int = 0x400;
    let mut pc2_reg_offset: static int = 1;
// interrupt service routine
    static irqreturn_t pc2_interrupt(int irq, void *arg);
    static irqreturn_t pc2a_interrupt(int irq, void *arg);
// pc2 specific registers and bits
// interrupt clear register address
    let mut pc2a_clear_intr_iobase: static int = 0x2f0;
#[no_mangle]
pub unsafe extern "C" fn CLEAR_INTR_REG(irq: c_uint) -> c_uint {
    static inline unsigned int CLEAR_INTR_REG(unsigned int irq)
    {
    return pc2a_clear_intr_iobase + irq;
    }
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("GPIB driver for PC2/PC2a and compatible devices");
//
// GPIB interrupt service routines
//
#[no_mangle]
pub unsafe extern "C" fn pc2_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    irqreturn_t pc2_interrupt(int irq, void *arg)
    {
    struct gpib_board *board = arg;
    struct pc2_priv *priv = board.private_data;
    unsigned long flags;
    irqreturn_t retval;
    spin_lock_irqsave(&board.spinlock, flags);
    retval = nec7210_interrupt(board, &priv.nec7210_priv);
    spin_unlock_irqrestore(&board.spinlock, flags);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn pc2a_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    irqreturn_t pc2a_interrupt(int irq, void *arg)
    {
    struct gpib_board *board = arg;
    struct pc2_priv *priv = board.private_data;
    int status1, status2;
    unsigned long flags;
    irqreturn_t retval;
    spin_lock_irqsave(&board.spinlock, flags);
// read interrupt status (also clears status)
    status1 = read_byte(&priv.nec7210_priv, ISR1);
    status2 = read_byte(&priv.nec7210_priv, ISR2);
// clear interrupt circuit
    if (priv.irq)
    outb(0xff, CLEAR_INTR_REG(priv.irq));
    retval = nec7210_interrupt_have_status(board, &priv.nec7210_priv, status1, status2);
    spin_unlock_irqrestore(&board.spinlock, flags);
    return retval;
    }
// wrappers for interface functions
    static int pc2_read(struct gpib_board *board, u8 *buffer, size_t length, int *end,
    size_t *bytes_read)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_read(board, &priv.nec7210_priv, buffer, length, end, bytes_read);
    }
    static int pc2_write(struct gpib_board *board, u8 *buffer, size_t length, int send_eoi,
    size_t *bytes_written)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_write(board, &priv.nec7210_priv, buffer, length, send_eoi, bytes_written);
    }
    static int pc2_command(struct gpib_board *board, u8 *buffer,
    size_t length, size_t *bytes_written)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_command(board, &priv.nec7210_priv, buffer, length, bytes_written);
    }
#[no_mangle]
unsafe extern "C" fn pc2_take_control(board: *mut gpib_board, synchronous: c_int) -> c_int {
    static int pc2_take_control(struct gpib_board *board, int synchronous)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_take_control(board, &priv.nec7210_priv, synchronous);
    }
#[no_mangle]
unsafe extern "C" fn pc2_go_to_standby(board: *mut gpib_board) -> c_int {
    static int pc2_go_to_standby(struct gpib_board *board)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_go_to_standby(board, &priv.nec7210_priv);
    }
#[no_mangle]
unsafe extern "C" fn pc2_request_system_control(board: *mut gpib_board, request_control: c_int) -> c_int {
    static int pc2_request_system_control(struct gpib_board *board, int request_control)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_request_system_control(board, &priv.nec7210_priv, request_control);
    }
#[no_mangle]
unsafe extern "C" fn pc2_interface_clear(board: *mut gpib_board, assert: c_int) {
    static void pc2_interface_clear(struct gpib_board *board, int assert)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_interface_clear(board, &priv.nec7210_priv, assert);
    }
#[no_mangle]
unsafe extern "C" fn pc2_remote_enable(board: *mut gpib_board, enable: c_int) {
    static void pc2_remote_enable(struct gpib_board *board, int enable)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_remote_enable(board, &priv.nec7210_priv, enable);
    }
#[no_mangle]
unsafe extern "C" fn pc2_enable_eos(board: *mut gpib_board, eos_byte: u8, compare_8_bits: c_int) -> c_int {
    static int pc2_enable_eos(struct gpib_board *board, u8 eos_byte, int compare_8_bits)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_enable_eos(board, &priv.nec7210_priv, eos_byte, compare_8_bits);
    }
#[no_mangle]
unsafe extern "C" fn pc2_disable_eos(board: *mut gpib_board) {
    static void pc2_disable_eos(struct gpib_board *board)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_disable_eos(board, &priv.nec7210_priv);
    }
#[no_mangle]
unsafe extern "C" fn pc2_update_status(board: *mut gpib_board, clear_mask: c_uint) -> c_uint {
    static unsigned int pc2_update_status(struct gpib_board *board, unsigned int clear_mask)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_update_status(board, &priv.nec7210_priv, clear_mask);
    }
#[no_mangle]
unsafe extern "C" fn pc2_primary_address(board: *mut gpib_board, address: c_uint) -> c_int {
    static int pc2_primary_address(struct gpib_board *board, unsigned int address)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_primary_address(board, &priv.nec7210_priv, address);
    }
#[no_mangle]
unsafe extern "C" fn pc2_secondary_address(board: *mut gpib_board, address: c_uint, enable: c_int) -> c_int {
    static int pc2_secondary_address(struct gpib_board *board, unsigned int address, int enable)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_secondary_address(board, &priv.nec7210_priv, address, enable);
    }
#[no_mangle]
unsafe extern "C" fn pc2_parallel_poll(board: *mut gpib_board, result: *mut u8) -> c_int {
    static int pc2_parallel_poll(struct gpib_board *board, u8 *result)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_parallel_poll(board, &priv.nec7210_priv, result);
    }
#[no_mangle]
unsafe extern "C" fn pc2_parallel_poll_configure(board: *mut gpib_board, config: u8) {
    static void pc2_parallel_poll_configure(struct gpib_board *board, u8 config)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_parallel_poll_configure(board, &priv.nec7210_priv, config);
    }
#[no_mangle]
unsafe extern "C" fn pc2_parallel_poll_response(board: *mut gpib_board, ist: c_int) {
    static void pc2_parallel_poll_response(struct gpib_board *board, int ist)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_parallel_poll_response(board, &priv.nec7210_priv, ist);
    }
#[no_mangle]
unsafe extern "C" fn pc2_serial_poll_response(board: *mut gpib_board, status: u8) {
    static void pc2_serial_poll_response(struct gpib_board *board, u8 status)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_serial_poll_response(board, &priv.nec7210_priv, status);
    }
#[no_mangle]
unsafe extern "C" fn pc2_serial_poll_status(board: *mut gpib_board) -> u8 {
    static u8 pc2_serial_poll_status(struct gpib_board *board)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_serial_poll_status(board, &priv.nec7210_priv);
    }
#[no_mangle]
unsafe extern "C" fn pc2_t1_delay(board: *mut gpib_board, nano_sec: c_uint) -> c_int {
    static int pc2_t1_delay(struct gpib_board *board, unsigned int nano_sec)
    {
    struct pc2_priv *priv = board.private_data;
    return nec7210_t1_delay(board, &priv.nec7210_priv, nano_sec);
    }
#[no_mangle]
unsafe extern "C" fn pc2_return_to_local(board: *mut gpib_board) {
    static void pc2_return_to_local(struct gpib_board *board)
    {
    struct pc2_priv *priv = board.private_data;
    nec7210_return_to_local(board, &priv.nec7210_priv);
    }
#[no_mangle]
unsafe extern "C" fn allocate_private(board: *mut gpib_board) -> c_int {
    static int allocate_private(struct gpib_board *board)
    {
    struct pc2_priv *priv;
    board.private_data = kzalloc_obj(struct pc2_priv);
    if (!board.private_data)
    return -ENOMEM;
    priv = board.private_data;
    init_nec7210_private(&priv.nec7210_priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_private(board: *mut gpib_board) {
    static void free_private(struct gpib_board *board)
    {
    kfree(board.private_data);
    board.private_data = core::ptr::null_mut();
    }
    static int pc2_generic_attach(struct gpib_board *board, const struct gpib_board_config *config,
    enum nec7210_chipset chipset)
    {
    struct pc2_priv *pc2_priv;
    struct nec7210_priv *nec_priv;
    int retval;
    board.status = 0;
    retval = allocate_private(board);
    if (retval)
    return retval;
    pc2_priv = board.private_data;
    nec_priv = &pc2_priv.nec7210_priv;
    nec_priv.read_byte = nec7210_ioport_read_byte;
    nec_priv.write_byte = nec7210_ioport_write_byte;
    nec_priv.type = chipset;

//
// board->dev hasn't been initialized, so forget about DMA until this driver
// is adapted to use isa_register_driver.
//
    if (config.ibdma)
// driver needs to be adapted to use isa_register_driver to get a struct device
    dev_err(board.gpib_dev, "DMA disabled for pc2 gpib");

    if (config.ibdma) {
    nec_priv.dma_buffer_length = 0x1000;
    nec_priv.dma_buffer = dma_alloc_coherent(board.dev,
    nec_priv.dma_buffer_length, &
    nec_priv.dma_buffer_addr, GFP_ATOMIC);
    if (!nec_priv.dma_buffer)
    return -ENOMEM;
// request isa dma channel
    if (request_dma(config.ibdma, "pc2")) {
    dev_err(board.gpib_dev, "can't request DMA %d\n", config.ibdma);
    return -1;
    }
    nec_priv.dma_channel = config.ibdma;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pc2_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int pc2_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    let mut isr_flags: c_int = 0;
    struct pc2_priv *pc2_priv;
    struct nec7210_priv *nec_priv;
    int retval;
    retval = pc2_generic_attach(board, config, NEC7210);
    if (retval)
    return retval;
    pc2_priv = board.private_data;
    nec_priv = &pc2_priv.nec7210_priv;
    nec_priv.offset = pc2_reg_offset;
    if (!request_region(config.ibbase, pc2_iosize, "pc2")) {
    dev_err(board.gpib_dev, "ioports are already in use\n");
    return -EBUSY;
    }
    nec_priv.iobase = config.ibbase;
    nec7210_board_reset(nec_priv, board);
// install interrupt handler
    if (config.ibirq) {
    if (request_irq(config.ibirq, pc2_interrupt, isr_flags, "pc2", board))	{
    dev_err(board.gpib_dev, "can't request IRQ %d\n", config.ibirq);
    return -EBUSY;
    }
    }
    pc2_priv.irq = config.ibirq;
// poll so we can detect assertion of ATN
    if (gpib_request_pseudo_irq(board, pc2_interrupt)) {
    dev_err(board.gpib_dev, "failed to allocate pseudo_irq\n");
    return -1;
    }
// set internal counter register for 8 MHz input clock
    write_byte(nec_priv, ICR | 8, AUXMR);
    nec7210_board_online(nec_priv, board);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pc2_detach(board: *mut gpib_board) {
    static void pc2_detach(struct gpib_board *board)
    {
    struct pc2_priv *pc2_priv = board.private_data;
    struct nec7210_priv *nec_priv;
    if (pc2_priv) {
    nec_priv = &pc2_priv.nec7210_priv;

    if (nec_priv.dma_channel)
    free_dma(nec_priv.dma_channel);

    gpib_free_pseudo_irq(board);
    if (pc2_priv.irq)
    free_irq(pc2_priv.irq, board);
    if (nec_priv.iobase) {
    nec7210_board_reset(nec_priv, board);
    release_region(nec_priv.iobase, pc2_iosize);
    }
    if (nec_priv.dma_buffer) {
    dma_free_coherent(board.dev, nec_priv.dma_buffer_length,
    nec_priv.dma_buffer, nec_priv.dma_buffer_addr);
    nec_priv.dma_buffer = core::ptr::null_mut();
    }
    }
    free_private(board);
    }
    static int pc2a_common_attach(struct gpib_board *board, const struct gpib_board_config *config,
    unsigned int num_registers, enum nec7210_chipset chipset)
    {
    unsigned int i, j;
    struct pc2_priv *pc2_priv;
    struct nec7210_priv *nec_priv;
    int retval;
    retval = pc2_generic_attach(board, config, chipset);
    if (retval)
    return retval;
    pc2_priv = board.private_data;
    nec_priv = &pc2_priv.nec7210_priv;
    nec_priv.offset = pc2a_reg_offset;
    switch (config.ibbase) {
    case 0x02e1:
    case 0x22e1:
    case 0x42e1:
    case 0x62e1:
    break;
    default:
    dev_err(board.gpib_dev, "PCIIa base range invalid, must be one of 0x[0246]2e1, but is 0x%x\n",
    config.ibbase);
    return -1;
    }
    if (config.ibirq) {
    if (config.ibirq < 2 || config.ibirq > 7) {
    dev_err(board.gpib_dev, "illegal interrupt level %i\n",
    config.ibirq);
    return -1;
    }
    } else	{
    dev_err(board.gpib_dev, "interrupt disabled, using polling mode (slow)\n");
    }

    let mut err: c_uint = 0;
    for (i = 0; i < num_registers; i++) {
    if (check_region(config.ibbase + i * pc2a_reg_offset, 1))
    err++;
    }
    if (config.ibirq && check_region(pc2a_clear_intr_iobase + config.ibirq, 1))
    err++;
    if (err) {
    dev_err(board.gpib_dev, "ioports are already in use");
    return -EBUSY;
    }

    for (i = 0; i < num_registers; i++) {
    if (!request_region(config.ibbase +
    i * pc2a_reg_offset, 1, "pc2a")) {
    dev_err(board.gpib_dev, "ioports are already in use");
    for (j = 0; j < i; j++)
    release_region(config.ibbase +
    j * pc2a_reg_offset, 1);
    return -EBUSY;
    }
    }
    nec_priv.iobase = config.ibbase;
    if (config.ibirq) {
    if (!request_region(pc2a_clear_intr_iobase + config.ibirq, 1, "pc2a"))  {
    dev_err(board.gpib_dev, "ioports are already in use");
    return -1;
    }
    pc2_priv.clear_intr_addr = pc2a_clear_intr_iobase + config.ibirq;
    if (request_irq(config.ibirq, pc2a_interrupt, 0, "pc2a", board)) {
    dev_err(board.gpib_dev, "can't request IRQ %d\n", config.ibirq);
    return -EBUSY;
    }
    }
    pc2_priv.irq = config.ibirq;
// poll so we can detect assertion of ATN
    if (gpib_request_pseudo_irq(board, pc2_interrupt)) {
    dev_err(board.gpib_dev, "failed to allocate pseudo_irq\n");
    return -1;
    }
// make sure interrupt is clear
    if (pc2_priv.irq)
    outb(0xff, CLEAR_INTR_REG(pc2_priv.irq));
    nec7210_board_reset(nec_priv, board);
// set internal counter register for 8 MHz input clock
    write_byte(nec_priv, ICR | 8, AUXMR);
    nec7210_board_online(nec_priv, board);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pc2a_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int pc2a_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    return pc2a_common_attach(board, config, pc2a_iosize, NEC7210);
    }
#[no_mangle]
unsafe extern "C" fn pc2a_cb7210_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int pc2a_cb7210_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    return pc2a_common_attach(board, config, pc2a_iosize, CB7210);
    }
#[no_mangle]
unsafe extern "C" fn pc2_2a_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int pc2_2a_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    return pc2a_common_attach(board, config, pc2_2a_iosize, NAT4882);
    }
#[no_mangle]
unsafe extern "C" fn pc2a_common_detach(board: *mut gpib_board, num_registers: c_uint) {
    static void pc2a_common_detach(struct gpib_board *board, unsigned int num_registers)
    {
    int i;
    struct pc2_priv *pc2_priv = board.private_data;
    struct nec7210_priv *nec_priv;
    if (pc2_priv) {
    nec_priv = &pc2_priv.nec7210_priv;

    if (nec_priv.dma_channel)
    free_dma(nec_priv.dma_channel);

    gpib_free_pseudo_irq(board);
    if (pc2_priv.irq)
    free_irq(pc2_priv.irq, board);
    if (nec_priv.iobase) {
    nec7210_board_reset(nec_priv, board);
    for (i = 0; i < num_registers; i++)
    release_region(nec_priv.iobase +
    i * pc2a_reg_offset, 1);
    }
    if (pc2_priv.clear_intr_addr)
    release_region(pc2_priv.clear_intr_addr, 1);
    if (nec_priv.dma_buffer) {
    dma_free_coherent(board.dev, nec_priv.dma_buffer_length,
    nec_priv.dma_buffer,
    nec_priv.dma_buffer_addr);
    nec_priv.dma_buffer = core::ptr::null_mut();
    }
    }
    free_private(board);
    }
#[no_mangle]
unsafe extern "C" fn pc2a_detach(board: *mut gpib_board) {
    static void pc2a_detach(struct gpib_board *board)
    {
    pc2a_common_detach(board, pc2a_iosize);
    }
#[no_mangle]
unsafe extern "C" fn pc2_2a_detach(board: *mut gpib_board) {
    static void pc2_2a_detach(struct gpib_board *board)
    {
    pc2a_common_detach(board, pc2_2a_iosize);
    }
    static struct gpib_interface pc2_interface = {
    .name =	"pcII",
    .attach =	pc2_attach,
    .detach =	pc2_detach,
    .read =	pc2_read,
    .write =	pc2_write,
    .command =	pc2_command,
    .take_control =	pc2_take_control,
    .go_to_standby =	pc2_go_to_standby,
    .request_system_control =	pc2_request_system_control,
    .interface_clear =	pc2_interface_clear,
    .remote_enable =	pc2_remote_enable,
    .enable_eos =	pc2_enable_eos,
    .disable_eos =	pc2_disable_eos,
    .parallel_poll =	pc2_parallel_poll,
    .parallel_poll_configure =	pc2_parallel_poll_configure,
    .parallel_poll_response =	pc2_parallel_poll_response,
    .local_parallel_poll_mode = core::ptr::null_mut(), // XXX
    .line_status =	core::ptr::null_mut(),
    .update_status =	pc2_update_status,
    .primary_address =	pc2_primary_address,
    .secondary_address =	pc2_secondary_address,
    .serial_poll_response =	pc2_serial_poll_response,
    .serial_poll_status =	pc2_serial_poll_status,
    .t1_delay = pc2_t1_delay,
    .return_to_local = pc2_return_to_local,
    };
    static struct gpib_interface pc2a_interface = {
    .name =	"pcIIa",
    .attach =	pc2a_attach,
    .detach =	pc2a_detach,
    .read =	pc2_read,
    .write =	pc2_write,
    .command =	pc2_command,
    .take_control =	pc2_take_control,
    .go_to_standby =	pc2_go_to_standby,
    .request_system_control =	pc2_request_system_control,
    .interface_clear =	pc2_interface_clear,
    .remote_enable =	pc2_remote_enable,
    .enable_eos =	pc2_enable_eos,
    .disable_eos =	pc2_disable_eos,
    .parallel_poll =	pc2_parallel_poll,
    .parallel_poll_configure =	pc2_parallel_poll_configure,
    .parallel_poll_response =	pc2_parallel_poll_response,
    .local_parallel_poll_mode = core::ptr::null_mut(), // XXX
    .line_status =	core::ptr::null_mut(),
    .update_status =	pc2_update_status,
    .primary_address =	pc2_primary_address,
    .secondary_address =	pc2_secondary_address,
    .serial_poll_response =	pc2_serial_poll_response,
    .serial_poll_status =	pc2_serial_poll_status,
    .t1_delay = pc2_t1_delay,
    .return_to_local = pc2_return_to_local,
    };
    static struct gpib_interface pc2a_cb7210_interface = {
    .name =	"pcIIa_cb7210",
    .attach =	pc2a_cb7210_attach,
    .detach =	pc2a_detach,
    .read =	pc2_read,
    .write =	pc2_write,
    .command =	pc2_command,
    .take_control =	pc2_take_control,
    .go_to_standby =	pc2_go_to_standby,
    .request_system_control =	pc2_request_system_control,
    .interface_clear =	pc2_interface_clear,
    .remote_enable =	pc2_remote_enable,
    .enable_eos =	pc2_enable_eos,
    .disable_eos =	pc2_disable_eos,
    .parallel_poll =	pc2_parallel_poll,
    .parallel_poll_configure =	pc2_parallel_poll_configure,
    .parallel_poll_response =	pc2_parallel_poll_response,
    .local_parallel_poll_mode = core::ptr::null_mut(), // XXX
    .line_status =	core::ptr::null_mut(), // XXX
    .update_status =	pc2_update_status,
    .primary_address =	pc2_primary_address,
    .secondary_address =	pc2_secondary_address,
    .serial_poll_response =	pc2_serial_poll_response,
    .serial_poll_status =	pc2_serial_poll_status,
    .t1_delay = pc2_t1_delay,
    .return_to_local = pc2_return_to_local,
    };
    static struct gpib_interface pc2_2a_interface = {
    .name =	"pcII_IIa",
    .attach =	pc2_2a_attach,
    .detach =	pc2_2a_detach,
    .read =	pc2_read,
    .write =	pc2_write,
    .command =	pc2_command,
    .take_control =	pc2_take_control,
    .go_to_standby =	pc2_go_to_standby,
    .request_system_control =	pc2_request_system_control,
    .interface_clear =	pc2_interface_clear,
    .remote_enable =	pc2_remote_enable,
    .enable_eos =	pc2_enable_eos,
    .disable_eos =	pc2_disable_eos,
    .parallel_poll =	pc2_parallel_poll,
    .parallel_poll_configure =	pc2_parallel_poll_configure,
    .parallel_poll_response =	pc2_parallel_poll_response,
    .local_parallel_poll_mode = core::ptr::null_mut(), // XXX
    .line_status =	core::ptr::null_mut(),
    .update_status =	pc2_update_status,
    .primary_address =	pc2_primary_address,
    .secondary_address =	pc2_secondary_address,
    .serial_poll_response =	pc2_serial_poll_response,
    .serial_poll_status =	pc2_serial_poll_status,
    .t1_delay = pc2_t1_delay,
    .return_to_local = pc2_return_to_local,
    };
#[no_mangle]
unsafe extern "C" fn pc2_init_module() -> int __init {
    static int __init pc2_init_module(void)
    {
    int ret;
    ret = gpib_register_driver(&pc2_interface, THIS_MODULE);
    if (ret) {
    pr_err("gpib_register_driver failed: error = %d\n", ret);
    return ret;
    }
    ret = gpib_register_driver(&pc2a_interface, THIS_MODULE);
    if (ret) {
    pr_err("gpib_register_driver failed: error = %d\n", ret);
    goto err_pc2a;
    }
    ret = gpib_register_driver(&pc2a_cb7210_interface, THIS_MODULE);
    if (ret) {
    pr_err("gpib_register_driver failed: error = %d\n", ret);
    goto err_cb7210;
    }
    ret = gpib_register_driver(&pc2_2a_interface, THIS_MODULE);
    if (ret) {
    pr_err("gpib_register_driver failed: error = %d\n", ret);
    goto err_pc2_2a;
    }
    return 0;
    err_pc2_2a:
    gpib_unregister_driver(&pc2a_cb7210_interface);
    err_cb7210:
    gpib_unregister_driver(&pc2a_interface);
    err_pc2a:
    gpib_unregister_driver(&pc2_interface);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pc2_exit_module() -> void __exit {
    static void __exit pc2_exit_module(void)
    {
    gpib_unregister_driver(&pc2_interface);
    gpib_unregister_driver(&pc2a_interface);
    gpib_unregister_driver(&pc2a_cb7210_interface);
    gpib_unregister_driver(&pc2_2a_interface);
    }
    module_init(pc2_init_module);
    module_exit(pc2_exit_module);
