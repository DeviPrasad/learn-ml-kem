```{=latex}
\newcommand{\AH}{\ensuremath{\widehat{a}}}
\newcommand{\is}{\leftarrow}
\newcommand{\Ctx}{\mathrm{ctx}}
\newcommand{\Param}[1]{\ensuremath{\mathit{#1}}}
\newcommand{\XOFInit}{\mathrm{XOF.Init}}

\newcommand{\XOFAbsorb}[2]{\mathrm{XOF.Absorb}(#1, #2)}
\newcommand{\XOFSqueeze}[2]{\mathrm{XOF.Squeeze}(#1, #2)}

\begin{algorithm}[H]
\caption*{\textbf{Annotated Algorithm 7} $\Name{SampleNTT}(b:\mathbb{B}^{34}) \rightarrow (\widehat{a}:\RingTq$)}
\begin{algorithmic}[1]

\Statex{\LComment{input byte array $b \in \mathbb{B}^{34}$ is a 32-byte seed along with two indices}}
\Statex{\LComment{output $\widehat{a} \in \RingTq$ is an array of coefficients of the NTT of a polynomial}}
\vspace*{2pt}

%line 1
\State{$ \Ctx \is \XOFInit() $}

%line 2
\State{$ \Ctx \is \XOFAbsorb{\Ctx}{\Param{b}} $}
    \Comment{input the given byte array into XOF}

%line 3
\State{$ j \is 0 $}

%line 4
\While {$ j < 256$}

    %line 5
    \State{$ (\Ctx, C) \is \XOFSqueeze{\Ctx}{3} $}
        \Comment{get a fresh 3-byte array $C$ from XOF}

    %line 6
    \State{$ d_1 \is C[0] + 256 \cdot (C[1] \ \mathrm{mod} 16) $}
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
