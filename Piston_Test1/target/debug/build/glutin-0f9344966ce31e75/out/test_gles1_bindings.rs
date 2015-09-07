
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
        pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000; 
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
        pub const TRUE: types::GLboolean = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ONE: types::GLenum = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERSION_ES_CL_1_0: types::GLenum = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERSION_ES_CM_1_1: types::GLenum = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERSION_ES_CL_1_1: types::GLenum = 1; 
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
        pub const FRONT: types::GLenum = 0x0404; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BACK: types::GLenum = 0x0405; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRONT_AND_BACK: types::GLenum = 0x0408; 
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
        pub const CURRENT_COLOR: types::GLenum = 0x0B00; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_NORMAL: types::GLenum = 0x0B02; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SMOOTH: types::GLenum = 0x0B10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE: types::GLenum = 0x0B11; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_SMOOTH: types::GLenum = 0x0B20; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LINE_WIDTH: types::GLenum = 0x0B21; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22; 
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
        pub const LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHADE_MODEL: types::GLenum = 0x0B54; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_MATERIAL: types::GLenum = 0x0B57; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FOG: types::GLenum = 0x0B60; 
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
        pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SCISSOR_BOX: types::GLenum = 0x0C10; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SCISSOR_TEST: types::GLenum = 0x0C11; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_WRITEMASK: types::GLenum = 0x0C23; 
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
        pub const FOG_HINT: types::GLenum = 0x0C54; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PACK_ALIGNMENT: types::GLenum = 0x0D05; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_SCALE: types::GLenum = 0x0D1C; 
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
        pub const MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36; 
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
        pub const SUBPIXEL_BITS: types::GLenum = 0x0D50; 
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
        pub const TEXTURE_2D: types::GLenum = 0x0DE1; 
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
        pub const FLOAT: types::GLenum = 0x1406; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FIXED: types::GLenum = 0x140C; 
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
        pub const MODELVIEW: types::GLenum = 0x1700; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PROJECTION: types::GLenum = 0x1701; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE: types::GLenum = 0x1702; 
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
        pub const REPEAT: types::GLenum = 0x2901; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00; 
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
        pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RESCALE_NORMAL: types::GLenum = 0x803A; 
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
        pub const TEXTURE_COORD_ARRAY: types::GLenum = 0x8078; 
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
        pub const TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A; 
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
        pub const TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MULTISAMPLE: types::GLenum = 0x809D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLES: types::GLenum = 0x80A9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE_MIN: types::GLenum = 0x8126; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_SIZE_MAX: types::GLenum = 0x8127; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const POINT_DISTANCE_ATTENUATION: types::GLenum = 0x8129; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLAMP_TO_EDGE: types::GLenum = 0x812F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GENERATE_MIPMAP: types::GLenum = 0x8191; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GENERATE_MIPMAP_HINT: types::GLenum = 0x8192; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALIASED_POINT_SIZE_RANGE: types::GLenum = 0x846D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE0: types::GLenum = 0x84C0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE1: types::GLenum = 0x84C1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE2: types::GLenum = 0x84C2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE3: types::GLenum = 0x84C3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE4: types::GLenum = 0x84C4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE5: types::GLenum = 0x84C5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE6: types::GLenum = 0x84C6; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE7: types::GLenum = 0x84C7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE8: types::GLenum = 0x84C8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE9: types::GLenum = 0x84C9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE10: types::GLenum = 0x84CA; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE11: types::GLenum = 0x84CB; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE12: types::GLenum = 0x84CC; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE13: types::GLenum = 0x84CD; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE14: types::GLenum = 0x84CE; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE15: types::GLenum = 0x84CF; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE16: types::GLenum = 0x84D0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE17: types::GLenum = 0x84D1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE18: types::GLenum = 0x84D2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE19: types::GLenum = 0x84D3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE20: types::GLenum = 0x84D4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE21: types::GLenum = 0x84D5; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE22: types::GLenum = 0x84D6; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE23: types::GLenum = 0x84D7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE24: types::GLenum = 0x84D8; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE25: types::GLenum = 0x84D9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE26: types::GLenum = 0x84DA; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE27: types::GLenum = 0x84DB; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE28: types::GLenum = 0x84DC; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE29: types::GLenum = 0x84DD; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE30: types::GLenum = 0x84DE; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE31: types::GLenum = 0x84DF; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CLIENT_ACTIVE_TEXTURE: types::GLenum = 0x84E1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const MAX_TEXTURE_UNITS: types::GLenum = 0x84E2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SUBTRACT: types::GLenum = 0x84E7; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMBINE: types::GLenum = 0x8570; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMBINE_RGB: types::GLenum = 0x8571; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMBINE_ALPHA: types::GLenum = 0x8572; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RGB_SCALE: types::GLenum = 0x8573; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ADD_SIGNED: types::GLenum = 0x8574; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const INTERPOLATE: types::GLenum = 0x8575; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONSTANT: types::GLenum = 0x8576; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PRIMARY_COLOR: types::GLenum = 0x8577; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PREVIOUS: types::GLenum = 0x8578; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC0_RGB: types::GLenum = 0x8580; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC1_RGB: types::GLenum = 0x8581; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC2_RGB: types::GLenum = 0x8582; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC0_ALPHA: types::GLenum = 0x8588; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC1_ALPHA: types::GLenum = 0x8589; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SRC2_ALPHA: types::GLenum = 0x858A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND0_RGB: types::GLenum = 0x8590; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND1_RGB: types::GLenum = 0x8591; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND2_RGB: types::GLenum = 0x8592; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND0_ALPHA: types::GLenum = 0x8598; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND1_ALPHA: types::GLenum = 0x8599; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const OPERAND2_ALPHA: types::GLenum = 0x859A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOT3_RGB: types::GLenum = 0x86AE; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOT3_RGBA: types::GLenum = 0x86AF; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BUFFER_SIZE: types::GLenum = 0x8764; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BUFFER_USAGE: types::GLenum = 0x8765; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ARRAY_BUFFER: types::GLenum = 0x8892; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const VERTEX_ARRAY_BUFFER_BINDING: types::GLenum = 0x8896; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NORMAL_ARRAY_BUFFER_BINDING: types::GLenum = 0x8897; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_ARRAY_BUFFER_BINDING: types::GLenum = 0x8898; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING: types::GLenum = 0x889A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STATIC_DRAW: types::GLenum = 0x88E4; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DYNAMIC_DRAW: types::GLenum = 0x88E8; 

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
            panic!("gles1 function was not loaded")
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[derive(Clone)]
        pub struct Gles1 {
/// Fallbacks: ActiveTextureARB
pub ActiveTexture: FnPtr,
pub AlphaFunc: FnPtr,
pub AlphaFuncx: FnPtr,
/// Fallbacks: BindBufferARB
pub BindBuffer: FnPtr,
/// Fallbacks: BindTextureEXT
pub BindTexture: FnPtr,
pub BlendFunc: FnPtr,
/// Fallbacks: BufferDataARB
pub BufferData: FnPtr,
/// Fallbacks: BufferSubDataARB
pub BufferSubData: FnPtr,
pub Clear: FnPtr,
pub ClearColor: FnPtr,
pub ClearColorx: FnPtr,
/// Fallbacks: ClearDepthfOES
pub ClearDepthf: FnPtr,
pub ClearDepthx: FnPtr,
pub ClearStencil: FnPtr,
/// Fallbacks: ClientActiveTextureARB
pub ClientActiveTexture: FnPtr,
pub ClipPlanef: FnPtr,
pub ClipPlanex: FnPtr,
pub Color4f: FnPtr,
pub Color4ub: FnPtr,
pub Color4x: FnPtr,
pub ColorMask: FnPtr,
pub ColorPointer: FnPtr,
/// Fallbacks: CompressedTexImage2DARB
pub CompressedTexImage2D: FnPtr,
/// Fallbacks: CompressedTexSubImage2DARB
pub CompressedTexSubImage2D: FnPtr,
/// Fallbacks: CopyTexImage2DEXT
pub CopyTexImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage2DEXT
pub CopyTexSubImage2D: FnPtr,
pub CullFace: FnPtr,
/// Fallbacks: DeleteBuffersARB
pub DeleteBuffers: FnPtr,
pub DeleteTextures: FnPtr,
pub DepthFunc: FnPtr,
pub DepthMask: FnPtr,
/// Fallbacks: DepthRangefOES
pub DepthRangef: FnPtr,
pub DepthRangex: FnPtr,
pub Disable: FnPtr,
pub DisableClientState: FnPtr,
/// Fallbacks: DrawArraysEXT
pub DrawArrays: FnPtr,
pub DrawElements: FnPtr,
pub Enable: FnPtr,
pub EnableClientState: FnPtr,
pub Finish: FnPtr,
pub Flush: FnPtr,
pub Fogf: FnPtr,
pub Fogfv: FnPtr,
pub Fogx: FnPtr,
pub Fogxv: FnPtr,
pub FrontFace: FnPtr,
pub Frustumf: FnPtr,
pub Frustumx: FnPtr,
/// Fallbacks: GenBuffersARB
pub GenBuffers: FnPtr,
pub GenTextures: FnPtr,
pub GetBooleanv: FnPtr,
/// Fallbacks: GetBufferParameterivARB
pub GetBufferParameteriv: FnPtr,
pub GetClipPlanef: FnPtr,
pub GetClipPlanex: FnPtr,
pub GetError: FnPtr,
pub GetFixedv: FnPtr,
pub GetFloatv: FnPtr,
pub GetIntegerv: FnPtr,
pub GetLightfv: FnPtr,
pub GetLightxv: FnPtr,
pub GetMaterialfv: FnPtr,
pub GetMaterialxv: FnPtr,
/// Fallbacks: GetPointervEXT, GetPointervKHR
pub GetPointerv: FnPtr,
pub GetString: FnPtr,
pub GetTexEnvfv: FnPtr,
pub GetTexEnviv: FnPtr,
pub GetTexEnvxv: FnPtr,
pub GetTexParameterfv: FnPtr,
pub GetTexParameteriv: FnPtr,
pub GetTexParameterxv: FnPtr,
pub Hint: FnPtr,
/// Fallbacks: IsBufferARB
pub IsBuffer: FnPtr,
pub IsEnabled: FnPtr,
pub IsTexture: FnPtr,
pub LightModelf: FnPtr,
pub LightModelfv: FnPtr,
pub LightModelx: FnPtr,
pub LightModelxv: FnPtr,
pub Lightf: FnPtr,
pub Lightfv: FnPtr,
pub Lightx: FnPtr,
pub Lightxv: FnPtr,
pub LineWidth: FnPtr,
pub LineWidthx: FnPtr,
pub LoadIdentity: FnPtr,
pub LoadMatrixf: FnPtr,
pub LoadMatrixx: FnPtr,
pub LogicOp: FnPtr,
pub Materialf: FnPtr,
pub Materialfv: FnPtr,
pub Materialx: FnPtr,
pub Materialxv: FnPtr,
pub MatrixMode: FnPtr,
pub MultMatrixf: FnPtr,
pub MultMatrixx: FnPtr,
/// Fallbacks: MultiTexCoord4fARB
pub MultiTexCoord4f: FnPtr,
pub MultiTexCoord4x: FnPtr,
pub Normal3f: FnPtr,
pub Normal3x: FnPtr,
pub NormalPointer: FnPtr,
pub Orthof: FnPtr,
pub Orthox: FnPtr,
pub PixelStorei: FnPtr,
/// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
pub PointParameterf: FnPtr,
/// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
pub PointParameterfv: FnPtr,
pub PointParameterx: FnPtr,
pub PointParameterxv: FnPtr,
pub PointSize: FnPtr,
pub PointSizex: FnPtr,
pub PolygonOffset: FnPtr,
pub PolygonOffsetx: FnPtr,
pub PopMatrix: FnPtr,
pub PushMatrix: FnPtr,
pub ReadPixels: FnPtr,
pub Rotatef: FnPtr,
pub Rotatex: FnPtr,
/// Fallbacks: SampleCoverageARB
pub SampleCoverage: FnPtr,
pub SampleCoveragex: FnPtr,
pub Scalef: FnPtr,
pub Scalex: FnPtr,
pub Scissor: FnPtr,
pub ShadeModel: FnPtr,
pub StencilFunc: FnPtr,
pub StencilMask: FnPtr,
pub StencilOp: FnPtr,
pub TexCoordPointer: FnPtr,
pub TexEnvf: FnPtr,
pub TexEnvfv: FnPtr,
pub TexEnvi: FnPtr,
pub TexEnviv: FnPtr,
pub TexEnvx: FnPtr,
pub TexEnvxv: FnPtr,
pub TexImage2D: FnPtr,
pub TexParameterf: FnPtr,
pub TexParameterfv: FnPtr,
pub TexParameteri: FnPtr,
pub TexParameteriv: FnPtr,
pub TexParameterx: FnPtr,
pub TexParameterxv: FnPtr,
/// Fallbacks: TexSubImage2DEXT
pub TexSubImage2D: FnPtr,
pub Translatef: FnPtr,
pub Translatex: FnPtr,
pub VertexPointer: FnPtr,
pub Viewport: FnPtr,
}
impl Gles1 {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Gles1 where F: FnMut(&str) -> *const __gl_imports::libc::c_void {
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
                Gles1 {
ActiveTexture: FnPtr::new(metaloadfn("glActiveTexture", &["glActiveTextureARB"])),
AlphaFunc: FnPtr::new(metaloadfn("glAlphaFunc", &[])),
AlphaFuncx: FnPtr::new(metaloadfn("glAlphaFuncx", &[])),
BindBuffer: FnPtr::new(metaloadfn("glBindBuffer", &["glBindBufferARB"])),
BindTexture: FnPtr::new(metaloadfn("glBindTexture", &["glBindTextureEXT"])),
BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
BufferData: FnPtr::new(metaloadfn("glBufferData", &["glBufferDataARB"])),
BufferSubData: FnPtr::new(metaloadfn("glBufferSubData", &["glBufferSubDataARB"])),
Clear: FnPtr::new(metaloadfn("glClear", &[])),
ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
ClearColorx: FnPtr::new(metaloadfn("glClearColorx", &[])),
ClearDepthf: FnPtr::new(metaloadfn("glClearDepthf", &["glClearDepthfOES"])),
ClearDepthx: FnPtr::new(metaloadfn("glClearDepthx", &[])),
ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
ClientActiveTexture: FnPtr::new(metaloadfn("glClientActiveTexture", &["glClientActiveTextureARB"])),
ClipPlanef: FnPtr::new(metaloadfn("glClipPlanef", &[])),
ClipPlanex: FnPtr::new(metaloadfn("glClipPlanex", &[])),
Color4f: FnPtr::new(metaloadfn("glColor4f", &[])),
Color4ub: FnPtr::new(metaloadfn("glColor4ub", &[])),
Color4x: FnPtr::new(metaloadfn("glColor4x", &[])),
ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
ColorPointer: FnPtr::new(metaloadfn("glColorPointer", &[])),
CompressedTexImage2D: FnPtr::new(metaloadfn("glCompressedTexImage2D", &["glCompressedTexImage2DARB"])),
CompressedTexSubImage2D: FnPtr::new(metaloadfn("glCompressedTexSubImage2D", &["glCompressedTexSubImage2DARB"])),
CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &["glCopyTexImage2DEXT"])),
CopyTexSubImage2D: FnPtr::new(metaloadfn("glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"])),
CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
DeleteBuffers: FnPtr::new(metaloadfn("glDeleteBuffers", &["glDeleteBuffersARB"])),
DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
DepthRangef: FnPtr::new(metaloadfn("glDepthRangef", &["glDepthRangefOES"])),
DepthRangex: FnPtr::new(metaloadfn("glDepthRangex", &[])),
Disable: FnPtr::new(metaloadfn("glDisable", &[])),
DisableClientState: FnPtr::new(metaloadfn("glDisableClientState", &[])),
DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &["glDrawArraysEXT"])),
DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
Enable: FnPtr::new(metaloadfn("glEnable", &[])),
EnableClientState: FnPtr::new(metaloadfn("glEnableClientState", &[])),
Finish: FnPtr::new(metaloadfn("glFinish", &[])),
Flush: FnPtr::new(metaloadfn("glFlush", &[])),
Fogf: FnPtr::new(metaloadfn("glFogf", &[])),
Fogfv: FnPtr::new(metaloadfn("glFogfv", &[])),
Fogx: FnPtr::new(metaloadfn("glFogx", &[])),
Fogxv: FnPtr::new(metaloadfn("glFogxv", &[])),
FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
Frustumf: FnPtr::new(metaloadfn("glFrustumf", &[])),
Frustumx: FnPtr::new(metaloadfn("glFrustumx", &[])),
GenBuffers: FnPtr::new(metaloadfn("glGenBuffers", &["glGenBuffersARB"])),
GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
GetBufferParameteriv: FnPtr::new(metaloadfn("glGetBufferParameteriv", &["glGetBufferParameterivARB"])),
GetClipPlanef: FnPtr::new(metaloadfn("glGetClipPlanef", &[])),
GetClipPlanex: FnPtr::new(metaloadfn("glGetClipPlanex", &[])),
GetError: FnPtr::new(metaloadfn("glGetError", &[])),
GetFixedv: FnPtr::new(metaloadfn("glGetFixedv", &[])),
GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
GetLightfv: FnPtr::new(metaloadfn("glGetLightfv", &[])),
GetLightxv: FnPtr::new(metaloadfn("glGetLightxv", &[])),
GetMaterialfv: FnPtr::new(metaloadfn("glGetMaterialfv", &[])),
GetMaterialxv: FnPtr::new(metaloadfn("glGetMaterialxv", &[])),
GetPointerv: FnPtr::new(metaloadfn("glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"])),
GetString: FnPtr::new(metaloadfn("glGetString", &[])),
GetTexEnvfv: FnPtr::new(metaloadfn("glGetTexEnvfv", &[])),
GetTexEnviv: FnPtr::new(metaloadfn("glGetTexEnviv", &[])),
GetTexEnvxv: FnPtr::new(metaloadfn("glGetTexEnvxv", &[])),
GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
GetTexParameterxv: FnPtr::new(metaloadfn("glGetTexParameterxv", &[])),
Hint: FnPtr::new(metaloadfn("glHint", &[])),
IsBuffer: FnPtr::new(metaloadfn("glIsBuffer", &["glIsBufferARB"])),
IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
LightModelf: FnPtr::new(metaloadfn("glLightModelf", &[])),
LightModelfv: FnPtr::new(metaloadfn("glLightModelfv", &[])),
LightModelx: FnPtr::new(metaloadfn("glLightModelx", &[])),
LightModelxv: FnPtr::new(metaloadfn("glLightModelxv", &[])),
Lightf: FnPtr::new(metaloadfn("glLightf", &[])),
Lightfv: FnPtr::new(metaloadfn("glLightfv", &[])),
Lightx: FnPtr::new(metaloadfn("glLightx", &[])),
Lightxv: FnPtr::new(metaloadfn("glLightxv", &[])),
LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
LineWidthx: FnPtr::new(metaloadfn("glLineWidthx", &[])),
LoadIdentity: FnPtr::new(metaloadfn("glLoadIdentity", &[])),
LoadMatrixf: FnPtr::new(metaloadfn("glLoadMatrixf", &[])),
LoadMatrixx: FnPtr::new(metaloadfn("glLoadMatrixx", &[])),
LogicOp: FnPtr::new(metaloadfn("glLogicOp", &[])),
Materialf: FnPtr::new(metaloadfn("glMaterialf", &[])),
Materialfv: FnPtr::new(metaloadfn("glMaterialfv", &[])),
Materialx: FnPtr::new(metaloadfn("glMaterialx", &[])),
Materialxv: FnPtr::new(metaloadfn("glMaterialxv", &[])),
MatrixMode: FnPtr::new(metaloadfn("glMatrixMode", &[])),
MultMatrixf: FnPtr::new(metaloadfn("glMultMatrixf", &[])),
MultMatrixx: FnPtr::new(metaloadfn("glMultMatrixx", &[])),
MultiTexCoord4f: FnPtr::new(metaloadfn("glMultiTexCoord4f", &["glMultiTexCoord4fARB"])),
MultiTexCoord4x: FnPtr::new(metaloadfn("glMultiTexCoord4x", &[])),
Normal3f: FnPtr::new(metaloadfn("glNormal3f", &[])),
Normal3x: FnPtr::new(metaloadfn("glNormal3x", &[])),
NormalPointer: FnPtr::new(metaloadfn("glNormalPointer", &[])),
Orthof: FnPtr::new(metaloadfn("glOrthof", &[])),
Orthox: FnPtr::new(metaloadfn("glOrthox", &[])),
PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
PointParameterf: FnPtr::new(metaloadfn("glPointParameterf", &["glPointParameterfARB", "glPointParameterfEXT", "glPointParameterfSGIS"])),
PointParameterfv: FnPtr::new(metaloadfn("glPointParameterfv", &["glPointParameterfvARB", "glPointParameterfvEXT", "glPointParameterfvSGIS"])),
PointParameterx: FnPtr::new(metaloadfn("glPointParameterx", &[])),
PointParameterxv: FnPtr::new(metaloadfn("glPointParameterxv", &[])),
PointSize: FnPtr::new(metaloadfn("glPointSize", &[])),
PointSizex: FnPtr::new(metaloadfn("glPointSizex", &[])),
PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
PolygonOffsetx: FnPtr::new(metaloadfn("glPolygonOffsetx", &[])),
PopMatrix: FnPtr::new(metaloadfn("glPopMatrix", &[])),
PushMatrix: FnPtr::new(metaloadfn("glPushMatrix", &[])),
ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
Rotatef: FnPtr::new(metaloadfn("glRotatef", &[])),
Rotatex: FnPtr::new(metaloadfn("glRotatex", &[])),
SampleCoverage: FnPtr::new(metaloadfn("glSampleCoverage", &["glSampleCoverageARB"])),
SampleCoveragex: FnPtr::new(metaloadfn("glSampleCoveragex", &[])),
Scalef: FnPtr::new(metaloadfn("glScalef", &[])),
Scalex: FnPtr::new(metaloadfn("glScalex", &[])),
Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
ShadeModel: FnPtr::new(metaloadfn("glShadeModel", &[])),
StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
TexCoordPointer: FnPtr::new(metaloadfn("glTexCoordPointer", &[])),
TexEnvf: FnPtr::new(metaloadfn("glTexEnvf", &[])),
TexEnvfv: FnPtr::new(metaloadfn("glTexEnvfv", &[])),
TexEnvi: FnPtr::new(metaloadfn("glTexEnvi", &[])),
TexEnviv: FnPtr::new(metaloadfn("glTexEnviv", &[])),
TexEnvx: FnPtr::new(metaloadfn("glTexEnvx", &[])),
TexEnvxv: FnPtr::new(metaloadfn("glTexEnvxv", &[])),
TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
TexParameterx: FnPtr::new(metaloadfn("glTexParameterx", &[])),
TexParameterxv: FnPtr::new(metaloadfn("glTexParameterxv", &[])),
TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &["glTexSubImage2DEXT"])),
Translatef: FnPtr::new(metaloadfn("glTranslatef", &[])),
Translatex: FnPtr::new(metaloadfn("glTranslatex", &[])),
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
        pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) -> Gles1 {
            Gles1::load_with(|name| loader.get_proc_addr(name))
        }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ActiveTexture(&self, texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ActiveTexture.f)(texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn AlphaFunc(&self, func: types::GLenum, ref_: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.AlphaFunc.f)(func, ref_) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn AlphaFuncx(&self, func: types::GLenum, ref_: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(self.AlphaFuncx.f)(func, ref_) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BindBuffer(&self, target: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindBuffer.f)(target, buffer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BindTexture(&self, target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindTexture.f)(target, texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BlendFunc(&self, sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BufferData(&self, target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::libc::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::libc::c_void, types::GLenum) -> ()>(self.BufferData.f)(target, size, data, usage) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn BufferSubData(&self, target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const __gl_imports::libc::c_void) -> ()>(self.BufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Clear(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.Clear.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearColorx(&self, red: types::GLfixed, green: types::GLfixed, blue: types::GLfixed, alpha: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.ClearColorx.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearDepthf(&self, d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ClearDepthf.f)(d) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearDepthx(&self, depth: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(self.ClearDepthx.f)(depth) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClearStencil(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ClearStencil.f)(s) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClientActiveTexture(&self, texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ClientActiveTexture.f)(texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClipPlanef(&self, p: types::GLenum, eqn: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.ClipPlanef.f)(p, eqn) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ClipPlanex(&self, plane: types::GLenum, equation: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(self.ClipPlanex.f)(plane, equation) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4f(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Color4f.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4ub(&self, red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte, alpha: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(self.Color4ub.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Color4x(&self, red: types::GLfixed, green: types::GLfixed, blue: types::GLfixed, alpha: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Color4x.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ColorMask(&self, red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(self.ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ColorPointer(&self, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.ColorPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CompressedTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CullFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.CullFace.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DeleteBuffers(&self, n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteBuffers.f)(n, buffers) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteTextures.f)(n, textures) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthFunc(&self, func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DepthFunc.f)(func) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthMask(&self, flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(self.DepthMask.f)(flag) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthRangef(&self, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.DepthRangef.f)(n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DepthRangex(&self, n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed) -> ()>(self.DepthRangex.f)(n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Disable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Disable.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DisableClientState(&self, array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DisableClientState.f)(array) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawArrays(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(self.DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DrawElements(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Enable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Enable.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn EnableClientState(&self, array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.EnableClientState.f)(array) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Finish(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Finish.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Flush(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Flush.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.Fogf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.Fogfv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogx(&self, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(self.Fogx.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Fogxv(&self, pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(self.Fogxv.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn FrontFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.FrontFace.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Frustumf(&self, l: types::GLfloat, r: types::GLfloat, b: types::GLfloat, t: types::GLfloat, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Frustumf.f)(l, r, b, t, n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Frustumx(&self, l: types::GLfixed, r: types::GLfixed, b: types::GLfixed, t: types::GLfixed, n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Frustumx.f)(l, r, b, t, n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GenBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenBuffers.f)(n, buffers) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenTextures.f)(n, textures) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetBooleanv(&self, pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(self.GetBooleanv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetBufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetBufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetClipPlanef(&self, plane: types::GLenum, equation: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetClipPlanef.f)(plane, equation) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetClipPlanex(&self, plane: types::GLenum, equation: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfixed) -> ()>(self.GetClipPlanex.f)(plane, equation) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetError(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetError.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetFixedv(&self, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfixed) -> ()>(self.GetFixedv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetFloatv(&self, pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetFloatv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetIntegerv(&self, pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(self.GetIntegerv.f)(pname, data) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetLightfv(&self, light: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetLightfv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetLightxv(&self, light: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(self.GetLightxv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMaterialfv(&self, face: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetMaterialfv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetMaterialxv(&self, face: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(self.GetMaterialxv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPointerv(&self, pname: types::GLenum, params: *const *mut __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::libc::c_void) -> ()>(self.GetPointerv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetString(&self, name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(self.GetString.f)(name) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexEnvfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexEnviv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexEnviv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexEnvxv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(self.GetTexEnvxv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetTexParameterxv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(self.GetTexParameterxv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Hint(&self, target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.Hint.f)(target, mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsBuffer.f)(buffer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsEnabled(&self, cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(self.IsEnabled.f)(cap) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsTexture.f)(texture) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.LightModelf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.LightModelfv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelx(&self, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(self.LightModelx.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LightModelxv(&self, pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(self.LightModelxv.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightf(&self, light: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.Lightf.f)(light, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightfv(&self, light: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.Lightfv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightx(&self, light: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(self.Lightx.f)(light, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Lightxv(&self, light: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(self.Lightxv.f)(light, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.LineWidth.f)(width) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LineWidthx(&self, width: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(self.LineWidthx.f)(width) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadIdentity(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.LoadIdentity.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadMatrixf(&self, m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.LoadMatrixf.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LoadMatrixx(&self, m: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfixed) -> ()>(self.LoadMatrixx.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn LogicOp(&self, opcode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.LogicOp.f)(opcode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialf(&self, face: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.Materialf.f)(face, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialfv(&self, face: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.Materialfv.f)(face, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialx(&self, face: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(self.Materialx.f)(face, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Materialxv(&self, face: types::GLenum, pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(self.Materialxv.f)(face, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MatrixMode(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.MatrixMode.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultMatrixf(&self, m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(self.MultMatrixf.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultMatrixx(&self, m: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfixed) -> ()>(self.MultMatrixx.f)(m) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultiTexCoord4f(&self, target: types::GLenum, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.MultiTexCoord4f.f)(target, s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MultiTexCoord4x(&self, texture: types::GLenum, s: types::GLfixed, t: types::GLfixed, r: types::GLfixed, q: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.MultiTexCoord4x.f)(texture, s, t, r, q) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3f(&self, nx: types::GLfloat, ny: types::GLfloat, nz: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Normal3f.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Normal3x(&self, nx: types::GLfixed, ny: types::GLfixed, nz: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Normal3x.f)(nx, ny, nz) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn NormalPointer(&self, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.NormalPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Orthof(&self, l: types::GLfloat, r: types::GLfloat, b: types::GLfloat, t: types::GLfloat, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Orthof.f)(l, r, b, t, n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Orthox(&self, l: types::GLfixed, r: types::GLfixed, b: types::GLfixed, t: types::GLfixed, n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Orthox.f)(l, r, b, t, n, f) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PixelStorei(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PixelStorei.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointParameterf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.PointParameterf.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointParameterfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.PointParameterfv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointParameterx(&self, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(self.PointParameterx.f)(pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointParameterxv(&self, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(self.PointParameterxv.f)(pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointSize(&self, size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.PointSize.f)(size) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PointSizex(&self, size: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(self.PointSizex.f)(size) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.PolygonOffset.f)(factor, units) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PolygonOffsetx(&self, factor: types::GLfixed, units: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed) -> ()>(self.PolygonOffsetx.f)(factor, units) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PopMatrix(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopMatrix.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn PushMatrix(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PushMatrix.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ReadPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::libc::c_void) -> ()>(self.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rotatef(&self, angle: types::GLfloat, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Rotatef.f)(angle, x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Rotatex(&self, angle: types::GLfixed, x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Rotatex.f)(angle, x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SampleCoverage(&self, value: types::GLfloat, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(self.SampleCoverage.f)(value, invert) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SampleCoveragex(&self, value: types::GLclampx, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLclampx, types::GLboolean) -> ()>(self.SampleCoveragex.f)(value, invert) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scalef(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Scalef.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scalex(&self, x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Scalex.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Scissor(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Scissor.f)(x, y, width, height) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ShadeModel(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ShadeModel.f)(mode) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilFunc(&self, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.StencilMask.f)(mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn StencilOp(&self, fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOp.f)(fail, zfail, zpass) }
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
            #[inline] pub unsafe fn TexEnvx(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(self.TexEnvx.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexEnvxv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(self.TexEnvxv.f)(target, pname, params) }
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
            #[inline] pub unsafe fn TexParameterx(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(self.TexParameterx.f)(target, pname, param) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexParameterxv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(self.TexParameterxv.f)(target, pname, params) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn TexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::libc::c_void) -> ()>(self.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Translatef(&self, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Translatef.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Translatex(&self, x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(self.Translatex.f)(x, y, z) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn VertexPointer(&self, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::libc::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::libc::c_void) -> ()>(self.VertexPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn Viewport(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Viewport.f)(x, y, width, height) }
}

        unsafe impl __gl_imports::Send for Gles1 {}
