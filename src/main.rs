use z3::{ast::BV, Config, Context, Optimize, SatResult, Solver};

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    solver.from_string(include_str!("input.smtlib2"));
    let value_to_minimize = BV::new_const(&ctx, "initial_a", 64);
    let opt = Optimize::new(&ctx);
    opt.minimize(&value_to_minimize);
    solver.check();
    let model = solver.get_model().unwrap();
    let automatically_minimized_value = model.get_const_interp(&value_to_minimize).unwrap();
    let mut manually_minimized_value = automatically_minimized_value.clone();
    while let SatResult::Sat = solver.check() {
        let model = solver.get_model().unwrap();
        manually_minimized_value = model.get_const_interp(&value_to_minimize).unwrap();
        solver.assert(&value_to_minimize.bvult(&manually_minimized_value));
    }
    assert_eq!(automatically_minimized_value, manually_minimized_value);
}
