#[macro_export]
macro_rules! map {
    () => (
        std::collections::HashMap::new()
    );
    ($($key:expr => $value:expr),* $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.into(), $value);
        )*
        map
    });
}
