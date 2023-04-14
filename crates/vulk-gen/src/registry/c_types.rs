use super::*;

pub type CtypeMap = HashMap<&'static str, &'static str>;

pub fn c_type_map() -> CtypeMap {
    let mut map = CtypeMap::new();
    map.insert("void", "c_void");
    map.insert("char", "c_char");
    map.insert("int", "c_int");
    map.insert("size_t", "usize");
    map.insert("float", "f32");
    map.insert("double", "f64");
    map.insert("int32_t", "i32");
    map.insert("int64_t", "i64");
    map.insert("uint8_t", "u8");
    map.insert("uint16_t", "u16");
    map.insert("uint32_t", "u32");
    map.insert("uint64_t", "u64");
    map
}
