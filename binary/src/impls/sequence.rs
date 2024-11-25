use std::mem::MaybeUninit;
use bytes::Buf;
use crate::{generate, Decode, Encode, Prefix, Reader, VarU32, Writer};

generate!(Array, <P: Prefix, T: Encode>, Vec<T>);

impl<P: Prefix, T: Encode> Encode for Array<P, T> {
    fn encode(&self, w: &mut Writer) {
        P::from(self.len()).encode(w);

        for item in &self.val {
            item.encode(w);
        }
    }
}

impl<P: Prefix, T: Encode + for<'a> Decode<'a>> Decode<'_> for Array<P, T> {
    fn decode(r: &mut Reader) -> Option<Self> {
        let len = P::decode(r)?.into();
        if r.remaining() < len {
            return None;
        }

        let data: Vec<T> = (0..len).map(|_| T::decode(r)).collect::<Option<_>>()?;
        Some(Array::new(data))
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode(&self, w: &mut Writer) {
        for item in self {
            item.encode(w);
        }
    }
}

impl<'a, T: Decode<'a>, const N: usize> Decode<'a> for [T; N] {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, elem) in data.iter_mut().enumerate() {
            match T::decode(r) {
                Some(val) => {
                    elem.write(val);
                }
                None => {
                    for elem in &mut data[..i] {
                        unsafe { elem.assume_init_drop() };
                    }
                    return None;
                }
            }
        }

        unsafe { Some(std::mem::transmute_copy(&data)) }
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, w: &mut Writer) {
        VarU32::from(self.len()).encode(w);

        for item in self {
            item.encode(w);
        }
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, w: &mut Writer) {
        self.as_slice().encode(w);
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let len: usize = VarU32::decode(r)?.into();
        let data = (0..len).map(|_| T::decode(r)).collect::<Option<_>>()?;

        Some(data)
    }
}