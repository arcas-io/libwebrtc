use log::error;

use crate::primitive_to_cxx_enum;

use self::ffi::{
    IceAttributeType, StunAddressFamily, StunAttributeType, StunAttributeValueType, StunErrorCode,
    StunMessageType,
};

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(i32)]
    enum StunMessageType {
        STUN_BINDING_REQUEST = 0x0001,
        STUN_BINDING_INDICATION = 0x0011,
        STUN_BINDING_RESPONSE = 0x0101,
        STUN_BINDING_ERROR_RESPONSE = 0x0111,

        // Method 0x80, GOOG-PING is a variant of STUN BINDING
        // that is sent instead of a STUN BINDING if the binding
        // was identical to the one before.
        GOOG_PING_REQUEST = 0x200,
        GOOG_PING_RESPONSE = 0x300,
        GOOG_PING_ERROR_RESPONSE = 0x310,
    }

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(u32)]
    enum StunAttributeType {
        STUN_ATTR_MAPPED_ADDRESS = 0x0001,     // Address
        STUN_ATTR_USERNAME = 0x0006,           // ByteString
        STUN_ATTR_MESSAGE_INTEGRITY = 0x0008,  // ByteString, 20 bytes
        STUN_ATTR_ERROR_CODE = 0x0009,         // ErrorCode
        STUN_ATTR_UNKNOWN_ATTRIBUTES = 0x000a, // UInt16List
        STUN_ATTR_REALM = 0x0014,              // ByteString
        STUN_ATTR_NONCE = 0x0015,              // ByteString
        STUN_ATTR_XOR_MAPPED_ADDRESS = 0x0020, // XorAddress
        STUN_ATTR_SOFTWARE = 0x8022,           // ByteString
        STUN_ATTR_ALTERNATE_SERVER = 0x8023,   // Address
        STUN_ATTR_FINGERPRINT = 0x8028,        // UInt32
        STUN_ATTR_ORIGIN = 0x802F,             // ByteString
        STUN_ATTR_RETRANSMIT_COUNT = 0xFF00,   // UInt32
    }

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(u32)]
    enum StunAttributeValueType {
        STUN_VALUE_UNKNOWN = 0,
        STUN_VALUE_ADDRESS = 1,
        STUN_VALUE_XOR_ADDRESS = 2,
        STUN_VALUE_UINT32 = 3,
        STUN_VALUE_UINT64 = 4,
        STUN_VALUE_BYTE_STRING = 5,
        STUN_VALUE_ERROR_CODE = 6,
        STUN_VALUE_UINT16_LIST = 7,
    }

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(u32)]
    enum StunErrorCode {
        STUN_ERROR_TRY_ALTERNATE = 300,
        STUN_ERROR_BAD_REQUEST = 400,
        STUN_ERROR_UNAUTHORIZED = 401,
        STUN_ERROR_UNKNOWN_ATTRIBUTE = 420,
        STUN_ERROR_STALE_NONCE = 438,
        STUN_ERROR_SERVER_ERROR = 500,
        STUN_ERROR_GLOBAL_FAILURE = 600,
    }

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(u32)]
    enum StunAddressFamily {
        // NB: UNDEF is not part of the STUN spec.
        STUN_ADDRESS_UNDEF = 0,
        STUN_ADDRESS_IPV4 = 1,
        STUN_ADDRESS_IPV6 = 2,
    }

    #[derive(Debug, PartialEq)]
    #[namespace = "cricket"]
    #[repr(u32)]
    enum IceAttributeType {
        // RFC 5245 ICE STUN attributes.
        STUN_ATTR_PRIORITY = 0x0024,        // UInt32
        STUN_ATTR_USE_CANDIDATE = 0x0025,   // No content, Length = 0
        STUN_ATTR_ICE_CONTROLLED = 0x8029,  // UInt64
        STUN_ATTR_ICE_CONTROLLING = 0x802A, // UInt64
        // The following attributes are in the comprehension-optional range
        // (0xC000-0xFFFF) and are not registered with IANA. These STUN attributes are
        // intended for ICE and should NOT be used in generic use cases of STUN
        // messages.
        //
        // Note that the value 0xC001 has already been assigned by IANA to
        // ENF-FLOW-DESCRIPTION
        // (https://www.iana.org/assignments/stun-parameters/stun-parameters.xml).
        STUN_ATTR_NOMINATION = 0xC001, // UInt32
        // UInt32. The higher 16 bits are the network ID. The lower 16 bits are the
        // network cost.
        STUN_ATTR_GOOG_NETWORK_INFO = 0xC057,
        // Experimental: Transaction ID of the last connectivity check received.
        STUN_ATTR_GOOG_LAST_ICE_CHECK_RECEIVED = 0xC058,
        // Uint16List. Miscellaneous attributes for future extension.
        STUN_ATTR_GOOG_MISC_INFO = 0xC059,
        // Obsolete.
        STUN_ATTR_GOOG_OBSOLETE_1 = 0xC05A,
        STUN_ATTR_GOOG_CONNECTION_ID = 0xC05B, // Not yet implemented.
        STUN_ATTR_GOOG_DELTA = 0xC05C,         // Not yet implemented.
        STUN_ATTR_GOOG_DELTA_ACK = 0xC05D,     // Not yet implemented.
        // MESSAGE-INTEGRITY truncated to 32-bit.
        STUN_ATTR_GOOG_MESSAGE_INTEGRITY_32 = 0xC060,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxIntegrityStatus {
        kNotSet,
        kNoIntegrity,  // Message-integrity attribute missing
        kIntegrityOk,  // Message-integrity checked OK
        kIntegrityBad, // Message-integrity verification failed
    }

    unsafe extern "C++" {
        include!("include/stun.h");

        type ArcasCxxIntegrityStatus;
        #[namespace = "cricket"]
        type StunMessageType;
        #[namespace = "cricket"]
        type StunAttributeType;
        #[namespace = "cricket"]
        type StunAttributeValueType;
        #[namespace = "cricket"]
        type StunErrorCode;
        #[namespace = "cricket"]
        type StunAddressFamily;

        #[namespace = "cricket"]
        type StunMessage;
        #[namespace = "cricket"]
        type IceAttributeType;
        type ArcasICEMessage;
        #[namespace = "cricket"]
        type StunAttribute;

        #[namespace = "cricket"]
        type StunAddressAttribute;
        #[namespace = "cricket"]
        type StunUInt32Attribute;
        #[namespace = "cricket"]
        type StunUInt64Attribute;
        #[namespace = "cricket"]
        type StunByteStringAttribute;
        #[namespace = "cricket"]
        type StunUInt16ListAttribute;
        #[namespace = "cricket"]
        type StunErrorCodeAttribute;
        #[namespace = "rtc"]
        type ByteBufferReader = crate::rtc_base::base::ffi::ByteBufferReader;

        // StunMessage
        fn create_arcas_ice_message() -> UniquePtr<ArcasICEMessage>;

        fn get_type(self: &ArcasICEMessage) -> i32;
        #[cxx_name = "SetType"]
        fn set_type(self: Pin<&mut ArcasICEMessage>, type_: i32);
        fn set_transaction_id(self: Pin<&mut ArcasICEMessage>, transaction_id: &CxxString);
        fn length(self: &ArcasICEMessage) -> usize;
        fn transaction_id(self: &ArcasICEMessage) -> &CxxString;
        fn reduced_transaction_id(self: &ArcasICEMessage) -> u32;
        fn unknown_attributes(self: &ArcasICEMessage) -> UniquePtr<CxxVector<u16>>;
        #[cxx_name = "GetAddress"]
        fn get_address(self: &ArcasICEMessage, attr_type: i32) -> *const StunAddressAttribute;
        #[cxx_name = "GetUInt32"]
        fn get_uint32(self: &ArcasICEMessage, attr_type: i32) -> *const StunUInt32Attribute;
        #[cxx_name = "GetUInt64"]
        fn get_uint64(self: &ArcasICEMessage, attr_type: i32) -> *const StunUInt64Attribute;
        #[cxx_name = "GetByteString"]
        fn get_byte_string(
            self: &ArcasICEMessage,
            attr_type: i32,
        ) -> *const StunByteStringAttribute;
        #[cxx_name = "GetUInt16List"]
        fn get_uint16_list(
            self: &ArcasICEMessage,
            attr_type: i32,
        ) -> *const StunUInt16ListAttribute;
        #[cxx_name = "GetErrorCode"]
        fn get_error_code(self: &ArcasICEMessage) -> *const StunErrorCodeAttribute;
        #[cxx_name = "GetErrorCodeValue"]
        fn get_error_code_value(self: &ArcasICEMessage) -> i32;
        #[cxx_name = "AddAttribute"]
        fn add_attribute(self: Pin<&mut ArcasICEMessage>, attr: UniquePtr<StunAttribute>);
        #[cxx_name = "RemoveAttribute"]
        fn remove_attribute(
            self: Pin<&mut ArcasICEMessage>,
            attr_type: i32,
        ) -> UniquePtr<StunAttribute>;
        #[cxx_name = "ClearAttributes"]
        fn clear_attributes(self: Pin<&mut ArcasICEMessage>);
        #[cxx_name = "ValidateMessageIntegrity"]
        fn validate_message_integrity(
            self: Pin<&mut ArcasICEMessage>,
            password: &CxxString,
        ) -> ArcasCxxIntegrityStatus;
        fn integrity(self: &ArcasICEMessage) -> ArcasCxxIntegrityStatus;
        #[cxx_name = "IntegrityOk"]
        fn integrity_ok(self: &ArcasICEMessage) -> bool;
        #[cxx_name = "rust_password"]
        fn password(self: &ArcasICEMessage) -> String;
        #[cxx_name = "rust_add_message_integrity"]
        fn add_message_integrity(self: Pin<&mut ArcasICEMessage>, password: String) -> bool;
        #[cxx_name = "rust_add_message_integrity32"]
        fn add_message_integrity32(self: Pin<&mut ArcasICEMessage>, password: String) -> bool;
        #[cxx_name = "AddFingerprint"]
        fn add_fingerprint(self: Pin<&mut ArcasICEMessage>) -> bool;
        #[cxx_name = "Read"]
        unsafe fn read(self: Pin<&mut ArcasICEMessage>, buffer: *mut ByteBufferReader) -> bool;
        #[cxx_name = "SetStunMagicCookie"]
        fn set_stun_magic_cookie(self: Pin<&mut ArcasICEMessage>, magic_cookie: u32);
    }
}

primitive_to_cxx_enum!(
    StunMessageType,
    i32,
    STUN_BINDING_REQUEST,
    STUN_BINDING_INDICATION,
    STUN_BINDING_RESPONSE,
    STUN_BINDING_ERROR_RESPONSE,
    GOOG_PING_REQUEST,
    GOOG_PING_RESPONSE,
    GOOG_PING_ERROR_RESPONSE
);

primitive_to_cxx_enum!(
    StunAttributeType,
    u32,
    STUN_ATTR_MAPPED_ADDRESS,
    STUN_ATTR_USERNAME,
    STUN_ATTR_MESSAGE_INTEGRITY,
    STUN_ATTR_ERROR_CODE,
    STUN_ATTR_UNKNOWN_ATTRIBUTES,
    STUN_ATTR_REALM,
    STUN_ATTR_NONCE,
    STUN_ATTR_XOR_MAPPED_ADDRESS,
    STUN_ATTR_SOFTWARE,
    STUN_ATTR_ALTERNATE_SERVER,
    STUN_ATTR_FINGERPRINT,
    STUN_ATTR_ORIGIN,
    STUN_ATTR_RETRANSMIT_COUNT
);

primitive_to_cxx_enum!(
    StunAttributeValueType,
    u32,
    STUN_VALUE_UNKNOWN,
    STUN_VALUE_ADDRESS,
    STUN_VALUE_XOR_ADDRESS,
    STUN_VALUE_UINT32,
    STUN_VALUE_UINT64,
    STUN_VALUE_BYTE_STRING,
    STUN_VALUE_ERROR_CODE,
    STUN_VALUE_UINT16_LIST
);

primitive_to_cxx_enum!(
    StunErrorCode,
    u32,
    STUN_ERROR_TRY_ALTERNATE,
    STUN_ERROR_BAD_REQUEST,
    STUN_ERROR_UNAUTHORIZED,
    STUN_ERROR_UNKNOWN_ATTRIBUTE,
    STUN_ERROR_STALE_NONCE,
    STUN_ERROR_SERVER_ERROR,
    STUN_ERROR_GLOBAL_FAILURE
);

primitive_to_cxx_enum!(
    StunAddressFamily,
    u32,
    STUN_ADDRESS_IPV4,
    STUN_ADDRESS_IPV6,
    STUN_ADDRESS_UNDEF
);

primitive_to_cxx_enum!(
    IceAttributeType,
    u32,
    STUN_ATTR_PRIORITY,
    STUN_ATTR_USE_CANDIDATE,
    STUN_ATTR_ICE_CONTROLLED,
    STUN_ATTR_ICE_CONTROLLING,
    STUN_ATTR_NOMINATION,
    STUN_ATTR_GOOG_NETWORK_INFO,
    STUN_ATTR_GOOG_LAST_ICE_CHECK_RECEIVED,
    STUN_ATTR_GOOG_MISC_INFO,
    STUN_ATTR_GOOG_OBSOLETE_1,
    STUN_ATTR_GOOG_CONNECTION_ID,
    STUN_ATTR_GOOG_DELTA,
    STUN_ATTR_GOOG_DELTA_ACK,
    STUN_ATTR_GOOG_MESSAGE_INTEGRITY_32
);

#[cfg(test)]
pub mod tests {
    use std::os::raw::c_char;

    use crate::{
        rtc_base::base::ffi::create_arcas_cxx_byte_buffer_reader, stun::ffi::StunMessageType,
    };

    use super::ffi::create_arcas_ice_message;

    #[test]
    fn it_should_create_stun_message() {
        let mut stun = create_arcas_ice_message();
        let unknown_attrs = stun.unknown_attributes();
        assert_eq!(unknown_attrs.len(), 0);

        let get_type = stun.get_type();
        assert_eq!(get_type, 0);

        // base64 encoded message... taken from the pion test browser examples.
        let test_stun_messsage = "AAEAGCESpEJQVkdXVHBjbjhBWlWALwARaHR0cHM6Ly9jeWRldi5ydS8AAAA=";
        let decoded = base64::decode(test_stun_messsage).unwrap();
        let decoded_ptr = decoded.as_ptr() as *const i8;
        let mut reader = unsafe { create_arcas_cxx_byte_buffer_reader(decoded_ptr, decoded.len()) };
        assert_eq!(reader.len(), decoded.len());

        unsafe {
            stun.pin_mut().read(reader.pin_mut().get_unchecked_mut());
        }

        assert_eq!(reader.len(), 0, "after .read the reader is empty");
        let get_type = stun.get_type();
        assert_eq!(get_type, StunMessageType::STUN_BINDING_REQUEST.repr)
    }
}
