use super::map::Map;

pub trait MapBuilder {
    fn build() -> Map;
}
