# Atomic

A personal project to simulate the hydrogen atom by numerically solving the Schrödinger equation in Julia.

## Goal

Solve the time-independent Schrödinger equation for the hydrogen atom:

$$\hat{H}\psi = E\psi$$

and compute/visualize the resulting wavefunctions and energy levels.

## Approach

- Work in spherical coordinates and exploit the separation of variables: $\psi(r,\theta,\phi) = R(r) Y_l^m(\theta,\phi)$
- Solve the radial equation numerically (e.g. finite differences or shooting method)
- Use the analytical spherical harmonics for the angular part
- Visualize electron probability densities $|\psi|^2$ for various quantum numbers $(n, l, m)$

## Tech

- **Julia** — numerical solving
- **Pluto** — interactive notebooks for exploration and visualization

## Status

Early stage / work in progress.
