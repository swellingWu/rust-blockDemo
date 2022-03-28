//! 序列化区块头
use bincode;
use serde::{Deserialize, Serialize};

//? 序列化函数
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8> 
    where T: Serialize,
{
    let saialized = bincode::serialize(value).unwrap();
    saialized
}
//? 反序列化函数
pub fn my_deserialize<'a,T> (bytes: &'a[u8]) -> T 
    where T: Deserialize<'a>
{
    let deserialize = bincode::deserialize(bytes).unwrap();
    deserialize
}

//= 测试序列化和反序列化
#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn coders_works() {
        let point = Point{x: 1,y: 1};
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);

        assert_eq!(de,point);
    }
}
