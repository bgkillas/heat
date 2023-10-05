use image::{ImageBuffer, Luma};
use std::{
    env::args,
    io::{stdin, Read},
};
fn main()
{
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    let mut start_grid: Vec<Vec<u32>>;
    let mut max = 0;
    if args.is_empty()
    {
        let mut i = 0;
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();
        start_grid = vec![Vec::new(); input.chars().filter(|c| c == &'\n').count()];
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
    }
    else
    {
        max = 1;
        let die: usize = args[0].parse().unwrap();
        let face: usize = args[1].parse().unwrap();
        start_grid = vec![Vec::new(); face.pow((die / 2) as u32)];
        let common_sum = (die + (die * face)) / 2;
        for place in 0..face.pow(die as u32)
        {
            let mut num = place;
            let mut faces = Vec::new();
            for pos in (0..die).rev()
            {
                let res = num / face.pow(pos as u32);
                num -= res * face.pow(pos as u32);
                faces.push(res + 1);
            }
            start_grid[place / face.pow((die / 2) as u32)].push(
                if faces.iter().sum::<usize>() == common_sum
                {
                    1
                }
                else
                {
                    0
                },
            );
        }
    }
    let mut grid: Vec<Vec<_>> = Vec::new();
    for row in start_grid
    {
        let mut grid_row = Vec::new();
        for num in row
        {
            grid_row.push([((num as f64 / max as f64) * 255.0).round() as u8])
        }
        grid.push(grid_row);
    }
    let mut img = ImageBuffer::new(grid.len() as u32, grid[0].len() as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut()
    {
        *pixel = Luma(grid[x as usize][y as usize])
    }
    img.save("/home/binary_image.png").unwrap();
}