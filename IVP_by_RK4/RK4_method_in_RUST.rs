fn main() {
    // Initial time
    let t_initial = 0.0;
    // Initial value of the solution y at time t_initial
    let y_initial = 0.5;
    // Final time
    let t_final = 2.0;
    // Number of steps
    let n = 10;
    // Calculating step size
    let h = (t_final - t_initial) / n as f64;

    // Calculating the t and y values using RK4
    let (t_vals, y_vals) = rk4(t_initial, y_initial, h, n);

    // Printing all t and y values i.e the solution at time steps:::::
    for (t, y) in t_vals.iter().zip(y_vals.iter()) {
        println!("The solution of the given differential equation at t = {:.2}, is y = {:.4}", t, y);
    }
}


/// Function for Runge-Kutta 4th order method
///     Parameters
///     - t_initial: initial time
///     - y_initial: initial y value
///     - h: step size
///     - n: number of steps
///
///     Returns
///     - A tuple containing two vectors: times and corresponding y values
fn rk4(t_initial: f64, y_initial: f64, h: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    // Initialize time t and solution y
    let mut t = t_initial;      // mut means mutable so that we can change the variable hereafter
    let mut y = y_initial;
    // Vectors to store time t and solution y values
    let mut t_vals = vec![t_initial];
    let mut y_vals = vec![y_initial];

    // Loop through each step
    for _ in 0..n {
        // Calculating the four k(approximate slopes) values as per RK4 method
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h * k1 / 2.0);
        let k3 = f(t + h / 2.0, y + h * k2 / 2.0);
        let k4 = f(t + h, y + h * k3);

        // Updating y using the weighted average of slopes
        y += h * (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0;
        // Moving to the next time step
        t += h;

        // Storing the new values in the corresponding vectors
        t_vals.push(t);
        y_vals.push(y);
    }

    // Returning the vectors of time t and solution y values
    (t_vals, y_vals)
}

/// Function to compute the derivative(or the function f(t,y) where y'=f(t,y)) y' = 1 - t^2 + y
///
///     Parameters
///     t: current time
///     y: current value of y
///
///     Returns
///     The value of the derivative at (t, y)
fn f(t: f64, y: f64) -> f64 {
    1.0 - t.powi(2) + y
}
