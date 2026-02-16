<head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/katex.min.css" integrity="sha384-Pu5+C18nP5dwykLJOhd2U4Xen7rjScHN/qusop27hdd2drI+lL5KvX7YntvT8yew" crossorigin="anonymous">
    <!-- The loading of KaTeX is deferred to speed up page rendering -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/katex.min.js" integrity="sha384-2B8pfmZZ6JlVoScJm/5hQfNS2TI/6hPqDZInzzPc8oHpN5SgeNOf4LzREO6p5YtZ" crossorigin="anonymous"></script>
    <!-- To automatically render math in text elements, include the auto-render extension: -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/contrib/auto-render.min.js" integrity="sha384-hCXGrW6PitJEwbkoStFjeJxv+fSOOQKOPbJxSfM6G5sWZjAyWhXiTIIAmQqnlLlh" crossorigin="anonymous"
        onload="renderMathInElement(document.body);"></script>
    <link rel="stylesheet" type="text/css" href="https://tikzjax.com/v1/fonts.css">
    <script src="https://tikzjax.com/v1/tikzjax.js"></script>
</head>

# The Annotated ML KEM Scheme
```{=latex}
\newcommand{\TComment}[1]{\qquad #1}
\newcommand{\LComment}[1]{\operatorname{---} #1}
\newcommand{\plusplus}{\mathbin{\raisebox{0.15ex}{\scalebox{0.85}{$+\mkern-1mu+$}}}}
\renewcommand{\algorithmicdo}{}
```

## The Annotated Key Generation Algorithm
```{=latex}
% \newcommand{\TComment}[1]{\qquad #1}
\algtext*{Do}
\begin{algorithm}
\caption*{\textbf{Annotated Algorithm 13} $\operatorname{K-PKE.KeyGen}(d: \mathbb{B}^{32}) \rightarrow (ek_{PKE}: \mathbb{B}^{384k+32}, \, dk_{PKE}: \mathbb{B}^{384k}$)}
\begin{algorithmic}[1]
% line 1
\State {$(\rho, \sigma) \leftarrow \operatorname{G}(d||k)$}
    {\Comment{expand 32+1 bytes to two pseudorandom 32-byte seeds}}

% line 2
\State {$N \leftarrow 0$}{}

% lines 3-7
\For{$i \leftarrow 0; \ i < k; \ i \plusplus$}
    \For{$j \leftarrow 0; \ j < k; \ j \plusplus$}
        \State{$\operatorname{A}[i,\, j]\leftarrow \operatorname{SampleNTT}(\rho \, || \, j \, || \, i)$}{}
    \EndFor
\EndFor

% lines 8-11
\For{$i \leftarrow 0; \ i < k; \ i \plusplus$}
    \State{$\operatorname{s}[i] \leftarrow \operatorname{SamplePolyCBD}_{\eta_{1}}(\operatorname{PRF}_{\eta_1}(\sigma, N))$}
    \State{$N \leftarrow N + 1$}
\EndFor

% lines 12-15
\For{$i \leftarrow 0; \ i < k; \ i \plusplus$}
    \State{$\operatorname{e}[i] \leftarrow \operatorname{SamplePolyCBD}_{\eta_{1}}(\operatorname{PRF}_{\eta_1}(\sigma, N))$}
    \State{$N \leftarrow N + 1$}
\EndFor

% line 16
\State{$\hat{\operatorname{s}} \leftarrow \operatorname{NTT}(\operatorname{s})$} {\Comment{run NTT $k$ times (once for each coordinate of s)}}

% line 17
\State{$\hat{\operatorname{e}} \leftarrow \operatorname{NTT}(\operatorname{e})$} {\Comment{run NTT $k$ times}}

% line 18
\State{$\hat{\operatorname{t}} \leftarrow \hat{\operatorname{A}} \circ \hat{\operatorname{s}} + \hat{\operatorname{e}}$} {\Comment{noisy linear system in NTT domain}}

% line 19
\State{$\operatorname{ek_{PKE}} \leftarrow \operatorname{ByteEncode_{12}} (\hat{\operatorname{t}}) \, || \, \rho  $}
    {\Comment{run $\operatorname{ByteEncode_{12}}$ $k$ times, then append $\hat{\operatorname{A}}$-seed }}

% line 20
\State{$\operatorname{dk_{PKE}} \leftarrow \operatorname{ByteEncode_{12}}(\hat{\operatorname{s}}) $}{}
    {\Comment{run $\operatorname{ByteEncode_{12}}$ $k$ times }}

% line 21
\State{\Return {$(\operatorname{ek_{PKE}, dk_{PKE}})$}}{}

\end{algorithmic}
\end{algorithm}
```
