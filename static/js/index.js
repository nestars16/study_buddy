import {highlight,resizeTextarea,enableTabbing,updateLineNumbers} from './editorActions.js'
import {downloadMarkdownToPDF, open_modal, closeModal, toggleMode, submitButtonAction, checkForLogInUser} from './api.js'

"use strict";

const url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
const webSocketConnection = new WebSocket(url.href);

let currentMode = '';
const highlightEl = document.getElementById("highlight");
const editor = document.getElementById("editor");
const display = document.getElementById("markdown-display");  
let refreshMathTexCounter = 10;

document.addEventListener("DOMContentLoaded", () => {

    const initialSetup = async () => {

        const toggleModesButton = document.getElementById("toggle-modes");
        const downloadButton = document.getElementById("download");
        const registerButton = document.getElementById("sign-up");
        const logInButton = document.getElementById("log-in");
        const closeModalButtons = document.querySelectorAll(".button-close");
        const submitForm = document.getElementById("user-form");
        const submitButton  = document.getElementById("submit-button");
        const logOutButton = document.getElementById("log-out");

        //submitForm.onsubmit = async (event) => {
        //    event.preventDefault();
        //    await submitButtonAction();
        //};

        submitButton.onclick = async (event) => {
            event.preventDefault();
            submitButton.classList.add("hidden"); 
            document.getElementById("loader").classList.remove("hidden"); 
            await submitButtonAction();
        }

        toggleModesButton.onclick = () => {
            currentMode = toggleMode(currentMode);
        }

        logOutButton.onclick = () => {
        }

        registerButton.onclick = () => {
            open_modal("Register",display);
        }

        logInButton.onclick = () => {
            open_modal("Log In",display);
        }

        for (let button of closeModalButtons) {
            button.onclick = () => {
                closeModal(display);
            }
        }

        editor.setAttribute("data-initialized",true);

        webSocketConnection.onmessage = (event) => {
            display.innerHTML = event.data;
        }
        
        webSocketConnection.onopen = () => {
            webSocketConnection.send(editor.value);
        }

        editor.oninput = () => {
            highlight(editor,highlightEl);
            webSocketConnection.send(editor.value);
            resizeTextarea(editor);
            refreshMathTexCounter += 1;         
        }

        editor.onkeyup = updateLineNumbers;

        editor.onkeydown = enableTabbing; 

        downloadButton.onclick = async () => {
            await downloadMarkdownToPDF(display.innerHTML, currentMode);
        }

        resizeTextarea(editor);
    }
    
    resizeTextarea(editor);
    highlight(editor,highlight);
    initialSetup();
    checkForLogInUser();
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
