# Burgers' Equation Solver using RK4 Method

This folder contains a Python script to numerically solve the Burgers' equation using the fourth-order Runge-Kutta (RK4) method. Burgers' equation is a fundamental partial differential equation from fluid mechanics which can describe shock waves and boundary layer phenomena in viscous flows.

## Description

The script implements the RK4 method to solve the Burgers' equation over a spatial domain with periodic boundary conditions. The numerical solution approach involves discretizing the spatial domain and then improving the solution in time through the RK4 iterative scheme.


## Configuration

Here are the different parameters used in code:

- **`L`**: Length of the spatial domain.
- **`N`**: Number of grid points in the spatial domain.
- **`dx`**: Distance between grid points, derived from `L` and `N`.
- **`nu`**: Viscosity coefficient.
- **`dt`**: Time step size.
- **`T`**: Total number of time steps calculated based on the desired simulation time and `dt`.

Adjusting these parameters will affect the stability and accuracy of the numerical solution. Reducing `dt` and increasing `N` can improve the stability.
