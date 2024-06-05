# Fourth-Order Runge-Kutta Method (RK4) Implementation in Rust and Python

This folder implements the Fourth-Order Runge-Kutta (RK4) numerical method to solve the ordinary differential equation (ODE) given by:

$y' = 1 - t^2 + y$

with the initial condition $y(0) = 0.5$ over the interval from $t = 0$ to $t = 2$, using 10 steps.

## Description

The script implements the RK4 method to solve ordinary differential equations (ODEs) of the form `dy/dt = f(t, y)`, where `f(t, y)` is a given function.

## Functions

- `f(t, y)`: Defines the function `f(t, y)` representing the derivative of `y` with respect to `t`.
- `rk4(t_start, y0, t_end, n)`: Solves the initial value problem (IVP) for the given ODE over a specified time interval using RK4.

## Parameters
- `t_start` = 0.0  (Initial value of t)
- `t_end` = 2.0    (Final value of t)
- `n` = 10         (Number of time steps)
- `y0` = 0.5       (Initial Condition)

## Usage

We can modify the function `f(t, y)` to define the desired ODE. We can adjust the initial time (`t_start`), initial condition (`y0`), final time (`t_end`), and number of steps (`n`) as needed. Then we can run the script to obtain the solution at different time steps.

## Comparison of RK4 Numerical Solution with Exact Solution

After finding the analytical solution of the given ODE w.r.t given IC, the comparison of the Numerical and Analytical solutions have been shown by Matplotlib.
