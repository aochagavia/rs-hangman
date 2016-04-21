// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct GuessResult {
    // message fields
    correct: ::std::option::Option<bool>,
    indices: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GuessResult {}

impl GuessResult {
    pub fn new() -> GuessResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GuessResult {
        static mut instance: ::protobuf::lazy::Lazy<GuessResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GuessResult,
        };
        unsafe {
            instance.get(|| {
                GuessResult {
                    correct: ::std::option::Option::None,
                    indices: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool correct = 1;

    pub fn clear_correct(&mut self) {
        self.correct = ::std::option::Option::None;
    }

    pub fn has_correct(&self) -> bool {
        self.correct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_correct(&mut self, v: bool) {
        self.correct = ::std::option::Option::Some(v);
    }

    pub fn get_correct<'a>(&self) -> bool {
        self.correct.unwrap_or(false)
    }

    // required bytes indices = 2;

    pub fn clear_indices(&mut self) {
        self.indices.clear();
    }

    pub fn has_indices(&self) -> bool {
        self.indices.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indices(&mut self, v: ::std::vec::Vec<u8>) {
        self.indices = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indices<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.indices.is_none() {
            self.indices.set_default();
        };
        self.indices.as_mut().unwrap()
    }

    // Take field
    pub fn take_indices(&mut self) -> ::std::vec::Vec<u8> {
        self.indices.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_indices<'a>(&'a self) -> &'a [u8] {
        match self.indices.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for GuessResult {
    fn is_initialized(&self) -> bool {
        if self.correct.is_none() {
            return false;
        };
        if self.indices.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.correct = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.indices));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.correct.is_some() {
            my_size += 2;
        };
        for value in self.indices.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.correct {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.indices.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GuessResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GuessResult {
    fn new() -> GuessResult {
        GuessResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<GuessResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "correct",
                    GuessResult::has_correct,
                    GuessResult::get_correct,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "indices",
                    GuessResult::has_indices,
                    GuessResult::get_indices,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GuessResult>(
                    "GuessResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GuessResult {
    fn clear(&mut self) {
        self.clear_correct();
        self.clear_indices();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GuessResult {
    fn eq(&self, other: &GuessResult) -> bool {
        self.correct == other.correct &&
        self.indices == other.indices &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GuessResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x67, 0x75, 0x65, 0x73, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x41, 0x0a, 0x0b, 0x47, 0x75, 0x65, 0x73, 0x73, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x08, 0x52, 0x07, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x12, 0x18, 0x0a,
    0x07, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x07,
    0x69, 0x6e, 0x64, 0x69, 0x63, 0x65, 0x73, 0x4a, 0xaa, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x03, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x03, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x01, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x01, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x01, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x02, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x02, 0x1d, 0x1e,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
