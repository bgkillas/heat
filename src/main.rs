use image::{ImageBuffer, Luma};
use std::{
    env::args,
    io::{stdin, Read},
};
fn main()
{
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    let mut grid: Vec<Vec<_>> = Vec::new();
    if args.is_empty()
    {
        let mut start_grid: Vec<Vec<u32>>;
        let mut max = 0;
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
        for row in start_grid
        {
            let mut grid_row = Vec::new();
            for num in row
            {
                grid_row.push(((num as f64 / max as f64) * 255.0).round() as u8)
            }
            grid.push(grid_row);
        }
    }
    else
    {
        let die: usize = args[0].parse().unwrap();
        let face: usize = args[1].parse().unwrap();
        grid = vec![Vec::new(); face.pow((die / 2) as u32)];
        let common_sum = (die + (die * face)) / 2;
        let line = face.pow((die / 2) as u32);
        let mut faces = Vec::new();
        let mut place = (0, 0);
        get_dimensions(
            die, face, common_sum, line, &mut place, &mut grid, &mut faces,
        )
    }
    let mut img = ImageBuffer::new(grid.len() as u32, grid[0].len() as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut()
    {
        *pixel = Luma([grid[x as usize][y as usize]])
    }
    if args.is_empty()
    {
        img.save("/home/binary_image.png").unwrap();
    }
    else
    {
        img.save(args.join("_") + ".png").unwrap();
    }
}
fn get_dimensions(
    die: usize,
    face: usize,
    common_sum: usize,
    line: usize,
    place: &mut (usize, usize),
    grid: &mut Vec<Vec<u8>>,
    faces: &mut Vec<usize>,
)
{
    if faces.len() != die
    {
        for i in 1..face + 1
        {
            faces.push(i);
            get_dimensions(die, face, common_sum, line, place, grid, faces);
            faces.pop();
        }
    }
    else
    {
        if place.0 >= line
        {
            place.0 = 0;
            place.1 += 1;
        }
        place.0 += 1;
        grid[place.1].push(
            if faces.iter().sum::<usize>() == common_sum
            {
                255u8
            }
            else
            {
                0u8
            },
        );
    }
}