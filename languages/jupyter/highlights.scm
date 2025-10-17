; Jupyter Notebook syntax highlighting
; Based on JSON structure with Jupyter-specific enhancements

; Standard JSON highlighting
(string) @string

(pair
  key: (string) @property.json_key)

(number) @number
(true) @boolean
(false) @boolean
(null) @constant

; Jupyter-specific highlighting
; Highlight cell types
(pair
  key: (string (string_content) @keyword)
  (#match? @keyword "^(cell_type|nbformat|nbformat_minor|metadata)$"))

; Highlight execution counts
(pair
  key: (string (string_content) @keyword)
  (#eq? @keyword "execution_count"))

; Highlight output types
(pair
  key: (string (string_content) @type)
  (#match? @type "^(output_type|name|text)$"))

; Highlight source code arrays
(pair
  key: (string (string_content) @keyword)
  (#eq? @keyword "source")
  value: (array) @embedded)
