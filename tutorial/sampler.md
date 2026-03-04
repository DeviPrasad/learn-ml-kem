## Compression and Decompression Algorithms.

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm} $\Name{Compress}_d(x: \Zq) \rightarrow y:\ZTwoPowD $}
\begin{algorithmic}[1]

\Statex{\LComment{$ d < 12 $}}
\State{\Return{$ \left\lceil (2^d\;\!/\;\!q) \right\rfloor \cdot x \Mod{2^d} $}}

\end{algorithmic}
\end{algorithm}
```

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm} $\Name{Decompress}_d(y: \ZTwoPowD) \rightarrow x:\Zq $}
\begin{algorithmic}[1]

\Statex{\LComment{$ d < 12 $}}
\State{\Return{$ \left\lceil (q\;\!/\;\!2^d) \right\rfloor \cdot y $}}

\end{algorithmic}
\end{algorithm}
```


## Algorithm 5. ByteEncode.

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 5} $\Name{ByteEncode}_d(F:\RingZm) \rightarrow B:\mathbb{B}^{32} $}

\begin{algorithmic}[1]
\Statex{ Encodes an array of \textit{d}-bit integers into a byte array for $1 \le d \le 12 $}
\vspace*{1pt}
\Statex{integer array $F: \RingZm$, where $m = 2^d$ if $d < 12$, and $m = q$ if $d = 12 $}
\vspace*{1pt}
%lines 1-7
\For ($ i \is 0; i < 256; i\plusplus $)
    \State{$ a \is F[i]$}
        {\Comment{$ a \in \Zm $}}
    \For ($ j \is 0; j < d; j\plusplus $)
        \State{$ b[i \cdot d + j] \is a \Mod{2} $}
            \Comment{$ b \in \{0, 1\}^{256 \cdot d}$}
        \State{$ a \is (a - b[i \cdot d + j])/2 $}
            \Comment{note $a - b[i \cdot d + j]$ \ is always even}
    \EndFor
\EndFor

%line 8
\State{$ B \is \BitsToBytes(b) $}

%line 9
\State{\Return $B$}

\end{algorithmic}
\end{algorithm}
```

## Algorithm 6. ByteDecode.
```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 6} $\Name{ByteDecode}_d(B:\mathbb{B}^{32}) \rightarrow F:\RingZm $}
\begin{algorithmic}[1]
\Statex{Decodes a byte array into an array of \textit{d}-bit integers for $1 \le d \le 12 $}
\vspace*{1pt}
\Statex{integer array $F: \RingZm$, where $m = 2^d$ if $d < 12$, and $m = q$ if $d = 12 $}
\vspace*{1pt}

%line 1
\State{$ b \is \BytesToBits(B) $}

%lines 2-4
\For ($ i \is 0; i < 256; i\plusplus $)
    \State{$F[i] \is \sum_{j \is 0}^{d-1} {b[i \cdot d + j] \cdot 2^j \Mod{m}} $}
\EndFor

%line 5
\State{\Return $F$ }
\end{algorithmic}
\end{algorithm}
```


## Algorithm 7: SampleNTT.

```{=latex}
\newcommand{\AH}{\ensuremath{\widehat{a}\;\!}}
\newcommand{\Ctx}{\mathrm{ctx}}
\newcommand{\XOFInit}{\mathrm{XOF.Init}}
\newcommand{\XOFAbsorb}[2]{\mathrm{XOF.Absorb}(#1, #2)}
\newcommand{\XOFSqueeze}[2]{\mathrm{XOF.Squeeze}(#1, #2)}

\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 7} $\Name{SampleNTT}(B:\mathbb{B}^{34}) \rightarrow (\widehat{a}:\RingTq$)}
\begin{algorithmic}[1]

\Statex{\LComment{input byte array $B \in \mathbb{B}^{34}$ is a 32-byte seed along with two indices}}
\Statex{\LComment{output $\widehat{a} \in \RingTq$ is an array of coefficients of the NTT of a polynomial}}
\vspace*{2pt}

%line 1
\State{$ \Ctx \is \XOFInit() $}

%line 2
\State{$ \Ctx \is \XOFAbsorb{\Ctx}{\Param{B}} $}
    \Comment{input the given byte array into XOF}

%line 3
\State{$ j \is 0 $}

%line 4
\While {$ j < 256$}

    %line 5
    \State{$ (\Ctx, C) \is \XOFSqueeze{\Ctx}{3} $}
        \Comment{get a fresh 3-byte array $C$ from XOF}

    %line 6
    \State{$ d_1 \is C[0] + 256 \cdot (C[1] \Mod{16}) $}
        \Comment{$0 \le d_1 < 2^{12}$}

    %line 7
    \State{$ d_2 \is (C[1]\,/\,16) + (16 \cdot C[2]) $}
        \Comment{$0 \le d_2 < 2^{12}$}

    %lines 8-11
    \If {$ d_1 < q$}\vspace*{1pt}
        \vspace*{1pt}
        %line 9
        \State{$ \AH[j] \is d_1 $} \Comment{$\AH \in \RingTq$}
        %line 10
        \State{$ j \is j + 1 $}
        \vspace*{1pt}
        %line 11
    \EndIf

    %lines 12-15
    \If {$d_2 < q$ \textbf{and} $j < 256$}
        \vspace*{1pt}
        %line 13
        \State{$ \AH[j] \is d_1 $} \Comment{$\AH \in \RingTq$}
        %line 14
        \State{$ j \is j + 1 $}
        \vspace*{1pt}
        %line 15
    \EndIf

%line 16
\EndWhile

%line 17
\State{\Return \AH}

\end{algorithmic}
\end{algorithm}
```

## Algorithm 8: SamplePolyCBD.

ML-KEM samples error (noise) polynomials whose coefficients are small integers. Depending on the chosen parameter set, each coefficient is drawn from a centered binomial distribution with support either [-2, 2] or [-3, 3].

Algorithm $SamplePolyCBD_\eta$ is parameterized by $\eta \in \{2, 3\}$. It receives a byte sequence $B$ of length $64 \eta$, and computes a polynomial $f$ with coefficients in the range $[-\eta, \eta]$.

The algorithm needs to compute 256 $centered$ coefficients. It computes each coefficient as the difference of two values $x$ and $y$. The value $x$ is the sum a sequence of $\eta$ number of bits, and sum the next $\eta$ number of bits yields $y$. It is easy to see that
$$
    0 \le x \le \eta \ \, \operatorname{and} \ \, 0 \le y \le \eta
$$

Therefore, their difference (x-y) satisfies the relation
$$
     -\eta \le x - y \le \eta
$$

It is easy to see that this method requires an input of size $64 \eta$ bytes to produce 256 coefficients. Exactly $2 \eta$ bits are required to to compute one coefficient of $f$ (i.e., $x$ - $y$). Therefore, we need $256 \times 2 \eta$ $bits$ in total, which is equal to $64 \eta$ $bytes$.

In line 1 of algorithm 8, parameter $B: \mathbb{B}^{64\eta}$ is converted into a sequence of $64*8*\eta$ bits. Lines 3 and 4 sum the next $\eta$ bits each giving $x$ and $y$. Line 5 computes $i^{th}$ coefficient as $x-y \operatorname{mod} q$.


```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 8} $\Name{SamplePolyCBD}_\eta(B:\mathbb{B}^{64\eta}) \rightarrow (f:\RingZq$)}
\begin{algorithmic}[1]
\Statex{\text{Takes a seed as input and outputs a pseudorandom sample from the distribution $D_\eta(R_q)$}}
\Statex{\LComment{$ \eta \in \{ 2, 3 \} $}}
\vspace*{1pt}
%line 1
\State{$ b \is \Name{BytesToBits}(\Param{B})$}

%lines 2-6
\For {$(i \is 0; i < 256; i\plusplus)$}
    \State{$ x \is \sum_{j \leftarrow 0}^{\eta-1}{b[2i\eta + j]} $} {\Comment{$ 0 \le x \le \eta$}}
    \State{$ y \is \sum_{j \leftarrow 0}^{\eta-1}{b[2i\eta + \eta + j]} $} {\Comment{$ 0 \le y \le \eta $}}
    \State{$ f[i] \is x - y \Mod{q}$}\vspace*{1pt}
        \Statex{\TComment{\ \ \LComment{$ -\eta \le (x-y) \le \eta $}} }\vspace*{1pt}
        \Statex{\TComment{\ \ \LComment{$ f[i] \in \{q-\eta, \ldots 0, 1, \eta \} $}} }\vspace*{1pt}
\EndFor

%line 7
\State{\Return $f$}
    {\Comment{$ 0 \le f[i] \le 2 \ \mathbf{or} \ (q-2) \le f[i] \le (q-1) $}}

\end{algorithmic}
\end{algorithm}
```

Algorithms \textbf{K-PKE.KeyGen} and \textbf{K-PKE.Encrypt} use $SamplePolyCBD_\eta$. \textbf{K-PKE.KeyGen} uses the parameter $\eta_1$ to produce \textit{two} noise polynomials, $s$ and $e$. \textbf{K-PKE.Encrypt} uses the parameter $\eta_1$ to sample a secret term $y$. It also uses $\eta_2$ to produce \textit{two} noise terms, $e_1$ and $e_2$.

Depending on the security parameters, $\eta_1 = 2$ or $\eta_1 = 3$. The parameter $\eta_2 = 2$ in all security settings. We can re-write $SamplePolyCBD_\eta$ as two algorithms, each specialized for the distinct value of $\eta$. This refactoring helps in understanding the implementation of \textit{centered binomial distribution} in FIPS 203. This idea is easily translated to Rust code, which is also shown below.


```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 8-A} $\Name{SamplePolyCBD}_2(B:\mathbb{B}^{128}) \rightarrow (f:\RingZq$)}
\begin{algorithmic}[1]
\Statex{$ \eta = 2 $}
\Statex{\text{Takes a seed as input and outputs a pseudorandom sample from the distribution $D_2(R_q)$}}
\vspace*{1pt}

%line 1
\State{$ b \is \Name{BytesToBits}(\Param{B})$}

%lines 2-6
\Statex{\LComment{recall $\eta = 2$}}
\For {$(i \is 0; i < 256; i\plusplus)$}
    \State{$ x \is b[4i] + b[4i+1]$} {\Comment{$ 0 \le x \le 2$}}
    \State{$ y \is b[4i+2] + b[4i+3]$} {\Comment{$ 0 \le y \le 2$}}
    \State{$ f[i] \is (x - y) \Mod{q} $}\vspace*{1pt}
        \Statex{\TComment{\ \ \LComment{$ -2 \le (x-y) \le 2 $}} }\vspace*{1pt}
        \Statex{\TComment{\ \ \LComment{$ f[i] \in \{q-2, q-1, 0, 1, 2 \} $}} }\vspace*{1pt}
\EndFor

%line 7
\State{\Return $f$}
    {\Comment{$ 0 \le f[i] \le 2 \ \mathbf{or} \ (q-2) \le f[i] \le (q-1) $}}

\end{algorithmic}
\end{algorithm}
```


```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 8-B} $\Name{SamplePolyCBD}_3(B:\mathbb{B}^{192}) \rightarrow (f:\RingZq$)}
\begin{algorithmic}[1]
\Statex{$ \eta = 3 $}
\Statex{\text{Takes a seed as input and outputs a pseudorandom sample from the distribution $D_3(R_q)$}}
\vspace*{1pt}

%line 1
\State{$ b \is \Name{BytesToBits}(\Param{B})$}

%lines 2-6
\Statex{\LComment{recall $\eta = 3$}}
\For {$(i \is 0; i < 256; i\plusplus)$}
    \State{$ x \is b[6i] + b[6i+1] + b[6i+2] $}   {\Comment{$ 0 \le x \le 3$}}
    \State{$ y \is b[6i+3] + b[6i+4] + b[6i+5] $} {\Comment{$ 0 \le y \le 3$}}
    \State{$ f[i] \is (x - y) \Mod{q} $}\vspace*{1pt}
        \Statex{\TComment{ \ \ \LComment{$-3 \le (x-y) \le 3 $}} }\vspace*{1pt}
        \Statex{\TComment{ \ \ \LComment{$f[i] \in \{ q-3, q-2, q-1, 0, 1, 2, 3 \} $}} }\vspace*{1pt}
\EndFor

%line 7
\State{\Return $f$}
    {\Comment{$ 0 \le f[i] \le 3 \ \mathbf{or} \ (q-3) \le f[i] \le (q-1) $}}

\end{algorithmic}
\end{algorithm}
```