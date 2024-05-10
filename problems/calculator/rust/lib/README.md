# Computer Algebra System (CAS) | Rust Solution

## Problem

The high level goal is to write a TI-89 Titanium clone.

This library should expose an ergonomic API with the following components:

- A high level AST
- A high level, ergonomic API for solving and re-writing ASTs
- A parser that consumes a String and returns an AST
- A way to render an AST to plain text
- A way to render an AST to LaTex

The AST re-writing API should be able to:

- Re-write in a simplified form
- Solve for a given variable
- Evaluate to a definite value
- Approximate to a definite value

The AST itself should support:

- Abstract primitives
  - Integer values (preserve integer values where possible)
  - Decimal values
  - Boolean values
  - Variables
  - Functions
- Constructed objects
  - Sets
  - Vectors
  - Matrices
  - Points
- Common numerical operations (add, subtract, etc. etc.)
- Common boolean operations (and, or, not, etc.)
- Common set operations (union, difference, intersection, etc.)
- Common vector operations (dot product, cross product, norm, etc.)
- Common matrix operations (determinant, transpose, eigenvalues, etc.)
- Common relational attributions (less than, equal to, greater than, etc.)
- Substantial support for differential calculus problems
- Substantial support for integral calculus problems
- Some preliminary support for differential equations

The parser must accept plain text input, similar to that of the TI-89's.
An example would be: `5x+30/(9/10+x^2-sqrt(y))`. Optionally, it may support latex input, i.e., $\displaystyle 5x+\frac{30}{\frac{9}{10}+x^2-\sqrt{y}}$.

## Methodology

I'll update this as I work, but let's just sketch out a preliminary plan for now.

- Develop a complete problem specification
  - This will hopefully help me better understand the scope and limits of my program
  - Hopefully it will also help with the next step
- Develop a formal grammar for mathematical expressions
- Develop an AST type
- Develop a renderer for the AST
- Develop a parser for the AST
- Develop the AST API

My main concern with this approach is that I may find, late in development, that my AST type system is actually non-ideal, or difficult to work with, in the abstract re-writing and solving stages. A bonus of using Rust is that refactoring is relatively comfortable and safe, though certainly time-consuming. If I want to continue with this approach, I'll need to think through the AST design carefully. My previous attempts may be helpful in guiding type design as well.

However, I do think I ultimately have to prepare, and assume, that at some point I will need to significantly refactor my type system and API. The goal will then be to make this as painless a transition as possible.

My other main concern is that, once I get to the abstract re-writing stage, I might have such a complicated type that I struggle actually implementing anything. A different approach might start with a minimal AST and minimal API and build up over time instead. However, the one upside of knowing my full AST ahead of time is that I might be better able to avoid issues where I unknowingly rely on patterns that don't generalize.

Regardless the path I chose, the first four steps are going to be:

1. Develop a complete problem specification
1. Develop a formal grammar (really just an extension of the previous step)
1. Develop an AST type
1. Develop a renderer for the AST (so I can see if its actually working!)

From there, it's an open question, whether I should focus on parsing or re-writing first. Really, it probably doesn't matter. I'll come back here once I've completed the first four steps and decide then.

## Formal Specification | Part 1, Grammar

$G=(N, \Sigma, P, S)$

### 1.1 Terminals

$\Sigma = \mathbb{Z} \ \cup \ \text{ASCII alphabetic} \ \cup \ \text{Operators} \ \cup \ \text{Relaters} \ \cup \ \text{Delimiters} \ \cup \ \text{Constructs} \ \cup \ \text{Special Symbols}$

$\text{Operators} = \begin{Bmatrix} + & - & \ast & / & \wedge \end{Bmatrix}$

$\text{Relaters} = \begin{Bmatrix}
=&\equiv&\cong&\sim&\simeq&\approx&\asymp&\propto\\
\neq&\ncong&\nsim\\
\lt&\le&\gt&\ge\\
\nleq&\nless&\ngtr&\ngeq\\
\cup&\cap&\subseteq&\subset&\in&\ni&\supset&\supseteq\\
\nsubseteq&\nsupseteq&\notin\\
\Leftarrow&\leftarrow&\rightarrow&\Rightarrow&\mapsto&\leftrightarrows\\
\iff&\exists&\nexists
\end{Bmatrix}$

$\text{Delimiters}=\begin{Bmatrix}
(&)&[&]&\{&\}&\langle&\rangle&|&\|&\lfloor&\rfloor&\lceil&\rceil&
\end{Bmatrix}$

$\text{Constructs}=\begin{Bmatrix}
\sum&\prod&\coprod&\int&\forall
\end{Bmatrix}$

$\text{Special Symbols}=\begin{Bmatrix}
\infty&\nabla&\partial&\Im&\Re&\emptyset&\varnothing&\imath\\
\alpha&\beta&\gamma&\chi&\delta&\epsilon&\eta&\kappa\\
\lambda&\mu&\nu&\omega&\phi&\pi&\rho&\sigma\\
\tau&\theta&\upsilon&\xi&\zeta\\
\digamma&\varepsilon&\varkappa&\varphi&\delta\\
\Gamma&\Lambda&\Omega&\Phi&\Pi&\Psi&\Sigma&\Theta&\Upsilon&\Xi&\aleph\\
\text{Newline}&;&,
\end{Bmatrix}$

### 1.2 Non-terminals

$N = \begin{Bmatrix}
\text{Paragraph}\\
E \sim \text{Expression}\\
\text{Infix}\\
\text{Prefix}\\
W \sim \text{Wrapped}\\
V \sim \text{Value}\\
\text{Number}\\
\text{Variable}\\
\mathcal{P} \sim \text{Polynomial}\\
\mathcal{M} \sim \text{Matrix}
\end{Bmatrix}$

### 1.3 Production rules

$e \in \text{Delimiter}$

$p \in \text{Constructs}$

$j \in \text{Operators} \cup \text{Relaters}$

#### Root

$S \to \epsilon$

$S \to \text{Paragraph}$

$S \to \text{Paragraph} \ \text{Newline} \ S$

#### Paragraph

$\text{Paragraph} \to E$

$\text{Paragraph} \to E; \text{Paragraph}$

#### Expression

$E \to \text{Infix}$

$E \to \text{Prefix}$

$E \to W$

#### Wrapped

$W \to e \ E \ e$

$W \to V$

#### Infix

$\text{Infix} \to E \ j \ E$

#### Prefix

$\text{Prefix} \to p \ \_ \ W \ \bold{\hat{\space}} \ W \ E$

TODO: fix forall issue

#### Value

$V \to \text{Number}$

$V \to \text{Variable}$

#### Number

$\text{Number} \to \mathbb{Z}$

$\text{Number} \to \mathbb{Z}.\mathbb{Z}$

#### Variable

$\text{Variable} \to \text{ASCII alphabetic string}$

$\text{Variable} \to \text{Special Symbols}$

#### Basic, informal, polynomial

$k \in \mathbb{Z}$

$c \in \text{Number}$

$v \in \text{Variable}$

$\mathcal{P}_k \to \epsilon$

$\mathcal{P}_k \to cv^k$

$\mathcal{P}_k \to \mathcal{P}_k + \mathcal{P}_{k-1}$

### 1.3 Stable forms

$S(V) = V \ \text{(all inputs of type Value, are stable)}$

## Formal Specification | Part 2, API

## AST Type Design

## Rendering | Part 1, Plain Text
