//! Automatically rewritten from C to Rust
//! Source: drivers/block/amiflop.c
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
// linux/amiga/amiflop.c
//
// Copyright (C) 1993  Greg Harp
// Portions of this driver are based on code contributed by Brad Pepers
//
// revised 28.5.95 by Joerg Dorchain
// - now no bugs(?) any more for both HD & DD
// - added support for 40 Track 5.25" drives, 80-track hopefully behaves
// like 3.5" dd (no way to test - are there any 5.25" drives out there
// that work on an A4000?)
// - wrote formatting routine (maybe dirty, but works)
//
// june/july 1995 added ms-dos support by Joerg Dorchain
// (portions based on messydos.device and various contributors)
// - currently only 9 and 18 sector disks
//
// - fixed a bug with the internal trackbuffer when using multiple
// disks the same time
// - made formatting a bit safer
// - added command line and machine based default for "silent" df0
//
// december 1995 adapted for 1.2.13pl4 by Joerg Dorchain
// - works but I think it's inefficient. (look in redo_fd_request)
// But the changes were very efficient. (only three and a half lines)
//
// january 1996 added special ioctl for tracking down read/write problems
// - usage ioctl(d, RAW_TRACK, ptr); the raw track buffer (MFM-encoded data
// is copied to area. (area should be large enough since no checking is
// done - 30K is currently sufficient). return the actual size of the
// trackbuffer
// - replaced udelays() by a timer (CIAA timer B) for the waits
// needed for the disk mechanic.
//
// february 1996 fixed error recovery and multiple disk access
// - both got broken the first time I tampered with the driver :-(
// - still not safe, but better than before
//
// revised Marts 3rd, 1996 by Jes Sorensen for use in the 1.3.28 kernel.
// - Minor changes to accept the kdev_t.
// - Replaced some more udelays with ms_delays. Udelay is just a loop,
// and so the delay will be different depending on the given
// processor :-(
// - The driver could use a major cleanup because of the new
// major/minor handling that came with kdev_t. It seems to work for
// the time being, but I can't guarantee that it will stay like
// that when we start using 16 (24?) bit minors.
//
// restructured jan 1997 by Joerg Dorchain
// - Fixed Bug accessing multiple disks
// - some code cleanup
// - added trackbuffer for each drive to speed things up
// - fixed some race conditions (who finds the next may send it to me ;-)
//

// Macro flag: #define RAW_IOCTL

pub const IOCTL_RAW_TRACK: c_uint = 0x5254524B  /* 'RTRK' */;

//
// Defines
//
// CIAAPRA bits (read only)
//

//
// CIAAPRB bits (read/write)
//

//
// DSKBYTR bits (read only)
//

// bits 7-0 are data
//
// ADKCON/ADKCONR bits
//

//
// DSKLEN bits
//

//
// INTENA/INTREQ bits
//

//
// Misc
//
pub const MFM_SYNC: c_uint = 0x4489          /* standard MFM sync value */;
// Values for FD_COMMAND
pub const FD_RECALIBRATE: c_uint = 0x07	/* move to track 0 */;
pub const FD_SEEK: c_uint = 0x0F	/* seek track */;
pub const FD_READ: c_uint = 0xE6	/* read with MT, MFM, SKip deleted */;
pub const FD_WRITE: c_uint = 0xC5	/* write with MT, MFM */;
pub const FD_SENSEI: c_uint = 0x08	/* Sense Interrupt Status */;
pub const FD_SPECIFY: c_uint = 0x03	/* specify HUT etc */;
pub const FD_FORMAT: c_uint = 0x4D	/* format one track */;
pub const FD_VERSION: c_uint = 0x10	/* get version code */;
pub const FD_CONFIGURE: c_uint = 0x13	/* configure FIFO operation */;
pub const FD_PERPENDICULAR: c_uint = 0x12	/* perpendicular r/w mode */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_data_type {
    pub /: *mut *mut *mut char name; / description of data type,
    pub /: *mut *mut int sects; / sectors per track,
    pub /: *mut *mut *mut int (read_fkt)(int); / read whole track,
    pub /: *mut *mut *mut void (write_fkt)(int); / write whole track,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_drive_type {
    pub /: *mut *mut unsigned long code; / code returned from drive,
    pub /: *mut *mut *mut char name; / description of drive,
    pub /: *mut *mut unsigned int tracks; / number of tracks,
    pub /: *mut *mut unsigned int heads; / number of heads,
    pub /: *mut *mut unsigned int read_size; / raw read size for one track,
    pub /: *mut *mut unsigned int write_size; / raw write size for one track,
    pub /: *mut *mut unsigned int sect_mult; / sectors and gap multiplier (HD = 2),
    pub /: *mut *mut unsigned int precomp1; / start track for precomp 1,
    pub /: *mut *mut unsigned int precomp2; / start track for precomp 2,
    pub /: *mut *mut unsigned int step_delay; / time (in ms) for delay after step,
    pub /: *mut *mut unsigned int settle_time; / time to settle after dir change,
    pub /: *mut *mut unsigned int side_time; / time needed to change sides,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amiga_floppy_struct {
    pub /: *mut *mut *mut fd_drive_type type; / type of floppy for this unit,
    pub /: *mut *mut *mut fd_data_type dtype; / type of floppy for this unit,
    pub /: *mut *mut int track; / current track (-1 == unknown),
    pub /: *mut *mut *mut unsigned char trackbuf; / current track (kmaloc()'d,
    pub /: *mut *mut int blocks; / total # blocks on disk,
    pub /: *mut *mut int changed; / true when not known,
    pub /: *mut *mut int disk; / disk in drive (-1 == unknown),
    pub /: *mut *mut int motor; / true when motor is at speed,
    pub /: *mut *mut int busy; / true when drive is active,
    pub /: *mut *mut int dirty; / true when trackbuf is not on disk,
    pub /: *mut *mut int status; / current error code for unit,
    pub gendisk: [*mut gendisk; 2],
    pub tag_set: blk_mq_tag_set,
}

//
// Error codes
//

pub const MFM_NOSYNC: c_int = 1;
pub const MFM_HEADER: c_int = 2;
pub const MFM_DATA: c_int = 3;
pub const MFM_TRACK: c_int = 4;
//
// Floppy ID values
//
pub const FD_NODRIVE: c_uint = 0x00000000  /* response when no unit is present */;
pub const FD_DD_3: c_uint = 0xffffffff  /* double-density 3.5" (880K) drive */;
pub const FD_HD_3: c_uint = 0x55555555  /* high-density 3.5" (1760K) drive */;
pub const FD_DD_5: c_uint = 0xaaaaaaaa  /* double-density 5.25" (440K) drive */;
    static DEFINE_MUTEX(amiflop_mutex);
    static unsigned long int fd_def_df0 = FD_DD_3;     /* default for df0 if it doesn't identify */
    module_param(fd_def_df0, ulong, 0);
    MODULE_DESCRIPTION("Amiga floppy driver");
    MODULE_LICENSE("GPL");
//
// Macros
//

    static struct fd_drive_type drive_types[] = {
// code	name	   tr he   rdsz   wrsz sm pc1 pc2 sd  st st
// warning: times are now in milliseconds (ms)
    { FD_DD_3,	"DD 3.5",  80, 2, 14716, 13630, 1, 80,161, 3, 18, 1},
    { FD_HD_3,	"HD 3.5",  80, 2, 28344, 27258, 2, 80,161, 3, 18, 1},
    { FD_DD_5,	"DD 5.25", 40, 2, 14716, 13630, 1, 40, 81, 6, 30, 2},
    { FD_NODRIVE, "No Drive", 0, 0,     0,     0, 0,  0,  0,  0,  0, 0}
    };
    let mut num_dr_types: static int = ARRAY_SIZE(drive_types);
    static int amiga_read(int), dos_read(int);
    static void amiga_write(int), dos_write(int);
    static struct fd_data_type data_types[] = {
    { "Amiga", 11 , amiga_read, amiga_write},
    { "MS-Dos", 9, dos_read, dos_write}
    };
// current info on each unit
    static struct amiga_floppy_struct unit[FD_MAX_UNITS];
    static struct timer_list flush_track_timer[FD_MAX_UNITS];
    static struct timer_list post_write_timer;
    static unsigned long post_write_timer_drive;
    static struct timer_list motor_on_timer;
    static struct timer_list motor_off_timer[FD_MAX_UNITS];
    static int on_attempts;
// Synchronization of FDC access
// request loop (trackbuffer)
    let mut fdc_busy: static volatile int = -1;
    static volatile int fdc_nested;
    static DECLARE_WAIT_QUEUE_HEAD(fdc_wait);
    static DECLARE_COMPLETION(motor_on_completion);
    static volatile int selected = -1;	/* currently selected drive */
    static int writepending;
    static int writefromint;
    static char *raw_buf;
    static DEFINE_SPINLOCK(amiflop_lock);

//
// These are global variables, as that's the easiest way to give
// information to interrupts. They are the data used for the current
// request.
//
    static volatile char block_flag;
    static DECLARE_WAIT_QUEUE_HEAD(wait_fd_block);
// MS-Dos MFM Coding tables (should go quick and easy)
    static unsigned char mfmencode[16]={
    0x2a, 0x29, 0x24, 0x25, 0x12, 0x11, 0x14, 0x15,
    0x4a, 0x49, 0x44, 0x45, 0x52, 0x51, 0x54, 0x55
    };
    static unsigned char mfmdecode[128];
// floppy internal millisecond timer stuff
    static DECLARE_COMPLETION(ms_wait_completion);

//
// Note that MAX_ERRORS=X doesn't imply that we retry every bad read
// max X times - some types of errors increase the errorcount by 2 or
// even 3, so we might actually retry only X/2 times before giving up.
//
pub const MAX_ERRORS: c_int = 12;

// Prevent "aliased" accesses.
    static int fd_ref[4] = { 0,0,0,0 };
    static int fd_device[4] = { 0, 0, 0, 0 };
//
// Here come the actual hardware access and helper functions.
// They are not reentrant and single threaded because all drives
// share the same hardware and the same trackbuffer.
//
// Milliseconds timer
#[no_mangle]
unsafe extern "C" fn ms_isr(irq: c_int, dummy: *mut c_void) -> irqreturn_t {
    static irqreturn_t ms_isr(int irq, void *dummy)
    {
    complete(&ms_wait_completion);
    return IRQ_HANDLED;
    }
// all waits are queued up
    A more generic routine would do a schedule a la timer.device */
#[no_mangle]
unsafe extern "C" fn ms_delay(ms: c_int) {
    static void ms_delay(int ms)
    {
    int ticks;
    static DEFINE_MUTEX(mutex);
    if (ms > 0) {
    mutex_lock(&mutex);
    ticks = MS_TICKS*ms-1;
    ciaa.tblo=ticks%256;
    ciaa.tbhi=ticks/256;
    ciaa.crb=0x19; /*count eclock, force load, one-shoot, start */
    wait_for_completion(&ms_wait_completion);
    mutex_unlock(&mutex);
    }
    }
// Hardware semaphore
// returns true when we would get the semaphore
#[no_mangle]
pub unsafe extern "C" fn try_fdc(drive: c_int) -> c_int {
    static inline int try_fdc(int drive)
    {
    drive &= 3;
    return ((fdc_busy < 0) || (fdc_busy == drive));
    }
#[no_mangle]
unsafe extern "C" fn get_fdc(drive: c_int) {
    static void get_fdc(int drive)
    {
    unsigned long flags;
    drive &= 3;

    printk("get_fdc: drive %d  fdc_busy %d  fdc_nested %d\n",drive,fdc_busy,fdc_nested);

    local_irq_save(flags);
    wait_event(fdc_wait, try_fdc(drive));
    fdc_busy = drive;
    fdc_nested++;
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rel_fdc() {
    static inline void rel_fdc(void)
    {

    if (fdc_nested == 0)
    printk("fd: unmatched rel_fdc\n");
    printk("rel_fdc: fdc_busy %d fdc_nested %d\n",fdc_busy,fdc_nested);

    fdc_nested--;
    if (fdc_nested == 0) {
    fdc_busy = -1;
    wake_up(&fdc_wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn fd_select(drive: c_int) {
    static void fd_select (int drive)
    {
    let mut prb: c_uchar = ~0;
    drive&=3;

    printk("selecting %d\n",drive);

    if (drive == selected)
    return;
    get_fdc(drive);
    selected = drive;
    if (unit[drive].track % 2 != 0)
    prb &= ~DSKSIDE;
    if (unit[drive].motor == 1)
    prb &= ~DSKMOTOR;
    ciab.prb |= (SELMASK(0)|SELMASK(1)|SELMASK(2)|SELMASK(3));
    ciab.prb = prb;
    prb &= ~SELMASK(drive);
    ciab.prb = prb;
    rel_fdc();
    }
#[no_mangle]
unsafe extern "C" fn fd_deselect(drive: c_int) {
    static void fd_deselect (int drive)
    {
    unsigned char prb;
    unsigned long flags;
    drive&=3;

    printk("deselecting %d\n",drive);

    if (drive != selected) {
    printk(KERN_WARNING "Deselecting drive %d while %d was selected!\n",drive,selected);
    return;
    }
    get_fdc(drive);
    local_irq_save(flags);
    selected = -1;
    prb = ciab.prb;
    prb |= (SELMASK(0)|SELMASK(1)|SELMASK(2)|SELMASK(3));
    ciab.prb = prb;
    local_irq_restore (flags);
    rel_fdc();
    }
#[no_mangle]
unsafe extern "C" fn motor_on_callback(unused: *mut timer_list) {
    static void motor_on_callback(struct timer_list *unused)
    {
    if (!(ciaa.pra & DSKRDY) || --on_attempts == 0) {
    complete_all(&motor_on_completion);
    } else {
    motor_on_timer.expires = jiffies + HZ/10;
    add_timer(&motor_on_timer);
    }
    }
#[no_mangle]
unsafe extern "C" fn fd_motor_on(nr: c_int) -> c_int {
    static int fd_motor_on(int nr)
    {
    nr &= 3;
    timer_delete(motor_off_timer + nr);
    if (!unit[nr].motor) {
    unit[nr].motor = 1;
    fd_select(nr);
    reinit_completion(&motor_on_completion);
    mod_timer(&motor_on_timer, jiffies + HZ/2);
    on_attempts = 10;
    wait_for_completion(&motor_on_completion);
    fd_deselect(nr);
    }
    if (on_attempts == 0) {
    on_attempts = -1;

    printk (KERN_ERR "motor_on failed, turning motor off\n");
    fd_motor_off (motor_off_timer + nr);
    return 0;

    printk (KERN_WARNING "DSKRDY not set after 1.5 seconds - assuming drive is spinning notwithstanding\n");

    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fd_motor_off(timer: *mut timer_list) {
    static void fd_motor_off(struct timer_list *timer)
    {
    unsigned long drive = ((unsigned long)timer -
    (unsigned long)&motor_off_timer[0]) /
    sizeof(motor_off_timer[0]);
    drive&=3;
    if (!try_fdc(drive)) {
// We would be blocked in an interrupt, so try again later
    timer.expires = jiffies + 1;
    add_timer(timer);
    return;
    }
    unit[drive].motor = 0;
    fd_select(drive);
    udelay (1);
    fd_deselect(drive);
    }
#[no_mangle]
unsafe extern "C" fn floppy_off(nr: c_uint) {
    static void floppy_off (unsigned int nr)
    {
    int drive;
    drive = nr & 3;
    mod_timer(motor_off_timer + drive, jiffies + 3*HZ);
    }
#[no_mangle]
unsafe extern "C" fn fd_calibrate(drive: c_int) -> c_int {
    static int fd_calibrate(int drive)
    {
    unsigned char prb;
    int n;
    drive &= 3;
    get_fdc(drive);
    if (!fd_motor_on (drive))
    return 0;
    fd_select (drive);
    prb = ciab.prb;
    prb |= DSKSIDE;
    prb &= ~DSKDIREC;
    ciab.prb = prb;
    for (n = unit[drive].type.tracks/2; n != 0; --n) {
    if (ciaa.pra & DSKTRACK0)
    break;
    prb &= ~DSKSTEP;
    ciab.prb = prb;
    prb |= DSKSTEP;
    udelay (2);
    ciab.prb = prb;
    ms_delay(unit[drive].type.step_delay);
    }
    ms_delay (unit[drive].type.settle_time);
    prb |= DSKDIREC;
    n = unit[drive].type.tracks + 20;
    for (;;) {
    prb &= ~DSKSTEP;
    ciab.prb = prb;
    prb |= DSKSTEP;
    udelay (2);
    ciab.prb = prb;
    ms_delay(unit[drive].type.step_delay + 1);
    if ((ciaa.pra & DSKTRACK0) == 0)
    break;
    if (--n == 0) {
    printk (KERN_ERR "fd%d: calibrate failed, turning motor off\n", drive);
    fd_motor_off (motor_off_timer + drive);
    unit[drive].track = -1;
    rel_fdc();
    return 0;
    }
    }
    unit[drive].track = 0;
    ms_delay(unit[drive].type.settle_time);
    rel_fdc();
    fd_deselect(drive);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fd_seek(drive: c_int, track: c_int) -> c_int {
    static int fd_seek(int drive, int track)
    {
    unsigned char prb;
    int cnt;

    printk("seeking drive %d to track %d\n",drive,track);

    drive &= 3;
    get_fdc(drive);
    if (unit[drive].track == track) {
    rel_fdc();
    return 1;
    }
    if (!fd_motor_on(drive)) {
    rel_fdc();
    return 0;
    }
    if (unit[drive].track < 0 && !fd_calibrate(drive)) {
    rel_fdc();
    return 0;
    }
    fd_select (drive);
    cnt = unit[drive].track/2 - track/2;
    prb = ciab.prb;
    prb |= DSKSIDE | DSKDIREC;
    if (track % 2 != 0)
    prb &= ~DSKSIDE;
    if (cnt < 0) {
    cnt = - cnt;
    prb &= ~DSKDIREC;
    }
    ciab.prb = prb;
    if (track % 2 != unit[drive].track % 2)
    ms_delay (unit[drive].type.side_time);
    unit[drive].track = track;
    if (cnt == 0) {
    rel_fdc();
    fd_deselect(drive);
    return 1;
    }
    do {
    prb &= ~DSKSTEP;
    ciab.prb = prb;
    prb |= DSKSTEP;
    udelay (1);
    ciab.prb = prb;
    ms_delay (unit[drive].type.step_delay);
    } while (--cnt != 0);
    ms_delay (unit[drive].type.settle_time);
    rel_fdc();
    fd_deselect(drive);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fd_get_drive_id(drive: c_int) -> c_ulong {
    static unsigned long fd_get_drive_id(int drive)
    {
    int i;
    let mut id: c_ulong = 0;
    drive&=3;
    get_fdc(drive);
// set up for ID
    MOTOR_ON;
    udelay(2);
    SELECT(SELMASK(drive));
    udelay(2);
    DESELECT(SELMASK(drive));
    udelay(2);
    MOTOR_OFF;
    udelay(2);
    SELECT(SELMASK(drive));
    udelay(2);
    DESELECT(SELMASK(drive));
    udelay(2);
// loop and read disk ID
    for (i=0; i<32; i++) {
    SELECT(SELMASK(drive));
    udelay(2);
// read and store value of DSKRDY
    id <<= 1;
    id |= (ciaa.pra & DSKRDY) ? 0 : 1;	/* cia regs are low-active! */
    DESELECT(SELMASK(drive));
    }
    rel_fdc();
//
// RB: At least A500/A2000's df0: don't identify themselves.
// As every (real) Amiga has at least a 3.5" DD drive as df0:
// we default to that if df0: doesn't identify as a certain
// type.
//
    if(drive == 0 && id == FD_NODRIVE)
    {
    id = fd_def_df0;
    printk(KERN_NOTICE "fd: drive 0 didn't identify, setting default %08lx\n", (ulong)fd_def_df0);
    }
// return the ID value
    return (id);
    }
#[no_mangle]
unsafe extern "C" fn fd_block_done(irq: c_int, dummy: *mut c_void) -> irqreturn_t {
    static irqreturn_t fd_block_done(int irq, void *dummy)
    {
    if (block_flag)
    custom.dsklen = 0x4000;
    if (block_flag == 2) { /* writing */
    writepending = 2;
    post_write_timer.expires = jiffies + 1; /* at least 2 ms */
    post_write_timer_drive = selected;
    add_timer(&post_write_timer);
    }
    else {                /* reading */
    block_flag = 0;
    wake_up (&wait_fd_block);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn raw_read(drive: c_int) {
    static void raw_read(int drive)
    {
    drive&=3;
    get_fdc(drive);
    wait_event(wait_fd_block, !block_flag);
    fd_select(drive);
// setup adkcon bits correctly
    custom.adkcon = ADK_MSBSYNC;
    custom.adkcon = ADK_SETCLR|ADK_WORDSYNC|ADK_FAST;
    custom.dsksync = MFM_SYNC;
    custom.dsklen = 0;
    custom.dskptr = (u_char *)ZTWO_PADDR((u_char *)raw_buf);
    custom.dsklen = unit[drive].type.read_size/sizeof(short) | DSKLEN_DMAEN;
    custom.dsklen = unit[drive].type.read_size/sizeof(short) | DSKLEN_DMAEN;
    block_flag = 1;
    wait_event(wait_fd_block, !block_flag);
    custom.dsklen = 0;
    fd_deselect(drive);
    rel_fdc();
    }
#[no_mangle]
unsafe extern "C" fn raw_write(drive: c_int) -> c_int {
    static int raw_write(int drive)
    {
    ushort adk;
    drive&=3;
    get_fdc(drive); /* corresponds to rel_fdc() in post_write() */
    if ((ciaa.pra & DSKPROT) == 0) {
    rel_fdc();
    return 0;
    }
    wait_event(wait_fd_block, !block_flag);
    fd_select(drive);
// clear adkcon bits
    custom.adkcon = ADK_PRECOMP1|ADK_PRECOMP0|ADK_WORDSYNC|ADK_MSBSYNC;
// set appropriate adkcon bits
    adk = ADK_SETCLR|ADK_FAST;
    if ((ulong)unit[drive].track >= unit[drive].type.precomp2)
    adk |= ADK_PRECOMP1;
#[no_mangle]
pub unsafe extern "C" fn if(unit[drive].type->precomp1: (ulong)unit[drive].track >=) -> else {
    else if ((ulong)unit[drive].track >= unit[drive].type.precomp1)
    adk |= ADK_PRECOMP0;
    custom.adkcon = adk;
    custom.dsklen = DSKLEN_WRITE;
    custom.dskptr = (u_char *)ZTWO_PADDR((u_char *)raw_buf);
    custom.dsklen = unit[drive].type.write_size/sizeof(short) | DSKLEN_DMAEN|DSKLEN_WRITE;
    custom.dsklen = unit[drive].type.write_size/sizeof(short) | DSKLEN_DMAEN|DSKLEN_WRITE;
    block_flag = 2;
    return 1;
    }
//
// to be called at least 2ms after the write has finished but before any
// other access to the hardware.
//
#[no_mangle]
unsafe extern "C" fn post_write(drive: c_ulong) {
    static void post_write (unsigned long drive)
    {

    printk("post_write for drive %ld\n",drive);

    drive &= 3;
    custom.dsklen = 0;
    block_flag = 0;
    writepending = 0;
    writefromint = 0;
    unit[drive].dirty = 0;
    wake_up(&wait_fd_block);
    fd_deselect(drive);
    rel_fdc(); /* corresponds to get_fdc() in raw_write */
    }
#[no_mangle]
unsafe extern "C" fn post_write_callback(timer: *mut timer_list) {
    static void post_write_callback(struct timer_list *timer)
    {
    post_write(post_write_timer_drive);
    }
//
// The following functions are to convert the block contents into raw data
// written to disk and vice versa.
// (Add other formats here ;-))
//
#[no_mangle]
unsafe extern "C" fn scan_sync(raw: c_ulong, end: c_ulong) -> c_ulong {
    static unsigned long scan_sync(unsigned long raw, unsigned long end)
    {
    ushort *ptr = (ushort *)raw, *endp = (ushort *)end;
    while (ptr < endp && *ptr++ != 0x4489)
    ;
    if (ptr < endp) {
    while (*ptr == 0x4489 && ptr < endp)
    ptr++;
    return (ulong)ptr;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn checksum(addr: *mut c_ulong, len: c_int) -> c_ulong {
    static inline unsigned long checksum(unsigned long *addr, int len)
    {
    let mut csum: c_ulong = 0;
    len /= sizeof(*addr);
    while (len-- > 0)
    csum ^= *addr++;
    csum = ((csum>>1) & 0x55555555)  ^  (csum & 0x55555555);
    return csum;
    }
    static unsigned long decode (unsigned long *data, unsigned long *raw,
    int len)
    {
    ulong *odd, *even;
// convert length from bytes to longwords
    len >>= 2;
    odd = raw;
    even = odd + len;
// prepare return pointer
    raw += len * 2;
    do {
// data++ = ((*odd++ & 0x55555555) << 1) | (*even++ & 0x55555555);
    } while (--len != 0);
    return (ulong)raw;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct header {
    pub magic: c_uchar,
    pub track: c_uchar,
    pub sect: c_uchar,
    pub ord: c_uchar,
    pub labels: [c_uchar; 16],
    pub hdrchk: c_ulong,
    pub datachk: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn amiga_read(drive: c_int) -> c_int {
    static int amiga_read(int drive)
    {
    unsigned long raw;
    unsigned long end;
    int scnt;
    unsigned long csum;
    struct header hdr;
    drive&=3;
    raw = (long) raw_buf;
    end = raw + unit[drive].type.read_size;
    for (scnt = 0;scnt < unit[drive].dtype.sects * unit[drive].type.sect_mult; scnt++) {
    if (!(raw = scan_sync(raw, end))) {
    printk (KERN_INFO "can't find sync for sector %d\n", scnt);
    return MFM_NOSYNC;
    }
    raw = decode ((ulong *)&hdr.magic, (ulong *)raw, 4);
    raw = decode ((ulong *)&hdr.labels, (ulong *)raw, 16);
    raw = decode ((ulong *)&hdr.hdrchk, (ulong *)raw, 4);
    raw = decode ((ulong *)&hdr.datachk, (ulong *)raw, 4);
    csum = checksum((ulong *)&hdr,
    (char *)&hdr.hdrchk-(char *)&hdr);

    printk ("(%x,%d,%d,%d) (%lx,%lx,%lx,%lx) %lx %lx\n",
    hdr.magic, hdr.track, hdr.sect, hdr.ord,
// (ulong *)&hdr.labels[0], *(ulong *)&hdr.labels[4],
// (ulong *)&hdr.labels[8], *(ulong *)&hdr.labels[12],
    hdr.hdrchk, hdr.datachk);

    if (hdr.hdrchk != csum) {
    printk(KERN_INFO "MFM_HEADER: %08lx,%08lx\n", hdr.hdrchk, csum);
    return MFM_HEADER;
    }
// verify track
    if (hdr.track != unit[drive].track) {
    printk(KERN_INFO "MFM_TRACK: %d, %d\n", hdr.track, unit[drive].track);
    return MFM_TRACK;
    }
    raw = decode ((ulong *)(unit[drive].trackbuf + hdr.sect*512),
    (ulong *)raw, 512);
    csum = checksum((ulong *)(unit[drive].trackbuf + hdr.sect*512), 512);
    if (hdr.datachk != csum) {
    printk(KERN_INFO "MFM_DATA: (%x:%d:%d:%d) sc=%d %lx, %lx\n",
    hdr.magic, hdr.track, hdr.sect, hdr.ord, scnt,
    hdr.datachk, csum);
    printk (KERN_INFO "data=(%lx,%lx,%lx,%lx)\n",
    ((ulong *)(unit[drive].trackbuf+hdr.sect*512))[0],
    ((ulong *)(unit[drive].trackbuf+hdr.sect*512))[1],
    ((ulong *)(unit[drive].trackbuf+hdr.sect*512))[2],
    ((ulong *)(unit[drive].trackbuf+hdr.sect*512))[3]);
    return MFM_DATA;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode(data: c_ulong, dest: *mut c_ulong) {
    static void encode(unsigned long data, unsigned long *dest)
    {
    unsigned long data2;
    data &= 0x55555555;
    data2 = data ^ 0x55555555;
    data |= ((data2 >> 1) | 0x80000000) & (data2 << 1);
    if (*(dest - 1) & 0x00000001)
    data &= 0x7FFFFFFF;
// dest = data;
    }
#[no_mangle]
unsafe extern "C" fn encode_block(dest: *mut c_ulong, src: *mut c_ulong, len: c_int) {
    static void encode_block(unsigned long *dest, unsigned long *src, int len)
    {
    int cnt, to_cnt = 0;
    unsigned long data;
// odd bits
    for (cnt = 0; cnt < len / 4; cnt++) {
    data = src[cnt] >> 1;
    encode(data, dest + to_cnt++);
    }
// even bits
    for (cnt = 0; cnt < len / 4; cnt++) {
    data = src[cnt];
    encode(data, dest + to_cnt++);
    }
    }
    static unsigned long *putsec(int disk, unsigned long *raw, int cnt)
    {
    struct header hdr;
    int i;
    disk&=3;
// raw = (raw[-1]&1) ? 0x2AAAAAAA : 0xAAAAAAAA;
    raw++;
// raw++ = 0x44894489;
    hdr.magic = 0xFF;
    hdr.track = unit[disk].track;
    hdr.sect = cnt;
    hdr.ord = unit[disk].dtype.sects * unit[disk].type.sect_mult - cnt;
    for (i = 0; i < 16; i++)
    hdr.labels[i] = 0;
    hdr.hdrchk = checksum((ulong *)&hdr,
    (char *)&hdr.hdrchk-(char *)&hdr);
    hdr.datachk = checksum((ulong *)(unit[disk].trackbuf+cnt*512), 512);
    encode_block(raw, (ulong *)&hdr.magic, 4);
    raw += 2;
    encode_block(raw, (ulong *)&hdr.labels, 16);
    raw += 8;
    encode_block(raw, (ulong *)&hdr.hdrchk, 4);
    raw += 2;
    encode_block(raw, (ulong *)&hdr.datachk, 4);
    raw += 2;
    encode_block(raw, (ulong *)(unit[disk].trackbuf+cnt*512), 512);
    raw += 256;
    return raw;
    }
#[no_mangle]
unsafe extern "C" fn amiga_write(disk: c_int) {
    static void amiga_write(int disk)
    {
    unsigned int cnt;
    unsigned long *ptr = (unsigned long *)raw_buf;
    disk&=3;
// gap space
    for (cnt = 0; cnt < 415 * unit[disk].type.sect_mult; cnt++)
// ptr++ = 0xaaaaaaaa;
// sectors
    for (cnt = 0; cnt < unit[disk].dtype.sects * unit[disk].type.sect_mult; cnt++)
    ptr = putsec (disk, ptr, cnt);
// (ushort *)ptr = (ptr[-1]&1) ? 0x2AA8 : 0xAAA8;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dos_header {
    unsigned char track,   /* 0-80 */
    side,    /* 0-1 */
    sec,     /* 0-...*/
    pub /: *mut *mut len_desc;/ 2,
    pub problem,: *mut *mut unsigned short crc; / on 68000 we got an alignment,
    but this compiler solves it  by adding silently
    adding a pad byte so data won't fit
    and this took about 3h to discover.... */
    pub /: *mut *mut unsigned char gap1[22]; / for longword-alignedness (0x4e),
}

// crc routines are borrowed from the messydos-handler
// excerpt from the messydos-device
    ; The CRC is computed not only over the actual data, but including
    ; the SYNC mark (3 * $a1) and the 'ID/DATA - Address Mark' ($fe/$fb).
    ; As we don't read or encode these fields into our buffers, we have to
    ; preload the registers containing the CRC with the values they would have
    ; after stepping over these fields.
    ;
    ; How CRCs "really" work:
    ;
    ; First, you should regard a bitstring as a series of coefficients of
    ; polynomials. We calculate with these polynomials in modulo-2
    ; arithmetic, in which both add and subtract are done the same as
    ; exclusive-or. Now, we modify our data (a very long polynomial) in
    ; such a way that it becomes divisible by the CCITT-standard 16-bit
    ;		 16   12   5
    ; polynomial:	x  + x	+ x + 1, represented by $11021. The easiest
    ; way to do this would be to multiply (using proper arithmetic) our
    ; datablock with $11021. So we have:
    ;   data * $11021		 =
    ;   data * ($10000 + $1021)      =
    ;   data * $10000 + data * $1021
    ; The left part of this is simple: Just add two 0 bytes. But then
    ; the right part (data $1021) remains difficult and even could have
    ; a carry into the left part. The solution is to use a modified
    ; multiplication, which has a result that is not correct, but with
    ; a difference of any multiple of $11021. We then only need to keep
    ; the 16 least significant bits of the result.
    ;
    ; The following algorithm does this for us:
    ;
    ;   unsigned char *data, c, crclo, crchi;
    ;   while (not done) {
    ;	c = *data++ + crchi;
    ;	crchi = (@ c) >> 8 + crclo;
    ;	crclo = @ c;
    ;   }
    ;
    ; Remember, + is done with EOR, the @ operator is in two tables (high
    ; and low byte separately), which is calculated as
    ;
    ;      $1021 * (c & $F0)
    ;  xor $1021 * (c & $0F)
    ;  xor $1021 * (c >> 4)         (* is regular multiplication)
    ;
    ;
    ; Anyway, the end result is the same as the remainder of the division of
    ; the data by $11021. I am afraid I need to study theory a bit more...
    my only works was to code this from manx to C....
//
#[no_mangle]
unsafe extern "C" fn dos_crc(data_a3: *mut *mut c_void, data_d0: c_int, data_d1: c_int, data_d3: c_int) -> c_ushort {
    static ushort dos_crc(void * data_a3, int data_d0, int data_d1, int data_d3)
    {
    static unsigned char CRCTable1[] = {
    0x00,0x10,0x20,0x30,0x40,0x50,0x60,0x70,0x81,0x91,0xa1,0xb1,0xc1,0xd1,0xe1,0xf1,
    0x12,0x02,0x32,0x22,0x52,0x42,0x72,0x62,0x93,0x83,0xb3,0xa3,0xd3,0xc3,0xf3,0xe3,
    0x24,0x34,0x04,0x14,0x64,0x74,0x44,0x54,0xa5,0xb5,0x85,0x95,0xe5,0xf5,0xc5,0xd5,
    0x36,0x26,0x16,0x06,0x76,0x66,0x56,0x46,0xb7,0xa7,0x97,0x87,0xf7,0xe7,0xd7,0xc7,
    0x48,0x58,0x68,0x78,0x08,0x18,0x28,0x38,0xc9,0xd9,0xe9,0xf9,0x89,0x99,0xa9,0xb9,
    0x5a,0x4a,0x7a,0x6a,0x1a,0x0a,0x3a,0x2a,0xdb,0xcb,0xfb,0xeb,0x9b,0x8b,0xbb,0xab,
    0x6c,0x7c,0x4c,0x5c,0x2c,0x3c,0x0c,0x1c,0xed,0xfd,0xcd,0xdd,0xad,0xbd,0x8d,0x9d,
    0x7e,0x6e,0x5e,0x4e,0x3e,0x2e,0x1e,0x0e,0xff,0xef,0xdf,0xcf,0xbf,0xaf,0x9f,0x8f,
    0x91,0x81,0xb1,0xa1,0xd1,0xc1,0xf1,0xe1,0x10,0x00,0x30,0x20,0x50,0x40,0x70,0x60,
    0x83,0x93,0xa3,0xb3,0xc3,0xd3,0xe3,0xf3,0x02,0x12,0x22,0x32,0x42,0x52,0x62,0x72,
    0xb5,0xa5,0x95,0x85,0xf5,0xe5,0xd5,0xc5,0x34,0x24,0x14,0x04,0x74,0x64,0x54,0x44,
    0xa7,0xb7,0x87,0x97,0xe7,0xf7,0xc7,0xd7,0x26,0x36,0x06,0x16,0x66,0x76,0x46,0x56,
    0xd9,0xc9,0xf9,0xe9,0x99,0x89,0xb9,0xa9,0x58,0x48,0x78,0x68,0x18,0x08,0x38,0x28,
    0xcb,0xdb,0xeb,0xfb,0x8b,0x9b,0xab,0xbb,0x4a,0x5a,0x6a,0x7a,0x0a,0x1a,0x2a,0x3a,
    0xfd,0xed,0xdd,0xcd,0xbd,0xad,0x9d,0x8d,0x7c,0x6c,0x5c,0x4c,0x3c,0x2c,0x1c,0x0c,
    0xef,0xff,0xcf,0xdf,0xaf,0xbf,0x8f,0x9f,0x6e,0x7e,0x4e,0x5e,0x2e,0x3e,0x0e,0x1e
    };
    static unsigned char CRCTable2[] = {
    0x00,0x21,0x42,0x63,0x84,0xa5,0xc6,0xe7,0x08,0x29,0x4a,0x6b,0x8c,0xad,0xce,0xef,
    0x31,0x10,0x73,0x52,0xb5,0x94,0xf7,0xd6,0x39,0x18,0x7b,0x5a,0xbd,0x9c,0xff,0xde,
    0x62,0x43,0x20,0x01,0xe6,0xc7,0xa4,0x85,0x6a,0x4b,0x28,0x09,0xee,0xcf,0xac,0x8d,
    0x53,0x72,0x11,0x30,0xd7,0xf6,0x95,0xb4,0x5b,0x7a,0x19,0x38,0xdf,0xfe,0x9d,0xbc,
    0xc4,0xe5,0x86,0xa7,0x40,0x61,0x02,0x23,0xcc,0xed,0x8e,0xaf,0x48,0x69,0x0a,0x2b,
    0xf5,0xd4,0xb7,0x96,0x71,0x50,0x33,0x12,0xfd,0xdc,0xbf,0x9e,0x79,0x58,0x3b,0x1a,
    0xa6,0x87,0xe4,0xc5,0x22,0x03,0x60,0x41,0xae,0x8f,0xec,0xcd,0x2a,0x0b,0x68,0x49,
    0x97,0xb6,0xd5,0xf4,0x13,0x32,0x51,0x70,0x9f,0xbe,0xdd,0xfc,0x1b,0x3a,0x59,0x78,
    0x88,0xa9,0xca,0xeb,0x0c,0x2d,0x4e,0x6f,0x80,0xa1,0xc2,0xe3,0x04,0x25,0x46,0x67,
    0xb9,0x98,0xfb,0xda,0x3d,0x1c,0x7f,0x5e,0xb1,0x90,0xf3,0xd2,0x35,0x14,0x77,0x56,
    0xea,0xcb,0xa8,0x89,0x6e,0x4f,0x2c,0x0d,0xe2,0xc3,0xa0,0x81,0x66,0x47,0x24,0x05,
    0xdb,0xfa,0x99,0xb8,0x5f,0x7e,0x1d,0x3c,0xd3,0xf2,0x91,0xb0,0x57,0x76,0x15,0x34,
    0x4c,0x6d,0x0e,0x2f,0xc8,0xe9,0x8a,0xab,0x44,0x65,0x06,0x27,0xc0,0xe1,0x82,0xa3,
    0x7d,0x5c,0x3f,0x1e,0xf9,0xd8,0xbb,0x9a,0x75,0x54,0x37,0x16,0xf1,0xd0,0xb3,0x92,
    0x2e,0x0f,0x6c,0x4d,0xaa,0x8b,0xe8,0xc9,0x26,0x07,0x64,0x45,0xa2,0x83,0xe0,0xc1,
    0x1f,0x3e,0x5d,0x7c,0x9b,0xba,0xd9,0xf8,0x17,0x36,0x55,0x74,0x93,0xb2,0xd1,0xf0
    };
// look at the asm-code - what looks in C a bit strange is almost as good as handmade
    register int i;
    register unsigned char *CRCT1, *CRCT2, *data, c, crch, crcl;
    CRCT1=CRCTable1;
    CRCT2=CRCTable2;
    data=data_a3;
    crcl=data_d1;
    crch=data_d0;
    for (i=data_d3; i>=0; i--) {
    c = (*data++) ^ crch;
    crch = CRCT1[c] ^ crcl;
    crcl = CRCT2[c];
    }
    return (crch<<8)|crcl;
    }
#[no_mangle]
pub unsafe extern "C" fn dos_hdr_crc(hdr: *mut dos_header) -> c_ushort {
    static inline ushort dos_hdr_crc (struct dos_header *hdr)
    {
    return dos_crc(&(hdr.track), 0xb2, 0x30, 3); /* precomputed magic */
    }
#[no_mangle]
pub unsafe extern "C" fn dos_data_crc(data: *mut c_uchar) -> c_ushort {
    static inline ushort dos_data_crc(unsigned char *data)
    {
    return dos_crc(data, 0xe2, 0x95 ,511); /* precomputed magic */
    }
#[no_mangle]
pub unsafe extern "C" fn dos_decode_byte(word: c_ushort) -> c_uchar {
    static inline unsigned char dos_decode_byte(ushort word)
    {
    register ushort w2;
    register unsigned char byte;
    register unsigned char *dec = mfmdecode;
    w2=word;
    w2>>=8;
    w2&=127;
    byte = dec[w2];
    byte <<= 4;
    w2 = word & 127;
    byte |= dec[w2];
    return byte;
    }
#[no_mangle]
unsafe extern "C" fn dos_decode(data: *mut c_uchar, raw: *mut c_ushort, len: c_int) -> c_ulong {
    static unsigned long dos_decode(unsigned char *data, unsigned short *raw, int len)
    {
    int i;
    for (i = 0; i < len; i++)
// data++=dos_decode_byte(*raw++);
    return ((ulong)raw);
    }

#[no_mangle]
unsafe extern "C" fn dbg(ptr: c_ulong) {
    static void dbg(unsigned long ptr)
    {
    printk("raw data @%08lx: %08lx, %08lx ,%08lx, %08lx\n", ptr,
    ((ulong *)ptr)[0], ((ulong *)ptr)[1],
    ((ulong *)ptr)[2], ((ulong *)ptr)[3]);
    }

#[no_mangle]
unsafe extern "C" fn dos_read(drive: c_int) -> c_int {
    static int dos_read(int drive)
    {
    unsigned long end;
    unsigned long raw;
    int scnt;
    unsigned short crc,data_crc[2];
    struct dos_header hdr;
    drive&=3;
    raw = (long) raw_buf;
    end = raw + unit[drive].type.read_size;
    for (scnt=0; scnt < unit[drive].dtype.sects * unit[drive].type.sect_mult; scnt++) {
    do { /* search for the right sync of each sec-hdr */
    if (!(raw = scan_sync (raw, end))) {
    printk(KERN_INFO "dos_read: no hdr sync on "
    "track %d, unit %d for sector %d\n",
    unit[drive].track,drive,scnt);
    return MFM_NOSYNC;
    }

    dbg(raw);

    } while (*((ushort *)raw)!=0x5554); /* loop usually only once done */
    raw+=2; /* skip over headermark */
    raw = dos_decode((unsigned char *)&hdr,(ushort *) raw,8);
    crc = dos_hdr_crc(&hdr);

    printk("(%3d,%d,%2d,%d) %x\n", hdr.track, hdr.side,
    hdr.sec, hdr.len_desc, hdr.crc);

    if (crc != hdr.crc) {
    printk(KERN_INFO "dos_read: MFM_HEADER %04x,%04x\n",
    hdr.crc, crc);
    return MFM_HEADER;
    }
    if (hdr.track != unit[drive].track/unit[drive].type.heads) {
    printk(KERN_INFO "dos_read: MFM_TRACK %d, %d\n",
    hdr.track,
    unit[drive].track/unit[drive].type.heads);
    return MFM_TRACK;
    }
    if (hdr.side != unit[drive].track%unit[drive].type.heads) {
    printk(KERN_INFO "dos_read: MFM_SIDE %d, %d\n",
    hdr.side,
    unit[drive].track%unit[drive].type.heads);
    return MFM_TRACK;
    }
    if (hdr.len_desc != 2) {
    printk(KERN_INFO "dos_read: unknown sector len "
    "descriptor %d\n", hdr.len_desc);
    return MFM_DATA;
    }

    printk("hdr accepted\n");

    if (!(raw = scan_sync (raw, end))) {
    printk(KERN_INFO "dos_read: no data sync on track "
    "%d, unit %d for sector%d, disk sector %d\n",
    unit[drive].track, drive, scnt, hdr.sec);
    return MFM_NOSYNC;
    }

    dbg(raw);

    if (*((ushort *)raw)!=0x5545) {
    printk(KERN_INFO "dos_read: no data mark after "
    "sync (%d,%d,%d,%d) sc=%d\n",
    hdr.track,hdr.side,hdr.sec,hdr.len_desc,scnt);
    return MFM_NOSYNC;
    }
    raw+=2;  /* skip data mark (included in checksum) */
    raw = dos_decode((unsigned char *)(unit[drive].trackbuf + (hdr.sec - 1) * 512), (ushort *) raw, 512);
    raw = dos_decode((unsigned char  *)data_crc,(ushort *) raw,4);
    crc = dos_data_crc(unit[drive].trackbuf + (hdr.sec - 1) * 512);
    if (crc != data_crc[0]) {
    printk(KERN_INFO "dos_read: MFM_DATA (%d,%d,%d,%d) "
    "sc=%d, %x %x\n", hdr.track, hdr.side,
    hdr.sec, hdr.len_desc, scnt,data_crc[0], crc);
    printk(KERN_INFO "data=(%lx,%lx,%lx,%lx,...)\n",
    ((ulong *)(unit[drive].trackbuf+(hdr.sec-1)*512))[0],
    ((ulong *)(unit[drive].trackbuf+(hdr.sec-1)*512))[1],
    ((ulong *)(unit[drive].trackbuf+(hdr.sec-1)*512))[2],
    ((ulong *)(unit[drive].trackbuf+(hdr.sec-1)*512))[3]);
    return MFM_DATA;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dos_encode_byte(byte: c_uchar) -> c_ushort {
    static inline ushort dos_encode_byte(unsigned char byte)
    {
    register unsigned char *enc, b2, b1;
    register ushort word;
    enc=mfmencode;
    b1=byte;
    b2=b1>>4;
    b1&=15;
    word=enc[b2] <<8 | enc [b1];
    return (word|((word&(256|64)) ? 0: 128));
    }
#[no_mangle]
unsafe extern "C" fn dos_encode_block(dest: *mut c_ushort, src: *mut c_uchar, len: c_int) {
    static void dos_encode_block(ushort *dest, unsigned char *src, int len)
    {
    int i;
    for (i = 0; i < len; i++) {
// dest=dos_encode_byte(*src++);
// dest|=((dest[-1]&1)||(*dest&0x4000))? 0: 0x8000;
    dest++;
    }
    }
    static unsigned long *ms_putsec(int drive, unsigned long *raw, int cnt)
    {
    static struct dos_header hdr={0,0,0,2,0,
    {78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78,78}};
    int i;
    static ushort crc[2]={0,0x4e4e};
    drive&=3;
// id gap 1
// the MFM word before is always 9254
    for(i=0;i<6;i++)
// raw++=0xaaaaaaaa;
// 3 sync + 1 headermark
// raw++=0x44894489;
// raw++=0x44895554;
// fill in the variable parts of the header
    hdr.track=unit[drive].track/unit[drive].type.heads;
    hdr.side=unit[drive].track%unit[drive].type.heads;
    hdr.sec=cnt+1;
    hdr.crc=dos_hdr_crc(&hdr);
// header (without "magic") and id gap 2
    dos_encode_block((ushort *)raw,(unsigned char *) &hdr.track,28);
    raw+=14;
// id gap 3
    for(i=0;i<6;i++)
// raw++=0xaaaaaaaa;
// 3 syncs and 1 datamark
// raw++=0x44894489;
// raw++=0x44895545;
// data
    dos_encode_block((ushort *)raw,
    (unsigned char *)unit[drive].trackbuf+cnt*512,512);
    raw+=256;
// data crc + jd's special gap (long words :-/)
    crc[0]=dos_data_crc(unit[drive].trackbuf+cnt*512);
    dos_encode_block((ushort *) raw,(unsigned char *)crc,4);
    raw+=2;
// data gap
    for(i=0;i<38;i++)
// raw++=0x92549254;
    return raw; /* wrote 652 MFM words */
    }
#[no_mangle]
unsafe extern "C" fn dos_write(disk: c_int) {
    static void dos_write(int disk)
    {
    int cnt;
    let mut raw: c_ulong = (unsigned long) raw_buf;
    unsigned long *ptr=(unsigned long *)raw;
    disk&=3;
// really gap4 + indexgap , but we write it first and round it up
    for (cnt=0;cnt<425;cnt++)
// ptr++=0x92549254;
// the following is just guessed
    if (unit[disk].type.sect_mult==2)  /* check for HD-Disks */
    for(cnt=0;cnt<473;cnt++)
// ptr++=0x92549254;
// now the index marks...
    for (cnt=0;cnt<20;cnt++)
// ptr++=0x92549254;
    for (cnt=0;cnt<6;cnt++)
// ptr++=0xaaaaaaaa;
// ptr++=0x52245224;
// ptr++=0x52245552;
    for (cnt=0;cnt<20;cnt++)
// ptr++=0x92549254;
// sectors
    for(cnt = 0; cnt < unit[disk].dtype.sects * unit[disk].type.sect_mult; cnt++)
    ptr=ms_putsec(disk,ptr,cnt);
// (ushort *)ptr = 0xaaa8; /* MFM word before is always 0x9254
    }
//
// Here comes the high level stuff (i.e. the filesystem interface)
// and helper functions.
// Normally this should be the only part that has to be adapted to
// different kernel versions.
//
// FIXME: this assumes the drive is still spinning -
// which is only true if we complete writing a track within three seconds
//
#[no_mangle]
unsafe extern "C" fn flush_track_callback(timer: *mut timer_list) {
    static void flush_track_callback(struct timer_list *timer)
    {
    unsigned long nr = ((unsigned long)timer -
    (unsigned long)&flush_track_timer[0]) /
    sizeof(flush_track_timer[0]);
    nr&=3;
    writefromint = 1;
    if (!try_fdc(nr)) {
// we might block in an interrupt, so try again later
    flush_track_timer[nr].expires = jiffies + 1;
    add_timer(flush_track_timer + nr);
    return;
    }
    get_fdc(nr);
    (*unit[nr].dtype.write_fkt)(nr);
    if (!raw_write(nr)) {
    printk (KERN_NOTICE "floppy disk write protected\n");
    writefromint = 0;
    writepending = 0;
    }
    rel_fdc();
    }
#[no_mangle]
unsafe extern "C" fn non_int_flush_track(nr: c_ulong) -> c_int {
    static int non_int_flush_track (unsigned long nr)
    {
    unsigned long flags;
    nr&=3;
    writefromint = 0;
    timer_delete(&post_write_timer);
    get_fdc(nr);
    if (!fd_motor_on(nr)) {
    writepending = 0;
    rel_fdc();
    return 0;
    }
    local_irq_save(flags);
    if (writepending != 2) {
    local_irq_restore(flags);
    (*unit[nr].dtype.write_fkt)(nr);
    if (!raw_write(nr)) {
    printk (KERN_NOTICE "floppy disk write protected "
    "in write!\n");
    writepending = 0;
    return 0;
    }
    wait_event(wait_fd_block, block_flag != 2);
    }
    else {
    local_irq_restore(flags);
    ms_delay(2); /* 2 ms post_write delay */
    post_write(nr);
    }
    rel_fdc();
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn get_track(drive: c_int, track: c_int) -> c_int {
    static int get_track(int drive, int track)
    {
    int error, errcnt;
    drive&=3;
    if (unit[drive].track == track)
    return 0;
    get_fdc(drive);
    if (!fd_motor_on(drive)) {
    rel_fdc();
    return -1;
    }
    if (unit[drive].dirty == 1) {
    timer_delete(flush_track_timer + drive);
    non_int_flush_track (drive);
    }
    errcnt = 0;
    while (errcnt < MAX_ERRORS) {
    if (!fd_seek(drive, track))
    return -1;
    raw_read(drive);
    error = (*unit[drive].dtype.read_fkt)(drive);
    if (error == 0) {
    rel_fdc();
    return 0;
    }
// Read Error Handling: recalibrate and try again
    unit[drive].track = -1;
    errcnt++;
    }
    rel_fdc();
    return -1;
    }
    static blk_status_t amiflop_rw_cur_segment(struct amiga_floppy_struct *floppy,
    struct request *rq)
    {
    let mut drive: c_int = floppy - unit;
    unsigned int cnt, block, track, sector;
    char *data;
    for (cnt = 0; cnt < blk_rq_cur_sectors(rq); cnt++) {

    printk("fd: sector %ld + %d requested for %s\n",
    blk_rq_pos(rq), cnt,
    (rq_data_dir(rq) == READ) ? "read" : "write");

    block = blk_rq_pos(rq) + cnt;
    track = block / (floppy.dtype.sects * floppy.type.sect_mult);
    sector = block % (floppy.dtype.sects * floppy.type.sect_mult);
    data = bio_data(rq.bio) + 512 * cnt;

    printk("access to track %d, sector %d, with buffer at "
    "0x%08lx\n", track, sector, data);

    if (get_track(drive, track) == -1)
    return BLK_STS_IOERR;
    if (rq_data_dir(rq) == READ) {
    memcpy(data, floppy.trackbuf + sector * 512, 512);
    } else {
    memcpy(floppy.trackbuf + sector * 512, data, 512);
// keep the drive spinning while writes are scheduled
    if (!fd_motor_on(drive))
    return BLK_STS_IOERR;
//
// setup a callback to write the track buffer
// after a short (1 tick) delay.
//
    floppy.dirty = 1;
// reset the timer
    mod_timer (flush_track_timer + drive, jiffies + 1);
    }
    }
    return BLK_STS_OK;
    }
    static blk_status_t amiflop_queue_rq(struct blk_mq_hw_ctx *hctx,
    const struct blk_mq_queue_data *bd)
    {
    struct request *rq = bd.rq;
    struct amiga_floppy_struct *floppy = rq.q.disk.private_data;
    blk_status_t err;
    if (!spin_trylock_irq(&amiflop_lock))
    return BLK_STS_DEV_RESOURCE;
    blk_mq_start_request(rq);
    do {
    err = amiflop_rw_cur_segment(floppy, rq);
    } while (blk_update_request(rq, err, blk_rq_cur_bytes(rq)));
    blk_mq_end_request(rq, err);
    spin_unlock_irq(&amiflop_lock);
    return BLK_STS_OK;
    }
#[no_mangle]
unsafe extern "C" fn fd_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> c_int {
    static int fd_getgeo(struct gendisk *disk, struct hd_geometry *geo)
    {
    struct amiga_floppy_struct *p = disk.private_data;
    geo.heads = p.type.heads;
    geo.sectors = p.dtype.sects * p.type.sect_mult;
    geo.cylinders = p.type.tracks;
    return 0;
    }
    static int fd_locked_ioctl(struct block_device *bdev, blk_mode_t mode,
    unsigned int cmd, unsigned long param)
    {
    struct amiga_floppy_struct *p = bdev.bd_disk.private_data;
    let mut drive: c_int = p - unit;
    static struct floppy_struct getprm;
    void __user *argp = (void __user *)param;
    switch(cmd){
    case FDFMTBEG:
    get_fdc(drive);
    if (fd_ref[drive] > 1) {
    rel_fdc();
    return -EBUSY;
    }
    if (fd_motor_on(drive) == 0) {
    rel_fdc();
    return -ENODEV;
    }
    if (fd_calibrate(drive) == 0) {
    rel_fdc();
    return -ENXIO;
    }
    floppy_off(drive);
    rel_fdc();
    break;
    case FDFMTTRK:
    if (param < p.type.tracks * p.type.heads)
    {
    get_fdc(drive);
    if (fd_seek(drive,param) != 0){
    memset(p.trackbuf, FD_FILL_BYTE,
    p.dtype.sects * p.type.sect_mult * 512);
    non_int_flush_track(drive);
    }
    floppy_off(drive);
    rel_fdc();
    }
    else
    return -EINVAL;
    break;
    case FDFMTEND:
    floppy_off(drive);
    invalidate_bdev(bdev);
    break;
    case FDGETPRM:
    memset((void *)&getprm, 0, sizeof (getprm));
    getprm.track=p.type.tracks;
    getprm.head=p.type.heads;
    getprm.sect=p.dtype.sects * p.type.sect_mult;
    getprm.size=p.blocks;
    if (copy_to_user(argp, &getprm, sizeof(struct floppy_struct)))
    return -EFAULT;
    break;
    case FDSETPRM:
    case FDDEFPRM:
    return -EINVAL;
    case FDFLUSH: /* unconditionally, even if not needed */
    timer_delete(flush_track_timer + drive);
    non_int_flush_track(drive);
    break;

    case IOCTL_RAW_TRACK:
    if (copy_to_user(argp, raw_buf, p.type.read_size))
    return -EFAULT;
    else
    return p.type.read_size;

    default:
    return -ENOSYS;
    }
    return 0;
    }
    static int fd_ioctl(struct block_device *bdev, blk_mode_t mode,
    unsigned int cmd, unsigned long param)
    {
    int ret;
    mutex_lock(&amiflop_mutex);
    ret = fd_locked_ioctl(bdev, mode, cmd, param);
    mutex_unlock(&amiflop_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fd_probe(dev: c_int) {
    static void fd_probe(int dev)
    {
    unsigned long code;
    int type;
    int drive;
    drive = dev & 3;
    code = fd_get_drive_id(drive);
// get drive type
    for (type = 0; type < num_dr_types; type++)
    if (drive_types[type].code == code)
    break;
    if (type >= num_dr_types) {
    printk(KERN_WARNING "fd_probe: unsupported drive type "
    "%08lx found\n", code);
    unit[drive].type = &drive_types[num_dr_types-1]; /* FD_NODRIVE */
    return;
    }
    unit[drive].type = drive_types + type;
    unit[drive].track = -1;
    unit[drive].disk = -1;
    unit[drive].motor = 0;
    unit[drive].busy = 0;
    unit[drive].status = -1;
    }
//
// floppy_open check for aliasing (/dev/fd0 can be the same as
// /dev/PS0 etc), and disallows simultaneous access to the same
// drive with different device numbers.
//
#[no_mangle]
unsafe extern "C" fn floppy_open(disk: *mut gendisk, mode: blk_mode_t) -> c_int {
    static int floppy_open(struct gendisk *disk, blk_mode_t mode)
    {
    let mut drive: c_int = disk.first_minor & 3;
    let mut system: c_int = (disk.first_minor & 4) >> 2;
    int old_dev;
    unsigned long flags;
    mutex_lock(&amiflop_mutex);
    old_dev = fd_device[drive];
    if (fd_ref[drive] && old_dev != system) {
    mutex_unlock(&amiflop_mutex);
    return -EBUSY;
    }
    if (unit[drive].type.code == FD_NODRIVE) {
    mutex_unlock(&amiflop_mutex);
    return -ENXIO;
    }
    if (mode & (BLK_OPEN_READ | BLK_OPEN_WRITE)) {
    disk_check_media_change(disk);
    if (mode & BLK_OPEN_WRITE) {
    int wrprot;
    get_fdc(drive);
    fd_select (drive);
    wrprot = !(ciaa.pra & DSKPROT);
    fd_deselect (drive);
    rel_fdc();
    if (wrprot) {
    mutex_unlock(&amiflop_mutex);
    return -EROFS;
    }
    }
    }
    local_irq_save(flags);
    fd_ref[drive]++;
    fd_device[drive] = system;
    local_irq_restore(flags);
    unit[drive].dtype=&data_types[system];
    unit[drive].blocks=unit[drive].type.heads*unit[drive].type.tracks*
    data_types[system].sects*unit[drive].type.sect_mult;
    set_capacity(unit[drive].gendisk[system], unit[drive].blocks);
    printk(KERN_INFO "fd%d: accessing %s-disk with %s-layout\n",drive,
    unit[drive].type.name, data_types[system].name);
    mutex_unlock(&amiflop_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn floppy_release(disk: *mut gendisk) {
    static void floppy_release(struct gendisk *disk)
    {
    struct amiga_floppy_struct *p = disk.private_data;
    let mut drive: c_int = p - unit;
    mutex_lock(&amiflop_mutex);
    if (unit[drive].dirty == 1) {
    timer_delete(flush_track_timer + drive);
    non_int_flush_track (drive);
    }
    if (!fd_ref[drive]--) {
    printk(KERN_CRIT "floppy_release with fd_ref == 0");
    fd_ref[drive] = 0;
    }

    floppy_off (drive);

    mutex_unlock(&amiflop_mutex);
    }
//
// check_events is never called from an interrupt, so we can relax a bit
// here, sleep etc. Note that floppy-on tries to set current_DOR to point
// to the desired drive, but it will probably not survive the sleep if
// several floppies are used at the same time: thus the loop.
//
#[no_mangle]
unsafe extern "C" fn amiga_check_events(disk: *mut gendisk, clearing: c_uint) -> unsigned {
    static unsigned amiga_check_events(struct gendisk *disk, unsigned int clearing)
    {
    struct amiga_floppy_struct *p = disk.private_data;
    let mut drive: c_int = p - unit;
    int changed;
    let mut first_time: static int = 1;
    if (first_time)
    changed = first_time--;
    else {
    get_fdc(drive);
    fd_select (drive);
    changed = !(ciaa.pra & DSKCHANGE);
    fd_deselect (drive);
    rel_fdc();
    }
    if (changed) {
    fd_probe(drive);
    p.track = -1;
    p.dirty = 0;
    writepending = 0; /* if this was true before, too bad! */
    writefromint = 0;
    return DISK_EVENT_MEDIA_CHANGE;
    }
    return 0;
    }
    static const struct block_device_operations floppy_fops = {
    .owner		= THIS_MODULE,
    .open		= floppy_open,
    .release	= floppy_release,
    .ioctl		= fd_ioctl,
    .getgeo		= fd_getgeo,
    .check_events	= amiga_check_events,
    };
    static const struct blk_mq_ops amiflop_mq_ops = {
    .queue_rq = amiflop_queue_rq,
    };
#[no_mangle]
unsafe extern "C" fn fd_alloc_disk(drive: c_int, system: c_int) -> c_int {
    static int fd_alloc_disk(int drive, int system)
    {
    struct queue_limits lim = {
    .features		= BLK_FEAT_ROTATIONAL,
    };
    struct gendisk *disk;
    int err;
    disk = blk_mq_alloc_disk(&unit[drive].tag_set, &lim, core::ptr::null_mut());
    if (IS_ERR(disk))
    return PTR_ERR(disk);
    disk.major = FLOPPY_MAJOR;
    disk.first_minor = drive + system;
    disk.minors = 1;
    disk.fops = &floppy_fops;
    disk.flags |= GENHD_FL_NO_PART;
    disk.events = DISK_EVENT_MEDIA_CHANGE;
    if (system)
    sprintf(disk.disk_name, "fd%d_msdos", drive);
    else
    sprintf(disk.disk_name, "fd%d", drive);
    disk.private_data = &unit[drive];
    set_capacity(disk, 880 * 2);
    unit[drive].gendisk[system] = disk;
    err = add_disk(disk);
    if (err)
    put_disk(disk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fd_alloc_drive(drive: c_int) -> c_int {
    static int fd_alloc_drive(int drive)
    {
    unit[drive].trackbuf = kmalloc(FLOPPY_MAX_SECTORS * 512, GFP_KERNEL);
    if (!unit[drive].trackbuf)
    goto out;
    memset(&unit[drive].tag_set, 0, sizeof(unit[drive].tag_set));
    unit[drive].tag_set.ops = &amiflop_mq_ops;
    unit[drive].tag_set.nr_hw_queues = 1;
    unit[drive].tag_set.nr_maps = 1;
    unit[drive].tag_set.queue_depth = 2;
    unit[drive].tag_set.numa_node = NUMA_NO_NODE;
    if (blk_mq_alloc_tag_set(&unit[drive].tag_set))
    goto out_cleanup_trackbuf;
    pr_cont(" fd%d", drive);
    if (fd_alloc_disk(drive, 0) || fd_alloc_disk(drive, 1))
    goto out_cleanup_tagset;
    return 0;
    out_cleanup_tagset:
    blk_mq_free_tag_set(&unit[drive].tag_set);
    out_cleanup_trackbuf:
    kfree(unit[drive].trackbuf);
    out:
    unit[drive].type.code = FD_NODRIVE;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn fd_probe_drives() -> int __init {
    static int __init fd_probe_drives(void)
    {
    int drive,drives,nomem;
    pr_info("FD: probing units\nfound");
    drives=0;
    nomem=0;
    for(drive=0;drive<FD_MAX_UNITS;drive++) {
    fd_probe(drive);
    if (unit[drive].type.code == FD_NODRIVE)
    continue;
    if (fd_alloc_drive(drive) < 0) {
    pr_cont(" no mem for fd%d", drive);
    nomem = 1;
    continue;
    }
    drives++;
    }
    if ((drives > 0) || (nomem == 0)) {
    if (drives == 0)
    pr_cont(" no drives");
    pr_cont("\n");
    return drives;
    }
    pr_cont("\n");
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn amiga_floppy_probe(pdev: *mut platform_device) -> int __init {
    static int __init amiga_floppy_probe(struct platform_device *pdev)
    {
    int i, ret;
    if (register_blkdev(FLOPPY_MAJOR,"fd"))
    return -EBUSY;
    ret = -ENOMEM;
    raw_buf = amiga_chip_alloc(RAW_BUF_SIZE, "Floppy");
    if (!raw_buf) {
    printk("fd: cannot get chip mem buffer\n");
    goto out_blkdev;
    }
    ret = -EBUSY;
    if (request_irq(IRQ_AMIGA_DSKBLK, fd_block_done, 0, "floppy_dma", core::ptr::null_mut())) {
    printk("fd: cannot get irq for dma\n");
    goto out_irq;
    }
    if (request_irq(IRQ_AMIGA_CIAA_TB, ms_isr, 0, "floppy_timer", core::ptr::null_mut())) {
    printk("fd: cannot get irq for timer\n");
    goto out_irq2;
    }
    ret = -ENODEV;
    if (fd_probe_drives() < 1) /* No usable drives */
    goto out_probe;
// initialize variables
    timer_setup(&motor_on_timer, motor_on_callback, 0);
    motor_on_timer.expires = 0;
    for (i = 0; i < FD_MAX_UNITS; i++) {
    timer_setup(&motor_off_timer[i], fd_motor_off, 0);
    motor_off_timer[i].expires = 0;
    timer_setup(&flush_track_timer[i], flush_track_callback, 0);
    flush_track_timer[i].expires = 0;
    unit[i].track = -1;
    }
    timer_setup(&post_write_timer, post_write_callback, 0);
    post_write_timer.expires = 0;
    for (i = 0; i < 128; i++)
    mfmdecode[i]=255;
    for (i = 0; i < 16; i++)
    mfmdecode[mfmencode[i]]=i;
// make sure that disk DMA is enabled
    custom.dmacon = DMAF_SETCLR | DMAF_DISK;
// init ms timer
    ciaa.crb = 8; /* one-shot, stop */
    return 0;
    out_probe:
    free_irq(IRQ_AMIGA_CIAA_TB, core::ptr::null_mut());
    out_irq2:
    free_irq(IRQ_AMIGA_DSKBLK, core::ptr::null_mut());
    out_irq:
    amiga_chip_free(raw_buf);
    out_blkdev:
    unregister_blkdev(FLOPPY_MAJOR,"fd");
    return ret;
    }
    static struct platform_driver amiga_floppy_driver = {
    .driver   = {
    .name	= "amiga-floppy",
    },
    };
#[no_mangle]
unsafe extern "C" fn amiga_floppy_init() -> int __init {
    static int __init amiga_floppy_init(void)
    {
    return platform_driver_probe(&amiga_floppy_driver, amiga_floppy_probe);
    }
    module_init(amiga_floppy_init);

#[no_mangle]
unsafe extern "C" fn amiga_floppy_setup(str: *mut c_char) -> int __init {
    static int __init amiga_floppy_setup (char *str)
    {
    int n;
    if (!MACH_IS_AMIGA)
    return 0;
    if (!get_option(&str, &n))
    return 0;
    printk (KERN_INFO "amiflop: Setting default df0 to %x\n", n);
    fd_def_df0 = n;
    return 1;
    }
    __setup("floppy=", amiga_floppy_setup);

    MODULE_ALIAS("platform:amiga-floppy");
