use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::vec::Vec;

#[derive(Copy, Clone)]
struct Colour
{
    r: u8,
    g: u8,
    b: u8
}

struct Image
{
    width: u32,
    height: u32,
    colours: Vec<Colour>
}

fn main()
{
    let filename = "test.ppm";
    let mut image = read_ppm(filename);
    edge_detect(&mut image);
    let filename_out = "test_out.ppm";
    write_ppm(filename_out, image);
}

fn read_ppm(file: &str) -> Image
{
    let mut retval: Vec<Colour> = Vec::new();
    let f = File::open(file).unwrap();
    let mut file = BufReader::new(&f);
    let mut line = String::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    //f.read_to_string(&mut s).unwrap();
    file.read_line(&mut line).unwrap();
    if line.trim() == "P6"
    {
        let mut line2 = String::new();
        file.read_line(&mut line2);
        let split: Vec<_> = line2.split(' ').collect();
        width = split[0].trim().parse::<u32>().unwrap();
        height = split[1].trim().parse::<u32>().unwrap();

        let mut line3 = String::new();
        file.read_line(&mut line3);
        if line3.trim() == "255"
        {
            //get all the colour information
            for _ in 0..(width * height)//range(0, width * height)
            {
                let mut buf: [u8; 3] = [0; 3];
                file.read_exact(&mut buf);
                let new_colour = Colour { r: buf[0], g: buf[1], b: buf[2] };
                retval.push(new_colour);
            }
        }
        else
        {
            println!("This file does not have the correct header ending of 255: {}", line);
        }
    }
    else
    {
        println!("The file does not start with P6: {}", line);
    }

    return Image{width: width, height: height, colours: retval};
}

fn edge_detect(image: &mut Image)
{
    let mut colours: Vec<Colour> = Vec::new();
    let mut last_colour: Colour = image.colours[0];
    let mut num_pixels = image.colours.len();

    for i in 0..num_pixels
    {
        let c: Colour = threshhold(image.colours[i], last_colour);
        colours.push(c);
        last_colour = image.colours[i];
    }

    image.colours = colours;
}

fn threshhold(a: Colour, b: Colour) -> Colour
{
    let intensityA = intensity_from_rgb(a) as i16;
    let intensityB = intensity_from_rgb(b) as i16;
    let thresh: i16 = 12;
    let mut c = Colour {r:0, g:0, b:0};
    if (intensityA - intensityB) > thresh || (intensityB - intensityA) > thresh
    {
        c = Colour {r:255, g:255, b:255};
    }
    
    return c;
}

fn intensity_from_rgb(c: Colour) -> u8
{
    return ((0.299f64 * (c.r as f64)) + (0.587f64 * (c.g as f64)) + (0.114f64 * (c.b as f64))) as u8;
}

fn write_ppm(filename: &str, image: Image)
{
    let mut f = File::create(filename).unwrap();
    f.write("P6\n".as_bytes()).unwrap();
    f.write(image.width.to_string().as_bytes()).unwrap();
    f.write(" ".as_bytes()).unwrap();
    f.write(image.height.to_string().as_bytes()).unwrap();
    f.write("\n255\n".as_bytes()).unwrap();

    for c in image.colours
    {
        f.write(&[c.r, c.g, c.b]);
    }
}
