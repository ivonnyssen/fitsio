#[derive(PartialEq, Debug)]
pub enum DataArray {
    U8(Vec<u8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

impl DataArray {
    pub fn from_u8(data: Vec<u8>) -> Self {
        Self::U8(data)
    }
    pub fn from_i16(data: Vec<i16>) -> Self {
        Self::I16(data)
    }
    pub fn from_i32(data: Vec<i32>) -> Self {
        Self::I32(data)
    }
    pub fn from_i64(data: Vec<i64>) -> Self {
        Self::I64(data)
    }
    pub fn from_f32(data: Vec<f32>) -> Self {
        Self::F32(data)
    }
    pub fn from_f64(data: Vec<f64>) -> Self {
        Self::F64(data)
    }
}
