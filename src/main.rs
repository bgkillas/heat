use image::{ImageBuffer, Rgb};
use std::io::{stdin, Read};
fn main()
{
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut start_grid: Vec<Vec<u32>> =
        vec![Vec::new(); input.chars().filter(|c| c == &'\n').count()];
    let mut i = 0;
    let mut max = 0;
    let mut word = String::new();
    for c in input.chars()
    {
        if c.is_numeric()
        {
            word.push(c)
        }
        else
        {
            let num = word.parse().unwrap();
            word.clear();
            if num > max
            {
                max = num;
            }
            start_grid[i].push(num);
            if c == '\n'
            {
                i += 1;
            }
        }
    }
    let mut grid: Vec<Vec<_>> = Vec::new();
    for row in start_grid
    {
        let mut grid_row = Vec::new();
        for num in row
        {
            grid_row.push([((num as f64 / max as f64) * 255.0).round() as u8; 3])
        }
        grid.push(grid_row);
    }
    let mut img = ImageBuffer::new(grid.len() as u32, grid[0].len() as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut()
    {
        *pixel = Rgb(grid[x as usize][y as usize])
    }
    img.save("/home/binary_image.png").unwrap();
}