//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;

/// Draws a standard image from the center to the passed vector to the specified layer
pub fn draw_ex_with_center(window: &mut Window, image: &mut Asset<Image>, coordinate: Vector, transform: Transform, layer: f32) -> Result<()> {
    image.execute(|image| {
        window.draw_ex(
            &image
                .area()
                .with_center((coordinate.x, coordinate.y)),
            Img(&image),
            transform,
            layer,
        );
        Ok(())
    })?;
    Ok(())
}

///Draws something from an Atlas at the given coordinates
pub fn draw_ex_atlas_with_center(window: &mut Window, atlas: &mut Asset<Atlas>, coordinate: Vector, transform: Transform, layer: f32, key: &str) -> Result<()> {
    atlas.execute(|image| {
        window.draw_ex(
            &image.get(key).expect("Failed to find key in draw").unwrap_image().area()
                .with_center((coordinate.x, coordinate.y)),
            Img(&image.get(key).expect("Failed to find key in draw").unwrap_image()),
            transform,
            layer,
        );
        Ok(())
    })?;
    Ok(())
}