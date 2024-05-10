///////////////////////////////////////////////////////////////////////////////

#import "@preview/unequivocal-ams:0.1.0": ams-article, theorem, proof

///////////////////////////////////////////////////////////////////////////////

#show: ams-article.with(
  title: [Numeric Calculator Specification],
  authors: (
    (
      name: "Aidan Beil",
    ),
  ),
  bibliography: bibliography("refs.bib")
)

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////


#show outline.entry.where(
  level: 1
): it => {
  strong(it)
}


#set par(
  first-line-indent: 0pt,
)


///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

#outline()
#pagebreak()

= Overview

- Regular PEMDAS
- Trig functions and inverses
- Exponential and logarithms
- Equality and relation tests
- Basic linear algebra
  - Matrix PEMDAS
  - Determinant
  - ... Form
  - ...
- Only real numeric values; no variables

\
== Input

A single string that may, or may not, match the grammar detailed in @grammar.

\
== Output

- If the input matches the grammar:
  - A real valued, purely numeric, reduction.
- Else:
  - A friendly error message.

#pagebreak()
= Expression Grammar <grammar>

#pagebreak()

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

