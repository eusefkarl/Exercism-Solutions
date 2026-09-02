use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gs: i64 = 10_i64.pow(9);
    let future = start + gs.seconds();
    future
}
