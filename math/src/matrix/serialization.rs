use super::*;
use crate::{FixedArrayVisitor, Vector};
use serde::{ser::SerializeTuple, Deserialize, Serialize};

impl<'de, T, const R: usize, const C: usize> Deserialize<'de> for Matrix<T, R, C>
where
    T: Deserialize<'de>,
    T: Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array = deserializer.deserialize_tuple_struct(
            "values",
            R,
            FixedArrayVisitor::<Vector<T, C>, R>::new(),
        )?;

        Ok(Matrix::from_row_vectors(array))
    }
}

impl<T: Serialize + Clone, const R: usize, const C: usize> Serialize for Matrix<T, R, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tup = serializer.serialize_tuple(C)?;
        for row in &self.values {
            let v = Vector::from((*row).clone());
            tup.serialize_element(&v)?;
        }
        tup.end()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use serde_yaml::*;

    #[test]
    fn test_serialization() {
        let mat = Mat4u::from_arrays([
            [10, 20, 30, 40],
            [10, 20, 30, 40],
            [10, 20, 30, 40],
            [10, 20, 30, 40],
        ]);
        let string = serde_yaml::to_string(&mat).unwrap();
        let v = serde_yaml::from_str(string.as_str()).unwrap();
        assert_eq!(mat, v);
    }
}
