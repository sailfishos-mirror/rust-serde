#![feature(custom_derive, plugin, test)]
#![plugin(serde2_macros)]

extern crate serde2;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;

use test::Bencher;

use rustc_serialize::{Decoder, Decodable};

use serde2::de::{Deserializer, Deserialize};

use Animal::{Dog, Frog};

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Debug, RustcDecodable)]
#[derive_deserialize]
enum Animal {
    Dog,
    Frog(String, isize)
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Error {
    EndOfStreamError,
    SyntaxError,
}

impl serde2::de::Error for Error {
    fn syntax_error() -> Error { Error::SyntaxError }

    fn end_of_stream_error() -> Error { Error::EndOfStreamError }

    fn missing_field_error(_: &'static str) -> Error { Error::SyntaxError }
}

//////////////////////////////////////////////////////////////////////////////

mod decoder {
    use rustc_serialize::Decoder;

    use super::{Animal, Error};
    use super::Animal::{Dog, Frog};
    use self::State::{AnimalState, IsizeState, StringState};

    enum State {
        AnimalState(Animal),
        IsizeState(isize),
        StringState(String),
    }

    pub struct AnimalDecoder {
        stack: Vec<State>,

    }

    impl AnimalDecoder {
        #[inline]
        pub fn new(animal: Animal) -> AnimalDecoder {
            AnimalDecoder {
                stack: vec!(AnimalState(animal)),
            }
        }
    }

    impl Decoder for AnimalDecoder {
        type Error = Error;

        fn error(&mut self, _: &str) -> Error { Error::SyntaxError }

        // Primitive types:
        fn read_nil(&mut self) -> Result<(), Error> { Err(Error::SyntaxError) }
        fn read_usize(&mut self) -> Result<usize, Error> { Err(Error::SyntaxError) }
        fn read_u64(&mut self) -> Result<u64, Error> { Err(Error::SyntaxError) }
        fn read_u32(&mut self) -> Result<u32, Error> { Err(Error::SyntaxError) }
        fn read_u16(&mut self) -> Result<u16, Error> { Err(Error::SyntaxError) }
        fn read_u8(&mut self) -> Result<u8, Error> { Err(Error::SyntaxError) }
        #[inline]
        fn read_isize(&mut self) -> Result<isize, Error> {
            match self.stack.pop() {
                Some(IsizeState(x)) => Ok(x),
                _ => Err(Error::SyntaxError),
            }
        }
        fn read_i64(&mut self) -> Result<i64, Error> { Err(Error::SyntaxError) }
        fn read_i32(&mut self) -> Result<i32, Error> { Err(Error::SyntaxError) }
        fn read_i16(&mut self) -> Result<i16, Error> { Err(Error::SyntaxError) }
        fn read_i8(&mut self) -> Result<i8, Error> { Err(Error::SyntaxError) }
        fn read_bool(&mut self) -> Result<bool, Error> { Err(Error::SyntaxError) }
        fn read_f64(&mut self) -> Result<f64, Error> { Err(Error::SyntaxError) }
        fn read_f32(&mut self) -> Result<f32, Error> { Err(Error::SyntaxError) }
        fn read_char(&mut self) -> Result<char, Error> { Err(Error::SyntaxError) }
        #[inline]
        fn read_str(&mut self) -> Result<String, Error> {
            match self.stack.pop() {
                Some(StringState(x)) => Ok(x),
                _ => Err(Error::SyntaxError),
            }
        }

        // Compound types:
        #[inline]
        fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            match self.stack.pop() {
                Some(AnimalState(animal)) => {
                    self.stack.push(AnimalState(animal));
                    if name == "Animal" {
                        f(self)
                    } else {
                        Err(Error::SyntaxError)
                    }
                }
                _ => Err(Error::SyntaxError)
            }
        }

        #[inline]
        fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder, usize) -> Result<T, Error>,
        {
            let name = match self.stack.pop() {
                Some(AnimalState(Dog)) => "Dog",
                Some(AnimalState(Frog(x0, x1))) => {
                    self.stack.push(IsizeState(x1));
                    self.stack.push(StringState(x0));
                    "Frog"
                }
                _ => { return Err(Error::SyntaxError); }
            };

            let idx = match names.iter().position(|n| *n == name) {
                Some(idx) => idx,
                None => { return Err(Error::SyntaxError); }
            };

            f(self, idx)
        }

        #[inline]
        fn read_enum_variant_arg<T, F>(&mut self, _a_idx: usize, f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            f(self)
        }

        fn read_enum_struct_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder, usize) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_enum_struct_variant_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_struct_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple<T, F>(&mut self, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_struct_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        // Specialized types:
        fn read_option<T, F>(&mut self, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder, bool) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        #[inline]
        fn read_seq<T, F>(&mut self, f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder, usize) -> Result<T, Error>,
        {
            f(self, 3)
        }

        #[inline]
        fn read_seq_elt<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            f(self)
        }

        fn read_map<T, F>(&mut self, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder, usize) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_map_elt_key<T, F>(&mut self, _idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_map_elt_val<T, F>(&mut self, _idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut AnimalDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

mod deserializer {
    use super::{Animal, Error};

    use serde2::de;

    enum State {
        AnimalState(Animal),
        IsizeState(isize),
        StringState(String),
        UnitState,
    }

    pub struct AnimalDeserializer {
        stack: Vec<State>,
    }

    impl AnimalDeserializer {
        #[inline]
        pub fn new(animal: Animal) -> AnimalDeserializer {
            AnimalDeserializer {
                stack: vec!(State::AnimalState(animal)),
            }
        }
    }

    impl de::Deserializer for AnimalDeserializer {
        type Error = Error;

        #[inline]
        fn visit<V>(&mut self, mut visitor: V) -> Result<V::Value, Error>
            where V: de::Visitor,
        {
            match self.stack.pop() {
                Some(State::AnimalState(Animal::Dog)) => {
                    self.stack.push(State::UnitState);
                    visitor.visit_enum("Animal", "Dog", DogVisitor {
                        de: self,
                    })
                }
                Some(State::AnimalState(Animal::Frog(x0, x1))) => {
                    self.stack.push(State::IsizeState(x1));
                    self.stack.push(State::StringState(x0));
                    visitor.visit_enum("Animal", "Frog", FrogVisitor {
                        de: self,
                        state: 0,
                    })
                }
                Some(State::IsizeState(value)) => {
                    visitor.visit_isize(value)
                }
                Some(State::StringState(value)) => {
                    visitor.visit_string(value)
                }
                Some(State::UnitState) => {
                    visitor.visit_unit()
                }
                None => {
                    Err(Error::EndOfStreamError)
                }
            }
        }
    }

    struct DogVisitor<'a> {
        de: &'a mut AnimalDeserializer,
    }

    impl<'a> de::EnumVisitor for DogVisitor<'a> {
        type Error = Error;

        fn visit_unit(&mut self) -> Result<(), Error> {
            de::Deserialize::deserialize(self.de)
        }
    }

    struct FrogVisitor<'a> {
        de: &'a mut AnimalDeserializer,
        state: usize,
    }

    impl<'a> de::EnumVisitor for FrogVisitor<'a> {
        type Error = Error;

        fn visit_seq<V>(&mut self, mut visitor: V) -> Result<V::Value, Error>
            where V: de::EnumSeqVisitor,
        {
            visitor.visit(self)
        }
    }

    impl<'a> de::SeqVisitor for FrogVisitor<'a> {
        type Error = Error;

        fn visit<T>(&mut self) -> Result<Option<T>, Error>
            where T: de::Deserialize,
        {
            match self.state {
                0 => {
                    self.state += 1;
                    Ok(Some(try!(de::Deserialize::deserialize(self.de))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(de::Deserialize::deserialize(self.de))))
                }
                _ => {
                    Ok(None)
                }
            }
        }

        fn end(&mut self) -> Result<(), Error> {
            if self.state == 2 {
                Ok(())
            } else {
                Err(Error::SyntaxError)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = 2 - self.state;
            (len, Some(len))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[bench]
fn bench_decoder_dog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Dog;

        let mut d = decoder::AnimalDecoder::new(animal.clone());
        let value: Animal = Decodable::decode(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_decoder_frog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Frog("Henry".to_string(), 349);

        let mut d = decoder::AnimalDecoder::new(animal.clone());
        let value: Animal = Decodable::decode(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_deserializer_dog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Dog;

        let mut d = deserializer::AnimalDeserializer::new(animal.clone());
        let value: Animal = Deserialize::deserialize(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_deserializer_frog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Frog("Henry".to_string(), 349);

        let mut d = deserializer::AnimalDeserializer::new(animal.clone());
        let value: Animal = Deserialize::deserialize(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}