use std::fmt::Write;

/// A trait used for pretty printing protobuf message.
pub trait PbPrint {
    fn fmt(&self, name: &str, buf: &mut String);
}

pub fn escape(data: &[u8], buf: &mut String) {
    buf.reserve(data.len() + 2);
    buf.push('"');
    for &c in data {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'"' => buf.push_str("\\\""),
            b'\\' => buf.push_str(r"\\"),
            _ => {
                if c >= 0x20 && c < 0x7f {
                    // c is printable
                    buf.push(c as char);
                } else {
                    buf.push('\\');
                    buf.push((b'0' + (c >> 6)) as char);
                    buf.push((b'0' + ((c >> 3) & 7)) as char);
                    buf.push((b'0' + (c & 7)) as char);
                }
            }
        }
    }
    buf.push('"');
}

pub fn hex_escape(data: &[u8], buf: &mut String) {
    hex::ToHex::write_hex_upper(&data, buf).unwrap();
}

#[inline]
pub fn push_start(name: &str, buf: &mut String) {
    if !buf.is_empty() {
        buf.push(' ');
    }
    buf.push_str(name);
}

#[inline]
pub fn push_message_start(name: &str, buf: &mut String) {
    push_start(name, buf);
    buf.push_str(" {");
}

#[inline]
pub fn push_field_start(name: &str, buf: &mut String) {
    push_start(name, buf);
    buf.push_str(": ");
}

impl<T: PbPrint> PbPrint for Option<T> {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        match self.as_ref() {
            None => return,
            Some(v) => v.fmt(name, buf),
        }
    }
}

impl PbPrint for String {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        if self.is_empty() {
            return;
        }
        push_field_start(name, buf);
        escape(self.as_bytes(), buf);
    }
}

impl PbPrint for Vec<u8> {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        if self.is_empty() {
            return;
        }
        push_field_start(name, buf);
        hex_escape(self, buf);
    }
}

impl<T: PbPrint> PbPrint for Vec<T> {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        if self.is_empty() {
            return;
        }
        for v in self {
            v.fmt(name, buf);
        }
    }
}

macro_rules! print_number {
    ($t:ty, $zero:expr) => {
        impl PbPrint for $t {
            #[inline]
            fn fmt(&self, name: &str, buf: &mut String) {
                if *self == $zero {
                    return;
                }
                push_field_start(name, buf);
                write!(buf, "{}", self).unwrap();
            }
        }
    };
}

print_number!(i32, 0);
print_number!(i64, 0);
print_number!(u32, 0);
print_number!(u64, 0);
print_number!(f32, 0.0);
print_number!(f64, 0.0);

impl PbPrint for bool {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        if !*self {
            return;
        }
        push_field_start(name, buf);
        buf.push_str("true");
    }
}

#[cfg(feature = "protobuf-codec")]
mod protobuf_impl {
    use super::PbPrint;
    use protobuf::{RepeatedField, SingularPtrField};

    impl<T: PbPrint> PbPrint for RepeatedField<T> {
        fn fmt(&self, name: &str, buf: &mut String) {
            if self.is_empty() {
                return;
            }
            for v in self {
                v.fmt(name, buf);
            }
        }
    }

    impl<T: PbPrint> PbPrint for SingularPtrField<T> {
        #[inline]
        fn fmt(&self, name: &str, buf: &mut String) {
            match self.as_ref() {
                None => return,
                Some(v) => v.fmt(name, buf),
            }
        }
    }
}

macro_rules! debug_to_pb_print {
    ($t:ty) => {
        impl PbPrint for $t {
            #[inline]
            fn fmt(&self, name: &str, buf: &mut String) {
                push_message_start(name, buf);
                let old_len = buf.len();
                write!(buf, "{:?}", self).unwrap();
                if buf.len() > old_len {
                    buf.push_str(" }");
                } else {
                    buf.push('}');
                }
            }
        }
    };
}

debug_to_pb_print!(raft::eraftpb::Entry);
debug_to_pb_print!(raft::eraftpb::Message);
debug_to_pb_print!(raft::eraftpb::HardState);
debug_to_pb_print!(raft::eraftpb::ConfState);
debug_to_pb_print!(raft::eraftpb::ConfChange);

impl PbPrint for raft::eraftpb::ConfChangeType {
    #[inline]
    fn fmt(&self, name: &str, buf: &mut String) {
        if *self == raft::eraftpb::ConfChangeType::default() {
            return;
        }
        push_field_start(name, buf);
        write!(buf, "{:?}", self).unwrap();
    }
}
