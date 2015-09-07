
        mod __gl_imports {
            extern crate gl_common;
            extern crate libc;
            pub use std::mem;
            pub use std::marker::Send;
        }
    

        pub mod types {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            #![allow(missing_copy_implementations)]
    
pub type GLenum = super::__gl_imports::libc::c_uint;
pub type GLboolean = super::__gl_imports::libc::c_uchar;
pub type GLbitfield = super::__gl_imports::libc::c_uint;
pub type GLvoid = super::__gl_imports::libc::c_void;
pub type GLbyte = super::__gl_imports::libc::c_char;
pub type GLshort = super::__gl_imports::libc::c_short;
pub type GLint = super::__gl_imports::libc::c_int;
pub type GLclampx = super::__gl_imports::libc::c_int;
pub type GLubyte = super::__gl_imports::libc::c_uchar;
pub type GLushort = super::__gl_imports::libc::c_ushort;
pub type GLuint = super::__gl_imports::libc::c_uint;
pub type GLsizei = super::__gl_imports::libc::c_int;
pub type GLfloat = super::__gl_imports::libc::c_float;
pub type GLclampf = super::__gl_imports::libc::c_float;
pub type GLdouble = super::__gl_imports::libc::c_double;
pub type GLclampd = super::__gl_imports::libc::c_double;
pub type GLeglImageOES = *const super::__gl_imports::libc::c_void;
pub type GLchar = super::__gl_imports::libc::c_char;
pub type GLcharARB = super::__gl_imports::libc::c_char;
#[cfg(target_os = "macos")] pub type GLhandleARB = *const super::__gl_imports::libc::c_void;
#[cfg(not(target_os = "macos"))] pub type GLhandleARB = super::__gl_imports::libc::c_uint;
pub type GLhalfARB = super::__gl_imports::libc::c_ushort;
pub type GLhalf = super::__gl_imports::libc::c_ushort;
pub type GLfixed = GLint;
pub type GLintptr = super::__gl_imports::libc::ptrdiff_t;
pub type GLsizeiptr = super::__gl_imports::libc::ptrdiff_t;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = super::__gl_imports::libc::ptrdiff_t;
pub type GLsizeiptrARB = super::__gl_imports::libc::ptrdiff_t;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
#[repr(C)] pub struct __GLsync;
pub type GLsync = *const __GLsync;
#[repr(C)] pub struct _cl_context;
#[repr(C)] pub struct _cl_event;
pub type GLDEBUGPROC = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLhalfNV = super::__gl_imports::libc::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
}
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_BIT: types::GLenum = 0x00000001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_BIT: types::GLenum = 0x00000002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_BIT: types::GLenum = 0x00000004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_BIT: types::GLenum = 0x00000008; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_STIPPLE_BIT: types::GLenum = 0x00000010; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MODE_BIT: types::GLenum = 0x00000020; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHTING_BIT: types::GLenum = 0x00000040; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_BIT: types::GLenum = 0x00000080; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_BUFFER_BIT: types::GLenum = 0x00000200; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VIEWPORT_BIT: types::GLenum = 0x00000800; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSFORM_BIT: types::GLenum = 0x00001000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ENABLE_BIT: types::GLenum = 0x00002000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const HINT_BIT: types::GLenum = 0x00008000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EVAL_BIT: types::GLenum = 0x00010000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIST_BIT: types::GLenum = 0x00020000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BIT: types::GLenum = 0x00040000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SCISSOR_BIT: types::GLenum = 0x00080000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIENT_PIXEL_STORE_BIT: types::GLenum = 0x00000001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIENT_VERTEX_ARRAY_BIT: types::GLenum = 0x00000002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIENT_ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FALSE: types::GLboolean = 0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NO_ERROR: types::GLenum = 0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ZERO: types::GLenum = 0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NONE: types::GLenum = 0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRUE: types::GLboolean = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE: types::GLenum = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINTS: types::GLenum = 0x0000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINES: types::GLenum = 0x0001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_LOOP: types::GLenum = 0x0002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_STRIP: types::GLenum = 0x0003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRIANGLES: types::GLenum = 0x0004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRIANGLE_STRIP: types::GLenum = 0x0005; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRIANGLE_FAN: types::GLenum = 0x0006; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const QUADS: types::GLenum = 0x0007; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const QUAD_STRIP: types::GLenum = 0x0008; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON: types::GLenum = 0x0009; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM: types::GLenum = 0x0100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LOAD: types::GLenum = 0x0101; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RETURN: types::GLenum = 0x0102; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MULT: types::GLenum = 0x0103; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ADD: types::GLenum = 0x0104; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEVER: types::GLenum = 0x0200; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LESS: types::GLenum = 0x0201; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EQUAL: types::GLenum = 0x0202; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LEQUAL: types::GLenum = 0x0203; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREATER: types::GLenum = 0x0204; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NOTEQUAL: types::GLenum = 0x0205; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GEQUAL: types::GLenum = 0x0206; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALWAYS: types::GLenum = 0x0207; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC_COLOR: types::GLenum = 0x0300; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC_ALPHA: types::GLenum = 0x0302; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DST_ALPHA: types::GLenum = 0x0304; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DST_COLOR: types::GLenum = 0x0306; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT_LEFT: types::GLenum = 0x0400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT_RIGHT: types::GLenum = 0x0401; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BACK_LEFT: types::GLenum = 0x0402; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BACK_RIGHT: types::GLenum = 0x0403; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT: types::GLenum = 0x0404; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BACK: types::GLenum = 0x0405; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LEFT: types::GLenum = 0x0406; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RIGHT: types::GLenum = 0x0407; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT_AND_BACK: types::GLenum = 0x0408; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX0: types::GLenum = 0x0409; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX1: types::GLenum = 0x040A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX2: types::GLenum = 0x040B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX3: types::GLenum = 0x040C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INVALID_ENUM: types::GLenum = 0x0500; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INVALID_VALUE: types::GLenum = 0x0501; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INVALID_OPERATION: types::GLenum = 0x0502; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STACK_OVERFLOW: types::GLenum = 0x0503; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STACK_UNDERFLOW: types::GLenum = 0x0504; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OUT_OF_MEMORY: types::GLenum = 0x0505; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _2D: types::GLenum = 0x0600; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _3D: types::GLenum = 0x0601; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _3D_COLOR: types::GLenum = 0x0602; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _3D_COLOR_TEXTURE: types::GLenum = 0x0603; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _4D_COLOR_TEXTURE: types::GLenum = 0x0604; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PASS_THROUGH_TOKEN: types::GLenum = 0x0700; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_TOKEN: types::GLenum = 0x0701; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_TOKEN: types::GLenum = 0x0702; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_TOKEN: types::GLenum = 0x0703; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BITMAP_TOKEN: types::GLenum = 0x0704; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DRAW_PIXEL_TOKEN: types::GLenum = 0x0705; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COPY_PIXEL_TOKEN: types::GLenum = 0x0706; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_RESET_TOKEN: types::GLenum = 0x0707; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EXP: types::GLenum = 0x0800; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EXP2: types::GLenum = 0x0801; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CW: types::GLenum = 0x0900; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CCW: types::GLenum = 0x0901; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COEFF: types::GLenum = 0x0A00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ORDER: types::GLenum = 0x0A01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOMAIN: types::GLenum = 0x0A02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_COLOR: types::GLenum = 0x0B00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_INDEX: types::GLenum = 0x0B01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_NORMAL: types::GLenum = 0x0B02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_COLOR: types::GLenum = 0x0B04; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_INDEX: types::GLenum = 0x0B05; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_TEXTURE_COORDS: types::GLenum = 0x0B06; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_POSITION: types::GLenum = 0x0B07; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_POSITION_VALID: types::GLenum = 0x0B08; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_RASTER_DISTANCE: types::GLenum = 0x0B09; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SMOOTH: types::GLenum = 0x0B10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE: types::GLenum = 0x0B11; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_SMOOTH: types::GLenum = 0x0B20; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_WIDTH: types::GLenum = 0x0B21; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_STIPPLE: types::GLenum = 0x0B24; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_STIPPLE_PATTERN: types::GLenum = 0x0B25; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_STIPPLE_REPEAT: types::GLenum = 0x0B26; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIST_MODE: types::GLenum = 0x0B30; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_LIST_NESTING: types::GLenum = 0x0B31; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIST_BASE: types::GLenum = 0x0B32; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIST_INDEX: types::GLenum = 0x0B33; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_MODE: types::GLenum = 0x0B40; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_SMOOTH: types::GLenum = 0x0B41; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_STIPPLE: types::GLenum = 0x0B42; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EDGE_FLAG: types::GLenum = 0x0B43; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CULL_FACE: types::GLenum = 0x0B44; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CULL_FACE_MODE: types::GLenum = 0x0B45; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT_FACE: types::GLenum = 0x0B46; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHTING: types::GLenum = 0x0B50; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT_MODEL_LOCAL_VIEWER: types::GLenum = 0x0B51; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHADE_MODEL: types::GLenum = 0x0B54; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_MATERIAL_FACE: types::GLenum = 0x0B55; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_MATERIAL_PARAMETER: types::GLenum = 0x0B56; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_MATERIAL: types::GLenum = 0x0B57; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG: types::GLenum = 0x0B60; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_INDEX: types::GLenum = 0x0B61; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_DENSITY: types::GLenum = 0x0B62; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_START: types::GLenum = 0x0B63; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_END: types::GLenum = 0x0B64; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_MODE: types::GLenum = 0x0B65; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_COLOR: types::GLenum = 0x0B66; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_RANGE: types::GLenum = 0x0B70; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_TEST: types::GLenum = 0x0B71; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_FUNC: types::GLenum = 0x0B74; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_CLEAR_VALUE: types::GLenum = 0x0B80; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_TEST: types::GLenum = 0x0B90; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_FUNC: types::GLenum = 0x0B92; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_FAIL: types::GLenum = 0x0B94; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_REF: types::GLenum = 0x0B97; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MATRIX_MODE: types::GLenum = 0x0BA0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMALIZE: types::GLenum = 0x0BA1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VIEWPORT: types::GLenum = 0x0BA2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MODELVIEW_STACK_DEPTH: types::GLenum = 0x0BA3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROJECTION_STACK_DEPTH: types::GLenum = 0x0BA4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_STACK_DEPTH: types::GLenum = 0x0BA5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MODELVIEW_MATRIX: types::GLenum = 0x0BA6; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROJECTION_MATRIX: types::GLenum = 0x0BA7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_MATRIX: types::GLenum = 0x0BA8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_TEST: types::GLenum = 0x0BC0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_TEST_FUNC: types::GLenum = 0x0BC1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_TEST_REF: types::GLenum = 0x0BC2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DITHER: types::GLenum = 0x0BD0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLEND_DST: types::GLenum = 0x0BE0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLEND_SRC: types::GLenum = 0x0BE1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLEND: types::GLenum = 0x0BE2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_LOGIC_OP: types::GLenum = 0x0BF1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LOGIC_OP: types::GLenum = 0x0BF1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX_BUFFERS: types::GLenum = 0x0C00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DRAW_BUFFER: types::GLenum = 0x0C01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const READ_BUFFER: types::GLenum = 0x0C02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SCISSOR_BOX: types::GLenum = 0x0C10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SCISSOR_TEST: types::GLenum = 0x0C11; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_CLEAR_VALUE: types::GLenum = 0x0C20; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_WRITEMASK: types::GLenum = 0x0C21; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_WRITEMASK: types::GLenum = 0x0C23; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_MODE: types::GLenum = 0x0C30; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA_MODE: types::GLenum = 0x0C31; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOUBLEBUFFER: types::GLenum = 0x0C32; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STEREO: types::GLenum = 0x0C33; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RENDER_MODE: types::GLenum = 0x0C40; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PERSPECTIVE_CORRECTION_HINT: types::GLenum = 0x0C50; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SMOOTH_HINT: types::GLenum = 0x0C51; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG_HINT: types::GLenum = 0x0C54; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GEN_S: types::GLenum = 0x0C60; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GEN_T: types::GLenum = 0x0C61; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GEN_R: types::GLenum = 0x0C62; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GEN_Q: types::GLenum = 0x0C63; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_I: types::GLenum = 0x0C70; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_S_TO_S: types::GLenum = 0x0C71; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_R: types::GLenum = 0x0C72; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_G: types::GLenum = 0x0C73; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_B: types::GLenum = 0x0C74; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_A: types::GLenum = 0x0C75; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_R_TO_R: types::GLenum = 0x0C76; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_G_TO_G: types::GLenum = 0x0C77; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_B_TO_B: types::GLenum = 0x0C78; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_A_TO_A: types::GLenum = 0x0C79; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_I_SIZE: types::GLenum = 0x0CB0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_S_TO_S_SIZE: types::GLenum = 0x0CB1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_R_SIZE: types::GLenum = 0x0CB2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_G_SIZE: types::GLenum = 0x0CB3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_B_SIZE: types::GLenum = 0x0CB4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_I_TO_A_SIZE: types::GLenum = 0x0CB5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_R_TO_R_SIZE: types::GLenum = 0x0CB6; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_G_TO_G_SIZE: types::GLenum = 0x0CB7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_B_TO_B_SIZE: types::GLenum = 0x0CB8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_MAP_A_TO_A_SIZE: types::GLenum = 0x0CB9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_LSB_FIRST: types::GLenum = 0x0D01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_ALIGNMENT: types::GLenum = 0x0D05; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP_COLOR: types::GLenum = 0x0D10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP_STENCIL: types::GLenum = 0x0D11; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_SHIFT: types::GLenum = 0x0D12; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_OFFSET: types::GLenum = 0x0D13; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED_SCALE: types::GLenum = 0x0D14; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED_BIAS: types::GLenum = 0x0D15; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ZOOM_X: types::GLenum = 0x0D16; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ZOOM_Y: types::GLenum = 0x0D17; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN_SCALE: types::GLenum = 0x0D18; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN_BIAS: types::GLenum = 0x0D19; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE_SCALE: types::GLenum = 0x0D1A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE_BIAS: types::GLenum = 0x0D1B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_SCALE: types::GLenum = 0x0D1C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_BIAS: types::GLenum = 0x0D1D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_SCALE: types::GLenum = 0x0D1E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_BIAS: types::GLenum = 0x0D1F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_EVAL_ORDER: types::GLenum = 0x0D30; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_LIGHTS: types::GLenum = 0x0D31; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_CLIP_PLANES: types::GLenum = 0x0D32; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_PIXEL_MAP_TABLE: types::GLenum = 0x0D34; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D35; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_NAME_STACK_DEPTH: types::GLenum = 0x0D37; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_PROJECTION_STACK_DEPTH: types::GLenum = 0x0D38; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_TEXTURE_STACK_DEPTH: types::GLenum = 0x0D39; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D3B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SUBPIXEL_BITS: types::GLenum = 0x0D50; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_BITS: types::GLenum = 0x0D51; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED_BITS: types::GLenum = 0x0D52; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN_BITS: types::GLenum = 0x0D53; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE_BITS: types::GLenum = 0x0D54; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_BITS: types::GLenum = 0x0D55; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_BITS: types::GLenum = 0x0D56; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_BITS: types::GLenum = 0x0D57; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_RED_BITS: types::GLenum = 0x0D58; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_GREEN_BITS: types::GLenum = 0x0D59; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_BLUE_BITS: types::GLenum = 0x0D5A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_ALPHA_BITS: types::GLenum = 0x0D5B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NAME_STACK_DEPTH: types::GLenum = 0x0D70; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUTO_NORMAL: types::GLenum = 0x0D80; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_COLOR_4: types::GLenum = 0x0D90; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_INDEX: types::GLenum = 0x0D91; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_NORMAL: types::GLenum = 0x0D92; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_TEXTURE_COORD_1: types::GLenum = 0x0D93; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_TEXTURE_COORD_2: types::GLenum = 0x0D94; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_TEXTURE_COORD_3: types::GLenum = 0x0D95; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_TEXTURE_COORD_4: types::GLenum = 0x0D96; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_VERTEX_3: types::GLenum = 0x0D97; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_VERTEX_4: types::GLenum = 0x0D98; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_COLOR_4: types::GLenum = 0x0DB0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_INDEX: types::GLenum = 0x0DB1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_NORMAL: types::GLenum = 0x0DB2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_TEXTURE_COORD_1: types::GLenum = 0x0DB3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_TEXTURE_COORD_2: types::GLenum = 0x0DB4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_TEXTURE_COORD_3: types::GLenum = 0x0DB5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_TEXTURE_COORD_4: types::GLenum = 0x0DB6; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_VERTEX_3: types::GLenum = 0x0DB7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_VERTEX_4: types::GLenum = 0x0DB8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_GRID_DOMAIN: types::GLenum = 0x0DD0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP1_GRID_SEGMENTS: types::GLenum = 0x0DD1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_GRID_DOMAIN: types::GLenum = 0x0DD2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAP2_GRID_SEGMENTS: types::GLenum = 0x0DD3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_1D: types::GLenum = 0x0DE0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_2D: types::GLenum = 0x0DE1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FEEDBACK_BUFFER_POINTER: types::GLenum = 0x0DF0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FEEDBACK_BUFFER_SIZE: types::GLenum = 0x0DF1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FEEDBACK_BUFFER_TYPE: types::GLenum = 0x0DF2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SELECTION_BUFFER_POINTER: types::GLenum = 0x0DF3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SELECTION_BUFFER_SIZE: types::GLenum = 0x0DF4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_WIDTH: types::GLenum = 0x1000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_HEIGHT: types::GLenum = 0x1001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COMPONENTS: types::GLenum = 0x1003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BORDER: types::GLenum = 0x1005; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DONT_CARE: types::GLenum = 0x1100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FASTEST: types::GLenum = 0x1101; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NICEST: types::GLenum = 0x1102; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AMBIENT: types::GLenum = 0x1200; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DIFFUSE: types::GLenum = 0x1201; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SPECULAR: types::GLenum = 0x1202; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POSITION: types::GLenum = 0x1203; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SPOT_DIRECTION: types::GLenum = 0x1204; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SPOT_EXPONENT: types::GLenum = 0x1205; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SPOT_CUTOFF: types::GLenum = 0x1206; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONSTANT_ATTENUATION: types::GLenum = 0x1207; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINEAR_ATTENUATION: types::GLenum = 0x1208; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const QUADRATIC_ATTENUATION: types::GLenum = 0x1209; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMPILE: types::GLenum = 0x1300; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMPILE_AND_EXECUTE: types::GLenum = 0x1301; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BYTE: types::GLenum = 0x1400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNSIGNED_BYTE: types::GLenum = 0x1401; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHORT: types::GLenum = 0x1402; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNSIGNED_SHORT: types::GLenum = 0x1403; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INT: types::GLenum = 0x1404; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNSIGNED_INT: types::GLenum = 0x1405; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FLOAT: types::GLenum = 0x1406; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _2_BYTES: types::GLenum = 0x1407; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _3_BYTES: types::GLenum = 0x1408; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const _4_BYTES: types::GLenum = 0x1409; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOUBLE: types::GLenum = 0x140A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLEAR: types::GLenum = 0x1500; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AND: types::GLenum = 0x1501; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AND_REVERSE: types::GLenum = 0x1502; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COPY: types::GLenum = 0x1503; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AND_INVERTED: types::GLenum = 0x1504; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NOOP: types::GLenum = 0x1505; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const XOR: types::GLenum = 0x1506; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OR: types::GLenum = 0x1507; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NOR: types::GLenum = 0x1508; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EQUIV: types::GLenum = 0x1509; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INVERT: types::GLenum = 0x150A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OR_REVERSE: types::GLenum = 0x150B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COPY_INVERTED: types::GLenum = 0x150C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OR_INVERTED: types::GLenum = 0x150D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NAND: types::GLenum = 0x150E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SET: types::GLenum = 0x150F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EMISSION: types::GLenum = 0x1600; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHININESS: types::GLenum = 0x1601; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AMBIENT_AND_DIFFUSE: types::GLenum = 0x1602; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_INDEXES: types::GLenum = 0x1603; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MODELVIEW: types::GLenum = 0x1700; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROJECTION: types::GLenum = 0x1701; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE: types::GLenum = 0x1702; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR: types::GLenum = 0x1800; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH: types::GLenum = 0x1801; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL: types::GLenum = 0x1802; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_INDEX: types::GLenum = 0x1900; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_INDEX: types::GLenum = 0x1901; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_COMPONENT: types::GLenum = 0x1902; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED: types::GLenum = 0x1903; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN: types::GLenum = 0x1904; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE: types::GLenum = 0x1905; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA: types::GLenum = 0x1906; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB: types::GLenum = 0x1907; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA: types::GLenum = 0x1908; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE: types::GLenum = 0x1909; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE_ALPHA: types::GLenum = 0x190A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BITMAP: types::GLenum = 0x1A00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT: types::GLenum = 0x1B00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE: types::GLenum = 0x1B01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FILL: types::GLenum = 0x1B02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RENDER: types::GLenum = 0x1C00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FEEDBACK: types::GLenum = 0x1C01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SELECT: types::GLenum = 0x1C02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FLAT: types::GLenum = 0x1D00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SMOOTH: types::GLenum = 0x1D01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const KEEP: types::GLenum = 0x1E00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const REPLACE: types::GLenum = 0x1E01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INCR: types::GLenum = 0x1E02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DECR: types::GLenum = 0x1E03; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VENDOR: types::GLenum = 0x1F00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RENDERER: types::GLenum = 0x1F01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERSION: types::GLenum = 0x1F02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EXTENSIONS: types::GLenum = 0x1F03; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const S: types::GLenum = 0x2000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T: types::GLenum = 0x2001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const R: types::GLenum = 0x2002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const Q: types::GLenum = 0x2003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MODULATE: types::GLenum = 0x2100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DECAL: types::GLenum = 0x2101; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_ENV_MODE: types::GLenum = 0x2200; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_ENV_COLOR: types::GLenum = 0x2201; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_ENV: types::GLenum = 0x2300; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EYE_LINEAR: types::GLenum = 0x2400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OBJECT_LINEAR: types::GLenum = 0x2401; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SPHERE_MAP: types::GLenum = 0x2402; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GEN_MODE: types::GLenum = 0x2500; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OBJECT_PLANE: types::GLenum = 0x2501; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EYE_PLANE: types::GLenum = 0x2502; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEAREST: types::GLenum = 0x2600; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINEAR: types::GLenum = 0x2601; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_WRAP_S: types::GLenum = 0x2802; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_WRAP_T: types::GLenum = 0x2803; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLAMP: types::GLenum = 0x2900; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const REPEAT: types::GLenum = 0x2901; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const R3_G3_B2: types::GLenum = 0x2A10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const V2F: types::GLenum = 0x2A20; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const V3F: types::GLenum = 0x2A21; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const C4UB_V2F: types::GLenum = 0x2A22; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const C4UB_V3F: types::GLenum = 0x2A23; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const C3F_V3F: types::GLenum = 0x2A24; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const N3F_V3F: types::GLenum = 0x2A25; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const C4F_N3F_V3F: types::GLenum = 0x2A26; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T2F_V3F: types::GLenum = 0x2A27; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T4F_V4F: types::GLenum = 0x2A28; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T2F_C4UB_V3F: types::GLenum = 0x2A29; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T2F_C3F_V3F: types::GLenum = 0x2A2A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T2F_N3F_V3F: types::GLenum = 0x2A2B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T2F_C4F_N3F_V3F: types::GLenum = 0x2A2C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const T4F_C4F_N3F_V4F: types::GLenum = 0x2A2D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE0: types::GLenum = 0x3000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE1: types::GLenum = 0x3001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE2: types::GLenum = 0x3002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE3: types::GLenum = 0x3003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE4: types::GLenum = 0x3004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIP_PLANE5: types::GLenum = 0x3005; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT0: types::GLenum = 0x4000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT1: types::GLenum = 0x4001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT2: types::GLenum = 0x4002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT3: types::GLenum = 0x4003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT4: types::GLenum = 0x4004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT5: types::GLenum = 0x4005; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT6: types::GLenum = 0x4006; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT7: types::GLenum = 0x4007; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA4: types::GLenum = 0x803B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA8: types::GLenum = 0x803C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA12: types::GLenum = 0x803D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA16: types::GLenum = 0x803E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE4: types::GLenum = 0x803F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE8: types::GLenum = 0x8040; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE12: types::GLenum = 0x8041; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE16: types::GLenum = 0x8042; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE4_ALPHA4: types::GLenum = 0x8043; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE6_ALPHA2: types::GLenum = 0x8044; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE8_ALPHA8: types::GLenum = 0x8045; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE12_ALPHA4: types::GLenum = 0x8046; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE12_ALPHA12: types::GLenum = 0x8047; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LUMINANCE16_ALPHA16: types::GLenum = 0x8048; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTENSITY: types::GLenum = 0x8049; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTENSITY4: types::GLenum = 0x804A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTENSITY8: types::GLenum = 0x804B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTENSITY12: types::GLenum = 0x804C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTENSITY16: types::GLenum = 0x804D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB4: types::GLenum = 0x804F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB5: types::GLenum = 0x8050; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB8: types::GLenum = 0x8051; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB10: types::GLenum = 0x8052; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB12: types::GLenum = 0x8053; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB16: types::GLenum = 0x8054; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA2: types::GLenum = 0x8055; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA4: types::GLenum = 0x8056; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB5_A1: types::GLenum = 0x8057; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA8: types::GLenum = 0x8058; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB10_A2: types::GLenum = 0x8059; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA12: types::GLenum = 0x805A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGBA16: types::GLenum = 0x805B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_LUMINANCE_SIZE: types::GLenum = 0x8060; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_INTENSITY_SIZE: types::GLenum = 0x8061; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_PRIORITY: types::GLenum = 0x8066; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_RESIDENT: types::GLenum = 0x8067; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY: types::GLenum = 0x8074; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMAL_ARRAY: types::GLenum = 0x8075; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY: types::GLenum = 0x8076; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_ARRAY: types::GLenum = 0x8077; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY: types::GLenum = 0x8078; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EDGE_FLAG_ARRAY: types::GLenum = 0x8079; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY_SIZE: types::GLenum = 0x807A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY_TYPE: types::GLenum = 0x807B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY_STRIDE: types::GLenum = 0x807C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMAL_ARRAY_TYPE: types::GLenum = 0x807E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMAL_ARRAY_STRIDE: types::GLenum = 0x807F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY_SIZE: types::GLenum = 0x8081; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY_TYPE: types::GLenum = 0x8082; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY_STRIDE: types::GLenum = 0x8083; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_ARRAY_TYPE: types::GLenum = 0x8085; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_ARRAY_STRIDE: types::GLenum = 0x8086; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EDGE_FLAG_ARRAY_STRIDE: types::GLenum = 0x808C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY_POINTER: types::GLenum = 0x808E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMAL_ARRAY_POINTER: types::GLenum = 0x808F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY_POINTER: types::GLenum = 0x8090; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INDEX_ARRAY_POINTER: types::GLenum = 0x8091; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const EDGE_FLAG_ARRAY_POINTER: types::GLenum = 0x8093; 

        #[allow(dead_code)]
        #[allow(missing_copy_implementations)]
        #[allow(raw_pointer_derive)]
        #[derive(Clone)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::libc::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::libc::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr {
                        f: missing_fn_panic as *const __gl_imports::libc::c_void,
                        is_loaded: false
                    }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {
                self.is_loaded
            }
        }
    
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gl function was not loaded")
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[derive(Clone)]
        pub struct Gl {
pub Accum: FnPtr,
pub AlphaFunc: FnPtr,
pub AreTexturesResident: FnPtr,
/// Fallbacks: ArrayElementEXT
pub ArrayElement: FnPtr,
pub Begin: FnPtr,
/// Fallbacks: BindTextureEXT
pub BindTexture: FnPtr,
pub Bitmap: FnPtr,
pub BlendFunc: FnPtr,
pub CallList: FnPtr,
pub CallLists: FnPtr,
pub Clear: FnPtr,
pub ClearAccum: FnPtr,
pub ClearColor: FnPtr,
pub ClearDepth: FnPtr,
pub ClearIndex: FnPtr,
pub ClearStencil: FnPtr,
pub ClipPlane: FnPtr,
pub Color3b: FnPtr,
pub Color3bv: FnPtr,
pub Color3d: FnPtr,
pub Color3dv: FnPtr,
pub Color3f: FnPtr,
pub Color3fv: FnPtr,
pub Color3i: FnPtr,
pub Color3iv: FnPtr,
pub Color3s: FnPtr,
pub Color3sv: FnPtr,
pub Color3ub: FnPtr,
pub Color3ubv: FnPtr,
pub Color3ui: FnPtr,
pub Color3uiv: FnPtr,
pub Color3us: FnPtr,
pub Color3usv: FnPtr,
pub Color4b: FnPtr,
pub Color4bv: FnPtr,
pub Color4d: FnPtr,
pub Color4dv: FnPtr,
pub Color4f: FnPtr,
pub Color4fv: FnPtr,
pub Color4i: FnPtr,
pub Color4iv: FnPtr,
pub Color4s: FnPtr,
pub Color4sv: FnPtr,
pub Color4ub: FnPtr,
pub Color4ubv: FnPtr,
pub Color4ui: FnPtr,
pub Color4uiv: FnPtr,
pub Color4us: FnPtr,
pub Color4usv: FnPtr,
pub ColorMask: FnPtr,
pub ColorMaterial: FnPtr,
pub ColorPointer: FnPtr,
pub CopyPixels: FnPtr,
/// Fallbacks: CopyTexImage1DEXT
pub CopyTexImage1D: FnPtr,
/// Fallbacks: CopyTexImage2DEXT
pub CopyTexImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage1DEXT
pub CopyTexSubImage1D: FnPtr,
/// Fallbacks: CopyTexSubImage2DEXT
pub CopyTexSubImage2D: FnPtr,
pub CullFace: FnPtr,
pub DeleteLists: FnPtr,
pub DeleteTextures: FnPtr,
pub DepthFunc: FnPtr,
pub DepthMask: FnPtr,
pub DepthRange: FnPtr,
pub Disable: FnPtr,
pub DisableClientState: FnPtr,
/// Fallbacks: DrawArraysEXT
pub DrawArrays: FnPtr,
pub DrawBuffer: FnPtr,
pub DrawElements: FnPtr,
pub DrawPixels: FnPtr,
pub EdgeFlag: FnPtr,
pub EdgeFlagPointer: FnPtr,
pub EdgeFlagv: FnPtr,
pub Enable: FnPtr,
pub EnableClientState: FnPtr,
pub End: FnPtr,
pub EndList: FnPtr,
pub EvalCoord1d: FnPtr,
pub EvalCoord1dv: FnPtr,
pub EvalCoord1f: FnPtr,
pub EvalCoord1fv: FnPtr,
pub EvalCoord2d: FnPtr,
pub EvalCoord2dv: FnPtr,
pub EvalCoord2f: FnPtr,
pub EvalCoord2fv: FnPtr,
pub EvalMesh1: FnPtr,
pub EvalMesh2: FnPtr,
pub EvalPoint1: FnPtr,
pub EvalPoint2: FnPtr,
pub FeedbackBuffer: FnPtr,
pub Finish: FnPtr,
pub Flush: FnPtr,
pub Fogf: FnPtr,
pub Fogfv: FnPtr,
pub Fogi: FnPtr,
pub Fogiv: FnPtr,
pub FrontFace: FnPtr,
pub Frustum: FnPtr,
pub GenLists: FnPtr,
pub GenTextures: FnPtr,
pub GetBooleanv: FnPtr,
pub GetClipPlane: FnPtr,
pub GetDoublev: FnPtr,
pub GetError: FnPtr,
pub GetFloatv: FnPtr,
pub GetIntegerv: FnPtr,
pub GetLightfv: FnPtr,
pub GetLightiv: FnPtr,
pub GetMapdv: FnPtr,
pub GetMapfv: FnPtr,
pub GetMapiv: FnPtr,
pub GetMaterialfv: FnPtr,
pub GetMaterialiv: FnPtr,
pub GetPixelMapfv: FnPtr,
pub GetPixelMapuiv: FnPtr,
pub GetPixelMapusv: FnPtr,
/// Fallbacks: GetPointervEXT, GetPointervKHR
pub GetPointerv: FnPtr,
pub GetPolygonStipple: FnPtr,
pub GetString: FnPtr,
pub GetTexEnvfv: FnPtr,
pub GetTexEnviv: FnPtr,
pub GetTexGendv: FnPtr,
pub GetTexGenfv: FnPtr,
pub GetTexGeniv: FnPtr,
pub GetTexImage: FnPtr,
pub GetTexLevelParameterfv: FnPtr,
pub GetTexLevelParameteriv: FnPtr,
pub GetTexParameterfv: FnPtr,
pub GetTexParameteriv: FnPtr,
pub Hint: FnPtr,
pub IndexMask: FnPtr,
pub IndexPointer: FnPtr,
pub Indexd: FnPtr,
pub Indexdv: FnPtr,
pub Indexf: FnPtr,
pub Indexfv: FnPtr,
pub Indexi: FnPtr,
pub Indexiv: FnPtr,
pub Indexs: FnPtr,
pub Indexsv: FnPtr,
pub Indexub: FnPtr,
pub Indexubv: FnPtr,
pub InitNames: FnPtr,
pub InterleavedArrays: FnPtr,
pub IsEnabled: FnPtr,
pub IsList: FnPtr,
pub IsTexture: FnPtr,
pub LightModelf: FnPtr,
pub LightModelfv: FnPtr,
pub LightModeli: FnPtr,
pub LightModeliv: FnPtr,
pub Lightf: FnPtr,
pub Lightfv: FnPtr,
pub Lighti: FnPtr,
pub Lightiv: FnPtr,
pub LineStipple: FnPtr,
pub LineWidth: FnPtr,
pub ListBase: FnPtr,
pub LoadIdentity: FnPtr,
pub LoadMatrixd: FnPtr,
pub LoadMatrixf: FnPtr,
pub LoadName: FnPtr,
pub LogicOp: FnPtr,
pub Map1d: FnPtr,
pub Map1f: FnPtr,
pub Map2d: FnPtr,
pub Map2f: FnPtr,
pub MapGrid1d: FnPtr,
pub MapGrid1f: FnPtr,
pub MapGrid2d: FnPtr,
pub MapGrid2f: FnPtr,
pub Materialf: FnPtr,
pub Materialfv: FnPtr,
pub Materiali: FnPtr,
pub Materialiv: FnPtr,
pub MatrixMode: FnPtr,
pub MultMatrixd: FnPtr,
pub MultMatrixf: FnPtr,
pub NewList: FnPtr,
pub Normal3b: FnPtr,
pub Normal3bv: FnPtr,
pub Normal3d: FnPtr,
pub Normal3dv: FnPtr,
pub Normal3f: FnPtr,
pub Normal3fv: FnPtr,
pub Normal3i: FnPtr,
pub Normal3iv: FnPtr,
pub Normal3s: FnPtr,
pub Normal3sv: FnPtr,
pub NormalPointer: FnPtr,
pub Ortho: FnPtr,
pub PassThrough: FnPtr,
pub PixelMapfv: FnPtr,
pub PixelMapuiv: FnPtr,
pub PixelMapusv: FnPtr,
pub PixelStoref: FnPtr,
pub PixelStorei: FnPtr,
pub PixelTransferf: FnPtr,
pub PixelTransferi: FnPtr,
pub PixelZoom: FnPtr,
pub PointSize: FnPtr,
pub PolygonMode: FnPtr,
pub PolygonOffset: FnPtr,
pub PolygonStipple: FnPtr,
pub PopAttrib: FnPtr,
pub PopClientAttrib: FnPtr,
pub PopMatrix: FnPtr,
pub PopName: FnPtr,
/// Fallbacks: PrioritizeTexturesEXT
pub PrioritizeTextures: FnPtr,
pub PushAttrib: FnPtr,
pub PushClientAttrib: FnPtr,
pub PushMatrix: FnPtr,
pub PushName: FnPtr,
pub RasterPos2d: FnPtr,
pub RasterPos2dv: FnPtr,
pub RasterPos2f: FnPtr,
pub RasterPos2fv: FnPtr,
pub RasterPos2i: FnPtr,
pub RasterPos2iv: FnPtr,
pub RasterPos2s: FnPtr,
pub RasterPos2sv: FnPtr,
pub RasterPos3d: FnPtr,
pub RasterPos3dv: FnPtr,
pub RasterPos3f: FnPtr,
pub RasterPos3fv: FnPtr,
pub RasterPos3i: FnPtr,
pub RasterPos3iv: FnPtr,
pub RasterPos3s: FnPtr,
pub RasterPos3sv: FnPtr,
pub RasterPos4d: FnPtr,
pub RasterPos4dv: FnPtr,
pub RasterPos4f: FnPtr,
pub RasterPos4fv: FnPtr,
pub RasterPos4i: FnPtr,
pub RasterPos4iv: FnPtr,
pub RasterPos4s: FnPtr,
pub RasterPos4sv: FnPtr,
pub ReadBuffer: FnPtr,
pub ReadPixels: FnPtr,
pub Rectd: FnPtr,
pub Rectdv: FnPtr,
pub Rectf: FnPtr,
pub Rectfv: FnPtr,
pub Recti: FnPtr,
pub Rectiv: FnPtr,
pub Rects: FnPtr,
pub Rectsv: FnPtr,
pub RenderMode: FnPtr,
pub Rotated: FnPtr,
pub Rotatef: FnPtr,
pub Scaled: FnPtr,
pub Scalef: FnPtr,
pub Scissor: FnPtr,
pub SelectBuffer: FnPtr,
pub ShadeModel: FnPtr,
pub StencilFunc: FnPtr,
pub StencilMask: FnPtr,
pub StencilOp: FnPtr,
pub TexCoord1d: FnPtr,
pub TexCoord1dv: FnPtr,
pub TexCoord1f: FnPtr,
pub TexCoord1fv: FnPtr,
pub TexCoord1i: FnPtr,
pub TexCoord1iv: FnPtr,
pub TexCoord1s: FnPtr,
pub TexCoord1sv: FnPtr,
pub TexCoord2d: FnPtr,
pub TexCoord2dv: FnPtr,
pub TexCoord2f: FnPtr,
pub TexCoord2fv: FnPtr,
pub TexCoord2i: FnPtr,
pub TexCoord2iv: FnPtr,
pub TexCoord2s: FnPtr,
pub TexCoord2sv: FnPtr,
pub TexCoord3d: FnPtr,
pub TexCoord3dv: FnPtr,
pub TexCoord3f: FnPtr,
pub TexCoord3fv: FnPtr,
pub TexCoord3i: FnPtr,
pub TexCoord3iv: FnPtr,
pub TexCoord3s: FnPtr,
pub TexCoord3sv: FnPtr,
pub TexCoord4d: FnPtr,
pub TexCoord4dv: FnPtr,
pub TexCoord4f: FnPtr,
pub TexCoord4fv: FnPtr,
pub TexCoord4i: FnPtr,
pub TexCoord4iv: FnPtr,
pub TexCoord4s: FnPtr,
pub TexCoord4sv: FnPtr,
pub TexCoordPointer: FnPtr,
pub TexEnvf: FnPtr,
pub TexEnvfv: FnPtr,
pub TexEnvi: FnPtr,
pub TexEnviv: FnPtr,
pub TexGend: FnPtr,
pub TexGendv: FnPtr,
pub TexGenf: FnPtr,
pub TexGenfv: FnPtr,
pub TexGeni: FnPtr,
pub TexGeniv: FnPtr,
pub TexImage1D: FnPtr,
pub TexImage2D: FnPtr,
pub TexParameterf: FnPtr,
pub TexParameterfv: FnPtr,
pub TexParameteri: FnPtr,
pub TexParameteriv: FnPtr,
/// Fallbacks: TexSubImage1DEXT
pub TexSubImage1D: FnPtr,
/// Fallbacks: TexSubImage2DEXT
pub TexSubImage2D: FnPtr,
pub Translated: FnPtr,
pub Translatef: FnPtr,
pub Vertex2d: FnPtr,
pub Vertex2dv: FnPtr,
pub Vertex2f: FnPtr,
pub Vertex2fv: FnPtr,
pub Vertex2i: FnPtr,
pub Vertex2iv: FnPtr,
pub Vertex2s: FnPtr,
pub Vertex2sv: FnPtr,
pub Vertex3d: FnPtr,
pub Vertex3dv: FnPtr,
pub Vertex3f: FnPtr,
pub Vertex3fv: FnPtr,
pub Vertex3i: FnPtr,
pub Vertex3iv: FnPtr,
pub Vertex3s: FnPtr,
pub Vertex3sv: FnPtr,
pub Vertex4d: FnPtr,
pub Vertex4dv: FnPtr,
pub Vertex4f: FnPtr,
pub Vertex4fv: FnPtr,
pub Vertex4i: FnPtr,
pub Vertex4iv: FnPtr,
pub Vertex4s: FnPtr,
pub Vertex4sv: FnPtr,
pub VertexPointer: FnPtr,
pub Viewport: FnPtr,
}
impl Gl {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Gl where F: FnMut(&str) -> *const __gl_imports::libc::c_void {
                let mut metaloadfn = |symbol: &str, symbols: &[&str]| {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols.iter() {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                };
                Gl {
Accum: FnPtr::new(metaloadfn("glAccum", &[])),
AlphaFunc: FnPtr::new(metaloadfn("glAlphaFunc", &[])),
AreTexturesResident: FnPtr::new(metaloadfn("glAreTexturesResident", &[])),
ArrayElement: FnPtr::new(metaloadfn("glArrayElement", &["glArrayElementEXT"])),
Begin: FnPtr::new(metaloadfn("glBegin", &[])),
BindTexture: FnPtr::new(metaloadfn("glBindTexture", &["glBindTextureEXT"])),
Bitmap: FnPtr::new(metaloadfn("glBitmap", &[])),
BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
CallList: FnPtr::new(metaloadfn("glCallList", &[])),
CallLists: FnPtr::new(metaloadfn("glCallLists", &[])),
Clear: FnPtr::new(metaloadfn("glClear", &[])),
ClearAccum: FnPtr::new(metaloadfn("glClearAccum", &[])),
ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
ClearDepth: FnPtr::new(metaloadfn("glClearDepth", &[])),
ClearIndex: FnPtr::new(metaloadfn("glClearIndex", &[])),
ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
ClipPlane: FnPtr::new(metaloadfn("glClipPlane", &[])),
Color3b: FnPtr::new(metaloadfn("glColor3b", &[])),
Color3bv: FnPtr::new(metaloadfn("glColor3bv", &[])),
Color3d: FnPtr::new(metaloadfn("glColor3d", &[])),
Color3dv: FnPtr::new(metaloadfn("glColor3dv", &[])),
Color3f: FnPtr::new(metaloadfn("glColor3f", &[])),
Color3fv: FnPtr::new(metaloadfn("glColor3fv", &[])),
Color3i: FnPtr::new(metaloadfn("glColor3i", &[])),
Color3iv: FnPtr::new(metaloadfn("glColor3iv", &[])),
Color3s: FnPtr::new(metaloadfn("glColor3s", &[])),
Color3sv: FnPtr::new(metaloadfn("glColor3sv", &[])),
Color3ub: FnPtr::new(metaloadfn("glColor3ub", &[])),
Color3ubv: FnPtr::new(metaloadfn("glColor3ubv", &[])),
Color3ui: FnPtr::new(metaloadfn("glColor3ui", &[])),
Color3uiv: FnPtr::new(metaloadfn("glColor3uiv", &[])),
Color3us: FnPtr::new(metaloadfn("glColor3us", &[])),
Color3usv: FnPtr::new(metaloadfn("glColor3usv", &[])),
Color4b: FnPtr::new(metaloadfn("glColor4b", &[])),
Color4bv: FnPtr::new(metaloadfn("glColor4bv", &[])),
Color4d: FnPtr::new(metaloadfn("glColor4d", &[])),
Color4dv: FnPtr::new(metaloadfn("glColor4dv", &[])),
Color4f: FnPtr::new(metaloadfn("glColor4f", &[])),
Color4fv: FnPtr::new(metaloadfn("glColor4fv", &[])),
Color4i: FnPtr::new(metaloadfn("glColor4i", &[])),
Color4iv: FnPtr::new(metaloadfn("glColor4iv", &[])),
Color4s: FnPtr::new(metaloadfn("glColor4s", &[])),
Color4sv: FnPtr::new(metaloadfn("glColor4sv", &[])),
Color4ub: FnPtr::new(metaloadfn("glColor4ub", &[])),
Color4ubv: FnPtr::new(metaloadfn("glColor4ubv", &[])),
Color4ui: FnPtr::new(metaloadfn("glColor4ui", &[])),
Color4uiv: FnPtr::new(metaloadfn("glColor4uiv", &[])),
Color4us: FnPtr::new(metaloadfn("glColor4us", &[])),
Color4usv: FnPtr::new(metaloadfn("glColor4usv", &[])),
ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
ColorMaterial: FnPtr::new(metaloadfn("glColorMaterial", &[])),
ColorPointer: FnPtr::new(metaloadfn("glColorPointer", &[])),
CopyPixels: FnPtr::new(metaloadfn("glCopyPixels", &[])),
CopyTexImage1D: FnPtr::new(metaloadfn("glCopyTexImage1D", &["glCopyTexImage1DEXT"])),
CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &["glCopyTexImage2DEXT"])),
CopyTexSubImage1D: FnPtr::new(metaloadfn("glCopyTexSubImage1D", &["glCopyTexSubImage1DEXT"])),
CopyTexSubImage2D: FnPtr::new(metaloadfn("glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"])),
CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
DeleteLists: FnPtr::new(metaloadfn("glDeleteLists", &[])),
DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
DepthRange: FnPtr::new(metaloadfn("glDepthRange", &[])),
Disable: FnPtr::new(metaloadfn("glDisable", &[])),
DisableClientState: FnPtr::new(metaloadfn("glDisableClientState", &[])),
DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &["glDrawArraysEXT"])),
DrawBuffer: FnPtr::new(metaloadfn("glDrawBuffer", &[])),
DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
DrawPixels: FnPtr::new(metaloadfn("glDrawPixels", &[])),
EdgeFlag: FnPtr::new(metaloadfn("glEdgeFlag", &[])),
EdgeFlagPointer: FnPtr::new(metaloadfn("glEdgeFlagPointer", &[])),
EdgeFlagv: FnPtr::new(metaloadfn("glEdgeFlagv", &[])),
Enable: FnPtr::new(metaloadfn("glEnable", &[])),
EnableClientState: FnPtr::new(metaloadfn("glEnableClientState", &[])),
End: FnPtr::new(metaloadfn("glEnd", &[])),
EndList: FnPtr::new(metaloadfn("glEndList", &[])),
EvalCoord1d: FnPtr::new(metaloadfn("glEvalCoord1d", &[])),
EvalCoord1dv: FnPtr::new(metaloadfn("glEvalCoord1dv", &[])),
EvalCoord1f: FnPtr::new(metaloadfn("glEvalCoord1f", &[])),
EvalCoord1fv: FnPtr::new(metaloadfn("glEvalCoord1fv", &[])),
EvalCoord2d: FnPtr::new(metaloadfn("glEvalCoord2d", &[])),
EvalCoord2dv: FnPtr::new(metaloadfn("glEvalCoord2dv", &[])),
EvalCoord2f: FnPtr::new(metaloadfn("glEvalCoord2f", &[])),
EvalCoord2fv: FnPtr::new(metaloadfn("glEvalCoord2fv", &[])),
EvalMesh1: FnPtr::new(metaloadfn("glEvalMesh1", &[])),
EvalMesh2: FnPtr::new(metaloadfn("glEvalMesh2", &[])),
EvalPoint1: FnPtr::new(metaloadfn("glEvalPoint1", &[])),
EvalPoint2: FnPtr::new(metaloadfn("glEvalPoint2", &[])),
FeedbackBuffer: FnPtr::new(metaloadfn("glFeedbackBuffer", &[])),
Finish: FnPtr::new(metaloadfn("glFinish", &[])),
Flush: FnPtr::new(metaloadfn("glFlush", &[])),
Fogf: FnPtr::new(metaloadfn("glFogf", &[])),
Fogfv: FnPtr::new(metaloadfn("glFogfv", &[])),
Fogi: FnPtr::new(metaloadfn("glFogi", &[])),
Fogiv: FnPtr::new(metaloadfn("glFogiv", &[])),
FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
Frustum: FnPtr::new(metaloadfn("glFrustum", &[])),
GenLists: FnPtr::new(metaloadfn("glGenLists", &[])),
GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
GetClipPlane: FnPtr::new(metaloadfn("glGetClipPlane", &[])),
GetDoublev: FnPtr::new(metaloadfn("glGetDoublev", &[])),
GetError: FnPtr::new(metaloadfn("glGetError", &[])),
GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
GetLightfv: FnPtr::new(metaloadfn("glGetLightfv", &[])),
GetLightiv: FnPtr::new(metaloadfn("glGetLightiv", &[])),
GetMapdv: FnPtr::new(metaloadfn("glGetMapdv", &[])),
GetMapfv: FnPtr::new(metaloadfn("glGetMapfv", &[])),
GetMapiv: FnPtr::new(metaloadfn("glGetMapiv", &[])),
GetMaterialfv: FnPtr::new(metaloadfn("glGetMaterialfv", &[])),
GetMaterialiv: FnPtr::new(metaloadfn("glGetMaterialiv", &[])),
GetPixelMapfv: FnPtr::new(metaloadfn("glGetPixelMapfv", &[])),
GetPixelMapuiv: FnPtr::new(metaloadfn("glGetPixelMapuiv", &[])),
GetPixelMapusv: FnPtr::new(metaloadfn("glGetPixelMapusv", &[])),
GetPointerv: FnPtr::new(metaloadfn("glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"])),
GetPolygonStipple: FnPtr::new(metaloadfn("glGetPolygonStipple", &[])),
GetString: FnPtr::new(metaloadfn("glGetString", &[])),
GetTexEnvfv: FnPtr::new(metaloadfn("glGetTexEnvfv", &[])),
GetTexEnviv: FnPtr::new(metaloadfn("glGetTexEnviv", &[])),
GetTexGendv: FnPtr::new(metaloadfn("glGetTexGendv", &[])),
GetTexGenfv: FnPtr::new(metaloadfn("glGetTexGenfv", &[])),
GetTexGeniv: FnPtr::new(metaloadfn("glGetTexGeniv", &[])),
GetTexImage: FnPtr::new(metaloadfn("glGetTexImage", &[])),
GetTexLevelParameterfv: FnPtr::new(metaloadfn("glGetTexLevelParameterfv", &[])),
GetTexLevelParameteriv: FnPtr::new(metaloadfn("glGetTexLevelParameteriv", &[])),
GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
Hint: FnPtr::new(metaloadfn("glHint", &[])),
IndexMask: FnPtr::new(metaloadfn("glIndexMask", &[])),
IndexPointer: FnPtr::new(metaloadfn("glIndexPointer", &[])),
Indexd: FnPtr::new(metaloadfn("glIndexd", &[])),
Indexdv: FnPtr::new(metaloadfn("glIndexdv", &[])),
Indexf: FnPtr::new(metaloadfn("glIndexf", &[])),
Indexfv: FnPtr::new(metaloadfn("glIndexfv", &[])),
Indexi: FnPtr::new(metaloadfn("glIndexi", &[])),
Indexiv: FnPtr::new(metaloadfn("glIndexiv", &[])),
Indexs: FnPtr::new(metaloadfn("glIndexs", &[])),
Indexsv: FnPtr::new(metaloadfn("glIndexsv", &[])),
Indexub: FnPtr::new(metaloadfn("glIndexub", &[])),
Indexubv: FnPtr::new(metaloadfn("glIndexubv", &[])),
InitNames: FnPtr::new(metaloadfn("glInitNames", &[])),
InterleavedArrays: FnPtr::new(metaloadfn("glInterleavedArrays", &[])),
IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
IsList: FnPtr::new(metaloadfn("glIsList", &[])),
IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
LightModelf: FnPtr::new(metaloadfn("glLightModelf", &[])),
LightModelfv: FnPtr::new(metaloadfn("glLightModelfv", &[])),
LightModeli: FnPtr::new(metaloadfn("glLightModeli", &[])),
LightModeliv: FnPtr::new(metaloadfn("glLightModeliv", &[])),
Lightf: FnPtr::new(metaloadfn("glLightf", &[])),
Lightfv: FnPtr::new(metaloadfn("glLightfv", &[])),
Lighti: FnPtr::new(metaloadfn("glLighti", &[])),
Lightiv: FnPtr::new(metaloadfn("glLightiv", &[])),
LineStipple: FnPtr::new(metaloadfn("glLineStipple", &[])),
LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
ListBase: FnPtr::new(metaloadfn("glListBase", &[])),
LoadIdentity: FnPtr::new(metaloadfn("glLoadIdentity", &[])),
LoadMatrixd: FnPtr::new(metaloadfn("glLoadMatrixd", &[])),
LoadMatrixf: FnPtr::new(metaloadfn("glLoadMatrixf", &[])),
LoadName: FnPtr::new(metaloadfn("glLoadName", &[])),
LogicOp: FnPtr::new(metaloadfn("glLogicOp", &[])),
Map1d: FnPtr::new(metaloadfn("glMap1d", &[])),
Map1f: FnPtr::new(metaloadfn("glMap1f", &[])),
Map2d: FnPtr::new(metaloadfn("glMap2d", &[])),
Map2f: FnPtr::new(metaloadfn("glMap2f", &[])),
MapGrid1d: FnPtr::new(metaloadfn("glMapGrid1d", &[])),
MapGrid1f: FnPtr::new(metaloadfn("glMapGrid1f", &[])),
MapGrid2d: FnPtr::new(metaloadfn("glMapGrid2d", &[])),
MapGrid2f: FnPtr::new(metaloadfn("glMapGrid2f", &[])),
Materialf: FnPtr::new(metaloadfn("glMaterialf", &[])),
Materialfv: FnPtr::new(metaloadfn("glMaterialfv", &[])),
Materiali: FnPtr::new(metaloadfn("glMateriali", &[])),
Materialiv: FnPtr::new(metaloadfn("glMaterialiv", &[])),
MatrixMode: FnPtr::new(metaloadfn("glMatrixMode", &[])),
MultMatrixd: FnPtr::new(metaloadfn("glMultMatrixd", &[])),
MultMatrixf: FnPtr::new(metaloadfn("glMultMatrixf", &[])),
NewList: FnPtr::new(metaloadfn("glNewList", &[])),
Normal3b: FnPtr::new(metaloadfn("glNormal3b", &[])),
Normal3bv: FnPtr::new(metaloadfn("glNormal3bv", &[])),
Normal3d: FnPtr::new(metaloadfn("glNormal3d", &[])),
Normal3dv: FnPtr::new(metaloadfn("glNormal3dv", &[])),
Normal3f: FnPtr::new(metaloadfn("glNormal3f", &[])),
Normal3fv: FnPtr::new(metaloadfn("glNormal3fv", &[])),
Normal3i: FnPtr::new(metaloadfn("glNormal3i", &[])),
Normal3iv: FnPtr::new(metaloadfn("glNormal3iv", &[])),
Normal3s: FnPtr::new(metaloadfn("glNormal3s", &[])),
Normal3sv: FnPtr::new(metaloadfn("glNormal3sv", &[])),
NormalPointer: FnPtr::new(metaloadfn("glNormalPointer", &[])),
Ortho: FnPtr::new(metaloadfn("glOrtho", &[])),
PassThrough: FnPtr::new(metaloadfn("glPassThrough", &[])),
PixelMapfv: FnPtr::new(metaloadfn("glPixelMapfv", &[])),
PixelMapuiv: FnPtr::new(metaloadfn("glPixelMapuiv", &[])),
PixelMapusv: FnPtr::new(metaloadfn("glPixelMapusv", &[])),
PixelStoref: FnPtr::new(metaloadfn("glPixelStoref", &[])),
PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
PixelTransferf: FnPtr::new(metaloadfn("glPixelTransferf", &[])),
PixelTransferi: FnPtr::new(metaloadfn("glPixelTransferi", &[])),
PixelZoom: FnPtr::new(metaloadfn("glPixelZoom", &[])),
PointSize: FnPtr::new(metaloadfn("glPointSize", &[])),
PolygonMode: FnPtr::new(metaloadfn("glPolygonMode", &[])),
PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
PolygonStipple: FnPtr::new(metaloadfn("glPolygonStipple", &[])),
PopAttrib: FnPtr::new(metaloadfn("glPopAttrib", &[])),
PopClientAttrib: FnPtr::new(metaloadfn("glPopClientAttrib", &[])),
PopMatrix: FnPtr::new(metaloadfn("glPopMatrix", &[])),
PopName: FnPtr::new(metaloadfn("glPopName", &[])),
PrioritizeTextures: FnPtr::new(metaloadfn("glPrioritizeTextures", &["glPrioritizeTexturesEXT"])),
PushAttrib: FnPtr::new(metaloadfn("glPushAttrib", &[])),
PushClientAttrib: FnPtr::new(metaloadfn("glPushClientAttrib", &[])),
PushMatrix: FnPtr::new(metaloadfn("glPushMatrix", &[])),
PushName: FnPtr::new(metaloadfn("glPushName", &[])),
RasterPos2d: FnPtr::new(metaloadfn("glRasterPos2d", &[])),
RasterPos2dv: FnPtr::new(metaloadfn("glRasterPos2dv", &[])),
RasterPos2f: FnPtr::new(metaloadfn("glRasterPos2f", &[])),
RasterPos2fv: FnPtr::new(metaloadfn("glRasterPos2fv", &[])),
RasterPos2i: FnPtr::new(metaloadfn("glRasterPos2i", &[])),
RasterPos2iv: FnPtr::new(metaloadfn("glRasterPos2iv", &[])),
RasterPos2s: FnPtr::new(metaloadfn("glRasterPos2s", &[])),
RasterPos2sv: FnPtr::new(metaloadfn("glRasterPos2sv", &[])),
RasterPos3d: FnPtr::new(metaloadfn("glRasterPos3d", &[])),
RasterPos3dv: FnPtr::new(metaloadfn("glRasterPos3dv", &[])),
RasterPos3f: FnPtr::new(metaloadfn("glRasterPos3f", &[])),
RasterPos3fv: FnPtr::new(metaloadfn("glRasterPos3fv", &[])),
RasterPos3i: FnPtr::new(metaloadfn("glRasterPos3i", &[])),
RasterPos3iv: FnPtr::new(metaloadfn("glRasterPos3iv", &[])),
RasterPos3s: FnPtr::new(metaloadfn("glRasterPos3s", &[])),
RasterPos3sv: FnPtr::new(metaloadfn("glRasterPos3sv", &[])),
RasterPos4d: FnPtr::new(metaloadfn("glRasterPos4d", &[])),
RasterPos4dv: FnPtr::new(metaloadfn("glRasterPos4dv", &[])),
RasterPos4f: FnPtr::new(metaloadfn("glRasterPos4f", &[])),
RasterPos4fv: FnPtr::new(metaloadfn("glRasterPos4fv", &[])),
RasterPos4i: FnPtr::new(metaloadfn("glRasterPos4i", &[])),
RasterPos4iv: FnPtr::new(metaloadfn("glRasterPos4iv", &[])),
RasterPos4s: FnPtr::new(metaloadfn("glRasterPos4s", &[])),
RasterPos4sv: FnPtr::new(metaloadfn("glRasterPos4sv", &[])),
ReadBuffer: FnPtr::new(metaloadfn("glReadBuffer", &[])),
ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
Rectd: FnPtr::new(metaloadfn("glRectd", &[])),
Rectdv: FnPtr::new(metaloadfn("glRectdv", &[])),
Rectf: FnPtr::new(metaloadfn("glRectf", &[])),
Rectfv: FnPtr::new(metaloadfn("glRectfv", &[])),
Recti: FnPtr::new(metaloadfn("glRecti", &[])),
Rectiv: FnPtr::new(metaloadfn("glRectiv", &[])),
Rects: FnPtr::new(metaloadfn("glRects", &[])),
Rectsv: FnPtr::new(metaloadfn("glRectsv", &[])),
RenderMode: FnPtr::new(metaloadfn("glRenderMode", &[])),
Rotated: FnPtr::new(metaloadfn("glRotated", &[])),
Rotatef: FnPtr::new(metaloadfn("glRotatef", &[])),
Scaled: FnPtr::new(metaloadfn("glScaled", &[])),
Scalef: FnPtr::new(metaloadfn("glScalef", &[])),
Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
SelectBuffer: FnPtr::new(metaloadfn("glSelectBuffer", &[])),
ShadeModel: FnPtr::new(metaloadfn("glShadeModel", &[])),
StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
TexCoord1d: FnPtr::new(metaloadfn("glTexCoord1d", &[])),
TexCoord1dv: FnPtr::new(metaloadfn("glTexCoord1dv", &[])),
TexCoord1f: FnPtr::new(metaloadfn("glTexCoord1f", &[])),
TexCoord1fv: FnPtr::new(metaloadfn("glTexCoord1fv", &[])),
TexCoord1i: FnPtr::new(metaloadfn("glTexCoord1i", &[])),
TexCoord1iv: FnPtr::new(metaloadfn("glTexCoord1iv", &[])),
TexCoord1s: FnPtr::new(metaloadfn("glTexCoord1s", &[])),
TexCoord1sv: FnPtr::new(metaloadfn("glTexCoord1sv", &[])),
TexCoord2d: FnPtr::new(metaloadfn("glTexCoord2d", &[])),
TexCoord2dv: FnPtr::new(metaloadfn("glTexCoord2dv", &[])),
TexCoord2f: FnPtr::new(metaloadfn("glTexCoord2f", &[])),
TexCoord2fv: FnPtr::new(metaloadfn("glTexCoord2fv", &[])),
TexCoord2i: FnPtr::new(metaloadfn("glTexCoord2i", &[])),
TexCoord2iv: FnPtr::new(metaloadfn("glTexCoord2iv", &[])),
TexCoord2s: FnPtr::new(metaloadfn("glTexCoord2s", &[])),
TexCoord2sv: FnPtr::new(metaloadfn("glTexCoord2sv", &[])),
TexCoord3d: FnPtr::new(metaloadfn("glTexCoord3d", &[])),
TexCoord3dv: FnPtr::new(metaloadfn("glTexCoord3dv", &[])),
TexCoord3f: FnPtr::new(metaloadfn("glTexCoord3f", &[])),
TexCoord3fv: FnPtr::new(metaloadfn("glTexCoord3fv", &[])),
TexCoord3i: FnPtr::new(metaloadfn("glTexCoord3i", &[])),
TexCoord3iv: FnPtr::new(metaloadfn("glTexCoord3iv", &[])),
TexCoord3s: FnPtr::new(metaloadfn("glTexCoord3s", &[])),
TexCoord3sv: FnPtr::new(metaloadfn("glTexCoord3sv", &[])),
TexCoord4d: FnPtr::new(metaloadfn("glTexCoord4d", &[])),
TexCoord4dv: FnPtr::new(metaloadfn("glTexCoord4dv", &[])),
TexCoord4f: FnPtr::new(metaloadfn("glTexCoord4f", &[])),
TexCoord4fv: FnPtr::new(metaloadfn("glTexCoord4fv", &[])),
TexCoord4i: FnPtr::new(metaloadfn("glTexCoord4i", &[])),
TexCoord4iv: FnPtr::new(metaloadfn("glTexCoord4iv", &[])),
TexCoord4s: FnPtr::new(metaloadfn("glTexCoord4s", &[])),
TexCoord4sv: FnPtr::new(metaloadfn("glTexCoord4sv", &[])),
TexCoordPointer: FnPtr::new(metaloadfn("glTexCoordPointer", &[])),
TexEnvf: FnPtr::new(metaloadfn("glTexEnvf", &[])),
TexEnvfv: FnPtr::new(metaloadfn("glTexEnvfv", &[])),
TexEnvi: FnPtr::new(metaloadfn("glTexEnvi", &[])),
TexEnviv: FnPtr::new(metaloadfn("glTexEnviv", &[])),
TexGend: FnPtr::new(metaloadfn("glTexGend", &[])),
TexGendv: FnPtr::new(metaloadfn("glTexGendv", &[])),
TexGenf: FnPtr::new(metaloadfn("glTexGenf", &[])),
TexGenfv: FnPtr::new(metaloadfn("glTexGenfv", &[])),
TexGeni: FnPtr::new(metaloadfn("glTexGeni", &[])),
TexGeniv: FnPtr::new(metaloadfn("glTexGeniv", &[])),
TexImage1D: FnPtr::new(metaloadfn("glTexImage1D", &[])),
TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
TexSubImage1D: FnPtr::new(metaloadfn("glTexSubImage1D", &["glTexSubImage1DEXT"])),
TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &["glTexSubImage2DEXT"])),
Translated: FnPtr::new(metaloadfn("glTranslated", &[])),
Translatef: FnPtr::new(metaloadfn("glTranslatef", &[])),
Vertex2d: FnPtr::new(metaloadfn("glVertex2d", &[])),
Vertex2dv: FnPtr::new(metaloadfn("glVertex2dv", &[])),
Vertex2f: FnPtr::new(metaloadfn("glVertex2f", &[])),
Vertex2fv: FnPtr::new(metaloadfn("glVertex2fv", &[])),
Vertex2i: FnPtr::new(metaloadfn("glVertex2i", &[])),
Vertex2iv: FnPtr::new(metaloadfn("glVertex2iv", &[])),
Vertex2s: FnPtr::new(metaloadfn("glVertex2s", &[])),
Vertex2sv: FnPtr::new(metaloadfn("glVertex2sv", &[])),
Vertex3d: FnPtr::new(metaloadfn("glVertex3d", &[])),
Vertex3dv: FnPtr::new(metaloadfn("glVertex3dv", &[])),
Vertex3f: FnPtr::new(metaloadfn("glVertex3f", &[])),
Vertex3fv: FnPtr::new(metaloadfn("glVertex3fv", &[])),
Vertex3i: FnPtr::new(metaloadfn("glVertex3i", &[])),
Vertex3iv: FnPtr::new(metaloadfn("glVertex3iv", &[])),
Vertex3s: FnPtr::new(metaloadfn("glVertex3s", &[])),
Vertex3sv: FnPtr::new(metaloadfn("glVertex3sv", &[])),
Vertex4d: FnPtr::new(metaloadfn("glVertex4d", &[])),
Vertex4dv: FnPtr::new(metaloadfn("glVertex4dv", &[])),
Vertex4f: FnPtr::new(metaloadfn("glVertex4f", &[])),
Vertex4fv: FnPtr::new(metaloadfn("glVertex4fv", &[])),
Vertex4i: FnPtr::new(metaloadfn("glVertex4i", &[])),
Vertex4iv: FnPtr::new(metaloadfn("glVertex4iv", &[])),
Vertex4s: FnPtr::new(metaloadfn("glVertex4s", &[])),
Vertex4sv: FnPtr::new(metaloadfn("glVertex4sv", &[])),
VertexPointer: FnPtr::new(metaloadfn("glVertexPointer", &[])),
Viewport: FnPtr::new(metaloadfn("glViewport", &[])),
}
        }

        /// Load each OpenGL symbol using a custom load function.
        ///
        /// ~~~ignore
        /// let gl = Gl::load(&glfw);
        /// ~~~
        #[allow(dead_code)]
        #[allow(unused_variables)]
        pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) -> Gl {
            Gl::load_with(|name| loader.get_proc_addr(name))
        }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Accum(&self, op: types::GLenum, value: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.Accum.f)(op, value) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn AlphaFunc(&self, func: types::GLenum, ref_: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.AlphaFunc.f)(func, ref_) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn AreTexturesResident(&self, n: types::GLsizei, textures: *const types::GLuint, residences: *mut types::GLboolean) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, *mut types::GLboolean) -> types::GLboolean>(self.AreTexturesResident.f)(n, textures, residences) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ArrayElement(&self, i: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ArrayElement.f)(i) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Begin(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Begin.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BindTexture(&self, target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindTexture.f)(target, texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Bitmap(&self, width: types::GLsizei, height: types::GLsizei, xorig: types::GLfloat, yorig: types::GLfloat, xmove: types::GLfloat, ymove: types::GLfloat, bitmap: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLsizei, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, *const types::GLubyte) -> ()>(self.Bitmap.f)(width, height, xorig, yorig, xmove, ymove, bitmap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BlendFunc(&self, sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CallList(&self, list: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.CallList.f)(list) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CallLists(&self, n: types::GLsizei, type_: types::GLenum, lists: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.CallLists.f)(n, type_, lists) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Clear(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.Clear.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearAccum(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ClearAccum.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearDepth(&self, depth: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.ClearDepth.f)(depth) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearIndex(&self, c: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ClearIndex.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearStencil(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ClearStencil.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClipPlane(&self, plane: types::GLenum, equation: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLdouble) -> ()>(self.ClipPlane.f)(plane, equation) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3b(&self, red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(self.Color3b.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3bv(&self, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(self.Color3bv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3d(&self, red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Color3d.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Color3dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3f(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Color3f.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Color3fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3i(&self, red: types::GLint, green: types::GLint, blue: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.Color3i.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Color3iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3s(&self, red: types::GLshort, green: types::GLshort, blue: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Color3s.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Color3sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3ub(&self, red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(self.Color3ub.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3ubv(&self, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(self.Color3ubv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3ui(&self, red: types::GLuint, green: types::GLuint, blue: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.Color3ui.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3uiv(&self, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLuint) -> ()>(self.Color3uiv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3us(&self, red: types::GLushort, green: types::GLushort, blue: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLushort, types::GLushort, types::GLushort) -> ()>(self.Color3us.f)(red, green, blue) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color3usv(&self, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLushort) -> ()>(self.Color3usv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4b(&self, red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte, alpha: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(self.Color4b.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4bv(&self, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(self.Color4bv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4d(&self, red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble, alpha: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Color4d.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Color4dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4f(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Color4f.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Color4fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4i(&self, red: types::GLint, green: types::GLint, blue: types::GLint, alpha: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Color4i.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Color4iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4s(&self, red: types::GLshort, green: types::GLshort, blue: types::GLshort, alpha: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Color4s.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Color4sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4ub(&self, red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte, alpha: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(self.Color4ub.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4ubv(&self, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(self.Color4ubv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4ui(&self, red: types::GLuint, green: types::GLuint, blue: types::GLuint, alpha: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.Color4ui.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4uiv(&self, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLuint) -> ()>(self.Color4uiv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4us(&self, red: types::GLushort, green: types::GLushort, blue: types::GLushort, alpha: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLushort, types::GLushort, types::GLushort, types::GLushort) -> ()>(self.Color4us.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4usv(&self, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLushort) -> ()>(self.Color4usv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ColorMask(&self, red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(self.ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ColorMaterial(&self, face: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.ColorMaterial.f)(face, mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ColorPointer(&self, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.ColorPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, type_: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum) -> ()>(self.CopyPixels.f)(x, y, width, height, type_) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexImage1D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage1D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(self.CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CullFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.CullFace.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DeleteLists(&self, list: types::GLuint, range: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei) -> ()>(self.DeleteLists.f)(list, range) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteTextures.f)(n, textures) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthFunc(&self, func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DepthFunc.f)(func) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthMask(&self, flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(self.DepthMask.f)(flag) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthRange(&self, near: types::GLdouble, far: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.DepthRange.f)(near, far) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Disable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Disable.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DisableClientState(&self, array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DisableClientState.f)(array) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawArrays(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(self.DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawBuffer(&self, buf: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DrawBuffer.f)(buf) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawElements(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawPixels(&self, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.DrawPixels.f)(width, height, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EdgeFlag(&self, flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(self.EdgeFlag.f)(flag) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EdgeFlagPointer(&self, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.EdgeFlagPointer.f)(stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EdgeFlagv(&self, flag: *const types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLboolean) -> ()>(self.EdgeFlagv.f)(flag) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Enable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Enable.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EnableClientState(&self, array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.EnableClientState.f)(array) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn End(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.End.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EndList(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.EndList.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord1d(&self, u: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.EvalCoord1d.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord1dv(&self, u: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.EvalCoord1dv.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord1f(&self, u: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.EvalCoord1f.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord1fv(&self, u: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.EvalCoord1fv.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord2d(&self, u: types::GLdouble, v: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.EvalCoord2d.f)(u, v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord2dv(&self, u: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.EvalCoord2dv.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord2f(&self, u: types::GLfloat, v: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.EvalCoord2f.f)(u, v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalCoord2fv(&self, u: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.EvalCoord2fv.f)(u) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalMesh1(&self, mode: types::GLenum, i1: types::GLint, i2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint) -> ()>(self.EvalMesh1.f)(mode, i1, i2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalMesh2(&self, mode: types::GLenum, i1: types::GLint, i2: types::GLint, j1: types::GLint, j2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.EvalMesh2.f)(mode, i1, i2, j1, j2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalPoint1(&self, i: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.EvalPoint1.f)(i) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EvalPoint2(&self, i: types::GLint, j: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.EvalPoint2.f)(i, j) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn FeedbackBuffer(&self, size: types::GLsizei, type_: types::GLenum, buffer: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, types::GLenum, *mut types::GLfloat) -> ()>(self.FeedbackBuffer.f)(size, type_, buffer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Finish(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Finish.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Flush(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Flush.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.Fogf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.Fogfv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogi(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.Fogi.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogiv(&self, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(self.Fogiv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn FrontFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.FrontFace.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Frustum(&self, left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Frustum.f)(left, right, bottom, top, zNear, zFar) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GenLists(&self, range: types::GLsizei) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei) -> types::GLuint>(self.GenLists.f)(range) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenTextures.f)(n, textures) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetBooleanv(&self, pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(self.GetBooleanv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetClipPlane(&self, plane: types::GLenum, equation: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(self.GetClipPlane.f)(plane, equation) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetDoublev(&self, pname: types::GLenum, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(self.GetDoublev.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetError(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetError.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetFloatv(&self, pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetFloatv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetIntegerv(&self, pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(self.GetIntegerv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetLightfv(&self, light: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetLightfv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetLightiv(&self, light: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetLightiv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMapdv(&self, target: types::GLenum, query: types::GLenum, v: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLdouble) -> ()>(self.GetMapdv.f)(target, query, v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMapfv(&self, target: types::GLenum, query: types::GLenum, v: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetMapfv.f)(target, query, v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMapiv(&self, target: types::GLenum, query: types::GLenum, v: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetMapiv.f)(target, query, v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMaterialfv(&self, face: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetMaterialfv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMaterialiv(&self, face: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetMaterialiv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPixelMapfv(&self, map: types::GLenum, values: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetPixelMapfv.f)(map, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPixelMapuiv(&self, map: types::GLenum, values: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLuint) -> ()>(self.GetPixelMapuiv.f)(map, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPixelMapusv(&self, map: types::GLenum, values: *mut types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLushort) -> ()>(self.GetPixelMapusv.f)(map, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPointerv(&self, pname: types::GLenum, params: *const *mut __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::libc::c_void) -> ()>(self.GetPointerv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPolygonStipple(&self, mask: *mut types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*mut types::GLubyte) -> ()>(self.GetPolygonStipple.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetString(&self, name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(self.GetString.f)(name) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexEnvfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexEnviv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexEnviv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexGendv(&self, coord: types::GLenum, pname: types::GLenum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLdouble) -> ()>(self.GetTexGendv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexGenfv(&self, coord: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexGenfv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexGeniv(&self, coord: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexGeniv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexImage(&self, target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, *mut __gl_imports::libc::c_void) -> ()>(self.GetTexImage.f)(target, level, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameterfv(&self, target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexLevelParameterfv.f)(target, level, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameteriv(&self, target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> ()>(self.GetTexLevelParameteriv.f)(target, level, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Hint(&self, target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.Hint.f)(target, mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IndexMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.IndexMask.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IndexPointer(&self, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.IndexPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexd(&self, c: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.Indexd.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexdv(&self, c: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Indexdv.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexf(&self, c: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.Indexf.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexfv(&self, c: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Indexfv.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexi(&self, c: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.Indexi.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexiv(&self, c: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Indexiv.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexs(&self, c: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort) -> ()>(self.Indexs.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexsv(&self, c: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Indexsv.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexub(&self, c: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte) -> ()>(self.Indexub.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Indexubv(&self, c: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(self.Indexubv.f)(c) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn InitNames(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.InitNames.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn InterleavedArrays(&self, format: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.InterleavedArrays.f)(format, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsEnabled(&self, cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(self.IsEnabled.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsList(&self, list: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsList.f)(list) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsTexture.f)(texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.LightModelf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.LightModelfv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModeli(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.LightModeli.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModeliv(&self, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(self.LightModeliv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightf(&self, light: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.Lightf.f)(light, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightfv(&self, light: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.Lightfv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lighti(&self, light: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.Lighti.f)(light, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightiv(&self, light: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.Lightiv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LineStipple(&self, factor: types::GLint, pattern: types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLushort) -> ()>(self.LineStipple.f)(factor, pattern) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.LineWidth.f)(width) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ListBase(&self, base: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ListBase.f)(base) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadIdentity(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.LoadIdentity.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadMatrixd(&self, m: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.LoadMatrixd.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadMatrixf(&self, m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.LoadMatrixf.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadName(&self, name: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.LoadName.f)(name) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LogicOp(&self, opcode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.LogicOp.f)(opcode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Map1d(&self, target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, stride: types::GLint, order: types::GLint, points: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLdouble, types::GLdouble, types::GLint, types::GLint, *const types::GLdouble) -> ()>(self.Map1d.f)(target, u1, u2, stride, order, points) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Map1f(&self, target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, stride: types::GLint, order: types::GLint, points: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLint, types::GLint, *const types::GLfloat) -> ()>(self.Map1f.f)(target, u1, u2, stride, order, points) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Map2d(&self, target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, ustride: types::GLint, uorder: types::GLint, v1: types::GLdouble, v2: types::GLdouble, vstride: types::GLint, vorder: types::GLint, points: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLdouble, types::GLdouble, types::GLint, types::GLint, types::GLdouble, types::GLdouble, types::GLint, types::GLint, *const types::GLdouble) -> ()>(self.Map2d.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Map2f(&self, target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, ustride: types::GLint, uorder: types::GLint, v1: types::GLfloat, v2: types::GLfloat, vstride: types::GLint, vorder: types::GLint, points: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLint, types::GLint, types::GLfloat, types::GLfloat, types::GLint, types::GLint, *const types::GLfloat) -> ()>(self.Map2f.f)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MapGrid1d(&self, un: types::GLint, u1: types::GLdouble, u2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.MapGrid1d.f)(un, u1, u2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MapGrid1f(&self, un: types::GLint, u1: types::GLfloat, u2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.MapGrid1f.f)(un, u1, u2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MapGrid2d(&self, un: types::GLint, u1: types::GLdouble, u2: types::GLdouble, vn: types::GLint, v1: types::GLdouble, v2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.MapGrid2d.f)(un, u1, u2, vn, v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MapGrid2f(&self, un: types::GLint, u1: types::GLfloat, u2: types::GLfloat, vn: types::GLint, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.MapGrid2f.f)(un, u1, u2, vn, v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialf(&self, face: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.Materialf.f)(face, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialfv(&self, face: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.Materialfv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materiali(&self, face: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.Materiali.f)(face, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialiv(&self, face: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.Materialiv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MatrixMode(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.MatrixMode.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultMatrixd(&self, m: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.MultMatrixd.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultMatrixf(&self, m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.MultMatrixf.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn NewList(&self, list: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.NewList.f)(list, mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3b(&self, nx: types::GLbyte, ny: types::GLbyte, nz: types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbyte, types::GLbyte, types::GLbyte) -> ()>(self.Normal3b.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3bv(&self, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLbyte) -> ()>(self.Normal3bv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3d(&self, nx: types::GLdouble, ny: types::GLdouble, nz: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Normal3d.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Normal3dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3f(&self, nx: types::GLfloat, ny: types::GLfloat, nz: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Normal3f.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Normal3fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3i(&self, nx: types::GLint, ny: types::GLint, nz: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.Normal3i.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Normal3iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3s(&self, nx: types::GLshort, ny: types::GLshort, nz: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Normal3s.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Normal3sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn NormalPointer(&self, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.NormalPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Ortho(&self, left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Ortho.f)(left, right, bottom, top, zNear, zFar) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PassThrough(&self, token: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.PassThrough.f)(token) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelMapfv(&self, map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLfloat) -> ()>(self.PixelMapfv.f)(map, mapsize, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelMapuiv(&self, map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLuint) -> ()>(self.PixelMapuiv.f)(map, mapsize, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelMapusv(&self, map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLushort) -> ()>(self.PixelMapusv.f)(map, mapsize, values) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelStoref(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.PixelStoref.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelStorei(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PixelStorei.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelTransferf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.PixelTransferf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelTransferi(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PixelTransferi.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelZoom(&self, xfactor: types::GLfloat, yfactor: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.PixelZoom.f)(xfactor, yfactor) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointSize(&self, size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.PointSize.f)(size) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PolygonMode(&self, face: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.PolygonMode.f)(face, mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.PolygonOffset.f)(factor, units) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PolygonStipple(&self, mask: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLubyte) -> ()>(self.PolygonStipple.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PopAttrib(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopAttrib.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PopClientAttrib(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopClientAttrib.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PopMatrix(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopMatrix.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PopName(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopName.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PrioritizeTextures(&self, n: types::GLsizei, textures: *const types::GLuint, priorities: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, *const types::GLfloat) -> ()>(self.PrioritizeTextures.f)(n, textures, priorities) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PushAttrib(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.PushAttrib.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PushClientAttrib(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.PushClientAttrib.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PushMatrix(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PushMatrix.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PushName(&self, name: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.PushName.f)(name) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2d(&self, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.RasterPos2d.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.RasterPos2dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2f(&self, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.RasterPos2f.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.RasterPos2fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2i(&self, x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.RasterPos2i.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.RasterPos2iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2s(&self, x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(self.RasterPos2s.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos2sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.RasterPos2sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3d(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.RasterPos3d.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.RasterPos3dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3f(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.RasterPos3f.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.RasterPos3fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3i(&self, x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.RasterPos3i.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.RasterPos3iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3s(&self, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(self.RasterPos3s.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos3sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.RasterPos3sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4d(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.RasterPos4d.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.RasterPos4dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4f(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.RasterPos4f.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.RasterPos4fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4i(&self, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.RasterPos4i.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.RasterPos4iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4s(&self, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.RasterPos4s.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RasterPos4sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.RasterPos4sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ReadBuffer(&self, src: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ReadBuffer.f)(src) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ReadPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::libc::c_void) -> ()>(self.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectd(&self, x1: types::GLdouble, y1: types::GLdouble, x2: types::GLdouble, y2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Rectd.f)(x1, y1, x2, y2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectdv(&self, v1: *const types::GLdouble, v2: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble, *const types::GLdouble) -> ()>(self.Rectdv.f)(v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectf(&self, x1: types::GLfloat, y1: types::GLfloat, x2: types::GLfloat, y2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Rectf.f)(x1, y1, x2, y2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectfv(&self, v1: *const types::GLfloat, v2: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat, *const types::GLfloat) -> ()>(self.Rectfv.f)(v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Recti(&self, x1: types::GLint, y1: types::GLint, x2: types::GLint, y2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Recti.f)(x1, y1, x2, y2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectiv(&self, v1: *const types::GLint, v2: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint, *const types::GLint) -> ()>(self.Rectiv.f)(v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rects(&self, x1: types::GLshort, y1: types::GLshort, x2: types::GLshort, y2: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Rects.f)(x1, y1, x2, y2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rectsv(&self, v1: *const types::GLshort, v2: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort, *const types::GLshort) -> ()>(self.Rectsv.f)(v1, v2) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RenderMode(&self, mode: types::GLenum) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLint>(self.RenderMode.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rotated(&self, angle: types::GLdouble, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Rotated.f)(angle, x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rotatef(&self, angle: types::GLfloat, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Rotatef.f)(angle, x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scaled(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Scaled.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scalef(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Scalef.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scissor(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Scissor.f)(x, y, width, height) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SelectBuffer(&self, size: types::GLsizei, buffer: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.SelectBuffer.f)(size, buffer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ShadeModel(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ShadeModel.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilFunc(&self, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.StencilMask.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilOp(&self, fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1d(&self, s: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.TexCoord1d.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.TexCoord1dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1f(&self, s: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.TexCoord1f.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.TexCoord1fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1i(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.TexCoord1i.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.TexCoord1iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1s(&self, s: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort) -> ()>(self.TexCoord1s.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord1sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.TexCoord1sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2d(&self, s: types::GLdouble, t: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.TexCoord2d.f)(s, t) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.TexCoord2dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2f(&self, s: types::GLfloat, t: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.TexCoord2f.f)(s, t) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.TexCoord2fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2i(&self, s: types::GLint, t: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.TexCoord2i.f)(s, t) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.TexCoord2iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2s(&self, s: types::GLshort, t: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(self.TexCoord2s.f)(s, t) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord2sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.TexCoord2sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3d(&self, s: types::GLdouble, t: types::GLdouble, r: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.TexCoord3d.f)(s, t, r) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.TexCoord3dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3f(&self, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.TexCoord3f.f)(s, t, r) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.TexCoord3fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3i(&self, s: types::GLint, t: types::GLint, r: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.TexCoord3i.f)(s, t, r) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.TexCoord3iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3s(&self, s: types::GLshort, t: types::GLshort, r: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(self.TexCoord3s.f)(s, t, r) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord3sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.TexCoord3sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4d(&self, s: types::GLdouble, t: types::GLdouble, r: types::GLdouble, q: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.TexCoord4d.f)(s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.TexCoord4dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4f(&self, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.TexCoord4f.f)(s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.TexCoord4fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4i(&self, s: types::GLint, t: types::GLint, r: types::GLint, q: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.TexCoord4i.f)(s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.TexCoord4iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4s(&self, s: types::GLshort, t: types::GLshort, r: types::GLshort, q: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.TexCoord4s.f)(s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoord4sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.TexCoord4sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexCoordPointer(&self, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.TexCoordPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexEnvf(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.TexEnvf.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexEnvfv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.TexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexEnvi(&self, target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.TexEnvi.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexEnviv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexEnviv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGend(&self, coord: types::GLenum, pname: types::GLenum, param: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLdouble) -> ()>(self.TexGend.f)(coord, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGendv(&self, coord: types::GLenum, pname: types::GLenum, params: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLdouble) -> ()>(self.TexGendv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGenf(&self, coord: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.TexGenf.f)(coord, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGenfv(&self, coord: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.TexGenfv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGeni(&self, coord: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.TexGeni.f)(coord, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexGeniv(&self, coord: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexGeniv.f)(coord, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexImage1D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexParameterf(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexParameteri(&self, target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexSubImage1D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Translated(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Translated.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Translatef(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Translatef.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2d(&self, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.Vertex2d.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Vertex2dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2f(&self, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.Vertex2f.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Vertex2fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2i(&self, x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.Vertex2i.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Vertex2iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2s(&self, x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort) -> ()>(self.Vertex2s.f)(x, y) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex2sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Vertex2sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3d(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Vertex3d.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Vertex3dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3f(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Vertex3f.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Vertex3fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3i(&self, x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.Vertex3i.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Vertex3iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3s(&self, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Vertex3s.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex3sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Vertex3sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4d(&self, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Vertex4d.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4dv(&self, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLdouble) -> ()>(self.Vertex4dv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4f(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Vertex4f.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4fv(&self, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.Vertex4fv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4i(&self, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Vertex4i.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4iv(&self, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(self.Vertex4iv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4s(&self, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.Vertex4s.f)(x, y, z, w) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Vertex4sv(&self, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(self.Vertex4sv.f)(v) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn VertexPointer(&self, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.VertexPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Viewport(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Viewport.f)(x, y, width, height) }
}

        unsafe impl __gl_imports::Send for Gl {}
