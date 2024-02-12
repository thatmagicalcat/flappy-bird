#[macro_export]
macro_rules! texture {
    [ $name:expr ] => {
        Texture2D::from_file_with_format(
            include_bytes!(concat!("../res/sprites/", $name)),
            Some(ImageFormat::Png),
        )
    };
}

#[macro_export]
macro_rules! sound {
    [ $name:expr ] => {
        load_sound_from_bytes(
            include_bytes!(concat!("../res/audio/", $name))
        ).await.unwrap()
    };
}
