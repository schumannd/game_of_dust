#[macro_use]
extern crate glium;
extern crate rand;

use rand::Rng;
use std::env;
use std::process;

use glium::{DisplayBuild, Surface};

fn determine_life(x : usize, y : usize, array : &mut [Vec<Vec<usize>>], round : usize) {
    // Live cells surrounding x,y
    let live =  array[(round + 1) % 2][x+1][y] + array[(round + 1) % 2][x][y+1] + 
        array[(round + 1) % 2][x-1][y] + array[(round + 1) % 2][x][y-1] + array[(round + 1) % 2][x+1][y+1] +
        array[(round + 1) % 2][x-1][y-1] + array[(round + 1) % 2][x+1][y-1] + array[(round + 1) % 2][x-1][y+1];

    // Rule of reproduction
    if live == 3 {
        array[round % 2][x][y] = 1;
    }

    // Rule of survival
    else if live == 2 {
        array[round % 2][x][y] = array[(round + 1) % 2][x][y];
    }

    //Rules of death
    else {
        array[round % 2][x][y] = 0;
    }
    // return array[round % 2][x][y];
}

fn grid_to_image(grid: &Vec<Vec<usize>>) -> Vec<Vec<(f32, f32, f32, f32)>> {
    let mut image = Vec::<Vec<(f32, f32, f32, f32)>>::new();

    for x in 1..grid.len() - 1 {
        image.push(Vec::<(f32, f32, f32, f32)>::new());
        for y in 1..grid[0].len() - 1 {
            let life = grid[x][y] as f32;
            image[x - 1 as usize].push((life, life, life, 1.0));
        }
    }
    return image;
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./test_proj <field_dimension>");
        process::exit(1);
    }

    let dimension: usize = args[1].trim().parse().unwrap();

    let mut field = Vec::new();

    for x in 0..dimension + 2 {
        field.push(Vec::new());
        for y in 0..dimension + 2 {
            if x == 0 || y == 0 || x == dimension + 1 || y == dimension + 1 {
                field[x].push(0);
            }else{
                field[x].push(rand::thread_rng().gen_range(0, 2));
            }
            
        }
    }
    let field2 = field.clone();
    let mut fields: [Vec<Vec<usize>>; 2] = [field, field2];


    // glium shit:
    let display = glium::glutin::WindowBuilder::new().with_dimensions(1024, 768).build_glium().unwrap();

    let vertices = vec![Vertex{position:[-1.0,-1.0]}, Vertex{position:[1.0,-1.0]}, Vertex{position:[-1.0,1.0]}, Vertex{position:[1.0,1.0]}];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    // shaders
    let vertex_shader_src = r#"
        #version 330
        in vec2 position;
        out vec2 f_uv;
        void main() {
            gl_Position = vec4(position, 0.0f, 1.0f);
            f_uv = (position + 1.0f) / 2.0f;
        }
    "#;


    let fragment_shader_src = r#"
        #version 330
        uniform sampler2D Texture;
        in vec2 f_uv;
        out vec3 o_color;
        void main() {
            o_color = texture(Texture, f_uv).rgb;
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut round = 0;
    loop {

        let mut target = display.draw();
        // target.clear_color(0.0, 0.0, 1.0, 1.0);

        //calc life
        for x in 1..dimension + 1 {
            for y in 1..dimension + 1 {
                determine_life(x, y, &mut fields, round);
            }
        }

        let texture = glium::texture::Texture2d::new(&display, grid_to_image(&fields[round % 2])).unwrap();
        let sampler = glium::uniforms::Sampler::new(&texture)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let uniforms = uniform! {
            Texture: sampler
        };

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        target.draw(&vertex_buffer, &no_indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        round += 1;
    }
}