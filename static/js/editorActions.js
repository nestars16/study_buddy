
export const highlight = (editor, highlightEl) => {
  window.requestAnimationFrame(() => {
    const highlighted = hljs.highlight(
      editor.value, 
        {language : "markdown"}
    ).value;
    highlightEl.innerHTML = highlighted;
  });
};

export const resizeTextarea = (textArea) => {
  if (!textArea) {
    return;
  }


  window.requestAnimationFrame(() => {
    textArea.style.height = 0;
    if (textArea.scrollHeight > 0) {
      textArea.style.height = `${textArea.scrollHeight + 2}px`;
    }
  });
};

export const enableTabbing =  (event) => {
            if (event.key === 'Tab') {
                event.preventDefault()

                editor.setRangeText(
                  '  ',
                  editor.selectionStart,
                  editor.selectionStart,
                  'end'
                )
              }
        }

export const updateLineNumbers = (event) => {
            const numberOfLines = event.target.value.split('\n').length

            lineNumbers.innerHTML = Array(numberOfLines)
                .fill('<span></span>')
                .join('')
        }

