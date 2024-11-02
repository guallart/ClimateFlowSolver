# Navier-Stokes Equations
The instantaneous continuity equation, momentum equation and energy equation for a compressible fluid [can be written](https://www.cfd-online.com/Wiki/Navier-Stokes_equations) as: 

$$
\frac{\partial \rho}{\partial t} +
\frac{\partial}{\partial x_j}\left[ \rho u_j \right] = 0
$$

$$
\frac{\partial}{\partial t}\left( \rho u_i \right) +
\frac{\partial}{\partial x_j}
\left[ \rho u_i u_j + p \delta_{ij} - \tau_{ji} \right] = 0, \quad i=1,2,3
$$

$$
\frac{\partial}{\partial t}\left( \rho e_0 \right) +
\frac{\partial}{\partial x_j}
\left[ \rho u_j e_0 + u_j p - u_i \tau_{ij} \right] = 0
$$

For a Newtonian fluid, assuming Stokes Law for mono-atomic gases, the viscous stress is given by: 

$$
\tau_{ij} = 2 \mu S_{ij}^*
$$

Where the trace-less viscous strain-rate is defined by: 
$$
S_{ij}^* \equiv
 \frac{1}{2} \left(\frac{\partial u_i}{\partial x_j} +
                \frac{\partial u_j}{\partial x_i} \right) -
                \frac{1}{3} \frac{\partial u_k}{\partial x_k} \delta_{ij}
$$

To close these equations it is also necessary to specify an equation of state. Assuming a calorically perfect gas the following relations are valid:

$$
\gamma \equiv \frac{C_p}{C_v} ~~,~~
p = \rho R T ~~,~~
e = C_v T ~~,~~
C_p - C_v = R
$$
Where $\gamma$, $C_p$, $C_v$ and R are constant.
The total energy $e_0$ is defined by: 

$$
e_0 \equiv e + \frac{u_k u_k}{2}
$$

# Schemes

- **Cell volume calculation**: using divergence theorem the cell volume [can be computed](https://www.youtube.com/watch?v=x2CsJUE8bZo&list=PLnJ8lIgfDbkp5DtCPtP2rcqEEUJk-PM8N&index=3) as: 
  $$V_P = \sum_{\text{faces}} \frac{1}{2} (\boldsymbol{x}_f \cdot \hat{\boldsymbol{n}}) A_f$$
  where $\boldsymbol{x}_f$ is measured from the cell centroid of the cell, not the global origin.

- **Gradient scheme**: [least squares](https://www.youtube.com/watch?v=7ymFkxx2R_k&list=PLnJ8lIgfDbkp5DtCPtP2rcqEEUJk-PM8N&index=4)
  $$\int_V  \nabla p \, dV \approx G^{-1} \boldsymbol{d}^T (p_N - p_P); \hspace{1cm} G = \boldsymbol{d}^T \boldsymbol{d} ; \hspace{1cm} \boldsymbol{d} = \boldsymbol{d}_N-\boldsymbol{d}_P$$
  with $P$ refereing to the current cell and $N$ to the neighbourning cell. $G$ can be computed only once at the beginning.
  Can be extended with a weight on each calculation for better performance. This is specially useful on thin cells.

  