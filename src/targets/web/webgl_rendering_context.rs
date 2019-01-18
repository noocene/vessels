#![allow(
    dead_code,
    unused_parens,
    unused_imports,
    clippy::unreadable_literal,
    clippy::too_many_arguments
)]

// Registry { types: {"Float32List": Typedef(Type { kind: Union([Type { kind: TypedArray(F32), optional: false }, Type { kind: Sequence(Type { kind: Named("GLfloat"), optional: false }), optional: false }]), optional: false }), "GLContext": Interface(Interface { inherits: None, mixins: {"WebGL2RenderingContextBase", "WebGLRenderingContextBase"}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "GLbitfield": Typedef(Type { kind: Primitive(U32), optional: false }), "GLboolean": Typedef(Type { kind: Primitive(Bool), optional: false }), "GLbyte": Typedef(Type { kind: Primitive(I8), optional: false }), "GLclampf": Typedef(Type { kind: Primitive(F32), optional: false }), "GLenum": Typedef(Type { kind: Primitive(U32), optional: false }), "GLfloat": Typedef(Type { kind: Primitive(F32), optional: false }), "GLint": Typedef(Type { kind: Primitive(I32), optional: false }), "GLint64": Typedef(Type { kind: Primitive(I64), optional: false }), "GLintptr": Typedef(Type { kind: Primitive(I64), optional: false }), "GLshort": Typedef(Type { kind: Primitive(I16), optional: false }), "GLsizei": Typedef(Type { kind: Primitive(I32), optional: false }), "GLsizeiptr": Typedef(Type { kind: Primitive(I64), optional: false }), "GLubyte": Typedef(Type { kind: Primitive(U8), optional: false }), "GLuint": Typedef(Type { kind: Primitive(U32), optional: false }), "GLuint64": Typedef(Type { kind: Primitive(U64), optional: false }), "GLushort": Typedef(Type { kind: Primitive(U16), optional: false }), "Int32List": Typedef(Type { kind: Union([Type { kind: TypedArray(I32), optional: false }, Type { kind: Sequence(Type { kind: Named("GLint"), optional: false }), optional: false }]), optional: false }), "TexImageSource": Typedef(Type { kind: Any, optional: false }), "Uint32List": Typedef(Type { kind: Union([Type { kind: TypedArray(U32), optional: false }, Type { kind: Sequence(Type { kind: Named("GLuint"), optional: false }), optional: false }]), optional: false }), "WebGL2RenderingContext": Interface(Interface { inherits: None, mixins: {"WebGL2RenderingContextBase", "WebGLRenderingContextBase"}, members: {}, is_hidden: false, has_class: true, rendering_context: Some("webgl2"), doc_comment: "" }), "WebGL2RenderingContextBase": Mixin(Mixin { members: {"ACTIVE_UNIFORM_BLOCKS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35382" })], "ALREADY_SIGNALED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37146" })], "ANY_SAMPLES_PASSED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35887" })], "ANY_SAMPLES_PASSED_CONSERVATIVE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36202" })], "COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6144" })], "COLOR_ATTACHMENT1": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36065" })], "COLOR_ATTACHMENT10": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36074" })], "COLOR_ATTACHMENT11": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36075" })], "COLOR_ATTACHMENT12": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36076" })], "COLOR_ATTACHMENT13": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36077" })], "COLOR_ATTACHMENT14": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36078" })], "COLOR_ATTACHMENT15": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36079" })], "COLOR_ATTACHMENT2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36066" })], "COLOR_ATTACHMENT3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36067" })], "COLOR_ATTACHMENT4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36068" })], "COLOR_ATTACHMENT5": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36069" })], "COLOR_ATTACHMENT6": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36070" })], "COLOR_ATTACHMENT7": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36071" })], "COLOR_ATTACHMENT8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36072" })], "COLOR_ATTACHMENT9": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36073" })], "COMPARE_REF_TO_TEXTURE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34894" })], "CONDITION_SATISFIED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37148" })], "COPY_READ_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36662" })], "COPY_READ_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36662" })], "COPY_WRITE_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36663" })], "COPY_WRITE_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36663" })], "CURRENT_QUERY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34917" })], "DEPTH": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6145" })], "DEPTH24_STENCIL8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35056" })], "DEPTH32F_STENCIL8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36013" })], "DEPTH_COMPONENT24": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33190" })], "DEPTH_COMPONENT32F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36012" })], "DEPTH_STENCIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34041" })], "DEPTH_STENCIL_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33306" })], "DRAW_BUFFER0": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34853" })], "DRAW_BUFFER1": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34854" })], "DRAW_BUFFER10": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34863" })], "DRAW_BUFFER11": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34864" })], "DRAW_BUFFER12": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34865" })], "DRAW_BUFFER13": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34866" })], "DRAW_BUFFER14": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34867" })], "DRAW_BUFFER15": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34868" })], "DRAW_BUFFER2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34855" })], "DRAW_BUFFER3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34856" })], "DRAW_BUFFER4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34857" })], "DRAW_BUFFER5": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34858" })], "DRAW_BUFFER6": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34859" })], "DRAW_BUFFER7": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34860" })], "DRAW_BUFFER8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34861" })], "DRAW_BUFFER9": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34862" })], "DRAW_FRAMEBUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36009" })], "DRAW_FRAMEBUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36006" })], "DYNAMIC_COPY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35050" })], "DYNAMIC_READ": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35049" })], "FLOAT_32_UNSIGNED_INT_24_8_REV": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36269" })], "FLOAT_MAT2x3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35685" })], "FLOAT_MAT2x4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35686" })], "FLOAT_MAT3x2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35687" })], "FLOAT_MAT3x4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35688" })], "FLOAT_MAT4x2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35689" })], "FLOAT_MAT4x3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35690" })], "FRAGMENT_SHADER_DERIVATIVE_HINT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35723" })], "FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33301" })], "FRAMEBUFFER_ATTACHMENT_BLUE_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33300" })], "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33296" })], "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33297" })], "FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33302" })], "FRAMEBUFFER_ATTACHMENT_GREEN_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33299" })], "FRAMEBUFFER_ATTACHMENT_RED_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33298" })], "FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33303" })], "FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36052" })], "FRAMEBUFFER_DEFAULT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33304" })], "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36182" })], "HALF_FLOAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5131" })], "INTERLEAVED_ATTRIBS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35980" })], "INT_2_10_10_10_REV": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36255" })], "INT_SAMPLER_2D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36298" })], "INT_SAMPLER_2D_ARRAY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36303" })], "INT_SAMPLER_3D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36299" })], "INT_SAMPLER_CUBE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36300" })], "INVALID_INDEX": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "4294967295" })], "MAX": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32776" })], "MAX_3D_TEXTURE_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32883" })], "MAX_ARRAY_TEXTURE_LAYERS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35071" })], "MAX_CLIENT_WAIT_TIMEOUT_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37447" })], "MAX_COLOR_ATTACHMENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36063" })], "MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35379" })], "MAX_COMBINED_UNIFORM_BLOCKS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35374" })], "MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35377" })], "MAX_DRAW_BUFFERS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34852" })], "MAX_ELEMENTS_INDICES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33001" })], "MAX_ELEMENTS_VERTICES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33000" })], "MAX_ELEMENT_INDEX": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36203" })], "MAX_FRAGMENT_INPUT_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37157" })], "MAX_FRAGMENT_UNIFORM_BLOCKS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35373" })], "MAX_FRAGMENT_UNIFORM_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35657" })], "MAX_PROGRAM_TEXEL_OFFSET": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35077" })], "MAX_SAMPLES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36183" })], "MAX_SERVER_WAIT_TIMEOUT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37137" })], "MAX_TEXTURE_LOD_BIAS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34045" })], "MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35978" })], "MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35979" })], "MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35968" })], "MAX_UNIFORM_BLOCK_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35376" })], "MAX_UNIFORM_BUFFER_BINDINGS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35375" })], "MAX_VARYING_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35659" })], "MAX_VERTEX_OUTPUT_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37154" })], "MAX_VERTEX_UNIFORM_BLOCKS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35371" })], "MAX_VERTEX_UNIFORM_COMPONENTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35658" })], "MIN": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32775" })], "MIN_PROGRAM_TEXEL_OFFSET": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35076" })], "OBJECT_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37138" })], "PACK_ROW_LENGTH": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3330" })], "PACK_SKIP_PIXELS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3332" })], "PACK_SKIP_ROWS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3331" })], "PIXEL_PACK_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35051" })], "PIXEL_PACK_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35053" })], "PIXEL_UNPACK_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35052" })], "PIXEL_UNPACK_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35055" })], "QUERY_RESULT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34918" })], "QUERY_RESULT_AVAILABLE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34919" })], "R11F_G11F_B10F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35898" })], "R16F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33325" })], "R16I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33331" })], "R16UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33332" })], "R32F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33326" })], "R32I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33333" })], "R32UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33334" })], "R8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33321" })], "R8I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33329" })], "R8UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33330" })], "R8_SNORM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36756" })], "RASTERIZER_DISCARD": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35977" })], "READ_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3074" })], "READ_FRAMEBUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36008" })], "READ_FRAMEBUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36010" })], "RED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6403" })], "RED_INTEGER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36244" })], "RENDERBUFFER_SAMPLES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36011" })], "RG": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33319" })], "RG16F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33327" })], "RG16I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33337" })], "RG16UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33338" })], "RG32F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33328" })], "RG32I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33339" })], "RG32UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33340" })], "RG8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33323" })], "RG8I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33335" })], "RG8UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33336" })], "RG8_SNORM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36757" })], "RGB10_A2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32857" })], "RGB10_A2UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36975" })], "RGB16F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34843" })], "RGB16I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36233" })], "RGB16UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36215" })], "RGB32F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34837" })], "RGB32I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36227" })], "RGB32UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36209" })], "RGB8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32849" })], "RGB8I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36239" })], "RGB8UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36221" })], "RGB8_SNORM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36758" })], "RGB9_E5": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35901" })], "RGBA16F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34842" })], "RGBA16I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36232" })], "RGBA16UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36214" })], "RGBA32F": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34836" })], "RGBA32I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36226" })], "RGBA32UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36208" })], "RGBA8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32856" })], "RGBA8I": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36238" })], "RGBA8UI": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36220" })], "RGBA8_SNORM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36759" })], "RGBA_INTEGER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36249" })], "RGB_INTEGER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36248" })], "RG_INTEGER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33320" })], "SAMPLER_2D_ARRAY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36289" })], "SAMPLER_2D_ARRAY_SHADOW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36292" })], "SAMPLER_2D_SHADOW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35682" })], "SAMPLER_3D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35679" })], "SAMPLER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35097" })], "SAMPLER_CUBE_SHADOW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36293" })], "SEPARATE_ATTRIBS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35981" })], "SIGNALED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37145" })], "SIGNED_NORMALIZED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36764" })], "SRGB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35904" })], "SRGB8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35905" })], "SRGB8_ALPHA8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35907" })], "STATIC_COPY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35046" })], "STATIC_READ": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35045" })], "STENCIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6146" })], "STREAM_COPY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35042" })], "STREAM_READ": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35041" })], "SYNC_CONDITION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37139" })], "SYNC_FENCE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37142" })], "SYNC_FLAGS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37141" })], "SYNC_FLUSH_COMMANDS_BIT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1" })], "SYNC_GPU_COMMANDS_COMPLETE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37143" })], "SYNC_STATUS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37140" })], "TEXTURE_2D_ARRAY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35866" })], "TEXTURE_3D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32879" })], "TEXTURE_BASE_LEVEL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33084" })], "TEXTURE_BINDING_2D_ARRAY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35869" })], "TEXTURE_BINDING_3D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32874" })], "TEXTURE_COMPARE_FUNC": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34893" })], "TEXTURE_COMPARE_MODE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34892" })], "TEXTURE_IMMUTABLE_FORMAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37167" })], "TEXTURE_IMMUTABLE_LEVELS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33503" })], "TEXTURE_MAX_LEVEL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33085" })], "TEXTURE_MAX_LOD": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33083" })], "TEXTURE_MIN_LOD": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33082" })], "TEXTURE_WRAP_R": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32882" })], "TIMEOUT_EXPIRED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37147" })], "TIMEOUT_IGNORED": [Const(Const { type_: Type { kind: Named("GLint64"), optional: false }, value: "-1" })], "TRANSFORM_FEEDBACK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36386" })], "TRANSFORM_FEEDBACK_ACTIVE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36388" })], "TRANSFORM_FEEDBACK_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36389" })], "TRANSFORM_FEEDBACK_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35982" })], "TRANSFORM_FEEDBACK_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35983" })], "TRANSFORM_FEEDBACK_BUFFER_MODE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35967" })], "TRANSFORM_FEEDBACK_BUFFER_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35973" })], "TRANSFORM_FEEDBACK_BUFFER_START": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35972" })], "TRANSFORM_FEEDBACK_PAUSED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36387" })], "TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35976" })], "TRANSFORM_FEEDBACK_VARYINGS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35971" })], "UNIFORM_ARRAY_STRIDE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35388" })], "UNIFORM_BLOCK_ACTIVE_UNIFORMS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35394" })], "UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35395" })], "UNIFORM_BLOCK_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35391" })], "UNIFORM_BLOCK_DATA_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35392" })], "UNIFORM_BLOCK_INDEX": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35386" })], "UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35398" })], "UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35396" })], "UNIFORM_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35345" })], "UNIFORM_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35368" })], "UNIFORM_BUFFER_OFFSET_ALIGNMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35380" })], "UNIFORM_BUFFER_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35370" })], "UNIFORM_BUFFER_START": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35369" })], "UNIFORM_IS_ROW_MAJOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35390" })], "UNIFORM_MATRIX_STRIDE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35389" })], "UNIFORM_OFFSET": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35387" })], "UNIFORM_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35384" })], "UNIFORM_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35383" })], "UNPACK_IMAGE_HEIGHT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32878" })], "UNPACK_ROW_LENGTH": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3314" })], "UNPACK_SKIP_IMAGES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32877" })], "UNPACK_SKIP_PIXELS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3316" })], "UNPACK_SKIP_ROWS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3315" })], "UNSIGNALED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37144" })], "UNSIGNED_INT_10F_11F_11F_REV": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35899" })], "UNSIGNED_INT_24_8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34042" })], "UNSIGNED_INT_2_10_10_10_REV": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33640" })], "UNSIGNED_INT_5_9_9_9_REV": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35902" })], "UNSIGNED_INT_SAMPLER_2D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36306" })], "UNSIGNED_INT_SAMPLER_2D_ARRAY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36311" })], "UNSIGNED_INT_SAMPLER_3D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36307" })], "UNSIGNED_INT_SAMPLER_CUBE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36308" })], "UNSIGNED_INT_VEC2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36294" })], "UNSIGNED_INT_VEC3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36295" })], "UNSIGNED_INT_VEC4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36296" })], "UNSIGNED_NORMALIZED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35863" })], "VERTEX_ARRAY_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34229" })], "VERTEX_ATTRIB_ARRAY_DIVISOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35070" })], "VERTEX_ATTRIB_ARRAY_INTEGER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35069" })], "WAIT_FAILED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37149" })], "beginQuery": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "query", optional: false, type_: Type { kind: Named("WebGLQuery"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "beginTransformFeedback": [Operation(Operation { args: [Argument { name: "primitiveMode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bindBufferBase": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "buffer", optional: false, type_: Type { kind: Named("WebGLBuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindBufferRange": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "buffer", optional: false, type_: Type { kind: Named("WebGLBuffer"), optional: true }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLsizeiptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bindSampler": [Operation(Operation { args: [Argument { name: "unit", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindTransformFeedback": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "tf", optional: false, type_: Type { kind: Named("WebGLTransformFeedback"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindVertexArray": [Operation(Operation { args: [Argument { name: "array", optional: false, type_: Type { kind: Named("WebGLVertexArrayObject"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "blitFramebuffer": [Operation(Operation { args: [Argument { name: "srcX0", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "srcY0", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "srcX1", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "srcY1", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "dstX0", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "dstY0", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "dstX1", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "dstY1", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "mask", optional: false, type_: Type { kind: Named("GLbitfield"), optional: false }, variadic: false }, Argument { name: "filter", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bufferData": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLsizeiptr"), optional: false }, variadic: false }, Argument { name: "usage", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: BufferSource, optional: true }, variadic: false }, Argument { name: "usage", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "usage", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "length", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bufferSubData": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstByteOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: BufferSource, optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstByteOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "length", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearBufferfi": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "drawbuffer", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "stencil", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearBufferfv": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "drawbuffer", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearBufferiv": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "drawbuffer", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearBufferuiv": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "drawbuffer", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clientWaitSync": [Operation(Operation { args: [Argument { name: "sync", optional: false, type_: Type { kind: Named("WebGLSync"), optional: false }, variadic: false }, Argument { name: "flags", optional: false, type_: Type { kind: Named("GLbitfield"), optional: false }, variadic: false }, Argument { name: "timeout", optional: false, type_: Type { kind: Named("GLuint64"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLenum"), optional: false }), doc_comment: "" })], "compressedTexImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "imageSize", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLengthOverride", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compressedTexImage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "imageSize", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLengthOverride", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compressedTexSubImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "imageSize", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLengthOverride", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compressedTexSubImage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "imageSize", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLengthOverride", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "copyBufferSubData": [Operation(Operation { args: [Argument { name: "readTarget", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "writeTarget", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "readOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "writeOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLsizeiptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "copyTexSubImage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "createQuery": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLQuery"), optional: true }), doc_comment: "" })], "createSampler": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLSampler"), optional: true }), doc_comment: "" })], "createTransformFeedback": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLTransformFeedback"), optional: true }), doc_comment: "" })], "createVertexArray": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLVertexArrayObject"), optional: true }), doc_comment: "" })], "deleteQuery": [Operation(Operation { args: [Argument { name: "query", optional: false, type_: Type { kind: Named("WebGLQuery"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteSampler": [Operation(Operation { args: [Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteSync": [Operation(Operation { args: [Argument { name: "sync", optional: false, type_: Type { kind: Named("WebGLSync"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteTransformFeedback": [Operation(Operation { args: [Argument { name: "tf", optional: false, type_: Type { kind: Named("WebGLTransformFeedback"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteVertexArray": [Operation(Operation { args: [Argument { name: "vertexArray", optional: false, type_: Type { kind: Named("WebGLVertexArrayObject"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "drawArraysInstanced": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "first", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "count", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "instanceCount", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawBuffers": [Operation(Operation { args: [Argument { name: "buffers", optional: false, type_: Type { kind: Sequence(Type { kind: Named("GLenum"), optional: false }), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawElementsInstanced": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "count", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "instanceCount", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawRangeElements": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "start", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "end", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "count", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "endQuery": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "endTransformFeedback": [Operation(Operation { args: [], return_type: None, doc_comment: "" })], "fenceSync": [Operation(Operation { args: [Argument { name: "condition", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "flags", optional: false, type_: Type { kind: Named("GLbitfield"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLSync"), optional: true }), doc_comment: "" })], "framebufferTextureLayer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachment", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "texture", optional: false, type_: Type { kind: Named("WebGLTexture"), optional: true }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "layer", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "getActiveUniformBlockName": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformBlockIndex", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: Some(Type { kind: String, optional: true }), doc_comment: "" })], "getActiveUniformBlockParameter": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformBlockIndex", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getActiveUniforms": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformIndices", optional: false, type_: Type { kind: Sequence(Type { kind: Named("GLuint"), optional: false }), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getBufferSubData": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcByteOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "dstBuffer", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "dstOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "length", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "getFragDataLocation": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "name", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLint"), optional: false }), doc_comment: "" })], "getIndexedParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getInternalformatParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getQuery": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLQuery"), optional: true }), doc_comment: "" })], "getQueryParameter": [Operation(Operation { args: [Argument { name: "query", optional: false, type_: Type { kind: Named("WebGLQuery"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getSamplerParameter": [Operation(Operation { args: [Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getSyncParameter": [Operation(Operation { args: [Argument { name: "sync", optional: false, type_: Type { kind: Named("WebGLSync"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getTransformFeedbackVarying": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLActiveInfo"), optional: true }), doc_comment: "" })], "getUniformBlockIndex": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformBlockName", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLuint"), optional: false }), doc_comment: "" })], "getUniformIndices": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformNames", optional: false, type_: Type { kind: Sequence(Type { kind: String, optional: false }), optional: false }, variadic: false }], return_type: Some(Type { kind: Sequence(Type { kind: Named("GLuint"), optional: false }), optional: true }), doc_comment: "" })], "invalidateFramebuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachments", optional: false, type_: Type { kind: Sequence(Type { kind: Named("GLenum"), optional: false }), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "invalidateSubFramebuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachments", optional: false, type_: Type { kind: Sequence(Type { kind: Named("GLenum"), optional: false }), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "isQuery": [Operation(Operation { args: [Argument { name: "query", optional: false, type_: Type { kind: Named("WebGLQuery"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isSampler": [Operation(Operation { args: [Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isSync": [Operation(Operation { args: [Argument { name: "sync", optional: false, type_: Type { kind: Named("WebGLSync"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isTransformFeedback": [Operation(Operation { args: [Argument { name: "tf", optional: false, type_: Type { kind: Named("WebGLTransformFeedback"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isVertexArray": [Operation(Operation { args: [Argument { name: "vertexArray", optional: false, type_: Type { kind: Named("WebGLVertexArrayObject"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "pauseTransformFeedback": [Operation(Operation { args: [], return_type: None, doc_comment: "" })], "readBuffer": [Operation(Operation { args: [Argument { name: "src", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "readPixels": [Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstData", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "dstOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "renderbufferStorageMultisample": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "samples", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "resumeTransformFeedback": [Operation(Operation { args: [], return_type: None, doc_comment: "" })], "samplerParameterf": [Operation(Operation { args: [Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "param", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "samplerParameteri": [Operation(Operation { args: [Argument { name: "sampler", optional: false, type_: Type { kind: Named("WebGLSampler"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "param", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pixels", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pboOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texImage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pboOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texStorage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "levels", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texStorage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "levels", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texSubImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pixels", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pboOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }, Argument { name: "srcOffset", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texSubImage3D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pboOffset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "zoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "depth", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcData", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "transformFeedbackVaryings": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "varyings", optional: false, type_: Type { kind: Sequence(Type { kind: String, optional: false }), optional: false }, variadic: false }, Argument { name: "bufferMode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1ui": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v0", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1uiv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2ui": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v0", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v1", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2uiv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3ui": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v0", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v1", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v2", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3uiv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4ui": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v0", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v1", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v2", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "v3", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4uiv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformBlockBinding": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "uniformBlockIndex", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "uniformBlockBinding", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix2x3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix2x4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix3x2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix3x4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix4x2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix4x3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }, Argument { name: "srcOffset", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "srcLength", optional: true, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribDivisor": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "divisor", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribI4i": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "w", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribI4iv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribI4ui": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "w", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribI4uiv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Uint32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribIPointer": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "stride", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "waitSync": [Operation(Operation { args: [Argument { name: "sync", optional: false, type_: Type { kind: Named("WebGLSync"), optional: false }, variadic: false }, Argument { name: "flags", optional: false, type_: Type { kind: Named("GLbitfield"), optional: false }, variadic: false }, Argument { name: "timeout", optional: false, type_: Type { kind: Named("GLint64"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })]} }), "WebGLActiveInfo": Interface(Interface { inherits: None, mixins: {}, members: {"name": [Attribute(Attribute { type_: Type { kind: String, optional: false }, setter: false, getter: true })], "size": [Attribute(Attribute { type_: Type { kind: Named("GLint"), optional: false }, setter: false, getter: true })], "type": [Attribute(Attribute { type_: Type { kind: Named("GLenum"), optional: false }, setter: false, getter: true })]}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLBuffer": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLContextAttributes": Dictionary(Dictionary { inherits: None, fields: {"alpha": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "antialias": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "depth": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "failIfMajorPerformanceCaveat": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "powerPreference": Field { type_: Type { kind: Named("WebGLPowerPreference"), optional: false } }, "premultipliedAlpha": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "preserveDrawingBuffer": Field { type_: Type { kind: Named("GLboolean"), optional: false } }, "stencil": Field { type_: Type { kind: Named("GLboolean"), optional: false } }}, is_hidden: false }), "WebGLContextEvent": Interface(Interface { inherits: Some("Event"), mixins: {}, members: {"statusMessage": [Attribute(Attribute { type_: Type { kind: String, optional: false }, setter: false, getter: true })]}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLContextEventInit": Dictionary(Dictionary { inherits: Some("EventInit"), fields: {"statusMessage": Field { type_: Type { kind: String, optional: false } }}, is_hidden: true }), "WebGLFramebuffer": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLObject": Interface(Interface { inherits: None, mixins: {}, members: {}, is_hidden: true, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLPowerPreference": Enum(Enum { variants: {"default", "high-performance", "low-power"} }), "WebGLProgram": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLQuery": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLRenderbuffer": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLRenderingContext": Interface(Interface { inherits: None, mixins: {"WebGLRenderingContextBase"}, members: {}, is_hidden: false, has_class: true, rendering_context: Some("webgl"), doc_comment: "" }), "WebGLRenderingContextBase": Mixin(Mixin { members: {"ACTIVE_ATTRIBUTES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35721" })], "ACTIVE_TEXTURE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34016" })], "ACTIVE_UNIFORMS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35718" })], "ALIASED_LINE_WIDTH_RANGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33902" })], "ALIASED_POINT_SIZE_RANGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33901" })], "ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6406" })], "ALPHA_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3413" })], "ALWAYS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "519" })], "ARRAY_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34962" })], "ARRAY_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34964" })], "ATTACHED_SHADERS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35717" })], "BACK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1029" })], "BLEND": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3042" })], "BLEND_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32773" })], "BLEND_DST_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32970" })], "BLEND_DST_RGB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32968" })], "BLEND_EQUATION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32777" })], "BLEND_EQUATION_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34877" })], "BLEND_EQUATION_RGB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32777" })], "BLEND_SRC_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32971" })], "BLEND_SRC_RGB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32969" })], "BLUE_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3412" })], "BOOL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35670" })], "BOOL_VEC2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35671" })], "BOOL_VEC3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35672" })], "BOOL_VEC4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35673" })], "BROWSER_DEFAULT_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37444" })], "BUFFER_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34660" })], "BUFFER_USAGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34661" })], "BYTE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5120" })], "CCW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2305" })], "CLAMP_TO_EDGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33071" })], "COLOR_ATTACHMENT0": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36064" })], "COLOR_BUFFER_BIT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "16384" })], "COLOR_CLEAR_VALUE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3106" })], "COLOR_WRITEMASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3107" })], "COMPILE_STATUS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35713" })], "COMPRESSED_TEXTURE_FORMATS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34467" })], "CONSTANT_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32771" })], "CONSTANT_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32769" })], "CONTEXT_LOST_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37442" })], "CULL_FACE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2884" })], "CULL_FACE_MODE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2885" })], "CURRENT_PROGRAM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35725" })], "CURRENT_VERTEX_ATTRIB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34342" })], "CW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2304" })], "DECR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7683" })], "DECR_WRAP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34056" })], "DELETE_STATUS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35712" })], "DEPTH_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36096" })], "DEPTH_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3414" })], "DEPTH_BUFFER_BIT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "256" })], "DEPTH_CLEAR_VALUE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2931" })], "DEPTH_COMPONENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6402" })], "DEPTH_COMPONENT16": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33189" })], "DEPTH_FUNC": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2932" })], "DEPTH_RANGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2928" })], "DEPTH_STENCIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34041" })], "DEPTH_STENCIL_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33306" })], "DEPTH_TEST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2929" })], "DEPTH_WRITEMASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2930" })], "DITHER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3024" })], "DONT_CARE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "4352" })], "DST_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "772" })], "DST_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "774" })], "DYNAMIC_DRAW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35048" })], "ELEMENT_ARRAY_BUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34963" })], "ELEMENT_ARRAY_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34965" })], "EQUAL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "514" })], "FASTEST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "4353" })], "FLOAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5126" })], "FLOAT_MAT2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35674" })], "FLOAT_MAT3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35675" })], "FLOAT_MAT4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35676" })], "FLOAT_VEC2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35664" })], "FLOAT_VEC3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35665" })], "FLOAT_VEC4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35666" })], "FRAGMENT_SHADER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35632" })], "FRAMEBUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36160" })], "FRAMEBUFFER_ATTACHMENT_OBJECT_NAME": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36049" })], "FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36048" })], "FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36051" })], "FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36050" })], "FRAMEBUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36006" })], "FRAMEBUFFER_COMPLETE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36053" })], "FRAMEBUFFER_INCOMPLETE_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36054" })], "FRAMEBUFFER_INCOMPLETE_DIMENSIONS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36057" })], "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36055" })], "FRAMEBUFFER_UNSUPPORTED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36061" })], "FRONT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1028" })], "FRONT_AND_BACK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1032" })], "FRONT_FACE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2886" })], "FUNC_ADD": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32774" })], "FUNC_REVERSE_SUBTRACT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32779" })], "FUNC_SUBTRACT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32778" })], "GENERATE_MIPMAP_HINT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33170" })], "GEQUAL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "518" })], "GREATER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "516" })], "GREEN_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3411" })], "HIGH_FLOAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36338" })], "HIGH_INT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36341" })], "IMPLEMENTATION_COLOR_READ_FORMAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35739" })], "IMPLEMENTATION_COLOR_READ_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35738" })], "INCR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7682" })], "INCR_WRAP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34055" })], "INT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5124" })], "INT_VEC2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35667" })], "INT_VEC3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35668" })], "INT_VEC4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35669" })], "INVALID_ENUM": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1280" })], "INVALID_FRAMEBUFFER_OPERATION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1286" })], "INVALID_OPERATION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1282" })], "INVALID_VALUE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1281" })], "INVERT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5386" })], "KEEP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7680" })], "LEQUAL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "515" })], "LESS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "513" })], "LINEAR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9729" })], "LINEAR_MIPMAP_LINEAR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9987" })], "LINEAR_MIPMAP_NEAREST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9985" })], "LINES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1" })], "LINE_LOOP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2" })], "LINE_STRIP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3" })], "LINE_WIDTH": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2849" })], "LINK_STATUS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35714" })], "LOW_FLOAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36336" })], "LOW_INT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36339" })], "LUMINANCE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6409" })], "LUMINANCE_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6410" })], "MAX_COMBINED_TEXTURE_IMAGE_UNITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35661" })], "MAX_CUBE_MAP_TEXTURE_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34076" })], "MAX_FRAGMENT_UNIFORM_VECTORS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36349" })], "MAX_RENDERBUFFER_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34024" })], "MAX_TEXTURE_IMAGE_UNITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34930" })], "MAX_TEXTURE_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3379" })], "MAX_VARYING_VECTORS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36348" })], "MAX_VERTEX_ATTRIBS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34921" })], "MAX_VERTEX_TEXTURE_IMAGE_UNITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35660" })], "MAX_VERTEX_UNIFORM_VECTORS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36347" })], "MAX_VIEWPORT_DIMS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3386" })], "MEDIUM_FLOAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36337" })], "MEDIUM_INT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36340" })], "MIRRORED_REPEAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33648" })], "NEAREST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9728" })], "NEAREST_MIPMAP_LINEAR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9986" })], "NEAREST_MIPMAP_NEAREST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "9984" })], "NEVER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "512" })], "NICEST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "4354" })], "NONE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "0" })], "NOTEQUAL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "517" })], "NO_ERROR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "0" })], "ONE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1" })], "ONE_MINUS_CONSTANT_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32772" })], "ONE_MINUS_CONSTANT_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32770" })], "ONE_MINUS_DST_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "773" })], "ONE_MINUS_DST_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "775" })], "ONE_MINUS_SRC_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "771" })], "ONE_MINUS_SRC_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "769" })], "OUT_OF_MEMORY": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1285" })], "PACK_ALIGNMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3333" })], "POINTS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "0" })], "POLYGON_OFFSET_FACTOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32824" })], "POLYGON_OFFSET_FILL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32823" })], "POLYGON_OFFSET_UNITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10752" })], "RED_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3410" })], "RENDERBUFFER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36161" })], "RENDERBUFFER_ALPHA_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36179" })], "RENDERBUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36007" })], "RENDERBUFFER_BLUE_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36178" })], "RENDERBUFFER_DEPTH_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36180" })], "RENDERBUFFER_GREEN_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36177" })], "RENDERBUFFER_HEIGHT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36163" })], "RENDERBUFFER_INTERNAL_FORMAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36164" })], "RENDERBUFFER_RED_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36176" })], "RENDERBUFFER_STENCIL_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36181" })], "RENDERBUFFER_WIDTH": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36162" })], "RENDERER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7937" })], "REPEAT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10497" })], "REPLACE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7681" })], "RGB": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6407" })], "RGB565": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36194" })], "RGB5_A1": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32855" })], "RGBA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6408" })], "RGBA4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32854" })], "SAMPLER_2D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35678" })], "SAMPLER_CUBE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35680" })], "SAMPLES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32937" })], "SAMPLE_ALPHA_TO_COVERAGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32926" })], "SAMPLE_BUFFERS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32936" })], "SAMPLE_COVERAGE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32928" })], "SAMPLE_COVERAGE_INVERT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32939" })], "SAMPLE_COVERAGE_VALUE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32938" })], "SCISSOR_BOX": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3088" })], "SCISSOR_TEST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3089" })], "SHADER_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35663" })], "SHADING_LANGUAGE_VERSION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35724" })], "SHORT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5122" })], "SRC_ALPHA": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "770" })], "SRC_ALPHA_SATURATE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "776" })], "SRC_COLOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "768" })], "STATIC_DRAW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35044" })], "STENCIL_ATTACHMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36128" })], "STENCIL_BACK_FAIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34817" })], "STENCIL_BACK_FUNC": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34816" })], "STENCIL_BACK_PASS_DEPTH_FAIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34818" })], "STENCIL_BACK_PASS_DEPTH_PASS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34819" })], "STENCIL_BACK_REF": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36003" })], "STENCIL_BACK_VALUE_MASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36004" })], "STENCIL_BACK_WRITEMASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36005" })], "STENCIL_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3415" })], "STENCIL_BUFFER_BIT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "1024" })], "STENCIL_CLEAR_VALUE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2961" })], "STENCIL_FAIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2964" })], "STENCIL_FUNC": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2962" })], "STENCIL_INDEX8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "36168" })], "STENCIL_PASS_DEPTH_FAIL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2965" })], "STENCIL_PASS_DEPTH_PASS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2966" })], "STENCIL_REF": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2967" })], "STENCIL_TEST": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2960" })], "STENCIL_VALUE_MASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2963" })], "STENCIL_WRITEMASK": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2968" })], "STREAM_DRAW": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35040" })], "SUBPIXEL_BITS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3408" })], "TEXTURE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5890" })], "TEXTURE0": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33984" })], "TEXTURE1": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33985" })], "TEXTURE10": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33994" })], "TEXTURE11": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33995" })], "TEXTURE12": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33996" })], "TEXTURE13": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33997" })], "TEXTURE14": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33998" })], "TEXTURE15": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33999" })], "TEXTURE16": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34000" })], "TEXTURE17": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34001" })], "TEXTURE18": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34002" })], "TEXTURE19": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34003" })], "TEXTURE2": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33986" })], "TEXTURE20": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34004" })], "TEXTURE21": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34005" })], "TEXTURE22": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34006" })], "TEXTURE23": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34007" })], "TEXTURE24": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34008" })], "TEXTURE25": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34009" })], "TEXTURE26": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34010" })], "TEXTURE27": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34011" })], "TEXTURE28": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34012" })], "TEXTURE29": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34013" })], "TEXTURE3": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33987" })], "TEXTURE30": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34014" })], "TEXTURE31": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34015" })], "TEXTURE4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33988" })], "TEXTURE5": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33989" })], "TEXTURE6": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33990" })], "TEXTURE7": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33991" })], "TEXTURE8": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33992" })], "TEXTURE9": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33993" })], "TEXTURE_2D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3553" })], "TEXTURE_BINDING_2D": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32873" })], "TEXTURE_BINDING_CUBE_MAP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34068" })], "TEXTURE_CUBE_MAP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34067" })], "TEXTURE_CUBE_MAP_NEGATIVE_X": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34070" })], "TEXTURE_CUBE_MAP_NEGATIVE_Y": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34072" })], "TEXTURE_CUBE_MAP_NEGATIVE_Z": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34074" })], "TEXTURE_CUBE_MAP_POSITIVE_X": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34069" })], "TEXTURE_CUBE_MAP_POSITIVE_Y": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34071" })], "TEXTURE_CUBE_MAP_POSITIVE_Z": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34073" })], "TEXTURE_MAG_FILTER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10240" })], "TEXTURE_MIN_FILTER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10241" })], "TEXTURE_WRAP_S": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10242" })], "TEXTURE_WRAP_T": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "10243" })], "TRIANGLES": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "4" })], "TRIANGLE_FAN": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "6" })], "TRIANGLE_STRIP": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5" })], "UNPACK_ALIGNMENT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "3317" })], "UNPACK_COLORSPACE_CONVERSION_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37443" })], "UNPACK_FLIP_Y_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37440" })], "UNPACK_PREMULTIPLY_ALPHA_WEBGL": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "37441" })], "UNSIGNED_BYTE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5121" })], "UNSIGNED_INT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5125" })], "UNSIGNED_SHORT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "5123" })], "UNSIGNED_SHORT_4_4_4_4": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32819" })], "UNSIGNED_SHORT_5_5_5_1": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "32820" })], "UNSIGNED_SHORT_5_6_5": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "33635" })], "VALIDATE_STATUS": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35715" })], "VENDOR": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7936" })], "VERSION": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "7938" })], "VERTEX_ATTRIB_ARRAY_BUFFER_BINDING": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34975" })], "VERTEX_ATTRIB_ARRAY_ENABLED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34338" })], "VERTEX_ATTRIB_ARRAY_NORMALIZED": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34922" })], "VERTEX_ATTRIB_ARRAY_POINTER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34373" })], "VERTEX_ATTRIB_ARRAY_SIZE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34339" })], "VERTEX_ATTRIB_ARRAY_STRIDE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34340" })], "VERTEX_ATTRIB_ARRAY_TYPE": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "34341" })], "VERTEX_SHADER": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "35633" })], "VIEWPORT": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "2978" })], "ZERO": [Const(Const { type_: Type { kind: Named("GLenum"), optional: false }, value: "0" })], "activeTexture": [Operation(Operation { args: [Argument { name: "texture", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "attachShader": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bindAttribLocation": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "name", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bindBuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "buffer", optional: false, type_: Type { kind: Named("WebGLBuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindFramebuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "framebuffer", optional: false, type_: Type { kind: Named("WebGLFramebuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindRenderbuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "renderbuffer", optional: false, type_: Type { kind: Named("WebGLRenderbuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "bindTexture": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "texture", optional: false, type_: Type { kind: Named("WebGLTexture"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "blendColor": [Operation(Operation { args: [Argument { name: "red", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "green", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "blue", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "alpha", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "blendEquation": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "blendEquationSeparate": [Operation(Operation { args: [Argument { name: "modeRGB", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "modeAlpha", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "blendFunc": [Operation(Operation { args: [Argument { name: "sfactor", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dfactor", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "blendFuncSeparate": [Operation(Operation { args: [Argument { name: "srcRGB", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstRGB", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "srcAlpha", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "dstAlpha", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bufferData": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLsizeiptr"), optional: false }, variadic: false }, Argument { name: "usage", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: BufferSource, optional: true }, variadic: false }, Argument { name: "usage", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "bufferSubData": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: BufferSource, optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "canvas": [Attribute(Attribute { type_: Type { kind: CanvasElement, optional: false }, setter: false, getter: true })], "checkFramebufferStatus": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLenum"), optional: false }), doc_comment: "" })], "clear": [Operation(Operation { args: [Argument { name: "mask", optional: false, type_: Type { kind: Named("GLbitfield"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearColor": [Operation(Operation { args: [Argument { name: "red", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "green", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "blue", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "alpha", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearDepth": [Operation(Operation { args: [Argument { name: "depth", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "clearStencil": [Operation(Operation { args: [Argument { name: "s", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "colorMask": [Operation(Operation { args: [Argument { name: "red", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "green", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "blue", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "alpha", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compileShader": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compressedTexImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "compressedTexSubImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "data", optional: false, type_: Type { kind: ArrayBufferView, optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "copyTexImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "copyTexSubImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "createBuffer": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLBuffer"), optional: true }), doc_comment: "" })], "createFramebuffer": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLFramebuffer"), optional: true }), doc_comment: "" })], "createProgram": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLProgram"), optional: true }), doc_comment: "" })], "createRenderbuffer": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLRenderbuffer"), optional: true }), doc_comment: "" })], "createShader": [Operation(Operation { args: [Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLShader"), optional: true }), doc_comment: "" })], "createTexture": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLTexture"), optional: true }), doc_comment: "" })], "cullFace": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "deleteBuffer": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("WebGLBuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteFramebuffer": [Operation(Operation { args: [Argument { name: "framebuffer", optional: false, type_: Type { kind: Named("WebGLFramebuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteProgram": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteRenderbuffer": [Operation(Operation { args: [Argument { name: "renderbuffer", optional: false, type_: Type { kind: Named("WebGLRenderbuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteShader": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "deleteTexture": [Operation(Operation { args: [Argument { name: "texture", optional: false, type_: Type { kind: Named("WebGLTexture"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "depthFunc": [Operation(Operation { args: [Argument { name: "func", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "depthMask": [Operation(Operation { args: [Argument { name: "flag", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "depthRange": [Operation(Operation { args: [Argument { name: "zNear", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "zFar", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "detachShader": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "disable": [Operation(Operation { args: [Argument { name: "cap", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "disableVertexAttribArray": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawArrays": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "first", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "count", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawElements": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "count", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "drawingBufferHeight": [Attribute(Attribute { type_: Type { kind: Named("GLsizei"), optional: false }, setter: false, getter: true })], "drawingBufferWidth": [Attribute(Attribute { type_: Type { kind: Named("GLsizei"), optional: false }, setter: false, getter: true })], "enable": [Operation(Operation { args: [Argument { name: "cap", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "enableVertexAttribArray": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "finish": [Operation(Operation { args: [], return_type: None, doc_comment: "" })], "flush": [Operation(Operation { args: [], return_type: None, doc_comment: "" })], "framebufferRenderbuffer": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachment", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "renderbuffertarget", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "renderbuffer", optional: false, type_: Type { kind: Named("WebGLRenderbuffer"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "framebufferTexture2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachment", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "textarget", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "texture", optional: false, type_: Type { kind: Named("WebGLTexture"), optional: true }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "frontFace": [Operation(Operation { args: [Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "generateMipmap": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "getActiveAttrib": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLActiveInfo"), optional: true }), doc_comment: "" })], "getActiveUniform": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLActiveInfo"), optional: true }), doc_comment: "" })], "getAttachedShaders": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }], return_type: Some(Type { kind: Sequence(Type { kind: Named("WebGLShader"), optional: false }), optional: true }), doc_comment: "" })], "getAttribLocation": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "name", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLint"), optional: false }), doc_comment: "" })], "getBufferParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getContextAttributes": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("WebGLContextAttributes"), optional: true }), doc_comment: "" })], "getError": [Operation(Operation { args: [], return_type: Some(Type { kind: Named("GLenum"), optional: false }), doc_comment: "" })], "getExtension": [Operation(Operation { args: [Argument { name: "name", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: Some(Type { kind: Object, optional: true }), doc_comment: "" })], "getFramebufferAttachmentParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "attachment", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getParameter": [Operation(Operation { args: [Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getProgramInfoLog": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }], return_type: Some(Type { kind: String, optional: true }), doc_comment: "" })], "getProgramParameter": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getRenderbufferParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getShaderInfoLog": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }], return_type: Some(Type { kind: String, optional: true }), doc_comment: "" })], "getShaderParameter": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getShaderPrecisionFormat": [Operation(Operation { args: [Argument { name: "shadertype", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "precisiontype", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLShaderPrecisionFormat"), optional: true }), doc_comment: "" })], "getShaderSource": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }], return_type: Some(Type { kind: String, optional: true }), doc_comment: "" })], "getSupportedExtensions": [Operation(Operation { args: [], return_type: Some(Type { kind: Sequence(Type { kind: String, optional: false }), optional: true }), doc_comment: "" })], "getTexParameter": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getUniform": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getUniformLocation": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }, Argument { name: "name", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: Some(Type { kind: Named("WebGLUniformLocation"), optional: true }), doc_comment: "" })], "getVertexAttrib": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Any, optional: false }), doc_comment: "" })], "getVertexAttribOffset": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLintptr"), optional: false }), doc_comment: "" })], "hint": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "mode", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "isBuffer": [Operation(Operation { args: [Argument { name: "buffer", optional: false, type_: Type { kind: Named("WebGLBuffer"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isContextLost": [Operation(Operation { args: [], return_type: Some(Type { kind: Primitive(Bool), optional: false }), doc_comment: "" })], "isEnabled": [Operation(Operation { args: [Argument { name: "cap", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isFramebuffer": [Operation(Operation { args: [Argument { name: "framebuffer", optional: false, type_: Type { kind: Named("WebGLFramebuffer"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isProgram": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isRenderbuffer": [Operation(Operation { args: [Argument { name: "renderbuffer", optional: false, type_: Type { kind: Named("WebGLRenderbuffer"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isShader": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "isTexture": [Operation(Operation { args: [Argument { name: "texture", optional: false, type_: Type { kind: Named("WebGLTexture"), optional: true }, variadic: false }], return_type: Some(Type { kind: Named("GLboolean"), optional: false }), doc_comment: "" })], "lineWidth": [Operation(Operation { args: [Argument { name: "width", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "linkProgram": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "pixelStorei": [Operation(Operation { args: [Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "param", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "polygonOffset": [Operation(Operation { args: [Argument { name: "factor", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "units", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "readPixels": [Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pixels", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "renderbufferStorage": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "sampleCoverage": [Operation(Operation { args: [Argument { name: "value", optional: false, type_: Type { kind: Named("GLclampf"), optional: false }, variadic: false }, Argument { name: "invert", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "scissor": [Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "shaderSource": [Operation(Operation { args: [Argument { name: "shader", optional: false, type_: Type { kind: Named("WebGLShader"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: String, optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilFunc": [Operation(Operation { args: [Argument { name: "func", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "ref", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "mask", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilFuncSeparate": [Operation(Operation { args: [Argument { name: "face", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "func", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "ref", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "mask", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilMask": [Operation(Operation { args: [Argument { name: "mask", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilMaskSeparate": [Operation(Operation { args: [Argument { name: "face", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "mask", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilOp": [Operation(Operation { args: [Argument { name: "fail", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "zfail", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "zpass", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "stencilOpSeparate": [Operation(Operation { args: [Argument { name: "face", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "fail", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "zfail", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "zpass", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "border", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pixels", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "internalformat", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texParameterf": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "param", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texParameteri": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pname", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "param", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "texSubImage2D": [Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "pixels", optional: false, type_: Type { kind: ArrayBufferView, optional: true }, variadic: false }], return_type: None, doc_comment: "" }), Operation(Operation { args: [Argument { name: "target", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "level", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "xoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "yoffset", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "format", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "source", optional: false, type_: Type { kind: Named("TexImageSource"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1f": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1i": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform1iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2f": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2i": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform2iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3f": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3i": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform3iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4f": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "w", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4i": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "w", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniform4iv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "v", optional: false, type_: Type { kind: Named("Int32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix2fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "value", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix3fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "value", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "uniformMatrix4fv": [Operation(Operation { args: [Argument { name: "location", optional: false, type_: Type { kind: Named("WebGLUniformLocation"), optional: true }, variadic: false }, Argument { name: "transpose", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "value", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "useProgram": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: true }, variadic: false }], return_type: None, doc_comment: "" })], "validateProgram": [Operation(Operation { args: [Argument { name: "program", optional: false, type_: Type { kind: Named("WebGLProgram"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib1f": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib1fv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib2f": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib2fv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib3f": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib3fv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib4f": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "x", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "z", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }, Argument { name: "w", optional: false, type_: Type { kind: Named("GLfloat"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttrib4fv": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "values", optional: false, type_: Type { kind: Named("Float32List"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "vertexAttribPointer": [Operation(Operation { args: [Argument { name: "index", optional: false, type_: Type { kind: Named("GLuint"), optional: false }, variadic: false }, Argument { name: "size", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "type", optional: false, type_: Type { kind: Named("GLenum"), optional: false }, variadic: false }, Argument { name: "normalized", optional: false, type_: Type { kind: Named("GLboolean"), optional: false }, variadic: false }, Argument { name: "stride", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "offset", optional: false, type_: Type { kind: Named("GLintptr"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })], "viewport": [Operation(Operation { args: [Argument { name: "x", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "y", optional: false, type_: Type { kind: Named("GLint"), optional: false }, variadic: false }, Argument { name: "width", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }, Argument { name: "height", optional: false, type_: Type { kind: Named("GLsizei"), optional: false }, variadic: false }], return_type: None, doc_comment: "" })]} }), "WebGLSampler": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLShader": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLShaderPrecisionFormat": Interface(Interface { inherits: None, mixins: {}, members: {"precision": [Attribute(Attribute { type_: Type { kind: Named("GLint"), optional: false }, setter: false, getter: true })], "rangeMax": [Attribute(Attribute { type_: Type { kind: Named("GLint"), optional: false }, setter: false, getter: true })], "rangeMin": [Attribute(Attribute { type_: Type { kind: Named("GLint"), optional: false }, setter: false, getter: true })]}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLSync": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLTexture": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLTransformFeedback": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLUniformLocation": Interface(Interface { inherits: None, mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" }), "WebGLVertexArrayObject": Interface(Interface { inherits: Some("WebGLObject"), mixins: {}, members: {}, is_hidden: false, has_class: true, rendering_context: None, doc_comment: "" })}, extensions: {} }
extern crate stdweb;

use self::stdweb::unstable::{TryFrom, TryInto};
use self::stdweb::web::html_element::CanvasElement;
use self::stdweb::web::{ArrayBuffer, RenderingContext, TypedArray};
use self::stdweb::{InstanceOf, JsSerialize, Once, Reference, UnsafeTypedArray, Value};

type ConversionError = <Reference as TryFrom<Value>>::Error;

pub trait AsTypedArray<'a, T> {
    type Result: JsSerialize;

    unsafe fn as_typed_array(self) -> Self::Result;
}

pub trait Extension: TryFrom<Value> {
    const NAME: &'static str;
}

macro_rules! define_array {
    ($elem:ty) => {
        impl<'a> AsTypedArray<'a, $elem> for &'a TypedArray<$elem> {
            type Result = Self;

            unsafe fn as_typed_array(self) -> Self::Result {
                self
            }
        }

        impl<'a> AsTypedArray<'a, $elem> for &'a [$elem] {
            type Result = UnsafeTypedArray<'a, $elem>;

            unsafe fn as_typed_array(self) -> Self::Result {
                UnsafeTypedArray::new(self)
            }
        }
    };
}

define_array!(i8);
define_array!(u8);
define_array!(i16);
define_array!(u16);
define_array!(i32);
define_array!(u32);
define_array!(f32);
define_array!(f64);

pub type Float32List = TypedArray<f32>;
pub type GLbitfield = u32;
pub type GLboolean = bool;
pub type GLbyte = i8;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLint64 = i64;
pub type GLintptr = i64;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLsizeiptr = i64;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLuint64 = u64;
pub type GLushort = u16;
pub type Int32List = TypedArray<i32>;
pub type TexImageSource = Value;
pub type Uint32List = TypedArray<u32>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebGLPowerPreference {
    #[serde(rename = "default")]
    Default,

    #[serde(rename = "high-performance")]
    HighPerformance,

    #[serde(rename = "low-power")]
    LowPower,
}
js_deserializable!(WebGLPowerPreference);
js_serializable!(WebGLPowerPreference);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGLContextAttributes {
    alpha: GLboolean,

    antialias: GLboolean,

    depth: GLboolean,

    #[serde(rename = "failIfMajorPerformanceCaveat")]
    fail_if_major_performance_caveat: GLboolean,

    #[serde(rename = "powerPreference")]
    power_preference: WebGLPowerPreference,

    #[serde(rename = "premultipliedAlpha")]
    premultiplied_alpha: GLboolean,

    #[serde(rename = "preserveDrawingBuffer")]
    preserve_drawing_buffer: GLboolean,

    stencil: GLboolean,
}
js_deserializable!(WebGLContextAttributes);
js_serializable!(WebGLContextAttributes);

#[derive(Debug, Clone, ReferenceType)]
pub struct GLContext(Reference);

impl GLContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 35382;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALREADY_SIGNALED: GLenum = 37146;
    pub const ALWAYS: GLenum = 519;
    pub const ANY_SAMPLES_PASSED: GLenum = 35887;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 36202;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR: GLenum = 6144;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_ATTACHMENT1: GLenum = 36065;
    pub const COLOR_ATTACHMENT10: GLenum = 36074;
    pub const COLOR_ATTACHMENT11: GLenum = 36075;
    pub const COLOR_ATTACHMENT12: GLenum = 36076;
    pub const COLOR_ATTACHMENT13: GLenum = 36077;
    pub const COLOR_ATTACHMENT14: GLenum = 36078;
    pub const COLOR_ATTACHMENT15: GLenum = 36079;
    pub const COLOR_ATTACHMENT2: GLenum = 36066;
    pub const COLOR_ATTACHMENT3: GLenum = 36067;
    pub const COLOR_ATTACHMENT4: GLenum = 36068;
    pub const COLOR_ATTACHMENT5: GLenum = 36069;
    pub const COLOR_ATTACHMENT6: GLenum = 36070;
    pub const COLOR_ATTACHMENT7: GLenum = 36071;
    pub const COLOR_ATTACHMENT8: GLenum = 36072;
    pub const COLOR_ATTACHMENT9: GLenum = 36073;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPARE_REF_TO_TEXTURE: GLenum = 34894;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONDITION_SATISFIED: GLenum = 37148;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const COPY_READ_BUFFER: GLenum = 36662;
    pub const COPY_READ_BUFFER_BINDING: GLenum = 36662;
    pub const COPY_WRITE_BUFFER: GLenum = 36663;
    pub const COPY_WRITE_BUFFER_BINDING: GLenum = 36663;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_QUERY: GLenum = 34917;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH: GLenum = 6145;
    pub const DEPTH24_STENCIL8: GLenum = 35056;
    pub const DEPTH32F_STENCIL8: GLenum = 36013;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_COMPONENT24: GLenum = 33190;
    pub const DEPTH_COMPONENT32F: GLenum = 36012;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DRAW_BUFFER0: GLenum = 34853;
    pub const DRAW_BUFFER1: GLenum = 34854;
    pub const DRAW_BUFFER10: GLenum = 34863;
    pub const DRAW_BUFFER11: GLenum = 34864;
    pub const DRAW_BUFFER12: GLenum = 34865;
    pub const DRAW_BUFFER13: GLenum = 34866;
    pub const DRAW_BUFFER14: GLenum = 34867;
    pub const DRAW_BUFFER15: GLenum = 34868;
    pub const DRAW_BUFFER2: GLenum = 34855;
    pub const DRAW_BUFFER3: GLenum = 34856;
    pub const DRAW_BUFFER4: GLenum = 34857;
    pub const DRAW_BUFFER5: GLenum = 34858;
    pub const DRAW_BUFFER6: GLenum = 34859;
    pub const DRAW_BUFFER7: GLenum = 34860;
    pub const DRAW_BUFFER8: GLenum = 34861;
    pub const DRAW_BUFFER9: GLenum = 34862;
    pub const DRAW_FRAMEBUFFER: GLenum = 36009;
    pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_COPY: GLenum = 35050;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const DYNAMIC_READ: GLenum = 35049;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 36269;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT2X3: GLenum = 35685;
    pub const FLOAT_MAT2X4: GLenum = 35686;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT3X2: GLenum = 35687;
    pub const FLOAT_MAT3X4: GLenum = 35688;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_MAT4X2: GLenum = 35689;
    pub const FLOAT_MAT4X3: GLenum = 35690;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 35723;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 33301;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 33300;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 33296;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 33297;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 33302;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 33299;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 33298;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 33303;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 36052;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_DEFAULT: GLenum = 33304;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 36182;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HALF_FLOAT: GLenum = 5131;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INTERLEAVED_ATTRIBS: GLenum = 35980;
    pub const INT_2_10_10_10_REV: GLenum = 36255;
    pub const INT_SAMPLER_2D: GLenum = 36298;
    pub const INT_SAMPLER_2D_ARRAY: GLenum = 36303;
    pub const INT_SAMPLER_3D: GLenum = 36299;
    pub const INT_SAMPLER_CUBE: GLenum = 36300;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_INDEX: GLenum = 4294967295;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX: GLenum = 32776;
    pub const MAX_3D_TEXTURE_SIZE: GLenum = 32883;
    pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 35071;
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: GLenum = 37447;
    pub const MAX_COLOR_ATTACHMENTS: GLenum = 36063;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35379;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 35374;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 35377;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_DRAW_BUFFERS: GLenum = 34852;
    pub const MAX_ELEMENTS_INDICES: GLenum = 33001;
    pub const MAX_ELEMENTS_VERTICES: GLenum = 33000;
    pub const MAX_ELEMENT_INDEX: GLenum = 36203;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 37157;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 35373;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35657;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 35077;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_SAMPLES: GLenum = 36183;
    pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 37137;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_LOD_BIAS: GLenum = 34045;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 35978;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 35979;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 35968;
    pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 35376;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 35375;
    pub const MAX_VARYING_COMPONENTS: GLenum = 35659;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 37154;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 35371;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 35658;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIN: GLenum = 32775;
    pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 35076;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const OBJECT_TYPE: GLenum = 37138;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const PACK_ROW_LENGTH: GLenum = 3330;
    pub const PACK_SKIP_PIXELS: GLenum = 3332;
    pub const PACK_SKIP_ROWS: GLenum = 3331;
    pub const PIXEL_PACK_BUFFER: GLenum = 35051;
    pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 35053;
    pub const PIXEL_UNPACK_BUFFER: GLenum = 35052;
    pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 35055;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const QUERY_RESULT: GLenum = 34918;
    pub const QUERY_RESULT_AVAILABLE: GLenum = 34919;
    pub const R11F_G11F_B10F: GLenum = 35898;
    pub const R16F: GLenum = 33325;
    pub const R16I: GLenum = 33331;
    pub const R16UI: GLenum = 33332;
    pub const R32F: GLenum = 33326;
    pub const R32I: GLenum = 33333;
    pub const R32UI: GLenum = 33334;
    pub const R8: GLenum = 33321;
    pub const R8I: GLenum = 33329;
    pub const R8UI: GLenum = 33330;
    pub const R8_SNORM: GLenum = 36756;
    pub const RASTERIZER_DISCARD: GLenum = 35977;
    pub const READ_BUFFER: GLenum = 3074;
    pub const READ_FRAMEBUFFER: GLenum = 36008;
    pub const READ_FRAMEBUFFER_BINDING: GLenum = 36010;
    pub const RED: GLenum = 6403;
    pub const RED_BITS: GLenum = 3410;
    pub const RED_INTEGER: GLenum = 36244;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_SAMPLES: GLenum = 36011;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RG: GLenum = 33319;
    pub const RG16F: GLenum = 33327;
    pub const RG16I: GLenum = 33337;
    pub const RG16UI: GLenum = 33338;
    pub const RG32F: GLenum = 33328;
    pub const RG32I: GLenum = 33339;
    pub const RG32UI: GLenum = 33340;
    pub const RG8: GLenum = 33323;
    pub const RG8I: GLenum = 33335;
    pub const RG8UI: GLenum = 33336;
    pub const RG8_SNORM: GLenum = 36757;
    pub const RGB: GLenum = 6407;
    pub const RGB10_A2: GLenum = 32857;
    pub const RGB10_A2UI: GLenum = 36975;
    pub const RGB16F: GLenum = 34843;
    pub const RGB16I: GLenum = 36233;
    pub const RGB16UI: GLenum = 36215;
    pub const RGB32F: GLenum = 34837;
    pub const RGB32I: GLenum = 36227;
    pub const RGB32UI: GLenum = 36209;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGB8: GLenum = 32849;
    pub const RGB8I: GLenum = 36239;
    pub const RGB8UI: GLenum = 36221;
    pub const RGB8_SNORM: GLenum = 36758;
    pub const RGB9_E5: GLenum = 35901;
    pub const RGBA: GLenum = 6408;
    pub const RGBA16F: GLenum = 34842;
    pub const RGBA16I: GLenum = 36232;
    pub const RGBA16UI: GLenum = 36214;
    pub const RGBA32F: GLenum = 34836;
    pub const RGBA32I: GLenum = 36226;
    pub const RGBA32UI: GLenum = 36208;
    pub const RGBA4: GLenum = 32854;
    pub const RGBA8: GLenum = 32856;
    pub const RGBA8I: GLenum = 36238;
    pub const RGBA8UI: GLenum = 36220;
    pub const RGBA8_SNORM: GLenum = 36759;
    pub const RGBA_INTEGER: GLenum = 36249;
    pub const RGB_INTEGER: GLenum = 36248;
    pub const RG_INTEGER: GLenum = 33320;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_2D_ARRAY: GLenum = 36289;
    pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 36292;
    pub const SAMPLER_2D_SHADOW: GLenum = 35682;
    pub const SAMPLER_3D: GLenum = 35679;
    pub const SAMPLER_BINDING: GLenum = 35097;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLER_CUBE_SHADOW: GLenum = 36293;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SEPARATE_ATTRIBS: GLenum = 35981;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SIGNALED: GLenum = 37145;
    pub const SIGNED_NORMALIZED: GLenum = 36764;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const SRGB: GLenum = 35904;
    pub const SRGB8: GLenum = 35905;
    pub const SRGB8_ALPHA8: GLenum = 35907;
    pub const STATIC_COPY: GLenum = 35046;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STATIC_READ: GLenum = 35045;
    pub const STENCIL: GLenum = 6146;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_COPY: GLenum = 35042;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const STREAM_READ: GLenum = 35041;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const SYNC_CONDITION: GLenum = 37139;
    pub const SYNC_FENCE: GLenum = 37142;
    pub const SYNC_FLAGS: GLenum = 37141;
    pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 1;
    pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 37143;
    pub const SYNC_STATUS: GLenum = 37140;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_2D_ARRAY: GLenum = 35866;
    pub const TEXTURE_3D: GLenum = 32879;
    pub const TEXTURE_BASE_LEVEL: GLenum = 33084;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 35869;
    pub const TEXTURE_BINDING_3D: GLenum = 32874;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_COMPARE_FUNC: GLenum = 34893;
    pub const TEXTURE_COMPARE_MODE: GLenum = 34892;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 37167;
    pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 33503;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MAX_LEVEL: GLenum = 33085;
    pub const TEXTURE_MAX_LOD: GLenum = 33083;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_MIN_LOD: GLenum = 33082;
    pub const TEXTURE_WRAP_R: GLenum = 32882;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TIMEOUT_EXPIRED: GLenum = 37147;
    pub const TIMEOUT_IGNORED: GLint64 = -1;
    pub const TRANSFORM_FEEDBACK: GLenum = 36386;
    pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 36388;
    pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 36389;
    pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 35982;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 35983;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 35967;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 35973;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 35972;
    pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 36387;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 35976;
    pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 35971;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNIFORM_ARRAY_STRIDE: GLenum = 35388;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 35394;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 35395;
    pub const UNIFORM_BLOCK_BINDING: GLenum = 35391;
    pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 35392;
    pub const UNIFORM_BLOCK_INDEX: GLenum = 35386;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 35398;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 35396;
    pub const UNIFORM_BUFFER: GLenum = 35345;
    pub const UNIFORM_BUFFER_BINDING: GLenum = 35368;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 35380;
    pub const UNIFORM_BUFFER_SIZE: GLenum = 35370;
    pub const UNIFORM_BUFFER_START: GLenum = 35369;
    pub const UNIFORM_IS_ROW_MAJOR: GLenum = 35390;
    pub const UNIFORM_MATRIX_STRIDE: GLenum = 35389;
    pub const UNIFORM_OFFSET: GLenum = 35387;
    pub const UNIFORM_SIZE: GLenum = 35384;
    pub const UNIFORM_TYPE: GLenum = 35383;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_IMAGE_HEIGHT: GLenum = 32878;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNPACK_ROW_LENGTH: GLenum = 3314;
    pub const UNPACK_SKIP_IMAGES: GLenum = 32877;
    pub const UNPACK_SKIP_PIXELS: GLenum = 3316;
    pub const UNPACK_SKIP_ROWS: GLenum = 3315;
    pub const UNSIGNALED: GLenum = 37144;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 35899;
    pub const UNSIGNED_INT_24_8: GLenum = 34042;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 33640;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 35902;
    pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 36306;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 36311;
    pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 36307;
    pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 36308;
    pub const UNSIGNED_INT_VEC2: GLenum = 36294;
    pub const UNSIGNED_INT_VEC3: GLenum = 36295;
    pub const UNSIGNED_INT_VEC4: GLenum = 36296;
    pub const UNSIGNED_NORMALIZED: GLenum = 35863;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ARRAY_BINDING: GLenum = 34229;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 35070;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 35069;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const WAIT_FAILED: GLenum = 37149;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn begin_query(&self, target: GLenum, query: &WebGLQuery) {
        js!( @{self}.beginQuery(@{target}, @{query}); );
    }

    pub fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        js!( @{self}.beginTransformFeedback(@{primitive_mode}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBufferBase(@{target}, @{index}, @{buffer}); );
    }

    pub fn bind_buffer_range(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: Option<&WebGLBuffer>,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @{self}.bindBufferRange(@{target}, @{index}, @{buffer}, @{(offset as f64)}, @{(size as f64)}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_sampler(&self, unit: GLuint, sampler: Option<&WebGLSampler>) {
        js!( @{self}.bindSampler(@{unit}, @{sampler}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn bind_transform_feedback(&self, target: GLenum, tf: Option<&WebGLTransformFeedback>) {
        js!( @{self}.bindTransformFeedback(@{target}, @{tf}); );
    }

    pub fn bind_vertex_array(&self, array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.bindVertexArray(@{array}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        js!( @{self}.blitFramebuffer(@{src_x0}, @{src_y0}, @{src_x1}, @{src_y1}, @{dst_x0}, @{dst_y0}, @{dst_x1}, @{dst_y1}, @{mask}, @{filter}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, src_data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}); );
    }

    pub fn buffer_data_2(
        &self,
        target: GLenum,
        src_data: &ArrayBuffer,
        usage: GLenum,
        src_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}, @{src_offset}, @{length}); );
    }

    pub fn buffer_sub_data(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
    ) {
        js!( @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}); );
    }

    pub fn buffer_sub_data_1(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}, @{src_offset}, @{length}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @{self}.clear(@{mask}); );
    }

    pub fn clear_bufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        js!( @{self}.clearBufferfi(@{buffer}, @{drawbuffer}, @{depth}, @{stencil}); );
    }

    pub fn clear_bufferfv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.clearBufferfv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.clearBufferiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferuiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.clearBufferuiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @{self}.clearStencil(@{s}); );
    }

    pub fn client_wait_sync(
        &self,
        sync: &WebGLSync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        (js! { return @{self}.clientWaitSync(@{sync}, @{flags}, @{(timeout as f64)}); })
            .try_into()
            .unwrap()
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image2_d_1(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{data}); );
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image3_d_1(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image2_d_1(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{data}); );
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image3_d_1(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @{self}.copyBufferSubData(@{read_target}, @{write_target}, @{(read_offset as f64)}, @{(write_offset as f64)}, @{(size as f64)}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.copyTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_query(&self) -> Option<WebGLQuery> {
        (js! { return @{self}.createQuery(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_sampler(&self) -> Option<WebGLSampler> {
        (js! { return @{self}.createSampler(); }).try_into().ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn create_transform_feedback(&self) -> Option<WebGLTransformFeedback> {
        (js! { return @{self}.createTransformFeedback(); })
            .try_into()
            .ok()
    }

    pub fn create_vertex_array(&self) -> Option<WebGLVertexArrayObject> {
        (js! { return @{self}.createVertexArray(); })
            .try_into()
            .ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_query(&self, query: Option<&WebGLQuery>) {
        js!( @{self}.deleteQuery(@{query}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_sampler(&self, sampler: Option<&WebGLSampler>) {
        js!( @{self}.deleteSampler(@{sampler}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_sync(&self, sync: Option<&WebGLSync>) {
        js!( @{self}.deleteSync(@{sync}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @{self}.deleteTexture(@{texture}); );
    }

    pub fn delete_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) {
        js!( @{self}.deleteTransformFeedback(@{tf}); );
    }

    pub fn delete_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.deleteVertexArray(@{vertex_array}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instance_count: GLsizei,
    ) {
        js!( @{self}.drawArraysInstanced(@{mode}, @{first}, @{count}, @{instance_count}); );
    }

    pub fn draw_buffers(&self, buffers: &[GLenum]) {
        js!( @{self}.drawBuffers(@{buffers}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn draw_elements_instanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
        instance_count: GLsizei,
    ) {
        js!( @{self}.drawElementsInstanced(@{mode}, @{count}, @{type_}, @{(offset as f64)}, @{instance_count}); );
    }

    pub fn draw_range_elements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @{self}.drawRangeElements(@{mode}, @{start}, @{end}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn end_query(&self, target: GLenum) {
        js!( @{self}.endQuery(@{target}); );
    }

    pub fn end_transform_feedback(&self) {
        js!( @{self}.endTransformFeedback(); );
    }

    pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> Option<WebGLSync> {
        (js! { return @{self}.fenceSync(@{condition}, @{flags}); })
            .try_into()
            .ok()
    }

    pub fn finish(&self) {
        js!( @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
        layer: GLint,
    ) {
        js!( @{self}.framebufferTextureLayer(@{target}, @{attachment}, @{texture}, @{level}, @{layer}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
    ) -> Option<String> {
        (js! { return @{self}.getActiveUniformBlockName(@{program}, @{uniform_block_index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_parameter(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniformBlockParameter(@{program}, @{uniform_block_index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_active_uniforms(
        &self,
        program: &WebGLProgram,
        uniform_indices: &[GLuint],
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniforms(@{program}, @{uniform_indices}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_sub_data(
        &self,
        target: GLenum,
        src_byte_offset: GLintptr,
        dst_buffer: &ArrayBuffer,
        dst_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.getBufferSubData(@{target}, @{(src_byte_offset as f64)}, @{dst_buffer}, @{dst_offset}, @{length}); );
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension({E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getFragDataLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_indexed_parameter(&self, target: GLenum, index: GLuint) -> Value {
        (js! { return @{self}.getIndexedParameter(@{target}, @{index}); })
            .try_into()
            .unwrap()
    }

    pub fn get_internalformat_parameter(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getInternalformatParameter(@{target}, @{internalformat}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_query(&self, target: GLenum, pname: GLenum) -> Option<WebGLQuery> {
        (js! { return @{self}.getQuery(@{target}, @{pname}); })
            .try_into()
            .ok()
    }

    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: GLenum) -> Value {
        (js! { return @{self}.getQueryParameter(@{query}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: GLenum) -> Value {
        (js! { return @{self}.getSamplerParameter(@{sampler}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: GLenum) -> Value {
        (js! { return @{self}.getSyncParameter(@{sync}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getTransformFeedbackVarying(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_block_index(
        &self,
        program: &WebGLProgram,
        uniform_block_name: &str,
    ) -> GLuint {
        (js! { return @{self}.getUniformBlockIndex(@{program}, @{uniform_block_name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_indices(
        &self,
        program: &WebGLProgram,
        uniform_names: &[&str],
    ) -> Option<Vec<GLuint>> {
        (js! { return @{self}.getUniformIndices(@{program}, @{uniform_names}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @{self}.hint(@{target}, @{mode}); );
    }

    pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
        js!( @{self}.invalidateFramebuffer(@{target}, @{attachments}); );
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: GLenum,
        attachments: &[GLenum],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.invalidateSubFramebuffer(@{target}, @{attachments}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_query(&self, query: Option<&WebGLQuery>) -> GLboolean {
        (js! { return @{self}.isQuery(@{query}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sampler(&self, sampler: Option<&WebGLSampler>) -> GLboolean {
        (js! { return @{self}.isSampler(@{sampler}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sync(&self, sync: Option<&WebGLSync>) -> GLboolean {
        (js! { return @{self}.isSync(@{sync}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn is_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) -> GLboolean {
        (js! { return @{self}.isTransformFeedback(@{tf}); })
            .try_into()
            .unwrap()
    }

    pub fn is_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) -> GLboolean {
        (js! { return @{self}.isVertexArray(@{vertex_array}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @{self}.linkProgram(@{program}); );
    }

    pub fn pause_transform_feedback(&self) {
        js!( @{self}.pauseTransformFeedback(); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_buffer(&self, src: GLenum) {
        js!( @{self}.readBuffer(@{src}); );
    }

    pub fn read_pixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}); );
    }

    pub fn read_pixels_1(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{(offset as f64)}); );
    }

    pub fn read_pixels_2(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: &ArrayBuffer,
        dst_offset: GLuint,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}, @{dst_offset}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.renderbufferStorageMultisample(@{target}, @{samples}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn resume_transform_feedback(&self) {
        js!( @{self}.resumeTransformFeedback(); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn sampler_parameterf(&self, sampler: &WebGLSampler, pname: GLenum, param: GLfloat) {
        js!( @{self}.samplerParameterf(@{sampler}, @{pname}, @{param}); );
    }

    pub fn sampler_parameteri(&self, sampler: &WebGLSampler, pname: GLenum, param: GLint) {
        js!( @{self}.samplerParameteri(@{sampler}, @{pname}, @{param}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_4(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image3_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}); );
    }

    pub fn tex_image3_d_3(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_storage2_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.texStorage2D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn tex_storage3_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        js!( @{self}.texStorage3D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}, @{depth}); );
    }

    pub fn tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_4(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image3_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: Option<&ArrayBuffer>,
        src_offset: GLuint,
    ) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn transform_feedback_varyings(
        &self,
        program: &WebGLProgram,
        varyings: &[&str],
        buffer_mode: GLenum,
    ) {
        js!( @{self}.transformFeedbackVaryings(@{program}, @{varyings}, @{buffer_mode}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint) {
        js!( @{self}.uniform1ui(@{location}, @{v0}); );
    }

    pub fn uniform1uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform1uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint) {
        js!( @{self}.uniform2ui(@{location}, @{v0}, @{v1}); );
    }

    pub fn uniform2uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform2uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        js!( @{self}.uniform3ui(@{location}, @{v0}, @{v1}, @{v2}); );
    }

    pub fn uniform3uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform3uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        js!( @{self}.uniform4ui(@{location}, @{v0}, @{v1}, @{v2}, @{v3}); );
    }

    pub fn uniform4uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform4uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_block_binding(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        js!( @{self}.uniformBlockBinding(@{program}, @{uniform_block_index}, @{uniform_block_binding}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix2x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        js!( @{self}.vertexAttribDivisor(@{index}, @{divisor}); );
    }

    pub fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @{self}.vertexAttribI4i(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4iv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.vertexAttribI4iv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        js!( @{self}.vertexAttribI4ui(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4uiv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.vertexAttribI4uiv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.vertexAttribIPointer(@{index}, @{size}, @{type_}, @{stride}, @{(offset as f64)}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLint64) {
        js!( @{self}.waitSync(@{sync}, @{flags}, @{(timeout as f64)}); );
    }
}

impl InstanceOf for GLContext {
    #[inline]
    fn instance_of(reference: &Reference) -> bool {
        js!(
            return [WebGLRenderingContext, WebGL2RenderingContext].includes( @{{reference}}.constructor );
        ).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGL2RenderingContext")]
pub struct WebGL2RenderingContext(Reference);

impl WebGL2RenderingContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 35382;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALREADY_SIGNALED: GLenum = 37146;
    pub const ALWAYS: GLenum = 519;
    pub const ANY_SAMPLES_PASSED: GLenum = 35887;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 36202;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR: GLenum = 6144;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_ATTACHMENT1: GLenum = 36065;
    pub const COLOR_ATTACHMENT10: GLenum = 36074;
    pub const COLOR_ATTACHMENT11: GLenum = 36075;
    pub const COLOR_ATTACHMENT12: GLenum = 36076;
    pub const COLOR_ATTACHMENT13: GLenum = 36077;
    pub const COLOR_ATTACHMENT14: GLenum = 36078;
    pub const COLOR_ATTACHMENT15: GLenum = 36079;
    pub const COLOR_ATTACHMENT2: GLenum = 36066;
    pub const COLOR_ATTACHMENT3: GLenum = 36067;
    pub const COLOR_ATTACHMENT4: GLenum = 36068;
    pub const COLOR_ATTACHMENT5: GLenum = 36069;
    pub const COLOR_ATTACHMENT6: GLenum = 36070;
    pub const COLOR_ATTACHMENT7: GLenum = 36071;
    pub const COLOR_ATTACHMENT8: GLenum = 36072;
    pub const COLOR_ATTACHMENT9: GLenum = 36073;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPARE_REF_TO_TEXTURE: GLenum = 34894;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONDITION_SATISFIED: GLenum = 37148;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const COPY_READ_BUFFER: GLenum = 36662;
    pub const COPY_READ_BUFFER_BINDING: GLenum = 36662;
    pub const COPY_WRITE_BUFFER: GLenum = 36663;
    pub const COPY_WRITE_BUFFER_BINDING: GLenum = 36663;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_QUERY: GLenum = 34917;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH: GLenum = 6145;
    pub const DEPTH24_STENCIL8: GLenum = 35056;
    pub const DEPTH32F_STENCIL8: GLenum = 36013;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_COMPONENT24: GLenum = 33190;
    pub const DEPTH_COMPONENT32F: GLenum = 36012;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DRAW_BUFFER0: GLenum = 34853;
    pub const DRAW_BUFFER1: GLenum = 34854;
    pub const DRAW_BUFFER10: GLenum = 34863;
    pub const DRAW_BUFFER11: GLenum = 34864;
    pub const DRAW_BUFFER12: GLenum = 34865;
    pub const DRAW_BUFFER13: GLenum = 34866;
    pub const DRAW_BUFFER14: GLenum = 34867;
    pub const DRAW_BUFFER15: GLenum = 34868;
    pub const DRAW_BUFFER2: GLenum = 34855;
    pub const DRAW_BUFFER3: GLenum = 34856;
    pub const DRAW_BUFFER4: GLenum = 34857;
    pub const DRAW_BUFFER5: GLenum = 34858;
    pub const DRAW_BUFFER6: GLenum = 34859;
    pub const DRAW_BUFFER7: GLenum = 34860;
    pub const DRAW_BUFFER8: GLenum = 34861;
    pub const DRAW_BUFFER9: GLenum = 34862;
    pub const DRAW_FRAMEBUFFER: GLenum = 36009;
    pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_COPY: GLenum = 35050;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const DYNAMIC_READ: GLenum = 35049;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 36269;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT2X3: GLenum = 35685;
    pub const FLOAT_MAT2X4: GLenum = 35686;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT3X2: GLenum = 35687;
    pub const FLOAT_MAT3X4: GLenum = 35688;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_MAT4X2: GLenum = 35689;
    pub const FLOAT_MAT4X3: GLenum = 35690;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 35723;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 33301;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 33300;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 33296;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 33297;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 33302;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 33299;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 33298;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 33303;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 36052;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_DEFAULT: GLenum = 33304;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 36182;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HALF_FLOAT: GLenum = 5131;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INTERLEAVED_ATTRIBS: GLenum = 35980;
    pub const INT_2_10_10_10_REV: GLenum = 36255;
    pub const INT_SAMPLER_2D: GLenum = 36298;
    pub const INT_SAMPLER_2D_ARRAY: GLenum = 36303;
    pub const INT_SAMPLER_3D: GLenum = 36299;
    pub const INT_SAMPLER_CUBE: GLenum = 36300;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_INDEX: GLenum = 4294967295;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX: GLenum = 32776;
    pub const MAX_3D_TEXTURE_SIZE: GLenum = 32883;
    pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 35071;
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: GLenum = 37447;
    pub const MAX_COLOR_ATTACHMENTS: GLenum = 36063;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35379;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 35374;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 35377;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_DRAW_BUFFERS: GLenum = 34852;
    pub const MAX_ELEMENTS_INDICES: GLenum = 33001;
    pub const MAX_ELEMENTS_VERTICES: GLenum = 33000;
    pub const MAX_ELEMENT_INDEX: GLenum = 36203;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 37157;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 35373;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35657;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 35077;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_SAMPLES: GLenum = 36183;
    pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 37137;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_LOD_BIAS: GLenum = 34045;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 35978;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 35979;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 35968;
    pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 35376;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 35375;
    pub const MAX_VARYING_COMPONENTS: GLenum = 35659;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 37154;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 35371;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 35658;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIN: GLenum = 32775;
    pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 35076;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const OBJECT_TYPE: GLenum = 37138;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const PACK_ROW_LENGTH: GLenum = 3330;
    pub const PACK_SKIP_PIXELS: GLenum = 3332;
    pub const PACK_SKIP_ROWS: GLenum = 3331;
    pub const PIXEL_PACK_BUFFER: GLenum = 35051;
    pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 35053;
    pub const PIXEL_UNPACK_BUFFER: GLenum = 35052;
    pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 35055;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const QUERY_RESULT: GLenum = 34918;
    pub const QUERY_RESULT_AVAILABLE: GLenum = 34919;
    pub const R11F_G11F_B10F: GLenum = 35898;
    pub const R16F: GLenum = 33325;
    pub const R16I: GLenum = 33331;
    pub const R16UI: GLenum = 33332;
    pub const R32F: GLenum = 33326;
    pub const R32I: GLenum = 33333;
    pub const R32UI: GLenum = 33334;
    pub const R8: GLenum = 33321;
    pub const R8I: GLenum = 33329;
    pub const R8UI: GLenum = 33330;
    pub const R8_SNORM: GLenum = 36756;
    pub const RASTERIZER_DISCARD: GLenum = 35977;
    pub const READ_BUFFER: GLenum = 3074;
    pub const READ_FRAMEBUFFER: GLenum = 36008;
    pub const READ_FRAMEBUFFER_BINDING: GLenum = 36010;
    pub const RED: GLenum = 6403;
    pub const RED_BITS: GLenum = 3410;
    pub const RED_INTEGER: GLenum = 36244;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_SAMPLES: GLenum = 36011;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RG: GLenum = 33319;
    pub const RG16F: GLenum = 33327;
    pub const RG16I: GLenum = 33337;
    pub const RG16UI: GLenum = 33338;
    pub const RG32F: GLenum = 33328;
    pub const RG32I: GLenum = 33339;
    pub const RG32UI: GLenum = 33340;
    pub const RG8: GLenum = 33323;
    pub const RG8I: GLenum = 33335;
    pub const RG8UI: GLenum = 33336;
    pub const RG8_SNORM: GLenum = 36757;
    pub const RGB: GLenum = 6407;
    pub const RGB10_A2: GLenum = 32857;
    pub const RGB10_A2UI: GLenum = 36975;
    pub const RGB16F: GLenum = 34843;
    pub const RGB16I: GLenum = 36233;
    pub const RGB16UI: GLenum = 36215;
    pub const RGB32F: GLenum = 34837;
    pub const RGB32I: GLenum = 36227;
    pub const RGB32UI: GLenum = 36209;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGB8: GLenum = 32849;
    pub const RGB8I: GLenum = 36239;
    pub const RGB8UI: GLenum = 36221;
    pub const RGB8_SNORM: GLenum = 36758;
    pub const RGB9_E5: GLenum = 35901;
    pub const RGBA: GLenum = 6408;
    pub const RGBA16F: GLenum = 34842;
    pub const RGBA16I: GLenum = 36232;
    pub const RGBA16UI: GLenum = 36214;
    pub const RGBA32F: GLenum = 34836;
    pub const RGBA32I: GLenum = 36226;
    pub const RGBA32UI: GLenum = 36208;
    pub const RGBA4: GLenum = 32854;
    pub const RGBA8: GLenum = 32856;
    pub const RGBA8I: GLenum = 36238;
    pub const RGBA8UI: GLenum = 36220;
    pub const RGBA8_SNORM: GLenum = 36759;
    pub const RGBA_INTEGER: GLenum = 36249;
    pub const RGB_INTEGER: GLenum = 36248;
    pub const RG_INTEGER: GLenum = 33320;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_2D_ARRAY: GLenum = 36289;
    pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 36292;
    pub const SAMPLER_2D_SHADOW: GLenum = 35682;
    pub const SAMPLER_3D: GLenum = 35679;
    pub const SAMPLER_BINDING: GLenum = 35097;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLER_CUBE_SHADOW: GLenum = 36293;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SEPARATE_ATTRIBS: GLenum = 35981;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SIGNALED: GLenum = 37145;
    pub const SIGNED_NORMALIZED: GLenum = 36764;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const SRGB: GLenum = 35904;
    pub const SRGB8: GLenum = 35905;
    pub const SRGB8_ALPHA8: GLenum = 35907;
    pub const STATIC_COPY: GLenum = 35046;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STATIC_READ: GLenum = 35045;
    pub const STENCIL: GLenum = 6146;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_COPY: GLenum = 35042;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const STREAM_READ: GLenum = 35041;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const SYNC_CONDITION: GLenum = 37139;
    pub const SYNC_FENCE: GLenum = 37142;
    pub const SYNC_FLAGS: GLenum = 37141;
    pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 1;
    pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 37143;
    pub const SYNC_STATUS: GLenum = 37140;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_2D_ARRAY: GLenum = 35866;
    pub const TEXTURE_3D: GLenum = 32879;
    pub const TEXTURE_BASE_LEVEL: GLenum = 33084;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 35869;
    pub const TEXTURE_BINDING_3D: GLenum = 32874;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_COMPARE_FUNC: GLenum = 34893;
    pub const TEXTURE_COMPARE_MODE: GLenum = 34892;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 37167;
    pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 33503;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MAX_LEVEL: GLenum = 33085;
    pub const TEXTURE_MAX_LOD: GLenum = 33083;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_MIN_LOD: GLenum = 33082;
    pub const TEXTURE_WRAP_R: GLenum = 32882;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TIMEOUT_EXPIRED: GLenum = 37147;
    pub const TIMEOUT_IGNORED: GLint64 = -1;
    pub const TRANSFORM_FEEDBACK: GLenum = 36386;
    pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 36388;
    pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 36389;
    pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 35982;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 35983;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 35967;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 35973;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 35972;
    pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 36387;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 35976;
    pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 35971;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNIFORM_ARRAY_STRIDE: GLenum = 35388;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 35394;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 35395;
    pub const UNIFORM_BLOCK_BINDING: GLenum = 35391;
    pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 35392;
    pub const UNIFORM_BLOCK_INDEX: GLenum = 35386;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 35398;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 35396;
    pub const UNIFORM_BUFFER: GLenum = 35345;
    pub const UNIFORM_BUFFER_BINDING: GLenum = 35368;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 35380;
    pub const UNIFORM_BUFFER_SIZE: GLenum = 35370;
    pub const UNIFORM_BUFFER_START: GLenum = 35369;
    pub const UNIFORM_IS_ROW_MAJOR: GLenum = 35390;
    pub const UNIFORM_MATRIX_STRIDE: GLenum = 35389;
    pub const UNIFORM_OFFSET: GLenum = 35387;
    pub const UNIFORM_SIZE: GLenum = 35384;
    pub const UNIFORM_TYPE: GLenum = 35383;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_IMAGE_HEIGHT: GLenum = 32878;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNPACK_ROW_LENGTH: GLenum = 3314;
    pub const UNPACK_SKIP_IMAGES: GLenum = 32877;
    pub const UNPACK_SKIP_PIXELS: GLenum = 3316;
    pub const UNPACK_SKIP_ROWS: GLenum = 3315;
    pub const UNSIGNALED: GLenum = 37144;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 35899;
    pub const UNSIGNED_INT_24_8: GLenum = 34042;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 33640;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 35902;
    pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 36306;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 36311;
    pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 36307;
    pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 36308;
    pub const UNSIGNED_INT_VEC2: GLenum = 36294;
    pub const UNSIGNED_INT_VEC3: GLenum = 36295;
    pub const UNSIGNED_INT_VEC4: GLenum = 36296;
    pub const UNSIGNED_NORMALIZED: GLenum = 35863;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ARRAY_BINDING: GLenum = 34229;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 35070;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 35069;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const WAIT_FAILED: GLenum = 37149;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn begin_query(&self, target: GLenum, query: &WebGLQuery) {
        js!( @{self}.beginQuery(@{target}, @{query}); );
    }

    pub fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        js!( @{self}.beginTransformFeedback(@{primitive_mode}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBufferBase(@{target}, @{index}, @{buffer}); );
    }

    pub fn bind_buffer_range(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: Option<&WebGLBuffer>,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @{self}.bindBufferRange(@{target}, @{index}, @{buffer}, @{(offset as f64)}, @{(size as f64)}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_sampler(&self, unit: GLuint, sampler: Option<&WebGLSampler>) {
        js!( @{self}.bindSampler(@{unit}, @{sampler}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn bind_transform_feedback(&self, target: GLenum, tf: Option<&WebGLTransformFeedback>) {
        js!( @{self}.bindTransformFeedback(@{target}, @{tf}); );
    }

    pub fn bind_vertex_array(&self, array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.bindVertexArray(@{array}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        js!( @{self}.blitFramebuffer(@{src_x0}, @{src_y0}, @{src_x1}, @{src_y1}, @{dst_x0}, @{dst_y0}, @{dst_x1}, @{dst_y1}, @{mask}, @{filter}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, src_data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}); );
    }

    pub fn buffer_data_2(
        &self,
        target: GLenum,
        src_data: &ArrayBuffer,
        usage: GLenum,
        src_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}, @{src_offset}, @{length}); );
    }

    pub fn buffer_sub_data(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
    ) {
        js!( @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}); );
    }

    pub fn buffer_sub_data_1(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}, @{src_offset}, @{length}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @{self}.clear(@{mask}); );
    }

    pub fn clear_bufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        js!( @{self}.clearBufferfi(@{buffer}, @{drawbuffer}, @{depth}, @{stencil}); );
    }

    pub fn clear_bufferfv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.clearBufferfv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.clearBufferiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferuiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.clearBufferuiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @{self}.clearStencil(@{s}); );
    }

    pub fn client_wait_sync(
        &self,
        sync: &WebGLSync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        (js! { return @{self}.clientWaitSync(@{sync}, @{flags}, @{(timeout as f64)}); })
            .try_into()
            .unwrap()
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image2_d_1(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{data}); );
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image3_d_1(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image2_d_1(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{data}); );
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image3_d_1(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @{self}.copyBufferSubData(@{read_target}, @{write_target}, @{(read_offset as f64)}, @{(write_offset as f64)}, @{(size as f64)}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.copyTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_query(&self) -> Option<WebGLQuery> {
        (js! { return @{self}.createQuery(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_sampler(&self) -> Option<WebGLSampler> {
        (js! { return @{self}.createSampler(); }).try_into().ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn create_transform_feedback(&self) -> Option<WebGLTransformFeedback> {
        (js! { return @{self}.createTransformFeedback(); })
            .try_into()
            .ok()
    }

    pub fn create_vertex_array(&self) -> Option<WebGLVertexArrayObject> {
        (js! { return @{self}.createVertexArray(); })
            .try_into()
            .ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_query(&self, query: Option<&WebGLQuery>) {
        js!( @{self}.deleteQuery(@{query}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_sampler(&self, sampler: Option<&WebGLSampler>) {
        js!( @{self}.deleteSampler(@{sampler}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_sync(&self, sync: Option<&WebGLSync>) {
        js!( @{self}.deleteSync(@{sync}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @{self}.deleteTexture(@{texture}); );
    }

    pub fn delete_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) {
        js!( @{self}.deleteTransformFeedback(@{tf}); );
    }

    pub fn delete_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.deleteVertexArray(@{vertex_array}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instance_count: GLsizei,
    ) {
        js!( @{self}.drawArraysInstanced(@{mode}, @{first}, @{count}, @{instance_count}); );
    }

    pub fn draw_buffers(&self, buffers: &[GLenum]) {
        js!( @{self}.drawBuffers(@{buffers}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn draw_elements_instanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
        instance_count: GLsizei,
    ) {
        js!( @{self}.drawElementsInstanced(@{mode}, @{count}, @{type_}, @{(offset as f64)}, @{instance_count}); );
    }

    pub fn draw_range_elements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @{self}.drawRangeElements(@{mode}, @{start}, @{end}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn end_query(&self, target: GLenum) {
        js!( @{self}.endQuery(@{target}); );
    }

    pub fn end_transform_feedback(&self) {
        js!( @{self}.endTransformFeedback(); );
    }

    pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> Option<WebGLSync> {
        (js! { return @{self}.fenceSync(@{condition}, @{flags}); })
            .try_into()
            .ok()
    }

    pub fn finish(&self) {
        js!( @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
        layer: GLint,
    ) {
        js!( @{self}.framebufferTextureLayer(@{target}, @{attachment}, @{texture}, @{level}, @{layer}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
    ) -> Option<String> {
        (js! { return @{self}.getActiveUniformBlockName(@{program}, @{uniform_block_index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_parameter(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniformBlockParameter(@{program}, @{uniform_block_index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_active_uniforms(
        &self,
        program: &WebGLProgram,
        uniform_indices: &[GLuint],
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniforms(@{program}, @{uniform_indices}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_sub_data(
        &self,
        target: GLenum,
        src_byte_offset: GLintptr,
        dst_buffer: &ArrayBuffer,
        dst_offset: GLuint,
        length: GLuint,
    ) {
        js!( @{self}.getBufferSubData(@{target}, @{(src_byte_offset as f64)}, @{dst_buffer}, @{dst_offset}, @{length}); );
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension({E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getFragDataLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_indexed_parameter(&self, target: GLenum, index: GLuint) -> Value {
        (js! { return @{self}.getIndexedParameter(@{target}, @{index}); })
            .try_into()
            .unwrap()
    }

    pub fn get_internalformat_parameter(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getInternalformatParameter(@{target}, @{internalformat}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_query(&self, target: GLenum, pname: GLenum) -> Option<WebGLQuery> {
        (js! { return @{self}.getQuery(@{target}, @{pname}); })
            .try_into()
            .ok()
    }

    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: GLenum) -> Value {
        (js! { return @{self}.getQueryParameter(@{query}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: GLenum) -> Value {
        (js! { return @{self}.getSamplerParameter(@{sampler}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: GLenum) -> Value {
        (js! { return @{self}.getSyncParameter(@{sync}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getTransformFeedbackVarying(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_block_index(
        &self,
        program: &WebGLProgram,
        uniform_block_name: &str,
    ) -> GLuint {
        (js! { return @{self}.getUniformBlockIndex(@{program}, @{uniform_block_name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_indices(
        &self,
        program: &WebGLProgram,
        uniform_names: &[&str],
    ) -> Option<Vec<GLuint>> {
        (js! { return @{self}.getUniformIndices(@{program}, @{uniform_names}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @{self}.hint(@{target}, @{mode}); );
    }

    pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
        js!( @{self}.invalidateFramebuffer(@{target}, @{attachments}); );
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: GLenum,
        attachments: &[GLenum],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.invalidateSubFramebuffer(@{target}, @{attachments}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_query(&self, query: Option<&WebGLQuery>) -> GLboolean {
        (js! { return @{self}.isQuery(@{query}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sampler(&self, sampler: Option<&WebGLSampler>) -> GLboolean {
        (js! { return @{self}.isSampler(@{sampler}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sync(&self, sync: Option<&WebGLSync>) -> GLboolean {
        (js! { return @{self}.isSync(@{sync}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn is_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) -> GLboolean {
        (js! { return @{self}.isTransformFeedback(@{tf}); })
            .try_into()
            .unwrap()
    }

    pub fn is_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) -> GLboolean {
        (js! { return @{self}.isVertexArray(@{vertex_array}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @{self}.linkProgram(@{program}); );
    }

    pub fn pause_transform_feedback(&self) {
        js!( @{self}.pauseTransformFeedback(); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_buffer(&self, src: GLenum) {
        js!( @{self}.readBuffer(@{src}); );
    }

    pub fn read_pixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}); );
    }

    pub fn read_pixels_1(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{(offset as f64)}); );
    }

    pub fn read_pixels_2(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: &ArrayBuffer,
        dst_offset: GLuint,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}, @{dst_offset}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.renderbufferStorageMultisample(@{target}, @{samples}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn resume_transform_feedback(&self) {
        js!( @{self}.resumeTransformFeedback(); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn sampler_parameterf(&self, sampler: &WebGLSampler, pname: GLenum, param: GLfloat) {
        js!( @{self}.samplerParameterf(@{sampler}, @{pname}, @{param}); );
    }

    pub fn sampler_parameteri(&self, sampler: &WebGLSampler, pname: GLenum, param: GLint) {
        js!( @{self}.samplerParameteri(@{sampler}, @{pname}, @{param}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_4(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image3_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}); );
    }

    pub fn tex_image3_d_3(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_storage2_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.texStorage2D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn tex_storage3_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        js!( @{self}.texStorage3D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}, @{depth}); );
    }

    pub fn tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_4(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: &ArrayBuffer,
        src_offset: GLuint,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image3_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: Option<&ArrayBuffer>,
        src_offset: GLuint,
    ) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn transform_feedback_varyings(
        &self,
        program: &WebGLProgram,
        varyings: &[&str],
        buffer_mode: GLenum,
    ) {
        js!( @{self}.transformFeedbackVaryings(@{program}, @{varyings}, @{buffer_mode}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint) {
        js!( @{self}.uniform1ui(@{location}, @{v0}); );
    }

    pub fn uniform1uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform1uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint) {
        js!( @{self}.uniform2ui(@{location}, @{v0}, @{v1}); );
    }

    pub fn uniform2uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform2uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        js!( @{self}.uniform3ui(@{location}, @{v0}, @{v1}, @{v2}); );
    }

    pub fn uniform3uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform3uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        js!( @{self}.uniform4ui(@{location}, @{v0}, @{v1}, @{v2}, @{v3}); );
    }

    pub fn uniform4uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.uniform4uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_block_binding(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        js!( @{self}.uniformBlockBinding(@{program}, @{uniform_block_index}, @{uniform_block_binding}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix2x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        js!( @{self}.vertexAttribDivisor(@{index}, @{divisor}); );
    }

    pub fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @{self}.vertexAttribI4i(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4iv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.vertexAttribI4iv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        js!( @{self}.vertexAttribI4ui(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4uiv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @{self}.vertexAttribI4uiv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.vertexAttribIPointer(@{index}, @{size}, @{type_}, @{stride}, @{(offset as f64)}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLint64) {
        js!( @{self}.waitSync(@{sync}, @{flags}, @{(timeout as f64)}); );
    }
}

impl RenderingContext for WebGL2RenderingContext {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("webgl2");
        )
        .try_into()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLActiveInfo")]
pub struct WebGLActiveInfo(Reference);

impl WebGLActiveInfo {
    pub fn name(&self) -> String {
        (js! { return @{self}.name; }).try_into().unwrap()
    }

    pub fn size(&self) -> GLint {
        (js! { return @{self}.size; }).try_into().unwrap()
    }

    pub fn type_(&self) -> GLenum {
        (js! { return @{self}.type; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLBuffer")]
pub struct WebGLBuffer(Reference);

impl WebGLBuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLContextEvent")]
pub struct WebGLContextEvent(Reference);

impl WebGLContextEvent {
    pub fn status_message(&self) -> String {
        (js! { return @{self}.statusMessage; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLFramebuffer")]
pub struct WebGLFramebuffer(Reference);

impl WebGLFramebuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLProgram")]
pub struct WebGLProgram(Reference);

impl WebGLProgram {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLQuery")]
pub struct WebGLQuery(Reference);

impl WebGLQuery {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLRenderbuffer")]
pub struct WebGLRenderbuffer(Reference);

impl WebGLRenderbuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLRenderingContext")]
pub struct WebGLRenderingContext(Reference);

impl WebGLRenderingContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALWAYS: GLenum = 519;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const RED_BITS: GLenum = 3410;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RGB: GLenum = 6407;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGBA: GLenum = 6408;
    pub const RGBA4: GLenum = 32854;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{data}, @{usage}); );
    }

    pub fn buffer_sub_data(&self, target: GLenum, offset: GLintptr, data: &ArrayBuffer) {
        js!( @{self}.bufferSubData(@{target}, @{(offset as f64)}, @{data}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @{self}.clear(@{mask}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @{self}.clearStencil(@{s}); );
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{data}); );
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: &ArrayBuffer,
    ) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{data}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @{self}.deleteTexture(@{texture}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn finish(&self) {
        js!( @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension({E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @{self}.hint(@{target}, @{mode}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @{self}.linkProgram(@{program}); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_pixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<&ArrayBuffer>,
    ) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }
}

impl RenderingContext for WebGLRenderingContext {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("webgl");
        )
        .try_into()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLSampler")]
pub struct WebGLSampler(Reference);

impl WebGLSampler {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLShader")]
pub struct WebGLShader(Reference);

impl WebGLShader {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLShaderPrecisionFormat")]
pub struct WebGLShaderPrecisionFormat(Reference);

impl WebGLShaderPrecisionFormat {
    pub fn precision(&self) -> GLint {
        (js! { return @{self}.precision; }).try_into().unwrap()
    }

    pub fn range_max(&self) -> GLint {
        (js! { return @{self}.rangeMax; }).try_into().unwrap()
    }

    pub fn range_min(&self) -> GLint {
        (js! { return @{self}.rangeMin; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLSync")]
pub struct WebGLSync(Reference);

impl WebGLSync {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLTexture")]
pub struct WebGLTexture(Reference);

impl WebGLTexture {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLTransformFeedback")]
pub struct WebGLTransformFeedback(Reference);

impl WebGLTransformFeedback {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLUniformLocation")]
pub struct WebGLUniformLocation(Reference);

impl WebGLUniformLocation {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLVertexArrayObject")]
pub struct WebGLVertexArrayObject(Reference);

impl WebGLVertexArrayObject {}
