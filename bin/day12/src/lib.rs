use std::fmt::Formatter;
use std::{fmt, panic};
use svg::Document;
use wasm_bindgen::prelude::*;
use web_sys::console;

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in 0..self.0.rows() {
            for c in 0..self.0.cols() {
                write!(f, "{:?}", self.0[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Height(elevation) => (b'a' + elevation) as char,
        };
        write!(f, "{c}")?;
        Ok(())
    }
}

#[derive(Default)]
enum Cell {
    #[default]
    Start,
    End,
    Height(u8),
}

#[wasm_bindgen]
pub struct Grid(grid::Grid<Cell>);

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Grid {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        console::log_1(&"Hello using web-sys".into());

        let lines: Vec<String> = input.lines().map(String::from).collect();
        let num_cols = lines[0].len();
        let sdata = lines.join("");
        let data: Vec<Cell> = sdata
            .chars()
            .map(|c| match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Height(c as u8 - b'a'),
                _ => panic!("invalid character: {c}"),
            })
            .collect();
        console::log_3(
            &"new Grid: rows, cols ".into(),
            &lines.len().into(),
            &num_cols.into(),
        );

        Grid(grid::Grid::from_vec(data, num_cols))
    }

    #[wasm_bindgen]
    pub fn to_svg(&self) -> String {
        const SIDE: usize = 64;

        let mut document = Document::new().set(
            "viewBox",
            (0, 0, self.0.cols() * SIDE, self.0.rows() * SIDE),
        );

        for y in 0..self.0.rows() {
            for x in 0..self.0.cols() {
                console::log_3(&"x ".into(), &x.into(), &y.into());

                let cell = self.0.get(y, x).unwrap();
                let (title, r, g, b) = match cell {
                    Cell::Start => ("start".to_string(), 216, 27, 96),
                    Cell::End => ("end".to_string(), 30, 136, 229),
                    Cell::Height(elevation) => {
                        let title = format!("elevation {elevation}");
                        let elevation = *elevation as f32 / 25.0;
                        let f = (elevation * 255.0) as u8;
                        (title, f, f, f)
                    }
                };
                let rect = svg::node::element::Rectangle::new()
                    .set("x", x * SIDE)
                    .set("y", y * SIDE)
                    .set("width", SIDE)
                    .set("height", SIDE)
                    .set("fill", format!("rgb({r}, {g}, {b})"))
                    .set("stroke", "white")
                    .set("stroke-width", "2px")
                    .add(svg::node::element::Title::new().add(svg::node::Text::new(title)));
                document = document.add(rect);
            }
        }

        document.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

fn in_bounds(grid: &Grid, coord: GridCoord) -> bool {
    coord.x < grid.0.cols() && coord.y < grid.0.rows()
}
