use backend::seed::seed;
use evaluation_fin_juillet_2024::{self as backend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = backend::etablish_connection();
    let mut con = pool.get()?;
    seed(&mut con)?;
    Ok(())
}
