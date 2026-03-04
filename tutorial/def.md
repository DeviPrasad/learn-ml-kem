
```{=latex}
\newcommand{\SixPtSp}{\vspace*{6pt}\\}
\newcommand{\OnePtSp}{\vspace*{1pt}\\}
\newcommand{\TwoPtSp}{\vspace*{2pt}\\}
\newcommand{\Red}[1]{\textcolor{red}{ #1 }}

\begin{tabular}[H]{r p{0.85\linewidth}}
$n$            & An integer value 256.\SixPtSp

$q$            & Denotes a prime integer $3329 = 2^8 \cdot 13 + 1$.\SixPtSp

$\zeta$        & Denotes the integer value 17.\\
               & This is the n-th root of unity modulo $q$ where $n = 256$.\\
               & In other words, $\zeta^{256} = 1 \Mod{q}$. \TwoPtSp
               & $\blacktriangleright $ Python Snippet 1:\OnePtSp
               & \quad\quad\texttt{{assert pow(17, 256, 3329) == 1}}\SixPtSp
$\mathbb{Z}$   & The set of integers.\SixPtSp

$\mathbb{Z}_q$ & The \textbf{\textit{field}} of integers modulo $q$.\\
               & The underlying set is \{0, 1, \ldots, 3328\}\\
               & This set admits addition and multiplication operations modulo $q$.\SixPtSp

$\mathbb{Z}_m$  & The (quotient) \textbf{\textit{ring}} of integers modulo arbitrary $m$.\\
                & The underlying set is \{0, 1, \ldots, m-1\}\\
                & This set admits addition and multiplication operations modulo $m$.\SixPtSp

$\mathbb{Z}^{n}_{m}$
               & The set of $n$-tuples over $\mathbb{Z}_m$, equipped with $\mathbb{Z}_m$-module structure.\\
               & It is a set of vectors/tuples of length $n$.\\
               & Each element of the vector/tuple is in $Z_m$.\TwoPtSp
               & $\blacktriangleright $ Example 1:\OnePtSp
               & \quad\quad $\{(0,3328,1),(1665,0,802),(3328,3328,3328),(100,200,300)\} \in \mathbb{Z}^{3}_{q}$ \TwoPtSp
               & $\blacktriangleright $ Counterexample 1:\OnePtSp
               & \quad\quad $\{(0,\Red{3329}),(\Red{65535},\Red{8802}),(0,0),(29,\Red{3330}),(100,200)\} \not{\in} \mathbb{Z}^{2}_{q}$
               \SixPtSp


$\mathbb{Z}_{q}[X]$
               & The ring of polynomials of an arbitrary degree with coefficients in $\mathbb{Z}_q$.\\
               & Addition and multiplication are usual polynomial operations.\\
               & The coefficients are reduced modulo $q$.\\
               & It is a polynomial ring over a field - coefficients are members of $\mathbb{Z}_q$.\SixPtSp


$f \in \mathbb{Z}_q$
               & A polynomial of the form $f = a_0 + a_1 X + a_2 X + \ldots + a_{m} X^{m}$,
                 where $a_j \in \mathbb{Z}_q$, and $m$ is arbitrary.\SixPtSp

$R_q$          & The ring $Z_q[X]/(X^n+1)$ consists of polynomials of the form $f$.\\
               & This ring admits addition and multiplication of polynomials modulo $X^n + 1$. \\
               & $f \in R_q$ is a polynomial in the ring $R_q$.\SixPtSp

$f \in R_q$    & is a polynomial of the form $f = a_0 + a_1 X + a_2 X + \ldots + a_{255} X^{255}$
                 where $a_j \in \mathbb{Z}_q$.\SixPtSp
\end{tabular}


In FIPS 203, $q$ is prime, and therefore $Z_q$ is a \textit{field}, i.e., every element has a multiplicative inverse.



```