/**
 * Author:  Nyxvectar Yan
 * Repo:    luogu-utils
 * Created: 01/30/2025
 */

pub mod get_input {    fn read_input<T: std::str::FromStr>() -> Result<Vec<T>, Box<dyn Error>>
where
    <T as std::str::FromStr>::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result: Result<Vec<T>, _> =
        input.trim().split_whitespace().map(|s| s.parse()).collect();
    result.map_err(|e| e.into())
}}
