import {
  highlight,
  resizeTextarea,
  enableTabbing,
  updateLineNumbers,
  openUserActionsModal,
  closeUserActionModal,
  closeErrorModal,
  closeAllDocumentsModal,
  closeDocumentCreationModal,
  toggleMode,
  openDocumentTitleModal,
  showUserPosts,
  disableButtonAndShowSpinner,
  enableLoadingScreen,
  disableLoadingScreen,
} from "./editorActions.js";
import {
  downloadMarkdownToPDF,
  submitButtonAction,
  checkForLogInUser,
  logOut,
  createDocument,
  fetchUserDocuments,
  fetchCurrentDocumentContent,
} from "./api.js";
("use strict");
const MAX_WS_RECONNECTION_ATTEMPTS = 5;
const WS_RECONNECTION_DELAY = 2000;

let currentMode = "dark";
let reconnectionAttempts = 0;
const highlightEl = document.getElementById("highlight");
const editor = document.getElementById("editor");
editor.textContent = "";
const display = document.getElementById("markdown-display");
let currentDocuments = [];
let refreshMathTexCounter = 10;

const url = new URL("/refresh", window.location.href);
url.protocol = url.protocol.replace("http", "ws");
const webSocketConnection = new WebSocket(url.href);
setupWebSocketHandlers();

function setupWebSocketHandlers() {
  webSocketConnection.onmessage = (event) => {
    display.innerHTML = event.data;
  };

  webSocketConnection.onopen = () => {
    webSocketConnection.send(editor.value);
    reconnectionAttempts = 0; // Reset attempts after a successful connection
  };

  function attemptReconnect() {
    if (reconnectionAttempts >= MAX_WS_RECONNECTION_ATTEMPTS) {
      console.error("Max reconnection attempts reached. Could not reconnect.");
      return;
    }

    ++reconnectionAttempts;

    setTimeout(() => {
      webSocketConnection = new WebSocket(url.href);
      setupWebSocketHandlers(); // Reassign handlers to the new websocket instance
    }, WS_RECONNECTION_DELAY);
  }

  webSocketConnection.onerror = (e) => {
    console.error(`Connection error: ${JSON.stringify(e)}`);
    attemptReconnect();
  };

  webSocketConnection.onclose = () => {
    console.error("Connection closed");
    attemptReconnect();
  };
}

async function initialSetup() {
  const toggleModesButton = document.getElementById("toggle-modes");
  const downloadButton = document.getElementById("download");
  const registerButton = document.getElementById("sign-up");
  const logInButton = document.getElementById("log-in");
  const submitButton = document.getElementById("submit-button");
  const logOutButton = document.getElementById("log-out");
  const addButton = document.getElementById("add-document");
  const documentTitleSubmit = document.getElementById("document-title-submit");
  const documentTitleForm = document.getElementById("document-title-form");
  const showAllDocumentsButton = document.getElementById("all-documents");
  const errorCloseButton = document.getElementById("error-modal-close");
  const forgotPasswordLink = document.getElementById("forgot-password");

  forgotPasswordLink.onclick = () => {
    closeUserActionModal();
  };
  const allDocumentsCloseButton = document.getElementById(
    "document-close-button",
  );
  const addDocumentCloseButton = document.getElementById(
    "user-document-title-close",
  );
  const userActionModalClose = document.getElementById("user-modal-close");
  const userModal = document.getElementById("user-modal");

  userActionModalClose.onclick = closeUserActionModal;
  errorCloseButton.onclick = closeErrorModal;
  allDocumentsCloseButton.onclick = closeAllDocumentsModal;
  addDocumentCloseButton.onclick = closeDocumentCreationModal;

  userModal.addEventListener("animationend", () => {
    setTimeout(() => {
      userModal.classList.remove("error-shake-modal");
    }, 200);
  });

  showAllDocumentsButton.onclick = async () => {
    enableLoadingScreen();
    currentDocuments = await fetchUserDocuments();
    disableLoadingScreen();
    showUserPosts(currentDocuments, currentMode, fetchCurrentDocumentContent);
  };

  addButton.onclick = openDocumentTitleModal;

  documentTitleForm.onsubmit = (event) => {
    event.preventDefault();
  };

  documentTitleSubmit.onclick = async () => {
    if (!document.getElementById("document-title-field").value) {
      return;
    }

    disableButtonAndShowSpinner(documentTitleSubmit, currentMode);

    await createDocument(document.getElementById("document-title-field").value);
  };

  submitButton.onclick = async (event) => {
    event.preventDefault();
    disableButtonAndShowSpinner(submitButton, currentMode);
    submitButton.disabled = true;
    await submitButtonAction();
  };

  toggleModesButton.onclick = () => {
    currentMode = toggleMode(currentMode);
  };

  logOutButton.onclick = async () => {
    disableButtonAndShowSpinner(logOutButton, currentMode);
    await logOut();
  };

  registerButton.onclick = () => {
    openUserActionsModal("Register");
  };

  logInButton.onclick = () => {
    openUserActionsModal("Log In");
  };

  editor.setAttribute("data-initialized", true);

  editor.oninput = () => {
    highlight(editor, highlightEl);
    webSocketConnection.send(editor.value);
    resizeTextarea(editor);
    refreshMathTexCounter += 1;
  };

  editor.onkeyup = updateLineNumbers;

  editor.onkeydown = enableTabbing;

  downloadButton.onclick = async () => {
    disableButtonAndShowSpinner(downloadButton, currentMode);
    await downloadMarkdownToPDF(display.innerHTML, currentMode);
  };

  resizeTextarea(editor);
}

document.addEventListener("DOMContentLoaded", () => {
  resizeTextarea(editor);
  highlight(editor, highlight);
  initialSetup();
  checkForLogInUser();
  setInterval(() => {
    if (refreshMathTexCounter === 0) {
      renderMathInElement(document.body, {
        delimiters: [
          { left: "$$", right: "$$", display: true },
          { left: "$", right: "$", display: false },
          { left: "\\(", right: "\\)", display: false },
          { left: "\\[", right: "\\]", display: true },
        ],
        throwOnError: false,
      });
    }
    if (refreshMathTexCounter > 0) {
      refreshMathTexCounter -= 1;
    }
  }, 150);
});
