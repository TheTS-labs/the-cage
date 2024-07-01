pub type PixelCoord = u16;

pub trait PixelCoords {
    fn transform_pixel_x(&self, pixel_x: PixelCoord) -> f64 { pixel_x as f64 }
    fn transform_pixel_y(&self, pixel_y: PixelCoord) -> f64 {
        if pixel_y == 0 {
            return 0.5;
        }

        pixel_y as f64 * 0.5
    }

    fn calculate_coords(&self, pixel_x: PixelCoord, pixel_y: PixelCoord) -> [f64; 2] {
        [self.transform_pixel_x(pixel_x), self.transform_pixel_y(pixel_y)]
    }
}