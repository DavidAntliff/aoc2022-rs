use std::fmt::Formatter;
use std::{fmt, panic};
use svg::Document;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Default, Copy, Clone)]
enum Cell {
    #[default]
    Start,
    End,
    Height(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Height(e) => e,
        }
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

#[wasm_bindgen]
pub struct Grid(grid::Grid<Cell>);

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

#[wasm_bindgen]
impl Grid {
    fn height(&self) -> usize {
        self.0.rows()
    }

    fn width(&self) -> usize {
        self.0.cols()
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.0.cols() && coord.y < self.0.rows()
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.0[coord.y][coord.x])
    }

    fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.0[coord.y][coord.x])
    }

    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Grid {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

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

        let mut document =
            Document::new().set("viewBox", (0, 0, self.width() * SIDE, self.height() * SIDE));

        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell = self.cell((x, y).into()).unwrap();
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

        let defs = svg::node::element::Definitions::new().add(
            svg::node::element::Marker::new()
                .set("id", "arrowhead")
                .set("markerWidth", 10)
                .set("markerHeight", 7)
                .set("refX", 10)
                .set("refY", 3.5)
                .set("orient", "auto")
                .add(
                    svg::node::element::Polygon::new()
                        .set("points", "0 0, 10 3.5, 0 7")
                        .set("fill", "#ffc107"),
                ),
        );
        document = document.add(defs);

        for y in 0..self.height() {
            for x in 0..self.width() {
                console::log_3(&"x, y ".into(), &x.into(), &y.into());
                let coord: GridCoord = (x, y).into();
                for ncoord in self.walkable_neighbors(coord) {
                    let side = SIDE as f64;
                    let (x, y) = (x as f64, y as f64);
                    let dx = ncoord.x as f64 - x;
                    let dy = ncoord.y as f64 - y;
                    console::log_3(&"dx, dy ".into(), &dx.into(), &dy.into());
                    console::log_3(
                        &"x1, y1 ".into(),
                        &(x + 0.5 + dx * 0.05).into(),
                        &(y + 0.5 + dy * 0.05).into(),
                    );
                    console::log_3(
                        &"x2, y2 ".into(),
                        &(x + 0.5 + dx * 0.45).into(),
                        &(y + 0.5 + dy * 0.45).into(),
                    );

                    let line = svg::node::element::Line::new()
                        .set("x1", (x + 0.5 + dx * 0.05) * side)
                        .set("y1", (y + 0.5 + dy * 0.05) * side)
                        .set("x2", (x + 0.5 + dx * 0.45) * side)
                        .set("y2", (y + 0.5 + dy * 0.45) * side)
                        .set("stroke", "#ffc107")
                        .set("stroke-width", "1px")
                        .set("marker-end", "url(#arrowhead)");
                    document = document.add(line);
                }
            }
        }

        document.to_string()
    }

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_elev = self.cell(coord).unwrap().elevation();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(|&coord| {
                let other_elev = self.cell(coord).unwrap().elevation();
                other_elev <= curr_elev + 1
            })
        })
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
