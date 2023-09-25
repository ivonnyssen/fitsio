#[derive(PartialEq, Debug)]
pub enum DataArray {
    U8(Vec<u8>, Vec<u32>, f32, f64),
    I16(Vec<i16>, Vec<u32>, f32, f64),
    I32(Vec<i32>, Vec<u32>, f32, f64),
    I64(Vec<i64>, Vec<u32>, f32, f64),
    F32(Vec<f32>, Vec<u32>, f32, f64),
    F64(Vec<f64>, Vec<u32>, f32, f64),
}

//physical value = BZERO + BSCALE × array value
impl DataArray {
    pub fn from_u8(
        data: Vec<u8>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::U8(data, dimensions, bzero, bscale)
    }
    pub fn from_i16(
        data: Vec<i16>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::I16(data, dimensions, bzero, bscale)
    }
    pub fn from_i32(
        data: Vec<i32>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::I32(data, dimensions, bzero, bscale)
    }
    pub fn from_i64(
        data: Vec<i64>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::I64(data, dimensions, bzero, bscale)
    }
    pub fn from_f32(
        data: Vec<f32>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::F32(data, dimensions, bzero, bscale)
    }
    pub fn from_f64(
        data: Vec<f64>,
        dimensions: Vec<u32>,
        bzero: Option<f32>,
        bscale: Option<f64>,
    ) -> Self {
        let bzero = bzero.unwrap_or(0.0);
        let bscale = bscale.unwrap_or(1.0);
        Self::F64(data, dimensions, bzero, bscale)
    }

    pub fn get_u8_value(&self, position: Vec<u32>) -> u8 {
        match self {
            Self::U8(data, dimensions, bzero, bscale) => {
                let (_, index) = dimensions.iter().zip(position.iter()).fold(
                    (None, None),
                    |mut acc: (Option<u32>, Option<u64>), (&d, &p)| {
                        acc.1 = match acc.1 {
                            Some(_) => Some(acc.1.unwrap() + p as u64 * acc.0.unwrap() as u64),
                            None => Some(p as u64),
                        };
                        acc.0 = match acc.0 {
                            Some(dimension) => Some(d * dimension),
                            None => Some(d),
                        };
                        acc
                    },
                );

                let value = data[index.unwrap() as usize];
                let physical_value = *bzero as f64 + *bscale * value as f64;
                physical_value as u8
            }
            _ => panic!("Wrong data type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_array_u8() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let bzero = 0.0;
        let bscale = 1.0;
        let data_array = DataArray::from_u8(data, vec![4, 3, 2], Some(bzero), Some(bscale));
        assert_eq!(data_array.get_u8_value(vec![0, 0, 0]), 1);
        assert_eq!(data_array.get_u8_value(vec![1, 0, 0]), 2);
        assert_eq!(data_array.get_u8_value(vec![2, 0, 0]), 3);
        assert_eq!(data_array.get_u8_value(vec![3, 0, 0]), 4);
        assert_eq!(data_array.get_u8_value(vec![0, 1, 0]), 5);
        assert_eq!(data_array.get_u8_value(vec![1, 1, 0]), 6);
        assert_eq!(data_array.get_u8_value(vec![2, 1, 0]), 7);
        assert_eq!(data_array.get_u8_value(vec![3, 1, 0]), 8);
        assert_eq!(data_array.get_u8_value(vec![0, 2, 0]), 9);
        assert_eq!(data_array.get_u8_value(vec![1, 2, 0]), 10);
        assert_eq!(data_array.get_u8_value(vec![2, 2, 0]), 11);
        assert_eq!(data_array.get_u8_value(vec![3, 2, 0]), 12);
        assert_eq!(data_array.get_u8_value(vec![0, 0, 1]), 13);
        assert_eq!(data_array.get_u8_value(vec![1, 0, 1]), 14);
        assert_eq!(data_array.get_u8_value(vec![2, 0, 1]), 15);
        assert_eq!(data_array.get_u8_value(vec![3, 0, 1]), 16);
    }
}
