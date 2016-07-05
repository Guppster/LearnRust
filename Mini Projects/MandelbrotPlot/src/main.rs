extern crate image;
extern crate num;
extern crate crossbeam;

use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;

use std::io::Write;
use std::fs::File;
use std::str::FromStr;

//Determins if the complex number is in the Mandelbrot set 
//using at most 'limit' iterations
fn escapes(c: Complex<f64>, limit: u32) -> Option<u32> 
{
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit 
        {
                z = z*z + c;
                if z.norm_sqr() > 4.0 
                {
                        return Some(i);
                }
        }

        return None;
}

//<T: FromStr> is "For any type T that implements the FromStr trait"
//This allows us to use <u32>, <f64>, etc
//This method requires a string and a seperator and returns a option pair of type T 
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> 
{
        //First check to see that the seperator exists in the string
        match s.find(separator)
        {
                //if it doesn't return none
                None => None,

                     //If it does exist, capture the index from the return of find()
                     Some(index) => 
                     {
                             //This statement splits the string at the seperator and parses both sides from string to T
                             match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) 
                             {
                                     //if the above match expression is successful return the option pair
                                     (Ok(l), Ok(r)) => Some((l,r)),

                                     //if it is anything else return None.	
                                     _ => None
                             }
                     }
        }
}

#[test]
fn test_parse_pair()
{
        assert_eq!(parse_pair::<i32>("",        ','), None);
        assert_eq!(parse_pair::<i32>("10,",     ','), None);
        assert_eq!(parse_pair::<i32>(",10",     ','), None);
        assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

//Return the point on the complex plane corresponding to a given pixel in the bitmap
//Takes in 4 tuples as arguements and returns a tuple of f64s
fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                upper_left: (f64, f64),
                lower_right: (f64, f64)) -> (f64,f64)
{
        //The tuple.0 or tuple.1 syntax is used to access elements in the tuple
        let (width, height) = (lower_right.0 - upper_left.0, upper_left.1 - lower_right.1);

        //Since this is the last statement in the function, it is the return
        (upper_left.0 + pixel.0 as f64 * width / bounds.0 as f64, upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

#[test]
fn test_pixel_to_point()
{
        assert_eq!(pixel_to_point((100,100), (25, 75), (-1.0, 1.0), (1.0, -1.0)), (-0.5, -0.5));
}

//Render a rectangle of the Mandelbrot set into a buffer of pixels
//The bounds argument gives the width and the height of the buffer 'pixels'
//Which hold one grayscale pixel per byte. The upper_left and lower_right
//arguments specify points on the complex plane corresponding to the upper left
//and lower right corners of the pixel buffer
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: (f64, f64), lower_right: (f64,f64))
{
        assert!(pixels.len() == bounds.0 * bounds.1);

        for r in 0 .. bounds.1
        {
                for c in 0 .. bounds.0
                {
                        let point = pixel_to_point(bounds, (c, r), upper_left, lower_right);

                        pixels[r * bounds.0 + c] = 
                                match escapes(Complex {re: point.0, im: point.1 }, 255)
                                {
                                        None => 0,
                                        Some(count) => 255 - count as u8
                                }
                }
        }
}

//Write the buffer 'pixels', whose dimensions are given by 'bounds', to the file named 'filename'
fn write_bitmap(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
        //The try statement returns the error code immediately if an error occurs
        //(Cannot be used in functions that don't return Result type. Eg. Main)
        let output = try!(File::create(filename));

        let encoder = PNGEncoder::new(output);
        try!(encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8)));

        Ok(())
}

fn main()
{
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 5
        {
                //Here unwrap will cause the program to exit if it cannot be written? 
                writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERLEFT").unwrap();
                writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]).unwrap();
                std::process::exit(1);
        }

        let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");

        let upper_left = parse_pair(&args[3], ',').expect("error parsing upper left corner point");

        let lower_right = parse_pair(&args[4], ',').expect("error parsing lower right corner point");

        let mut pixels = vec![0; bounds.0 * bounds.1];

        //Decide to use 8 threads 
        let threads = 8;

        //Compute how many rows of pixels each band should have
        let band_rows = bounds.1 / threads + 1;

        {
                //Devide the pixel buffer into bands
                let bands: Vec<_> = pixels.chunks_mut(band_rows * bounds.0).collect();

                //My first closure expression in rust
                crossbeam::scope(|scope| {
                                for (i, band) in bands.into_iter().enumerate() {
                                let top = band_rows * i;
                                let height = band.len() / bounds.0;
                                let band_bounds = (bounds.0, height);
                                let band_upper_left = pixel_to_point(bounds, (0, top),
                                                upper_left, lower_right);
                                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
                                                upper_left, lower_right);

                                scope.spawn(move || {
                                                render(band, band_bounds, band_upper_left, band_lower_right);
                                                });
                                }
                                });
        }
        write_bitmap(&args[1], &pixels, bounds).expect("error writing PNG file");
}

