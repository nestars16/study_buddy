import {highlight,resizeTextarea,enableTabbing,updateLineNumbers} from './editorActions.js'
import {downloadMarkdownToPDF} from './api.js'

"use strict";

const url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
const webSocketConnection = new WebSocket(url.href);

const highlightEl = document.getElementById("highlight");
const editor = document.getElementById("editor");
const display = document.getElementById("markdown-display");  
let unrenderedSnapshot = "";
let refreshMathTexCounter = 10;

document.addEventListener("DOMContentLoaded", () => {


    const initialSetup = async () => {

        const downloadButton = document.getElementById("download");

        editor.setAttribute("data-initialized",true);

        webSocketConnection.onmessage = (event) => {
            display.innerHTML = event.data;
            unrenderedSnapshot = event.data;
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

            await downloadMarkdownToPDF(display.innerHTML, "dark");
        }

        resizeTextarea(editor);
    }

    resizeTextarea(editor);
    highlight(editor,highlight);
    initialSetup();
    editor.value = "";

    setInterval(() => {

        if(refreshMathTexCounter === 0) {
            renderMathInElement(document.body, {
              // customised options
              // • auto-render specific keys, e.g.:
              delimiters: [
                  {left: '$$', right: '$$', display: true},
                  {left: '$', right: '$', display: false},
                  {left: '\\(', right: '\\)', display: false},
                  {left: '\\[', right: '\\]', display: true}
              ],
              // • rendering keys, e.g.:
              throwOnError : false,
        });
        }

        if(refreshMathTexCounter > 0) {
            refreshMathTexCounter -= 1;
        }
    }, 150);

})
