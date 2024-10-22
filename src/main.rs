use app::App;
use args::Args;
use board::board_struct::Board;
use error::Error;
use solver::{bt_solver::BtSolver, Solver};
use termint::{enums::Color, widgets::StrSpanExtension};

mod app;
mod args;
mod board;
mod error;
mod solver;
mod tests;

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
    // let mut app = App::default();
    app.run()
    // test_solver()
}

#[allow(unused)]
fn test_solver() -> Result<(), Error> {
    let mut board = Board::default();
    if BtSolver::solve(&mut board) {
        println!("Solved!");
        print_board(&board);
        Ok(())
    } else {
        Err(Error::Msg("not solved".to_string()))
    }
}

pub fn print_board(board: &Board) {
    for y in 0..board.size() {
        for x in 0..board.size() {
            print!("{} ", board[x + y * board.size()].value());
        }
        println!();
    }
}
