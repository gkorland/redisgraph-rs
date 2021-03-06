use crate::{
    assignments::FromCell,
    client_type_error,
    result_set::{Node, Relation, Scalar},
    RedisGraphError, RedisGraphResult, RedisString, ResultSet,
};

impl FromCell for Scalar {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let scalar = result_set.get_scalar(row_idx, column_idx)?;
        Ok(scalar.clone())
    }
}

impl FromCell for () {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let scalar = result_set.get_scalar(row_idx, column_idx)?;
        match scalar {
            Scalar::Nil => Ok(()),
            any => client_type_error!("failed to construct value: expected nil, found {:?}", any),
        }
    }
}

impl<T: FromCell> FromCell for Option<T> {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let scalar = result_set.get_scalar(row_idx, column_idx)?;
        match scalar {
            Scalar::Nil => Ok(None),
            _ => T::from_cell(result_set, row_idx, column_idx).map(Some),
        }
    }
}

impl FromCell for bool {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let scalar = result_set.get_scalar(row_idx, column_idx)?;
        match scalar {
            Scalar::Boolean(boolean) => Ok(*boolean),
            any => client_type_error!(
                "failed to construct value: expected boolean, found {:?}",
                any
            ),
        }
    }
}

macro_rules! impl_from_scalar_for_integer {
    ($t:ty) => {
        impl FromCell for $t {
            fn from_cell(
                result_set: &ResultSet,
                row_idx: usize,
                column_idx: usize,
            ) -> RedisGraphResult<Self> {
                let scalar = result_set.get_scalar(row_idx, column_idx)?;
                match scalar {
                    Scalar::Integer(int) => Ok(*int as $t),
                    any => client_type_error!(
                        "failed to construct value: expected integer, found {:?}",
                        any
                    ),
                }
            }
        }
    };
}

impl_from_scalar_for_integer!(u8);
impl_from_scalar_for_integer!(u16);
impl_from_scalar_for_integer!(u32);
impl_from_scalar_for_integer!(u64);
impl_from_scalar_for_integer!(usize);

impl_from_scalar_for_integer!(i8);
impl_from_scalar_for_integer!(i16);
impl_from_scalar_for_integer!(i32);
impl_from_scalar_for_integer!(i64);
impl_from_scalar_for_integer!(isize);

macro_rules! impl_from_scalar_for_float {
    ($t:ty) => {
        impl FromCell for $t {
            fn from_cell(
                result_set: &ResultSet,
                row_idx: usize,
                column_idx: usize,
            ) -> RedisGraphResult<Self> {
                let scalar = result_set.get_scalar(row_idx, column_idx)?;
                match scalar {
                    Scalar::Double(double) => Ok(*double as $t),
                    any => client_type_error!(
                        "failed to construct value: expected double, found {:?}",
                        any
                    ),
                }
            }
        }
    };
}

impl_from_scalar_for_float!(f32);
impl_from_scalar_for_float!(f64);

impl FromCell for RedisString {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let scalar = result_set.get_scalar(row_idx, column_idx)?;
        match scalar {
            Scalar::String(data) => Ok(data.clone()),
            any => client_type_error!(
                "failed to construct value: expected string, found {:?}",
                any
            ),
        }
    }
}

impl FromCell for String {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let redis_string = RedisString::from_cell(result_set, row_idx, column_idx)?;
        String::from_utf8(redis_string.into()).map_err(|_| RedisGraphError::InvalidUtf8)
    }
}

impl FromCell for Node {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let node = result_set.get_node(row_idx, column_idx)?;
        Ok(node.clone())
    }
}

impl FromCell for Relation {
    fn from_cell(
        result_set: &ResultSet,
        row_idx: usize,
        column_idx: usize,
    ) -> RedisGraphResult<Self> {
        let relation = result_set.get_relation(row_idx, column_idx)?;
        Ok(relation.clone())
    }
}
