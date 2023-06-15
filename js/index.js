"use strict";

const url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
const webSocketConnection = new WebSocket(url.href);

const script = document.createElement('script');
script.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js';
script.async = true;
document.head.appendChild(script);

const highlight = (editor, highlightEl) => {
  window.requestAnimationFrame(() => {
    const highlighted = hljs.highlight(
      editor.value, 
        {language : "markdown"}
    ).value;
    highlightEl.innerHTML = highlighted;
  });
};

const resizeTextarea = (textArea) => {
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



document.addEventListener("DOMContentLoaded", () => {

   let setUpMathJax = () => {

        let appendMathJaxScript = () => {
        };

        window.MathJax = {
          tex: {
            inlineMath: [['$', '$'], ['\\(', '\\)']],
            extensions :['noErrors.js', 'noUndefined.js']
          },
          svg: {
            fontCache: 'global'
          }
        };
        
    }


    setUpMathJax();
    const highlightEl = document.getElementById("highlight");
    const editor = document.getElementById("editor");


    const initialSetup = async () => {

        const lineNumbers = document.querySelector(".line-numbers");

        editor.setAttribute("data-initialized",true);

        webSocketConnection.onmessage = (event) => {
            document.getElementById("markdown-display").innerHTML = event.data;  
            window.MathJax.typeset();
        }
        
        webSocketConnection.onopen = (event) => {
            webSocketConnection.send(editor.value);
        }

        editor.oninput = (event) => {
            highlight(editor,highlightEl);
            webSocketConnection.send(editor.value);
            resizeTextarea(editor);
            window.MathJax.typeset();
        }

        editor.onkeyup = (event) => {
            const numberOfLines = event.target.value.split('\n').length

            lineNumbers.innerHTML = Array(numberOfLines)
                .fill('<span></span>')
                .join('')
        }

        editor.onkeydown = (event) => {
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

        resizeTextarea(editor);
    }

    resizeTextarea(editor);
    highlight(editor,highlight);
    initialSetup();

})
