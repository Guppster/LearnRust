extern crate image;

use image::ColorType;
use image::png::PNGEncoder;

use std::fs::File;
use std::str:FromStr;

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
	assert_eq!(pixel_to_point((100,100), (25, 75)m (-1.0, 1.0), (1.0, -1.0)), (-0.5, -0.5));
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
	let output = match File::create(filename)
	{
		Ok(f) => { f }
		Err(e) => { return Err(e); }
	}

	let encoder = PNGEncoder::new(output);
	try!(encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8)));
	
	Ok(());
}
