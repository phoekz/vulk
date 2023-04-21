use super::*;

#[test]
fn enum_display() {
    assert_eq!(format!("{}", vk::StructureType::ApplicationInfo), "ApplicationInfo");
}

#[test]
fn flag_bits_display() {
    assert_eq!(format!("{}", vk::AccessFlagBits2::DescriptorBufferReadEXT), "DescriptorBufferReadEXT");
    assert_eq!(format!("{}", vk::CullModeFlagBits::Front | vk::CullModeFlagBits::Back), "Front | Back | FrontAndBack");
}

#[test]
fn flag_bits_display_unknown_bits() {
    let flags: vk::BufferUsageFlags = unsafe { std::mem::transmute(0b1000000000) };
    let flags = flags | vk::BufferUsageFlagBits::TransferSrc;
    assert_eq!(format!("{flags}"), "TransferSrc | 0b1000000000");
}