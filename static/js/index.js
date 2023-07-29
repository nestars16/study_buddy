import {highlight,resizeTextarea,enableTabbing,updateLineNumbers, openModal, closeModal, toggleMode, openDocumentTitleModal, showUserPosts} from './editorActions.js'
import {downloadMarkdownToPDF, submitButtonAction, checkForLogInUser, LogOut, createDocument, fetchUserDocuments,fetchCurrentDocumentContent} from './api.js'

"use strict";

const url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
const webSocketConnection = new WebSocket(url.href);

let currentMode = 'dark';
const highlightEl = document.getElementById("highlight");
const editor = document.getElementById("editor");
const display = document.getElementById("markdown-display");  
let currentDocuments = [];
let refreshMathTexCounter = 10;

    const initialSetup = async () => {

        const toggleModesButton = document.getElementById("toggle-modes");
        const downloadButton = document.getElementById("download");
        const registerButton = document.getElementById("sign-up");
        const logInButton = document.getElementById("log-in");
        const closeModalButtons = document.querySelectorAll(".button-close");
        const submitButton  = document.getElementById("submit-button");
        const logOutButton = document.getElementById("log-out");
        const addButton = document.getElementById("add-document");
        const documentTitleSubmit = document.getElementById("document-title-submit");
        const documentTitleForm = document.getElementById("document-title-form");
        const showAllDocumentsButton = document.getElementById("all-documents");

        showAllDocumentsButton.onclick = async () => {
            currentDocuments = await fetchUserDocuments();

            showUserPosts(currentDocuments,currentMode,fetchCurrentDocumentContent);
        }

        addButton.onclick = () => {
           openDocumentTitleModal(); 
        }

        documentTitleForm.onsubmit = (event) => {
           event.preventDefault(); 
        }

        documentTitleSubmit.onclick = async () => {
            
            if(!document.getElementById("document-title-field").value) {
                return;
            }

            await createDocument(document.getElementById("document-title-field").value);
        }

        submitButton.onclick = async (event) => {
            event.preventDefault();
            await submitButtonAction();
        }

        toggleModesButton.onclick = () => {
            currentMode = toggleMode(currentMode);
        }

        logOutButton.onclick = async () => {
            await LogOut();
        }

        registerButton.onclick = () => {
            openModal("Register");
        }

        logInButton.onclick = () => {
            openModal("Log In");
        }

        for (let button of closeModalButtons) {
            console.log(button);
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

document.addEventListener("DOMContentLoaded", () => {

    
    resizeTextarea(editor);
    highlight(editor,highlight);
    initialSetup();
    checkForLogInUser();
    editor.value = "";

    setInterval(() => {

        if(refreshMathTexCounter === 0) {
            renderMathInElement(document.body, {
              delimiters: [
                  {left: '$$', right: '$$', display: true},
                  {left: '$', right: '$', display: false},
                  {left: '\\(', right: '\\)', display: false},
                  {left: '\\[', right: '\\]', display: true}
              ],
              throwOnError : false,
        });
        }

        if(refreshMathTexCounter > 0) {
            refreshMathTexCounter -= 1;
        }
    }, 150);

})
