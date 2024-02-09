
#let problem_counter = counter("problem")

#let prob(body) = {
  // let current_problem = problem_counter.step()
  [== Problem #problem_counter.step() #problem_counter.display()]
  block(fill:rgb(250, 255, 250),
   width: 100%,
   inset:8pt,
   radius: 4pt,
   stroke:rgb(31, 199, 31),
   body)
  }

// Some math operators
#let prox = [#math.op("prox")]
#let proj = [#math.op("proj")]
#let argmin = [#math.arg]+[#math.min]

#let logo_esir = "esir.png"
#let logo_univ = "univ.png"

#let to-string(content) = {
  if content.has("text") {
    content.text
  } else if content.has("children") {
    content.children.map(to-string).join("")
  } else if content.has("body") {
    to-string(content.body)
  } else if content == [ ] {
    " "
  }
}



// Initiate the document title, author...
#let assignment_class(title, author, course_id, professor_name, due_time, body) = {
  set document(title: title, author: author)
  set heading(numbering: "1.")
  set page(
    paper:"us-letter", 
    header: locate( 
        loc => if (
            counter(page).at(loc).first()==1) { none } 
        else if (counter(page).at(loc).first()==2) { align(right, 
              [ *#course_id: #title*  ]
            ) }
        else { 
          align(right, 
            [ *#course_id: #title* ]
          ) 
        }
    ), 
    footer: locate(loc => {
      if counter(page).at(loc).first() != 1 {
      let page_number = counter(page).at(loc).first()-1
      let total_pages = counter(page).final(loc).last()-1
      align(center)[Page #page_number of #total_pages]
      }
    }))

  align(center, [
    #grid(
      columns: (30%,30%),
      align(center, image(logo_esir, width: 100%)),
      align(center, image(logo_univ, width: 100%)),
  )])
  block(height:25%,fill:none)
  align(center, text(17pt)[
    *#course_id: #title*])
  align(center, text(10pt)[
    A rendre le #due_time])
  align(center, [_Responsable: #professor_name _])
  block(height:35%,fill:none)
  align(center)[*#author*]
  
  pagebreak(weak: false)

  body
}