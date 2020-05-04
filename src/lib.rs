#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use jni::objects::*;
use jni::sys::{jbyteArray, jint, jlong};
use jni::*;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPVOID, MAX_PATH};
use winapi::shared::ntdef::{FALSE, TRUE};
use winapi::um::processenv::GetCurrentDirectoryA;
use winapi::um::winnt::{BOOLEAN, CHAR, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_fileLength(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    unsafe {
        return PHYSFS_fileLength(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_tell(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    unsafe {
        return PHYSFS_tell(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_readBytes(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    java_buf: jbyteArray,
    offset: jint,
    len: jint,
) -> jint {
    unsafe {
        let mut vec = vec![0 as i8; len as usize];
        let c_buf = &mut vec[..];

        let res = PHYSFS_readBytes(
            fd as *mut PHYSFS_File,
            c_buf.as_mut_ptr() as *mut libc::c_void,
            len as PHYSFS_uint64,
        ) as jint;

        env.set_byte_array_region(java_buf, offset, c_buf).unwrap();

        return res;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_openRead(
    env: JNIEnv,
    obj: JObject,
    file_name: JString,
) -> jlong {
    unsafe {
        let physfs_file = PHYSFS_openRead((**env.get_string(file_name).unwrap()).as_ptr());
        return physfs_file as jlong;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_seek(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    pos: jlong,
) -> jint {
    unsafe {
        return PHYSFS_seek(fd as *mut PHYSFS_File, pos as PHYSFS_uint64);
    }
}


#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_eof(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    unsafe {
        return PHYSFS_eof(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_close(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    unsafe {
        return PHYSFS_close(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_exists(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    unsafe {
        return PHYSFS_exists((**env.get_string(file_name).unwrap()).as_ptr());
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    append: jint,
) -> jint {
    unsafe {
        return PHYSFS_mount(
            (**env.get_string(file_name).unwrap()).as_ptr(),
            std::ptr::null(),
            append,
        );
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mountAt(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    mount_point: JString,
    append: jint,
) -> jint {
    unsafe {
        return PHYSFS_mount(
            (**env.get_string(file_name).unwrap()).as_ptr(),
            (**env.get_string(mount_point).unwrap()).as_ptr(),
            append,
        );
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_unmount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    unsafe {
        return PHYSFS_unmount((**env.get_string(file_name).unwrap()).as_ptr());
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_getLastErrorCode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    unsafe {
        return PHYSFS_getLastErrorCode();
    }
}

#[allow(unused_variables)]
#[no_mangle]
extern "system" fn DllMain(dllHandle: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOLEAN {
    unsafe {
        match reason {
            DLL_PROCESS_ATTACH => {
                if PHYSFS_init(std::ptr::null()) > 0 {
                    let mut vec = vec![0 as CHAR; MAX_PATH as usize];
                    let c_str = &mut vec[..];

                    if GetCurrentDirectoryA(MAX_PATH as u32, c_str.as_mut_ptr() as *mut CHAR) > 0 {
                        if PHYSFS_setWriteDir(c_str.as_mut_ptr() as *mut CHAR) > 0 {
                            if PHYSFS_addToSearchPath(c_str.as_mut_ptr() as *mut CHAR, 0) > 0 {
                                return TRUE;
                            }
                        }
                    }
                }

                return FALSE;
            }
            DLL_PROCESS_DETACH => {
                if PHYSFS_deinit() > 0 {
                    return TRUE;
                }

                return FALSE;
            }
            _ => {
                return TRUE;
            }
        }
    }
}
