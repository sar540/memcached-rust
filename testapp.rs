#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, main)]
extern "C" {
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pid_t = __pid_t;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type sigval_t = sigval;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type pthread_t = libc::c_ulong;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type __sigchld_clock_t = __clock_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub si_addr_bnd: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __sigchld_clock_t,
    pub si_stime: __sigchld_clock_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: sigval_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_8,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_8 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulong;
pub type in_port_t = uint16_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type test_return = libc::c_uint;
pub const TEST_FAIL: test_return = 2;
pub const TEST_PASS: test_return = 1;
pub const TEST_SKIP: test_return = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct conn {
    pub sock: libc::c_int,
    pub read: Option<unsafe extern "C" fn(_: *mut conn, _: *mut libc::c_void,
                                          _: size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(_: *mut conn,
                                           _: *const libc::c_void, _: size_t)
                          -> ssize_t>,
}
pub type TEST_FUNC = Option<unsafe extern "C" fn() -> test_return>;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_9 {
    pub request: libc::c_int,
    pub response: libc::c_int,
    pub bytes: [libc::c_char; 1024],
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_10 {
    pub request: libc::c_int,
    pub response: libc::c_int,
    pub bytes: [libc::c_char; 1024],
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_11 {
    pub request: libc::c_int,
    pub response: libc::c_int,
    pub bytes: [libc::c_char; 1024],
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_12 {
    pub request: libc::c_int,
    pub response: libc::c_int,
    pub bytes: [libc::c_char; 1024],
}
pub const max: C2RustUnnamed_13 = 2;
pub type C2RustUnnamed_13 = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct testcase {
    pub description: *const libc::c_char,
    pub function: TEST_FUNC,
}
static mut server_pid: pid_t = 0;
static mut con: *mut conn = 0 as *const conn as *mut conn;
unsafe extern "C" fn close_conn() {
    if con.is_null() { return }
    if (*con).sock > 0 as libc::c_int { close((*con).sock); }
    free(con as *mut libc::c_void);
    con = 0 as *mut conn;
}
unsafe extern "C" fn cache_create_test() -> test_return { return TEST_PASS; }
#[no_mangle]
pub static mut constructor_pattern: uint64_t =
    0xdeadcafebabebeef as libc::c_ulong;
unsafe extern "C" fn cache_constructor_test() -> test_return {
    let mut ptr: *mut uint64_t = 0 as *mut uint64_t;
    let mut pattern: uint64_t = *ptr;
    return if pattern == constructor_pattern {
               TEST_PASS as libc::c_int
           } else { TEST_FAIL as libc::c_int } as test_return;
}
unsafe extern "C" fn cache_fail_constructor_test() -> test_return {
    let mut ret: test_return = TEST_PASS;
    let mut ptr: *mut uint64_t = 0 as *mut uint64_t;
    if !ptr.is_null() { ret = TEST_FAIL }
    return ret;
}
static mut destruct_data: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
unsafe extern "C" fn cache_destructor_test() -> test_return {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    return if ptr == destruct_data as *mut libc::c_char {
               TEST_PASS as libc::c_int
           } else { TEST_FAIL as libc::c_int } as test_return;
}
unsafe extern "C" fn cache_reuse_test() -> test_return {
    let mut ii: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ii = 0 as libc::c_int;
    while ii < 100 as libc::c_int {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if p == ptr {
        } else {
            __assert_fail(b"p == ptr\x00" as *const u8 as *const libc::c_char,
                          b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                              *const libc::c_char,
                          168 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"enum test_return cache_reuse_test(void)\x00")).as_ptr());
        };
        ii += 1
    }
    return TEST_PASS;
}
unsafe extern "C" fn cache_bulkalloc(mut datasize: size_t) -> test_return {
    let mut ptr: [*mut libc::c_void; 1024] = [0 as *mut libc::c_void; 1024];
    let mut ii: libc::c_int = 0 as libc::c_int;
    while ii < 1024 as libc::c_int {
        if !ptr[ii as usize].is_null() {
        } else {
            __assert_fail(b"ptr[ii] != 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                              *const libc::c_char,
                          185 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 41],
                                                    &[libc::c_char; 41]>(b"enum test_return cache_bulkalloc(size_t)\x00")).as_ptr());
        };
        memset(ptr[ii as usize], 0xff as libc::c_int, datasize);
        ii += 1
    }
    let mut ii_0: libc::c_int = 0 as libc::c_int;
    while ii_0 < 1024 as libc::c_int { ii_0 += 1 }
    return TEST_PASS;
}
unsafe extern "C" fn test_issue_161() -> test_return {
    let mut ret: test_return = cache_bulkalloc(1 as libc::c_int as size_t);
    if ret as libc::c_uint == TEST_PASS as libc::c_int as libc::c_uint {
        ret = cache_bulkalloc(512 as libc::c_int as size_t)
    }
    return ret;
}
unsafe extern "C" fn cache_redzone_test() -> test_return {
    /* Ignore SIGABRT */
    let mut old_action: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_8{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut action: sigaction =
        {
            let mut init =
                sigaction{__sigaction_handler:
                              C2RustUnnamed_8{sa_handler:
                                                  ::std::mem::transmute::<libc::intptr_t,
                                                                          __sighandler_t>(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::intptr_t),},
                          sa_mask: __sigset_t{__val: [0; 16],},
                          sa_flags: 0 as libc::c_int,
                          sa_restorer: None,};
            init
        };
    sigemptyset(&mut action.sa_mask);
    sigaction(6 as libc::c_int, &mut action, &mut old_action);
    /* check memory debug.. */
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old: libc::c_char = *p.offset(-(1 as libc::c_int as isize));
    *p.offset(-(1 as libc::c_int as isize)) =
        0 as libc::c_int as libc::c_char;
    *p.offset(-(1 as libc::c_int as isize)) = old;
    *p.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as isize) =
        0 as libc::c_int as libc::c_char;
    /* restore signal handler */
    sigaction(6 as libc::c_int, &mut old_action,
              0 as *mut sigaction); // empty
    return TEST_PASS; // non-numeric
}
unsafe extern "C" fn test_safe_strtoul() -> test_return {
    let mut val: uint32_t = 0; // non-numeric
    if safe_strtoul(b"123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoul(\"123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      245 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      246 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if safe_strtoul(b"+123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoul(\"+123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      247 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      248 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if safe_strtoul(b"\x00" as *const u8 as *const libc::c_char, &mut val) ==
           0 {
    } else {
        __assert_fail(b"!safe_strtoul(\"\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      249 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if safe_strtoul(b"123BOGUS\x00" as *const u8 as *const libc::c_char,
                    &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoul(\"123BOGUS\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      250 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    if safe_strtoul(b" issue221\x00" as *const u8 as *const libc::c_char,
                    &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoul(\" issue221\", &val)\x00" as *const u8
                          as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      251 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    /* Not sure what it does, but this works with ICC :/
       assert(!safe_strtoul("92837498237498237498029383", &val)); // out of range
    */
    // extremes:
    if safe_strtoul(b"4294967295\x00" as *const u8 as *const libc::c_char,
                    &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtoul(\"4294967295\", &val)\x00" as *const u8
                          as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      257 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr()); // 2**32 - 1
    };
    if val as libc::c_long == 4294967295 as libc::c_long {
    } else {
        __assert_fail(b"val == 4294967295L\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      258 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr());
    };
    /* This actually works on 64-bit ubuntu
       assert(!safe_strtoul("4294967296", &val)); // 2**32
    */
    if safe_strtoul(b"-1\x00" as *const u8 as *const libc::c_char, &mut val)
           == 0 {
    } else {
        __assert_fail(b"!safe_strtoul(\"-1\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      262 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoul(void)\x00")).as_ptr()); // negative
    }; // empty
    return TEST_PASS; // non-numeric
}
unsafe extern "C" fn test_safe_strtoull() -> test_return {
    let mut val: uint64_t = 0; // out of range
    if safe_strtoull(b"123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoull(\"123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      269 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr()); // non-numeric
    };
    if val == 123 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      270 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if safe_strtoull(b"+123\x00" as *const u8 as *const libc::c_char,
                     &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtoull(\"+123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      271 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      272 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if safe_strtoull(b"\x00" as *const u8 as *const libc::c_char, &mut val) ==
           0 {
    } else {
        __assert_fail(b"!safe_strtoull(\"\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      273 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if safe_strtoull(b"123BOGUS\x00" as *const u8 as *const libc::c_char,
                     &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoull(\"123BOGUS\", &val)\x00" as *const u8
                          as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      274 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if safe_strtoull(b"92837498237498237498029383\x00" as *const u8 as
                         *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoull(\"92837498237498237498029383\", &val)\x00"
                          as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      275 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    if safe_strtoull(b" issue221\x00" as *const u8 as *const libc::c_char,
                     &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoull(\" issue221\", &val)\x00" as *const u8
                          as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      276 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr());
    };
    // extremes:
    if safe_strtoull(b"18446744073709551615\x00" as *const u8 as
                         *const libc::c_char, &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtoull(\"18446744073709551615\", &val)\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      279 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr()); // 2**64 - 1
    }; // 2**64
    if val as libc::c_ulonglong == 18446744073709551615 as libc::c_ulonglong {
    } else {
        __assert_fail(b"val == 18446744073709551615ULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      280 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr()); // negative
    }; // empty
    if safe_strtoull(b"18446744073709551616\x00" as *const u8 as
                         *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoull(\"18446744073709551616\", &val)\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      281 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr()); // non-numeric
    }; // out of range
    if safe_strtoull(b"-1\x00" as *const u8 as *const libc::c_char, &mut val)
           == 0 {
    } else {
        __assert_fail(b"!safe_strtoull(\"-1\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      282 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"enum test_return test_safe_strtoull(void)\x00")).as_ptr()); // non-numeric
    };
    return TEST_PASS;
}
unsafe extern "C" fn test_safe_strtoll() -> test_return {
    let mut val: int64_t = 0;
    if safe_strtoll(b"123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoll(\"123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      288 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      289 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b"+123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoll(\"+123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      290 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      291 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b"-123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtoll(\"-123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      292 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if val == -(123 as libc::c_int) as libc::c_long {
    } else {
        __assert_fail(b"val == -123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      293 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b"\x00" as *const u8 as *const libc::c_char, &mut val) ==
           0 {
    } else {
        __assert_fail(b"!safe_strtoll(\"\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      294 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b"123BOGUS\x00" as *const u8 as *const libc::c_char,
                    &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoll(\"123BOGUS\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      295 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b"92837498237498237498029383\x00" as *const u8 as
                        *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoll(\"92837498237498237498029383\", &val)\x00"
                          as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      296 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if safe_strtoll(b" issue221\x00" as *const u8 as *const libc::c_char,
                    &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoll(\" issue221\", &val)\x00" as *const u8
                          as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      297 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    // extremes:
    if safe_strtoll(b"18446744073709551615\x00" as *const u8 as
                        *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoll(\"18446744073709551615\", &val)\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      300 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr()); // 2**64 - 1
    }; // 2**63 - 1
    if safe_strtoll(b"9223372036854775807\x00" as *const u8 as
                        *const libc::c_char, &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtoll(\"9223372036854775807\", &val)\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      301 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    if val as libc::c_longlong == 9223372036854775807 as libc::c_longlong {
    } else {
        __assert_fail(b"val == 9223372036854775807LL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      302 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr());
    };
    /*
      assert(safe_strtoll("-9223372036854775808", &val)); // -2**63
      assert(val == -9223372036854775808LL);
    */
    if safe_strtoll(b"-9223372036854775809\x00" as *const u8 as
                        *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtoll(\"-9223372036854775809\", &val)\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      307 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr()); // -2**63 - 1
    };
    // We'll allow space to terminate the string.  And leading space.
    if safe_strtoll(b" 123 foo\x00" as *const u8 as *const libc::c_char,
                    &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtoll(\" 123 foo\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      310 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr()); // empty
    }; // non-numeric
    if val == 123 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      311 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"enum test_return test_safe_strtoll(void)\x00")).as_ptr()); // out of range
    }; // non-numeric
    return TEST_PASS;
}
unsafe extern "C" fn test_safe_strtol() -> test_return {
    let mut val: int32_t = 0;
    if safe_strtol(b"123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtol(\"123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      317 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      318 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b"+123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtol(\"+123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      319 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      320 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b"-123\x00" as *const u8 as *const libc::c_char, &mut val)
           != 0 {
    } else {
        __assert_fail(b"safe_strtol(\"-123\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      321 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if val == -(123 as libc::c_int) {
    } else {
        __assert_fail(b"val == -123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      322 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b"\x00" as *const u8 as *const libc::c_char, &mut val) == 0
       {
    } else {
        __assert_fail(b"!safe_strtol(\"\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      323 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b"123BOGUS\x00" as *const u8 as *const libc::c_char,
                   &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtol(\"123BOGUS\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      324 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b"92837498237498237498029383\x00" as *const u8 as
                       *const libc::c_char, &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtol(\"92837498237498237498029383\", &val)\x00"
                          as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      325 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if safe_strtol(b" issue221\x00" as *const u8 as *const libc::c_char,
                   &mut val) == 0 {
    } else {
        __assert_fail(b"!safe_strtol(\" issue221\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      326 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    // extremes:
    /* This actually works on 64-bit ubuntu
       assert(!safe_strtol("2147483648", &val)); // (expt 2.0 31.0)
    */
    if safe_strtol(b"2147483647\x00" as *const u8 as *const libc::c_char,
                   &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtol(\"2147483647\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      332 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr()); // (- (expt 2.0 31) 1)
    };
    if val as libc::c_long == 2147483647 as libc::c_long {
    } else {
        __assert_fail(b"val == 2147483647L\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      333 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    /* This actually works on 64-bit ubuntu
       assert(!safe_strtol("-2147483649", &val)); // (- (expt -2.0 31) 1)
    */
    // We'll allow space to terminate the string.  And leading space.
    if safe_strtol(b" 123 foo\x00" as *const u8 as *const libc::c_char,
                   &mut val) != 0 {
    } else {
        __assert_fail(b"safe_strtol(\" 123 foo\", &val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      339 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    if val == 123 as libc::c_int {
    } else {
        __assert_fail(b"val == 123\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      340 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"enum test_return test_safe_strtol(void)\x00")).as_ptr());
    };
    return TEST_PASS;
}
unsafe extern "C" fn test_issue_44() -> test_return {
    let mut port: in_port_t = 0;
    let mut pid: pid_t = 0;
    if kill(pid, 1 as libc::c_int) == 0 as libc::c_int {
    } else {
        __assert_fail(b"kill(pid, SIGHUP) == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      487 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_44(void)\x00")).as_ptr());
    };
    sleep(1 as libc::c_int as libc::c_uint);
    if kill(pid, 15 as libc::c_int) == 0 as libc::c_int {
    } else {
        __assert_fail(b"kill(pid, SIGTERM) == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      489 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_44(void)\x00")).as_ptr());
    };
    return TEST_PASS;
}
unsafe extern "C" fn test_vperror() -> test_return {
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut oldstderr: libc::c_int = dup(2 as libc::c_int);
    if oldstderr >= 0 as libc::c_int {
    } else {
        __assert_fail(b"oldstderr >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      592 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    let mut tmpl: [libc::c_char; 24] = [0; 24];
    strncpy(tmpl.as_mut_ptr(),
            b"/tmp/test_file.XXXXXXX\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 23]>() as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong));
    let mut newfile: libc::c_int = mkstemp(tmpl.as_mut_ptr());
    if newfile > 0 as libc::c_int {
    } else {
        __assert_fail(b"newfile > 0\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      597 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    rv = dup2(newfile, 2 as libc::c_int);
    if rv == 2 as libc::c_int {
    } else {
        __assert_fail(b"rv == STDERR_FILENO\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      599 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    rv = close(newfile);
    if rv == 0 as libc::c_int {
    } else {
        __assert_fail(b"rv == 0\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      601 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    *__errno_location() = 5 as libc::c_int;
    vperror(b"Old McDonald had a farm.  %s\x00" as *const u8 as
                *const libc::c_char,
            b"EI EIO\x00" as *const u8 as *const libc::c_char);
    /* Restore stderr */
    rv = dup2(oldstderr, 2 as libc::c_int);
    if rv == 2 as libc::c_int {
    } else {
        __assert_fail(b"rv == STDERR_FILENO\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      608 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    /* Go read the file */
    let mut buf: [libc::c_char; 80] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut efile: *mut FILE =
        fopen(tmpl.as_mut_ptr(),
              b"r\x00" as *const u8 as *const libc::c_char);
    if !efile.is_null() {
    } else {
        __assert_fail(b"efile\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      614 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    let mut prv: *mut libc::c_char =
        fgets(buf.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as
                  libc::c_int, efile);
    if !prv.is_null() {
    } else {
        __assert_fail(b"prv\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      616 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"enum test_return test_vperror(void)\x00")).as_ptr());
    };
    fclose(efile);
    unlink(tmpl.as_mut_ptr());
    let mut expected: [libc::c_char; 80] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    snprintf(expected.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
             b"Old McDonald had a farm.  EI EIO: %s\n\x00" as *const u8 as
                 *const libc::c_char, strerror(5 as libc::c_int));
    /*
    fprintf(stderr,
            "\nExpected:  ``%s''"
            "\nGot:       ``%s''\n", expected, buf);
    */
    return if strcmp(expected.as_mut_ptr(), buf.as_mut_ptr()) ==
                  0 as libc::c_int {
               TEST_PASS as libc::c_int
           } else { TEST_FAIL as libc::c_int } as test_return;
}
unsafe extern "C" fn send_ascii_command(mut buf: *const libc::c_char) {
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let mut ptr: *const libc::c_char = buf;
    let mut len: size_t = strlen(buf);
    loop  {
        let mut nw: ssize_t =
            (*con).write.expect("non-null function pointer")(con as
                                                                 *mut libc::c_void
                                                                 as *mut conn,
                                                             ptr.offset(offset
                                                                            as
                                                                            isize)
                                                                 as
                                                                 *const libc::c_void,
                                                             len.wrapping_sub(offset
                                                                                  as
                                                                                  libc::c_ulong));
        if nw == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() != 4 as libc::c_int {
                fprintf(stderr,
                        b"Failed to write: %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
                abort();
            }
        } else { offset += nw }
        if !((offset as libc::c_ulong) < len) { break ; }
    };
}
/*
 * This is a dead slow single byte read, but it should only read out
 * _one_ response and I don't have an input buffer... The current
 * implementation only supports single-line responses, so if you want to use
 * it for get commands you need to implement that first ;-)
 */
unsafe extern "C" fn read_ascii_response(mut buffer: *mut libc::c_char,
                                         mut size: size_t) {
    let mut offset: off_t = 0 as libc::c_int as off_t;
}
unsafe extern "C" fn test_issue_92() -> test_return {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      685 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_92(void)\x00")).as_ptr());
    };
    send_ascii_command(b"stats cachedump 1 0 0\r\n\x00" as *const u8 as
                           *const libc::c_char);
    read_ascii_response(buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong);
    if strncmp(buffer.as_mut_ptr(),
               b"END\x00" as *const u8 as *const libc::c_char,
               strlen(b"END\x00" as *const u8 as *const libc::c_char)) ==
           0 as libc::c_int {
    } else {
        __assert_fail(b"strncmp(buffer, \"END\", strlen(\"END\")) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      690 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_92(void)\x00")).as_ptr());
    };
    send_ascii_command(b"stats cachedump 200 0 0\r\n\x00" as *const u8 as
                           *const libc::c_char);
    read_ascii_response(buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong);
    if strncmp(buffer.as_mut_ptr(),
               b"CLIENT_ERROR\x00" as *const u8 as *const libc::c_char,
               strlen(b"CLIENT_ERROR\x00" as *const u8 as
                          *const libc::c_char)) == 0 as libc::c_int {
    } else {
        __assert_fail(b"strncmp(buffer, \"CLIENT_ERROR\", strlen(\"CLIENT_ERROR\")) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      694 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_92(void)\x00")).as_ptr());
    };
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      698 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"enum test_return test_issue_92(void)\x00")).as_ptr());
    };
    return TEST_PASS;
}
unsafe extern "C" fn test_issue_102() -> test_return {
    let mut buffer: [libc::c_char; 4096] = [0; 4096];
    memset(buffer.as_mut_ptr() as *mut libc::c_void, ' ' as i32,
           ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong);
    buffer[(::std::mem::size_of::<[libc::c_char; 4096]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
               as usize] = '\u{0}' as i32 as libc::c_char;
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      709 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    send_ascii_command(buffer.as_mut_ptr());
    /* verify that the server closed the connection */
    if (*con).read.expect("non-null function pointer")(con,
                                                       buffer.as_mut_ptr() as
                                                           *mut libc::c_void,
                                                       ::std::mem::size_of::<[libc::c_char; 4096]>()
                                                           as libc::c_ulong)
           == 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"con->read(con, buffer, sizeof(buffer)) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      713 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      717 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    snprintf(buffer.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
             b"gets \x00" as *const u8 as *const libc::c_char);
    let mut offset: size_t = 5 as libc::c_int as size_t;
    while offset < 4000 as libc::c_int as libc::c_ulong {
        offset =
            (offset as
                 libc::c_ulong).wrapping_add(snprintf(buffer.as_mut_ptr().offset(offset
                                                                                     as
                                                                                     isize),
                                                      (::std::mem::size_of::<[libc::c_char; 4096]>()
                                                           as
                                                           libc::c_ulong).wrapping_sub(offset),
                                                      b"%010u \x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      offset as libc::c_uint)
                                                 as libc::c_ulong) as size_t
                as size_t
    }
    send_ascii_command(buffer.as_mut_ptr());
    usleep(250 as libc::c_int as __useconds_t);
    send_ascii_command(b"\r\n\x00" as *const u8 as *const libc::c_char);
    let mut rsp: [libc::c_char; 80] = [0; 80];
    read_ascii_response(rsp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 80]>() as
                            libc::c_ulong);
    if strncmp(rsp.as_mut_ptr(),
               b"END\x00" as *const u8 as *const libc::c_char,
               strlen(b"END\x00" as *const u8 as *const libc::c_char)) ==
           0 as libc::c_int {
    } else {
        __assert_fail(b"strncmp(rsp, \"END\", strlen(\"END\")) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      732 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    buffer[3 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    send_ascii_command(buffer.as_mut_ptr());
    usleep(250 as libc::c_int as __useconds_t);
    send_ascii_command(b"\r\n\x00" as *const u8 as *const libc::c_char);
    read_ascii_response(rsp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 80]>() as
                            libc::c_ulong);
    if strncmp(rsp.as_mut_ptr(),
               b"END\x00" as *const u8 as *const libc::c_char,
               strlen(b"END\x00" as *const u8 as *const libc::c_char)) ==
           0 as libc::c_int {
    } else {
        __assert_fail(b"strncmp(rsp, \"END\", strlen(\"END\")) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      738 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    memset(buffer.as_mut_ptr() as *mut libc::c_void, ' ' as i32,
           ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong);
    let mut len: libc::c_int =
        snprintf(buffer.as_mut_ptr().offset(101 as libc::c_int as isize),
                 (::std::mem::size_of::<[libc::c_char; 4096]>() as
                      libc::c_ulong).wrapping_sub(101 as libc::c_int as
                                                      libc::c_ulong),
                 b"gets foo\x00" as *const u8 as *const libc::c_char);
    buffer[(101 as libc::c_int + len) as usize] = ' ' as i32 as libc::c_char;
    buffer[(::std::mem::size_of::<[libc::c_char; 4096]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
               as usize] = '\u{0}' as i32 as libc::c_char;
    send_ascii_command(buffer.as_mut_ptr());
    /* verify that the server closed the connection */
    if (*con).read.expect("non-null function pointer")(con,
                                                       buffer.as_mut_ptr() as
                                                           *mut libc::c_void,
                                                       ::std::mem::size_of::<[libc::c_char; 4096]>()
                                                           as libc::c_ulong)
           == 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"con->read(con, buffer, sizeof(buffer)) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      746 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      750 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_102(void)\x00")).as_ptr());
    };
    return TEST_PASS;
}
unsafe extern "C" fn start_memcached_server() -> test_return {
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      759 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"enum test_return start_memcached_server(void)\x00")).as_ptr());
    };
    return TEST_PASS;
}
unsafe extern "C" fn stop_memcached_server() -> test_return {
    close_conn();
    if server_pid != -(1 as libc::c_int) {
        if kill(server_pid, 15 as libc::c_int) == 0 as libc::c_int {
        } else {
            __assert_fail(b"kill(server_pid, SIGTERM) == 0\x00" as *const u8
                              as *const libc::c_char,
                          b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                              *const libc::c_char,
                          766 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"enum test_return stop_memcached_server(void)\x00")).as_ptr());
        };
    }
    return TEST_PASS;
}
unsafe extern "C" fn shutdown_memcached_server() -> test_return {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    close_conn();
    if !con.is_null() {
    } else {
        __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      777 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"enum test_return shutdown_memcached_server(void)\x00")).as_ptr());
    };
    send_ascii_command(b"shutdown\r\n\x00" as *const u8 as
                           *const libc::c_char);
    /* verify that the server closed the connection */
    if (*con).read.expect("non-null function pointer")(con,
                                                       buffer.as_mut_ptr() as
                                                           *mut libc::c_void,
                                                       ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                           as libc::c_ulong)
           == 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(b"con->read(con, buffer, sizeof(buffer)) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      781 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"enum test_return shutdown_memcached_server(void)\x00")).as_ptr());
    };
    close_conn();
    /* We set server_pid to -1 so that we don't later call kill() */
    if kill(server_pid, 0 as libc::c_int) == 0 as libc::c_int {
        server_pid = -(1 as libc::c_int)
    }
    return TEST_PASS;
}
unsafe extern "C" fn ext_command(mut buf: *mut libc::c_char,
                                 mut bufsz: size_t, mut cmd: uint8_t,
                                 mut ext: *const libc::c_void,
                                 mut extlen: size_t,
                                 mut key: *const libc::c_void,
                                 mut keylen: size_t,
                                 mut dta: *const libc::c_void,
                                 mut dtalen: size_t) -> off_t {
    let mut ext_offset: off_t = 0;
    let mut key_offset: off_t =
        (ext_offset as libc::c_ulong).wrapping_add(extlen) as off_t;
    let mut dta_offset: off_t =
        (key_offset as libc::c_ulong).wrapping_add(keylen) as off_t;
    if ext != 0 as *mut libc::c_void {
        memcpy(buf.offset(ext_offset as isize) as *mut libc::c_void, ext,
               extlen);
    }
    if key != 0 as *mut libc::c_void {
        memcpy(buf.offset(key_offset as isize) as *mut libc::c_void, key,
               keylen);
    }
    if dta != 0 as *mut libc::c_void {
        memcpy(buf.offset(dta_offset as isize) as *mut libc::c_void, dta,
               dtalen);
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn raw_command(mut buf: *mut libc::c_char,
                                 mut bufsz: size_t, mut cmd: uint8_t,
                                 mut key: *const libc::c_void,
                                 mut keylen: size_t,
                                 mut dta: *const libc::c_void,
                                 mut dtalen: size_t) -> off_t {
    /* all of the storage commands use the same command layout */
    return ext_command(buf, bufsz, cmd, 0 as *const libc::c_void,
                       0 as libc::c_int as size_t, key, keylen, dta, dtalen);
}
unsafe extern "C" fn test_binary_noop() -> test_return {
    let mut buffer: C2RustUnnamed_9 = C2RustUnnamed_9{request: 0,};
    let mut len: size_t = 0;
    return TEST_PASS;
}
unsafe extern "C" fn test_binary_quit() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_quitq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_set() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_setq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_add() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_addq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_replace() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_replaceq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_delete() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_deleteq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_get() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_getk() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_gat() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_gatk() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_getq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_getkq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_gatq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_gatkq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_incr() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_incrq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_decr() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_decrq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_version() -> test_return {
    let mut buffer: C2RustUnnamed_10 = C2RustUnnamed_10{request: 0,};
    let mut len: size_t = 0;
    return TEST_PASS;
}
unsafe extern "C" fn test_binary_flush() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_flushq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_append() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_prepend() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_appendq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_prependq() -> test_return {
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn test_binary_stat() -> test_return {
    let mut buffer: C2RustUnnamed_11 = C2RustUnnamed_11{request: 0,};
    let mut len: size_t = 0;
    return TEST_PASS;
}
unsafe extern "C" fn test_binary_illegal() -> test_return {
    let mut cmd: uint8_t = 0x25 as libc::c_int as uint8_t;
    while cmd as libc::c_int != 0 as libc::c_int {
        let mut buffer: C2RustUnnamed_12 = C2RustUnnamed_12{request: 0,};
        let mut len: size_t =
            raw_command(buffer.bytes.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong, cmd, 0 as *const libc::c_void,
                        0 as libc::c_int as size_t, 0 as *const libc::c_void,
                        0 as libc::c_int as size_t) as size_t;
        cmd = cmd.wrapping_add(1)
    }
    return TEST_PASS;
}
#[no_mangle]
pub static mut hickup_thread_running: libc::c_int = 0;
unsafe extern "C" fn test_binary_pipeline_hickup_chunk(mut buffer:
                                                           *mut libc::c_void,
                                                       mut buffersize: size_t)
 -> test_return {
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let mut key: [*mut libc::c_char; 256] = [0 as *mut libc::c_char; 256];
    let mut value: uint64_t = 0xfeedfacedeadbeef as libc::c_ulong;
    /* Ignoring SASL */
    /* I don't want to pass on the quit commands ;-) */
    /* FALLTHROUGH */
    return TEST_PASS;
}
unsafe extern "C" fn test_binary_pipeline_hickup() -> test_return {
    let mut buffersize: size_t =
        (65 as libc::c_int * 1024 as libc::c_int) as size_t;
    let mut buffer: *mut libc::c_void = malloc(buffersize);
    let mut ii: libc::c_int = 0;
    let mut tid: pthread_t = 0;
    let mut ret: libc::c_int = 0;
    ret =
        pthread_create(&mut tid, 0 as *const pthread_attr_t,
                       Some(binary_hickup_recv_verification_thread as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> *mut libc::c_void),
                       0 as *mut libc::c_void);
    if ret != 0 as libc::c_int {
        fprintf(stderr,
                b"Can\'t create thread: %s\n\x00" as *const u8 as
                    *const libc::c_char, strerror(ret));
        free(buffer);
        return TEST_FAIL
    }
    /* Allow the thread to start */
    usleep(250 as libc::c_int as __useconds_t);
    srand(time(0 as *mut time_t) as libc::c_int as libc::c_uint);
    ii = 0 as libc::c_int;
    while ii < 2 as libc::c_int {
        test_binary_pipeline_hickup_chunk(buffer, buffersize);
        ii += 1
    }
    /* send quitq to shut down the read thread ;-) */
    let mut len: size_t = 0;
    pthread_join(tid, 0 as *mut *mut libc::c_void);
    free(buffer);
    return TEST_PASS;
}
unsafe extern "C" fn test_issue_101() -> test_return {
    let mut ret: test_return = TEST_PASS;
    let mut conns: [*mut conn; 2] = [0 as *mut conn; 2];
    let mut ii: libc::c_int = 0 as libc::c_int;
    let mut child: pid_t = 0 as libc::c_int;
    if !getenv(b"SKIP_TEST_101\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        return TEST_SKIP
    }
    let mut command: *const libc::c_char =
        b"stats\r\nstats\r\nstats\r\nstats\r\nstats\r\n\x00" as *const u8 as
            *const libc::c_char;
    let mut cmdlen: size_t = strlen(command);
    ii = 0 as libc::c_int;
    while ii < max as libc::c_int {
        conns[ii as usize] = 0 as *mut conn;
        if !conns[ii as usize].is_null() {
        } else {
            __assert_fail(b"conns[ii]\x00" as *const u8 as
                              *const libc::c_char,
                          b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                              *const libc::c_char,
                          2009 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
        };
        if (*conns[ii as usize]).sock > 0 as libc::c_int {
        } else {
            __assert_fail(b"conns[ii]->sock > 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                              *const libc::c_char,
                          2010 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
        };
        ii += 1
    }
    /* Send command on the connection until it blocks */
    ii = 0 as libc::c_int;
    while ii < max as libc::c_int { ii += 1 }
    child = fork();
    if child == -(1 as libc::c_int) {
        abort();
    } else {
        if child > 0 as libc::c_int {
            let mut stat: libc::c_int = 0;
            let mut c: pid_t = 0;
            loop  {
                c = waitpid(child, &mut stat, 0 as libc::c_int);
                if !(c == -(1 as libc::c_int) &&
                         *__errno_location() == 4 as libc::c_int) {
                    break ;
                }
            }
            if c == child {
            } else {
                __assert_fail(b"c == child\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                                  *const libc::c_char,
                              2041 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
            };
            if stat == 0 as libc::c_int {
            } else {
                __assert_fail(b"stat == 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                                  *const libc::c_char,
                              2042 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
            };
        } else {
            if !con.is_null() {
            } else {
                __assert_fail(b"con\x00" as *const u8 as *const libc::c_char,
                              b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                                  *const libc::c_char,
                              2045 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
            };
            ret = test_binary_noop();
            exit(0 as libc::c_int);
        }
    }
    /* close all connections */
    ii = 0 as libc::c_int;
    while ii < max as libc::c_int {
        let mut c_0: *mut conn = conns[ii as usize];
        if !c_0.is_null() {
            if (*c_0).sock > 0 as libc::c_int { close((*c_0).sock); }
            free(conns[ii as usize] as *mut libc::c_void);
            conns[ii as usize] = 0 as *mut conn
        }
        ii += 1
    }
    if kill(server_pid, 15 as libc::c_int) == 0 as libc::c_int {
    } else {
        __assert_fail(b"kill(server_pid, SIGTERM) == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"/tmp/.tmp5dgoiM/source.c\x00" as *const u8 as
                          *const libc::c_char,
                      2068 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"enum test_return test_issue_101(void)\x00")).as_ptr());
    };
    return ret;
}
#[no_mangle]
pub static mut testcases: [testcase; 53] =
    unsafe {
        [{
             let mut init =
                 testcase{description:
                              b"cache_create\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_create_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"cache_constructor\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_constructor_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"cache_constructor_fail\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_fail_constructor_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"cache_destructor\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_destructor_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"cache_reuse\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_reuse_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"cache_redzone\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(cache_redzone_test as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"issue_161\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_issue_161 as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"strtol\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_safe_strtol as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"strtoll\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_safe_strtoll as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"strtoul\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_safe_strtoul as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"strtoull\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_safe_strtoull as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"issue_44\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_issue_44 as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"vperror\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_vperror as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"issue_101\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_issue_101 as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"start_server\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(start_memcached_server as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"issue_92\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_issue_92 as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"issue_102\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_issue_102 as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_noop\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_noop as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_quit\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_quit as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_quitq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_quitq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_set\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_set as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_setq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_setq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_add\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_add as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_addq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_addq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_replace\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_replace as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_replaceq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_replaceq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_delete\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_delete as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_deleteq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_deleteq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_get\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_get as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_getq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_getq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_getk\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_getk as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_getkq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_getkq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_gat\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_gat as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_gatq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_gatq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_gatk\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_gatk as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_gatkq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_gatkq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_incr\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_incr as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_incrq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_incrq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_decr\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_decr as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_decrq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_decrq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_version\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_version as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_flush\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_flush as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_flushq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_flushq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_append\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_append as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_appendq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_appendq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_prepend\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_prepend as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_prependq\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_prependq as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_stat\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_stat as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_illegal\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_illegal as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"binary_pipeline_hickup\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(test_binary_pipeline_hickup as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"shutdown\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(shutdown_memcached_server as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description:
                              b"stop_server\x00" as *const u8 as
                                  *const libc::c_char,
                          function:
                              Some(stop_memcached_server as
                                       unsafe extern "C" fn()
                                           -> test_return),};
             init
         },
         {
             let mut init =
                 testcase{description: 0 as *const libc::c_char,
                          function: None,};
             init
         }]
    };
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut exitcode: libc::c_int = 0 as libc::c_int;
    let mut ii: libc::c_int = 0 as libc::c_int;
    let mut num_cases: libc::c_int = 0 as libc::c_int;
    num_cases = 0 as libc::c_int;
    while !testcases[num_cases as usize].description.is_null() {
        /* Just counting */
        num_cases += 1
    }
    printf(b"1..%d\n\x00" as *const u8 as *const libc::c_char, num_cases);
    ii = 0 as libc::c_int;
    while !testcases[ii as usize].description.is_null() {
        fflush(stdout);
        /* the test program shouldn't run longer than 10 minutes... */
        alarm(600 as libc::c_int as libc::c_uint);
        let mut ret: test_return =
            testcases[ii as
                          usize].function.expect("non-null function pointer")();
        if ret as libc::c_uint == TEST_SKIP as libc::c_int as libc::c_uint {
            fprintf(stdout,
                    b"ok # SKIP %d - %s\n\x00" as *const u8 as
                        *const libc::c_char, ii + 1 as libc::c_int,
                    testcases[ii as usize].description);
        } else if ret as libc::c_uint ==
                      TEST_PASS as libc::c_int as libc::c_uint {
            fprintf(stdout,
                    b"ok %d - %s\n\x00" as *const u8 as *const libc::c_char,
                    ii + 1 as libc::c_int,
                    testcases[ii as usize].description);
        } else {
            fprintf(stdout,
                    b"not ok %d - %s\n\x00" as *const u8 as
                        *const libc::c_char, ii + 1 as libc::c_int,
                    testcases[ii as usize].description);
            exitcode = 1 as libc::c_int
        }
        fflush(stdout);
        ii += 1
    }
    return exitcode;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
