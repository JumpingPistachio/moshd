use image::{DynamicImage, Rgb};

pub fn relay(iimg: DynamicImage, modifier: i32) -> DynamicImage {
    // Buffer (whole image) and Pixel (individual pixel) 
    let mut buf = iimg.into_rgb8();
    let mut pix_lead: Rgb<u8>;

    // Either use input or decide an "ideal" one based on image height + some math
    let n: u32;
    if modifier > 0 {
        n = modifier as u32; 
        } else { 
        let mut f = buf.height() as f32;
        f = f.sqrt()/4.0;
        n = f.ceil() as u32;
    }

    for x in 0..(buf.width()){
        pix_lead = *buf.get_pixel(x, 0);
        
        for y in 0..(buf.height()){
            if y % n == 0 {
                pix_lead = *buf.get_pixel(x, y);
            }
            buf.put_pixel(x, y, pix_lead); 
        }
    }

    println!("\nModifier: {}", n);
    return DynamicImage::ImageRgb8(buf);
}

pub fn slice_3d(iimg: DynamicImage, modifier: i32) -> DynamicImage {
    // Buffer (whole image) and Pixel (individual pixel) 
    let mut buf = iimg.into_rgb8();
    let orig_buf = buf.clone();
    let mut pix_lead: Rgb<u8>;
    let width = buf.width()-1;
    let height = buf.height()-1;

    for x in 0..(width){
        if x % modifier as u32 == 0 {
            for y in 0..(height){
                pix_lead = *orig_buf.get_pixel(width-x, y);
                buf.put_pixel(x,y, pix_lead);
            } 
        }
        
    }

    return DynamicImage::ImageRgb8(buf);
}


