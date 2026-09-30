use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 30;

pub fn cube() {
    println!("Hello, world!");

    #[derive(Clone, Copy)]
    struct Point3D {
        x: f64, // left/right
        y: f64, // up, down
        z: f64, // front, back
    }

    // Just a corner
    let vertices = [
        //parte superior
        Point3D { x: -1.0, y: -1.0, z: -1.0 }, // 0
        Point3D { x:  1.0, y: -1.0, z: -1.0 }, // 1
        Point3D { x: -1.0, y:  1.0, z: -1.0 }, // 2
        Point3D { x:  1.0, y:  1.0, z: -1.0 }, // 3

        //parte inferior
        Point3D { x: -1.0, y: -1.0, z:  1.0 }, // 4
        Point3D { x:  1.0, y: -1.0, z:  1.0 }, // 5
        Point3D { x: -1.0, y:  1.0, z:  1.0 }, // 6
        Point3D { x:  1.0, y:  1.0, z:  1.0 }, // 7
    ];

    let edges = [
        (0, 1),
        (1, 3),
        (3, 2),
        (2, 0),

        (4, 5),
        (5, 7),
        (7, 6),
        (6, 4),

        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    fn project(point: Point3D, width: i32, height: i32) -> (i32, i32) {
        let distance = 4.0;
        let scale = 20.0;

        let z = point.z + distance;

        /*
        O ponto central no terminal não é o (0,0):

        (0,0)
        +-------------------------------->
        |
        |
        |
        |
        v

        Como é um plano 2D o (0,0) é basicamente a primeira célula da primeira coluna,
        seria o primeiro item da lista, não o item que está no centro da página.

        O item do centro da página é obviamente o (comprimento / 2, largura / 2), se a largura
        é 20, e o comprimento é 20, o elemento central é o (10, 10)

        +|01-02-03-04-05-06-07-08-09-10-11-12-13-14-15-16-17-18-19-20
       1|                            |
       2|                            |
       3|                            |
       4|                            |
       5|                            |
       6|                            |
       7|                            |
       8|                            |
       9|                            ↓
      10|------------------------- → X
      11|
      12|
      14|
      15|
      16|
      17|
      18|
      19|
      20|
         */

        let x = point.x / z;
        let y = point.y / z;

        /*
        Por que * 2.0 no X/Y?

        Os caracteres do terminal não são quadrados, são mais altos do que largos,
        dessa forma o cubo iria parecer achatado

        Dessa forma

        x * scale * 2.0

        Estica horizontalmente a imagem
         */

        let screen_x =
            width / 2 + (x * scale * 2.0) as i32;

        let screen_y =
            height / 2 - (y * scale) as i32;

        (screen_x, screen_y)
    }

    /*
    sin e cos

    ↓

    pegam um ponto

    ↓

    calculam onde ele fica depois de girar
     */

    fn rotate(point: Point3D, angle_x: f64, angle_y: f64) -> Point3D {
        // Rotação em X
        let y1 =
            point.y * angle_x.cos() - point.z * angle_x.sin();

        let z1 =
            point.y * angle_x.sin() + point.z * angle_x.cos();

        let x1 = point.x;

        // Rotação em Y

        let x2 =
            x1 * angle_y.cos() + z1 * angle_y.sin();

        let z2 =
            -x1 * angle_y.sin() + z1 * angle_y.cos();

        let y2 = y1;

        Point3D {
            x: x2,
            y: y2,
            z: z2,
        }
    }

    fn draw_line(
        buffer: &mut Vec<Vec<char>>,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
    ) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };

        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            if y0 >= 0
                && y0 < buffer.len() as i32
                && x0 >= 0
                && x0 < buffer[0].len() as i32
            {
                buffer[y0 as usize][x0 as usize] = '#';
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    let mut angle_x: f64 = 0.0;
    let mut angle_y: f64 = 0.0;

    print!("\x1B[?25l");

    loop {

        // Tela
        let mut buffer =
            vec![vec![' '; WIDTH]; HEIGHT];

        // Ultima posição de cada ponto depois da rotação + projeção
        let mut projected_points = Vec::new();

        for vertex in vertices {
            let rotated = rotate(vertex, angle_x, angle_y);

            let projected = project(rotated, WIDTH as i32, HEIGHT as i32);

            projected_points.push(projected);
        }

        // Desenha cada aresta
        for &(start, end) in &edges {
            let (x0, y0) = projected_points[start];
            let (x1, y1) = projected_points[end];

            draw_line(
                &mut buffer,
                x0, y0,
                x1, y1,
            );
        }

        for &(x, y) in &projected_points {
            if x >= 0
                && x < WIDTH as i32
                && y >= 0
                && y < HEIGHT as i32
            {
                buffer[y as usize][x as usize] = 'O';
            }
        }

        // ANSI:
        // limpa a tela e move o cursor para o topo
        print!("\x1B[H");

        let mut frame = String::new();

        for row in &buffer {
            for character in row {
                frame.push(*character);
            }

            frame.push('\n');
        }

        print!("{frame}");

        io::stdout().flush().unwrap();

        // muda os angulos
        angle_x += 0.02;
        angle_y += 0.035;

        // 60 fps
        thread::sleep(
            Duration::from_millis(33)
        );
    }
}
