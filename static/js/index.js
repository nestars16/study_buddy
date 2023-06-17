import {highlight,resizeTextarea,enableTabbing,updateLineNumbers} from './editorActions.js'
import {downloadMarkdownToPDF} from './api.js'

"use strict";

const url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
const webSocketConnection = new WebSocket(url.href);

const script = document.createElement('script');
script.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js';
document.head.appendChild(script);

const setUpMathJax = () => {
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
const highlightEl = document.getElementById("highlight");
const editor = document.getElementById("editor");
const display = document.getElementById("markdown-display");  
let refreshMathTexCounter = 0;

document.addEventListener("DOMContentLoaded", () => {

    setUpMathJax();

    const initialSetup = async () => {

        const downloadButton = document.getElementById("download");

        editor.setAttribute("data-initialized",true);

        webSocketConnection.onmessage = (event) => {
            display.innerHTML = event.data;
        }

        
        webSocketConnection.onopen = (event) => {
            webSocketConnection.send(editor.value);
        }

        editor.oninput = (event) => {
            highlight(editor,highlightEl);
            webSocketConnection.send(editor.value);
            resizeTextarea(editor);
            refreshMathTexCounter += 1;         
        }

        editor.onkeyup = updateLineNumbers;

        editor.onkeydown = enableTabbing; 

        downloadButton.onclick = async (event) => {
            console.log("clicked download");
            await downloadMarkdownToPDF(display.innerHTML, "");
        }

        resizeTextarea(editor);
    }

    resizeTextarea(editor);
    highlight(editor,highlight);
    initialSetup();
    editor.value = "";

    setInterval(() => {

        if(refreshMathTexCounter === 0) {
            window.MathJax.typeset();
        }

        if(refreshMathTexCounter > 0) {
            refreshMathTexCounter -= 1;
        }
    }, 150);

})
