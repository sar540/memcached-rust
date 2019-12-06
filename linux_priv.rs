#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
}
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type sigval_t = sigval;
pub type __sigchld_clock_t = __clock_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_7,
    pub _timer: C2RustUnnamed_6,
    pub _rt: C2RustUnnamed_5,
    pub _sigchld: C2RustUnnamed_4,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub si_addr_bnd: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __sigchld_clock_t,
    pub si_stime: __sigchld_clock_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: sigval_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_8,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_8 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
static mut kill_msg: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn handle_syssig(mut signum: libc::c_int,
                                   mut si: *mut siginfo_t,
                                   mut thread_context: *mut libc::c_void) {
    let mut syscall_no: libc::c_int = (*si)._sifields._sigsys._syscall;
    // Replace the characters in the kill message with the syscall number. We
    // can't safely printf (even write is not really valid, but we're crashing
    // anyway).
    *kill_msg.offset(39) =
        (syscall_no / 100i32 % 10i32 + '0' as i32) as libc::c_char;
    *kill_msg.offset(40) =
        (syscall_no / 10i32 % 10i32 + '0' as i32) as libc::c_char;
    *kill_msg.offset(41) = (syscall_no % 10i32 + '0' as i32) as libc::c_char;
    write(2i32, kill_msg, strlen(kill_msg)) == -1i32;
    // We can't use the nice exit() version because it causes at_exit handlers
    // to be looked up and run. We can't take any locks while handling the
    // signal, so _exit() is the only thing to do safely.
    _exit(1i32);
}
static mut act: sigaction =
    unsafe {
        {
            let mut init =
                sigaction{__sigaction_handler:
                              C2RustUnnamed_8{sa_sigaction:
                                                  Some(handle_syssig as
                                                           unsafe extern "C" fn(_:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    *mut siginfo_t,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               -> ()),},
                          sa_mask: __sigset_t{__val: [0; 16],},
                          sa_flags: 4i32,
                          sa_restorer: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn setup_privilege_violations_handler() {
    kill_msg =
        malloc(strlen(b"Seccomp policy failure. Caught syscall ???. 
					  This is either an exploit attempt, or your system is not supported yet.\n\x00"
                          as *const u8 as
                          *const libc::c_char).wrapping_add(1i32 as
                                                                libc::c_ulong))
            as *mut libc::c_char;
    strcpy(kill_msg,
           b"Seccomp policy failure.
		   Caught syscall ???. This is either an exploit attempt, or your system is not supported yet.\n\x00"
               as *const u8 as *const libc::c_char);
    sigaction(31i32, &act, 0 as *mut sigaction);
}
