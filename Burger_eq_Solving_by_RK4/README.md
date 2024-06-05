# Burgers' Equation Solver using RK4 Method

This folder contains a Python script to numerically `solve` and `visualize` the solution of the Burgers' equation using the fourth-order Runge-Kutta (RK4) method. Burgers' equation is a fundamental partial differential equation from fluid mechanics which can describe shock waves and boundary layer phenomena in viscous flows.

## Description

The script implements the RK4 method to solve the Burgers' equation over a spatial domain with periodic boundary conditions. The numerical solution approach involves discretizing the spatial domain and then improving the solution in time through the RK4 iterative scheme.

Burgers' equation is a fundamental partial differential equation from fluid mechanics. The equation in one dimension is given by:

$\frac{\partial u}{\partial t}$ + $u \frac{\partial u}{\partial x}$ = $\nu \frac{\partial^2 u}{\partial x^2}$

where $u$ is the velocity field, $\nu$ is the viscosity coefficient, $x$ is the spatial coordinate, and $t$ is time.

## Configuration

### Initial and Boundary Conditions

- **Initial Condition:** The initial velocity field $\( u \)$ is given by $\( \sin(\pi x) \)$, where $\( x \)$ is the spatial coordinate.
- **Boundary Condition:** Periodic boundary conditions are applied, meaning the solution wraps around the domain edges.

### Parameters:
Here are the different parameters used in code:

- **`L`**: Length of the spatial domain ($2\pi$).
- **`N`**: Number of grid points in the spatial domain (10).
- **`dx`**: Distance between grid points, derived from `L` and `N` ($\frac{L}{N-1}$).
- **`nu`**: Viscosity coefficient (0.01).
- **`dt`**: Time step size (0.1).
- **`T`**: Total number of time steps calculated based on the desired simulation time and `dt` ($\frac{1}{dt}$).

Adjusting these parameters will affect the stability and accuracy of the numerical solution. Reducing `dt` and increasing `N` can improve the stability.

### Functions:
- `burgers_equation(u, dx, nu)`: Calculates the right-hand side of Burgers' equation.
- `rk4_step(u, dx, dt, nu)`: Performs a single RK4 time step to update the solution 𝑢.

## Output:
- After running the numerical code, we should see a table of u values at different time step and spatial domain values.
- After running the visualization code, we should see a plot shows the evolution of the velocity field 𝑢 over the spatial domain at different time steps.
