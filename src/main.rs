use app::App;
use args::Args;
use board::{board_struct::Board, cell::Cell};
use error::Error;
use solver::look_ahead_bit::ac3_bit_solver::Ac3BitSolver;
use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::{Rect, Vec2},
    widgets::{StrSpanExtension, Widget},
};

mod app;
mod args;
mod board;
mod checker;
mod error;
mod solver;
mod tests;
mod tui;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".fg(Color::Red), e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        Args::help();
        return Ok(());
    }
    let mut app = App::new(args.size, args.solver);
    app.run()
}

#[allow(unused)]
fn test_solver() -> Result<(), Error> {
    let mut board = Board {
        cells: vec![Cell::empty(); 16],
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
    };
    board.hor_conds[1] = Some(true);
    board.hor_conds[2] = Some(true);
    board.hor_conds[11] = Some(true);
    board.ver_conds[0] = Some(false);
    board.ver_conds[4] = Some(true);
    board.ver_conds[5] = Some(false);
    board.ver_conds[6] = Some(false);

    if Ac3BitSolver::solve(&mut board) {
        println!("Solved!");
        let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 10));
        board.render(&mut buffer);
        buffer.render();
        Ok(())
    } else {
        Err(Error::Msg("not solved".to_string()))
    }
}
