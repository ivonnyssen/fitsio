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

    pub fn get_u8_value(&self, position: Vec<u32>) -> Option<u8> {
        match self {
            Self::U8(data, dimensions, bzero, bscale) => {
                let index = calculate_index(dimensions, position);
                Some((*bzero as f64 + *bscale * data[index] as f64) as u8)
            }
            _ => None,
        }
    }

    pub fn get_i16_value(&self, position: Vec<u32>) -> Option<i16> {
        match self {
            Self::I16(data, dimensions, bzero, bscale) => {
                let index = calculate_index(dimensions, position);
                Some((*bzero as f64 + *bscale * data[index] as f64) as i16)
            }
            _ => None,
        }
    }
}

fn calculate_index(dimensions: &[u32], position: Vec<u32>) -> usize {
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
    index.unwrap_or(0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_array_u8() {
        let data = vec![
            1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
        ];
        let bzero = 0.0;
        let bscale = 1.0;
        let data_array = DataArray::from_u8(data, vec![4, 3, 2], Some(bzero), Some(bscale));
        assert_eq!(data_array.get_u8_value(vec![0, 0, 0]), Some(1));
        assert_eq!(data_array.get_u8_value(vec![1, 0, 0]), Some(2));
        assert_eq!(data_array.get_u8_value(vec![2, 0, 0]), Some(3));
        assert_eq!(data_array.get_u8_value(vec![3, 0, 0]), Some(4));
        assert_eq!(data_array.get_u8_value(vec![0, 1, 0]), Some(5));
        assert_eq!(data_array.get_u8_value(vec![1, 1, 0]), Some(6));
        assert_eq!(data_array.get_u8_value(vec![2, 1, 0]), Some(7));
        assert_eq!(data_array.get_u8_value(vec![3, 1, 0]), Some(8));
        assert_eq!(data_array.get_u8_value(vec![0, 2, 0]), Some(9));
        assert_eq!(data_array.get_u8_value(vec![1, 2, 0]), Some(10));
        assert_eq!(data_array.get_u8_value(vec![2, 2, 0]), Some(11));
        assert_eq!(data_array.get_u8_value(vec![3, 2, 0]), Some(12));
        assert_eq!(data_array.get_u8_value(vec![0, 0, 1]), Some(13));
        assert_eq!(data_array.get_u8_value(vec![1, 0, 1]), Some(14));
        assert_eq!(data_array.get_u8_value(vec![2, 0, 1]), Some(15));
        assert_eq!(data_array.get_u8_value(vec![3, 0, 1]), Some(16));
    }

    #[test]
    fn data_array_i16() {
        let data = vec![1i16, 2i16, 3i16, 4i16];
        let bzero = 0.0;
        let bscale = 1.0;
        let data_array = DataArray::from_i16(data, vec![2, 2], Some(bzero), Some(bscale));
        assert_eq!(data_array.get_i16_value(vec![0, 0]), Some(1));
        assert_eq!(data_array.get_i16_value(vec![1, 0]), Some(2));
        assert_eq!(data_array.get_i16_value(vec![0, 1]), Some(3));
        assert_eq!(data_array.get_i16_value(vec![1, 1]), Some(4));
    }
}
