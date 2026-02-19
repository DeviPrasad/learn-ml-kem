# The Annotated Key Generation Algorithm

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 13} $\Name{K-PKE.KeyGen}(d:\mathbb{B}^{32}) \rightarrow (ek_{PKE}:\mathbb{B}^{384k+32}, dk_{PKE}: \mathbb{B}^{384k}$)}
\begin{algorithmic}[1]

% line 1
\State {$(\rho, \sigma) \leftarrow \Name{G}(d\,||\,k)$}
    {\Comment{expand 32+1 bytes to two pseudorandom 32-byte seeds}}
    \Statex{\LComment{$ \Name{G} \doteq \Name{SHA3-512} $}}

% line 2
\State {$N \leftarrow 0$}{}

% lines 3-7
\Statex{}
\Statex \LComment{$ \HatA: \RingTqKxK $}
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$} {\Comment{generate matrix $\HatA \in \RingTqKxK $}}
    \For{$(j \leftarrow 0; \ j < k; \ j \plusplus)$}
        \State{$ \HatA[i,\, j] \leftarrow \SampleNTT(\rho \, || \, j \, || \, i)$}
            {\Comment{$j$ and $i$ are bytes 33 and 34 of the input}}
    \EndFor
\EndFor

% lines 8-11
\Statex{}
\Statex \LComment{$\NameBold{s}: \mathbb{R}^{k}_{\eta_1}$ \ is a vector of $k$ polynomials with coefficients $\in \eta_1$ }
\Statex \LComment{$\eta_1 = 2 \ for \ \Name{ML-KEM-768 \ and \ ML-KEM-1024}$}
\Statex \LComment{$\eta_1 = 3 \ for \ \Name{in ML-KEM-512}$}
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$}
    {\Comment{generate $\NameBold{s} \in \RingZqK$}}
    \State{$\NameBold{s}[i] \leftarrow \SamplePolyCbdEtaOne(\PrfEtaOne(\sigma, N)) $}
        {\Comment{$\NameBold{s}[i] \in \RingZq$ sampled from CBD }}
    \Statex{\Comment{$\Name{PRF} \doteq \Name{SHA3-SHAKE256} $}}
    \State{$N \leftarrow N + 1$}
\EndFor

% lines 12-15
\Statex{}
\Statex \LComment{$\operatorname{e}: \mathbb{R}^{k}_{\eta_1}$  \ is a vector of $k$ polynomials with coefficients $\in \eta_1 $}
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$}
    {\Comment{generate $\NameBold{e} \in \RingZqK$}}
    \State{$ \NameBold{e}[i] \leftarrow \SamplePolyCbdEtaOne(\PrfEtaOne(\sigma, N)) $}
        {\Comment{$\NameBold{e}[i] \in \RingZq$ sampled from CBD }}
    \State{$N \leftarrow N + 1$}
\EndFor

% line 16
\Statex{}
\State{$ \HatNameBold{s}:\RingTqK \leftarrow \NTT(\NameBold{s}) $}
    {\Comment{run $\NTT$ $k$ times (once for each coordinate of $\NameBold{s}$)}}

% line 17
\State{$ \HatNameBold{e}:\RingTqK \leftarrow \NTT(\NameBold{e}) $}
    {\Comment{run $\NTT$ $k$ times}}

% line 18
\State{$ \HatBoldT: \RingTqK \leftarrow \HatA \circ \HatNameBold{s} + \HatNameBold{e} $}
    {\Comment{noisy linear system in NTT domain}}
    \Statex{\LComment{$\RingTqK = \RingTqKxK \circ \RingTqK + \RingTqK$}}

% line 19
\Statex{}
\State{$ \EKPKE: \mathbb{B}^{384k+32} \leftarrow \ByteEncodeQ(\HatBoldT || \, \rho) $}
    \Statex{\Comment{run $\ByteEncodeQ$ $k$ times, then append $\HatA$-seed }}

% line 20
\State{$ \DKPKE: \mathbb{B}^{384k} \leftarrow \ByteEncodeQ(\HatBoldS) $}
    {\Comment{run $\ByteEncodeQ$ $k$ times }}

\Statex{}
% line 21
\State{$ \Return \ (\EKPKE, \DKPKE) $}

\end{algorithmic}
\end{algorithm}
```

# The Annotated K-PKE.Encrypt Algorithm

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 14} $\Name{K-PKE.Encrypt}(\EKPKE:\mathbb{B}^{384k+32}, m: \mathbb{B}^{32}, r:\mathbb{B}^{32}) \rightarrow c:\mathbb{B}^{32(d_{u}k + d_v)}$}
\begin{algorithmic}[1]

% line 1
\State {$N \leftarrow 0$}{}

% line 2
\State {$ \HatBoldT:(\RingZq)^k \leftarrow \ByteDecodeQ(\EKPKE[0:384k])$}
    {\Comment{run $\ByteDecodeQ$ $k$ times to decode $\HatBoldT$ }}

%line 3
\State{$\rho \leftarrow \EKPKE[384k:384k+32]$} {\Comment{extract 32 bytes seed from $\EKPKE$}}

%lines 4-8
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$}
    {\Comment{re-generate matrix $\HatA \in \RingTqKxK $ sampled in Alg. 13}}
    \For{$(j \leftarrow 0; \ j < k; \ j \plusplus)$}
        \State{$\HatA[i,\, j] \leftarrow \SampleNTT(\rho \, || \, j \, || \, i)$}{}
            {\Comment{$j$ and $i$ are bytes 33 and 34 of the input}}
    \EndFor
\EndFor


%lines 9-12
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$}
    {\Comment{generate $\NameBold{y} \in \RingZqK $}}
    \State{$ \NameBold{y}[i] \leftarrow \SamplePolyCbdEtaOne(\PrfEtaOne(r, N)) $}
        \Comment{$\NameBold{y}[i] \in \RingZq$ sampled from CBD}
    \State{$ N \leftarrow N + 1 $}
\EndFor

%lines 13-16
\For{$(i \leftarrow 0; \ i < k; \ i \plusplus)$}
    {\Comment{generate $\NameBold{e_1} \in \RingZqK $}}
    \State{$ \NameBold{e_1}[i] \leftarrow \SamplePolyCbdEtaTwo(\PrfEtaTwo(r, N)) $}
    \State{$ N \leftarrow N + 1 $}
\EndFor

%lines 17
\State{$ e_2 \leftarrow \SamplePolyCbdEtaTwo(\PrfEtaTwo(r, N)) $}
    {\Comment{sample $e_2 \in \RingZq$ from CBD}}

%lines 18
\State{$ \HatBoldY \leftarrow \NTT(\BoldY) $}
    \Comment{run $\NTT$ $k$ times}
%lines 19
\State{$ \mathbf{u} \leftarrow \NTTInv(\HatA^{\T} \circ \HatBoldY) + \Name{\mathbf{e_1}} $}
    \Comment{run $\NTTInv$ $k$ times}

%lines 20
\State{$\mu \leftarrow \DecompressOne(\ByteDecodeOne(m)) $}

%lines 21
\State{$v \leftarrow \NTTInv(\HatBoldT^{\T} \circ \HatBoldY) + e_2 + \mu$}
    \Comment{encode plaintext $m$ into polynomial $v$}

%lines 22
\State {$c_1 \leftarrow \ByteDecodeDU(\CompressDU(\Name{\mathbf{u}}))$}
\Comment{run $\ByteDecodeDU$ and $\CompressDU$ $k$ times }

%lines 23
\State {$c_2 \leftarrow \ByteDecodeDV(\CompressDV(v))$}

%lines 24
\State {\Return{$c \leftarrow (c_1||c_2)$}}

\end{algorithmic}
\end{algorithm}
```

# The Annotated K-PKE.Decrypt Algorithm

```{=latex}
\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 15} $\Name{K-PKE.Decrypt}(\DKPKE:\mathbb{B}^{384k}, c:\mathbb{B}^{32(d_u k + d_v)}) \rightarrow m: \mathbb{B}^{32}$}
\begin{algorithmic}[1]

% line 1
\State {$c_1 \leftarrow c[0 : 32 d_u k] $}

%line 2
\State {$c_2 \leftarrow c[32 d_u k : 32 (d_u k + d_v)] $}

%line 3
\State{$ \NameBold{u}' \leftarrow \DecompressDU(\ByteDecodeDU(c_1)) $}

%line 4
\State{$ v' \leftarrow \DecompressDV(\ByteDecodeDV(c_2)) $}

%line 5
\State{$ \HatBoldS \leftarrow \ByteDecodeQ(\DKPKE) $}
    \Comment{run $\ByteDecodeQ$ $k$ times}

%line 6
\State{$ w \leftarrow v' - \NTTInv(\HatBoldS^{\T} \circ \NTT(\NameBold{u'})) $}
    \Comment{run $\NTT$ $k$ times; run $\NTTInv$ once}
%line 7
\State{$ m \leftarrow \ByteEncodeOne(\CompressOne(w))$}
    {\Comment{decode plaintext $m$ from polynomial $v$}}

% line 8
\State{\Return m}

\end{algorithmic}
\end{algorithm}

```