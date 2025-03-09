#let style(doc) = [
  #set text(lang: "ru", font: "PT Serif", hyphenate: true)

  #set heading(numbering: "1.1.")

  #show heading.where(level: 1): it => {
    pagebreak(weak: true)
    it
    v(1em, weak: true)
  }
  #show heading.where(level: 2): it => {
    pagebreak(weak: true)
    it
    v(1em, weak: true)
  }

  #show link: underline

  #set page("a4")

  #set bibliography(style: "gost-r-705-2008-numeric")

  #doc
]
