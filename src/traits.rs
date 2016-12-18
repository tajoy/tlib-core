
#![forbid(non_camel_case_types)]
// #![allow(missing_docs)]


extern crate rustc_serialize as serialize;

pub trait Configurable: serialize::Decodable + Default {}

pub trait Configer: serialize::Decoder {

    fn config<T: serialize::Decodable, S>(dataSource: &S) -> Result<T, Self::Error>;
}

#[cfg(test)]
mod tests {

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TestStruct {
        data_int: u8,
        data_str: String,
        data_vector: Vec<u8>,
    }

    #[test]
    fn Test1() {
        let object = TestStruct {
            data_int: 1,
            data_str: "homura".to_string(),
            data_vector: vec![2, 3, 4, 5],
        };

        // Serialize using `json::encode`
        let encoded = json::encode(&object).unwrap();

        println!("{:?}", encoded);

        // Deserialize using `json::decode`
        let decoded: TestStruct = json::decode(&encoded).unwrap();
    }

}
